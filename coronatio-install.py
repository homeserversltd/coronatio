#!/usr/bin/env python3
"""Root-owned, fixed-root Coronatio converger.

Receipt contract: coronatio.install.receipt.v1; live contract: coronatio.health.v1.
The fixed source identity, source_tree_sha256, static_sha256, and binary_sha256
are attested in every converged receipt.
"""
from __future__ import annotations
import argparse, hashlib, json, os, shutil, stat, subprocess, sys, tempfile, time
import urllib.error, urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent
SOURCE_DEST = Path("/opt/coronatio/source")
BINARY_DEST = Path("/usr/local/bin/coronatio")
SERVICE_NAME = "coronatio.service"
HEALTH_URL = "http://127.0.0.1:3013/health"
MARKER = ".coronatio-installed-source-sha"
CARGO_FALLBACKS = (Path("/usr/local/bin/cargo"), Path("/opt/cargo/bin/cargo"))
HEX40 = set("0123456789abcdef")


def lower_hex(value: str, n: int) -> bool:
    return isinstance(value, str) and len(value) == n and all(c in "0123456789abcdef" for c in value)


def fixed_source_sha() -> str:
    status = subprocess.run(["git", "status", "--porcelain", "--untracked-files=all"], cwd=REPO_ROOT, text=True, capture_output=True, check=True)
    if status.stdout.strip(): raise RuntimeError("authoritative source tree is dirty")
    sha = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO_ROOT, text=True).strip()
    if not lower_hex(sha, 40): raise RuntimeError("authoritative git HEAD is not 40 lowercase hex")
    return sha


def tree_sha256(root: Path) -> str:
    root = root.resolve()
    h = hashlib.sha256()
    tracked = subprocess.check_output(["git", "ls-files", "-z"], cwd=REPO_ROOT).decode().split("\0")[:-1] if root == REPO_ROOT.resolve() else [p.relative_to(root).as_posix() for p in root.rglob("*")]
    for rel in sorted(tracked):
        path = root / rel
        st = path.lstat()
        if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode):
            raise RuntimeError(f"unsafe nonregular source artifact: {rel}")
        before = (st.st_dev, st.st_ino, st.st_size, st.st_mtime_ns)
        data = path.read_bytes()
        after = path.lstat()
        if before != (after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns): raise RuntimeError(f"source changed while hashing: {rel}")
        h.update(rel.encode() + b"\0" + f"{stat.S_IMODE(st.st_mode):04o}".encode() + b"\0" + hashlib.sha256(data).digest())
    return h.hexdigest()


def file_sha256(path: Path) -> str:
    st = path.lstat()
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode): raise RuntimeError(f"unsafe binary artifact: {path}")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def resolve_cargo() -> str:
    value = shutil.which("cargo")
    if value: return value
    for p in CARGO_FALLBACKS:
        if p.is_file() and os.access(p, os.X_OK): return str(p)
    raise RuntimeError("cargo not found")


def build_release(source_sha: str) -> Path:
    cargo = resolve_cargo(); env = os.environ | {"CORONATIO_SOURCE_SHA": source_sha, "CORONATIO_BUILD_SHA": source_sha}
    subprocess.run([cargo, "build", "--release"], cwd=REPO_ROOT, env=env, check=True)
    binary = REPO_ROOT / "target/release/coronatio"
    if not binary.is_file(): raise RuntimeError("release binary missing after build")
    return binary


def health_payload(url: str, timeout: float) -> dict:
    with urllib.request.urlopen(urllib.request.Request(url, headers={"Accept":"application/json"}), timeout=timeout) as response:
        if not 200 <= response.status < 300: raise RuntimeError(f"health HTTP {response.status}")
        value = json.loads(response.read(16384).decode())
    if not isinstance(value, dict): raise RuntimeError("health is not a JSON object")
    return value

def exact_health(url: str, source_sha: str, timeout: float) -> None:
    value = health_payload(url, timeout)
    if not (value.get("schema") == "coronatio.health.v1" and value.get("ok") is True and value.get("service") == "coronatio" and value.get("source_sha") == source_sha and value.get("build_sha") == source_sha):
        raise RuntimeError("health schema or exact source/build SHA differs")

def active() -> bool:
    return subprocess.run(["systemctl","is-active","--quiet",SERVICE_NAME], check=False).returncode == 0

def atomic_copy(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    fd, name = tempfile.mkstemp(prefix=f".{destination.name}.", dir=destination.parent); tmp=Path(name)
    try:
        with os.fdopen(fd, "wb") as out, source.open("rb") as inp: shutil.copyfileobj(inp,out); out.flush(); os.fsync(out.fileno())
        os.chmod(tmp,0o755); os.chown(tmp,0,0); os.replace(tmp,destination)
        dfd=os.open(destination.parent,os.O_DIRECTORY); os.fsync(dfd); os.close(dfd)
    finally: tmp.unlink(missing_ok=True)

def stage_source(source_sha: str) -> tuple[Path,str]:
    static = REPO_ROOT / "static"; static_hash=tree_sha256(static)
    stage=Path(tempfile.mkdtemp(prefix=".coronatio-source.",dir=SOURCE_DEST.parent))
    try:
        shutil.copytree(static,stage/"static",symlinks=True)
        if tree_sha256(stage/"static") != static_hash: raise RuntimeError("static staging hash differs")
        (stage/MARKER).write_text(source_sha+"\n",encoding="ascii"); os.chmod(stage/MARKER,0o644)
        return stage,static_hash
    except: shutil.rmtree(stage,ignore_errors=True); raise

def receipt(ok: bool, status: str, source_sha: str|None=None, source_tree_sha256: str|None=None, static_sha256: str|None=None, binary_sha256: str|None=None, **extra: object) -> dict:
    return {"schema":"coronatio.install.receipt.v1","ok":ok,"status":status,"source_sha":source_sha,"source_tree_sha256":source_tree_sha256,"static_sha256":static_sha256,"binary_sha256":binary_sha256,**extra}

def converge(args: argparse.Namespace) -> dict:
    sha=fixed_source_sha(); source_hash=tree_sha256(REPO_ROOT); binary=build_release(sha); binary_hash=file_sha256(binary)
    if args.build_only: return receipt(False,"diagnostic",sha,source_hash,tree_sha256(REPO_ROOT/"static"),binary_hash,firstMissingSignal="build-only is non-mutating")
    if args.health_only: exact_health(args.health_url,sha,args.health_timeout); return receipt(False,"diagnostic",sha,source_hash,None,None,firstMissingSignal="health-only is diagnostic")
    marker=SOURCE_DEST/MARKER
    try:
        installed_static=tree_sha256(SOURCE_DEST/"static") if marker.is_file() else None
        if marker.read_text().strip()==sha and installed_static==tree_sha256(REPO_ROOT/"static") and BINARY_DEST.is_file() and file_sha256(BINARY_DEST)==binary_hash and active():
            exact_health(args.health_url,sha,args.health_timeout); return receipt(True,"no-op",sha,source_hash,installed_static,binary_hash)
    except OSError: pass
    stage, static_hash=stage_source(sha); old_source=SOURCE_DEST.with_name(".coronatio-source.rollback"); old_binary=BINARY_DEST.with_name(".coronatio.rollback")
    swapped_source=swapped_binary=False
    try:
        if SOURCE_DEST.exists(): os.replace(SOURCE_DEST,old_source)
        os.replace(stage,SOURCE_DEST); swapped_source=True
        if BINARY_DEST.exists(): os.replace(BINARY_DEST,old_binary)
        atomic_copy(binary,BINARY_DEST); swapped_binary=True
        subprocess.run(["systemctl","restart",SERVICE_NAME],check=True)
        for _ in range(args.health_retries):
            try: exact_health(args.health_url,sha,args.health_timeout); break
            except Exception: time.sleep(args.health_delay)
        else: raise RuntimeError("health did not attest exact installed identity")
        shutil.rmtree(old_source,ignore_errors=True); old_binary.unlink(missing_ok=True)
        return receipt(True,"converged",sha,source_hash,static_hash,binary_hash)
    except Exception as exc:
        rollback=True
        try:
            if swapped_binary and old_binary.exists(): os.replace(old_binary,BINARY_DEST)
            if swapped_source and old_source.exists(): shutil.rmtree(SOURCE_DEST,ignore_errors=True); os.replace(old_source,SOURCE_DEST)
        except Exception: rollback=False
        raise RuntimeError(json.dumps(receipt(False,"failed",sha,source_hash,static_hash,binary_hash,firstMissingSignal=str(exc),rollback=rollback)))
    finally: shutil.rmtree(stage,ignore_errors=True)

def parse_args(argv=None):
    p=argparse.ArgumentParser(); p.add_argument("--build-only",action="store_true"); p.add_argument("--health-only",action="store_true"); p.add_argument("--health-url",default=HEALTH_URL); p.add_argument("--health-retries",type=int,default=30); p.add_argument("--health-delay",type=float,default=1); p.add_argument("--health-timeout",type=float,default=2); return p.parse_args(argv)
def main(argv=None):
    a=parse_args(argv)
    try: result=converge(a)
    except Exception as exc:
        try: result=json.loads(str(exc))
        except json.JSONDecodeError: result=receipt(False,"failed",firstMissingSignal=str(exc),rollback=False)
        print(json.dumps(result,sort_keys=True)); return 1
    print(json.dumps(result,sort_keys=True)); return 0 if result.get("ok") else 1
if __name__=="__main__": raise SystemExit(main())
