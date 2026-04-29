#!/usr/bin/env bash
# Codex round-2 audit on CO1.7-extra v1 (post round-1 scope split).
# Implementer-paranoid angle: did the Occam-driven scope split actually resolve
# round-1's M1 (substrate gap) + M2 (purity violations)? Are M3-M5 fixes correctly
# applied? Any new defects in the smaller atom?
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_extra_codex_round2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.7-extra v1 (Round 2; post round-1 scope split)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-2 (parallel).

**Mandate**: round 2 dual external audit on CO1.7-extra v1 — a SCOPE-SPLIT atom carved out of the round-1-CHALLENGED bundled CO1.7.5 v1. Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**What changed since round-1**: ArchitectAI executed Occam-driven scope split (B2: split by dependency profile) per user's "无损压缩即智能" principle. The previous bundled atom is now TWO atoms:
- **CO1.7-extra (this audit target)**: D2 head_t close + D3 Sequencer entry-point wiring + 1 substrate-independent test. **No FC1/FC2 substrate dependency**.
- **CO1.7.5 (future)**: D1 transition bodies + 3 D4 tests + un-ignore replay test. Gated on CO P2.x substrate atoms.

Round-1 must-fix items disposition:
- M1 (substrate gap) → D1 deferred to future CO1.7.5; CO1.7-extra has zero substrate dep
- M2 (purity violations) → moved with D1; not in CO1.7-extra
- M3 (compile defects: TuringBus / Kernel derives / Sequencer placement) → fixed in v1
- M4 (§ 0.3 process passive) → § 0.4 active reconciliation commitment + STATE v1.5 issue filing committed
- M5 (Q1 head_commit_oid_hex) → synthesis: default None + mandatory override + defensive test
- M6/M7 (mapping table + RejectedAttemptSummary) → moved with D1
- M8 (smoke staleness) → footer re-run at f7fc19f, 8/8 PASS

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` (~378 lines)
2. **Round-1 audit artifacts** (context): `handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`, `GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`, `CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md`
3. **Frozen prior specs**: CO1.7 v1.2 (round-3 PASS/PASS — supplies LedgerWriter trait + Sequencer machinery + head_commit_oid)
4. **Shipped code (target of D2/D3)**: src/state/sequencer.rs + src/bottom_white/ledger/transition_ledger.rs (D2 trait method addition + apply_one stage 9 patch); src/bus.rs (TuringBus forwarder) + src/kernel.rs (Kernel field) (D3 STEP_B target)
5. **Reference**: src/state/q_state.rs (NodeId types); src/bottom_white/cas/store.rs (CasStore put/get for D4 test)

## Round 2 audit questions

**Q1. Round-1 M1 closure (substrate gap)**: spec § 0.1 declares D1 transition bodies + 3 D4 tests moved to future CO1.7.5 atom; CO1.7-extra has "no substrate dependency". Verify:
- The 3 deliverables actually in CO1.7-extra (D2 + D3 + 1 D4 test) — are any of them transitively substrate-dependent? Specifically: does D2's `q_w.head_t = NodeId(commit_oid_hex)` write require ANY economic_state_t / predicate / tool registry method that doesn't exist? Does D3's TuringBus/Kernel wiring touch substrate?
- The cas_payload_round_trip test — uses only `CasStore::put` + `get`. Both exist (CO1.4 + CO1.4-extra shipped). Confirm no hidden substrate dep.
- Is the scope split a **valid** application of Anti-Oreo three-layer separation, or did the spec smuggle FC2/FC1 work into CO1.7-extra under a different name?

**Q2. Round-1 M2 closure (D1 purity violations)**: D1 is now out of scope. Confirm:
- Spec § 4 explicitly defers transition bodies + their purity contract concerns to future CO1.7.5
- Spec does NOT contain residual D1 specifications that could mislead implementation

**Q3. Round-1 M3 closure (compile defects)**: verify each fix:
- M3a: spec § 2.1 uses `TuringBus` everywhere (NOT `Bus`). Grep the spec body — any residual `Bus` references that should be `TuringBus`?
- M3b: Kernel field annotated `#[serde(skip)]` (spec § 2.1). Sequencer struct gets `#[derive(Debug)]` (spec § 2.1). Are these necessary AND sufficient? Specifically: does Sequencer's `Arc<RwLock<dyn LedgerWriter>>` field allow blanket `Debug` derive, or is manual impl actually required (per Q1' open)?
- M3c: Sequencer placement justification (spec § 2.2) — three arguments: (1) parallel to existing Tape/NodeId pattern, (2) state lives in Q_t not Kernel, (3) doc-comment patch. Does this hold up under scrutiny? Or does it just paper over the layering concern?

**Q4. Round-1 M4 closure (§ 0.4 active reconciliation)**: spec § 0.4 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue. Verify:
- Is "filing an issue" sufficient, or does the spec need to actually DRAFT the v1.5 patch text inline?
- Spec asserts "downstream-spec supersession authority principle" — is this assertion within ArchitectAI's authority, or does it require a constitution-level amendment?
- Two carry-forward supersessions (NodeId head_t binding + SignalKind 4-variant) migrate to future CO1.7.5 — but they actually take effect HERE (D2 sets head_t = NodeId(commit_oid_hex), not NodeId::from_state_root). Is the migration framing correct?

**Q5. Round-1 M5 closure (Q1 synthesis)**: spec § 1.2 + § 3.2 implement default `None` + mandatory override + defensive `git2_writer_returns_some_after_commit` test. Verify:
- The defensive test is sufficient: does it actually catch silent stagnation, or only the most obvious failure mode?
- "Mandatory override" — is this enforced by the language (compiler) or only by spec/convention? If only by convention, what guarantees future LedgerWriter impls don't inherit the default-None silently?
- Q1' (NEW open): Sequencer Debug derive completeness. Spec proposes `finish_non_exhaustive()` fallback if blanket-derive fails. Is `finish_non_exhaustive` safe/sufficient? Any leak risk?

**Q6. Atomicity claim refinement (Q-B from round-1, refined here)**: spec § 1.1 says under acquired locks, `writer.commit() Ok → AtomicU64::store → field assignments` is infallible. The refined claim acknowledges this only fully holds when `head_commit_oid_hex` returns Some (Git2). Is this refinement now correct, or does it still overclaim?

**Q7. STEP_B ceremony argument** (Q-C from round-1, rebased): spec § 2.3 rebased the combined-ceremony argument from "Phase 0 minimum sufficient version is binding" (Codex r1 said advisory) to "functional coupling" (each half compile-error or no-op without other). Is functional-coupling a stronger criterion that justifies combined ceremony, or is it an alternative way to phrase the same advisory request?

**Q8. New defects in v1**: the scope split changed the atom shape. Any new defects introduced by the rewrite that weren't in the bundled v1?
- Spec body coherence: any internal contradictions / dangling references to D1/D4-3-tests that should now be removed?
- Test coverage: 2 tests (cas_payload_round_trip + git2_writer_returns_some_after_commit) — sufficient for D2 + D3 + 1 D4 test scope, or should v1.1 add a head_t-advancement integration test that exercises the actual D2 code path?
- LoC estimate (~150-230 LoC): defensible? Or does Sequencer Debug + serde-skip + with_sequencer constructor + 2 trait method overrides come out higher?

**Q9. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is CO1.7-extra implementable end-to-end (cargo test --workspace passing with the 2 new tests)? Specific blockers — any v4-blocking deps that aren't in the inheritance list (CO1.7-impl + CO1.4-extra)?

## Output format

# Codex CO1.7-extra Round-2 Audit
## Q1 Round-1 M1 closure (substrate gap)
## Q2 Round-1 M2 closure (D1 purity)
## Q3 Round-1 M3 closure (compile defects)
## Q4 Round-1 M4 closure (§ 0.4 reconciliation)
## Q5 Round-1 M5 closure (Q1 synthesis) + Q1' Sequencer Debug
## Q6 Atomicity claim refinement
## Q7 STEP_B functional-coupling argument
## Q8 New defects in v1
## Q9 Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers + source file line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean spec = PASS.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.7-extra v1 (target of audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: round-1 merged verdict (the document driving this scope split)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: CO1.7 v1.2 spec (frozen, round-3 PASS/PASS) — supplies inheritance\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped src/state/sequencer.rs (D2 target)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/sequencer.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bottom_white/ledger/transition_ledger.rs (D2 trait target)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bus.rs (D3 STEP_B target — TuringBus)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bus.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/kernel.rs (D3 STEP_B target — Kernel)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/kernel.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/state/q_state.rs (NodeId tuple struct + Q_t)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/q_state.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bottom_white/cas/store.rs (D4 test surfaces)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/cas/store.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-2 audit. Cite spec § + line where possible.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7-extra r2] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7-extra Round-2 Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: spec v1 (post round-1 scope split)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7-extra r2] API returned in ${elapsed}s" >&2
echo "[codex co1.7-extra r2] saved: $OUT" >&2
