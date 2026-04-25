#!/usr/bin/env bash
# Codex pre-batch audit — Phase B B7 + B7-extra (rollback toggle, calibration runner).
# GATE: PASS/CHALLENGE/VETO BEFORE running the 576-run p_0 calibration batch
# (~$3-5 API + ~8h wall-clock + freezes p_0 into Trust Root — irreversible cost).
#
# Independent of Gemini. Output: CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md"
TMP_PROMPT="$(mktemp /tmp/b7_extra_codex.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Pre-Batch Audit — Phase B B7 + B7-extra (Trust Root + Rollback Toggle + Calibration)

**Role**: skeptical adversarial reviewer. Independent of Gemini. CLAUDE.md "Audit Standard": VETO > CHALLENGE > PASS; conservative reading wins on disagreement.

**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration (= retread the full $3-5 + 8h + Trust Root churn). PASS only if you would stake your independence on the batch result being valid.

**Thesis v2 alignment** (just frozen by user, 2026-04-25): the 5-step compile loop is `Proposal (LLM) → Feedback from Ground Truth (Lean / FS / external compiler) → Logging (ground-truth-validated, isolated from active context) → Capability Compilation → ↑ H-VPPUT`. Audit must check whether the calibration treatment path honors the ground-truth-feedback anchor or accidentally degrades into LLM-as-Judge.

**State of the code**:
- 187/187 cargo test --workspace PASS + 20 deferred-stub `#[ignore]`
- B7 commits: `be42f43` (Trust Root + Boot freeze), `0cc48bc` (TRACE_MATRIX v1 alignment fix)
- B7-extra commits: `973a9fd` (toggle), `b0ae03e` (calibration runner + estimator), `35e221d` (synthetic_short_circuit field), `3868b1d` (architect-insights A+B), `1875543` (thesis-v2 audit C+D)
- Smoke verified:
  - Easy (mathd_algebra_107, n3 swarm, 4 runs, 39s): all SOLVED tx=1; toggle inert; jsonl V2 + calibration tags ✓
  - Hard #1 (aime_1983_p2, treatment, toggle ON): tx_count=50 + stderr "[rollback_sim] firing at tx=50" ✓
  - Hard #2 (rebuilt with synthetic_short_circuit field, same problem): tx_count=50, solved=false, **synthetic_short_circuit=true** ✓, total_run_token_count=39947 (50 tx of real activity)

**Trust Root manifest** (16 files):
- PREREG § 1.8 base 8: src/{kernel,wal,bus}.rs, experiments/.../lean4_oracle.rs, constitution.md, cases/MANIFEST.sha256, handover/preregistration/{PPUT_CCL_SPLITS,PREREG}.{json,md}
- Mid-term audit add 6: src/drivers/llm_http.rs, experiments/.../{cost_aggregator,wall_clock,post_hoc_verifier,jsonl_schema}.rs, experiments/.../bin/evaluator.rs
- B6 add 1: src/sdk/prompt_guard.rs
- B7-extra add 1: experiments/minif2f_v4/src/rollback_sim.rs

**4 architect-insight findings already filed** (A+B in `B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md`; C+D in `THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md`):
- (A) synthetic predicate at evaluator-level instead of bus.register_predicate (Phase Z trait unmerged) — measurement-only path, doesn't affect production
- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
- (C) WAL Omega* events declared but never emitted in production
- (D) bus.record_rejection mixes policy + ground-truth class labels with no provenance tag

The audit MUST decide whether these 4 findings are sufficient mitigation OR whether any of them upgrade to a pre-batch blocker.

---

## Specific questions (cite line/file/§ for every finding)

### Q1 — Constitutional anchor of synthetic-veto-at-tx-50

The B7-extra design claim: "synthetic ∏p=0 from tx 50 onward" maps to FC1-E18 (∏p=0 → Q_t preservation, repeated 150x) followed by natural FC2-N22 HALT via `HaltReason::MaxTxExhausted`. The IMPLEMENTATION is a short-circuit at tx==50 in `experiments/minif2f_v4/src/bin/evaluator.rs:503-518`, with a `synthetic_short_circuit=true` field stamping the row.

Vulnerabilities to scrutinize:
- (Q1.a) Is the constitutional anchor SOUND, or is it a post-hoc rationalization? Specifically: an honest 150-tx vetoed loop would (i) call `client.generate` 150x more, (ii) accumulate cost, (iii) walk the bus's `evaluate_predicates` path (currently inline forbidden_patterns, since bus.register_predicate API unmerged). The short-circuit does NONE of these. Does the equivalence claim hold UNDER PROPERLY DEFINED equivalence relations, or is it a convenience overlay?
- (Q1.b) The `synthetic_short_circuit` field's doc comment warns about cost-asymmetry. Is the warning sufficient, or should the evaluator EMIT a stderr-level WARN at every short-circuit so log-readers also see it (currently `info!` not `warn!`)?
- (Q1.c) `should_simulate_rollback(tx, enabled) = enabled && tx == 50` fires EXACTLY ONCE at tx==50. If for any reason the loop body re-enters tx==50 (it can't, but hypothetically), would the short-circuit double-fire / re-stamp? Defensive bound check needed?
- (Q1.d) Does the `synthetic_short_circuit` flag correctly serialize as `null` (omitted) for control runs and `true` only for treatment? (The smoke confirms this empirically; check the code path.)

### Q2 — Trust Root manifest sufficiency

PREREG § 1.8 base + audit add + B6 + B7-extra = 16 files. `genesis_payload.toml` itself is NOT self-hashed (chicken-and-egg, documented as Section 6 of TRACE_MATRIX_v1).

Vulnerabilities to scrutinize:
- (Q2.a) `src/boot.rs` (the verifier itself) is intentionally NOT in Trust Root — TRACE_MATRIX_v1 § 4 records the threat-model rationale (passive-tamper vs malicious-recompile). Is this a CHALLENGE (verifier should self-attest) or a PASS (chicken-and-egg)?
- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
- (Q2.c) The 16-file list omits `experiments/minif2f_v4/src/lib.rs` (which `pub mod rollback_sim;` — i.e., gates module visibility). Tampering with this lib.rs could remove rollback_sim from compilation entirely. Defensible?
- (Q2.d) `cases/MANIFEST.sha256` is hashed once into Trust Root, but the case files themselves only get integrity-checked transitively (manifest hash → manifest content → case hashes). Is this depth sufficient, or should each case file have a direct manifest entry?
- (Q2.e) `Cargo.lock` is NOT in Trust Root. A malicious actor could swap a dependency version (e.g., a sha2 backdoor). Should it be?

### Q3 — p_0 calibration semantics

PREREG § 5.5 specifies `p_0 = sum_p max_seed(regression_p) / 144` where `regression_p = 1 iff control SOLVED && treatment UNSOLVED`. Treatment short-circuits at tx 50.

Vulnerabilities to scrutinize:
- (Q3.a) The synthetic short-circuit at tx 50 measures "how often does control take ≥ 50 tx to solve". Is that a FAIR proxy for "how often does a Phase E artifact corrupt mid-run state"? An artifact-induced failure at tx 50 in a real Phase E run would produce a different cost/time profile than our short-circuit. Does this matter for j-RR ≤ p_0 guardrail?
- (Q3.b) `compute_p0.py` per-problem uses `max(over seeds)` — worst-case framing. If only 1 of 2 seeds regressed, the problem still counts as 1. With 2 seeds, this DOUBLES the regression rate vs `mean(over seeds)`. PREREG § 5.5 line 450 explicitly says max — agreed?
- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. For a synthetic_short_circuit row, both will be False (correctly UNSOLVED). For a control row that SOLVED, both will be True. Is the predicate correct under the v2 RunAggregate schema?
- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. If the runner script's row-enrichment failed for any subset (e.g., python3 OOM'd on one row), would calibration silently miss data? Should we fail loudly?
- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?

### Q4 — Ground-truth feedback honesty (thesis v2)

Thesis v2 claim 7: "white-box predicates settle state transitions based on STRICT FEEDBACK FROM GROUND TRUTH". Treatment short-circuit synthesizes the ∏p=0 verdict WITHOUT actually running ∏p (= without forbidden_patterns check, without oracle.verify_omega).

Vulnerabilities to scrutinize:
- (Q4.a) For control runs (no toggle), every state transition IS ground-truth-feedback-anchored: append → forbidden_patterns + oracle path. ✓
- (Q4.b) For treatment runs (toggle on), the short-circuit BYPASSES the predicate path entirely. The "synthetic ∏p=0" is asserted by the evaluator without running any ∏p check. Does this violate thesis v2 claim 7?
- (Q4.c) Counter-argument: the calibration TREATMENT mode is itself a measurement protocol artifact, not a production state transition. The thesis applies to production proposals; calibration is meta. Is this counter-argument STRUCTURALLY sound, or a lazy escape hatch?
- (Q4.d) Each emitted row (control + treatment) has `verified: bool` from `make_pput`. For treatment short-circuits, `verified = false` is asserted in `make_pput(... runtime_accepted=false, post_hoc_verified=false ...)` — but it's asserted by the CALLER, not derived from a Lean call. Is this a sufficient ground-truth anchor (caller knows it didn't accept), or should we call the oracle even for synthetic short-circuits to confirm (paranoid but pure)?

### Q5 — Findings C+D pre-batch impact

Finding C: WAL Omega* events declared but never emitted. Finding D: bus.record_rejection mixes policy + ground-truth class labels.

Vulnerabilities to scrutinize:
- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)? `compute_p0.py` joins on calibration_problem_id + calibration_seed and reads only `solved` / `progress_verified` — neither field comes from WAL or recent_rejections. Argument for "no impact": the calibration jsonl IS ground-truth-validated independently. Verify.
- (Q5.b) During the 576-run batch, does any row's `verified` field DEPEND on bus events that don't fire under treatment short-circuit? E.g., if `verified` were derived from "WAL has OmegaAccepted event" (it isn't, but conceivably), the short-circuit's no-WAL-event would corrupt verified. Trace the actual `verified` derivation.
- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50 (correctly reflects 50 tx of real activity). Does anything downstream INTERPRET these as "if they were lower than control, the treatment toggle is bug-prone"? compute_p0.py doesn't; flag if any other tooling does.

### Q6 — Sanity gate enforcement

PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2 if ceiling violated.

Vulnerabilities to scrutinize:
- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it and continue? Test by inspection: where in the pipeline does compute_p0.py run, and what handles exit code?
- (Q6.b) p_0 > 0.10 means treatment failed too aggressively. The mitigation per PREREG = "redesign rollback simulation, redo". Are we confident the synthetic-short-circuit-at-tx-50 design is FAR enough from the ceiling that we won't hit it? If most adaptation problems take >50 tx to solve, p_0 will be HIGH. Pre-batch evidence (smoke) doesn't tell us this.
- (Q6.c) If p_0 is in (0.05, 0.10] — borderline — is that a passive accept, or should a borderline result trigger automatic re-calibration with a different threshold (e.g., tx 75) to verify p_0 is stable?

### Q7 — Calibration run economics + failure mode resilience

576 runs × ~50s/run avg = ~8h wall-clock. ~$3-5 API spend at deepseek-chat rates. Runner script timeout per problem = 2400s = 40 min.

Vulnerabilities to scrutinize:
- (Q7.a) If DeepSeek API drifts mid-batch (memory: project_deepseek_drift_2026-04-24 — `deepseek-chat` drifted Apr 24 with mathd_algebra_246 FAIL→SOLVE on same seeds), the resulting p_0 is INCONSISTENT across the run. PREREG § 1.8 commits source_pool_sha256 + heldout_sealed_hash, but NOT model_snapshot. Should we abort + redo if drift detected, or accept noise?
- (Q7.b) If any individual run hits the 40-min timeout, the runner skips with MEASUREMENT_ERROR. Does this artificially raise p_0 (by counting timeouts as treatment-UNSOLVED while control might have SOLVED)?
- (Q7.c) Runner does NOT have a resume mode for partial-batch failure. If the SSH session drops at run 300/576, restarting would re-run everything. Acceptable, or should we add resume?
- (Q7.d) The runner's oracle preflight (C-012) only checks `(1:ℝ) + 1 = 2 := by norm_num`. If Mathlib breaks for a more complex tactic mid-batch, calibration silently produces FALSE for problems that should have SOLVED. Is preflight depth sufficient?
- (Q7.e) Total cost upper bound (576 runs × 40-min timeout × API cost per minute) — is the user's $3-5 estimate stable, or could a worst-case run-up to 576 × 40-min = 384 hours of compute lead to a 50x cost overrun?

### Q8 — Constitutional flowchart compliance (FC1/FC2/FC3 trace)

For each B7-extra component, verify FC anchor:

| Component | Claimed FC | Actual call path | Verify |
|---|---|---|---|
| `rollback_sim::should_simulate_rollback` | FC1-E18 + FC2-N22 (synthetic) | evaluator-layer guard at loop top | ? |
| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
| `synthetic_short_circuit` field | calibration disambiguator | jsonl emit | ? |
| `src/main.rs` panic | FC3-N1 boot + FC3-E14 error→re-init→boot | pre-init failure | ? |

Vulnerabilities to scrutinize:
- (Q8.a) Is each "Verify" column legitimately ✅, or are there mappings that look forced?
- (Q8.b) TRACE_MATRIX_v1 promoted FC3-N34 from 📅 to ✅. Is the promotion premature — i.e., is the implementation a partial fulfillment of FC3-N34, or a complete one?
- (Q8.c) Should TRACE_MATRIX_v2 be drafted before launching the batch (so the audit trail shows the matrix at batch-launch time, not retroactively)?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 P0 issues with line/file references — must be fixable BEFORE the batch (not "after we see the result")
- Conviction (low/med/high)
- Specific recommendation: PROCEED with batch / FIX-THEN-PROCEED / REDESIGN

PASS only if the batch result will be defensible to an external observer at Phase B → C audit packet review. The audit gate exists to PREVENT the user from spending $3-5 + 8h on a result that is later found to be invalid.

---

# Appended source files

BRIEF_EOF

# Source files for audit
printf '\n## experiments/minif2f_v4/src/rollback_sim.rs (B7-extra, NEW)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/experiments/minif2f_v4/src/rollback_sim.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## src/boot.rs (B7, NEW)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/boot.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## src/main.rs (B7 panic site)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/main.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## genesis_payload.toml (B7 + B7-extra)\n\n```toml\n' >> "$TMP_PROMPT"
cat "${ROOT}/genesis_payload.toml" >> "$TMP_PROMPT"

printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"

printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"

printf '\n```\n\n## evaluator.rs B7-extra wiring diff (be42f43~1..HEAD)\n\n```diff\n' >> "$TMP_PROMPT"
cat /tmp/b7_b7extra_evaluator_diff.patch >> "$TMP_PROMPT"

printf '\n```\n\n## handover/alignment/TRACE_MATRIX_v1_2026-04-25.md\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n## handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n## handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n## PREREG § 5.5 (calibration protocol)\n\n' >> "$TMP_PROMPT"
awk '/^### 5\.5 p_0 calibration protocol/,/^### 5\.6/' "${ROOT}/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n## PREREG § 1.7-1.8 (ArtifactState + Trust Root)\n\n' >> "$TMP_PROMPT"
awk '/^### 1\.7 /,/^## 2\./' "${ROOT}/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" >> "$TMP_PROMPT"

printf '\n\n---\n\nNow give your INDEPENDENT pre-batch audit. Cite §/file:line for every finding.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex b7-extra-prebatch] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex PPUT-CCL Phase B B7-extra Pre-Batch Audit\n'
  printf '**Date**: 2026-04-25\n'
  printf '**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n'
  printf '**Test baseline**: 187/187 PASS + 20 ignored\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex b7-extra-prebatch] API returned in ${elapsed}s" >&2
echo "[codex b7-extra-prebatch] saved: $OUT" >&2
