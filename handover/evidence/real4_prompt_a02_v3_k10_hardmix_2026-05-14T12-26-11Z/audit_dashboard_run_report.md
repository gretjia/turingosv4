=================================================================
 TB-8 Audit Dashboard — run_id=real4_prompt_a02_v3_k10_hardmix_2026-05-14T12-26-11Z_0512b50_t000 epoch=3
=================================================================

§1 Run metadata
---------------
  head_commit_oid: d7c0128988b2e183015d25ce3b46f0b8b267490e
  final_state_root: 61498ed287b14921b698b32cbfffc6f9b3a2c911d00bd10acee85b604ffa5ceb
  final_ledger_root: 5481cf7c53c1e8c81254d2ca659abfde35d60ec1c7baa09b40d6b5edaa3583d5
  initial_q_state_loaded_from_disk: true

§2 Chain stats + 7 indicators
------------------------------
  L4 entries:  8
  L4.E entries: 27
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
  tx_count                : 35
  proposal_count          : 20
  golden_path_token_count : 20959
  gp_payload (CID hex)    : -
  gp_path                 : -
  tactic_diversity        : 1
  failed_branch_count     : 27
  chain_oracle_verified   : true ✓ (Lean accepted ≥1 proof; oracle-level)
  chain_economic_finalized: false (always false in TB-7; settlement = TB-9 territory)
  tool_dist:
    step_complete: 15

§4 Per-agent activity
---------------------
  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗
  ------------------+--------+-------+-------+---------+--------
  Agent_0           | ✓      | 1     | 3     | 0       | 1
  Agent_1           | ✓      | 0     | 1     | 0       | 0
  Agent_2           | ✓      | 0     | 1     | 0       | 0
  Agent_3           | ✓      | 0     | 2     | 0       | 0
  Agent_4           | ✓      | 0     | 1     | 0       | 0
  Agent_5           | ✓      | 0     | 1     | 0       | 0
  Agent_6           | ✓      | 0     | 2     | 0       | 0
  Agent_7           | ✓      | 0     | 2     | 0       | 0
  Agent_8           | ✓      | 0     | 2     | 0       | 0
  Agent_9           | ✓      | 0     | 1     | 0       | 0
  Agent_solver_0    | ✓      | 0     | 0     | 0       | 0
  Agent_user_0      | ✓      | 0     | 0     | 0       | 0
  MarketMakerBudget | ✓      | 0     | 0     | 0       | 0
  tb6-smoke-agent   | ✗      | 0     | 3     | 0       | 0
  tb6-smoke-sponsor | ✗      | 0     | 0     | 0       | 0
  tb7-7-sponsor     | ✗      | 0     | 0     | 0       | 0

§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Verify          | Agent_0    | -          | -          | -      | PolicyViolation
  L4.E  |   0 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | EscrowMissing
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b1 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_3    | step_complete | Agent_3.b4 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_6    | step_complete | Agent_6.b7 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_8    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_9    | step_complete | Agent_9.b10 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b11 | -      | PolicyViolation
  L4.E  |   0 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | EscrowMissing
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b1 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_1    | step_complete | Agent_1.b2 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_2    | step_complete | Agent_2.b3 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_3    | step_complete | Agent_3.b4 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b5 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_5    | step_complete | Agent_5.b6 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_6    | step_complete | Agent_6.b7 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_7    | step_complete | Agent_7.b8 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b9 | -      | EscrowMissing
  L4    |   1 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
  L4    |   2 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
  L4    |   3 | Work            | Agent_0    | step_complete | Agent_0.b1 | ✓      | -
        payload: nlinarith
  L4    |   4 | TaskOpen        | MarketMakerBudget | -          | -          | -      | -
  L4    |   5 | MarketSeed      | -          | -          | -          | -      | -
  L4    |   6 | CpmmPool        | -          | -          | -          | -      | -
  L4    |   7 | TerminalSummary | -          | -          | -          | -      | -
  L4    |   8 | TerminalSummary | -          | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: MissingParentTxViolation ✗ (≥1 multi-attempt branch with missing parent_tx — wiring broken)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  ✓depth=0  [ORACLE] | agent=Agent_0 | tactic=step_complete | tx=worktx-task-n10_mathd_algebra_107_1778761591928-omega-pertactic-1
           payload: nlinarith

§8 Cross-checks
---------------
  audit_trail_rows         : 6
  chain_proposal_count     : 20
  audit_rows == proposal_count: ✗ (gap)
  audit_trail_chain_valid     : ✓
  (Note: pre-TB-7.6 the agent_audit_trail.jsonl is populated only
   by the synthetic-seed hook; full per-LLM-proposal audit-trail
   wiring is part of TB-7.6 carry-forward action #4 / #5.)

§9 TB-8 Claims (claim_status + payout_amount)
----------------------------------------------
  (no Confirm-VerifyTx observed; n/a — claim_status / payout: n/a)

§10 TB-9 Durable identity (agent keystore registry)
---------------------------------------------------
  durable_keystore_path: /home/zephryj/.turingos/keystore/agent_keystore.enc
  durable_keystore_present: ✓ (cross-run identity available)
  agents_in_manifest: 13
  agent_id          | pubkey_in_manifest | tape_activity
  ------------------+--------------------+---------------
  Agent_0           | ✓ (durable-backed) | Work✓=1 Work✗=3 Verify✓=0 Verify✗=1
  Agent_1           | ✓ (durable-backed) | Work✓=0 Work✗=1 Verify✓=0 Verify✗=0
  Agent_2           | ✓ (durable-backed) | Work✓=0 Work✗=1 Verify✓=0 Verify✗=0
  Agent_3           | ✓ (durable-backed) | Work✓=0 Work✗=2 Verify✓=0 Verify✗=0
  Agent_4           | ✓ (durable-backed) | Work✓=0 Work✗=1 Verify✓=0 Verify✗=0
  Agent_5           | ✓ (durable-backed) | Work✓=0 Work✗=1 Verify✓=0 Verify✗=0
  Agent_6           | ✓ (durable-backed) | Work✓=0 Work✗=2 Verify✓=0 Verify✗=0
  Agent_7           | ✓ (durable-backed) | Work✓=0 Work✗=2 Verify✓=0 Verify✗=0
  Agent_8           | ✓ (durable-backed) | Work✓=0 Work✗=2 Verify✓=0 Verify✗=0
  Agent_9           | ✓ (durable-backed) | Work✓=0 Work✗=1 Verify✓=0 Verify✗=0
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
    n10_mathd_alg… | task-n10_mathd_al… | DegradedLLM     |       11 | c2de2404ef35df4d9080b721c91d11b…
    n10_mathd_alg… | task-n10_mathd_al… | DegradedLLM     |        9 | d3fd3fe59c9c249afb206ca69dcd28f…

  Architect mandate (§6.2 ruling 2026-05-02) ✓:
    O(1) chain cost / O(N) auditability — failure evidence anchored on L4
    via system-emitted system_signature; raw log requires audit-role access
    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).

§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n10… | worktx-task-n10… | Long  | FirstLong       | Agent_0        |      1000000 |      2
    ─── Total Long: 1000000 micro | Total Short: 0 micro | exposure rows: 1 ───

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
    worktx-task-n10_mathd_algebra_1…         1000000               0   1000000/1000000         0/1000000

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

  Per-event Cid counts (capsule bytes live in CAS;
  audit-role required to fetch private_detail):

    event_id                                           cid_count
    ------------------------------------------------------------
    task-n10_mathd_algebra_125_1778761613256                   1
    task-n10_mathd_algebra_141_1778761753210                   1
    ─── total: 2 capsule Cid(s) across 2 event(s) ───

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
