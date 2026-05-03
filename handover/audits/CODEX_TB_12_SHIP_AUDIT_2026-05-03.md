# Codex TB-12 Ship Audit (Class 3 implementation-paranoid)

**Verdict**: CHALLENGE

## Q1 NodePosition second money ledger?
PASS.

Evidence: `EconomicState` adds `node_positions_t` as a serde-defaulted flat index, with comments saying it is not a Coin holding and is not counted in `total_supply_micro` (`src/state/q_state.rs:176`, `src/state/q_state.rs:179`, `src/state/q_state.rs:191`). The monetary sum in `total_supply_micro` iterates balances, escrows, stakes, and challenge cases only (`src/economy/monetary_invariant.rs:154`, `src/economy/monetary_invariant.rs:157`, `src/economy/monetary_invariant.rs:160`, `src/economy/monetary_invariant.rs:169`), and there is no `node_positions_t` term in that function (`src/economy/monetary_invariant.rs:152`, `src/economy/monetary_invariant.rs:172`).

WorkTx existing money movement remains balance debit plus `stakes_t` credit (`src/state/sequencer.rs:544`, `src/state/sequencer.rs:550`), then the NodePosition side-effect writes only `node_positions_t` (`src/state/sequencer.rs:572`, `src/state/sequencer.rs:586`). ChallengeTx existing money movement remains balance debit plus `challenge_cases_t` credit (`src/state/sequencer.rs:796`, `src/state/sequencer.rs:802`), then the NodePosition side-effect writes only `node_positions_t` (`src/state/sequencer.rs:825`, `src/state/sequencer.rs:847`). Inference: CR-12.1 and CR-12.2 are satisfied for NodePosition itself because `NodePosition.amount` is documented as not a Coin holding (`src/state/typed_tx.rs:658`, `src/state/typed_tx.rs:660`) and is not included in the conservation sum.

## Q2 Replay deterministic?
PASS.

Evidence: `sg_12_5_node_positions_replay_deterministic` runs two fresh harnesses with identical inputs and asserts equal `NodePositionsIndex` values (`tests/tb_12_node_exposure_index.rs:340`, `tests/tb_12_node_exposure_index.rs:367`, `tests/tb_12_node_exposure_index.rs:369`). True ChainTape replay re-decodes each `TypedTx` from CAS and re-runs `dispatch_transition` for every L4 entry (`src/bottom_white/ledger/transition_ledger.rs:461`, `src/bottom_white/ledger/transition_ledger.rs:468`, `src/bottom_white/ledger/transition_ledger.rs:487`). The replay helper used by `lean_market view-positions` returns reconstructed `QState` through `replay_full_transition` (`experiments/minif2f_v4/src/bin/lean_market.rs:648`, `experiments/minif2f_v4/src/bin/lean_market.rs:706`).

Field derivation is deterministic: WorkTx positions use `work.tx_id`, `work.task_id`, `work.agent_id`, `work.stake.0`, and `work.timestamp_logical` (`src/state/sequencer.rs:573`, `src/state/sequencer.rs:582`). ChallengeTx positions use `challenge.tx_id`, `challenge.target_work_tx`, `challenge.challenger_agent`, `challenge.stake.0`, `challenge.timestamp_logical`, and task_id derived from `stakes_t[target_work_tx]` (`src/state/sequencer.rs:827`, `src/state/sequencer.rs:843`). The field-locking test verifies FirstLong and ChallengeShort source-field equality, including `ChallengeShort.node_id == target_work_tx` and not its own tx id (`tests/tb_12_node_exposure_index.rs:448`, `tests/tb_12_node_exposure_index.rs:465`). Inference: no environment, time, or randomness input participates in NodePosition field derivation; all fields come from typed tx payload or prior QState.

## Q3 VerifyTx bond avoid market classification?
PASS.

Evidence: PositionSide comments state VerifyTx.bond is neither Long nor Short and remains a responsibility bond (`src/state/typed_tx.rs:606`, `src/state/typed_tx.rs:610`). The VerifyTx arm locks bond into `stakes_t` and may create a claim on Confirm, but it contains no NodePosition write (`src/state/sequencer.rs:611`, `src/state/sequencer.rs:650`, `src/state/sequencer.rs:683`, `src/state/sequencer.rs:756`). A source grep found `node_positions_t` writes in `src/state/sequencer.rs` only at the WorkTx and ChallengeTx write sites (`src/state/sequencer.rs:586`, `src/state/sequencer.rs:847`).

SG-12.3 locks this behavior: after a VerifyTx, the position count remains 1 and the only position remains the original FirstLong (`tests/tb_12_node_exposure_index.rs:269`, `tests/tb_12_node_exposure_index.rs:301`).

## Q4 NodePosition avoid total supply counting?
CHALLENGE.

Evidence for the narrow NodePosition question is good: `NodePosition.amount` is explicitly documented as not a Coin holding and not in `total_supply_micro` (`src/state/typed_tx.rs:658`, `src/state/typed_tx.rs:660`), and `total_supply_micro` has no `node_positions_t` term (`src/economy/monetary_invariant.rs:152`, `src/economy/monetary_invariant.rs:172`). SG-12.4 exercises WorkTx plus ChallengeTx, then asserts `assert_total_ctf_conserved` and confirms two NodePositions exist (`tests/tb_12_node_exposure_index.rs:306`, `tests/tb_12_node_exposure_index.rs:334`).

Challenge: the audit prompt required confirming exactly 5 holdings: balances, escrows, stakes, claims-active, and challenge cases. The implementation currently enumerates 4 holdings: balances, escrows, stakes, and challenge cases (`src/economy/monetary_invariant.rs:154`, `src/economy/monetary_invariant.rs:157`, `src/economy/monetary_invariant.rs:160`, `src/economy/monetary_invariant.rs:169`). It intentionally omits `claims_t` as intent metadata (`src/economy/monetary_invariant.rs:163`, `src/economy/monetary_invariant.rs:165`), and its unit test is named `ctf_counts_all_four_holding_subindexes` with expected sum excluding claims (`src/economy/monetary_invariant.rs:582`, `src/economy/monetary_invariant.rs:629`). SG-12.4 does not create an active claim, so it does not cover the requested "claims-active" term (`tests/tb_12_node_exposure_index.rs:306`, `tests/tb_12_node_exposure_index.rs:317`).

Observed inference: NodePosition is not counted, so the critical "NodePosition counted as Coin" halting trigger is not triggered. The holding-count acceptance criterion is still unresolved because current code and tests prove 4 holdings, while the prompt asks for 5.

## Q5 TB-12 accidentally implement trading?
CHALLENGE.

Evidence against TB-12 adding trading variants: `PositionKind` has only `FirstLong` and `ChallengeShort` (`src/state/typed_tx.rs:624`, `src/state/typed_tx.rs:640`), and `TypedTx` has no MarketBuy, MarketSell, MarketOrder, MarketTrade, CompleteSet, or MarketSeed variant (`src/state/typed_tx.rs:1219`, `src/state/typed_tx.rs:1230`). The TB-12 schema comments explicitly forbid MarketBuy/MarketSell, price_yes/price_no, CompleteSet, MarketSeedTx, AMM, and CPMM in this layer (`src/state/typed_tx.rs:599`, `src/state/typed_tx.rs:603`). SG-12.7 checks that observed production positions use only FirstLong/Long and ChallengeShort/Short (`tests/tb_12_node_exposure_index.rs:372`, `tests/tb_12_node_exposure_index.rs:408`).

Challenge: the required forbidden-token grep over current `src/` is not clean. `src/lib.rs` exports `prediction_market` (`src/lib.rs:3`). That module is an actual CPMM trading implementation: it declares a CPMM binary market (`src/prediction_market.rs:1`, `src/prediction_market.rs:10`), implements `buy_yes` and `buy_no` (`src/prediction_market.rs:81`, `src/prediction_market.rs:113`), and implements redemption (`src/prediction_market.rs:145`, `src/prediction_market.rs:148`). `Kernel` still stores `BinaryMarket` maps and a bounty market (`src/kernel.rs:21`, `src/kernel.rs:27`), with a comment referencing a CPMM book (`src/kernel.rs:29`). Inference: this appears to be legacy non-TB-12 code rather than a NodePosition or EconomicState path, but the strict `src/` forbidden-token gate cannot be reported as PASS without an explicit allowlist, quarantine, or removal.

## Q6 Forged or replayed Work/Challenge accept bypass stake gate?
PASS.

Evidence: WorkTx rejects `stake.micro_units() <= 0` before any money mutation or NodePosition derivation (`src/state/sequencer.rs:495`, `src/state/sequencer.rs:498`). The WorkTx NodePosition write also has a defense-in-depth `if work.stake.micro_units() > 0` gate (`src/state/sequencer.rs:572`). ChallengeTx rejects zero stake before target liveness, balance checks, `challenge_cases_t`, or NodePosition mutation (`src/state/sequencer.rs:771`, `src/state/sequencer.rs:773`), and its NodePosition write is also gated by `if challenge.stake.micro_units() > 0` (`src/state/sequencer.rs:825`).

Upstream rejection is tested for ChallengeTx zero stake (`src/state/sequencer.rs:3571`, `src/state/sequencer.rs:3583`). The TB-12 integration test file documents the WorkTx zero-stake negative case as unreachable because the TB-3 WorkTx accept arm rejects before NodePosition derivation; the NodePosition gate is defense-in-depth (`tests/tb_12_node_exposure_index.rs:411`, `tests/tb_12_node_exposure_index.rs:418`). Inference: replay cannot bypass this because ChainTape replay re-runs the same `dispatch_transition` path (`src/bottom_white/ledger/transition_ledger.rs:487`, `src/bottom_white/ledger/transition_ledger.rs:490`).

## Q7 No node_market_t canonical EconomicState field?
PASS.

Evidence: `EconomicState` lists 11 fields ending with serde-defaulted `runs_t` and `node_positions_t`, with no `node_market_t` field (`src/state/q_state.rs:158`, `src/state/q_state.rs:192`). `NodePositionsIndex` is a flat `BTreeMap<TxId, NodePosition>` (`src/state/q_state.rs:485`, `src/state/q_state.rs:492`). The local grep for `node_market_t` in `src/` found only comments and the guard test, not a struct field; the runtime test asserts the serialized EconomicState object does not contain `node_market_t` (`src/state/q_state.rs:726`, `src/state/q_state.rs:739`).

## Q8 Halting triggers none triggered?
PASS.

Evidence: CTF conservation for NodePosition derivation is exercised by SG-12.4 and a second invariant test (`tests/tb_12_node_exposure_index.rs:321`, `tests/tb_12_node_exposure_index.rs:331`, `tests/tb_12_node_exposure_index.rs:485`, `tests/tb_12_node_exposure_index.rs:488`). WorkTx/ChallengeTx position mismatch is guarded by `position_fields_derived_from_source_tx_exactly` (`tests/tb_12_node_exposure_index.rs:422`, `tests/tb_12_node_exposure_index.rs:465`). NodePosition counted-as-Coin is not observed because `total_supply_micro` omits `node_positions_t` (`src/economy/monetary_invariant.rs:152`, `src/economy/monetary_invariant.rs:172`). Replay divergence is guarded by SG-12.5 and by the production replay path re-running `dispatch_transition` (`tests/tb_12_node_exposure_index.rs:340`, `tests/tb_12_node_exposure_index.rs:369`, `src/bottom_white/ledger/transition_ledger.rs:487`, `src/bottom_white/ledger/transition_ledger.rs:490`).

Inference: the listed halting triggers are not triggered by the NodePosition implementation. The final audit verdict remains CHALLENGE because Q4 and Q5 have ship-gate acceptance mismatches.

## Net findings
- Bug class: Audit/spec mismatch. Q4 expects a 5-holding total-supply enumeration including claims-active, but code and tests enforce a 4-holding TB-8 model excluding claims (`src/economy/monetary_invariant.rs:163`, `src/economy/monetary_invariant.rs:165`, `src/economy/monetary_invariant.rs:582`, `src/economy/monetary_invariant.rs:629`). Q5 strict forbidden-token grep over `src/` hits a legacy exported CPMM trading module (`src/lib.rs:3`, `src/prediction_market.rs:1`, `src/prediction_market.rs:113`).
- Severity: blocker for a clean TB-12 ship-gate PASS; no direct NodePosition money-ledger, replay, or stake-bypass exploit was found.
- Remediation: Reconcile the total-supply contract before ship: either ratify/update TB-12 audit docs to the TB-8 4-holding model, or add the requested active-claims term and tests. Also quarantine, feature-gate, remove, or explicitly allowlist legacy `prediction_market`/`Kernel` CPMM code before claiming the forbidden-token gate PASS.

## Final verdict
CHALLENGE
