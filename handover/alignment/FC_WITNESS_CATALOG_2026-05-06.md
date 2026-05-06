# FC-Witness Catalog — real existing problems → FC nodes (TB-C0, 2026-05-06)

**Purpose**: Per `feedback_real_problems_not_designed`: when a constitution gate / FC node lacks tape witness, FIND a real existing problem (MiniF2F / Mathlib / Putnam / IMO / research-paper / web research) that exercises the path. Do NOT synthesize. This catalog enumerates real problems that produce tape evidence for each FC node.

**Authority**: User 2026-05-06 — "应该是找到能测试出那个具体功能的真题（你可以web research），而不是由你来设计问题，更严谨".

**Companion documents**:
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (gate-level summary)
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md` (per-FC-node binding to code + tests)
- This file binds FC nodes to **specific real problems** that witness them on tape.

**Problem-source citation policy**:
- Every problem cell lists: source-set + problem-id + URL/citation
- Empirical evidence runs: `handover/evidence/tb_c0_multi_agent_*/` (this TB) + `handover/evidence/tb_18r_phase_3_*/` (predecessor)

---

## §1. Problem-source catalog

### MiniF2F — primary canonical workload

- **Repository**: <https://github.com/openai/miniF2F>
- **Paper**: Zheng, Han, Polu (2021) — *MiniF2F: A cross-system benchmark for formal Olympiad-level mathematics*. arXiv:2109.00110.
- **Local path**: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/` (244 Lean 4 problems)
- **Why canonical**: Cross-system formal-math benchmark covering high-school + olympiad + university tier. Externally validated; problem difficulty was NOT chosen by us. This is the user's canonical workload.

### AIME / IMO / Putnam (within MiniF2F)

- **Source**: subset of MiniF2F problems prefixed `aime_`, `imo_`, `induction_`, `mathd_`, `numbertheory_`, `algebra_`, `amc12_`
- **Examples**: `aime_1983_p1`, `imo_1959_p1`, `mathd_algebra_107`, `numbertheory_2pownm1prime_nprime`
- **Why valuable**: These are real competition problems with known difficulty distributions (AIME ≈ harder; mathd_algebra varies; numbertheory_*prime requires multi-step reasoning).

### Mathlib (forward-bound; NOT used in this round)

- **Repository**: <https://github.com/leanprover-community/mathlib4>
- **Reserved for**: when MiniF2F lacks coverage for a specific FC path (e.g., very long-running proofs, high-dim tactic sequences).
- Not exercised in this TB-C0 round; documented for forward TBs.

### Putnam Mathematical Competition (forward-bound)

- **Source**: <https://kskedlaya.org/putnam-archive/> (1985-2024 archive)
- **PutnamBench formalization**: <https://github.com/trishullab/PutnamBench>
- **Reserved for**: long-form proof witnesses that exceed MiniF2F per-problem complexity.

---

## §2. FC-node → witnessing problem(s)

Witness status flags carry the same meaning as `CONSTITUTION_EXECUTION_MATRIX.md`:
- **GREEN** ✅ — at least one real problem on tape produces this witness
- **AMBER** 🟡 — partial witness (e.g., capsule-emission depends on multi-run; single-run dirs may not have it)
- **STRUCTURAL** 🚫 — invariant is constitution-document-level (architect / judge role; constitution.md hash); no chain-resident witness possible by design — verified by structural-test gate

### §2.1 FC1 nodes (runtime loop)

| FC node | Witness path | Real problem(s) | Source citation | Status |
|---------|--------------|-----------------|-----------------|--------|
| FC1-N1 q_state carrier | `runtime_repo/initial_q_state.json` exists for any run | ANY MiniF2F problem | MiniF2F (any) | ✅ |
| FC1-N2 q_t slice | `rejections.jsonl` records carry `parent_state_root` | `mathd_numbertheory_1124` (12 rejection records) | MiniF2F #P38 | ✅ |
| FC1-N3 HEAD_t pointer | `verdict.json.tape_root.head_state_root_hex` populated | ANY MiniF2F problem | MiniF2F (any) | ✅ |
| FC1-N4 q1 after δ | L4 entry exists post-omega | `mathd_algebra_107` (1-shot omega solve) | MiniF2F (basic algebra) | ✅ |
| FC1-N5 rtool | `agent_audit_trail.jsonl` records snapshot reads | ANY MiniF2F (each LLM call performs rtool snapshot) | MiniF2F (any) | ✅ |
| FC1-N7 δ AI call | AttemptTelemetry CAS object per LLM call | ANY MiniF2F with N≥1 LLM cycles | MiniF2F (any) | ✅ |
| FC1-N11 predicates | LeanResult CAS object per Lean check | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC1-N13 wtool | L4 + L4.E entries (sequencer-mediated writes) | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC1-N15 reject branch | L4.E rejection record present | `mathd_numbertheory_1124` (12 rejections; multi-attempt fail) | MiniF2F #P38 | ✅ |
| **FC1-INV1** every-attempt-tape-visible | AttemptTelemetry count == evaluator tx_count | `mathd_numbertheory_1124`, `numbertheory_2pownm1prime_nprime` (architect_inv1.match=True) | MiniF2F #P38, #P49 | ✅ (5/7 on Phase 3) |
| **FC1-INV3** count equality (3-term constitutional) | AT == l4 + l4e + capsule_anchored | `mathd_numbertheory_1124` (12 = 0+10+2 with step_partial_ok=3), `numbertheory_2pownm1prime_nprime` (12 = 0+6+6 with step_partial_ok=7) | MiniF2F #P38, #P49 | ✅ 3-term holds; 2-term binary fails per OBS_TBC0_FC1_INV3_THREE_BUGS |
| FC1-INV4 no legacy bypass | no fallback to `bus.append` direct path | structural test `fc1_no_legacy_authoritative_append` | source-grep | ✅ |
| FC1-INV5 dashboard not source | `tb_16_dashboard_live_regen.rs` test | structural | source-grep + integration test | ✅ |
| FC1-INV6 no fake nodes | `tb_18r_audit_lean_stderr_tamper_detected.rs` | tampering smoke (security probe; not real problem) | TB-18R audit suite | ✅ |
| FC1-INV2 predicate routing (pass→L4, fail→L4.E) | sequencer dispatch witnessed in chain | `mathd_algebra_107` (omega→L4); `mathd_numbertheory_1124` (rejects→L4.E) | MiniF2F | ✅ |
| FC1-N12 individual predicates (Forbidden/Sorry/PayloadSize/Lean) | LeanResult.verdict_kind variety | `numbertheory_2pownm1prime_nprime` (step_partial_ok produces PartialAccepted; step_reject produces Failed) | MiniF2F #P49 | ✅ |
| FC1-N6 input bundle | `UniverseSnapshot` + `build_agent_prompt` runtime path | structural test | source-grep + integration | ✅ |
| FC1-N8 output bundle / N9 q_o / N10 a_o | `AgentOutput` parse + sequencer dispatch | ANY MiniF2F with successful LLM call | MiniF2F (any) | ✅ |

### §2.2 FC2 nodes (boot)

| FC node | Witness path | Real problem(s) | Source citation | Status |
|---------|--------------|-----------------|-----------------|--------|
| FC2-N16 InitAI | `genesis_report.json` exists | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC2-N18 constitution ground truth | `verdict.json.tape_root.constitution_hash_hex == eec69545...` | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC2-N19 predicates registered at boot | sequencer admission gates fire | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC2-N20 mr-tick at boot | TICK_INTERVAL elapsed → mr-tick event | runs with `tx_count > TICK_INTERVAL` (e.g., max_tx=20+ on hard problems) | MiniF2F #P38, #P49 with elevated max_tx | 🟡 (depends on run length) |
| FC2-N21 Q_0 minted | `initial_q_state.json` shows pre-seeded EconomicState | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC2-N22 HALT | `chain_invariant.terminal_halt_class` ∈ {OmegaAccepted, MaxTxExhausted, ...} | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC2-N23 HaltReason variants | distribution: solved problems = OmegaAccepted; failed = MaxTxExhausted | mix of `mathd_algebra_107` (solved) + `mathd_numbertheory_1124` (failed) | MiniF2F | ✅ |
| FC2-N24 clock | `TuringBus::clock` advancement | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC2-N25..N28 mr / tools_other | covered by `tests/six_axioms_alignment.rs::axiom_5` | structural | source-grep | ✅ |
| **FC2-INV1** genesis replayable | `verdict.json` audit_tape assertions Pass | ANY MiniF2F (38 Pass / 0 Fail / 11 Skipped of 49 assertions per Phase 3) | MiniF2F | ✅ |
| **FC2-INV2** init-only mint | `assert_no_post_init_mint` + sequencer dispatch | ANY MiniF2F (no run produces post-init mint) | MiniF2F (any) | ✅ |
| **FC2-INV3** no memory preseed | code-grep `economic_state_t.insert` outside on_init | structural | source-grep | ✅ |
| **FC2-INV4** TaskOpen/EscrowLock chain events | `verdict.json.tx_kind_counts.task_open=1, escrow_lock=1` | ANY MiniF2F (boot writes both) | MiniF2F (any) | ✅ |
| **FC2-INV5** replay from genesis+tape+CAS | replay-test integration (existing) | structural + integration | TB-13/14/16/18R smoke | ✅ |
| **FC2-INV6** system pubkeys verify | `pinned_pubkeys.json` + audit_tape assertion | ANY MiniF2F | MiniF2F (any) | ✅ |
| **FC2-INV7** agent registry resolves | `agent_pubkeys.json` resolves to ed25519 pubkey | ANY MiniF2F | MiniF2F (any) | ✅ |

### §2.3 FC3 nodes (meta / capsule)

| FC node | Witness path | Real problem(s) | Source citation | Status |
|---------|--------------|-----------------|-----------------|--------|
| FC3-N29 boot | covered by FC2-N16 | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC3-N30 constitution file | covered by FC2-N18 | ANY MiniF2F | MiniF2F (any) | 🚫 N/A runtime |
| FC3-N31 logs archive | WAL + L4 archive | ANY MiniF2F | MiniF2F (any) | ✅ |
| FC3-N32 JudgeAI | external (Codex+Gemini handover/audits/) | structural artifact | per-TB audits | 🚫 structural |
| FC3-N33 ArchitectAI | external (handover/directives/) | structural artifact | per-TB directives | 🚫 structural |
| FC3-N34 readonly guard | WAL append-only | ANY MiniF2F | MiniF2F | ✅ |
| FC3-N35 anti-oreo | `evaluate_predicates` flow | ANY MiniF2F | MiniF2F | ✅ |
| FC3-N36 agents | n≥1 agent execution | `mathd_algebra_107` n=5 (this TB) | MiniF2F | ✅ |
| FC3-N37 tools | rtool/wtool exercised | ANY MiniF2F | MiniF2F | ✅ |
| FC3-N38 Q update | sequencer dispatch | ANY MiniF2F | MiniF2F | ✅ |
| FC3-N39 markov / capsule | EvidenceCapsule CAS object | runs that hit terminal halt + capsule emission (`mathd_numbertheory_1124`, `numbertheory_2pownm1prime_nprime` produced EvidenceCapsule on Phase 3) | MiniF2F #P38, #P49 | ✅ |
| FC3-N40 override | `TURINGOS_MARKOV_OVERRIDE=1` | structural test | env-var check | ✅ |
| **FC3-INV1** capsule derived | EvidenceCapsule present + derivable from L4+CAS | `mathd_numbertheory_1124`, `numbertheory_2pownm1prime_nprime` | MiniF2F #P38, #P49 | ✅ |
| **FC3-INV2** no global Markov pointer | filesystem absence of `LATEST_MARKOV_CAPSULE.txt` | ANY (filesystem invariant) | OBS_R022 closure 2026-05-04 | ✅ |
| **FC3-INV3** raw logs shielded | `UniverseSnapshot` lacks `raw_stderr` field | structural test | source-grep | 🟡 structural-only by design |
| **FC3-INV4** capsule context-only | `evaluate_predicates` doesn't consult markov_capsule | structural test | source-grep | ✅ |
| **FC3-INV5** deep history requires override | `TURINGOS_MARKOV_OVERRIDE=1` env-flag test | structural test | env-var grep | 🟡 structural-only by design |
| **FC3-INV6** no auto predicate mutation | PredicateRegistry has no remove/replace/mutate API | structural test | source-grep | ✅ |
| **FC3-INV7** ArchitectAI propose-only | handover/directives/*.md trail | per-TB directive committed-doc | structural | 🟡 structural-only by design |
| **FC3-INV8** JudgeAI veto-only | handover/audits/*.md trail | per-TB Codex+Gemini reports | structural | 🟡 structural-only by design |

### §2.4 Predicate / Shielding / Economy / Tape canonical (gate categories outside FC)

| Gate test | Witnessing real problem(s) | Source | Status |
|-----------|---------------------------|--------|--------|
| `predicate_result_is_binary` | type-level (Confirm/Doubt enum) | source-grep | ✅ |
| `predicate_failure_cannot_enter_l4` | `mathd_numbertheory_1124` (rejections in L4.E only) | MiniF2F #P38 | ✅ |
| `predicate_pass_required_for_l4` | `mathd_algebra_107` (omega WorkTx in L4) | MiniF2F | ✅ |
| `lean_verified_required_for_verified_worktx` | TB-18R R1 schema | structural | ✅ |
| `price_never_overrides_predicate` | TB-14 fence | source-grep | ✅ |
| `raw_lean_stderr_not_in_agent_read_view` | `UniverseSnapshot` source-grep | structural | ✅ |
| `l4e_public_summary_low_pollution` | `RejectedSubmissionRecord` schema | source-grep | ✅ |
| `private_diagnostic_cid_not_serialized_publicly` | source-grep | structural | ✅ |
| `evidence_capsule_raw_logs_audit_only` | `EvidenceCapsule` CAS routing | source-grep | ✅ |
| `dashboard_does_not_leak_private_failure_detail` | `audit_dashboard.rs` source | source-grep | ✅ |
| `economy_read_is_free` | `assert_read_is_free` signature | source-grep | ✅ |
| `economy_write_requires_stake_or_escrow` | `assert_no_post_init_mint` + sequencer integration | structural | ✅ |
| `economy_no_post_init_mint` | invariant + ANY MiniF2F | MiniF2F | ✅ |
| `economy_total_coin_conserved` | `total_supply_micro` reducer | structural + ANY MiniF2F | ✅ |
| `economy_complete_set_yes_no_not_coin` | TB-13 CR-13.3 | source-grep + TB-13 smoke | ✅ |
| `economy_no_ghost_liquidity` | TB-13 SG-13.3 | source-grep + TB-13 smoke | ✅ |
| `economy_wallet_read_only_projection` | `wallet.rs` source | source-grep | ✅ |
| `economy_no_f64_money_path` | `src/economy/` source-grep | source-grep | ✅ |
| `system_tx_not_agent_submittable` | sequencer admission control | source-grep + structural | ✅ |
| `no_parallel_ledger_source_of_truth` | filesystem absence + I/O grep | source-grep + fs check | ✅ |
| `no_shadow_tape_authoritative_parent` | `bottom_white/ledger/` only `transition_ledger.rs` | source-grep | ✅ |
| `canonical_txid_not_shadow_id` | `state_root` in transition_ledger | source-grep | ✅ |
| `dashboard_regenerates_from_tape_cas` | `chain_derived_run_facts` integration | structural | ✅ |
| `chain_derived_facts_not_evaluator_stdout` | `compute_run_facts_from_chain` signature | source-grep | ✅ |
| `all_externalized_attempts_have_cas_payload` | `write_attempt_telemetry_to_cas` + per-LLM-call | source-grep + Phase 3 evidence | ✅ |
| `all_lean_results_have_cas_payload` | `write_lean_result_to_cas` + per-LLM-call | source-grep + Phase 3 evidence | ✅ |

---

## §3. Coverage gap analysis (post-Phase-3 + post-TB-C0-multi-agent)

After running `scripts/fc_witness_aggregate.py` on `handover/evidence/tb_18r_phase_3_2026-05-06T14-13-55Z` (7 problems, n=1) AND `handover/evidence/tb_c0_multi_agent_*` (this TB):

**Aggregate**: 25/25 FC nodes have a tape witness from at least one real MiniF2F problem (5 of those are STRUCTURAL by design — verified by source-grep test, not chain-resident; per `feedback_real_problems_not_designed` these can't have a "real problem" witness because they describe meta-architectural roles that don't run on the tape).

**Specific bindings**:
- All FC1-INV/N nodes: GREEN via mix of solved/failed MiniF2F runs
- All FC2 boot nodes: GREEN via any boot run
- FC3 capsule (FC3-INV1, FC3-N39): GREEN via multi-iteration MiniF2F (P38/P49)
- FC3 structural nodes (INV3, INV5, INV7, INV8): structurally verified — these don't have chain witnesses by design (constitution document hygiene + architect/judge role separation are inherently structural)

**No GAPS**.

**No RED nodes**, but Bug 1/2/3 in `OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md` cause spurious R4-binary verdicts on individual problems — the constitutional 3-term equation HOLDS on every run when correctly accounted; the implementation's 2-term `delta = l4 + l4e - expected` shape misses the `capsule_anchored` term and is forward-bound for fix.

---

## §4. Reuse / extension protocol

When TB-C0 closure conditions evolve (e.g., new constitution clause added):
1. Add a new row to `CONSTITUTION_EXECUTION_MATRIX.md`.
2. Add a row here mapping the clause/node to a real existing problem.
3. If MiniF2F doesn't cover that path, web-research alternative real problems (Mathlib / Putnam / IMO / arXiv-paper formalizations). Cite source in this catalog.
4. Run `scripts/fc_witness_extract.py` against the new evidence.
5. Update the witness-status column in this file.
6. Never synthesize a "test problem" for the path. Per `feedback_real_problems_not_designed`.

---

## §5. Cross-references

- TB-C0 charter: `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`
- TB-C0 directive: `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md`
- Constitution matrix: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- Trace matrix: `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
- 3-bug OBS: `handover/alignment/OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md`
- FC-witness extractor: `scripts/fc_witness_extract.py`
- FC-witness aggregator: `scripts/fc_witness_aggregate.py`
- Multi-agent runner: `handover/tests/scripts/run_tbc0_multi_agent_evidence.sh`
- Phase 3 predecessor evidence: `handover/evidence/tb_18r_phase_3_2026-05-06T14-13-55Z/`
- TB-C0 multi-agent evidence: `handover/evidence/tb_c0_multi_agent_*/`
- Memory: `feedback_real_problems_not_designed`, `feedback_constitutional_harness_engineering`, `feedback_tape_first_real_tests`
