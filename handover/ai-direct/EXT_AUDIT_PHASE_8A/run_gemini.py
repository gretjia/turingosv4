#!/usr/bin/env python3
"""Gemini diff audit for Phase 8.A (Step-B Phase 1c)."""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

audit_dir = ROOT / "handover/ai-direct/EXT_AUDIT_PHASE_8A"
brief = (audit_dir / "brief.md").read_text()
diff = (audit_dir / "phase_8a.diff").read_text()

context_paths = [
    "src/sdk/snapshot.rs",
    "src/sdk/tools/wallet.rs",
    "src/sdk/tool.rs",
    "handover/audits/EXT_CODEX_2026-04-22.md",
    "handover/ai-direct/STEP_B_PROTOCOL.md",
]
ctx_parts = ["\n\n---\n\n# 附件：补充上下文\n"]
for p in context_paths:
    full = ROOT / p
    if full.exists():
        content = full.read_text(errors="replace")
        if len(content) > 80_000:
            content = content[:80_000] + "\n\n[...truncated...]\n"
        ctx_parts.append(f"\n## FILE: {p}\n```\n{content}\n```\n")

full_prompt = brief + "\n\n---\n\n# The Diff to Audit\n\n```diff\n" + diff + "\n```\n" + "".join(ctx_parts)
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
out = ROOT / "handover/audits/EXT_GEMINI_PHASE_8A_2026-04-22.md"
header = f"# Gemini Phase 8.A Diff Audit\n**Date**: 2026-04-22\n**Elapsed**: {elapsed:.1f}s\n**Prompt size**: {len(full_prompt):,} chars\n**Branch**: experiment/phase-8a-snapshot-fix @ 54a32af\n\n---\n\n"
out.write_text(header + text)
print(f"[gemini] saved: {out}")
