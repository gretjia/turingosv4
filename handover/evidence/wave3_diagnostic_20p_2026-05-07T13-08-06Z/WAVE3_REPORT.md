# Wave 3 — 20-problem diagnostic report

**Authority**: PROJECT_PLAN.md §2 Week 2 (last item) + §4 last allowed scale.
**Phase**: Constitution Landing First substrate validation (post-b7bde23 / pre-§5 TB sequence unfreeze).
**Run**: `handover/evidence/wave3_diagnostic_20p_2026-05-07T13-08-06Z/`
**Smoke predecessor**: `handover/evidence/wave3_diagnostic_20p_smoke_2026-05-07T13-04-35Z/` (1-problem; per `feedback_smoke_before_batch`).

---

## 1. What this run validates

`b7bde23 Constitution Landing First` landed three Wave-3 substrate components:

- `src/state/head_t_witness.rs` — G-009 C1 immediate 6-field HEAD_t witness (Art. 0.4)
- `src/runtime/prompt_capsule.rs` — G-016/019/021/028 Class-3 PromptCapsule (Art. III prompt persistence)
- `cases/pcp_corpus/` — G-012 9-class adversarial Lean corpus (Art. I.1.1 PCP)

Per CR-C0.1 / CLAUDE.md §11 _no tape, no test_, GREEN status from cargo tests alone is insufficient; real-LLM tape evidence under load is the load-bearing condition. Phase 3 7-problem (commit `cc59b4d`) ran on the **predecessor** binary; this 20-problem run is the **first real-LLM tape evidence on post-b7bde23 binaries** (rebuilt 2026-05-07 13:03–13:04Z).

The diagnostic answers one question:

> Does the post-b7bde23 substrate degrade FC1 / FC2 / FC3 invariants between 7 and 20 problems?

Answer: **no degradation observed.** FC1 hard invariant holds problem-by-problem and in aggregate; no fake-accepted nodes; no missing typed records.

---

## 2. FC1 hard invariant — load-bearing result

The FC1 Runtime Loop Gate hard invariant (CLAUDE.md §6 / TB-18R R4):

```
evaluator_reported_completed_llm_calls
==
  l4_work_attempt_count
+ l4e_work_attempt_count
+ capsule_anchored_attempt_count
```

| Layer | Result |
|---|---|
| Per-problem `architect_inv1_check.json::match` | **20/20 = True** (architect §5 #1) |
| Per-problem `chain_invariant.json::invariant_verdict` | **20/20 = "Ok"** (R4 6-field invariant) |
| Per-problem `delta` | **20/20 = 0** |
| Aggregate LHS == aggregate RHS | **140 = 7 + 129 + 4** ✓ |

Aggregate breakdown:

| Term | Value | Cross-check |
|---|---|---|
| LHS — `completed_llm_calls_total` | **140** | `tool_dist.step` total (per `OBS_TB18R_INV1_NONLLM_TX_2026-05-07`) |
| RHS-a — `l4_work_attempt_total` | **7** | == `omega_wtool=7` == solved=7 (one accepted WorkTx per solved problem) |
| RHS-b — `l4e_work_attempt_total` | **129** | == `step_reject=129` (predicate-fail / Lean-fail / parse-fail rejections) |
| RHS-c — `capsule_anchored_attempt_total` | **4** | == `step_partial_ok=4` (typed `PartialAccepted` records emitted in 4 problems) |

**Reading**: every externalized LLM-Lean cycle is represented exactly once on tape, in either L4 (accepted), L4.E (rejected), or as a capsule-anchored typed-partial record. No invisible attempts, no drift.

---

## 3. Audit gates (audit_tape per-problem)

| Gate | Result |
|---|---|
| `audit=PROCEED` | **20/20** |
| `id45=Pass` (typed `LeanVerdictKind` 4-arm match per Phase 2 §5.2) | **20/20** |
| `evaluator_failures_excluding_timeout` | **0** |
| `missing_problem_files` | **0** |
| Total `audit_tape` assertions | **39 passed / 0 failed / 11 skipped** per problem |

`step_partial_ok > 0` problems (typed `AttemptOutcome::PartialAccepted` records emitted under load):

- P03 mathd_algebra_114
- P08 mathd_algebra_246
- P15 aime_1990_p4
- P18 induction_11div10tonmn1ton

Phase 3 7-problem had only 1 `step_partial_ok` event (P04). 4× density at 20-problem scale is consistent with substrate working as designed; the typed record path is exercised, not vestigial.

---

## 4. Statistical signal (Art. I.2 — for diagnostic purposes only)

Per `feedback_minif2f_scaling_policy` and PROJECT_PLAN §4: 20-problem n=1 is **harness diagnostic**, not benchmark. The numbers below are reported per CLAUDE.md §17 Reporting Standard but **do not constitute an H-VPPU claim, formal benchmark pass, or real-world readiness signal**.

| Metric | Value |
|---|---|
| N | 20 |
| Solved | 7 |
| Solve rate (point estimate) | 0.350 |
| Solve rate Wilson 95% CI | [0.181, 0.567] |
| ΣPPUT (all 20) | 61.50 |
| Mean PPUT on solved | 8.79 |
| Halt distribution | OmegaAccepted = 7 / MaxTxExhausted = 13 |
| Tool dist | step=140, omega_wtool=7, step_reject=129, step_partial_ok=4 |

Solve set: P01 mathd_algebra_107, P04 mathd_algebra_125, P05 mathd_algebra_141, P06 mathd_algebra_171, P07 mathd_algebra_176, P08 mathd_algebra_246, P10 aime_1989_p8.

The 13 unsolved problems halted by `MaxTxExhausted` at MAX_TX=12 with `inv1_match=True` and `chain_invariant_verdict=Ok` — this is **expected model-capability ceiling**, not a substrate failure. AIME / IMO / induction problems exceed deepseek-chat's single-call solve capacity at this budget.

---

## 5. What this run does NOT validate

- **Not a benchmark.** N=20 × n=1 × single seed × single model. `BenchmarkManifest` schema (PROJECT_PLAN §1 / `feedback_benchmark_manifest_required`) applies at scale (50+ × n>1 × multi-seed) and is not yet built.
- **Not H-VPPU evidence.** Per `feedback_minif2f_scaling_policy`, M0/M1 are harness-prep, not H-VPPU claims.
- **Not real-world readiness.** MiniF2F is formal-proof-only; real-world readiness depends on TB-19+/TB-20/TB-21 sandbox pilots.
- **Does not retire residual AMBER rows.** Constitution Execution Matrix has many AMBER rows whose kill conditions need orthogonal evidence (e.g., Art. III.1 shielding requires injection probe; Art. IV replay requires divergence test).

---

## 6. Per-problem ledger

| # | Problem | Solved | Halt | tx | LLM | tool_dist | inv1 | inv_verdict |
|---|---|---|---|---|---|---|---|---|
| P01 | mathd_algebra_107 | ✓ | OmegaAccepted | 1 | 1 | step=1, omega=1 | True | Ok |
| P02 | mathd_algebra_113 | – | MaxTxExhausted | 12 | 9 | step=9, reject=9 | True | Ok |
| P03 | mathd_algebra_114 | – | MaxTxExhausted | 12 | 12 | step=12, reject=11, partial_ok=1 | True | Ok |
| P04 | mathd_algebra_125 | ✓ | OmegaAccepted | 1 | 1 | step=1, omega=1 | True | Ok |
| P05 | mathd_algebra_141 | ✓ | OmegaAccepted | 1 | 1 | step=1, omega=1 | True | Ok |
| P06 | mathd_algebra_171 | ✓ | OmegaAccepted | 1 | 1 | step=1, omega=1 | True | Ok |
| P07 | mathd_algebra_176 | ✓ | OmegaAccepted | 1 | 1 | step=1, omega=1 | True | Ok |
| P08 | mathd_algebra_246 | ✓ | OmegaAccepted | 2 | 2 | step=2, omega=1, partial_ok=1 | True | Ok |
| P09 | aime_1983_p2 | – | MaxTxExhausted | 12 | 9 | step=9, reject=9 | True | Ok |
| P10 | aime_1989_p8 | ✓ | OmegaAccepted | 1 | 1 | step=1, omega=1 | True | Ok |
| P11 | amc12_2000_p1 | – | MaxTxExhausted | 12 | 12 | step=12, reject=12 | True | Ok |
| P12 | amc12_2000_p6 | – | MaxTxExhausted | 12 | 9 | step=9, reject=9 | True | Ok |
| P13 | algebra_sqineq_at2malt1 | – | MaxTxExhausted | 12 | 9 | step=9, reject=9 | True | Ok |
| P14 | amc12a_2002_p6 | – | MaxTxExhausted | 12 | 12 | step=12, reject=12 | True | Ok |
| P15 | aime_1990_p4 | – | MaxTxExhausted | 12 | 12 | step=12, reject=11, partial_ok=1 | True | Ok |
| P16 | imo_1959_p1 | – | MaxTxExhausted | 12 | 12 | step=12, reject=12 | True | Ok |
| P17 | imo_1962_p2 | – | MaxTxExhausted | 12 | 9 | step=9, reject=9 | True | Ok |
| P18 | induction_11div10tonmn1ton | – | MaxTxExhausted | 12 | 9 | step=9, reject=8, partial_ok=1 | True | Ok |
| P19 | induction_12dvd4expnp1p20 | – | MaxTxExhausted | 12 | 9 | step=9, reject=9 | True | Ok |
| P20 | algebra_2varlineareq_…feqn10_zeq7 | – | MaxTxExhausted | 12 | 9 | step=9, reject=9 | True | Ok |

`tool_dist` notation: step = LLM-Lean cycle count (FC1 invariant LHS); omega = wtool wrapper for omega-success step (subset of step); reject = predicate-fail / Lean-fail; partial_ok = typed `PartialAccepted` record.

---

## 7. Constitution Matrix rows promoted by this evidence

The following rows were 🟢 GREEN post-b7bde23 based on cargo test evidence only. This run is the first real-LLM tape evidence:

- **Art. 0.4 Q_t / G-009 HEAD_t C1 witness** — 20-problem `chain_invariant.json::invariant_verdict=Ok` confirms HEAD_t reconstructs and advances on every accepted L4 transition under load.
- **Art. III prompt persistence (G-016/019/021/028 PromptCapsule)** — substrate compiles into evaluator binary; 140 LLM-Lean cycles ran without panicking on Class-3 PromptCapsule path (real-LLM-bind into capsule still pending; this run is binary-compatibility evidence).
- **Art. I.1.1 PCP / 疑罪从无** — no false accepts: `verified=True` count (7) exactly matches `omega_wtool` count (7) matches `l4_work_attempt_count` (7) matches `solved` count (7). Predicate gate held under 140 cycles.
- **Art. 0.2 Tape Canonical** — aggregate FC1 equation hold over 140 cycles is structural evidence against shadow-ledger drift.

These promotions are evidence-additive, not status-changing on the matrix surface. The matrix file's GREEN status was already set in b7bde23; this run **earns** that GREEN per CR-C0.7.

---

## 8. Resume conditions for §5 TB sequence (PROJECT_PLAN §3)

Status post-this-run:

- ✅ FC composite green (this report; FC1 hard invariant 20/20)
- ✅ `HEAD_t` C1 green (this report)
- ✅ PCP synthetic corpus green (`tests/constitution_pcp_corpus.rs` 7 tests; substrate landed b7bde23)
- ✅ `PromptCapsule` anchored (substrate landed b7bde23)
- ✅ P38 / P49 attempt equality green (Phase 3 7-problem 2026-05-07 + this 20-problem)
- ⏳ Art. III ≥ 60% LANDED+PARTIAL — needs matrix audit (separate task)
- ⏳ Art. 0 ≥ 70% LANDED+PARTIAL — needs matrix audit (separate task)
- ⏳ `cargo test --workspace` 0 fail — verify before next ship
- ⏳ `scripts/run_constitution_gates.sh` 0 fail — verify before next ship
- ⏳ no unresolved critical BLOCKED-DECISION — verify

---

## 9. Reproduction

```bash
# Rebuild (mtime ≥ src ./head_t_witness.rs / prompt_capsule.rs)
cargo build --release --bin audit_tape --bin tb_18r_compute_invariant -p turingosv4
cargo build --release --bin evaluator -p minif2f_v4

# Smoke (1 problem)
bash handover/tests/scripts/run_tb_18r_phase_3_evidence.sh \
  --smoke --out-dir handover/evidence/wave3_diagnostic_20p_smoke_<UTC>

# Batch (20 problems)
bash handover/tests/scripts/run_tb_18r_phase_3_evidence.sh \
  --problems-file handover/tests/scripts/m0_problems.txt \
  --out-dir handover/evidence/wave3_diagnostic_20p_<UTC>
```

Aggregate: `python3 -c "<see WAVE3_AGGREGATE.json reproduction script in commit log>"`

Substrate gate: `git merge-base --is-ancestor 3f51667 HEAD` must succeed.
LLM proxy: `curl http://localhost:8080/health` must return `{"status": "ok"}`.

---

## 10. Outputs

- `WAVE3_AGGREGATE.json` — 446-line per-problem + aggregate JSON (this dir)
- `PHASE_3_RUN_MANIFEST.json` — frozen run manifest (problem list, model, timeout, git_head)
- `PHASE_3_BATCH_SUMMARY.json` — script-side aggregator (note: `evaluable=false` / `lean_results=0` are aggregator field-name misses; ground-truth invariants are in per-problem `architect_inv1_check.json` + `chain_invariant.json`)
- 20× per-problem dirs `P{01..20}_<name>/` containing: `runtime_repo/` (ChainTape repo), `cas/` (CAS objects + index), `evaluator.{stdout,stderr}`, `extracted_pput.json`, `verdict.json` (audit_tape), `chain_invariant.json` (R4 invariant), `architect_inv1_check.json` (architect §5 #1), `verdict_kind_summary.json`, `README.md`
