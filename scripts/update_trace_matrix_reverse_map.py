#!/usr/bin/env python3
"""CO1.13.3 — TRACE_MATRIX_v3 § F.2 reverse-map populator.

Per spec handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md v1.1.1 § 1.3:
shares its parser with CO1.13.2 `scripts/check_trace_matrix.py` (invoked as
`--mode reverse-map`) so a single source of truth controls how doc-comments
are extracted and which lines count.

Behavior:
- Default: refresh § F.2 in handover/alignment/TRACE_MATRIX_v3_2026-04-27.md.
- `--dry-run`: print rendered block to stdout; do not modify the file.
- `--check`: exit 1 if § F.2 in file differs from rendered output; useful for CI.

The script is idempotent: re-running with no source changes produces no diff.
"""
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DOC_PATH = PROJECT_ROOT / "handover" / "alignment" / "TRACE_MATRIX_v3_2026-04-27.md"
CHECK_SCRIPT = PROJECT_ROOT / "scripts" / "check_trace_matrix.py"

SECTION_HEADER = "### § F.2 Reverse-map snapshot"
SECTION_FOOTER = "— end of § F manual snapshot."


def collect_entries() -> list[tuple[str, int, str, str]]:
    """Invoke check_trace_matrix.py --mode reverse-map and parse TSV output."""
    if not CHECK_SCRIPT.exists():
        raise FileNotFoundError(CHECK_SCRIPT)
    out = subprocess.check_output(
        ["python3", str(CHECK_SCRIPT), "--mode", "reverse-map"],
        cwd=PROJECT_ROOT,
        text=True,
    )
    rows = []
    for line in out.splitlines():
        if not line.strip():
            continue
        parts = line.split("\t", 3)
        if len(parts) != 4:
            continue
        rel, ln, sym, trace = parts
        rows.append((rel, int(ln), sym, trace))
    return rows


def render_section(rows: list[tuple[str, int, str, str]]) -> str:
    head = subprocess.check_output(
        ["git", "rev-parse", "--short", "HEAD"], cwd=PROJECT_ROOT, text=True
    ).strip()
    by_file: dict[str, list[tuple[int, str, str]]] = defaultdict(list)
    for rel, ln, sym, trace in rows:
        by_file[rel].append((ln, sym, trace))

    lines = [f"{SECTION_HEADER} (HEAD `{head}`, auto-refreshed by CO1.13.3)"]
    for rel in sorted(by_file.keys()):
        lines.append("")
        lines.append(f"#### `{rel}`")
        lines.append("")
        lines.append("| Line | Symbol | TRACE_MATRIX backlink |")
        lines.append("|---:|---|---|")
        for ln, sym, trace in sorted(by_file[rel]):
            sym_md = f"`{sym}`" if not sym.startswith("(") else sym
            trace_md = trace.replace("|", r"\|")
            if len(trace_md) > 110:
                trace_md = trace_md[:107] + "..."
            lines.append(f"| {ln} | {sym_md} | {trace_md} |")
    lines.append("")
    lines.append(
        f"**Total**: {len(rows)} `///`-doc-comment backlinks across "
        f"{len(by_file)} source files (HEAD `{head}`). "
        f"Auto-refreshed by `scripts/update_trace_matrix_reverse_map.py` per CO1.13.3."
    )
    lines.append("")
    lines.append(SECTION_FOOTER)
    return "\n".join(lines)


def splice(doc_text: str, rendered: str) -> str:
    """Replace the § F.2 ... footer block in doc_text with `rendered`."""
    pattern = re.compile(
        r"^### § F\.2 .*?^" + re.escape(SECTION_FOOTER) + r"\s*$",
        re.MULTILINE | re.DOTALL,
    )
    if not pattern.search(doc_text):
        raise SystemExit(
            "TRACE_MATRIX_v3 doc missing § F.2 anchor or footer; "
            "ensure CO1.13.1 manual population is in place."
        )
    return pattern.sub(rendered, doc_text)


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--dry-run", action="store_true", help="print rendered block; do not modify file")
    p.add_argument("--check", action="store_true", help="exit 1 if doc out of sync with HEAD")
    args = p.parse_args()

    rows = collect_entries()
    rendered = render_section(rows)

    if args.dry_run:
        sys.stdout.write(rendered + "\n")
        return 0

    text = DOC_PATH.read_text()
    new_text = splice(text, rendered)
    if new_text == text:
        if args.check:
            return 0
        sys.stderr.write("trace_matrix § F.2: already in sync; no change written.\n")
        return 0
    if args.check:
        sys.stderr.write(
            "trace_matrix § F.2: out of sync with HEAD. "
            "Run scripts/update_trace_matrix_reverse_map.py to refresh.\n"
        )
        return 1
    DOC_PATH.write_text(new_text)
    sys.stderr.write(
        f"trace_matrix § F.2: refreshed ({len(rows)} entries written to {DOC_PATH.name}).\n"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
