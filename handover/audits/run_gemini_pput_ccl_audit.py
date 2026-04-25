#!/usr/bin/env python3
"""Gemini adversarial review of PREREG_PPUT_CCL_2026-04-26 (Phase A4 round 1).

Pre-registration audit — gates Phase B (kernel instrumentation). Conservative
ruling per VETO > CHALLENGE > PASS. PASS/PASS required before A5 commit gate.

This is round 1 of the PPUT-CCL arc dual-audit. Round 2+ would re-run if
CHALLENGE/VETO comes back from round 1 and PREREG is amended.
"""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 round 1)

**Role**: skeptical adversarial reviewer. You are NOT the same Gemini DeepThink instance that issued the upstream FULL PASS — treat this as a fresh, independent adversarial round (Codex is also auditing in parallel; do NOT confer).

**Mandate**: find pre-registration weaknesses that would not survive ICLR/NeurIPS Systems track adversarial review. No sycophancy. The arc author has just absorbed a substantial DeepThink FULL PASS and may be in confirmation-bias drift; it is your job to prevent that.

**Stakes**: this PREREG locks a 30-day research arc with a single sealed heldout-54 evaluation. Once Phase A5 commits, this PREREG becomes immutable except via formal addendum (which itself requires re-audit). A flaw caught here saves the arc; a flaw missed here invalidates it.

## What you are auditing

A pre-registration document for a PPUT-driven Capability Compilation Loop (CCL) research arc. The proposal:

1. Defines **Verified PPUT** (V-PPUT) = `Progress / (C × T)` where Progress ∈ {0, 1} gated on Lean ground truth, C is FULL token cost (all agents × all branches × all failures × tool stdout), T is wall-clock first-read → final-accept.
2. Names **Held-out Verified PPUT (H-VPPUT)** as the sole optimization signal (North Star).
3. Splits MiniF2F/Test (244 problems) deterministically into **adaptation 144 / meta_validation 46 / heldout 54** via SHA-256 hash bucketing on a frozen seed string `20260426_PPUT_CCL`.
4. Defines **WBCG_PPUT** as count of artifacts in state `Certified` (per a 4-state machine: Accepted → Quarantined → Certified, with Reverted as terminal-failure) that show ΔPPUT_heldout > 0 + non-positive ΔFAR/ΔCPR + RR=0 + Rollbackable.
5. Specifies a **Trust Root** of immutable files (kernel.rs, wal.rs, bus.rs, lean4_oracle.rs, constitution.md, cases/, genesis_payload.toml [pput_accounting_0], the splits JSON, the PREREG itself); ArchitectAI writes outside `user_space/Δ_*/` trigger syscall-layer EPERM.
6. Sets a **30-day phased plan**: Phase A pre-flight (you are here) → B kernel instrumentation → C ablation smoke (Soft Law / Panopticon / Amnesia / Homogeneous / Full as `--mode` flags on a single binary) → D shadow CCL (ArchitectAI proposes / AuditorAI runs § 3.5 doc meta-predicates / human user is live meta-predicate) → E single sealed heldout eval.
7. Declares **5 directional one-sided hypotheses** H1-H5 with Bonferroni α = 0.01 / family-of-5.
8. Forbids certain claim language under C-070 + C-052; bans solve-rate as headline; demotes VTR to descriptive.
9. **Anti-Goodhart battery**: 11 conformance tests (10 metering-axis from architect + 1 added: no PPUT in agent prompt). PLUS **§ 3.5 artifact-content meta-predicates** (4 tests: no_raw_failed_trace / no_exact_adaptation_solution_with_hash_check / parametric_templates / scope_and_expiration).
10. **Cognitive isolation**: ArchitectAI is sole reader of L_t; AuditorAI sees only candidate Δ + ArchitectAI summary, never raw L_t. Conformance tests `architect_sole_lt_reader.rs` + `auditor_sees_candidate_only.rs`.
11. **Backbone**: `deepseek-v4-flash` thinking-off Phases B+C (single-model to keep ablation axes clean); heterogeneous LLMs introduced at Phase D (ArchitectAI=v4-flash thinking-on, AuditorAI=Gemini 2.5 Pro).

## Your task — round-1 verdict

Independent verdict on PREREG: **PASS** (proceed to Phase B), **CHALLENGE** (revise), or **VETO** (fundamental flaw).

Score along these axes:

1. **DEFINITION** — are V-PPUT, C_i, T_i, ArtifactState, Trust Root all unambiguously definable? Any room for retroactive re-interpretation?
2. **STAT** — Bonferroni α=0.01 on family-of-5; one-sided directional H1-H5. Is the family closed? Are there secondary tests that should also be in family? Is N=20 paired adaptation hard-10 enough for H1-H4 power?
3. **LEAKAGE** — three-split with hash bucketing; heldout sealed by hash; conformance test `test_heldout_ids_inaccessible`. Are there leakage paths NOT covered (e.g., adaptation + meta_val solutions becoming features in user-space artifacts that get used at heldout)?
4. **GOODHART** — 11 metering tests + 4 content tests. Is there a residual attack surface? Specifically: can ArchitectAI generate an artifact that legitimately looks like a "white-box rule" but is actually a thinly-veiled lookup table for adaptation-set pattern → solution?
5. **GATE-H REACHABILITY** — Gate H requires Certified artifact + ΔPPUT_heldout > 0 + 6 other conditions. Is this empirically attainable in a 30-day arc with realistic Phase D throughput, or is it set so high that null result is overdetermined regardless of system quality?
6. **CCL-HUMAN-LOAD** — user is live meta-predicate in Phase D. Estimated 10 min/day. If user is unavailable for 1-2 days mid-Phase-D, what happens? Is this a single-point-of-failure for the arc?
7. **HETEROGENEITY-TIMING** — heterogeneous LLMs only at Phase D. Does this risk Soft Law / Panopticon / Amnesia ablation results in Phase C being model-specific (deepseek-v4-flash specifically) and not generalizing?
8. **TRUST-ROOT-ENFORCEMENT** — syscall-layer EPERM trap is described but the implementation is gestured to in Phase B7. What if Phase B7 implementation cannot reach syscall-level enforcement (only soft-refusal possible in Rust without special privileges)? Does the arc continue with reduced enforcement, or BLOCKER?
9. **REPRO** — script `split_pput_ccl.py` is committed; seed is the literal string `20260426_PPUT_CCL`; expected counts are 146/49/49 nominal but realized 144/46/54. Is this acceptable per pre-reg's ±5 tolerance? Is the bucketing rule correctly implemented?
10. **CLAIM-LANG** — § 10 forbids "TuringOS achieves capability compilation" without Gate H. But what about partial-PASS scenarios? E.g., 4/4 ablations succeed, ArchitectAI generates 5 Quarantined artifacts but 0 Certified at heldout — what's the publishable claim? Is § 10 specific enough?

For each axis, judge: **PASS** (clean), **CHALLENGE** (specific revision needed; cite paragraph + suggest fix), or **VETO** (fundamental, do not proceed without rethinking the whole arc).

End with:
- One-line **VERDICT** (PASS / CHALLENGE / VETO)
- If CHALLENGE: top 3 P0 must-fix items; top 3 P1 should-fix items
- One sentence on whether you would have flagged this PREREG as wholly inadequate had Codex independently issued PASS — i.e., your level of conviction.

You MUST be willing to PASS if the PREREG is genuinely sound. The arc author has gone through two rounds of architect FULL PASS already; pre-commitment to CHALLENGE would be intellectually dishonest. But if you see a NEW issue both architect rounds missed, escalate it as a fresh blocker.

---

# PREREG_PPUT_CCL_2026-04-26.md (the document being audited)

"""

prereg = (ROOT / "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md").read_text()
architect_v1 = (ROOT / "handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md").read_text()
architect_v2 = (ROOT / "handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md").read_text()
splits_json = (ROOT / "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json").read_text()
notepad = (ROOT / "handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md").read_text()
constitution = (ROOT / "constitution.md").read_text()

full_prompt = (
    brief +
    prereg +
    "\n\n---\n\n# Architect v1 directive (PPUT measure-theoretic FULL PASS, 2026-04-25)\n\n" + architect_v1 +
    "\n\n---\n\n# Architect v2 directive (Gemini DeepThink ontological FULL PASS, 2026-04-26)\n\n" + architect_v2 +
    "\n\n---\n\n# Three-split JSON output (heldout sealed by SHA-256)\n\n```json\n" + splits_json + "\n```\n" +
    "\n\n---\n\n# AUTO_RESEARCH_NOTEPAD.md (current state of arc; Paper 1 PASS/PASS just landed)\n\n" + notepad +
    "\n\n---\n\n# constitution.md (Trust Root reference)\n\n" + constitution +
    "\n\n---\n\nNow give your INDEPENDENT round-1 audit of the PREREG. Be specific: cite paragraph/§ for every CHALLENGE finding. Do not speculate beyond the document."
)

print(f"[gemini] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
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
out = ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md"
header = (f"# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 round 1)\n"
          f"**Date**: 2026-04-26\n"
          f"**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini] saved: {out}")
