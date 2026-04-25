# TuringOS v4 — Handover State
**Updated**: 2026-04-25 (B7-extra round-4 audit in flight; batch pending PASS/PASS)
**Session Summary**: B7 (Trust Root + Boot freeze) → 用户 atomic-alignment critique (3 flowcharts) → B7 alignment fix → B7-extra rollback toggle + calibration scripts → dual audit round-1 VETO/VETO → 13-fix landing → simplifier pass → **constitution amendment (sudo)**: V.1.1 sudo scope + V.1.2 ArchitectAI commit authority + V.1.3 JudgeAI→Veto-AI + V.3 amendment log → re-audit round 2 VETO/PASS (Codex caught self-inflicted regression: Q7.b silently absorbed TRUST_ROOT_TAMPERED panics) → round-2 fix → re-audit round 3 CHALLENGE/CHALLENGE (problem_file_missing absorption + boot preflight `||true` exit-discard + EXIT=0+empty PPUT_RESULT non-exhaustive) → round-3 fix (commit `d0d474e`) → **round-4 audit in flight**. **187/187 cargo test PASS** + 20 ignored. Trust Root manifest **20 files** (was 15 — added main.rs / Cargo.lock / runner.sh / compute_p0.py per audit). User authorized auto-research overnight to PROCEED on PASS/PASS.

> **新 session 入口**: 读这个文件 + `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` § B7-extra + `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (claim-7 ground-truth feedback findings C+D for Phase D) + `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (findings A+B abstraction depth) + `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` § F-2026-04-25-04..-08 (this session's findings)。这 5 个文件足以无 context 接手。

## Current State

### Active research arc
**PPUT-driven Capability Compilation Loop (CCL)** — 30-day arc 2026-04-26 → 2026-05-26.
- North Star: Held-out Verified PPUT (H-VPPUT) on heldout-54
- Success criterion: WBCG_PPUT > 0 (≥1 Certified user-space artifact)
- Caps: 30 wall-clock days + USD 500 API budget (硬停)
- Backbone: `deepseek-v4-flash` thinking-off (Phase B+C); 异构 LLM at Phase D (v4-flash thinking-on + Gemini 2.5 Pro)

### Phase A — COMPLETE (Phase B 待启动)
- **A1 ✅** PREREG drafted, 4 rounds revised — `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` (922 行 round 4)
- **A2 ✅** 60/20/20 split frozen — adaptation 144 / meta_val 46 / heldout 54; sealed hash `51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b`
- **A3 ✅** Notepad pivoted (F-2026-04-25-02 entry)
- **A4 ✅** Dual external audit PASS/PASS (Codex + Gemini, round 4)
- **A5 ✅** Commit gate cleared — commit `913255d`

### Phase B — IN PROGRESS (days 4-10 of 30-day arc)
Detailed plan: `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md`
Items (all expanded with file paths + acceptance criteria in plan doc):
- **B1 ✅ DONE** JSONL schema v2 (proposal + run-level) — `experiments/minif2f_v4/src/jsonl_schema.rs`; 3 acceptance tests green; legacy `PputResult` shape readable via `RunRecord::from_json` schema_version dispatcher
- **B2 ✅ DONE** C_i 全成本聚合器 — `experiments/minif2f_v4/src/cost_aggregator.rs`; `RunCostAccumulator` records prompt+completion+tool_stdout across all proposals (winning + failed); `test_failed_branches_counted_in_total_cost` PASS; `GenerateResponse.prompt_tokens` exposed for post-hoc API counts
- **B3 ✅ DONE** T_i wall-clock — `experiments/minif2f_v4/src/wall_clock.rs`; `RunWallClock.mark_first_read` (idempotent) + `mark_final_accept` (last-call-wins for B4 post-hoc); `test_wall_clock_first_read_to_final_accept` PASS at strict ≥7100ms (mid-term P0-C fix moved mark_first_read BEFORE prompt construction)
- **B4 ✅ DONE** `pput_verified` vs `pput_runtime` — `experiments/minif2f_v4/src/post_hoc_verifier.rs`; `compute_progress_runtime` / `compute_progress_verified` / `verify_post_hoc`; `make_pput(runtime_accepted, post_hoc_verified, ...)` caller declares both legs explicitly (mid-term P0-A fix); `test_pput_verified_zero_when_lean_rejects` PASS
- **B5 ✅ DONE** Conformance battery + deferred P0s closed:
  - P0-B (schema v2 emit alignment): PputResult now carries every B1 RunAggregate v2 field as non-Optional; emitted rows dispatch via `RunRecord::V2`; `test_emit_dispatches_as_v2` + `test_emit_soft_law_divergence_signal` PASS
  - P0-D (hybrid_v1): disabled with deprecation message — was dropping failed-leg C_i via `..r2` spread
  - P0-E (flip saturation): replaced with `assert!`; `test_flip_underflow_panics` covers
  - 10/10 anti-Goodhart conformance (PREREG § 3): `experiments/minif2f_v4/tests/pput_anti_goodhart.rs` — all_model_tokens_counted / tool_stdout_hash_logged / no_hidden_unmetered_generation / no_problem_id_hardcode / no_metric_file_access_by_agents / no_pput_in_agent_prompt / golden_path_requires_ground_truth / failed_branches_in_total_cost / wall_clock_first_read_to_final_accept / heldout_ids_inaccessible
  - 5/5 heldout operational sealing (PREREG § 2.3 L1-L5): `experiments/minif2f_v4/tests/heldout_operational_sealing.rs` — file-path read isolation / agent prompt context blacklist / tool-call hash-invocation gate / hash + seed substring co-occurrence / source-pool enumeration block
  - 24 Phase C/D/B7 stubs scaffolded (`#[ignore]` with contract docs): artifact_content_predicates (4) + artifact_lookup_evasion (4) + architect_sole_lt_reader (3) + auditor_sees_candidate_only (3) + mode_flag_binary_purity (6) + trust_root_immutability (4)
- **B6 ✅ DONE** PPUT-context-leak runtime gate — `src/sdk/prompt_guard.rs` (separate module so prompt.rs stays pure for B5 static-grep): `assert_no_metric_leak(prompt)` panics with `PPUT_CONTEXT_LEAK_DETECTED` on any of 8 forbidden substrings (case-insensitive); wired before every `client.generate` call site in evaluator (oneshot + swarm). 10 unit tests (clean prompt + 9 leak fixtures including substring/case-insensitive/middle-of-text variants). Static side already covered by B5 `test_no_pput_in_agent_prompt`.
- **B7 ✅ DONE** Trust Root + Boot freeze:
  - `genesis_payload.toml` (new): `[pput_accounting_0]` (PREREG § 1.8 invariants — schema_version, progress/cost/time defs, verified_predicate, heldout_sealed_hash, source_pool_sha256, k_max=10, n_max=34, baseline_regression_rate placeholder); `[trust_root]` (15 SHA-256 entries — independently re-derived: PREREG § 1.8 base 8 + audit accounting 6 + B6 prompt_guard 1)
  - `cases/MANIFEST.sha256` (new): 45-entry sorted SHA-256 manifest of `cases/C-*.yaml`, hashed-once into Trust Root as proxy for the case-law glob
  - `src/boot.rs` (new, +pub in lib.rs): `verify_trust_root(repo_root)` parses [trust_root] section (hand-rolled minimal TOML parser, no new dep — compression principle), recomputes SHA-256 per path, returns `TrustRootError::Tampered{path,expected,actual}` on mismatch; 6 unit tests (parse/blank/comment/missing-section/intact-repo/tempdir-tamper)
  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
  - `experiments/minif2f_v4/tests/trust_root_immutability.rs`: 4 `#[ignore]` stubs unsealed → 4 PASS (immutable_at_boot / simulated_write_aborts / manifest_includes_b2_b4_files / pput_accounting_0_section_present); manifest test enforces the union list (PREREG § 1.8 base + audit add + B6) — any reduction breaks the test
  - **181/181 workspace test PASS** (171 pre-B7 + 6 boot unit + 4 unsealed)
- **B7-extra ⚙ IN PROGRESS** rollback toggle landed (commit `973a9fd`) + calibration runner/estimator (commit `b0ae03e`) + smoke probe running (1 problem × 4 runs):
  - `experiments/minif2f_v4/src/rollback_sim.rs`: `ROLLBACK_TX_THRESHOLD = 50` (PREREG-frozen), `ROLLBACK_ENV_VAR = "SIMULATE_ROLLBACK_AT_TX_50"`, `should_simulate_rollback(tx, enabled)` — 6 unit tests
  - evaluator.rs run_swarm reads toggle, short-circuits at tx == 50 to existing max-tx exhaustion exit (FC2-N22 HALT via MaxTxExhausted, no new variant)
  - `handover/preregistration/scripts/run_p0_calibration.sh`: iterates adaptation-144 × seeds [31415, 2718] × {control, treatment} = 576 runs; `--smoke` flag = 4-run probe
  - `handover/preregistration/scripts/compute_p0.py`: control/treatment pair → regression_p_seed → max-over-seeds → p_0; PREREG § 5.5 ceiling = 0.10
  - **Smoke verified 2026-04-25**: easy problem mathd_algebra_107 (4 runs, 39s) — infra + jsonl V2 + calibration tags ✓; hard problem aime_1983_p2 with toggle ON (8.5 min) — tx_count=50 + synthetic_short_circuit=true + stderr "[rollback_sim] firing at tx=50" ✓. Field cost-asymmetry doc-comment warns downstream PPUT analysis to honor flag.
  - **Next**: user GO → 576-run batch (~$3-5, ~8h overnight) → compute_p0.py → write p_0 to genesis_payload.toml [pput_accounting_0] → recompute Trust Root + commit jsonl into manifest → Gate B dual-audit Phase B → Phase C transition
- **B7-alignment ✅ DONE** (commit `0cc48bc`) — TRACE_MATRIX v1 (FC3-N34 promoted ✅, B7-extra rows added), src/boot.rs + src/main.rs FC backlinks, OBS_BOOT_FAIL_NOT_HALT (boot panic ≠ FC2-N22, closer to FC3-E14)

### Active background processes
- 无运行中实验 (Phase A 双审已全部完成)
- Codex CLI broker (legacy `pid 348391` → phase-8a-snapshot worktree) — Paper 1 残留, 与 PPUT-CCL 不相关

## What's broken / incomplete

### PPUT-CCL Phase B — to-do (after B7 close)
- B7-extra (p_0 calibration) 未跑：576 runs (288 control + 288 treatment) on adaptation-144 × seeds [31415, 2718], `--simulate-rollback-at-tx-50` toggle 待加；p_0 ∈ (0, 0.10] sanity gate；冻结进 `[pput_accounting_0].baseline_regression_rate` + `.baseline_regression_jsonl_sha256` + 把 jsonl 加入 [trust_root]
- 20 Phase C/D conformance stubs `#[ignore]` 待对应 phase 解封 (artifact_content 4 / lookup_evasion 4 / architect_sole_lt 3 / auditor_sees_candidate 3 / mode_flag_binary_purity 6) — B7 解封了 trust_root_immutability 4 个
- `--mode` flag 未在 evaluator binary 实现 (Phase C C5 工作)
- Trust Root 自身不自哈希 (chicken-and-egg)：`genesis_payload.toml` 自身 tamper 不会被 Boot 检测；语义锚点 = `[pput_accounting_0]` 字段值；如要更强保证，未来可在编译时把 [trust_root] 哈希常量 inline 进 binary（Phase C+ 议题，非 Gate B 阻塞）

### Mid-term dual audit (2026-04-25) deferred items — 必须 B5 起步先解决
binding checklist: `handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md`
- **P0-B**: schema v2 emit alignment — evaluator emit 仍是 legacy `PputResult` 而非 v2 `RunAggregate` (no `schema_version`, missing `progress: u8` / `run_id` / `split` / `mode` / model_snapshot 等); B1 dispatcher 把新行误判为 Legacy
- **P0-D**: hybrid_v1 condition `..r2` field-spread 丢弃失败 oneshot 的 C_i (Codex 发现)
- **P0-E**: `RunCostAccumulator::flip_last_failed_to_accepted` 静默饱和 — 应改为 `assert!` 暴露 over-flip wiring bug

### B1 evaluator emit gap (still open after B2-B4 mid-term audit)
- B1 的 `RunAggregate` v2 schema 仍未在任何 emit path 上线；B2-B4 加的字段都打在 legacy `PputResult` 上。P0-B 处理这个差距 — B5 第一步。

### Paper 1 — separate stack
- arXiv 投稿延后 (tag `paper1-v2.1.1` 待 LaTeX 转换 + 元数据)
- 不 gate PPUT-CCL arc; 用户可任意时间回头处理

## Open Questions for User
**全部 D1-D4 已 resolved 2026-04-26 default 接受** — 见 `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` Resolved 区。本 session 关闭；下一 session 在 Phase B B5 起步（先 pickup deferred P0-B/D/E，再写 conformance battery）。

## Next session — first action

> **Why session paused at B7/B7-extra boundary**: B7-extra needs the `--simulate-rollback-at-tx-50` toggle in evaluator binary + 576 runs overnight (~$3-5 API spend, ~8 wall-hours). Toggle is a small change but the runs are real money — let user explicitly green-light before kicking off. Also C-035 still applies: dual-audit Phase B → C transition will read this LATEST + B7 commit + p_0 jsonl, and a fresh session writes that audit packet cleanly.

1. Read `LATEST.md` (this file) + `PHASE_B_IMPLEMENTATION_PLAN.md` § B7-extra + smoke `cargo test --workspace` (baseline = **181/181 parallel green** + 20 ignored stubs)
2. **Confirm with user** before starting 576-run calibration (cost gate)
3. Implement `--simulate-rollback-at-tx-50` toggle in `experiments/minif2f_v4/src/bin/evaluator.rs` (per PHASE_B § B7-extra) — small change, can land before the runs
4. Run B7-extra: 288 control + 288 treatment on adaptation-144 × seeds [31415, 2718]; compute p_0 = sum_p max_seed(SOLVED_control AND UNSOLVED_treatment) / 144; sanity gate p_0 ∈ (0, 0.10]
5. Freeze: write p_0 to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`; SHA-256 the calibration jsonl → `.baseline_regression_jsonl_sha256`; add jsonl path to `[trust_root]`; recompute every Trust Root hash (genesis itself changed); commit
6. Then Gate B exit: dual-audit Phase B → Phase C transition packet (Codex + Gemini)

## Mid-term audit (2026-04-25) summary
- Codex (274s, 67K char prompt) + Gemini (62s, 67K char prompt): both **CHALLENGE**, high conviction
- Convergent P0s on architectural fragility (Phase-C-safety) + schema v2 drift; Codex-only on first-read placement, hybrid_v1, flip saturation
- **Fixed in B2-B4 commit**: P0-A (make_pput refactored — caller MUST declare runtime + post_hoc legs), P0-C (mark_first_read moved before prompt construction; conformance test back to ≥7100ms strict)
- **Deferred to B5**: P0-B/D/E (binding checklist `B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md`)
- **Compute spent on mid-term audit**: ~$3-5 (~67K char prompt × 2). Phase B audit budget remaining for B5/B6/B7/B7-extra transition: ~$10-15.

## Reference (canonical sources of truth)

### PPUT-CCL arc
| 文件 | 用途 |
|---|---|
| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | 完整 pre-registration spec (round 4 frozen) — 总章法 |
| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | 三 split frozen output + sealed hash |
| `handover/preregistration/scripts/split_pput_ccl.py` | 可重现 split 生成 (seed = `20260426_PPUT_CCL`) |
| `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` | **Phase B 详细实施计划** — 新 session 接手 Phase B 必读 |
| `handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md` | 架构师 v1 测度论 FULL PASS (verbatim) |
| `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md` | 架构师 v2 本体论 FULL PASS (verbatim) |
| `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md` | 最终 round-4 PASS/PASS verdict + 4-round 演化总结 |
| `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md` | round-1 CHALLENGE verdict (历史) |
| `handover/audits/{CODEX,GEMINI}_PPUT_CCL_AUDIT*_2026-04-26.md` | 4 rounds × 2 auditors = 8 audit files |
| `handover/audits/run_{codex,gemini}_pput_ccl_audit*.{py,sh}` | 可重现双审 (4 rounds) |
| `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` | 待用户回的决策点 |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` (F-2026-04-25-02) | arc launch finding |

### Paper 1 (separate, lower priority)
| 文件 | 用途 |
|---|---|
| `handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md` | Paper 1 final draft (tag `paper1-v2.1.1`) |
| `handover/audits/DUAL_AUDIT_V2_1_VERDICT_2026-04-25.md` | Paper 1 R3 PASS/PASS verdict |

### Repo state
- HEAD: B7 close commit (Trust Root + Boot freeze; SHA stamped at commit time, ahead of `fa93943`)
- origin/main HEAD: `fd291d7`; **11 local commits ahead, none pushed** — `913255d`/`4e4afc7`/`47b3dba`/`2a8921b`/`c6087f7`/`34b71c0`/`c30ca81`/`282a459`/`06e1b25`/`fa93943` + B7
- Working tree: `rules/enforcement.log` modified (session-runtime artifact, do not stage)
- Tags pushed: `paper1-v2.1.1`, `archive/art-ii1-v3-abandoned-20260416`
- Branches: `main` (active), 23 archive refs preserved

### Memory entry points (auto-loaded每 session)
- `MEMORY.md` indexes `project_pput_ccl_arc.md` → points here (`LATEST.md`)
- `feedback_phased_checkpoint.md`, `feedback_dual_audit*.md`, `feedback_step_b_protocol.md` are critical for Phase B execution discipline

## Compute spent
- Phase A4 dual audit: ~$15-20 across 4 rounds (~440K Codex tokens + ~310K Gemini tokens)
- B2-B4 mid-term dual audit: ~$3-5 (~67K char prompt × 2; Codex 274s + Gemini 62s)
- Remaining budget: ~$475-480 for Phase B-E (288 p_0 runs + Phase C ablation N=20 × 5 modes + Phase D shadow CCL + Phase E sealed eval + remaining B5/B6/B7 audits)
