import importlib.util
import io
import json
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
INSTALLER = ROOT / "coronatio-install.py"
NEW, OLD, TOKEN = "a" * 40, "b" * 40, "1" * 16


def load_installer():
    spec = importlib.util.spec_from_file_location("coronatio_installer_fresh", INSTALLER)
    module = importlib.util.module_from_spec(spec); assert spec and spec.loader; spec.loader.exec_module(module); return module


class Commands:
    def __init__(self): self.calls = []
    def __call__(self, cmd, *, cwd=None, env=None): self.calls.append(tuple(cmd))


class CrashRecoveryTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(); self.root = Path(self.tmp.name)
        self.runtime = self.root / "opt" / "coronatio"; self.runtime.mkdir(parents=True)
        self.binary = self.root / "usr" / "local" / "bin" / "coronatio"; self.binary.parent.mkdir(parents=True)

    def tearDown(self): self.tmp.cleanup()

    def old_release(self, m):
        source = self.runtime / "source" / "static"; source.mkdir(parents=True, exist_ok=True)
        (source / "crown.css").write_text("old"); (self.runtime / "source" / "runtime.rs").write_text("old-runtime")
        (self.runtime / "source" / m.SOURCE_SHA_NAME).write_text(OLD + "\n")
        self.binary.write_text("old-bin"); self.binary.chmod(0o755)
        (self.runtime / m.BINARY_SHA_NAME).write_text(OLD + "\n")
        (self.runtime / m.MANIFEST_NAME).write_text(json.dumps({"source_sha": OLD, "source_tree_sha256":"old", "static_sha256":"old", "binary_sha256":"old"}))

    def tx(self, m, state):
        p = m.expected_paths(self.runtime, self.binary, TOKEN)
        old_digests = {"source":m.artifact_digest("source", self.runtime / "source"), "binary":m.artifact_digest("binary", self.binary), "manifest":m.artifact_digest("manifest", self.runtime / m.MANIFEST_NAME), "binary_sha":m.artifact_digest("binary_sha", self.runtime / m.BINARY_SHA_NAME)}
        return {"version":2, "token":TOKEN, "state":state, "new_sha":NEW, **{k:str(v) for k,v in p.items()}, "old":{"source":True,"binary":True,"manifest":True,"binary_sha":True}, "old_digests":old_digests}

    def partial(self, m, state):
        self.old_release(m); tx = self.tx(m, state); p = m.expected_paths(self.runtime, self.binary, TOKEN)
        stage = p["source_stage"].parent; (stage / "source" / "static").mkdir(parents=True); (stage / "source" / "static" / "crown.css").write_text("new")
        p["binary_stage"].write_text("new-bin"); p["binary_stage"].chmod(0o755)
        ordered = [("old_source_moved", self.runtime / "source", p["source_backup"]), ("new_source_moved", p["source_stage"], self.runtime / "source"), ("old_binary_moved", self.binary, p["binary_backup"]), ("new_binary_moved", p["binary_stage"], self.binary), ("old_manifest_moved", self.runtime / m.MANIFEST_NAME, p["manifest_backup"]), ("old_binary_sha_moved", self.runtime / m.BINARY_SHA_NAME, p["binary_sha_backup"])]
        states = ["prepared", "stopped", "old_source_moved", "new_source_moved", "old_binary_moved", "new_binary_moved", "old_manifest_moved", "new_manifest_written", "old_binary_sha_moved", "witness_updated", "service_started", "health_verified", "committed"]
        for marker, src, dst in ordered:
            if states.index(state) >= states.index(marker):
                if src.exists(): m.atomic_replace(src, dst)
        if states.index(state) >= states.index("new_manifest_written"): (self.runtime / m.MANIFEST_NAME).write_text(json.dumps({"source_sha":NEW,"source_tree_sha256":"new","static_sha256":"new","binary_sha256":"new"}))
        if states.index(state) >= states.index("witness_updated"):
            (self.runtime / m.BINARY_SHA_NAME).write_text(NEW + "\n")
            if (self.runtime / "source").exists(): (self.runtime / "source" / m.SOURCE_SHA_NAME).write_text(NEW + "\n")
        m.durable_json(self.runtime / m.JOURNAL_NAME, tx)

    def assert_old(self, m):
        self.assertEqual((self.runtime / "source" / "static" / "crown.css").read_text(), "old")
        self.assertEqual((self.runtime / "source" / "runtime.rs").read_text(), "old-runtime")
        self.assertEqual(self.binary.read_text(), "old-bin")
        self.assertEqual(m.read_sha(self.runtime / "source" / m.SOURCE_SHA_NAME), OLD)

    def test_actual_filesystem_shape_at_every_forward_journal_boundary_recovers_idempotently(self):
        m = load_installer()
        for state in sorted(m.FORWARD_STATES - {"committed"}):
            with self.subTest(state=state), tempfile.TemporaryDirectory() as isolated:
                self.tmp.cleanup(); self.tmp = tempfile.TemporaryDirectory(dir=isolated); self.root = Path(self.tmp.name)
                self.runtime = self.root / "opt" / "coronatio"; self.runtime.mkdir(parents=True); self.binary = self.root / "usr" / "local" / "bin" / "coronatio"; self.binary.parent.mkdir(parents=True)
                self.partial(m, state); calls = Commands()
                with patch.object(m.subprocess, "run"), patch.object(m, "run", calls): self.assertTrue(load_installer().recover_transaction(self.runtime, self.binary, restart=False))
                self.assert_old(m); self.assertFalse((self.runtime / m.JOURNAL_NAME).exists())
                with patch.object(m, "run", calls): self.assertFalse(load_installer().recover_transaction(self.runtime, self.binary, restart=False))

    def test_prepared_and_stopped_have_no_backups_and_preserve_every_destination(self):
        m = load_installer()
        for state in ("prepared", "stopped"):
            self.old_release(m); p = m.expected_paths(self.runtime, self.binary, TOKEN); m.durable_json(self.runtime / m.JOURNAL_NAME, self.tx(m, state))
            with patch.object(m, "run", Commands()): m.recover_transaction(self.runtime, self.binary, restart=False)
            self.assert_old(m); self.assertFalse(any(x.exists() for x in p.values()))

    def test_malformed_incomplete_and_path_escaping_journal_fail_closed(self):
        m = load_installer(); self.old_release(m)
        for tx in ({}, {"version":2}, {**self.tx(m,"prepared"), "source_stage":"/tmp/escape"}):
            m.durable_json(self.runtime / m.JOURNAL_NAME, tx)
            with self.assertRaises(RuntimeError): m.recover_transaction(self.runtime, self.binary, restart=False)
            self.assertTrue((self.runtime / m.JOURNAL_NAME).exists()); self.assert_old(m)

    def test_source_tree_digest_detects_nonstatic_loss_and_binary_requires_regular_executable(self):
        m = load_installer(); self.old_release(m); source = self.runtime / "source"
        manifest = {"source_sha":OLD,"source_tree_sha256":m.source_tree_digest(source),"static_sha256":m.digest_tree(source / "static"),"binary_sha256":m.digest_file(self.binary, executable=True)}
        (self.runtime / m.MANIFEST_NAME).write_text(json.dumps(manifest)); self.assertTrue(m.current_matches(self.runtime, self.binary, OLD))
        (source / "runtime.rs").unlink(); self.assertFalse(m.current_matches(self.runtime, self.binary, OLD))
        (source / "runtime.rs").write_text("old-runtime"); self.binary.chmod(0o644); self.assertFalse(m.current_matches(self.runtime, self.binary, OLD))

    def test_tree_hash_is_deterministic_and_refuses_symlinks(self):
        m = load_installer(); tree = self.root / "tree"; (tree / "nested").mkdir(parents=True)
        (tree / "nested" / "asset.css").write_text("body{}")
        first = m.digest_tree(tree); self.assertEqual(first, m.digest_tree(tree)); self.assertIsNotNone(first)
        (tree / "link").symlink_to(tree / "nested" / "asset.css")
        self.assertIsNone(m.digest_tree(tree))

    def test_missing_parent_is_created_or_unsafe_parent_refused(self):
        m = load_installer(); runtime = self.root / "missing" / "coronatio"; binary = self.root / "bin" / "coronatio"
        # Unit roots are not root-owned, so the security predicate refuses them before staging.
        with self.assertRaises(RuntimeError): m.ensure_runtime_parents(runtime, binary)
        unsafe = self.root / "unsafe"; unsafe.write_text("not a directory")
        with self.assertRaises(RuntimeError): m.ensure_runtime_parents(unsafe / "coronatio", binary)

    def test_health_requires_typed_exact_sha(self):
        m = load_installer()
        class Response:
            status = 200
            def __init__(self, body): self.body = body
            def read(self): return self.body
            def __enter__(self): return self
            def __exit__(self, *args): return False
        good = json.dumps({"ok":True,"schema":"coronatio.health.v1","service":"coronatio","source_sha":NEW,"build_sha":NEW}).encode()
        with patch.object(m.urllib.request, "urlopen", return_value=Response(good)): self.assertTrue(m.probe_health("http://test", NEW, 1)[0])
        for body in (b"{}", json.dumps({"ok":True,"schema":"bad","service":"coronatio","source_sha":NEW,"build_sha":NEW}).encode(), json.dumps({"ok":True,"schema":"coronatio.health.v1","service":"coronatio","source_sha":OLD,"build_sha":NEW}).encode(), json.dumps({"ok":True,"schema":"coronatio.health.v1","service":"coronatio","source_sha":NEW}).encode()):
            with patch.object(m.urllib.request, "urlopen", return_value=Response(body)): self.assertFalse(m.probe_health("http://test", NEW, 1)[0])

    def test_committed_cleanup_removes_every_declared_artifact(self):
        m = load_installer(); self.old_release(m); tx = self.tx(m, "committed"); p = m.expected_paths(self.runtime, self.binary, TOKEN)
        for path in p.values(): path.parent.mkdir(parents=True, exist_ok=True); path.write_text("x")
        m.durable_json(self.runtime / m.JOURNAL_NAME, tx); m.recover_transaction(self.runtime, self.binary, restart=False)
        self.assertFalse((self.runtime / m.JOURNAL_NAME).exists()); self.assertFalse(any(x.exists() for x in p.values()))

    def test_absent_prior_release_is_restored_to_exact_absence_at_every_boundary(self):
        m = load_installer()
        for state in m.FORWARD_STATES - {"committed"}:
            with self.subTest(state=state), tempfile.TemporaryDirectory() as isolated:
                runtime = Path(isolated) / "opt" / "coronatio"; runtime.mkdir(parents=True)
                binary = Path(isolated) / "usr" / "local" / "bin" / "coronatio"; binary.parent.mkdir(parents=True)
                p = m.expected_paths(runtime, binary, TOKEN)
                (p["source_stage"].parent / "source").mkdir(parents=True)
                (p["source_stage"] / "new").write_text("candidate")
                p["binary_stage"].write_text("candidate"); p["binary_stage"].chmod(0o755)
                (runtime / "source").mkdir(); (runtime / "source" / "candidate").write_text("candidate")
                binary.write_text("candidate"); binary.chmod(0o755)
                (runtime / m.MANIFEST_NAME).write_text("candidate")
                (runtime / m.BINARY_SHA_NAME).write_text("candidate")
                tx = {"version": 2, "token": TOKEN, "state": state, "new_sha": NEW,
                      **{key: str(value) for key, value in p.items()},
                      "old": {"source": False, "binary": False, "manifest": False, "binary_sha": False}, "old_digests": {}}
                m.durable_json(runtime / m.JOURNAL_NAME, tx)
                with patch.object(m, "run", Commands()): self.assertTrue(m.recover_transaction(runtime, binary, restart=False))
                for path in (runtime / "source", binary, runtime / m.MANIFEST_NAME, runtime / m.BINARY_SHA_NAME):
                    self.assertFalse(path.exists(), f"{state}: {path} must return to prior absence")
                self.assertFalse((runtime / m.JOURNAL_NAME).exists())
                with patch.object(m, "run", Commands()): self.assertFalse(m.recover_transaction(runtime, binary, restart=False))

    def test_foreign_destination_at_rollback_seam_fails_closed_and_preserves_journal(self):
        m = load_installer(); self.old_release(m); p = m.expected_paths(self.runtime, self.binary, TOKEN)
        old_digest = m.artifact_digest("binary", self.binary)
        m.atomic_replace(self.binary, p["binary_backup"])
        self.binary.write_text("foreign"); self.binary.chmod(0o755)
        p["binary_backup"].unlink()
        tx = self.tx(m, "new_binary_moved"); tx["old_digests"] = {"binary": old_digest}
        m.durable_json(self.runtime / m.JOURNAL_NAME, tx)
        with patch.object(m, "run", Commands()), self.assertRaises(RuntimeError):
            m.recover_transaction(self.runtime, self.binary, restart=False)
        self.assertEqual(self.binary.read_text(), "foreign")
        self.assertTrue((self.runtime / m.JOURNAL_NAME).exists())

    def test_receipt_parser_accepts_actual_main_success_and_refuses_missing_malformed_or_mismatched(self):
        m = load_installer(); digest = "c" * 64
        converged = m.ConvergeResult("no-op", NEW, digest, digest, digest)
        output = io.StringIO()
        with patch.object(m, "source_head", return_value=NEW), patch.object(m, "ensure_runtime_parents"), patch.object(m.Path, "mkdir"), patch.object(m, "acquire_install_lock", return_value=7), patch.object(m, "release_install_lock"), patch.object(m, "converge", return_value=converged), patch("sys.stdout", output):
            self.assertEqual(m.main([]), 0)
        receipt = json.loads(output.getvalue())
        self.assertTrue(m.valid_success_receipt(receipt, NEW))
        for changed in ({"source_tree_sha256": None}, {"binary_sha256": "C" * 64}, {"source_sha": OLD}, {"status": "diagnostic"}):
            bad = dict(receipt); bad.update(changed)
            self.assertFalse(m.valid_success_receipt(bad, NEW))

    def test_fixed_lock_refuses_second_acquisition(self):
        m = load_installer(); lock_root = self.root / "lock"; lock_root.mkdir()
        class RootStat:
            st_mode = 0o100600
            st_uid = 0
        with patch.object(m.os, "fstat", return_value=RootStat()):
            first = m.acquire_install_lock(lock_root)
            try:
                with self.assertRaises(RuntimeError): m.acquire_install_lock(lock_root)
            finally:
                m.release_install_lock(first)
        self.assertTrue((lock_root / m.LOCK_NAME).is_file())

    def test_failure_receipt_preserves_known_hashes_and_actual_recovery_outcome(self):
        m = load_installer(); digest = "d" * 64; manifest = {"source_sha": NEW, "source_tree_sha256": digest, "static_sha256": digest, "binary_sha256": digest}
        for files, service in (("restored", "restored"), ("failed", "failed")):
            output = io.StringIO()
            failure = m.ConvergeFailure("health failed", source_sha=NEW, manifest=manifest, rollback_files=files, rollback_service=service, restored=files == "restored")
            with patch.object(m, "source_head", return_value=NEW), patch.object(m, "ensure_runtime_parents"), patch.object(m.Path, "mkdir"), patch.object(m, "acquire_install_lock", return_value=7), patch.object(m, "release_install_lock"), patch.object(m, "converge", side_effect=failure), patch("sys.stdout", output):
                self.assertEqual(m.main([]), 1)
            receipt = json.loads(output.getvalue())
            self.assertEqual(receipt["rollbackFiles"], files)
            self.assertEqual(receipt["rollbackService"], service)
            self.assertEqual(receipt["binary_sha256"], digest)

if __name__ == "__main__": unittest.main()
