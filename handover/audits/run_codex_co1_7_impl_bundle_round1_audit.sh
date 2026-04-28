#!/usr/bin/env bash
# Codex round-1 audit on CO1.7-impl BUNDLE (A1+A2+A3+A4) + CO1.4-extra.
# Pre-CO1.7.5 implementation gate per spec § 12.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_impl_codex_round1.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Audit — CO1.7-impl BUNDLE Round-1 (Pre-CO1.7.5 implementation gate)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (parallel).

**Mandate**: round-1 dual external audit on the **CO1.7-impl bundle** (A1 + A2 + A3 + A4) + **CO1.4-extra** atom, all mid-implementation commits. Per CLAUDE.md "Audit Standard" + spec § 12: this PASS/PASS gates entry to CO1.7.5 (per-kind transition function bodies + STEP_B wiring to bus.rs/kernel.rs).

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Atoms in this bundle

| Atom | Commit | LoC | Tests | Description |
|---|---|---|---|---|
| **A1** Git2LedgerWriter | `a03cc52` | ~525 | 5 | git2-rs commit chain on `refs/transitions/main`; tree blobs (payload_cid / signature / entry_canonical); deterministic author time (logical_t, 0); reopen_recovers_chain test for cold-restart capability |
| **A2** Sequencer | `609d8d5` | ~290 (in 430 incl. tests) | 5 | K1 dual counter (next_submit_id / next_logical_t); tokio mpsc bounded queue; apply_one 9 stages per spec § 3; K3 head_t deferred to CO1.7.5+ wiring; ApplyError wrapper (impl deviation from spec § 3 line 307 — TransitionError stays closed-taxonomy + additive-only per CO1.1.4-pre1 § 7.2) |
| **A3** dispatch_transition | `609d8d5` | ~30 | 1 | Exhaustive 7-variant enum match; every variant returns NotYetImplemented stub (CO1.7.5 fills) |
| **A4** replay_full_transition | `272fcf4` | ~140 + LedgerCasView trait | 4 (1 #[ignore]) | 9-stage full-mode replay; new ReplayError::Transition variant; LedgerCasView trait (DIV-4 closure); tests prove stages 1-6 PASS, stage 7 stubs; #[ignore]'d test for full state_root reconstruction (CO1.7.5) |
| **CO1.4-extra** | `b6b7574` | ~110 + tests | 5 | Sidecar JSONL CAS index persistence; closes Art 0.2 cold-replay gate per CO1.1.4-pre1 § 0.1 cross-atom ordering |

**Total**: ~1100 LoC + 20 conformance tests + 1 #[ignore]'d. **234/0 → 237/0 lib PASS**.

## Companion specs (frozen, read first)

- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — pre-implementation PASS/PASS (rounds 1-3)
- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` v1.2.2 — ABI surface PASS/PASS (rounds 1-5)
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — frozen invariants + § 3 transition pseudocode

## What is at stake

- **PASS/PASS** unblocks **CO1.7.5** (per-kind transition function bodies + STEP_B wiring to bus.rs/kernel.rs); CO1.7.5 is the final L4 atom
- **CHALLENGE** triggers v1.1 patch round on the bundle
- **VETO** would mean a foundational design issue requiring re-spec

## Round-1 audit questions

**Q-A. A1 (Git2LedgerWriter) integration discipline**: tree-blob choice (payload_cid + signature + entry_canonical) vs original commit-message-body approach (which had a binary normalization bug). Is the deterministic author time (`(logical_t as i64, 0)`) sufficient for byte-identical commit OIDs across runs? Are the 5 tests load-bearing (open_recovers_chain proves cold-restart; what else might miss)?

**Q-B. A2 Sequencer K1 invariant + K3 deferred head_t**: K1 — submit advances submit_id; commit advances logical_t. Verify rejected-submission path does NOT advance logical_t (test `apply_one_stub_does_not_consume_logical_t` covers stub mode; what about future real-transition rejection path)? K3 — head_t mutation deferred to CO1.7.5+. Is leaving `q.head_t` un-mutated (still at QState::default empty string) during the entire CO1.7-impl runtime ACCEPTABLE for Art 0.4 (Q_t = ⟨q_t, HEAD_t, tape_t⟩) compliance? Or is this a gap that should at least set head_t to a placeholder pointing at the latest commit OID?

**Q-C. A2 Sequencer ApplyError vs spec § 3 line 307 deviation**: spec writes apply_one signature as `Result<LedgerEntry, TransitionError>`; impl widens to `Result<LedgerEntry, ApplyError>` with TransitionError as a wrapped variant. **Defensible vs spec drift?** Argument for: TransitionError stays closed-taxonomy (CO1.1.4-pre1 § 7.2 additive-only commitment); infra errors (CasError / KeypairError / LedgerWriterError) shouldn't pollute it. Argument against: spec is authoritative, deviations need explicit closure.

**Q-D. A2 Sequencer concurrency / panic safety**: apply_one is sync and acquires multiple `RwLock` write guards (Q_t + writer). What happens if apply_one panics mid-way? Q_t lock poisoned (we map to ApplyError::QStateLockPoisoned but the sequencer task continues). Is this acceptable? Is there a runtime concern about run() being a long-running async task that loses progress on panic?

**Q-E. A3 dispatch_transition exhaustive match contract**: every variant returns NotYetImplemented. Is this the right pattern for "transition body not yet implemented", or should the stubs use unimplemented!() macro for clearer crash semantics? (Tests verify the Err return path; CO1.7.5 fills bodies preserving the match arms.)

**Q-F. A4 replay_full_transition staging**: 9 stages per spec § 4. Implementation order (1-2-3 chain checks → 4 sig verify → 5 CAS lookup → 6 decode → 7 dispatch → 8 state_root match → 9 ledger_root match) — defensible? Or should sig verify come AFTER CAS lookup (cheaper to fail-fast on missing payload)?

**Q-G. A4 LedgerCasView trait**: narrow read-only interface. Necessary for testability + future MetaCas backend, OR over-engineering since CasStore is the only impl?

**Q-H. CO1.4-extra sidecar JSONL discipline**: append BEFORE in-memory insert (so crash mid-write keeps runtime consistent — durable+memory both present, or neither). Strict mode on corrupted JSONL line (returns IndexParse error vs skip-and-warn). Both correct?

**Q-I. CO1.4-extra durability gap on idempotent put**: idempotent put short-circuits before sidecar write (correct — content is already durable from prior put). But what about a partial write that left the in-memory index updated but sidecar not flushed? Current code appends THEN inserts — addresses this. Confirm no other gap.

**Q-J. Cross-atom A2↔A4 consistency**: Sequencer.apply_one stages 5-9 (sign + ledger_root fold + commit) vs replay_full_transition stages 4-9 (verify + decode + dispatch + state_root + ledger_root). Are these symmetric — does every byte the sequencer writes get verified by replay? Specifically: signing payload digest pre-image must be EXACTLY what apply_one constructed at stage 5 vs what replay reconstructs at stage 4.

**Q-K. New defects independent of catalog**:
- Type errors that cargo check missed?
- Spec ↔ code parity drift?
- Test gaps: anything from STATE_TRANSITION_SPEC § 4 (27 invariants) that CO1.7-impl bundle should provide a witness for but doesn't?
- Doc-comment hygiene (lesson #11 from CO1.1.4-pre1)?

**Q-L. Implementation gating**: with bundle at 237/0 PASS, is CO1.7.5 (per-kind transition bodies + STEP_B bus.rs/kernel.rs wiring) implementable end-to-end against this bundle? Specific blockers to call out.

## Output format

# Codex CO1.7-impl Bundle Round-1 Audit
## Q-A A1 Git2LedgerWriter
## Q-B A2 K1 + K3 deferred head_t
## Q-C A2 ApplyError vs spec § 3 line 307
## Q-D A2 panic safety
## Q-E A3 stub pattern
## Q-F A4 staging order
## Q-G A4 LedgerCasView trait
## Q-H CO1.4-extra sidecar discipline
## Q-I CO1.4-extra durability gap
## Q-J cross-atom A2↔A4 symmetry
## Q-K New defects
## Q-L Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite line numbers. Real defects = CHALLENGE; foundational design flaw = VETO; clean bundle = PASS.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
printf '\n# CO1.7 spec v1.2 (already PASS/PASS)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# CO1.1.4-pre1 spec v1.2.2 (already PASS/PASS)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Implementation A1+A4: src/bottom_white/ledger/transition_ledger.rs\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Implementation A2+A3: src/state/sequencer.rs\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/sequencer.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Implementation CO1.4-extra: src/bottom_white/cas/store.rs\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/cas/store.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting: src/state/typed_tx.rs (TypedTx ABI; PASS/PASS)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting: src/bottom_white/ledger/system_keypair.rs (signing primitives)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting: src/state/q_state.rs (QState shape + indices)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/q_state.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-1 audit on the bundle. Cite line numbers.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7-impl bundle r1] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7-impl Bundle Round-1 Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: A1+A2+A3+A4 + CO1.4-extra (all mid-implementation commits)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7-impl bundle r1] API returned in ${elapsed}s" >&2
echo "[codex co1.7-impl bundle r1] saved: $OUT" >&2
