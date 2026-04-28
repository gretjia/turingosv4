#!/usr/bin/env bash
# Codex round-1 audit on CO1.7 transition_ledger spec v1 + type-skeleton smoke artifact.
# Implementer-review angle: does the spec compose with shipped code? Are the 5
# DIV-1..5 catalog entries + 11 open Qs + 1 spec bug found by smoke real, and
# what new defects exist that smoke + paper-review missed?
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_codex_round1.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.7 transition_ledger spec v1 + type-skeleton (Round 1)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (running in parallel).

**Mandate**: round 1 dual external audit on the **joint artifact** (spec v1 + Rust type-skeleton). Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator (Claude generated; you review). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Why a joint artifact, not paper-only**: per memory `feedback_smoke_before_batch`, plan audit ≠ runtime check. Skeleton smoke already caught 1 spec bug + 5 DIV cataloged before this audit. Paper-review-only would have missed those.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` (~440 lines, 15 sections)
2. **Type skeleton**: `src/bottom_white/ledger/transition_ledger.rs` (~370 lines incl. 6 inline tests, 196/0 lib PASS, TR-tracked)
3. **Companion specs (frozen)**: STATE_TRANSITION_SPEC v1.4, SYSTEM_KEYPAIR_SECURITY_v1, WP v2.2 § 5.L4

## What is at stake

- **PASS** unblocks CO1.7.5 (step_transition body) + CO1.8 (L5 materializer) + CO1.9 (L6 signal indices) + Git runtime_repo wiring + economy 6 transition fn instantiation
- **CHALLENGE** triggers spec v1.1 patch round
- **VETO** blocks CO1.7 implementation entirely until major rework

## Round 1 audit questions

**Q-A. DIV-1..5 validity**: are the 5 cataloged spec ↔ shipped-code divergences real, or did the skeleton mis-read the actual API?
- DIV-1 LedgerEntry not in CanonicalMessage 3-variant enum → real or strawman?
- DIV-2 Q_t mutation API absent → does CO P2.x economy genuinely block? Workaround?
- DIV-3 epoch field missing in spec v1 § 1 → skeleton fix correct? Anything else missing?
- DIV-4 CasReader trait → narrowed to LedgerCasView; trait segregation defensible?
- DIV-5 CasStore::put 5-param signature → sequencer responsibility correctly placed?

**Q-B. Q9 spec bug closure**: the smoke caught canonical_digest_unsigned circular dependency on resulting_ledger_root. Skeleton's fix (digest covers 7 fields, EXCLUDES resulting_ledger_root AND system_signature). Is the exclusion list complete? Should `epoch` also be excluded if it's set by sequencer not the agent? What about `tx_payload_cid` if CAS put is non-deterministic across implementations?

**Q-C. Sequencer correctness**: spec § 3 sequencer pseudocode (apply_one). Issues to check:
1. RwLock pattern — read snapshot, compute pure, then exclusive write — correct? Concurrent readers between snapshot and commit could observe inconsistent (Q_t, ledger_root_t) — race?
2. AtomicU64::fetch_add(1, SeqCst) for logical_t — sufficient for I-LOGTIME monotonicity? Failure cases?
3. `keypair.sign_entry(entry)` is a placeholder; the actual sign primitive needs to ride CanonicalMessage (DIV-1). What's the correct API to expose?
4. apply_one builds LedgerEntry with resulting_ledger_root computed BEFORE knowing system_signature; is this ordering forced by Q9, and does it open any attack vector (e.g., signature does not bind ledger_root)?

**Q-D. Replay completeness**: skeleton replay_chain_integrity does parent_state_root + ledger_root chain check only. It does NOT re-run pure transitions to independently verify resulting_state_root (deferred per DIV-2 / CO1.7.5+).
- Is partial replay an acceptable v1 deliverable, OR is it a I-DETHASH violation by construction?
- If partial: is the spec sufficiently honest about what replay v1 does NOT verify?
- Should v1.1 add a "trust mode" flag distinguishing chain-only-replay vs full-transition-replay?

**Q-E. Atom scope creep**: spec § 0 says "out of scope: L5 materializer (CO1.8), L6 signal indices (CO1.9), MetaTx full schema (v4.1)". But § 3 sequencer.apply_one already references state_root_t materialization (`q_w.head_t = NodeId::from_state_root(...)`). Where is the L5 boundary?
- Is the L4/L5 line drawn correctly or does CO1.7 implicitly need a L5 stub?
- Does this create a CO1.7 ↔ CO1.8 ordering hazard (each blocks the other)?

**Q-F. STEP_B disposition**: spec § 9 claims "no STEP_B parallel-branch ceremony required" because transition_ledger.rs + sequencer.rs are NEW files. Verify against `STEP_B_PROTOCOL.md` + memory `feedback_step_b_protocol`. Are there indirect modifications (e.g., changes to bus.rs/kernel.rs/wal.rs that the sequencer integration WILL force later)?

**Q-G. Open Q1-Q7 (original spec) + Q8-Q11 (skeleton-found) judgment**:
- Q1 SubmissionQueue type (tokio mpsc / crossbeam / std mpsc) — your recommendation?
- Q4 system_signature placement (inside struct vs sidecar tuple) — your recommendation?
- Q5 dispatch via enum-match vs MetaTransitionInterface trait — your recommendation?
- Q7 genesis ledger_root_t (Hash::ZERO vs sha256 of genesis_payload.toml) — your recommendation?
- Q8 CanonicalMessage extension (a) vs sibling sign primitive (b) — your recommendation?
- Q11 (you may add): are there NEW open Qs not in spec § 11 that should be?

**Q-H. New defects**: independent of catalog, what does the joint artifact still get wrong?
- Type errors that cargo check missed (rare but possible if behind a cfg)?
- Spec ↔ skeleton inconsistencies?
- Missing invariants — anything from STATE_TRANSITION_SPEC § 4 (27 invariants) that CO1.7 should enforce but skeleton/spec doesn't?
- Conformance tests: skeleton has 6; spec § 7 promises 8; what's the gap?

**Q-I. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is the joint artifact implementable end-to-end (i.e., `cargo test --lib transition_ledger` will pass with stubs filled in by CO1.7.5+)? Specific blockers to call out.

## Output format

# Codex CO1.7 Round-1 Audit
## Q-A DIV-1..5 validity
## Q-B Q9 spec bug closure
## Q-C Sequencer correctness
## Q-D Replay completeness
## Q-E Atom scope creep
## Q-F STEP_B disposition
## Q-G Open Q recommendations
## Q-H New defects
## Q-I Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers + skeleton line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean joint artifact = PASS.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.7 spec v1 (target of audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Type-skeleton (target of audit, joint with spec above)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: SYSTEM_KEYPAIR_SECURITY_v1 (frozen, CO1.7.0a-f)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: WP v2.2 § 5.L4 + § 6 transition protocol\n\n' >> "$TMP_PROMPT"
sed -n '360,460p' "${ROOT}/handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped CO1.7.0a-f system_keypair source (for DIV-1 ground truth)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped CO1.4 CAS source (for DIV-4/5 ground truth)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/cas/schema.rs" "${ROOT}/src/bottom_white/cas/store.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-1 audit. Cite spec § + line where possible.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7 r1] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7 transition_ledger Round-1 Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: spec v1 + type-skeleton joint artifact\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7 r1] API returned in ${elapsed}s" >&2
echo "[codex co1.7 r1] saved: $OUT" >&2
