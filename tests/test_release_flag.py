import contextlib
import hashlib
import io
import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from ci import release_flag, release_publish


SHA = "a" * 40
PIPELINE_URL = "https://ci.home.arpa/coronatio/42"
BINARY_NAME = "coronatio-x86_64"
SIDECAR_NAME = BINARY_NAME + ".sha256"
CREATED_AT = "2026-09-04T12:34:56Z"
RELEASE_CREATED_AT = "2026-09-04T00:00:00Z"


class FakeForgejo:
    def __init__(self, remote_binary, flag=None, tag_status=200, upload_status=201, race_flag=None):
        digest = hashlib.sha256(remote_binary).hexdigest()
        self.release = {
            "id": 7,
            "created_at": RELEASE_CREATED_AT,
            "tag_name": release_publish.release_tag(SHA),
            "target_commitish": SHA,
            "assets": [
                {"name": BINARY_NAME, "browser_download_url": "https://fake/binary"},
                {"name": SIDECAR_NAME, "browser_download_url": "https://fake/sidecar"},
            ],
        }
        self.bodies = {
            "https://fake/binary": remote_binary,
            "https://fake/sidecar": f"{digest}  {BINARY_NAME}".encode() + bytes((10,)),
        }
        if flag is not None:
            self.add_flag(flag)
        self.tag_status = tag_status
        self.upload_status = upload_status
        self.race_flag = race_flag
        self.calls = []

    def add_flag(self, body):
        self.release["assets"].append({
            "name": release_flag.FLAG_NAME,
            "browser_download_url": "https://fake/flag",
        })
        self.bodies["https://fake/flag"] = body

    def __call__(self, method, url, token, body=None, content_type=None, accept=None):
        self.calls.append({
            "method": method,
            "url": url,
            "token": token,
            "body": body,
            "content_type": content_type,
            "accept": accept,
        })
        if method == "GET" and url.endswith(f"/tags/{release_publish.release_tag(SHA)}"):
            if self.tag_status != 200:
                return self.tag_status, b"{}"
            return 200, json.dumps(self.release).encode()
        if method == "GET" and url in self.bodies:
            return 200, self.bodies[url]
        if method == "POST" and "/assets?name=release.flag" in url:
            if self.upload_status == 409 and self.race_flag is not None:
                self.add_flag(self.race_flag)
            elif self.upload_status in (200, 201):
                self.add_flag(body)
            return self.upload_status, b"{}"
        raise AssertionError(f"unexpected fake request: {method} {url}")


class ReleaseFlagTests(unittest.TestCase):
    def run_main(self, fake, local_binary=b"local-binary"):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cargo = """[package]
name = "coronatio"
version = "0.1.0"
edition = "2021"
"""
            (root / "Cargo.toml").write_text(cargo)
            binary_path = root / "target" / "release" / "coronatio"
            binary_path.parent.mkdir(parents=True)
            binary_path.write_bytes(local_binary)
            environment = {
                "FORGEJO_TOKEN": "test-token",
                "CI_COMMIT_SHA": SHA,
                "CI_PIPELINE_URL": PIPELINE_URL,
                "CARGO_TARGET_DIR": str(root / "target"),
            }
            stdout = io.StringIO()
            with mock.patch.dict(os.environ, environment, clear=False):
                with contextlib.redirect_stdout(stdout):
                    with mock.patch.object(release_publish, "request", side_effect=fake):
                        old_directory = os.getcwd()
                        os.chdir(root)
                        try:
                            with mock.patch.object(
                                release_flag,
                                "current_utc_timestamp",
                                return_value=CREATED_AT,
                            ):
                                release_flag.main()
                        finally:
                            os.chdir(old_directory)
            return stdout.getvalue()

    def test_flag_bytes_are_deterministic_and_canonicalize_utc(self):
        digest = "b" * 64
        expected = ("{\n"
                    '  "schema": "estate.release-flag.v1",\n'
                    '  "component": "coronatio",\n'
                    f'  "source_sha": "{SHA}",\n'
                    f'  "sha256": "{digest}",\n'
                    '  "flagged_at": "2026-09-04T12:34:56Z",\n'
                    f'  "pipeline_url": "{PIPELINE_URL}"\n'
                    "}\n").encode()
        first = release_flag.flag_bytes(SHA, digest, "2026-09-04T12:34:56+00:00", PIPELINE_URL)
        second = release_flag.flag_bytes(SHA, digest, "2026-09-04T12:34:56+00:00", PIPELINE_URL)
        self.assertEqual(first, expected)
        self.assertEqual(first, second)

    def test_upload_occurs_only_after_verified_release_assets(self):
        binary = b"local-binary"
        fake = FakeForgejo(binary)
        output = self.run_main(fake, binary)
        post_indexes = [index for index, call in enumerate(fake.calls) if call["method"] == "POST"]
        self.assertEqual(len(post_indexes), 1)
        upload_index = post_indexes[0]
        self.assertTrue(all(call["method"] == "GET" for call in fake.calls[:upload_index]))
        upload = fake.calls[upload_index]
        self.assertEqual(upload["content_type"], "application/json")
        self.assertEqual(upload["body"], release_flag.flag_bytes(
            SHA, hashlib.sha256(binary).hexdigest(), CREATED_AT, PIPELINE_URL,
        ))
        self.assertIn('"status":"flagged"', output)

    def test_identical_existing_flag_is_noop_without_post(self):
        binary = b"local-binary"
        flag = release_flag.flag_bytes(
            SHA, hashlib.sha256(binary).hexdigest(), CREATED_AT, PIPELINE_URL,
        )
        fake = FakeForgejo(binary, flag=flag)
        output = self.run_main(fake, binary)
        self.assertFalse(any(call["method"] == "POST" for call in fake.calls))
        self.assertIn('"status":"no-op"', output)

    def test_differing_existing_flag_fails_without_overwrite(self):
        binary = b"local-binary"
        fake = FakeForgejo(binary, flag=b"different")
        with self.assertRaises(SystemExit):
            self.run_main(fake, binary)
        self.assertFalse(any(call["method"] == "POST" for call in fake.calls))

    def test_extra_key_existing_flag_fails_without_post(self):
        binary = b"local-binary"
        flag = release_flag.flag_bytes(
            SHA, hashlib.sha256(binary).hexdigest(), CREATED_AT, PIPELINE_URL,
        )
        payload = json.loads(flag)
        payload["extra"] = "not-allowed"
        invalid_flag = (json.dumps(payload, indent=2) + "\n").encode()
        fake = FakeForgejo(binary, flag=invalid_flag)
        with self.assertRaises(SystemExit):
            self.run_main(fake, binary)
        self.assertFalse(any(call["method"] == "POST" for call in fake.calls))

    def test_non_200_release_get_fails(self):
        binary = b"local-binary"
        fake = FakeForgejo(binary, tag_status=500)
        with self.assertRaises(SystemExit):
            self.run_main(fake, binary)
        self.assertEqual([call["method"] for call in fake.calls], ["GET"])

    def test_409_race_identical_flag_succeeds(self):
        binary = b"local-binary"
        flag = release_flag.flag_bytes(
            SHA, hashlib.sha256(binary).hexdigest(), CREATED_AT, PIPELINE_URL,
        )
        fake = FakeForgejo(binary, upload_status=409, race_flag=flag)
        output = self.run_main(fake, binary)
        self.assertEqual(sum(call["method"] == "POST" for call in fake.calls), 1)
        self.assertIn('"status":"no-op"', output)

    def test_409_race_differing_flag_fails(self):
        binary = b"local-binary"
        fake = FakeForgejo(binary, upload_status=409, race_flag=b"different")
        with self.assertRaises(SystemExit):
            self.run_main(fake, binary)
        self.assertEqual(sum(call["method"] == "POST" for call in fake.calls), 1)

    def test_digest_mismatch_blocks_flag_upload(self):
        fake = FakeForgejo(b"published-binary")
        with self.assertRaises(SystemExit):
            self.run_main(fake, b"local-binary")
        self.assertFalse(any(call["method"] == "POST" for call in fake.calls))


if __name__ == "__main__":
    unittest.main()
