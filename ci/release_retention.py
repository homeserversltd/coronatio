#!/usr/bin/env python3
"""Retain the newest Coronatio source releases and remove older release refs."""

import argparse
import datetime as dt
import json
import os
import re
import subprocess
import sys
import urllib.parse
from typing import NoReturn

try:
    from . import release_publish as publisher
except ImportError:  # pragma: no cover - exercised when run as a script
    import release_publish as publisher


KEEP_RELEASES = 20
SHA_RE = re.compile(r"^sha-([0-9a-f]{40})$")
ALLOWED_REPOS = ("coronatio", "caduceus", "kether")


def fail(message) -> NoReturn:
    print(f"release_retention: {message}", file=sys.stderr)
    raise SystemExit(1)


def valid_release(release):
    if not isinstance(release, dict):
        fail("release list contains a non-object")
    draft = release.get("draft")
    if draft is True:
        return None
    if draft is not False:
        fail("release draft field is not boolean")
    tag = release.get("tag_name")
    if not isinstance(tag, str):
        return None
    match = SHA_RE.fullmatch(tag)
    if match is None or release.get("target_commitish") != match.group(1):
        return None
    release_id = release.get("id")
    if not isinstance(release_id, int) or isinstance(release_id, bool) or release_id < 1:
        fail("eligible release has an invalid numeric id")
    created_at = release.get("created_at")
    if not isinstance(created_at, str):
        fail(f"eligible release {release_id} has no created_at timestamp")
    try:
        parsed = dt.datetime.fromisoformat(created_at.replace("Z", "+00:00"))
    except ValueError:
        fail(f"eligible release {release_id} has an invalid created_at timestamp")
    if parsed.tzinfo is None:
        fail(f"eligible release {release_id} created_at is not timezone-aware")
    return {"id": release_id, "tag": release["tag_name"], "sha": match.group(1), "created_at": parsed}


def list_releases(repo, token):
    records = []
    page = 1
    while True:
        query = urllib.parse.urlencode({"limit": 50, "page": page})
        url = f"{publisher.API_ROOT}/repos/{publisher.OWNER}/{repo}/releases?{query}"
        status, raw = publisher.request("GET", url, token)
        if status != 200:
            fail(f"GET {repo} releases page {page} returned HTTP {status}")
        batch = publisher.decode(raw, f"{repo} release list page {page}")
        if not isinstance(batch, list):
            fail(f"{repo} release list page {page} is not an array")
        records.extend(batch)
        if len(batch) < 50:
            break
        page += 1
    return records


def plan(repo, records, published_sha):
    eligible = [record for item in records if (record := valid_release(item)) is not None]
    eligible.sort(key=lambda item: (item["created_at"], item["id"]), reverse=True)
    keep = {item["id"] for item in eligible[:KEEP_RELEASES]}
    protected = [item for item in eligible if item["sha"] == published_sha]
    keep.update(item["id"] for item in protected)
    return {
        "repo": repo,
        "kept_count": len(keep),
        "kept_ids": sorted(keep),
        "deleted": [item for item in eligible if item["id"] not in keep],
    }


def tag_ref_url(repo, tag):
    quoted = urllib.parse.quote(tag, safe="")
    return f"{publisher.API_ROOT}/repos/{publisher.OWNER}/{repo}/git/refs/tags/{quoted}"


def verify_tag_absent(repo, tag, token):
    status, _raw = publisher.request("GET", tag_ref_url(repo, tag), token)
    if status != 404:
        fail(f"verification of {repo} git ref {tag} returned HTTP {status}; expected 404")


def git_delete_tag(repo, tag, token):
    # Forgejo's tag-delete endpoint can return 404 while the underlying ref remains.
    # Supply auth through Git's environment config, never arguments or output.
    import base64

    authorization = base64.b64encode(f"token:{token}".encode("utf-8")).decode("ascii")
    env = os.environ.copy()
    env.update({
        "GIT_CONFIG_COUNT": "1",
        "GIT_CONFIG_KEY_0": "http.https://git.home.arpa/.extraheader",
        "GIT_CONFIG_VALUE_0": f"Authorization: Basic {authorization}",
        "GIT_TERMINAL_PROMPT": "0",
    })
    remote = f"https://git.home.arpa/{publisher.OWNER}/{repo}.git"
    try:
        result = subprocess.run(
            ["git", "push", remote, "--delete", tag],
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
    except OSError as exc:
        fail(f"git ref removal for {repo}/{tag} could not run: {exc}")
    if result.returncode != 0:
        # Avoid forwarding Git output, which could disclose auth-related details.
        fail(f"git ref removal for {repo}/{tag} failed with exit status {result.returncode}")


def delete_release(repo, record, token):
    url = f"{publisher.RELEASES}/{record['id']}" if repo == publisher.REPO else (
        f"{publisher.API_ROOT}/repos/{publisher.OWNER}/{repo}/releases/{record['id']}"
    )
    status, _raw = publisher.request("DELETE", url, token)
    if status not in (200, 204):
        fail(f"DELETE {repo} release {record['id']} returned HTTP {status}")
    status, _raw = publisher.request("GET", url, token)
    if status != 404:
        fail(f"verification of {repo} release {record['id']} returned HTTP {status}; expected 404")

    tag_url = f"{publisher.API_ROOT}/repos/{publisher.OWNER}/{repo}/tags/{urllib.parse.quote(record['tag'], safe='')}"
    status, _raw = publisher.request("DELETE", tag_url, token)
    if status not in (200, 202, 204, 404):
        fail(f"DELETE {repo} tag {record['tag']} returned HTTP {status}")
    tag_delete_404 = status == 404

    ref_status, _raw = publisher.request("GET", tag_ref_url(repo, record["tag"]), token)
    removed_by_git = False
    already_absent = False
    if ref_status == 200:
        git_delete_tag(repo, record["tag"], token)
        verify_tag_absent(repo, record["tag"], token)
        removed_by_git = True
    elif ref_status == 404:
        already_absent = True
    else:
        fail(f"GET {repo} git ref {record['tag']} returned HTTP {ref_status}")
    return {
        "tag_delete_404": tag_delete_404,
        "ref_removed_by_git": removed_by_git,
        "ref_already_absent": already_absent,
    }


def run(repo, token, published_sha, dry_run):
    result = plan(repo, list_releases(repo, token), published_sha)
    tag_delete_404 = []
    refs_removed_by_git = []
    refs_already_absent = []
    if not dry_run:
        for record in result["deleted"]:
            handling = delete_release(repo, record, token)
            tag = record["tag"]
            if handling["tag_delete_404"]:
                tag_delete_404.append(tag)
            if handling["ref_removed_by_git"]:
                refs_removed_by_git.append(tag)
            if handling["ref_already_absent"]:
                refs_already_absent.append(tag)
    return {
        "repo": repo,
        "kept_count": result["kept_count"],
        "kept_ids": result["kept_ids"],
        "deleted_ids": [record["id"] for record in result["deleted"]],
        "deleted_tags": [record["tag"] for record in result["deleted"]],
        "tag_delete_404": tag_delete_404,
        "refs_removed_by_git": refs_removed_by_git,
        "refs_already_absent": refs_already_absent,
    }


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", choices=ALLOWED_REPOS, default="coronatio",
                        help="release repository (non-Coronatio choices are comparative dry-run only)")
    parser.add_argument("--dry-run", action="store_true", help="GET-only plan; never mutate Forgejo")
    return parser.parse_args()


def main():
    args = parse_args()
    if not args.dry_run and args.repo != "coronatio":
        fail("live retention is restricted to coronatio")
    if not args.dry_run and os.environ.get("CI_COMMIT_BRANCH") != "main":
        fail("live retention is restricted to the main branch")
    token = os.environ.get("FORGEJO_TOKEN", "")
    if not token:
        fail("FORGEJO_TOKEN is required")
    published_sha = os.environ.get("CI_COMMIT_SHA", "")
    if re.fullmatch(r"[0-9a-f]{40}", published_sha) is None:
        fail("CI_COMMIT_SHA must be exactly 40 lowercase hexadecimal characters")
    receipt = run(args.repo, token, published_sha, args.dry_run)
    print(json.dumps({
        "schema": "coronatio.release_retention.v1",
        "ok": True,
        "status": "planned" if args.dry_run else "retained",
        "dry_run": args.dry_run,
        **receipt,
    }, separators=(",", ":")))


if __name__ == "__main__":
    main()
