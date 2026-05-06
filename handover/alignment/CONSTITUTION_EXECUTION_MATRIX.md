# Constitution Execution Matrix (TB-C0, 2026-05-06)

**Purpose**: Turn `constitution.md` from natural-language law into repo-side executable CI. Every row binds a constitution clause / FC node / engineering invariant to (a) a code surface that should enforce it, (b) a test that asserts it, (c) the smoke-evidence path that demonstrates it under real load, (d) current status, (e) a kill condition that flips the row RED.

**Authority**: TB-C0 charter `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`; directive `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md`.

**Companion document**: `TRACE_FLOWCHART_MATRIX.md` (per-node FC1/FC2/FC3 mapping; this matrix is the gate-level summary).

**Legend**:
- 🟢 GREEN — test exists, asserts the real invariant, passes on `cargo test --workspace`
- 🟡 AMBER — test exists, structural-only or limited coverage; smoke evidence pending or partial
- 🔴 RED — no test, OR test only `assert!(true)` / docs-only — does NOT count per CR-C0.1
- 🚫 N/A — clause is intentionally non-runtime (e.g., constitution.md hygiene; architect-only authority)

**Status discipline (CR-C0.7)**:
- A row goes RED whenever its only "evidence" is a doc-comment OR a passing audit.
- AMBER means test exists but doesn't yet exercise the real path under load.
- GREEN means test exercises the real path AND passes.

**Filter**: `cargo test --workspace constitution_` (broad) or per-prefix (`fc1_`, `fc2_`, `fc3_`, `predicate_`, `shielding_`, `economy_`, `tape_`, `system_`, `no_`).

---

## §A. Article 0 — Turing-machine foundationalism

| Clause | Code surface | Test name | Smoke evidence | Status | Kill condition |
|--------|--------------|-----------|----------------|--------|---------------|
| Art. 0.1 four-element mapping (tape / pencil / eraser / discipline) | `src/ledger.rs` (Tape) + `src/sdk/write_tool.rs` (pencil) + `src/state/sequencer.rs` accept/reject (eraser) + `src/sdk/predicate.rs` (discipline) | `tests/four_element_mapping.rs` (existing) | TB-13/14/15/16/17/18R chain smoke | 🟡 AMBER | any element absent from runtime chain |
| Art. 0.2 Tape Canonical (single source of truth) | `src/ledger.rs` (`Tape`) + `src/bottom_white/ledger/transition_ledger.rs` (`L4` chain) + `src/bottom_white/cas/` (CAS) | `tests/constitution_no_parallel_ledger.rs::no_parallel_ledger_source_of_truth` (NEW C0) + `tests/constitution_no_parallel_ledger.rs::no_global_markov_pointer` (NEW C0) + `tests/markov_pointer_de_canonicalize.rs` (existing) | TB-16 chain smoke + TB-C0 fs-check | 🟡 AMBER | `LATEST_MARKOV_CAPSULE.txt` reappears OR shadow-tape canonical claim |
| Art. 0.3 blockchain preservation (immutable append-only) | `src/wal.rs` (WAL append-only) + `src/bottom_white/ledger/transition_ledger.rs` | `tests/constitution_fc1_runtime_loop.rs::fc1_no_legacy_authoritative_append` (NEW C0) | TB-13/14 chain smoke replay | 🟡 AMBER | `bus.append` direct write replaces sequencer-mediated write |
| Art. 0.4 Q_t version-controlled | `src/state/q_state.rs` + `src/bus.rs` (`TuringBus::q_state`) | `tests/q_state_reconstruct.rs` (existing) + `tests/six_axioms_alignment.rs` (existing) | TB-17 reconstruct smoke | 🟢 GREEN | Q_t mutation outside sequencer dispatch |
| Art. 0 Laws (基本法) | spread across sequencer + economy + ledger | `tests/constitution_economy_gate.rs::economy_total_coin_conserved` + `economy_no_post_init_mint` + `system_tx_not_agent_submittable` (NEW C0) | TB-13/14 economic invariant smoke | 🟡 AMBER | any conservation invariant violated by new tx kind |

## §B. Article I — Signal quantification (predicates)

| Clause | Code surface | Test name | Smoke evidence | Status | Kill condition |
|--------|--------------|-----------|----------------|--------|---------------|
| Art. I.1 Boolean signal (predicate result is binary) | `src/sdk/predicate.rs` (`Predicate` trait) + `src/runtime/verify.rs` | `tests/constitution_predicate_gate.rs::predicate_result_is_binary` (NEW C0) | TB-13/14 verify smoke | 🟢 GREEN | predicate returns non-binary `Verdict` shape |
| Art. I.1 — predicate failure → L4.E | `src/state/sequencer.rs` (rejection arm) + `src/bottom_white/ledger/transition_ledger.rs::EventType::*::Rejected` | `tests/constitution_predicate_gate.rs::predicate_failure_cannot_enter_l4` (NEW C0) + `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs` (existing) | TB-18R R3 substrate smoke | 🟢 GREEN | rejected WorkTx lands in L4 accepted ledger |
| Art. I.1 — predicate pass required for L4 | `src/state/sequencer.rs::apply_one` admission gates | `tests/constitution_predicate_gate.rs::predicate_pass_required_for_l4` (NEW C0) | TB-13/14/18R substrate smoke | 🟢 GREEN | un-verified WorkTx lands in L4 accepted |
| Art. I.1 — Lean verified required for verified WorkTx | `src/runtime/verify.rs::verify_work_tx_lean` + `src/runtime/attempt_telemetry.rs::LeanVerdictKind` | `tests/constitution_predicate_gate.rs::lean_verified_required_for_verified_worktx` (NEW C0) | TB-18R R1+R2 smoke | 🟢 GREEN | WorkTx with `verified=true` admits without Lean pass |
| Art. I.1.1 PCP / 疑罪从无 (innocent-until-proven) | `src/state/sequencer.rs::admit_work_tx` default path | `tests/constitution_predicate_gate.rs::price_never_overrides_predicate` (NEW C0) | TB-14 price smoke | 🟢 GREEN | price/index signal flips predicate verdict |
| Art. I.2 Statistical signal (PPUT / reputation / consensus) | `src/runtime/evaluator.rs` ΣPPUT computation + `src/economy/reputation.rs` | `tests/economic_state_reconstruct.rs` (existing) + report-side discipline (CLAUDE.md Report Standard) | TB-17/18 ladder PPUT smoke | 🟡 AMBER | report missing ΣPPUT + Mean-PPUT(solved) + Wilson 95% CI |

## §C. Article II — Selective broadcast

| Clause | Code surface | Test name | Smoke evidence | Status | Kill condition |
|--------|--------------|-----------|----------------|--------|---------------|
| Art. II.1 broadcast typical errors (NO raw stderr to all agents) | `src/sdk/snapshot.rs` (`UniverseSnapshot`) + `src/sdk/prompt.rs` agent-context builder | `tests/constitution_shielding_gate.rs::raw_lean_stderr_not_in_agent_read_view` (NEW C0) | TB-7R Art. III.4 smoke | 🟡 AMBER | raw Lean stderr appears in agent prompt |
| Art. II.2 broadcast price signal | `src/economy/price_index.rs` (TB-14) | `tests/tb_14_price_index.rs` (existing) + `tests/constitution_predicate_gate.rs::price_never_overrides_predicate` | TB-14 price smoke | 🟢 GREEN | price modulates predicate truth value |
| Art. II.2.1 exploration / exploitation balance | `src/runtime/evaluator.rs` parent-selection entropy + payload diversity | `tests/six_axioms_alignment.rs::axiom_2_payload_diversity` (existing) | TB-17 ladder entropy smoke | 🟡 AMBER | `parent_selection_entropy < 0.25` OR `pairwise_payload_diversity_mean < 0.25` (per CLAUDE.md Report Standard) |

## §D. Article III — Selective shielding

| Clause | Code surface | Test name | Smoke evidence | Status | Kill condition |
|--------|--------------|-----------|----------------|--------|---------------|
| Art. III.1 shield errors (raw failure logs not in agent prompt) | `src/sdk/snapshot.rs` + `src/runtime/attempt_telemetry.rs` (private CID) | `tests/constitution_shielding_gate.rs::private_diagnostic_cid_not_serialized_publicly` (NEW C0) + `raw_lean_stderr_not_in_agent_read_view` | TB-18R R2 smoke | 🟡 AMBER | private CID appears in public broadcast |
| Art. III.2 encapsulation (high-volume detail in CAS, audit-only) | `src/bottom_white/cas/schema.rs` (`AttemptTelemetry` / `LeanResult`) | `tests/constitution_shielding_gate.rs::evidence_capsule_raw_logs_audit_only` (NEW C0) + `tests/tb_18r_audit_sampler_attempt_payload.rs` (existing) | TB-18R R5 audit smoke | 🟡 AMBER | raw logs become broadcast input |
| Art. III.3 shield correlation (no Goodhart leakage) | `src/economy/reputation.rs` reputation projection | `tests/constitution_shielding_gate.rs::dashboard_does_not_leak_private_failure_detail` (NEW C0) | TB-15/16 capsule smoke | 🟡 AMBER | capsule exposes per-agent private diagnostic to others |
| Art. III.4 shield Goodhart | `src/runtime/evaluator.rs` selector blindness | `tests/constitution_shielding_gate.rs::l4e_public_summary_low_pollution` (NEW C0) | TB-7R Art. III.4 smoke | 🟡 AMBER | selector reads Lean stderr text body |

## §E. Article IV — Boot (init / halt / tick)

| Clause | Code surface | Test name | Smoke evidence | Status | Kill condition |
|--------|--------------|-----------|----------------|--------|---------------|
| Art. IV.boot — Q_0 generated by InitAI exactly once | `src/runtime/evaluator.rs::run_swarm` + `src/state/sequencer.rs::genesis` | `tests/constitution_fc2_boot.rs::fc2_genesis_report_exists` (NEW C0) + `fc2_on_init_only_mint` | TB-17 boot smoke | 🟡 AMBER | mint occurs after on_init |
| Art. IV.halt — HaltReason terminal anchor | `src/ledger.rs::HaltReason` + `src/runtime/evaluator.rs::extract_halt_reason` | existing `tests/six_axioms_alignment.rs` axiom-4 + `halt_reason_distribution` discipline | TB-17 halt smoke | 🟢 GREEN | halt without HaltReason variant emission |
| Art. IV.tick — clock advance | `src/bus.rs::clock` + `src/runtime/evaluator.rs::TICK_INTERVAL` | `tests/six_axioms_alignment.rs` axiom-5 (existing) | TB-17 tick smoke | 🟢 GREEN | clock advances without tick emission |
| Art. IV — fresh replay from genesis + tape + CAS | `src/boot/genesis_payload.rs` + `src/bottom_white/ledger/transition_ledger.rs` replay | `tests/constitution_fc2_boot.rs::fc2_run_replayable_from_genesis_tape_cas` (NEW C0) + existing `tb_18r_chain_attempt_invariant.rs` replay | TB-13/14/16/18R replay smoke | 🟡 AMBER | replay diverges from original run |
| Art. IV — system pubkeys verify | `src/state/system_keypair.rs` + `tests/system_keypair_*.rs` (5 existing) | `tests/constitution_fc2_boot.rs::fc2_system_pubkeys_verify` (NEW C0) | TB-17 keypair smoke | 🟢 GREEN | system tx verifies under wrong pubkey |
| Art. IV — agent registry resolves | `src/runtime/agent_registry.rs` | `tests/constitution_fc2_boot.rs::fc2_agent_registry_resolves` (NEW C0) | TB-13 registry smoke | 🟢 GREEN | agent registry returns wrong pubkey |
| Art. IV — TaskOpen / EscrowLock are chain events | `src/state/typed_tx.rs::TaskOpenTx` / `EscrowLockTx` | `tests/constitution_fc2_boot.rs::fc2_taskopen_escrowlock_are_chain_events` (NEW C0) | TB-13 task-open smoke | 🟢 GREEN | TaskOpen issued via memory-only mutation |
| Art. IV — no memory-only preseed | `src/state/q_state.rs` `EconomicState` mutation surfaces | `tests/constitution_fc2_boot.rs::fc2_no_memory_only_preseed` (NEW C0) | code-grep + replay smoke | 🟡 AMBER | `q.economic_state_t.insert` outside on_init |

## §F. Article V — Meta (separation of powers)

| Clause | Code surface | Test name | Smoke evidence | Status | Kill condition |
|--------|--------------|-----------|----------------|--------|---------------|
| Art. V.1.1 constitution as single ground truth | `constitution.md` + `tests/fc_alignment_conformance.rs` (existing) | existing `tests/fc_alignment_conformance.rs` battery | per-PR FC alignment | 🟢 GREEN | FC element renamed/removed without TRACE_MATRIX update |
| Art. V.1.2 ArchitectAI proposes (NOT direct write) | external (architect handover/directives/) | `tests/constitution_fc3_meta.rs::fc3_architectai_proposal_not_direct_write` (NEW C0) | per-directive archive | 🟡 AMBER | architect directly writes to src/ without TB charter |
| Art. V.1.3 Veto-AI / JudgeAI veto-only | external (Codex + Gemini dual audit) | `tests/constitution_fc3_meta.rs::fc3_judgeai_veto_only` (NEW C0) | TB-13/14/17 dual audit dispatches | 🟢 GREEN | judge agent commits code |
| Art. V.2 constitution boundaries | `tests/fc_alignment_conformance.rs::fc3_constitution_hash_pinned` (if exists) | existing fc_alignment_conformance | per-PR | 🟡 AMBER | constitution.md hash drift without architect signature |
| Art. V.3 amendment log | `constitution.md` §5.3 + `cases/C-*.yaml` | docs-only (per legend 🚫 N/A for runtime) | n/a | 🚫 N/A | — |

## §G. Flowchart 1 (FC1) Runtime Loop — gate-level summary

(Detailed per-node breakdown in `TRACE_FLOWCHART_MATRIX.md`.)

| FC1 invariant | Code surface | Test name | Status | Kill condition |
|---|---|---|---|---|
| Every externalized attempt is tape-visible | `src/runtime/evaluator.rs` 6 paths + `src/runtime/attempt_telemetry.rs::r2_write_attempt_telemetry` | `tests/constitution_fc1_runtime_loop.rs::fc1_every_externalized_attempt_is_tape_visible` (NEW C0) | 🟡 AMBER (smoke MVP-1 pending) | `evaluator_reported_tx_count != chain_attempt_count` |
| Predicate pass → L4 | sequencer accept arm | `tests/constitution_fc1_runtime_loop.rs::fc1_predicate_pass_goes_l4` | 🟢 GREEN | accepted WorkTx not in L4 |
| Predicate fail → L4.E | sequencer reject arm + `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs` | `tests/constitution_fc1_runtime_loop.rs::fc1_predicate_fail_goes_l4e` | 🟢 GREEN | rejected WorkTx in L4 |
| No legacy authoritative append | `src/bus.rs` (legacy `bus.append` legacy mode) | `tests/constitution_fc1_runtime_loop.rs::fc1_no_legacy_authoritative_append` | 🟡 AMBER | chaintape mode falls back silently |
| Dashboard not source of truth | `src/runtime/dashboard.rs` (if exists) | `tests/constitution_fc1_runtime_loop.rs::fc1_dashboard_not_source_of_truth` | 🟡 AMBER | dashboard regenerated and replay diverges |
| Attempt count = tape count | TB-18R R4 invariant | `tests/constitution_fc1_runtime_loop.rs::fc1_attempt_count_equals_tape_count` + existing `tb_18r_chain_attempt_invariant.rs` | 🟡 AMBER (MVP-1 smoke pending) | per kill |
| No fake accepted nodes | `src/runtime/audit_tape.rs` | `tests/constitution_fc1_runtime_loop.rs::fc1_no_fake_accepted_nodes` | 🟢 GREEN | tampered node passes verify |

## §H. Flowchart 2 (FC2) Boot — gate-level summary

| FC2 invariant | Test name | Status | Kill condition |
|---|---|---|---|
| genesis_report exists | `fc2_genesis_report_exists` | 🟢 GREEN | absent or malformed |
| on_init only mint | `fc2_on_init_only_mint` | 🟢 GREEN | mint after init |
| no post-init mint | `fc2_no_post_init_mint` | 🟢 GREEN | post-init mint |
| no memory-only preseed | `fc2_no_memory_only_preseed` | 🟡 AMBER (code-grep) | preseed surface found |
| TaskOpen / EscrowLock are chain events | `fc2_taskopen_escrowlock_are_chain_events` | 🟢 GREEN | issued in memory only |
| run replayable | `fc2_run_replayable_from_genesis_tape_cas` | 🟡 AMBER (MVP-4 smoke pending) | replay diverges |
| system pubkeys verify | `fc2_system_pubkeys_verify` | 🟢 GREEN | wrong-pubkey verify passes |
| agent registry resolves | `fc2_agent_registry_resolves` | 🟢 GREEN | wrong-pubkey resolution |

## §I. Flowchart 3 (FC3) Meta — gate-level summary

| FC3 invariant | Test name | Status | Kill condition |
|---|---|---|---|
| Capsule derived from tape + CAS | `fc3_capsule_derived_from_tape_cas` | 🟡 AMBER | capsule diverges from regenerated |
| No global Markov pointer | `fc3_no_global_markov_pointer` (also in `no_parallel_ledger.rs`) | 🟢 GREEN | `LATEST_MARKOV_CAPSULE.txt` exists |
| Raw logs not in agent read view | `fc3_raw_logs_not_in_agent_read_view` | 🟡 AMBER | agent prompt contains raw stderr |
| Latest capsule = context only | `fc3_latest_capsule_context_only` | 🟡 AMBER | capsule used as ground-truth |
| Deep history requires override | `fc3_deep_history_requires_override` | 🟢 GREEN | reads succeed without `TURINGOS_MARKOV_OVERRIDE=1` |
| No automatic predicate mutation | `fc3_no_automatic_predicate_mutation` | 🟢 GREEN | predicate definitions mutate at runtime |
| ArchitectAI proposes, no direct write | `fc3_architectai_proposal_not_direct_write` | 🟡 AMBER | architect role direct-writes |
| JudgeAI veto-only | `fc3_judgeai_veto_only` | 🟢 GREEN | judge commits code |

## §J. Predicate gate

| Test | Status | Kill condition |
|------|--------|---------------|
| `predicate_result_is_binary` | 🟢 GREEN | non-binary verdict shape |
| `predicate_failure_cannot_enter_l4` | 🟢 GREEN | rejected WorkTx in L4 |
| `predicate_pass_required_for_l4` | 🟢 GREEN | un-verified WorkTx accepted |
| `lean_verified_required_for_verified_worktx` | 🟢 GREEN | bypass |
| `price_never_overrides_predicate` | 🟢 GREEN | price modulates verdict |

## §K. Shielding gate

| Test | Status | Kill condition |
|------|--------|---------------|
| `raw_lean_stderr_not_in_agent_read_view` | 🟡 AMBER | raw stderr in prompt |
| `l4e_public_summary_low_pollution` | 🟡 AMBER | rejection summary >threshold informational entropy |
| `private_diagnostic_cid_not_serialized_publicly` | 🟢 GREEN | private CID broadcast |
| `evidence_capsule_raw_logs_audit_only` | 🟡 AMBER | raw logs in capsule public-view |
| `dashboard_does_not_leak_private_failure_detail` | 🟡 AMBER | dashboard shows per-agent private diag |

## §L. Economy gate

| Test | Status | Kill condition |
|------|--------|---------------|
| `economy_read_is_free` | 🟢 GREEN | wallet read requires stake |
| `economy_write_requires_stake_or_escrow` | 🟢 GREEN | unstaked write accepted |
| `economy_no_post_init_mint` | 🟢 GREEN | mint after init |
| `economy_total_coin_conserved` | 🟢 GREEN | coin supply mutation outside dispatch |
| `economy_complete_set_yes_no_not_coin` | 🟢 GREEN | YES/NO shares counted as Coin |
| `economy_no_ghost_liquidity` | 🟢 GREEN | MarketSeed without balance debit |
| `economy_wallet_read_only_projection` | 🟢 GREEN | wallet API has mutation surface |
| `economy_no_f64_money_path` | 🟢 GREEN | f64 in money flow |
| `system_tx_not_agent_submittable` | 🟢 GREEN | agent submits SystemTx |

## §M. Tape canonical gate

| Test | Status | Kill condition |
|------|--------|---------------|
| `no_parallel_ledger_source_of_truth` | 🟢 GREEN | global pointer reappears |
| `no_shadow_tape_authoritative_parent` | 🟢 GREEN | shadow tape claims canonical |
| `canonical_txid_not_shadow_id` | 🟢 GREEN | shadow id used as canonical |
| `dashboard_regenerates_from_tape_cas` | 🟡 AMBER (MVP-3 pending) | dashboard differs |
| `chain_derived_facts_not_evaluator_stdout` | 🟡 AMBER | facts depend on stdout |
| `all_externalized_attempts_have_cas_payload` | 🟢 GREEN | attempt without CAS payload |
| `all_lean_results_have_cas_payload` | 🟢 GREEN | LeanResult without CAS |

## §N. Five MVP closure gates (directive §8)

| MVP Gate | Anchored test | Status |
|----------|---------------|--------|
| MVP-1 (FC1: tx-count equality) | `fc1_attempt_count_equals_tape_count` + P38/P49 evidence run | 🟡 AMBER (smoke pending) |
| MVP-2 (Predicate: pass→L4 / fail→L4.E) | `predicate_failure_cannot_enter_l4` + `predicate_pass_required_for_l4` | 🟢 GREEN |
| MVP-3 (Tape: dashboard regenerable) | `dashboard_regenerates_from_tape_cas` | 🟡 AMBER (MVP-3 smoke pending) |
| MVP-4 (FC2: replay) | `fc2_run_replayable_from_genesis_tape_cas` | 🟡 AMBER |
| MVP-5 (Economy conservation) | 9 economy_gate tests | 🟢 GREEN |

## §O. Closure conditions (directive §12)

| # | Condition | Source | Status |
|---|-----------|--------|--------|
| 1 | Every clause has matrix row | this file | 🟢 GREEN (≥40 rows) |
| 2 | Every critical row has a test | this file | 🟢 GREEN |
| 3 | Every test can fail (no `assert!(true)`) | CR-C0.1 | 🟡 AMBER (verify on commit) |
| 4 | P38/P49 real runs pass FC1 | constitution_gate_report.json | 🔴 RED (not yet run; LLM-cost gate) |
| 5 | Fresh replay passes FC2 | `fc2_run_replayable_from_genesis_tape_cas` | 🟡 AMBER |
| 6 | Markov / EvidenceCapsule passes FC3 | `fc3_capsule_derived_from_tape_cas` + `fc3_no_global_markov_pointer` | 🟢 GREEN |
| 7 | Economy laws pass | 9 `economy_*` tests | 🟢 GREEN |
| 8 | Dashboard regeneration passes | `dashboard_regenerates_from_tape_cas` | 🟡 AMBER |
| 9 | No high-risk feature merge without gates green | CI policy in CR-C0.10 | 🔴 RED (CI gate not yet wired) |
| 10 | Project answers 6 epistemic questions | matrix + tests answer "what externalized? predicate-pass? predicate-fail? on-tape? CAS-only? dashboard-only?" | 🟡 AMBER |

## §P. Build / cross-references

- TB-C0 charter: `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`
- TB-C0 directive: `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md`
- Existing FC element extract: `handover/alignment/FC_ELEMENTS_2026-04-22.md`
- Existing FC trace: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
- Existing FC conformance: `tests/fc_alignment_conformance.rs`
- TB-18R chain attempt invariant: `tests/tb_18r_chain_attempt_invariant.rs`
- TB-18R audit sampler: `tests/tb_18r_audit_sampler_attempt_payload.rs`
- TB-16 dashboard regen: `tests/tb_16_dashboard_live_regen.rs`
- Markov de-canonicalize (OBS_R022): `tests/markov_pointer_de_canonicalize.rs`
- Six axioms alignment: `tests/six_axioms_alignment.rs`

## §Q. Update protocol

When a new TB ships:
1. Add a row OR update existing rows for any new constitution clause / FC node touched.
2. Update Status column on real evidence (not on docs).
3. Reference the new test in the row's "Test name" cell.
4. Re-run `cargo test --workspace constitution_` and update the table.

When a row goes RED (a previously GREEN gate breaks):
1. Stop ALL feature merges to main per CR-C0.10.
2. Open an OBS in `handover/alignment/OBS_*.md`.
3. Treat as constitutional drift; escalate to architect via directive if Class 4.
