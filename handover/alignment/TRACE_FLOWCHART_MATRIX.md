# Trace Flowchart Matrix (TB-C0, 2026-05-06)

**Purpose**: Per-node FC1 / FC2 / FC3 mapping → code surface → constitution gate test. Companion to `CONSTITUTION_EXECUTION_MATRIX.md` (which is gate-level summary). This file is the granular per-node binding.

**Lineage**:
- Raw flowchart node enumeration: `FC_ELEMENTS_2026-04-22.md` (134 elements: 48 nodes + 63 edges + 23 subgraphs)
- Existing symbol-level mapping: `TRACE_MATRIX_v0_2026-04-22.md` (43 rows; orphan/justification framing)
- Predecessor (TB-tracking variant, archived): `TRACE_FLOWCHART_MATRIX_v0_2026-05-02.md` — that file tracked TB → flowchart contact; this file binds FC nodes → tests
- This file ADDS the constitution-test binding (TB-C0 test name) that the prior matrices don't have

**Filter**: `cargo test --workspace constitution_` for all gate tests; `cargo test --workspace fc1_` / `fc2_` / `fc3_` per flowchart.

**Legend**:
- ✅ test exists and is GREEN
- 🟡 test exists, AMBER (smoke or full evidence pending)
- 🔴 test missing OR test is `assert!(true)` (must NOT remain RED on close)
- 🚫 N/A (constitution-document-level, not runtime)
- 📅 deferred (Phase 11+ JudgeAI/ArchitectAI runtime; explicit out-of-scope)

---

## §1. Four canonical flowchart hashes (carried forward from v0)

```text
Flowchart 1a — Runtime loop, page 8
  rtool / input / Agent δ / output / predicates ∏p / write tool path
  SHA256: a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5

Flowchart 1b — Runtime loop continuation, page 9
  predicates branch / write tool / Q_{t+1} / map-reduce tick
  SHA256: b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d

Flowchart 2 — Boot + full architecture, page 13
  Initialization (human → InitAI → predicates / Q0 / mr) + runtime loop + Finalization
  SHA256: 6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333

Flowchart 3 — Meta-architecture, page 17
  Constitution + logs archive (read-only) → JudgeAI / ArchitectAI →
  anti-oreo runtime (top / agents / tools) → log → archive → feedback → re-init
  SHA256: c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd
```

These hashes are immutable architectural contracts; if a flowchart changes, that is a Class 4 sudo event.

---

## §2. FC1 — Basic runtime cycle (per-node)

**Cycle**: `Q_t → rtool → input → AI(δ) → output → ∏p (predicates) → wtool → Q_{t+1}` (constitution lines 325-379, header `graph TD`).

| FC1 Node | Constitution label | Code surface | Constitution gate test | Status |
|---|---|---|---|---|
| FC1-N1 (q0) | `Q_t = ⟨q_t, HEAD_t, tape_t⟩` carrier | `TuringBus`, `Kernel::tape` | `fc2_genesis_report_exists` (Q_0 init) + `fc1_attempt_count_equals_tape_count` (Q_t advancement integrity) | 🟡 |
| FC1-N2 (q_t) | `q_t` slice | `QState`, `TuringBus::q_state` | covered by `tests/q_state_reconstruct.rs` (existing) | ✅ |
| FC1-N3 (HEAD_t) | `HEAD_t` head pointer | `time_arrow().last()` | covered by `tests/co1_7_extra_git2_writer_head_oid_defense.rs` (existing) | ✅ |
| FC1-N4 (q1) | `Q_{t+1}` after δ | `TuringBus::append_*` + sequencer accept | `fc1_predicate_pass_goes_l4` | ✅ |
| FC1-N5 (rtool) | read tool — ground-truth context fetch | `ReadTool::project`, `DefaultReadTool` | `fc3_raw_logs_not_in_agent_read_view` (shielding-side; constrains what rtool exposes) | 🟡 |
| FC1-N6 (input = ⟨q_i, s_i⟩) | input bundle to Agent | `UniverseSnapshot`, `build_agent_prompt` | covered by existing `tests/fc_alignment_conformance.rs::fc1_n6_*` | ✅ |
| FC1-N7 (δ / AI) | external Agent (LLM call) | `ResilientLLMClient::generate` | `fc1_every_externalized_attempt_is_tape_visible` (every δ invocation must produce tape WorkTx) | 🟡 |
| FC1-N8 (output = ⟨q_o, a_o⟩) | Agent output | `AgentOutput`, `parse_agent_output` | covered by existing fc_alignment_conformance | ✅ |
| FC1-N9 (q_o) | proposed q-delta | `AgentOutput::q_delta` | `fc1_attempt_count_equals_tape_count` (q_o reaches tape via WorkTx) | 🟡 |
| FC1-N10 (a_o) | action | `AgentOutput::action` | `fc1_predicate_pass_goes_l4` + `fc1_predicate_fail_goes_l4e` | ✅ |
| FC1-N11 (∏p predicates) | predicate composition | `TuringBus::evaluate_predicates`, `Predicate` trait | `predicate_result_is_binary` + `predicate_pass_required_for_l4` | ✅ |
| FC1-N12 (individual p) | individual predicates (Forbidden/Sorry/PayloadSize/Lean) | `{Forbidden,Sorry,PayloadSize}Predicate`, `Lean4Oracle::verify_*` | `lean_verified_required_for_verified_worktx` + `price_never_overrides_predicate` | ✅ |
| FC1-N13 (wtool) | write tool | `WriteTool::write`, `TuringBus::append_oracle_accepted` | `fc1_no_legacy_authoritative_append` (no direct bus.append bypass) | 🟡 |
| FC1-N14 (Q_{t+1} success) | accepted-branch advance | `append_internal`, `halt_with_reason` | `fc1_predicate_pass_goes_l4` | ✅ |
| FC1-N15 (Q_t reject branch) | rejection branch (∏p = 0) | `PartialVerdict::Reject`, `BusResult::Vetoed`, sequencer reject arm | `fc1_predicate_fail_goes_l4e` (rejection lands in L4.E, not L4) | ✅ |

### FC1 invariant battery (TB-C0 NEW gate tests)

| Invariant | Test | What it asserts |
|---|---|---|
| **FC1-INV1** every externalized attempt → tape | `fc1_every_externalized_attempt_is_tape_visible` | for any LLM-Lean cycle that yields q-delta or composite proof, a WorkTx OR rejection-WorkTx OR anchored EvidenceCapsule item exists with `attempt_chain_root` linkage |
| **FC1-INV2** predicate routing | `fc1_predicate_pass_goes_l4` + `fc1_predicate_fail_goes_l4e` | sequencer apply_one routes per verdict |
| **FC1-INV3** count equality | `fc1_attempt_count_equals_tape_count` | `evaluator_reported_tx_count == L4_WorkTx_attempt_count + L4E_WorkTx_rejection_count + explicitly_anchored_capsule_attempt_count` |
| **FC1-INV4** no legacy bypass | `fc1_no_legacy_authoritative_append` | in chaintape mode, `bus.append_*` direct path is fail-closed |
| **FC1-INV5** dashboard not source | `fc1_dashboard_not_source_of_truth` | dropping dashboard and replaying from L4 + CAS produces same chain_derived_run_facts |
| **FC1-INV6** no fake nodes | `fc1_no_fake_accepted_nodes` | tampering with WorkTx fields fails replay verify |

---

## §3. FC2 — Boot / init / halt / tick (per-node)

**Topology**: `human spec / constitution.md → predicates / tools (init) → Q_0 → runtime loop` (constitution lines 441-530, indented `flowchart TD` block).

| FC2 Node | Constitution label | Code surface | Constitution gate test | Status |
|---|---|---|---|---|
| FC2-N16 (InitAI) | bootstrap entity | `run_swarm`, `run_oneshot` | `fc2_genesis_report_exists` + `fc2_on_init_only_mint` | ✅ |
| FC2-N17 (human architect) | architect (manual) | `constitution.md` author | `fc3_architectai_proposal_not_direct_write` (FC3 binding) | 🚫 N/A runtime |
| FC2-N18 (law / ground truth) | constitution.md | `constitution.md` | `fc3_constitution_hash_pinned` (existing fc_alignment_conformance) | 🚫 N/A runtime |
| FC2-N19 (initAI →once predicates) | predicate registration at boot | `TuringBus::register_predicate` | `fc2_taskopen_escrowlock_are_chain_events` (boot-time admission gates) | ✅ |
| FC2-N20 (initAI →once mr) | mr-tick at boot | TICK_INTERVAL + emit_mr_tick_node | covered by `tests/six_axioms_alignment.rs::axiom_5` | ✅ |
| FC2-N21 (initAI →once Q0) | Q_0 minted | `Kernel::new`, `TuringBus::init` | `fc2_on_init_only_mint` + `fc2_no_post_init_mint` | ✅ |
| FC2-N22 (HALT) | halted state | `QState::Halted`, `halt_with_reason` | covered by `tests/six_axioms_alignment.rs::axiom_4` | ✅ |
| FC2-N23 (HaltReason variants) | terminal anchor distribution | `HaltReason` enum | covered by existing | ✅ |
| FC2-N24 (clock) | tick clock | `TuringBus::clock` | covered by `tests/six_axioms_alignment.rs::axiom_5` | ✅ |
| FC2-N25 (mr) | map-reduce tick | inline mr_summary, `emit_mr_tick_node` | covered by existing | ✅ |
| FC2-N26 (mr →map tape0) | mr reads tape | `tape.time_arrow().len()` | covered by existing | ✅ |
| FC2-N27 (mr →reduce tape1) | mr emits tape node | `emit_mr_tick_node` | covered by existing | ✅ |
| FC2-N28 (tools_other) | non-rtool/wtool tool surface | `WriteTool::write_with_tools`, `TuringBus::tools` | `fc2_taskopen_escrowlock_are_chain_events` | ✅ |

### FC2 invariant battery (TB-C0 NEW gate tests)

| Invariant | Test | What it asserts |
|---|---|---|
| **FC2-INV1** genesis exists | `fc2_genesis_report_exists` | `genesis_payload.toml` parses + trust root verifies |
| **FC2-INV2** init-only mint | `fc2_on_init_only_mint` + `fc2_no_post_init_mint` | total Coin supply set at on_init; never mints elsewhere |
| **FC2-INV3** no memory preseed | `fc2_no_memory_only_preseed` | code-grep: no `q.economic_state_t.insert(` outside on_init helpers |
| **FC2-INV4** chain events | `fc2_taskopen_escrowlock_are_chain_events` | TaskOpen/EscrowLock issued via Sequencer dispatch, not memory mutation |
| **FC2-INV5** replayable | `fc2_run_replayable_from_genesis_tape_cas` | tear down state, replay from genesis_report + L4 + CAS, recover identical chain_derived_run_facts |
| **FC2-INV6** pubkeys verify | `fc2_system_pubkeys_verify` | system tx signature verifies under genesis_payload.toml-pinned pubkey |
| **FC2-INV7** registry resolves | `fc2_agent_registry_resolves` | agent_pubkeys.json → AgentKeypairRegistry resolves correct pubkey |

---

## §4. FC3 — Meta / anti-oreo / system topology (per-node)

**Topology**: `boot → constitution / logs (read-only) → JudgeAI (veto) ← ArchitectAI (propose) → tools / logs / Q update` (constitution lines 670-714, indented `graph TB` block).

| FC3 Node | Constitution label | Code surface | Constitution gate test | Status |
|---|---|---|---|---|
| FC3-N29 (boot) | system boot | `async fn main`, `TuringBus::boot` | `fc2_genesis_report_exists` (FC2 anchor) | ✅ |
| FC3-N30 (constitution file) | constitution.md as ground truth | `constitution.md` | covered by `tests/fc_alignment_conformance.rs` | 🚫 N/A runtime |
| FC3-N31 (logs archive) | WAL + L4 archive | `Wal::write_event`, transition_ledger | `fc3_raw_logs_not_in_agent_read_view` | 🟡 |
| FC3-N32 (JudgeAI) | external veto agent (Codex/Gemini) | external | `fc3_judgeai_veto_only` | ✅ |
| FC3-N33 (ArchitectAI) | external propose agent | external (Claude code editing) | `fc3_architectai_proposal_not_direct_write` | 🟡 |
| FC3-N34 (readonly guard) | constitution + logs read-only | WAL append-only semantics | `fc3_no_automatic_predicate_mutation` | ✅ |
| FC3-N35 (anti-oreo top→agents→tools) | top-only does signal mgmt | `evaluate_predicates` flow | covered by existing fc_alignment_conformance | ✅ |
| FC3-N36 (agents) | swarm of N agents | `let agent_ids` round-robin | covered by existing | ✅ |
| FC3-N37 (tools) | bottom tools (rtool/wtool) | `TuringTool` trait | covered by existing | ✅ |
| FC3-N38 (Q update) | Q-delta application via wtool | sequencer dispatch | `fc1_predicate_pass_goes_l4` (FC1 anchor) | ✅ |
| FC3-N39 (markov / capsule) | Markov capsule (TB-15+) | `markov_capsule.rs` | `fc3_capsule_derived_from_tape_cas` + `fc3_no_global_markov_pointer` | 🟡 |
| FC3-N40 (override) | deep-history override | `TURINGOS_MARKOV_OVERRIDE=1` | `fc3_deep_history_requires_override` | ✅ |

### FC3 invariant battery (TB-C0 NEW gate tests)

| Invariant | Test | What it asserts |
|---|---|---|
| **FC3-INV1** capsule derived | `fc3_capsule_derived_from_tape_cas` | regenerating capsule from L4 + CAS gives identical CID |
| **FC3-INV2** no global pointer | `fc3_no_global_markov_pointer` (also in `no_parallel_ledger.rs`) | `LATEST_MARKOV_CAPSULE.txt` does NOT exist |
| **FC3-INV3** raw shielding | `fc3_raw_logs_not_in_agent_read_view` | UniverseSnapshot prompt contents do not contain raw Lean stderr |
| **FC3-INV4** capsule context-only | `fc3_latest_capsule_context_only` | capsule used as agent context, not as predicate-feeding ground-truth |
| **FC3-INV5** override required | `fc3_deep_history_requires_override` | reading deep-history without `TURINGOS_MARKOV_OVERRIDE=1` returns Default/empty |
| **FC3-INV6** no auto mutation | `fc3_no_automatic_predicate_mutation` | predicate set is registered at boot and not mutated thereafter |
| **FC3-INV7** architect propose-only | `fc3_architectai_proposal_not_direct_write` | architect role lands changes via charter/directive trail, not direct src/ commit |
| **FC3-INV8** judge veto-only | `fc3_judgeai_veto_only` | judge role does NOT commit code; verdict-only |

---

## §5. Cross-flowchart bindings (gate categories outside the 3 FCs)

| Gate category | Tests | FC anchor (which flowcharts these enforce) |
|---|---|---|
| Predicate gate (5 tests) | `predicate_*` | FC1-N11 + FC1-N12 |
| Shielding gate (5 tests) | `raw_*` / `l4e_*` / `private_*` / `evidence_*` / `dashboard_does_not_*` | FC1-N5 (rtool sees) + FC3-N31 (logs archive) |
| Economy gate (9 tests) | `economy_*` + `system_tx_not_agent_submittable` | Art. 0 Laws + FC2-N21 (Q_0 mint) + FC2-N28 (tools_other) |
| Tape canonical gate (7 tests) | `no_parallel_ledger` / `no_shadow_tape` / `canonical_txid` / `dashboard_regen` / `chain_derived_facts` / `all_*_have_cas_payload` | Art. 0.2 + FC1-INV4 + FC1-INV5 |
| No parallel ledger (dedicated) | `no_parallel_ledger.rs` battery | Art. 0.2 dedicated fence |

---

## §6. Test → MVP-gate cross-reference

| MVP closure gate | Bound tests |
|---|---|
| MVP-1 FC1 tx-count equality | `fc1_attempt_count_equals_tape_count` + `fc1_every_externalized_attempt_is_tape_visible` (+ P38/P49 evidence run) |
| MVP-2 Predicate routing | `predicate_failure_cannot_enter_l4` + `predicate_pass_required_for_l4` + `fc1_predicate_pass_goes_l4` + `fc1_predicate_fail_goes_l4e` |
| MVP-3 Dashboard regen | `dashboard_regenerates_from_tape_cas` + `fc1_dashboard_not_source_of_truth` |
| MVP-4 Replay | `fc2_run_replayable_from_genesis_tape_cas` + `fc2_genesis_report_exists` |
| MVP-5 Economy conservation | all 9 `economy_*` + `system_tx_not_agent_submittable` |

---

## §7. Test → 6-epistemic-questions cross-reference (closure condition #10)

The project must be able to answer:

| Question | Test that answers it |
|---|---|
| "What did the Agent externalize?" | `fc1_every_externalized_attempt_is_tape_visible` + `all_externalized_attempts_have_cas_payload` |
| "What passed predicates?" | `fc1_predicate_pass_goes_l4` + `predicate_pass_required_for_l4` |
| "What failed?" | `fc1_predicate_fail_goes_l4e` + `predicate_failure_cannot_enter_l4` |
| "What is on tape?" | `fc1_attempt_count_equals_tape_count` + `fc2_run_replayable_from_genesis_tape_cas` |
| "What is only in CAS?" | `evidence_capsule_raw_logs_audit_only` + `all_lean_results_have_cas_payload` |
| "What is only dashboard?" | `dashboard_regenerates_from_tape_cas` + `fc1_dashboard_not_source_of_truth` (i.e., dashboard is regenerable view, not authoritative) |

---

## §8. Update protocol (per CR-C0.7)

When an FC node's enforcement changes:
1. Update the row in this matrix.
2. Add the new test name to `CONSTITUTION_EXECUTION_MATRIX.md` row.
3. Update Status to ✅ only after `cargo test --workspace constitution_` confirms green.
4. Smoke-evidence column flips from 🟡 → ✅ only after the relevant TB ladder run produces the artifact.

When an FC node is renamed in constitution.md:
1. Phase Z′ 6-stage rerun applies (architect-only).
2. Update `FC_ELEMENTS_*.md` raw extract.
3. Update this matrix and `TRACE_MATRIX_v0_*.md`.
4. Bump matrix file version: `TRACE_FLOWCHART_MATRIX_vN.md`.
