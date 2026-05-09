#!/usr/bin/env python3
"""Gemini Stage C Polymarket Overall PRE-§8 audit (Phase F.9 cap).

Per `feedback_dual_audit` Class-4 timing rule + `feedback_no_batch_class4_signoff`
(per-atom §8 cadence preserved): each Class-4 atom (P-M2 + P-M4 + P-M6) had
its own §8 dispatch + ratification; this audit caps the FULL Stage C
sequence (P-M2..P-M9) at the Phase F.9 §8 boundary.

Conservative-wins: VETO > CHALLENGE > PASS. Round cap 2.
"""
import json
import os
import re
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [
    Path("/home/zephryj/projects/turingosv3/.env"),
    ROOT / ".env",
]
ROUND = os.environ.get("TB_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_STAGE_C_OVERALL_AUDIT_2026-05-09_{ROUND}.md"

if ROUND not in {"R1", "R2", "R3"}:
    print("[gemini C overall] error: TB_AUDIT_ROUND must be R1 or R2", file=sys.stderr)
    sys.exit(2)
if OUT.exists():
    print(f"[gemini C overall] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
    sys.exit(2)


def load_env() -> dict:
    env = {}
    for fp in ENV_FILES:
        if not fp.exists():
            continue
        for line in fp.read_text().splitlines():
            if "=" in line and not line.strip().startswith("#"):
                key, value = line.split("=", 1)
                env.setdefault(key.strip(), value.strip().strip('"').strip("'"))
    return env


def append_file(rel: str, lang: str = "") -> str:
    fp = ROOT / rel
    if not fp.exists():
        return f"\n\n---\n\n## {rel}\n\n(MISSING: expected file not found)\n"
    text = fp.read_text()
    # Truncate per-file at ~50KB to keep prompt size manageable.
    if len(text) > 50_000:
        text = text[:50_000] + f"\n\n... [truncated; original {len(text):,} chars]\n"
    return f"\n\n---\n\n## {rel}\n\n```{lang}\n{text}\n```\n"


def extract_verdict(text: str) -> str:
    patterns = [
        r"(?im)^##\s*VERDICT:\s*(PASS|CHALLENGE|VETO)\b",
        r"(?im)^VERDICT:\s*(PASS|CHALLENGE|VETO)\b",
        r"(?im)^Final aggregate verdict:\s*(PASS|CHALLENGE|VETO)\b",
    ]
    for p in patterns:
        m = re.search(p, text)
        if m:
            return m.group(1).upper()
    for tok in ("VETO", "CHALLENGE", "PASS"):
        if re.search(rf"\b{tok}\b", text):
            return tok
    return "UNKNOWN"


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini C overall] GEMINI_API_KEY not found in .env files", file=sys.stderr)
    sys.exit(2)

brief = f"""# Gemini Stage C Polymarket Overall PRE-§8 Audit — {ROUND}

You are Gemini 2.5 Pro acting as a skeptical architectural reviewer for
TuringOS v4 Stage C Polymarket FULL SEQUENCE — capping the Phase F.9 §8
boundary post all 8 P-M atoms shipped.

This is the Gemini half of the PRE-§8 dual-audit per `feedback_dual_audit`
Class-4 timing rule. Codex G2 implementation-paranoid half dispatches in
parallel.

## Context

Stage C Polymarket session #27 was VETOED + fully rolled back (HEAD `01dd825`,
2026-05-09). 4 defects in the batch §8 packet. Phase E mechanism gates landed
(2026-05-09 session #28). Phase F per-atom rebuild + ship over sessions #29
(P-M2) + #30 (P-M3) + #31 (P-M4) + #32 (P-M5 + P-M6 + P-M7 + P-M8 + P-M9).

Each Class-4 atom got its own per-atom §8:
- P-M2 (Phase F.1): SHIPPED FINAL session #29; PRE-§8 dual audit R1 CHALLENGE → R2 PASS.
- P-M4 (Phase F.3): SHIPPED FINAL session #31; PRE-§8 dual audit R1 PASS/PASS first-try.
- P-M6 (Phase F.5): SHIPPED FINAL session #32; PRE-§8 dual audit R1 PASS/PASS first-try.

Non-Class-4 atoms shipped under Class-3 / Class 1-2 framing without §8:
- P-M3 (Phase F.2): Class 3; SHIPPED session #30.
- P-M5 (Phase F.4): Class 3; SHIPPED session #32.
- P-M7 (Phase F.6): Class 1-2; SHIPPED session #32.
- P-M8 (Phase F.7): Class 1-2; SHIPPED session #32.
- P-M9 (Phase F.8): Class 2-3 controlled market smoke; SHIPPED session #32.

This Phase F.9 audit caps the full Stage C sequence at the OVERALL §8
boundary. The architect manual §6 forbidden list and §7 forbidden list
both apply here as cross-cutting checks.

Conservative merge ranking applies: VETO > CHALLENGE > PASS. Round cap 2.

## Audit target

HEAD `55c8d35` on origin/main = P-M9 ship commit (full Stage C sequence
shipped). All P-M atoms shipped on origin/main.

Validation at HEAD `55c8d35`:
- Constitution gates: 231 PASS / 0 failed / 1 ignored (was 175 pre-Stage-C; +56)
- Workspace tests: ~1370 PASS / 0 failed / 151 ignored
- Trust Root verify: PASS
- 8 architect §7.x verbatim test batteries all GREEN
- 5-gate end-to-end smoke (P-M9) PASS

## Required questions

Answer Q1 through Q10 explicitly. Mark each as PASS / CHALLENGE / VETO.

Q1. **Stage C aggregate ship integrity**: Verify all 8 P-M atoms have
    landed on origin/main with their respective architect §7.x verbatim
    test batteries GREEN. Cross-check against the §1 atom ship status
    table in the §8 packet. Are any atoms missing from origin/main?
    Are any test batteries incomplete (fewer than architect-mandated
    test count)?

Q2. **Per-atom §8 cadence preservation** (`feedback_no_batch_class4_signoff`):
    Verify each Class-4 atom (P-M2 + P-M4 + P-M6) received its own per-
    atom §8 sign-off, NOT batched. Cross-check against the §5 dual audit
    pattern history table. Is the per-atom cadence respected? Is the
    Stage C overall §8 (this packet) properly framed as a sequence-cap
    rather than a batch §8?

Q3. **Defect closure verification** (session #27 batch §8 VETO targets):
    For each of the 4 defects (1 P-M6 strict-equality monetary, 2 P-M6
    vacuous rollback, 3 P-M2 timestamp_logical drift, 4 P-M4
    event_id_kind rename), verify Phase E mechanism + Phase F closure
    are both in place. Are all 4 defects mechanically prevented from
    recurrence in future Class-4 atoms? Are E.1 + E.2 + E.3 binding
    states all `Landed` for all P-M atoms that need them?

Q4. **Architect §6 forbidden list compliance**: Verify each item from
    architect manual §8 forbidden list is enforced:
    - automatic per-node 100 YES + 100 NO without collateral
    - Treasury magic seed without debit
    - f64 money math
    - DPMM / pro-rata payout inside CTF track
    - price-based settlement
    - agent-submitted MarketResolveTx
    - agent-submitted system resolution
    - AMM before CompleteSet
    - trading before audit tools
    - public chain before sandbox
    - real money before readiness gate
    Cross-check against §3 architect alignment summary table in the
    §8 packet. Are any items missing enforcement?

Q5. **CTF / Coin conservation across the full sequence**: Run the
    polymarket smoke (`tests/constitution_polymarket_smoke.rs`) and
    verify the 5 architect §7.10 gates are ALL satisfied:
    - no ghost liquidity
    - total coin conserved
    - no price-as-truth
    - no raw log broadcast
    - all activity replayable
    Are the gate assertions in the smoke test sufficient evidence?

Q6. **Architect §7.5 + §7.7 verbatim spec adherence at integration
    boundary**: P-M4 (§7.5 5-field state struct) + P-M6 (§7.7 9-step
    composite) interact at the assert_complete_set_balanced symmetric-
    branch enforcement (P-M4 extended to count pool reserves; P-M6
    router post-state must satisfy strict equality). Verify the
    cross-atom integration is correct — would a future router buy
    through a pool created by P-M4 admission yield a post-state that
    passes the P-M4-extended invariant? Trace the math.

Q7. **Replay-determinism end-to-end**: HEAD `55c8d35` represents 8
    accepted Class-3/4 atoms + numerous Class-1/2 atoms. Verify:
    - Trust Root verify passes (cargo test --lib
      boot::tests::verify_trust_root_passes_on_intact_repo).
    - cargo test --workspace: 0 failures.
    - bash scripts/run_constitution_gates.sh: 231/0/1 PASS.
    - The cfg(debug_assertions) failure-injection hook in P-M6 admission
      arm cannot influence production --release replay.

Q8. **Forward queue scope check**: Verify the forward queue in the §8
    packet §8 (Stage D / K.1-6 / real-problem testing / C.5 PromptCapsule
    / B.4 CAS Merkle) is consistent with:
    - User authorization scope `直到polymarket全部落地并自主开展真题测试`.
    - `feedback_launch_priority` (Lean Proof Task Market MVP precedes
      NodeMarket / Polymarket / multi-org / public-chain at TB-7+ era;
      Stage D real-world-readiness even further out).
    - Remediation directive forward queue.
    Is the forward queue properly scoped? Does it correctly defer
    Stage D real-world-readiness behind explicit architect ship gate?

Q9. **User multi-clause §8 form**: Per CLAUDE.md §10, the user's session
    #32 boot authorization is multi-clause. Verify the structural
    analysis in the §8 packet §6 is correct — clause 1 names act `授权` +
    `自主执行` + scope `直到polymarket全部落地`. Compare to canonical
    Class-4 §8 forms (TB-C0 "好，确认可以 ship" / Stage A3 "同意 sign-off" /
    P-M4 "签字，同意后续执行"). Is the user authorization structurally
    equivalent at Class-4 §8 strength? Is it conditional on dual audit
    PASS (which is what this audit is testing)?

Q10. **Strategic risk + cross-cutting concerns**: What, if anything, in
    Stage C as a whole is visibly wrong or missing? In particular
    consider:
    - Pool drain attacks across multiple swap directions.
    - MEV / front-running between price quote + router buy submission.
    - Dust accumulation in long-running pools (k drift).
    - Interaction between P-M2 merge + P-M6 router (can a sequence of
      mint → router buy → merge produce an unaccounted Coin
      conservation gap?).
    - Pool resolution / unwind path (Resolved / Closed PoolStatus
      transitions are deferred to future TB; what's the safety story?).

## Verdict format

```
Q1: PASS|CHALLENGE|VETO - <reason>
Q2: PASS|CHALLENGE|VETO - <reason>
Q3: PASS|CHALLENGE|VETO - <reason>
Q4: PASS|CHALLENGE|VETO - <reason>
Q5: PASS|CHALLENGE|VETO - <reason>
Q6: PASS|CHALLENGE|VETO - <reason>
Q7: PASS|CHALLENGE|VETO - <reason>
Q8: PASS|CHALLENGE|VETO - <reason>
Q9: PASS|CHALLENGE|VETO - <reason>
Q10: PASS|CHALLENGE|VETO - <reason>

## VERDICT: PASS|CHALLENGE|VETO
Conviction: low|medium|high
Recommendation: PROCEED|FIX-THEN-PROCEED|REDESIGN
Remediations:
- <only for CHALLENGE/VETO; actionable and scoped>
```

If any Q is CHALLENGE, aggregate must be CHALLENGE unless another Q is VETO.

---

# Ground Truth Excerpts
"""

for rel, lang in [
    ("handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md", "markdown"),
    ("handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md", "markdown"),
    ("handover/directives/2026-05-09_STAGE_C_POLYMARKET_OVERALL_§8_PACKET.md", "markdown"),
    ("tests/constitution_polymarket_smoke.rs", "rust"),
    ("tests/constitution_router_buy_with_coin.rs", "rust"),
    ("tests/constitution_audit_views.rs", "rust"),
    ("tests/constitution_router_price_quote.rs", "rust"),
    ("CLAUDE.md", "markdown"),
]:
    brief += append_file(rel, lang)

print(f"[gemini C overall] prompt size: {len(brief):,} chars", file=sys.stderr)

url = (
    "https://generativelanguage.googleapis.com/v1beta/models/"
    f"gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
)
body = json.dumps(
    {
        "contents": [{"parts": [{"text": brief}]}],
        "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
    }
).encode()
req = urllib.request.Request(
    url,
    data=body,
    headers={"Content-Type": "application/json"},
    method="POST",
)

t0 = time.time()
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as exc:
    print(f"[gemini C overall] error: {exc}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini C overall] API returned in {elapsed:.1f}s", file=sys.stderr)

try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as exc:
    print(f"[gemini C overall] malformed API response: {exc}", file=sys.stderr)
    print(json.dumps(data, indent=2)[:4000], file=sys.stderr)
    sys.exit(1)

verdict = extract_verdict(text)
header = f"""# Gemini Stage C Polymarket Overall PRE-§8 Audit — {ROUND}

**Round**: {ROUND}
**Date**: 2026-05-09
**Model**: gemini-2.5-pro
**Elapsed**: {elapsed:.1f}s
**Prompt size**: {len(brief):,} chars
**HEAD**: 55c8d35
**Final aggregate verdict**: {verdict}

---

## Verbatim Gemini Response

"""

OUT.write_text(header + text)
print(f"[gemini C overall] saved: {OUT}")
print(f"[gemini C overall] verdict: {verdict}")
