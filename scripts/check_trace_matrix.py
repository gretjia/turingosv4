#!/usr/bin/env python3
"""TRACE_MATRIX backlink enforcement (R-022) + reverse-map populator (CO1.13.3).

Per spec handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md v1.1.1
§ 1.2 + § 1.3 + § 2.1 + § 2.2.

Modes:
  --mode commit      Pre-commit: diff git index against HEAD for NEW pub items.
                     BLOCK on missing /// TRACE_MATRIX backlink unless § J
                     orphan or commit-msg [R-022-skip:] token justifies.
  --mode ci          PR-style: diff against origin/main (or --base-ref).
                     Catches PRs that bypassed install_hooks.sh.
  --mode reverse-map Shared parser: walk src/*.rs and emit (file, line,
                     symbol, trace_text) tuples to stdout. CO1.13.3
                     (update_trace_matrix_reverse_map.py) consumes.

Invariants enforced (per spec § 2.4):
  I-FORWARD     R-022 fires on NEW pub symbols only (not legacy)
  I-REMOVAL     R-022 also blocks REMOVAL of existing /// TRACE_MATRIX lines
  I-SCOPE       Scope table per § 1.3 honored
  I-LOG         Every event (PASS/BLOCK/SKIP) appends structured log entry
"""
from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path

# ─────────────────────────── constants ────────────────────────────────
PROJECT_ROOT = Path(__file__).resolve().parent.parent
ENFORCEMENT_LOG = PROJECT_ROOT / "rules" / "enforcement.log"
TRACE_MATRIX_DOC = (
    PROJECT_ROOT / "handover" / "alignment" / "TRACE_MATRIX_v3_2026-04-27.md"
)
SRC_PREFIX = "src/"
SKIP_TOKEN_RE = re.compile(
    r"\[R-022-skip:\s*(.+?)\]", re.DOTALL
)
JUSTIFICATION_RE = re.compile(
    r"(cases/C-?\w+|PREREG-§[\w.]+|OBS_R022_[\w-]+\.md)"
)
PUB_BLOCK_RE = re.compile(
    r"^\s*pub(?:\([^)]*\))?\s+(?:async\s+|unsafe\s+|extern\s+\"\w+\"\s+)*"
    r"(fn|struct|enum|trait|const|mod|type|static)\s+(\w+)"
)
PUB_USE_RE = re.compile(
    r"^\s*pub(?:\([^)]*\))?\s+use\b"
)
TRACE_DOC_RE = re.compile(r"^\s*///\s*TRACE_MATRIX\s+(.+)$")
DOC_ATTR_RE = re.compile(r"^\s*(///|//|#\[)|^\s*$")


@dataclass
class PubItem:
    file: str
    line: int  # 1-indexed line in NEW file content
    kind: str  # fn/struct/enum/trait/const/mod/type/static
    name: str
    is_pub_crate: bool
    in_cfg_test: bool


# ───────────────────────── git helpers ────────────────────────────────
def run(cmd: list[str], cwd: Path = PROJECT_ROOT, check: bool = False) -> str:
    res = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=check)
    return res.stdout


def git_diff(mode: str, base_ref: str | None) -> str:
    if mode == "commit":
        return run(["git", "diff", "--cached", "--unified=0", "--", SRC_PREFIX])
    if mode == "ci":
        ref = base_ref or "origin/main"
        return run(
            ["git", "diff", f"{ref}...HEAD", "--unified=0", "--", SRC_PREFIX]
        )
    raise ValueError(f"unknown mode {mode}")


def staged_file_content(path: str, mode: str, base_ref: str | None) -> list[str]:
    """Return the AFTER-state of a file (staged or HEAD) as a list of lines."""
    if mode == "commit":
        out = run(["git", "show", f":{path}"])
    else:
        out = run(["git", "show", f"HEAD:{path}"])
    return out.splitlines()


def staged_tree_hash(mode: str) -> str:
    if mode == "commit":
        return run(["git", "write-tree"]).strip() or "uncommitted"
    return run(["git", "rev-parse", "HEAD"]).strip()


def commit_message(mode: str) -> str:
    if mode == "commit":
        # Test override takes precedence so integration tests can supply a message
        # without running an actual `git commit` (the message is only written to
        # COMMIT_EDITMSG by git's own commit driver).
        env_msg = os.environ.get("GIT_COMMIT_MSG", "")
        if env_msg:
            return env_msg
        msg_file = PROJECT_ROOT / ".git" / "COMMIT_EDITMSG"
        if msg_file.exists():
            return msg_file.read_text(errors="replace")
        return ""
    return run(["git", "log", "-1", "--pretty=%B"])


# ─────────────────────── diff parsing ─────────────────────────────────
def parse_added_lines(diff: str) -> list[tuple[str, int, str]]:
    """Yield (file, new_line_number, line_text) for each '+' added line."""
    out: list[tuple[str, int, str]] = []
    cur_file = None
    new_ln = 0
    for line in diff.splitlines():
        if line.startswith("+++ b/"):
            cur_file = line[6:]
        elif line.startswith("@@"):
            m = re.search(r"\+(\d+)(?:,(\d+))?", line)
            if m:
                new_ln = int(m.group(1))
        elif line.startswith("+") and not line.startswith("+++"):
            if cur_file:
                out.append((cur_file, new_ln, line[1:]))
            new_ln += 1
        elif not line.startswith("-") and not line.startswith("\\"):
            new_ln += 1
    return out


def parse_removed_trace_lines(diff: str) -> list[tuple[str, str]]:
    """Yield (file, removed_line_text) for each '-' line that contained '/// TRACE_MATRIX '."""
    out: list[tuple[str, str]] = []
    cur_file = None
    for line in diff.splitlines():
        if line.startswith("--- a/"):
            cur_file = line[6:]
        elif line.startswith("-") and not line.startswith("---"):
            text = line[1:]
            if "/// TRACE_MATRIX " in text and cur_file:
                out.append((cur_file, text))
    return out


# ─────────────────────── scope-table classifier ────────────────────────
def classify_added_pub(file_lines: list[str], idx0: int) -> PubItem | None:
    """Given full file lines and 0-indexed line index of an added line,
    return PubItem if it declares a pub-style item subject to R-022 BLOCK."""
    if idx0 < 0 or idx0 >= len(file_lines):
        return None
    line = file_lines[idx0]
    if PUB_USE_RE.match(line):
        return None  # EXEMPT per § 1.3
    m = PUB_BLOCK_RE.match(line)
    if not m:
        return None
    is_pub_crate = "pub(crate)" in line
    # detect cfg(test) by walking up enclosing mod
    in_test = enclosing_cfg_test(file_lines, idx0)
    if in_test:
        return None  # EXEMPT per § 1.3
    return PubItem(
        file="",  # filled by caller
        line=idx0 + 1,
        kind=m.group(1),
        name=m.group(2),
        is_pub_crate=is_pub_crate,
        in_cfg_test=False,
    )


def enclosing_cfg_test(lines: list[str], idx0: int) -> bool:
    """Heuristic: walk back to find an enclosing `#[cfg(test)] mod tests {`."""
    depth = 0
    for i in range(idx0, -1, -1):
        ln = lines[i]
        depth += ln.count("}") - ln.count("{")
        if depth < 0 and re.search(r"#\[cfg\(test\)\]", lines[max(0, i - 5) : i + 1].__str__()):
            return True
    return False


# ─────────────────────── backlink walk ─────────────────────────────────
def has_trace_backlink(lines: list[str], pub_idx0: int) -> bool:
    """Walk back through contiguous doc/attribute/comment/blank lines from
    pub_idx0-1; return True if any line is `/// TRACE_MATRIX ...`."""
    i = pub_idx0 - 1
    while i >= 0:
        if not DOC_ATTR_RE.match(lines[i]):
            break
        if TRACE_DOC_RE.match(lines[i]):
            return True
        i -= 1
    return False


# ─────────────────────── § J orphan lookup ─────────────────────────────
def orphan_lookup(file: str, symbol: str) -> tuple[bool, str]:
    """Search TRACE_MATRIX_v3.md § J.2 for a row matching (file, symbol).
    Returns (found, justification_ref or '')."""
    if not TRACE_MATRIX_DOC.exists():
        return (False, "")
    text = TRACE_MATRIX_DOC.read_text(errors="replace")
    m = re.search(r"### § J\.2 .*?(?=^### |^## |\Z)", text, re.DOTALL | re.MULTILINE)
    if not m:
        return (False, "")
    block = m.group(0)
    # Look for `| <file> | <symbol> | ... | <justification_ref> |`
    pattern = re.compile(
        r"^\|\s*" + re.escape(file) + r"\s*\|\s*" + re.escape(symbol) + r"\b.*?$",
        re.MULTILINE,
    )
    rm = pattern.search(block)
    if not rm:
        return (False, "")
    cells = [c.strip() for c in rm.group(0).strip("|").split("|")]
    # schema: File | Symbol | Class | Justification ref | Opened | Graduation | Notes
    if len(cells) < 4:
        return (False, "")
    just = cells[3]
    if not JUSTIFICATION_RE.search(just):
        return (False, "")
    if not justification_ref_exists(just):
        return (False, "")
    return (True, just)


def justification_ref_exists(ref: str) -> bool:
    """Validate that a justification reference resolves in repo."""
    m = JUSTIFICATION_RE.search(ref)
    if not m:
        return False
    token = m.group(1)
    if token.startswith("cases/"):
        # cases/C-xxx → cases/C-xxx.yaml (may be flat or nested)
        candidate = PROJECT_ROOT / f"{token}.yaml"
        if candidate.exists():
            return True
        return any(PROJECT_ROOT.glob(f"{token}.yaml"))
    if token.startswith("PREREG-§"):
        # Symbolic reference to PREREG section; accept as-is (caller responsible)
        return True
    if token.startswith("OBS_R022_") and token.endswith(".md"):
        # OBS_R022_*.md must live under handover/alignment/
        return any((PROJECT_ROOT / "handover" / "alignment").glob(token))
    return False


# ─────────────────────── skip-token validator ──────────────────────────
def skip_token_justification(msg: str) -> tuple[bool, str]:
    """Return (justified, reason) for the first valid [R-022-skip:] token."""
    m = SKIP_TOKEN_RE.search(msg)
    if not m:
        return (False, "")
    reason = m.group(1).strip()
    j = JUSTIFICATION_RE.search(reason)
    if not j:
        return (False, reason)
    if not justification_ref_exists(j.group(0)):
        return (False, reason)
    return (True, reason)


# ─────────────────────── structured logging ───────────────────────────
def log_event(
    verdict: str,
    file: str,
    line: int,
    symbol: str,
    reason: str,
    tree_hash: str,
) -> None:
    ts = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    entry = (
        f'{ts} R-022-{verdict} commit={tree_hash} file={file} line={line} '
        f'symbol={symbol} reason="{reason}"\n'
    )
    ENFORCEMENT_LOG.parent.mkdir(parents=True, exist_ok=True)
    with ENFORCEMENT_LOG.open("a") as f:
        f.write(entry)


# ─────────────────────── modes ────────────────────────────────────────
def mode_check(mode: str, base_ref: str | None) -> int:
    diff = git_diff(mode, base_ref)
    added = parse_added_lines(diff)
    removed_traces = parse_removed_trace_lines(diff)

    msg = commit_message(mode)
    skip_ok, skip_reason = skip_token_justification(msg)
    tree_hash = staged_tree_hash(mode)

    violations: list[str] = []

    # Group adds by file → walk file content once per file
    by_file: dict[str, list[int]] = {}
    for path, ln, _ in added:
        if not path.startswith(SRC_PREFIX):
            continue
        by_file.setdefault(path, []).append(ln)

    for path, line_nums in by_file.items():
        try:
            file_lines = staged_file_content(path, mode, base_ref)
        except subprocess.CalledProcessError:
            continue
        for ln in sorted(set(line_nums)):
            idx0 = ln - 1
            if idx0 >= len(file_lines):
                continue
            item = classify_added_pub(file_lines, idx0)
            if item is None:
                continue
            item.file = path
            symbol = f"pub_{item.kind}_{item.name}"
            if has_trace_backlink(file_lines, idx0):
                log_event("PASS", path, ln, symbol, "doc-block backlink found", tree_hash)
                continue
            found, just = orphan_lookup(path, item.name)
            if found:
                log_event(
                    "PASS", path, ln, symbol, f"§ J orphan ({just})", tree_hash
                )
                continue
            if skip_ok:
                log_event("SKIP", path, ln, symbol, skip_reason, tree_hash)
                continue
            log_event(
                "BLOCK",
                path,
                ln,
                symbol,
                "missing TRACE_MATRIX backlink + no § J + no skip-token",
                tree_hash,
            )
            violations.append(
                f"  {path}:{ln} pub {item.kind} {item.name} — missing /// TRACE_MATRIX backlink"
            )

    # Removal detection (§ 2.1 step + I-REMOVAL)
    for path, removed_line in removed_traces:
        if not path.startswith(SRC_PREFIX):
            continue
        if skip_ok:
            log_event("SKIP", path, 0, "trace_removal", skip_reason, tree_hash)
            continue
        log_event(
            "BLOCK",
            path,
            0,
            "trace_removal",
            f"removed: {removed_line.strip()[:80]}",
            tree_hash,
        )
        violations.append(
            f"  {path} — removed TRACE_MATRIX backlink: {removed_line.strip()[:80]}"
        )

    if violations:
        sys.stderr.write("BLOCKED by R-022 (TRACE_MATRIX pub-symbol-block):\n")
        for v in violations:
            sys.stderr.write(v + "\n")
        sys.stderr.write(
            "\nRemediation: add /// TRACE_MATRIX <FC-id>: <role> doc-comment, "
            "register in TRACE_MATRIX_v3.md § J with justification ref, or "
            "include [R-022-skip: <reason; cases/Cxxx | PREREG-§n.m | "
            "OBS_R022_*.md REQUIRED>] token in commit message.\n"
        )
        return 2
    return 0


def mode_reverse_map() -> int:
    """Walk src/*.rs; emit one TSV row per /// TRACE_MATRIX <text> + symbol pair.
    Format: <file>\t<line>\t<symbol>\t<trace_text>"""
    src = PROJECT_ROOT / "src"
    for f in sorted(src.rglob("*.rs")):
        rel = f.relative_to(PROJECT_ROOT).as_posix()
        lines = f.read_text(errors="replace").splitlines()
        for i, line in enumerate(lines):
            tm = TRACE_DOC_RE.match(line)
            if not tm:
                continue
            trace_text = tm.group(1).rstrip(".")
            j = i + 1
            sym = "(opaque)"
            while j < len(lines):
                if not DOC_ATTR_RE.match(lines[j]):
                    break
                j += 1
            if j < len(lines):
                m = PUB_BLOCK_RE.match(lines[j])
                if m:
                    sym = f"pub {m.group(1)} {m.group(2)}"
                else:
                    m2 = re.match(r"^\s*(\w+)\b", lines[j])
                    if m2:
                        sym = m2.group(1)
            print(f"{rel}\t{i + 1}\t{sym}\t{trace_text}")
    return 0


# ─────────────────────── entry point ──────────────────────────────────
def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--mode", required=True, choices=["commit", "ci", "reverse-map"])
    p.add_argument("--base-ref", default=None, help="for --mode ci (default origin/main)")
    args = p.parse_args()
    if args.mode == "reverse-map":
        return mode_reverse_map()
    return mode_check(args.mode, args.base_ref)


if __name__ == "__main__":
    sys.exit(main())
