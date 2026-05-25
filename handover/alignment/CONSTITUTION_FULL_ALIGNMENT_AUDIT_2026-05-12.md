# Constitution + 3-Flowchart + Economy Full-Landing Alignment Audit

> **ARCHIVAL DERIVED VIEW — not current authority.** Retained only as historical evidence. Current flowchart topology authority is `constitution.md` FC1 `455-509`, FC2 `571-660`, FC3 `826-870`, plus the active matrices that explicitly derive from those blocks.

> **Auditor**: Claude (orchestrator role)
> **Audit timestamp**: 2026-05-12 session #46
> **Repo HEAD**: `4c56ab7` (post-G3.2 packet commit on `main`)
> **Scope**: (1) constitution.md Art. 0..V + Laws逐 clause; (2) 3 flowchart hashes + per-node mapping (FC1 15N+6INV / FC2 13N+8INV / FC3 12N+8INV); (3) Economy fully-landing per CLAUDE.md §13 + `src/economy/monetary_invariant.rs` + Polymarket-era invariants
> **Strict-reading rule**: per `feedback_no_workarounds_strict_constitution` (user verbatim "我不要凑活") — AMBER-as-permanent is unacceptable; structural-only-by-design rows MUST have runtime witnesses where mechanically possible; "deferred forward" is acceptable ONLY when explicit forward TB or §10 architect-reclassification exists.

---

## §A. Methodology

### A.1 Inputs

| Source | Path | Lines | Authority rank (per CLAUDE.md §22) |
|--------|------|-------|------------------------------------|
| Constitution | `constitution.md` | 886 | 1 (supreme) |
| 3 Flowcharts (inline) | `constitution.md:455-509` (FC1) + `:571-660` (FC2) + `:826-870` (FC3); canonical 4 SHA256 hashes in `TRACE_FLOWCHART_MATRIX.md:24-40` | n/a | 2 |
| Execution matrix | `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` | 248 | 4 (executable gates) |
| Trace flowchart matrix | `handover/alignment/TRACE_FLOWCHART_MATRIX.md` | 202 | 5 (per-node bindings) |
| Prior gap analysis | `handover/alignment/CONSTITUTION_GAP_ANALYSIS_2026-05-07.md` | 338 | historical baseline |
| Landing manifest | `handover/alignment/CONSTITUTION_LANDING_MANIFEST_2026-05-09.md` | 299 | historical baseline |
| CLAUDE.md project file | `/home/zephryj/projects/turingosv4/CLAUDE.md` | n/a | 1 (operating law) |
| MEMORY.md | `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/MEMORY.md` | n/a | session-state |

### A.2 Classification taxonomy

Per `CONSTITUTION_EXECUTION_MATRIX.md:10-13` legend:
- 🟢 **GREEN** — test exists, asserts real invariant, passes `cargo test --workspace`
- 🟡 **AMBER** — test exists, structural-only or limited coverage; smoke evidence pending or partial
- 🔴 **RED** — no test, OR test only `assert!(true)` / docs-only
- 🚫 **N/A** — clause is intentionally non-runtime
- 📅 **DEFERRED-FORWARD** — explicit forward TB / forward gate (not the same as RED)

### A.3 Severity classification (for gap inventory §G)

- **S0 (blocker)** — load-bearing freeze trigger per CLAUDE.md §20; new feature work halts until closed
- **S1 (constitutional debt)** — clause has zero runtime implementation (true 🔴 RED)
- **S2 (forward-bound)** — gap surfaced + tracked + DEFERRED-FORWARD with concrete owner
- **S3 (real-load coverage)** — structural gate GREEN, real-load smoke partial or missing
- **S4 (architectural fidelity)** — current implementation satisfies semantic version; constitution names a stronger version that's gated to a future Phase

### A.4 Re-runnable verification commands

```bash
# Matrix AMBER count (current-status only; filters "was 🟡 AMBER" historical annotations)
awk -F'|' '/^## §[A-Z]/ {section=$0} /^\|/ && !/^\|---/ && !/clause/ {
  for(i=1;i<=NF;i++){if(match($i,/🟢 GREEN|🟡 AMBER|🔴 RED/)){
    status=substr($i,RSTART,RLENGTH); if(status=="🟡 AMBER") print section": "$2" => "status; break}}
}' handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md

# Constitution gate runner (authoritative test count)
bash scripts/run_constitution_gates.sh

# Gardener absence (Art. III.1 §379-380 strict-RED gap)
grep -rnE 'gardener_agent|gardener\b|garbage_collect.*agent' src/

# Q_t version-control git substrate (Art. 0.4 path-A grep — semantic-only)
grep -rE "Repository::|git2::|libgit2|Command git" src/ experiments/minif2f_v4/src/
```

---

## §B. Constitution Article-by-Article Alignment (Art. 0..V + Laws)

### B.0 Art. 0 — Turing-machine foundationalism (constitution.md:27-160)

| Clause | Code surface | Test | Real-load witness | Status | Gap |
|--------|--------------|------|---------------------|--------|------|
| **Art. 0.1** 四要素映射 (paper/pencil/rubber/discipline) | `src/ledger.rs::Tape` (paper) + `src/sdk/write_tool.rs` (pencil) + `src/state/sequencer.rs` accept/reject (rubber) + `src/sdk/predicate.rs` (discipline) | `tests/four_element_mapping.rs` (5 tests) + `tests/constitution_wave3_evidence_binding.rs::wave3_50p_chain_invariant_all_pass` | Wave 3 50p binding (460 cycles exercise all 4 elements) | 🟢 GREEN | none |
| **Art. 0.2** Tape Canonical | `src/ledger.rs::Tape` + `src/bottom_white/ledger/transition_ledger.rs::L4` + `src/bottom_white/cas/` | `tests/constitution_no_parallel_ledger.rs::no_parallel_ledger_source_of_truth` + `tests/markov_pointer_de_canonicalize.rs` + Wave 3 50p binding | TB-16 + Wave 3 50p (runtime_repo present per problem; canonical-tape-source confirmed at scale) | 🟢 GREEN | **S2/S4 partial**: 24-violation 10-commit plan per Art. 0.2 §修复义务 — only commits 1-4 explicitly tracked. Commits 5-10 (WAL+mr-tick / synthetic short-circuit / Boltzmann pick / search-board-wallet sidecar projections / Lean error string + halt detail / WAL hash chain + audit guard) status NOT surfaced in current matrix. Strict-reading question: are 5-10 silently satisfied by post-TB-C0 work or actually open? — surface for explicit architect verdict |
| **Art. 0.3** 区块链化保留 (hash chain Merkle DAG) | `src/wal.rs` (WAL append-only) + `src/bottom_white/ledger/transition_ledger.rs` (Stage A3 multi-ref ChainTape SHIPPED) | `tests/constitution_fc1_runtime_loop.rs::fc1_no_legacy_authoritative_append` + Wave 3 50p binding | Stage A3 R5 smoke (refs/chaintape/{l4,l4e,cas} live) | 🟢 GREEN | **S4 architectural fidelity**: constitution §Art. 0.3 + §Phase E gate names "Phase E 加 Merkle root + heldout_sealed_hash 双锁"; **`refs/chaintape/cas` strict-Merkle commit-chain redesign DEFERRED to Stage A3.6 enhancement TB** per LATEST.md "Open after Polymarket" + matrix §A.4 DEFERRED-FORWARD note. Polymarket runtime unaffected. Class 3-4 / ~3-5 days |
| **Art. 0.4** Q_t version-controlled (G-009 HEAD_t) | `src/state/q_state.rs` + `src/state/head_t_witness.rs` + Stage A3 multi-ref refs/chaintape/{l4,l4e,cas} | `tests/constitution_head_t_witness.rs` (5 tests; C1) + `tests/constitution_head_t_c2_multi_ref.rs` (7 tests; C2) | Stage A3 R5 + R3.5 real-LLM smoke | 🟢 GREEN (C1+C2 substrate FULLY VERIFIED 2026-05-08) | **S4 architectural fidelity**: constitution Art. 0.4 explicit 3-path framing — Path A (语义版 ~3wk) / Path B (真 git ~6-8wk) / Path C (延期版 hybrid). Path C SHIPPED. **Path B is constitution-stated Phase E gate FORCING REQUIREMENT** (verbatim: "Phase E gate 强制 B"). Not yet at Phase E gate; deferred per constitution explicit clause |
| **Art. 0 Laws (基本法)** Information Free / Investment Costs Money / 1Coin=1YES+1NO / on_init only mint | `src/economy/monetary_invariant.rs` + `src/state/sequencer.rs` | `tests/constitution_economy_gate.rs` (9 tests all PASS) + Wave 3 50p binding | Wave 3 50p (50 problems × economic-flow cycles; no mint/burn outside on_init; no agent-submitted SystemTx) | 🟢 GREEN | none |

**Art. 0 verdict**: substantively GREEN. 2 S4 architectural-fidelity gaps (commits 5-10 surfacing + Path B Phase E gate) — both constitution-acknowledged deferrals, not silent debt.

### B.I Art. I — Signal quantification (constitution.md:163-298)

| Clause | Code surface | Test | Real-load witness | Status | Gap |
|--------|--------------|------|---------------------|--------|------|
| **Art. I.1** Boolean signal (binary predicate) | `src/sdk/predicate.rs::Predicate` trait + `src/runtime/verify.rs` | `tests/constitution_predicate_gate.rs::predicate_result_is_binary` | TB-13/14 verify smoke | 🟢 GREEN | none |
| **Art. I.1** predicate fail → L4.E | `src/state/sequencer.rs` reject arm + `src/bottom_white/ledger/transition_ledger.rs::EventType::*::Rejected` | `tests/constitution_predicate_gate.rs::predicate_failure_cannot_enter_l4` + `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs` | TB-18R R3 + Wave 3 50p | 🟢 GREEN | none |
| **Art. I.1** predicate pass required for L4 | `src/state/sequencer.rs::apply_one` | `tests/constitution_predicate_gate.rs::predicate_pass_required_for_l4` | TB-13/14/18R + Wave 3 50p | 🟢 GREEN | none |
| **Art. I.1** Lean verified required for verified WorkTx | `src/runtime/verify.rs::verify_work_tx_lean` | `tests/constitution_predicate_gate.rs::lean_verified_required_for_verified_worktx` | TB-18R R1+R2 + Wave 3 50p | 🟢 GREEN | none |
| **Art. I.1.1** PCP 疑罪从无 (9-class adversarial corpus) | `src/state/sequencer.rs::admit_work_tx` + `cases/pcp_corpus/` (Constitution Landing First 2026-05-07) + `cases/pcp_corpus_phase2/` (Stage B3 R3) | `tests/constitution_predicate_gate.rs::price_never_overrides_predicate` + `tests/constitution_pcp_corpus.rs` (7 tests) + `tests/constitution_pcp_corpus_phase2.rs` (real MiniF2F-derived; SG-18B.q8a) | Phase 3 cc59b4d evidence smoke + Wave 3 50p | 🟢 GREEN | none |
| **Art. I.2** Statistical signal (PPUT / Wilson CI / reputation) | `src/runtime/wilson_ci.rs` (helper) + `src/runtime/evaluator.rs` ΣPPUT + `src/economy/reputation.rs` `Reputation(i64)` | `tests/constitution_wilson_ci.rs` (5 tests) + `tests/economic_state_reconstruct.rs` | helper available; aggregate report wire-up DEFERRED | 🟢 GREEN at helper layer | **S3/S2 forward-bound**: Wilson CI computation done OFFLINE today (`experiments/minif2f_v4/src/jsonl_schema.rs::pput`); helper `WilsonCi::new_95(k,n)` landed but aggregate report wire-up forward (matrix §B kill-condition closed at helper layer per §I FC3-INV1 precedent). **Strict reading**: replay-determinism would benefit from in-tree assertion. Real debt minor. AND `reputations_t` mutation never fires (closed under G3.2 packet Surface 4 Gap-A) — see §F gap 2 below |

**Art. I verdict**: 5 GREEN + 1 GREEN-at-helper-layer. The only meaningful gap is `reputations_t` never mutated (statistical signal degenerate on real-LLM runs) — bundled under G3.2 packet Surface 4.

### B.II Art. II — Selective broadcast (constitution.md:299-358)

| Clause | Code surface | Test | Real-load witness | Status | Gap |
|--------|--------------|------|---------------------|--------|------|
| **Art. II.1** broadcast typical errors (NO raw stderr) | `src/sdk/snapshot.rs::UniverseSnapshot` + `src/sdk/prompt.rs` + `src/runtime/audit_assertions.rs::assert_30_typical_error_summary_no_private_detail` | `tests/constitution_shielding_gate.rs::raw_lean_stderr_not_in_agent_read_view` + Wave 3 50p shielding binding | LeanResult max 146B / 447 instances + 2074-CAS-object no-leakage schema_id aggregate | 🟢 GREEN | none |
| **Art. II.2** broadcast price signal | `src/economy/price_index.rs` (TB-14) | `tests/tb_14_price_index.rs` + `tests/constitution_predicate_gate.rs::price_never_overrides_predicate` | TB-14 price smoke | 🟢 GREEN | none |
| **Art. II.2.1** exploration / exploitation balance | `src/runtime/diversity.rs` helper + `src/runtime/audit_assertions.rs::assert_e_boltzmann_parent_selection_diversity` (id=43; V3L-14 fix ≥0.5 threshold) + Boltzmann routing | `tests/constitution_diversity.rs` (7 tests + 12 inline lib tests) | helper available; aggregate report integration DEFERRED | 🟢 GREEN at helper layer | **S3 forward-bound**: same shape as Art. I.2 Wilson CI — helper `parent_selection_shannon_entropy` + `distinct_payload_fraction` + `DiversityReport::is_below_alarm_floor(0.25)` LANDED; aggregate-report wire-up forward (matrix §C kill closed at helper). Real debt small but architecturally relevant per CLAUDE.md Report Standard |

**Art. II verdict**: 2 GREEN + 1 GREEN-at-helper-layer. Both helper-layer gaps are same-shape (aggregate-report integration); could close in 1 atom.

### B.III Art. III — Selective shielding (constitution.md:360-426)

| Clause | Code surface | Test | Real-load witness | Status | Gap |
|--------|--------------|------|---------------------|--------|------|
| **Art. III.1** shield errors (raw failure logs not in agent prompt) | `src/sdk/snapshot.rs` + `src/runtime/attempt_telemetry.rs` (private CID) | `tests/constitution_shielding_gate.rs::private_diagnostic_cid_not_serialized_publicly` + Wave 3 50p shielding binding | AttemptTelemetry max 469B + TypedTx.v1 max 459B / 668 instances + capsule_count == raw_log_companion_count (1:1) | 🟢 GREEN | none |
| **Art. III.1 Gardener Agent** (constitution §379-380 verbatim "部署后台'园丁 Agent'，定期扫描并屏蔽那些偏离黄金原则的陈旧代码与过期文档") | `grep -rnE 'gardener_agent\|gardener\b\|garbage_collect.*agent' src/` → **0 hits** (verified 2026-05-12 audit) | none | none | 🔴 **RED** | **S1 constitutional debt — TRUE GAP**: the ONLY 🔴 RED unimplemented constitutional clause in the entire project. Listed as G-020 in `CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md`; 📅 deferred to forward TB charter (post-TB-21) per Pass 2 audit. Per `feedback_no_workarounds_strict_constitution`, this is constitution-mandated and unimplemented → demands either (a) forward TB with concrete owner + class + ship-window OR (b) explicit architect §10 reclassification (e.g., "Gardener is a Phase E-and-beyond concept; current pre-Phase-E architecture obviates") |
| **Art. III.2** encapsulation (high-volume in CAS, audit-only) | `src/bottom_white/cas/schema.rs::ObjectType::AttemptTelemetry/LeanResult/PromptCapsule/EvidenceCapsule` | `tests/constitution_shielding_gate.rs::evidence_capsule_raw_logs_audit_only` + `tests/tb_18r_audit_sampler_attempt_payload.rs` + Wave 3 50p shielding binding | capsule shell max 485B / 41 instances + raw_log companion max 389B / 41 instances + 1:1 capsule/companion count | 🟢 GREEN | none |
| **Art. III.3** shield correlation (no Goodhart leakage) | `src/economy/reputation.rs` projection | `tests/constitution_shielding_gate.rs::dashboard_does_not_leak_private_failure_detail` + Wave 3 50p shielding binding | no schema_id matching forbidden tokens across 2074-CAS aggregate | 🟢 GREEN | none |
| **Art. III.4** shield Goodhart (selector blindness) | `src/runtime/evaluator.rs` selector blindness (implicit) | `tests/constitution_shielding_gate.rs::l4e_public_summary_low_pollution` + Wave 3 50p shielding binding | TransitionError.display max 48B avg 34B / 95 instances | 🟢 GREEN | **S3 real-load**: implicit guarantee (no explicit "selector cannot read raw stderr by type" gate). Prior gap analysis 2026-05-07 §3 noted this as architecturally important; Wave 3 50p binding 2026-05-08 size-bound on `TransitionError.display` rules out selector receiving full diagnostic. Closed at evidence level |
| **Art. III prompt persistence** (G-016 / G-019 / G-021 / G-028) | `src/runtime/prompt_capsule.rs` (Class-3 7-field capsule per architect §4.3) + `src/bottom_white/cas/schema.rs::ObjectType::PromptCapsule` | `tests/constitution_prompt_capsule.rs` (7 tests) + 3 inline tests | CAS round-trip smoke; evaluator runtime wire-up DEFERRED | 🟢 GREEN at gate level | **S2 forward-bound — OBS_G2P_PROMPT_BODY_OBSERVABILITY 2026-05-12**: evaluator emit of PromptCapsule per LLM call + `AttemptTelemetry.prompt_capsule_cid` field is forward-bound to G2P.4 (Class 2-3). Without runtime emit, cannot empirically prove `=== Pending Peer Reviews ===` / `=== Your Position ===` blocks reached the LLM (G2P R1 Q1 CHALLENGE from Codex G2). Per CLAUDE.md §4.3 verbatim: "PromptCapsule -> CAS; AttemptTelemetry / WorkTx references prompt_capsule_cid; L4/L4.E anchor references the attempt" — the canonical default policy IS Class-3 emit; G2P.4 is the closure |

**Art. III verdict**: 5 GREEN + 1 GREEN-at-gate-level + **1 RED (Gardener Agent)**. Gardener is THE single constitutional-debt RED row in the matrix.

### B.IV Art. IV — Boot (constitution.md:513-661)

| Clause | Code surface | Test | Real-load witness | Status | Gap |
|--------|--------------|------|---------------------|--------|------|
| **Art. IV.boot** Q_0 generated by InitAI exactly once | `src/runtime/evaluator.rs::run_swarm` + `src/state/sequencer.rs::genesis` | `tests/constitution_fc2_boot.rs::fc2_genesis_report_exists` + `fc2_on_init_only_mint` | TB-17 + Wave 3 50p (50 problems × genesis present) | 🟢 GREEN | none |
| **Art. IV.halt** HaltReason terminal anchor | `src/ledger.rs::HaltReason` + `src/runtime/evaluator.rs::extract_halt_reason` | `tests/six_axioms_alignment.rs` axiom-4 | TB-17 halt smoke | 🟢 GREEN | none |
| **Art. IV.tick** clock advance | `src/bus.rs::clock` + TICK_INTERVAL | `tests/six_axioms_alignment.rs` axiom-5 | TB-17 tick smoke | 🟢 GREEN | none |
| **Art. IV** fresh replay from genesis + tape + CAS | `src/boot/genesis_payload.rs` + transition_ledger replay | `tests/constitution_fc2_boot.rs::fc2_run_replayable_from_genesis_tape_cas` + `tb_18r_chain_attempt_invariant.rs` + Wave 3 50p binding | audit_proceed=50 + id45_pass=50 + inv1_match=50 three-observer agreement | 🟢 GREEN (MVP-4) | none |
| **Art. IV** system pubkeys verify | `src/state/system_keypair.rs` | `tests/constitution_fc2_boot.rs::fc2_system_pubkeys_verify` + `tests/system_keypair_*.rs` (5 existing) | TB-17 keypair smoke | 🟢 GREEN | none |
| **Art. IV** agent registry resolves | `src/runtime/agent_registry.rs` | `tests/constitution_fc2_boot.rs::fc2_agent_registry_resolves` | TB-13 registry smoke | 🟢 GREEN | none |
| **Art. IV** TaskOpen / EscrowLock are chain events | `src/state/typed_tx.rs::TaskOpenTx` / `EscrowLockTx` | `tests/constitution_fc2_boot.rs::fc2_taskopen_escrowlock_are_chain_events` | TB-13 task-open smoke | 🟢 GREEN | none |
| **Art. IV** no memory-only preseed | `src/state/q_state.rs::EconomicState` | `tests/constitution_fc2_boot.rs::fc2_no_memory_only_preseed` (source-grep) + `wave3_50p_no_memory_only_preseed_binding` (Wave 3 50p replay-determinism witness) | 50/50 audit_proceed + 50/50 FC1-INV1 cross-observer agreement | 🟢 GREEN | none |

**Art. IV verdict**: 8/8 GREEN. Boot path is the cleanest article.

### B.V Art. V — Meta / separation of powers (constitution.md:664-815)

| Clause | Code surface | Test | Real-load witness | Status | Gap |
|--------|--------------|------|---------------------|--------|------|
| **Art. V.1.1** constitution as single ground truth (sudo权限仅作用于constitution.md) | `constitution.md` + Trust Root manifest `genesis_payload.toml` | `tests/fc_alignment_conformance.rs` battery | per-PR FC alignment + Stage A3 multi-ref ChainTape preserves Trust Root | 🟢 GREEN | none |
| **Art. V.1.2** ArchitectAI proposes (NOT direct write to src/ without TB charter) | external (architect handover/directives/) + `genesis_payload.toml` manifest | `tests/constitution_fc3_meta.rs::fc3_architectai_proposal_not_direct_write` + `tests/constitution_fc3_evidence_binding.rs::fc3_inv7_architect_proposes_no_direct_write_git_witness` | git-author scan: only `gretjia`/`Claude` authors; zero `codex@`/`gemini@`/`judgeai`/`architect_direct`/`audit-role` across full git history | 🟢 GREEN | none |
| **Art. V.1.3** Veto-AI veto-only (PASS/VETO; no主观质量评判) | external (Codex + Gemini dual audit) | `tests/constitution_fc3_meta.rs::fc3_judgeai_veto_only` + `tests/constitution_fc3_evidence_binding.rs::fc3_inv8_judgeai_veto_only_audit_dir_witness` | audit-dir 371-file file-extension whitelist: zero `.rs`/`.toml`/`.lock`/`.cargo` | 🟢 GREEN | none |
| **Art. V.2** constitution boundaries (hash drift requires architect signature) | `tests/fc_alignment_conformance.rs::fc3_constitution_hash_pinned` + `tests/constitution_fc3_evidence_binding.rs::fc3_art_v2_constitution_boundaries_witness` | full git-log scan over `constitution.md` commits: every commit modifying it cites tracer-prefix anchor (TB/Stage/Phase/CO/directive/charter/amendment/Art./公理/宪法/sudo/Initial-commit/V3L) | full git-history walk passes 2026-05-08 | 🟢 GREEN | none |
| **Art. V.3** amendment log | `constitution.md` §5.3 + `cases/C-*.yaml` | `tests/constitution_art_v3_amendment_log.rs` (6 tests; round-8): section_exists_and_parseable + every_amendment_has_four_populated_columns + every_amendment_triggered_by_human_architect + every_amendment_date_is_iso_format + constitution_hash_matches_trust_root_manifest + historical_amendments_remain_recorded | structural + trust-root binding | 🟢 GREEN (round-8: was 🔴 RED, was 🚫 N/A) | none |

**Art. V verdict**: 5/5 GREEN.

### B.Laws Art. 0 Laws — economy laws (constitution.md:155-161; expanded in CLAUDE.md §13)

Covered in §F below (full economy gap analysis).

---

## §C. FC1 (Runtime Loop) Per-Node Alignment

**Canonical SHA256 (TRACE_FLOWCHART_MATRIX.md:24-31)**:
- FC1a: `a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5`
- FC1b: `b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d`

### C.1 Per-node coverage (15 nodes)

| Node | Constitution role | Code surface | Test | Status |
|------|--------------------|---------------|------|--------|
| FC1-N1 (q0) | `Q_t = ⟨q_t, HEAD_t, tape_t⟩` carrier | `TuringBus` + `Kernel::tape` | `fc2_genesis_report_exists` + `fc1_attempt_count_equals_tape_count` + Wave 3 50p | ✅ |
| FC1-N2 (q_t) | q_t slice | `QState` + `TuringBus::q_state` | `tests/q_state_reconstruct.rs` | ✅ |
| FC1-N3 (HEAD_t) | head pointer | `time_arrow().last()` + `head_t_witness::HeadTWitness` (G-009 C1+C2) | `tests/constitution_head_t_witness.rs` + `tests/constitution_head_t_c2_multi_ref.rs` | ✅ |
| FC1-N4 (q1) | `Q_{t+1}` after δ | `TuringBus::append_*` + sequencer accept | `fc1_predicate_pass_goes_l4` | ✅ |
| FC1-N5 (rtool) | read tool — ground-truth context fetch | `ReadTool::project` + `DefaultReadTool` | `fc3_raw_logs_not_in_agent_read_view` + Wave 3 50p shielding binding | ✅ |
| FC1-N6 (input ⟨q_i, s_i⟩) | input bundle | `UniverseSnapshot` + `build_agent_prompt` | `tests/fc_alignment_conformance.rs::fc1_n6_*` | ✅ |
| FC1-N7 (δ / AI) | external LLM call | `ResilientLLMClient::generate` | `fc1_every_externalized_attempt_is_tape_visible` + Wave 3 50p (460 cycles) | ✅ |
| FC1-N8 (output ⟨q_o, a_o⟩) | Agent output | `AgentOutput` + `parse_agent_output` | fc_alignment_conformance | ✅ |
| FC1-N9 (q_o) | proposed q-delta | `AgentOutput::q_delta` | `fc1_attempt_count_equals_tape_count` + Wave 3 50p | ✅ |
| FC1-N10 (a_o) | action | `AgentOutput::action` | `fc1_predicate_pass_goes_l4` + `fc1_predicate_fail_goes_l4e` | ✅ |
| FC1-N11 (∏p predicates) | predicate composition | `TuringBus::evaluate_predicates` + `Predicate` trait | `predicate_result_is_binary` + `predicate_pass_required_for_l4` | ✅ |
| FC1-N12 (individual p) | Forbidden/Sorry/PayloadSize/Lean | `{Forbidden,Sorry,PayloadSize}Predicate` + `Lean4Oracle::verify_*` | `lean_verified_required_for_verified_worktx` + `price_never_overrides_predicate` | ✅ |
| FC1-N13 (wtool) | write tool | `WriteTool::write` + `TuringBus::append_oracle_accepted` | `fc1_no_legacy_authoritative_append` + Wave 3 50p | ✅ |
| FC1-N14 (Q_{t+1} success) | accepted advance | `append_internal` + `halt_with_reason` | `fc1_predicate_pass_goes_l4` | ✅ |
| FC1-N15 (Q_t reject branch) | ∏p=0 rejection | `PartialVerdict::Reject` + `BusResult::Vetoed` + sequencer reject arm | `fc1_predicate_fail_goes_l4e` | ✅ |

### C.2 Invariant battery (6 INV)

| Invariant | Test | Real-load | Status |
|-----------|------|-----------|--------|
| **FC1-INV1** every externalized attempt → tape | `fc1_every_externalized_attempt_is_tape_visible` + `wave3_50p_aggregate_fc1_invariant_holds` | 460 = 9 + 400 + 51 cycles across 50/50 + G3 9-task `aggregate_verdict.json:409 = "PROCEED"` | ✅ GREEN |
| **FC1-INV2** predicate routing | `fc1_predicate_pass_goes_l4` + `fc1_predicate_fail_goes_l4e` | Wave 3 50p | ✅ GREEN |
| **FC1-INV3** count equality | `fc1_attempt_count_equals_tape_count` (LHS scope `tool_dist.step + parse_fail + llm_err` per `OBS_TB18R_INV1_NONLLM_TX` 2026-05-07) | Wave 3 50p 50/50 verdict=Ok delta=0 | ✅ GREEN |
| **FC1-INV4** no legacy bypass | `fc1_no_legacy_authoritative_append` + `wave3_50p_chaintape_runtime_repo_present` | 460 sequencer-mediated cycles; runtime_repo present per problem | ✅ GREEN |
| **FC1-INV5** dashboard not source of truth | `fc1_dashboard_not_source_of_truth` + `wave3_50p_dashboard_regen_matches_chain` | 50/50 chain_invariant.json regen matches chain | ✅ GREEN (MVP-3) |
| **FC1-INV6** no fake accepted nodes | `fc1_no_fake_accepted_nodes` + `wave3_50p_solve_count_three_observer_agreement` + L4.E body integrity gate (`tests/constitution_l4e_body_integrity.rs` 7 tests; session #34 2026-05-10) | tamper detection real witness (L4.E blob byte-flip Halt + L4.E ref corruption Halt) | ✅ GREEN |

**FC1 verdict**: 15/15 nodes + 6/6 INV all GREEN. **Zero gap.** G3.2 STRENGTHENS FC1 by adding 1 admission predicate (bankruptcy risk-cap); not closing an existing FC1 gap.

---

## §D. FC2 (Boot) Per-Node Alignment

**Canonical SHA256**: `6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333`

### D.1 Per-node coverage (13 nodes)

| Node | Role | Code | Test | Status |
|------|------|------|------|--------|
| FC2-N16 (InitAI) | bootstrap entity | `run_swarm` + `run_oneshot` | `fc2_genesis_report_exists` + `fc2_on_init_only_mint` | ✅ |
| FC2-N17 (human architect) | manual external | constitution.md author | `fc3_architectai_proposal_not_direct_write` (FC3 binding) | 🚫 N/A runtime |
| FC2-N18 (law / ground truth) | constitution.md | `constitution.md` | `fc3_constitution_hash_pinned` | 🚫 N/A runtime |
| FC2-N19 (initAI → predicates) | boot-time predicate registration | `TuringBus::register_predicate` | `fc2_taskopen_escrowlock_are_chain_events` | ✅ |
| FC2-N20 (initAI → mr) | mr-tick at boot | TICK_INTERVAL + `emit_mr_tick_node` | `six_axioms_alignment.rs::axiom_5` | ✅ |
| FC2-N21 (initAI → Q0) | Q_0 minted | `Kernel::new` + `TuringBus::init` | `fc2_on_init_only_mint` + `fc2_no_post_init_mint` | ✅ |
| FC2-N22 (HALT) | halted state | `QState::Halted` + `halt_with_reason` | `six_axioms_alignment::axiom_4` | ✅ |
| FC2-N23 (HaltReason variants) | terminal distribution | `HaltReason` enum | existing | ✅ |
| FC2-N24 (clock) | tick clock | `TuringBus::clock` | `six_axioms_alignment::axiom_5` | ✅ |
| FC2-N25 (mr) | map-reduce tick | inline mr_summary + `emit_mr_tick_node` | existing | ✅ |
| FC2-N26 (mr → map tape0) | mr reads tape | `tape.time_arrow().len()` | existing | ✅ |
| FC2-N27 (mr → reduce tape1) | mr emits tape node | `emit_mr_tick_node` | existing | ✅ |
| FC2-N28 (tools_other) | non-rtool/wtool tools | `WriteTool::write_with_tools` + `TuringBus::tools` | `fc2_taskopen_escrowlock_are_chain_events` | ✅ |

### D.2 Invariant battery (8 INV; INV8 is G1.1 resume-mode addition)

| Invariant | Test | Real-load | Status |
|-----------|------|-----------|--------|
| **FC2-INV1** genesis exists | `fc2_genesis_report_exists` | trust root verify | ✅ |
| **FC2-INV2** init-only mint | `fc2_on_init_only_mint` + `fc2_no_post_init_mint` | Wave 3 50p | ✅ |
| **FC2-INV3** no memory preseed | `fc2_no_memory_only_preseed` + `wave3_50p_no_memory_only_preseed_binding` | Wave 3 50p replay-determinism | ✅ |
| **FC2-INV4** chain events | `fc2_taskopen_escrowlock_are_chain_events` | TB-13 smoke | ✅ |
| **FC2-INV5** replayable | `fc2_run_replayable_from_genesis_tape_cas` + `wave3_50p_replay_assertions_all_pass` | three-observer agreement | ✅ (MVP-4) |
| **FC2-INV6** pubkeys verify | `fc2_system_pubkeys_verify` | TB-17 | ✅ |
| **FC2-INV7** registry resolves | `fc2_agent_registry_resolves` | TB-13 | ✅ |
| **FC2-INV8** resume-from-existing-chain (G1.1; architect §8 SIGNED 2026-05-11) | `tests/constitution_g1_resume.rs` (sg_g1_1..g1_5) | G1.1 mini smoke + G1.2-7 9-task full batch at `handover/evidence/g_phase_g1_2_full_*/` | ✅ |

**FC2 verdict**: 13/13 nodes + 8/8 INV all GREEN. **Zero gap.**

---

## §E. FC3 (Meta / Anti-Oreo) Per-Node Alignment

**Canonical SHA256**: `c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd`

### E.1 Per-node coverage (12 nodes)

| Node | Role | Code | Test | Status |
|------|------|------|------|--------|
| FC3-N29 (boot) | system boot | `async fn main` + `TuringBus::boot` | `fc2_genesis_report_exists` (FC2 anchor) | ✅ |
| FC3-N30 (constitution file) | ground truth | `constitution.md` | `fc_alignment_conformance.rs` | 🚫 N/A runtime |
| FC3-N31 (logs archive) | WAL + L4 archive | `Wal::write_event` + transition_ledger | `fc3_raw_logs_not_in_agent_read_view` + Wave 3 50p shielding | ✅ |
| FC3-N32 (JudgeAI / Veto-AI) | external veto agent | Codex + Gemini external | `fc3_judgeai_veto_only` + `fc3_inv8_judgeai_veto_only_audit_dir_witness` | ✅ |
| FC3-N33 (ArchitectAI) | external propose agent | Claude editing | `fc3_architectai_proposal_not_direct_write` + `fc3_inv7_architect_proposes_no_direct_write_git_witness` | ✅ |
| FC3-N34 (readonly guard) | constitution + logs read-only | WAL append-only | `fc3_no_automatic_predicate_mutation` | ✅ |
| FC3-N35 (anti-oreo top→agents→tools) | signal mgmt flow | `evaluate_predicates` | fc_alignment_conformance | ✅ |
| FC3-N36 (agents) | swarm of N agents | `let agent_ids` | existing | ✅ |
| FC3-N37 (tools) | bottom tools | `TuringTool` trait | existing | ✅ |
| FC3-N38 (Q update) | Q-delta via wtool | sequencer dispatch | `fc1_predicate_pass_goes_l4` (FC1 anchor) | ✅ |
| FC3-N39 (markov / capsule) | Markov capsule | `markov_capsule.rs` | `fc3_capsule_derived_from_tape_cas` + `fc3_no_global_markov_pointer` + `fc3_inv1_capsule_integrity_regen.rs` (4 tests; round-8) | ✅ |
| FC3-N40 (override) | deep-history override | `TURINGOS_MARKOV_OVERRIDE=1` | `fc3_deep_history_requires_override` + `fc3_inv5_deep_history_default_deny_runtime_witness` | ✅ |

### E.2 Invariant battery (8 INV; all promoted from AMBER to GREEN 2026-05-08 "宪法完整落地")

| Invariant | Test | Real-load | Status |
|-----------|------|-----------|--------|
| **FC3-INV1** capsule derived from tape + CAS | `fc3_capsule_derived_from_tape_cas` + `tests/constitution_fc3_inv1_capsule_integrity_regen.rs` (capsule_id == sha256(canonical_bytes) on P08 + P05 + P07 real evidence) | real TB-C0 batch P08 39 step_partial_ok | ✅ |
| **FC3-INV2** no global Markov pointer | `fc3_no_global_markov_pointer` + `no_parallel_ledger.rs` | filesystem invariant | ✅ |
| **FC3-INV3** raw logs not in agent read view | `fc3_raw_logs_not_in_agent_read_view` + `fc3_inv3_raw_logs_size_bound` | Wave 3 50p (lean_result.v2 max ≤1024B + TransitionError.display max ≤256B / 2074-CAS-object aggregate) | ✅ |
| **FC3-INV4** capsule context-only | `fc3_latest_capsule_context_only` + `fc3_inv4_latest_capsule_context_only_real_witness` | Wave 3 50p replay-determinism: 50/50 chain_invariant verdict=Ok delta=0 (capsule NOT a state_root input) | ✅ |
| **FC3-INV5** deep-history default-deny | `fc3_deep_history_requires_override` + `fc3_inv5_deep_history_default_deny_runtime_witness` (production helper exercise: `try_deep_history_read_with_override_check(false)` returns Err; (true) returns Ok) | binary gate; not vacuous | ✅ |
| **FC3-INV6** no automatic predicate mutation | `fc3_no_automatic_predicate_mutation` | structural | ✅ |
| **FC3-INV7** architect propose-only | `fc3_architectai_proposal_not_direct_write` + `fc3_inv7_architect_proposes_no_direct_write_git_witness` | full git-history witness | ✅ |
| **FC3-INV8** judge veto-only | `fc3_judgeai_veto_only` + `fc3_inv8_judgeai_veto_only_audit_dir_witness` | 371-file file-extension whitelist | ✅ |

**FC3 verdict**: 12/12 nodes + 8/8 INV all GREEN. **Zero gap.** The 2026-05-08 "宪法完整落地" closure (per user verbatim "我要宪法完整落地" + "我不要凑活") brought the last 7 AMBER → GREEN via runtime witnesses — strict-constitution stance prevailed over architect's "structural-only by design" framing.

---

## §F. Economy Fully-Landing Gap Analysis

Per CLAUDE.md §13 economy laws + `src/economy/monetary_invariant.rs` assertion functions + Polymarket-era invariants (Stage C P-M2..P-M9 SHIPPED).

### F.1 Constitutional economy laws (CLAUDE.md §13 + constitution.md:155-161 Laws)

| Law | Code surface | Test (matrix §L) | Real-load witness | Status | Gap |
|-----|--------------|-------------------|---------------------|--------|------|
| **Information is Free** (Art. 0 Law 1; CLAUDE.md §13) | `src/economy/monetary_invariant.rs::assert_read_is_free` (line 469) | `economy_read_is_free` | Wave 3 50p (50 problems × wallet/search read paths; no fee debit) | 🟢 GREEN | none |
| **Only Investment Costs Money** (Art. 0 Law 2; CLAUDE.md §13) | sequencer admission stake/escrow gates | `economy_write_requires_stake_or_escrow` | Wave 3 50p | 🟢 GREEN | none |
| **1 Coin = 1 YES + 1 NO** (CTF conservation; CLAUDE.md §13) | `src/economy/monetary_invariant.rs::assert_total_ctf_conserved` (line 434) + `assert_complete_set_balanced` (line 523; Phase E.3 strict sym branch + CTF-MIN-SAFE asymmetric post-resolution branch) | `economy_complete_set_yes_no_not_coin` + `constitution_economy_strict_equality.rs` (Phase E.3 lint gate: `// CTF-MIN-SAFE:` marker on every `min(` call in `assert_complete_set_balanced`) | Wave 3 50p + Stage C P-M6 (`buy_yes_mints_complete_set` strict-equality + `buy_yes_no_ghost_liquidity` symmetric balance witness) | 🟢 GREEN | none |
| **on_init is the only legal base-Coin mint** (CLAUDE.md §13) | `src/economy/monetary_invariant.rs::assert_no_post_init_mint` (line 332; allow-list-by-TypedTx variant) | `economy_no_post_init_mint` + `fc2_on_init_only_mint` + `fc2_no_post_init_mint` | Wave 3 50p (50 problems × no post-init mint observed) | 🟢 GREEN | none |
| **Total Coin conserved** (CLAUDE.md §13) | `total_supply_micro` 6-holding sum (balances_t + conditional_collateral_t + lp_share_balances_t + cpmm_pool_reserves + claims_t + ...) preserved across every accepted tx | `economy_total_coin_conserved` + Stage C P-M6 `buy_yes_no_ghost_liquidity` (post-2-router state: sum_yes_traders + pool.pool_yes == collateral; symmetric strict) | Wave 3 50p + Stage C 50p+ | 🟢 GREEN | none |
| **YES/NO shares are claims, not Coin** (CLAUDE.md §13) | `EconomicState.conditional_share_balances_t` separate from `balances_t`; CTF-MIN-SAFE marker | `economy_complete_set_yes_no_not_coin` | Stage C P-M2 (MarketSeed 7-field) + P-M4 (CpmmPool reserves NOT counted in balances_t per architect §7.5 rules 2+3) | 🟢 GREEN | none |
| **No ghost liquidity** (CLAUDE.md §13) | `src/state/sequencer.rs::dispatch_market_seed` requires balance debit; `DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY` 2026-05-02 + P-M3 SHIPPED | `economy_no_ghost_liquidity` + Stage C `constitution_market_seed_hardening.rs` (5 architect §7.4 verbatim test names) | TB-13 + Stage C P-M3 | 🟢 GREEN | none |
| **WalletTool is read-only projection** (CLAUDE.md §13) | `src/sdk/tools/wallet.rs` (STEP_B-listed) | `economy_wallet_read_only_projection` | source-grep + structural | 🟢 GREEN | **S2 forward-bound**: `WalletBackend` trait abstraction (charter §0.66, sessions #41-#45 carry-forward) — current wallet read paths are direct `balances_t` reads; future on-chain wallet swap needs trait boundary. Class-4 schema-adjacent. Open Q: §8 packet during G-Phase or after G7? |
| **System tx cannot be agent-submitted** (CLAUDE.md §13) | `src/state/sequencer.rs::submit_agent_tx` admission filter | `system_tx_not_agent_submittable` | Wave 3 50p | 🟢 GREEN | none |
| **No `f64` money path** (CLAUDE.md §12 + §13) | `i64` micro-units everywhere; source-grep + cargo check-cfg | `economy_no_f64_money_path` | Wave 3 50p (50 problems × integer arithmetic; no f64 in money flow) | 🟢 GREEN | none |
| **Market price is a statistical signal, not truth** (CLAUDE.md §13) | `tests/constitution_predicate_gate.rs::price_never_overrides_predicate` + dashboard SG-14.6 banner contract | `price_never_overrides_predicate` + `tests/constitution_polymarket_smoke.rs::price_is_signal_not_truth_banner` | TB-14 price smoke | 🟢 GREEN | none |

### F.2 Polymarket-era invariants (Stage C P-M2..P-M9 SHIPPED 2026-05-09)

| Invariant | Code surface | Test | Status |
|-----------|--------------|------|--------|
| **CPMM constant-product** `poolY1 * poolN1 >= poolY * poolN` (>= because integer floor; architect §7.7 line 902 verbatim) | sequencer router admission arm (P-M5 + P-M6) | `tests/constitution_router_buy_with_coin.rs::buy_yes_with_coin_matches_formula` + `buy_no_with_coin_matches_symmetric_formula` + `swap_no_for_yes_constant_product_non_decreasing` + `swap_yes_for_no_constant_product_non_decreasing` | 🟢 GREEN |
| **Strict-equality complete-set balanced** (Phase E.3 sym branch; Defect-1 patch) | `assert_complete_set_balanced` symmetric branch | `tests/constitution_economy_strict_equality.rs` (CTF-MIN-SAFE marker lint gate) + P-M6 test 4 `buy_yes_mints_complete_set` | 🟢 GREEN |
| **Atomic 9-step composite rollback** (Defect-2 patch; cfg(debug_assertions) failure injection) | `src/state/sequencer.rs::check_router_test_failure_injection` (cfg(debug_assertions); compiled OUT in --release) | `tests/constitution_router_buy_with_coin.rs::router_atomic_rollback_on_failure` + `router_atomic_rollback_witnessed_at_every_step` (defense-in-depth all 9 steps) | 🟢 GREEN |
| **Pool reserves + LP shares NOT Coin** (architect §7.5 rules 2+3) | EconomicState sub-fields `cpmm_pools_t` + `lp_share_balances_t` separate from `balances_t` | `tests/constitution_cpmm_pool.rs` (P-M4 7 architect-verbatim tests) + P-M5 test 5 `swap_uses_integer_math_no_f64` | 🟢 GREEN |
| **Fail-closed event-state gate** (P-M9 Q10 closure 2026-05-09; `.ok_or(EventNotOpen)?` not `.unwrap_or(Open)`) | event_state precondition reads `task_markets_t` BTreeMap | `tests/constitution_event_state_gate.rs` (10 tests: 6 reject paths × 2 post-resolution states + 3 missing-entry reject paths + 1 positive control) | 🟢 GREEN |
| **E.1 verbatim binding gate** (architect-spec'd struct names + field shapes mechanically enforced) | `tests/constitution_architect_verbatim_struct_binding.rs::architect_verbatim_struct_field_bindings` | catalog of `StructBinding { atom_id, manual_section, struct_name, impl_path, expected_fields, landing_status }` entries | 🟢 GREEN |
| **E.2 atomic-rollback witness gate** (pattern catalog rejects vacuous rollback tests) | `tests/constitution_class4_atomic_rollback_witness.rs::MID_MUTATION_INJECTION_PATTERNS` | static-layer pattern lint + dynamic-layer rollback test | 🟢 GREEN |
| **E.3 strict-equality lint** (`assert_complete_set_balanced` `min(` calls require `// CTF-MIN-SAFE:` marker) | `tests/constitution_economy_strict_equality.rs` | source-side marker gate | 🟢 GREEN |

### F.3 Economy real-economic-activity witnesses (G3 9-task smoke 2026-05-12)

Per `handover/audits/CODEX_G2_TB_G_G3_VERDICT.md:18-36` empirical §G PnL trajectory:

| Agent | Initial balance | Current balance | Realized PnL | Positions | Reputation | Solvency |
|-------|-----------------|------------------|--------------|-----------|------------|----------|
| `tb7-7-sponsor` | 10_000_000 μC | 9_900_000 μC | -100_000 (escrow) | 0 | 0 | solvent |
| `Agent_0` | 1_000_000 μC | 999_000 μC | -1_000 (stake+claim) | 2 (active) | 0 | solvent |
| `MarketMakerBudget` | 5_000_000 μC | 4_900_000 μC | -100_000 (collateral) | 1 (LP) | 0 | solvent |
| 10 other preseed agents | 1_000_000 μC | 1_000_000 μC | 0 | 0 | 0 | solvent |

**3/13 non-flat rows** — real economic activity observed on chain (escrow lock + stake+claim cycle + LP collateral). G3.4 silent-zero-forbidden contract correctly ABSENT (3/13 non-flat).

### F.4 Real economy STRICT-landing gaps (per `feedback_no_workarounds_strict_constitution`)

Ordered by severity:

#### S0 — blockers / freeze triggers

**None.** Every CLAUDE.md §20 freeze trigger (FC1/FC2/FC3 / Tape canonical / Economy conservation / No-fake-accepted / System-tx-not-agent-submittable / Dashboard-regeneratable / Attempt equality) is currently GREEN.

#### S1 — constitutional debt (true 🔴 RED)

**None on economy surface.** (Gardener Agent — Art. III.1 — is the only S1 RED in the entire project; it's not an economy law.)

#### S2 — forward-bound gaps (DEFERRED-FORWARD with concrete owner)

| Gap | Owner / forward TB | Constitution clause | Severity |
|-----|--------------------|---------------------|----------|
| **G3.2 bankruptcy risk-cap admission** | THIS packet (Class-4 §8 pending architect) | architect §G3 SG-G3.3 + SG-G3.4 (bankrupt receives AutopsyCapsule + cannot continue unlimited risk-taking); CLAUDE.md §9 Class-4 | **HIGHEST S2** (only AMBER row in matrix §R) |
| **G2P verifier reputation accumulation** (Gap-A) | G3.2 packet Surface 4 (under Q4 bundle decision) OR sibling G3.5 atom | Art. I.2 (statistical signal): `reputations_t` never mutated by any sequencer arm; statistical signal Art. I.2 thus has degenerate empirical surface (every agent reputation=0 in §G PnL trajectory) | S2 high (Art. I.2 partially landed) |
| **G2P verifier bond return** (Gap-B) | G3.2 packet Surface 4 (under Q3 + Q4) OR sibling G3.5 atom | CLAUDE.md §13 "Only Investment Costs Money" implies bond is investment-with-claim, not permanent debit; current code makes every accepted VerifyTx a permanent debit (OBS_G2P_VERIFY_PEER_REWARD §2.2) | S2 high (economic incentive misaligned with constitution) |
| **PromptCapsule swarm-write at evaluator runtime** | G2P.4 (Class 2-3 autonomous) | Art. III.2 (encapsulation) + Art. III.3 + Art. III.4; CLAUDE.md §4.3 verbatim default policy; OBS_G2P_PROMPT_BODY_OBSERVABILITY 2026-05-12 | S2 medium (gate-level GREEN; runtime emit forward) |
| **WalletBackend trait abstraction** | post-G-Phase or post-G7 (charter §0.66 carry-forward question) | CLAUDE.md §13 "WalletTool is read-only projection" — currently direct balances_t read; future on-chain wallet swap needs trait | S2 medium (forward Class-4 schema-adjacent) |
| **Wilson CI aggregate-report integration** | post-G-Phase or sibling Class-2 atom | Art. I.2 statistical signal: helper landed; aggregate-report wire-up forward | S2 low (real debt; minor) |
| **Boltzmann entropy + payload diversity aggregate-report integration** | post-G-Phase or sibling Class-2 atom | Art. II.2.1 exploration/exploitation: helpers landed; aggregate-report wire-up forward | S2 low (real debt; minor) |
| **`refs/chaintape/cas` strict-Merkle commit-chain redesign** | Stage A3.6 enhancement TB (post-Polymarket; carry-forward) | Art. 0.3 区块链化保留 (Phase E gate: "Merkle root + heldout_sealed_hash 双锁") | S2 low (architectural fidelity; Polymarket runtime unaffected) |
| **Q_t Path B真git substrate (~6-8 weeks)** | constitution Art. 0.4 explicit "Phase E gate 强制 B" | Art. 0.4 verbatim Phase E gate | S4 architectural fidelity (constitution-acknowledged deferral; Path C SHIPPED satisfies current Phase) |
| **Art. 0.2 commits 5-10 explicit tracking** | charter / matrix amendment | Art. 0.2 §修复义务 verbatim 10-commit plan | S4 architectural fidelity (commits 5-10 may be silently satisfied by post-TB-C0 work; needs explicit architect verdict) |

#### S3 — real-load coverage gaps (structural GREEN; smoke partial)

**None remaining** on economy surface. All economy gates have real-load Wave 3 50p binding witnesses (matrix §L all 9 GREEN with Wave-3 evidence). Stage C P-M2..P-M9 added Polymarket-era invariants under real production paths.

---

## §G. Gap Inventory — Consolidated

### G.1 Summary table by severity

| Severity | Count | Gaps |
|----------|-------|------|
| S0 (freeze trigger) | 0 | — |
| S1 (constitutional debt, 🔴 RED) | **1** | Gardener Agent (Art. III.1 §379-380) |
| S2 (forward-bound DEFERRED-FORWARD with owner) | **10** | G3.2 risk-cap admission / Gap-A reputation / Gap-B bond-return / G2P.4 PromptCapsule swarm-write / WalletBackend trait / Wilson CI report integration / Boltzmann diversity report integration / refs/chaintape/cas strict-Merkle / Path B git substrate / Art. 0.2 commits 5-10 tracking |
| S3 (real-load coverage) | 0 | — (all Wave-3 50p bound) |
| S4 (architectural fidelity; constitution-acknowledged deferral) | 2 | Path B (Phase E gate) / Art. 0.2 commits 5-10 (overlap with S2 above) |

### G.2 The single 🔴 RED gap — Gardener Agent (Art. III.1)

**Constitution verbatim** (constitution.md:379-380):

> 顶层白盒需要像清理内存一样，部署后台"园丁 Agent"，定期扫描并屏蔽那些偏离黄金原则的陈旧代码与过期文档，确保系统熵值不会随时间失控。

**Current state** (2026-05-12 audit; verified by `grep -rnE 'gardener_agent|gardener\b|garbage_collect.*agent' src/` returning 0 hits):

- Zero `gardener_agent` / `gardener` / `garbage_collect.*agent` surface in `src/`
- Listed as G-020 in `CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md`
- 📅 Deferred to forward TB charter (post-TB-21) per Pass 2 audit

**Why this is the ONLY 🔴 RED**: every other constitution clause has at least one of: (a) executable test, (b) source-grep witness, (c) Wave-3 50p real-load binding, (d) explicit DEFERRED-FORWARD with concrete owner. Gardener has none.

**Architect decision surface for Gardener**:

| Option | Description | Pros | Cons |
|--------|-------------|------|------|
| **A** Forward TB with concrete owner | Charter `TB-GD` Gardener Agent atom; Class 2-3 (background analyzer; no sequencer admission touch); ship-window: post-G-Phase | Mechanically closes the only RED row | Adds 1 more TB to the queue; competing priority with G3.2 / G4.2 §8 work |
| **B** Architect §10 reclassification | Architect ruling: Gardener is a Phase E+ concept; current pre-Phase-E architecture obviates (the "park as forward" status becomes explicit constitutional decision, not silent debt) | Preserves constitutional integrity without TB cost | Requires explicit ratification document and amendment-log entry (Art. V.3) |
| **C** Reframe Gardener as already-landed | Map Gardener semantic to existing surfaces (e.g., `tests/constitution_closure_3_no_trivial_asserts.rs` CR-C0.1 scanner = "gardener-scope-for-test-vacuity"; Trust Root verification = "gardener-scope-for-load-bearing-bytes") | Zero new code | Requires architect to confirm the analogy holds; risk of being a "凑活" workaround per user verbatim |

**Recommendation**: Option **B** (architect §10 reclassification) — most aligned with `feedback_no_workarounds_strict_constitution` because it puts the deferral on explicit constitutional record. Option A is OK if the architect wants to ship Gardener now; Option C is the "凑活" path and should be rejected.

### G.3 Top-priority S2 gap — G3.2 risk-cap admission (this packet)

Already covered in:
- `handover/directives/2026-05-12_TB_G_G3_2_§8_PACKET.md` (363 lines)
- `handover/audits/G3_2_PACKET_CONSTITUTIONAL_AUDIT_2026-05-12.md` (563 lines)

Status: AWAITING architect Q1..Q6 adjudication + verbatim §8 sign-off.

---

## §H. Forward Path Prioritization

### H.1 Phase-bounded forward atoms (post-G3.2 + G4.2 ship)

| Priority | Atom | Class | Closes gap |
|----------|------|-------|------------|
| 1 | G3.2 §8 (this packet) — risk-cap admission + AutopsyCapsule emit + (optional) Gap-A/B bundle | 4 | matrix §R G3 AMBER → GREEN; closes S2 #1-3 |
| 2 | G4.2 §8 — agent_model_assignment genesis schema | 4 | architect §G4 SG-G4.3..G4.4 |
| 3 | G2P.4 — PromptCapsule swarm-write at evaluator | 2-3 (autonomous under parent §8) | S2 PromptCapsule observability; closes G2P R1 Q1 CHALLENGE |
| 4 | G5.1 / G5.2 / G5.3 — opportunity scheduler + role classifier + §I roles | 2-3 | architect §G5 SG-G5.1..G5.7 |
| 5 | G6.1 / G6.2 / G6.3 — epistemic pricing observe-only | 1-2 | architect §G6 SG-G6.1..G6.6 |
| 6 | G7.1 / G7.2 / G7.3 / G7.4 — structural smoke + mid-tier flag + TB-G+1 stub | 1-2 | architect §G7 SG-G7.1..G7.17 |

### H.2 Phase-Eメ gate forward atoms (constitution-acknowledged deferral; post-G-Phase)

| Atom | Class | Closes gap |
|------|-------|------------|
| Stage A3.6 enhancement — refs/chaintape/cas strict-Merkle commit-chain | 3-4 | Art. 0.3 verbatim Phase E gate |
| Path B真git substrate migration (~6-8 weeks) | 4 STEP_B-massive | Art. 0.4 verbatim Phase E gate |
| Wilson CI / Boltzmann diversity aggregate-report integration | 2 | Art. I.2 + Art. II.2.1 statistical signal report-wire-up |
| WalletBackend trait abstraction | 4 schema-adjacent | CLAUDE.md §13 "WalletTool is read-only projection" forward |

### H.3 Architect-decision-needed atoms (S1 + S4)

| Atom | Decision required |
|------|-------------------|
| **Gardener Agent** | Option A (forward TB) / B (§10 reclassification) / C (reframe as already-landed) |
| **Art. 0.2 commits 5-10 explicit tracking** | Either (a) explicit row-by-row matrix amendment OR (b) architect §10 verdict that commits 5-10 are silently satisfied by post-TB-C0 work |

---

## §I. Audit Verdict + Architect Decision Surface

### I.1 Audit verdict

**Status**: **✅ PROCEED on G3.2 packet ship + parallel G-Phase forward atoms**.

The constitution + 3 FCs + economy are **substantively GREEN at gate level + at real-load Wave 3 50p binding level**. The 2026-05-08 "宪法完整落地" closure brought the last 7 AMBER → GREEN via runtime witnesses per user verbatim "我不要凑活". The 2026-05-09 Stage C Polymarket completion landed economy invariants at production scale. The 2026-05-12 G3 observability layer (G3.1+G3.3+G3.4 SHIPPED with Codex G2 PROCEED 12/12 conviction HIGH) brought §R G3 from 🔴 RED → 🟡 AMBER.

**Single 🔴 RED row remaining**: Gardener Agent (Art. III.1 §379-380). Requires architect §10 decision (Option A / B / C in §G.2 above).

**Single 🟡 AMBER row remaining**: §R G3 (G3.2 packet pending architect §8 sign-off).

All other constitutional surfaces are GREEN with Wave-3 50p real-load binding witnesses.

### I.2 Architect decision surface (consolidated)

| Decision | Document | Priority |
|----------|----------|----------|
| **G3.2 §8 packet Q1..Q6 + sign-off** | `handover/directives/2026-05-12_TB_G_G3_2_§8_PACKET.md` + `handover/audits/G3_2_PACKET_CONSTITUTIONAL_AUDIT_2026-05-12.md` | **HIGHEST** (closes only AMBER row) |
| **Gardener Agent Option A / B / C** | this report §G.2 | HIGH (closes only RED row; constitutional debt clarity) |
| **Art. 0.2 commits 5-10 explicit tracking** | this report §G.1 last S4 row | MEDIUM (architectural-fidelity housekeeping) |
| **WalletBackend trait abstraction §8 timing (during G-Phase vs post-G7)** | charter §0.66 carry-forward | MEDIUM (forward Class-4 schema-adjacent) |
| **G4.2 §8 packet (separate from G3.2 per `feedback_no_batch_class4_signoff`)** | TBD: `handover/directives/2026-05-XX_TB_G_G4_2_§8_PACKET.md` | MEDIUM (next Class-4 atom in queue) |

### I.3 Closure of "宪法完全落地" reading

Per user verbatim 2026-05-08 "我要宪法完整落地" + "我不要凑活" + per `feedback_no_workarounds_strict_constitution`:

**Strict reading state at HEAD `4c56ab7` (2026-05-12)**:
- ✅ FC1 — 15/15 nodes + 6/6 INV all GREEN with real-load witnesses
- ✅ FC2 — 13/13 nodes + 8/8 INV all GREEN with real-load witnesses
- ✅ FC3 — 12/12 nodes + 8/8 INV all GREEN with real-load witnesses (last 7 AMBER → GREEN 2026-05-08 strict closure)
- ✅ Art. 0 — 5/5 clauses GREEN with Wave-3 50p binding; 2 S4 architectural-fidelity items (Path B / commits 5-10) constitution-acknowledged Phase E deferrals
- ✅ Art. I — 6/6 clauses GREEN (1 at helper layer; aggregate-report wire-up forward)
- ✅ Art. II — 3/3 clauses GREEN (1 at helper layer; same forward)
- 🔴 Art. III — 5/6 clauses GREEN + **1 RED (Gardener Agent)** + 1 forward (PromptCapsule swarm-write)
- ✅ Art. IV — 8/8 clauses GREEN
- ✅ Art. V — 5/5 clauses GREEN
- ✅ Economy laws (Art. 0 Laws / CLAUDE.md §13) — 10/10 clauses GREEN at gate + Wave-3 50p + Stage C P-M6 real-load
- ✅ Polymarket-era invariants — 8/8 invariants GREEN at production scale
- 🟡 G-Phase generative arena — 4/7 modules GREEN (G1 / G2 / G2P) + 1 AMBER (G3 pending G3.2) + 3 RED (G4 / G5 / G6 / G7 forward)

**Single load-bearing constitutional debt**: Gardener Agent.

**Single load-bearing AMBER**: G3.2 packet (this session).

All other AMBER status has been mechanically closed via Wave-3 50p binding, Stage A3 multi-ref ChainTape, Stage A3 head_t_witness C1+C2, Stage C P-M2..P-M9 Polymarket invariants, 2026-05-08 strict FC3 closure, and 2026-05-12 G3 observability layer.

---

## §J. Citation Index (re-runnable verification)

### J.1 Constitution clauses cited (constitution.md)

| Clause | Lines | Subject |
|--------|-------|---------|
| Art. 0 | 27-160 | Turing-machine foundationalism + 4 sub-clauses + Laws |
| Art. I | 163-298 | Signal quantification |
| Art. II | 299-358 | Selective broadcast |
| Art. III | 360-426 | Selective shielding |
| Art. IV | 513-661 | Boot |
| Art. V | 664-815 | Meta / separation of powers |

### J.2 Flowchart canonical SHA256 (TRACE_FLOWCHART_MATRIX.md:24-40)

| FC | SHA256 |
|----|--------|
| FC1a | `a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5` |
| FC1b | `b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d` |
| FC2 | `6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333` |
| FC3 | `c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd` |

### J.3 Source files cited

| File | Subject |
|------|---------|
| `src/economy/monetary_invariant.rs:240,287,332,434,469,523` | 6 production economy assertion functions |
| `src/state/sequencer.rs` | sequencer admission arms (4 for G3.2 forward) |
| `src/state/typed_tx.rs:165-207` | RejectionClass enum |
| `src/runtime/agent_pnl.rs:307-317,325-331` | G3.1 solvency classifier + preseed helper |
| `src/runtime/autopsy_capsule.rs:253-329,395-398,172` | TB-15 capsule writer + activation gate + AuditOnly default |
| `src/runtime/wilson_ci.rs` | Wilson CI helper (Art. I.2) |
| `src/runtime/diversity.rs` | Boltzmann entropy + payload diversity helper (Art. II.2.1) |
| `src/runtime/prompt_capsule.rs` | Class-3 PromptCapsule (Art. III.2 default policy) |
| `src/state/head_t_witness.rs` | G-009 HEAD_t C1+C2 (Art. 0.4) |
| `src/bottom_white/ledger/transition_ledger.rs` | Stage A3 multi-ref ChainTape (Art. 0.2 + 0.3) |

### J.4 Test files cited (matrix §J / §L / §M / etc.)

All `tests/constitution_*.rs` files registered in `scripts/run_constitution_gates.sh`. Authoritative count: `402/0/1` per LATEST.md session #45 close.

### J.5 Evidence files cited

| File | Witness |
|------|---------|
| `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/aggregate_verdict.json:409` | `"verdict": "PROCEED"` |
| `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/PERSISTENCE_BINDING_REPORT.json:5-6` | `is_passing=true n_witnessed=4` |
| `handover/evidence/WAVE3_50P_AGGREGATE.json` (referenced; not directly read this audit) | Wave 3 50p aggregate evidence |
| `handover/audits/CODEX_G2_TB_G_G3_VERDICT.md:1-39` | G3 observability layer Codex PROCEED 12/12 conviction HIGH |

### J.6 Re-runnable verification commands (re-confirms this audit)

```bash
# 1. Current AMBER row count (expect 1: §R G3)
awk -F'|' '/^## §[A-Z]/ {section=$0} /^\|/ && !/^\|---/ && !/clause/ {
  for(i=1;i<=NF;i++){if(match($i,/🟢 GREEN|🟡 AMBER|🔴 RED/)){
    status=substr($i,RSTART,RLENGTH); if(status=="🟡 AMBER") print section": "$2" => "status; break}}
}' handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md

# 2. Gardener absence (expect 0 hits)
grep -rnE 'gardener_agent|gardener\b|garbage_collect.*agent' src/

# 3. Constitution gates pass count (expect 402/0/1 per session #45 close)
bash scripts/run_constitution_gates.sh 2>&1 | tail -5

# 4. Trust Root verify (expect PASS at HEAD 4c56ab7)
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo

# 5. Q_t Path B git substrate absence (expect 0 hits per constitution Art. 0.4 Path C SHIPPED)
grep -rE "Repository::|git2::|libgit2|Command git" src/ experiments/minif2f_v4/src/ | grep -v "//\|test"
```

---

**End of constitution + 3-flowchart + economy full-landing alignment audit.**
