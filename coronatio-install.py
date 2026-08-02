#!/usr/bin/env python3
"""Build and converge one immutable Coronatio snapshot with restart-safe recovery."""
from __future__ import annotations

import argparse
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
SOURCE_SHA_NAME = ".coronatio-installed-source-sha"
BINARY_SHA_NAME = ".coronatio-installed-binary-sha"
MANIFEST_NAME = ".coronatio-release-manifest.json"
JOURNAL_NAME = ".coronatio-transaction.json"
SERVICE_NAME = "coronatio.service"
HEALTH_URL = "http://127.0.0.1:3013/health"
CARGO_FALLBACKS = (Path("/usr/local/bin/cargo"), Path("/opt/cargo/bin/cargo"))
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
TOKEN_RE = re.compile(r"^[0-9a-f]{16}$")
FORWARD_STATES = {"prepared", "stopped", "old_source_moved", "new_source_moved", "old_binary_moved", "new_binary_moved", "old_manifest_moved", "new_manifest_written", "old_binary_sha_moved", "witness_updated", "service_started", "health_verified", "committed"}


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
    return value if isinstance(value, dict) and all(isinstance(value.get(k), str) for k in required) and SHA_RE.fullmatch(value["source_sha"]) else None


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
        durable_unlink(archive)
        env = dict(os.environ)
        env["CORONATIO_SOURCE_SHA"] = head
        env["CORONATIO_BUILD_SHA"] = head
        run([cargo, "build", "--release"], cwd=source, env=env)
        built = source / "target" / "release" / "coronatio"
        if not digest_file(built, executable=True): raise RuntimeError("release binary missing, non-regular, or non-executable")
        binary_stage = binary_dest.parent / f".coronatio-stage-bin-{token}"
        shutil.copy2(built, binary_stage); binary_stage.chmod(0o755); fsync_dir(binary_stage.parent)
        shutil.rmtree(source / "target"); fsync_dir(source)
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


def validate_journal(runtime_root: Path, binary_dest: Path, tx: Any) -> dict[str, Any]:
    if not isinstance(tx, dict) or tx.get("version") != 2 or not isinstance(tx.get("token"), str) or not TOKEN_RE.fullmatch(tx["token"]) or tx.get("state") not in FORWARD_STATES: raise RuntimeError("malformed transaction journal; preserving artifacts")
    if not isinstance(tx.get("new_sha"), str) or not SHA_RE.fullmatch(tx["new_sha"]) or not isinstance(tx.get("old"), dict): raise RuntimeError("incomplete transaction journal; preserving artifacts")
    expected = expected_paths(runtime_root, binary_dest, tx["token"])
    for name, path in expected.items():
        if tx.get(name) != str(path): raise RuntimeError("path-invalid transaction journal; preserving artifacts")
    for name in ("source", "binary", "manifest", "binary_sha"):
        if not isinstance(tx["old"].get(name), bool): raise RuntimeError("incomplete transaction journal; preserving artifacts")
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


def restore_old(destination: Path, backup: Path, existed: bool) -> Path | None:
    """Restore without unlinking a valid destination; a displaced candidate is retained until cleanup."""
    displaced = backup.with_name(backup.name + ".recovery")
    if existed:
        if backup.exists():
            if destination.exists() or destination.is_symlink():
                if displaced.exists() or displaced.is_symlink(): remove_path(displaced)
                atomic_replace(destination, displaced)
            atomic_replace(backup, destination)
        elif not (destination.exists() or destination.is_symlink()):
            raise RuntimeError(f"recovery lacks old {destination}; preserving journal")
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
    except (FileNotFoundError, subprocess.CalledProcessError): pass
    p = expected_paths(runtime_root, binary_dest, tx["token"]); old = tx["old"]
    # os.replace restores directly over a candidate; re-entry after an interrupted restore sees the old destination and keeps it.
    restore_old(runtime_root / "source", p["source_backup"], old["source"])
    restore_old(binary_dest, p["binary_backup"], old["binary"])
    restore_old(runtime_root / MANIFEST_NAME, p["manifest_backup"], old["manifest"])
    restore_old(runtime_root / BINARY_SHA_NAME, p["binary_sha_backup"], old["binary_sha"])
    set_state(runtime_root, tx, "prepared")
    cleanup_transaction(runtime_root, binary_dest, tx)
    if restart and (runtime_root / "source").exists() and binary_dest.exists(): run(["systemctl", "start", SERVICE_NAME])
    return True


def probe_health(url: str, expected_sha: str, timeout_seconds: float) -> tuple[bool, str]:
    try:
        with urllib.request.urlopen(urllib.request.Request(url), timeout=timeout_seconds) as response:
            if not 200 <= response.status < 300:
                return False, f"HTTP {response.status}"
            value = json.loads(response.read().decode("utf-8"))
        if not isinstance(value, dict) or value.get("schema") != "coronatio.health.v1" or value.get("ok") is not True or value.get("service") != "coronatio":
            return False, "invalid health schema"
        source_sha, build_sha = value.get("source_sha"), value.get("build_sha")
        valid = all(isinstance(item, str) and SHA_RE.fullmatch(item) for item in (source_sha, build_sha))
        return (bool(valid and source_sha == expected_sha and build_sha == expected_sha), "health JSON")
    except (urllib.error.URLError, TimeoutError, OSError, ValueError, UnicodeDecodeError) as exc:
        return False, str(exc)


def health_gate(url: str, expected_sha: str, retries: int, delay_seconds: float, timeout_seconds: float) -> None:
    for attempt in range(retries):
        ok, _ = probe_health(url, expected_sha, timeout_seconds)
        if ok: return
        if attempt + 1 < retries: time.sleep(delay_seconds)
    raise RuntimeError("health gate failed exact source-SHA attestation")


def ensure_runtime_parents(runtime_root: Path, binary_dest: Path) -> None:
    """Refuse symlink/non-directory parents before creating transaction stages."""
    for parent in (runtime_root.parent, binary_dest.parent):
        if parent.exists() or parent.is_symlink():
            st = parent.lstat()
            if parent.is_symlink() or not stat.S_ISDIR(st.st_mode):
                raise RuntimeError(f"unsafe fixed product parent: {parent}")
            if st.st_uid != 0:
                raise RuntimeError(f"fixed product parent is not root-owned: {parent}")
        else:
            parent.mkdir(parents=True, mode=0o755)
            st = parent.lstat()
            if st.st_uid != 0 or not stat.S_ISDIR(st.st_mode):
                raise RuntimeError(f"failed to create safe fixed product parent: {parent}")


def converge(*, runtime_root: Path, binary_dest: Path, health_url: str, retries: int, delay_seconds: float, timeout_seconds: float, cargo: str | None = None) -> str:
    ensure_runtime_parents(runtime_root, binary_dest)
    runtime_root.mkdir(parents=False, exist_ok=True); recover_transaction(runtime_root, binary_dest)
    head = source_head(); assert_clean_snapshot(head)
    if current_matches(runtime_root, binary_dest, head) and service_active():
        health_gate(health_url, head, retries, delay_seconds, timeout_seconds); print(f"Coronatio converge no-op: source and running SHA {head}"); return head
    assert_service_exists()
    if current_matches(runtime_root, binary_dest, head):
        run(["systemctl", "start", SERVICE_NAME]); health_gate(health_url, head, retries, delay_seconds, timeout_seconds); return head
    token = hashlib.sha256((head + str(time.time_ns())).encode()).hexdigest()[:16]
    source_stage, binary_stage, manifest = snapshot_and_build(runtime_root, binary_dest, head, cargo or resolve_cargo(), token)
    p = expected_paths(runtime_root, binary_dest, token)
    tx: dict[str, Any] = {"version": 2, "token": token, "state": "prepared", "new_sha": head, "source_stage": str(source_stage), "binary_stage": str(binary_stage), "source_backup": str(p["source_backup"]), "binary_backup": str(p["binary_backup"]), "manifest_backup": str(p["manifest_backup"]), "binary_sha_backup": str(p["binary_sha_backup"]), "old": {"source": (runtime_root / "source").exists(), "binary": binary_dest.exists(), "manifest": (runtime_root / MANIFEST_NAME).exists(), "binary_sha": (runtime_root / BINARY_SHA_NAME).exists()}}
    set_state(runtime_root, tx, "prepared")
    try:
        run(["systemctl", "stop", SERVICE_NAME]); set_state(runtime_root, tx, "stopped")
        if tx["old"]["source"]: atomic_replace(runtime_root / "source", p["source_backup"])
        set_state(runtime_root, tx, "old_source_moved"); atomic_replace(source_stage, runtime_root / "source"); durable_write(runtime_root / "source" / SOURCE_SHA_NAME, (head + "\n").encode()); set_state(runtime_root, tx, "new_source_moved")
        if tx["old"]["binary"]: atomic_replace(binary_dest, p["binary_backup"])
        set_state(runtime_root, tx, "old_binary_moved"); atomic_replace(binary_stage, binary_dest); set_state(runtime_root, tx, "new_binary_moved")
        if tx["old"]["manifest"]: atomic_replace(runtime_root / MANIFEST_NAME, p["manifest_backup"])
        set_state(runtime_root, tx, "old_manifest_moved"); durable_json(runtime_root / MANIFEST_NAME, manifest); set_state(runtime_root, tx, "new_manifest_written")
        if tx["old"]["binary_sha"]: atomic_replace(runtime_root / BINARY_SHA_NAME, p["binary_sha_backup"])
        set_state(runtime_root, tx, "old_binary_sha_moved"); durable_write(runtime_root / BINARY_SHA_NAME, (head + "\n").encode()); set_state(runtime_root, tx, "witness_updated")
        run(["systemctl", "start", SERVICE_NAME]); set_state(runtime_root, tx, "service_started"); health_gate(health_url, head, retries, delay_seconds, timeout_seconds)
        if not current_matches(runtime_root, binary_dest, head): raise RuntimeError("postcondition artifact parity mismatch")
        set_state(runtime_root, tx, "health_verified"); set_state(runtime_root, tx, "committed"); cleanup_transaction(runtime_root, binary_dest, tx); print(f"Coronatio converge complete: source_sha={head}"); return head
    except BaseException:
        recover_transaction(runtime_root, binary_dest); raise


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--build-only", action="store_true")
    mode.add_argument("--health-only", action="store_true")
    parser.add_argument("--health-url", default=HEALTH_URL)
    parser.add_argument("--health-retries", type=int, default=30)
    parser.add_argument("--health-delay", type=float, default=1.0)
    parser.add_argument("--health-timeout", type=float, default=2.0)
    args = parser.parse_args(argv)
    if args.health_retries < 1 or args.health_delay < 0 or args.health_timeout <= 0:
        parser.error("health retries must be >= 1, delay >= 0, and timeout > 0")
    return args


def install_receipt(ok: bool, status: str, *, source_sha: str | None = None, error: str | None = None, rollback: str = "not-needed") -> dict[str, object]:
    return {
        "schema": "coronatio.install.receipt.v1",
        "ok": ok,
        "status": status,
        "source_sha": source_sha,
        "firstMissingSignal": error,
        "rollback": rollback,
        "rollbackFiles": rollback,
        "rollbackService": "not-attempted" if rollback == "not-needed" else rollback,
    }


def main(argv: list[str] | None = None) -> int:
    source_sha: str | None = None
    try:
        args = parse_args(argv)
        source_sha = source_head()
        if args.health_only:
            health_gate(args.health_url, source_sha, args.health_retries, args.health_delay, args.health_timeout)
            result = install_receipt(False, "diagnostic", source_sha=source_sha, error="health-only is nonconverged")
        elif args.build_only:
            root = Path(tempfile.mkdtemp(prefix="coronatio-build-"))
            try:
                snapshot_and_build(root, BINARY_DEST, source_sha, resolve_cargo(), "0" * 16)
            finally:
                shutil.rmtree(root, ignore_errors=True)
            result = install_receipt(False, "diagnostic", source_sha=source_sha, error="build-only is nonconverged")
        else:
            converge(runtime_root=RUNTIME_ROOT, binary_dest=BINARY_DEST, health_url=args.health_url, retries=args.health_retries, delay_seconds=args.health_delay, timeout_seconds=args.health_timeout)
            result = install_receipt(True, "converged", source_sha=source_sha)
        print(json.dumps(result, sort_keys=True))
        return 0 if result["ok"] else 1
    except (FileNotFoundError, RuntimeError, subprocess.CalledProcessError, OSError) as exc:
        print(json.dumps(install_receipt(False, "failed", source_sha=source_sha, error=str(exc), rollback="failed"), sort_keys=True))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
