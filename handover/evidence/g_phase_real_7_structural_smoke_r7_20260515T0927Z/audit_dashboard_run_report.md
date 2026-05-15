=================================================================
 TB-8 Audit Dashboard — run_id=g_phase_real_7_structural_smoke_r7_20260515T0927Z_5239f4b_t000 epoch=3
=================================================================

§1 Run metadata
---------------
  head_commit_oid: ad146b0b7699d3bbd6dddc9fdd88962ef5837077
  l4e_last_hash: 6ef8e67b40b3f2f3910a532d4154828ba62eb58abfa2cc2ab6639b13a0a7cb3c
  final_state_root: 70ef4022f6477e81a9a9a19a4f007f778cc8500015312d0aa144b57154fa54b6
  final_ledger_root: b0c3802069c8d10c211c39f6fd9c7869ce3eb2a923dcd8e0786a41b658989419
  initial_q_state_loaded_from_disk: true

§2 Chain stats + 7 indicators
------------------------------
  L4 entries:  36
  L4.E entries: 21
  ledger_root_verified              : ✓
  system_signatures_verified        : ✓
  state_reconstructed               : ✓
  economic_state_reconstructed      : ✓
  cas_payloads_retrievable          : ✓
  agent_signatures_verified [Gate 4]: ✓
  proposal_telemetry_cas_retrievable [Gate 5]: ✓
  ALL 7 PASS                        : GREEN

§3 ChainDerivedRunFacts (§4.4 bit-exact set)
---------------------------------------------
  solved                  : false
  verified                : false
  tx_count                : 57
  proposal_count          : 21
  golden_path_token_count : 0
  gp_payload (CID hex)    : e017f523a718e7d7cbd7f37eae1f524869a81c5973028c2e8ef663bde3963d7e
  gp_path                 : tb16-arena-boltzmann-seed
  tactic_diversity        : 1
  failed_branch_count     : 21
  chain_oracle_verified   : false (no oracle-verified WorkTx)
  chain_economic_finalized: false (always false in TB-7; settlement = TB-9 territory)
  tool_dist:
    tb16-arena-boltzmann-seed: 3

§4 Per-agent activity
---------------------
  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗
  ------------------+--------+-------+-------+---------+--------
  Agent_0           | ✓      | 3     | 3     | 0       | 0
  Agent_1           | ✓      | 0     | 3     | 0       | 0
  Agent_2           | ✓      | 0     | 3     | 3       | 0
  Agent_3           | ✓      | 0     | 3     | 0       | 0
  Agent_4           | ✓      | 0     | 3     | 0       | 0
  Agent_5           | ✓      | 0     | 0     | 0       | 0
  Agent_6           | ✓      | 0     | 0     | 0       | 0
  Agent_7           | ✓      | 0     | 0     | 0       | 0
  Agent_8           | ✓      | 0     | 0     | 0       | 0
  Agent_9           | ✓      | 0     | 0     | 0       | 0
  Agent_solver_0    | ✓      | 0     | 0     | 0       | 0
  Agent_user_0      | ✓      | 0     | 0     | 0       | 0
  MarketMakerBudget | ✓      | 0     | 0     | 0       | 0
  tb6-smoke-agent   | ✗      | 0     | 3     | 0       | 0
  tb6-smoke-sponsor | ✗      | 0     | 0     | 0       | 0

§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_0    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_4    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_0    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_4    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_0    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | PredicateFailed
  L4.E  |   0 | Work            | Agent_4    | -          | -          | -      | PredicateFailed
  L4    |   1 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
  L4    |   2 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
  L4    |   3 | MarketSeed      | -          | -          | -          | -      | -
  L4    |   4 | CpmmPool        | -          | -          | -          | -      | -
  L4    |   5 | BuyWithCoinRouter | Agent_1    | -          | -          | -      | -
  L4    |   6 | BuyWithCoinRouter | Agent_2    | -          | -          | -      | -
  L4    |   7 | Work            | Agent_0    | tb16-arena-boltzmann-seed | Agent_0.b5 | -      | -
        payload: tb16-x-2-4-boltzmann-seed-iter-0
  L4    |   8 | Verify          | Agent_2    | -          | -          | -      | -
  L4    |   9 | Challenge       | Agent_3    | -          | -          | -      | -
  L4    |  10 | FinalizeReward  | system (solver=Agent_0) | -          | -          | -      | -
  L4    |  11 | EventResolve    | -          | -          | -          | -      | -
  L4    |  12 | TerminalSummary | -          | -          | -          | -      | -
  L4    |  13 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
  L4    |  14 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
  L4    |  15 | MarketSeed      | -          | -          | -          | -      | -
  L4    |  16 | CpmmPool        | -          | -          | -          | -      | -
  L4    |  17 | BuyWithCoinRouter | Agent_1    | -          | -          | -      | -
  L4    |  18 | BuyWithCoinRouter | Agent_2    | -          | -          | -      | -
  L4    |  19 | Work            | Agent_0    | tb16-arena-boltzmann-seed | Agent_0.b5 | -      | -
        payload: tb16-x-2-4-boltzmann-seed-iter-0
  L4    |  20 | Verify          | Agent_2    | -          | -          | -      | -
  L4    |  21 | Challenge       | Agent_3    | -          | -          | -      | -
  L4    |  22 | FinalizeReward  | system (solver=Agent_0) | -          | -          | -      | -
  L4    |  23 | EventResolve    | -          | -          | -          | -      | -
  L4    |  24 | TerminalSummary | -          | -          | -          | -      | -
  L4    |  25 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
  L4    |  26 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
  L4    |  27 | MarketSeed      | -          | -          | -          | -      | -
  L4    |  28 | CpmmPool        | -          | -          | -          | -      | -
  L4    |  29 | BuyWithCoinRouter | Agent_1    | -          | -          | -      | -
  L4    |  30 | BuyWithCoinRouter | Agent_2    | -          | -          | -      | -
  L4    |  31 | Work            | Agent_0    | tb16-arena-boltzmann-seed | Agent_0.b5 | -      | -
        payload: tb16-x-2-4-boltzmann-seed-iter-0
  L4    |  32 | Verify          | Agent_2    | -          | -          | -      | -
  L4    |  33 | Challenge       | Agent_3    | -          | -          | -      | -
  L4    |  34 | FinalizeReward  | system (solver=Agent_0) | -          | -          | -      | -
  L4    |  35 | EventResolve    | -          | -          | -          | -      | -
  L4    |  36 | TerminalSummary | -          | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: MultiAttemptDagValid ✓ (≥1 multi-attempt branch with all parent_tx edges present)
  edges:
    [Agent_0.b5] worktx-task-n5_numbertheory_2pownm1prime_nprime_1778837658702-tb16-arena-boltzmann-seed-iter-0 → worktx-task-n5_aime_1983_p1_1778837677909-tb16-arena-boltzmann-seed-iter-0
    [Agent_0.b5] worktx-task-n5_aime_1983_p1_1778837677909-tb16-arena-boltzmann-seed-iter-0 → worktx-task-n5_imo_1964_p2_1778837711592-tb16-arena-boltzmann-seed-iter-0

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)

§8 Cross-checks
---------------
  audit_trail_rows         : 6
  chain_proposal_count     : 21
  audit_rows == proposal_count: ✗ (gap)
  audit_trail_chain_valid     : ✓
  (Note: pre-TB-7.6 the agent_audit_trail.jsonl is populated only
   by the synthetic-seed hook; full per-LLM-proposal audit-trail
   wiring is part of TB-7.6 carry-forward action #4 / #5.)

§9 TB-8 Claims (claim_status + payout_amount)
----------------------------------------------
  claim_id                          | task_id        | solver        | status     | payout_micro | created@t | finalized@t
  ----------------------------------+----------------+---------------+------------+--------------+-----------+------------
  claim-verifytx-Agent_2-real7-scr… | task-n5_numbe… | Agent_0       | Finalized  |       100000 |         8 | 10
  claim-verifytx-Agent_2-real7-scr… | task-n5_aime_… | Agent_0       | Finalized  |       100000 |        20 | 22
  claim-verifytx-Agent_2-real7-scr… | task-n5_imo_1… | Agent_0       | Finalized  |       100000 |        32 | 34

  Aggregate: 3 claims observed | 0 Open | 3 Finalized | total_payout = 300000 micro

§10 TB-9 Durable identity (agent keystore registry)
---------------------------------------------------
  durable_keystore_path: /home/zephryj/.turingos/keystore/agent_keystore.enc
  durable_keystore_present: ✓ (cross-run identity available)
  agents_in_manifest: 13
  agent_id          | pubkey_in_manifest | tape_activity
  ------------------+--------------------+---------------
  Agent_0           | ✓ (durable-backed) | Work✓=3 Work✗=3 Verify✓=0 Verify✗=0
  Agent_1           | ✓ (durable-backed) | Work✓=0 Work✗=3 Verify✓=0 Verify✗=0
  Agent_2           | ✓ (durable-backed) | Work✓=0 Work✗=3 Verify✓=3 Verify✗=0
  Agent_3           | ✓ (durable-backed) | Work✓=0 Work✗=3 Verify✓=0 Verify✗=0
  Agent_4           | ✓ (durable-backed) | Work✓=0 Work✗=3 Verify✓=0 Verify✗=0
  Agent_5           | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0
  Agent_6           | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0
  Agent_7           | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0
  Agent_8           | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0
  Agent_9           | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0
  Agent_solver_0    | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0
  Agent_user_0      | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0
  MarketMakerBudget | ✓ (durable-backed) | Work✓=0 Work✗=0 Verify✓=0 Verify✗=0

  Note: cross-run identity is empirically observable by
  comparing this run's `agent_pubkeys.json` to a sibling run
  that loaded the same TURINGOS_AGENT_KEYSTORE_PATH — equal
  pubkey rows ⇒ TB-9 mandate "agent identity survives run
  restart" satisfied.

§11 TB-10 User Tasks (sponsored by Agent_user_*; lean_market product surface)
------------------------------------------------------------------------------
  (no Agent_user_*-sponsored TaskOpen on chain; lean_market run-task
   not invoked, or evaluator ran in self-funded preseed mode
   [TURINGOS_USER_TASK_MODE unset]; n/a)

§12 TB-11 Epistemic Exhaust + Capital Liberation (architect §6.2; 2026-05-02)
------------------------------------------------------------------------------
  Exhausted runs (RunExhaustedTx ≡ TerminalSummaryTx):
    run_id         | task_id            | outcome         | attempts | evidence_capsule_cid (hex)
    ---------------+--------------------+-----------------+----------+--------------------------------
    n5_numbertheo… | task-n5_numberthe… | MaxTxExhausted  |        1 | 5d103aefd3247c18fc0980824466e44…
    n5_aime_1983_… | task-n5_aime_1983… | MaxTxExhausted  |        1 | 8d63d627fcf722982fa8488ed46cf17…
    n5_imo_1964_p… | task-n5_imo_1964_… | MaxTxExhausted  |        1 | b32ea19b16c5cb566a4c72c8f30b93c…

  Architect mandate (§6.2 ruling 2026-05-02) ✓:
    O(1) chain cost / O(N) auditability — failure evidence anchored on L4
    via system-emitted system_signature; raw log requires audit-role access
    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).

§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      5
    challengetx-Age… | worktx-task-n5_… | Short | ChallengeShort  | Agent_3        |        10000 |     91
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      5
    challengetx-Age… | worktx-task-n5_… | Short | ChallengeShort  | Agent_3        |        10000 |     91
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      5
    challengetx-Age… | worktx-task-n5_… | Short | ChallengeShort  | Agent_3        |        10000 |     91
    ─── Total Long: 3000 micro | Total Short: 30000 micro | exposure rows: 6 ───

  Per-node exposure aggregation:
    node_id          | long_micro | short_micro | net (long − short)
    -----------------+------------+-------------+--------------------
    worktx-task-n5_… |       1000 |       10000 |              -9000
    worktx-task-n5_… |       1000 |       10000 |              -9000
    worktx-task-n5_… |       1000 |       10000 |              -9000

  Architect mandate (§3 + §10 ruling 2026-05-03) ✓:
    NodePosition is an IMMUTABLE EXPOSURE RECORD, NOT active position balance.
    NodePosition.amount is NOT a Coin holding (CR-12.1) and is NOT counted in
    total_supply_micro (CR-12.2). NO trading. NO price. NO settlement in TB-12.
    NodeMarketEntry is TB-14 derived view; flat NodePositionsIndex is canonical.

§14 TB-14 PriceIndex (architect 2026-05-03 §5.1 + §5.5 SG-14.6)
---------------------------------------------------------------
  PRICE IS SIGNAL, NOT TRUTH.
    Architect §5.1 ruling 2026-05-03: the price index is a
    derived statistical broadcast over canonical NodePositionsIndex
    long/short interest. It MUST NOT influence predicate gates
    (CR-14.1 / halt-trigger #1) or L4/L4.E classification
    (CR-14.2 / halt-trigger #2). Boolean predicates establish
    absolute bounds; the price view is for relative-effectiveness
    measurement only.

  Per-node entries (price as integer-rational n/d, never decimal):

    node_id                               long_micro     short_micro    price_yes(n/d)     price_no(n/d)
    --------------------------------------------------------------------------------------------------
    worktx-task-n5_aime_1983_p1_177…            1000           10000        1000/11000       10000/11000
    worktx-task-n5_imo_1964_p2_1778…            1000           10000        1000/11000       10000/11000
    worktx-task-n5_numbertheory_2po…            1000           10000        1000/11000       10000/11000

  Architect mandate (§5.1 ruling 2026-05-03) ✓:
    Price is signal, not truth. NodeMarketEntry is a derived view —
    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.
    NO price-based settlement. NO Goodhart leak of private predicates.

§15 TB-15 Autopsy + Markov (architect 2026-05-02 §6.5 SG-15.6)
--------------------------------------------------------------
  AUTOPSY IS PRIVATE — public summary shown only when typical
  (≥3 cluster). Raw private details require audit-role access.
    Architect §6.4 ruling 2026-05-02: capsule audit detail is
    AuditOnly; NEVER enters AgentVisibleProjection (CR-15.1 +
    halt-trigger #1 + #4).
    Typical-error broadcast surface uses public_summary text
    only (CR-15.2 + halt-trigger #5).

  (no agent_autopsies_t entries in this snapshot — no
  TaskBankruptcyTx has fired during the chain window)
  Acceptable signal-state: a run with zero accepted
  TaskBankruptcyTx yields an empty AutopsyIndex by
  TB-15 Atom 3 charter scope (single trigger site).

  Markov default (FR-15.4): next-session boot reads
  constitution.md + latest Markov capsule. deeper history
  requires TURINGOS_MARKOV_OVERRIDE=1 (CR-15.6 +
  halt-trigger #6 — default-deny gate).

  (no latest Markov capsule pointer — supply
  --markov-capsule-cid <hex> on the audit_dashboard
  invocation, or run `generate_markov_capsule` to
  emit a per-run capsule and pass its cid here.
  Per architect OBS_R022 ruling 2026-05-04 the
  global LATEST_MARKOV_CAPSULE.txt file has been
  de-canonicalized — runtime path no longer reads it)

  Architect mandate (§6.5 SG-15.6 + §6.4 ruling 2026-05-02) ✓:
    Dashboard regenerates capsule summary from ChainTape + CAS;
    NO raw private detail in dashboard output. Markov default
    prevents context poisoning — full failure history not auto-
    replayed; only constitution + latest capsule by default.

§16 TB-16 SANDBOX BANNER (architect 2026-05-03 §7.4 CR-16.7 + §7.5 SG-16.8)
==========================================================================
  ⚠ SANDBOX-RUN — NOT PRODUCTION — NO REAL FUNDS
    Agent IDs are sandbox-prefixed (Agent_solver_/Agent_verifier_/
    Agent_user_/tb7-7-sponsor/tb16-). Total Coin sourced from
    runtime::bootstrap::default_pput_preseed_pairs() (35_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.


=== TB-N3 RUN REPORT ===
run_id: g_phase_real_7_structural_smoke_r7_20260515T0927Z_5239f4b_t000
epoch: 3

## §A Citation tree (accepted WorkTx by agent)
  - Agent_0: 3 accepted WorkTx

## §B Role activity
  - Agent_0: work_accepted=3 work_rejected=3 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_1: work_accepted=0 work_rejected=3 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=3
  - Agent_2: work_accepted=0 work_rejected=3 verify_accepted=3 verify_rejected=0 challenge_accepted=0 invest_accepted=3
  - Agent_3: work_accepted=0 work_rejected=3 verify_accepted=0 verify_rejected=0 challenge_accepted=3 invest_accepted=0
  - Agent_4: work_accepted=0 work_rejected=3 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_5: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_6: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_7: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_8: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_9: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_solver_0: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_user_0: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - MarketMakerBudget: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - tb6-smoke-agent: work_accepted=0 work_rejected=3 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - tb6-smoke-sponsor: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0

## §C Market tx counts
  TaskOpen: 3
  EscrowLock: 3
  Work: 3
  Verify: 3
  FinalizeReward: 3
  EventResolve: 3
  CompleteSetMint: 0
  MarketSeed: 3
  CpmmPool: 3
  CpmmSwap: 0
  BuyWithCoinRouter: 6
  CompleteSetRedeem: 0
  CompleteSetMerge: 0
  Challenge: 3
  ChallengeResolve: 0
  TaskBankruptcy: 0
  TaskExpire: 0
  TerminalSummary: 3
  accepted_work_tx_total: 3

## §D Top contested nodes (by seed total μC)
  (no node-survive pools seeded)

## §E Budget burn report
  pools_created: 0
  market_seed_total: 0 μC
  treasury_budget_start: 5000000 μC (MarketMakerBudget genesis)
  treasury_budget_end: 5000000 μC (= start - market_seed_total)
  pools_skipped_budget: 3
  router_buy_yes: 3
  router_buy_no: 3

## §F MarketDecisionTrace summary
  total_traces: 5
  outcome[no_trade] = 5
  submitted_vs_traced_ratio: 0/5 = 0%
  no_trade reason breakdown (observed, sorted by count):
    no_perceived_edge = 5

## §F.A NoTradeReason exhaustive breakdown
  (architect §G2 13-variant taxonomy; stable insertion order; zeros included for forward grep stability)
  no_prompt_tool = 0
  no_parsed_invest = 0
  malformed_node = 0
  zero_amount = 0
  amount_exceeds_balance = 0
  no_pool = 0
  router_rejected = 0
  agent_declined = 0
  too_fast_solve = 0
  slippage_out_zero = 0
  unknown = 0
  no_perceived_edge = 5
  prompt_budget_exceeded = 0

## §F.X Peer-verify coverage
  accepted_worktx_total: 3
  accepted_worktx_with_verify: 3
  coverage_pct: 100%
  peer_verifications_total: 3
  non_solver_verifications: 3
  per-agent peer_verify_count:
    - Agent_2 (non_solver): 3

## §G PnL trajectory
  (per-agent realized/unrealized PnL over the batch; integer-rational μC; cost basis 1 μC/share-pair)
  - tb7-7-sponsor: balance=9700000 μC (initial 10000000); realized=-300000; unrealized=0; positions=0; rep=0; solvent
  - Agent_user_0: balance=10000000 μC (initial 10000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_0: balance=1297000 μC (initial 1000000); realized=297000; unrealized=0; positions=6; rep=0; solvent
  - Agent_1: balance=970000 μC (initial 1000000); realized=-30000; unrealized=-261; positions=3; rep=0; solvent
  - Agent_2: balance=970000 μC (initial 1000000); realized=-30000; unrealized=282; positions=3; rep=3; solvent
  - Agent_3: balance=970000 μC (initial 1000000); realized=-30000; unrealized=0; positions=3; rep=0; solvent
  - Agent_4: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_5: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_6: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_7: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_8: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_9: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - MarketMakerBudget: balance=4700000 μC (initial 5000000); realized=-300000; unrealized=0; positions=3; rep=0; solvent

## §G.2 RiskCapImpactReport
  (bankruptcy risk-cap admission rejections; derived from L4.E + CAS + replayed QState)
  risk_cap_rejections: 0
  (no BankruptcyRiskCapExceeded L4.E rows in this run)

## §G.3 Model-family activity
  source: GenesisReport + AttemptTelemetry + ChainTape + CAS
  interpretation: activity/divergence only; no model ranking
  hidden_switch_verdict: Proceed
  - deepseek: attempt_count_by_model_family=15 accepted_worktx_by_model_family=3 l4e_rejection_by_model_family=15 verify_count_by_model_family=3 challenge_count_by_model_family=3 invest_count_by_model_family=6 pnl_by_model_family=207021μC

## §I Role activity classifier
  source: public ChainTape/CAS activity counts only
  Agent_0: role=Solver work=3 verify=0 challenge=0 invest=0
  Agent_1: role=Trader work=0 verify=0 challenge=0 invest=3
  Agent_2: role=Trader work=0 verify=3 challenge=0 invest=3
  Agent_3: role=Challenger work=0 verify=0 challenge=3 invest=0
  Agent_4: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_5: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_6: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_7: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_8: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_9: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_solver_0: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_user_0: role=Observer work=0 verify=0 challenge=0 invest=0
  MarketMakerBudget: role=Observer work=0 verify=0 challenge=0 invest=0
  tb6-smoke-agent: role=Observer work=0 verify=0 challenge=0 invest=0
  tb6-smoke-sponsor: role=Observer work=0 verify=0 challenge=0 invest=0

## §J Epistemic pricing feedback (observe-only)
  source: MarketDecisionTrace + ChainTape market activity
  interpretation: price is signal, not truth; no predicate authority
  citation_vs_price: submitted_market_traces=0 total_market_traces=5
  high_price_selection_rate: observed_market_visible_actions=6 (integer count; benchmark protocol required before ranking claims)
  unresolved_challenged_filter: open Challenge targets are excluded from prompt market_context top-K

## §J.1 Opportunity Scheduler recommendation (observe-only)
  interpretation: non-binding materialized view; price is signal, not truth
  recommendation does not change sequencer admission or L4/L4.E predicates
  head_t: HEAD_t(l4_head=ad146b0b7699d3bbd6dddc9fdd88962ef5837077,l4e_head=6ef8e67b40b3f2f3910a532d4154828ba62eb58abfa2cc2ab6639b13a0a7cb3c,cas_root=778e860f87286352343e0d5836f974487fda7bd26804d2415c382cbee42bc99c,state_root=70ef4022f6477e81a9a9a19a4f007f778cc8500015312d0aa144b57154fa54b6,run_id=g_phase_real_7_structural_smoke_r7_20260515T0927Z_5239f4b_t000)
  observe_only: true
  visible_agents: 15
  visible_nodes: 0
  price_signals: 3
  pnl_signals: 13
  recommended_agent: Agent_0
  recommended_role: Trader
  recommended_action: observe_market_signal
  price_signal_sample:
    - event=task-n5_numbertheory_2pownm1prime_nprime_1778837658702 price=100000/200000 depth_micro=200000
    - event=task-n5_aime_1983_p1_1778837677909 price=100000/200000 depth_micro=200000
    - event=task-n5_imo_1964_p2_1778837711592 price=100000/200000 depth_micro=200000
  pnl_signal_sample:
    - agent=tb7-7-sponsor realized_pnl=-300000μC unrealized_pnl=0μC available=9700000μC risk_cap=1000000μC
    - agent=Agent_user_0 realized_pnl=0μC unrealized_pnl=0μC available=10000000μC risk_cap=1000000μC
    - agent=Agent_0 realized_pnl=297000μC unrealized_pnl=0μC available=1297000μC risk_cap=100000μC
  persisted_scheduler_trace_cas_count: 15
    - scheduler_trace_cid=cid:0c3f11bd9a82b319da5a1073c7faf4c93d209bc8433509675fc3e51f4f9a79cd
    - scheduler_trace_cid=cid:105f2079af0da5cd0ff8114f69bac08527a60ba6f1e866adb3af64c9ab4c1a2f
    - scheduler_trace_cid=cid:1cf90c6e35ef91238e0b679db026139160a8beea698b456e2e09a74bc4eeb030

## §K G7 structural smoke
  minimum_tier_green: true
  clean_negative: false
  forward_tb_stub_required: false
  one_runtime_repo: true
  multi_agent: true
  persistent_state: true
  agent_count: 15
  active_role_count: 5
  task_count: 3
  task_outcome_market_count: 3
  scripted_attempt_prediction_market_count: 3
  buy_yes_router_count: 3
  buy_no_or_short_count: 6
  verify_tx_count: 3
  challenge_tx_or_no_challenge_reason_count: 3
  event_resolve_count: 3
  pnl_delta_count: 6
  loss_occurred: false
  autopsy_capsule_count: 0
  autopsy_if_loss_satisfied: true
  no_forced_live_investment: true
  market_actions_chain_visible: true
  no_ghost_liquidity: true
  clean_v3_comparison: true
  does_not_claim_identical_v3_equivalence: true
  proof_related_actions: 6
  market_visible_actions: 9
  no_trade_reason_count: 5

## §H PRICE IS SIGNAL, NOT TRUTH
  Pool reserves and prices in this report are derived
  views over ChainTape + CAS evidence. Prices are
  expressed as integer-rational (numerator/denominator).
  No Coin minted post-init; no ghost liquidity; no f64.
