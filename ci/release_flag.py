#!/usr/bin/env python3
"""Attach the deterministic estate release flag to a published release."""

import hashlib
import json
import os
import sys
import tomllib
import urllib.parse
from datetime import datetime, timedelta, timezone
from typing import NoReturn

try:
    from . import release_publish as publisher
except ImportError:  # pragma: no cover - exercised when run as a script
    import release_publish as publisher


FLAG_NAME = "release.flag"
SCHEMA = "estate.release-flag.v1"
COMPONENT = "coronatio"
SURFACE_PATH = "src/bands/shell/ux/face-surface.json"
SEAT_URL = (
    "https://git.home.arpa/HOMESERVERSLTD/caduceus/raw/branch/main/"
    "schema/estate.release-flag.v1.json"
)
REQUIRED_FLAG_KEYS = frozenset({
    "schema",
    "component",
    "source_sha",
    "flagged_at",
    "pipeline_url",
})
LINEAGE_KEYS = frozenset({
    "version",
    "surface_names",
    "surface_sha256",
    "derived_from",
    "bump",
})


def fail(message) -> NoReturn:
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


def canonical_json_bytes(payload):
    return (json.dumps(payload, indent=2) + "\n").encode("utf-8")


def surface_digest(names):
    return hashlib.sha256(("\n".join(names) + "\n").encode("utf-8")).hexdigest()


def flag_bytes(source_sha, digest, flagged_at, pipeline_url, lineage=None):
    payload = {
        "schema": SCHEMA,
        "component": COMPONENT,
        "source_sha": source_sha,
        "sha256": digest,
        "flagged_at": canonical_utc_timestamp(flagged_at),
        "pipeline_url": pipeline_url,
    }
    if lineage is not None:
        payload["lineage"] = lineage
    return canonical_json_bytes(payload)


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


def load_seat(token):
    status, raw = publisher.request("GET", SEAT_URL, token)
    if status != 200:
        fail(f"release flag schema seat returned HTTP {status}")
    seat = publisher.decode(raw, "release flag schema seat")
    if not isinstance(seat, dict) or seat.get("schema") != SCHEMA:
        fail("release flag schema seat has a foreign schema id")
    fields = seat.get("fields")
    required = seat.get("required")
    if not isinstance(fields, dict) or not isinstance(required, list):
        fail("release flag schema seat has an invalid field declaration")
    if not REQUIRED_FLAG_KEYS.issubset(required) or any(key not in fields for key in required):
        fail("release flag schema seat is missing the frozen kernel")
    lineage = fields.get("lineage")
    lineage_fields = lineage.get("fields") if isinstance(lineage, dict) else None
    if (
        not isinstance(lineage, dict)
        or lineage.get("type") != "object"
        or not isinstance(lineage_fields, dict)
        or not LINEAGE_KEYS.issubset(lineage_fields)
    ):
        fail("release flag schema seat has a desynchronized lineage declaration")
    return seat


def load_surface_names():
    try:
        with open(SURFACE_PATH, "rb") as surface_file:
            surface = json.load(surface_file, object_pairs_hook=_strict_json_object)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, ValueError) as exc:
        fail(f"cannot read Coronatio face surface: {exc}")
    if not isinstance(surface, dict) or surface.get("schema") != "coronatio.face-surface.v1":
        fail("Coronatio face surface has a foreign schema")
    names = []
    for field in ("primitives", "author_face"):
        values = surface.get(field)
        if not isinstance(values, list) or any(not isinstance(value, str) or not value for value in values):
            fail(f"Coronatio face surface {field} must be a list of nonempty names")
        names.extend(values)
    return sorted(set(names))


def commit_count(sha, token):
    commits_url = f"{publisher.API_ROOT}/repos/{publisher.OWNER}/{publisher.REPO}/commits"
    query = urllib.parse.urlencode({"sha": sha, "limit": 1, "page": 1})
    status, raw, headers = publisher.request(
        "GET", f"{commits_url}?{query}", token, return_headers=True,
    )
    if status != 200:
        fail(f"Coronatio commit count returned HTTP {status}")
    commits = publisher.decode(raw, "Coronatio commit count")
    if not isinstance(commits, list):
        fail("Coronatio commit list is not an array")
    total = {key.lower(): value for key, value in headers.items()}.get("x-total-count")
    if total is not None:
        try:
            count = int(total)
        except ValueError as exc:
            fail(f"Coronatio commit count header is invalid: {exc}")
        if count < 1:
            fail("Coronatio commit count header is not positive")
        return count

    count = 0
    page = 1
    while True:
        query = urllib.parse.urlencode({"sha": sha, "limit": 50, "page": page})
        status, raw = publisher.request("GET", f"{commits_url}?{query}", token)
        if status != 200:
            fail(f"Coronatio commit count page {page} returned HTTP {status}")
        batch = publisher.decode(raw, f"Coronatio commit count page {page}")
        if not isinstance(batch, list):
            fail("Coronatio commit list page is not an array")
        count += len(batch)
        if len(batch) < 50:
            if count < 1:
                fail("Coronatio commit history is empty")
            return count
        page += 1


def release_for_sha(sha, token):
    tag = publisher.release_tag(sha)
    tag_url = f"{publisher.RELEASES}/tags/{urllib.parse.quote(tag, safe='')}"
    status, raw = publisher.request("GET", tag_url, token)
    if status != 200:
        fail(f"GET release tag returned HTTP {status}")
    release = publisher.decode(raw, "existing release")
    if not isinstance(release, dict):
        fail("existing release response is not an object")
    publisher.verify_release_identity(release, sha)
    return tag_url, release


def _strict_json_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON object key: {key}")
        result[key] = value
    return result


def parse_flag(actual, description):
    try:
        payload = json.loads(actual, object_pairs_hook=_strict_json_object)
    except (UnicodeDecodeError, json.JSONDecodeError, TypeError, ValueError):
        fail(f"{description} is not valid JSON")
    if not isinstance(payload, dict):
        fail(f"{description} is not a JSON object")
    if not REQUIRED_FLAG_KEYS.issubset(payload):
        fail(f"{description} is missing a frozen kernel field")
    if payload.get("schema") != SCHEMA:
        fail(f"{description} has an invalid schema")
    if any(not isinstance(payload[key], str) or not payload[key] for key in REQUIRED_FLAG_KEYS):
        fail(f"{description} frozen kernel values must be nonempty strings")
    if actual != canonical_json_bytes(payload):
        fail(f"{description} has non-canonical contents")
    return payload


def parse_version(version, description):
    if not isinstance(version, str):
        fail(f"{description} lineage version is missing")
    parts = version.split(".")
    if len(parts) != 3 or any(not part.isdigit() or (len(part) > 1 and part.startswith("0")) for part in parts):
        fail(f"{description} lineage version is invalid")
    return tuple(int(part) for part in parts)


def parse_lineage(payload, description, optional=False):
    lineage = payload.get("lineage")
    if lineage is None and optional:
        return None
    if not isinstance(lineage, dict):
        fail(f"{description} lineage is missing or invalid")
    missing = LINEAGE_KEYS.difference(lineage)
    if missing:
        if optional:
            return None
        fail(f"{description} lineage is missing fields: {', '.join(sorted(missing))}")
    version = parse_version(lineage.get("version"), description)
    names = lineage.get("surface_names")
    if not isinstance(names, list) or any(not isinstance(name, str) or not name for name in names):
        fail(f"{description} lineage surface_names is invalid")
    if names != sorted(set(names)):
        fail(f"{description} lineage surface_names is not sorted unique")
    if lineage.get("surface_sha256") != surface_digest(names):
        fail(f"{description} lineage surface digest conflicts with its names")
    derived_from = lineage.get("derived_from")
    if derived_from is not None and (
        not isinstance(derived_from, str)
        or len(derived_from) != 40
        or any(character not in "0123456789abcdef" for character in derived_from)
    ):
        fail(f"{description} lineage derived_from is invalid")
    if lineage.get("bump") not in {"major", "minor", "patch", "initial"}:
        fail(f"{description} lineage bump is invalid")
    source_sha = payload.get("source_sha")
    if len(source_sha) != 40 or any(character not in "0123456789abcdef" for character in source_sha):
        fail(f"{description} source_sha is invalid")
    return {
        "version": version,
        "surface_names": names,
        "source_sha": source_sha,
    }


def previous_lineage(token, current_sha):
    page = 1
    while True:
        query = urllib.parse.urlencode({"limit": 50, "page": page})
        status, raw = publisher.request("GET", f"{publisher.RELEASES}?{query}", token)
        if status != 200:
            fail(f"GET release list returned HTTP {status}")
        releases = publisher.decode(raw, "release list")
        if not isinstance(releases, list):
            fail("release list response is not an array")
        for release in releases:
            if not isinstance(release, dict):
                fail("release list contains a non-object")
            if release.get("target_commitish") == current_sha:
                continue
            asset = publisher.assets_of(release).get(FLAG_NAME)
            if asset is None:
                continue
            actual = publisher.download(asset, token, FLAG_NAME)
            payload = parse_flag(actual, "previous release.flag")
            if payload.get("component") != COMPONENT:
                fail("previous release.flag has a conflicting component")
            return parse_lineage(payload, "previous release.flag", optional=True)
        if len(releases) < 50:
            return None
        page += 1


def derive_lineage(names, previous, revision_count):
    if previous is None:
        return {
            "version": "0.0.1",
            "surface_names": names,
            "surface_sha256": surface_digest(names),
            "derived_from": None,
            "bump": "initial",
        }

    previous_major, previous_minor, previous_patch = previous["version"]
    if revision_count <= previous_patch:
        fail(
            "Coronatio commit count does not move monotonically beyond the previous lineage patch "
            f"({revision_count} <= {previous_patch})"
        )
    previous_names = set(previous["surface_names"])
    current_names = set(names)
    if previous_names - current_names:
        bump = "major"
        major, minor = previous_major + 1, 0
    elif current_names - previous_names:
        bump = "minor"
        major, minor = previous_major, previous_minor + 1
    else:
        bump = "patch"
        major, minor = previous_major, previous_minor
    return {
        "version": f"{major}.{minor}.{revision_count}",
        "surface_names": names,
        "surface_sha256": surface_digest(names),
        "derived_from": previous["source_sha"],
        "bump": bump,
    }


def verify_existing_flag(actual, source_sha, digest, pipeline_url, lineage):
    payload = parse_flag(actual, "existing release.flag")
    if payload.get("component") != COMPONENT:
        fail("existing release.flag has an invalid component")
    if payload.get("source_sha") != source_sha:
        fail("existing release.flag has a conflicting source SHA")
    if payload.get("sha256") != digest:
        fail("existing release.flag has a conflicting digest")
    if payload.get("pipeline_url") != pipeline_url:
        fail("existing release.flag has a conflicting pipeline URL")
    actual_lineage = parse_lineage(payload, "existing release.flag")
    expected_lineage = {
        "version": parse_version(lineage["version"], "expected release.flag"),
        "surface_names": lineage["surface_names"],
        "source_sha": source_sha,
    }
    if actual_lineage != expected_lineage:
        fail("existing release.flag has conflicting lineage")
    if payload["lineage"].get("surface_sha256") != lineage["surface_sha256"]:
        fail("existing release.flag has a conflicting surface digest")
    if payload["lineage"].get("derived_from") != lineage["derived_from"]:
        fail("existing release.flag has conflicting lineage ancestry")
    if payload["lineage"].get("bump") != lineage["bump"]:
        fail("existing release.flag has a conflicting lineage bump")
    return actual


def flag_asset(release, token, source_sha, digest, pipeline_url, lineage):
    assets = publisher.assets_of(release)
    asset = assets.get(FLAG_NAME)
    if asset is None:
        return False
    actual = publisher.download(asset, token, FLAG_NAME)
    verify_existing_flag(actual, source_sha, digest, pipeline_url, lineage)
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


def receipt(status, sha, binary_name, sidecar_name, digest, pipeline_url, tag_url, lineage):
    print(json.dumps({
        "schema": "coronatio.release_flag.v1",
        "ok": True,
        "status": status,
        "project": publisher.PROJECT,
        "tag": publisher.release_tag(sha),
        "source_sha": sha,
        "assets": [binary_name, sidecar_name, FLAG_NAME],
        "sha256": digest,
        "pipeline_url": pipeline_url,
        "release_url": tag_url,
        "lineage": lineage,
    }, separators=(",", ":")))


def main():
    token = os.environ.get("FORGEJO_TOKEN", "")
    if not token:
        fail("FORGEJO_TOKEN is required")
    load_seat(token)
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

    names = load_surface_names()
    lineage = derive_lineage(names, previous_lineage(token, sha), commit_count(sha, token))
    if flag_asset(release, token, sha, digest, pipeline_url, lineage):
        receipt("no-op", sha, binary_name, sidecar_name, digest, pipeline_url, tag_url, lineage)
        return

    expected = flag_bytes(sha, digest, current_utc_timestamp(), pipeline_url, lineage)
    release_id = release.get("id")
    if not isinstance(release_id, int) or isinstance(release_id, bool):
        fail("release has no numeric id")
    upload_url = f"{publisher.RELEASES}/{release_id}/assets?{urllib.parse.urlencode({'name': FLAG_NAME})}"
    status, _raw = publisher.request(
        "POST", upload_url, token, expected, content_type="application/json",
    )
    if status == 409:
        reread_flag(tag_url, token, expected, "release race reread")
        receipt("no-op", sha, binary_name, sidecar_name, digest, pipeline_url, tag_url, lineage)
        return
    if status not in (200, 201):
        fail(f"upload of {FLAG_NAME} returned HTTP {status}")

    reread_flag(tag_url, token, expected, "release reread")
    receipt("flagged", sha, binary_name, sidecar_name, digest, pipeline_url, tag_url, lineage)


if __name__ == "__main__":
    main()
