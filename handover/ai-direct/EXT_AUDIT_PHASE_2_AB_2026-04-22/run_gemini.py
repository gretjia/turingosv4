#!/usr/bin/env python3
"""Gemini audit for Phase 2 A/B verdict + Phase 9 market pivot."""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

audit_dir = ROOT / "handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22"
brief = (audit_dir / "brief.md").read_text()
tsv = (audit_dir / "per_problem.tsv").read_text()

context_paths = [
    "handover/audits/PPUT_RAW_DATA_2026-04-22.md",
    "handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md",
    "handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md",
    "handover/ai-direct/REGISTRATION_PHASE_9_2026-04-22.md",
    "handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md",
    "handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md",
    "experiments/minif2f_v4/logs/phase8_baseline_main_oneshot_20260422T122117.jsonl",
    ".claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/logs/phase8_experiment_oneshot_20260422T122119.jsonl",
    "constitution.md",
]
ctx_parts = ["\n\n---\n\n# 附件\n"]
for p in context_paths:
    full = ROOT / p
    if full.exists():
        content = full.read_text(errors="replace")
        if len(content) > 50_000:
            content = content[:50_000] + "\n\n[...truncated...]\n"
        ctx_parts.append(f"\n## FILE: {p}\n```\n{content}\n```\n")

full_prompt = (brief
    + "\n\n---\n\n# Paired per-problem TSV\n\n```tsv\n"
    + tsv + "\n```\n"
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
out = ROOT / "handover/audits/EXT_GEMINI_PHASE_2_AB_2026-04-22.md"
header = (f"# Gemini Phase 2 A/B + Phase 9.M Pivot Audit\n"
          f"**Date**: 2026-04-22\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini] saved: {out}")
