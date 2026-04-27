# Codex Spec Freeze Audit (CO1.SPEC.0.5)

## Scope and artifact SHA

Spec under audit: `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md`.

SHA256 verified locally: `f7a45de3dc922661940a1f7743119e787ad8e35696e93e444eb68d04f85a3771`.

Cross-references read: `constitution.md`, `src/bus.rs`, `src/kernel.rs`, `src/prediction_market.rs`, `handover/whitepapers/*`, `handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md`, `handover/audits/CODEX_T_S_REVIEW_2026-04-27.md`, `handover/audits/GEMINI_V32_REVIEW_2026-04-27.md`, and CO P0/plan artifacts. The prompt path `handover/whitepaper/` is stale; the repository path is `handover/whitepapers/`.

No `cargo build` or `cargo test` was run. No source file was modified.

## Q1 Pseudocode completeness — [CHALLENGE]

Verdict: [CHALLENGE]

Evidence:
- v1.1 adds binding schemas and transition pseudocode beyond the original WorkTx-only core: scope lists `QState`, `WorkTx`, `VerifyTx`, `ChallengeTx`, `RejectedAttemptSummary`, and `TerminalSummaryTx` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:24-29`; `verify_transition` is present at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:352-405`, `challenge_transition` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:407-501`, `reuse_transition` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:503-557`, `finalize_reward_transition` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:559-614`, and terminal summary at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:616-649`.
- The current bus transition surface is larger than those functions. `append_internal` includes forbidden-pattern/payload gates, tool hooks, invest-only routing, node construction, kernel append, WAL write, market creation, founder grants, post-hooks, ledger append, and counters at `src/bus.rs:179-333`. Invest-only routing mutates wallet/market state at `src/bus.rs:229-252`. Market creation happens immediately after append at `src/bus.rs:285-290`. Founder grants are env-gated and wallet-mutating at `src/bus.rs:292-312`. Halt settlement mutates markets/wallets at `src/bus.rs:335-412`.
- `Kernel` also has stateful economic behavior not mapped to a transition variant: markets and bounty state are fields at `src/kernel.rs:19-32`, bounty open/resolve is at `src/kernel.rs:63-103`, and market create/buy/resolve/ticker APIs are at `src/kernel.rs:114-206`.
- The economic chapter says RSP-1 includes `TaskMarket`, `EscrowVault`, `ContributionLedger`, `PredicateRunner`, `AttributionEngine`, `ChallengeCourt`, `SettlementEngine`, `ReputationIndex`, and `PriceIndex` at `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:79-93`. v1.1 has no `TaskMarketPublishTx`, `InvestTx`, `MarketCreateTx`, `MarketResolveTx`, `TaskExpireTx`, or bounty-refund transition.

Failure scenario:
- Branch A preserves `InvestOnly` as a direct side effect through `debit_wallet`/`buy_yes`/`buy_no`; Branch B converts it to an L4 transition or removes it until P2. Both can pass the five core transition tests while producing different snapshots, wallet state, and ledger roots for the same current bus workload.

Required fixes:
- Add a transition coverage matrix for every current bus/kernel state mutation: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TerminalSummaryTx`, plus explicit disposition for `Invest`, `TaskMarketPublish/MarketCreate`, `MarketResolve/RunEnd`, bounty settlement/refund, snapshot/read view, WAL append, and tool hook effects.
- If some legacy behavior is intentionally retired before CO1.1.4, mark it as retired with a conformance test that fails if it remains a hidden side effect.
- Blocks CO P1? Yes for CO1.1.4/CO1.1.5 spec freeze. It does not block a read-only gix spike, but it blocks launching the bus/kernel split as STEP_B against this spec.

## Q2 Invariant exhaustiveness — [CHALLENGE]

Verdict: [CHALLENGE]

Evidence:
- The spec declares 22 transition invariants at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:654-681`. The economic chapter declares 12 economic invariants at `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:56-77`. Constitution Art. 0.2 requires all cost/time/provenance/market/wallet/rejection/search/routing/tick signals to be reconstructible from tape at `constitution.md:54-65`.
- Missing invariant: stake return on successful finalization. Spec §3 locks solver stake and debits balance at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:313-317`; §3.4 credits reward, finalizes claim, debits escrow, and pays royalties at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:593-605`, but never unlocks or returns the solver stake. WP economic Inv 3-5 require escrow-only rewards, no post-init mint, and event-bound YES/NO rights at `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:63-65`. Concrete buggy implementation that slips through: implement `finalize_reward_transition` exactly as written; all 22 listed tests can pass while every successful solver permanently loses locked stake.
- Missing invariant: task/bounty expiry refund. The spec has no task-expiry transition or refund invariant. Current `Kernel::resolve_bounty` returns an empty payout when no golden path exists at `src/kernel.rs:92-96`; `TuringBus::halt_and_settle` only credits agents for returned payout rows at `src/bus.rs:357-360`, so there is no explicit refund ledger event or escrow return. This conflicts with economic Inv 3 at `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:63` and Law 2 at `constitution.md:159-160`. Concrete buggy implementation that slips through: a task expires with locked bounty and no accepted claim; no listed invariant detects the stuck or vanished escrow.
- Missing invariant: deterministic finalize batch order. The spec says finalization is tick-triggered for any expired provisional claim at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:559-561`, and `I-FINALIZE-EXCLUSIVE` says runtime serializes at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:677`, but no ordering key is defined for multiple claims expiring at the same logical tick. Current kernel market resolution iterates a `HashMap` at `src/kernel.rs:165-171`, and tickers iterate `HashMap` then sort only by float price at `src/kernel.rs:187-204`. Constitution Art. 0.4 requires `Q_t` to be a version-controlled state tuple at `constitution.md:114-123`. Concrete buggy implementation that slips through: finalize claims by `HashMap` iteration; branch A appends `Finalize(C1), Finalize(C2)`, branch B appends `Finalize(C2), Finalize(C1)`, balances match but ledger/state roots diverge.
- Challenge-window edge semantics are under-specified. `ChallengeTx` rejects when `tx.timestamp_logical >= window.opens_at + window.duration_ticks` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:419-423`, while finalization checks only `w.is_open()` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:573-581`. No invariant defines whether the close tick is inclusive or exclusive across both challenge and finalize paths.

Required fixes:
- Add `I-STAKE-RETURN`: successful unchallenged finalization returns/unlocks the solver stake exactly once.
- Add `I-TASK-EXPIRY-REFUND` or `I-BOUNTY-REFUND`: expired tasks without accepted/finalized work return escrow by a deterministic transition.
- Add `I-FINALIZE-BATCH-ORDER`: when multiple claims become finalizable at the same logical tick, emit finalize tx in a total order such as `(expires_at, claim_id)`.
- Add `I-CHALLENGE-WINDOW-EDGE`: define the exact inclusive/exclusive window rule and reuse it in both challenge and finalize.
- Blocks CO P1? Yes for spec freeze. These are not implementation polish; they are invariant holes that two STEP_B branches can encode differently.

## Q3 Hidden input classification — [CHALLENGE]

Verdict: [CHALLENGE]

Evidence:
- The table covers major known issues: `SystemTime::now`, `TAPE_ECONOMY_V2`, `FOUNDER_GRANT_GAMMA`, `system_lp_amount`, counters, graveyard, tool iteration, and wallet string lookup at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:233-247`.
- It misses `HAYEK_BOUNTY` and `BOUNTY_LP`. Current `init` opens the bounty market from env at `src/bus.rs:141-150`, and `halt_and_settle` resolves bounty payouts from env at `src/bus.rs:349-360`.
- It misses proposal-generation randomness and its config surface. `BoltzmannParams::from_env` reads `BOLTZMANN_TEMP`, `FRONTIER_CAP`, `DEPTH_WEIGHT`, and `PRICE_GATE_ALPHA` at `src/sdk/actor.rs:22-39`; selection samples `rng.gen::<f64>()` at `src/sdk/actor.rs:137-143`; evaluator seeds `StdRng` from `BOLTZMANN_SEED` at `experiments/minif2f_v4/src/bin/evaluator.rs:693-697`. Spec `I-NORANDOM` covers transition internals at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:674`, but the table does not say whether routing seed/config are `Q_t`, tx input, or off-tape proposal-only data.
- It under-scopes `HashMap` risk. The spec says "BTreeMap, not HashMap, everywhere" at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:98`, but the §2 test only greps modules containing `q_state` or `transition` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:249-253`. Current `Kernel` stores markets in a `HashMap` at `src/kernel.rs:19-21`, resolves by `HashMap` iteration at `src/kernel.rs:165-171`, and builds tickers from `HashMap` iteration at `src/kernel.rs:187-204`.
- It under-specifies floating arithmetic. Current money/market code uses `f64` reserves and LP totals at `src/prediction_market.rs:21-27`, accepts `lp_coins: f64` at `src/prediction_market.rs:54-67`, and trades with `coins_in: f64` at `src/prediction_market.rs:87-133`. The spec forbids f64 money at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:253` and `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:667`, but royalty math still says `let royalty = reward * edge.weight` without a rounding rule at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:600-604`.
- It does not classify filesystem effects. Current append writes WAL nodes/events at `src/bus.rs:279-282` and `src/bus.rs:319-327`; the spec says "No I/O" at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:260-261`, but it needs an explicit boundary between pure transition output and the runtime committing that output to WAL/git.
- HYPOTHESIS: future tokio task ordering can become a hidden input if multiple submitted tx are accepted by whichever task completes first. Current evaluator is async via `#[tokio::main]` at `experiments/minif2f_v4/src/bin/evaluator.rs:192-193`, and Phase C explicitly exercises 5 modes x 10 problems x 2 seeds at `experiments/minif2f_v4/src/bin/evaluator.rs:2095-2108`; the spec has no `tokio::spawn` claim to audit in `src/bus.rs`/`src/kernel.rs`, but it must still ban async completion order as an L4 ordering source.

Failure scenario:
- A branch keeps `HAYEK_BOUNTY=1` as an env-gated runtime branch and another promotes it to `TaskMarket.config`; both can pass `no_hidden_inputs.rs` as currently described because that table does not mention the variable.

Required fixes:
- Extend §2 with `HAYEK_BOUNTY`, `BOUNTY_LP`, Boltzmann params, `BOLTZMANN_SEED`, async completion ordering, WAL/git commit boundary, all `HashMap` in state-derived paths, and all `f64` money/weight arithmetic.
- Add deterministic rounding: e.g. `MicroCoin * RoyaltyRatePpm -> MicroCoin` using integer floor or round-half-up, with one named rule.
- Blocks CO P1? Yes for CO1.1.4/CO1.1.5 and CO1.7; otherwise hidden-input rot can be moved into new modules under a passing test name.

## Q4 Type system feasibility — [PASS]

Verdict: [PASS]

Evidence:
- There is no Rust type-system blocker for the requested shapes. `QState` and nested state maps use `BTreeMap` in the spec at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:42-96`; `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `RejectedAttemptSummary`, `TerminalSummaryTx`, and `MetaTx` are ordinary Rust structs/enums at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:100-223`.
- Existing code already uses ordinary Rust structs/enums for comparable state: `TuringBus` at `src/bus.rs:40-51`, `BusResult` at `src/bus.rs:65-71`, `Kernel` at `src/kernel.rs:19-32`, and `KernelError` at `src/kernel.rs:217-238`.
- `BTreeMap`/newtype-based Rust implementation is straightforward. The hard parts are not Rust expressibility; they are serialization, money type ordering, and transition coverage.

Required fixes:
- None for type-system feasibility itself.
- Prerequisite tracked in Q7: define and land `MicroCoin`, `StakeMicroCoin`, and `RoyaltyRate` before any P1 code that compiles these schemas.
- Blocks CO P1? No on type feasibility alone.

## Q5 STEP_B comparison metric — [CHALLENGE]

Verdict: [CHALLENGE]

Evidence:
- The spec says STEP_B comparison is "branch X conforms to spec" rather than code similarity at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:681` and `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:748-755`.
- That is a correct metric statement but not mechanically checkable yet. The spec invokes `tx.canonical_digest()` for signatures at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:279-281`, `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:372-374`, and `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:425-428`, but it never defines canonical serialization bytes for any tx or `QState`.
- Gemini's prior cross-review named differential fuzzing as the strongest STEP_B test at `handover/audits/GEMINI_V32_REVIEW_2026-04-27.md:51-58`. v1.1 still lists test names, not a runner ABI or fixture format, at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:654-681`.

Failure scenario:
- Branch A signs/verifies JSON with sorted keys; Branch B signs/verifies bincode or Rust derive order. Each branch locally passes `I-SIG`, but cross-branch replay rejects signatures or produces different state roots. The spec has no canonical byte fixture to decide which branch is wrong.

Required fixes:
- Add a STEP_B conformance section with:
  - canonical serialization format for every tx, `QState`, `SignalBundle`, and `TransitionError`;
  - a CLI/library ABI: input bytes = `(genesis_q, tx_seq)`, output bytes = `(q_final, signals_seq, errors_seq, ledger_root, state_root)`;
  - golden fixtures for all 22 invariants plus adversarial edge cases;
  - differential fuzzing using a deterministic seed recorded in the fixture;
  - byte-for-byte compare of both success and rejection outputs.
- Blocks CO P1? Yes for STEP_B atoms CO1.1.4/CO1.1.5. Without this, "spec conformance" is a human review label, not a gate.

## Q6 Determinism under concurrency — [CHALLENGE]

Verdict: [CHALLENGE]

Evidence:
- Current `TuringBus` is explicitly a serial reactor: `src/bus.rs:37-39`. It serially appends a ledger event and then increments `tx_count` and `clock` at `src/bus.rs:319-330`.
- The spec is single-transition pseudocode and only says the runtime serializes finalize/slash exclusivity at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:677`; it does not define the serialization point, queue, or ordering key for concurrent tx arrivals.
- CO P1 preflight acknowledges concurrent per-cell repo initialization: Phase C runs "5 modes x 10 problems x N seeds in parallel" at `handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md:18-23`, and a synthetic 5-mode concurrent-init test is planned at `handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md:154-158`.
- The evaluator also has an async runtime at `experiments/minif2f_v4/src/bin/evaluator.rs:192-193` and a 5-mode integration companion at `experiments/minif2f_v4/src/bin/evaluator.rs:2095-2108`.

Failure scenario:
- Two agent tasks finish in the same tick against the same parent root. Branch A orders by async completion time; Branch B orders by agent id. Both can satisfy `I-PARENT` for the first tx and reject the second, but they may choose different winners and therefore different state roots.

Required fixes:
- Define a single L4 append sequencer per `(runtime_repo, run_id)` before transition execution. Ordering key must be deterministic, for example `(logical_t, tx_id)` assigned by the sequencer, not by async completion.
- Define cross-cell isolation: different mode/problem/seed cells must use disjoint `runtime_repo` and disjoint `QState`. Shared repositories require ref locks plus deterministic retry semantics.
- Define finalize batch order for multiple claims expiring at the same tick: e.g. `(expires_at, claim_id)`.
- Blocks CO P1? Yes for CO1.1.4/CO1.1.5 and CO1.7. The gix concurrency spike can proceed only if treated as substrate preflight, not as proof of transition determinism.

## Q7 Money type prerequisite — [CHALLENGE]

Verdict: [CHALLENGE]

Evidence:
- The spec uses `StakeMicroCoin` and `MicroCoin` in P1-facing schemas: `WorkTx.stake` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:103-115`, `TxStatus::FinalizedReward(MicroCoin)` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:118-124`, and `VerifyTx`/`ChallengeTx` bond/stake at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:136-154`.
- CO1.7 is in Phase 1 and includes transition ledger/retry metadata at `handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md:191-204`; CO P2.0a `i64 micro-coin` is scheduled only after P1 at `handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md:208-210`.
- Current production economy uses `f64`: `BusConfig.system_lp_amount` at `src/bus.rs:16-21`, `Kernel.bounty_lp_seed` at `src/kernel.rs:19-32`, and `BinaryMarket` reserves/trades at `src/prediction_market.rs:21-27` and `src/prediction_market.rs:87-133`.
- Silent rounding remains undefined. Royalty payout is `reward * edge.weight` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:600-604`, while the cap comment references a fractional representation at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:530-533` without defining storage or rounding.

Failure scenario:
- CO1.7 implements `TransitionTx` with `i64`, while CO1.1.4 keeps bus/kernel APIs in `f64`. A later P2.0a migration changes serialization and hashes for already-written transition records, breaking replay and STEP_B comparison.

Required fixes:
- Move `MicroCoin(i64)`, `StakeMicroCoin`, and `RoyaltyRatePpm` to a P1 prerequisite atom before CO1.7 and before any STEP_B branch serializes tx records.
- Define overflow behavior and rounding once: checked arithmetic for balances/stakes, deterministic integer division for royalties, and explicit error on overflow.
- Blocks CO P1? Blocks spec freeze and CO1.7. Early non-money scaffolding can proceed only if the plan is amended before launch to move the type prerequisite into P1.

## Q8 Holistic verdict

Verdict: [CHALLENGE]

STEP_B-ready: NO.

CO-P1-launch-ready: NO.

Evidence:
- The spec is materially improved over the WorkTx-only form, but the current bus/kernel split target has more transition surface than the five core functions cover; see `src/bus.rs:179-333` and `src/kernel.rs:63-206`.
- The spec itself says every transition test must pass before CO1.1.4 starts at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:681`, but the comparison harness and canonical serialization are not specified.
- The dependency graph says CO1.1.4 is blocked by spec freeze at `handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:94-96`, CO1.1.5 is blocked by CO1.1.4 at `handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:97-99`, and CO1.7.5 depends on the transition schema/QState stack at `handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:103-108`.

Failure scenario:
- CO1.1.4 starts after this freeze; one branch preserves legacy env/f64/invest behavior as compatibility code, another removes it as out-of-scope. Both can claim compliance because the spec lacks transition coverage and STEP_B byte fixtures.

Required pre-CO-P1 amendments:
- Patch `STATE_TRANSITION_SPEC` to v1.2 or v1.1a with the transition coverage matrix, hidden-input table expansion, STEP_B runner/serialization rules, concurrency sequencer, MicroCoin prerequisite, rounding rule, stake return, task-expiry refund, finalize batch order, and challenge-window edge rule.
- Amend the sprint dependency graph so `MicroCoin` lands before CO1.7 and before any serialized tx fixture.
- Add a conformance fixture directory or spec appendix before CO1.1.4/CO1.1.5 starts.

Blocks CO P1? Yes. Recommendation is NO-GO until the amendments above are made and re-audited.

## Q9 Top-3 watchpoints for STEP_B

Verdict: [CHALLENGE]

1. Serialization and digest boundary.
   - Watchpoint: `tx.canonical_digest()` appears in pseudocode but no canonical bytes are defined at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:279-281`, `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:372-374`, and `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:425-428`.
   - Failure scenario: branches self-verify incompatible tx bytes.
   - Minimal fix: one canonical serialization format plus golden tx digest vectors.
   - Blocks CO P1? Yes for STEP_B.

2. Ordering under parallel execution and finalize batches.
   - Watchpoint: current bus relies on a serial reactor at `src/bus.rs:37-39`; spec only says runtime serializes finalization/slash at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:677`; Phase C/P1 preflight explicitly has parallel cells at `handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md:18-23`.
   - Failure scenario: two tx or two expired claims are ordered by async completion or `HashMap` iteration.
   - Minimal fix: L4 sequencer and deterministic finalize order.
   - Blocks CO P1? Yes for transition implementation.

3. Money and hidden-input migration.
   - Watchpoint: spec requires MicroCoin at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:667`, but current money code is `f64` at `src/prediction_market.rs:21-27`; env-controlled economy branches remain in `src/bus.rs:141-150`, `src/bus.rs:292-312`, and `src/bus.rs:349-360`.
   - Failure scenario: one branch wraps legacy floats and env flags; another moves them into `QState`; both pass shallow WorkTx tests but diverge on replay.
   - Minimal fix: P1 MicroCoin atom, expanded hidden-input table, integer royalty rounding rule, and tests that fail on `std::env::var` in transition dependency paths.
   - Blocks CO P1? Yes for CO1.7 and STEP_B.

## Q10 Prior-round closure check

Verdict: [CHALLENGE]

Evidence:
- T+S CHALLENGE closure: mostly addressed, not fully. The T+S review required typed state/tx schemas, deterministic pseudocode, named invariants, and generated conformance tests at `handover/audits/CODEX_T_S_REVIEW_2026-04-27.md:11-17` and `handover/audits/CODEX_T_S_REVIEW_2026-04-27.md:103-108`. v1.1 supplies that form at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:22-29` and `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:257-681`. The same T+S review also required bounded system-generated retry metadata and terminal summary for no-accept runs at `handover/audits/CODEX_T_S_REVIEW_2026-04-27.md:53-59` and `handover/audits/CODEX_T_S_REVIEW_2026-04-27.md:103-108`; v1.1 addresses that at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:165-204` and `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:616-649`.
- Gemini v3.2 CHALLENGE closure: incomplete-pseudocode issue is substantially addressed. Gemini flagged WorkTx-only pseudocode at `handover/audits/GEMINI_V32_REVIEW_2026-04-27.md:12-20` and again at `handover/audits/GEMINI_V32_REVIEW_2026-04-27.md:93-101`; v1.1 now includes Verify/Challenge/Reuse/Finalize/Terminal functions at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:352-649` and adds randomness/liveness/window/finalize invariants at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:674-679`.
- Walk-through gap closure: three of four are materially closed. Verifier bond release was requested at `handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:452-463` and is implemented at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:461-480`. Royalty cap was requested at `handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:464-468` and is implemented at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:525-548`. Multi-verifier quorum was requested at `handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:470-474` and is at least explicitly defaulted/deferred at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:356`.
- Prose-vs-pseudocode contradiction: false-challenge penalty is not actually implemented. The patch log claims §3.2 stage 4d was amended for false-challenge reputation penalty at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:5-8`, and §5.1 says the default is configurable at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:721-728`. But `challenge_transition` returns `CounterexampleInsufficient` before any state transition at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:434-439`, and stage 4d only adjusts solver/challenger reputation after a successful challenge at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:457-459`. A nonzero `false_challenge_reputation_penalty` config has no executable path.
- Prose-vs-prose contradiction: §4 says total 22 invariants at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:681`, but §8 still describes the spec as "a list of 16 named invariants" at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:759-765`.
- Re-scan items: silent rounding is missing in royalty math at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:600-604`; challenge-window edge semantics are ambiguous between `ChallengeTx` and finalize at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:419-423` and `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:573-581`; multiple finalizations same tick lack an ordering rule under `I-FINALIZE-EXCLUSIVE` at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:677`.

Failure scenario:
- A branch implements nonzero false-challenge penalty because prose says configurable; another ignores it because pseudocode returns before mutation. Both can claim spec support.

Required fixes:
- Either implement failed-challenge penalty as a separate state-changing rejection/penalty transition or declare `false_challenge_reputation_penalty` fixed to zero for v4 and remove "configurable" prose.
- Update §8 invariant count from 16 to 22.
- Add rounding, challenge-window edge, and finalize-batch rules before freezing.
- Blocks CO P1? Yes for spec freeze; these are direct contradictions in the audited artifact.

## Final verdict: CHALLENGE

Worst per-question verdict is CHALLENGE. No VETO is issued because the defects are concrete spec amendments, not a rejection of the architecture. The artifact is not freeze-ready.

## Recommendation on CO P1 launch: NO-GO

NO-GO for CO P1 launch as a spec-freeze gate. Limited read-only or spike work may continue, but CO1.1.4, CO1.1.5, and CO1.7 must not start against this spec until the required amendments are made and re-audited.
