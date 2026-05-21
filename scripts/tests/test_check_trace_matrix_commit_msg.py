#!/usr/bin/env python3
"""R022_HOOK_FIX_2026-05-22 — unit + integration tests for the commit-msg
hook path of scripts/check_trace_matrix.py.

Pins the fix for the footgun where `git commit -m`/`-F` leaves
.git/COMMIT_EDITMSG STALE (previous commit's text) when read from a
pre-commit hook. The fix:

  1. check_trace_matrix.py grew a --message-file <path> flag with priority
     order: --message-file > GIT_COMMIT_MSG env > .git/COMMIT_EDITMSG.
  2. The R-022 check moved from pre-commit to commit-msg, which receives
     the in-flight message file path as $1.

Tests:
  - commit_message() honors --message-file when it exists
  - commit_message() priority: --message-file > GIT_COMMIT_MSG > COMMIT_EDITMSG
  - End-to-end: git commit -F file with valid [R-022-skip:] token → PASS
  - End-to-end: same removal without skip token → BLOCK

Run: `python3 scripts/tests/test_check_trace_matrix_commit_msg.py`
"""
import importlib.util
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
SCRIPT = ROOT / "scripts" / "check_trace_matrix.py"

spec = importlib.util.spec_from_file_location("check_trace_matrix", SCRIPT)
ctm = importlib.util.module_from_spec(spec)
sys.modules["check_trace_matrix"] = ctm
spec.loader.exec_module(ctm)


class CommitMessageUnitTest(unittest.TestCase):
    """Pure-function tests of commit_message() priority ordering."""

    def setUp(self):
        self._saved_env = os.environ.pop("GIT_COMMIT_MSG", None)

    def tearDown(self):
        if self._saved_env is not None:
            os.environ["GIT_COMMIT_MSG"] = self._saved_env
        else:
            os.environ.pop("GIT_COMMIT_MSG", None)

    def test_message_file_takes_priority(self):
        with tempfile.NamedTemporaryFile("w", suffix=".txt", delete=False) as f:
            f.write("from message file [R-022-skip: cases/Cxxx]")
            mf = f.name
        try:
            os.environ["GIT_COMMIT_MSG"] = "from env (should lose)"
            msg = ctm.commit_message("commit", message_file=mf)
            self.assertIn("from message file", msg)
            self.assertNotIn("from env", msg)
        finally:
            os.unlink(mf)

    def test_env_used_when_no_message_file(self):
        os.environ["GIT_COMMIT_MSG"] = "from env"
        msg = ctm.commit_message("commit", message_file=None)
        self.assertEqual(msg, "from env")

    def test_message_file_missing_falls_through_to_env(self):
        os.environ["GIT_COMMIT_MSG"] = "env fallback"
        msg = ctm.commit_message("commit", message_file="/nonexistent/path/xyz")
        self.assertEqual(msg, "env fallback")


class CommitMsgHookEndToEndTest(unittest.TestCase):
    """Spin up a throwaway git repo, stage a /// TRACE_MATRIX REMOVAL, and
    verify the --message-file flag drives the R-022 verdict end-to-end.

    Runs mode_check() in-process with PROJECT_ROOT + ENFORCEMENT_LOG
    monkey-patched at the temp repo. Avoids polluting the real repo's
    enforcement.log."""

    def setUp(self):
        self.tmp = tempfile.mkdtemp(prefix="r022_hook_test_")
        self.repo = Path(self.tmp) / "repo"
        self.repo.mkdir()
        self._git("init", "-q", "-b", "main")
        self._git("config", "user.email", "test@example.com")
        self._git("config", "user.name", "test")
        self._git("config", "commit.gpgsign", "false")

        # Seed: src/lib.rs with a /// TRACE_MATRIX backlink on a pub mod.
        src = self.repo / "src"
        src.mkdir()
        seed = (
            "/// TRACE_MATRIX FC1-N4: leaf module.\n"
            "pub mod leaf;\n"
        )
        (src / "lib.rs").write_text(seed)
        (src / "leaf.rs").write_text("pub fn noop() {}\n")
        self._git("add", "src/lib.rs", "src/leaf.rs")
        self._git("commit", "-q", "-m", "seed")

        # Redirect script globals at the temp repo so git ops + ref
        # resolution operate on it (instead of the real project root).
        self._saved_root = ctm.PROJECT_ROOT
        self._saved_log = ctm.ENFORCEMENT_LOG
        ctm.PROJECT_ROOT = self.repo
        ctm.ENFORCEMENT_LOG = self.repo / "rules" / "enforcement.log"

        # GIT_COMMIT_MSG must not leak in from the parent environment, or
        # it would mask --message-file in the priority chain.
        self._saved_env = os.environ.pop("GIT_COMMIT_MSG", None)

    def tearDown(self):
        ctm.PROJECT_ROOT = self._saved_root
        ctm.ENFORCEMENT_LOG = self._saved_log
        if self._saved_env is not None:
            os.environ["GIT_COMMIT_MSG"] = self._saved_env
        import shutil

        shutil.rmtree(self.tmp, ignore_errors=True)

    def _git(self, *args):
        subprocess.run(
            ["git", *args], cwd=self.repo, check=True, capture_output=True
        )

    def _run_check(self, message_file):
        """Call mode_check() in-process; capture stderr."""
        import contextlib
        import io

        buf = io.StringIO()
        with contextlib.redirect_stderr(buf):
            rc = ctm.mode_check("commit", None, message_file)
        return rc, buf.getvalue()

    def _stage_removal(self):
        """Stage the deletion of the /// TRACE_MATRIX backlink + the pub mod
        it annotated. Mirrors the R1 hotfix shape (removed pub mod
        external_market_snapshot;)."""
        (self.repo / "src" / "lib.rs").write_text("// (mod removed)\n")
        self._git("add", "src/lib.rs")

    def _write_msg(self, body):
        msg_path = Path(self.tmp) / "msg.txt"
        msg_path.write_text(body)
        return str(msg_path)

    def _seed_obs_doc(self, name):
        """Skip-token refs resolve via justification_ref_exists() which
        looks under <PROJECT_ROOT>/handover/alignment/. Seed a sentinel."""
        obs = self.repo / "handover" / "alignment" / name
        obs.parent.mkdir(parents=True, exist_ok=True)
        obs.write_text("# test sentinel\n")
        return obs

    def test_removal_with_valid_skip_token_passes(self):
        self._seed_obs_doc("OBS_R022_TEST_HOOK_FIX_SENTINEL.md")
        self._stage_removal()
        msg = self._write_msg(
            "test removal [R-022-skip: OBS_R022_TEST_HOOK_FIX_SENTINEL.md]\n"
        )
        rc, stderr = self._run_check(msg)
        self.assertEqual(rc, 0, msg=f"expected PASS, got rc={rc} stderr={stderr}")

    def test_removal_without_skip_token_blocks(self):
        self._stage_removal()
        msg = self._write_msg("plain removal, no skip token\n")
        rc, stderr = self._run_check(msg)
        self.assertEqual(rc, 2, msg=f"expected BLOCK (rc=2), got rc={rc}")
        self.assertIn("R-022", stderr)

    def test_removal_with_invalid_skip_ref_blocks(self):
        """A skip token whose justification ref doesn't resolve must NOT
        bypass the block — guards against stale OBS filenames."""
        self._stage_removal()
        msg = self._write_msg(
            "removal [R-022-skip: OBS_R022_DOES_NOT_EXIST.md]\n"
        )
        rc, _ = self._run_check(msg)
        self.assertEqual(rc, 2)


if __name__ == "__main__":
    unittest.main()
