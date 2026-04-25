#!/usr/bin/env python3
"""Gemini pre-batch audit — Phase B B7 + B7-extra (rollback toggle, calibration runner).
GATE: PASS/CHALLENGE/VETO BEFORE running 576-run p_0 calibration batch.
Independent of Codex. Output: GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md"""
import json, sys, time, subprocess, urllib.request, urllib.error
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Pre-Batch Audit — Phase B B7 + B7-extra (Trust Root + Rollback Toggle + Calibration)

**Role**: skeptical adversarial reviewer. Independent of Codex. CLAUDE.md "Audit Standard": VETO > CHALLENGE > PASS; conservative reading wins on disagreement.

**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration. PASS only if you would stake your independence on the batch result being valid.

**Thesis v2 alignment** (just frozen by user, 2026-04-25): the 5-step compile loop is `Proposal (LLM) → Feedback from Ground Truth (Lean / FS / external compiler) → Logging (ground-truth-validated, isolated from active context) → Capability Compilation → ↑ H-VPPUT`. Audit must check whether the calibration treatment path honors the ground-truth-feedback anchor or accidentally degrades into LLM-as-Judge.

**State of the code**:
- 187/187 cargo test --workspace PASS + 20 deferred-stub `#[ignore]`
- B7 commits: `be42f43` (Trust Root + Boot freeze), `0cc48bc` (TRACE_MATRIX v1 alignment fix)
- B7-extra commits: `973a9fd` (toggle), `b0ae03e` (calibration runner + estimator), `35e221d` (synthetic_short_circuit field), `3868b1d` (architect-insights A+B), `1875543` (thesis-v2 audit C+D)
- Smoke verified:
  - Easy (mathd_algebra_107, n3 swarm, 4 runs, 39s): all SOLVED tx=1; toggle inert; jsonl V2 + calibration tags ✓
  - Hard #1 (aime_1983_p2, treatment, toggle ON): tx_count=50 + stderr "[rollback_sim] firing at tx=50" ✓
  - Hard #2 (rebuilt with synthetic_short_circuit field, same problem): tx_count=50, solved=false, **synthetic_short_circuit=true** ✓, total_run_token_count=39947

**Trust Root manifest** (16 files):
- PREREG § 1.8 base 8: src/{kernel,wal,bus}.rs, experiments/.../lean4_oracle.rs, constitution.md, cases/MANIFEST.sha256, handover/preregistration/{PPUT_CCL_SPLITS,PREREG}.{json,md}
- Mid-term audit add 6: src/drivers/llm_http.rs, experiments/.../{cost_aggregator,wall_clock,post_hoc_verifier,jsonl_schema}.rs, experiments/.../bin/evaluator.rs
- B6 add 1: src/sdk/prompt_guard.rs
- B7-extra add 1: experiments/minif2f_v4/src/rollback_sim.rs

**4 architect-insight findings already filed**:
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
- (Q1.a) Is the constitutional anchor SOUND, or is it a post-hoc rationalization? Specifically: an honest 150-tx vetoed loop would (i) call `client.generate` 150x more, (ii) accumulate cost, (iii) walk the bus's `evaluate_predicates` path. The short-circuit does NONE of these. Does the equivalence claim hold UNDER PROPERLY DEFINED equivalence relations, or is it a convenience overlay?
- (Q1.b) The `synthetic_short_circuit` field's doc comment warns about cost-asymmetry. Is the warning sufficient, or should the evaluator EMIT a stderr-level WARN at every short-circuit so log-readers also see it (currently `info!` not `warn!`)?
- (Q1.c) Does the `synthetic_short_circuit` flag correctly serialize as `null` (omitted) for control runs and `true` only for treatment? (The smoke confirms this empirically; check the code path.)

### Q2 — Trust Root manifest sufficiency

PREREG § 1.8 base + audit add + B6 + B7-extra = 16 files. `genesis_payload.toml` itself is NOT self-hashed (chicken-and-egg, documented as Section 6 of TRACE_MATRIX_v1).

Vulnerabilities to scrutinize:
- (Q2.a) `src/boot.rs` (the verifier itself) is intentionally NOT in Trust Root — TRACE_MATRIX_v1 § 4 records the threat-model rationale (passive-tamper vs malicious-recompile). Is this a CHALLENGE (verifier should self-attest) or a PASS (chicken-and-egg)?
- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
- (Q2.c) The 16-file list omits `experiments/minif2f_v4/src/lib.rs`. Tampering with this lib.rs could remove rollback_sim from compilation entirely. Defensible?
- (Q2.d) `cases/MANIFEST.sha256` is hashed once; case files only get integrity-checked transitively. Is this depth sufficient?
- (Q2.e) `Cargo.lock` is NOT in Trust Root. A malicious actor could swap a dependency version. Should it be?

### Q3 — p_0 calibration semantics

PREREG § 5.5 specifies `p_0 = sum_p max_seed(regression_p) / 144` where `regression_p = 1 iff control SOLVED && treatment UNSOLVED`. Treatment short-circuits at tx 50.

Vulnerabilities to scrutinize:
- (Q3.a) The synthetic short-circuit at tx 50 measures "how often does control take ≥ 50 tx to solve". Is that a FAIR proxy for "how often does a Phase E artifact corrupt mid-run state"? Does this matter for j-RR ≤ p_0 guardrail?
- (Q3.b) Per-problem `max(over seeds)` framing — with 2 seeds, this DOUBLES the regression rate vs `mean`. PREREG § 5.5 line 450 explicitly says max — agreed?
- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. Is the predicate correct under the v2 RunAggregate schema?
- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. Should we fail loudly?
- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?

### Q4 — Ground-truth feedback honesty (thesis v2)

Thesis v2 claim 7: "white-box predicates settle state transitions based on STRICT FEEDBACK FROM GROUND TRUTH". Treatment short-circuit synthesizes the ∏p=0 verdict WITHOUT actually running ∏p.

Vulnerabilities to scrutinize:
- (Q4.a) For control runs (no toggle), every state transition IS ground-truth-feedback-anchored. ✓
- (Q4.b) For treatment runs (toggle on), the short-circuit BYPASSES the predicate path entirely. Does this violate thesis v2 claim 7?
- (Q4.c) Counter-argument: the calibration TREATMENT mode is itself a measurement protocol artifact, not a production state transition. Is this STRUCTURALLY sound?
- (Q4.d) Each emitted row has `verified: bool` from `make_pput`. For treatment short-circuits, `verified = false` is asserted by the CALLER, not derived from a Lean call. Is this a sufficient ground-truth anchor?

### Q5 — Findings C+D pre-batch impact

Finding C: WAL Omega* events declared but never emitted. Finding D: bus.record_rejection mixes policy + ground-truth class labels.

Vulnerabilities to scrutinize:
- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)?
- (Q5.b) During the 576-run batch, does any row's `verified` field DEPEND on bus events that don't fire under treatment short-circuit?
- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50. Does anything downstream INTERPRET these as suspicious?

### Q6 — Sanity gate enforcement

PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2.

Vulnerabilities to scrutinize:
- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it?
- (Q6.b) p_0 > 0.10 means treatment failed too aggressively. If most adaptation problems take >50 tx to solve, p_0 will be HIGH. Pre-batch evidence (smoke) doesn't tell us this.
- (Q6.c) If p_0 is in (0.05, 0.10] — borderline — is that a passive accept, or should a borderline result trigger automatic re-calibration?

### Q7 — Calibration run economics + failure mode resilience

576 runs × ~50s/run avg = ~8h wall-clock. ~$3-5 API. Runner timeout per problem = 2400s = 40 min.

Vulnerabilities to scrutinize:
- (Q7.a) DeepSeek API drift mid-batch (memory: `deepseek-chat` drifted Apr 24). Should we abort + redo if drift detected?
- (Q7.b) If any individual run hits the 40-min timeout, runner skips with MEASUREMENT_ERROR. Does this artificially raise p_0?
- (Q7.c) Runner does NOT have a resume mode. If SSH drops at run 300/576, restart re-runs everything. Acceptable?
- (Q7.d) Oracle preflight (C-012) only checks `(1:ℝ) + 1 = 2`. Is preflight depth sufficient?
- (Q7.e) Total cost upper bound — is the user's $3-5 estimate stable, or could worst-case timeout-runs cause 10x overrun?

### Q8 — Constitutional flowchart compliance (FC1/FC2/FC3 trace)

For each B7-extra component, verify FC anchor:

| Component | Claimed FC | Actual call path | Verify |
|---|---|---|---|
| `rollback_sim::should_simulate_rollback` | FC1-E18 + FC2-N22 (synthetic) | evaluator-layer guard at loop top | ? |
| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
| `synthetic_short_circuit` field | calibration disambiguator | jsonl emit | ? |
| `src/main.rs` panic | FC3-N1 boot + FC3-E14 error→re-init→boot | pre-init failure | ? |

Vulnerabilities to scrutinize:
- (Q8.a) Is each "Verify" column legitimately ✅?
- (Q8.b) TRACE_MATRIX_v1 promoted FC3-N34 from 📅 to ✅. Is the promotion premature?
- (Q8.c) Should TRACE_MATRIX_v2 be drafted before launching the batch?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 P0 issues with line/file references — must be fixable BEFORE the batch
- Conviction (low/med/high)
- Specific recommendation: PROCEED with batch / FIX-THEN-PROCEED / REDESIGN

PASS only if the batch result will be defensible to an external observer at Phase B → C audit packet review.

---

# Appended source files

"""

# Source files
rollback_sim = (ROOT / "experiments/minif2f_v4/src/rollback_sim.rs").read_text()
boot_rs = (ROOT / "src/boot.rs").read_text()
main_rs = (ROOT / "src/main.rs").read_text()
genesis = (ROOT / "genesis_payload.toml").read_text()
runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
trace_v1 = (ROOT / "handover/alignment/TRACE_MATRIX_v1_2026-04-25.md").read_text()
findings_ab = (ROOT / "handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md").read_text()
findings_cd = (ROOT / "handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md").read_text()

prereg = (ROOT / "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md").read_text()
# § 5.5 calibration protocol
def slice_section(text, start_pat, end_pat):
    lines = text.splitlines()
    out = []
    in_section = False
    for line in lines:
        if line.startswith(start_pat):
            in_section = True
        elif line.startswith(end_pat) and in_section:
            break
        if in_section:
            out.append(line)
    return "\n".join(out)

prereg_55 = slice_section(prereg, "### 5.5 p_0 calibration protocol", "### 5.6")
prereg_17_18 = slice_section(prereg, "### 1.7 ", "## 2.")

diff_text = Path("/tmp/b7_b7extra_evaluator_diff.patch").read_text()

full_prompt = (
    brief +
    f"\n\n## experiments/minif2f_v4/src/rollback_sim.rs (B7-extra, NEW)\n\n```rust\n{rollback_sim}\n```\n" +
    f"\n## src/boot.rs (B7, NEW)\n\n```rust\n{boot_rs}\n```\n" +
    f"\n## src/main.rs (B7 panic site)\n\n```rust\n{main_rs}\n```\n" +
    f"\n## genesis_payload.toml\n\n```toml\n{genesis}\n```\n" +
    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n{runner_sh}\n```\n" +
    f"\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n{compute_py}\n```\n" +
    f"\n## evaluator.rs B7-extra wiring diff (be42f43~1..HEAD)\n\n```diff\n{diff_text}\n```\n" +
    f"\n## handover/alignment/TRACE_MATRIX_v1_2026-04-25.md\n\n{trace_v1}\n" +
    f"\n---\n\n## handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md\n\n{findings_ab}\n" +
    f"\n---\n\n## handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md\n\n{findings_cd}\n" +
    f"\n---\n\n## PREREG § 5.5 (calibration protocol)\n\n{prereg_55}\n" +
    f"\n---\n\n## PREREG § 1.7-1.8 (ArtifactState + Trust Root)\n\n{prereg_17_18}\n" +
    "\n---\n\nNow give your INDEPENDENT pre-batch audit. Cite §/file:line for every finding."
)

print(f"[gemini b7-extra-prebatch] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()
headers = {"Content-Type": "application/json"}

t0 = time.time()
req = urllib.request.Request(url, data=body, headers=headers, method="POST")
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except urllib.error.HTTPError as e:
    print(f"[gemini b7-extra-prebatch] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini b7-extra-prebatch] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md"
header = (f"# Gemini PPUT-CCL Phase B B7-extra Pre-Batch Audit\n"
          f"**Date**: 2026-04-25\n"
          f"**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n"
          f"**Test baseline**: 187/187 PASS + 20 ignored\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini b7-extra-prebatch] saved: {out}")
