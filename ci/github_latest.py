#!/usr/bin/env python3
"""Mirror Coronatio's Forgejo SHA release onto GitHub's single `latest` release."""

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import time
import tomllib
import urllib.error
import urllib.parse
import urllib.request
from typing import NoReturn

FORGEJO_API = "https://git.home.arpa/api/v1"
GITHUB_API = "https://api.github.com"
OWNER = "HOMESERVERSLTD"
REPO = "coronatio"
PROJECT = f"{OWNER}/{REPO}"
FLAG_NAME = "release.flag"
FLAG_SCHEMA = "estate.release-flag.v1"
FORGEJO_ORIGIN = ("https", "git.home.arpa", 443)
GITHUB_API_ORIGIN = ("https", "api.github.com", 443)
GITHUB_UPLOAD_ORIGIN = ("https", "uploads.github.com", 443)
FORGEJO_GIT_URL = "https://git.home.arpa/HOMESERVERSLTD/coronatio.git"
MAX_REDIRECTS = 5
MIRROR_MAX_ATTEMPTS = 6
MIRROR_BACKOFF_SECONDS = (2, 5, 10, 20, 30)


class PublishError(Exception):
    """A safe-to-report failure that never contains request credentials."""


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


def fail(message) -> NoReturn:
    raise PublishError(message)


def safe_text(value, secrets=()):
    text = str(value).replace("\r", " ").replace("\n", " ")
    for secret in secrets:
        if secret:
            text = text.replace(secret, "[REDACTED]")
    return text[:500]


def origin(url):
    parsed = urllib.parse.urlsplit(url)
    if parsed.scheme.lower() != "https" or not parsed.hostname:
        fail("refusing a non-HTTPS or malformed URL")
    return parsed.scheme.lower(), parsed.hostname.lower(), parsed.port or 443


def http_request(method, url, *, token=None, auth_origins=(), body=None, headers=None,
                 secrets=(), max_bytes=None):
    """Make a verified-TLS request, retaining credentials only at the exact origin."""
    if PLAN_MODE and method != "GET":
        fail("plan mode refused a non-GET request")
    request_headers = {
        "Accept": "application/json",
        "User-Agent": "coronatio-github-latest-publisher",
    }
    if headers:
        request_headers.update(headers)
    auth_origins = set(auth_origins)
    current_url = url
    original_origin = origin(url)
    for redirect_count in range(MAX_REDIRECTS + 1):
        current_origin = origin(current_url)
        current_headers = dict(request_headers)
        if token and current_origin == original_origin and current_origin in auth_origins:
            current_headers["Authorization"] = token
        request = urllib.request.Request(
            current_url, data=body, headers=current_headers, method=method,
        )
        opener = urllib.request.build_opener(NoRedirect())
        try:
            with opener.open(request, timeout=60) as response:
                status = response.status
                response_headers = dict(response.headers.items())
                raw = response.read(max_bytes + 1 if max_bytes is not None else -1)
        except urllib.error.HTTPError as exc:
            status = exc.code
            response_headers = dict(exc.headers.items()) if exc.headers else {}
            if 300 <= status < 400:
                location = response_headers.get("Location") or response_headers.get("location")
                if not location:
                    return status, b"", response_headers
                if redirect_count == MAX_REDIRECTS:
                    fail(f"{method} {current_url} exceeded the redirect limit")
                next_url = urllib.parse.urljoin(current_url, location)
                origin(next_url)
                current_url = next_url
                continue
            raw = exc.read(max_bytes + 1 if max_bytes is not None else -1)
        except (urllib.error.URLError, TimeoutError, OSError) as exc:
            reason = getattr(exc, "reason", exc)
            detail = safe_text(reason, secrets)
            fail(f"{method} {current_url} transport failure ({type(exc).__name__}: {detail})")
        if max_bytes is not None and len(raw) > max_bytes:
            fail(f"{method} {current_url} response exceeds the allowed size")
        return status, raw, response_headers
    fail(f"{method} {url} exceeded the redirect limit")


def api_request(method, url, *, token=None, service="forgejo", payload=None):
    if payload is None:
        body = None
        headers = {}
    else:
        body = json.dumps(payload, separators=(",", ":")).encode("utf-8")
        headers = {"Content-Type": "application/json"}
    if service == "forgejo":
        auth_origins = {FORGEJO_ORIGIN}
        token_value = f"token {token}" if token else None
    elif service == "github":
        auth_origins = {GITHUB_API_ORIGIN}
        token_value = f"Bearer {token}" if token else None
        headers["X-GitHub-Api-Version"] = "2022-11-28"
    else:
        fail("unknown API service")
    return http_request(
        method, url, token=token_value, auth_origins=auth_origins, body=body,
        headers=headers, secrets=(token,) if token else (), max_bytes=16 * 1024 * 1024,
    )


def decode_json(raw, description):
    try:
        return json.loads(raw, object_pairs_hook=strict_object)
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError) as exc:
        fail(f"{description} returned invalid JSON ({type(exc).__name__})")


def strict_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate JSON object key")
        result[key] = value
    return result


def commit_sha(value, label):
    if not isinstance(value, str) or re.fullmatch(r"[0-9a-f]{40}", value) is None:
        fail(f"{label} must be exactly 40 lowercase hexadecimal characters")
    return value


def expected_asset_names():
    try:
        with open("Cargo.toml", "rb") as cargo_file:
            cargo = tomllib.load(cargo_file)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        fail(f"cannot read Cargo metadata ({type(exc).__name__})")
    package = cargo.get("package")
    if not isinstance(package, dict) or package.get("name") != REPO:
        fail(f"Cargo package name must be {REPO}")
    declarations = cargo.get("bin", [])
    if not isinstance(declarations, list):
        fail("Cargo binary declaration is invalid")
    if declarations:
        if len(declarations) != 1 or not isinstance(declarations[0], dict):
            fail("Coronatio must declare exactly one binary target")
        binary_decl = declarations[0].get("name")
    else:
        binary_decl = package.get("name")
    if not isinstance(binary_decl, str) or not binary_decl:
        fail("Coronatio binary declaration is missing")
    binary_name = f"{binary_decl}-x86_64"
    return binary_name, f"{binary_name}.sha256", FLAG_NAME


def forgejo_release_url(sha):
    tag = f"sha-{sha}"
    return f"{FORGEJO_API}/repos/{OWNER}/{REPO}/releases/tags/{urllib.parse.quote(tag, safe='')}"


def forgejo_main_sha(token):
    url = f"{FORGEJO_API}/repos/{OWNER}/{REPO}/branches/main"
    status, raw, _headers = api_request("GET", url, token=token)
    if status != 200:
        fail(f"Forgejo main branch GET returned HTTP {status}")
    branch = decode_json(raw, "Forgejo main branch")
    commit = branch.get("commit") if isinstance(branch, dict) else None
    if not isinstance(commit, dict):
        fail("Forgejo main branch response has no commit object")
    return commit_sha(commit.get("id"), "Forgejo main head")


def load_source_release(sha, forgejo_token):
    url = forgejo_release_url(sha)
    status, raw, _headers = api_request("GET", url, token=forgejo_token)
    if status == 404:
        return None
    if status != 200:
        fail(f"Forgejo per-SHA Release GET returned HTTP {status}")
    release = decode_json(raw, "Forgejo per-SHA Release")
    if not isinstance(release, dict):
        fail("Forgejo per-SHA Release response is not an object")
    if release.get("tag_name") != f"sha-{sha}":
        fail("Forgejo per-SHA Release tag identity conflicts with the source SHA")
    if release.get("target_commitish") != sha:
        fail("Forgejo per-SHA Release target_commitish conflicts with the source SHA")
    if release.get("draft") is True:
        fail("Forgejo per-SHA Release is a draft")
    return url, release


def named_assets(release, expected_names, description):
    assets = release.get("assets")
    if not isinstance(assets, list):
        fail(f"{description} has no asset list")
    by_name = {}
    for asset in assets:
        if not isinstance(asset, dict):
            fail(f"{description} contains an invalid asset")
        name = asset.get("name")
        if not isinstance(name, str) or not name or name in by_name:
            fail(f"{description} contains an invalid or duplicate asset name")
        by_name[name] = asset
    if set(by_name) != set(expected_names):
        missing = sorted(set(expected_names) - set(by_name))
        extra = sorted(set(by_name) - set(expected_names))
        fail(f"{description} asset set mismatch (missing={missing}, extra={extra})")
    return by_name


def download_forgejo_asset(asset, name, forgejo_token):
    url = asset.get("browser_download_url")
    if not isinstance(url, str) or not url:
        fail(f"Forgejo asset {name} has no browser download URL")
    if origin(url) != FORGEJO_ORIGIN:
        fail(f"Forgejo asset {name} points outside the Forgejo origin")
    status, raw, _headers = http_request(
        "GET", url, token=f"token {forgejo_token}", auth_origins={FORGEJO_ORIGIN},
        headers={"Accept": "application/octet-stream"}, secrets=(forgejo_token,),
        max_bytes=512 * 1024 * 1024,
    )
    if status != 200:
        fail(f"Forgejo asset {name} download returned HTTP {status}")
    return raw


def validate_flag(flag_bytes, sha, binary_digest):
    flag = decode_json(flag_bytes, "Forgejo release.flag")
    if not isinstance(flag, dict):
        fail("Forgejo release.flag is not a JSON object")
    if flag.get("schema") != FLAG_SCHEMA:
        fail("Forgejo release.flag has a foreign schema")
    if flag.get("component") != REPO:
        fail("Forgejo release.flag has a conflicting component")
    if flag.get("source_sha") != sha:
        fail("Forgejo release.flag source_sha conflicts with the source SHA")
    if flag.get("sha256") != binary_digest:
        fail("Forgejo release.flag digest conflicts with the Coronatio binary")


def load_source_assets(release, names, sha, forgejo_token):
    assets = named_assets(release, names, "Forgejo per-SHA Release")
    received = {}
    for name in names:
        received[name] = download_forgejo_asset(assets[name], name, forgejo_token)
    binary_name, sidecar_name, flag_name = names
    binary = received[binary_name]
    binary_digest = hashlib.sha256(binary).hexdigest()
    expected_sidecar = f"{binary_digest}  {binary_name}\n".encode("ascii")
    if received[sidecar_name] != expected_sidecar:
        fail(f"Forgejo {sidecar_name} does not exactly match the downloaded binary")
    validate_flag(received[flag_name], sha, binary_digest)
    digests = {
        name: hashlib.sha256(received[name]).hexdigest()
        for name in names
    }
    return received, digests, binary_digest


def refs_url(service, ref="tags/latest"):
    if service == "forgejo":
        return f"{FORGEJO_API}/repos/{OWNER}/{REPO}/git/refs/{ref}"
    return f"{GITHUB_API}/repos/{OWNER}/{REPO}/git/ref/{ref}"


def git_tag_url(service, sha):
    if service == "forgejo":
        return f"{FORGEJO_API}/repos/{OWNER}/{REPO}/git/tags/{urllib.parse.quote(sha, safe='')}"
    return f"{GITHUB_API}/repos/{OWNER}/{REPO}/git/tags/{urllib.parse.quote(sha, safe='')}"


def normalize_ref(raw, expected_ref):
    if isinstance(raw, list):
        if not raw:
            return None
        if len(raw) != 1:
            fail(f"git ref lookup for {expected_ref} returned multiple references")
        raw = raw[0]
    if not isinstance(raw, dict):
        fail(f"git ref lookup for {expected_ref} returned an invalid object")
    ref_name = raw.get("ref")
    if ref_name is not None and ref_name != expected_ref:
        fail(f"git ref lookup returned {ref_name!r}, expected {expected_ref}")
    obj = raw.get("object")
    if not isinstance(obj, dict):
        fail(f"git ref {expected_ref} has no object")
    object_sha = obj.get("sha")
    object_type = obj.get("type")
    if not isinstance(object_sha, str) or re.fullmatch(r"[0-9a-f]{40}", object_sha) is None:
        fail(f"git ref {expected_ref} has an invalid object SHA")
    if object_type not in {"commit", "tag"}:
        fail(f"git ref {expected_ref} has an unsupported object type")
    return {"ref": expected_ref, "object_sha": object_sha, "object_type": object_type}


def get_ref(service, token, description):
    expected_ref = "refs/tags/latest"
    url = refs_url(service)
    status, raw, _headers = api_request("GET", url, token=token, service=service)
    if status == 404:
        return None
    if status != 200:
        fail(f"{description} GET returned HTTP {status}")
    return normalize_ref(decode_json(raw, description), expected_ref)


def peeled_target(service, token, ref, description):
    if ref is None:
        return None
    current = ref
    seen = set()
    for _depth in range(8):
        if current["object_type"] == "commit":
            return current["object_sha"]
        object_sha = current["object_sha"]
        if object_sha in seen:
            fail(f"{description} contains a cyclic annotated tag")
        seen.add(object_sha)
        status, raw, _headers = api_request(
            "GET", git_tag_url(service, object_sha), token=token, service=service,
        )
        if status != 200:
            fail(f"{description} annotated tag lookup returned HTTP {status}")
        tag = decode_json(raw, f"{description} annotated tag")
        obj = tag.get("object") if isinstance(tag, dict) else None
        if not isinstance(obj, dict):
            fail(f"{description} annotated tag has no target object")
        target_sha = obj.get("sha")
        target_type = obj.get("type")
        if not isinstance(target_sha, str) or re.fullmatch(r"[0-9a-f]{40}", target_sha) is None:
            fail(f"{description} annotated tag has an invalid target SHA")
        if target_type not in {"commit", "tag"}:
            fail(f"{description} annotated tag has an unsupported target type")
        current = {"object_sha": target_sha, "object_type": target_type}
    fail(f"{description} annotated tag nesting exceeds the allowed depth")


def wait_for_github_mirror(sha, token):
    for attempt in range(1, MIRROR_MAX_ATTEMPTS + 1):
        ref = get_ref("github", token, "GitHub mirrored latest ref")
        target = peeled_target("github", token, ref, "GitHub mirrored latest ref")
        if ref is not None and target == sha:
            return ref, target
        if attempt < MIRROR_MAX_ATTEMPTS:
            time.sleep(MIRROR_BACKOFF_SECONDS[attempt - 1])
    fail("GitHub refs/tags/latest did not mirror the source SHA within the bounded readiness window")


def git_ref_description(service, ref, target_sha):
    if ref is None:
        return {"exists": False, "target_sha": None, "matches_source": False}
    return {
        "exists": True,
        "object_sha": ref["object_sha"],
        "object_type": ref["object_type"],
        "target_sha": target_sha,
        "matches_source": target_sha is not None,
    }


def local_git_state():
    try:
        head = subprocess.run(
            ["git", "rev-parse", "--verify", "HEAD^{commit}"],
            stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, check=False,
        )
        branch = subprocess.run(
            ["git", "symbolic-ref", "--quiet", "HEAD"],
            stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, check=False,
        )
    except OSError:
        fail("unable to inspect the CI checkout with git")
    if head.returncode != 0:
        fail("unable to resolve the CI checkout HEAD")
    head_sha = head.stdout.strip()
    commit_sha(head_sha, "local HEAD")
    if branch.returncode not in {0, 1}:
        fail("unable to inspect the CI checkout branch")
    branch_ref = branch.stdout.strip() if branch.returncode == 0 else None
    return head_sha, branch_ref


def push_forgejo_latest(sha, token, expected_state):
    git_env = os.environ.copy()
    ssl_cert_file = git_env.get("SSL_CERT_FILE")
    if not ssl_cert_file or not os.path.isfile(ssl_cert_file):
        fail("SSL_CERT_FILE must name the installed Forgejo CA bundle")
    git_env["GIT_SSL_CAINFO"] = ssl_cert_file
    git_env["GIT_CONFIG_COUNT"] = "1"
    git_env["GIT_CONFIG_KEY_0"] = "http.https://git.home.arpa/.extraheader"
    git_env["GIT_CONFIG_VALUE_0"] = f"Authorization: token {token}"
    git_env["GIT_TERMINAL_PROMPT"] = "0"
    git_env["GIT_ASKPASS"] = "/bin/false"
    git_env["SSH_ASKPASS"] = "/bin/false"
    for key in tuple(git_env):
        if key.startswith("GIT_TRACE") or key == "GIT_CURL_VERBOSE":
            git_env.pop(key, None)
    command = [
        "git", "-c", "credential.helper=", "-c", "http.followRedirects=false",
        "push", "--force", "--no-follow-tags", "--no-verify",
        "--recurse-submodules=no", "--porcelain", FORGEJO_GIT_URL,
        f"{sha}:refs/tags/latest",
    ]
    try:
        result = subprocess.run(
            command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
            check=False, env=git_env,
        )
    except OSError:
        fail("Forgejo latest tag git push could not execute")
    if result.returncode != 0:
        fail(f"Forgejo latest tag git push failed (exit code {result.returncode})")
    if local_git_state() != expected_state:
        fail("Forgejo latest tag push changed the CI checkout HEAD or branch")


def move_forgejo_latest(sha, token, current_ref):
    local_state = local_git_state()
    if local_state[0] != sha:
        fail("CI checkout HEAD does not match CI_COMMIT_SHA")
    if current_ref and current_ref["object_type"] == "commit" and current_ref["object_sha"] == sha:
        return "no-op"
    if current_ref is None:
        push_forgejo_latest(sha, token, local_state)
        return "created"
    push_forgejo_latest(sha, token, local_state)
    return "force-moved"


def github_release_url(tag="latest"):
    return f"{GITHUB_API}/repos/{OWNER}/{REPO}/releases/tags/{urllib.parse.quote(tag, safe='')}"


def get_github_release(token, description="GitHub latest Release"):
    status, raw, _headers = api_request(
        "GET", github_release_url(), token=token, service="github",
    )
    if status == 404:
        return None
    if status != 200:
        fail(f"{description} GET returned HTTP {status}")
    release = decode_json(raw, description)
    if not isinstance(release, dict) or release.get("tag_name") != "latest":
        fail("GitHub returned an invalid Release for tag latest")
    release_id = release.get("id")
    if not isinstance(release_id, int) or isinstance(release_id, bool):
        fail("GitHub latest Release has no numeric id")
    return release


def github_release_assets(release_id, token):
    all_assets = []
    page = 1
    while True:
        query = urllib.parse.urlencode({"per_page": 100, "page": page})
        url = f"{GITHUB_API}/repos/{OWNER}/{REPO}/releases/{release_id}/assets?{query}"
        status, raw, _headers = api_request("GET", url, token=token, service="github")
        if status != 200:
            fail(f"GitHub Release asset page {page} GET returned HTTP {status}")
        batch = decode_json(raw, f"GitHub Release asset page {page}")
        if not isinstance(batch, list) or any(not isinstance(asset, dict) for asset in batch):
            fail("GitHub Release asset list is invalid")
        all_assets.extend(batch)
        if len(batch) < 100:
            return all_assets
        page += 1
        if page > 1000:
            fail("GitHub Release asset pagination exceeded the safety limit")


def github_asset_bytes(asset, name):
    url = asset.get("browser_download_url")
    if not isinstance(url, str) or not url:
        fail(f"GitHub asset {name} has no browser download URL")
    parsed = urllib.parse.urlsplit(url)
    if parsed.scheme.lower() != "https" or (parsed.hostname or "").lower() != "github.com":
        fail(f"GitHub asset {name} points outside github.com")
    status, raw, _headers = http_request(
        "GET", url, headers={"Accept": "application/octet-stream"},
        max_bytes=512 * 1024 * 1024,
    )
    if status != 200:
        fail(f"GitHub asset {name} download returned HTTP {status}")
    return raw


def github_assets_match(release, source_assets, expected_names, token):
    assets = github_release_assets(release["id"], token)
    by_name = {}
    for asset in assets:
        name = asset.get("name")
        if not isinstance(name, str) or not name or name in by_name:
            return False, assets
        by_name[name] = asset
    if set(by_name) != set(expected_names):
        return False, assets
    for name in expected_names:
        actual = github_asset_bytes(by_name[name], name)
        if actual != source_assets[name]:
            return False, assets
    return True, assets


def github_upload_url(release):
    url = release.get("upload_url")
    if not isinstance(url, str) or not url:
        fail("GitHub Release has no upload URL")
    upload_base = url.split("{", 1)[0]
    parsed = urllib.parse.urlsplit(upload_base)
    if origin(upload_base) != GITHUB_UPLOAD_ORIGIN:
        fail("GitHub Release upload URL is not on uploads.github.com")
    expected_path = f"/repos/{OWNER}/{REPO}/releases/{release.get('id')}/assets"
    if parsed.path.rstrip("/") != expected_path:
        fail("GitHub Release upload URL points to a different release")
    return upload_base


def delete_github_assets(release, assets, token):
    for asset in assets:
        asset_id = asset.get("id")
        if not isinstance(asset_id, int) or isinstance(asset_id, bool):
            fail("GitHub Release contains an asset without a numeric id")
        url = f"{GITHUB_API}/repos/{OWNER}/{REPO}/releases/assets/{asset_id}"
        status, _raw, _headers = api_request(
            "DELETE", url, token=token, service="github",
        )
        if status != 204:
            fail(f"GitHub old asset deletion returned HTTP {status}")


def create_or_update_release(sha, token, release):
    payload = {
        "tag_name": "latest",
        "target_commitish": sha,
        "name": "Coronatio latest",
        "draft": False,
        "prerelease": False,
    }
    if release is None:
        url = f"{GITHUB_API}/repos/{OWNER}/{REPO}/releases"
        status, raw, _headers = api_request(
            "POST", url, token=token, service="github", payload=payload,
        )
        if status not in {201}:
            fail(f"GitHub latest Release creation returned HTTP {status}")
    else:
        url = f"{GITHUB_API}/repos/{OWNER}/{REPO}/releases/{release['id']}"
        status, raw, _headers = api_request(
            "PATCH", url, token=token, service="github", payload=payload,
        )
        if status != 200:
            fail(f"GitHub latest Release update returned HTTP {status}")
    updated = decode_json(raw, "GitHub latest Release write")
    if not isinstance(updated, dict) or updated.get("tag_name") != "latest":
        fail("GitHub latest Release write returned a conflicting tag")
    if updated.get("target_commitish") != sha:
        fail("GitHub latest Release target_commitish does not match the source SHA")
    if updated.get("draft") is True or updated.get("prerelease") is True:
        fail("GitHub latest Release write returned draft or prerelease state")
    return updated


def upload_github_assets(release, source_assets, expected_names, token):
    upload_base = github_upload_url(release)
    for name in expected_names:
        content_type = "application/octet-stream" if name.endswith("-x86_64") else "text/plain; charset=utf-8"
        if name == FLAG_NAME:
            content_type = "application/json"
        url = f"{upload_base}?{urllib.parse.urlencode({'name': name})}"
        status, raw, _headers = http_request(
            "POST", url, token=f"Bearer {token}", auth_origins={GITHUB_UPLOAD_ORIGIN},
            body=source_assets[name], headers={
                "Accept": "application/vnd.github+json",
                "Content-Type": content_type,
                "X-GitHub-Api-Version": "2022-11-28",
            }, secrets=(token,), max_bytes=16 * 1024 * 1024,
        )
        if status not in {201}:
            fail(f"GitHub asset upload for {name} returned HTTP {status}")
        uploaded = decode_json(raw, f"GitHub upload of {name}")
        if not isinstance(uploaded, dict) or uploaded.get("name") != name:
            fail(f"GitHub upload response for {name} has a conflicting asset name")


def asset_digests(source_assets, names):
    return [
        {"name": name, "size": len(source_assets[name]), "sha256": hashlib.sha256(source_assets[name]).hexdigest()}
        for name in names
    ]


def emit(payload):
    print(json.dumps(payload, separators=(",", ":"), sort_keys=True))


def emit_stale_noop(sha, forgejo_main_head, forgejo_latest_move=None):
    emit({
        "schema": "coronatio.github_latest.v1",
        "status": "no-op",
        "source_sha": sha,
        "reason": "stale main pipeline; Forgejo main no longer matches source SHA",
        "forgejo_main_sha": forgejo_main_head,
        "forgejo_main_matches_source": False,
        "forgejo_latest_move": forgejo_latest_move,
        "github_release_mutation": "skipped",
    })


def plan(sha, forgejo_token, source_release, names, forgejo_main_head):
    if source_release is None:
        github_ref = get_ref("github", None, "anonymous GitHub latest ref")
        github_target = peeled_target("github", None, github_ref, "anonymous GitHub latest ref")
        github_release = get_github_release(None, "anonymous GitHub latest Release")
        github_assets = []
        if github_release is not None:
            github_assets = github_release_assets(github_release["id"], None)
        emit({
            "schema": "coronatio.github_latest.v1",
            "status": "no-op",
            "source_sha": sha,
            "forgejo_main_sha": forgejo_main_head,
            "forgejo_main_matches_source": forgejo_main_head == sha,
            "reason": "Forgejo per-SHA Release is absent (HTTP 404)",
            "expected_assets": list(names),
            "assets": [],
            "forgejo_latest_move": None,
            "github_latest": {
                "ref_exists": github_ref is not None,
                "ref_object_sha": github_ref["object_sha"] if github_ref else None,
                "ref_object_type": github_ref["object_type"] if github_ref else None,
                "ref_target_sha": github_target,
                "release": None if github_release is None else {
                    "id": github_release["id"],
                    "tag_name": github_release.get("tag_name"),
                    "asset_names": [asset.get("name") for asset in github_assets],
                },
            },
        })
        return
    source_url, release = source_release
    source_assets, digests, binary_digest = load_source_assets(
        release, names, sha, forgejo_token,
    )
    del binary_digest
    current_forgejo = get_ref("forgejo", forgejo_token, "Forgejo latest ref")
    forgejo_target = peeled_target("forgejo", forgejo_token, current_forgejo, "Forgejo latest ref")
    if current_forgejo is None:
        move = "create"
    elif current_forgejo["object_type"] == "commit" and current_forgejo["object_sha"] == sha:
        move = "no-op"
    else:
        move = "force-move"
    github_ref = get_ref("github", None, "anonymous GitHub latest ref")
    github_target = peeled_target("github", None, github_ref, "anonymous GitHub latest ref")
    github_release = get_github_release(None, "anonymous GitHub latest Release")
    github_assets = []
    if github_release is not None:
        github_assets = github_release_assets(github_release["id"], None)
    github_release_summary = None if github_release is None else {
        "id": github_release["id"],
        "tag_name": github_release.get("tag_name"),
        "target_commitish": github_release.get("target_commitish"),
        "asset_names": [asset.get("name") for asset in github_assets],
    }
    emit({
        "schema": "coronatio.github_latest.v1",
        "status": "plan",
        "source_sha": sha,
        "forgejo_main_sha": forgejo_main_head,
        "forgejo_main_matches_source": forgejo_main_head == sha,
        "forgejo_source_release": {
            "tag": release.get("tag_name"),
            "target_commitish": release.get("target_commitish"),
            "url": source_url,
        },
        "assets": asset_digests(source_assets, names),
        "binary_sha256": digests[names[0]],
        "forgejo_latest": {
            "current_object_sha": current_forgejo["object_sha"] if current_forgejo else None,
            "current_object_type": current_forgejo["object_type"] if current_forgejo else None,
            "current_target_sha": forgejo_target,
            "proposed_action": move,
            "proposed_target_sha": sha,
        },
        "github_latest": {
            "ref_exists": github_ref is not None,
            "ref_object_sha": github_ref["object_sha"] if github_ref else None,
            "ref_object_type": github_ref["object_type"] if github_ref else None,
            "ref_target_sha": github_target,
            "ref_matches_source": github_target == sha,
            "release": github_release_summary,
        },
    })


def live(sha, forgejo_token, github_token, source_release, names, forgejo_main_head):
    if source_release is None:
        emit({
            "schema": "coronatio.github_latest.v1",
            "status": "no-op",
            "source_sha": sha,
            "forgejo_main_sha": forgejo_main_head,
            "forgejo_main_matches_source": forgejo_main_head == sha,
            "reason": "Forgejo per-SHA Release is absent (HTTP 404)",
            "expected_assets": list(names),
            "forgejo_latest_move": None,
        })
        return
    release_url, release = source_release
    source_assets, digests, binary_digest = load_source_assets(
        release, names, sha, forgejo_token,
    )
    current_forgejo = get_ref("forgejo", forgejo_token, "Forgejo latest ref")
    forgejo_main_head = forgejo_main_sha(forgejo_token)
    if forgejo_main_head != sha:
        emit_stale_noop(sha, forgejo_main_head)
        return
    operation = move_forgejo_latest(sha, forgejo_token, current_forgejo)
    verified_forgejo = get_ref("forgejo", forgejo_token, "Forgejo latest ref readback")
    verified_target = peeled_target("forgejo", forgejo_token, verified_forgejo, "Forgejo latest ref readback")
    if (
        verified_forgejo is None
        or verified_forgejo["object_type"] != "commit"
        or verified_forgejo["object_sha"] != sha
        or verified_target != sha
    ):
        fail("Forgejo latest ref readback does not directly target the source commit")

    try:
        github_ref, github_target = wait_for_github_mirror(sha, github_token)
    except PublishError:
        forgejo_main_head = forgejo_main_sha(forgejo_token)
        if forgejo_main_head != sha:
            emit_stale_noop(sha, forgejo_main_head, operation)
            return
        raise

    github_release = get_github_release(github_token)
    if github_release is not None and github_release.get("tag_name") != "latest":
        fail("existing GitHub Release is not tagged latest")
    matched = False
    existing_assets = []
    if github_release is not None:
        matched, existing_assets = github_assets_match(
            github_release, source_assets, names, github_token,
        )

    # Recheck the mirrored ref immediately before any GitHub Release mutation;
    # GitHub's release-create API can otherwise create a missing tag implicitly.
    github_ref = get_ref("github", github_token, "GitHub mirrored latest ref preflight")
    github_target = peeled_target("github", github_token, github_ref, "GitHub mirrored latest ref preflight")
    if github_ref is None or github_target != sha:
        fail("GitHub refs/tags/latest changed before Release update; refusing downstream tag creation")
    forgejo_main_head = forgejo_main_sha(forgejo_token)
    if forgejo_main_head != sha:
        emit_stale_noop(sha, forgejo_main_head, operation)
        return

    if matched:
        if github_release is None:
            fail("GitHub asset comparison returned a match without a Release")
        metadata_changed = (
            github_release.get("target_commitish") != sha
            or github_release.get("name") != "Coronatio latest"
            or github_release.get("draft") is True
            or github_release.get("prerelease") is True
        )
        if metadata_changed:
            github_release = create_or_update_release(sha, github_token, github_release)
            status = "updated"
        else:
            status = "no-op"
    else:
        was_new_release = github_release is None
        for asset in existing_assets:
            asset_id = asset.get("id")
            if not isinstance(asset_id, int) or isinstance(asset_id, bool):
                fail("GitHub Release contains an asset without a numeric id")
        github_release = create_or_update_release(sha, github_token, github_release)
        if existing_assets:
            delete_github_assets(github_release, existing_assets, github_token)
        remaining = github_release_assets(github_release["id"], github_token)
        if remaining:
            fail("GitHub old assets remain after deletion; refusing upload")
        upload_github_assets(github_release, source_assets, names, github_token)
        status = "published" if was_new_release else "updated"

    reread_release = get_github_release(github_token, "GitHub latest Release readback")
    if reread_release is None or reread_release.get("id") != github_release.get("id"):
        fail("GitHub latest Release readback identity changed")
    if reread_release.get("target_commitish") != sha:
        fail("GitHub latest Release readback target_commitish conflicts with the source SHA")
    final_matches, final_assets = github_assets_match(
        reread_release, source_assets, names, github_token,
    )
    if not final_matches:
        fail("GitHub latest Release assets differ from the verified Forgejo source bytes")
    final_ref = get_ref("github", github_token, "GitHub mirrored latest ref final readback")
    final_target = peeled_target("github", github_token, final_ref, "GitHub mirrored latest ref final readback")
    if final_ref is None or final_target != sha:
        fail("GitHub refs/tags/latest no longer resolves to the source SHA")
    emit({
        "schema": "coronatio.github_latest.v1",
        "status": status,
        "source_sha": sha,
        "forgejo_main_sha": forgejo_main_head,
        "forgejo_main_matches_source": True,
        "forgejo_source_release": release_url,
        "forgejo_latest_move": operation,
        "github_release_tag": reread_release.get("tag_name"),
        "github_release_id": reread_release.get("id"),
        "github_ref_target_sha": final_target,
        "assets": asset_digests(source_assets, names),
        "binary_sha256": binary_digest,
        "verified_asset_names": [asset.get("name") for asset in final_assets],
    })


PLAN_MODE = False


def main():
    global PLAN_MODE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--plan", action="store_true", help="GET-only Forgejo/GitHub plan; no mutations")
    args = parser.parse_args()
    PLAN_MODE = args.plan

    sha = commit_sha(os.environ.get("CI_COMMIT_SHA", ""), "CI_COMMIT_SHA")
    forgejo_token = os.environ.get("FORGEJO_TOKEN", "")
    if not forgejo_token:
        fail("FORGEJO_TOKEN is required")
    names = expected_asset_names()
    forgejo_main_head = forgejo_main_sha(forgejo_token)
    if not args.plan and forgejo_main_head != sha:
        emit_stale_noop(sha, forgejo_main_head)
        return
    source_release = load_source_release(sha, forgejo_token)
    if args.plan:
        plan(sha, forgejo_token, source_release, names, forgejo_main_head)
        return
    if source_release is None:
        live(sha, forgejo_token, "", source_release, names, forgejo_main_head)
        return
    github_token = os.environ.get("GITHUB_TOKEN", "")
    if not github_token:
        fail("GITHUB_TOKEN is required")
    live(sha, forgejo_token, github_token, source_release, names, forgejo_main_head)


if __name__ == "__main__":
    try:
        main()
    except PublishError as exc:
        print(f"github_latest: error: {safe_text(exc)}", file=sys.stderr)
        raise SystemExit(1)
    except KeyboardInterrupt:
        print("github_latest: interrupted", file=sys.stderr)
        raise SystemExit(1)
