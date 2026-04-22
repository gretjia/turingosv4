#!/usr/bin/env python3
"""Trigger Gemini / DeepSeek external audits for TuringOS v4 Phase 7."""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

TARGET = sys.argv[1] if len(sys.argv) > 1 else "gemini"
ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

if TARGET == "gemini":
    task_file = ROOT / "handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md"
    context_paths = [
        "constitution.md",
        "handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md",
        "src/bus.rs",
        "src/kernel.rs",
        "src/ledger.rs",
        "src/wal.rs",
        "src/prediction_market.rs",
        "src/sdk/tools/wallet.rs",
        "tests/reward_pull_conservation.rs",
        "tests/wal_resume.rs",
        "experiments/minif2f_v4/src/lean4_oracle.rs",
        "experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md",
        "experiments/minif2f_v4/logs/templadder_n8_20260421T164014.jsonl",
    ]
    proofs_dir = ROOT / "experiments/minif2f_v4/proofs"
    if proofs_dir.exists():
        for p in sorted(proofs_dir.glob("*.lean"))[:8]:
            context_paths.append(str(p.relative_to(ROOT)))
    out_file = ROOT / "handover/audits/EXT_GEMINI_2026-04-21.md"
elif TARGET == "deepseek":
    task_file = ROOT / "handover/ai-direct/EXT_AUDIT_2026-04-21/deepseek_mechanism_design.md"
    context_paths = [
        "constitution.md",
        "handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md",
        "handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md",
        "handover/ai-direct/LATEST.md",
        "VIA_NEGATIVA.md",
        "CLAUDE.md",
        "experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md",
        "handover/ai-direct/CHECKPOINT_PHASE_7_TURING_2026-04-21.md",
        "handover/ai-direct/SESSION_REPORT_FULL_2026-04-21.md",
    ]
    out_file = ROOT / "handover/audits/EXT_DEEPSEEK_2026-04-21.md"
else:
    sys.exit(f"Unknown target: {TARGET}")

task_prompt = task_file.read_text()

ctx_parts = ["\n\n---\n\n# 附件：相关文件全文（供审计对照）\n"]
for p in context_paths:
    full = ROOT / p
    if not full.exists():
        print(f"WARN: missing {p}", file=sys.stderr)
        continue
    try:
        content = full.read_text(errors="replace")
    except Exception as e:
        print(f"WARN: read {p} failed: {e}", file=sys.stderr)
        continue
    if len(content) > 120_000:
        content = content[:120_000] + "\n\n[...truncated...]\n"
    ctx_parts.append(f"\n## FILE: {p}\n```\n{content}\n```\n")

full_prompt = task_prompt + "".join(ctx_parts)
print(f"[{TARGET}] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

t0 = time.time()
if TARGET == "gemini":
    url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
    body = json.dumps({
        "contents": [{"parts": [{"text": full_prompt}]}],
        "generationConfig": {"temperature": 0.2, "maxOutputTokens": 32768},
    }).encode()
    headers = {"Content-Type": "application/json"}
else:
    url = "https://api.deepseek.com/v1/chat/completions"
    body = json.dumps({
        "model": "deepseek-reasoner",
        "messages": [{"role": "user", "content": full_prompt}],
        "max_tokens": 8192,
    }).encode()
    headers = {"Content-Type": "application/json", "Authorization": f"Bearer {env['DEEPSEEK_API_KEY']}"}

req = urllib.request.Request(url, data=body, headers=headers, method="POST")
try:
    with urllib.request.urlopen(req, timeout=1500) as resp:
        data = json.loads(resp.read())
except urllib.error.HTTPError as e:
    err_body = e.read().decode(errors="replace")
    print(f"[{TARGET}] HTTP {e.code}: {err_body[:2000]}", file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f"[{TARGET}] ERROR: {e}", file=sys.stderr)
    sys.exit(2)

elapsed = time.time() - t0
print(f"[{TARGET}] API returned in {elapsed:.1f}s", file=sys.stderr)

if TARGET == "gemini":
    try:
        text = data["candidates"][0]["content"]["parts"][0]["text"]
    except (KeyError, IndexError) as e:
        print(f"[gemini] unexpected response: {json.dumps(data)[:2000]}", file=sys.stderr)
        sys.exit(3)
else:
    msg = data["choices"][0]["message"]
    text = msg["content"]
    reasoning = msg.get("reasoning_content", "")
    if reasoning:
        text = f"## Reasoning chain\n\n{reasoning}\n\n---\n\n## Final answer\n\n{text}"

out_file.parent.mkdir(parents=True, exist_ok=True)
header = f"# External Audit Result: {TARGET.upper()}\n**Date**: 2026-04-21\n**Elapsed**: {elapsed:.1f}s\n**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n"
out_file.write_text(header + text)
print(f"[{TARGET}] saved: {out_file}")
