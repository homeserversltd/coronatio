#!/usr/bin/env python3
"""Attach the deterministic estate release flag to a published release."""

import hashlib
import json
import os
import sys
import tomllib
import urllib.parse
from datetime import datetime, timedelta, timezone

try:
    from . import release_publish as publisher
except ImportError:  # pragma: no cover - exercised when run as a script
    import release_publish as publisher


FLAG_NAME = "release.flag"
SCHEMA = "estate.release-flag.v1"
COMPONENT = "coronatio"
FLAG_KEYS = frozenset({
    "schema",
    "component",
    "source_sha",
    "sha256",
    "flagged_at",
    "pipeline_url",
})


def fail(message):
    print(f"release_flag: {message}", file=sys.stderr)
    raise SystemExit(1)


def canonical_utc_timestamp(timestamp):
    if not isinstance(timestamp, str) or not timestamp:
        fail("flagged_at must be a nonempty ISO8601 timestamp")
    value = timestamp[:-1] + "+00:00" if timestamp.endswith("Z") else timestamp
    try:
        parsed = datetime.fromisoformat(value)
    except ValueError as exc:
        fail(f"flagged_at is not valid ISO8601: {exc}")
    if parsed.tzinfo is None or parsed.utcoffset() != timedelta(0):
        fail("flagged_at must be timezone-aware UTC")
    return parsed.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")


def current_utc_timestamp():
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def flag_bytes(source_sha, digest, flagged_at, pipeline_url):
    payload = {
        "schema": SCHEMA,
        "component": COMPONENT,
        "source_sha": source_sha,
        "sha256": digest,
        "flagged_at": canonical_utc_timestamp(flagged_at),
        "pipeline_url": pipeline_url,
    }
    return (json.dumps(payload, indent=2) + "\n").encode("utf-8")


def load_cargo():
    try:
        with open("Cargo.toml", "rb") as cargo_file:
            cargo = tomllib.load(cargo_file)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        fail(f"cannot read Cargo metadata: {exc}")

    package = cargo.get("package")
    if not isinstance(package, dict) or package.get("name") != publisher.REPO:
        fail(f"Cargo package name must be {publisher.REPO}")
    version = package.get("version")
    if not isinstance(version, str) or not version:
        fail("Cargo package version is missing")

    declarations = cargo.get("bin", [])
    if not isinstance(declarations, list):
        fail("Coronatio binary declaration is invalid")
    if declarations:
        if len(declarations) != 1 or not isinstance(declarations[0], dict):
            fail("Coronatio must declare at most one binary target")
        binary_decl = declarations[0].get("name")
    else:
        binary_decl = package.get("name")
    if not isinstance(binary_decl, str) or not binary_decl:
        fail("Cargo binary declaration is missing")
    return version, binary_decl


def release_for_sha(sha, token):
    tag_url = f"{publisher.RELEASES}/tags/{urllib.parse.quote(sha, safe='')}"
    status, raw = publisher.request("GET", tag_url, token)
    if status != 200:
        fail(f"GET release tag returned HTTP {status}")
    release = publisher.decode(raw, "existing release")
    if not isinstance(release, dict):
        fail("existing release response is not an object")
    return tag_url, release


def _strict_json_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON object key: {key}")
        result[key] = value
    return result


def verify_existing_flag(actual, source_sha, digest, pipeline_url):
    try:
        payload = json.loads(actual, object_pairs_hook=_strict_json_object)
    except (UnicodeDecodeError, json.JSONDecodeError, TypeError, ValueError):
        fail("existing release.flag is not valid JSON")
    if not isinstance(payload, dict):
        fail("existing release.flag is not a JSON object")
    if set(payload) != FLAG_KEYS:
        fail("existing release.flag has invalid keys")
    if any(not isinstance(value, str) for value in payload.values()):
        fail("existing release.flag values must be strings")
    if payload["schema"] != SCHEMA:
        fail("existing release.flag has an invalid schema")
    if payload["component"] != COMPONENT:
        fail("existing release.flag has an invalid component")
    if payload["source_sha"] != source_sha:
        fail("existing release.flag has a conflicting source SHA")
    if payload["sha256"] != digest:
        fail("existing release.flag has a conflicting digest")
    if payload["pipeline_url"] != pipeline_url:
        fail("existing release.flag has a conflicting pipeline URL")

    expected = flag_bytes(source_sha, digest, payload["flagged_at"], pipeline_url)
    if actual != expected:
        fail("existing release.flag has non-canonical contents")
    return expected


def flag_asset(release, token, source_sha, digest, pipeline_url):
    assets = publisher.assets_of(release)
    asset = assets.get(FLAG_NAME)
    if asset is None:
        return False
    actual = publisher.download(asset, token, FLAG_NAME)
    verify_existing_flag(actual, source_sha, digest, pipeline_url)
    return True


def reread_flag(tag_url, token, expected, description):
    status, raw = publisher.request("GET", tag_url, token)
    if status != 200:
        fail(f"{description} returned HTTP {status}")
    release = publisher.decode(raw, description)
    if not isinstance(release, dict):
        fail(f"{description} response is not an object")
    assets = publisher.assets_of(release)
    asset = assets.get(FLAG_NAME)
    if asset is None:
        fail(f"{description} has no {FLAG_NAME} asset")
    if publisher.download(asset, token, FLAG_NAME) != expected:
        fail(f"downloaded {FLAG_NAME} differs from the expected flag")
    return release


def receipt(status, sha, binary_name, sidecar_name, digest, pipeline_url, tag_url):
    print(json.dumps({
        "schema": "coronatio.release_flag.v1",
        "ok": True,
        "status": status,
        "project": publisher.PROJECT,
        "tag": sha,
        "source_sha": sha,
        "assets": [binary_name, sidecar_name, FLAG_NAME],
        "sha256": digest,
        "pipeline_url": pipeline_url,
        "release_url": tag_url,
    }, separators=(",", ":")))


def main():
    token = os.environ.get("FORGEJO_TOKEN", "")
    if not token:
        fail("FORGEJO_TOKEN is required")
    sha = os.environ.get("CI_COMMIT_SHA", "")
    if len(sha) != 40 or any(char not in "0123456789abcdef" for char in sha):
        fail("CI_COMMIT_SHA must be exactly 40 lowercase hexadecimal characters")
    pipeline_url = os.environ.get("CI_PIPELINE_URL", "")
    if not pipeline_url or not pipeline_url.strip():
        fail("CI_PIPELINE_URL is required")

    version, binary_decl = load_cargo()
    del version
    target_directory = os.environ.get("CARGO_TARGET_DIR", "target")
    tag, _name, binary_name, sidecar_name = publisher.release_identity(sha, binary_decl)
    binary_path = os.path.join(target_directory, "release", binary_decl)
    if not os.path.isfile(binary_path):
        fail(f"release binary does not exist: {binary_path}")
    try:
        with open(binary_path, "rb") as binary_file:
            binary = binary_file.read()
    except OSError as exc:
        fail(f"cannot read release binary {binary_path}: {exc}")
    digest = hashlib.sha256(binary).hexdigest()

    tag_url, release = release_for_sha(sha, token)
    existing_digest = publisher.verify_existing(release, token, binary_name, sidecar_name)
    if existing_digest != digest:
        fail("local release binary conflicts with its published digest")
    if flag_asset(release, token, sha, digest, pipeline_url):
        receipt("no-op", sha, binary_name, sidecar_name, digest, pipeline_url, tag_url)
        return

    expected = flag_bytes(sha, digest, current_utc_timestamp(), pipeline_url)

    release_id = release.get("id")
    if not isinstance(release_id, int) or isinstance(release_id, bool):
        fail("release has no numeric id")
    upload_url = f"{publisher.RELEASES}/{release_id}/assets?{urllib.parse.urlencode({'name': FLAG_NAME})}"
    status, _raw = publisher.request(
        "POST", upload_url, token, expected, content_type="application/json",
    )
    if status == 409:
        reread_flag(tag_url, token, expected, "release race reread")
        receipt("no-op", sha, binary_name, sidecar_name, digest, pipeline_url, tag_url)
        return
    if status not in (200, 201):
        fail(f"upload of {FLAG_NAME} returned HTTP {status}")

    reread_flag(tag_url, token, expected, "release reread")
    receipt("flagged", sha, binary_name, sidecar_name, digest, pipeline_url, tag_url)


if __name__ == "__main__":
    main()
