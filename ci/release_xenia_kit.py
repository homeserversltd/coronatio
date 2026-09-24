#!/usr/bin/env python3
"""Cut an immutable xenia-kit release from Coronatio's face cadence."""

import argparse
import base64
import hashlib
import json
import os
import re
import ssl
import sys
import urllib.error
import urllib.parse
import urllib.request
from datetime import datetime, timedelta, timezone
from typing import NoReturn

try:
    from . import release_publish as publisher
except ImportError:  # pragma: no cover - exercised when run as a script
    import release_publish as publisher


API_ROOT = "https://git.home.arpa/api/v1"
OWNER = "HOMESERVERSLTD"
CROWN_REPO = "coronatio"
KIT_REPO = "xenia-kit"
FLAG_NAME = "release.flag"
FLAG_SCHEMA = "estate.release-flag.v1"
KIT_COMPONENT = "xenia-kit"
KIT_UX_PATH = "src/ux.rs"
KIT_DECLARATION_PATH = "xenia-kit.release.json"
SEAT_URL = (
    "https://git.home.arpa/HOMESERVERSLTD/caduceus/raw/branch/main/"
    "schema/estate.release-flag.v1.json"
)
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
STRUCT_RE = re.compile(r"(?m)^pub struct ([A-Za-z_][A-Za-z0-9_]*)\b")
LINEAGE_KEYS = frozenset({
    "version",
    "surface_names",
    "surface_sha256",
    "derived_from",
    "bump",
})


def fail(message) -> NoReturn:
    print(f"release_xenia_kit: {message}", file=sys.stderr)
    raise SystemExit(1)


def canonical_json_bytes(payload):
    return (json.dumps(payload, indent=2) + "\n").encode("utf-8")


def canonical_timestamp(value):
    if not isinstance(value, str) or not value:
        fail("flagged_at must be a nonempty ISO8601 timestamp")
    parsed_value = value[:-1] + "+00:00" if value.endswith("Z") else value
    try:
        parsed = datetime.fromisoformat(parsed_value)
    except ValueError as exc:
        fail(f"flagged_at is not valid ISO8601: {exc}")
    if parsed.tzinfo is None or parsed.utcoffset() != timedelta(0):
        fail("flagged_at must be timezone-aware UTC")
    return parsed.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")


def now_utc():
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def valid_sha(value, description):
    if not isinstance(value, str) or SHA_RE.fullmatch(value) is None:
        fail(f"{description} must be exactly 40 lowercase hexadecimal characters")
    return value


def strict_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON object key: {key}")
        result[key] = value
    return result


def decode_json(raw, description):
    try:
        value = json.loads(raw, object_pairs_hook=strict_object)
    except (UnicodeDecodeError, json.JSONDecodeError, TypeError, ValueError) as exc:
        fail(f"{description} is invalid JSON: {exc}")
    return value


def parse_flag(raw, component, description, require_lineage=True):
    payload = decode_json(raw, description)
    if not isinstance(payload, dict):
        fail(f"{description} is not an object")
    for key in ("schema", "component", "source_sha", "flagged_at", "pipeline_url"):
        if not isinstance(payload.get(key), str) or not payload[key]:
            fail(f"{description} is missing frozen kernel field {key}")
    if payload["schema"] != FLAG_SCHEMA:
        fail(f"{description} has a foreign schema")
    if payload["component"] != component:
        fail(f"{description} has component {payload['component']!r}, expected {component!r}")
    valid_sha(payload["source_sha"], f"{description} source_sha")
    if raw != canonical_json_bytes(payload):
        fail(f"{description} is not canonical two-space JSON with one trailing LF")
    lineage = payload.get("lineage")
    if lineage is None and not require_lineage:
        return payload, None
    if not isinstance(lineage, dict):
        fail(f"{description} has no lineage object")
    if not LINEAGE_KEYS.issubset(lineage):
        fail(f"{description} lineage is incomplete")
    version = parse_version(lineage["version"], description)
    names = lineage["surface_names"]
    if not isinstance(names, list) or any(not isinstance(name, str) or not name for name in names):
        fail(f"{description} lineage surface_names is invalid")
    if names != sorted(set(names)):
        fail(f"{description} lineage surface_names is not sorted unique")
    if lineage["surface_sha256"] != surface_digest(names):
        fail(f"{description} lineage surface digest conflicts with its names")
    derived_from = lineage["derived_from"]
    if derived_from is not None:
        valid_sha(derived_from, f"{description} lineage derived_from")
    if lineage["bump"] not in {"initial", "major", "minor", "patch"}:
        fail(f"{description} lineage bump is invalid")
    return payload, {
        "version": version,
        "surface_names": names,
        "source_sha": payload["source_sha"],
    }


def parse_version(value, description):
    if not isinstance(value, str):
        fail(f"{description} lineage version is invalid")
    parts = value.split(".")
    if len(parts) != 3 or any(not part.isdigit() or (len(part) > 1 and part.startswith("0")) for part in parts):
        fail(f"{description} lineage version is invalid")
    return tuple(int(part) for part in parts)


def surface_digest(names):
    return hashlib.sha256(("\n".join(names) + "\n").encode("utf-8")).hexdigest()


def derive_lineage(names, previous, revision_count):
    if not isinstance(revision_count, int) or isinstance(revision_count, bool) or revision_count < 1:
        fail("kit commit count must be a positive integer")
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
            "kit commit count does not move monotonically beyond the previous lineage patch "
            f"({revision_count} <= {previous_patch})"
        )
    old_names = set(previous["surface_names"])
    new_names = set(names)
    if old_names - new_names:
        bump = "major"
        major, minor = previous_major + 1, 0
    elif new_names - old_names:
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


def public_builder_names(source):
    if not isinstance(source, str):
        fail("kit src/ux.rs content must be text")
    names = []
    for name in STRUCT_RE.findall(source):
        escaped = re.escape(name)
        renders = re.search(rf"\bimpl\s+Render\s+for\s+{escaped}\b", source)
        rendered_by_macro = re.search(rf"\brenderer!\(\s*{escaped}\s*,", source)
        if renders or rendered_by_macro:
            names.append(name)
    names = sorted(set(names))
    if not names:
        fail("kit src/ux.rs declares no public rendering builder structs")
    return names


def kit_release_body(crown_sha, requires_face):
    return canonical_json_bytes({
        "crown_release_sha": valid_sha(crown_sha, "crown release SHA"),
        "requires_face": requires_face,
    })


def kit_flag_bytes(kit_sha, body_digest, flagged_at, pipeline_url, lineage):
    return canonical_json_bytes({
        "schema": FLAG_SCHEMA,
        "component": KIT_COMPONENT,
        "source_sha": valid_sha(kit_sha, "kit main SHA"),
        "sha256": body_digest,
        "flagged_at": canonical_timestamp(flagged_at),
        "pipeline_url": pipeline_url,
        "lineage": lineage,
    })


class Forgejo:
    def __init__(self, token):
        self.token = token

    def request(self, method, url, body=None, content_type=None, accept=None):
        headers = {
            "Authorization": f"token {self.token}",
            "User-Agent": "coronatio-xenia-kit-release",
        }
        if content_type:
            headers["Content-Type"] = content_type
        if accept:
            headers["Accept"] = accept
        if isinstance(body, (dict, list)):
            body = json.dumps(body, separators=(",", ":")).encode("utf-8")
            headers["Content-Type"] = "application/json"
        request = urllib.request.Request(url, data=body, headers=headers, method=method)
        try:
            with urllib.request.urlopen(request, timeout=180) as response:
                return response.status, response.read(), dict(response.headers.items())
        except urllib.error.HTTPError as exc:
            return exc.code, exc.read(), dict(exc.headers.items())
        except (urllib.error.URLError, TimeoutError, OSError) as exc:
            fail(f"{method} {url} transport failure: {exc}")

    def get_json(self, url, description):
        status, raw, headers = self.request("GET", url)
        if status != 200:
            fail(f"{description} returned HTTP {status}")
        return decode_json(raw, description), headers

    def download(self, asset, description):
        if not isinstance(asset, dict) or not isinstance(asset.get("browser_download_url"), str):
            fail(f"{description} has no browser download URL")
        status, raw, _headers = self.request(
            "GET", asset["browser_download_url"], accept="application/octet-stream",
        )
        if status != 200:
            fail(f"{description} download returned HTTP {status}")
        return raw


def releases_url(repo):
    return f"{API_ROOT}/repos/{OWNER}/{repo}/releases"


def tag_url(repo, sha):
    tag = publisher.release_tag(sha)
    return f"{releases_url(repo)}/tags/{urllib.parse.quote(tag, safe='')}"


def assets_of(release, description):
    if not isinstance(release, dict) or not isinstance(release.get("assets"), list):
        fail(f"{description} has no asset list")
    assets = {}
    for asset in release["assets"]:
        name = asset.get("name") if isinstance(asset, dict) else None
        if not isinstance(name, str) or not name or name in assets:
            fail(f"{description} has an invalid or duplicate asset name")
        assets[name] = asset
    return assets


def validate_seat(seat):
    if not isinstance(seat, dict) or seat.get("schema") != FLAG_SCHEMA:
        fail("release flag schema seat has a foreign schema id")
    fields = seat.get("fields")
    required = seat.get("required")
    frozen = {"schema", "component", "source_sha", "flagged_at", "pipeline_url"}
    if not isinstance(fields, dict) or not isinstance(required, list):
        fail("release flag schema seat has an invalid field declaration")
    if not frozen.issubset(required) or any(key not in fields for key in required):
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


def list_releases(client, repo):
    result = []
    page = 1
    while True:
        query = urllib.parse.urlencode({"limit": 50, "page": page})
        batch, _headers = client.get_json(f"{releases_url(repo)}?{query}", f"{repo} release list")
        if not isinstance(batch, list):
            fail(f"{repo} release list is not an array")
        result.extend(batch)
        if len(batch) < 50:
            return result
        page += 1


def flag_from_release(client, release, component, description, require_lineage=True):
    asset = assets_of(release, description).get(FLAG_NAME)
    if asset is None:
        return None, None
    return parse_flag(client.download(asset, f"{description} {FLAG_NAME}"), component, description, require_lineage)


def previous_lineage(client, releases, current_sha, component, description):
    for release in releases:
        if not isinstance(release, dict):
            fail(f"{description} release list contains a non-object")
        if release.get("target_commitish") == current_sha:
            continue
        payload, lineage = flag_from_release(
            client, release, component, f"previous {description} release.flag", require_lineage=False,
        )
        if payload is None:
            continue
        return lineage
    return None


def content_at(client, repo, path, sha):
    quoted_path = urllib.parse.quote(path, safe="/")
    query = urllib.parse.urlencode({"ref": sha})
    value, _headers = client.get_json(
        f"{API_ROOT}/repos/{OWNER}/{repo}/contents/{quoted_path}?{query}",
        f"{repo} {path} at {sha}",
    )
    if not isinstance(value, dict) or value.get("encoding") != "base64" or not isinstance(value.get("content"), str):
        fail(f"{repo} {path} content response is not base64 file content")
    try:
        return base64.b64decode(value["content"], validate=True)
    except (ValueError, TypeError) as exc:
        fail(f"{repo} {path} content is invalid base64: {exc}")


def kit_main_sha(client):
    branch, _headers = client.get_json(
        f"{API_ROOT}/repos/{OWNER}/{KIT_REPO}/branches/main", "xenia-kit main branch",
    )
    if not isinstance(branch, dict) or not isinstance(branch.get("commit"), dict):
        fail("xenia-kit main branch response has no commit")
    return valid_sha(branch["commit"].get("id"), "xenia-kit main head")


def kit_commit_count(client, sha):
    query = urllib.parse.urlencode({"sha": sha, "limit": 1, "page": 1})
    commits, headers = client.get_json(
        f"{API_ROOT}/repos/{OWNER}/{KIT_REPO}/commits?{query}", "xenia-kit commit count",
    )
    if not isinstance(commits, list):
        fail("xenia-kit commit list is not an array")
    lowered = {key.lower(): value for key, value in headers.items()}
    total = lowered.get("x-total-count") or lowered.get("x-total")
    if total is not None:
        try:
            count = int(total)
        except ValueError as exc:
            fail(f"xenia-kit commit count header is invalid: {exc}")
        if count < 1:
            fail("xenia-kit commit count header is not positive")
        return count

    count = 0
    page = 1
    while True:
        query = urllib.parse.urlencode({"sha": sha, "limit": 50, "page": page})
        batch, _headers = client.get_json(
            f"{API_ROOT}/repos/{OWNER}/{KIT_REPO}/commits?{query}",
            f"xenia-kit commit count page {page}",
        )
        if not isinstance(batch, list):
            fail("xenia-kit commit list page is not an array")
        count += len(batch)
        if len(batch) < 50:
            if count < 1:
                fail("xenia-kit commit history is empty")
            return count
        page += 1


def crown_surface_moved(current_lineage, previous):
    if previous is None:
        return True
    return current_lineage["surface_names"] != previous["surface_names"]


def read_fixture(path):
    try:
        with open(path, "rb") as source:
            fixture = json.load(source, object_pairs_hook=strict_object)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, ValueError) as exc:
        fail(f"cannot read dry-run fixture: {exc}")
    if not isinstance(fixture, dict):
        fail("dry-run fixture must be an object")
    return fixture


def fixture_flag(value, component, description, require_lineage=True):
    if value is None:
        return None, None
    if not isinstance(value, dict):
        fail(f"dry-run fixture {description} must be an object or null")
    return parse_flag(canonical_json_bytes(value), component, description, require_lineage)


def release_plan(kit_sha, body, flag, release_id=None):
    create_payload = {
        "tag_name": publisher.release_tag(kit_sha),
        "name": f"xenia-kit {kit_sha[:8]}",
        "target_commitish": kit_sha,
        "body": body.decode("utf-8"),
        "draft": False,
        "prerelease": False,
    }
    identity = release_id if release_id is not None else "{release_id_from_create_response}"
    upload_url = f"{releases_url(KIT_REPO)}/{identity}/assets?{urllib.parse.urlencode({'name': FLAG_NAME})}"
    return [
        {"method": "POST", "url": releases_url(KIT_REPO), "json": create_payload},
        {
            "method": "POST",
            "url": upload_url,
            "content_type": "application/json",
            "body_utf8": flag.decode("utf-8"),
            "depends_on": "request[0].response.id",
        },
        {"method": "GET", "url": tag_url(KIT_REPO, kit_sha)},
        {
            "method": "GET",
            "url": "{browser_download_url_from_release.flag_asset}",
            "accept": "application/octet-stream",
            "depends_on": "request[2].response.assets[name=release.flag]",
        },
    ]


def verify_existing_release(client, release, kit_sha, body, expected_flag, fixture_flag_payload=None):
    expected_name = f"xenia-kit {kit_sha[:8]}"
    for field, expected in (
        ("tag_name", publisher.release_tag(kit_sha)),
        ("target_commitish", kit_sha),
        ("name", expected_name),
        ("body", body.decode("utf-8")),
    ):
        if release.get(field) != expected:
            fail(f"existing xenia-kit release has conflicting {field}")
    assets = assets_of(release, "existing xenia-kit release")
    if set(assets) != {FLAG_NAME}:
        fail("existing xenia-kit release has assets other than the one release.flag")
    if fixture_flag_payload is None:
        actual_flag = client.download(assets[FLAG_NAME], "existing xenia-kit release.flag")
    else:
        actual_flag = canonical_json_bytes(fixture_flag_payload)
    payload, _lineage = parse_flag(actual_flag, KIT_COMPONENT, "existing xenia-kit release.flag")
    if payload.get("sha256") != hashlib.sha256(body).hexdigest():
        fail("existing xenia-kit release.flag digest conflicts with the canonical release body")
    expected_payload = decode_json(expected_flag, "expected xenia-kit release.flag")
    for key, expected in expected_payload.items():
        if key == "lineage":
            continue
        if payload.get(key) != expected:
            fail(f"existing xenia-kit release.flag has conflicting {key}")
    actual_lineage = payload.get("lineage")
    expected_lineage = expected_payload.get("lineage")
    if not isinstance(actual_lineage, dict) or not isinstance(expected_lineage, dict):
        fail("existing xenia-kit release.flag lineage is invalid")
    for key, expected in expected_lineage.items():
        if actual_lineage.get(key) != expected:
            fail(f"existing xenia-kit release.flag lineage has conflicting {key}")


def dry_run(args, crown_sha, pipeline_url):
    fixture = read_fixture(args.fixture)
    validate_seat(fixture.get("release_flag_seat"))
    current_payload, current_lineage = fixture_flag(
        fixture.get("current_crown_flag"), CROWN_REPO, "current crown release.flag",
    )
    if current_payload["source_sha"] != crown_sha:
        fail("dry-run current crown release.flag source_sha conflicts with CI_COMMIT_SHA")
    _previous_payload, prior_crown = fixture_flag(
        fixture.get("previous_crown_flag"), CROWN_REPO, "previous crown release.flag",
        require_lineage=False,
    )
    moved = crown_surface_moved(current_lineage, prior_crown)
    has_prior = fixture.get("has_prior_kit_release")
    if not isinstance(has_prior, bool):
        fail("dry-run fixture has_prior_kit_release must be boolean")
    if not moved and has_prior:
        print(json.dumps({
            "status": "skipped",
            "dry_run": True,
            "crown_surface_moved": False,
            "has_prior_kit_release": True,
            "requests": [],
        }, separators=(",", ":")))
        return

    kit_sha = valid_sha(fixture.get("kit_sha"), "dry-run kit_sha")
    revision_count = fixture.get("kit_commit_count")
    ux_source = fixture.get("kit_ux")
    declaration = fixture.get("kit_release_declaration")
    if not isinstance(declaration, dict) or "requires_face" not in declaration:
        fail("dry-run kit_release_declaration must contain requires_face")
    _previous_kit_payload, prior_kit = fixture_flag(
        fixture.get("previous_kit_flag"), KIT_COMPONENT, "previous kit release.flag",
        require_lineage=False,
    )
    names = public_builder_names(ux_source)
    lineage = derive_lineage(names, prior_kit, revision_count)
    body = kit_release_body(crown_sha, declaration["requires_face"])
    digest = hashlib.sha256(body).hexdigest()
    flagged_at = args.flagged_at or now_utc()

    existing = fixture.get("existing_kit_release")
    if existing is not None:
        if not isinstance(existing, dict) or not isinstance(existing.get("release"), dict):
            fail("dry-run existing_kit_release must contain a release object")
        existing_flag = existing.get("release_flag")
        if not isinstance(existing_flag, dict):
            fail("dry-run existing_kit_release must contain release_flag")
        existing_timestamp = existing_flag.get("flagged_at")
        expected_flag = kit_flag_bytes(kit_sha, digest, existing_timestamp, pipeline_url, lineage)
        verify_existing_release(None, existing["release"], kit_sha, body, expected_flag, existing_flag)
        print(json.dumps({
            "status": "no-op",
            "dry_run": True,
            "crown_surface_moved": moved,
            "has_prior_kit_release": has_prior,
            "kit_sha": kit_sha,
            "lineage": lineage,
            "requests": [],
        }, separators=(",", ":")))
        return

    flag = kit_flag_bytes(kit_sha, digest, flagged_at, pipeline_url, lineage)
    print(json.dumps({
        "status": "planned",
        "dry_run": True,
        "crown_surface_moved": moved,
        "has_prior_kit_release": has_prior,
        "kit_sha": kit_sha,
        "body_sha256": digest,
        "lineage": lineage,
        "requests": release_plan(kit_sha, body, flag),
    }, separators=(",", ":")))


def live_run(crown_sha, pipeline_url, token):
    client = Forgejo(token)
    seat, _headers = client.get_json(SEAT_URL, "release flag schema seat")
    validate_seat(seat)
    current_release, _headers = client.get_json(tag_url(CROWN_REPO, crown_sha), "current crown release")
    publisher.verify_release_identity(current_release, crown_sha)
    current_payload, current_lineage = flag_from_release(
        client, current_release, CROWN_REPO, "current crown release.flag",
    )
    if current_payload is None or current_lineage is None:
        fail("current crown release has no lineage-bearing release.flag")
    if current_payload["source_sha"] != crown_sha:
        fail("current crown release.flag source_sha conflicts with CI_COMMIT_SHA")

    crown_releases = list_releases(client, CROWN_REPO)
    prior_crown = previous_lineage(client, crown_releases, crown_sha, CROWN_REPO, "crown")
    moved = crown_surface_moved(current_lineage, prior_crown)

    kit_releases = list_releases(client, KIT_REPO)
    has_prior = bool(kit_releases)
    if not moved and has_prior:
        print(json.dumps({
            "status": "skipped",
            "dry_run": False,
            "crown_surface_moved": False,
            "has_prior_kit_release": True,
            "requests": [],
        }, separators=(",", ":")))
        return

    kit_sha = kit_main_sha(client)
    ux_raw = content_at(client, KIT_REPO, KIT_UX_PATH, kit_sha)
    declaration_raw = content_at(client, KIT_REPO, KIT_DECLARATION_PATH, kit_sha)
    try:
        ux_source = ux_raw.decode("utf-8")
    except UnicodeDecodeError as exc:
        fail(f"xenia-kit {KIT_UX_PATH} is not UTF-8: {exc}")
    declaration = decode_json(declaration_raw, f"xenia-kit {KIT_DECLARATION_PATH}")
    if not isinstance(declaration, dict) or "requires_face" not in declaration:
        fail(f"xenia-kit {KIT_DECLARATION_PATH} does not declare requires_face")

    prior_kit = previous_lineage(client, kit_releases, kit_sha, KIT_COMPONENT, "xenia-kit")
    names = public_builder_names(ux_source)
    lineage = derive_lineage(names, prior_kit, kit_commit_count(client, kit_sha))
    body = kit_release_body(crown_sha, declaration["requires_face"])
    body_digest = hashlib.sha256(body).hexdigest()

    current_url = tag_url(KIT_REPO, kit_sha)
    status, raw, _headers = client.request("GET", current_url)
    if status == 200:
        release = decode_json(raw, "existing xenia-kit release")
        assets = assets_of(release, "existing xenia-kit release")
        existing_raw = client.download(assets.get(FLAG_NAME), "existing xenia-kit release.flag")
        existing_payload, _existing_lineage = parse_flag(
            existing_raw, KIT_COMPONENT, "existing xenia-kit release.flag",
        )
        expected_flag = kit_flag_bytes(
            kit_sha, body_digest, existing_payload["flagged_at"], pipeline_url, lineage,
        )
        verify_existing_release(client, release, kit_sha, body, expected_flag)
        print(json.dumps({
            "status": "no-op",
            "dry_run": False,
            "crown_surface_moved": moved,
            "has_prior_kit_release": has_prior,
            "kit_sha": kit_sha,
            "lineage": lineage,
            "requests": [],
        }, separators=(",", ":")))
        return
    if status != 404:
        fail(f"GET xenia-kit release tag returned HTTP {status}")

    flagged_at = now_utc()
    flag = kit_flag_bytes(kit_sha, body_digest, flagged_at, pipeline_url, lineage)
    create_payload = {
        "tag_name": publisher.release_tag(kit_sha),
        "name": f"xenia-kit {kit_sha[:8]}",
        "target_commitish": kit_sha,
        "body": body.decode("utf-8"),
        "draft": False,
        "prerelease": False,
    }
    status, raw, _headers = client.request("POST", releases_url(KIT_REPO), create_payload)
    if status == 409:
        release, _headers = client.get_json(current_url, "xenia-kit release collision reread")
        assets = assets_of(release, "xenia-kit release collision reread")
        existing_raw = client.download(assets.get(FLAG_NAME), "xenia-kit collision release.flag")
        existing_payload, _lineage = parse_flag(
            existing_raw, KIT_COMPONENT, "xenia-kit collision release.flag",
        )
        expected_flag = kit_flag_bytes(
            kit_sha, body_digest, existing_payload["flagged_at"], pipeline_url, lineage,
        )
        verify_existing_release(client, release, kit_sha, body, expected_flag)
        print(json.dumps({
            "status": "no-op",
            "dry_run": False,
            "crown_surface_moved": moved,
            "has_prior_kit_release": has_prior,
            "kit_sha": kit_sha,
            "lineage": lineage,
            "requests": [],
        }, separators=(",", ":")))
        return
    if status not in (200, 201):
        fail(f"xenia-kit release creation returned HTTP {status}")
    release = decode_json(raw, "xenia-kit release creation")
    release_id = release.get("id") if isinstance(release, dict) else None
    if not isinstance(release_id, int) or isinstance(release_id, bool):
        fail("created xenia-kit release has no numeric id")
    if assets_of(release, "created xenia-kit release"):
        fail("created xenia-kit release unexpectedly has assets")

    upload_url = (
        f"{releases_url(KIT_REPO)}/{release_id}/assets?"
        + urllib.parse.urlencode({"name": FLAG_NAME})
    )
    status, _raw, _headers = client.request(
        "POST", upload_url, flag, content_type="application/json",
    )
    if status == 409:
        collided, _headers = client.get_json(current_url, "xenia-kit flag collision reread")
        verify_existing_release(client, collided, kit_sha, body, flag)
        print(json.dumps({
            "status": "no-op",
            "dry_run": False,
            "crown_surface_moved": moved,
            "has_prior_kit_release": has_prior,
            "kit_sha": kit_sha,
            "lineage": lineage,
            "requests": [],
        }, separators=(",", ":")))
        return
    if status not in (200, 201):
        fail(f"xenia-kit release.flag upload returned HTTP {status}")

    published, _headers = client.get_json(current_url, "published xenia-kit release reread")
    verify_existing_release(client, published, kit_sha, body, flag)
    print(json.dumps({
        "status": "published",
        "dry_run": False,
        "crown_surface_moved": moved,
        "has_prior_kit_release": has_prior,
        "kit_sha": kit_sha,
        "body_sha256": body_digest,
        "lineage": lineage,
        "requests": release_plan(kit_sha, body, flag, release_id=release_id),
    }, separators=(",", ":")))


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="emit the exact planned request set without mutation")
    parser.add_argument("--fixture", help="offline dry-run input JSON for moved/unmoved proof")
    parser.add_argument("--flagged-at", help="deterministic dry-run release.flag timestamp")
    args = parser.parse_args()
    if args.fixture and not args.dry_run:
        parser.error("--fixture requires --dry-run")
    if args.dry_run and not args.fixture:
        parser.error("--dry-run requires --fixture so it cannot depend on network state")
    if args.flagged_at and not args.dry_run:
        parser.error("--flagged-at is dry-run only")
    return args


def main():
    args = parse_args()
    crown_sha = valid_sha(os.environ.get("CI_COMMIT_SHA", ""), "CI_COMMIT_SHA")
    pipeline_url = os.environ.get("CI_PIPELINE_URL", "")
    if not pipeline_url or not pipeline_url.strip():
        fail("CI_PIPELINE_URL is required")
    if args.dry_run:
        dry_run(args, crown_sha, pipeline_url)
        return
    token = os.environ.get("FORGEJO_TOKEN", "")
    if not token:
        fail("FORGEJO_TOKEN is required")
    live_run(crown_sha, pipeline_url, token)


if __name__ == "__main__":
    main()
