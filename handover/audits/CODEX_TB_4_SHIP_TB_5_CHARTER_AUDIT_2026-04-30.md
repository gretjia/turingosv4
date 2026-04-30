# Codex External Audit — TB-4 Ship + TB-5 Charter v1

Date: 2026-04-30
Repo HEAD audited: `cca74b5833d1efeebc2902e4a8223b14835d342a`

## Verification Anchors

- `cargo test --workspace 2>&1 | grep "^test result:" | awk ...` observed: `PASS=571 FAIL=0` (expected `PASS=571 FAIL=0`).
- `sha256sum src/state/sequencer.rs src/state/typed_tx.rs src/state/q_state.rs` observed:
  - `783e2291c56871a028b860a6fe323d3adc5c00c0e82626cbb05dd40928196bfb  src/state/sequencer.rs`
  - `9e0044486d3e53ff4768c4bfbb0e19c873f6f29ff142b3b5e130888f0bcd1593  src/state/typed_tx.rs`
  - `9d1ce20dd607f252efa4b6617451228bd9e5f2327b9e4c4a6ce1e3596bdab76a  src/state/q_state.rs`
- These match the Trust Root manifest at [genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:227), [genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:228), and [genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:229).
- `grep -rn "NoStakeTx\|VerifierBondTx\|ChallengeStakeTx\|VerifierStakeTx" src/` observed zero hits.
- `cargo test --test tb_3_bridge_deletion_invariant` observed 2 passed / 0 failed; the scanner and positive control are defined at [tests/tb_3_bridge_deletion_invariant.rs](/home/zephryj/projects/turingosv4/tests/tb_3_bridge_deletion_invariant.rs:1).
- Targeted invariant checks observed green: `economic_state_has_nine_sub_fields` at [src/state/q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:460) and `ctf_counts_all_five_holding_subindexes` at [src/economy/monetary_invariant.rs](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:485).
- Lean re-verification attempt: `proof_mathd_algebra_125.lean` typechecked, but emitted one warning for unused `h₀` at [handover/evidence/tb_4_medium_batch_2026-04-30/proof_mathd_algebra_125.lean](/home/zephryj/projects/turingosv4/handover/evidence/tb_4_medium_batch_2026-04-30/proof_mathd_algebra_125.lean:18). This is not zero diagnostics, although it is not a typecheck failure.

## Part A (TB-4 ship)

A1: CHALLENGE [line-grounding mismatch; remediation patch is documentation-only]

The implementation matches the TB-4 RSP-2 shape: `TypedTx` has exactly 9 variants and no phantom variant at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:739); `EconomicState` has 9 sub-fields at [src/state/q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:155); `ChallengeCase.target_work_tx` is additive at [src/state/q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:337); Verify and Challenge dispatch arms perform balance-to-stake/case transfers at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:379) and [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:439); `WorkTx` has no status field at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:222); and `VerifyTx.verdict` is never read by the dispatch arm, only documented as L4 signal at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:375).

The miss: TB-4 charter v2 quotes `opened_at_round: q.logical_t` at [handover/tracer_bullets/TB-4_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-4_charter_2026-04-30.md:148) and [handover/tracer_bullets/TB-4_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-4_charter_2026-04-30.md:195), but the shipped code writes `q.q_t.current_round` at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:480), and I43 asserts `current_round` at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:721). ROADMAP current state also records `opened_at_round = q.q_t.current_round` at [handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md](/home/zephryj/projects/turingosv4/handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md:216). Remediation: patch TB-4 charter v2 occurrences of `q.logical_t` / `q_logical_t_at_accept` to `q.q_t.current_round` / `q_current_round_at_accept`; no source change required.

A2: PASS

Sampled forbidden lines are enforced: no new `TypedTx` variants beyond the 9 at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:745); I44 scans `src/` for the four phantom names at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:959) and [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:986); its positive control proves the scanner is live at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:1022). Rejected Verify/Challenge do not mutate `economic_state_t` at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:678). No `current_round - opened_at_round` or equivalent closure arithmetic exists in `src/` by grep; the only `opened_at_round` source write is the anchor at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:480).

A3: PASS

The three ship proofs are backed by the named tests: Verify path I31/I33/I35/I37 at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:246), Challenge path I32/I34/I36/I38/I39/I40 at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:415), and replay/property/window/no-drift I41-I44 at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:781). Full local `cargo test --workspace` observed `PASS=571 FAIL=0`.

A4: PASS

Directive Q1 no hard dedup is exercised by second Challenge/Verify accepts at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:923). Q2 parent-root schema bumps are present at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:240) and [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:266). Q3/Q7 error variants are present at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:964). Q4 multi-challenger is explicit at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:593). Q6 reputation is untouched by the dispatch arms; `reputations_t` is only a field declaration at [src/state/q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:161). Anti-drift clauses RSP-2 != RSP-3 / Verify != judge / Challenge != slash are reflected by no `remove`/slash behavior in the Verify/Challenge arms at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:379) and [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:439).

A5: PASS

WP-canonical reconciliation is preserved. Source grep has zero hits for `NoStakeTx` / `VerifierBondTx` / `ChallengeStakeTx` / `VerifierStakeTx`; the `TypedTx` enum contains `Verify` and `Challenge` but no phantom stake/bond variants at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:745). I44 is genuine and has a positive control at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:1025).

A6: PASS

Silent-regression checks pass locally. The bridge invariant test passed 2/0 and its scanner is the TB-3 guard. The 9-sub-field invariant is tested at [src/state/q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:460). The 5-holding CTF sum is implemented at [src/economy/monetary_invariant.rs](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:95) and tested at [src/economy/monetary_invariant.rs](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:485).

## Part B (TB-5 charter v1)

B1: PASS

`ChallengeResolveTx` is correctly classified as first-class allowed-named, not phantom. ROADMAP names `challenge_resolve_tx` in the P3 transaction list at [handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md](/home/zephryj/projects/turingosv4/handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md:178). WP § 19 names ChallengeCourt as "挑战期 + 反例 + 冻结 + 回滚 + slash" at [handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:87). The charter distinguishes allowed-named from forbidden-phantom at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:45) and [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:48).

B2: VETO [structural blocker: system-emitted authority is specified but not enforceable by the proposed dispatch shape]

The schema mirrors existing system-emitted forms: `FinalizeRewardTx` has `epoch` and `system_signature` at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:312), `TaskExpireTx` has the same system shape at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:328), and system-emitted `HasSubmitter` impls return `None` at [src/state/typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:810). TB-5 correctly proposes the same fields at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:80).

The blocker is enforcement. The charter says "all submitted ChallengeResolveTx with valid system signature accept" at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:201), but its dispatch pseudocode never verifies `resolve.system_signature` at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:300). Current `Sequencer::submit` accepts a bare `TypedTx` from any caller at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:863), and `dispatch_transition` has no keypair or pinned-pubkey argument at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:262). The project has a verifier primitive, but it requires pinned pubkeys at [src/bottom_white/ledger/system_keypair.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:500). Without redesign, an arbitrary caller can submit a `ChallengeResolveTx{Released}` payload that refunds a bond and removes a challenge case, which violates the charter's own system-only requirement at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:57).

Required redesign: before TB-5 implementation, add an enforceable system-tx ingress boundary. Acceptable shapes are either (1) a `Sequencer::emit_system_tx(...)` path that constructs/signs `ChallengeResolveTx` internally and keeps `submit()` agent-only, or (2) a pre-dispatch signature verifier with pinned system pubkeys passed into the sequencer and a rejection test proving an all-zero or wrong-epoch `system_signature` cannot mutate Q_t. Add tests for invalid system signature, public-submit system tx rejection, and valid system tx acceptance.

B3: PASS

The entry-shape additives are scoped correctly: `accepted_at_round` is appended to `StakeEntry` with `#[serde(default)]`, and `challenge_window_length` is appended to `TaskMarketEntry` with a default function at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:236). They do not add `EconomicState` sub-fields, whose 9-field shape is at [src/state/q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:155). v2 should add missing-field serde tests for both entry types, not just accepted-at-round.

B4: PASS

The Released vs UpheldDeferred split is atomically clean once B2's authority blocker is fixed. Released refunds `balances_t[case.challenger]` and removes exactly one `challenge_cases_t` row at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:137). UpheldDeferred has zero Q_t mutation and leaves the ChallengeCase for TB-6 at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:142). The no solver/verifier release boundary is explicit at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:158).

B5: CHALLENGE [boundary is right, but forbidden #26 depends on the B2 redesign]

The slash/settlement/reputation boundary is otherwise clean: no slash execution at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:417), no `SlashTx`/`SettlementTx`/`ProvisionalAcceptTx` at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:418), no window-closure math at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:422), and no predicate evaluation of counterexamples at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:426). Remediation: amend forbidden #26 with the concrete system-tx ingress design from B2.

B6:

  Q1: PASS. Use global default `10` for TB-5 minimum scope; the charter default function is at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:267), and the recommendation avoids a TaskOpenTx schema bump at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:509).

  Q2: PASS. Set `accepted_at_round` on both Solver YES stake and Verifier bond; the plan states this at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:272) and [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:285).

  Q3: CHALLENGE. `SystemSignatureForbiddenAtAgentSubmit` is not useful as a reserved no-code-path variant if the real problem is that `submit()` has no caller identity at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:863). Remediation: either drop the variant, or keep it only after introducing an agent-vs-system ingress split that can actually emit it.

  Q4: PASS. Prefer enum over bool; the charter's `Released | UpheldDeferred` enum is at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:90), and the growability rationale is sound at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:512).

  Q5: CHALLENGE. Recommend Option A, not Option B. The charter says Option B mirrors TB-4 at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:496), but TB-4 charter says it restored dual-audit discipline at [handover/tracer_bullets/TB-4_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-4_charter_2026-04-30.md:548). Remediation: make Option A the default for TB-5 v2; allow Option B only after B2 is redesigned and separately authorized.

  Q6: PASS. Do not remove the ChallengeCase on UpheldDeferred; the charter preserves the slash target at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:514).

  Q7: PASS. Do not add `ProvisionalAcceptTx` in TB-5; the charter keeps provisional state implicit via `stakes_t` and defers explicit form to later RSP at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:515).

B7: CHALLENGE [audit-mode recommendation should be Option A]

TB-5 is not just "smaller than TB-4" in risk terms: it adds a new system-emitted variant that can move money and remove a challenge case at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:137). Combined with the B2 authorization blocker, self-audit + smoke is insufficient. Concrete charter v2 patch: replace "Charter v1 recommendation: Option B" at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:501) with "Option A until the system-tx ingress/signature-verification design is audited green."

B8: VETO [unstated dependency: system-tx authorization substrate]

The charter correctly states no current-round mutation at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:171), and TB-4's `ChallengeCase` already carries `target_work_tx` at [src/state/q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:337). The missing dependency is an enforceable system-emitted transaction ingress. Current `SubmissionEnvelope` contains only `{ submit_id, tx }` at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:669), so there is no way to distinguish "operator/system emitted" from "agent submitted" at dispatch time. Required redesign is the B2 system-tx ingress/signature-verification substrate before TB-5 can proceed.

## Overall Verdict

PART A: CHALLENGE

The shipped TB-4 code and tests are sound under local verification (`PASS=571 FAIL=0`, Trust Root matches, anti-drift grep clean), but the charter/source wording mismatch on `opened_at_round = q.logical_t` versus the actual `q.q_t.current_round` anchor must be corrected so the ship record is line-grounded.

PART B: VETO

TB-5 charter v1 has a structural system-authority hole: it proposes a state-mutating `ChallengeResolveTx` but does not specify an enforceable system-keypair-only ingress or signature verification path. This must be redesigned before implementation.

## Top-3 Must-Fix Items

1. B2/B8 system-tx authority redesign: add `emit_system_tx` or a pinned-pubkey pre-dispatch verifier; prove invalid system signatures and public-submit system tx cannot mutate Q_t. Evidence: current `submit()` accepts bare `TypedTx` at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:863), while TB-5 Released mutates money/cases at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:137).
2. B7 audit-mode patch: make TB-5 v2 default to Option A narrow dual external audit, not Option B self-audit, because TB-5 adds a new system-emitted economic mutator and the charter's TB-4 precedent claim conflicts with TB-4's dual-audit text at [handover/tracer_bullets/TB-4_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-4_charter_2026-04-30.md:548).
3. A1 TB-4 ship-record patch: replace `q.logical_t` wording with `q.q_t.current_round` in TB-4 charter v2 and derived audit prose. Evidence: charter says `q.logical_t` at [handover/tracer_bullets/TB-4_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-4_charter_2026-04-30.md:148), source writes `current_round` at [src/state/sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:480).

## Optional Improvements

- Add explicit missing-field serde tests for both `StakeEntry.accepted_at_round` and `TaskMarketEntry.challenge_window_length`, including old-shape fixture deserialization. The charter currently lists I52 only for accepted-at-round at [handover/tracer_bullets/TB-5_charter_2026-04-30.md](/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-5_charter_2026-04-30.md:398).
- Fix the Lean proof artifact warning in `proof_mathd_algebra_125.lean` if "zero diagnostics" remains the published reproducibility standard; the proof typechecks, but Lean emitted an unused-variable warning at [handover/evidence/tb_4_medium_batch_2026-04-30/proof_mathd_algebra_125.lean](/home/zephryj/projects/turingosv4/handover/evidence/tb_4_medium_batch_2026-04-30/proof_mathd_algebra_125.lean:18).
- Extend TB-5 anti-drift scanner with positive controls for the new forbidden literals (`SlashTx`, `SettlementTx`, `ProvisionalAcceptTx`, `ReputationUpdateTx`) while preserving the existing phantom-variant positive control at [tests/tb_4_rsp2_admission_surface.rs](/home/zephryj/projects/turingosv4/tests/tb_4_rsp2_admission_surface.rs:1025).
