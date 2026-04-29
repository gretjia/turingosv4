#!/usr/bin/env bash
# Codex round-1 audit on CO1.7.5 spec v1 (transition bodies + runtime wiring).
# Implementer-review angle: does the spec compose with shipped code? Are the
# 5 smoke-closed Qs actually closed in code/spec consistency? Are the 12
# self-audit patches load-bearing? What new defects exist that smoke +
# self-audit missed?
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_5_codex_round1.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.7.5 spec v1 DRAFT (Round 1)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (running in parallel).

**Mandate**: round 1 dual external audit on CO1.7.5 spec v1 (self-audited; pre round-1). Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator (Claude generated; you review). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Scope clarification**: this is a **spec-only** audit (no skeleton has been written yet — the implementation is gated post-PASS/PASS per CLAUDE.md "Audit Standard"). The smoke verification at § 7 + footer + the 12 patches in the patch log are part of the audit surface. Verify every smoke-cited line/byte against the actual repo at HEAD `334111a`.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` (372 lines)
2. **Frozen prior specs (carry-forward authority)**: CO1.7 v1.2 (round-3 PASS/PASS), CO1.1.4-pre1 (PASS/PASS, ABI lock), STATE_TRANSITION_SPEC v1.4 (round-4 PASS/PASS)
3. **Shipped code (target of D1-D4)**: src/state/sequencer.rs (apply_one + dispatch_transition stubs), src/state/typed_tx.rs (frozen TypedTx + 22 TransitionError + 4 SignalKind), src/bottom_white/ledger/transition_ledger.rs (Git2LedgerWriter + InMemoryLedgerWriter), src/state/q_state.rs (Q_t with NodeId tuple struct)
4. **STEP_B-restricted touch surface**: src/bus.rs + src/kernel.rs (currently use legacy crate::ledger; CO1.7.5 D3 adds Sequencer wiring). Hygiene OBS sedimented at `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`.

## What is at stake

- **PASS** unblocks CO1.7.5-impl (final L4 atom; Wave 6 #1 100% closure)
- **CHALLENGE** triggers spec v1.1 patch round
- **VETO** blocks CO1.7.5 implementation entirely until major rework

## Round 1 audit questions

**Q-A. Two-supersession framework (§ 0.3)**: spec declares STATE v1.4 § 3 line 412 (NodeId::from_state_root) and STATE v1.4 § 3 lines 403-409 (BoolSignal/StatSignal richness) are "carried forward as superseded by CO1.7 K3 v1.2 + CO1.1.4-pre1". Is this:
- Authority-chain valid? CO1.7 v1.2 is round-3 PASS/PASS (downstream of STATE v1.4 round-4 PASS/PASS). Can a downstream spec's resolution *legitimately* supersede an upstream PASS/PASS line, OR does this require a Phase Z' check / STATE re-audit?
- Code-consistent? `src/state/typed_tx.rs:830-854` ships 4-variant SignalKind (Empty/Finalize/TaskExpired/TerminalSummary) — does the spec's emit table (§ 0.3.2) actually align with the shipped enum?
- Forward-compatible? CO1.9 will extend SignalKind. Does CO1.7.5 carry-forward block / harden / stay-neutral about CO1.9 extension?

**Q-B. D2 head_t close (§ 1 D2)**: spec proposes `q_w.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer_w.commit(&entry)?` in apply_one stage 9. Verify:
- Atomicity proof real? Spec claims "no failure point between commit success and head_t store under acquired lock". Walk through every line; could the AtomicU64::store, the `*q_w = q_next` move, or the field assignments fail in any way?
- NodeId disambiguation correct? `src/ledger.rs:13` has `pub type NodeId = String` (legacy; imported by bus.rs+kernel.rs); `src/state/q_state.rs:49` has `pub struct NodeId(pub String)` (new tuple struct). q.head_t is the new variant per `q_state.rs:311 pub head_t: NodeId`. Does the `state::q_state::NodeId(commit_oid_hex)` constructor call type-check?
- Trait method `LedgerWriter::head_commit_oid_hex(&self) -> Option<String>` default-None — Q1 OPEN: should the default be `unimplemented!()` instead, forcing every impl to declare? What's the case for / against each?

**Q-C. D3 combined STEP_B ceremony (§ 1 D3)**: spec rejects per-file STEP_B (Q3 closed) in favor of one A/B unit covering kernel.rs + bus.rs together. Justification: "Bus forwarder is meaningless without Kernel field; STEP_B Phase 0 minimum-sufficient version".
- Sound? Per `STEP_B_PROTOCOL.md` Phase 0, is "minimum sufficient" a binding criterion for ceremony scoping, or is it advisory?
- Architecture: Sequencer field lives in Kernel only; Bus forwards via `self.kernel.sequencer`. Bus stays struct-shape-compatible. Is this the right ownership boundary, or should Sequencer live above Kernel (e.g., in a runtime layer that owns Bus)?
- Risk: combined ceremony fails one A/B byte-identity check (either bus.rs or kernel.rs diverges) — does the spec specify what to do? (Restart whole ceremony, or split-and-redo?)

**Q-D. D1 transition body purity (§ 1 D1)**: spec § 1 D1 promises every transition body
- takes `(&QState, &TxVariant, &PredicateRegistry, &ToolRegistry)` and returns `Result<(QState, SignalBundle), TransitionError>`
- "no I/O, no env reads, no clock reads, no HashMap iteration, no f64 arithmetic on monetary values"
- mutates `q_next` cloned from `q`; returns byte-identically deterministic across processes

Verify against STATE § 3 Rust pseudocode: are there hidden non-deterministic deps? E.g., HashMap iteration in `q.economic_state_t.task_markets_t.get(...).map(|tm| tm.config.verifier_bond_on_slash)` (STATE § 3.2 line 537)? f64 in `prediction_market.rs:21-27,87-133`? STATE § 3.3 royalty rounding? Specific calls + lines.

**Q-E. Q5 mapping table completeness (§ 3.1 closure)**: spec Q5 closure table maps STATE § 3.1-3.7 rejection paths to 22 shipped TransitionError variants. Audit each:
- Are there rejection paths in STATE § 3 / § 3.1 / § 3.2 / § 3.3 / § 3.4 / § 3.6 / § 3.7 that the table missed?
- Are there shipped variants that are NEVER triggered in CO1.7.5? (Dead-variant audit.)
- Minimal-payload pattern: spec asserts "rich context flows via RejectedAttemptSummary side channel". Where IS RejectedAttemptSummary in the codebase? Verify the side-channel is real, not aspirational.

**Q-F. Q4 SignalKind closure (§ 0.3.2 emit table)**: 4 of 7 transition bodies emit `SignalKind::Empty`. Is "Empty" semantically correct for Work/Verify/Challenge/Reuse, or does this hide observable-state loss (e.g., reputation deltas in STATE § 3.2 line 530-531 silently dropped)?
- If observable-state loss: is CO1.9 the right home for the missing variants, or should CO1.7.5 add 1-2 Bool/Stat variants (forcing a CO1.1.4-pre1 micro-amendment)?
- Determinism: Empty signal vs full signal — both byte-identically deterministic, but the full signal exposes more API surface to L6 indices (CO1.9). Is the v1 minimization safe?

**Q-G. Spec ↔ smoke / patch consistency**: 12 patches applied (P1-P4 smoke, P5-P12 self-audit). Verify each P5-P12 patch is real in the v1 spec body (no spec ↔ patch-log drift), and verify smoke S1-S8 + footer claims hold against repo HEAD `334111a`.

**Q-H. New defects**: independent of audit prompt structure, what does the spec still get wrong?
- Spec claims CO1.7-impl is "PASS/PASS-equivalent" at commit `2461fe6`; does this hold? (Pre-implementation gate from CO1.7 § 12 — verify.)
- Spec claims `Git2LedgerWriter::head_commit_oid()` is exposed; verify line cited (`transition_ledger.rs:674`).
- Spec § 5 LoC estimate (1,000-1,560 LoC) — defensible given STATE § 3.x complexity?

**Q-I. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is the spec implementable end-to-end (i.e., CO1.7.5-impl can be written, audited, merged without scope explosion)? Specific blockers to call out — any v4-blocking deps that aren't in CO1.4-extra / CO1.7-impl / CO1.1.4-pre1 inheritance?

## Output format

# Codex CO1.7.5 Round-1 Audit
## Q-A Two-supersession framework
## Q-B D2 head_t close
## Q-C D3 combined STEP_B
## Q-D D1 transition body purity
## Q-E Q5 mapping table completeness
## Q-F Q4 SignalKind closure
## Q-G Spec ↔ smoke/patch consistency
## Q-H New defects
## Q-I Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers + source file line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean spec = PASS.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.7.5 spec v1 (target of audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: CO1.7 spec v1.2 (frozen, round-3 PASS/PASS) — supplies K3 v1.2 head_t resolution authority\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: CO1.1.4-pre1 spec (frozen, PASS/PASS) — supplies ABI lock authority\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS) — pseudocode authority for D1\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped src/state/sequencer.rs (current stub state, target of D1+D2)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/sequencer.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/state/typed_tx.rs (frozen ABI: TypedTx + 22 TransitionError + 4 SignalKind)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bottom_white/ledger/transition_ledger.rs (frozen Sequencer machinery; supplies head_commit_oid)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/state/q_state.rs (Q_t + NodeId tuple struct)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/q_state.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bus.rs (D3 STEP_B touch surface — current legacy ledger usage)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bus.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/kernel.rs (D3 STEP_B touch surface — current legacy ledger usage)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/kernel.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: hygiene OBS (path drift sediment, applied this session)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\nNow give your INDEPENDENT round-1 audit. Cite spec § + line where possible.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7.5 r1] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7.5 transition_bodies_and_runtime_wiring Round-1 Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: spec v1 DRAFT (self-audited; pre round-1)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7.5 r1] API returned in ${elapsed}s" >&2
echo "[codex co1.7.5 r1] saved: $OUT" >&2
