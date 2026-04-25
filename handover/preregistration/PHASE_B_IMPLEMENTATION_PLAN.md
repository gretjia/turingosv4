# Phase B Implementation Plan — Kernel Instrumentation + PPUT Accounting

**Phase B window**: days 4-10 of 30-day arc (2026-04-29 → 2026-05-05 wall-clock)
**Authoritative spec**: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 6 Phase B (lines 377-385)
**Gate B exit criterion**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 11 anti-Goodhart conformance tests + 5-layer sealing + 4 content-meta-predicates + 4 lookup-evasion + Trust Root immutability PASS. Heldout never touched.

> **For new sessions**: this plan is self-contained. Read this + PREREG § 5 (definitions) + PREREG § 1.7 + § 1.8 (ArtifactState + Trust Root) + PREREG § 3 + § 3.5 + § 3.5.1 (conformance batteries). All file paths are relative to repo root `/home/zephryj/projects/turingosv4/`.

## Item-by-item plan

### B1 — JSONL schema v2 (proposal-level + run-level)

**What**: extend the per-tx jsonl schema and per-run aggregate schema with the fields needed for Verified PPUT computation + LOO heldout protocol + ArtifactState tracking.

**Files to modify**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — jsonl emit functions (search current code for `serde_json::to_string` and `writeln`)
- (new) `experiments/minif2f_v4/src/jsonl_schema.rs` — versioned schema struct with serde derives

**Per-proposal schema (new fields, on top of current)**:
```rust
struct ProposalRow {
    // existing
    run_id: String,
    problem_id: String,
    agent_id: String,
    role: String,
    branch_id: String,
    proposal_hash: String,
    accepted: bool,
    // new for PPUT v2
    split: String,              // "adaptation" | "meta_validation" | "heldout"
    schema_version: String,     // "v2.0"
    context_hash: String,       // hash of input prompt (for retrieval-equivalence audit)
    predicate_result: i32,      // runtime predicate accept = 1, reject = 0
    ground_truth_result: Option<i32>,  // Lean post-hoc verify: 1 / 0 / null=not-yet-checked
    lean_error_category: Option<String>,
    raw_error_hash: Option<String>,
    rollback_to: Option<String>,    // hash of Q^world snapshot to roll back to
    prompt_tokens: u64,
    completion_tokens: u64,
    tool_tokens: u64,                // length of all tool stdout summed
    total_tokens: u64,               // = prompt + completion + tool
    wall_time_ms: u64,
    start_time: String,              // ISO 8601 UTC
    end_time: String,
    ast_depth: u32,
    peer_agents_in_branch: Vec<String>,
    tool_stdout_hash: Option<String>,    // SHA-256 of concatenated tool stdout
    is_on_golden_path: bool,
    golden_path_id: Option<String>,
    // PPUT-CCL meta-loop attribution (Phase D+ but emit field nullable in B)
    architect_artifact_id: Option<String>,   // if this proposal triggered by ArchitectAI artifact
    auditor_attestation: Option<String>,
}
```

**Per-run aggregate schema (new fields)**:
```rust
struct RunAggregate {
    // existing
    run_id: String,
    problem_id: String,
    solved: bool,
    // new for PPUT v2
    schema_version: String,
    split: String,
    verified: bool,                          // Lean post-hoc PASS
    golden_path_token_count: u64,
    total_run_token_count: u64,              // C_i (sum over all proposals)
    total_wall_time_ms: u64,                 // T_i
    progress: u8,                            // 0 or 1 (Lean ground truth)
    pput_runtime: f64,                       // legacy / runtime accept-based, NEVER for North Star
    pput_verified: f64,                      // Progress / (C_i × T_i / 1000)  [token-second]
    pput_m_verified: f64,                    // 10^6 × pput_verified
    failed_branch_count: u32,
    rollback_count: u32,
    // guardrails
    far: f64,
    err: f64,
    iac: f64,
    cpr: f64,
    // model snapshot per F-2026-04-22-08 (drift defense)
    model_snapshot: String,                  // exact model id + API revision
    git_sha: String,
    binary_sha256: String,
    mode: String,                            // "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous"
}
```

**Acceptance criteria**:
- `cargo test test_jsonl_schema_v2_round_trip` PASS (serialize + deserialize back)
- `cargo test test_pput_verified_zero_when_progress_zero` PASS
- Old jsonl files (Paper 1 era) still readable via `schema_version` discriminant

**Estimated effort**: half day

---

### B2 — C_i full-cost aggregator

**What**: instrument every tx so that `total_tokens` = prompt + completion + tool stdout, summed across ALL proposals in the run (failed + succeeded). Currently the codebase counts per-call tokens but doesn't aggregate failed-branch tokens into the run total.

**Files to modify**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — main loop where proposals are dispatched and rejected
- `experiments/minif2f_v4/src/sdk/tools/*.rs` — every tool's `execute` method must return `tool_stdout: String` (or hash + length)
- (new) `src/cost_aggregator.rs` — `RunCostAccumulator` struct that sums token counts per (run_id, problem_id)

**Sketch**:
```rust
pub struct RunCostAccumulator {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub tool_tokens: u64,
}

impl RunCostAccumulator {
    pub fn record_proposal(&mut self, prompt: &str, completion: &str, tool_calls: &[ToolCall]) {
        self.prompt_tokens += count_tokens(prompt);
        self.completion_tokens += count_tokens(completion);
        for tc in tool_calls {
            self.tool_tokens += count_tokens(&tc.stdout);
        }
    }
    pub fn total(&self) -> u64 { self.prompt_tokens + self.completion_tokens + self.tool_tokens }
}
```

**Acceptance criteria**:
- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
- Manual sanity: spot-check 3 historical Phase 1 runs; recompute C_i, ensure jsonl emits match.

**Estimated effort**: 1 day

---

### B3 — T_i wall-clock instrumentation

**What**: T_i = end_time − start_time, where:
- `start_time` = first read of task statement by any agent in the run
- `end_time` = final ground-truth Lean accept OR external timeout

Currently: T_i is approximated as evaluator wall-clock from main loop start to OMEGA accept; but Lean verify time is sometimes excluded.

**Files to modify**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — record `start_time` at first prompt construction, `end_time` after final Lean call returns
- `experiments/minif2f_v4/src/lean4_oracle.rs` — ensure verify time is bracketed inside the same wall-clock window

**Acceptance criteria**:
- `cargo test test_wall_clock_first_read_to_final_accept` (anti-Goodhart per PREREG § 3): synthetic run with 100ms prompt construction + 5s LLM call + 2s Lean verify → assert `wall_time_ms ≥ 7100`.
- Manual: compare new T_i to legacy `total_run_time` for 3 historical runs; new should be ≥ legacy (wider bracket).

**Estimated effort**: half day

---

### B4 — `pput_verified` vs `pput_runtime` 双字段

**What**: separate two PPUT computations on every run:
- `pput_runtime` = Progress_runtime / (C_i × T_i)  [where Progress_runtime = 1 if runtime/evaluator accepted]
- `pput_verified` = Progress_verified / (C_i × T_i)  [where Progress_verified = 1 if Lean post-hoc verifies the golden path]

For Soft Law mode (which fakes runtime acceptance), `pput_runtime` may inflate but `pput_verified` should drop to 0. This is the H1 detection mechanism.

**Files to modify**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — after recording run, run `lean4_oracle.verify_omega_detailed(golden_path_payload)` independently and emit both fields.
- (new) `experiments/minif2f_v4/src/post_hoc_verifier.rs` — explicit post-hoc verification path that does NOT short-circuit on runtime accept

**Acceptance criteria**:
- `cargo test test_pput_verified_zero_when_lean_rejects` (PREREG § 3 `test_golden_path_requires_ground_truth_acceptance`): synthesize a run that records runtime accept + Lean reject → assert `progress = 0`, `pput_verified = 0.0`.
- For Phase C ablation Soft Law mode: confirm `pput_runtime > 0` but `pput_verified` reflects Lean truth.

**Estimated effort**: half day

---

### B5 — Conformance test battery (anti-Goodhart × 11 + sealing × 5-layer + content × 4 + lookup-evasion × 4)

**What**: implement the test predicates defined in PREREG § 3 + § 3.5 + § 3.5.1. These run as `cargo test` and gate every commit.

**Files to create**:
- `tests/pput_anti_goodhart.rs` — 11 tests per PREREG § 3
- `tests/heldout_operational_sealing.rs` — 5-layer sealing per PREREG § 2.3 (L1-L5)
- `tests/artifact_content_predicates.rs` — 4 content predicates per PREREG § 3.5
- `tests/artifact_lookup_evasion.rs` — 4 lookup-evasion predicates per PREREG § 3.5.1
- `tests/architect_sole_lt_reader.rs` — D4 cognitive isolation conformance
- `tests/auditor_sees_candidate_only.rs` — D4 cognitive isolation conformance
- `tests/mode_flag_binary_purity.rs` — Phase C C5 conformance
- `tests/trust_root_immutability.rs` — Gate B conformance

**Test names (must match PREREG § 3)**:
```
test_all_model_tokens_counted
test_tool_stdout_hash_logged
test_no_hidden_unmetered_generation
test_no_problem_id_hardcode
test_no_metric_file_access_by_agents
test_no_pput_in_agent_prompt
test_golden_path_requires_ground_truth
test_failed_branches_in_total_cost
test_wall_clock_first_read_to_final_accept
test_heldout_ids_inaccessible
test_no_pput_in_agent_prompt   (added 11th, see PREREG § 3 line "ADD: test_no_pput_in_agent_prompt")
```

**§ 3.5 content predicates**:
```
test_docs_contain_no_raw_failed_trace
test_docs_do_not_include_exact_adaptation_solution
test_docs_code_blocks_are_parametric_templates
test_docs_include_scope_and_expiration
```

**§ 3.5.1 lookup-evasion predicates**:
```
test_docs_no_problem_id_keys
test_docs_no_theorem_name_keys
test_docs_rolling_hash_multi_window
test_docs_max_dict_cardinality
```

**Acceptance criteria**:
- `cargo test --tests pput_anti_goodhart` PASS (all 11)
- `cargo test --tests heldout_operational_sealing` PASS (all 5 layers covered)
- `cargo test --tests artifact_content_predicates artifact_lookup_evasion architect_sole_lt_reader auditor_sees_candidate_only mode_flag_binary_purity trust_root_immutability` PASS
- Each test must FAIL when fed a deliberately-violating fixture (test the test).

**Estimated effort**: 1 day

---

### B6 — PPUT-context-leak audit (静态分析 + 运行时门)

**What**: ensure PPUT scalars never enter agent prompt context (PREREG § 3 `test_no_pput_in_agent_prompt`). PPUT is a strong optimization signal; exposing it to agents creates Goodhart attack surface.

**Implementation**:
1. **Static analysis (grep-based)**: scan all prompt-construction code paths in `experiments/minif2f_v4/src/sdk/prompt.rs` and `evaluator.rs` for any reference to `pput_*`, `verified_pput`, `H_VPPUT`, dashboard scalars. Whitelist: dashboard / logging / aggregator code paths only.
2. **Runtime gate**: add a `PromptBuilder::assert_no_metric_leak()` method that scans the final assembled prompt string for the literal substrings `"pput="`, `"PPUT-M"`, `"H-VPPUT"`, `"WBCG"` etc. before sending to LLM.

**Files to modify / create**:
- `experiments/minif2f_v4/src/sdk/prompt.rs` — add metric-leak audit check
- (new) `tests/no_pput_in_agent_prompt.rs` — conformance test (synthesize prompts with PPUT references, assert builder rejects them)

**Acceptance criteria**:
- `cargo test test_no_pput_in_agent_prompt` PASS
- Build-time grep for `pput` in `src/sdk/prompt.rs` returns 0 hits (or only in the assertion code itself)

**Estimated effort**: half day

---

### B7 — Boot freeze + Trust Root immutability

**What**: per PREREG § 1.8, freeze the Trust Root with SHA-256 of each protected file in `genesis_payload.toml [trust_root]`. Boot computes hashes at startup, compares to genesis, aborts on mismatch.

**Trust Root files** (PREREG § 1.8 list):
```
src/kernel.rs
src/wal.rs
src/bus.rs
experiments/minif2f_v4/src/lean4_oracle.rs
constitution.md
cases/C-*.yaml  (glob; track manifest)
genesis_payload.toml [pput_accounting_0] section
handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
```

**Files to modify**:
- (new) `genesis_payload.toml` (might already exist; check first) — add `[pput_accounting_0]` section + `[trust_root]` section
- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
- (new) `tests/trust_root_immutability.rs` — simulate ArchitectAI write attempt to each Trust Root path → assert process panics or returns EPERM (per PREREG § 7 Gate B fallback enforcement)

**`pput_accounting_0` section content**:
```toml
[pput_accounting_0]
schema_version = "1.0"
progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
k_max = 10
n_max = 34

[trust_root]
"src/kernel.rs" = "<sha256>"
"src/wal.rs" = "<sha256>"
"src/bus.rs" = "<sha256>"
"experiments/minif2f_v4/src/lean4_oracle.rs" = "<sha256>"
"constitution.md" = "<sha256>"
"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "<sha256>"
"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "<sha256>"
# cases/* tracked via cases/MANIFEST.sha256 (glob)
```

**Acceptance criteria**:
- `cargo test test_trust_root_immutable_at_boot` PASS
- `cargo test test_trust_root_simulated_write_aborts` PASS
- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`

**Estimated effort**: 1 day

---

### B7-extra — p_0 calibration (data + freeze)

**What**: per PREREG § 5.5, run the calibration protocol to compute `p_0` (baseline regression rate); freeze into `genesis_payload.toml [pput_accounting_0]`.

**Pre-requisite**: B7 + the `--simulate-rollback-at-tx-50` toggle implemented in evaluator.

**Toggle implementation**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — add `--simulate-rollback-at-tx-50` flag. At tx 50, force `Tape::rollback_to(snapshot_at_tx_25)` regardless of state. Then continue normal loop.

**Calibration runs**:
- Run `evaluator --mode full` on adaptation-144 × 2 seeds [31415, 2718] = 288 runs (control)
- Run `evaluator --mode full --simulate-rollback-at-tx-50` on same 288 = treatment
- Total: 576 runs, ~50s each chat oneshot, ~8 wall-hours, ~$3-5 API spend

**Compute p_0**:
- For each (problem, seed): regression_p = 1 iff control SOLVED AND treatment UNSOLVED
- Per-problem regression: max over the 2 seeds (worst case per PREREG § 5.5)
- p_0 = sum_p regression_p / 144

**Sanity gate**: if p_0 > 0.10, ABORT — toggle too aggressive (per PREREG § 5.5 ceiling). Redesign rollback simulation, redo.

**Freeze**:
- Write p_0 value to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
- Compute SHA-256 of the calibration jsonl file → write to `[pput_accounting_0].baseline_regression_jsonl_sha256`
- Add the calibration jsonl path to `[trust_root]`

**Acceptance criteria**:
- 288 control + 288 treatment runs completed; jsonl committed
- p_0 ∈ (0, 0.10]
- Trust Root re-hashed; Boot re-verifies
- Dual-audit packet for Phase B → C transition includes the p_0 result

**Estimated effort**: 1-2 days wall-clock (mostly LLM call time, can run unattended overnight)

---

## Phase B execution order (dependency graph)

```
B1 (schema)  → B2 (cost)  ┐
                          ├→ B4 (verified vs runtime PPUT) → B5 (conformance) → Gate B
B3 (wall-time)  ──────────┘                                 ↓
                                                        B6 (context leak)
B7 (Trust Root + Boot freeze) — depends on B1-B6 metadata to know what to lock
                            ↓
                       B7-extra (p_0 calibration) — runs after toggle code (part of B7)
```

Concretely:
- Day 1 (post-A5): B1 schema + B3 wall-time (parallel, independent)
- Day 2: B2 cost aggregator
- Day 3: B4 verified-vs-runtime separation
- Day 4: B5 conformance battery
- Day 5: B6 context-leak audit + B7 part 1 (genesis_payload.toml structure)
- Day 6: B7 part 2 (Boot integration + immutability tests) + commit toggle for B7-extra
- Day 7: B7-extra p_0 calibration runs (overnight) + Phase B → C audit packet

## Phase B exit checklist (Gate B)

- [ ] All B1-B7 + B7-extra acceptance criteria met
- [ ] `cargo test` passes (full suite)
- [ ] 11 anti-Goodhart conformance tests PASS
- [ ] 5-layer sealing conformance tests PASS
- [ ] 4 doc-content + 4 lookup-evasion meta-predicates PASS (with deliberate-violation fixtures)
- [ ] Trust Root immutability tests PASS
- [ ] `pput_accounting_0` block in `genesis_payload.toml` filled (including p_0 + jsonl hash + Trust Root hashes)
- [ ] On hard-10 adaptation × 1 seed, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?)
- [ ] CHECKPOINT_PHASE_B_2026-05-*.md document with the sanity-run output committed
- [ ] Notepad updated with F-2026-05-*-XX entry summarizing Phase B results
- [ ] LATEST.md updated with Phase B done / Phase C ready

## Open implementation questions (to be resolved during Phase B; not blocking start)

1. **Token counter source of truth**: should we use the LLM API's reported token count (post-hoc, accurate), or pre-call estimation (real-time)? Default: post-hoc API-reported, since C_i is for accounting not budgeting.
2. **`tool_tokens` granularity**: count chars/4 as token approximation, or actually run a tokenizer? Default: chars/4 approximation (consistent with PREREG § 1.2 spirit; precision not load-bearing).
3. **`--mode` flag implementation order**: implement in B5 alongside conformance, or defer to Phase C start? Default: B5 (so the binary purity test can be run as part of Gate B).
4. **`p_0` ceiling 0.10 — what if 0/144 = exactly 0**: this passes the ceiling but means the rollback simulation has no effect, which would invalidate j-RR's role. If observed p_0 = 0, redesign the rollback simulation toggle (probably `--simulate-rollback-at-tx-25` is too late; try tx-10 or per-call corruption).

## Resources & references

- `STEP_B_PROTOCOL.md` — restricted-file change protocol; **most B work is plumbing, not behavior change to bus.rs/kernel.rs/wal.rs (those are Trust Root and only modified for hash registration)**. STEP_B_PROTOCOL applies if any actual behavior change to those files becomes necessary.
- `feedback_smoke_before_batch.md` — must smoke-test before B7-extra calibration batch starts
- `feedback_phased_checkpoint.md` — CHECKPOINT_PHASE_B doc + 7 red-line check + auto-pause at Gate B
- `feedback_dual_audit.md` — Phase B → C transition gets Phase C's dual-audit packet (per PREREG § 6 Phase C C4)
