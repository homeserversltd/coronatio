#!/usr/bin/env python3
"""Retain the newest Coronatio source releases and remove older release refs."""

import argparse
import datetime as dt
import json
import os
import re
import sys
import urllib.parse
from typing import NoReturn

try:
    from . import release_publish as publisher
except ImportError:  # pragma: no cover - exercised when run as a script
    import release_publish as publisher


KEEP_RELEASES = 20
SHA_RE = re.compile(r"^sha-([0-9a-f]{40})$")


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
    return {"id": release_id, "tag": tag, "sha": match.group(1), "created_at": parsed}


def list_releases(token):
    records = []
    page = 1
    while True:
        query = urllib.parse.urlencode({"limit": 50, "page": page})
        url = f"{publisher.RELEASES}?{query}"
        status, raw = publisher.request("GET", url, token)
        if status != 200:
            fail(f"GET {publisher.REPO} releases page {page} returned HTTP {status}")
        batch = publisher.decode(raw, f"{publisher.REPO} release list page {page}")
        if not isinstance(batch, list):
            fail(f"{publisher.REPO} release list page {page} is not an array")
        records.extend(batch)
        if len(batch) < 50:
            break
        page += 1
    return records


def plan(records, published_sha):
    eligible = [record for item in records if (record := valid_release(item)) is not None]
    eligible.sort(key=lambda item: (item["created_at"], item["id"]), reverse=True)
    keep = {item["id"] for item in eligible[:KEEP_RELEASES]}
    keep.update(item["id"] for item in eligible if item["sha"] == published_sha)
    kept = [item for item in eligible if item["id"] in keep]
    return {
        "kept_count": len(keep),
        "kept_ids": [item["id"] for item in kept],
        "kept_tags": [item["tag"] for item in kept],
        "deleted": [item for item in eligible if item["id"] not in keep],
    }


def tag_url(tag):
    quoted = urllib.parse.quote(tag, safe="")
    return f"{publisher.API_ROOT}/repos/{publisher.OWNER}/{publisher.REPO}/tags/{quoted}"


def tag_ref_url(tag):
    quoted = urllib.parse.quote(tag, safe="")
    return f"{publisher.API_ROOT}/repos/{publisher.OWNER}/{publisher.REPO}/git/refs/tags/{quoted}"


def delete_release(record, token):
    url = f"{publisher.RELEASES}/{record['id']}"
    status, _raw = publisher.request("DELETE", url, token)
    if status not in (200, 204):
        fail(f"DELETE {publisher.REPO} release {record['id']} returned HTTP {status}")
    status, _raw = publisher.request("GET", url, token)
    if status != 404:
        fail(f"verification of {publisher.REPO} release {record['id']} returned HTTP {status}; expected 404")

    status, _raw = publisher.request("DELETE", tag_url(record["tag"]), token)
    if status not in (200, 202, 204, 404):
        fail(f"DELETE {publisher.REPO} tag {record['tag']} returned HTTP {status}")
    tag_delete_404 = status == 404

    ref_status, _raw = publisher.request("GET", tag_ref_url(record["tag"]), token)
    if ref_status != 404:
        fail(f"tag-ref-remains-{record['tag']} (HTTP {ref_status})")
    return {"tag_delete_404": tag_delete_404, "remaining_tag_ref": ref_status == 200,
            "ref_already_absent": ref_status == 404}


def run(token, published_sha, dry_run):
    result = plan(list_releases(token), published_sha)
    tag_delete_404 = []
    remaining_tag_refs = []
    refs_already_absent = []
    if not dry_run:
        for record in result["deleted"]:
            handling = delete_release(record, token)
            tag = record["tag"]
            if handling["tag_delete_404"]:
                tag_delete_404.append(tag)
            if handling["remaining_tag_ref"]:
                remaining_tag_refs.append(tag)
            if handling["ref_already_absent"]:
                refs_already_absent.append(tag)
    return {
        "repo": publisher.REPO,
        "kept_count": result["kept_count"],
        "kept_ids": result["kept_ids"],
        "kept_tags": result["kept_tags"],
        "deleted_ids": [record["id"] for record in result["deleted"]],
        "deleted_tags": [record["tag"] for record in result["deleted"]],
        "tag_delete_404": tag_delete_404,
        "remaining_tag_refs": remaining_tag_refs,
        "refs_already_absent": refs_already_absent,
    }


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="GET-only plan; never mutate Forgejo")
    return parser.parse_args()


def main():
    args = parse_args()
    if not args.dry_run and os.environ.get("CI_COMMIT_BRANCH") != "main":
        fail("live retention is restricted to the main branch")
    token = os.environ.get("FORGEJO_TOKEN", "")
    if not token:
        fail("FORGEJO_TOKEN is required")
    published_sha = os.environ.get("CI_COMMIT_SHA", "")
    if re.fullmatch(r"[0-9a-f]{40}", published_sha) is None:
        fail("CI_COMMIT_SHA must be exactly 40 lowercase hexadecimal characters")
    receipt = run(token, published_sha, args.dry_run)
    print(json.dumps({
        "schema": "coronatio.release_retention.v1",
        "ok": True,
        "status": "planned" if args.dry_run else "retained",
        "dry_run": args.dry_run,
        **receipt,
    }, separators=(",", ":")))


if __name__ == "__main__":
    main()
