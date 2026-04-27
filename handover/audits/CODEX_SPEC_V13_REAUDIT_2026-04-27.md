# Codex Spec v1.3 Re-Audit (CO1.SPEC.0.5 round 3)

Spec SHA checked: `bda052e76392f37039759361acc0bbe7b8b9b1a7bc17c2438d01ee7b4ffbe497`.
Patch commit checked: `1e3f2ff`.

Audit note: `CODEX_SPEC_V12_REAUDIT_2026-04-27.md` contains 8 `PARTIAL` rows + 1 `NOT-CLOSED` row + 5 `NEW` issues = 14 named residual rows, while the round-3 prompt says 13. I audited all 14 so no residual is silently dropped.

## Per-residual closure check

| item-id | round-2 status | round-3 status | evidence |
|---|---:|---:|---|
| Q1.1 transition coverage matrix / legacy economic tx disposition | NOT-CLOSED | PARTIAL | v1.3 adds a legacy disposition table for `Invest`, `TaskMarketPublish`, `MarketCreate`, `MarketResolve`, `RunEnd`, WAL, and tool hooks at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:981-995`. However the patch log says `TaskMarketPublishTx` is retired at `:12`, while § 5.3 says `TaskMarketPublishTx` is a new v1 transition deferred to CO P2.1 at `:988`, and the specified retirement grep at `:995` does not include `TaskMarketPublish`. Current legacy economic surfaces still exist in source, e.g. `src/bus.rs:229-252`, `src/bus.rs:285-290`, `src/bus.rs:336-369`, `src/kernel.rs:114-126`, and `src/kernel.rs:156-206`, which is expected pre-CO1.1.4/1.1.5 but reinforces that closure is only spec-level. |
| Q1.3 task expiry / bounty refund transition | PARTIAL | CLOSED | `task_expire_transition` is now pure over `(q, tx)` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:705-709`; signature/parent checks are inside the pure transition at `:713-720`; the expiry guard now rejects any claim for the task at `:723-730`; runtime logical-time assignment and signing are explicitly before pure transition entry at `:769-776`. |
| Q1.4 agent implicit init | PARTIAL | CLOSED | v1.3 defines `HasSubmitter` and per-tx `submitter_id()` for `WorkTx`, `VerifyTx`, `ChallengeTx`, and `ReuseTx` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:782-792`; the init helper uses the resolved submitter at `:795-805`; the rule requires every agent-submitted transition to call it first in stage 4 at `:808`. This is verifiable from spec text, though the earlier pseudocode snippets are not duplicated with the call inline. |
| Q2.3 `I-FINALIZE-BATCH-ORDER` | PARTIAL | CLOSED | The invariant uses `(expires_at_logical ASC, claim_id ASC)` and explicitly says not `target_work_tx` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:882`; § 5.2.3 uses `claim.claim_id` at `:964-967`; the conformance test also asserts `(expires_at, claim_id)` at `:971-973`. |
| Q2.4 `I-CHALLENGE-WINDOW-EDGE` | PARTIAL | PARTIAL | The invariant now defines the half-open interval and states both challenge and finalize must use `is_open(q.q_t.current_round)` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:883`. But the actual challenge pseudocode still hand-codes `tx.timestamp_logical >= window.opens_at + window.duration_ticks` at `:481-487`, while finalize still calls `w.is_open()` with no `now` argument at `:638-642`. `rg` finds no `ChallengeWindow::is_open(now)` definition in spec or source, so the binding is not mechanically expressed in the transition pseudocode. |
| Q5 STEP_B canonical serialization / ABI / fixtures | PARTIAL | PARTIAL | § 2.5 still specifies bincode config and tx fixture requirements at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:285-316`, but full golden fixture corpus and differential fuzz seed remain deferred at `:318`. The spec still does not pin a runner ABI or golden byte outputs for `QState`, `SignalBundle`, and `TransitionError`; this residual cannot be closed from v1.3 spec text. |
| Q6 concurrency sequencer / cross-cell isolation / finalize order | PARTIAL | PARTIAL | v1.3 keeps one sequencer per `(runtime_repo, run_id)` and serial commit at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:941-951`, cross-cell isolation at `:953-960`, and fixes finalize order at `:962-973`. Remaining gap: the sequencer still "receives tx submissions in any order" and executes queue "submission order" at `:943-948` without a deterministic tie-breaker for simultaneous concurrent submissions; the test at `:971` only proves same input order gives same state root. |
| Q7 MicroCoin P1 prerequisite | PARTIAL | CLOSED | Plan v3.2-fix3 marks CO P2.0a deprecated/promoted at `handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md:137-143`, defines CO1.0a as the blocking MicroCoin precondition for CO1.7 at `:143-154`, places CO1.0a on the CO P1 critical path at `:202-215`, and marks CO P2.0a deprecated in P2 at `:221-222`. |
| Q10 false-challenge contradiction and invariant count cleanup | PARTIAL | CLOSED | The old v1.1 patch-log wording is now explicitly obsolete and fixed to 0 / not configurable at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:25-28`; § 5.1 says only 11.2/11.3/11.4 are user-overridable and 11.1 is not at `:926-933`; invariant count remains 27 at `:854-886`. |
| NEW-1 task_expire_transition not pure | NEW | CLOSED | The runtime argument was removed from the transition signature at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:705-709`; runtime signing/logical time happens before sequencer entry at `:769-776`. |
| NEW-2 task expiry can refund accepted/provisional claims | NEW | CLOSED | Stage 3 now rejects if `claims_t.any_claim_for_task(tx.task_id)` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:723-730`, not only finalized claims. |
| NEW-3 agent implicit init references non-existent `tx.agent_id` for all tx types | NEW | CLOSED | `HasSubmitter` maps `WorkTx.agent_id`, `VerifyTx.verifier_agent`, `ChallengeTx.challenger_agent`, and `ReuseTx -> None` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:782-792`. |
| NEW-4 finalize batch order internally inconsistent (`claim_id` vs `target_work_tx`) | NEW | CLOSED | Invariant, § 5.2.3, and test all use `claim_id`: `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:882`, `:964-967`, `:971-973`. |
| NEW-5 STEP_B serialization not mechanically pinned | NEW | PARTIAL | Same as Q5: § 2.5 pins bincode config for tx digest at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:285-316`, but full fixtures/fuzz seed remain deferred at `:318`, and runner ABI plus output bytes for `QState`, `SignalBundle`, and `TransitionError` are still not specified. |

## Regression check on previously CLOSED items

| item-id | round-3 regression status | evidence |
|---|---:|---|
| Q1.2 finalize stake unlock | NOT-REGRESSED | Finalize still unlocks and credits the solver stake at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:662-667`; `I-STAKE-RETURN` remains at `:880`. |
| Q2.1 `I-STAKE-RETURN` | NOT-REGRESSED | The named invariant remains present at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:880`, backed by finalize stage 3a at `:662-667`. |
| Q2.2 `I-BOUNTY-REFUND` | NOT-REGRESSED | The named invariant remains present at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:881`; task expiry refunds bounty and locked stakes at `:732-741`, with the stricter no-claim guard at `:723-730`. Minor wording note: invariant text still says "when no claim finalized", while the transition now says no claim may exist. |
| Q3.1 hidden-input table expansion | NOT-REGRESSED | Hidden input classifications for bounty env vars, Boltzmann/seed, broad `HashMap`, async ordering, WAL/git effects, and f64 money remain at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:267-273`; tests remain listed at `:278-283`. |
| Q3.2 royalty rounding rule | NOT-REGRESSED | Royalty math still uses micro-units, checked multiplication, and integer floor division by `1_000_000` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:673-687`; f64 money remains banned/promoted at `:273`. |

## Any new issues introduced by v1.3 patches

1. `TaskMarketPublishTx` disposition is internally inconsistent: the v1.3 patch log says it is retired in CO1.1.4 at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:12`, while § 5.3 says it is a new v1 transition deferred to CO P2.1 at `:988`, and the conformance grep at `:995` omits it.
2. The challenge-window edge rule is asserted but not embodied in pseudocode: the invariant requires `is_open(q.q_t.current_round)` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:883`, but challenge uses a direct timestamp comparison at `:481-487`, finalize uses `w.is_open()` with no argument at `:638-642`, and no method definition is present.
3. `TaskExpireTx.bounty_refunded` is signed and ledgered but not validated against the escrow amount: the field is declared at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:757-765`, the transition computes `bounty` from state at `:732-736`, and then appends the original tx at `:747-752` without checking `tx.bounty_refunded == bounty`.

## Holistic verdict: PASS / CHALLENGE / VETO

Verdict: CHALLENGE.

v1.3 closes several round-2 blockers, especially task expiry purity/race behavior, submitter resolution, finalize order, false-challenge prose, and the MicroCoin critical path. It does not fully close the spec-freeze gate because STEP_B serialization/ABI remains under-specified, L4 concurrent submission ordering is still not deterministic, and the challenge-window edge rule remains split between normative invariant text and inconsistent pseudocode.

No VETO: the remaining issues are fixable spec amendments.

## CO P1 launch (for CO1.1.4/1.1.5/1.7): GO / NO-GO / NEEDS-FIX

Recommendation: NO-GO for CO1.1.4, CO1.1.5, and CO1.7 until the PARTIAL rows above are patched.

Read-only substrate/spike work can continue, but STEP_B implementation branches should not start against this artifact because Q5/NEW-5 and Q6 still leave room for branch divergence.

## Top-3 residual concerns

1. STEP_B is still not mechanically pinned: no runner ABI, no byte-level fixtures for `QState` / `SignalBundle` / `TransitionError`, and full fixture/fuzz corpus remains deferred.
2. L4 sequencer ordering still relies on unspecified concurrent queue submission order; same input order determinism is not enough for simultaneous submissions.
3. v1.3 introduced/left contradictions in normative-vs-pseudocode areas: `TaskMarketPublishTx` retired vs deferred, and `is_open(now)` required vs not used consistently in transition pseudocode.
