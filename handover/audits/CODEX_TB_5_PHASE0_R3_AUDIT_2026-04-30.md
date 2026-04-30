**Audit Mode**: Codex-only (single-auditor) per directive supplement
2026-04-30 (handover/directives/2026-04-30_TB5_audit_mode_supplement.md).
Gemini strategic-tier (gemini-2.5-pro or stronger) was unavailable at
audit time due to MODEL_CAPACITY_EXHAUSTED 429 errors on the
cloudcode-pa.googleapis.com endpoint. Degraded Gemini was deliberately
NOT invoked as substitute per parent directive § 4 Q4 ('不要把 degraded
Gemini 当作完整战略审计').

This Codex verdict IS the ship-gate authority for the audited subject.
There is no second strategic auditor to merge against. Per
feedback_dual_audit_conflict conservative-merge, single Codex VETO /
CHALLENGE / PASS controls. If strategic-tier Gemini becomes available
post-ship, an opportunistic supplemental verdict may be appended but
does NOT override this round's verdict.

**Round**: 3 (narrow). Scope: Q2/Q4/Q6/Q7 only per round-2 verdict's own
recommendation. Q1/Q3/Q5/Q8 PASS-ed in round 2 (or carried Q2/Q4
dependencies that round 3 closes); not re-audited.

## Q2 - Apply-One Stage 1.5 Exhaustiveness + L4.E Routing: PASS

Preflight §4.5 now requires stage 1.5 to be exhaustive over the four system variants and forbids direct-return rejection bypasses (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:395-399`). The helper sketch maps `ChallengeResolve`, `FinalizeReward`, `TaskExpire`, and `TerminalSummary` to their respective `CanonicalMessage::*Signing` arms (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:467-500`). The planned `ChallengeResolveSigning` arm is declared in §4.3 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:307-324`), while the existing three current system-signing arms are already present in `CanonicalMessage` and `canonical_digest()` (`src/bottom_white/ledger/system_keypair.rs:228-243`, `src/bottom_white/ledger/system_keypair.rs:472-483`).

The `record_rejection` helper is correctly factored from the current `apply_one` dispatch rejection path: current source snapshots Q, enters dispatch at `src/state/sequencer.rs:939-953`, builds CAS payload and diagnostic CIDs at `src/state/sequencer.rs:956-994`, falls back system-submitters to `SYSTEM_AGENT_ID_STR` at `src/state/sequencer.rs:996-1003`, appends L4.E at `src/state/sequencer.rs:1005-1020`, and returns `ApplyError::Transition` at `src/state/sequencer.rs:1022-1024`. The amended preflight helper preserves that shape (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:401-445`), and the stage 1.5 failure path calls `self.record_rejection(submit_id, &q_snapshot, &tx, &transition_err)?` before returning `ApplyError::Transition` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:448-461`).

The TerminalSummary epoch sentinel is defensible as a short-term fail-closed defense-in-depth path: current `TerminalSummaryTx` has no `epoch` field (`src/state/typed_tx.rs:353-363`), `SystemEpoch::new(0)` is constructible (`src/bottom_white/ledger/system_keypair.rs:40-48`), and `verify_system_signature` returns false if no key is pinned for the supplied epoch (`src/bottom_white/ledger/system_keypair.rs:501-508`). The preflight also flags the possible future source of truth as LedgerEntry envelope context (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:486-495`).

The required tests are now specified: U28/I66 assert L4.E append behavior, and the per-variant zero-signature coverage names for FinalizeReward, TaskExpire, and TerminalSummary are listed (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:506-512`). `src/state/sequencer.rs` remains unchanged at the genesis-pinned hash `783e2291c56871a028b860a6fe323d3adc5c00c0e82626cbb05dd40928196bfb` (`genesis_payload.toml:228`).

## Q4 - Monetary Invariant Cascade + ChallengeStatus Single Definition: CHALLENGE

The monetary cascade amendment itself is correct and minimal. Preflight §2 permits only `TypedTx::ChallengeResolve(_) => Ok(())` in `assert_no_post_init_mint` plus the fixture update, while explicitly keeping the 5-holding count and `total_supply_micro` unchanged (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:74-80`). Preflight §6.3 repeats the exact exhaustive-match arm and fixture requirement (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:712-740`), matching the current source locations: `assert_no_post_init_mint` is at `src/economy/monetary_invariant.rs:209-227`, and the fixture table is at `src/economy/monetary_invariant.rs:342-360`.

The 5-holding CTF invariant remains preserved in the unchanged source: `total_supply_micro` documents and counts balances, escrows, stakes, claims, and `challenge_cases_t.bond` only (`src/economy/monetary_invariant.rs:95-103`, `src/economy/monetary_invariant.rs:118-137`). The 9-sub-field `EconomicState` shape also remains unchanged in current source (`src/state/q_state.rs:151-166`) and is explicitly preserved by the preflight amendment (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:708-710`) and charter §4.4 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:126-134`).

The blocking issue is a remaining charter table contradiction on `ChallengeStatus`. Preflight §2 correctly says `typed_tx.rs` gets `ChallengeResolution` but not `ChallengeStatus`, and `q_state.rs` is the single definition site imported by `sequencer.rs` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:39-51`). Preflight §6.2 also binds `ChallengeStatus` to `src/state/q_state.rs` only and says `sequencer.rs` imports it via `use crate::state::q_state::ChallengeStatus;` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:659-685`). However charter §5.2 still attributes `ChallengeStatus` to `src/state/typed_tx.rs` and does not list the enum in the `q_state.rs` row (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:297-306`). This fails the round-3 Q4 verification item that both file-touch tables attribute `ChallengeStatus` to `q_state.rs`, not `typed_tx.rs`.

Concrete remediation: amend charter §5.2 so the `src/state/typed_tx.rs` row says `ChallengeResolveTx + ChallengeResolution` only, and the `src/state/q_state.rs` row says `ChallengeCase +status field + ChallengeStatus enum`. While touching the nearby text, align the stale atom work-breakdown row that still says Atom 4 puts `ChallengeStatus typed schema` in `typed_tx.rs` (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:283-292`).

## Q6 - Charter §5.3 To Preflight §8 Test Matrix Unification: CHALLENGE

Charter §5.3 has been updated to the intended richer matrix: T1-T5, U22-U33, I60-I89, per-variant I66.a/I66.b/I66.c zero-signature coverage, I88/I89, and `~37` / `~608/608` (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:309-372`). The test names in charter now use the corrected form, including `challenge_resolve_canonical_digest_is_deterministic` (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:313-316`).

Preflight §8 is not unified with that matrix. It lists T1-T5 and U22-U33 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:878-904`), but its integration list stops at I87 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:906-943`). It does not include I88/I89 in §8, does not list I66.a/I66.b/I66.c under I66, and still states `~33 new TB-5 tests` with target `~604/604` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:915-945`). This contradicts preflight §10/§11, which now claim `~37` / `~608`, I88/I89, and per-variant zero-sig additions (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:985-993`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:1026-1029`).

Concrete remediation: amend preflight §8 to exactly mirror charter §5.3 and preflight §10: add I66.a/I66.b/I66.c under I66, add I88/I89, update the integration count to about 17 plus the three I66 sub-tests, and update the target to `~608/608`. Then the statement that charter §5.3 mirrors preflight §8 will become true (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:309-312`).

## Q7 - Atom Plan Executability: PASS

The required Atoms 3 and 4 swap is present in charter §8: Atom 2 is the substrate ingress with no `ChallengeResolveTx` types, Atom 3 is the ABI plus `ChallengeStatus` and monetary cascade, and Atom 4 is `emit_system_tx` plus apply-one stage 1.5 with exhaustive verification and `record_rejection` (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:424-435`). Charter §8 also documents the compile-green dependencies, including Atom 3 before Atom 4 and the mandatory monetary cascade in Atom 3 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:439-446`).

Preflight §10 mirrors the executable plan. It records the reason for the swap (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:971-977`), includes the new Atom 1.5 row (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:979-984`), puts the ABI + `ChallengeStatus` + `monetary_invariant.rs` cascade in Atom 3 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:985`), and puts `emit_system_tx` + apply-one stage 1.5 in Atom 4 after the ABI exists (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:986-998`).

## Overall Round-3 Verdict: CHALLENGE

Q2 PASS, Q4 CHALLENGE, Q6 CHALLENGE, Q7 PASS. There is no VETO: the remaining failures are document consistency defects in the Atom 1.5 amendments, not source-code regressions. Observed `sha256sum src/state/sequencer.rs src/state/typed_tx.rs src/state/q_state.rs src/economy/monetary_invariant.rs` output:

```text
783e2291c56871a028b860a6fe323d3adc5c00c0e82626cbb05dd40928196bfb  src/state/sequencer.rs
9e0044486d3e53ff4768c4bfbb0e19c873f6f29ff142b3b5e130888f0bcd1593  src/state/typed_tx.rs
9d1ce20dd607f252efa4b6617451228bd9e5f2327b9e4c4a6ce1e3596bdab76a  src/state/q_state.rs
bdcedf6941b368ced72865fd1587887a6b1f68a703dd3f8440d1c7240f63d19f  src/economy/monetary_invariant.rs
```

Those match the trust-root entries for `src/economy/monetary_invariant.rs`, `src/state/q_state.rs`, `src/state/sequencer.rs`, and `src/state/typed_tx.rs` (`genesis_payload.toml:210`, `genesis_payload.toml:227-229`).

The verification anchor `cargo test --workspace 2>&1 | grep '^test result:' | awk '{p+=$4; f+=$6} END {print "PASS="p, "FAIL="f}'` produced `PASS=571 FAIL=0`, matching the expected TB-4 baseline.

## Top-3 Must-Fix Items

1. Fix charter §5.2 so `ChallengeStatus` is attributed to `src/state/q_state.rs`, not `src/state/typed_tx.rs` (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:297-306`).
2. Fix preflight §8 so it includes I66.a/I66.b/I66.c, I88/I89, and `~37` / `~608/608`, matching charter §5.3 and preflight §10 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:906-945`, `handover/tracer_bullets/TB-5_charter_2026-04-30.md:309-372`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:985-993`).
3. After those two doc edits, run a narrow round-4 recheck limited to Q4/Q6; Q2 and Q7 need not be re-opened unless the edits touch §4.5 or §10.

## Optional Improvements

- Make the TerminalSummary epoch convention explicit before any legitimate TerminalSummary emitter lands: either require a pinned epoch-0 key for `TerminalSummaryTx` verification or source the epoch from the ledger envelope context, as already contemplated by preflight §4.5 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:486-495`).
- Clean the stale charter §5.1 atom work-breakdown rows while fixing §5.2, because they still reflect pre-swap Atom 3/4 ownership (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:283-292`).

## Round-4 Narrowing Recommendation

Round 4 is warranted only after the two doc remediations above. Narrow it to Q4/Q6 document reconciliation: `ChallengeStatus` file ownership in charter §5.2, and exact charter §5.3 / preflight §8 test matrix equality.
