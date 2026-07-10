#!/usr/bin/env python3
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent
BINARY_SOURCE = REPO_ROOT / "target" / "release" / "coronatio"
BINARY_DEST = Path("/usr/local/bin/coronatio")
SERVICE_NAME = "coronatio.service"
HEALTH_URL = "http://127.0.0.1:3013/health"
CARGO_FALLBACKS = (Path("/usr/local/bin/cargo"), Path("/opt/cargo/bin/cargo"))


def run(cmd: list[str], *, cwd: Path = REPO_ROOT) -> None:
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, check=True, cwd=cwd)


def fail(message: str) -> int:
    print(message, file=sys.stderr)
    return 1


def resolve_cargo() -> str:
    path_cargo = shutil.which("cargo")
    if path_cargo:
        return path_cargo
    for candidate in CARGO_FALLBACKS:
        if candidate.exists() and os.access(candidate, os.X_OK):
            return str(candidate)
    searched = ["PATH"] + [str(path) for path in CARGO_FALLBACKS]
    raise FileNotFoundError(f"cargo not found; searched {', '.join(searched)}")


def build_release(cargo: str) -> None:
    run([cargo, "build", "--release"])
    if not BINARY_SOURCE.is_file():
        raise FileNotFoundError(f"release binary missing after build: {BINARY_SOURCE}")


def assert_service_exists() -> None:
    try:
        run(["systemctl", "cat", SERVICE_NAME])
    except (FileNotFoundError, subprocess.CalledProcessError) as exc:
        raise RuntimeError(f"{SERVICE_NAME} is absent or unreadable; unit birth belongs to the field/deployables surface") from exc


def install_binary() -> None:
    staging_path = BINARY_DEST.with_name(f".{BINARY_DEST.name}.new")
    run(["install", "-o", "root", "-g", "root", "-m", "755", str(BINARY_SOURCE), str(staging_path)])
    run(["mv", "-f", str(staging_path), str(BINARY_DEST)])


def restart_service() -> None:
    run(["systemctl", "restart", SERVICE_NAME])


def probe_health(url: str, timeout_seconds: float) -> tuple[bool, str]:
    request = urllib.request.Request(url, headers={"Accept": "application/json,text/plain,*/*"})
    try:
        with urllib.request.urlopen(request, timeout=timeout_seconds) as response:
            body = response.read(4096).decode("utf-8", errors="replace")
            status = getattr(response, "status", response.getcode())
            if 200 <= status < 300:
                return True, f"HTTP {status}: {body[:200]}"
            return False, f"HTTP {status}: {body[:200]}"
    except (urllib.error.URLError, TimeoutError, OSError) as exc:
        return False, str(exc)


def health_gate(url: str, retries: int, delay_seconds: float, timeout_seconds: float) -> None:
    last_error = "not attempted"
    for attempt in range(1, retries + 1):
        ok, detail = probe_health(url, timeout_seconds)
        if ok:
            print(f"health gate passed on attempt {attempt}: {detail}")
            return
        last_error = detail
        print(f"health gate attempt {attempt}/{retries} failed: {detail}", file=sys.stderr)
        if attempt < retries:
            time.sleep(delay_seconds)
    raise RuntimeError(f"health gate failed for {url}: {last_error}")


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build and converge the Coronatio service binary.")
    parser.add_argument("--build-only", action="store_true", help="Run cargo build --release only; do not mutate the host")
    parser.add_argument("--health-only", action="store_true", help="Only run the health gate; intended for diagnostics and failure tests")
    parser.add_argument("--health-url", default=HEALTH_URL, help="Health endpoint to probe after restart")
    parser.add_argument("--health-retries", type=int, default=30, help="Number of health probe attempts")
    parser.add_argument("--health-delay", type=float, default=1.0, help="Seconds between health probe attempts")
    parser.add_argument("--health-timeout", type=float, default=2.0, help="Per-probe timeout in seconds")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    if args.health_retries < 1:
        return fail("--health-retries must be >= 1")

    try:
        if args.health_only:
            health_gate(args.health_url, args.health_retries, args.health_delay, args.health_timeout)
            return 0

        cargo = resolve_cargo()
        build_release(cargo)
        if args.build_only:
            print("Build-only mode complete.")
            return 0

        assert_service_exists()
        install_binary()
        restart_service()
        health_gate(args.health_url, args.health_retries, args.health_delay, args.health_timeout)
        print("Coronatio converge complete.")
        return 0
    except (FileNotFoundError, RuntimeError, subprocess.CalledProcessError) as exc:
        return fail(str(exc))


if __name__ == "__main__":
    raise SystemExit(main())
