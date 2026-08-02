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
        return {"version":2, "token":TOKEN, "state":state, "new_sha":NEW, **{k:str(v) for k,v in p.items()}, "old":{"source":True,"binary":True,"manifest":True,"binary_sha":True}}

    def partial(self, m, state):
        self.old_release(m); p = m.expected_paths(self.runtime, self.binary, TOKEN)
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
        m.durable_json(self.runtime / m.JOURNAL_NAME, self.tx(m, state))

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

if __name__ == "__main__": unittest.main()
