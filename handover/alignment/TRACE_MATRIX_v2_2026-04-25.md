# TRACE_MATRIX v2 — Constitutional Flowchart ↔ Rust Code (2026-04-25 post-A0)

**Predecessor**: `TRACE_MATRIX_v1_2026-04-25.md`
**Trigger**: Phase A0 (harness modernization) shipped:
- A0a: 4 new rules (R-014/R-015/R-018/R-019) + judge.sh constitution-special-case + R-016 fc_trace_in_commit hook (commit 2e7f75a)
- A0b: tests/fc_alignment_conformance.rs witness battery — 17 PASS + 9 ignored stubs (commit d8950ee)
- A0c: 5 new cases C-071..C-075 sediment session decisions (commit 2a65339)
- A0d (this doc): Trust Root manifest 20 → 24 (this commit); v2 documents the harness as constitutional artifact
- A4 (post-A3): decomposed metrics — `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms` added as non-Optional v2 fields + `compute_tactic_diversity` helper; per-row decomposition of `solve_rate` / `tokens_per_solve` / `time_per_solve` (all derivable from existing `progress` / `total_run_token_count` / `total_wall_time_ms`). FC-trace: FC2-N22 (HALT decomposition for `hit_max_tx`) + FC1-N11 (∏p decision diversity for `tactic_diversity`) + FC1-N12 (oracle scope for `verifier_wait_ms`).
- A5 (post-A4): per-agent budget normalization — new `budget_regime` module (`BUDGET_REGIME` + `MAX_TRANSACTIONS` env vars; 4-variant enum; pure parser + scaler + env-coupled resolver); `budget_regime` + `budget_max_transactions` added as non-Optional v2 fields on `RunAggregate` and the legacy `PputResult`; loop bound at `run_swarm` switched from hardcoded `let max_transactions = 200` to `resolve_budget(n_agents)` — default (env unset) preserves Phase B baseline (`total_proposal × 200`) bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3 satisfied: `MaxTxExhausted` rows now disambiguated across N values. FC-trace: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime). Trust Root manifest 25 → 26.
- A6 (post-A5): per-line FC tagging via structured JSON events — new `fc_trace` module (pure stdlib; zero new deps); `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31); `fc_event!`-style `emit_event` API; `FC_TRACE=1` gate (cached in `OnceLock`); `FC_TRACE_FILE=<path>` redirects emit to file (default sink stderr). Six anchor sites wired in `run_swarm`: FC2-N22 synthetic short-circuit, FC2-N20 mr tick, FC2-N22 OMEGA full-proof, FC2-N22 OMEGA per-tactic, FC2-N22 natural MaxTxExhausted (with `budget_regime` payload), FC1-N12 verify bracket (oneshot). End-to-end smoke test exercises FC_TRACE=1 in a child process (subprocess required because `OnceLock` caches the gate-read; resolves item 7 of TRACE_MATRIX § 5 "Per-line FC tagging via tracing crate"). FC-trace: meta-witness for the 5-step compile loop (Proposal → Lean ground truth → Logging → Capability compilation → ↑H-VPPUT). Trust Root manifest 26 → 27.
- A7 (post-A6): heterogeneous-LLM provider plumbing — `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change (per-provider multi-key round-robin: 3 SiliconFlow keys split concurrent traffic across separate rate-limit pools, mitigating V3L-27 single-key N=30 collapse). `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` probe each of the 3 keys (Qwen/Qwen2.5-7B-Instruct, max_tokens=8) — A7 verified all 3 keys responding 2026-04-26 (1.5–3s latency, 33+1 tokens; round-robin distributes [2,2,2] across 6 calls). New `siliconflow:<model>` provider-prefix syntax in `detect_provider()` for unambiguous routing in `AGENT_MODELS` payloads (Phase D heterogeneous swarms). Memory `reference_siliconflow.md` records SiliconFlow as the heterogeneous-LLM lane (NOT a fallback target). FC-trace: FC1-N7 (δ/AI provider expansion — heterogeneous δ instances across SF catalog enable Phase D meta-loop). Trust Root manifest 28 → 31 (proxy + 2 smoke scripts).
- A8e3 (post-A8e2, FIX-THEN-PROCEED after A8 round-3 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent on packet staleness; no VETO): six narrow cleanup fixes. **H1** A8 packet § 3 A1 description: "Mathematically conservative" → "least-strict admissible value" with explicit Type-I implications. **H2** packet § 3 A6 description: 6 → 9 anchor sites (added F4 swarm sites explicitly). **H3** packet § 6 Q2.a / Q4.a / Q4.d marked CLOSED with closure rationale. **H4** `genesis_payload.toml` TR header phrasing about A1: "conservative ceiling" → "max-tolerated ceiling". **H5** § 5 item 7 here in TRACE_MATRIX: "(commit pending) + 6 wired" → "CLOSED + 9 wired". **H6** G1 wrapper test fail-closed on missing `python3` (was soft skip; Codex R3 caught the silent-pass risk); explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgrades (logged loudly). Trust Root manifest unchanged at 34 (only re-hashes; no new files). Non-blocking observation deferred to Phase B+: `make_pput` signature is now 21 positional args (Gemini R3 Finding 2) — refactor to a builder pattern is recommended but not gating.
- A8e2 (post-A8e, FIX-THEN-PROCEED after A8 round-2 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent): three dual-audit fixes. **G1** (Codex R2#1 + Gemini R2#1) `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` wraps `python3 scripts/test_llm_proxy.py` in a Rust integration test, making the V3L-27 round-robin conformance gate run on every `cargo test --workspace`. **G2** (Codex R2#2) `PREREG_AMENDMENT § 8` audit-requirements paragraph reworded — round-1 fix corrected § 2 but left § 8 still calling the substitution a "strictest plausible bar"; now consistent across the doc. Re-hashed in Trust Root. **G3** (Codex R2#3 + Gemini R2#2) `A8_EXIT_PACKET` updated for round-2 reality: § 2 cumulative-tests table extended with A8e/A8e2 columns, § 4 manifest-clean assertion bumped 30→34, § 5 round-1 risk #5 (no FC1-N12 in swarm verify path) removed (closed by F4), § 6 Q7.a/b numbers updated, round-2 outcome + round-3 fixes section appended; TRACE_MATRIX stale `run_corr_id` symbol row replaced; "in CI" softened to "via Rust wrapper post-A8e2". FC-trace: meta governance + V3L-27 mitigation gating (FC1-N7). Trust Root manifest 33 → 34 (`tests/llm_proxy_python_conformance.rs`).
- A8e (post-A7, FIX-THEN-PROCEED after A8 round-1 audit Codex CHALLENGE / Gemini VETO): six dual-audit fixes. **F1** (Codex#2 + Gemini Q4) `experiments/minif2f_v4/src/run_id.rs` mints one stable run_id per run, threaded into both `emit_event` and `make_pput` (replaces `run_corr_id` vs internal `run_id` ms drift; oneshot also stops using `oneshot_{problem_file}` placeholder). **F2** (Gemini VETO Q6) `scripts/test_llm_proxy.py` adds 15-test conformance battery — round-robin distributes [2,2,2] across 6 calls, single-key pool degrades to all-zero, two-key pool alternates [2,2]; exercises the V3L-27 mitigation without a live API. (A8e2 G1 wires this into `cargo test --workspace` via a Rust shell-out wrapper — see § 2 below.) **F3** (Codex#4) routing matrix bug fixed — `Qwen/Qwen2.5-7B-Instruct` was misrouting to DashScope because `m.startswith("qwen")` won after the slash check; now slash-form ⇒ siliconflow first, bare-qwen ⇒ dashscope second. **F4** (Codex#3) FC1-N12 emit added to all 3 swarm verify sites (`verify_omega_detailed` × 2 + `verify_partial`), each with verdict + elapsed_ms + path stamp; closes the swarm-side gap in the 5-step compile-loop witness. **F5** (Codex#5) `genesis_payload.toml` header + TRACE_MATRIX milestones reconciled to actual 31 entries (round-1 packet undercounted by 1 — agent_models.rs from A3 was already in TR before A1). **F6** (Codex#1) `PREREG_AMENDMENT § 2` wording corrected — `p_0=0.10` substitution is the LEAST-strict admissible ceiling (max tolerated value for `j-RR ≤ p_0`), NOT the strictest possible substitute as round-1 mistakenly claimed; explicit statistical-implications paragraph added (no Type-I inflation since `j-RR` is descriptive; substitution may be less protective than eventual calibrated `p_0 < 0.10`). FC-trace: FC1-N12 (oracle scope swarm coverage) + FC1-N7 (δ provider routing) + Art. V (amendment governance). Trust Root manifest 31 → 33 (run_id.rs + test_llm_proxy.py).

**Scope**: delta from v1. Read v0 + v1 first.

---

## § 1. Status flips: 17 ⚠️ → ✅ via fc_alignment_conformance.rs witnesses

A0b added the missing `tests/fc_alignment_conformance.rs` (was only in `.claude/worktrees/phase-8a-snapshot/`). 17 ✅ rows in TRACE_MATRIX now have automated witness tests. Symbol drift is now caught at `cargo test` time, not at next dual audit.

| FC ID | v1 Status | v2 Status | Witness test |
|---|---|---|---|
| FC1-N1 (Q_t carrier) | ⚠️ | ✅ | `fc1_n1_q_state_carrier_present` |
| FC1-N4 (tape) | ⚠️ | ✅ | `fc1_n4_tape_constructible_with_time_arrow` |
| FC1-N6 (input UniverseSnapshot) | ✅ | ✅ + witness | `fc1_n6_input_universe_snapshot_present` |
| FC1-N7 (δ/AI ResilientLLMClient) | ✅ | ✅ + witness | `fc1_n7_delta_ai_client_type` |
| FC1-N8/N9/N10 (output / q_o / a_o) | ✅ | ✅ + witness | `fc1_n8_n9_n10_output_agent_output_parseable` |
| FC1-N11 (∏p production-path forbidden_pattern) | ⚠️ | ✅ | `fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern` |
| FC1-N13 (wtool bus.append) | ⚠️ | ✅ | `fc1_n13_wtool_bus_append_present` |
| FC1-N15 / E18 (∏p=0 → Q_t preserve) | ⚠️ | ✅ | `fc1_n11_n15_e18_*` (same test) |
| FC2-N20/N27 (mr tick) | ✅ | ✅ + witness | `fc2_n20_n27_tick_mr_present` |
| FC2-N22 (HALT) | ⚠️ | ✅ | `fc2_n22_halt_via_halt_and_settle` |
| FC2-N23 (HaltReason — only OmegaAccepted typed) | ✅ | ✅ + witness | `fc2_n23_event_type_omega_accepted_canonical` |
| FC3-N31 (Wal logs archive) | ⚠️ | ✅ | `fc3_n31_logs_archive_wal_present` |
| FC3-N34 (readonly guard verify_trust_root) | ✅ | ✅ + 3 witnesses | `fc3_n34_*` (3 tests) |
| FC3-N39 (Ledger log) | ✅ | ✅ + witness | `fc3_n39_log_ledger_present_and_appendable` |
| FC3-S3 (readonly subgraph manifest) | (new in v1) | ✅ | `fc3_s3_readonly_subgraph_manifest_size` (>=20 entries assertion) |
| FC3-E14 (boot panic immediate-abort) | (new in v1) | ✅ | `fc3_e14_boot_panic_immediate_abort_documented` |
| (Veto-AI Art. V.1.3 amendment) | (cases C-072) | ✅ via case-law | C-072 yaml |

## § 2. New code symbols (Phase A0–A3)

| Symbol | File | FC anchor | Status |
|---|---|---|---|
| `tests/fc_alignment_conformance.rs` (17 witness fns + 9 ignored stubs) | `tests/fc_alignment_conformance.rs` | meta-witness for FC1/FC2/FC3 ↔ symbol mapping; CLAUDE.md "Conformance tests" requirement | ✅ |
| `rules/active/R-014_trust_root_manifest_drift.yaml` | `rules/active/R-014*.yaml` | FC3-S3 readonly subgraph runtime reminder | ✅ |
| `rules/active/R-015_trace_matrix_pub_symbol.yaml` | `rules/active/R-015*.yaml` | CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素" | ✅ |
| `rules/active/R-018_constitution_amendment_sudo.yaml` | `rules/active/R-018*.yaml` | Art. V.1.1 amendment 2026-04-25 (sudo only for constitution.md) | ✅ |
| `rules/active/R-019_model_snapshot_canonical.yaml` | `rules/active/R-019*.yaml` | FC1-N7 δ/AI canonical identity | ✅ |
| `judge.sh` constitution.md special case | `.claude/hooks/judge.sh:50-67` | FC3-N3 sudo-gate enforcement (closes silent-bypass via `*.md` skip-list) | ✅ |
| `judge.sh` R-016 fc_trace_in_commit | `.claude/hooks/judge.sh:48-56` | FC-first rule (memory feedback_fc_first_problem_handling + case C-074) | ✅ |
| `parse_swarm_condition_n` (A2) | `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N16 InitAI orchestration entry — discriminates `oneshot` vs `n<N>` swarm code paths; FC1-N11 ∏p reached only via swarm | ✅ |
| `agent_models::{AGENT_MODELS_ENV_VAR, PHASE_D_HETERO_GATE_ENV_VAR, AgentModelsError, parse_agent_models, expand_agent_models, resolve_agent_models}` (A3) | `experiments/minif2f_v4/src/agent_models.rs` | FC1-N7 δ/AI per-agent assignment; gates Phase B+C single-model invariant (notepad F-2026-04-25-02) | ✅ |
| `RunAggregate::{hit_max_tx, tactic_diversity, verifier_wait_ms}` + `compute_tactic_diversity` (A4) | `experiments/minif2f_v4/src/jsonl_schema.rs` | FC2-N22 HALT decomposition (hit_max_tx splits natural max-tx exhaustion from OMEGA accept and from B7-extra synthetic short-circuit); FC1-N11 ∏p decision diversity (tactic_diversity = distinct/total over append+complete+step proposals); FC1-N12 oracle scope (verifier_wait_ms = cumulative Lean wall-clock per run, ≤ total_wall_time_ms by construction) | ✅ |
| `make_pput` A4 args + per-call-site verifier brackets + per-tool proposal hashing (A4) | `experiments/minif2f_v4/src/bin/evaluator.rs` | wires the 3 fields at every emit site (oneshot + swarm OMEGA + swarm step Complete + swarm synthetic short-circuit + swarm natural max-tx exhaustion); 5 unit/conformance tests (`test_a4_decomposed_metrics_round_trip`, `test_a4_tactic_diversity_helper`, `test_a4_verifier_wait_bounded_by_total_wall_time`, `test_a4_emit_max_tx_exhaustion_row`, `test_a4_synthetic_short_circuit_does_not_set_hit_max_tx`) | ✅ |
| `budget_regime::{BUDGET_REGIME_ENV_VAR, MAX_TRANSACTIONS_ENV_VAR, DEFAULT_MAX_TRANSACTIONS, BudgetRegime, BudgetError, parse_budget_regime, parse_max_transactions, effective_max_tx, resolve_budget}` (A5) | `experiments/minif2f_v4/src/budget_regime.rs` | FC2-N22 HALT decomposition by budget regime — declares which partitioning rule (`total_proposal` / `per_agent` / `token_total` / `wall_clock`) governed the loop bound. Phase A scope = first two regimes implemented; latter two declared startup-fatal `UnimplementedRegime` so a misconfigured run aborts before consuming LLM budget. PREREG_AMENDMENT_p0_defer § 3 condition 3 dependency cleared. | ✅ |
| `RunAggregate::{budget_regime, budget_max_transactions}` + `PputResult::{budget_regime, budget_max_transactions}` (A5) | `experiments/minif2f_v4/src/jsonl_schema.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N22: every emitted v2 row stamps the regime label + base budget so downstream PPUT analysis can join on the partitioning rule. Loop bound at `run_swarm` startup = `resolve_budget(n_agents).effective_max_tx`; default (env unset) preserves the Phase B baseline `total_proposal × 200` bit-for-bit. 16 unit tests (15 in `budget_regime::tests` + 1 `test_a5_budget_regime_round_trip` in jsonl_schema). | ✅ |
| `fc_trace::{FcId, FC_TRACE_*ENV*, fc_trace_enabled, emit_event, json_str}` (A6) | `experiments/minif2f_v4/src/fc_trace.rs` | meta-witness for FC1 / FC2 / FC3 path coverage. 7-variant `FcId` enum produces stable strings (`FC1-N7` / `FC1-N11` / `FC1-N12` / `FC1-E18` / `FC2-N20` / `FC2-N22` / `FC3-N31`) that Phase D consumers + TRACE_MATRIX rows join on. `FC_TRACE=1` gate cached in `OnceLock` (zero-overhead in production). 6 unit tests (label stability + JSON escape + cold-path no-op). | ✅ |
| `run_id::mint_run_id` correlation key + 9 wired FC events (A6 + A8e F4) | `experiments/minif2f_v4/src/run_id.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs` | A6 introduced a per-run correlation id (originally named `run_corr_id`, now unified into `run_id`) anchoring all events. A8e fix F1 (Codex#2 + Gemini Q4) split this out into a dedicated module + threaded the same id into `make_pput` so FC events and v2 jsonl rows share one identifier (no ms drift). Anchor sites: FC2-N22 synthetic short-circuit / mr tick FC2-N20 / OMEGA full-proof FC2-N22 / OMEGA per-tactic FC2-N22 / natural MaxTxExhausted FC2-N22 (with `budget_regime` payload from A5) / FC1-N12 oneshot verify (A6) / FC1-N12 swarm `verify_omega_detailed` × 2 paths (A8e F4) / FC1-N12 swarm `verify_partial` (A8e F4). End-to-end smoke `tests/fc_trace_smoke.rs` exercises FC_TRACE=1 in a child process (forced because `OnceLock` caches the gate-read). | ✅ |
| `tests/llm_proxy_python_conformance.rs` Rust→Python wrapper (A8e2 fix G1) | `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` | A8 round-2 audit (Codex R2#1 + Gemini R2#1): the round-robin conformance battery in `scripts/test_llm_proxy.py` was only manual; tests that don't run automatically are documentation, not gates. G1 wires the Python suite into `cargo test --workspace` via a Rust integration test that shells to `python3` and asserts exit 0 + the unittest "OK" trailer. The V3L-27 mitigation (case C-027) is now a recurring conformance gate exercised on every commit that touches Rust + every CI pipeline that already exercises Rust tests. | ✅ |
| `llm_proxy.py` v4 (multi-key round-robin) + `detect_provider` `siliconflow:` prefix (A7) | `src/drivers/llm_proxy.py` | FC1-N7 δ/AI provider expansion — three SiliconFlow keys form a 3-element round-robin pool keyed on `_per_key_requests[provider]`. Phase D heterogeneous swarms can address SF models via `AGENT_MODELS=siliconflow:Qwen/Qwen2.5-7B-Instruct,...`. Mitigates V3L-27 (case C-027) single-key N=30 401/429 collapse documented in `cases/V3_LESSONS.md`. | ✅ |
| `smoke_siliconflow.sh` + `_smoke_siliconflow.py` (A7) | `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` | A7 acceptance gate — 3 keys × 1 probe each (Qwen2.5-7B-Instruct, max_tokens=8). Verified all 3 SiliconFlow keys responding 2026-04-26 + proxy round-robin distributes [2,2,2] across 6 calls. PASS gates Phase D heterogeneous-swarm work. | ✅ |

## § 3. Trust Root manifest expansion: 20 → 24

Per case **C-075 (DO-178C tool qualification)**: governance instrumentation is itself constitutional; tampering with rules / judge.sh / conformance tests = silent constitutional drift.

| New entry | Why in Trust Root |
|---|---|
| `rules/MANIFEST.sha256` (proxy for 14 rules/active/R-*.yaml) | Same pattern as cases/MANIFEST.sha256: glob hashed once, manifest tracked in Trust Root. Tampering with R-018 enforcement = "warn" silently bypasses constitution sudo gate. |
| `rules/engine.py` | The interpreter of the rules. Tampering with engine.py = silent rule bypass even with intact rule files. |
| `.claude/hooks/judge.sh` | The PreToolUse hook that invokes engine.py + implements R-016 fc_trace + constitution.md special-case. Tampering = bypass entire gate stack. |
| `tests/fc_alignment_conformance.rs` | Witness battery for TRACE_MATRIX ✅ rows. Tampering = false PASS hides drift. |

**Total: 24 entries** (15 from B7 + 1 B7-extra rollback_sim + 4 dual-audit fixes + 4 A0 harness). A1 (PREREG amendment) → 25; A3 (agent_models.rs) → 26; A5 (budget_regime.rs) → 27; A6 (fc_trace.rs) → 28; A7 (llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py) → 31; A8e (run_id.rs + test_llm_proxy.py) → 33; A8e2 (llm_proxy_python_conformance.rs) → 34. When B7-extra calibration eventually runs, the calibration jsonl makes 35 entries; future Phase C's `--mode` flag binary (TBD location) makes 36.

## § 4. New constitutional case-law (A0c)

5 new cases C-071..C-075 (commit 2a65339) sediment 2026-04-25 session decisions as constitutional precedent. Each cross-referenced in TRACE_MATRIX rows:

| Case | Anchors | Rules / hooks enforcing |
|---|---|---|
| C-071 constitution amendment process | Art. V.1.1 + V.3 | R-018 (BLOCK) + judge.sh special-case |
| C-072 Veto-AI scope narrowing | Art. V.1.3 | manual via dual audit; future FC3-N32 runtime |
| C-073 ArchitectAI commit authority | Art. V.1.2 | implicit via 19-commit session validation |
| C-074 FC-first problem handling | All FC + Alignment Standard | R-016 (WARN on git commit without FC-trace) |
| C-075 DO-178C tool qualification | PREREG § 1.8 + Art. V.1.1 | R-014 (warn on .rs edit) + 24-file manifest expansion |

## § 5. Open work flagged for future TRACE_MATRIX_v3

1. **TRACE_MATRIX_v?.md docs themselves** — currently NOT in Trust Root (would cause self-reference loop). Acceptable since these are documentation, not enforcement. Phase D (when ArchitectAI runtime comes online) may need to formalize doc-Trust-Root semantics.
2. **rules/SCHEMA.yaml** — defines rule format but engine.py doesn't validate against it. Lower priority; add to Trust Root if SCHEMA itself is referenced by automated tests.
3. **build-check.sh + session-end.sh** — sister hooks of judge.sh. Lower-priority gates (build verification, session telemetry); add to Trust Root in next harness cycle.
4. **R-016 fc_trace_in_commit upgrade** — currently WARN-level. If post-Phase-D evidence shows FC-trace discipline still slipping, promote to BLOCK-level.
5. **R-020 ground_truth_label** — sketched in A0a planning but not implemented (grep on PputResult/RunAggregate field additions to enforce thesis claim 7 ground-truth source). Defer to next harness cycle.
6. **FC2-N23 HaltReason full taxonomy as Rust enum** — currently only OmegaAccepted is typed; other 4 variants live as jsonl strings. Phase C+ Soft Law mode work may force this typing.
7. ~~**Per-line FC tagging via tracing crate** — Plan agent's recommendation in N-experiments brainstorm. Phase A6 deferred; will land before Phase B (homogeneous experiments).~~ **CLOSED**: A6 (`89994c7`) shipped `fc_trace.rs` + 7 anchor sites; A8e fix F4 added 2 swarm-side `verify_omega_detailed` paths + 1 `verify_partial`, bringing the total to **9 wired anchor sites**. Implementation chose pure stdlib over the `tracing` crate to avoid a new dep tree; the macro surface (`emit_event` + `FcId` enum) was kept small so Phase D+ can swap to a real `tracing-subscriber` bridge locally.

## § 6. Updated counts (v2)

Compared to v1:
- ✅ count: 16 → **33** (+17 from fc_alignment_conformance.rs witness battery; +4 from new symbols/rules; +4 from manifest expansion; +5 case-law entries; -3 stale)
- 📅/📄 count: 9 → **9** (Phase 11+ deferred unchanged; some clarified with case references)
- 🔨/⚠️ count: 0 → **0** (no actionable rows pending in v2 scope)
- New cases: 5 (C-071..C-075)
- New rules: 4 active (R-014/R-015/R-018/R-019) + 1 hook-level (R-016)

Manifest size milestones:
- B7 → 15
- B7-extra → 16
- B7-extra round-1 audit-fix → 20
- A0 (this v2) → 24
- A1 PREREG amendment → 25
- A3 agent_models.rs → 26
- A5 budget_regime.rs → 27
- A6 fc_trace.rs → 28
- A7 llm_proxy.py + smoke_siliconflow.{sh,py} → 31
- A8e run_id.rs + test_llm_proxy.py → 33
- A8e2 llm_proxy_python_conformance.rs → **34**
- (planned) B7-extra calibration freeze → 35
- (planned) Phase C mode-flag binary → 36+

## § 7. Cross-references

- `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md` (immutable baseline)
- `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md` (B7 + B7-extra v1)
- `handover/alignment/FC_ELEMENTS_2026-04-22.md` (canonical FC node IDs)
- `handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` (FC3-E14 vs FC2-N22 distinction)
- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (Findings A+B)
- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (Findings C+D)
- `cases/C-071`..`C-075`.yaml (Phase A0 case-law)
- `~/.claude/.../memory/feedback_fc_first_problem_handling.md` (FC-first rule memory)
