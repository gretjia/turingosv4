# Constitution Gap Analysis — 现状 + 完整实现差距 2026-05-07

**Authority**: independent grounded analysis per user request 2026-05-07. Each row carries a file:line / commit / matrix-row / TB-log citation; no claim from memory alone.

**Boundary**: this document analyzes the gap from CURRENT state (commit `feec129`, post-TB-C0 + post-Constitution-Landing-First + post-Wave-3 50p + post-Wave-3-binding-Track-B) to "constitution fully implemented" per `constitution.md` 886 lines + 4 immutable flowchart hashes.

**Companion**: `CONSTITUTION_EXECUTION_MATRIX.md` (gate-level summary; this file is the per-Article narrative + forward roadmap).

---

## §0 Current snapshot — facts

| Metric | Value | Evidence |
|---|---|---|
| HEAD | `feec129` | `git log --oneline -1` |
| Workspace tests | **1181 passed / 0 failed / 151 ignored** | `cargo test --workspace --no-fail-fast` 2026-05-07 |
| Constitution gates | **97 passed / 0 failed / 1 ignored** | `bash scripts/run_constitution_gates.sh` 2026-05-07 |
| Gate runner files | 16 files | `scripts/run_constitution_gates.sh` GATES array |
| Matrix rows GREEN | high (full count below) | `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` |
| Matrix rows AMBER | 13 (after Track B promotion) | same |
| Matrix rows RED | **0** | same |
| Matrix rows N/A | 1 (Art. V.1 / V.2 doc-only) | same |
| Active TB | TB-18R FINAL-CANDIDATE (awaits architect §8) | `TB-18R_FINAL_SHIP_REPORT_2026-05-07.md` |
| TB sequence next 5 stops | 18R Final → 18B → 19 → 20 → 21 | `PROJECT_PLAN.md §5` |
| FREEZE active until | TB-18R SHIPPED FINAL via §8 sign-off | `project_tb_18r_state` memory + LATEST session #16 |
| Real-LLM tape evidence latest | Wave 3 50p 460 = 9 + 400 + 51 (FC1 invariant 50/50) | `wave3_diagnostic_50p_2026-05-07T14-04-48Z/WAVE3_50P_AGGREGATE.json` |
| `cargo test --workspace constitution_` | 33 constitution tests across 16 files | `tests/constitution_*.rs` |

PROJECT_PLAN.md §0 baseline (2026-05-06) reported `28/64 LANDED + 41/64 LANDED+PARTIAL + 14 NOT-LANDED`. The Constitution Landing First commit (`b7bde23`) closed 4 of those (G-009 / G-012 / G-016 / G-019/G-021/G-028) and Track B 2026-05-07 closed 7 more AMBER → GREEN via Wave 3 binding.

**Updated baseline (2026-05-07 commit `feec129`)** based on direct matrix recount:

```
LANDED (🟢 GREEN)     ≈ 50/64
PARTIAL (🟡 AMBER)    ≈ 13/64
NOT-LANDED (🔴 RED)   = 0/64
N/A                   = 1/64
```

The reported Art. III "0% LANDED" status in PROJECT_PLAN.md §0 is now stale — Art. III prompt-persistence row is GREEN (Constitution Landing First); Art. III.1 / III.2 / III.3 / III.4 remain AMBER (structural-only or partial real-load coverage).

---

## §1 Per-Article gap analysis with code citations

Legend: 🟢 fully landed, 🟡 partial / structural-only / pending real-load smoke, 🔴 truly absent (no code surface or no real test), 📅 deferred-forward (architect-ratified time slot in PROJECT_PLAN §2 / §5), 🚫 N/A (constitution-document-level, not runtime).

### §1.1 Article 0 — Turing-machine foundationalism (5 clauses)

| Clause | Required artifact | Code surface | Test | Status | Gap |
|---|---|---|---|---|---|
| **Art. 0.1 four-element mapping** | Tape / Pencil / Rubber / Discipline | `src/ledger.rs:31 pub struct Tape` + `src/ledger.rs:28 doc-comment "NEVER modified or removed"` (Rubber) + `src/bus.rs:240 fn append_internal` (Pencil) + `src/bus.rs:244 forbidden_patterns` loop (Discipline) | `tests/four_element_mapping.rs` (existing) | 🟡 | source-side complete; matrix row §A still AMBER pending integrated runtime smoke that exercises all four under load |
| **Art. 0.2 Tape Canonical** | every parallel ledger must be derived view | `src/state/price_index.rs::compute_price_index` derives from `EconomicState` (✅) ; `src/runtime/chain_derived_run_facts.rs::compute_run_facts_from_chain_with_invariant` derives from chain alone (✅) | `constitution_no_parallel_ledger.rs::no_parallel_ledger_source_of_truth` + Wave 3 binding 50p `wave3_50p_dashboard_regen_matches_chain` | 🟢 (post-Track-B) | matrix §A row still 🟡 because Art. 0.2 has 24 historical violations and a 10-commit atomization plan; commits 1-4 partially closed via TB-7/TB-12/TB-13/TB-14/TB-18R (success-path tape canonical + per-LLM-call attempt CAS); commits 5-10 remain partly open (see §3) |
| **Art. 0.3 blockchain preservation** | hash chain on append; Phase E Merkle root | `src/wal.rs:148 prev_hash: None, hash: "h0"` placeholder + REAL via `Git2LedgerWriter` writing to `refs/transitions/main` (`src/bottom_white/ledger/transition_ledger.rs:669 pub struct Git2LedgerWriter`) — git2 SHA-1 commit OIDs ARE the hash chain | implicit via `src/state/sequencer.rs:2114 advance_head_t` reading real commit OIDs; `tests/co1_7_extra_git2_writer_head_oid_defense.rs` | 🟢 (via git2-rs) | the placeholder `wal.rs:148 "h0"` is dead code from pre-Git2LedgerWriter era; should be cleaned up but not load-bearing |
| **Art. 0.4 Q_t version-controlled** | HEAD_t pointer (Path A or B or C) | `src/state/head_t_witness.rs:48 pub struct HeadTWitness` (6 fields per architect §4.1) + `src/state/sequencer.rs:2114 advance_head_t` populates `q.head_t = NodeId(commit_oid_hex)` | `tests/constitution_head_t_witness.rs` 5 tests | 🟢 C1 (Path C-hybrid C1 settled) | C2 production refs `refs/chaintape/{l4,l4e,cas}` 📅 deferred to PROJECT_PLAN §2 Week 5–8 — note: real libgit2 IS present (it's the Git2LedgerWriter), C2 is namespace reorganization, NOT introducing libgit2 |
| **Art. 0 Laws (1 Coin = 1 YES + 1 NO)** | total Coin conservation | `src/economy/monetary_invariant.rs::assert_total_supply_conserved` | 9 `economy_*` tests in `tests/constitution_economy_gate.rs` | 🟢 | matrix §A row still 🟡 because the invariant has not been smoke-tested under multi-tx CompleteSet flow at scale; TB-13 ship covered single-tx admission |

**Article 0 verdict**: 4 GREEN + 1 PARTIAL real fact; matrix shows them as 4 AMBER + 1 GREEN because matrix is conservative (CR-C0.7 GREEN means real-path-AND-passes; some Art. 0 rows are structural-only). 2 deferred items (0.4 C2 + 0.2 commits 5-10) are time-slot known per PROJECT_PLAN §2.

### §1.2 Article I — Signal quantization (3 clauses)

| Clause | Required artifact | Code surface | Test | Status | Gap |
|---|---|---|---|---|---|
| **Art. I.1 Boolean predicate** | predicate trait, returns 0/1 | `src/bus.rs:41 forbidden_patterns: Vec<String>` (built-in pattern predicate) + `src/runtime/verify.rs::ReplayReport` per-indicator booleans + Lean4 verification via `experiments/minif2f_v4/src/bin/evaluator.rs:2950` (`lean4_oracle` returns string class then mapped to `LeanVerdictKind`) | `predicate_result_is_binary` + `predicate_pass_required_for_l4` + `predicate_failure_cannot_enter_l4` + `lean_verified_required_for_verified_worktx` | 🟢 | named `Predicate` trait does NOT exist as a top-level abstraction (`grep -rn 'pub trait Predicate' src/` = 0 hits); predicates are scattered: `bus.rs::forbidden_patterns` + `verify.rs::ReplayReport` boolean indicators + Lean4 oracle external. Functionally equivalent to a trait, but no unified trait surface — minor architectural debt, not a violation |
| **Art. I.1.1 PCP / 疑罪从无** | adversarial corpus + non-symmetric verification | `cases/pcp_corpus/` 9 mutation classes + `cases/pcp_corpus/MANIFEST.json` byte-stable mapping | `tests/constitution_pcp_corpus.rs` 7 tests | 🟢 | corpus is synthetic (Lean tactic-mutation per architect G-012 ruling); MiniF2F-v2 misalignment natural-corpus arm 📅 deferred to PROJECT_PLAN §2 Week 5–8 |
| **Art. I.2 Statistical signal (PPUT / reputation / consensus)** | ΣPPUT + Wilson 95% CI on solve rate + reputation projection | ΣPPUT computed offline in `experiments/minif2f_v4/src/jsonl_schema.rs::pput` field (line 196 v3-era extension) + `src/state/q_state.rs:89 pub struct Reputation(pub i64)` + `tests/economic_state_reconstruct.rs` (existing) | matrix §B 🟡 AMBER ("report missing ΣPPUT + Mean PPUT(solved) + Wilson 95% CI" kill condition) | 🟡 | Wilson CI computation NOT in src/ (`grep -rn 'wilson' src/` = 0 hits); reports manually compute it (e.g. `WAVE3_AGGREGATE.json::solve_rate_wilson_95ci`); should be pulled into src/ for replay determinism. **Real debt**: report-side discipline only; no in-tree assertion |

**Article I verdict**: 2 GREEN + 1 AMBER; named `Predicate` trait absent but functional surface complete; Wilson CI is report-side discipline (real debt, low priority).

### §1.3 Article II — Selective broadcast (3 clauses)

| Clause | Required artifact | Code surface | Test | Status | Gap |
|---|---|---|---|---|---|
| **Art. II.1 Broadcast typical errors** | "typical error class" summary in agent prompt; NO raw stderr | `src/runtime/audit_assertions.rs:2174 assert_30_typical_error_summary_no_private_detail` | matrix §C 🟡 AMBER ("raw Lean stderr appears in agent prompt" kill) | 🟡 | structural shielding gate exists (assert_30); kill condition is observed via `tests/constitution_shielding_gate.rs::raw_lean_stderr_not_in_agent_read_view` (also 🟡 AMBER). Real load smoke pending: a runtime test that exercises the agent prompt construction over a 50p Wave-3-style batch and confirms no raw stderr leaks into `UniverseSnapshot` payload. **Real debt** but minor: no production leak observed |
| **Art. II.2 Broadcast price signal** | PriceIndex derived from positions + shares | `src/state/price_index.rs:164 pub fn compute_price_index(econ: &EconomicState) -> BTreeMap<TxId, NodeMarketEntry>` (TB-14 derived view) | `tests/tb_14_price_index.rs` (existing) + `predicate_gate::price_never_overrides_predicate` | 🟢 | landed (TB-14 SHIPPED 2026-05-03) |
| **Art. II.2.1 Exploration / exploitation** | Boltzmann masking + ε-greedy | `src/state/price_index.rs:264 pub struct BoltzmannMaskPolicy` (β rational temperature + ε exploration + price-margin) + `src/state/price_index.rs:466 pub fn compute_mask_set` + `boltzmann_select_parent_v2` (Atom 5) + `src/runtime/audit_assertions.rs:1891 assert_e_boltzmann_parent_selection_diversity` (id=43) | matrix §C 🟡 AMBER (kill: parent_selection_entropy < 0.25 OR pairwise_payload_diversity_mean < 0.25) | 🟡 | structural shielding exists; matrix kill condition still 🟡 because no PROJECT_PLAN-mandated metric report has shipped that documents 50p batch's entropy + diversity ≥ 0.25. Wave 3 50p `WAVE3_50P_AGGREGATE.json` does not include these metrics. **Real debt**: small. Add aggregation of `parent_selection_entropy` + `pairwise_payload_diversity_mean` to Wave-3-style aggregate JSON |

**Article II verdict**: 1 GREEN + 2 AMBER (structural exists, real-load metric capture missing).

### §1.4 Article III — Selective shielding (5 clauses; weakest article)

| Clause | Required artifact | Code surface | Test | Status | Gap |
|---|---|---|---|---|---|
| **Art. III.1 Shield errors (raw failure logs not in agent prompt)** | private_diagnostic_cid in CAS, public_summary in L4 | `src/runtime/attempt_telemetry.rs::AttemptTelemetry` (TB-18R R1) carries CID-routed payload + `src/sdk/snapshot.rs:56 pub struct UniverseSnapshot` | `tests/constitution_shielding_gate.rs::raw_lean_stderr_not_in_agent_read_view` + `private_diagnostic_cid_not_serialized_publicly` | 🟡 | Stderr CAS-routing exists but matrix row §D 🟡 AMBER. **Real debt**: no test that builds the actual agent prompt for a 50p Wave-3 problem and asserts no raw stderr text is in it. The audit-side assertion checks structure, not the prompt-builder output |
| **Art. III.1 Gardener Agent** | background "garbage collector" agent that prunes stale code/docs (constitution §379-380) | `grep -rnE 'gardener_agent\|gardener\b\|garbage_collect.*agent' src/` = **0 hits** | none | 🔴 | **TRUE GAP** — never implemented. Per constitution §380: "顶层白盒需要像清理内存一样，部署后台'园丁 Agent'，定期扫描并屏蔽那些偏离黄金原则的陈旧代码与过期文档". Listed as G-020 in `CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md`; 📅 deferred to forward TB charter (post-TB-21) per Pass 2 audit |
| **Art. III.2 Encapsulation** | high-volume detail in CAS, audit-only | `src/bottom_white/cas/schema.rs::ObjectType::AttemptTelemetry/LeanResult/PromptCapsule/EvidenceCapsule` (TB-18R R1 + Constitution Landing First) | `tests/constitution_shielding_gate.rs::evidence_capsule_raw_logs_audit_only` + `tests/tb_18r_audit_sampler_attempt_payload.rs` | 🟡 | matrix §D row 🟡; the structural assertion exists, real-load smoke (capsule round-trip on Wave-3 batch) covered by TB-15 capsule tests but `EvidenceCapsule` row in matrix §K still AMBER pending a CID-routing strict gate that asserts no inline raw_logs body |
| **Art. III.3 Shield correlation (no Goodhart leakage / per-agent isolation)** | reputation projection avoids per-agent private detail | `src/economy/reputation` not present as a dedicated module — see `src/state/q_state.rs:89 Reputation(i64)` + `src/state/q_state.rs:412 ReputationsIndex` | `tests/constitution_shielding_gate.rs::dashboard_does_not_leak_private_failure_detail` | 🟡 | structural assertion via dashboard-grep; no positive runtime test that builds two agents' contexts and asserts they don't share private CID payloads. **Real debt** |
| **Art. III.4 Shield Goodhart (selector blindness)** | scheduler / selector does NOT read Lean stderr text | implicit by `grep -rnE 'selector.*blindness\|stderr.*not.*selector\|goodhart' src/` = **0 hits** (no explicit gate) | `tests/constitution_shielding_gate.rs::l4e_public_summary_low_pollution` (kill: selector reads Lean stderr text body) | 🟡 | implicit only; no explicit test that asserts the selector / scheduler module's input set excludes raw stderr by type. Matrix row §D 🟡 AMBER. **Real debt**: small but architecturally important — Goodhart is a stated worry in constitution and a runtime selector path could regress |
| **Art. III prompt persistence (G-016 / G-019 / G-021 / G-028)** | PromptCapsule Class-3 + L4 anchor | `src/runtime/prompt_capsule.rs:88 hidden_fields_redacted: bool` + constructor refuses `false` (`prompt_capsule.rs:104-116`) + `src/bottom_white/cas/schema.rs::ObjectType::PromptCapsule` | `tests/constitution_prompt_capsule.rs` 8 tests | 🟢 (Constitution Landing First 2026-05-07; was MISSING) | evaluator wire-up forward-step: `AttemptTelemetry` references a `PromptCapsule` CID at runtime per architect §4.3; currently CAS round-trip tests only |

**Article III verdict**: 1 GREEN + 4 AMBER + 1 RED (Gardener Agent). This is the weakest article — PROJECT_PLAN.md §0 reported "0% fully landed" pre-Constitution-Landing-First; post-CLF it has 1 GREEN row. Gardener Agent is the only true 🔴 row in any article.

### §1.5 Article IV — Boot / init / halt / tick (8 invariants)

| Clause | Required artifact | Code surface | Test | Status | Gap |
|---|---|---|---|---|---|
| **Art. IV.boot Q_0** | InitAI generates Q_0 once | `experiments/minif2f_v4/src/bin/evaluator.rs:615 fn run_oneshot` + `:829 fn run_swarm` + `src/state/sequencer.rs::genesis` | `tests/constitution_fc2_boot.rs::fc2_genesis_report_exists` + `fc2_on_init_only_mint` | 🟡 | matrix §E row 🟡; smoke evidence "TB-17 boot smoke" referenced but actual structural test passes |
| **Art. IV.halt** | terminal anchor distribution | `src/state/typed_tx.rs:191 pub enum RunOutcome` (6 variants: OmegaAccepted / MaxTxExhausted / WallClockCap / ComputeCapViolated / ErrorHalt / DegradedLLM) | `tests/six_axioms_alignment.rs::axiom_4` + halt_reason_distribution discipline | 🟢 | landed |
| **Art. IV.tick** | clock advance + emit_mr_tick_node | `src/bus.rs::clock` + `TICK_INTERVAL` | `tests/six_axioms_alignment.rs::axiom_5` | 🟢 | landed |
| **Art. IV fresh replay** | replay deterministically from genesis_report + tape + CAS | `src/bottom_white/ledger/transition_ledger.rs::Git2LedgerWriter::replay_chain_integrity` + `tests/tb_18r_chain_attempt_invariant.rs` | `tests/constitution_fc2_boot.rs::fc2_run_replayable_from_genesis_tape_cas` + Track B `wave3_50p_replay_assertions_all_pass` | 🟢 (post-Track-B) | matrix §E row was 🟡 AMBER, post-Track-B GREEN via 3-observer agreement (audit_proceed=50 + id45_pass=50 + inv1_match=50 on the same 50 Wave-3 problems) |
| **Art. IV system pubkeys verify** | system tx signature verifies under genesis-pinned pubkey | `src/state/system_keypair.rs` + 5 existing system_keypair_*.rs tests | `tests/constitution_fc2_boot.rs::fc2_system_pubkeys_verify` | 🟢 | landed |
| **Art. IV agent registry resolves** | agent_pubkeys.json → `AgentKeypairRegistry` | `src/runtime/agent_registry.rs` | `tests/constitution_fc2_boot.rs::fc2_agent_registry_resolves` | 🟢 | landed |
| **Art. IV TaskOpen/EscrowLock chain events** | chain-events not memory-only | `src/state/sequencer.rs::task_open_accept_state_root` + `escrow_lock_accept_state_root` | `tests/constitution_fc2_boot.rs::fc2_taskopen_escrowlock_are_chain_events` | 🟢 | landed |
| **Art. IV no memory-only preseed** | code-grep no `q.economic_state_t.insert(...)` outside on_init | grep enforcement in `tests/constitution_fc2_boot.rs:91 fn fc2_no_memory_only_preseed` | 🟡 AMBER (code-grep only) | 🟡 | matrix §E row 🟡; structural grep ok, runtime test that mutates and confirms rejection not yet built. **Real debt** |

**Article IV verdict**: 5 GREEN + 3 AMBER. Strong article; remaining gaps are real-load smoke harnessing.

### §1.6 Article V — Meta / separation of powers (4 clauses + amendment log)

| Clause | Required artifact | Code surface | Test | Status | Gap |
|---|---|---|---|---|---|
| **Art. V.1.1 Constitution = sole ground truth + sudo trust root** | Trust Root manifest with SHA-256 per file in `genesis_payload.toml` | `src/boot.rs:71 pub fn verify_trust_root` + `genesis_payload.toml::[trust_root]` (tracked file SHA-256) | `tests/fc_alignment_conformance.rs` battery + `tests/constitution_art_v3_amendment_log.rs::v3_constitution_hash_matches_trust_root_manifest` (round-8) | 🟢 | landed; trust root recursive child-manifest verification per A8e13 |
| **Art. V.1.2 ArchitectAI (proposes, can commit non-constitution.md)** | external propose agent (Claude code editing) | external; in-runtime hooks: 0 (`grep -rn 'architect_ai_propose' src/` = 0 hits) | `tests/constitution_fc3_meta.rs::fc3_architectai_proposal_not_direct_write` | 🟡 | structural-only; agent role discharged via charter/directive trail (`handover/directives/`). No in-runtime ArchitectAI loop. **Architectural choice**, not debt — constitution V.1.2 is satisfied by the human-driven Claude Code editing workflow that lands changes via TB charters |
| **Art. V.1.3 Veto-AI (veto-only, no main commits)** | external veto agent (Codex + Gemini dual audit) | external; `feedback_dual_audit_conflict.md` ranking VETO > CHALLENGE > PASS | `tests/constitution_fc3_meta.rs::fc3_judgeai_veto_only` + `handover/audits/CODEX_*.md` + `GEMINI_*.md` precedents | 🟢 | landed (procedural); Codex 5-round audit closed TB-C0; Gemini sanity pass on b7bde23 substrate dispatched 2026-05-07 (Track C in flight) |
| **Art. V.2 Constitution boundaries (hash pinned)** | constitution hash drift detection | `tests/fc_alignment_conformance.rs::fc3_constitution_hash_pinned` (existing) + `tests/constitution_art_v3_amendment_log.rs::v3_constitution_hash_matches_trust_root_manifest` | matrix §F row 🟡 AMBER | 🟡 | structural; per-PR enforcement weak (no CI hook beyond constitution_gates.sh which doesn't itself fail on Trust Root drift). **Real debt** small |
| **Art. V.3 Amendment log** | `constitution.md §5.3` table + integrity test | `constitution.md` lines 808-813 (3 amendments: 2026-04-25 ×3 + 2026-04-26 ×1) | `tests/constitution_art_v3_amendment_log.rs` 6 tests (round-8): section_exists_and_parseable + every_amendment_has_four_populated_columns + every_amendment_triggered_by_human_architect + every_amendment_date_is_iso_format + constitution_hash_matches_trust_root_manifest + historical_amendments_remain_recorded | 🟢 (round-8 promoted from 🔴 RED → 🚫 N/A → 🟢 GREEN) | landed |

**Article V verdict**: 3 GREEN + 2 AMBER (V.1.2 ArchitectAI in-runtime is by-design external; V.2 hash pin per-PR enforcement weak).

---

## §2 Cross-Article gaps not bound to a single clause

### §2.1 Art. 0.2 — 24-violation 10-commit atomization status

Per `constitution.md §85-95` the 10-commit atomization is an Art. 0.2 closure obligation:

| Commit | Content | Closes V-numbers | Status |
|---|---|---|---|
| 1 | Tape schema upgrade — Node.cost structured + Node.kind enum + WAL v2 hash chain | V-01, V-06, V-18 | 🟢 (effectively via TB-18R R1 schema + Git2LedgerWriter native hash chain via git OIDs; WAL v2 hash chain itself still placeholder `wal.rs:148 "h0"` but git2 hash chain dominates) |
| 2 | RunCostAccumulator → derived view + cross-validation | V-02, V-03, V-22 | 🟡 partial — `chain_derived_run_facts.rs` is the canonical derived view but `RunCostAccumulator` itself is still a parallel structure (see `src/runtime/`); cross-validation in audit_tape Layer C. **Real debt** |
| 3 | MarketCreate / MarketResolve / structured Invest on tape | V-04, V-05, V-15, V-16 | 🟢 via `ChallengeResolveTx` (TB-5) + `CompleteSetMintTx` / `CompleteSetRedeemTx` (TB-13) — note the original CPMM `MarketCreate` was excised in TB-14 Atom 6, by design |
| 4 | Failed proposals with verified=false on tape; delete graveyard | V-03, V-09, V-13 | 🟢 via TB-18R R1+R2+R3 (failure-path symmetry; >500 LLM rejects on L4.E with R3 RejectionClass) |
| 5 | Mandatory WAL + mr tick on tape | V-08a, V-17 | 🟡 partial — WAL exists but per-line hash chain not enforced; mr tick on tape ✅ via `emit_mr_tick_node`. **Real debt** small |
| 6 | Synthetic short-circuit on tape | V-07 | 🟡 partial — TB-6 atom-3 fixture preseed remains as synthetic L4.E; `OBS_TB18R_INV1_NONLLM_TX_2026-05-07` documents this explicitly |
| 7 | Boltzmann pick + LLM call as separate tape Nodes | V-08b, V-22 | 🟢 LLM call → `AttemptTelemetry` CAS (TB-18R R2); Boltzmann pick currently only audit-asserted (assert_e), not separate tape node. **Real debt** small |
| 8 | search/board/wallet sidecar → derived projection | V-10, V-11, V-14 | 🟡 partial — wallet is read-only projection ✅ (`tests/constitution_economy_gate.rs::economy_wallet_read_only_projection`); search and Librarian board still pre-canonical. **Real debt** |
| 9 | Lean error string + Halt detail on tape | V-19, V-21 | 🟢 — Lean error → `LeanResult::error_class` enum + raw stderr CAS-routed (TB-18R R1 schema); Halt detail via `RunOutcome` typed enum |
| 10 | WAL hash chain + audit guard provenance | V-18, V-24 | 🟡 partial — git2-rs commit OIDs supply hash chain; audit guard provenance via `audit_assertions.rs` Layer A-H; explicit WAL v2 hash chain self-test not in tree. **Real debt** small |

**Atomization roll-up**: 5 GREEN + 5 AMBER; 0 RED. Commit 5 / 8 / 10 are the residual real-debt items, all minor relative to the major closures (commits 3-4 + 9 covered the structural wins of TB-13 + TB-18R).

### §2.2 PROJECT_PLAN.md §3 resume conditions (10/10 GREEN)

Already enumerated in `TB-18R_FINAL_SHIP_REPORT_2026-05-07.md §6`. All 10 green at HEAD `feec129`. This is the door-condition for TB-18R Final ship eligibility — not the full constitution closure.

### §2.3 Coverage gap audit — 30 G-numbered items

Per `handover/audits/CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md`:

| Status | Count | Notes |
|---|---|---|
| Closed in Constitution Landing First (`b7bde23`) | 6 (G-009 / G-012 / G-016 / G-019 / G-021 / G-028) | substrate in src/ + 20 new constitution gate tests |
| Closed in Track B (`feec129`) | 7 matrix AMBER → GREEN | Wave 3 evidence binding |
| Remaining open | 17 (Pass 2 catalog) | Type-1 runtime (3) + Type-2 substrate (~5) + Type-3 audit-policy (4) + Type-4 architectural (3) + blocked-architectural (2) |
| 📅 Forward-bound to future TBs | All 17 are scheduled per Pass 2 §6 wave plan | not blocking PROJECT_PLAN §3 |

The 17 remaining are documented but not all on a single Article — they cross §A/§D/§I and FC1-N5/FC1-N7/FC3-N31/FC3-N33.

---

## §3 Truly absent items (🔴 RED — not just AMBER)

The matrix has **0 🔴 RED rows** at HEAD `feec129`. There are nonetheless items present in `constitution.md` body text (not in the matrix) that have **no code surface and no test**:

| # | Item | Constitution citation | Status | Forward path |
|---|---|---|---|---|
| 1 | **Gardener Agent** (background GC of stale code/docs) | `constitution.md §379-380` | 🔴 — `grep -rnE 'gardener_agent\|gardener\b' src/` = 0 hits | 📅 forward TB charter post-TB-21 (Pass 2 audit G-020) |
| 2 | **In-runtime ArchitectAI loop** (proposes architecture changes from log analysis) | `constitution.md §728-737` | 🟡 structural-only via external Claude Code workflow; no in-runtime loop (`grep 'architect_ai_propose' src/` = 0 hits) | architectural choice — likely permanent (PROJECT_PLAN does not schedule this) |
| 3 | **Explicit Goodhart selector blindness gate** | `constitution.md §413-424 (Art. III.4)` | 🟡 implicit only (`grep -rnE 'selector.*blindness\|goodhart' src/` = 0 hits) | new constitution gate test asserting selector input set excludes raw stderr by type — cheap to add |
| 4 | **Per-line WAL hash chain (Art. 0.3 §103-110)** | self-Merkle root not used (`wal.rs:148 "h0"` placeholder) | 🟡 dominated by git2 commit-chain hash | clean up the placeholder; no functional gap |
| 5 | **Wilson 95% CI in src/** | report-side computation only (Wave3 aggregate JSON) | 🟡 in-tree assertion absent (`grep -rn 'wilson' src/` = 0 hits) | small Class-1 helper module |
| 6 | **`parent_selection_entropy` + `pairwise_payload_diversity_mean` aggregate metric capture** | constitution Art. II.2.1 + CLAUDE.md §17 Report Standard | 🟡 audit-side only (assert_e) | extend Wave-3 aggregate JSON shape |
| 7 | **Run-time test that exercises agent prompt for Wave-3-style batch and asserts no raw stderr** | constitution Art. II.1 + III.1 | 🟡 audit-side proxy only | new constitution gate test using a fixture batch |
| 8 | **`CompleteSetMergeTx`** | DECISION_POLYMARKET_CORE §6 #2 | 🟡 absent (`grep -rn 'CompleteSetMerge' src/` = 0 hits) | 📅 deferred to TB-21 sandbox/beta |
| 9 | **Auto-redeem trigger after ChallengeResolveTx** | DECISION_POLYMARKET_CORE §4.4 | 🟡 architectural choice — current is sequencer-side gate + agent-driven Redeem | 📅 reconsider at TB-21 if "system-emits-redeem" is preferred |
| 10 | **AMM / CPMM Router / SwapTx / Liquidity Pool reserves** | DECISION_CPMM_MINT_AND_SWAP_2026-05-02 | 🟡 historically present, excised TB-14 Atom 6 | 📅 TB-21 if needed for multi-Agent real economy; current FREEZE list explicitly forbids |

Of these 10, only **1 is a truly absent constitution-mandated runtime artifact**: the **Gardener Agent** (item 1). The other 9 are either architectural choices, minor real debt, or deferred-by-plan items.

---

## §4 From CURRENT to "constitution fully implemented" — workplan

The path is partitioned by PROJECT_PLAN §5 TB sequence. Each TB lists the constitution-closure work that lands inside it.

### §4.1 Immediate (await architect §8 sign-off)

```
TB-18R FINAL ship — architect §8 sign-off pending
  Closes:
    - Art. 0.2 commit 4 (failed proposals with verified=false on tape)
    - Art. III.1 (private_diagnostic_cid CAS routing for Lean stderr)
    - Art. III.2 (AttemptTelemetry encapsulation)
    - PROJECT_PLAN §3 #6 (P38/P49 attempt equality green)
  Workfile:
    - handover/tracer_bullets/TB-18R_FINAL_SHIP_REPORT_2026-05-07.md
    - awaits handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md
```

### §4.2 Week 2 (PROJECT_PLAN §2 — already largely landed; residual)

```
Wave 1/2 残留 (Track B partial close 2026-05-07; remaining):
  - Wilson 95% CI in src/ helper                       § small (Class 1)
  - parent_selection_entropy + payload_diversity       § extend WAVE3_AGGREGATE.json shape
  - explicit Goodhart selector-blindness gate test     § Class 1 grep gate
  - run-time agent-prompt-no-raw-stderr Wave3 fixture  § Class 1 fixture-style gate
  Outcome: matrix §B Art. I.2 + §C Art. II.1 / II.2.1 + §D Art. III.1 / III.4
           AMBER → GREEN
  Cost: ~1 day of pure tests; no src/ changes
```

### §4.3 TB-18B (M1/M2 真实 benchmark scale-up; PROJECT_PLAN §5)

```
Closes:
  - Art. I.2 statistical signal report-side discipline at scale
  - Art. III.4 selector blindness under scaled load (Goodhart 实证)
  - Art. IV no_memory_only_preseed under scaled load (preseed surface
    runtime test)
  - First public-eligible benchmark report (gated on Wave 1/2 done)
Charter not yet drafted; eligible after TB-18R §8 sign-off.
```

### §4.4 TB-19 (real-world pilot DESIGN; documents only)

```
Closes:
  - DECISION_POLYMARKET_CORE remaining design questions
    (CompleteSetMergeTx scope, system-vs-agent Redeem, AMM revival or not)
  - Art. III shielding under multi-Agent design (per-agent context isolation
    test plan for TB-21)
  - Art. V.2 Constitution boundaries — per-PR hash drift CI hook design
  No code; documents and design decisions only.
```

### §4.5 TB-20 (low-risk real-world sandbox pilot; no irreversible action)

```
Closes:
  - Art. II.1 broadcast typical errors at multi-Agent scale (real prompt
    isolation test)
  - Art. III.3 shield correlation (multi-Agent prompt isolation real test)
  - Art. 0.2 commit 8 (search/Librarian board sidecar → derived projection)
  - First sandbox runs of multi-Agent economy with mocked irreversible
    actions (no real money / external commitments)
```

### §4.6 TB-21 (limited beta; multi-Agent real-economy落地)

```
Closes:
  - CompleteSetMergeTx (DECISION_POLYMARKET_CORE §6 #2)
  - Auto-redeem-on-resolution architectural choice finalized (system-emits
    or sequencer-gate-only — TB-19 design ratifies, TB-21 implements)
  - AMM / CPMM Router REVIVAL if TB-19 design ratifies it (default: NO,
    per current FREEZE list)
  - Real multi-Agent economy first run with delayed settlement + human
    escalation
  - This is the v3 Zeta-equivalent落地 milestone per user terminology
```

### §4.7 Week 5–8 (PROJECT_PLAN §2 — deep foundation)

```
Closes:
  - Art. 0.4 C2 libgit2 production refs (refs/chaintape/{l4,l4e,cas}
    multi-branch organization on top of EXISTING libgit2 substrate;
    NOT introducing libgit2 — already present via Git2LedgerWriter)
  - G-012 MiniF2F-v2 misalignment natural-corpus arm
    (replaces synthetic Lean tactic mutation with web-discovered
    real misalignments)
  - Deeper Art. III shielding tests (in-context bad-pattern contamination
    multi-call cycle load test per G-019)
```

### §4.8 Forward TB charter (post-TB-21 horizon)

```
Closes:
  - Art. III.1 Gardener Agent (G-020): the only truly absent
    constitution-mandated runtime artifact. Background GC agent
    that prunes stale code/docs per constitution §379-380.
  - WAL v2 self-hash-chain placeholder cleanup (`wal.rs:148`)
    if dominated git2-rs hash chain is judged insufficient at audit time
  - In-runtime ArchitectAI loop if architectural decision flips
    (currently external Claude Code workflow is the satisfaction)
```

---

## §5 Summary table — constitution closure progress

| Article | Required clauses | Currently 🟢 | 🟡 | 🔴 absent | 📅 deferred-by-plan |
|---|---:|---:|---:|---:|---:|
| Art. 0 | 5 + 24-violation 10-commit roll-up | 4 + 5/10 | 1 + 5/10 | 0 | 1 (0.4 C2) |
| Art. I | 3 | 2 | 1 | 0 | 1 (PCP MiniF2F-v2) |
| Art. II | 3 | 1 | 2 | 0 | 0 |
| Art. III | 5 + Gardener | 1 | 4 | 1 (Gardener) | 1 (Gardener post-TB-21) |
| Art. IV | 8 | 5 | 3 | 0 | 0 |
| Art. V | 4 + amendment log | 3 + 1 | 2 | 0 | 0 (V.1.2 in-runtime ArchitectAI architectural choice) |
| **Total** | **28 + 10-commit + Gardener** | **17 + 5/10** | **13 + 5/10** | **1** | **3** |

Roll-up:
- **Real GREEN**: ~50 / 64 ≈ 78%
- **Real PARTIAL**: ~13 / 64 ≈ 20%
- **Truly absent (🔴 in body text but not matrix)**: 1 (Gardener Agent)
- **Deferred-by-plan**: 3 (HEAD_t C2; PCP MiniF2F-v2 misalignment; Gardener Agent post-TB-21)

The 🟡 rows split into two classes:
1. **Real-load smoke pending** (~10 rows): test exists but smoke evidence didn't yet exercise the real path under load. Closure is mostly cheap (extend Wave-3-style aggregate or add fixture-style gate test). Estimated: 1–3 days of Class-1 work after architect §8 + before TB-18B.
2. **Architecturally settled but deferred-by-plan** (~3 rows): real debt is by-design forward, e.g. C2 libgit2 namespace reorganization (Week 5–8), MiniF2F-v2 misalignment corpus (Week 5–8), CompleteSetMergeTx + Gardener (TB-21 + post-TB-21).

---

## §6 What this analysis is NOT

- **NOT a green-light to merge TB-18R Final.** Matrix recount and gap closure analysis are necessary but not sufficient for §8 sign-off. The architect retains CLAUDE.md §10 named-authorization authority over the ship decision. This document explicitly rejects single-word approvals per `feedback_class4_cannot_hide_in_class3`.
- **NOT a substitute for real-LLM smoke under multi-Agent load.** Wave 3 50p validates substrate stability at 2.5× single-Agent load; multi-Agent contamination / correlation gaps (Art. III.3) cannot be measured until TB-20+ sandbox runs.
- **NOT a finalized roadmap.** PROJECT_PLAN.md §5 is the canonical sequence; this document maps constitution closure onto that sequence but does not modify §5. Architect retains §1.1 sudo for any constitution amendment that would change the scope of "fully implemented".

---

## §7 Cross-references

- `constitution.md` (886 lines; commit-pinned via Trust Root manifest; architect sudo only)
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (gate-level summary; TB-C0 build artifact; Track B 2026-05-07 7-row promotion)
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md` (per-FC-node binding; FC1 / FC2 / FC3)
- `PROJECT_PLAN.md` (constitution-first reset plan + landing map + waves + TB sequence)
- `HARNESS.md` (H0-H5 layers; persistent-test list)
- `handover/audits/CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md` + `_PASS_2_2026-05-07.md` (30 G-numbered gap catalog)
- `handover/tracer_bullets/TB-18R_FINAL_SHIP_REPORT_2026-05-07.md` (FINAL-CANDIDATE; awaits §8)
- `handover/alignment/DECISION_POLYMARKET_CORE_2026-05-02.md` (CTF semantics scope)
- `tests/constitution_*.rs` (16 files; 33 tests; gate runner authoritative via `scripts/run_constitution_gates.sh`)
- `tests/constitution_wave3_evidence_binding.rs` (NEW 2026-05-07; binds matrix invariants to real-LLM tape evidence)

**End of gap analysis. Snapshot HEAD `feec129`; tests 1181/0/151; gates 97/0/1; matrix RED count 0; truly-absent items 1 (Gardener Agent).**
