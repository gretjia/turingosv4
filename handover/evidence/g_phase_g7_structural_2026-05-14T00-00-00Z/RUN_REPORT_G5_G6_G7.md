=================================================================
 TB-8 Audit Dashboard — run_id=g_phase_g4_2_mini_challenge_fix_2026-05-13T14-33-04Z_0096ad7_t000 epoch=3
=================================================================

§1 Run metadata
---------------
  head_commit_oid: 29114224d1e3dc58bbca053d63ce8e3ce9149cf8
  final_state_root: c24fd6c24313a12dcb2da9ed67f1d08477a13679b4d57a3f15feedf32f739d09
  final_ledger_root: f218680d3f4f00b0c9171ae39607bb53cbd95eec5048b76ccc6e42ced443323a
  initial_q_state_loaded_from_disk: true

§2 Chain stats + 7 indicators
------------------------------
  L4 entries:  8
  L4.E entries: 154
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
  tx_count                : 162
  proposal_count          : 147
  golden_path_token_count : 33607
  gp_payload (CID hex)    : -
  gp_path                 : -
  tactic_diversity        : 1
  failed_branch_count     : 154
  chain_oracle_verified   : true ✓ (Lean accepted ≥1 proof; oracle-level)
  chain_economic_finalized: false (always false in TB-7; settlement = TB-9 territory)
  tool_dist:
    step_complete: 33

§4 Per-agent activity
---------------------
  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗
  ------------------+--------+-------+-------+---------+--------
  Agent_0           | ✓      | 0     | 16    | 0       | 0
  Agent_1           | ✓      | 0     | 16    | 0       | 0
  Agent_2           | ✓      | 0     | 16    | 0       | 0
  Agent_3           | ✓      | 0     | 16    | 0       | 0
  Agent_4           | ✓      | 1     | 13    | 0       | 1
  Agent_5           | ✓      | 0     | 14    | 0       | 0
  Agent_6           | ✓      | 0     | 14    | 0       | 0
  Agent_7           | ✓      | 0     | 14    | 0       | 0
  Agent_8           | ✓      | 0     | 11    | 0       | 0
  Agent_9           | ✓      | 0     | 13    | 0       | 0
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
  L4.E  |   0 | Work            | Agent_0    | -          | -          | -      | ParseFailed
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Verify          | Agent_4    | -          | -          | -      | PolicyViolation
  L4.E  |   0 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | EscrowMissing
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b1 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b4 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b7 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b8 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b10 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b13 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b16 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | -          | -          | -      | LeanFailed
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | EscrowMissing
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b1 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b2 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b3 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b4 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b5 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b6 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b7 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b8 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b9 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b10 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b11 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b12 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b13 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b14 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b15 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b16 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b17 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b18 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b19 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b20 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b21 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b22 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_4    | step_complete | Agent_4.b23 | -      | PolicyViolation
  L4.E  |   0 | Work            | Agent_5    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_6    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_7    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_8    | step_complete | Agent_8.b24 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_9    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_0    | step_complete | Agent_0.b25 | -      | EscrowMissing
  L4.E  |   0 | Work            | Agent_1    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_2    | -          | -          | -      | LlmError
  L4.E  |   0 | Work            | Agent_3    | -          | -          | -      | LlmError
  L4    |   1 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
  L4    |   2 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
  L4    |   3 | Work            | Agent_4    | step_complete | Agent_4.b1 | ✓      | -
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
  ✓depth=0  [ORACLE] | agent=Agent_4 | tactic=step_complete | tx=worktx-task-n10_mathd_algebra_107_1778682813290-omega-pertactic-1
           payload: nlinarith

§8 Cross-checks
---------------
  audit_trail_rows         : 6
  chain_proposal_count     : 147
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
  Agent_0           | ✓ (durable-backed) | Work✓=0 Work✗=16 Verify✓=0 Verify✗=0
  Agent_1           | ✓ (durable-backed) | Work✓=0 Work✗=16 Verify✓=0 Verify✗=0
  Agent_2           | ✓ (durable-backed) | Work✓=0 Work✗=16 Verify✓=0 Verify✗=0
  Agent_3           | ✓ (durable-backed) | Work✓=0 Work✗=16 Verify✓=0 Verify✗=0
  Agent_4           | ✓ (durable-backed) | Work✓=1 Work✗=13 Verify✓=0 Verify✗=1
  Agent_5           | ✓ (durable-backed) | Work✓=0 Work✗=14 Verify✓=0 Verify✗=0
  Agent_6           | ✓ (durable-backed) | Work✓=0 Work✗=14 Verify✓=0 Verify✗=0
  Agent_7           | ✓ (durable-backed) | Work✓=0 Work✗=14 Verify✓=0 Verify✗=0
  Agent_8           | ✓ (durable-backed) | Work✓=0 Work✗=11 Verify✓=0 Verify✗=0
  Agent_9           | ✓ (durable-backed) | Work✓=0 Work✗=13 Verify✓=0 Verify✗=0
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
    n10_mathd_alg… | task-n10_mathd_al… | DegradedLLM     |       17 | f714ffef526a74e5d14e3ef8a1b4e3e…
    n10_mathd_alg… | task-n10_mathd_al… | WallClockCap    |       25 | 5ea14b31f101da568e8a374476c5f10…

  Architect mandate (§6.2 ruling 2026-05-02) ✓:
    O(1) chain cost / O(N) auditability — failure evidence anchored on L4
    via system-emitted system_signature; raw log requires audit-role access
    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).

§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n10… | worktx-task-n10… | Long  | FirstLong       | Agent_4        |      1000000 |      2
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
    task-n10_mathd_algebra_125_1778682896295                   1
    task-n10_mathd_algebra_141_1778683264763                   1
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


=== TB-N3 RUN REPORT ===
run_id: g_phase_g4_2_mini_challenge_fix_2026-05-13T14-33-04Z_0096ad7_t000
epoch: 3

## §A Citation tree (accepted WorkTx by agent)
  - Agent_4: 1 accepted WorkTx

## §B Role activity
  - Agent_0: work_accepted=0 work_rejected=16 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_1: work_accepted=0 work_rejected=16 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_2: work_accepted=0 work_rejected=16 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_3: work_accepted=0 work_rejected=16 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_4: work_accepted=1 work_rejected=13 verify_accepted=0 verify_rejected=1 challenge_accepted=0 invest_accepted=0
  - Agent_5: work_accepted=0 work_rejected=14 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_6: work_accepted=0 work_rejected=14 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_7: work_accepted=0 work_rejected=14 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_8: work_accepted=0 work_rejected=11 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_9: work_accepted=0 work_rejected=13 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_solver_0: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - Agent_user_0: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - MarketMakerBudget: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - tb6-smoke-agent: work_accepted=0 work_rejected=3 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - tb6-smoke-sponsor: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0
  - tb7-7-sponsor: work_accepted=0 work_rejected=0 verify_accepted=0 verify_rejected=0 challenge_accepted=0 invest_accepted=0

## §C Market tx counts
  TaskOpen: 2
  EscrowLock: 1
  Work: 1
  Verify: 0
  FinalizeReward: 0
  EventResolve: 0
  CompleteSetMint: 0
  MarketSeed: 1
  CpmmPool: 1
  CpmmSwap: 0
  BuyWithCoinRouter: 0
  CompleteSetRedeem: 0
  CompleteSetMerge: 0
  Challenge: 0
  ChallengeResolve: 0
  TaskBankruptcy: 0
  TaskExpire: 0
  TerminalSummary: 2
  accepted_work_tx_total: 1

## §D Top contested nodes (by seed total μC)
  - node_survive:worktx-task-n10_mathd_algebra_107_1778682813290-omega-pertactic-1: 100000 μC

## §E Budget burn report
  pools_created: 1
  market_seed_total: 100000 μC
  treasury_budget_start: 5000000 μC (MarketMakerBudget genesis)
  treasury_budget_end: 4900000 μC (= start - market_seed_total)
  pools_skipped_budget: 0
  router_buy_yes: 0
  router_buy_no: 0

## §F MarketDecisionTrace summary
  total_traces: 0
  submitted_vs_traced_ratio: 0/0 = n/a (no traces)

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
  no_perceived_edge = 0
  prompt_budget_exceeded = 0

## §F.X Peer-verify coverage
  accepted_worktx_total: 1
  accepted_worktx_with_verify: 0
  coverage_pct: 0%
  peer_verifications_total: 0
  non_solver_verifications: 0
  MECHANISM BOTTLENECK (architect §8.2 ship gate unmet — no non-solver VerifyTx):
    1. round-robin scheduler `agent_idx = tx % n_agents`
       (G-Phase directive amendment G-4 "伪多智能体"):
       Agent_i may never be selected for verify path;
       G5.1 opportunity scheduler + 7-action menu is the
       forward fix.
    2. `=== Pending Peer Reviews ===` prompt block (G2P.1)
       must be active on the swarm prompt path so agents
       perceive eligible targets at the δ Agent externalized
       output node.
    3. agent verify_peer bond budget (TB-N1 A4 admission step-2.5)
       may exceed balance after WorkTx stake locked — confirm
       persistent-batch preseed (TURINGOS_CHAINTAPE_PRESEED=1)
       seeds non-solver agents with adequate balance.

## §G PnL trajectory
  (per-agent realized/unrealized PnL over the batch; integer-rational μC; cost basis 1 μC/share-pair)
  - tb7-7-sponsor: balance=9900000 μC (initial 10000000); realized=-100000; unrealized=0; positions=0; rep=0; solvent
  - Agent_user_0: balance=10000000 μC (initial 10000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_0: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_1: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_2: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_3: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_4: balance=0 μC (initial 1000000); realized=-1000000; unrealized=0; positions=2; rep=0; bankrupt
  - Agent_5: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_6: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_7: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_8: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - Agent_9: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
  - MarketMakerBudget: balance=4900000 μC (initial 5000000); realized=-100000; unrealized=0; positions=1; rep=0; solvent

## §G.2 RiskCapImpactReport
  (bankruptcy risk-cap admission rejections; derived from L4.E + CAS + replayed QState)
  risk_cap_rejections: 9
  agent_id | balance_before_micro | risk_cap_micro | tx_kind | task_id | another_agent_continued | solve_outcome
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_125_1778682896295 | false | DegradedLLM
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap
  - Agent_4 | 0 | 100000 | work | task-n10_mathd_algebra_141_1778683264763 | false | WallClockCap

## §G.3 Model-family activity
  source: GenesisReport + AttemptTelemetry + ChainTape + CAS
  interpretation: activity/divergence only; no model ranking
  hidden_switch_verdict: Proceed
  - claude: attempt_count_by_model_family=30 accepted_worktx_by_model_family=0 l4e_rejection_by_model_family=30 verify_count_by_model_family=0 challenge_count_by_model_family=0 invest_count_by_model_family=0 pnl_by_model_family=0μC
  - deepseek: attempt_count_by_model_family=44 accepted_worktx_by_model_family=1 l4e_rejection_by_model_family=41 verify_count_by_model_family=0 challenge_count_by_model_family=0 invest_count_by_model_family=0 pnl_by_model_family=-1000000μC
  - openai: attempt_count_by_model_family=43 accepted_worktx_by_model_family=0 l4e_rejection_by_model_family=43 verify_count_by_model_family=0 challenge_count_by_model_family=0 invest_count_by_model_family=0 pnl_by_model_family=0μC
  - qwen: attempt_count_by_model_family=30 accepted_worktx_by_model_family=0 l4e_rejection_by_model_family=30 verify_count_by_model_family=0 challenge_count_by_model_family=0 invest_count_by_model_family=0 pnl_by_model_family=0μC

## §I Role activity classifier
  source: public ChainTape/CAS activity counts only
  Agent_0: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_1: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_2: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_3: role=Observer work=0 verify=0 challenge=0 invest=0
  Agent_4: role=Solver work=1 verify=0 challenge=0 invest=0
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
  tb7-7-sponsor: role=Observer work=0 verify=0 challenge=0 invest=0

## §J Epistemic pricing feedback (observe-only)
  source: MarketDecisionTrace + ChainTape market activity
  interpretation: price is signal, not truth; no predicate authority
  citation_vs_price: submitted_market_traces=0 total_market_traces=0
  high_price_selection_rate: observed_market_visible_actions=1 (integer count; benchmark protocol required before ranking claims)
  unresolved_challenged_filter: open Challenge targets are excluded from prompt market_context top-K

## §K G7 structural smoke
  minimum_tier_green: true
  clean_negative: false
  forward_tb_stub_required: false
  one_runtime_repo: true
  multi_agent: true
  persistent_state: true
  proof_related_actions: 1
  market_visible_actions: 1
  no_trade_reason_count: 0

## §H PRICE IS SIGNAL, NOT TRUTH
  Pool reserves and prices in this report are derived
  views over ChainTape + CAS evidence. Prices are
  expressed as integer-rational (numerator/denominator).
  No Coin minted post-init; no ghost liquidity; no f64.
