#!/usr/bin/env python3
"""Build and converge one immutable Coronatio snapshot with restart-safe recovery."""
from __future__ import annotations

import argparse
import fcntl
import hashlib
import json
import os
import re
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import time
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parent
BINARY_DEST = Path("/usr/local/bin/coronatio")
RUNTIME_ROOT = Path("/opt/coronatio")
WORKSPACE_ROOT = Path("/var/opt/hermes/workspace")
SOURCE_SHA_NAME = ".coronatio-installed-source-sha"
BINARY_SHA_NAME = ".coronatio-installed-binary-sha"
MANIFEST_NAME = ".coronatio-release-manifest.json"
JOURNAL_NAME = ".coronatio-transaction.json"
LOCK_NAME = ".coronatio-install.lock"
SERVICE_NAME = "coronatio.service"
HEALTH_URL = "http://127.0.0.1:3013/health"
CARGO_FALLBACKS = (Path("/usr/local/bin/cargo"), Path("/opt/cargo/bin/cargo"))
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
TOKEN_RE = re.compile(r"^[0-9a-f]{16}$")
FORWARD_STATES = {"prepared", "stopped", "old_source_moved", "new_source_moved", "old_binary_moved", "new_binary_moved", "old_manifest_moved", "new_manifest_written", "old_binary_sha_moved", "witness_updated", "service_started", "health_verified", "committed"}


class ConvergeResult:
    def __init__(self, status: str, source_sha: str, source_tree_sha256: str, static_sha256: str, binary_sha256: str):
        self.status = status
        self.source_sha = source_sha
        self.source_tree_sha256 = source_tree_sha256
        self.static_sha256 = static_sha256
        self.binary_sha256 = binary_sha256

    def manifest(self) -> dict[str, str]:
        return {"source_sha": self.source_sha, "source_tree_sha256": self.source_tree_sha256,
                "static_sha256": self.static_sha256, "binary_sha256": self.binary_sha256}


class ConvergeFailure(RuntimeError):
    def __init__(self, message: str, *, source_sha: str | None, manifest: dict[str, str] | None,
                 rollback_files: str, rollback_service: str, restored: bool):
        super().__init__(message)
        self.source_sha, self.manifest = source_sha, manifest
        self.rollback_files, self.rollback_service, self.restored = rollback_files, rollback_service, restored


def run(cmd: list[str], *, cwd: Path = REPO_ROOT, env: dict[str, str] | None = None) -> None:
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, check=True, cwd=cwd, env=env)


def fsync_dir(path: Path) -> None:
    fd = os.open(path, os.O_RDONLY | getattr(os, "O_DIRECTORY", 0))
    try: os.fsync(fd)
    finally: os.close(fd)


def durable_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(f".{path.name}.tmp-{os.getpid()}")
    with tmp.open("wb") as handle:
        handle.write(data); handle.flush(); os.fsync(handle.fileno())
    os.replace(tmp, path); fsync_dir(path.parent)


def durable_json(path: Path, value: dict[str, Any]) -> None:
    durable_write(path, (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode())


def durable_unlink(path: Path) -> None:
    path.unlink(missing_ok=True); fsync_dir(path.parent)


def fsync_file(path: Path) -> None:
    with path.open("rb") as handle:
        os.fsync(handle.fileno())


def fsync_tree(root: Path) -> None:
    """Persist every staged file and directory before it becomes a swap candidate."""
    for directory, _, files in os.walk(root, topdown=False):
        current = Path(directory)
        for name in files:
            candidate = current / name
            if candidate.is_file() and not candidate.is_symlink():
                fsync_file(candidate)
        fsync_dir(current)


def durable_copy(source: Path, destination: Path) -> None:
    if destination.exists() or destination.is_symlink():
        raise RuntimeError(f"staging path already exists: {destination}")
    shutil.copy2(source, destination)
    destination.chmod(0o755)
    fsync_file(destination)
    fsync_dir(destination.parent)


def acquire_install_lock(runtime_root: Path, *, expected_uid: int) -> int:
    """Acquire the fixed, nofollow process lock; caller owns the descriptor lifetime."""
    lock = runtime_root / LOCK_NAME
    try:
        fd = os.open(lock, os.O_RDWR | os.O_CREAT | getattr(os, "O_NOFOLLOW", 0), 0o600)
        info = os.fstat(fd)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != expected_uid:
            raise RuntimeError("unsafe installer lock file")
        try:
            fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as exc:
            raise RuntimeError("another Coronatio installer is active") from exc
        return fd
    except BaseException:
        try: os.close(fd)  # type: ignore[name-defined]
        except (OSError, UnboundLocalError): pass
        raise


def release_install_lock(fd: int) -> None:
    try: fcntl.flock(fd, fcntl.LOCK_UN)
    finally: os.close(fd)


def atomic_replace(source: Path, destination: Path) -> None:
    if source.stat().st_dev != destination.parent.stat().st_dev:
        raise RuntimeError(f"cross-filesystem replacement refused: {source} -> {destination}")
    os.replace(source, destination); fsync_dir(source.parent)
    if source.parent != destination.parent: fsync_dir(destination.parent)


def remove_path(path: Path) -> None:
    if path.is_dir() and not path.is_symlink(): shutil.rmtree(path)
    else: path.unlink(missing_ok=True)
    fsync_dir(path.parent)


def resolve_cargo() -> str:
    return shutil.which("cargo") or next((str(p) for p in CARGO_FALLBACKS if p.exists() and os.access(p, os.X_OK)), "") or (_ for _ in ()).throw(FileNotFoundError("cargo not found"))


def git(*args: str, cwd: Path = REPO_ROOT) -> str:
    return subprocess.check_output(["git", *args], cwd=cwd, text=True).strip()


def source_head() -> str:
    head = git("rev-parse", "HEAD")
    if not SHA_RE.fullmatch(head): raise RuntimeError("source HEAD is not a full lowercase Git SHA")
    return head


def assert_clean_snapshot(head: str) -> None:
    if git("diff", "--quiet", "--exit-code") or git("diff", "--cached", "--quiet", "--exit-code") or git("status", "--porcelain=v1", "--untracked-files=all"):
        raise RuntimeError("source snapshot is dirty")
    if source_head() != head: raise RuntimeError("source HEAD changed while preparing snapshot")


MAX_TREE_ENTRIES = 100_000
MAX_TREE_DEPTH = 64


def stable_file_digest(path: Path, *, executable: bool = False) -> str | None:
    try:
        before = path.lstat()
        if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
            return None
        if executable and not before.st_mode & stat.S_IXUSR:
            return None
        flags = os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0)
        fd = os.open(path, flags)
        try:
            opened = os.fstat(fd)
            if (opened.st_dev, opened.st_ino) != (before.st_dev, before.st_ino):
                return None
            digest = hashlib.sha256()
            while block := os.read(fd, 1024 * 1024):
                digest.update(block)
        finally:
            os.close(fd)
        after = path.lstat()
        if (before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns) != (
            after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns
        ):
            return None
        return digest.hexdigest()
    except OSError:
        return None


def digest_file(path: Path, *, executable: bool = False) -> str | None:
    return stable_file_digest(path, executable=executable)


def digest_tree(root: Path, *, excluded: set[str] | None = None) -> str | None:
    """Hash a bounded tree using path, type, mode, and file digest records.

    Directories are part of the grammar. Symlinks and every other nonregular
    node are refused, including a symlinked root or ancestor discovered while
    walking. The same grammar is used for staged candidate and installed trees.
    """
    excluded = excluded or set()
    try:
        for ancestor in (root, *root.parents):
            if ancestor.is_symlink():
                return None
        root_stat = root.lstat()
        if not stat.S_ISDIR(root_stat.st_mode):
            return None
        digest = hashlib.sha256()
        entries = 0

        def walk(directory: Path, relative: Path, depth: int) -> bool:
            nonlocal entries
            if depth > MAX_TREE_DEPTH:
                return False
            children = sorted(os.scandir(directory), key=lambda item: item.name)
            for child in children:
                entries += 1
                if entries > MAX_TREE_ENTRIES:
                    return False
                child_relative = relative / child.name
                rel_text = child_relative.as_posix()
                if rel_text in excluded or any(rel_text.startswith(item + "/") for item in excluded):
                    continue
                before = Path(child.path).lstat()
                mode = stat.S_IMODE(before.st_mode)
                if stat.S_ISDIR(before.st_mode):
                    digest.update(f"D\0{rel_text}\0{mode:04o}\0".encode())
                    if not walk(Path(child.path), child_relative, depth + 1):
                        return False
                elif stat.S_ISREG(before.st_mode):
                    file_digest = stable_file_digest(Path(child.path))
                    after = Path(child.path).lstat()
                    if file_digest is None or (before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns) != (
                        after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns
                    ):
                        return False
                    digest.update(f"F\0{rel_text}\0{mode:04o}\0".encode())
                    digest.update(bytes.fromhex(file_digest))
                else:
                    return False
            return True

        return digest.hexdigest() if walk(root, Path(), 0) else None
    except OSError:
        return None


def source_tree_digest(source: Path) -> str | None:
    # Git archive supplies the whole runtime source. Only build output and installer witness are excluded.
    return digest_tree(source, excluded={"target", SOURCE_SHA_NAME})


def read_sha(path: Path) -> str | None:
    try: value = path.read_text("ascii").strip()
    except OSError: return None
    return value if SHA_RE.fullmatch(value) else None


def read_manifest(path: Path) -> dict[str, str] | None:
    try: value = json.loads(path.read_text("utf-8"))
    except (OSError, ValueError): return None
    required = ("source_sha", "source_tree_sha256", "static_sha256", "binary_sha256")
    return value if isinstance(value, dict) and all(isinstance(value.get(k), str) for k in required) and SHA_RE.fullmatch(value["source_sha"]) and all(SHA256_RE.fullmatch(value[k]) for k in required[1:]) else None


def current_matches(runtime_root: Path, binary_dest: Path, head: str) -> bool:
    source = runtime_root / "source"; manifest = read_manifest(runtime_root / MANIFEST_NAME)
    return bool(manifest and manifest["source_sha"] == head and read_sha(source / SOURCE_SHA_NAME) == head and read_sha(runtime_root / BINARY_SHA_NAME) == head and manifest["source_tree_sha256"] == source_tree_digest(source) and manifest["static_sha256"] == digest_tree(source / "static") and manifest["binary_sha256"] == digest_file(binary_dest, executable=True))


def assert_service_exists() -> None:
    try: run(["systemctl", "cat", SERVICE_NAME])
    except (FileNotFoundError, subprocess.CalledProcessError) as exc: raise RuntimeError(f"{SERVICE_NAME} is absent or unreadable") from exc


def service_active() -> bool:
    try:
        subprocess.run(["systemctl", "is-active", "--quiet", SERVICE_NAME], check=True)
        return True
    except (FileNotFoundError, subprocess.CalledProcessError): return False


def snapshot_and_build(runtime_root: Path, binary_dest: Path, head: str, cargo: str, token: str) -> tuple[Path, Path, dict[str, str]]:
    assert_clean_snapshot(head)
    stage = runtime_root / f".coronatio-stage-{token}"
    stage.mkdir(mode=0o700); fsync_dir(runtime_root)
    source = stage / "source"; archive = stage / "snapshot.tar"
    binary_stage: Path | None = None
    try:
        with archive.open("wb") as out:
            subprocess.run(["git", "archive", "--format=tar", head], cwd=REPO_ROOT, stdout=out, check=True); out.flush(); os.fsync(out.fileno())
        with tarfile.open(archive) as tf: tf.extractall(source, filter="data")
        fsync_tree(source)
        durable_unlink(archive)
        env = dict(os.environ)
        env["CORONATIO_SOURCE_SHA"] = head
        env["CORONATIO_BUILD_SHA"] = head
        run([cargo, "build", "--release"], cwd=source, env=env)
        built = source / "target" / "release" / "coronatio"
        if not digest_file(built, executable=True): raise RuntimeError("release binary missing, non-regular, or non-executable")
        binary_stage = binary_dest.parent / f".coronatio-stage-bin-{token}"
        durable_copy(built, binary_stage)
        shutil.rmtree(source / "target"); fsync_dir(source)
        fsync_tree(source)
        assert_clean_snapshot(head)
        manifest = {"source_sha": head, "source_tree_sha256": source_tree_digest(source), "static_sha256": digest_tree(source / "static"), "binary_sha256": digest_file(binary_stage, executable=True)}
        if not all(manifest.values()): raise RuntimeError("immutable snapshot artifact digest failed")
        return source, binary_stage, manifest  # type: ignore[return-value]
    except BaseException:
        remove_path(stage)
        if binary_stage is not None and binary_stage.exists(): remove_path(binary_stage)
        raise


def journal_path(runtime_root: Path) -> Path: return runtime_root / JOURNAL_NAME


def expected_paths(runtime_root: Path, binary_dest: Path, token: str) -> dict[str, Path]:
    return {"source_stage": runtime_root / f".coronatio-stage-{token}" / "source", "binary_stage": binary_dest.parent / f".coronatio-stage-bin-{token}", "source_backup": runtime_root / f".coronatio-source-backup-{token}", "binary_backup": binary_dest.parent / f".{binary_dest.name}.backup-{token}", "manifest_backup": runtime_root / f".{MANIFEST_NAME}.backup-{token}", "binary_sha_backup": runtime_root / f".{BINARY_SHA_NAME}.backup-{token}"}


def artifact_digest(name: str, path: Path) -> str | None:
    if name == "source":
        return source_tree_digest(path)
    return digest_file(path, executable=name == "binary")


def validate_journal(runtime_root: Path, binary_dest: Path, tx: Any) -> dict[str, Any]:
    if not isinstance(tx, dict) or tx.get("version") != 2 or not isinstance(tx.get("token"), str) or not TOKEN_RE.fullmatch(tx["token"]) or tx.get("state") not in FORWARD_STATES: raise RuntimeError("malformed transaction journal; preserving artifacts")
    if not isinstance(tx.get("new_sha"), str) or not SHA_RE.fullmatch(tx["new_sha"]) or not isinstance(tx.get("old"), dict): raise RuntimeError("incomplete transaction journal; preserving artifacts")
    expected = expected_paths(runtime_root, binary_dest, tx["token"])
    for name, path in expected.items():
        if tx.get(name) != str(path): raise RuntimeError("path-invalid transaction journal; preserving artifacts")
    for name in ("source", "binary", "manifest", "binary_sha"):
        if not isinstance(tx["old"].get(name), bool): raise RuntimeError("incomplete transaction journal; preserving artifacts")
    old_digests = tx.get("old_digests")
    if not isinstance(old_digests, dict):
        raise RuntimeError("missing old artifact digests; preserving journal")
    for name in ("source", "binary", "manifest", "binary_sha"):
        digest = old_digests.get(name)
        if tx["old"][name] and (not isinstance(digest, str) or not SHA256_RE.fullmatch(digest)):
            raise RuntimeError("invalid old artifact digest; preserving journal")
        if not tx["old"][name] and digest is not None:
            raise RuntimeError("absent artifact cannot have old digest; preserving journal")
    return tx


def load_journal(runtime_root: Path, binary_dest: Path) -> dict[str, Any] | None:
    path = journal_path(runtime_root)
    if not path.exists(): return None
    try: raw = json.loads(path.read_text("utf-8"))
    except (OSError, ValueError) as exc: raise RuntimeError("unreadable transaction journal; preserving artifacts") from exc
    return validate_journal(runtime_root, binary_dest, raw)


def set_state(runtime_root: Path, tx: dict[str, Any], state: str) -> None:
    if state not in FORWARD_STATES: raise ValueError(state)
    tx["state"] = state; durable_json(journal_path(runtime_root), tx)


def restore_old(destination: Path, backup: Path, existed: bool, expected_digest: str | None, name: str) -> Path | None:
    """Restore only an attested prior artifact; never bless an intervening one."""
    displaced = backup.with_name(backup.name + ".recovery")
    if existed:
        if backup.exists():
            if expected_digest and artifact_digest(name, backup) != expected_digest:
                raise RuntimeError(f"backup identity mismatch for {name}; preserving journal")
            if destination.exists() or destination.is_symlink():
                if displaced.exists() or displaced.is_symlink(): remove_path(displaced)
                atomic_replace(destination, displaced)
            atomic_replace(backup, destination)
            if expected_digest and artifact_digest(name, destination) != expected_digest:
                raise RuntimeError(f"restored identity mismatch for {name}; preserving journal")
        elif not (destination.exists() or destination.is_symlink()):
            raise RuntimeError(f"recovery lacks old {destination}; preserving journal")
        elif expected_digest and artifact_digest(name, destination) != expected_digest:
            raise RuntimeError(f"foreign destination for {name}; preserving journal")
    elif destination.exists() or destination.is_symlink():
        remove_path(destination)
    return displaced if displaced.exists() or displaced.is_symlink() else None


def cleanup_transaction(runtime_root: Path, binary_dest: Path, tx: dict[str, Any]) -> None:
    for path in expected_paths(runtime_root, binary_dest, tx["token"]).values():
        for artifact in (path, path.with_name(path.name + ".recovery")):
            if artifact.exists() or artifact.is_symlink(): remove_path(artifact)
    parent = runtime_root / f".coronatio-stage-{tx['token']}"
    if parent.exists(): remove_path(parent)
    durable_unlink(journal_path(runtime_root))


def recover_transaction(runtime_root: Path, binary_dest: Path, *, restart: bool = True) -> bool:
    tx = load_journal(runtime_root, binary_dest)
    if tx is None: return False
    if tx["state"] == "committed":
        cleanup_transaction(runtime_root, binary_dest, tx); return True
    try: run(["systemctl", "stop", SERVICE_NAME])
    except (FileNotFoundError, subprocess.CalledProcessError) as exc:
        raise RuntimeError("recovery cannot stop service; preserving journal") from exc
    p = expected_paths(runtime_root, binary_dest, tx["token"]); old = tx["old"]
    old_digests = tx.get("old_digests", {})
    # os.replace restores directly over a candidate only after proving the old identity.
    restore_old(runtime_root / "source", p["source_backup"], old["source"], old_digests.get("source"), "source")
    restore_old(binary_dest, p["binary_backup"], old["binary"], old_digests.get("binary"), "binary")
    restore_old(runtime_root / MANIFEST_NAME, p["manifest_backup"], old["manifest"], old_digests.get("manifest"), "manifest")
    restore_old(runtime_root / BINARY_SHA_NAME, p["binary_sha_backup"], old["binary_sha"], old_digests.get("binary_sha"), "binary_sha")
    set_state(runtime_root, tx, "prepared")
    cleanup_transaction(runtime_root, binary_dest, tx)
    if restart and (runtime_root / "source").exists() and binary_dest.exists(): run(["systemctl", "start", SERVICE_NAME])
    return True


def fetch_health(url: str, expected_sha: str, timeout_seconds: float) -> tuple[bool, str, dict[str, Any] | None]:
    try:
        with urllib.request.urlopen(urllib.request.Request(url), timeout=timeout_seconds) as response:
            status = response.status
            body = response.read().decode("utf-8")
        value = json.loads(body)
        if isinstance(value, dict):
            value = {**value, "http_status": status}
        if not 200 <= status < 300:
            return False, f"HTTP {status}", value if isinstance(value, dict) else None
        if (
            not isinstance(value, dict)
            or value.get("schema") != "coronatio.health.v1"
            or value.get("ok") is not True
            or value.get("service") != "coronatio"
        ):
            return False, "invalid health schema", value if isinstance(value, dict) else None
        source_sha, build_sha = value.get("source_sha"), value.get("build_sha")
        valid = all(isinstance(item, str) and SHA_RE.fullmatch(item) for item in (source_sha, build_sha))
        return bool(valid and source_sha == expected_sha and build_sha == expected_sha), "health JSON", value
    except (urllib.error.URLError, TimeoutError, OSError, ValueError, UnicodeDecodeError) as exc:
        return False, str(exc), None


def probe_health(url: str, expected_sha: str, timeout_seconds: float) -> tuple[bool, str]:
    ok, detail, _ = fetch_health(url, expected_sha, timeout_seconds)
    return ok, detail


def health_gate(url: str, expected_sha: str, retries: int, delay_seconds: float, timeout_seconds: float) -> dict[str, Any]:
    last_detail = "health gate failed exact source-SHA attestation"
    for attempt in range(retries):
        ok, detail, value = fetch_health(url, expected_sha, timeout_seconds)
        last_detail = detail
        if ok:
            return value or {}
        if attempt + 1 < retries:
            time.sleep(delay_seconds)
    raise RuntimeError(last_detail)


def ensure_owned_directory(path: Path, expected_uid: int, label: str) -> None:
    st = path.lstat()
    if path.is_symlink() or not stat.S_ISDIR(st.st_mode) or st.st_uid != expected_uid:
        raise RuntimeError(f"unsafe {label}: {path}")


def ensure_runtime_parents(runtime_root: Path, binary_dest: Path, *, expected_uid: int) -> None:
    """Refuse symlink/non-directory parents before creating transaction stages."""
    for parent in (runtime_root.parent, binary_dest.parent):
        if parent.exists() or parent.is_symlink():
            ensure_owned_directory(parent, expected_uid, "fixed product parent")
        else:
            parent.mkdir(parents=True, mode=0o755)
            ensure_owned_directory(parent, expected_uid, "created product parent")


def result_from_manifest(status: str, manifest: dict[str, str]) -> ConvergeResult:
    values = (manifest.get("source_sha"), manifest.get("source_tree_sha256"), manifest.get("static_sha256"), manifest.get("binary_sha256"))
    if not isinstance(values[0], str) or not SHA_RE.fullmatch(values[0]) or any(not isinstance(value, str) or not SHA256_RE.fullmatch(value) for value in values[1:]):
        raise RuntimeError("invalid installed release manifest")
    return ConvergeResult(status, values[0], values[1], values[2], values[3])


def converge(*, runtime_root: Path, binary_dest: Path, health_url: str, retries: int, delay_seconds: float, timeout_seconds: float, cargo: str | None = None, expected_uid: int = 0) -> ConvergeResult:
    ensure_runtime_parents(runtime_root, binary_dest, expected_uid=expected_uid)
    runtime_root.mkdir(parents=False, exist_ok=True)
    ensure_owned_directory(runtime_root, expected_uid, "runtime root")
    recover_transaction(runtime_root, binary_dest)
    head = source_head(); assert_clean_snapshot(head)
    installed = read_manifest(runtime_root / MANIFEST_NAME)
    if current_matches(runtime_root, binary_dest, head):
        if not service_active():
            assert_service_exists(); run(["systemctl", "start", SERVICE_NAME])
            health_gate(health_url, head, retries, delay_seconds, timeout_seconds)
            return result_from_manifest("converged", installed or {})
        health_gate(health_url, head, retries, delay_seconds, timeout_seconds)
        return result_from_manifest("no-op", installed or {})
    assert_service_exists()
    token = hashlib.sha256((head + str(time.time_ns())).encode()).hexdigest()[:16]
    source_stage, binary_stage, manifest = snapshot_and_build(runtime_root, binary_dest, head, cargo or resolve_cargo(), token)
    p = expected_paths(runtime_root, binary_dest, token)
    destinations = {"source": runtime_root / "source", "binary": binary_dest, "manifest": runtime_root / MANIFEST_NAME, "binary_sha": runtime_root / BINARY_SHA_NAME}
    old = {name: path.exists() for name, path in destinations.items()}
    old_digests = {name: artifact_digest(name, path) for name, path in destinations.items() if old[name]}
    if any(value is None for value in old_digests.values()):
        raise RuntimeError("existing release artifact is not attestable")
    tx: dict[str, Any] = {"version": 2, "token": token, "state": "prepared", "new_sha": head,
        **{key: str(value) for key, value in p.items()}, "old": old, "old_digests": old_digests}
    set_state(runtime_root, tx, "prepared")
    try:
        run(["systemctl", "stop", SERVICE_NAME]); set_state(runtime_root, tx, "stopped")
        if old["source"]: atomic_replace(destinations["source"], p["source_backup"])
        set_state(runtime_root, tx, "old_source_moved"); atomic_replace(source_stage, destinations["source"]); durable_write(destinations["source"] / SOURCE_SHA_NAME, (head + "\n").encode()); fsync_tree(destinations["source"]); set_state(runtime_root, tx, "new_source_moved")
        if old["binary"]: atomic_replace(binary_dest, p["binary_backup"])
        set_state(runtime_root, tx, "old_binary_moved"); atomic_replace(binary_stage, binary_dest); fsync_file(binary_dest); fsync_dir(binary_dest.parent); set_state(runtime_root, tx, "new_binary_moved")
        if old["manifest"]: atomic_replace(destinations["manifest"], p["manifest_backup"])
        set_state(runtime_root, tx, "old_manifest_moved"); durable_json(destinations["manifest"], manifest); set_state(runtime_root, tx, "new_manifest_written")
        if old["binary_sha"]: atomic_replace(destinations["binary_sha"], p["binary_sha_backup"])
        set_state(runtime_root, tx, "old_binary_sha_moved"); durable_write(destinations["binary_sha"], (head + "\n").encode()); set_state(runtime_root, tx, "witness_updated")
        run(["systemctl", "start", SERVICE_NAME]); set_state(runtime_root, tx, "service_started"); health_gate(health_url, head, retries, delay_seconds, timeout_seconds)
        if not current_matches(runtime_root, binary_dest, head): raise RuntimeError("postcondition artifact parity mismatch")
        exact = read_manifest(destinations["manifest"])
        result = result_from_manifest("converged", exact or {})
        if result.manifest() != manifest: raise RuntimeError("installed receipt manifest mismatch")
        set_state(runtime_root, tx, "health_verified"); set_state(runtime_root, tx, "committed"); cleanup_transaction(runtime_root, binary_dest, tx)
        return result
    except BaseException as exc:
        try:
            restored = recover_transaction(runtime_root, binary_dest)
            rollback = "restored" if restored else "not-needed"
            raise ConvergeFailure(str(exc), source_sha=head, manifest=manifest, rollback_files=rollback, rollback_service=rollback, restored=restored) from exc
        except ConvergeFailure:
            raise
        except BaseException as recovery:
            raise ConvergeFailure(str(exc), source_sha=head, manifest=manifest, rollback_files="failed", rollback_service="failed", restored=False) from recovery


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--build-only", action="store_true")
    mode.add_argument("--health-only", action="store_true")
    parser.add_argument("--scratch-root", type=Path, help="safe non-production root; uses ROOT/runtime and ROOT/bin/coronatio")
    parser.add_argument("--health-url", default=HEALTH_URL)
    parser.add_argument("--health-retries", type=int, default=30)
    parser.add_argument("--health-delay", type=float, default=1.0)
    parser.add_argument("--health-timeout", type=float, default=2.0)
    args = parser.parse_args(argv)
    if args.health_retries < 1 or args.health_delay < 0 or args.health_timeout <= 0:
        parser.error("health retries must be >= 1, delay >= 0, and timeout > 0")
    if args.scratch_root is not None and (args.build_only or args.health_only):
        parser.error("--scratch-root is only valid for the real installer")
    return args


def install_receipt(
    ok: bool,
    status: str,
    *,
    result: ConvergeResult | None = None,
    source_sha: str | None = None,
    manifest: dict[str, str] | None = None,
    error: str | None = None,
    rollback_files: str = "not-needed",
    rollback_service: str = "not-attempted",
) -> dict[str, object]:
    manifest = result.manifest() if result else manifest or {}
    source_sha = result.source_sha if result else source_sha or manifest.get("source_sha")
    return {
        "schema": "coronatio.install.receipt.v1",
        "ok": ok,
        "status": status,
        "source_sha": source_sha,
        "source_tree_sha256": manifest.get("source_tree_sha256"),
        "static_sha256": manifest.get("static_sha256"),
        "binary_sha256": manifest.get("binary_sha256"),
        "firstMissingSignal": error,
        "rollback": rollback_files,
        "rollbackFiles": rollback_files,
        "rollbackService": rollback_service,
    }


def health_receipt(
    ok: bool,
    status: str,
    *,
    observed: dict[str, Any] | None = None,
    manifest: dict[str, str] | None = None,
    error: str | None = None,
    health_url: str = HEALTH_URL,
) -> dict[str, object]:
    observed = observed or {}
    manifest = manifest or {}
    return {
        "schema": "coronatio.health.v1",
        "ok": ok,
        "status": status,
        "service": "coronatio",
        "health_url": health_url,
        "http_status": observed.get("http_status"),
        "source_sha": observed.get("source_sha"),
        "build_sha": observed.get("build_sha"),
        "source_tree_sha256": manifest.get("source_tree_sha256"),
        "static_sha256": manifest.get("static_sha256"),
        "binary_sha256": manifest.get("binary_sha256"),
        "firstMissingSignal": error,
    }


def valid_success_receipt(receipt: object, head: str) -> bool:
    """Fulcrum-equivalent success parser: exact head plus all lowerhex artifact hashes."""
    return bool(
        isinstance(receipt, dict)
        and receipt.get("schema") == "coronatio.install.receipt.v1"
        and receipt.get("ok") is True
        and receipt.get("status") in {"no-op", "converged"}
        and receipt.get("source_sha") == head
        and SHA_RE.fullmatch(head)
        and all(
            isinstance(receipt.get(key), str) and SHA256_RE.fullmatch(receipt[key])
            for key in ("source_tree_sha256", "static_sha256", "binary_sha256")
        )
    )


def scratch_destinations(root: Path) -> tuple[Path, Path]:
    if not root.is_absolute():
        raise RuntimeError("--scratch-root must be an absolute path")
    if WORKSPACE_ROOT.is_symlink() or not WORKSPACE_ROOT.is_dir():
        raise RuntimeError("workspace root is unavailable or symlinked")
    try:
        relative = root.relative_to(WORKSPACE_ROOT)
    except ValueError as exc:
        raise RuntimeError("scratch root must be beneath /var/opt/hermes/workspace") from exc
    if not relative.parts:
        raise RuntimeError("scratch root cannot be the workspace root")
    current = WORKSPACE_ROOT
    for part in relative.parts:
        current /= part
        if current.is_symlink():
            raise RuntimeError("scratch root contains a symlinked ancestor")
        if current.exists() and not current.is_dir():
            raise RuntimeError("scratch root contains a non-directory ancestor")
    resolved = root.resolve(strict=False)
    if resolved == WORKSPACE_ROOT or WORKSPACE_ROOT not in resolved.parents:
        raise RuntimeError("resolved scratch root must be beneath workspace root")
    return resolved / "runtime", resolved / "bin" / BINARY_DEST.name


def main(argv: list[str] | None = None) -> int:
    source_sha: str | None = None
    lock_fd: int | None = None
    health = health_receipt(False, "not-attempted")
    try:
        args = parse_args(argv)
        source_sha = source_head()
        if args.health_only:
            observed = health_gate(args.health_url, source_sha, args.health_retries, args.health_delay, args.health_timeout)
            health = health_receipt(True, "diagnostic", observed=observed, health_url=args.health_url)
            result = install_receipt(True, "diagnostic", source_sha=source_sha)
        elif args.build_only:
            root = Path(tempfile.mkdtemp(prefix="coronatio-build-"))
            try:
                # Both runtime and binary stage stay under the disposable root.
                (root / "bin").mkdir()
                snapshot_and_build(root, root / "bin" / "coronatio", source_sha, resolve_cargo(), "0" * 16)
            finally:
                shutil.rmtree(root, ignore_errors=True)
            result = install_receipt(True, "diagnostic", source_sha=source_sha)
        else:
            scratch = args.scratch_root is not None
            expected_uid = os.geteuid() if scratch else 0
            runtime_root, binary_dest = (
                scratch_destinations(args.scratch_root) if scratch else (RUNTIME_ROOT, BINARY_DEST)
            )
            ensure_runtime_parents(runtime_root, binary_dest, expected_uid=expected_uid)
            runtime_root.mkdir(parents=False, exist_ok=True)
            ensure_owned_directory(runtime_root, expected_uid, "runtime root")
            lock_fd = acquire_install_lock(runtime_root, expected_uid=expected_uid)
            converged = converge(
                runtime_root=runtime_root,
                binary_dest=binary_dest,
                health_url=args.health_url,
                retries=args.health_retries,
                delay_seconds=args.health_delay,
                timeout_seconds=args.health_timeout,
                expected_uid=expected_uid,
            )
            observed = health_gate(args.health_url, source_sha, args.health_retries, args.health_delay, args.health_timeout)
            health = health_receipt(
                True,
                "verified",
                observed=observed,
                manifest=converged.manifest(),
                health_url=args.health_url,
            )
            result = install_receipt(True, converged.status, result=converged)
            if not valid_success_receipt(result, source_sha):
                raise RuntimeError("installer generated an invalid success receipt")
        print(json.dumps(health, sort_keys=True))
        print(json.dumps(result, sort_keys=True))
        return 0
    except ConvergeFailure as exc:
        health = health_receipt(False, "failed", manifest=exc.manifest, error=str(exc))
        result = install_receipt(
            False,
            "failed",
            source_sha=exc.source_sha,
            manifest=exc.manifest,
            error=str(exc),
            rollback_files=exc.rollback_files,
            rollback_service=exc.rollback_service,
        )
        print(json.dumps(health, sort_keys=True))
        print(json.dumps(result, sort_keys=True))
        return 1
    except (FileNotFoundError, RuntimeError, subprocess.CalledProcessError, OSError) as exc:
        health["firstMissingSignal"] = str(exc)
        result = install_receipt(
            False,
            "failed",
            source_sha=source_sha,
            error=str(exc),
            rollback_files="not-attempted",
            rollback_service="not-attempted",
        )
        print(json.dumps(health, sort_keys=True))
        print(json.dumps(result, sort_keys=True))
        return 1
    finally:
        if lock_fd is not None:
            release_install_lock(lock_fd)


if __name__ == "__main__":
    raise SystemExit(main())
