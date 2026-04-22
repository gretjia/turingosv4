#!/usr/bin/env python3
"""Gemini re-audit for Phase 8 R1-α + R2 + R3 (final VETO addressment)."""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

audit_dir = ROOT / "handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA"
brief = (audit_dir / "brief.md").read_text()
diff = (audit_dir / "amendment.diff").read_text()

context_paths = [
    "handover/audits/EXT_CODEX_PHASE_8_R1R8_2026-04-22.md",
    "handover/audits/EXT_GEMINI_PHASE_8_R1R8_2026-04-22.md",
    ".claude/worktrees/phase-8a-snapshot/src/sdk/oracle_receipt.rs",
    ".claude/worktrees/phase-8a-snapshot/src/sdk/predicate.rs",
    ".claude/worktrees/phase-8a-snapshot/tests/oracle_receipt_bus.rs",
    ".claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/src/lean4_oracle.rs",
]
ctx_parts = ["\n\n---\n\n# 附件\n"]
for p in context_paths:
    full = ROOT / p
    if full.exists():
        content = full.read_text(errors="replace")
        if len(content) > 60_000:
            content = content[:60_000] + "\n\n[...truncated...]\n"
        ctx_parts.append(f"\n## FILE: {p}\n```\n{content}\n```\n")

full_prompt = (brief
    + "\n\n---\n\n# Amendment diff (commit 4a72507, +645 -255)\n\n```diff\n"
    + diff + "\n```\n"
    + "".join(ctx_parts))
print(f"[gemini] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {"temperature": 0.1, "maxOutputTokens": 16384},
}).encode()
headers = {"Content-Type": "application/json"}

t0 = time.time()
req = urllib.request.Request(url, data=body, headers=headers, method="POST")
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except urllib.error.HTTPError as e:
    print(f"[gemini] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md"
header = (f"# Gemini Phase 8 R1-α + R2 + R3 Re-Audit\n"
          f"**Date**: 2026-04-22\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n"
          f"**Target commit**: 4a72507 (R1-α + R2 + R3 addressment)\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini] saved: {out}")
