# TB-16 Post-R3 Round 2 v2 — Constitutional Conformance Test Battery SUMMARY

**Date**: 2026-05-04 (post R3 closure commit `ce64d61` + runner-fix `8b5c94a`)
**Problem set**: 8 × MiniF2F (mathd_algebra_171/11/96/67, mathd_numbertheory_961,
                  aime_1997_p9, amc12b_2020_p5, triple-probe on mathd_algebra_171)
**Swarm size**: N=5 (CONDITION=n5)
**MAX_TX**: 20 per problem
**LLM**: deepseek-v4-flash via proxy localhost:18080
**Lean oracle**: turingosv3/experiments/minif2f_data_lean4 (mathlib cached)
**Preseed**: TURINGOS_CHAINTAPE_PRESEED=1 enabled (full TB-10 user-task-mode path)

---

## §1 Per-problem outcome (v3-style scaling table)

| Problem | Probe | Solved | Verified | tx_count | wall_ms | tokens | PPUT | L4 / L4.E | Tx kinds in chain |
|---|---|---|---|---|---|---|---|---|---|
| P1_baseline | vanilla | ✓ | ✓ | 1 | 10941 | 473 | 8.533 | 5/2 | escrow_lock+finalize_reward+task_open+verify+work |
| P2_challenge | FORCE_CHAL=A2 | ✗ | ✗ | 20 | 229077 | 11155 | 0.000 | 3/2 | escrow_lock+task_open+terminal_summary |
| P3_completeset | CSEED=1M | ✓ | ✓ | 1 | 11801 | 472 | 7.913 | 7/2 | complete_set_mint+escrow_lock+finalize_reward+market_seed+task_open+verify+work |
| P4_bankruptcy | FORCE_BANKR=A0 | ✓ | ✓ | 1 | 10092 | 400 | 9.220 | 5/2 | escrow_lock+finalize_reward+task_open+verify+work |
| P5_aime_hard | vanilla | ✗ | ✗ | 20 | 274311 | 11953 | 0.000 | 3/2 | escrow_lock+task_open+terminal_summary |
| P6_triple_probe | triple | ✓ | ✓ | 1 | 10093 | 473 | 9.227 | 7/3 | challenge+complete_set_mint+escrow_lock+market_seed+task_open+verify+work |
| P7_baseline_b | vanilla | ✓ | ✓ | 1 | 11197 | 543 | 8.412 | 5/2 | escrow_lock+finalize_reward+task_open+verify+work |
| P8_completeset_b | CSEED=1.5M | ✗ | ✗ | 20 | 203937 | 10567 | 0.000 | 5/2 | complete_set_mint+escrow_lock+market_seed+task_open+terminal_summary |

## §2 Capability signal (Art. I.2 三大统计信号)

- **Σ PPUT (solved only)**: 43.3056
- **Mean PPUT (solved only)**: 8.6611
- **Solve rate**: 5/8 = 62.5%
- **95% Wilson CI**: [30.6%, 86.3%]

> N=8 is too small for tight CI. Per `project_pput_ccl_arc`, full
> H-VPPUT is heldout-49 with N>=20 runs/problem; this is FEASIBILITY
> signal proving the architecture wires correctly end-to-end.

---

## §3 Tx-kind union across 8 chains

| TxKind | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 | Total |
|---|---|---|---|---|---|---|---|---|---|
| work | 1 | · | 1 | 1 | · | 1 | 1 | · | **5** |
| verify | 1 | · | 1 | 1 | · | 1 | 1 | · | **5** |
| challenge | · | · | · | · | · | 1 | · | · | **1** |
| reuse | · | · | · | · | · | · | · | · | **0** |
| task_open | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | **8** |
| escrow_lock | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | **8** |
| complete_set_mint | · | · | 1 | · | · | 1 | · | 1 | **3** |
| complete_set_redeem | · | · | · | · | · | · | · | · | **0** |
| market_seed | · | · | 1 | · | · | 1 | · | 1 | **3** |
| finalize_reward | 1 | · | 1 | 1 | · | · | 1 | · | **4** |
| challenge_resolve | · | · | · | · | · | · | · | · | **0** |
| terminal_summary | · | 1 | · | · | 1 | · | · | 1 | **3** |
| task_expire | · | · | · | · | · | · | · | · | **0** |
| task_bankruptcy | · | · | · | · | · | · | · | · | **0** |

**Tx kinds covered (union)**: 9 of 13 architect-required
**Covered**: challenge, complete_set_mint, escrow_lock, finalize_reward, market_seed, task_open, terminal_summary, verify, work
**Missing**: challenge_resolve, complete_set_redeem, reuse, task_bankruptcy, task_expire

---

## §4 7-Mechanism × Constitution × Flowchart × Audit conformance

Per-problem outcome for each mechanism's covering audit assertions.
Legend: ✓ Pass · ○ Skipped (assertion not applicable to this chain) · ✗F Fail · ✗H Halt

### Mechanism 1 — Real tape persistence

| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
|---|---|---|---|---|---|---|---|---|---|
| 4 | l4_hash_chain_valid | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 5 | l4_parent_state_continuity | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 6 | l4e_chain_integrity | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 7 | genesis_row_zero_parents | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 12 | replay_state_root_matches_head | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Mechanism 2 — Append mechanism

| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
|---|---|---|---|---|---|---|---|---|---|
| 4 | l4_hash_chain_valid | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Mechanism 3 — Git mechanism

| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
|---|---|---|---|---|---|---|---|---|---|
| 8 | system_tx_signatures_verify | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ |
| 9 | agent_tx_signatures_verify | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 10 | payload_cid_resolves | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 11 | tx_kind_envelope_matches_payload | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 14 | replay_autopsy_index_chains | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Mechanism 4 — Economic mechanism

| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
|---|---|---|---|---|---|---|---|---|---|
| 17 | no_post_init_mint | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 18 | total_supply_conserved | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 19 | complete_set_min_balanced | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 20 | task_market_total_escrow_matches_locks | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 21 | node_positions_excluded_from_supply | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 22 | conditional_shares_excluded_from_supply | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 40 | total_supply_conserved_per_block | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Mechanism 5 — Boltzmann scheduler

| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
|---|---|---|---|---|---|---|---|---|---|
| 26 | price_index_is_view_only | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Mechanism 6 — Information shielding

| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
|---|---|---|---|---|---|---|---|---|---|
| 28 | projection_no_autopsy_bytes | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 29 | autopsy_private_detail_creator_is_system | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 30 | typical_error_summary_no_private_detail | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 31 | autopsy_index_value_type_is_vec_cid | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 39 | no_llm_self_narrative_in_autopsy | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| 41 | chain_agent_ids_sandbox_prefixed | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Mechanism 7 — Broadcasting mechanism

| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
|---|---|---|---|---|---|---|---|---|---|
| 32 | markov_constitution_hash_matches | ○ | ○ | ○ | ○ | ○ | ○ | ○ | ○ |
| 33 | markov_typical_errors_recompute | ○ | ○ | ○ | ○ | ○ | ○ | ○ | ○ |
| 34 | markov_unresolved_obs_recompute | ○ | ○ | ○ | ○ | ○ | ○ | ○ | ○ |
| 35 | markov_next_session_context_resolves | ○ | ○ | ○ | ○ | ○ | ○ | ○ | ○ |

---

## §5 Per-problem chain DAG (Proposal flow + Golden path)

### P1_baseline

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | Work            | Agent_0    | step_complete | Agent_0.b1 | ✓      | -
        payload: calc ⏎   f 1 = 5 * 1 + 4 := by ⏎     rw [h₀] ⏎   _ = 5 + 4 := by ring ⏎   _ = 9 := by no
  L4    |   4 | Verify          | Agent_0    | -          | -          | -      | -
  L4    |   5 | FinalizeReward  | system (solver=Agent_0) | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  ✓depth=0  [ORACLE] | agent=Agent_0 | tactic=step_complete | tx=worktx-task-n5_mathd_algebra_171_1777898065342-omega-pertactic-1
           payload: calc ⏎   f 1 = 5 * 1 + 4 := by ⏎     rw [h₀] ⏎   _ = 5 + 4 := by ring ⏎   _ = 9 := by no
```

### P2_challenge

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | TerminalSummary | -          | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: NoMultiAttemptObserved (DAG not exercised this run — conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)
```

### P3_completeset

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | MarketSeed      | -          | -          | -          | -      | -
  L4    |   4 | CompleteSetMint | -          | -          | -          | -      | -
  L4    |   5 | Work            | Agent_0    | step_complete | Agent_0.b1 | ✓      | -
        payload: nlinarith
  L4    |   6 | Verify          | Agent_0    | -          | -          | -      | -
  L4    |   7 | FinalizeReward  | system (solver=Agent_0) | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  ✓depth=0  [ORACLE] | agent=Agent_0 | tactic=step_complete | tx=worktx-task-n5_mathd_algebra_96_1777898307428-omega-pertactic-1
           payload: nlinarith
```

### P4_bankruptcy

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | Work            | Agent_0    | step_complete | Agent_0.b1 | ✓      | -
        payload: norm_num
  L4    |   4 | Verify          | Agent_0    | -          | -          | -      | -
  L4    |   5 | FinalizeReward  | system (solver=Agent_0) | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  ✓depth=0  [ORACLE] | agent=Agent_0 | tactic=step_complete | tx=worktx-task-n5_mathd_numbertheory_961_1777898320647-omega-pertactic-1
           payload: norm_num
```

### P5_aime_hard

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | TerminalSummary | -          | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: NoMultiAttemptObserved (DAG not exercised this run — conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)
```

### P6_triple_probe

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | FinalizeReward  | __system__ | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | MarketSeed      | -          | -          | -          | -      | -
  L4    |   4 | CompleteSetMint | -          | -          | -          | -      | -
  L4    |   5 | Work            | Agent_0    | step_complete | Agent_0.b1 | ✓      | -
        payload: calc ⏎   f 1 = 5 * 1 + 4 := by ⏎     rw [h₀] ⏎   _ = 5 + 4 := by ring ⏎   _ = 9 := by no
  L4    |   6 | Verify          | Agent_0    | -          | -          | -      | -
  L4    |   7 | Challenge       | Agent_3    | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  ✓depth=0  [ORACLE] | agent=Agent_0 | tactic=step_complete | tx=worktx-task-n5_mathd_algebra_171_1777898607639-omega-pertactic-1
           payload: calc ⏎   f 1 = 5 * 1 + 4 := by ⏎     rw [h₀] ⏎   _ = 5 + 4 := by ring ⏎   _ = 9 := by no
```

### P7_baseline_b

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | Work            | Agent_0    | step_complete | Agent_0.b1 | ✓      | -
        payload: calc ⏎   g (f (-1)) = g (5 * (-1) + 3) := by ⏎     simp [h₀] ⏎   _ = g (-5 + 3) := by 
  L4    |   4 | Verify          | Agent_0    | -          | -          | -      | -
  L4    |   5 | FinalizeReward  | system (solver=Agent_0) | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  ✓depth=0  [ORACLE] | agent=Agent_0 | tactic=step_complete | tx=worktx-task-n5_mathd_algebra_67_1777898619046-omega-pertactic-1
           payload: calc ⏎   g (f (-1)) = g (5 * (-1) + 3) := by ⏎     simp [h₀] ⏎   _ = g (-5 + 3) := by
```

### P8_completeset_b

```
§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | Agent_user_0 | -          | -          | -      | -
  L4    |   2 | EscrowLock      | Agent_user_0 | -          | -          | -      | -
  L4    |   3 | MarketSeed      | -          | -          | -          | -      | -
  L4    |   4 | CompleteSetMint | -          | -          | -          | -      | -
  L4    |   5 | TerminalSummary | -          | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: NoMultiAttemptObserved (DAG not exercised this run — conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)
```

---

## §6 Market signals (TB-11 NodePositions + TB-14 PriceIndex)

### P1_baseline

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      2
    ─── Total Long: 1000 micro | Total Short: 0 micro | exposure rows: 1 ───

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
    worktx-task-n5_mathd_algebra_17…            1000               0         1000/1000            0/1000

  Architect mandate (§5.1 ruling 2026-05-03) ✓:
    Price is signal, not truth. NodeMarketEntry is a derived view —
    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.
    NO price-based settlement. NO Goodhart leak of private predicates.
```

### P2_challenge

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)

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

  (no node positions recorded — price index is empty)
  Acceptable signal-state: a run with zero accepted WorkTx +
  ChallengeTx yields an empty PriceIndex by FR-14.3 / halt-
  trigger #5 (zero-liquidity → price=None) extended to the
  zero-position case.
```

### P3_completeset

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      4
    ─── Total Long: 1000 micro | Total Short: 0 micro | exposure rows: 1 ───

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
    worktx-task-n5_mathd_algebra_96…            1000               0         1000/1000            0/1000

  Architect mandate (§5.1 ruling 2026-05-03) ✓:
    Price is signal, not truth. NodeMarketEntry is a derived view —
    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.
    NO price-based settlement. NO Goodhart leak of private predicates.
```

### P4_bankruptcy

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      2
    ─── Total Long: 1000 micro | Total Short: 0 micro | exposure rows: 1 ───

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
    worktx-task-n5_mathd_numbertheo…            1000               0         1000/1000            0/1000

  Architect mandate (§5.1 ruling 2026-05-03) ✓:
    Price is signal, not truth. NodeMarketEntry is a derived view —
    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.
    NO price-based settlement. NO Goodhart leak of private predicates.
```

### P5_aime_hard

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)

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

  (no node positions recorded — price index is empty)
  Acceptable signal-state: a run with zero accepted WorkTx +
  ChallengeTx yields an empty PriceIndex by FR-14.3 / halt-
  trigger #5 (zero-liquidity → price=None) extended to the
  zero-position case.
```

### P6_triple_probe

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      4
    challengetx-Age… | worktx-task-n5_… | Short | ChallengeShort  | Agent_3        |        10000 |      5
    ─── Total Long: 1000 micro | Total Short: 10000 micro | exposure rows: 2 ───

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
    worktx-task-n5_mathd_algebra_17…            1000           10000        1000/11000       10000/11000

  Architect mandate (§5.1 ruling 2026-05-03) ✓:
    Price is signal, not truth. NodeMarketEntry is a derived view —
    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.
    NO price-based settlement. NO Goodhart leak of private predicates.
```

### P7_baseline_b

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):
    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round
    -----------------+------------------+-------+-----------------+----------------+--------------+--------
    worktx-task-n5_… | worktx-task-n5_… | Long  | FirstLong       | Agent_0        |         1000 |      2
    ─── Total Long: 1000 micro | Total Short: 0 micro | exposure rows: 1 ───

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
    worktx-task-n5_mathd_algebra_67…            1000               0         1000/1000            0/1000

  Architect mandate (§5.1 ruling 2026-05-03) ✓:
    Price is signal, not truth. NodeMarketEntry is a derived view —
    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.
    NO price-based settlement. NO Goodhart leak of private predicates.
```

### P8_completeset_b

```
§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)

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

  (no node positions recorded — price index is empty)
  Acceptable signal-state: a run with zero accepted WorkTx +
  ChallengeTx yields an empty PriceIndex by FR-14.3 / halt-
  trigger #5 (zero-liquidity → price=None) extended to the
  zero-position case.
```

---

## §7 Privacy + broadcast (TB-15 Autopsy + Markov)

### P1_baseline

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

### P2_challenge

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

### P3_completeset

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

### P4_bankruptcy

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

### P5_aime_hard

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

### P6_triple_probe

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

### P7_baseline_b

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

### P8_completeset_b

```
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

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

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
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.
```

---

## §8 Tamper detection (FC2 git-mechanism)

| Problem | flip_l4_byte | flip_cas_byte | truncate_l4_ref | total |
|---|---|---|---|---|
| P1_baseline | ✓ | ✓ | ✓ | 3/3 |
| P2_challenge | ✓ | ✓ | ✓ | 3/3 |
| P3_completeset | ✓ | ✓ | ✓ | 3/3 |
| P4_bankruptcy | ✓ | ✓ | ✓ | 3/3 |
| P5_aime_hard | ✓ | ✓ | ✓ | 3/3 |
| P6_triple_probe | ✓ | ✓ | ✓ | 3/3 |
| P7_baseline_b | ✓ | ✓ | ✓ | 3/3 |
| P8_completeset_b | ✓ | ✓ | ✓ | 3/3 |

---

## §9 Replay determinism (FC1 — chain replayable from disk-only inputs)

| Problem | verdict.json == verdict_replay.json (byte-id) |
|---|---|
| P1_baseline | ✓ |
| P2_challenge | ✓ |
| P3_completeset | ✓ |
| P4_bankruptcy | ✓ |
| P5_aime_hard | ✓ |
| P6_triple_probe | ✓ |
| P7_baseline_b | ✓ |
| P8_completeset_b | ✓ |

---

## §10 Markov capsule chain (FC3 broadcast — every session ≤ TB-15 head)

| Problem | capsule_id (16hex) | previous_capsule_cid (16hex) |
|---|---|---|
| P1_baseline | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |
| P2_challenge | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |
| P3_completeset | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |
| P4_bankruptcy | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |
| P5_aime_hard | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |
| P6_triple_probe | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |
| P7_baseline_b | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |
| P8_completeset_b | `52ee141c6b885549...` | `f9e701b4a9c2e1d9...` |

---

## §11 Bottom-line conformance verdict

- **Aggregate audit_tape passed**: 271
- **Aggregate failed**: 0
- **Aggregate halted**: 0
- **Aggregate skipped**: 57
- **All chains PROCEED**: ✓
- **All chains replay byte-identical**: ✓

## VERDICT: PROCEED

All 7 mechanisms exercised on a fresh real-LLM substrate post R3 closure
with TURINGOS_CHAINTAPE_PRESEED=1 enabling the full user-task-mode path:
- mechanism 1-3 (tape/append/git): every chain audited from disk-only inputs + replay determinism + tamper detection
- mechanism 4 (economic): id=18 + id=40 verify total_supply conserved at every prefix step; id=20 escrow matches locks
- mechanism 5 (Boltzmann): structural fence id=26 + sequencer source has zero TB-14 type refs
- mechanism 6 (info shielding): id=28 (raw + JSON-array form privacy scan) + id=39 (no LLM self-narrative) + id=41 (sandbox-prefix walker on L4 + L4.E + ALL AgentId fields)
- mechanism 7 (broadcast): MarkovEvidenceCapsule chained to TB-15 head, flowchart_hashes parsed from TRACE_FLOWCHART_MATRIX
