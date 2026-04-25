# Codex PPUT-CCL Phase B B7-extra Pre-Batch Audit
**Date**: 2026-04-25
**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
**Test baseline**: 187/187 PASS + 20 ignored
**Prompt size**: 86117 chars

---

Reading prompt from stdin...
OpenAI Codex v0.124.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dc597-a26f-7863-bc34-0474776f8903
--------
user
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


## experiments/minif2f_v4/src/rollback_sim.rs (B7-extra, NEW)

```rust
// PPUT-CCL Phase B B7-extra — synthetic rollback simulation.
//
// Constitutional anchor (TRACE_MATRIX_v1 § 7.2): the
// `--simulate-rollback-at-tx-50` toggle (PREREG § 5.5) is realized as
// "every proposal from tx 50 onward is vetoed", which is constitutionally
// the **FC1-E18** edge (∏p=0 → Q_t preservation) repeated for tx
// 50..max_transactions. The run then exhausts naturally and exits via
// the existing **FC2-N22 HALT** with `HaltReason::MaxTxExhausted` — no
// new HaltReason variant is introduced and no new constitutional surface
// is created.
//
// For efficiency, the swarm loop short-circuits at tx == threshold
// instead of running ~150 guaranteed-vetoed iterations. The short-circuit
// is observably equivalent: identical exit state, identical cost
// accumulator (no extra LLM calls would have happened in vetoed tx),
// identical wall-clock close. The only observable difference is
// `tx_count` stamped at threshold rather than `max_transactions` — a
// useful diagnostic signal that distinguishes a calibration-treatment
// run from a real exhaustion.
//
// Threat model: the threshold is fixed at 50 per PREREG § 5.5 frozen
// spec. The env var `SIMULATE_ROLLBACK_AT_TX_50` is a binary toggle
// (`"1"` to enable). The threshold is intentionally not exposed as a
// runtime parameter — pre-registration discipline (C-070) requires that
// what we calibrate is exactly what is committed in genesis_payload.toml.

/// PREREG § 5.5: the synthetic rollback fires at this transaction index
/// in the swarm loop. Frozen — must match the value committed in the
/// pre-registration hash chain.
pub const ROLLBACK_TX_THRESHOLD: u64 = 50;

/// Env var name read by the evaluator. `"1"` enables the toggle; any
/// other value (or absence) is "off".
pub const ROLLBACK_ENV_VAR: &str = "SIMULATE_ROLLBACK_AT_TX_50";

/// True iff the calibration treatment toggle is enabled in the current
/// process environment.
pub fn rollback_simulation_enabled() -> bool {
    std::env::var(ROLLBACK_ENV_VAR)
        .ok()
        .as_deref()
        == Some("1")
}

/// True iff the swarm loop should short-circuit at this `tx` index. The
/// short-circuit is constitutionally equivalent to "synthetic ∏p=0 from
/// here, naturally exhaust at `max_transactions`" — see module header.
///
/// `enabled` is a parameter (not read from env) so unit tests can drive
/// the predicate without process-global state.
pub fn should_simulate_rollback(tx: u64, enabled: bool) -> bool {
    enabled && tx == ROLLBACK_TX_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fires_at_threshold_when_enabled() {
        assert!(should_simulate_rollback(50, true));
    }

    #[test]
    fn does_not_fire_before_threshold() {
        for tx in [0_u64, 1, 25, 49] {
            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
        }
    }

    #[test]
    fn does_not_fire_after_threshold() {
        // Constitutional reading: at tx > 50, the synthetic ∏p has already
        // begun returning Reject; the loop continues but accumulates no
        // progress. Short-circuit fires exactly once at tx == threshold,
        // not on every tx after.
        for tx in [51_u64, 60, 100, 199] {
            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
        }
    }

    #[test]
    fn never_fires_when_disabled() {
        for tx in [0_u64, 49, 50, 51, 199] {
            assert!(!should_simulate_rollback(tx, false), "tx={tx}");
        }
    }

    #[test]
    fn threshold_constant_matches_prereg() {
        // PREREG § 5.5 freezes the threshold at 50. If this assertion ever
        // fails, the codebase has drifted from the pre-registration hash
        // chain — recompute Trust Root and dual-audit before continuing.
        assert_eq!(ROLLBACK_TX_THRESHOLD, 50);
    }

    #[test]
    fn env_var_name_matches_prereg() {
        // PREREG § 5.5 names the toggle `--simulate-rollback-at-tx-50`;
        // the env-var equivalent (the v4 evaluator does not use clap)
        // mirrors that name uppercased + underscored.
        assert_eq!(ROLLBACK_ENV_VAR, "SIMULATE_ROLLBACK_AT_TX_50");
    }
}

```

## src/boot.rs (B7, NEW)

```rust
// PPUT-CCL Phase B B7 — Trust Root + Boot freeze (PREREG § 1.8 + § 7).
//
// Constitutional anchor: FC3-S3 `readonly` subgraph (constitution.md
// line 670, system-level flowchart). The constitutional readonly base
// is {constitution-as-ground-truth, logs-archive-as-ground-truth}; B7
// extends this base per PREREG § 1.8 to also cover the case-law glob,
// pre-registration spec, heldout splits, and the PPUT accounting layer.
// TRACE_MATRIX_v0 row FC3-N34 was 📅 Phase 11+ ("FS-level readonly
// check at init") — B7 implements it via SHA-256 manifest verification.
// See `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md`.
//
// At Boot we hash every tracked file and compare against the
// `[trust_root]` manifest in `genesis_payload.toml`. Any mismatch =>
// `TrustRootError::Tampered { .. }`. `src/main.rs` panics with
// `TRUST_ROOT_TAMPERED`.
//
// Manifest derivation (Phase B7, independently re-derived from PREREG
// § 1.8 + B2-B4 mid-term audit recommendation + B6 prompt_guard add):
// see header comment in `genesis_payload.toml`.
//
// TOML parsing is hand-rolled (~30 LOC). The manifest format is flat:
// section header + `"path" = "hash"` lines. Adding a `toml` crate
// dependency would drag in ~5 transitive crates for what we can do
// in-line; compression principle (CLAUDE.md "反奥利奥架构") wins.

use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// TRACE_MATRIX FC3-N34: failure variants of the readonly-guard verification.
/// Constitutional role = the diagnostic surface that distinguishes
/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
/// `GenesisParse` (manifest itself unreadable, also a violation but a
/// different fix path).
#[derive(Debug)]
pub enum TrustRootError {
    GenesisRead(std::io::Error),
    GenesisParse(String),
    SectionMissing(&'static str),
    FileRead { path: PathBuf, err: std::io::Error },
    Tampered { path: PathBuf, expected: String, actual: String },
}

impl std::fmt::Display for TrustRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
            Self::Tampered { path, expected, actual } => write!(
                f,
                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
                path.display(), expected, actual
            ),
        }
    }
}

impl std::error::Error for TrustRootError {}

/// TRACE_MATRIX FC3-N34: implementation of the constitutional `readonly`
/// subgraph (constitution.md FC3, system-level flowchart). Verifies every
/// tracked file's SHA-256 against the `genesis_payload.toml [trust_root]`
/// manifest at Boot. Mismatch => Boot abort; the readonly guarantee that
/// the constitution requires of {constitution, logs} (extended per PREREG
/// § 1.8 to the full PPUT-accounting base) is enforced here.
///
/// `repo_root` is the directory containing `genesis_payload.toml` (typically
/// the workspace root). Paths in the manifest are interpreted relative to it.
pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
    let genesis_path = repo_root.join("genesis_payload.toml");
    let genesis_text = fs::read_to_string(&genesis_path).map_err(TrustRootError::GenesisRead)?;
    let manifest = parse_trust_root_section(&genesis_text)?;
    if !has_section(&genesis_text, "pput_accounting_0") {
        return Err(TrustRootError::SectionMissing("pput_accounting_0"));
    }
    for (rel_path, expected_hash) in &manifest {
        let full = repo_root.join(rel_path);
        let bytes = fs::read(&full).map_err(|err| TrustRootError::FileRead {
            path: full.clone(),
            err,
        })?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let actual = hex_lower(&hasher.finalize());
        if actual != *expected_hash {
            return Err(TrustRootError::Tampered {
                path: full,
                expected: expected_hash.clone(),
                actual,
            });
        }
    }
    Ok(())
}

/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
/// the trust_root_immutability conformance battery (Phase B7) reads the
/// manifest directly to assert it includes the audit-recommended PPUT
/// accounting layer.
///
/// Parses the `[trust_root]` section of `genesis_payload.toml` into ordered
/// `(path, sha256)` pairs. Hand-rolled — accepts the narrow subset we emit
/// (quoted-key = quoted-value, comments, blank lines).
pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
    let mut in_section = false;
    let mut entries = Vec::new();
    for (lineno, raw) in text.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(header) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            in_section = header.trim() == "trust_root";
            continue;
        }
        if !in_section {
            continue;
        }
        let (key, value) = line.split_once('=').ok_or_else(|| {
            TrustRootError::GenesisParse(format!("line {}: missing '=' in [trust_root]", lineno + 1))
        })?;
        let key = unquote(key.trim()).ok_or_else(|| {
            TrustRootError::GenesisParse(format!("line {}: key not quoted", lineno + 1))
        })?;
        let value = unquote(value.trim()).ok_or_else(|| {
            TrustRootError::GenesisParse(format!("line {}: value not quoted", lineno + 1))
        })?;
        entries.push((key.to_string(), value.to_string()));
    }
    if entries.is_empty() {
        return Err(TrustRootError::SectionMissing("trust_root"));
    }
    Ok(entries)
}

fn has_section(text: &str, name: &str) -> bool {
    text.lines().any(|raw| {
        let line = strip_comment(raw).trim();
        line
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .map(|h| h.trim() == name)
            .unwrap_or(false)
    })
}

fn strip_comment(line: &str) -> &str {
    let mut in_string = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '#' if !in_string => return &line[..i],
            _ => {}
        }
    }
    line
}

fn unquote(s: &str) -> Option<&str> {
    s.strip_prefix('"').and_then(|s| s.strip_suffix('"'))
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        // turingosv4 lib is at repo root; CARGO_MANIFEST_DIR == repo root.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn parse_strips_inline_comment_and_blanks() {
        let toml = r#"
            [pput_accounting_0]
            schema_version = "1.0"

            [trust_root]
            # leading comment
            "a/b.rs" = "deadbeef"   # trailing comment
            "c/d.md" = "cafebabe"
        "#;
        let entries = parse_trust_root_section(toml).unwrap();
        assert_eq!(
            entries,
            vec![
                ("a/b.rs".to_string(), "deadbeef".to_string()),
                ("c/d.md".to_string(), "cafebabe".to_string()),
            ]
        );
    }

    #[test]
    fn parse_errors_on_unquoted_key() {
        let toml = "[trust_root]\nfoo = \"deadbeef\"\n";
        assert!(matches!(
            parse_trust_root_section(toml),
            Err(TrustRootError::GenesisParse(_))
        ));
    }

    #[test]
    fn parse_errors_when_section_missing() {
        let toml = "[pput_accounting_0]\nschema_version = \"1.0\"\n";
        assert!(matches!(
            parse_trust_root_section(toml),
            Err(TrustRootError::SectionMissing("trust_root"))
        ));
    }

    #[test]
    fn verify_trust_root_passes_on_intact_repo() {
        verify_trust_root(&repo_root()).expect("intact repo verifies");
    }

    #[test]
    fn verify_trust_root_detects_tamper_in_tempdir() {
        // Build a fake repo: copy genesis_payload.toml + the smallest tracked
        // file (cases/MANIFEST.sha256 is short), tamper the file, expect
        // Tampered error.
        let tmp = tempdir();
        let genesis_src = repo_root().join("genesis_payload.toml");
        let genesis_dst = tmp.join("genesis_payload.toml");
        // Trim the manifest down to a single-entry section so the test does
        // not need to mirror every tracked file.
        let single_entry = r#"[pput_accounting_0]
schema_version = "1.0"

[trust_root]
"only.txt" = "0000000000000000000000000000000000000000000000000000000000000000"
"#;
        fs::write(&genesis_dst, single_entry).unwrap();
        fs::write(tmp.join("only.txt"), "tampered").unwrap();

        let err = verify_trust_root(&tmp).expect_err("tamper must be detected");
        match err {
            TrustRootError::Tampered { path, expected, actual } => {
                assert!(path.ends_with("only.txt"));
                assert_eq!(expected, "0".repeat(64));
                assert_ne!(actual, expected);
            }
            other => panic!("expected Tampered, got {other:?}"),
        }
        // suppress unused
        let _ = genesis_src;
    }

    #[test]
    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
        let tmp = tempdir();
        let payload = "hello";
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let hash = hex_lower(&hasher.finalize());
        let genesis = format!(
            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{hash}\"\n"
        );
        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
        fs::write(tmp.join("only.txt"), payload).unwrap();
        verify_trust_root(&tmp).expect("matching hash verifies");
    }

    fn tempdir() -> PathBuf {
        // Minimal tempdir without adding a `tempfile` dep.
        let pid = std::process::id();
        let nano = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("turingosv4-boot-test-{pid}-{nano}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}

```

## src/main.rs (B7 panic site)

```rust
use std::path::PathBuf;

// TRACE_MATRIX FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`):
// constitution FC3 ties `boot` to a re-init loop driven by `error`. Phase B7
// implements the immediate-abort variant of FC3-E14 — Trust Root mismatch
// at Boot panics the process; the surrounding harness (batch runner,
// shell) is the "re-init" layer that decides whether to retry. Future
// in-process re-init (TRACE_MATRIX FC3-N41 row, currently 📅 Phase 11+)
// would replace this panic with a structured retry loop. See
// `handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` for why
// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
        panic!("TRUST_ROOT_TAMPERED: {e}");
    }
    println!("TuringOS v4 — Trust Root verified");
}

```

## genesis_payload.toml (B7 + B7-extra)

```toml
# TuringOS v4 — Genesis payload (Phase B7).
#
# Frozen at Phase B B7 (2026-04-25). Two binding sections:
#
#   [pput_accounting_0] — semantic invariants of the PPUT measure
#                         (PREREG § 1.8). baseline_regression_rate +
#                         baseline_regression_jsonl_sha256 are placeholders
#                         until B7-extra (p_0 calibration) lands.
#
#   [trust_root]        — SHA-256 of every load-bearing file. Boot
#                         (`turingosv4::boot::verify_trust_root`) recomputes
#                         each hash and aborts with TRUST_ROOT_TAMPERED on
#                         mismatch.
#
# Manifest derivation (independently re-derived in B7 from PREREG § 1.8 +
# B2-B4 mid-term audit recommendation + B6 prompt_guard add):
#
#   PREREG § 1.8 base (8):
#     src/kernel.rs, src/wal.rs, src/bus.rs,
#     experiments/minif2f_v4/src/lean4_oracle.rs,
#     constitution.md, cases/MANIFEST.sha256 (proxy for cases/*.yaml glob),
#     handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,
#     handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
#
#   Mid-term audit add (PPUT accounting layer, 6):
#     src/drivers/llm_http.rs (cost source of truth),
#     experiments/minif2f_v4/src/cost_aggregator.rs (B2),
#     experiments/minif2f_v4/src/wall_clock.rs (B3),
#     experiments/minif2f_v4/src/post_hoc_verifier.rs (B4),
#     experiments/minif2f_v4/src/jsonl_schema.rs (B1),
#     experiments/minif2f_v4/src/bin/evaluator.rs (the wiring)
#
#   B6 add (1):
#     src/sdk/prompt_guard.rs (PPUT-context-leak runtime gate)
#
#   B7-extra add (1):
#     experiments/minif2f_v4/src/rollback_sim.rs (PPUT-CCL § 5.5
#       calibration treatment toggle — synthetic ∏p=0 from tx 50,
#       constitutionally FC1-E18 + FC2-N22-MaxTxExhausted)
#
# Total: 16 files. genesis_payload.toml itself is conceptually frozen but
# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
# values are the semantic anchor.
#
# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.

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
"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
"experiments/minif2f_v4/src/jsonl_schema.rs" = "7647a2ea842f4394d55e1ce85032642c18ba61f962bd2c5c8714c618e743777c"
"experiments/minif2f_v4/src/rollback_sim.rs" = "0c4dd5a922edddd43769915f0d2c13332331ecbee74e09294f6b2243ec7607cd"
"experiments/minif2f_v4/src/bin/evaluator.rs" = "265314baa700ce0c795e3ec7efed5a77ebc81a01884bae33592d1a0e8d8ce32c"
"constitution.md" = "b304b431b7d3fbb8d1630f76ac4bc3eaa80383dde41fb3bc69fbe5bb263658ef"
"cases/MANIFEST.sha256" = "3759631afae97c631be9a4038479b8d3681af25885f9758c32ad88c22f17fc3f"
"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"

```

## handover/preregistration/scripts/run_p0_calibration.sh (NEW)

```bash
#!/usr/bin/env bash
# PPUT-CCL B7-extra — p_0 calibration runner.
#
# PREREG § 5.5 protocol:
#   - control:    evaluator on adaptation-144 × seeds [31415, 2718]
#   - treatment:  same + SIMULATE_ROLLBACK_AT_TX_50=1
#   - 288 + 288 = 576 runs total.
#   - regression_p = 1 iff control SOLVED && treatment UNSOLVED, same (problem, seed)
#   - p_0 = sum_p max_seed(regression_p) / 144
#
# Constitutional anchor (TRACE_MATRIX_v1 § 2):
#   treatment runs route through the existing FC1-E18 (∏p=0 → Q_t)
#   semantics — see experiments/minif2f_v4/src/rollback_sim.rs header.
#
# Usage:
#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
#
#   --smoke  run 1 problem × 2 seeds × 2 modes = 4 runs (~5 min, ~$0.05)
#            for pre-batch verification per feedback_smoke_before_batch.md
#   (no flag) full 576-run batch (~8h, ~$3-5 — needs explicit user GO)
#
# Prerequisites (same as run_batch.sh):
#   export DEEPSEEK_API_KEY=...
#   export LLM_PROXY_URL=http://localhost:8080  (default)
#   export ACTIVE_MODEL=deepseek-chat           (default)

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Auto-load v3 .env for API keys if not already set
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$PROJECT_ROOT/experiments/minif2f_v4/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"

SMOKE=0
if [ "${1:-}" = "--smoke" ]; then
    SMOKE=1
fi

# PREREG § 5.5: condition fixed at n3 (3-agent swarm — needs >=50 tx capacity).
# Boltzmann seeds frozen at PREREG values.
CONDITION="n3"
SEEDS=(31415 2718)
MODES=("control" "treatment")

mkdir -p "$LOG_DIR"

if [ "$SMOKE" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
else
    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
fi

# Resolve adaptation-144 problem list from frozen splits.
# Each problem ID maps to <MINIF2F_DIR>/MiniF2F/Test/<id>.lean.
ADAPTATION_IDS=$(python3 -c "
import json
d = json.load(open('$SPLITS_JSON'))
for pid in d['splits']['adaptation']['problem_ids']:
    print(pid)
")

if [ "$SMOKE" -eq 1 ]; then
    # Smoke: pick one short mathd_algebra problem (typically solved in <50 tx).
    SMOKE_ID=$(echo "$ADAPTATION_IDS" | grep "^mathd_algebra" | head -1)
    if [ -z "$SMOKE_ID" ]; then
        SMOKE_ID=$(echo "$ADAPTATION_IDS" | head -1)
    fi
    ADAPTATION_IDS="$SMOKE_ID"
    echo "[smoke] using single problem: $SMOKE_ID"
fi

# Build evaluator (release).
echo "Building evaluator (release)..."
(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

# C-012 oracle preflight (memory feedback_oracle_preflight.md).
echo "Oracle preflight..."
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
    \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) \
    -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
    exit 2
fi
PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1)
PREFLIGHT_CODE=$?
if [ "$PREFLIGHT_CODE" -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
    echo "$PREFLIGHT_OUT" | head -c 500
    exit 2
fi
echo "Oracle preflight OK."

# Run loop. Each (mode, seed, problem) combination = 1 run.
TOTAL_PROBLEMS=$(echo "$ADAPTATION_IDS" | wc -l)
TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
echo ""
echo "=== p_0 calibration ==="
echo "Mode count:    ${#MODES[@]} (control + treatment)"
echo "Seed count:    ${#SEEDS[@]} (${SEEDS[*]})"
echo "Problem count: $TOTAL_PROBLEMS"
echo "Total runs:    $TOTAL_RUNS"
echo ""

BATCH_START=$(date +%s)
RUN_IDX=0
for MODE in "${MODES[@]}"; do
    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
    : > "$OUT_FILE"
    : > "$STDERR_LOG"
    case "$MODE" in
        control)   ROLLBACK_FLAG="" ;;
        treatment) ROLLBACK_FLAG="1" ;;
    esac
    for SEED in "${SEEDS[@]}"; do
        while IFS= read -r PID; do
            [ -z "$PID" ] && continue
            RUN_IDX=$((RUN_IDX + 1))
            PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
            if [ ! -f "$PROBLEM" ]; then
                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND, skip"
                continue
            fi
            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
            OUTPUT=$(timeout 2400 env \
                CONDITION="$CONDITION" \
                MINIF2F_DIR="$MINIF2F_DIR" \
                BOLTZMANN_SEED="$SEED" \
                SIMULATE_ROLLBACK_AT_TX_50="$ROLLBACK_FLAG" \
                RUST_LOG=info \
                "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || true
            PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)
            if [ -n "$PPUT_JSON" ]; then
                # Stamp mode + seed + problem_id for downstream pairing analysis
                # (PREREG § 5.5 estimator does control-vs-treatment join on
                # (problem, seed)).
                ENRICHED=$(echo "$PPUT_JSON" | python3 -c "
import json, sys
row = json.loads(sys.stdin.read())
row['calibration_mode'] = '$MODE'
row['calibration_seed'] = $SEED
row['calibration_problem_id'] = '$PID'
print(json.dumps(row))
")
                echo "$ENRICHED" >> "$OUT_FILE"
                HAS_GP=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('has_golden_path', False))")
                TX=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tx_count', 0))")
                if [ "$HAS_GP" = "True" ]; then
                    echo "SOLVED (tx=$TX)"
                else
                    echo "UNSOLVED (tx=$TX)"
                fi
            else
                echo "MEASUREMENT_ERROR"
            fi
        done <<< "$ADAPTATION_IDS"
    done
done

BATCH_END=$(date +%s)
WALL_TIME=$((BATCH_END - BATCH_START))

echo ""
echo "╔═══════════════════════════════════════════╗"
echo "║   p_0 CALIBRATION SUMMARY"
echo "╠═══════════════════════════════════════════╣"
echo "║ Wall time:      ${WALL_TIME}s"
echo "║ Control jsonl:  ${OUT_PREFIX}_control.jsonl"
echo "║ Treatment jsonl: ${OUT_PREFIX}_treatment.jsonl"
echo "╚═══════════════════════════════════════════╝"
echo ""
if [ "$SMOKE" -eq 1 ]; then
    echo "Smoke complete. Verify (1) treatment row tx_count == 50 if it would have"
    echo "exceeded 50, (2) both rows parse via RunRecord::V2, (3) calibration_mode +"
    echo "calibration_seed + calibration_problem_id are present. Then re-run without"
    echo "--smoke for the full 576-run batch."
else
    echo "Compute p_0:"
    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
    echo "    --control ${OUT_PREFIX}_control.jsonl \\"
    echo "    --treatment ${OUT_PREFIX}_treatment.jsonl"
    echo ""
    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
fi

```

## handover/preregistration/scripts/compute_p0.py (NEW)

```python
#!/usr/bin/env python3
"""PPUT-CCL B7-extra — compute p_0 from calibration jsonl.

PREREG § 5.5 estimator:
    For each (problem, seed): regression_p_seed = 1 iff control SOLVED
                              AND treatment UNSOLVED.
    Per-problem regression:   max over the 2 seeds (worst case).
    p_0:                      sum_p regression_p / N_problems.

Sanity gate: if p_0 > 0.10, ABORT — toggle too aggressive (PREREG § 5.5 ceiling).

Usage:
    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from collections import defaultdict
from pathlib import Path


def load_jsonl(path: Path) -> list[dict]:
    rows = []
    with path.open() as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rows.append(json.loads(line))
    return rows


def solved(row: dict) -> bool:
    """PREREG § 1.3 progress = 1 iff Lean ground-truth verifies golden_path.

    The B4 split (`progress_runtime` vs `progress_verified`) frames the verified
    leg as authoritative. Fall back to `has_golden_path` for legacy rows.
    """
    if "progress_verified" in row and row["progress_verified"] is not None:
        return int(row["progress_verified"]) == 1
    return bool(row.get("has_golden_path", False))


def compute(control_rows: list[dict], treatment_rows: list[dict]) -> dict:
    # Index by (problem_id, seed). calibration_problem_id and calibration_seed
    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
    # either tag.
    def index(rows):
        out = {}
        for r in rows:
            pid = r.get("calibration_problem_id")
            seed = r.get("calibration_seed")
            if pid is None or seed is None:
                continue
            out[(pid, seed)] = r
        return out

    c = index(control_rows)
    t = index(treatment_rows)

    pairs = sorted(set(c.keys()) & set(t.keys()))
    if not pairs:
        sys.exit("ERROR: no overlapping (problem, seed) pairs between control and treatment")

    # Per-problem worst-case regression (max over seeds).
    per_problem_regression: dict[str, int] = defaultdict(int)
    n_pairs = 0
    n_control_solved = 0
    n_treatment_solved = 0
    n_regression_pairs = 0
    for pid, seed in pairs:
        cr = c[(pid, seed)]
        tr = t[(pid, seed)]
        cs = solved(cr)
        ts = solved(tr)
        n_pairs += 1
        if cs:
            n_control_solved += 1
        if ts:
            n_treatment_solved += 1
        regression = 1 if (cs and not ts) else 0
        if regression:
            n_regression_pairs += 1
        if regression > per_problem_regression[pid]:
            per_problem_regression[pid] = regression

    n_problems = len({pid for pid, _ in pairs})
    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0

    return {
        "n_problems": n_problems,
        "n_pairs": n_pairs,
        "n_control_solved": n_control_solved,
        "n_treatment_solved": n_treatment_solved,
        "n_regression_pairs": n_regression_pairs,
        "n_regression_problems_max_seed": sum(per_problem_regression.values()),
        "p0": p0,
        "p0_ceiling": 0.10,
        "ceiling_pass": p0 <= 0.10,
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--control", required=True, type=Path)
    ap.add_argument("--treatment", required=True, type=Path)
    ap.add_argument("--out-json", type=Path, default=None,
                    help="Write structured result to this path")
    args = ap.parse_args()

    control_rows = load_jsonl(args.control)
    treatment_rows = load_jsonl(args.treatment)

    result = compute(control_rows, treatment_rows)
    print(json.dumps(result, indent=2))

    if args.out_json:
        args.out_json.write_text(json.dumps(result, indent=2) + "\n")

    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
    h = hashlib.sha256()
    for path in (args.control, args.treatment):
        h.update(path.read_bytes())
    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
    print(f"  {h.hexdigest()}")

    if not result["ceiling_pass"]:
        print(
            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
            file=sys.stderr,
        )
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())

```

## evaluator.rs B7-extra wiring diff (be42f43~1..HEAD)

```diff
diff --git a/experiments/minif2f_v4/src/bin/evaluator.rs b/experiments/minif2f_v4/src/bin/evaluator.rs
index 1bdb807..c7c0526 100644
--- a/experiments/minif2f_v4/src/bin/evaluator.rs
+++ b/experiments/minif2f_v4/src/bin/evaluator.rs
@@ -138,6 +138,21 @@ struct PputResult {
     gp_path: Option<String>,
     #[serde(skip_serializing_if = "Option::is_none")]
     gp_proof_file: Option<String>,
+    /// PPUT-CCL B7-extra (PREREG § 5.5 calibration treatment): set to
+    /// `Some(true)` iff the synthetic rollback short-circuit fired in
+    /// this run — i.e. SIMULATE_ROLLBACK_AT_TX_50=1 AND the run reached
+    /// `rollback_sim::ROLLBACK_TX_THRESHOLD`. Distinguishes calibration
+    /// treatment exits from natural max-tx exhaustions (both stamp the
+    /// same legacy halt path; this field is the disambiguator).
+    ///
+    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
+    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
+    /// vetoed loop, because the LLM calls for tx 51-199 never happened.
+    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
+    /// p_0 estimation is unaffected; downstream PPUT analysis on these
+    /// rows MUST honor this flag and exclude or specially treat them.
+    #[serde(skip_serializing_if = "Option::is_none")]
+    synthetic_short_circuit: Option<bool>,
     // Note (mid-term audit P0-B fix 2026-04-25): the prior Option versions of
     // total_run_token_count / failed_branch_count / total_wall_time_ms /
     // verified / pput_runtime / pput_verified / pput_m_verified were promoted
@@ -497,8 +512,44 @@ async fn run_swarm(
     let search_cap: u32 = std::env::var("SEARCH_CAP")
         .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
     let mut search_count: HashMap<String, u32> = HashMap::new();
+    // PPUT-CCL B7-extra (PREREG § 5.5): calibration treatment toggle.
+    // When enabled, every proposal at tx >= ROLLBACK_TX_THRESHOLD is
+    // synthetically vetoed. Constitutionally that is FC1-E18 (∏p=0 → Q_t)
+    // applied repeatedly; the run then exhausts at FC2-N22 HALT via
+    // `HaltReason::MaxTxExhausted`. We short-circuit at the threshold tx
+    // for efficiency — see `rollback_sim.rs` module header for why this
+    // is observably equivalent to running the loop to natural exhaustion.
+    let rollback_sim_on = minif2f_v4::rollback_sim::rollback_simulation_enabled();
+    if rollback_sim_on {
+        info!("[rollback_sim] PREREG § 5.5 calibration treatment ON \
+               (synthetic veto at tx >= {})", minif2f_v4::rollback_sim::ROLLBACK_TX_THRESHOLD);
+    }
 
     for tx in 0..max_transactions {
+        // PPUT-CCL B7-extra: short-circuit guard. Constitutional anchor
+        // FC1-E18 + FC2-N22 (existing MaxTxExhausted variant). Stamps
+        // tx_count at the threshold, not at max_transactions, so jsonl
+        // analysis can distinguish a calibration treatment exit from a
+        // real natural exhaustion.
+        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
+            info!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
+                   short-circuit to MaxTxExhausted exit", tx);
+            wc.mark_final_accept();
+            let mut result = make_pput(problem_file, &condition, model,
+                                       false, false, start, 0, 0,
+                                       tx as u64, Some(tool_dist), None,
+                                       None, None, None,
+                                       Some(acc.total_run_token_count()),
+                                       Some(acc.failed_branch_count),
+                                       wc.elapsed_ms());
+            // B7-extra disambiguator: distinguish this calibration-treatment
+            // exit from a natural max-tx exhaustion in downstream PPUT
+            // analysis. See PputResult::synthetic_short_circuit doc-comment
+            // for the cost-asymmetry note.
+            result.synthetic_short_circuit = Some(true);
+            return result;
+        }
+
         // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
         // bracket at the top of the FIRST tx (before chain/skill/board build
         // and before build_agent_prompt). Idempotent — only the first tx's
@@ -1198,6 +1249,9 @@ fn make_pput(
         gp_payload,
         gp_path,
         gp_proof_file,
+        // B7-extra: only the calibration-treatment short-circuit site mutates
+        // this to Some(true). Default = None (most callers).
+        synthetic_short_circuit: None,
     }
 }
 
diff --git a/experiments/minif2f_v4/src/lib.rs b/experiments/minif2f_v4/src/lib.rs
index 6730060..94f7f39 100644
--- a/experiments/minif2f_v4/src/lib.rs
+++ b/experiments/minif2f_v4/src/lib.rs
@@ -3,3 +3,4 @@ pub mod jsonl_schema;
 pub mod cost_aggregator;
 pub mod wall_clock;
 pub mod post_hoc_verifier;
+pub mod rollback_sim;

```

## handover/alignment/TRACE_MATRIX_v1_2026-04-25.md

# TRACE_MATRIX v1 — Constitutional Flowchart ↔ Rust Code (2026-04-25)

**Predecessor**: `TRACE_MATRIX_v0_2026-04-22.md`
**Trigger**: Phase B B7 (Trust Root + Boot freeze) shipped runtime code that (a) implements the Phase 11+ deferred FC3-N34 row and (b) introduces new files to the readonly base. Per CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素", v1 documents the new mappings before downstream work piles on top.

**Scope**: delta only. v0 rows that did not change are still authoritative — read v0 first.

**Legend** (unchanged from v0):
- ✅ well-aligned · ⚠️ partial · 🔨 missing-actionable · 📅 deferred Phase 11+ · 📄 docs-only

---

## § 1. Status flips (rows that changed since v0)

| FC Element ID | v0 Status | v1 Status | Justification |
|---|---|---|---|
| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |

No rows regressed. No previously ✅ rows changed.

---

## § 2. New code symbols added in B7 (FC anchors)

| Symbol | File:Line | FC Anchor | DocComment | Status |
|---|---|---|---|---|
| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
| `boot::parse_trust_root_section` | `src/boot.rs:91` | FC3-N34 (helper) | Y (line 86-90) | ✅ |
| `boot::TrustRootError` | `src/boot.rs:24` | FC3-N34 (failure variants) | Y (line 19-23) | ✅ |
| `fn main` (Trust Root verify call site) | `src/main.rs:11` | FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`) | Y (line 3-10) | ✅ |
| `rollback_sim::should_simulate_rollback` | `experiments/minif2f_v4/src/rollback_sim.rs:48` | FC1-E18 (∏p=0 → Q_t) repeated · FC2-N22 HALT (existing `MaxTxExhausted` variant) | Y (file header + fn doc) | ✅ |
| `rollback_sim::rollback_simulation_enabled` | `experiments/minif2f_v4/src/rollback_sim.rs:39` | same FC1-E18 + FC2-N22 anchor (env-var read for the predicate) | Y | ✅ |
| `rollback_sim::ROLLBACK_TX_THRESHOLD` | `experiments/minif2f_v4/src/rollback_sim.rs:34` | PREREG § 5.5 frozen constant (calibration anchor — not a runtime parameter) | Y | ✅ |
| `rollback_sim::ROLLBACK_ENV_VAR` | `experiments/minif2f_v4/src/rollback_sim.rs:38` | env-var name (mirrors PREREG § 5.5 `--simulate-rollback-at-tx-50`) | Y | ✅ |
| `evaluator.rs` short-circuit at line 503-518 | `experiments/minif2f_v4/src/bin/evaluator.rs:503` | FC1-E18 + FC2-N22 (call-site of the synthetic predicate) | Y (block comment) | ✅ |

Internal helpers (`has_section`, `strip_comment`, `unquote`, `hex_lower`) are private — no FC backlink required (per CLAUDE.md scoping to `pub` symbols).

---

## § 3. New `readonly` extensions (FC3-S3 subgraph membership change)

The constitutional FC3-S3 `readonly` subgraph contains FC3-N3 (`constitution as ground truth`) and FC3-N4 (`logs archive as ground truth`). PREREG § 1.8 (round-4 dual-audit PASS/PASS) extended this base for the PPUT-CCL experiment. Each addition is a research-protocol orphan with explicit constitutional justification (case-law / measurement-fidelity / pre-registration commitment).

| Path (manifest entry) | Justification |
|---|---|
| `src/kernel.rs` | FC3-N10 (`tape Q`) source — kernel topology immutability is a Law-1 invariant (Art. I.1) |
| `src/wal.rs` | FC3-N11 (`log`) implementation — append-only WAL is the constitutional logs-archive surface |
| `src/bus.rs` | FC1-N11/N13/N14 (`∏p`, `wtool`, `Q_{t+1}`) implementation — execution semantics that PPUT measures rest on |
| `src/drivers/llm_http.rs` | FC1-N7 (`δ / AI`) — cost source-of-truth (prompt_tokens / completion_tokens). Tampering with this defeats every C_i count |
| `src/sdk/prompt_guard.rs` | B6 PPUT-context-leak runtime gate — measurement-isolation invariant (no metric reaches agent prompt) |
| `experiments/minif2f_v4/src/lean4_oracle.rs` | FC1-N12 (∏p ground-truth oracle) — Lean ground-truth cannot drift mid-experiment |
| `experiments/minif2f_v4/src/cost_aggregator.rs` | B2 PPUT cost C_i computation — accounting invariant |
| `experiments/minif2f_v4/src/wall_clock.rs` | B3 PPUT time T_i computation — accounting invariant |
| `experiments/minif2f_v4/src/post_hoc_verifier.rs` | B4 verified-vs-runtime PPUT separation — accounting invariant |
| `experiments/minif2f_v4/src/jsonl_schema.rs` | B1 emit schema — auditable artifact format |
| `experiments/minif2f_v4/src/rollback_sim.rs` | B7-extra calibration toggle; PREREG § 5.5 commits a frozen `ROLLBACK_TX_THRESHOLD = 50` and a binary `SIMULATE_ROLLBACK_AT_TX_50` env var — tampering with either defeats the p_0 measurement |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | the wiring; tampering with it defeats every layer above |
| `constitution.md` | FC3-N3 (constitution as ground truth) — direct |
| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | sealed heldout split — pre-registration commitment per § 2.3 |
| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | the spec being committed to — pre-registration anchor |

`genesis_payload.toml` itself is **not** self-hashed (chicken-and-egg). The semantic anchor is the `[pput_accounting_0]` section content, not its hash. Section 6 below records this limitation.

**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).

---

## § 4. `src/boot.rs` is **not** in the Trust Root manifest

Conscious choice — recorded here so the next reviewer does not file it as an oversight:

- Trust Root's threat model = passive tamper between runs (file-system edits without recompile).
- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
- Adding `src/boot.rs` to its own manifest gives a slightly stronger passive-tamper guarantee (catches edits to boot.rs without recompile, e.g. on a deployed system where the binary and source are out of sync) at the cost of one more file to maintain.
- Phase B7 chooses the smaller surface. Phase C+ may revisit if signed-binary attestation lands.

---

## § 5. Boot panic ↔ FC mapping

`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:

- FC2-N22 HALT requires the kernel/bus to be initialized (HaltReason variants are emitted by `TuringBus::halt_with_reason`).
- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
- Closer match: FC3-E14 (`init → error → re-init → boot`). Boot-panic is the immediate-abort variant; the surrounding harness (batch runner, supervisord, shell wrapper) is the "re-init" actor.

See `OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` for the rationale to keep this as panic rather than promoting it into HaltReason. No constitution change requested.

---

## § 6. Updated stats (v1)

Compared to v0:
- ✅ count: **15 → 16** (+1: FC3-N34 promoted from 📅)
- 📅 deferred: **7 → 6** (-1)
- New orphan rows: **15 readonly extension paths** (above § 3) — each with constitutional justification, none requiring constitution change

Targets at end of Phase B (Stage 2/3 completion + B7):
- ✅ count: 38 + 1 = 39
- 📅/📄: 10 - 1 + 0 = 9
- 🔨/⚠️: 0 (per v0 § 4 actionable plan)

v1 does not address remaining v0 ⚠️ rows; those are Stage 2/3 work that has not yet landed (out of B7 scope).

---

## § 7. Outstanding work flagged for next alignment cycle

1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
2. ~~**TRACE_MATRIX of B7-extra (p_0 calibration toggle)**~~ — landed. Final implementation differs slightly from the original sketch in this section: the constitutional `bus.register_predicate(...)` API does not currently exist on `main` (it lives on the unmerged `phase-z-wtool-tools` branch — TRACE_MATRIX_v0 row FC1-N11 references it aspirationally). Rather than scope-creep B7-extra into reviving Phase Z, the synthetic predicate is implemented at the evaluator layer in `rollback_sim.rs` with an explicit short-circuit at the threshold tx. The constitutional anchor (FC1-E18 ∏p=0 → Q_t repeated, then FC2-N22 HALT via existing `MaxTxExhausted`) is unchanged; only the abstraction depth differs. Listed under § 2 above as ✅ entries.
3. **`src/boot.rs` self-hash decision** (§ 4 above) is open — Phase C+ revisit point.


---

## handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md

# B7-extra Abstraction-Depth Findings — 2026-04-25

**Status**: filed for Phase Z merge planning + Phase D ArchitectAI design constraints. Not blocking Phase B → C transition.
**Trigger**: thesis re-alignment audit (user, 2026-04-25 mid-session) found two abstraction-depth gaps where the implementation works correctly *within its scope* but is below the constitutionally cleanest design depth.
**Constitutional anchors**: thesis claims 7 ("white-box predicates settle state transitions") + 11 ("legible/enforceable/reversible state").

---

## Finding (A) — Synthetic predicate at evaluator-level, not bus-level

### What is

`SIMULATE_ROLLBACK_AT_TX_50=1` triggers a short-circuit at evaluator's `run_swarm` loop (`experiments/minif2f_v4/src/bin/evaluator.rs:503-518`) via `rollback_sim::should_simulate_rollback(tx, enabled)`. The kernel/bus is not informed; from `TuringBus`'s perspective tx 50 is just a tx that never happened.

### What it should be

Constitutionally cleanest = a `Predicate` impl `AlwaysRejectAfterTxN(50)` registered via `bus.register_predicate(...)` at swarm boot, returning `Reject` for all tx ≥ 50. Then the bus's existing `evaluate_predicates(ctx, payload)` routes through `Result::Vetoed` for every subsequent append, naturally exhausts at `max_transactions`, and emits `HaltReason::MaxTxExhausted` from the bus's own machinery.

This implementation properly exercises the **FC1-N11 ∏p product gate** (TRACE_MATRIX FC1-N11). The current evaluator-level short-circuit BYPASSES the ∏p gate for these synthetic ticks — the bus has no record of "we tried to append at tx 50-199 and were vetoed", because we never asked the bus.

### Why it's not done that way

`Predicate` trait + `bus.register_predicate` API live on the unmerged `phase-z-wtool-tools` branch (commit `74b2ce7`). TRACE_MATRIX_v0 row FC1-N11 references them aspirationally but they never landed on `main`. Reviving Phase Z to enable a one-line predicate registration in B7-extra would be ~10x scope-creep on a measurement task.

### Where the gap matters

| Layer | Effect |
|---|---|
| Production runs (control, no toggle) | **No effect** — production goes through the existing inline forbidden_patterns + Lean4Oracle white-box predicate surface (`src/bus.rs:183`) |
| Calibration treatment runs | The bus's tape never records 150 vetoed tx attempts; only the evaluator's tx_count==50 stamp + synthetic_short_circuit=true field signal what happened |
| Phase D ArchitectAI design | When ArchitectAI generates user-space artifacts, the *real* predicate path will be exercised by `bus.append → ∏p → wtool` for every artifact-generated proposal. Phase D MUST land before any production system can enforce capability-compilation invariants at FC1-N11 depth — Phase D's predicate surface IS the full Phase Z ambition restored |
| Phase B → C audit | Auditor should be told: "B7-extra synthetic veto is at evaluator layer; the bus's ∏p path is not exercised by calibration treatment. The control path (production) IS exercised, so the cost/time/progress measurements are valid for production-equivalent runs." |

### Action items

1. **Phase Z merge** (when scheduled): port `phase-z-wtool-tools` Predicate trait + `bus.register_predicate` + 3 default impls (`ForbiddenPatternPredicate`, `SorryPredicate`, `PayloadSizePredicate`) onto main.
2. **B7-extra refactor** (post-Phase Z merge): replace evaluator short-circuit with `bus.register_predicate(AlwaysRejectAfterTxN(ROLLBACK_TX_THRESHOLD))` at run_swarm boot when toggle is on. Drop the short-circuit. The synthetic_short_circuit flag becomes an artifact of how p_0 was historically calibrated; do not re-calibrate (Trust Root has the frozen value).
3. **Phase D design** (PREREG § 6 D2): document that ArchitectAI's user-space artifacts traverse the real ∏p path, so the abstraction depth gap closes naturally.

---

## Finding (B) — Cost asymmetry in synthetic-short-circuit treatment runs

### What is

When `synthetic_short_circuit=true` is stamped on a calibration treatment run's jsonl row, the `total_run_token_count` (C_i) reflects only the cost of tx 0-49 (the actual LLM calls that happened). A "true" 150-tx vetoed loop would have cost ~3x more (LLM calls every tx, all rejected).

### Why it's not perfect

PREREG § 5.5 conceives of `--simulate-rollback-at-tx-50` as "synthetic mid-run rollback mimicking worst-case stochastic edge case Phase E artifacts could trigger". A worst-case Phase E artifact would corrupt state but the loop would continue spending budget. We short-circuit instead — saving wall-clock but understating cost.

### Where the gap matters

| Analysis | Effect |
|---|---|
| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
| Per-row PPUT computation on calibration jsonl | Affected — `pput_runtime` and `pput_verified` for synthetic_short_circuit rows are 0/x = 0 (correct, since solved=false), but if anyone interpreted the cost C_i as "what a real Phase E artifact would have cost", that would be wrong |
| Aggregate cost statistics if these rows included | Significantly underreports |

### Mitigation in place

Field doc-comment on `PputResult::synthetic_short_circuit` warns:

> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.

### Action items

1. **Any future PPUT aggregation tool**: filter out `synthetic_short_circuit=true` rows or document explicitly that they are calibration-treatment-only.
2. **Phase B → C audit packet**: include this finding so external auditors know what they're looking at.
3. **Future calibration redesigns** (if needed): consider whether to (a) accept this asymmetry and remove the short-circuit (run all 200 tx, ~3x calibration cost ≈ +$10 over current $3-5), or (b) keep short-circuit and document.

---

## Why these findings are not thesis-level drift

Thesis claim 7 (white-box predicates settle state) is satisfied for the **production path**: every real proposal flows through `bus.append → forbidden_patterns + Lean4Oracle` (white-box, deterministic, inspectable). The synthetic predicate's measurement-only path bypasses this for the specific purpose of estimating p_0 (a measurement quantity). The thesis applies to the system's claim about how it handles real proposals, not to how it does internal calibration measurements on itself.

Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.

Both findings are **abstraction-depth** issues, recorded for Phase Z merge planning and Phase D design constraints, not blockers for Phase B → C transition.

---

## Cross-references

- `experiments/minif2f_v4/src/rollback_sim.rs` — module + 6 unit tests
- `experiments/minif2f_v4/src/bin/evaluator.rs:503-518` — short-circuit site
- `experiments/minif2f_v4/src/bin/evaluator.rs` `synthetic_short_circuit` field — disambiguator with cost-asymmetry warning
- `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md` § 7.2 — implementation note
- `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 5.5 — calibration protocol spec
- Unmerged branch `phase-z-wtool-tools` commit `74b2ce7` — full Phase Z Predicate trait + bus.register_predicate API


---

## handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md

# Thesis v2 audit — "feedback from ground truth" alignment

**Date**: 2026-04-25 (post user thesis update — added "feedback from ground truth" as physical anchor)
**Purpose**: re-audit current Phase B7-extra implementation against the new claim 7 ("white-box predicates settle state transitions based on STRICT FEEDBACK FROM GROUND TRUTH") and claim 8 ("failure logs MUST be ground-truth-validated before quarantine").
**Status**: 2 alignment gaps identified, both Phase D scope (consumer-side), neither blocks Phase B → C.

---

## Causal chain audit

```
Proposal (LLM)
  → Feedback from Ground Truth (Lean / FS / external compiler)
  → Logging (ground-truth-validated, isolated from active context)
  → Capability Compilation (Phase D)
  → ↑ H-VPPUT
```

For each link, where does the current Phase B7-extra system stand?

### Link 1 — Proposal (black-box LLM)
- ✅ Aligned. `client.generate` → `parse_agent_output` → `AgentOutput { tool: append/complete/invest/search }`. The LLM never settles state; it only proposes.

### Link 2 — Feedback from Ground Truth
- ✅ **Aligned for capability claims** (`complete` action triggering `oracle.verify_omega_detailed` at evaluator.rs:777,783,1016). Lean compiler returns objective verdict.
- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
- ⚠️ **Distinction**: non-capability transitions (`append` for tape comments, `invest` for market stakes, `search` for retrieval) are settled by **policy** (`forbidden_patterns`, `Sorry`, `PayloadSize`, market math) — NOT ground-truth. Per thesis spirit, this is acceptable: only capability claims need ground-truth. Tape comments are organisational; market stakes are economic; neither claims capability.
- 📅 Phase E sealed eval on heldout-54 = ground-truth on capability **generalization** (Gate H requirement).

### Link 3 — Logging (ground-truth-validated, isolated from active context)

Two surfaces:

#### 3a. WAL / `Ledger` (durable, structural)
- `EventType` enum **defines** OmegaInvoke / OmegaAccepted / OmegaRejected / OmegaError (`src/ledger.rs:155-158`)
- ❌ **GAP (Finding C)**: these Omega* events are **never emitted in production code**. Only RunStart, Append, Invest, MarketCreate, MarketResolve, RunEnd reach the ledger. The structured oracle verdict trail does not exist in the WAL.
- Phase D ArchitectAI reading WAL alone would see NO Lean verdicts; it would have to infer success from Append events and absence of Append events from "no proposal" — losing the distinction between "tried + rejected" and "never tried".

#### 3b. `recent_rejections` (in-memory queue, fed to next-tx agent prompt)
- On oracle reject (evaluator.rs:876): `bus.record_rejection(agent_id, class.label())`
- The CLASS LABEL is an abstracted classification from `classify_lean_error(err_detail)` — deterministic transform of ground-truth output, NOT raw oracle stderr.
- ✅ **Aligned for thesis spirit (claim 8)**: agents see abstracted labels (e.g., "TacticFailed"), not raw ground-truth content. The strong physical isolation = "raw oracle stderr is in stdout, not in agent prompt". Step-B v3 (C-022 shield) was specifically introduced to enforce this.
- ⚠️ **GAP (Finding D)**: `recent_rejections` mixes class labels from ground-truth rejects (Lean verdict) with class labels from policy rejects ("Forbidden pattern: decide", "Payload too long: ..."). Phase D consumer reading these labels has no field telling it which is which. Ground-truth rejects and policy rejects should be DIFFERENT signals to ArchitectAI.

#### 3c. `PputResult` jsonl emit (durable, per-run summary)
- ✅ **Aligned**. `verified: bool` field comes from `oracle.verify_omega_detailed` (B4 post_hoc_verifier separates `runtime_accepted` from `post_hoc_verified`). Each emitted row IS ground-truth-validated at run granularity.
- For PPUT-CCL Phase D, the calibration jsonl IS the ground-truth-validated log Phase D consumes for capability compilation.

### Link 4 — Capability Compilation (Phase D, not yet built)
- 📅 Will read PputResult jsonl + WAL + stdout/stderr
- The thesis-v2 critical requirement: ArchitectAI proposals MUST be tagged with which leg of the ground-truth feedback they're learning from. Phase D design must account for Findings C and D.

### Link 5 — ↑ H-VPPUT (held-out verified PPUT)
- 📅 Phase E sealed eval on heldout-54
- ✅ Trust Root + B7 Boot freeze + B7-extra p_0 calibration build the measurement scaffolding for this. North Star is achievable iff the chain above is honored.

---

## Findings (C, D — extending B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md)

### Finding C — WAL Omega* events never emitted

**What**: `EventType::{OmegaInvoke, OmegaAccepted, OmegaRejected, OmegaError}` are declared in `src/ledger.rs:155-158` but no production code path emits them. The WAL contains only structural events; ground-truth verdicts are visible in stdout/jsonl, not in the durable ledger.

**Why it matters per thesis-v2**: Claim 8 says ground-truth-validated failure logs must be quarantined for capability compilation. The WAL is the canonical durable log surface; if it lacks ground-truth events, Phase D ArchitectAI must reach into other surfaces (jsonl + stdout) to reconstruct ground-truth feedback. The reconstruction is doable but fragile.

**Severity**: Phase D scope. Phase B's calibration jsonl is correctly ground-truth-validated at run granularity; intra-run ground-truth events are missing from WAL but will surface in stderr (raw) + jsonl (`verified` + tool_dist counters).

**Action**: Phase D ArchitectAI design must specify which surface(s) it reads. Recommend: jsonl as primary ground-truth source, WAL as structural cross-reference, stderr as fallback for fine-grained diagnostics. OR (more invasive): emit OmegaInvoke/Accepted/Rejected/Error to ledger from `oracle.verify_omega_detailed` call sites (touches evaluator.rs but not bus.rs/kernel.rs — outside STEP_B_PROTOCOL restricted-file zone).

### Finding D — `recent_rejections` mixes policy + ground-truth class labels

**What**: `bus.record_rejection(author, reason)` is called from BOTH:
- Policy paths: forbidden_pattern (`bus.rs:186`), payload too long (`bus.rs:198`), too many lines (`bus.rs:205`), tool veto (`bus.rs:215`)
- Ground-truth path: oracle reject after classification (`evaluator.rs:876`)

The `recent_rejections(author, max)` consumer (line 594, fed into agent prompt) sees a flat list of class labels, no provenance tag.

**Why it matters per thesis-v2**: Claim 7 says white-box predicates settle state with ground-truth feedback. For agent next-tx learning (FC1's basic cycle), labels of any kind may be acceptable — agents adapt to any signal that helps them produce non-rejectable output. But for Phase D ArchitectAI compiling capability, the signal source matters: ArchitectAI should learn from ground-truth (Lean said this proof tactic doesn't work) differently than from policy (we banned `decide` for brute-force-prevention).

**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).

**Action**: when Phase D ArchitectAI consumer is built, tag bus rejections at the call site with `provenance: GroundTruth | Policy` — extend BusResult::Vetoed or add a separate `record_rejection_classified(author, reason, provenance)` API. Backwards-compatible if the existing `record_rejection` defaults provenance to Policy (since 4 of 5 call sites are policy).

---

## Why neither finding blocks Phase B → C

Phase B delivers MEASUREMENT SCAFFOLDING. The PPUT-CCL arc Gate B → C transition validates:
- A1-A5 ✓
- B1-B7 ✓ (Trust Root + Boot freeze landed)
- B7-extra (in flight): p_0 calibration via 576 runs

The calibration jsonl rows are **per-run ground-truth-validated** (`verified` field comes from Lean4Oracle). Findings C+D are about **intra-run** ground-truth event granularity (per-tx, per-proposal) — needed by Phase D ArchitectAI but not by Phase B's measurement gate.

Thesis-v2 alignment status:
- Phase B: ✅ aligned (per-run ground-truth-validated jsonl is the measurement surface)
- Phase C: should plan for Finding C/D before Phase D consumer is built
- Phase D: MUST close Findings C+D as a precondition

---

## Cross-references

- `~/.claude/.../memory/project_thesis.md` — thesis v2 with 5-step compile loop + 11 atomic claims
- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` — Findings A+B (abstraction depth)
- `src/ledger.rs:147-160` — EventType enum (Omega* declared, never emitted in production)
- `src/bus.rs:186,198,205,215,444` — record_rejection call sites (4 policy + 1 from evaluator at oracle reject)
- `experiments/minif2f_v4/src/bin/evaluator.rs:876` — bus.record_rejection at oracle reject
- `experiments/minif2f_v4/src/lean4_oracle.rs:112,177` — Lean ground-truth verdict surfaces
- PREREG § 1.7 ArtifactState lifecycle — Phase D consumer specification anchor


---

## PREREG § 5.5 (calibration protocol)

### 5.5 p_0 calibration protocol

`p_0` (baseline regression rate) — full calibration protocol:

```
Toggle:        evaluator --mode full         (control)
               evaluator --mode full --simulate-rollback-at-tx-50  (treatment;
                 forces a synthetic mid-run rollback at tx 50, mimicking the
                 worst-case stochastic edge case Phase E artifacts could trigger)
Sample:        FULL adaptation-144 problem set × Boltzmann seeds [31415, 2718]
                 = 288 paired runs.
Estimator:     for each problem p, define regression_p = 1 iff control SOLVED
                 (Lean PASS) AND treatment UNSOLVED for the SAME (problem, seed).
               Per-problem regression: max over the 2 seeds (worst case).
               p_0 = sum_p regression_p / 144.
Schedule:      Phase B7 (mandatory; not deferrable to Phase D). Phase D cannot
               start without p_0 frozen.
Freeze point:  end of Phase B7. Value written to
                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
               with SHA-256 of the calibration jsonl in
                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
Audit trail:   raw calibration jsonl committed to
                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
Ceiling:       if measured p_0 > 0.10, ABORT — toggle too aggressive or system
               too brittle. NOT an opportunity for j-RR auto-pass.
Dual audit:    p_0 calibration result in Phase B → C audit packet.
```

This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.

### 5.6 Family total (clean restatement)


---

## PREREG § 1.7-1.8 (ArtifactState + Trust Root)

### 1.7 ArtifactState — layered confidence (Patch A from Gemini DeepThink 2026-04-26)

For ANY user-space artifact Δ generated by ArchitectAI (broadcast rule, doc, skill, retrievable tool):

```
ArtifactState(Δ) ∈ {Accepted, Quarantined, Certified, Reverted}

Accepted    : Δ has just passed § 3 metering + § 3.5 + § 3.5.1 AuditorAI
              content meta-predicate battery; eligible for human review (§ 6 D5).
Quarantined : Δ has been Accepted AND user-approved (§ 6 D5) AND has measured
              positive ΔPPUT on meta_validation set with N_use ≥ 3.
              Quarantined artifacts are callable in adaptation / meta_val runs
              and ENTER Phase E heldout sealed eval (§ 6 E2). Their meta_val
              effects are MEASURED but provisional — meta_val ΔPPUT > 0 is a
              FILTER for Phase E entry, not a certification.
Certified   : Δ entered Phase E AND survived (round 4 — three inferential + one
              guardrail + rollback): j-PPUT, j-FAR, j-CPR all reject null at
              family-corrected α (per § 5.3 + § 9), AND j-RR ≤ p_0 point check
              (per § 5.4 — descriptive guardrail, NOT inferential), AND the
              rollback sub-eval confirms Rollbackable=1 (per § 6 E2). Counts
              toward WBCG_PPUT (§ 1.6 + § 7 Gate H).
Reverted    : Δ failed any of the three primary tests, OR violated the j-RR
              point-check guardrail, OR caused regression in post-Phase-E
              audit, OR failed the rollback sub-eval (§ 6 E2). Permanently
              flagged; rollback executed; cannot re-enter promotion within
              this arc (resubmission requires new artifact_id + design change
              per C-035).
```

Critical invariants:
- **Task** Progress stays binary (`Lean accepts ⇒ 1`, else `0`); 4-state machine applies ONLY to artifacts. Mixing the two would corrupt PPUT (Lean gives a perfect predicate; artifact certification is a non-perfect-predicate process — they must not be conflated).
- **WBCG_PPUT counts only `Certified`**. `Quarantined` artifacts contribute to PPUT measurement but not to capability-compilation success. This blocks "degenerate caching" — high-frequency-but-neutral artifacts cannot inflate WBCG.
- **`Reverted` is permanent**. Once an artifact is reverted, it cannot be re-promoted within this arc; resubmission requires a NEW artifact_id with documented design change (per C-035 zero-tolerance).

### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)

The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):

```
Trust Root := {
  src/kernel.rs,                    -- bus / kernel core
  src/wal.rs,                       -- write-ahead log
  src/bus.rs,                       -- predicate registration / capability gates
  experiments/minif2f_v4/src/lean4_oracle.rs,  -- ground-truth predicate
  constitution.md,                  -- normative spec
  cases/C-*.yaml,                   -- common-law precedent (append-only by human)
  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
  handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,  -- frozen splits
  handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md     -- this file
}
```

Enforcement mechanism (Phase B7 work item):
1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
3. ArchitectAI write tool whitelists `user_space/Δ_*/`; any path outside whitelist returns `EPERM` from the syscall layer (not a soft refusal at the LLM prompt layer).
4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.

Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.

This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.

## 2. three-split protocol (irreversible)


---

Now give your INDEPENDENT pre-batch audit. Cite §/file:line for every finding.

codex
I’ll audit against the actual workspace, not only the pasted snippets, so the file and line citations are grounded. I’m first checking repo instructions and then I’ll trace the calibration path, trust-root manifest, and runner/estimator behavior.
exec
/bin/bash -lc 'pwd && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
 M rules/enforcement.log
?? handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md
?? handover/audits/run_codex_b7_extra_prebatch_audit.sh
?? handover/audits/run_gemini_b7_extra_prebatch_audit.py

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
CLAUDE.md

exec
/bin/bash -lc "sed -n '1,260p' CLAUDE.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TuringOS v4

## What
Silicon-Native Microkernel for LLM Formal Verification Swarm.
Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.

## Why
- 唯一对齐文档: `constitution.md` (反奥利奥架构)
- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)

## Code Standard (Art. I.1 + C-004 + C-027)
- `cargo check` / `cargo test` 必过；`.env` 永不 commit
- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码

## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
- Generator ≠ Evaluator：代码作者不可是唯一审计者
- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
- 宪法违规立即 BLOCKER，不可延期、不可"可接受"

## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪

## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）

## Alignment Standard (Art. IV + C-069)
- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
  (后续 rev: `TRACE_MATRIX_vN.md`)
- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法

## Common Law (宪法 + 判例)
宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）

## Docs (按需加载)
| 文档 | 何时加载 |
|------|---------|
| `docs/architecture.md` | 修改 src/ 核心模块时 |
| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
| `docs/hardware.md` | SSH/部署/远程操作时 |
| `docs/experiments.md` | 创建或运行实验时 |
| `docs/rules.md` | 触发规则或修改规则时 |

## User
独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.

codex
The project’s local audit rule confirms the conservative gate: constitutional violations block, and VETO dominates CHALLENGE/PASS. I’m now checking the exact implementation paths and scripts for silent failure or bypass conditions before forming the verdict.
exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '120,170p;490,540p;760,890p;990,1030p;1180,1265p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   120	    boltzmann_seed: Option<u64>,
   121	    // C-036 harness telemetry: bypass-detection signals for multi-agent runs.
   122	    // tool_dist: counts per tool ({complete, append, invest, parse_fail, llm_err}).
   123	    //   complete=N append=0 ⇒ tape-bypass (Art. II.1 broadcast unused).
   124	    // unique_payload_ratio: distinct OMEGA payloads / total OMEGA attempts.
   125	    //   <0.30 ⇒ catastrophic agent correlation (F-2026-04-18-01).
   126	    #[serde(skip_serializing_if = "Option::is_none")]
   127	    tool_dist: Option<HashMap<String, u32>>,
   128	    #[serde(skip_serializing_if = "Option::is_none")]
   129	    unique_payload_ratio: Option<f64>,
   130	    // Phase 0 (C-039 candidate): persisted full proof + path so external verifiers can
   131	    // re-run `lean --stdin` from disk artifacts alone, without trusting in-memory runtime.
   132	    // gp_payload = the exact text fed to oracle.verify_omega_detailed at OMEGA accept.
   133	    // gp_path = "alone" (payload self-contained) or "tape+payload" (Art. IV dual-path 2).
   134	    // gp_proof_file = relative path to the standalone .lean archive (problem + proof).
   135	    #[serde(skip_serializing_if = "Option::is_none")]
   136	    gp_payload: Option<String>,
   137	    #[serde(skip_serializing_if = "Option::is_none")]
   138	    gp_path: Option<String>,
   139	    #[serde(skip_serializing_if = "Option::is_none")]
   140	    gp_proof_file: Option<String>,
   141	    /// PPUT-CCL B7-extra (PREREG § 5.5 calibration treatment): set to
   142	    /// `Some(true)` iff the synthetic rollback short-circuit fired in
   143	    /// this run — i.e. SIMULATE_ROLLBACK_AT_TX_50=1 AND the run reached
   144	    /// `rollback_sim::ROLLBACK_TX_THRESHOLD`. Distinguishes calibration
   145	    /// treatment exits from natural max-tx exhaustions (both stamp the
   146	    /// same legacy halt path; this field is the disambiguator).
   147	    ///
   148	    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
   149	    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
   150	    /// vetoed loop, because the LLM calls for tx 51-199 never happened.
   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
   152	    /// p_0 estimation is unaffected; downstream PPUT analysis on these
   153	    /// rows MUST honor this flag and exclude or specially treat them.
   154	    #[serde(skip_serializing_if = "Option::is_none")]
   155	    synthetic_short_circuit: Option<bool>,
   156	    // Note (mid-term audit P0-B fix 2026-04-25): the prior Option versions of
   157	    // total_run_token_count / failed_branch_count / total_wall_time_ms /
   158	    // verified / pput_runtime / pput_verified / pput_m_verified were promoted
   159	    // to non-Optional v2 fields above. Phase B always has values for them.
   160	}
   161	
   162	#[tokio::main]
   163	async fn main() {
   164	    env_logger::init();
   165	    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
   166	    // Control binary (main branch) has no such set_var → classifier_version serializes as None.
   167	    // This makes it impossible to mistake one binary for the other in post-hoc analysis.
   168	    std::env::set_var("CLASSIFIER_VERSION", CLASSIFIER_VERSION);
   169	
   170	    let args: Vec<String> = std::env::args().collect();
   490	        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
   491	        format!("Agent_{}:skill{}:t={:.2}", i, s, t)
   492	    }).collect();
   493	    info!("[swarm/{}] {}", condition, agent_cfg.join(" "));
   494	
   495	    // C-036 telemetry counters.
   496	    let mut tool_dist: HashMap<String, u32> = HashMap::new();
   497	    let mut omega_payload_hashes: HashSet<u64> = HashSet::new();
   498	    let mut omega_attempts: u32 = 0;
   499	    let mut zero_ticks_run: u32 = 0;
   500	    let mut zero_tick_warned = false;
   501	    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
   502	    // across all proposals (winning + failed branches). Read at terminal
   503	    // make_pput sites and stamped on the emitted jsonl row.
   504	    let mut acc = RunCostAccumulator::new();
   505	    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
   506	    // call. Opened on first tx's prompt build, closed before each return.
   507	    let mut wc = RunWallClock::new();
   508	    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
   509	    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
   510	    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
   511	    // from the tool list so agents stop wasting budget on name-match misses.
   512	    let search_cap: u32 = std::env::var("SEARCH_CAP")
   513	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   514	    let mut search_count: HashMap<String, u32> = HashMap::new();
   515	    // PPUT-CCL B7-extra (PREREG § 5.5): calibration treatment toggle.
   516	    // When enabled, every proposal at tx >= ROLLBACK_TX_THRESHOLD is
   517	    // synthetically vetoed. Constitutionally that is FC1-E18 (∏p=0 → Q_t)
   518	    // applied repeatedly; the run then exhausts at FC2-N22 HALT via
   519	    // `HaltReason::MaxTxExhausted`. We short-circuit at the threshold tx
   520	    // for efficiency — see `rollback_sim.rs` module header for why this
   521	    // is observably equivalent to running the loop to natural exhaustion.
   522	    let rollback_sim_on = minif2f_v4::rollback_sim::rollback_simulation_enabled();
   523	    if rollback_sim_on {
   524	        info!("[rollback_sim] PREREG § 5.5 calibration treatment ON \
   525	               (synthetic veto at tx >= {})", minif2f_v4::rollback_sim::ROLLBACK_TX_THRESHOLD);
   526	    }
   527	
   528	    for tx in 0..max_transactions {
   529	        // PPUT-CCL B7-extra: short-circuit guard. Constitutional anchor
   530	        // FC1-E18 + FC2-N22 (existing MaxTxExhausted variant). Stamps
   531	        // tx_count at the threshold, not at max_transactions, so jsonl
   532	        // analysis can distinguish a calibration treatment exit from a
   533	        // real natural exhaustion.
   534	        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
   535	            info!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
   536	                   short-circuit to MaxTxExhausted exit", tx);
   537	            wc.mark_final_accept();
   538	            let mut result = make_pput(problem_file, &condition, model,
   539	                                       false, false, start, 0, 0,
   540	                                       tx as u64, Some(tool_dist), None,
   760	                                    .map(|n| n.payload.clone())
   761	                                    .collect::<Vec<_>>()
   762	                                    .join("\n");
   763	                                let tape_len = bus.kernel.tape.time_arrow().len();
   764	                                // C-036: track payload diversity over what agent proposed.
   765	                                let mut h = std::collections::hash_map::DefaultHasher::new();
   766	                                payload.hash(&mut h);
   767	                                omega_payload_hashes.insert(h.finish());
   768	                                omega_attempts += 1;
   769	                                info!("[tx {}] OMEGA claim by {} (tape_nodes={}, payload_len={})",
   770	                                      tx, agent_id, tape_len, payload.len());
   771	                                let oracle = Lean4Oracle::new(
   772	                                    problem_statement.to_string(),
   773	                                    theorem_name.to_string(),
   774	                                    lean_path.to_string(),
   775	                                );
   776	                                // Path 1: payload alone
   777	                                let r_alone = oracle.verify_omega_detailed(payload);
   778	                                let (full_proof, path_choice, r_final) = match &r_alone {
   779	                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
   780	                                    _ if !tape_chain.is_empty() => {
   781	                                        // Path 2: tape + payload
   782	                                        let combined = format!("{}\n{}", tape_chain, payload);
   783	                                        let r_combined = oracle.verify_omega_detailed(&combined);
   784	                                        if matches!(r_combined, Ok((true, _))) {
   785	                                            *tool_dist.entry("complete_via_tape".into()).or_insert(0) += 1;
   786	                                        }
   787	                                        (combined, "tape+payload", r_combined)
   788	                                    }
   789	                                    _ => (payload.clone(), "alone", r_alone.clone()),
   790	                                };
   791	                                // PPUT-CCL B3: close bracket AFTER both Lean verify paths return.
   792	                                // Soft Law (Phase C) cannot exit ahead of verify-time accounting.
   793	                                wc.mark_final_accept();
   794	                                match r_final {
   795	                                    Ok((true, _)) => {
   796	                                        // PPUT-CCL B2: this proposal verified — flip the failed
   797	                                        // record made at parse time into the run's accepted slot.
   798	                                        acc.flip_last_failed_to_accepted();
   799	                                        // Phase 0 (C-039): persist the winning artifact so external
   800	                                        // verifiers can re-run lean from disk alone.
   801	                                        let preview: String = full_proof.chars().take(500).collect();
   802	                                        info!(">>> OMEGA ACCEPTED <<< (path={}, payload[0..500]={:?})",
   803	                                              path_choice, preview);
   804	                                        let proof_file = persist_proof_artifact(
   805	                                            problem_file, &theorem_name, &problem_statement,
   806	                                            &full_proof, path_choice, agent_id,
   807	                                        );
   808	                                        // Phase 2.1 (C-043 candidate): mandatory wtool. Art. IV says
   809	                                        // `∏p = 1 ⟹ Q_{t+1} = wtool(output)`. Before halting, write
   810	                                        // the winning payload as a tape node through the standard
   811	                                        // append pipeline. This automatically fires founder grant
   812	                                        // (Phase 2 reward-pull) for the winning author and makes
   813	                                        // every solve end with a canonical tape node on the GP.
   814	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
   815	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
   816	                                        // Use oracle-blessed path: Lean has already accepted this
   817	                                        // payload, so bus-level forbidden_patterns and size caps
   818	                                        // would only re-reject legitimate tactics (e.g. `omega`,
   819	                                        // `decide` used inside a verified proof — not brute-force).
   820	                                        let omega_node_id = match bus.append_oracle_accepted(
   821	                                            agent_id, payload, parent.as_deref(),
   822	                                        ) {
   823	                                            Ok(BusResult::Appended { node_id }) => Some(node_id),
   824	                                            Ok(BusResult::Vetoed { reason }) => {
   825	                                                warn!("[art-iv] OMEGA wtool VETO (unexpected after oracle accept): {}", reason);
   826	                                                None
   827	                                            }
   828	                                            _ => None,
   829	                                        };
   830	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
   831	                                            .filter_map(|id| bus.kernel.tape.get(id))
   832	                                            .map(|n| n.payload.len() as u64)
   833	                                            .sum();
   834	                                        // C-012: gp_tokens reflects the actual tape (now containing
   835	                                        // the winner), no double-count needed.
   836	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
   837	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
   838	                                        let gp_nodes = gp.len();
   839	                                        if omega_node_id.is_some() {
   840	                                            info!("[art-iv] OMEGA written as tape node; gp_nodes={}", gp_nodes);
   841	                                        }
   842	                                        bus.halt_and_settle(&gp).ok();
   843	                                        // Phase 4: persist wallet state so next problem's run
   844	                                        // inherits carried-over balances (reputation).
   845	                                        if let Some(ref wp) = wallet_state_path {
   846	                                            if let Some(w) = bus.tools.iter()
   847	                                                .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
   848	                                            {
   849	                                                if let Err(e) = w.save_to_disk(wp) {
   850	                                                    warn!("[wallet] save failed to {:?}: {}", wp, e);
   851	                                                }
   852	                                            }
   853	                                        }
   854	                                        let upr = if omega_attempts > 0 {
   855	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
   856	                                        } else { None };
   857	                                        // P0-A: Phase B swarm complete — runtime gate IS the
   858	                                        // Lean verify_omega_detailed call we just consumed
   859	                                        // (Ok((true, _))). Both legs hold. Phase C Soft Law
   860	                                        // would inject `verify_post_hoc(&oracle, &full_proof)`
   861	                                        // here and pass its result as post_hoc_verified.
   862	                                        return make_pput(problem_file, &condition, model,
   863	                                                        true, true,
   864	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
   865	                                                        Some(tool_dist), upr,
   866	                                                        Some(full_proof.clone()),
   867	                                                        Some(path_choice.to_string()),
   868	                                                        proof_file,
   869	                                                        Some(acc.total_run_token_count()),
   870	                                                        Some(acc.failed_branch_count),
   871	                                                        wc.elapsed_ms());
   872	                                    }
   873	                                    Ok((false, err_detail)) => {
   874	                                        // Step-B v3: classify + record class label (C-022 shield).
   875	                                        let class = classify_lean_error(&err_detail);
   876	                                        bus.record_rejection(agent_id, class.label());
   877	                                        // PPUT-CCL B2: rejection error feeds back into next prompt's
   878	                                        // recent_rejections — count those bytes against C_i.
   879	                                        acc.record_tool_stdout(&err_detail);
   880	                                        let preview: String = payload.chars().take(300).collect();
   881	                                        warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
   882	                                    }
   883	                                    Err(e) => {
   884	                                        warn!("[tx {}] OMEGA oracle error: {}", tx, e);
   885	                                    }
   886	                                }
   887	                            }
   888	                        }
   889	                        "invest" => {
   890	                            *tool_dist.entry("invest".into()).or_insert(0) += 1;
   990	                                }
   991	                            }
   992	                        }
   993	                        "step" => {
   994	                            // Phase 7 (C-043+ Turing δ-step): submit ONE tactic,
   995	                            // oracle classifies the accumulated tape+tactic prefix
   996	                            // as Complete / PartialOk / Reject. Writes a tape node
   997	                            // on PartialOk and Complete so the DAG grows one cell
   998	                            // at a time — the Art. IV semantics Turing 1936 defines.
   999	                            *tool_dist.entry("step".into()).or_insert(0) += 1;
  1000	                            if let Some(tactic) = &action.payload {
  1001	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
  1002	                                    .filter_map(|id| bus.kernel.tape.get(id))
  1003	                                    .map(|n| n.payload.clone())
  1004	                                    .collect::<Vec<_>>()
  1005	                                    .join("\n");
  1006	                                let prefix = if tape_chain.is_empty() {
  1007	                                    tactic.clone()
  1008	                                } else {
  1009	                                    format!("{}\n{}", tape_chain, tactic)
  1010	                                };
  1011	                                let oracle = Lean4Oracle::new(
  1012	                                    problem_statement.to_string(),
  1013	                                    theorem_name.to_string(),
  1014	                                    lean_path.to_string(),
  1015	                                );
  1016	                                let verdict = oracle.verify_partial(&prefix);
  1017	                                // PPUT-CCL B3: close bracket after step-verify returns.
  1018	                                wc.mark_final_accept();
  1019	                                match verdict {
  1020	                                    PartialVerdict::Complete => {
  1021	                                        acc.flip_last_failed_to_accepted();
  1022	                                        info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
  1023	                                              bus.kernel.tape.time_arrow().len() + 1);
  1024	                                        let proof_file = persist_proof_artifact(
  1025	                                            problem_file, &theorem_name, &problem_statement,
  1026	                                            &prefix, "per_tactic", agent_id,
  1027	                                        );
  1028	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1029	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
  1030	                                        let _ = bus.append_oracle_accepted(
  1180	        compute_progress_verified(runtime_accepted, post_hoc_verified);
  1181	    let pput_runtime = compute_pput(progress_runtime, c_i, t_i);
  1182	    let pput_verified = compute_pput(progress_verified, c_i, t_i);
  1183	    let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i);
  1184	
  1185	    // V2 fields read from env (per-process globals).
  1186	    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
  1187	        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
  1188	                   (Phase B convention; pre-registration requires SPLIT \
  1189	                   for Phase C+ ablation runs)");
  1190	        "adaptation".to_string()
  1191	    });
  1192	    let mode = std::env::var("MODE").unwrap_or_else(|_| "full".to_string());
  1193	    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
  1194	        .unwrap_or_else(|_| model.to_string());
  1195	    let git_sha = build_sha.clone().unwrap_or_default();
  1196	    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();
  1197	
  1198	    // problem_id = basename without .lean
  1199	    let problem_id = std::path::Path::new(problem)
  1200	        .file_stem()
  1201	        .and_then(|s| s.to_str())
  1202	        .unwrap_or(problem)
  1203	        .to_string();
  1204	    // run_id = condition + problem_id + ts (collision-free for sequential runs)
  1205	    let ts = std::time::SystemTime::now()
  1206	        .duration_since(std::time::UNIX_EPOCH)
  1207	        .map(|d| d.as_millis())
  1208	        .unwrap_or(0);
  1209	    let run_id = format!("{}_{}_{}", condition, problem_id, ts);
  1210	
  1211	    PputResult {
  1212	        // ── B1 v2 schema fields ──
  1213	        schema_version: "v2.0".to_string(),
  1214	        run_id,
  1215	        problem_id,
  1216	        solved: runtime_accepted,
  1217	        split,
  1218	        verified: post_hoc_verified,
  1219	        golden_path_token_count: gp_tokens,
  1220	        total_run_token_count: c_i,
  1221	        total_wall_time_ms: t_i,
  1222	        progress: progress_verified,
  1223	        pput_runtime,
  1224	        pput_verified,
  1225	        pput_m_verified,
  1226	        failed_branch_count: failed_count,
  1227	        // Phase B placeholders — Phase C+ wires these as the modes activate.
  1228	        rollback_count: 0,
  1229	        far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
  1230	        model_snapshot,
  1231	        git_sha,
  1232	        binary_sha256,
  1233	        mode,
  1234	        // ── Legacy diagnostic fields ──
  1235	        problem: problem.to_string(),
  1236	        condition: condition.to_string(),
  1237	        model: model.to_string(),
  1238	        has_golden_path: has_gp,
  1239	        time_secs: elapsed,
  1240	        pput,
  1241	        gp_token_count: gp_tokens,
  1242	        gp_node_count: gp_nodes,
  1243	        tx_count,
  1244	        build_sha,
  1245	        classifier_version,
  1246	        boltzmann_seed,
  1247	        tool_dist,
  1248	        unique_payload_ratio,
  1249	        gp_payload,
  1250	        gp_path,
  1251	        gp_proof_file,
  1252	        // B7-extra: only the calibration-treatment short-circuit site mutates
  1253	        // this to Some(true). Default = None (most callers).
  1254	        synthetic_short_circuit: None,
  1255	    }
  1256	}
  1257	
  1258	/// Phase 0 (C-039 candidate): persist a self-contained, re-verifiable proof artifact.
  1259	/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
  1260	/// the exact code that the Lean oracle accepted. An external verifier can run
  1261	/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
  1262	/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
  1263	fn persist_proof_artifact(
  1264	    problem_file: &str, theorem_name: &str, problem_statement: &str,
  1265	    full_proof: &str, path_choice: &str, agent_id: &str,

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/rollback_sim.rs | sed -n '1,140p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL Phase B B7-extra — synthetic rollback simulation.
     2	//
     3	// Constitutional anchor (TRACE_MATRIX_v1 § 7.2): the
     4	// `--simulate-rollback-at-tx-50` toggle (PREREG § 5.5) is realized as
     5	// "every proposal from tx 50 onward is vetoed", which is constitutionally
     6	// the **FC1-E18** edge (∏p=0 → Q_t preservation) repeated for tx
     7	// 50..max_transactions. The run then exhausts naturally and exits via
     8	// the existing **FC2-N22 HALT** with `HaltReason::MaxTxExhausted` — no
     9	// new HaltReason variant is introduced and no new constitutional surface
    10	// is created.
    11	//
    12	// For efficiency, the swarm loop short-circuits at tx == threshold
    13	// instead of running ~150 guaranteed-vetoed iterations. The short-circuit
    14	// is observably equivalent: identical exit state, identical cost
    15	// accumulator (no extra LLM calls would have happened in vetoed tx),
    16	// identical wall-clock close. The only observable difference is
    17	// `tx_count` stamped at threshold rather than `max_transactions` — a
    18	// useful diagnostic signal that distinguishes a calibration-treatment
    19	// run from a real exhaustion.
    20	//
    21	// Threat model: the threshold is fixed at 50 per PREREG § 5.5 frozen
    22	// spec. The env var `SIMULATE_ROLLBACK_AT_TX_50` is a binary toggle
    23	// (`"1"` to enable). The threshold is intentionally not exposed as a
    24	// runtime parameter — pre-registration discipline (C-070) requires that
    25	// what we calibrate is exactly what is committed in genesis_payload.toml.
    26	
    27	/// PREREG § 5.5: the synthetic rollback fires at this transaction index
    28	/// in the swarm loop. Frozen — must match the value committed in the
    29	/// pre-registration hash chain.
    30	pub const ROLLBACK_TX_THRESHOLD: u64 = 50;
    31	
    32	/// Env var name read by the evaluator. `"1"` enables the toggle; any
    33	/// other value (or absence) is "off".
    34	pub const ROLLBACK_ENV_VAR: &str = "SIMULATE_ROLLBACK_AT_TX_50";
    35	
    36	/// True iff the calibration treatment toggle is enabled in the current
    37	/// process environment.
    38	pub fn rollback_simulation_enabled() -> bool {
    39	    std::env::var(ROLLBACK_ENV_VAR)
    40	        .ok()
    41	        .as_deref()
    42	        == Some("1")
    43	}
    44	
    45	/// True iff the swarm loop should short-circuit at this `tx` index. The
    46	/// short-circuit is constitutionally equivalent to "synthetic ∏p=0 from
    47	/// here, naturally exhaust at `max_transactions`" — see module header.
    48	///
    49	/// `enabled` is a parameter (not read from env) so unit tests can drive
    50	/// the predicate without process-global state.
    51	pub fn should_simulate_rollback(tx: u64, enabled: bool) -> bool {
    52	    enabled && tx == ROLLBACK_TX_THRESHOLD
    53	}
    54	
    55	#[cfg(test)]
    56	mod tests {
    57	    use super::*;
    58	
    59	    #[test]
    60	    fn fires_at_threshold_when_enabled() {
    61	        assert!(should_simulate_rollback(50, true));
    62	    }
    63	
    64	    #[test]
    65	    fn does_not_fire_before_threshold() {
    66	        for tx in [0_u64, 1, 25, 49] {
    67	            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
    68	        }
    69	    }
    70	
    71	    #[test]
    72	    fn does_not_fire_after_threshold() {
    73	        // Constitutional reading: at tx > 50, the synthetic ∏p has already
    74	        // begun returning Reject; the loop continues but accumulates no
    75	        // progress. Short-circuit fires exactly once at tx == threshold,
    76	        // not on every tx after.
    77	        for tx in [51_u64, 60, 100, 199] {
    78	            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
    79	        }
    80	    }
    81	
    82	    #[test]
    83	    fn never_fires_when_disabled() {
    84	        for tx in [0_u64, 49, 50, 51, 199] {
    85	            assert!(!should_simulate_rollback(tx, false), "tx={tx}");
    86	        }
    87	    }
    88	
    89	    #[test]
    90	    fn threshold_constant_matches_prereg() {
    91	        // PREREG § 5.5 freezes the threshold at 50. If this assertion ever
    92	        // fails, the codebase has drifted from the pre-registration hash
    93	        // chain — recompute Trust Root and dual-audit before continuing.
    94	        assert_eq!(ROLLBACK_TX_THRESHOLD, 50);
    95	    }
    96	
    97	    #[test]
    98	    fn env_var_name_matches_prereg() {
    99	        // PREREG § 5.5 names the toggle `--simulate-rollback-at-tx-50`;
   100	        // the env-var equivalent (the v4 evaluator does not use clap)
   101	        // mirrors that name uppercased + underscored.
   102	        assert_eq!(ROLLBACK_ENV_VAR, "SIMULATE_ROLLBACK_AT_TX_50");
   103	    }
   104	}

exec
/bin/bash -lc "nl -ba src/boot.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL Phase B B7 — Trust Root + Boot freeze (PREREG § 1.8 + § 7).
     2	//
     3	// Constitutional anchor: FC3-S3 `readonly` subgraph (constitution.md
     4	// line 670, system-level flowchart). The constitutional readonly base
     5	// is {constitution-as-ground-truth, logs-archive-as-ground-truth}; B7
     6	// extends this base per PREREG § 1.8 to also cover the case-law glob,
     7	// pre-registration spec, heldout splits, and the PPUT accounting layer.
     8	// TRACE_MATRIX_v0 row FC3-N34 was 📅 Phase 11+ ("FS-level readonly
     9	// check at init") — B7 implements it via SHA-256 manifest verification.
    10	// See `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md`.
    11	//
    12	// At Boot we hash every tracked file and compare against the
    13	// `[trust_root]` manifest in `genesis_payload.toml`. Any mismatch =>
    14	// `TrustRootError::Tampered { .. }`. `src/main.rs` panics with
    15	// `TRUST_ROOT_TAMPERED`.
    16	//
    17	// Manifest derivation (Phase B7, independently re-derived from PREREG
    18	// § 1.8 + B2-B4 mid-term audit recommendation + B6 prompt_guard add):
    19	// see header comment in `genesis_payload.toml`.
    20	//
    21	// TOML parsing is hand-rolled (~30 LOC). The manifest format is flat:
    22	// section header + `"path" = "hash"` lines. Adding a `toml` crate
    23	// dependency would drag in ~5 transitive crates for what we can do
    24	// in-line; compression principle (CLAUDE.md "反奥利奥架构") wins.
    25	
    26	use sha2::{Digest, Sha256};
    27	use std::fs;
    28	use std::path::{Path, PathBuf};
    29	
    30	/// TRACE_MATRIX FC3-N34: failure variants of the readonly-guard verification.
    31	/// Constitutional role = the diagnostic surface that distinguishes
    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
    33	/// `GenesisParse` (manifest itself unreadable, also a violation but a
    34	/// different fix path).
    35	#[derive(Debug)]
    36	pub enum TrustRootError {
    37	    GenesisRead(std::io::Error),
    38	    GenesisParse(String),
    39	    SectionMissing(&'static str),
    40	    FileRead { path: PathBuf, err: std::io::Error },
    41	    Tampered { path: PathBuf, expected: String, actual: String },
    42	}
    43	
    44	impl std::fmt::Display for TrustRootError {
    45	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    46	        match self {
    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
    51	            Self::Tampered { path, expected, actual } => write!(
    52	                f,
    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
    54	                path.display(), expected, actual
    55	            ),
    56	        }
    57	    }
    58	}
    59	
    60	impl std::error::Error for TrustRootError {}
    61	
    62	/// TRACE_MATRIX FC3-N34: implementation of the constitutional `readonly`
    63	/// subgraph (constitution.md FC3, system-level flowchart). Verifies every
    64	/// tracked file's SHA-256 against the `genesis_payload.toml [trust_root]`
    65	/// manifest at Boot. Mismatch => Boot abort; the readonly guarantee that
    66	/// the constitution requires of {constitution, logs} (extended per PREREG
    67	/// § 1.8 to the full PPUT-accounting base) is enforced here.
    68	///
    69	/// `repo_root` is the directory containing `genesis_payload.toml` (typically
    70	/// the workspace root). Paths in the manifest are interpreted relative to it.
    71	pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
    72	    let genesis_path = repo_root.join("genesis_payload.toml");
    73	    let genesis_text = fs::read_to_string(&genesis_path).map_err(TrustRootError::GenesisRead)?;
    74	    let manifest = parse_trust_root_section(&genesis_text)?;
    75	    if !has_section(&genesis_text, "pput_accounting_0") {
    76	        return Err(TrustRootError::SectionMissing("pput_accounting_0"));
    77	    }
    78	    for (rel_path, expected_hash) in &manifest {
    79	        let full = repo_root.join(rel_path);
    80	        let bytes = fs::read(&full).map_err(|err| TrustRootError::FileRead {
    81	            path: full.clone(),
    82	            err,
    83	        })?;
    84	        let mut hasher = Sha256::new();
    85	        hasher.update(&bytes);
    86	        let actual = hex_lower(&hasher.finalize());
    87	        if actual != *expected_hash {
    88	            return Err(TrustRootError::Tampered {
    89	                path: full,
    90	                expected: expected_hash.clone(),
    91	                actual,
    92	            });
    93	        }
    94	    }
    95	    Ok(())
    96	}
    97	
    98	/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
    99	/// the trust_root_immutability conformance battery (Phase B7) reads the
   100	/// manifest directly to assert it includes the audit-recommended PPUT
   101	/// accounting layer.
   102	///
   103	/// Parses the `[trust_root]` section of `genesis_payload.toml` into ordered
   104	/// `(path, sha256)` pairs. Hand-rolled — accepts the narrow subset we emit
   105	/// (quoted-key = quoted-value, comments, blank lines).
   106	pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
   107	    let mut in_section = false;
   108	    let mut entries = Vec::new();
   109	    for (lineno, raw) in text.lines().enumerate() {
   110	        let line = strip_comment(raw).trim();
   111	        if line.is_empty() {
   112	            continue;
   113	        }
   114	        if let Some(header) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
   115	            in_section = header.trim() == "trust_root";
   116	            continue;
   117	        }
   118	        if !in_section {
   119	            continue;
   120	        }
   121	        let (key, value) = line.split_once('=').ok_or_else(|| {
   122	            TrustRootError::GenesisParse(format!("line {}: missing '=' in [trust_root]", lineno + 1))
   123	        })?;
   124	        let key = unquote(key.trim()).ok_or_else(|| {
   125	            TrustRootError::GenesisParse(format!("line {}: key not quoted", lineno + 1))
   126	        })?;
   127	        let value = unquote(value.trim()).ok_or_else(|| {
   128	            TrustRootError::GenesisParse(format!("line {}: value not quoted", lineno + 1))
   129	        })?;
   130	        entries.push((key.to_string(), value.to_string()));
   131	    }
   132	    if entries.is_empty() {
   133	        return Err(TrustRootError::SectionMissing("trust_root"));
   134	    }
   135	    Ok(entries)
   136	}
   137	
   138	fn has_section(text: &str, name: &str) -> bool {
   139	    text.lines().any(|raw| {
   140	        let line = strip_comment(raw).trim();
   141	        line
   142	            .strip_prefix('[')
   143	            .and_then(|s| s.strip_suffix(']'))
   144	            .map(|h| h.trim() == name)
   145	            .unwrap_or(false)
   146	    })
   147	}
   148	
   149	fn strip_comment(line: &str) -> &str {
   150	    let mut in_string = false;
   151	    for (i, c) in line.char_indices() {
   152	        match c {
   153	            '"' => in_string = !in_string,
   154	            '#' if !in_string => return &line[..i],
   155	            _ => {}
   156	        }
   157	    }
   158	    line
   159	}
   160	
   161	fn unquote(s: &str) -> Option<&str> {
   162	    s.strip_prefix('"').and_then(|s| s.strip_suffix('"'))
   163	}
   164	
   165	fn hex_lower(bytes: &[u8]) -> String {
   166	    let mut out = String::with_capacity(bytes.len() * 2);
   167	    for b in bytes {
   168	        out.push_str(&format!("{:02x}", b));
   169	    }
   170	    out
   171	}
   172	
   173	#[cfg(test)]
   174	mod tests {
   175	    use super::*;
   176	
   177	    fn repo_root() -> PathBuf {
   178	        // turingosv4 lib is at repo root; CARGO_MANIFEST_DIR == repo root.
   179	        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
   180	    }

exec
/bin/bash -lc "nl -ba src/main.rs | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	use std::path::PathBuf;
     2	
     3	// TRACE_MATRIX FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`):
     4	// constitution FC3 ties `boot` to a re-init loop driven by `error`. Phase B7
     5	// implements the immediate-abort variant of FC3-E14 — Trust Root mismatch
     6	// at Boot panics the process; the surrounding harness (batch runner,
     7	// shell) is the "re-init" layer that decides whether to retry. Future
     8	// in-process re-init (TRACE_MATRIX FC3-N41 row, currently 📅 Phase 11+)
     9	// would replace this panic with a structured retry loop. See
    10	// `handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` for why
    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
    12	fn main() {
    13	    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    14	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
    16	    }
    17	    println!("TuringOS v4 — Trust Root verified");
    18	}

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '534,560p;1030,1088p;1088,1165p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   534	        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
   535	            info!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
   536	                   short-circuit to MaxTxExhausted exit", tx);
   537	            wc.mark_final_accept();
   538	            let mut result = make_pput(problem_file, &condition, model,
   539	                                       false, false, start, 0, 0,
   540	                                       tx as u64, Some(tool_dist), None,
   541	                                       None, None, None,
   542	                                       Some(acc.total_run_token_count()),
   543	                                       Some(acc.failed_branch_count),
   544	                                       wc.elapsed_ms());
   545	            // B7-extra disambiguator: distinguish this calibration-treatment
   546	            // exit from a natural max-tx exhaustion in downstream PPUT
   547	            // analysis. See PputResult::synthetic_short_circuit doc-comment
   548	            // for the cost-asymmetry note.
   549	            result.synthetic_short_circuit = Some(true);
   550	            return result;
   551	        }
   552	
   553	        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
   554	        // bracket at the top of the FIRST tx (before chain/skill/board build
   555	        // and before build_agent_prompt). Idempotent — only the first tx's
   556	        // call sticks; subsequent calls no-op. PREREG § 5 / plan B3 define
   557	        // T_i as "first agent prompt construction"; this is the earliest
   558	        // moment the agent begins constructing its prompt.
   559	        wc.mark_first_read();
   560	
  1030	                                        let _ = bus.append_oracle_accepted(
  1031	                                            agent_id, tactic, parent.as_deref(),
  1032	                                        );
  1033	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
  1034	                                            .filter_map(|id| bus.kernel.tape.get(id))
  1035	                                            .map(|n| n.payload.len() as u64)
  1036	                                            .sum();
  1037	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
  1038	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
  1039	                                        let gp_nodes = gp.len();
  1040	                                        bus.halt_and_settle(&gp).ok();
  1041	                                        let upr = if omega_attempts > 0 {
  1042	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1043	                                        } else { None };
  1044	                                        // P0-A: Phase B swarm step Complete — runtime gate IS
  1045	                                        // the Lean verify_partial call (PartialVerdict::Complete).
  1046	                                        // Both legs hold. Phase C Soft Law diverges here.
  1047	                                        return make_pput(problem_file, &condition, model,
  1048	                                                        true, true,
  1049	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
  1050	                                                        Some(tool_dist), upr,
  1051	                                                        Some(prefix.clone()),
  1052	                                                        Some("per_tactic".to_string()),
  1053	                                                        proof_file,
  1054	                                                        Some(acc.total_run_token_count()),
  1055	                                                        Some(acc.failed_branch_count),
  1056	                                                        wc.elapsed_ms());
  1057	                                    }
  1058	                                    PartialVerdict::PartialOk => {
  1059	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1060	                                        match bus.append_oracle_accepted(
  1061	                                            agent_id, tactic, parent.as_deref(),
  1062	                                        ) {
  1063	                                            Ok(BusResult::Appended { node_id }) => {
  1064	                                                *tool_dist.entry("step_partial_ok".into()).or_insert(0) += 1;
  1065	                                                info!("[tx {}] {} step+{} partial OK (depth={})",
  1066	                                                      tx, agent_id, node_id,
  1067	                                                      bus.kernel.tape.time_arrow().len());
  1068	                                            }
  1069	                                            Ok(BusResult::Vetoed { reason }) => {
  1070	                                                warn!("[tx {}] step partial OK but bus vetoed: {}", tx, reason);
  1071	                                            }
  1072	                                            _ => {}
  1073	                                        }
  1074	                                    }
  1075	                                    PartialVerdict::Reject(reason) => {
  1076	                                        let class = classify_lean_error(&reason);
  1077	                                        bus.record_rejection(agent_id, class.label());
  1078	                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
  1079	                                        acc.record_tool_stdout(&reason);
  1080	                                        *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
  1081	                                        let preview = reason.chars().take(200).collect::<String>();
  1082	                                        warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
  1083	                                    }
  1084	                                }
  1085	                            }
  1086	                        }
  1087	                        other => {
  1088	                            *tool_dist.entry(format!("other:{}", other)).or_insert(0) += 1;
  1088	                            *tool_dist.entry(format!("other:{}", other)).or_insert(0) += 1;
  1089	                        }
  1090	                    },
  1091	                    Err(e) => {
  1092	                        *tool_dist.entry("parse_fail".into()).or_insert(0) += 1;
  1093	                        // Step-B v3: parse failures feed the class graveyard too.
  1094	                        let class = classify_parse_error(&format!("{}", e));
  1095	                        bus.record_rejection(agent_id, class.label());
  1096	                        // PPUT-CCL B2: classifier label flows into next prompt's errors.
  1097	                        acc.record_tool_stdout(class.label());
  1098	                        warn!("[tx {}] parse: {} ({})", tx, e, class.label());
  1099	                    }
  1100	                }
  1101	            }
  1102	            Err(e) => {
  1103	                *tool_dist.entry("llm_err".into()).or_insert(0) += 1;
  1104	                warn!("[tx {}] LLM: {}", tx, e);
  1105	            }
  1106	        }
  1107	    }
  1108	
  1109	    let upr = if omega_attempts > 0 {
  1110	        Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1111	    } else { None };
  1112	    // Phase 4: also save wallet state on no-OMEGA exit. Agents may have
  1113	    // invested/lost Coin during the run; durability should not depend on a win.
  1114	    if let Some(ref wp) = wallet_state_path {
  1115	        if let Some(w) = bus.tools.iter()
  1116	            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
  1117	        {
  1118	            let _ = w.save_to_disk(wp);
  1119	        }
  1120	    }
  1121	    // No OMEGA found → PPUT = 0
  1122	    // B3: close bracket on max-tx exhaustion path.
  1123	    // P0-A: max-tx exhaustion → neither leg fired.
  1124	    wc.mark_final_accept();
  1125	    make_pput(problem_file, &condition, model,
  1126	              false, false, start, 0, 0,
  1127	              max_transactions as u64, Some(tool_dist), upr,
  1128	              None, None, None,
  1129	              Some(acc.total_run_token_count()),
  1130	              Some(acc.failed_branch_count),
  1131	              wc.elapsed_ms())
  1132	}
  1133	
  1134	fn make_pput(
  1135	    problem: &str, condition: &str, model: &str,
  1136	    runtime_accepted: bool, post_hoc_verified: bool, start: Instant,
  1137	    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
  1138	    tool_dist: Option<HashMap<String, u32>>,
  1139	    unique_payload_ratio: Option<f64>,
  1140	    gp_payload: Option<String>,
  1141	    gp_path: Option<String>,
  1142	    gp_proof_file: Option<String>,
  1143	    total_run_token_count: Option<u64>,
  1144	    failed_branch_count: Option<u32>,
  1145	    total_wall_time_ms: Option<u64>,
  1146	) -> PputResult {
  1147	    // PPUT-CCL Phase B B4 (mid-term audit P0-A fix 2026-04-25):
  1148	    // make_pput is now PURELY computational. The caller MUST decide both
  1149	    // `runtime_accepted` (did the evaluator's runtime gate fire?) and
  1150	    // `post_hoc_verified` (did Lean independently confirm the proof?). The
  1151	    // prior implementation derived `post_hoc_verified = has_gp` internally,
  1152	    // which would have laundered Phase C Soft Law fake-accepts into the
  1153	    // North Star pput_verified. Forcing the caller to pass both legs makes
  1154	    // Soft Law's design point unmissable: any caller that fakes runtime
  1155	    // accept must explicitly pass post_hoc_verified=verify_post_hoc(...)
  1156	    // or the divergence will surface immediately.
  1157	    //
  1158	    // Phase B all callers pass `(runtime_accepted, post_hoc_verified) = (X, X)`
  1159	    // because runtime IS Lean today. Phase C diverges at the Soft Law
  1160	    // mode call site, not inside this function.
  1161	    let has_gp = runtime_accepted; // legacy `has_golden_path` field semantics
  1162	    let elapsed = start.elapsed().as_secs_f64();
  1163	    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
  1164	    // C-012 provenance: populated from env vars; None when unset (backward compat).
  1165	    let build_sha = std::env::var("BUILD_SHA").ok();

exec
/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env bash
     2	# PPUT-CCL B7-extra — p_0 calibration runner.
     3	#
     4	# PREREG § 5.5 protocol:
     5	#   - control:    evaluator on adaptation-144 × seeds [31415, 2718]
     6	#   - treatment:  same + SIMULATE_ROLLBACK_AT_TX_50=1
     7	#   - 288 + 288 = 576 runs total.
     8	#   - regression_p = 1 iff control SOLVED && treatment UNSOLVED, same (problem, seed)
     9	#   - p_0 = sum_p max_seed(regression_p) / 144
    10	#
    11	# Constitutional anchor (TRACE_MATRIX_v1 § 2):
    12	#   treatment runs route through the existing FC1-E18 (∏p=0 → Q_t)
    13	#   semantics — see experiments/minif2f_v4/src/rollback_sim.rs header.
    14	#
    15	# Usage:
    16	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
    17	#
    18	#   --smoke  run 1 problem × 2 seeds × 2 modes = 4 runs (~5 min, ~$0.05)
    19	#            for pre-batch verification per feedback_smoke_before_batch.md
    20	#   (no flag) full 576-run batch (~8h, ~$3-5 — needs explicit user GO)
    21	#
    22	# Prerequisites (same as run_batch.sh):
    23	#   export DEEPSEEK_API_KEY=...
    24	#   export LLM_PROXY_URL=http://localhost:8080  (default)
    25	#   export ACTIVE_MODEL=deepseek-chat           (default)
    26	
    27	set -uo pipefail
    28	
    29	SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    30	PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
    31	
    32	# Auto-load v3 .env for API keys if not already set
    33	if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    34	    source "$HOME/projects/turingosv3/.env"
    35	fi
    36	export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
    37	export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
    38	
    39	MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
    40	LOG_DIR="$PROJECT_ROOT/experiments/minif2f_v4/logs"
    41	TIMESTAMP=$(date +%Y%m%dT%H%M%S)
    42	SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"
    43	
    44	SMOKE=0
    45	if [ "${1:-}" = "--smoke" ]; then
    46	    SMOKE=1
    47	fi
    48	
    49	# PREREG § 5.5: condition fixed at n3 (3-agent swarm — needs >=50 tx capacity).
    50	# Boltzmann seeds frozen at PREREG values.
    51	CONDITION="n3"
    52	SEEDS=(31415 2718)
    53	MODES=("control" "treatment")
    54	
    55	mkdir -p "$LOG_DIR"
    56	
    57	if [ "$SMOKE" -eq 1 ]; then
    58	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
    59	else
    60	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
    61	fi
    62	
    63	# Resolve adaptation-144 problem list from frozen splits.
    64	# Each problem ID maps to <MINIF2F_DIR>/MiniF2F/Test/<id>.lean.
    65	ADAPTATION_IDS=$(python3 -c "
    66	import json
    67	d = json.load(open('$SPLITS_JSON'))
    68	for pid in d['splits']['adaptation']['problem_ids']:
    69	    print(pid)
    70	")
    71	
    72	if [ "$SMOKE" -eq 1 ]; then
    73	    # Smoke: pick one short mathd_algebra problem (typically solved in <50 tx).
    74	    SMOKE_ID=$(echo "$ADAPTATION_IDS" | grep "^mathd_algebra" | head -1)
    75	    if [ -z "$SMOKE_ID" ]; then
    76	        SMOKE_ID=$(echo "$ADAPTATION_IDS" | head -1)
    77	    fi
    78	    ADAPTATION_IDS="$SMOKE_ID"
    79	    echo "[smoke] using single problem: $SMOKE_ID"
    80	fi
    81	
    82	# Build evaluator (release).
    83	echo "Building evaluator (release)..."
    84	(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
    85	EVALUATOR="$PROJECT_ROOT/target/release/evaluator"
    86	
    87	# C-012 oracle preflight (memory feedback_oracle_preflight.md).
    88	echo "Oracle preflight..."
    89	LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
    90	PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
    91	    \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) \
    92	    -type d 2>/dev/null | tr '\n' ':')
    93	if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
    94	    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
    95	    exit 2
    96	fi
    97	PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
    98	    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1)
    99	PREFLIGHT_CODE=$?
   100	if [ "$PREFLIGHT_CODE" -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
   101	    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
   102	    echo "$PREFLIGHT_OUT" | head -c 500
   103	    exit 2
   104	fi
   105	echo "Oracle preflight OK."
   106	
   107	# Run loop. Each (mode, seed, problem) combination = 1 run.
   108	TOTAL_PROBLEMS=$(echo "$ADAPTATION_IDS" | wc -l)
   109	TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
   110	echo ""
   111	echo "=== p_0 calibration ==="
   112	echo "Mode count:    ${#MODES[@]} (control + treatment)"
   113	echo "Seed count:    ${#SEEDS[@]} (${SEEDS[*]})"
   114	echo "Problem count: $TOTAL_PROBLEMS"
   115	echo "Total runs:    $TOTAL_RUNS"
   116	echo ""
   117	
   118	BATCH_START=$(date +%s)
   119	RUN_IDX=0
   120	for MODE in "${MODES[@]}"; do
   121	    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
   122	    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
   123	    : > "$OUT_FILE"
   124	    : > "$STDERR_LOG"
   125	    case "$MODE" in
   126	        control)   ROLLBACK_FLAG="" ;;
   127	        treatment) ROLLBACK_FLAG="1" ;;
   128	    esac
   129	    for SEED in "${SEEDS[@]}"; do
   130	        while IFS= read -r PID; do
   131	            [ -z "$PID" ] && continue
   132	            RUN_IDX=$((RUN_IDX + 1))
   133	            PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
   134	            if [ ! -f "$PROBLEM" ]; then
   135	                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND, skip"
   136	                continue
   137	            fi
   138	            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
   139	            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
   140	            OUTPUT=$(timeout 2400 env \
   141	                CONDITION="$CONDITION" \
   142	                MINIF2F_DIR="$MINIF2F_DIR" \
   143	                BOLTZMANN_SEED="$SEED" \
   144	                SIMULATE_ROLLBACK_AT_TX_50="$ROLLBACK_FLAG" \
   145	                RUST_LOG=info \
   146	                "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || true
   147	            PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)
   148	            if [ -n "$PPUT_JSON" ]; then
   149	                # Stamp mode + seed + problem_id for downstream pairing analysis
   150	                # (PREREG § 5.5 estimator does control-vs-treatment join on
   151	                # (problem, seed)).
   152	                ENRICHED=$(echo "$PPUT_JSON" | python3 -c "
   153	import json, sys
   154	row = json.loads(sys.stdin.read())
   155	row['calibration_mode'] = '$MODE'
   156	row['calibration_seed'] = $SEED
   157	row['calibration_problem_id'] = '$PID'
   158	print(json.dumps(row))
   159	")
   160	                echo "$ENRICHED" >> "$OUT_FILE"
   161	                HAS_GP=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('has_golden_path', False))")
   162	                TX=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tx_count', 0))")
   163	                if [ "$HAS_GP" = "True" ]; then
   164	                    echo "SOLVED (tx=$TX)"
   165	                else
   166	                    echo "UNSOLVED (tx=$TX)"
   167	                fi
   168	            else
   169	                echo "MEASUREMENT_ERROR"
   170	            fi
   171	        done <<< "$ADAPTATION_IDS"
   172	    done
   173	done
   174	
   175	BATCH_END=$(date +%s)
   176	WALL_TIME=$((BATCH_END - BATCH_START))
   177	
   178	echo ""
   179	echo "╔═══════════════════════════════════════════╗"
   180	echo "║   p_0 CALIBRATION SUMMARY"
   181	echo "╠═══════════════════════════════════════════╣"
   182	echo "║ Wall time:      ${WALL_TIME}s"
   183	echo "║ Control jsonl:  ${OUT_PREFIX}_control.jsonl"
   184	echo "║ Treatment jsonl: ${OUT_PREFIX}_treatment.jsonl"
   185	echo "╚═══════════════════════════════════════════╝"
   186	echo ""
   187	if [ "$SMOKE" -eq 1 ]; then
   188	    echo "Smoke complete. Verify (1) treatment row tx_count == 50 if it would have"
   189	    echo "exceeded 50, (2) both rows parse via RunRecord::V2, (3) calibration_mode +"
   190	    echo "calibration_seed + calibration_problem_id are present. Then re-run without"
   191	    echo "--smoke for the full 576-run batch."
   192	else
   193	    echo "Compute p_0:"
   194	    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
   195	    echo "    --control ${OUT_PREFIX}_control.jsonl \\"
   196	    echo "    --treatment ${OUT_PREFIX}_treatment.jsonl"
   197	    echo ""
   198	    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
   199	fi

exec
/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env python3
     2	"""PPUT-CCL B7-extra — compute p_0 from calibration jsonl.
     3	
     4	PREREG § 5.5 estimator:
     5	    For each (problem, seed): regression_p_seed = 1 iff control SOLVED
     6	                              AND treatment UNSOLVED.
     7	    Per-problem regression:   max over the 2 seeds (worst case).
     8	    p_0:                      sum_p regression_p / N_problems.
     9	
    10	Sanity gate: if p_0 > 0.10, ABORT — toggle too aggressive (PREREG § 5.5 ceiling).
    11	
    12	Usage:
    13	    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
    14	"""
    15	
    16	from __future__ import annotations
    17	
    18	import argparse
    19	import hashlib
    20	import json
    21	import sys
    22	from collections import defaultdict
    23	from pathlib import Path
    24	
    25	
    26	def load_jsonl(path: Path) -> list[dict]:
    27	    rows = []
    28	    with path.open() as f:
    29	        for line in f:
    30	            line = line.strip()
    31	            if not line:
    32	                continue
    33	            rows.append(json.loads(line))
    34	    return rows
    35	
    36	
    37	def solved(row: dict) -> bool:
    38	    """PREREG § 1.3 progress = 1 iff Lean ground-truth verifies golden_path.
    39	
    40	    The B4 split (`progress_runtime` vs `progress_verified`) frames the verified
    41	    leg as authoritative. Fall back to `has_golden_path` for legacy rows.
    42	    """
    43	    if "progress_verified" in row and row["progress_verified"] is not None:
    44	        return int(row["progress_verified"]) == 1
    45	    return bool(row.get("has_golden_path", False))
    46	
    47	
    48	def compute(control_rows: list[dict], treatment_rows: list[dict]) -> dict:
    49	    # Index by (problem_id, seed). calibration_problem_id and calibration_seed
    50	    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
    51	    # either tag.
    52	    def index(rows):
    53	        out = {}
    54	        for r in rows:
    55	            pid = r.get("calibration_problem_id")
    56	            seed = r.get("calibration_seed")
    57	            if pid is None or seed is None:
    58	                continue
    59	            out[(pid, seed)] = r
    60	        return out
    61	
    62	    c = index(control_rows)
    63	    t = index(treatment_rows)
    64	
    65	    pairs = sorted(set(c.keys()) & set(t.keys()))
    66	    if not pairs:
    67	        sys.exit("ERROR: no overlapping (problem, seed) pairs between control and treatment")
    68	
    69	    # Per-problem worst-case regression (max over seeds).
    70	    per_problem_regression: dict[str, int] = defaultdict(int)
    71	    n_pairs = 0
    72	    n_control_solved = 0
    73	    n_treatment_solved = 0
    74	    n_regression_pairs = 0
    75	    for pid, seed in pairs:
    76	        cr = c[(pid, seed)]
    77	        tr = t[(pid, seed)]
    78	        cs = solved(cr)
    79	        ts = solved(tr)
    80	        n_pairs += 1
    81	        if cs:
    82	            n_control_solved += 1
    83	        if ts:
    84	            n_treatment_solved += 1
    85	        regression = 1 if (cs and not ts) else 0
    86	        if regression:
    87	            n_regression_pairs += 1
    88	        if regression > per_problem_regression[pid]:
    89	            per_problem_regression[pid] = regression
    90	
    91	    n_problems = len({pid for pid, _ in pairs})
    92	    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
    93	
    94	    return {
    95	        "n_problems": n_problems,
    96	        "n_pairs": n_pairs,
    97	        "n_control_solved": n_control_solved,
    98	        "n_treatment_solved": n_treatment_solved,
    99	        "n_regression_pairs": n_regression_pairs,
   100	        "n_regression_problems_max_seed": sum(per_problem_regression.values()),
   101	        "p0": p0,
   102	        "p0_ceiling": 0.10,
   103	        "ceiling_pass": p0 <= 0.10,
   104	    }
   105	
   106	
   107	def main() -> int:
   108	    ap = argparse.ArgumentParser()
   109	    ap.add_argument("--control", required=True, type=Path)
   110	    ap.add_argument("--treatment", required=True, type=Path)
   111	    ap.add_argument("--out-json", type=Path, default=None,
   112	                    help="Write structured result to this path")
   113	    args = ap.parse_args()
   114	
   115	    control_rows = load_jsonl(args.control)
   116	    treatment_rows = load_jsonl(args.treatment)
   117	
   118	    result = compute(control_rows, treatment_rows)
   119	    print(json.dumps(result, indent=2))
   120	
   121	    if args.out_json:
   122	        args.out_json.write_text(json.dumps(result, indent=2) + "\n")
   123	
   124	    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
   125	    h = hashlib.sha256()
   126	    for path in (args.control, args.treatment):
   127	        h.update(path.read_bytes())
   128	    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
   129	    print(f"  {h.hexdigest()}")
   130	
   131	    if not result["ceiling_pass"]:
   132	        print(
   133	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
   134	            file=sys.stderr,
   135	        )
   136	        return 2
   137	    return 0
   138	
   139	
   140	if __name__ == "__main__":
   141	    sys.exit(main())

exec
/bin/bash -lc "nl -ba genesis_payload.toml | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4 — Genesis payload (Phase B7).
     2	#
     3	# Frozen at Phase B B7 (2026-04-25). Two binding sections:
     4	#
     5	#   [pput_accounting_0] — semantic invariants of the PPUT measure
     6	#                         (PREREG § 1.8). baseline_regression_rate +
     7	#                         baseline_regression_jsonl_sha256 are placeholders
     8	#                         until B7-extra (p_0 calibration) lands.
     9	#
    10	#   [trust_root]        — SHA-256 of every load-bearing file. Boot
    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
    13	#                         mismatch.
    14	#
    15	# Manifest derivation (independently re-derived in B7 from PREREG § 1.8 +
    16	# B2-B4 mid-term audit recommendation + B6 prompt_guard add):
    17	#
    18	#   PREREG § 1.8 base (8):
    19	#     src/kernel.rs, src/wal.rs, src/bus.rs,
    20	#     experiments/minif2f_v4/src/lean4_oracle.rs,
    21	#     constitution.md, cases/MANIFEST.sha256 (proxy for cases/*.yaml glob),
    22	#     handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,
    23	#     handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
    24	#
    25	#   Mid-term audit add (PPUT accounting layer, 6):
    26	#     src/drivers/llm_http.rs (cost source of truth),
    27	#     experiments/minif2f_v4/src/cost_aggregator.rs (B2),
    28	#     experiments/minif2f_v4/src/wall_clock.rs (B3),
    29	#     experiments/minif2f_v4/src/post_hoc_verifier.rs (B4),
    30	#     experiments/minif2f_v4/src/jsonl_schema.rs (B1),
    31	#     experiments/minif2f_v4/src/bin/evaluator.rs (the wiring)
    32	#
    33	#   B6 add (1):
    34	#     src/sdk/prompt_guard.rs (PPUT-context-leak runtime gate)
    35	#
    36	#   B7-extra add (1):
    37	#     experiments/minif2f_v4/src/rollback_sim.rs (PPUT-CCL § 5.5
    38	#       calibration treatment toggle — synthetic ∏p=0 from tx 50,
    39	#       constitutionally FC1-E18 + FC2-N22-MaxTxExhausted)
    40	#
    41	# Total: 16 files. genesis_payload.toml itself is conceptually frozen but
    42	# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
    43	# values are the semantic anchor.
    44	#
    45	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
    46	
    47	[pput_accounting_0]
    48	schema_version = "1.0"
    49	progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
    50	cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
    51	time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
    52	verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
    53	heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
    54	source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
    55	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
    56	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
    57	k_max = 10
    58	n_max = 34
    59	
    60	[trust_root]
    61	"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
    62	"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
    63	"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
    64	"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
    65	"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
    66	"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
    67	"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
    68	"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
    69	"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
    70	"experiments/minif2f_v4/src/jsonl_schema.rs" = "7647a2ea842f4394d55e1ce85032642c18ba61f962bd2c5c8714c618e743777c"
    71	"experiments/minif2f_v4/src/rollback_sim.rs" = "0c4dd5a922edddd43769915f0d2c13332331ecbee74e09294f6b2243ec7607cd"
    72	"experiments/minif2f_v4/src/bin/evaluator.rs" = "265314baa700ce0c795e3ec7efed5a77ebc81a01884bae33592d1a0e8d8ce32c"
    73	"constitution.md" = "b304b431b7d3fbb8d1630f76ac4bc3eaa80383dde41fb3bc69fbe5bb263658ef"
    74	"cases/MANIFEST.sha256" = "3759631afae97c631be9a4038479b8d3681af25885f9758c32ad88c22f17fc3f"
    75	"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
    76	"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"

exec
/bin/bash -lc "rg -n \"5\\.5 p_0|Ceiling:|Estimator:|Freeze point|Audit trail|Toggle:|Boltzmann|baseline_regression|ArtifactState|Trust Root composition|cases/C\" handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
92:| WBCG_PPUT | sum over candidate artifacts Δ of `1[ArtifactState(Δ) = Certified]` where `Certified` requires (i) j-PPUT + j-FAR + j-CPR all reject null at family-corrected α per § 5.3 + § 9, (ii) j-RR ≤ p_0 point-check guardrail per § 5.4, (iii) Rollbackable(Δ) = 1 per § 6 E2 (see § 1.7 for ArtifactState; § 7 Gate H for full reachability conditions) | Capability-compilation success |
98:### 1.7 ArtifactState — layered confidence (Patch A from Gemini DeepThink 2026-04-26)
103:ArtifactState(Δ) ∈ {Accepted, Quarantined, Certified, Reverted}
132:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
143:  cases/C-*.yaml,                   -- common-law precedent (append-only by human)
384:| C (hard-10 ablation) | **per-problem** | 10 | hard-10 adaptation problems × 2 random Boltzmann seeds; seeds are noise-reducing replicates (correlated within problem). Per-problem score = paired-binary aggregated over 2 seeds. |
385:| E (heldout-54) | **per-problem** | 54 | All 54 heldout problems × 3 pre-committed Boltzmann seeds; seeds are replicates. Per-problem score = paired sign of mean VPPUT over 3 seeds. |
406:For each artifact j ∈ {1, ..., k} (k ≤ k_max = 10) entering Phase E with `ArtifactState = Quarantined`, **three** primary inferential tests run on heldout-54 (NOT four — j-RR is a descriptive guardrail per § 5.4):
437:### 5.5 p_0 calibration protocol
442:Toggle:        evaluator --mode full         (control)
446:Sample:        FULL adaptation-144 problem set × Boltzmann seeds [31415, 2718]
448:Estimator:     for each problem p, define regression_p = 1 iff control SOLVED
454:Freeze point:  end of Phase B7. Value written to
455:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
457:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
458:Audit trail:   raw calibration jsonl committed to
462:Ceiling:       if measured p_0 > 0.10, ABORT — toggle too aggressive or system
481:Gate H: count of artifacts j with ArtifactState(j) = Certified ≥ 1
512:- C2 **hard-10 adaptation × 2 random Boltzmann routing seeds; independent unit = per-problem (seeds = noise-reducing replicates), not (problem, seed)** (round 3 fix per Codex round-2 R3-1 — within-problem seed correlation). Hard-10 sample is drawn from the adaptation set (144 problems) using the same hash-based deterministic procedure as the three-split: `random.Random("hard10_pput_ccl_seed").sample(adaptation_set, 10)`. The 10 problem IDs + their fingerprint are pre-committed in Phase A5. Boltzmann seeds = `[31415, 2718]`. Total per-mode runs: 5 modes × 10 problems × 2 seeds = 100 jsonl rows. McNemar per condition pair uses **n = 10 per-problem paired-binary observations** (paired sign of `mean(VPPUT_seed1, VPPUT_seed2)_full > mean(VPPUT_seed1, VPPUT_seed2)_alt`). See § 9.2 + § 9.5 for power analysis.
541:  - Pre-committed seeds: `[20260426, 31415, 2718]` (3 Boltzmann routing seeds; same 3 seeds used for every sub-evaluation below for paired comparison).
567:| H | **Heldout WBCG_PPUT > 0 with ArtifactState = Certified gate** (round 4 — j-RR moved out of inferential family per Codex round-3): at least one ArchitectAI-generated user-space artifact Δ has `ArtifactState(Δ) = Certified` per § 1.7 — meaning (i) was Accepted (passed § 3 + § 3.5 AuditorAI battery), (ii) was Quarantined (user-approved + ΔPPUT_meta_val > 0 + N_use ≥ 3 on meta_val), (iii) at heldout sealed eval **three** primary inferential tests reject null at family-corrected α (per § 5.3 + § 9): **j-PPUT** (sign test on n=54), **j-FAR** (non-inferiority sign test), **j-CPR** (non-inferiority sign test), AND (iv) **j-RR ≤ p_0 point check guardrail** (per § 5.4 — descriptive, NOT inferential, NO α correction), AND (v) is rollbackable (artifact directory deletion at the protocol level restores prior heldout PPUT — verified by an explicit rollback sub-eval, see § 6 E2). Quarantined-only artifacts (failed any of the five conditions) do NOT count toward Gate H. |
760:| 2026-04-26 | **Patch A** (Gemini DeepThink): § 1.7 ArtifactState 4-state machine for artifacts (Accepted / Quarantined / Certified / Reverted); § 1.6 WBCG_PPUT row + § 7 Gate H tightened to "Certified-only". Task Progress remains binary (Lean perfect predicate). | Gemini DeepThink 2026-04-26 PPUT-driven FULL PASS confirmation |
761:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '40,120p;400,475p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    40	      - every retry and failed branch
    41	      - every architect/auditor meta-loop call attributed to task i
    42	      - every proxy / middleware augmentation
    43	```
    44	
    45	C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
    46	
    47	### 1.3 Time T_i (wall-clock, end-to-end)
    48	
    49	```
    50	T_i = end_time - start_time   (seconds)
    51	   start_time = first read of the task statement by any agent in the run
    52	   end_time   = final ground-truth accept (Lean PASS) OR external timeout
    53	```
    54	
    55	Lean verification time is included (it is a real cost). Architect/auditor meta-loop time is included.
    56	
    57	### 1.4 Verified PPUT
    58	
    59	```
    60	VPPUT_i = Progress_i / (C_i × T_i)
    61	```
    62	
    63	If `Progress_i = 0`: `VPPUT_i = 0` regardless of how much was spent.
    64	
    65	For dashboard readability we report scaled values:
    66	```
    67	PPUT-M_i = 10^6 × VPPUT_i    (per million token-second)
    68	PPUT-B_i = 10^9 × VPPUT_i    (per billion token-second)
    69	```
    70	Raw `VPPUT_i` retained in jsonl for paper / audit. PPUT-M is the dashboard default.
    71	
    72	### 1.5 Held-out Verified PPUT (NORTH STAR)
    73	
    74	```
    75	H-VPPUT = (sum_{i in heldout_set} Progress_i) / (sum_{i in heldout_set} C_i × T_i)
    76	```
    77	
    78	This is the sole optimization signal. All other dashboard metrics are guardrails or diagnostics.
    79	
    80	### 1.6 Auxiliary metrics (frozen list)
    81	
    82	| Metric | Definition | Role |
    83	|---|---|---|
    84	| FAR | False Accept Rate: runtime accepts that Lean rejects post-hoc / total runtime accepts | Safety guardrail |
    85	| RR  | Regression Rate: tasks SOLVED at baseline that become UNSOLVED after a meta-change | Backwards-compat guardrail |
    86	| ERR | External Recovery Rate: post-rollback retry success rate on tasks that previously hit recoverable failure | Tape / Q^world hygiene |
    87	| CPR | Context Pollution Rate: bytes of rejected-trace text observed in agent prompts / total prompt bytes | Info hygiene |
    88	| IAC | Inter-Agent Correlation: pairwise payload Jaccard mean across agents in the same branch | Diversity (Art. II.2.1) |
    89	| Generalization Gap | (PPUT_adaptation − PPUT_meta_val) on a fixed model snapshot | Adaptation overfit detector |
    90	| AH  | Autonomy Hours: continuous wall-clock without human sudo | Long-run autonomy |
    91	| HIF | Human Intervention Frequency: count of human-overrides per 10K tx | Long-run autonomy |
    92	| WBCG_PPUT | sum over candidate artifacts Δ of `1[ArtifactState(Δ) = Certified]` where `Certified` requires (i) j-PPUT + j-FAR + j-CPR all reject null at family-corrected α per § 5.3 + § 9, (ii) j-RR ≤ p_0 point-check guardrail per § 5.4, (iii) Rollbackable(Δ) = 1 per § 6 E2 (see § 1.7 for ArtifactState; § 7 Gate H for full reachability conditions) | Capability-compilation success |
    93	
    94	`N_use ≥ 3` (minimum re-use threshold for an artifact to count as "real").
    95	
    96	VTR (Verified Tactic Rate) is **demoted to descriptive** — micro-step gameable. Reported but not gating.
    97	
    98	### 1.7 ArtifactState — layered confidence (Patch A from Gemini DeepThink 2026-04-26)
    99	
   100	For ANY user-space artifact Δ generated by ArchitectAI (broadcast rule, doc, skill, retrievable tool):
   101	
   102	```
   103	ArtifactState(Δ) ∈ {Accepted, Quarantined, Certified, Reverted}
   104	
   105	Accepted    : Δ has just passed § 3 metering + § 3.5 + § 3.5.1 AuditorAI
   106	              content meta-predicate battery; eligible for human review (§ 6 D5).
   107	Quarantined : Δ has been Accepted AND user-approved (§ 6 D5) AND has measured
   108	              positive ΔPPUT on meta_validation set with N_use ≥ 3.
   109	              Quarantined artifacts are callable in adaptation / meta_val runs
   110	              and ENTER Phase E heldout sealed eval (§ 6 E2). Their meta_val
   111	              effects are MEASURED but provisional — meta_val ΔPPUT > 0 is a
   112	              FILTER for Phase E entry, not a certification.
   113	Certified   : Δ entered Phase E AND survived (round 4 — three inferential + one
   114	              guardrail + rollback): j-PPUT, j-FAR, j-CPR all reject null at
   115	              family-corrected α (per § 5.3 + § 9), AND j-RR ≤ p_0 point check
   116	              (per § 5.4 — descriptive guardrail, NOT inferential), AND the
   117	              rollback sub-eval confirms Rollbackable=1 (per § 6 E2). Counts
   118	              toward WBCG_PPUT (§ 1.6 + § 7 Gate H).
   119	Reverted    : Δ failed any of the three primary tests, OR violated the j-RR
   120	              point-check guardrail, OR caused regression in post-Phase-E
   400	**Test mechanics**: for each problem p in hard-10, average VPPUT over 2 seeds per mode → compare per-problem means → produce paired-binary (`1` if Full mean > Alt mean, `0` otherwise) → McNemar one-sided exact binomial on the 10 paired-binary observations.
   401	
   402	**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.
   403	
   404	### 5.3 Phase E inferential hypotheses (3 per artifact)
   405	
   406	For each artifact j ∈ {1, ..., k} (k ≤ k_max = 10) entering Phase E with `ArtifactState = Quarantined`, **three** primary inferential tests run on heldout-54 (NOT four — j-RR is a descriptive guardrail per § 5.4):
   407	
   408	| ID family | Hypothesis (one-sided) | Primary endpoint (n=54) | Direction |
   409	|---|---|---|---|
   410	| `j-PPUT` | Artifact j increases held-out VPPUT | Sign test on n=54 per-problem signs of `mean(VPPUT_all_p) − mean(VPPUT_minus_j_p)` (mean over 3 seeds; leave-one-out per § 6 E2) | sign > 0 |
   411	| `j-FAR` | Artifact j does not increase FAR | Non-inferiority sign test on n=54 per-problem signs of `FAR_all_p − FAR_minus_j_p` | sign ≤ 0 |
   412	| `j-CPR` | Artifact j does not increase CPR | Non-inferiority sign test on n=54 per-problem signs of `CPR_all_p − CPR_minus_j_p` | sign ≤ 0 |
   413	
   414	`j-RR` is **NOT** in the inferential family — see § 5.4.
   415	
   416	### 5.4 j-RR as descriptive guardrail (round 4 fix per Codex round-3 R4-2)
   417	
   418	**Why this changed**: Round 3 placed j-RR in the inferential family with H0: `regression_rate ≥ p_0 = 0.10`. Codex round-3 noted this is mathematically unwinnable: best case 0/54 regressions gives `p = 0.9^54 ≈ 0.00343`, but smallest Holm threshold at N_max = 44 was 0.05/44 ≈ 0.00114 → fails to reject even with zero regressions. Gate H unreachable.
   419	
   420	**Round 4 fix**: j-RR is reclassified as a **descriptive guardrail** (point estimate check, no α correction, no place in family):
   421	
   422	```
   423	j-RR criterion (point check, NOT inferential):
   424	  r_j = |{p in heldout : SOLVED(p, baseline) = 1
   425	                       AND SOLVED(p, all_artifacts_active) = 0
   426	                       AND SOLVED(p, minus_j) = 1}|
   427	  -- Problems where artifact j SPECIFICALLY caused a regression versus
   428	  -- baseline, attribution by leave-one-out: removing j fixes the regression.
   429	
   430	  RR_j = r_j / 54        (point estimate, NOT a hypothesis test)
   431	
   432	  Pass criterion: RR_j ≤ p_0   where p_0 is calibrated per § 5.5.
   433	```
   434	
   435	**Why descriptive is statistically correct here**: j-RR is a constraint, not a discovery hypothesis. We do not need to argue "RR_j is significantly less than p_0"; we only need RR_j ≤ p_0. A point check is precisely the right tool. No multiplicity correction applies because this is not a test.
   436	
   437	### 5.5 p_0 calibration protocol
   438	
   439	`p_0` (baseline regression rate) — full calibration protocol:
   440	
   441	```
   442	Toggle:        evaluator --mode full         (control)
   443	               evaluator --mode full --simulate-rollback-at-tx-50  (treatment;
   444	                 forces a synthetic mid-run rollback at tx 50, mimicking the
   445	                 worst-case stochastic edge case Phase E artifacts could trigger)
   446	Sample:        FULL adaptation-144 problem set × Boltzmann seeds [31415, 2718]
   447	                 = 288 paired runs.
   448	Estimator:     for each problem p, define regression_p = 1 iff control SOLVED
   449	                 (Lean PASS) AND treatment UNSOLVED for the SAME (problem, seed).
   450	               Per-problem regression: max over the 2 seeds (worst case).
   451	               p_0 = sum_p regression_p / 144.
   452	Schedule:      Phase B7 (mandatory; not deferrable to Phase D). Phase D cannot
   453	               start without p_0 frozen.
   454	Freeze point:  end of Phase B7. Value written to
   455	                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
   456	               with SHA-256 of the calibration jsonl in
   457	                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
   458	Audit trail:   raw calibration jsonl committed to
   459	                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
   460	               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
   461	               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
   462	Ceiling:       if measured p_0 > 0.10, ABORT — toggle too aggressive or system
   463	               too brittle. NOT an opportunity for j-RR auto-pass.
   464	Dual audit:    p_0 calibration result in Phase B → C audit packet.
   465	```
   466	
   467	This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.
   468	
   469	### 5.6 Family total (clean restatement)
   470	
   471	```
   472	inferential_family_size = 4 (Phase C) + 3·k (Phase E)
   473	N_max = 4 + 3·k_max = 4 + 3·10 = 34   (frozen for ALL Holm decisions; see § 9)
   474	```
   475	

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_MATRIX_v1_2026-04-25.md | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TRACE_MATRIX v1 — Constitutional Flowchart ↔ Rust Code (2026-04-25)
     2	
     3	**Predecessor**: `TRACE_MATRIX_v0_2026-04-22.md`
     4	**Trigger**: Phase B B7 (Trust Root + Boot freeze) shipped runtime code that (a) implements the Phase 11+ deferred FC3-N34 row and (b) introduces new files to the readonly base. Per CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素", v1 documents the new mappings before downstream work piles on top.
     5	
     6	**Scope**: delta only. v0 rows that did not change are still authoritative — read v0 first.
     7	
     8	**Legend** (unchanged from v0):
     9	- ✅ well-aligned · ⚠️ partial · 🔨 missing-actionable · 📅 deferred Phase 11+ · 📄 docs-only
    10	
    11	---
    12	
    13	## § 1. Status flips (rows that changed since v0)
    14	
    15	| FC Element ID | v0 Status | v1 Status | Justification |
    16	|---|---|---|---|
    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
    18	
    19	No rows regressed. No previously ✅ rows changed.
    20	
    21	---
    22	
    23	## § 2. New code symbols added in B7 (FC anchors)
    24	
    25	| Symbol | File:Line | FC Anchor | DocComment | Status |
    26	|---|---|---|---|---|
    27	| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
    28	| `boot::parse_trust_root_section` | `src/boot.rs:91` | FC3-N34 (helper) | Y (line 86-90) | ✅ |
    29	| `boot::TrustRootError` | `src/boot.rs:24` | FC3-N34 (failure variants) | Y (line 19-23) | ✅ |
    30	| `fn main` (Trust Root verify call site) | `src/main.rs:11` | FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`) | Y (line 3-10) | ✅ |
    31	| `rollback_sim::should_simulate_rollback` | `experiments/minif2f_v4/src/rollback_sim.rs:48` | FC1-E18 (∏p=0 → Q_t) repeated · FC2-N22 HALT (existing `MaxTxExhausted` variant) | Y (file header + fn doc) | ✅ |
    32	| `rollback_sim::rollback_simulation_enabled` | `experiments/minif2f_v4/src/rollback_sim.rs:39` | same FC1-E18 + FC2-N22 anchor (env-var read for the predicate) | Y | ✅ |
    33	| `rollback_sim::ROLLBACK_TX_THRESHOLD` | `experiments/minif2f_v4/src/rollback_sim.rs:34` | PREREG § 5.5 frozen constant (calibration anchor — not a runtime parameter) | Y | ✅ |
    34	| `rollback_sim::ROLLBACK_ENV_VAR` | `experiments/minif2f_v4/src/rollback_sim.rs:38` | env-var name (mirrors PREREG § 5.5 `--simulate-rollback-at-tx-50`) | Y | ✅ |
    35	| `evaluator.rs` short-circuit at line 503-518 | `experiments/minif2f_v4/src/bin/evaluator.rs:503` | FC1-E18 + FC2-N22 (call-site of the synthetic predicate) | Y (block comment) | ✅ |
    36	
    37	Internal helpers (`has_section`, `strip_comment`, `unquote`, `hex_lower`) are private — no FC backlink required (per CLAUDE.md scoping to `pub` symbols).
    38	
    39	---
    40	
    41	## § 3. New `readonly` extensions (FC3-S3 subgraph membership change)
    42	
    43	The constitutional FC3-S3 `readonly` subgraph contains FC3-N3 (`constitution as ground truth`) and FC3-N4 (`logs archive as ground truth`). PREREG § 1.8 (round-4 dual-audit PASS/PASS) extended this base for the PPUT-CCL experiment. Each addition is a research-protocol orphan with explicit constitutional justification (case-law / measurement-fidelity / pre-registration commitment).
    44	
    45	| Path (manifest entry) | Justification |
    46	|---|---|
    47	| `src/kernel.rs` | FC3-N10 (`tape Q`) source — kernel topology immutability is a Law-1 invariant (Art. I.1) |
    48	| `src/wal.rs` | FC3-N11 (`log`) implementation — append-only WAL is the constitutional logs-archive surface |
    49	| `src/bus.rs` | FC1-N11/N13/N14 (`∏p`, `wtool`, `Q_{t+1}`) implementation — execution semantics that PPUT measures rest on |
    50	| `src/drivers/llm_http.rs` | FC1-N7 (`δ / AI`) — cost source-of-truth (prompt_tokens / completion_tokens). Tampering with this defeats every C_i count |
    51	| `src/sdk/prompt_guard.rs` | B6 PPUT-context-leak runtime gate — measurement-isolation invariant (no metric reaches agent prompt) |
    52	| `experiments/minif2f_v4/src/lean4_oracle.rs` | FC1-N12 (∏p ground-truth oracle) — Lean ground-truth cannot drift mid-experiment |
    53	| `experiments/minif2f_v4/src/cost_aggregator.rs` | B2 PPUT cost C_i computation — accounting invariant |
    54	| `experiments/minif2f_v4/src/wall_clock.rs` | B3 PPUT time T_i computation — accounting invariant |
    55	| `experiments/minif2f_v4/src/post_hoc_verifier.rs` | B4 verified-vs-runtime PPUT separation — accounting invariant |
    56	| `experiments/minif2f_v4/src/jsonl_schema.rs` | B1 emit schema — auditable artifact format |
    57	| `experiments/minif2f_v4/src/rollback_sim.rs` | B7-extra calibration toggle; PREREG § 5.5 commits a frozen `ROLLBACK_TX_THRESHOLD = 50` and a binary `SIMULATE_ROLLBACK_AT_TX_50` env var — tampering with either defeats the p_0 measurement |
    58	| `experiments/minif2f_v4/src/bin/evaluator.rs` | the wiring; tampering with it defeats every layer above |
    59	| `constitution.md` | FC3-N3 (constitution as ground truth) — direct |
    60	| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
    61	| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | sealed heldout split — pre-registration commitment per § 2.3 |
    62	| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | the spec being committed to — pre-registration anchor |
    63	
    64	`genesis_payload.toml` itself is **not** self-hashed (chicken-and-egg). The semantic anchor is the `[pput_accounting_0]` section content, not its hash. Section 6 below records this limitation.
    65	
    66	**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
    67	
    68	---
    69	
    70	## § 4. `src/boot.rs` is **not** in the Trust Root manifest
    71	
    72	Conscious choice — recorded here so the next reviewer does not file it as an oversight:
    73	
    74	- Trust Root's threat model = passive tamper between runs (file-system edits without recompile).
    75	- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
    76	- Adding `src/boot.rs` to its own manifest gives a slightly stronger passive-tamper guarantee (catches edits to boot.rs without recompile, e.g. on a deployed system where the binary and source are out of sync) at the cost of one more file to maintain.
    77	- Phase B7 chooses the smaller surface. Phase C+ may revisit if signed-binary attestation lands.
    78	
    79	---
    80	
    81	## § 5. Boot panic ↔ FC mapping
    82	
    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
    84	
    85	- FC2-N22 HALT requires the kernel/bus to be initialized (HaltReason variants are emitted by `TuringBus::halt_with_reason`).
    86	- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
    87	- Closer match: FC3-E14 (`init → error → re-init → boot`). Boot-panic is the immediate-abort variant; the surrounding harness (batch runner, supervisord, shell wrapper) is the "re-init" actor.
    88	
    89	See `OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` for the rationale to keep this as panic rather than promoting it into HaltReason. No constitution change requested.
    90	
    91	---
    92	
    93	## § 6. Updated stats (v1)
    94	
    95	Compared to v0:
    96	- ✅ count: **15 → 16** (+1: FC3-N34 promoted from 📅)
    97	- 📅 deferred: **7 → 6** (-1)
    98	- New orphan rows: **15 readonly extension paths** (above § 3) — each with constitutional justification, none requiring constitution change
    99	
   100	Targets at end of Phase B (Stage 2/3 completion + B7):
   101	- ✅ count: 38 + 1 = 39
   102	- 📅/📄: 10 - 1 + 0 = 9
   103	- 🔨/⚠️: 0 (per v0 § 4 actionable plan)
   104	
   105	v1 does not address remaining v0 ⚠️ rows; those are Stage 2/3 work that has not yet landed (out of B7 scope).
   106	
   107	---
   108	
   109	## § 7. Outstanding work flagged for next alignment cycle
   110	
   111	1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
   112	2. ~~**TRACE_MATRIX of B7-extra (p_0 calibration toggle)**~~ — landed. Final implementation differs slightly from the original sketch in this section: the constitutional `bus.register_predicate(...)` API does not currently exist on `main` (it lives on the unmerged `phase-z-wtool-tools` branch — TRACE_MATRIX_v0 row FC1-N11 references it aspirationally). Rather than scope-creep B7-extra into reviving Phase Z, the synthetic predicate is implemented at the evaluator layer in `rollback_sim.rs` with an explicit short-circuit at the threshold tx. The constitutional anchor (FC1-E18 ∏p=0 → Q_t repeated, then FC2-N22 HALT via existing `MaxTxExhausted`) is unchanged; only the abstraction depth differs. Listed under § 2 above as ✅ entries.
   113	3. **`src/boot.rs` self-hash decision** (§ 4 above) is open — Phase C+ revisit point.

exec
/bin/bash -lc "nl -ba handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# B7-extra Abstraction-Depth Findings — 2026-04-25
     2	
     3	**Status**: filed for Phase Z merge planning + Phase D ArchitectAI design constraints. Not blocking Phase B → C transition.
     4	**Trigger**: thesis re-alignment audit (user, 2026-04-25 mid-session) found two abstraction-depth gaps where the implementation works correctly *within its scope* but is below the constitutionally cleanest design depth.
     5	**Constitutional anchors**: thesis claims 7 ("white-box predicates settle state transitions") + 11 ("legible/enforceable/reversible state").
     6	
     7	---
     8	
     9	## Finding (A) — Synthetic predicate at evaluator-level, not bus-level
    10	
    11	### What is
    12	
    13	`SIMULATE_ROLLBACK_AT_TX_50=1` triggers a short-circuit at evaluator's `run_swarm` loop (`experiments/minif2f_v4/src/bin/evaluator.rs:503-518`) via `rollback_sim::should_simulate_rollback(tx, enabled)`. The kernel/bus is not informed; from `TuringBus`'s perspective tx 50 is just a tx that never happened.
    14	
    15	### What it should be
    16	
    17	Constitutionally cleanest = a `Predicate` impl `AlwaysRejectAfterTxN(50)` registered via `bus.register_predicate(...)` at swarm boot, returning `Reject` for all tx ≥ 50. Then the bus's existing `evaluate_predicates(ctx, payload)` routes through `Result::Vetoed` for every subsequent append, naturally exhausts at `max_transactions`, and emits `HaltReason::MaxTxExhausted` from the bus's own machinery.
    18	
    19	This implementation properly exercises the **FC1-N11 ∏p product gate** (TRACE_MATRIX FC1-N11). The current evaluator-level short-circuit BYPASSES the ∏p gate for these synthetic ticks — the bus has no record of "we tried to append at tx 50-199 and were vetoed", because we never asked the bus.
    20	
    21	### Why it's not done that way
    22	
    23	`Predicate` trait + `bus.register_predicate` API live on the unmerged `phase-z-wtool-tools` branch (commit `74b2ce7`). TRACE_MATRIX_v0 row FC1-N11 references them aspirationally but they never landed on `main`. Reviving Phase Z to enable a one-line predicate registration in B7-extra would be ~10x scope-creep on a measurement task.
    24	
    25	### Where the gap matters
    26	
    27	| Layer | Effect |
    28	|---|---|
    29	| Production runs (control, no toggle) | **No effect** — production goes through the existing inline forbidden_patterns + Lean4Oracle white-box predicate surface (`src/bus.rs:183`) |
    30	| Calibration treatment runs | The bus's tape never records 150 vetoed tx attempts; only the evaluator's tx_count==50 stamp + synthetic_short_circuit=true field signal what happened |
    31	| Phase D ArchitectAI design | When ArchitectAI generates user-space artifacts, the *real* predicate path will be exercised by `bus.append → ∏p → wtool` for every artifact-generated proposal. Phase D MUST land before any production system can enforce capability-compilation invariants at FC1-N11 depth — Phase D's predicate surface IS the full Phase Z ambition restored |
    32	| Phase B → C audit | Auditor should be told: "B7-extra synthetic veto is at evaluator layer; the bus's ∏p path is not exercised by calibration treatment. The control path (production) IS exercised, so the cost/time/progress measurements are valid for production-equivalent runs." |
    33	
    34	### Action items
    35	
    36	1. **Phase Z merge** (when scheduled): port `phase-z-wtool-tools` Predicate trait + `bus.register_predicate` + 3 default impls (`ForbiddenPatternPredicate`, `SorryPredicate`, `PayloadSizePredicate`) onto main.
    37	2. **B7-extra refactor** (post-Phase Z merge): replace evaluator short-circuit with `bus.register_predicate(AlwaysRejectAfterTxN(ROLLBACK_TX_THRESHOLD))` at run_swarm boot when toggle is on. Drop the short-circuit. The synthetic_short_circuit flag becomes an artifact of how p_0 was historically calibrated; do not re-calibrate (Trust Root has the frozen value).
    38	3. **Phase D design** (PREREG § 6 D2): document that ArchitectAI's user-space artifacts traverse the real ∏p path, so the abstraction depth gap closes naturally.
    39	
    40	---
    41	
    42	## Finding (B) — Cost asymmetry in synthetic-short-circuit treatment runs
    43	
    44	### What is
    45	
    46	When `synthetic_short_circuit=true` is stamped on a calibration treatment run's jsonl row, the `total_run_token_count` (C_i) reflects only the cost of tx 0-49 (the actual LLM calls that happened). A "true" 150-tx vetoed loop would have cost ~3x more (LLM calls every tx, all rejected).
    47	
    48	### Why it's not perfect
    49	
    50	PREREG § 5.5 conceives of `--simulate-rollback-at-tx-50` as "synthetic mid-run rollback mimicking worst-case stochastic edge case Phase E artifacts could trigger". A worst-case Phase E artifact would corrupt state but the loop would continue spending budget. We short-circuit instead — saving wall-clock but understating cost.
    51	
    52	### Where the gap matters
    53	
    54	| Analysis | Effect |
    55	|---|---|
    56	| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
    57	| Per-row PPUT computation on calibration jsonl | Affected — `pput_runtime` and `pput_verified` for synthetic_short_circuit rows are 0/x = 0 (correct, since solved=false), but if anyone interpreted the cost C_i as "what a real Phase E artifact would have cost", that would be wrong |
    58	| Aggregate cost statistics if these rows included | Significantly underreports |
    59	
    60	### Mitigation in place
    61	
    62	Field doc-comment on `PputResult::synthetic_short_circuit` warns:
    63	
    64	> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
    65	
    66	### Action items
    67	
    68	1. **Any future PPUT aggregation tool**: filter out `synthetic_short_circuit=true` rows or document explicitly that they are calibration-treatment-only.
    69	2. **Phase B → C audit packet**: include this finding so external auditors know what they're looking at.
    70	3. **Future calibration redesigns** (if needed): consider whether to (a) accept this asymmetry and remove the short-circuit (run all 200 tx, ~3x calibration cost ≈ +$10 over current $3-5), or (b) keep short-circuit and document.
    71	
    72	---
    73	
    74	## Why these findings are not thesis-level drift
    75	
    76	Thesis claim 7 (white-box predicates settle state) is satisfied for the **production path**: every real proposal flows through `bus.append → forbidden_patterns + Lean4Oracle` (white-box, deterministic, inspectable). The synthetic predicate's measurement-only path bypasses this for the specific purpose of estimating p_0 (a measurement quantity). The thesis applies to the system's claim about how it handles real proposals, not to how it does internal calibration measurements on itself.
    77	
    78	Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.
    79	
    80	Both findings are **abstraction-depth** issues, recorded for Phase Z merge planning and Phase D design constraints, not blockers for Phase B → C transition.
    81	
    82	---
    83	
    84	## Cross-references
    85	
    86	- `experiments/minif2f_v4/src/rollback_sim.rs` — module + 6 unit tests
    87	- `experiments/minif2f_v4/src/bin/evaluator.rs:503-518` — short-circuit site
    88	- `experiments/minif2f_v4/src/bin/evaluator.rs` `synthetic_short_circuit` field — disambiguator with cost-asymmetry warning
    89	- `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md` § 7.2 — implementation note
    90	- `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 5.5 — calibration protocol spec
    91	- Unmerged branch `phase-z-wtool-tools` commit `74b2ce7` — full Phase Z Predicate trait + bus.register_predicate API

exec
/bin/bash -lc "nl -ba handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Thesis v2 audit — "feedback from ground truth" alignment
     2	
     3	**Date**: 2026-04-25 (post user thesis update — added "feedback from ground truth" as physical anchor)
     4	**Purpose**: re-audit current Phase B7-extra implementation against the new claim 7 ("white-box predicates settle state transitions based on STRICT FEEDBACK FROM GROUND TRUTH") and claim 8 ("failure logs MUST be ground-truth-validated before quarantine").
     5	**Status**: 2 alignment gaps identified, both Phase D scope (consumer-side), neither blocks Phase B → C.
     6	
     7	---
     8	
     9	## Causal chain audit
    10	
    11	```
    12	Proposal (LLM)
    13	  → Feedback from Ground Truth (Lean / FS / external compiler)
    14	  → Logging (ground-truth-validated, isolated from active context)
    15	  → Capability Compilation (Phase D)
    16	  → ↑ H-VPPUT
    17	```
    18	
    19	For each link, where does the current Phase B7-extra system stand?
    20	
    21	### Link 1 — Proposal (black-box LLM)
    22	- ✅ Aligned. `client.generate` → `parse_agent_output` → `AgentOutput { tool: append/complete/invest/search }`. The LLM never settles state; it only proposes.
    23	
    24	### Link 2 — Feedback from Ground Truth
    25	- ✅ **Aligned for capability claims** (`complete` action triggering `oracle.verify_omega_detailed` at evaluator.rs:777,783,1016). Lean compiler returns objective verdict.
    26	- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
    27	- ⚠️ **Distinction**: non-capability transitions (`append` for tape comments, `invest` for market stakes, `search` for retrieval) are settled by **policy** (`forbidden_patterns`, `Sorry`, `PayloadSize`, market math) — NOT ground-truth. Per thesis spirit, this is acceptable: only capability claims need ground-truth. Tape comments are organisational; market stakes are economic; neither claims capability.
    28	- 📅 Phase E sealed eval on heldout-54 = ground-truth on capability **generalization** (Gate H requirement).
    29	
    30	### Link 3 — Logging (ground-truth-validated, isolated from active context)
    31	
    32	Two surfaces:
    33	
    34	#### 3a. WAL / `Ledger` (durable, structural)
    35	- `EventType` enum **defines** OmegaInvoke / OmegaAccepted / OmegaRejected / OmegaError (`src/ledger.rs:155-158`)
    36	- ❌ **GAP (Finding C)**: these Omega* events are **never emitted in production code**. Only RunStart, Append, Invest, MarketCreate, MarketResolve, RunEnd reach the ledger. The structured oracle verdict trail does not exist in the WAL.
    37	- Phase D ArchitectAI reading WAL alone would see NO Lean verdicts; it would have to infer success from Append events and absence of Append events from "no proposal" — losing the distinction between "tried + rejected" and "never tried".
    38	
    39	#### 3b. `recent_rejections` (in-memory queue, fed to next-tx agent prompt)
    40	- On oracle reject (evaluator.rs:876): `bus.record_rejection(agent_id, class.label())`
    41	- The CLASS LABEL is an abstracted classification from `classify_lean_error(err_detail)` — deterministic transform of ground-truth output, NOT raw oracle stderr.
    42	- ✅ **Aligned for thesis spirit (claim 8)**: agents see abstracted labels (e.g., "TacticFailed"), not raw ground-truth content. The strong physical isolation = "raw oracle stderr is in stdout, not in agent prompt". Step-B v3 (C-022 shield) was specifically introduced to enforce this.
    43	- ⚠️ **GAP (Finding D)**: `recent_rejections` mixes class labels from ground-truth rejects (Lean verdict) with class labels from policy rejects ("Forbidden pattern: decide", "Payload too long: ..."). Phase D consumer reading these labels has no field telling it which is which. Ground-truth rejects and policy rejects should be DIFFERENT signals to ArchitectAI.
    44	
    45	#### 3c. `PputResult` jsonl emit (durable, per-run summary)
    46	- ✅ **Aligned**. `verified: bool` field comes from `oracle.verify_omega_detailed` (B4 post_hoc_verifier separates `runtime_accepted` from `post_hoc_verified`). Each emitted row IS ground-truth-validated at run granularity.
    47	- For PPUT-CCL Phase D, the calibration jsonl IS the ground-truth-validated log Phase D consumes for capability compilation.
    48	
    49	### Link 4 — Capability Compilation (Phase D, not yet built)
    50	- 📅 Will read PputResult jsonl + WAL + stdout/stderr
    51	- The thesis-v2 critical requirement: ArchitectAI proposals MUST be tagged with which leg of the ground-truth feedback they're learning from. Phase D design must account for Findings C and D.
    52	
    53	### Link 5 — ↑ H-VPPUT (held-out verified PPUT)
    54	- 📅 Phase E sealed eval on heldout-54
    55	- ✅ Trust Root + B7 Boot freeze + B7-extra p_0 calibration build the measurement scaffolding for this. North Star is achievable iff the chain above is honored.
    56	
    57	---
    58	
    59	## Findings (C, D — extending B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md)
    60	
    61	### Finding C — WAL Omega* events never emitted
    62	
    63	**What**: `EventType::{OmegaInvoke, OmegaAccepted, OmegaRejected, OmegaError}` are declared in `src/ledger.rs:155-158` but no production code path emits them. The WAL contains only structural events; ground-truth verdicts are visible in stdout/jsonl, not in the durable ledger.
    64	
    65	**Why it matters per thesis-v2**: Claim 8 says ground-truth-validated failure logs must be quarantined for capability compilation. The WAL is the canonical durable log surface; if it lacks ground-truth events, Phase D ArchitectAI must reach into other surfaces (jsonl + stdout) to reconstruct ground-truth feedback. The reconstruction is doable but fragile.
    66	
    67	**Severity**: Phase D scope. Phase B's calibration jsonl is correctly ground-truth-validated at run granularity; intra-run ground-truth events are missing from WAL but will surface in stderr (raw) + jsonl (`verified` + tool_dist counters).
    68	
    69	**Action**: Phase D ArchitectAI design must specify which surface(s) it reads. Recommend: jsonl as primary ground-truth source, WAL as structural cross-reference, stderr as fallback for fine-grained diagnostics. OR (more invasive): emit OmegaInvoke/Accepted/Rejected/Error to ledger from `oracle.verify_omega_detailed` call sites (touches evaluator.rs but not bus.rs/kernel.rs — outside STEP_B_PROTOCOL restricted-file zone).
    70	
    71	### Finding D — `recent_rejections` mixes policy + ground-truth class labels
    72	
    73	**What**: `bus.record_rejection(author, reason)` is called from BOTH:
    74	- Policy paths: forbidden_pattern (`bus.rs:186`), payload too long (`bus.rs:198`), too many lines (`bus.rs:205`), tool veto (`bus.rs:215`)
    75	- Ground-truth path: oracle reject after classification (`evaluator.rs:876`)
    76	
    77	The `recent_rejections(author, max)` consumer (line 594, fed into agent prompt) sees a flat list of class labels, no provenance tag.
    78	
    79	**Why it matters per thesis-v2**: Claim 7 says white-box predicates settle state with ground-truth feedback. For agent next-tx learning (FC1's basic cycle), labels of any kind may be acceptable — agents adapt to any signal that helps them produce non-rejectable output. But for Phase D ArchitectAI compiling capability, the signal source matters: ArchitectAI should learn from ground-truth (Lean said this proof tactic doesn't work) differently than from policy (we banned `decide` for brute-force-prevention).
    80	
    81	**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
    82	
    83	**Action**: when Phase D ArchitectAI consumer is built, tag bus rejections at the call site with `provenance: GroundTruth | Policy` — extend BusResult::Vetoed or add a separate `record_rejection_classified(author, reason, provenance)` API. Backwards-compatible if the existing `record_rejection` defaults provenance to Policy (since 4 of 5 call sites are policy).
    84	
    85	---
    86	
    87	## Why neither finding blocks Phase B → C
    88	
    89	Phase B delivers MEASUREMENT SCAFFOLDING. The PPUT-CCL arc Gate B → C transition validates:
    90	- A1-A5 ✓
    91	- B1-B7 ✓ (Trust Root + Boot freeze landed)
    92	- B7-extra (in flight): p_0 calibration via 576 runs
    93	
    94	The calibration jsonl rows are **per-run ground-truth-validated** (`verified` field comes from Lean4Oracle). Findings C+D are about **intra-run** ground-truth event granularity (per-tx, per-proposal) — needed by Phase D ArchitectAI but not by Phase B's measurement gate.
    95	
    96	Thesis-v2 alignment status:
    97	- Phase B: ✅ aligned (per-run ground-truth-validated jsonl is the measurement surface)
    98	- Phase C: should plan for Finding C/D before Phase D consumer is built
    99	- Phase D: MUST close Findings C+D as a precondition
   100	
   101	---
   102	
   103	## Cross-references
   104	
   105	- `~/.claude/.../memory/project_thesis.md` — thesis v2 with 5-step compile loop + 11 atomic claims
   106	- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` — Findings A+B (abstraction depth)
   107	- `src/ledger.rs:147-160` — EventType enum (Omega* declared, never emitted in production)
   108	- `src/bus.rs:186,198,205,215,444` — record_rejection call sites (4 policy + 1 from evaluator at oracle reject)
   109	- `experiments/minif2f_v4/src/bin/evaluator.rs:876` — bus.record_rejection at oracle reject
   110	- `experiments/minif2f_v4/src/lean4_oracle.rs:112,177` — Lean ground-truth verdict surfaces
   111	- PREREG § 1.7 ArtifactState lifecycle — Phase D consumer specification anchor

exec
/bin/bash -lc "nl -ba src/bus.rs | sed -n '170,225p;430,455p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   170	    /// by construction legitimate — re-rejecting at bus level would block the
   171	    /// wtool write that Art. IV mandates. Only oracle-accepted payloads should
   172	    /// take this path. Payload-size caps are also relaxed (proofs are longer than
   173	    /// agent scratch steps).
   174	    pub fn append_oracle_accepted(&mut self, author: &str, payload: &str,
   175	                                   parent_id: Option<&str>) -> Result<BusResult, String> {
   176	        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ true)
   177	    }
   178	
   179	    fn append_internal(&mut self, author: &str, payload: &str,
   180	                       parent_id: Option<&str>, oracle_blessed: bool) -> Result<BusResult, String> {
   181	        // Phase 0: Forbidden pattern check — skipped for oracle-accepted payloads.
   182	        if !oracle_blessed {
   183	            for pattern in &self.config.forbidden_patterns {
   184	                if payload.contains(pattern.as_str()) {
   185	                    let reason = format!("Forbidden pattern: {}", pattern);
   186	                    self.record_rejection(author, &reason);
   187	                    return Ok(BusResult::Vetoed { reason });
   188	                }
   189	            }
   190	        }
   191	
   192	        // Phase 0b: Payload size limits (V3L-21). Skipped for oracle-accepted since
   193	        // real proofs can legitimately exceed the per-step scratch budget.
   194	        if !oracle_blessed {
   195	            if payload.len() > self.config.max_payload_chars {
   196	                let reason = format!("Payload too long: {} > {} chars",
   197	                                     payload.len(), self.config.max_payload_chars);
   198	                self.record_rejection(author, &reason);
   199	                return Ok(BusResult::Vetoed { reason });
   200	            }
   201	            let line_count = payload.lines().count();
   202	            if line_count > self.config.max_payload_lines {
   203	                let reason = format!("Too many lines: {} > {}",
   204	                                     line_count, self.config.max_payload_lines);
   205	                self.record_rejection(author, &reason);
   206	                return Ok(BusResult::Vetoed { reason });
   207	            }
   208	        }
   209	
   210	        // Phase 1: Tool pre-append hooks
   211	        let mut signal = ToolSignal::Pass;
   212	        for tool in &mut self.tools {
   213	            match tool.on_pre_append(author, payload) {
   214	                ToolSignal::Veto(reason) => {
   215	                    self.record_rejection(author, &reason);
   216	                    return Ok(BusResult::Vetoed { reason });
   217	                }
   218	                ToolSignal::InvestOnly { target_node, amount, direction } => {
   219	                    signal = ToolSignal::InvestOnly { target_node, amount, direction };
   220	                    break;
   221	                }
   222	                ToolSignal::YieldReward { reward } => {
   223	                    signal = ToolSignal::YieldReward { reward };
   224	                }
   225	                ToolSignal::Pass => {}
   430	            if tool.manifest() == "wallet" {
   431	                if let Some(wallet) = tool.as_any_mut().downcast_mut::<crate::sdk::tools::wallet::WalletTool>() {
   432	                    wallet.credit(agent, amount);
   433	                    return;
   434	                }
   435	            }
   436	        }
   437	    }
   438	
   439	    /// Record a rejection in the graveyard.
   440	    /// Step-B v3: ALL stored entries are bounded class labels (C-022 shield enforced at write).
   441	    /// If `reason` is already a valid class label (starts with "err:"), stored as-is.
   442	    /// Otherwise normalized to a bus-level class via `bus_classify`.
   443	    /// Exposed publicly so evaluator.rs can populate from OMEGA-reject and parse-fail.
   444	    pub fn record_rejection(&mut self, author: &str, reason: &str) {
   445	        let label = Self::bus_classify(reason);
   446	        self.graveyard
   447	            .entry(author.to_string())
   448	            .or_default()
   449	            .push(label.to_string());
   450	    }
   451	
   452	    /// Bus-level classifier: coerces any rejection reason to a bounded label.
   453	    /// This is the write-side shield that enforces Art. II.1 end-to-end.
   454	    /// The finite label set is the union of:
   455	    ///   - "err:" prefixed labels from sdk::error_abstraction (caller-classified)

exec
/bin/bash -lc 'rg -n "OmegaInvoke|OmegaAccepted|OmegaRejected|OmegaError|record_rejection|append_oracle_accepted|halt_and_settle|evaluate_predicates|forbidden_patterns" src experiments/minif2f_v4/src | head -120' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/evaluator.rs:388:        forbidden_patterns: vec![
experiments/minif2f_v4/src/bin/evaluator.rs:419:                    forbidden_patterns: vec![
experiments/minif2f_v4/src/bin/evaluator.rs:817:                                        // payload, so bus-level forbidden_patterns and size caps
experiments/minif2f_v4/src/bin/evaluator.rs:820:                                        let omega_node_id = match bus.append_oracle_accepted(
experiments/minif2f_v4/src/bin/evaluator.rs:842:                                        bus.halt_and_settle(&gp).ok();
experiments/minif2f_v4/src/bin/evaluator.rs:876:                                        bus.record_rejection(agent_id, class.label());
experiments/minif2f_v4/src/bin/evaluator.rs:1030:                                        let _ = bus.append_oracle_accepted(
experiments/minif2f_v4/src/bin/evaluator.rs:1040:                                        bus.halt_and_settle(&gp).ok();
experiments/minif2f_v4/src/bin/evaluator.rs:1060:                                        match bus.append_oracle_accepted(
experiments/minif2f_v4/src/bin/evaluator.rs:1077:                                        bus.record_rejection(agent_id, class.label());
experiments/minif2f_v4/src/bin/evaluator.rs:1095:                        bus.record_rejection(agent_id, class.label());
src/ledger.rs:147:/// V3L-09: explicit vocabulary — only OmegaAccepted is a true OMEGA event.
src/ledger.rs:155:    OmegaInvoke,
src/ledger.rs:156:    OmegaAccepted,
src/ledger.rs:157:    OmegaRejected,
src/ledger.rs:158:    OmegaError,
src/ledger.rs:170:            EventType::OmegaInvoke => write!(f, "OmegaInvoke"),
src/ledger.rs:171:            EventType::OmegaAccepted => write!(f, "OmegaAccepted"),
src/ledger.rs:172:            EventType::OmegaRejected => write!(f, "OmegaRejected"),
src/ledger.rs:173:            EventType::OmegaError => write!(f, "OmegaError"),
src/ledger.rs:481:        // V3L-09: only OmegaAccepted is the canonical OMEGA event
src/ledger.rs:483:        ledger.append(EventType::OmegaInvoke, Some("n1".into()), None, None).unwrap();
src/ledger.rs:484:        ledger.append(EventType::OmegaAccepted, Some("n1".into()), None, None).unwrap();
src/ledger.rs:486:        assert_eq!(ledger.events()[1].event_type, EventType::OmegaAccepted);
experiments/minif2f_v4/src/jsonl_schema.rs:252:        let legacy_line = r#"{"problem":"/tmp/foo.lean","condition":"n8","model":"deepseek-chat","has_golden_path":true,"time_secs":781.99,"pput":0.127,"gp_token_count":769,"gp_node_count":7,"tx_count":16,"build_sha":"61ccc21","classifier_version":"v1_2026-04-16-a","boltzmann_seed":141421,"halt_reason":"OmegaAccepted","reputation_at_end":{"Agent_1":2}}"#;
experiments/minif2f_v4/src/jsonl_schema.rs:261:                           Some("OmegaAccepted"));
src/bus.rs:21:    pub forbidden_patterns: Vec<String>,
src/bus.rs:30:            forbidden_patterns: Vec::new(),
src/bus.rs:167:    /// The forbidden_patterns list (C-011) exists to prevent agents from appending
src/bus.rs:174:    pub fn append_oracle_accepted(&mut self, author: &str, payload: &str,
src/bus.rs:183:            for pattern in &self.config.forbidden_patterns {
src/bus.rs:186:                    self.record_rejection(author, &reason);
src/bus.rs:198:                self.record_rejection(author, &reason);
src/bus.rs:205:                self.record_rejection(author, &reason);
src/bus.rs:215:                    self.record_rejection(author, &reason);
src/bus.rs:336:    pub fn halt_and_settle(&mut self, golden_path: &[NodeId]) -> Result<(), String> {
src/bus.rs:444:    pub fn record_rejection(&mut self, author: &str, reason: &str) {
src/bus.rs:599:            forbidden_patterns: vec!["FORBIDDEN".to_string()],
src/bus.rs:674:    fn test_bus_halt_and_settle() {
src/bus.rs:677:            bus.halt_and_settle(&[node_id.clone()]).unwrap();

exec
/bin/bash -lc "nl -ba src/ledger.rs | sed -n '140,170p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   140	        Self::new()
   141	    }
   142	}
   143	
   144	// ── Ledger event log ────────────────────────────────────────────
   145	
   146	/// Event types for the append-only event ledger.
   147	/// V3L-09: explicit vocabulary — only OmegaAccepted is a true OMEGA event.
   148	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
   149	pub enum EventType {
   150	    RunStart,
   151	    Append,
   152	    Invest,
   153	    MarketCreate,
   154	    MarketResolve,
   155	    OmegaInvoke,
   156	    OmegaAccepted,
   157	    OmegaRejected,
   158	    OmegaError,
   159	    RunEnd,
   160	}
   161	
   162	impl fmt::Display for EventType {
   163	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   164	        match self {
   165	            EventType::RunStart => write!(f, "RunStart"),
   166	            EventType::Append => write!(f, "Append"),
   167	            EventType::Invest => write!(f, "Invest"),
   168	            EventType::MarketCreate => write!(f, "MarketCreate"),
   169	            EventType::MarketResolve => write!(f, "MarketResolve"),
   170	            EventType::OmegaInvoke => write!(f, "OmegaInvoke"),

codex
The main path confirms a few important facts: the treatment exits before the first prompt of tx 50, `synthetic_short_circuit` is only set at that return site, and the estimator currently skips missing calibration tags. I’m checking whether those facts cross the line from documented limitation into pre-batch blocker.
exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/lib.rs | sed -n '1,60p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	pub mod lean4_oracle;
     2	pub mod jsonl_schema;
     3	pub mod cost_aggregator;
     4	pub mod wall_clock;
     5	pub mod post_hoc_verifier;
     6	pub mod rollback_sim;

exec
/bin/bash -lc "rg -n \"MODEL_SNAPSHOT|ACTIVE_MODEL|BINARY_SHA256|BUILD_SHA|SPLIT|MODE|BOLTZMANN_SEED|max_transactions|MAX\" experiments/minif2f_v4/src/bin/evaluator.rs handover/preregistration/scripts/run_p0_calibration.sh experiments/minif2f_v4/src -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/preregistration/scripts/run_p0_calibration.sh:25:#   export ACTIVE_MODEL=deepseek-chat           (default)
handover/preregistration/scripts/run_p0_calibration.sh:37:export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
handover/preregistration/scripts/run_p0_calibration.sh:42:SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"
handover/preregistration/scripts/run_p0_calibration.sh:53:MODES=("control" "treatment")
handover/preregistration/scripts/run_p0_calibration.sh:67:d = json.load(open('$SPLITS_JSON'))
handover/preregistration/scripts/run_p0_calibration.sh:109:TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
handover/preregistration/scripts/run_p0_calibration.sh:112:echo "Mode count:    ${#MODES[@]} (control + treatment)"
handover/preregistration/scripts/run_p0_calibration.sh:120:for MODE in "${MODES[@]}"; do
handover/preregistration/scripts/run_p0_calibration.sh:121:    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
handover/preregistration/scripts/run_p0_calibration.sh:122:    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
handover/preregistration/scripts/run_p0_calibration.sh:125:    case "$MODE" in
handover/preregistration/scripts/run_p0_calibration.sh:135:                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND, skip"
handover/preregistration/scripts/run_p0_calibration.sh:138:            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
handover/preregistration/scripts/run_p0_calibration.sh:139:            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
handover/preregistration/scripts/run_p0_calibration.sh:143:                BOLTZMANN_SEED="$SEED" \
handover/preregistration/scripts/run_p0_calibration.sh:155:row['calibration_mode'] = '$MODE'
experiments/minif2f_v4/src/bin/evaluator.rs:36:const DEFAULT_BOLTZMANN_SEED: u64 = 74677;  // same as sample seed (BTC/USD external)
experiments/minif2f_v4/src/bin/evaluator.rs:62:    /// "adaptation" | "meta_validation" | "heldout" — read from SPLIT env;
experiments/minif2f_v4/src/bin/evaluator.rs:100:    /// MODE env, default "full" Phase B.
experiments/minif2f_v4/src/bin/evaluator.rs:174:        eprintln!("  MINIF2F_DIR, LLM_PROXY_URL, ACTIVE_MODEL env vars");
experiments/minif2f_v4/src/bin/evaluator.rs:182:    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-reasoner".into());
experiments/minif2f_v4/src/bin/evaluator.rs:477:    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
experiments/minif2f_v4/src/bin/evaluator.rs:478:        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
experiments/minif2f_v4/src/bin/evaluator.rs:480:    let max_transactions = 200;
experiments/minif2f_v4/src/bin/evaluator.rs:528:    for tx in 0..max_transactions {
experiments/minif2f_v4/src/bin/evaluator.rs:531:        // tx_count at the threshold, not at max_transactions, so jsonl
experiments/minif2f_v4/src/bin/evaluator.rs:1127:              max_transactions as u64, Some(tool_dist), upr,
experiments/minif2f_v4/src/bin/evaluator.rs:1165:    let build_sha = std::env::var("BUILD_SHA").ok();
experiments/minif2f_v4/src/bin/evaluator.rs:1167:    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
experiments/minif2f_v4/src/bin/evaluator.rs:1186:    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
experiments/minif2f_v4/src/bin/evaluator.rs:1187:        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
experiments/minif2f_v4/src/bin/evaluator.rs:1188:                   (Phase B convention; pre-registration requires SPLIT \
experiments/minif2f_v4/src/bin/evaluator.rs:1192:    let mode = std::env::var("MODE").unwrap_or_else(|_| "full".to_string());
experiments/minif2f_v4/src/bin/evaluator.rs:1193:    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
experiments/minif2f_v4/src/bin/evaluator.rs:1196:    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();
experiments/minif2f_v4/src/bin/evaluator.rs:1323:        std::env::set_var("SPLIT", "adaptation");
experiments/minif2f_v4/src/bin/evaluator.rs:1324:        std::env::set_var("MODE", "full");
experiments/minif2f_v4/src/bin/evaluator.rs:1366:        std::env::remove_var("SPLIT");
experiments/minif2f_v4/src/bin/evaluator.rs:1367:        std::env::remove_var("MODE");
experiments/minif2f_v4/src/bin/evaluator.rs:1377:        std::env::set_var("SPLIT", "adaptation");
experiments/minif2f_v4/src/bin/evaluator.rs:1378:        std::env::set_var("MODE", "soft_law");
experiments/minif2f_v4/src/bin/evaluator.rs:1401:        std::env::remove_var("SPLIT");
experiments/minif2f_v4/src/bin/evaluator.rs:1402:        std::env::remove_var("MODE");
experiments/minif2f_v4/src/bin/evaluator.rs:36:const DEFAULT_BOLTZMANN_SEED: u64 = 74677;  // same as sample seed (BTC/USD external)
experiments/minif2f_v4/src/bin/evaluator.rs:62:    /// "adaptation" | "meta_validation" | "heldout" — read from SPLIT env;
experiments/minif2f_v4/src/bin/evaluator.rs:100:    /// MODE env, default "full" Phase B.
experiments/minif2f_v4/src/bin/evaluator.rs:174:        eprintln!("  MINIF2F_DIR, LLM_PROXY_URL, ACTIVE_MODEL env vars");
experiments/minif2f_v4/src/bin/evaluator.rs:182:    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-reasoner".into());
experiments/minif2f_v4/src/bin/evaluator.rs:477:    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
experiments/minif2f_v4/src/bin/evaluator.rs:478:        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
experiments/minif2f_v4/src/bin/evaluator.rs:480:    let max_transactions = 200;
experiments/minif2f_v4/src/bin/evaluator.rs:528:    for tx in 0..max_transactions {
experiments/minif2f_v4/src/bin/evaluator.rs:531:        // tx_count at the threshold, not at max_transactions, so jsonl
experiments/minif2f_v4/src/bin/evaluator.rs:1127:              max_transactions as u64, Some(tool_dist), upr,
experiments/minif2f_v4/src/bin/evaluator.rs:1165:    let build_sha = std::env::var("BUILD_SHA").ok();
experiments/minif2f_v4/src/bin/evaluator.rs:1167:    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
experiments/minif2f_v4/src/bin/evaluator.rs:1186:    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
experiments/minif2f_v4/src/bin/evaluator.rs:1187:        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
experiments/minif2f_v4/src/bin/evaluator.rs:1188:                   (Phase B convention; pre-registration requires SPLIT \
experiments/minif2f_v4/src/bin/evaluator.rs:1192:    let mode = std::env::var("MODE").unwrap_or_else(|_| "full".to_string());
experiments/minif2f_v4/src/bin/evaluator.rs:1193:    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
experiments/minif2f_v4/src/bin/evaluator.rs:1196:    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();
experiments/minif2f_v4/src/bin/evaluator.rs:1323:        std::env::set_var("SPLIT", "adaptation");
experiments/minif2f_v4/src/bin/evaluator.rs:1324:        std::env::set_var("MODE", "full");
experiments/minif2f_v4/src/bin/evaluator.rs:1366:        std::env::remove_var("SPLIT");
experiments/minif2f_v4/src/bin/evaluator.rs:1367:        std::env::remove_var("MODE");
experiments/minif2f_v4/src/bin/evaluator.rs:1377:        std::env::set_var("SPLIT", "adaptation");
experiments/minif2f_v4/src/bin/evaluator.rs:1378:        std::env::set_var("MODE", "soft_law");
experiments/minif2f_v4/src/bin/evaluator.rs:1401:        std::env::remove_var("SPLIT");
experiments/minif2f_v4/src/bin/evaluator.rs:1402:        std::env::remove_var("MODE");
experiments/minif2f_v4/src/rollback_sim.rs:7:// 50..max_transactions. The run then exhausts naturally and exits via
experiments/minif2f_v4/src/rollback_sim.rs:17:// `tx_count` stamped at threshold rather than `max_transactions` — a
experiments/minif2f_v4/src/rollback_sim.rs:47:/// here, naturally exhaust at `max_transactions`" — see module header.
experiments/minif2f_v4/src/wall_clock.rs:68:    /// which is the right thing for the no-OMEGA exit at max_transactions.

exec
/bin/bash -lc 'rg -n "p0|compute_p0|baseline_regression|p0_calibration|MEASUREMENT_ERROR|synthetic_short_circuit|total_run_token_count" -S .' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'rg -n "trust_root|verify_trust_root|src/main.rs|Cargo.lock|cases/MANIFEST|genesis_payload" tests src handover experiments/minif2f_v4/src -S' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
./tools/aggregate_e1v2.py:117:        # exclude problems where either side is MEASUREMENT_ERROR
./experiments/minif2f_v4/run_list.sh:68:        echo "MEASUREMENT_ERROR/TIMEOUT"
./experiments/minif2f_v4/run_subset.sh:74:        echo "MEASUREMENT_ERROR"
./experiments/minif2f_v4/src/cost_aggregator.rs:9://   total_run_token_count = Σ over EVERY proposal in the run of
./experiments/minif2f_v4/src/cost_aggregator.rs:96:    pub fn total_run_token_count(&self) -> u64 {
./experiments/minif2f_v4/src/cost_aggregator.rs:138:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./experiments/minif2f_v4/src/cost_aggregator.rs:148:        assert_eq!(acc.total_run_token_count(), 0);
./experiments/minif2f_v4/run_batch.sh:180:        echo "MEASUREMENT_ERROR (exit=$EXIT_CODE, see $(basename "$STDERR_LOG"))"
./experiments/minif2f_v4/src/jsonl_schema.rs:93:    pub total_run_token_count: u64,
./experiments/minif2f_v4/src/jsonl_schema.rs:200:            total_run_token_count: 4096,
./experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./experiments/minif2f_v4/src/bin/evaluator.rs:70:    total_run_token_count: u64,
./experiments/minif2f_v4/src/bin/evaluator.rs:148:    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
./experiments/minif2f_v4/src/bin/evaluator.rs:149:    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
./experiments/minif2f_v4/src/bin/evaluator.rs:151:    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./experiments/minif2f_v4/src/bin/evaluator.rs:155:    synthetic_short_circuit: Option<bool>,
./experiments/minif2f_v4/src/bin/evaluator.rs:157:    // total_run_token_count / failed_branch_count / total_wall_time_ms /
./experiments/minif2f_v4/src/bin/evaluator.rs:211:            // total_run_token_count from r1 were silently discarded). PPUT-CCL
./experiments/minif2f_v4/src/bin/evaluator.rs:305:                                 Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:334:                              Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:343:                              Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:353:                              Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:364:            eprintln!("MEASUREMENT_ERROR oneshot LLM: {}", e);
./experiments/minif2f_v4/src/bin/evaluator.rs:542:                                       Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:547:            // analysis. See PputResult::synthetic_short_circuit doc-comment
./experiments/minif2f_v4/src/bin/evaluator.rs:549:            result.synthetic_short_circuit = Some(true);
./experiments/minif2f_v4/src/bin/evaluator.rs:869:                                                        Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:1054:                                                        Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:1129:              Some(acc.total_run_token_count()),
./experiments/minif2f_v4/src/bin/evaluator.rs:1143:    total_run_token_count: Option<u64>,
./experiments/minif2f_v4/src/bin/evaluator.rs:1174:    let c_i = total_run_token_count.unwrap_or(0);
./experiments/minif2f_v4/src/bin/evaluator.rs:1220:        total_run_token_count: c_i,
./experiments/minif2f_v4/src/bin/evaluator.rs:1254:        synthetic_short_circuit: None,
./experiments/minif2f_v4/src/bin/evaluator.rs:1353:                assert_eq!(agg.total_run_token_count, 2000);
./experiments/minif2f_v4/tests/trust_root_immutability.rs:129:        "baseline_regression_rate",
./experiments/minif2f_v4/tests/trust_root_immutability.rs:130:        "baseline_regression_jsonl_sha256",
./experiments/minif2f_v4/tests/pput_anti_goodhart.rs:360:    assert_eq!(acc.total_run_token_count(), expected as u64,
./experiments/minif2f_v4/analysis/run_interleaved.sh:64:    # exit_code 2 = MEASUREMENT_ERROR (API/env failure per our convention)
./experiments/minif2f_v4/analysis/run_interleaved.sh:136:        echo "MEASUREMENT_ERROR/TIMEOUT"
./genesis_payload.toml:6:#                         (PREREG § 1.8). baseline_regression_rate +
./genesis_payload.toml:7:#                         baseline_regression_jsonl_sha256 are placeholders
./genesis_payload.toml:55:baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
./genesis_payload.toml:56:baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
./experiments/minif2f_v4/run_broad.sh:91:        echo "MEASUREMENT_ERROR/TIMEOUT"
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:21:**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration. PASS only if you would stake your independence on the batch result being valid.
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:28:- B7-extra commits: `973a9fd` (toggle), `b0ae03e` (calibration runner + estimator), `35e221d` (synthetic_short_circuit field), `3868b1d` (architect-insights A+B), `1875543` (thesis-v2 audit C+D)
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:32:  - Hard #2 (rebuilt with synthetic_short_circuit field, same problem): tx_count=50, solved=false, **synthetic_short_circuit=true** ✓, total_run_token_count=39947
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:42:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:54:The B7-extra design claim: "synthetic ∏p=0 from tx 50 onward" maps to FC1-E18 (∏p=0 → Q_t preservation, repeated 150x) followed by natural FC2-N22 HALT via `HaltReason::MaxTxExhausted`. The IMPLEMENTATION is a short-circuit at tx==50 in `experiments/minif2f_v4/src/bin/evaluator.rs:503-518`, with a `synthetic_short_circuit=true` field stamping the row.
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:58:- (Q1.b) The `synthetic_short_circuit` field's doc comment warns about cost-asymmetry. Is the warning sufficient, or should the evaluator EMIT a stderr-level WARN at every short-circuit so log-readers also see it (currently `info!` not `warn!`)?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:59:- (Q1.c) Does the `synthetic_short_circuit` flag correctly serialize as `null` (omitted) for control runs and `true` only for treatment? (The smoke confirms this empirically; check the code path.)
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:79:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:80:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. Should we fail loudly?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:81:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:100:- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50. Does anything downstream INTERPRET these as suspicious?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:104:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2.
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:117:- (Q7.b) If any individual run hits the 40-min timeout, runner skips with MEASUREMENT_ERROR. Does this artificially raise p_0?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:130:| `synthetic_short_circuit` field | calibration disambiguator | jsonl emit | ? |
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:159:runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:160:compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:191:    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n{runner_sh}\n```\n" +
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:192:    f"\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n{compute_py}\n```\n" +
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:227:          f"**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n"
./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:33:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:74:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:82:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:19:**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration (= retread the full $3-5 + 8h + Trust Root churn). PASS only if you would stake your independence on the batch result being valid.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:26:- B7-extra commits: `973a9fd` (toggle), `b0ae03e` (calibration runner + estimator), `35e221d` (synthetic_short_circuit field), `3868b1d` (architect-insights A+B), `1875543` (thesis-v2 audit C+D)
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:30:  - Hard #2 (rebuilt with synthetic_short_circuit field, same problem): tx_count=50, solved=false, **synthetic_short_circuit=true** ✓, total_run_token_count=39947 (50 tx of real activity)
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:40:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:52:The B7-extra design claim: "synthetic ∏p=0 from tx 50 onward" maps to FC1-E18 (∏p=0 → Q_t preservation, repeated 150x) followed by natural FC2-N22 HALT via `HaltReason::MaxTxExhausted`. The IMPLEMENTATION is a short-circuit at tx==50 in `experiments/minif2f_v4/src/bin/evaluator.rs:503-518`, with a `synthetic_short_circuit=true` field stamping the row.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:56:- (Q1.b) The `synthetic_short_circuit` field's doc comment warns about cost-asymmetry. Is the warning sufficient, or should the evaluator EMIT a stderr-level WARN at every short-circuit so log-readers also see it (currently `info!` not `warn!`)?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:58:- (Q1.d) Does the `synthetic_short_circuit` flag correctly serialize as `null` (omitted) for control runs and `true` only for treatment? (The smoke confirms this empirically; check the code path.)
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:77:- (Q3.b) `compute_p0.py` per-problem uses `max(over seeds)` — worst-case framing. If only 1 of 2 seeds regressed, the problem still counts as 1. With 2 seeds, this DOUBLES the regression rate vs `mean(over seeds)`. PREREG § 5.5 line 450 explicitly says max — agreed?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:78:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. For a synthetic_short_circuit row, both will be False (correctly UNSOLVED). For a control row that SOLVED, both will be True. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:79:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. If the runner script's row-enrichment failed for any subset (e.g., python3 OOM'd on one row), would calibration silently miss data? Should we fail loudly?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:80:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:97:- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)? `compute_p0.py` joins on calibration_problem_id + calibration_seed and reads only `solved` / `progress_verified` — neither field comes from WAL or recent_rejections. Argument for "no impact": the calibration jsonl IS ground-truth-validated independently. Verify.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:99:- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50 (correctly reflects 50 tx of real activity). Does anything downstream INTERPRET these as "if they were lower than control, the treatment toggle is bug-prone"? compute_p0.py doesn't; flag if any other tooling does.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:103:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2 if ceiling violated.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:106:- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it and continue? Test by inspection: where in the pipeline does compute_p0.py run, and what handles exit code?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:116:- (Q7.b) If any individual run hits the 40-min timeout, the runner skips with MEASUREMENT_ERROR. Does this artificially raise p_0 (by counting timeouts as treatment-UNSOLVED while control might have SOLVED)?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:129:| `synthetic_short_circuit` field | calibration disambiguator | jsonl emit | ? |
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:166:printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:167:cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:169:printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:170:cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:199:  printf '**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n'
./handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md:39:The fix for `run_oneshot` is correct. It now properly emits a `MEASUREMENT_ERROR` and returns a non-successful result (`evaluator.rs:253`) instead of silently degrading to an in-memory bus. This upholds the durability guarantee for oneshot runs.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:666:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:17:    -   **Cost**: The cost accumulator is *not* identical. A true 150-tx vetoed loop would involve LLM calls for each proposal, accumulating cost. The short-circuit explicitly prevents this. This is acknowledged in Finding (B) (`B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS...`) and the `synthetic_short_circuit` doc comment (`evaluator.rs:149-151`), which directly contradicts the `rollback_sim.rs` header's claim.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:19:    -   **Conclusion**: The equivalence is a convenience that is valid *only because* `compute_p0.py` ignores cost and intra-run state, joining only on the final `SOLVED`/`UNSOLVED` status. The constitutional anchor is weak; it achieves an outcome analogous to FC2-N22 HALT, but it does not *traverse* the constitutional path of FC1-E18 (repeatedly). The documentation in `rollback_sim.rs` should be amended to state this is a *functionally equivalent outcome for p_0 estimation* rather than a generally "observably equivalent" process.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:23:-   **(Q1.c) Serialization of `synthetic_short_circuit`**: **PASS**. The implementation is correct. The field `synthetic_short_circuit` in `PputResult` (`evaluator.rs:142`) is an `Option<bool>` with `#[serde(skip_serializing_if = "Option::is_none")]`. The default construction path in `make_pput` (`evaluator.rs:1252`) sets it to `None`, ensuring it is omitted for control runs. The treatment short-circuit path (`evaluator.rs:517`) explicitly sets it to `Some(true)`. This matches the smoke test verification.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:41:-   **(Q3.b) `max(over seeds)` framing**: **PASS**. `PREREG § 5.5` line 450 explicitly specifies `max over the 2 seeds`. The implementation in `compute_p0.py:84` correctly reflects this policy (`if regression > per_problem_regression[pid]: ...`). This is a consistent implementation of a pre-registered, "worst-case" policy choice.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:43:-   **(Q3.c) `solved()` predicate**: **PASS**. The `solved()` function in `compute_p0.py:44-48` correctly prioritizes `progress_verified` over the legacy `has_golden_path`, which aligns with the B4 audit findings and the `RunAggregate::V2` schema.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:45:-   **(Q3.d) Silent drop in `compute_p0.py`**: **CHALLENGE**. The script at `compute_p0.py:56-58` silently skips rows that are missing `calibration_problem_id` or `calibration_seed`. A failure in the runner script (`run_p0_calibration.sh:170-177`) to stamp a row would cause that data point to be silently dropped, biasing the `p_0` result without warning. The script should fail loudly if any row from either input file is missing the required keys.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:53:-   **(Q4.c) Counter-argument (measurement artifact)**: **PASS**. This is the correct and necessary framing. The thesis describes the intended production behavior of the system. The calibration is a meta-process to measure a parameter *about* the system. It is acceptable for the measurement apparatus to not perfectly mirror the system under test, as long as the deviation is understood and accounted for. Finding (A) and the `synthetic_short_circuit` flag provide this accounting.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:60:-   **(Q5.c) Downstream interpretation**: **PASS**. The `synthetic_short_circuit` flag and its associated doc comment (`evaluator.rs:149-151`) are sufficient mitigation for Finding (B) (cost asymmetry). Any downstream tool that misinterprets the cost of these rows is ignoring the explicit warning provided in the data schema.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:64:-   **(Q6.a) `exit 2` abort**: **CHALLENGE**. The runner script `run_p0_calibration.sh` does not use `set -e`. If `compute_p0.py` exits with code 2, the script will continue executing, printing the summary box. An automated runner could miss the failure. The script should use `set -e` or explicitly check the exit code of `compute_p0.py` and abort.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:71:-   **(Q7.b) Timeout impact on `p_0`**: **CHALLENGE**. A run that times out is logged as `MEASUREMENT_ERROR` (`run_p0_calibration.sh:195`). This means the (problem, seed) pair will be missing from one of the jsonl files. `compute_p0.py` will then exclude this pair from its analysis (`set(c.keys()) & set(t.keys())`). This introduces sampling bias. A timeout is a valid outcome (UNSOLVED) and should be treated as such, not as a data error. The runner script should be modified to emit a valid UNSOLVED jsonl row for timed-out runs.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:100:3.  **CHALLENGE Fix**: Modify `compute_p0.py` to fail loudly if any row is missing the required `calibration_*` keys, preventing silent data loss and biased results. (`Q3.d`)
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:101:4.  **CHALLENGE Fix**: Modify `run_p0_calibration.sh` to treat a timeout as a valid `UNSOLVED` outcome and emit a corresponding JSONL row, instead of a `MEASUREMENT_ERROR` that leads to data exclusion. (`Q7.b`)
./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:37:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:78:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:86:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
./handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md:48:**Audit framing** (Codex): hybrid_v1 condition runs `run_oneshot` first; on failure runs `run_swarm`. The merged result construction at evaluator.rs:128-141 uses `..r2` field-spread to inherit, keeping only `r2`'s C_i / T_i / failed_branch_count / total_run_token_count / total_wall_time_ms. The failed oneshot leg's tokens vanish from the merged total.
./handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md:59:       total_run_token_count: match (r1.total_run_token_count, r2.total_run_token_count) {
./handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md:80:- If aggregation chosen: a synthetic test where r1.total_run_token_count = 1000, r2.total_run_token_count = 5000 → combined.total_run_token_count == 6000 (NOT 5000).
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:518:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:520:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:522:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1327:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1329:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1331:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2737:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md:43:- `eprintln!("MEASUREMENT_ERROR ...")` emitted on WAL failure.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:26:**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration (= retread the full $3-5 + 8h + Trust Root churn). PASS only if you would stake your independence on the batch result being valid.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:33:- B7-extra commits: `973a9fd` (toggle), `b0ae03e` (calibration runner + estimator), `35e221d` (synthetic_short_circuit field), `3868b1d` (architect-insights A+B), `1875543` (thesis-v2 audit C+D)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:37:  - Hard #2 (rebuilt with synthetic_short_circuit field, same problem): tx_count=50, solved=false, **synthetic_short_circuit=true** ✓, total_run_token_count=39947 (50 tx of real activity)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:59:The B7-extra design claim: "synthetic ∏p=0 from tx 50 onward" maps to FC1-E18 (∏p=0 → Q_t preservation, repeated 150x) followed by natural FC2-N22 HALT via `HaltReason::MaxTxExhausted`. The IMPLEMENTATION is a short-circuit at tx==50 in `experiments/minif2f_v4/src/bin/evaluator.rs:503-518`, with a `synthetic_short_circuit=true` field stamping the row.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:63:- (Q1.b) The `synthetic_short_circuit` field's doc comment warns about cost-asymmetry. Is the warning sufficient, or should the evaluator EMIT a stderr-level WARN at every short-circuit so log-readers also see it (currently `info!` not `warn!`)?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:65:- (Q1.d) Does the `synthetic_short_circuit` flag correctly serialize as `null` (omitted) for control runs and `true` only for treatment? (The smoke confirms this empirically; check the code path.)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:84:- (Q3.b) `compute_p0.py` per-problem uses `max(over seeds)` — worst-case framing. If only 1 of 2 seeds regressed, the problem still counts as 1. With 2 seeds, this DOUBLES the regression rate vs `mean(over seeds)`. PREREG § 5.5 line 450 explicitly says max — agreed?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:85:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. For a synthetic_short_circuit row, both will be False (correctly UNSOLVED). For a control row that SOLVED, both will be True. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:86:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. If the runner script's row-enrichment failed for any subset (e.g., python3 OOM'd on one row), would calibration silently miss data? Should we fail loudly?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:87:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:104:- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)? `compute_p0.py` joins on calibration_problem_id + calibration_seed and reads only `solved` / `progress_verified` — neither field comes from WAL or recent_rejections. Argument for "no impact": the calibration jsonl IS ground-truth-validated independently. Verify.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:106:- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50 (correctly reflects 50 tx of real activity). Does anything downstream INTERPRET these as "if they were lower than control, the treatment toggle is bug-prone"? compute_p0.py doesn't; flag if any other tooling does.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:110:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2 if ceiling violated.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:113:- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it and continue? Test by inspection: where in the pipeline does compute_p0.py run, and what handles exit code?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:123:- (Q7.b) If any individual run hits the 40-min timeout, the runner skips with MEASUREMENT_ERROR. Does this artificially raise p_0 (by counting timeouts as treatment-UNSOLVED while control might have SOLVED)?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:136:| `synthetic_short_circuit` field | calibration disambiguator | jsonl emit | ? |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:591:#                         (PREREG § 1.8). baseline_regression_rate +
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:592:#                         baseline_regression_jsonl_sha256 are placeholders
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:640:baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:641:baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:665:## handover/preregistration/scripts/run_p0_calibration.sh (NEW)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:683:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:725:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:727:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:836:                echo "MEASUREMENT_ERROR"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:861:    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:865:    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:870:## handover/preregistration/scripts/compute_p0.py (NEW)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:885:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:922:    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:964:    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:973:        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:974:        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:975:        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1000:    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1005:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1035:+    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1036:+    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1038:+    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1042:+    synthetic_short_circuit: Option<bool>,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1044:     // total_run_token_count / failed_branch_count / total_wall_time_ms /
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1077:+                                       Some(acc.total_run_token_count()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1082:+            // analysis. See PputResult::synthetic_short_circuit doc-comment
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1084:+            result.synthetic_short_circuit = Some(true);
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1097:+        synthetic_short_circuit: None,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1180:**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1263:| Calibration treatment runs | The bus's tape never records 150 vetoed tx attempts; only the evaluator's tx_count==50 stamp + synthetic_short_circuit=true field signal what happened |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1270:2. **B7-extra refactor** (post-Phase Z merge): replace evaluator short-circuit with `bus.register_predicate(AlwaysRejectAfterTxN(ROLLBACK_TX_THRESHOLD))` at run_swarm boot when toggle is on. Drop the short-circuit. The synthetic_short_circuit flag becomes an artifact of how p_0 was historically calibrated; do not re-calibrate (Trust Root has the frozen value).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1279:When `synthetic_short_circuit=true` is stamped on a calibration treatment run's jsonl row, the `total_run_token_count` (C_i) reflects only the cost of tx 0-49 (the actual LLM calls that happened). A "true" 150-tx vetoed loop would have cost ~3x more (LLM calls every tx, all rejected).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1289:| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1290:| Per-row PPUT computation on calibration jsonl | Affected — `pput_runtime` and `pput_verified` for synthetic_short_circuit rows are 0/x = 0 (correct, since solved=false), but if anyone interpreted the cost C_i as "what a real Phase E artifact would have cost", that would be wrong |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1295:Field doc-comment on `PputResult::synthetic_short_circuit` warns:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1297:> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1301:1. **Any future PPUT aggregation tool**: filter out `synthetic_short_circuit=true` rows or document explicitly that they are calibration-treatment-only.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1311:Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1321:- `experiments/minif2f_v4/src/bin/evaluator.rs` `synthetic_short_circuit` field — disambiguator with cost-asymmetry warning
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1411:**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1466:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1468:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1470:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1669:   148	    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1670:   149	    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1672:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1676:   155	    synthetic_short_circuit: Option<bool>,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1678:   157	    // total_run_token_count / failed_branch_count / total_wall_time_ms /
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1852:   869	                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1955:  1220	        total_run_token_count: c_i,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1989:  1254	        synthetic_short_circuit: None,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2327:   542	                                       Some(acc.total_run_token_count()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2332:   547	            // analysis. See PputResult::synthetic_short_circuit doc-comment
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2334:   549	            result.synthetic_short_circuit = Some(true);
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2370:  1054	                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2446:  1129	              Some(acc.total_run_token_count()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2460:  1143	    total_run_token_count: Option<u64>,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2485:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2502:    16	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2544:    58	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2546:    60	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2655:   169	                echo "MEASUREMENT_ERROR"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2680:   194	    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2684:   198	    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2688:/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2702:    13	    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2739:    50	    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2781:    92	    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2790:   101	        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2791:   102	        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2792:   103	        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2817:   128	    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2822:   133	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2840:     6	#                         (PREREG § 1.8). baseline_regression_rate +
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2841:     7	#                         baseline_regression_jsonl_sha256 are placeholders
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2889:    55	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2890:    56	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2913:/bin/bash -lc "rg -n \"5\\.5 p_0|Ceiling:|Estimator:|Freeze point|Audit trail|Toggle:|Boltzmann|baseline_regression|ArtifactState|Trust Root composition|cases/C\" handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2928:455:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2929:457:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3078:   455	                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3080:   457	                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3082:   459	                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3168:    66	**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3249:    30	| Calibration treatment runs | The bus's tape never records 150 vetoed tx attempts; only the evaluator's tx_count==50 stamp + synthetic_short_circuit=true field signal what happened |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3256:    37	2. **B7-extra refactor** (post-Phase Z merge): replace evaluator short-circuit with `bus.register_predicate(AlwaysRejectAfterTxN(ROLLBACK_TX_THRESHOLD))` at run_swarm boot when toggle is on. Drop the short-circuit. The synthetic_short_circuit flag becomes an artifact of how p_0 was historically calibrated; do not re-calibrate (Trust Root has the frozen value).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3265:    46	When `synthetic_short_circuit=true` is stamped on a calibration treatment run's jsonl row, the `total_run_token_count` (C_i) reflects only the cost of tx 0-49 (the actual LLM calls that happened). A "true" 150-tx vetoed loop would have cost ~3x more (LLM calls every tx, all rejected).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3275:    56	| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3276:    57	| Per-row PPUT computation on calibration jsonl | Affected — `pput_runtime` and `pput_verified` for synthetic_short_circuit rows are 0/x = 0 (correct, since solved=false), but if anyone interpreted the cost C_i as "what a real Phase E artifact would have cost", that would be wrong |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3281:    62	Field doc-comment on `PputResult::synthetic_short_circuit` warns:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3283:    64	> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3287:    68	1. **Any future PPUT aggregation tool**: filter out `synthetic_short_circuit=true` rows or document explicitly that they are calibration-treatment-only.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3297:    78	Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3307:    88	- `experiments/minif2f_v4/src/bin/evaluator.rs` `synthetic_short_circuit` field — disambiguator with cost-asymmetry warning
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3395:    81	**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3593:The main path confirms a few important facts: the treatment exits before the first prompt of tx 50, `synthetic_short_circuit` is only set at that return site, and the estimator currently skips missing calibration tags. I’m checking whether those facts cross the line from documented limitation into pre-batch blocker.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3605:/bin/bash -lc "rg -n \"MODEL_SNAPSHOT|ACTIVE_MODEL|BINARY_SHA256|BUILD_SHA|SPLIT|MODE|BOLTZMANN_SEED|max_transactions|MAX\" experiments/minif2f_v4/src/bin/evaluator.rs handover/preregistration/scripts/run_p0_calibration.sh experiments/minif2f_v4/src -g '*.rs'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3607:handover/preregistration/scripts/run_p0_calibration.sh:25:#   export ACTIVE_MODEL=deepseek-chat           (default)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3608:handover/preregistration/scripts/run_p0_calibration.sh:37:export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3609:handover/preregistration/scripts/run_p0_calibration.sh:42:SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3610:handover/preregistration/scripts/run_p0_calibration.sh:53:MODES=("control" "treatment")
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3611:handover/preregistration/scripts/run_p0_calibration.sh:67:d = json.load(open('$SPLITS_JSON'))
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3612:handover/preregistration/scripts/run_p0_calibration.sh:109:TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3613:handover/preregistration/scripts/run_p0_calibration.sh:112:echo "Mode count:    ${#MODES[@]} (control + treatment)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3614:handover/preregistration/scripts/run_p0_calibration.sh:120:for MODE in "${MODES[@]}"; do
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3615:handover/preregistration/scripts/run_p0_calibration.sh:121:    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3616:handover/preregistration/scripts/run_p0_calibration.sh:122:    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3617:handover/preregistration/scripts/run_p0_calibration.sh:125:    case "$MODE" in
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3618:handover/preregistration/scripts/run_p0_calibration.sh:135:                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND, skip"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3619:handover/preregistration/scripts/run_p0_calibration.sh:138:            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3620:handover/preregistration/scripts/run_p0_calibration.sh:139:            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3621:handover/preregistration/scripts/run_p0_calibration.sh:143:                BOLTZMANN_SEED="$SEED" \
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3622:handover/preregistration/scripts/run_p0_calibration.sh:155:row['calibration_mode'] = '$MODE'
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:475:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:477:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:479:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1885:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4425:`p_0` is calibrated as a Phase B6 deliverable (Phase B7 if delayed): measured baseline regression rate when ablating any single recent code change on adaptation set. Pre-committed `p_0` value goes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` before Phase E starts. Phase E cannot start without `p_0` frozen.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4847:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6164:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:43:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:84:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:92:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:155:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:289:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:291:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:293:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:341://   total_run_token_count = Σ over EVERY proposal in the run of
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:418:    pub fn total_run_token_count(&self) -> u64 {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:460:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:470:        assert_eq!(acc.total_run_token_count(), 0);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:878:+    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:928:+                                 Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:951:+                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:959:+                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:968:+                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1030:+                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1074:+                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1108:+              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1118:+    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1134:+        match (total_run_token_count, total_wall_time_ms) {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1161:+        total_run_token_count,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1329:    83	    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1479:     9	//   total_run_token_count = Σ over EVERY proposal in the run of
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1556:    86	    pub fn total_run_token_count(&self) -> u64 {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1598:   128	        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1608:   138	        assert_eq!(acc.total_run_token_count(), 0);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2203:   244	                                 Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2226:   267	                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2233:   274	                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2241:   282	                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2252:   293	            eprintln!("MEASUREMENT_ERROR oneshot LLM: {}", e);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2709:   745	                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2895:   926	                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2968:   999	              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2982:  1013	    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2999:  1030	        match (total_run_token_count, total_wall_time_ms) {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3037:  1068	        total_run_token_count,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3166:    93	    pub total_run_token_count: u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3273:   200	            total_run_token_count: 4096,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3311:   238	        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3312:   239	        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3336:/bin/bash -lc "rg -n \"client\\.generate|record_llm_call|record_proposal|flip_last_failed|verify_omega|verify_partial|mark_first_read|mark_final_accept|make_pput|verify_post_hoc|total_run_token_count|pput_verified|RunAggregate|progress\" experiments/minif2f_v4/src src -g '*.rs'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3338:experiments/minif2f_v4/src/cost_aggregator.rs:9://   total_run_token_count = Σ over EVERY proposal in the run of
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3343:experiments/minif2f_v4/src/cost_aggregator.rs:86:    pub fn total_run_token_count(&self) -> u64 {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3348:experiments/minif2f_v4/src/cost_aggregator.rs:128:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3349:experiments/minif2f_v4/src/cost_aggregator.rs:138:        assert_eq!(acc.total_run_token_count(), 0);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3401:experiments/minif2f_v4/src/bin/evaluator.rs:83:    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3411:experiments/minif2f_v4/src/bin/evaluator.rs:244:                                 Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3416:experiments/minif2f_v4/src/bin/evaluator.rs:267:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3418:experiments/minif2f_v4/src/bin/evaluator.rs:274:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3420:experiments/minif2f_v4/src/bin/evaluator.rs:282:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3431:experiments/minif2f_v4/src/bin/evaluator.rs:745:                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3436:experiments/minif2f_v4/src/bin/evaluator.rs:926:                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3439:experiments/minif2f_v4/src/bin/evaluator.rs:999:              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3441:experiments/minif2f_v4/src/bin/evaluator.rs:1013:    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3444:experiments/minif2f_v4/src/bin/evaluator.rs:1030:        match (total_run_token_count, total_wall_time_ms) {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3451:experiments/minif2f_v4/src/bin/evaluator.rs:1068:        total_run_token_count,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3457:experiments/minif2f_v4/src/jsonl_schema.rs:93:    pub total_run_token_count: u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3476:experiments/minif2f_v4/src/jsonl_schema.rs:200:            total_run_token_count: 4096,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3487:experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3488:experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4464:/bin/bash -lc "rg -n \"serde_json::from_str::<.*PputResult|PputResult|RunRecord::from_json|RunAggregate|total_run_token_count|failed_branch_count|total_wall_time_ms|progress|schema_version|pput_verified\" experiments src handover -g '*.rs' -g '*.py' -g '*.sh' -g '*.md' -g '*.jsonl' -g '*.toml'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4466:experiments/minif2f_v4/src/cost_aggregator.rs:9://   total_run_token_count = Σ over EVERY proposal in the run of
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4472:experiments/minif2f_v4/src/cost_aggregator.rs:86:    pub fn total_run_token_count(&self) -> u64 {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4473:experiments/minif2f_v4/src/cost_aggregator.rs:128:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4475:experiments/minif2f_v4/src/cost_aggregator.rs:138:        assert_eq!(acc.total_run_token_count(), 0);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4525:experiments/minif2f_v4/src/bin/evaluator.rs:83:    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4535:experiments/minif2f_v4/src/bin/evaluator.rs:244:                                 Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4537:experiments/minif2f_v4/src/bin/evaluator.rs:267:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4539:experiments/minif2f_v4/src/bin/evaluator.rs:274:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4541:experiments/minif2f_v4/src/bin/evaluator.rs:282:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4544:experiments/minif2f_v4/src/bin/evaluator.rs:745:                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4546:experiments/minif2f_v4/src/bin/evaluator.rs:926:                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4548:experiments/minif2f_v4/src/bin/evaluator.rs:999:              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4550:experiments/minif2f_v4/src/bin/evaluator.rs:1013:    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4555:experiments/minif2f_v4/src/bin/evaluator.rs:1030:        match (total_run_token_count, total_wall_time_ms) {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4563:experiments/minif2f_v4/src/bin/evaluator.rs:1068:        total_run_token_count,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4577:experiments/minif2f_v4/src/jsonl_schema.rs:93:    pub total_run_token_count: u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4602:experiments/minif2f_v4/src/jsonl_schema.rs:200:            total_run_token_count: 4096,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4617:experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4618:experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4629:handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:33:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4636:handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:74:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4639:handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:82:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4666:handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:37:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4673:handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:78:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4676:handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:86:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4686:handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:666:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4718:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2737:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4741:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:847:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4754:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2164:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4766:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:43:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4773:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:84:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4776:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:92:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4778:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:155:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4785:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:341://   total_run_token_count = Σ over EVERY proposal in the run of
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4791:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:418:    pub fn total_run_token_count(&self) -> u64 {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4792:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:460:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4794:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:470:        assert_eq!(acc.total_run_token_count(), 0);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4844:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:878:+    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4851:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:928:+                                 Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4853:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:951:+                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4855:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:959:+                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4857:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:968:+                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4859:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1030:+                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4861:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1074:+                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4863:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1108:+              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4865:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1118:+    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4870:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1134:+        match (total_run_token_count, total_wall_time_ms) {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4878:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1161:+        total_run_token_count,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4884:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1329:    83	    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4894:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1479:     9	//   total_run_token_count = Σ over EVERY proposal in the run of
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4900:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1556:    86	    pub fn total_run_token_count(&self) -> u64 {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4901:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1598:   128	        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4903:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1608:   138	        assert_eq!(acc.total_run_token_count(), 0);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4951:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2203:   244	                                 Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4953:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2226:   267	                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4955:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2233:   274	                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4957:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2241:   282	                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4960:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2709:   745	                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4962:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2895:   926	                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4964:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2968:   999	              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4966:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2982:  1013	    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4971:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2999:  1030	        match (total_run_token_count, total_wall_time_ms) {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4979:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3037:  1068	        total_run_token_count,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4993:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3166:    93	    pub total_run_token_count: u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5018:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3273:   200	            total_run_token_count: 4096,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5033:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3311:   238	        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5034:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3312:   239	        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5039:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3336:/bin/bash -lc "rg -n \"client\\.generate|record_llm_call|record_proposal|flip_last_failed|verify_omega|verify_partial|mark_first_read|mark_final_accept|make_pput|verify_post_hoc|total_run_token_count|pput_verified|RunAggregate|progress\" experiments/minif2f_v4/src src -g '*.rs'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5040:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3338:experiments/minif2f_v4/src/cost_aggregator.rs:9://   total_run_token_count = Σ over EVERY proposal in the run of
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5041:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3343:experiments/minif2f_v4/src/cost_aggregator.rs:86:    pub fn total_run_token_count(&self) -> u64 {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5042:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3348:experiments/minif2f_v4/src/cost_aggregator.rs:128:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5043:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3349:experiments/minif2f_v4/src/cost_aggregator.rs:138:        assert_eq!(acc.total_run_token_count(), 0);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5091:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3401:experiments/minif2f_v4/src/bin/evaluator.rs:83:    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5095:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3411:experiments/minif2f_v4/src/bin/evaluator.rs:244:                                 Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5096:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3416:experiments/minif2f_v4/src/bin/evaluator.rs:267:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5097:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3418:experiments/minif2f_v4/src/bin/evaluator.rs:274:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5098:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3420:experiments/minif2f_v4/src/bin/evaluator.rs:282:                              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5099:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3431:experiments/minif2f_v4/src/bin/evaluator.rs:745:                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5100:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3436:experiments/minif2f_v4/src/bin/evaluator.rs:926:                                                        Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5101:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3439:experiments/minif2f_v4/src/bin/evaluator.rs:999:              Some(acc.total_run_token_count()),
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5102:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3441:experiments/minif2f_v4/src/bin/evaluator.rs:1013:    total_run_token_count: Option<u64>,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5104:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3444:experiments/minif2f_v4/src/bin/evaluator.rs:1030:        match (total_run_token_count, total_wall_time_ms) {
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5111:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3451:experiments/minif2f_v4/src/bin/evaluator.rs:1068:        total_run_token_count,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5117:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3457:experiments/minif2f_v4/src/jsonl_schema.rs:93:    pub total_run_token_count: u64,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5136:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3476:experiments/minif2f_v4/src/jsonl_schema.rs:200:            total_run_token_count: 4096,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5147:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3487:experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5148:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3488:experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5186:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1885:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5205:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4847:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5218:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6164:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5233:handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:144:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5241:handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:69:    total_run_token_count: u64,              // C_i (sum over all proposals)
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5249:handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:129:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5275:handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:35:Removed silent in-memory fallback in run_oneshot. If `TuringBus::with_wal_path` fails, emit `MEASUREMENT_ERROR oneshot WAL: <e>` and return a non-result PputResult (condition=`measurement_error`, solved=false, tx=1). Batch runner retries; no bus without WAL will ever claim OMEGA.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5368:    69	    total_run_token_count: u64,              // C_i (sum over all proposals)
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5428:   129	- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5517:   283	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5518:   284	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5564:   330	- Write p_0 value to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5565:   331	- Compute SHA-256 of the calibration jsonl file → write to `[pput_accounting_0].baseline_regression_jsonl_sha256`
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5933:handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:69:    total_run_token_count: u64,              // C_i (sum over all proposals)
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5937:handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:129:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:6233:experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:6234:experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:425:`p_0` is calibrated as a Phase B6 deliverable (Phase B7 if delayed): measured baseline regression rate when ablating any single recent code change on adaptation set. Pre-committed `p_0` value goes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` before Phase E starts. Phase E cannot start without `p_0` frozen.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:847:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2164:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/evidence/ADVERSARIAL_AUDIT_2026-04-23.md:19:| 7 | Hidden `MEASUREMENT_ERROR` mid-batch | **None** in any E1 or Phase 9.A jsonl | AUDIT 8 |
./handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md:81:**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:30:| Calibration treatment runs | The bus's tape never records 150 vetoed tx attempts; only the evaluator's tx_count==50 stamp + synthetic_short_circuit=true field signal what happened |
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:37:2. **B7-extra refactor** (post-Phase Z merge): replace evaluator short-circuit with `bus.register_predicate(AlwaysRejectAfterTxN(ROLLBACK_TX_THRESHOLD))` at run_swarm boot when toggle is on. Drop the short-circuit. The synthetic_short_circuit flag becomes an artifact of how p_0 was historically calibrated; do not re-calibrate (Trust Root has the frozen value).
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:46:When `synthetic_short_circuit=true` is stamped on a calibration treatment run's jsonl row, the `total_run_token_count` (C_i) reflects only the cost of tx 0-49 (the actual LLM calls that happened). A "true" 150-tx vetoed loop would have cost ~3x more (LLM calls every tx, all rejected).
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:56:| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:57:| Per-row PPUT computation on calibration jsonl | Affected — `pput_runtime` and `pput_verified` for synthetic_short_circuit rows are 0/x = 0 (correct, since solved=false), but if anyone interpreted the cost C_i as "what a real Phase E artifact would have cost", that would be wrong |
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:62:Field doc-comment on `PputResult::synthetic_short_circuit` warns:
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:64:> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:68:1. **Any future PPUT aggregation tool**: filter out `synthetic_short_circuit=true` rows or document explicitly that they are calibration-treatment-only.
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:78:Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:88:- `experiments/minif2f_v4/src/bin/evaluator.rs` `synthetic_short_circuit` field — disambiguator with cost-asymmetry warning
./handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:144:golden_path_token_count, total_run_token_count, total_wall_time_ms,
./handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md:100:| 2718 | **complete 50/50** (halt_reason fix applied) | 13/50 | 75.35 | 5.80 CI [3.65, 7.94] | 263 | {MaxTxExhausted: 36, OmegaAccepted: 13, MEASUREMENT_ERROR: 1} |
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2166:+    let p0 = snap.get_portfolio("Agent_0").expect("Agent_0 portfolio");
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2168:+    let pos_a = p0.get(&node_a).expect("Agent_0 position on node A");
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2173:+    assert!(p0.get(&node_b).is_none(),
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:35:Removed silent in-memory fallback in run_oneshot. If `TuringBus::with_wal_path` fails, emit `MEASUREMENT_ERROR oneshot WAL: <e>` and return a non-result PputResult (condition=`measurement_error`, solved=false, tx=1). Batch runner retries; no bus without WAL will ever claim OMEGA.
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:255:- No hidden MEASUREMENT_ERROR
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:281:- **C-027 `max_transactions` hardcoded**: silently ignored env var; fix at commit `d721506`. Previously this caused outer wallclock timeouts on hard problems to register as `MEASUREMENT_ERROR` rather than structured `FAIL`.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:40:      Instead, emit `MEASUREMENT_ERROR oneshot WAL: <e>` and return a
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:336:+                                    Emitting MEASUREMENT_ERROR for this problem.", e);
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:337:+                            eprintln!("MEASUREMENT_ERROR oneshot WAL: {}", e);
./handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:5:**Data**: E1 v2 data collected 2026-04-24 (4 Boltzmann seeds × 3 conditions × 10 hard problems = 120 paired trials). Serialized to max 2 parallel batches after proxy-saturation finding (§ 3.5). Zero MEASUREMENT_ERROR in final 120 trials. Runtime BUILD_SHA: `29ab43a`.
./handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:105:During the initial v2 data run, we observed that launching 12 concurrent batches × 8 agents = 96 concurrent DeepSeek API requests produced a 73% MEASUREMENT_ERROR rate (the 900s outer wallclock fired before MAX_TRANSACTIONS=50 was reached). At ≤ 2 concurrent batches, MEASUREMENT_ERROR rate was 0%. We therefore serialized the run to max 2 parallel batches. This constraint was NOT in the original PREREG; we flag it here as a deviation from pre-reg, with clean-data re-run.
./handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:130:All numbers below are computed by `tools/aggregate_e1v2.py` from raw jsonl; see `handover/preregistration/E1v2_RESULTS_2026-04-24.json` for the machine-readable source. Zero MEASUREMENT_ERROR events in the 120 final trials (serial re-run after § 3.5 deviation).
./handover/ai-direct/REGISTRATION_PHASE_9_2026-04-22.md:238:- `MEASUREMENT_ERROR oneshot/swarm WAL` → retry once, then abandon seed (substitute with NEXT pre-registered backup seed: `{31, 1618, 1729}` in order)
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:15:| MEASUREMENT_ERROR (outer 900s timeout hit) | 22 | **73%** |
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:22:| Configuration | MEASUREMENT_ERROR rate |
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:37:- MEASUREMENT_ERROR clusters on HARDEST problems (amc12_2000_p12, amc12_2000_p6) where MAX_TRANSACTIONS=50 is actually hit, not early solves
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:39:**Artifact class**: when proxy saturates, MEASUREMENT_ERROR is emitted instead of FAIL. This is silently equivalent to "this condition failed on this problem" in aggregation scripts, but in reality the evaluator never had a chance to either solve or halt cleanly.
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:47:All v1 batches ran 2-way parallel (A + B simultaneously per seed). MEASUREMENT_ERROR count across v1:
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:52:- `E1_A_seed2718`: 0 (1 MEASUREMENT_ERROR seen was on problem 11 / 20, proxy load lower then)
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:61:### 3.2 Phase 9.A baseline data: 1 MEASUREMENT_ERROR on seed 2718 (negligible, <2%)
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:67:12-way parallel batches produced 73% MEASUREMENT_ERROR. Unusable. Will re-run with reduced concurrency.
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:81:    produces MEASUREMENT_ERROR artifacts that contaminate FAIL statistics.
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:123:- **Null-measurement class**: MEASUREMENT_ERROR must be classified (not silently aggregated as FAIL)
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:131:User caught this BEFORE I did. The proxy saturation was producing MEASUREMENT_ERROR at 73%, and my earlier reports said "this is a hard problem, common-mode on both sides, doesn't bias paired Δ". That was wrong — the contamination is not benign; it destroys all FAIL statistics.
./handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:133:Paper 1 v2 methodology will explicitly include `max_parallel_batches` + `MEASUREMENT_ERROR exclusion rule`.
./handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md:74:abort_condition: if any batch returns MEASUREMENT_ERROR > 2/10, investigate before interpreting; do NOT pool.
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:69:    total_run_token_count: u64,              // C_i (sum over all proposals)
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:129:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:283:baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:284:baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:330:- Write p_0 value to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:331:- Compute SHA-256 of the calibration jsonl file → write to `[pput_accounting_0].baseline_regression_jsonl_sha256`
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:455:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:457:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:459:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/ai-direct/PLAN_V3_1_2026-04-15.md:79:**Definition**: Problems where all 3 conditions emitted a PPUT_RESULT (not abort-truncated, not MEASUREMENT_ERROR).
./handover/ai-direct/LATEST.md:39:  - `genesis_payload.toml` (new): `[pput_accounting_0]` (PREREG § 1.8 invariants — schema_version, progress/cost/time defs, verified_predicate, heldout_sealed_hash, source_pool_sha256, k_max=10, n_max=34, baseline_regression_rate placeholder); `[trust_root]` (15 SHA-256 entries — independently re-derived: PREREG § 1.8 base 8 + audit accounting 6 + B6 prompt_guard 1)
./handover/ai-direct/LATEST.md:48:  - `handover/preregistration/scripts/run_p0_calibration.sh`: iterates adaptation-144 × seeds [31415, 2718] × {control, treatment} = 576 runs; `--smoke` flag = 4-run probe
./handover/ai-direct/LATEST.md:49:  - `handover/preregistration/scripts/compute_p0.py`: control/treatment pair → regression_p_seed → max-over-seeds → p_0; PREREG § 5.5 ceiling = 0.10
./handover/ai-direct/LATEST.md:50:  - **Smoke verified 2026-04-25**: easy problem mathd_algebra_107 (4 runs, 39s) — infra + jsonl V2 + calibration tags ✓; hard problem aime_1983_p2 with toggle ON (8.5 min) — tx_count=50 + synthetic_short_circuit=true + stderr "[rollback_sim] firing at tx=50" ✓. Field cost-asymmetry doc-comment warns downstream PPUT analysis to honor flag.
./handover/ai-direct/LATEST.md:51:  - **Next**: user GO → 576-run batch (~$3-5, ~8h overnight) → compute_p0.py → write p_0 to genesis_payload.toml [pput_accounting_0] → recompute Trust Root + commit jsonl into manifest → Gate B dual-audit Phase B → Phase C transition
./handover/ai-direct/LATEST.md:61:- B7-extra (p_0 calibration) 未跑：576 runs (288 control + 288 treatment) on adaptation-144 × seeds [31415, 2718], `--simulate-rollback-at-tx-50` toggle 待加；p_0 ∈ (0, 0.10] sanity gate；冻结进 `[pput_accounting_0].baseline_regression_rate` + `.baseline_regression_jsonl_sha256` + 把 jsonl 加入 [trust_root]
./handover/ai-direct/LATEST.md:90:5. Freeze: write p_0 to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`; SHA-256 the calibration jsonl → `.baseline_regression_jsonl_sha256`; add jsonl path to `[trust_root]`; recompute every Trust Root hash (genesis itself changed); commit
./handover/preregistration/scripts/run_p0_calibration.sh:16:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/preregistration/scripts/run_p0_calibration.sh:58:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/preregistration/scripts/run_p0_calibration.sh:60:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/preregistration/scripts/run_p0_calibration.sh:169:                echo "MEASUREMENT_ERROR"
./handover/preregistration/scripts/run_p0_calibration.sh:194:    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/preregistration/scripts/run_p0_calibration.sh:198:    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/preregistration/scripts/compute_p0.py:13:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/preregistration/scripts/compute_p0.py:50:    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/preregistration/scripts/compute_p0.py:92:    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/preregistration/scripts/compute_p0.py:101:        "p0": p0,
./handover/preregistration/scripts/compute_p0.py:102:        "p0_ceiling": 0.10,
./handover/preregistration/scripts/compute_p0.py:103:        "ceiling_pass": p0 <= 0.10,
./handover/preregistration/scripts/compute_p0.py:128:    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
./handover/preregistration/scripts/compute_p0.py:133:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:66:**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).

 succeeded in 0ms:
src/main.rs:14:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
src/boot.rs:13:// `[trust_root]` manifest in `genesis_payload.toml`. Any mismatch =>
src/boot.rs:14:// `TrustRootError::Tampered { .. }`. `src/main.rs` panics with
src/boot.rs:19:// see header comment in `genesis_payload.toml`.
src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
src/boot.rs:64:/// tracked file's SHA-256 against the `genesis_payload.toml [trust_root]`
src/boot.rs:69:/// `repo_root` is the directory containing `genesis_payload.toml` (typically
src/boot.rs:71:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
src/boot.rs:72:    let genesis_path = repo_root.join("genesis_payload.toml");
src/boot.rs:74:    let manifest = parse_trust_root_section(&genesis_text)?;
src/boot.rs:98:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
src/boot.rs:99:/// the trust_root_immutability conformance battery (Phase B7) reads the
src/boot.rs:103:/// Parses the `[trust_root]` section of `genesis_payload.toml` into ordered
src/boot.rs:106:pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
src/boot.rs:115:            in_section = header.trim() == "trust_root";
src/boot.rs:122:            TrustRootError::GenesisParse(format!("line {}: missing '=' in [trust_root]", lineno + 1))
src/boot.rs:133:        return Err(TrustRootError::SectionMissing("trust_root"));
src/boot.rs:188:            [trust_root]
src/boot.rs:193:        let entries = parse_trust_root_section(toml).unwrap();
src/boot.rs:205:        let toml = "[trust_root]\nfoo = \"deadbeef\"\n";
src/boot.rs:207:            parse_trust_root_section(toml),
src/boot.rs:216:            parse_trust_root_section(toml),
src/boot.rs:217:            Err(TrustRootError::SectionMissing("trust_root"))
src/boot.rs:222:    fn verify_trust_root_passes_on_intact_repo() {
src/boot.rs:223:        verify_trust_root(&repo_root()).expect("intact repo verifies");
src/boot.rs:227:    fn verify_trust_root_detects_tamper_in_tempdir() {
src/boot.rs:228:        // Build a fake repo: copy genesis_payload.toml + the smallest tracked
src/boot.rs:229:        // file (cases/MANIFEST.sha256 is short), tamper the file, expect
src/boot.rs:232:        let genesis_src = repo_root().join("genesis_payload.toml");
src/boot.rs:233:        let genesis_dst = tmp.join("genesis_payload.toml");
src/boot.rs:239:[trust_root]
src/boot.rs:245:        let err = verify_trust_root(&tmp).expect_err("tamper must be detected");
src/boot.rs:259:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
src/boot.rs:266:            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{hash}\"\n"
src/boot.rs:268:        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
src/boot.rs:270:        verify_trust_root(&tmp).expect("matching hash verifies");
handover/audits/run_gemini_b7_extra_prebatch_audit.py:21:**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration. PASS only if you would stake your independence on the batch result being valid.
handover/audits/run_gemini_b7_extra_prebatch_audit.py:35:- PREREG § 1.8 base 8: src/{kernel,wal,bus}.rs, experiments/.../lean4_oracle.rs, constitution.md, cases/MANIFEST.sha256, handover/preregistration/{PPUT_CCL_SPLITS,PREREG}.{json,md}
handover/audits/run_gemini_b7_extra_prebatch_audit.py:63:PREREG § 1.8 base + audit add + B6 + B7-extra = 16 files. `genesis_payload.toml` itself is NOT self-hashed (chicken-and-egg, documented as Section 6 of TRACE_MATRIX_v1).
handover/audits/run_gemini_b7_extra_prebatch_audit.py:67:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
handover/audits/run_gemini_b7_extra_prebatch_audit.py:69:- (Q2.d) `cases/MANIFEST.sha256` is hashed once; case files only get integrity-checked transitively. Is this depth sufficient?
handover/audits/run_gemini_b7_extra_prebatch_audit.py:70:- (Q2.e) `Cargo.lock` is NOT in Trust Root. A malicious actor could swap a dependency version. Should it be?
handover/audits/run_gemini_b7_extra_prebatch_audit.py:129:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
handover/audits/run_gemini_b7_extra_prebatch_audit.py:131:| `src/main.rs` panic | FC3-N1 boot + FC3-E14 error→re-init→boot | pre-init failure | ? |
handover/audits/run_gemini_b7_extra_prebatch_audit.py:157:main_rs = (ROOT / "src/main.rs").read_text()
handover/audits/run_gemini_b7_extra_prebatch_audit.py:158:genesis = (ROOT / "genesis_payload.toml").read_text()
handover/audits/run_gemini_b7_extra_prebatch_audit.py:189:    f"\n## src/main.rs (B7 panic site)\n\n```rust\n{main_rs}\n```\n" +
handover/audits/run_gemini_b7_extra_prebatch_audit.py:190:    f"\n## genesis_payload.toml\n\n```toml\n{genesis}\n```\n" +
handover/audits/run_gemini_b7_extra_prebatch_audit.py:227:          f"**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n"
handover/audits/run_codex_b7_extra_prebatch_audit.sh:19:**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration (= retread the full $3-5 + 8h + Trust Root churn). PASS only if you would stake your independence on the batch result being valid.
handover/audits/run_codex_b7_extra_prebatch_audit.sh:33:- PREREG § 1.8 base 8: src/{kernel,wal,bus}.rs, experiments/.../lean4_oracle.rs, constitution.md, cases/MANIFEST.sha256, handover/preregistration/{PPUT_CCL_SPLITS,PREREG}.{json,md}
handover/audits/run_codex_b7_extra_prebatch_audit.sh:62:PREREG § 1.8 base + audit add + B6 + B7-extra = 16 files. `genesis_payload.toml` itself is NOT self-hashed (chicken-and-egg, documented as Section 6 of TRACE_MATRIX_v1).
handover/audits/run_codex_b7_extra_prebatch_audit.sh:66:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
handover/audits/run_codex_b7_extra_prebatch_audit.sh:68:- (Q2.d) `cases/MANIFEST.sha256` is hashed once into Trust Root, but the case files themselves only get integrity-checked transitively (manifest hash → manifest content → case hashes). Is this depth sufficient, or should each case file have a direct manifest entry?
handover/audits/run_codex_b7_extra_prebatch_audit.sh:69:- (Q2.e) `Cargo.lock` is NOT in Trust Root. A malicious actor could swap a dependency version (e.g., a sha2 backdoor). Should it be?
handover/audits/run_codex_b7_extra_prebatch_audit.sh:128:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
handover/audits/run_codex_b7_extra_prebatch_audit.sh:130:| `src/main.rs` panic | FC3-N1 boot + FC3-E14 error→re-init→boot | pre-init failure | ? |
handover/audits/run_codex_b7_extra_prebatch_audit.sh:160:printf '\n```\n\n## src/main.rs (B7 panic site)\n\n```rust\n' >> "$TMP_PROMPT"
handover/audits/run_codex_b7_extra_prebatch_audit.sh:161:cat "${ROOT}/src/main.rs" >> "$TMP_PROMPT"
handover/audits/run_codex_b7_extra_prebatch_audit.sh:163:printf '\n```\n\n## genesis_payload.toml (B7 + B7-extra)\n\n```toml\n' >> "$TMP_PROMPT"
handover/audits/run_codex_b7_extra_prebatch_audit.sh:164:cat "${ROOT}/genesis_payload.toml" >> "$TMP_PROMPT"
handover/audits/run_codex_b7_extra_prebatch_audit.sh:199:  printf '**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n'
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:208:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:215:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:218:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:220:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:254:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:366:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:398:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:512:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1176:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1217:   - Boot freeze: `pput_accounting_0` block in `genesis_payload.toml`
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:29:-   **(Q2.b) `src/main.rs` omission**: **VETO**. This is a critical omission. `TRACE_MATRIX_v1 § 4` defends omitting `boot.rs`, but `main.rs` is the *caller* that enforces the check. If `main.rs:12` is commented out, the entire Trust Root verification is silently bypassed. An attacker with passive file system access (the stated threat model) could make this one-line change without recompiling (if source is deployed alongside binary) or as part of a malicious recompile. The call to `verify_trust_root` is the lynchpin of the entire security model; its call site must be immutable. `src/main.rs` must be added to the Trust Root manifest.
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:33:-   **(Q2.d) `cases/MANIFEST.sha256` depth**: **PASS**. Hashing a manifest of file hashes is a standard and sufficient integrity mechanism (cf. git tree objects).
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:35:-   **(Q2.e) `Cargo.lock` omission**: **VETO**. This is a severe supply-chain vulnerability. The `Cargo.lock` file freezes the exact versions of all transitive dependencies. Without it in the Trust Root, an attacker could modify it to inject a malicious version of a dependency (e.g., `serde`, `sha2`, `tokio`). The build would succeed, but the resulting binary would be compromised. This completely undermines the integrity of the execution environment. `Cargo.lock` must be added to the Trust Root.
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:90:The discovery of two critical omissions from the Trust Root manifest (`src/main.rs` and `Cargo.lock`) represents a fundamental failure of the system's integrity guarantee. These are not minor issues; they undermine the entire premise of the `boot::verify_trust_root` check. The batch cannot proceed until these are fixed, as the resulting `genesis_payload.toml` would be based on a compromised verification process.
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:98:1.  **VETO Fix**: Add `src/main.rs` to the `[trust_root]` manifest in `genesis_payload.toml`. Its integrity is as important as the verifier it calls. (`Q2.b`)
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:99:2.  **VETO Fix**: Add `Cargo.lock` to the `[trust_root]` manifest. This is non-negotiable for supply-chain security and reproducible builds. (`Q2.e`)
handover/audits/run_codex_pput_ccl_audit_round3.sh:44:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md:130:Both auditors recommend adding the new B2-B4 metering files to the `[trust_root]` section of `genesis_payload.toml` (PREREG § 1.8). Codex adds three more.
handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md:140:When B7 lands, update `PREREG § 1.8` Trust Root file list AND `genesis_payload.toml [trust_root]` SHA-256 entries to include all six. The PREREG already specifies fallback enforcement; this just expands the manifest.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:207:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:214:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:217:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:219:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:253:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:518:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:570:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:624:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:824:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:840:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:898:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1070:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1077:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1080:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1082:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1116:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1327:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1378:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1432:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1630:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1646:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1930:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2279:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2286:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2289:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2291:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2325:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2437:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2469:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2583:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3247:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3288:   - Boot freeze: `pput_accounting_0` block in `genesis_payload.toml`
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:26:**Mandate**: PRE-BATCH gate. The 576-run p_0 calibration is about to launch (~$3-5 API spend, ~8 wall-hours, output freezes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` AND becomes part of Trust Root). Any defect found BEFORE the batch is cheap; any defect found AFTER means recomputing the entire calibration (= retread the full $3-5 + 8h + Trust Root churn). PASS only if you would stake your independence on the batch result being valid.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:40:- PREREG § 1.8 base 8: src/{kernel,wal,bus}.rs, experiments/.../lean4_oracle.rs, constitution.md, cases/MANIFEST.sha256, handover/preregistration/{PPUT_CCL_SPLITS,PREREG}.{json,md}
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:69:PREREG § 1.8 base + audit add + B6 + B7-extra = 16 files. `genesis_payload.toml` itself is NOT self-hashed (chicken-and-egg, documented as Section 6 of TRACE_MATRIX_v1).
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:73:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:75:- (Q2.d) `cases/MANIFEST.sha256` is hashed once into Trust Root, but the case files themselves only get integrity-checked transitively (manifest hash → manifest content → case hashes). Is this depth sufficient, or should each case file have a direct manifest entry?
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:76:- (Q2.e) `Cargo.lock` is NOT in Trust Root. A malicious actor could swap a dependency version (e.g., a sha2 backdoor). Should it be?
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:135:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:137:| `src/main.rs` panic | FC3-N1 boot + FC3-E14 error→re-init→boot | pre-init failure | ? |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:186:// what we calibrate is exactly what is committed in genesis_payload.toml.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:284:// `[trust_root]` manifest in `genesis_payload.toml`. Any mismatch =>
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:285:// `TrustRootError::Tampered { .. }`. `src/main.rs` panics with
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:290:// see header comment in `genesis_payload.toml`.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:318:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:319:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:320:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:335:/// tracked file's SHA-256 against the `genesis_payload.toml [trust_root]`
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:340:/// `repo_root` is the directory containing `genesis_payload.toml` (typically
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:342:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:343:    let genesis_path = repo_root.join("genesis_payload.toml");
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:345:    let manifest = parse_trust_root_section(&genesis_text)?;
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:369:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:370:/// the trust_root_immutability conformance battery (Phase B7) reads the
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:374:/// Parses the `[trust_root]` section of `genesis_payload.toml` into ordered
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:377:pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:386:            in_section = header.trim() == "trust_root";
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:393:            TrustRootError::GenesisParse(format!("line {}: missing '=' in [trust_root]", lineno + 1))
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:404:        return Err(TrustRootError::SectionMissing("trust_root"));
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:459:            [trust_root]
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:464:        let entries = parse_trust_root_section(toml).unwrap();
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:476:        let toml = "[trust_root]\nfoo = \"deadbeef\"\n";
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:478:            parse_trust_root_section(toml),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:487:            parse_trust_root_section(toml),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:488:            Err(TrustRootError::SectionMissing("trust_root"))
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:493:    fn verify_trust_root_passes_on_intact_repo() {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:494:        verify_trust_root(&repo_root()).expect("intact repo verifies");
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:498:    fn verify_trust_root_detects_tamper_in_tempdir() {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:499:        // Build a fake repo: copy genesis_payload.toml + the smallest tracked
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:500:        // file (cases/MANIFEST.sha256 is short), tamper the file, expect
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:503:        let genesis_src = repo_root().join("genesis_payload.toml");
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:504:        let genesis_dst = tmp.join("genesis_payload.toml");
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:510:[trust_root]
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:516:        let err = verify_trust_root(&tmp).expect_err("tamper must be detected");
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:530:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:537:            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{hash}\"\n"
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:539:        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:541:        verify_trust_root(&tmp).expect("matching hash verifies");
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:559:## src/main.rs (B7 panic site)
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:575:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:583:## genesis_payload.toml (B7 + B7-extra)
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:595:#   [trust_root]        — SHA-256 of every load-bearing file. Boot
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:596:#                         (`turingosv4::boot::verify_trust_root`) recomputes
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:606:#     constitution.md, cases/MANIFEST.sha256 (proxy for cases/*.yaml glob),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:626:# Total: 16 files. genesis_payload.toml itself is conceptually frozen but
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:645:[trust_root]
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:659:"cases/MANIFEST.sha256" = "3759631afae97c631be9a4038479b8d3681af25885f9758c32ad88c22f17fc3f"
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:996:    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1131:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1141:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1142:| `boot::parse_trust_root_section` | `src/boot.rs:91` | FC3-N34 (helper) | Y (line 86-90) | ✅ |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1144:| `fn main` (Trust Root verify call site) | `src/main.rs:11` | FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`) | Y (line 3-10) | ✅ |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1174:| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1178:`genesis_payload.toml` itself is **not** self-hashed (chicken-and-egg). The semantic anchor is the `[pput_accounting_0]` section content, not its hash. Section 6 below records this limitation.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1189:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1197:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1311:Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1356:- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1466:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1533:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1540:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1543:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1545:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2029:    25	// what we calibrate is exactly what is committed in genesis_payload.toml.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2125:    13	// `[trust_root]` manifest in `genesis_payload.toml`. Any mismatch =>
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2126:    14	// `TrustRootError::Tampered { .. }`. `src/main.rs` panics with
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2131:    19	// see header comment in `genesis_payload.toml`.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2159:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2160:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2161:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2176:    64	/// tracked file's SHA-256 against the `genesis_payload.toml [trust_root]`
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2181:    69	/// `repo_root` is the directory containing `genesis_payload.toml` (typically
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2183:    71	pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2184:    72	    let genesis_path = repo_root.join("genesis_payload.toml");
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2186:    74	    let manifest = parse_trust_root_section(&genesis_text)?;
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2210:    98	/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2211:    99	/// the trust_root_immutability conformance battery (Phase B7) reads the
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2215:   103	/// Parses the `[trust_root]` section of `genesis_payload.toml` into ordered
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2218:   106	pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2227:   115	            in_section = header.trim() == "trust_root";
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2234:   122	            TrustRootError::GenesisParse(format!("line {}: missing '=' in [trust_root]", lineno + 1))
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2245:   133	        return Err(TrustRootError::SectionMissing("trust_root"));
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2295:/bin/bash -lc "nl -ba src/main.rs | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2310:    14	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2813:   124	    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2833:/bin/bash -lc "nl -ba genesis_payload.toml | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2844:    10	#   [trust_root]        — SHA-256 of every load-bearing file. Boot
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2845:    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2855:    21	#     constitution.md, cases/MANIFEST.sha256 (proxy for cases/*.yaml glob),
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2875:    41	# Total: 16 files. genesis_payload.toml itself is conceptually frozen but
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2894:    60	[trust_root]
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2908:    74	"cases/MANIFEST.sha256" = "3759631afae97c631be9a4038479b8d3681af25885f9758c32ad88c22f17fc3f"
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2928:455:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2937:761:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3078:   455	                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3119:    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3129:    27	| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3130:    28	| `boot::parse_trust_root_section` | `src/boot.rs:91` | FC3-N34 (helper) | Y (line 86-90) | ✅ |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3132:    30	| `fn main` (Trust Root verify call site) | `src/main.rs:11` | FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`) | Y (line 3-10) | ✅ |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3162:    60	| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3166:    64	`genesis_payload.toml` itself is **not** self-hashed (chicken-and-egg). The semantic anchor is the `[pput_accounting_0]` section content, not its hash. Section 6 below records this limitation.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3177:    75	- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3185:    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3297:    78	Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3340:    26	- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
handover/audits/run_gemini_pput_ccl_audit.py:38:5. Specifies a **Trust Root** of immutable files (kernel.rs, wal.rs, bus.rs, lean4_oracle.rs, constitution.md, cases/, genesis_payload.toml [pput_accounting_0], the splits JSON, the PREREG itself); ArchitectAI writes outside `user_space/Δ_*/` trigger syscall-layer EPERM.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:46:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:218:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:225:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:228:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:230:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:264:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:475:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:526:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:580:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:778:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:794:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1078:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1427:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1434:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1437:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1439:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1473:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1585:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1617:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1731:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2395:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2436:   - Boot freeze: `pput_accounting_0` block in `genesis_payload.toml`
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4216:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4223:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4226:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4228:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4262:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4425:`p_0` is calibrated as a Phase B6 deliverable (Phase B7 if delayed): measured baseline regression rate when ablating any single recent code change on adaptation set. Pre-committed `p_0` value goes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` before Phase E starts. Phase E cannot start without `p_0` frozen.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4459:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4513:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4683:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5357:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5706:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5713:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5716:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5718:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5752:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5864:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5896:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6010:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6674:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6715:   - Boot freeze: `pput_accounting_0` block in `genesis_payload.toml`
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:289:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3715:./Cargo.lock:921: "untrusted",
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3716:./Cargo.lock:963: "untrusted",
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3717:./Cargo.lock:1334:name = "untrusted"
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3763:./handover/audits/run_codex_pput_ccl_audit_round3.sh:44:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3773:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:215:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3775:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:218:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3777:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3778:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:254:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3779:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:398:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3780:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:512:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3790:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1176:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3857:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:214:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3859:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:217:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3861:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3862:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:253:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3867:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:624:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3868:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:824:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3870:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:840:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3871:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:898:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3878:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1077:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3880:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1080:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3882:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3883:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1116:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3888:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1432:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3889:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1630:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3891:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1646:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3892:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1930:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3902:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2286:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3904:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2289:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3906:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3907:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2325:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3908:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2469:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3909:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2583:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3919:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3247:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3932:./handover/audits/run_gemini_pput_ccl_audit.py:38:5. Specifies a **Trust Root** of immutable files (kernel.rs, wal.rs, bus.rs, lean4_oracle.rs, constitution.md, cases/, genesis_payload.toml [pput_accounting_0], the splits JSON, the PREREG itself); ArchitectAI writes outside `user_space/Δ_*/` trigger syscall-layer EPERM.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3938:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:46:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3945:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:225:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3947:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:228:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3949:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3950:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:264:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3955:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:580:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3956:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:778:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3958:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:794:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3959:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1078:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3969:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1434:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3971:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1437:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3973:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3974:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1473:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3975:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1617:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3976:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1731:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3986:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2395:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4004:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4223:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4006:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4226:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4008:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4009:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4262:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4011:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4513:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4012:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4683:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4023:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5357:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4033:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5713:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4035:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5716:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4037:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4038:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5752:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4039:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5896:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4040:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6010:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4050:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6674:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4068:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:223:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4070:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:226:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4072:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4073:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:262:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4075:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:513:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4076:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:683:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4087:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1357:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4097:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1713:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4099:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1716:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4101:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4102:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1752:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4103:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1896:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4104:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2010:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4114:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2674:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4155:./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:18:| 微内核 Rust 代码 | `src/bus.rs`、`src/kernel.rs`、`src/ledger.rs`、`src/wal.rs`、`src/prediction_market.rs`、`src/lib.rs`、`src/main.rs` |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4331:./handover/ai-direct/LATEST.md:32:- B7 Boot freeze: `pput_accounting_0` in genesis_payload.toml + Trust Root immutability tests
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4334:./handover/ai-direct/LATEST.md:44:- Trust Root 集成未实现 (genesis_payload.toml `[trust_root]` SHA-256 表未生成)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4345:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:188:- `tests/trust_root_immutability.rs` — Gate B conformance
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4346:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:224:- `cargo test --tests artifact_content_predicates artifact_lookup_evasion architect_sole_lt_reader auditor_sees_candidate_only mode_flag_binary_purity trust_root_immutability` PASS
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4348:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:253:**What**: per PREREG § 1.8, freeze the Trust Root with SHA-256 of each protected file in `genesis_payload.toml [trust_root]`. Boot computes hashes at startup, compares to genesis, aborts on mismatch.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4352:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:269:- (new) `genesis_payload.toml` (might already exist; check first) — add `[pput_accounting_0]` section + `[trust_root]` section
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4353:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4354:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:271:- (new) `tests/trust_root_immutability.rs` — simulate ArchitectAI write attempt to each Trust Root path → assert process panics or returns EPERM (per PREREG § 7 Gate B fallback enforcement)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4355:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:288:[trust_root]
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4358:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:300:- `cargo test test_trust_root_immutable_at_boot` PASS
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4359:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:301:- `cargo test test_trust_root_simulated_write_aborts` PASS
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4361:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:332:- Add the calibration jsonl path to `[trust_root]`
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4365:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:373:- [ ] `pput_accounting_0` block in `genesis_payload.toml` filled (including p_0 + jsonl hash + Trust Root hashes)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4373:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:151:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4375:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:154:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4377:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4378:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:190:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4383:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:561:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4384:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:761:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4386:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:777:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4387:./handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json:275:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4389:./handover/preregistration/scripts/split_pput_ccl.py:133:        "trust_root_note": (
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5487:   188	- `tests/trust_root_immutability.rs` — Gate B conformance
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5497:   263	genesis_payload.toml [pput_accounting_0] section
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5503:   269	- (new) `genesis_payload.toml` (might already exist; check first) — add `[pput_accounting_0]` section + `[trust_root]` section
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5504:   270	- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5505:   271	- (new) `tests/trust_root_immutability.rs` — simulate ArchitectAI write attempt to each Trust Root path → assert process panics or returns EPERM (per PREREG § 7 Gate B fallback enforcement)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5522:   288	[trust_root]
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5530:   296	# cases/* tracked via cases/MANIFEST.sha256 (glob)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5534:   300	- `cargo test test_trust_root_immutable_at_boot` PASS
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5535:   301	- `cargo test test_trust_root_simulated_write_aborts` PASS
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5544:   310	**What**: per PREREG § 5.5, run the calibration protocol to compute `p_0` (baseline regression rate); freeze into `genesis_payload.toml [pput_accounting_0]`.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5564:   330	- Write p_0 value to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5566:   332	- Add the calibration jsonl path to `[trust_root]`
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5595:   361	- Day 5: B6 context-leak audit + B7 part 1 (genesis_payload.toml structure)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5607:   373	- [ ] `pput_accounting_0` block in `genesis_payload.toml` filled (including p_0 + jsonl hash + Trust Root hashes)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5672:   144	  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5679:   151	1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5682:   154	4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5684:   156	Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5696:   168	This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5735:   507	- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5789:   561	| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5983:handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:507:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:216:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:223:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:226:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:228:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:262:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:425:`p_0` is calibrated as a Phase B6 deliverable (Phase B7 if delayed): measured baseline regression rate when ablating any single recent code change on adaptation set. Pre-committed `p_0` value goes into `genesis_payload.toml [pput_accounting_0].baseline_regression_rate` before Phase E starts. Phase E cannot start without `p_0` frozen.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:459:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:513:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:683:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1357:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1706:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1713:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1716:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1718:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1752:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1864:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1896:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2010:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2674:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2715:   - Boot freeze: `pput_accounting_0` block in `genesis_payload.toml`
handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md:26:- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:78:Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.
handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:18:| 微内核 Rust 代码 | `src/bus.rs`、`src/kernel.rs`、`src/ledger.rs`、`src/wal.rs`、`src/prediction_market.rs`、`src/lib.rs`、`src/main.rs` |
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:33:   - Boot freeze: `pput_accounting_0` block in `genesis_payload.toml`
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:69:diff --git a/Cargo.lock b/Cargo.lock
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:71:--- a/Cargo.lock
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:72:+++ b/Cargo.lock
handover/ai-direct/LATEST.md:36:  - 24 Phase C/D/B7 stubs scaffolded (`#[ignore]` with contract docs): artifact_content_predicates (4) + artifact_lookup_evasion (4) + architect_sole_lt_reader (3) + auditor_sees_candidate_only (3) + mode_flag_binary_purity (6) + trust_root_immutability (4)
handover/ai-direct/LATEST.md:39:  - `genesis_payload.toml` (new): `[pput_accounting_0]` (PREREG § 1.8 invariants — schema_version, progress/cost/time defs, verified_predicate, heldout_sealed_hash, source_pool_sha256, k_max=10, n_max=34, baseline_regression_rate placeholder); `[trust_root]` (15 SHA-256 entries — independently re-derived: PREREG § 1.8 base 8 + audit accounting 6 + B6 prompt_guard 1)
handover/ai-direct/LATEST.md:40:  - `cases/MANIFEST.sha256` (new): 45-entry sorted SHA-256 manifest of `cases/C-*.yaml`, hashed-once into Trust Root as proxy for the case-law glob
handover/ai-direct/LATEST.md:41:  - `src/boot.rs` (new, +pub in lib.rs): `verify_trust_root(repo_root)` parses [trust_root] section (hand-rolled minimal TOML parser, no new dep — compression principle), recomputes SHA-256 per path, returns `TrustRootError::Tampered{path,expected,actual}` on mismatch; 6 unit tests (parse/blank/comment/missing-section/intact-repo/tempdir-tamper)
handover/ai-direct/LATEST.md:42:  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
handover/ai-direct/LATEST.md:43:  - `experiments/minif2f_v4/tests/trust_root_immutability.rs`: 4 `#[ignore]` stubs unsealed → 4 PASS (immutable_at_boot / simulated_write_aborts / manifest_includes_b2_b4_files / pput_accounting_0_section_present); manifest test enforces the union list (PREREG § 1.8 base + audit add + B6) — any reduction breaks the test
handover/ai-direct/LATEST.md:51:  - **Next**: user GO → 576-run batch (~$3-5, ~8h overnight) → compute_p0.py → write p_0 to genesis_payload.toml [pput_accounting_0] → recompute Trust Root + commit jsonl into manifest → Gate B dual-audit Phase B → Phase C transition
handover/ai-direct/LATEST.md:52:- **B7-alignment ✅ DONE** (commit `0cc48bc`) — TRACE_MATRIX v1 (FC3-N34 promoted ✅, B7-extra rows added), src/boot.rs + src/main.rs FC backlinks, OBS_BOOT_FAIL_NOT_HALT (boot panic ≠ FC2-N22, closer to FC3-E14)
handover/ai-direct/LATEST.md:61:- B7-extra (p_0 calibration) 未跑：576 runs (288 control + 288 treatment) on adaptation-144 × seeds [31415, 2718], `--simulate-rollback-at-tx-50` toggle 待加；p_0 ∈ (0, 0.10] sanity gate；冻结进 `[pput_accounting_0].baseline_regression_rate` + `.baseline_regression_jsonl_sha256` + 把 jsonl 加入 [trust_root]
handover/ai-direct/LATEST.md:62:- 20 Phase C/D conformance stubs `#[ignore]` 待对应 phase 解封 (artifact_content 4 / lookup_evasion 4 / architect_sole_lt 3 / auditor_sees_candidate 3 / mode_flag_binary_purity 6) — B7 解封了 trust_root_immutability 4 个
handover/ai-direct/LATEST.md:64:- Trust Root 自身不自哈希 (chicken-and-egg)：`genesis_payload.toml` 自身 tamper 不会被 Boot 检测；语义锚点 = `[pput_accounting_0]` 字段值；如要更强保证，未来可在编译时把 [trust_root] 哈希常量 inline 进 binary（Phase C+ 议题，非 Gate B 阻塞）
handover/ai-direct/LATEST.md:90:5. Freeze: write p_0 to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`; SHA-256 the calibration jsonl → `.baseline_regression_jsonl_sha256`; add jsonl path to `[trust_root]`; recompute every Trust Root hash (genesis itself changed); commit
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:12:Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:20:| Triggered by | constitutional condition (∏p=0 cumulative, max-tx exhausted, OmegaAccepted, wall-clock cap, compute-cap, error-halt) | manifest hash mismatch in genesis_payload.toml |
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:52:If/when Phase 11+ lands an in-process re-init mechanism (TRACE_MATRIX FC3-N41), it can use `Result<(), TrustRootError>` from `verify_trust_root` instead of letting the panic propagate. At that point the panic becomes the *uncaught* path and structured retry becomes the *caught* path. Constitution still does not need to change — both paths land at FC3-E14.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:56:- `src/main.rs:11-14` — call site, TRACE_MATRIX backlink to FC3-E14
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:57:- `src/boot.rs:62` — `verify_trust_root` itself, TRACE_MATRIX backlink to FC3-N34
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:17:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:27:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:28:| `boot::parse_trust_root_section` | `src/boot.rs:91` | FC3-N34 (helper) | Y (line 86-90) | ✅ |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:30:| `fn main` (Trust Root verify call site) | `src/main.rs:11` | FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`) | Y (line 3-10) | ✅ |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:60:| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:64:`genesis_payload.toml` itself is **not** self-hashed (chicken-and-egg). The semantic anchor is the `[pput_accounting_0]` section content, not its hash. Section 6 below records this limitation.
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:75:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:83:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:144:  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:151:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:154:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:156:Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:190:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:455:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:507:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:561:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:761:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:777:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json:275:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
handover/preregistration/scripts/compute_p0.py:124:    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:188:- `tests/trust_root_immutability.rs` — Gate B conformance
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:224:- `cargo test --tests artifact_content_predicates artifact_lookup_evasion architect_sole_lt_reader auditor_sees_candidate_only mode_flag_binary_purity trust_root_immutability` PASS
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:253:**What**: per PREREG § 1.8, freeze the Trust Root with SHA-256 of each protected file in `genesis_payload.toml [trust_root]`. Boot computes hashes at startup, compares to genesis, aborts on mismatch.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:263:genesis_payload.toml [pput_accounting_0] section
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:269:- (new) `genesis_payload.toml` (might already exist; check first) — add `[pput_accounting_0]` section + `[trust_root]` section
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:271:- (new) `tests/trust_root_immutability.rs` — simulate ArchitectAI write attempt to each Trust Root path → assert process panics or returns EPERM (per PREREG § 7 Gate B fallback enforcement)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:288:[trust_root]
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:296:# cases/* tracked via cases/MANIFEST.sha256 (glob)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:300:- `cargo test test_trust_root_immutable_at_boot` PASS
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:301:- `cargo test test_trust_root_simulated_write_aborts` PASS
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:310:**What**: per PREREG § 5.5, run the calibration protocol to compute `p_0` (baseline regression rate); freeze into `genesis_payload.toml [pput_accounting_0]`.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:330:- Write p_0 value to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:332:- Add the calibration jsonl path to `[trust_root]`
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:361:- Day 5: B6 context-leak audit + B7 part 1 (genesis_payload.toml structure)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:373:- [ ] `pput_accounting_0` block in `genesis_payload.toml` filled (including p_0 + jsonl hash + Trust Root hashes)
handover/preregistration/scripts/split_pput_ccl.py:25:  - source pool fingerprint matches expected (printed; compare to genesis_payload)
handover/preregistration/scripts/split_pput_ccl.py:133:        "trust_root_note": (
handover/preregistration/scripts/split_pput_ccl.py:148:    print("          genesis_payload.toml [pput_accounting_0] section (Phase B7).")
experiments/minif2f_v4/src/rollback_sim.rs:25:// what we calibrate is exactly what is committed in genesis_payload.toml.

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/tests/trust_root_immutability.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL Phase B7 — Trust Root immutability (PREREG § 1.8 + § 7 Gate B).
     2	//
     3	// Boot computes SHA-256 of every Trust Root file at process start and
     4	// compares against the genesis_payload.toml [trust_root] manifest. Any
     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
     6	//
     7	// Trust Root manifest (PREREG § 1.8 + B2-B4 mid-term audit recommendation
     8	// + B6 prompt_guard add + B7-extra rollback_sim add):
     9	//   src/kernel.rs
    10	//   src/wal.rs
    11	//   src/bus.rs
    12	//   src/drivers/llm_http.rs                           (B2-B4 audit add)
    13	//   src/sdk/prompt_guard.rs                           (B6 add)
    14	//   experiments/minif2f_v4/src/lean4_oracle.rs
    15	//   experiments/minif2f_v4/src/cost_aggregator.rs     (B2)
    16	//   experiments/minif2f_v4/src/wall_clock.rs          (B3)
    17	//   experiments/minif2f_v4/src/post_hoc_verifier.rs   (B4)
    18	//   experiments/minif2f_v4/src/jsonl_schema.rs        (B1)
    19	//   experiments/minif2f_v4/src/rollback_sim.rs        (B7-extra)
    20	//   experiments/minif2f_v4/src/bin/evaluator.rs       (the wiring)
    21	//   constitution.md
    22	//   handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
    23	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
    24	//   cases/MANIFEST.sha256                             (proxy for cases/*.yaml)
    25	
    26	use std::fs;
    27	use std::path::{Path, PathBuf};
    28	use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
    29	
    30	fn repo_root() -> PathBuf {
    31	    // CARGO_MANIFEST_DIR for this test crate is experiments/minif2f_v4 — repo
    32	    // root is two levels up.
    33	    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    34	        .join("..")
    35	        .join("..")
    36	        .canonicalize()
    37	        .expect("repo root resolves")
    38	}
    39	
    40	fn read_genesis() -> String {
    41	    fs::read_to_string(repo_root().join("genesis_payload.toml")).expect("genesis exists")
    42	}
    43	
    44	#[test]
    45	fn test_trust_root_immutable_at_boot() {
    46	    // Cold-start with intact files: Boot computes SHA-256s, all match
    47	    // genesis manifest, process continues. No abort.
    48	    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
    49	}
    50	
    51	#[test]
    52	fn test_trust_root_simulated_write_aborts() {
    53	    // Simulated tampering: build a self-contained fake-repo in a tempdir
    54	    // with a single Trust Root entry whose recorded hash does not match
    55	    // the file content; assert verify_trust_root returns Tampered.
    56	    let tmp = make_tempdir("trust_root_tamper");
    57	    let zero_hash = "0".repeat(64);
    58	    let genesis = format!(
    59	        "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{zero_hash}\"\n"
    60	    );
    61	    fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
    62	    fs::write(tmp.join("only.txt"), "tampered content").unwrap();
    63	
    64	    match verify_trust_root(&tmp) {
    65	        Err(TrustRootError::Tampered { path, expected, actual }) => {
    66	            assert!(path.ends_with("only.txt"));
    67	            assert_eq!(expected, zero_hash);
    68	            assert_ne!(actual, expected);
    69	        }
    70	        other => panic!("expected Tampered, got {other:?}"),
    71	    }
    72	}
    73	
    74	#[test]
    75	fn test_trust_root_manifest_includes_b2_b4_files() {
    76	    // Mid-term audit recommendation: B2 (cost_aggregator), B3 (wall_clock),
    77	    // B4 (post_hoc_verifier), B1 (jsonl_schema), evaluator.rs, llm_http.rs
    78	    // MUST be in the Trust Root manifest. B6 added prompt_guard.rs.
    79	    let entries = parse_trust_root_section(&read_genesis()).expect("parse trust_root");
    80	    let keys: Vec<&str> = entries.iter().map(|(k, _)| k.as_str()).collect();
    81	
    82	    let required = [
    83	        // PREREG § 1.8 base
    84	        "src/kernel.rs",
    85	        "src/wal.rs",
    86	        "src/bus.rs",
    87	        "experiments/minif2f_v4/src/lean4_oracle.rs",
    88	        "constitution.md",
    89	        "cases/MANIFEST.sha256",
    90	        "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json",
    91	        "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
    92	        // Mid-term audit accounting layer
    93	        "src/drivers/llm_http.rs",
    94	        "experiments/minif2f_v4/src/cost_aggregator.rs",
    95	        "experiments/minif2f_v4/src/wall_clock.rs",
    96	        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
    97	        "experiments/minif2f_v4/src/jsonl_schema.rs",
    98	        "experiments/minif2f_v4/src/bin/evaluator.rs",
    99	        // B6 add
   100	        "src/sdk/prompt_guard.rs",
   101	        // B7-extra add
   102	        "experiments/minif2f_v4/src/rollback_sim.rs",
   103	    ];
   104	
   105	    for path in required {
   106	        assert!(
   107	            keys.contains(&path),
   108	            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
   109	        );
   110	    }
   111	}
   112	
   113	#[test]
   114	fn test_pput_accounting_0_section_present() {
   115	    // genesis_payload.toml must contain [pput_accounting_0] with the PREREG
   116	    // § 1.8 keys.
   117	    let genesis = read_genesis();
   118	    let body = extract_section(&genesis, "pput_accounting_0").expect("section present");
   119	    let body = body.as_str();
   120	
   121	    let required_keys = [
   122	        "schema_version",
   123	        "progress_definition",
   124	        "cost_definition",
   125	        "time_definition",
   126	        "verified_predicate",
   127	        "heldout_sealed_hash",
   128	        "source_pool_sha256",
   129	        "baseline_regression_rate",
   130	        "baseline_regression_jsonl_sha256",
   131	        "k_max",
   132	        "n_max",
   133	    ];
   134	    for key in required_keys {
   135	        let needle = format!("{key} =");
   136	        assert!(
   137	            body.contains(&needle),
   138	            "[pput_accounting_0] missing key: {key}"
   139	        );
   140	    }
   141	
   142	    // Frozen invariants from PREREG § 1.8: heldout sealed hash, k_max, n_max.
   143	    assert!(body.contains(
   144	        "\"51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b\""
   145	    ), "heldout_sealed_hash diverges from PREREG § 2.3");
   146	    assert!(body.contains("k_max = 10"), "k_max must be 10 per PREREG");
   147	    assert!(body.contains("n_max = 34"), "n_max must be 34 per PREREG");
   148	}
   149	
   150	// --- helpers ---
   151	
   152	fn extract_section(text: &str, name: &str) -> Option<String> {
   153	    // Line-anchored scan: skip commented-out section headers (e.g. inside
   154	    // the file's leading docstring) and only match real headers in column 0.
   155	    let mut in_section = false;
   156	    let mut body = String::new();
   157	    let target = format!("[{name}]");
   158	    for line in text.lines() {
   159	        let trimmed = line.trim_start();
   160	        if trimmed.starts_with('#') {
   161	            continue;
   162	        }
   163	        if trimmed.starts_with('[') && trimmed.ends_with(']') {
   164	            in_section = trimmed == target;
   165	            continue;
   166	        }
   167	        if in_section {
   168	            body.push_str(line);
   169	            body.push('\n');
   170	        }
   171	    }
   172	    if body.is_empty() {
   173	        None
   174	    } else {
   175	        Some(body)
   176	    }
   177	}
   178	
   179	fn make_tempdir(tag: &str) -> PathBuf {
   180	    let pid = std::process::id();

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '70,115p;210,245p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    70	    /// Phase D+ meta-loop attribution; nullable in Phase B.
    71	    #[serde(skip_serializing_if = "Option::is_none")]
    72	    pub architect_artifact_id: Option<String>,
    73	    #[serde(skip_serializing_if = "Option::is_none")]
    74	    pub auditor_attestation: Option<String>,
    75	}
    76	
    77	/// Per-run aggregate row.
    78	///
    79	/// `pput_runtime` = legacy / runtime-accept-based — NEVER the North Star.
    80	/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
    81	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    82	pub struct RunAggregate {
    83	    pub run_id: String,
    84	    pub problem_id: String,
    85	    pub solved: bool,
    86	
    87	    pub schema_version: String,
    88	    pub split: String,
    89	    /// Lean post-hoc PASS (B4).
    90	    pub verified: bool,
    91	    pub golden_path_token_count: u64,
    92	    /// C_i — sum over all proposals (B2).
    93	    pub total_run_token_count: u64,
    94	    /// T_i — wall-clock first-read → final-accept (B3).
    95	    pub total_wall_time_ms: u64,
    96	    /// 0 or 1 (Lean ground truth).
    97	    pub progress: u8,
    98	    /// Runtime/accept-based; may inflate under Soft Law (H1 detection).
    99	    pub pput_runtime: f64,
   100	    /// Verified PPUT — Progress / (C_i × T_i / 1000), units = 1/(token·second).
   101	    pub pput_verified: f64,
   102	    /// 10^6 × pput_verified — display unit (PREREG § 5).
   103	    pub pput_m_verified: f64,
   104	    pub failed_branch_count: u32,
   105	    pub rollback_count: u32,
   106	
   107	    pub far: f64,
   108	    pub err: f64,
   109	    pub iac: f64,
   110	    pub cpr: f64,
   111	
   112	    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
   113	    pub model_snapshot: String,
   114	    pub git_sha: String,
   115	    pub binary_sha256: String,
   210	            git_sha: "913255d".into(),
   211	            binary_sha256: "deadbeef".into(),
   212	            mode: "full".into(),
   213	        }
   214	    }
   215	
   216	    #[test]
   217	    fn test_jsonl_schema_v2_round_trip() {
   218	        let original = sample_run();
   219	        let line = serde_json::to_string(&original).expect("serialize");
   220	        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
   221	        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
   222	        assert!(line.contains("\"schema_version\":\"v2.0\""),
   223	                "serialized line must stamp schema_version");
   224	    }
   225	
   226	    #[test]
   227	    fn test_pput_verified_zero_when_progress_zero() {
   228	        // PREREG § 3 anti-Goodhart: a run that did not verify must report
   229	        // pput_verified = 0 regardless of cost / wall-time.
   230	        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
   231	        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
   232	
   233	        // And the struct round-trips with the zero stamped in.
   234	        let mut r = sample_run();
   235	        r.solved = false;
   236	        r.verified = false;
   237	        r.progress = 0;
   238	        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
   239	        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
   240	        assert_eq!(r.pput_verified, 0.0);
   241	        assert_eq!(r.pput_m_verified, 0.0);
   242	
   243	        // Defensive: degenerate cost/time also clamps to 0 (synthetic test fixtures).
   244	        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
   245	        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/lean4_oracle.rs | sed -n '90,190p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    90	
    91	    /// OMEGA verification — the ultimate boolean predicate.
    92	    /// Feeds problem_statement + proof_chain to Lean 4 and checks:
    93	    /// - "No goals to be solved" → OMEGA (true)
    94	    /// - Any error → not OMEGA (false)
    95	    ///
    96	    /// V3L-02: deterministic — no random elements in verification.
    97	    /// Rule 22 v2 clause 4: proof_chain passed verbatim, no byte modification.
    98	    pub fn verify_omega(&self, proof_chain: &str) -> Result<bool, String> {
    99	        self.verify_omega_detailed(proof_chain).map(|(ok, _)| ok)
   100	    }
   101	
   102	    /// Step-B v3: return (success, error_output) so callers can classify.
   103	    /// Empty error string on success; raw combined stderr/stdout (truncated) on reject.
   104	    /// Callers MUST pass through classify_lean_error before broadcast (C-022).
   105	    ///
   106	    /// F-2026-04-20-05 fix: run check_payload here too. Previously this was only
   107	    /// enforced via on_pre_append when the payload traversed bus.append. The
   108	    /// `complete` action calls this function directly and historically bypassed
   109	    /// the forbidden-pattern check, allowing `native_decide` brute-force bytecode
   110	    /// bypass to register as OMEGA. Enforcing here closes the hole without
   111	    /// breaking bus-path callers (idempotent double-check).
   112	    pub fn verify_omega_detailed(&self, proof_chain: &str) -> Result<(bool, String), String> {
   113	        if let Err(reason) = self.check_payload(proof_chain) {
   114	            log::warn!("[oracle] payload rejected pre-Lean: {}", reason);
   115	            return Ok((false, format!("forbidden_payload: {}", reason)));
   116	        }
   117	        let full_code = format!("{}\n{}", self.problem_statement, proof_chain);
   118	        if has_word_boundary(&full_code, "sorry") || has_word_boundary(&full_code, "sorryAx") {
   119	            return Ok((false, "sorry_in_proof".into()));
   120	        }
   121	
   122	        // Compute gas limit based on code size.
   123	        // Mathlib import alone takes ~50s on this VM, so base must be high.
   124	        let lines = full_code.lines().count();
   125	        let timeout_secs = 120 + (lines as u64);
   126	        let timeout = Duration::from_secs(timeout_secs.min(300));
   127	
   128	        // Execute in sandbox
   129	        let sandbox = LocalProcessSandbox::new(
   130	            &self.lean_binary,
   131	            &["--stdin"],
   132	        );
   133	
   134	        // Set LEAN_PATH environment for Mathlib resolution
   135	        std::env::set_var("LEAN_PATH", &self.lean_path);
   136	
   137	        match sandbox.execute(&full_code, timeout) {
   138	            Ok(SandboxResult::Completed { stdout, stderr, exit_code }) => {
   139	                let combined = format!("{}\n{}", stdout, stderr);
   140	                if combined.contains("declaration uses 'sorry'") {
   141	                    log::warn!("oracle reject reason: declaration uses 'sorry'");
   142	                    return Ok((false, "declaration_uses_sorry".into()));
   143	                }
   144	                if combined.contains("No goals to be solved") {
   145	                    return Ok((true, String::new()));
   146	                }
   147	                if exit_code == 0 && !combined.contains("error:") {
   148	                    return Ok((true, String::new()));
   149	                }
   150	                let err_preview: String = combined.lines()
   151	                    .filter(|l| l.contains("error") || l.contains("unexpected") || l.contains("expected"))
   152	                    .take(4)
   153	                    .collect::<Vec<_>>()
   154	                    .join(" | ");
   155	                let detail = if err_preview.is_empty() { combined.chars().take(800).collect::<String>() } else { err_preview };
   156	                log::warn!("oracle reject reason (exit={}): {}", exit_code, detail);
   157	                Ok((false, detail))
   158	            }
   159	            Ok(SandboxResult::Timeout) => Err("Lean 4 verification timed out".into()),
   160	            Err(e) => Err(format!("Sandbox error: {}", e)),
   161	        }
   162	    }
   163	
   164	    /// Phase 7 (Turing δ-step): three-way classification for partial proofs.
   165	    /// Elaborates `problem_statement + proof_prefix` (the accumulated tactic
   166	    /// chain) and returns:
   167	    ///   Complete   — Lean accepts, no remaining goals → OMEGA, halt the run
   168	    ///   PartialOk  — tactics elaborate without type errors but goals remain →
   169	    ///                the prefix is a valid computation path, write Q_{t+1} and
   170	    ///                continue
   171	    ///   Reject(r)  — any true elaboration error → prefix is invalid, do NOT
   172	    ///                write; agent must try a different step
   173	    ///
   174	    /// This is the constitutional δ that Art. IV mermaid demands: a single
   175	    /// transition rule that updates Q_t one step at a time, with ∏p as the
   176	    /// pass/fail on that single step — not on the whole computation.
   177	    pub fn verify_partial(&self, proof_prefix: &str) -> PartialVerdict {
   178	        if let Err(reason) = self.check_payload(proof_prefix) {
   179	            log::warn!("[oracle/partial] rejected pre-Lean: {}", reason);
   180	            return PartialVerdict::Reject(format!("forbidden_payload: {}", reason));
   181	        }
   182	        let full_code = format!("{}\n{}", self.problem_statement, proof_prefix);
   183	        if has_word_boundary(&full_code, "sorry") || has_word_boundary(&full_code, "sorryAx") {
   184	            return PartialVerdict::Reject("sorry_in_proof".into());
   185	        }
   186	
   187	        let lines = full_code.lines().count();
   188	        let timeout_secs = 120 + (lines as u64);
   189	        let timeout = Duration::from_secs(timeout_secs.min(300));
   190	

exec
/bin/bash -lc "nl -ba handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md | sed -n '1,150p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Proxy Saturation Finding — 2026-04-24 early
     2	
     3	**Trigger**: user observation — "如果 proxy timeout 的原因是 deepseek 服务商不能提供高并发导致的, 影响了实验结果, 就应该反思, 不然结果没意义"
     4	
     5	**Context**: E1 v2 data regeneration running 12 batches × 8 agents = 96 concurrent LLM requests at full saturation.
     6	
     7	---
     8	
     9	## § 1. Observed contamination
    10	
    11	First 30 outcomes across 12 parallel batches:
    12	
    13	| Outcome | Count | % |
    14	|---|---|---|
    15	| MEASUREMENT_ERROR (outer 900s timeout hit) | 22 | **73%** |
    16	| SOLVED | 5 | 17% |
    17	| FAIL (proper MaxTxExhausted) | 0 | 0% |
    18	| pending | 3 | 10% |
    19	
    20	Compare to identical evaluator binary + identical sample run at ≤2 concurrent batches:
    21	
    22	| Configuration | MEASUREMENT_ERROR rate |
    23	|---|---|
    24	| 12-way parallel (this run) | 73% |
    25	| 2-way parallel (Paper 1 v1 runs) | 0-3% |
    26	| 1-way serial (Phase 9.A initial) | 0% |
    27	
    28	---
    29	
    30	## § 2. Diagnosis
    31	
    32	**Root cause**: DeepSeek chat API rate-limits or concurrency-caps at ~16-20 concurrent requests per API key. At 12 batches × 8 agents = 96 concurrent, individual request latency balloons from ~5-10s to ~15-20s+. With MAX_TRANSACTIONS=50 × 15-20s/tx = 750-1000s, the outer 900s cap fires before internal MaxTxExhausted halt.
    33	
    34	**Evidence**:
    35	- Per-problem timing from Paper 1 v1 (2-way parallel): chat responses ~8-12s
    36	- Per-problem timing from E1 v2 (12-way parallel): ~15-20s per response on hard problems
    37	- MEASUREMENT_ERROR clusters on HARDEST problems (amc12_2000_p12, amc12_2000_p6) where MAX_TRANSACTIONS=50 is actually hit, not early solves
    38	
    39	**Artifact class**: when proxy saturates, MEASUREMENT_ERROR is emitted instead of FAIL. This is silently equivalent to "this condition failed on this problem" in aggregation scripts, but in reality the evaluator never had a chance to either solve or halt cleanly.
    40	
    41	---
    42	
    43	## § 3. Implication for prior data
    44	
    45	### 3.1 Paper 1 v1 data (all E1 × 3 seeds + easy-set + ablation): **NOT affected**
    46	
    47	All v1 batches ran 2-way parallel (A + B simultaneously per seed). MEASUREMENT_ERROR count across v1:
    48	- `E1_A_homogeneous`: 0
    49	- `E1_B_heterogeneous`: 0
    50	- `E1_A_seed31415`: 0
    51	- `E1_B_seed31415`: 0
    52	- `E1_A_seed2718`: 0 (1 MEASUREMENT_ERROR seen was on problem 11 / 20, proxy load lower then)
    53	- `E1_B_seed2718`: 0
    54	- `E1_A_seed2357`: 0
    55	- `E1_B_seed2357`: 0
    56	- `E1_A_easy_ctrl`, `E1_B_easy_ctrl`: 0
    57	- `E1_ablation_no_meta_seed141421`: 0
    58	
    59	**v1 data integrity confirmed**. Paper 1 v1 CHALLENGE verdict stands on its methodological merits (P0-1 sample bias, P0-2 McNemar labeling, P0-3 overclaim, P0-4 mechanism N=1, P0-5 ablation scope). Proxy saturation is NOT a new blocker for v1.
    60	
    61	### 3.2 Phase 9.A baseline data: 1 MEASUREMENT_ERROR on seed 2718 (negligible, <2%)
    62	
    63	Phase 9.A ran sequentially with cap=200. Minor contamination. Does not invalidate seed 2718 aggregate.
    64	
    65	### 3.3 E1 v2 in-progress data: DISCARD
    66	
    67	12-way parallel batches produced 73% MEASUREMENT_ERROR. Unusable. Will re-run with reduced concurrency.
    68	
    69	---
    70	
    71	## § 4. New harness constraint: MAX_PARALLEL_BATCHES
    72	
    73	Add to PREREG template:
    74	
    75	```yaml
    76	concurrency_policy:
    77	  max_parallel_evaluator_processes: 2
    78	  rationale: |
    79	    DeepSeek chat API saturates above ~16 concurrent requests; 2 batches ×
    80	    8 agents = 16 is the empirically-safe ceiling. Higher parallelism
    81	    produces MEASUREMENT_ERROR artifacts that contaminate FAIL statistics.
    82	  enforcement: tools/concurrency_gate.sh (to be implemented) — wraps
    83	    evaluator launch in a semaphore.
    84	```
    85	
    86	---
    87	
    88	## § 5. Updated PREREG_E1V2_HETEROGENEITY
    89	
    90	The PREREG committed earlier (`handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md`) did NOT include a concurrency policy. This is now identified as a missing mandatory field.
    91	
    92	Per C-070, PREREG is frozen at commit time. We have two options:
    93	1. **Amend with explicit "deviation flagged"** — re-run E1 v2 with max_parallel=2, note in paper that "pre-reg did not specify concurrency; the proxy saturation discovered during execution required re-running at max_parallel=2, with no changes to sample/seeds/conditions"
    94	2. **New PREREG (v3)** — supersede with fully-specified version
    95	
    96	Option 1 is honest and documented — going with that.
    97	
    98	---
    99	
   100	## § 6. Re-run plan (immediate)
   101	
   102	- **Discard**: all 12 in-progress jsonl files from E1 v2 first attempt
   103	- **Re-launch**: 4 seeds × 3 conditions = 12 batches, but **serialized in 6 rounds of 2 parallel batches**
   104	- **Estimated time**: 6 rounds × ~1h = 6h wallclock
   105	- **Budget**: ≈$15-20 LLM
   106	
   107	Re-launch pairing strategy (A and B of same seed paired to preserve paired-design structure):
   108	- Round 1: A_s141421 + B_s141421
   109	- Round 2: Abl_s141421 + A_s31415
   110	- Round 3: B_s31415 + Abl_s31415
   111	- Round 4: A_s2718 + B_s2718
   112	- Round 5: Abl_s2718 + A_s2357
   113	- Round 6: B_s2357 + Abl_s2357
   114	
   115	Each round waits for previous to complete before starting.
   116	
   117	---
   118	
   119	## § 7. Methodology lesson for C-070 amendment
   120	
   121	C-070 § ruling currently specifies pre-reg + multiplicity + N≥3. Missing:
   122	- **Concurrency / load policy**: resource constraints must be pre-registered
   123	- **Null-measurement class**: MEASUREMENT_ERROR must be classified (not silently aggregated as FAIL)
   124	
   125	Both will be added to C-070's precedent section in a follow-up commit.
   126	
   127	---
   128	
   129	## § 8. Credit
   130	
   131	User caught this BEFORE I did. The proxy saturation was producing MEASUREMENT_ERROR at 73%, and my earlier reports said "this is a hard problem, common-mode on both sides, doesn't bias paired Δ". That was wrong — the contamination is not benign; it destroys all FAIL statistics.
   132	
   133	Paper 1 v2 methodology will explicitly include `max_parallel_batches` + `MEASUREMENT_ERROR exclusion rule`.

exec
/bin/bash -lc "nl -ba cases/MANIFEST.sha256 | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	d32815e27a722527a7b37e3af8a0f226fb45575e204db08e456081fabbb5dcdd  C-001_post_genesis_minting.yaml
     2	82dd64d06ae3e5c738229c82e6920d1c66a97f46269fb3047f32cc99a1445759  C-002_redistribute_pool_rebirth.yaml
     3	5d0ed176f1048253a13be16d5bebd3c022292fbc7986d6909ea90ce7f15e2c73  C-003_oracle_midstep_blocking.yaml
     4	b2c4a750ad18d055e8a5af21fc5eeebfb9e630896ae7dc28f6441ef875240321  C-004_kernel_domain_leak.yaml
     5	0de749e0bf82de8ff18bf16ad7bd0dfcf433bf18422781d6cf29ae87f8848091  C-005_greedy_router.yaml
     6	3e18c7a2902be49d4baf8936ffcfcab4dbf75e173d5286a2e0aaf68712ab9124  C-006_falsifier_perverse_incentive.yaml
     7	6874e1f6d0a004f80d1bf1994958382f679ae78acdf33e9dd4a959e8531fed01  C-007_tls_deadlock.yaml
     8	a353ee9b9f8e2027857d1e43f42a04563948f65c9808cba84f53ab66e8016381  C-008_proxy_502.yaml
     9	aee0beea7984ec8eb74fcd3c86993124366fc17dbc82dd4651b02d818a83f96f  C-009_format_fragility.yaml
    10	7997b4b9b46fad012a1736bbab4301a55d6ec7ff3d611c9f1ecb811da309237b  C-010_generator_equals_evaluator.yaml
    11	72afbc25939b87cc754dfcff333a699af9887b0480d288a72fd10ec053f27f6f  C-011_brute_force_formalization.yaml
    12	188356e9c5fa9b0603ee020508a394951ec7227436cd1d674b8ac1d5828b6cde  C-012_measurement_correctness.yaml
    13	0f94a4f4d4c24f0f9c5e3ffebf526d41df7ff4baa3624b3a672b9d7d3a29efdd  C-013_fake_depth_dedup.yaml
    14	adc41dd0f1f6aec2027819710dba1d3149e211a59d94367ff2976d25f7475583  C-014_identity_theft.yaml
    15	b5ee333bb74ee9adc22806e7fa65a641105518d1c3f765ccaedc51ffca1c604a  C-015_sorry_cheating.yaml
    16	9ad6e0faa15e3fef123355958b2e5f188ce18ee68ef22286c78c5fd854de6174  C-016_oracle_determinism.yaml
    17	73eebcc7b87c922a1f005db1bc3ef5bceced2fd43b6230713f606f0cdfe8f1fb  C-017_silent_failure.yaml
    18	77be80b2c4bf1ca90572c8da3d8f6f2430362ae5ae80613096c906e2b4c3fa35  C-018_wallet_semantics.yaml
    19	9624b14478dad8bece51d2fedda4b8a48620bf12992b99ab2b3b32f05d60ddd0  C-019_price_waveform.yaml
    20	1fbe8af1c550d4f5bde625a646348b91d09a7aab7ef413b002d379f8b84600f3  C-020_obsolete_threshold.yaml
    21	5efbbfb0fcd26c9714f8d355f9ba4414a1308d3b55b7f7654ff745988ba59ce3  C-021_smart_model_shallow.yaml
    22	bd5c476b777823e29582f2612a761db2bf1f54ff8353003b33b938242730b574  C-022_context_poisoning.yaml
    23	aee9a33d3f4d484d03a9b982206d6e7fa84b1b19d8ba50f32b80f4fd1e3af0ed  C-023_dual_chamber.yaml
    24	e5772558b9b9b884b36dc2dd763881403c3291cb5308588494ee47a5a0eaa2c2  C-024_investment_crowds_building.yaml
    25	e0a5661201008983a92486e142b202c9748830eac8b8fe51c75c8493067573f0  C-025_attention_dilution.yaml
    26	c59cb94b91b32bf9be2e1b81985e1379def35adad0f33c443d05b71c83336bfb  C-026_multistep_packing.yaml
    27	d6c8169e27c940450344b8b029a88b7f8e4e41ca9244cde16e79169992e8cc2c  C-027_hardcoded_params.yaml
    28	81a27d18e024b5f1c5f2b37db25485865e283a7df44cc22ff573fef7182254f9  C-028_tmp_data_loss.yaml
    29	598b7a502581a6e3f9594503017d1422002ddb52f6826f8da5aff6e6ebf71fcb  C-029_joinset_silent_exit.yaml
    30	14e4cf39fa74dd209a41cbb359741d9d445e87aa94e8c574a5eb5207ad44adfb  C-030_cascade_failure.yaml
    31	c666761c8b9f0003623f0f1da8c5c6e765210ebd7caa9d15ce45d331765dc3b2  C-031_institution_over_tuning.yaml
    32	f595cf5f4f1e9dbfee0a94aeca9f7f0ddb1d70fee415cbe8dec5c97881c6a93e  C-032_oracle_mode_contamination.yaml
    33	b289ac03f993cfdf1c4433260f8ee0b92300ed1bb1771b9c13f8ffaaa8feeea3  C-033_causal_attribution.yaml
    34	5996589d7da9bdfbee11cc0575a7b3f939ea7bbc00e7c61bf4d9fc37af39286e  C-034_over_alignment_dogmatism.yaml
    35	6a332c1ab9be1f847c03676c0f461860799f7edfa359c94a36dcbd61a6aec88b  C-035_zero_tolerance.yaml
    36	886936cad0829e96a47f53d3c40ca000df07d11ce8ba207446a3c0019cec6523  C-036_diversity_probe.yaml
    37	dbc21f2c5669673b7513634fa11f91ea9263592723be9a3e6506b7f9def13a96  C-037_tape_wal_persistence.yaml
    38	45cdfd72927bc3c951736ddcf657b6fcbec786ea294e7cebbc662d95988d1467  C-039_proof_artifact_persistence.yaml
    39	b579083a3c1b8a04db3a7bb41f1168a1ff958cd719c6ccb564a03d79787364f0  C-041_cross_problem_wallet_persistence.yaml
    40	ae2b5089a12b0b490d97e908bbf5535a722a30da350c08eaf70422f09d761ccd  C-043_mandatory_wtool_on_omega.yaml
    41	8881de56596927aa48bb7334b9da2af9e1c1c34e6bca3d1e9ea2325907682f12  C-052_pput_as_sole_metric.yaml
    42	cae762a2b38a8015d2bd57a85a000f91cc5c4895f9835cca1c7fcd7f01696f6a  C-066_external_agent_numeric_verification.yaml
    43	640425073ac469d1caf2e2396a060defbf9db8f8e47d32392bdb2e5bdbff5678  C-068_external_model_behavior_drift.yaml
    44	89a83b0813cd57f711310afa9517851c241ce62653fc7c2206b3d5b7852ad10b  C-069_constitutional_alignment_audit_protocol.yaml
    45	b4939e0ab2f1fad570fbf7c7cb2a32ad8267a8cf6d8a383e8ba352f88d975c6f  C-070_preregistration_and_multiplicity_discipline.yaml

exec
/bin/bash -lc "nl -ba Cargo.toml | sed -n '1,120p' && wc -l Cargo.lock" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	[package]
     2	name = "turingosv4"
     3	version = "0.1.0"
     4	edition = "2021"
     5	description = "Silicon-Native Microkernel for LLM Formal Verification Swarm"
     6	
     7	[dependencies]
     8	libc = "0.2"
     9	log = "0.4"
    10	rand = "0.8"
    11	reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
    12	serde = { version = "1", features = ["derive"] }
    13	serde_json = "1"
    14	sha2 = "0.10"
    15	tokio = { version = "1", features = ["rt", "rt-multi-thread", "time", "macros", "process", "io-util", "sync"] }
    16	
    17	[dev-dependencies]
    18	tokio-test = "0.4"
    19	
    20	[workspace]
    21	members = [".", "experiments/minif2f_v4"]
1684 Cargo.lock

exec
/bin/bash -lc 'rg -n "sha2|serde|tokio|tempfile|env_logger" Cargo.lock Cargo.toml experiments/minif2f_v4/Cargo.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/Cargo.toml:13:tokio = { version = "1", features = ["full"] }
experiments/minif2f_v4/Cargo.toml:14:serde = { version = "1", features = ["derive"] }
experiments/minif2f_v4/Cargo.toml:15:serde_json = "1"
experiments/minif2f_v4/Cargo.toml:17:env_logger = "0.11"
Cargo.toml:12:serde = { version = "1", features = ["derive"] }
Cargo.toml:13:serde_json = "1"
Cargo.toml:14:sha2 = "0.10"
Cargo.toml:15:tokio = { version = "1", features = ["rt", "rt-multi-thread", "time", "macros", "process", "io-util", "sync"] }
Cargo.toml:18:tokio-test = "0.4"
Cargo.lock:182:name = "env_logger"
Cargo.lock:344: "tokio",
Cargo.lock:358: "tokio",
Cargo.lock:359: "tokio-rustls",
Cargo.lock:382: "tokio",
Cargo.lock:503: "serde",
Cargo.lock:528: "serde_core",
Cargo.lock:597: "env_logger",
Cargo.lock:600: "serde",
Cargo.lock:601: "serde_json",
Cargo.lock:602: "tokio",
Cargo.lock:721: "tokio",
Cargo.lock:895: "serde",
Cargo.lock:896: "serde_json",
Cargo.lock:897: "serde_urlencoded",
Cargo.lock:899: "tokio",
Cargo.lock:900: "tokio-rustls",
Cargo.lock:985:name = "serde"
Cargo.lock:990: "serde_core",
Cargo.lock:991: "serde_derive",
Cargo.lock:995:name = "serde_core"
Cargo.lock:1000: "serde_derive",
Cargo.lock:1004:name = "serde_derive"
Cargo.lock:1015:name = "serde_json"
Cargo.lock:1022: "serde",
Cargo.lock:1023: "serde_core",
Cargo.lock:1028:name = "serde_urlencoded"
Cargo.lock:1036: "serde",
Cargo.lock:1040:name = "sha2"
Cargo.lock:1177:name = "tokio"
Cargo.lock:1189: "tokio-macros",
Cargo.lock:1194:name = "tokio-macros"
Cargo.lock:1205:name = "tokio-rustls"
Cargo.lock:1211: "tokio",
Cargo.lock:1215:name = "tokio-stream"
Cargo.lock:1222: "tokio",
Cargo.lock:1226:name = "tokio-test"
Cargo.lock:1232: "tokio",
Cargo.lock:1233: "tokio-stream",
Cargo.lock:1246: "tokio",
Cargo.lock:1314: "serde",
Cargo.lock:1315: "serde_json",
Cargo.lock:1316: "sha2",
Cargo.lock:1317: "tokio",
Cargo.lock:1318: "tokio-test",
Cargo.lock:1348: "serde",

exec
/bin/bash -lc 'git ls-files Cargo.lock src/main.rs experiments/minif2f_v4/src/lib.rs handover/preregistration/scripts/run_p0_calibration.sh handover/preregistration/scripts/compute_p0.py' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
Cargo.lock
experiments/minif2f_v4/src/lib.rs
handover/preregistration/scripts/compute_p0.py
handover/preregistration/scripts/run_p0_calibration.sh
src/main.rs

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/Cargo.toml | sed -n '1,80p' && rg -n \"verify_trust_root|turingosv4::boot\" experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
     1	[package]
     2	name = "minif2f_v4"
     3	version = "0.1.0"
     4	edition = "2021"
     5	description = "MiniF2F v4 — Formal verification swarm experiment"
     6	
     7	[[bin]]
     8	name = "evaluator"
     9	path = "src/bin/evaluator.rs"
    10	
    11	[dependencies]
    12	turingosv4 = { path = "../.." }
    13	tokio = { version = "1", features = ["full"] }
    14	serde = { version = "1", features = ["derive"] }
    15	serde_json = "1"
    16	log = "0.4"
    17	env_logger = "0.11"
    18	rand = "0.8"

codex
**VERDICT: VETO**

Do not launch the 576-run batch yet. This is fixable before the batch, so my recommendation is **FIX-THEN-PROCEED**, not redesign. Conviction: **high**.

**Top Blockers**

1. **The calibration batch can run stale or unverified code.**  
   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.

2. **`compute_p0.py` can silently compute p₀ on an incomplete subset.**  
   PREREG requires full adaptation-144 × seeds and denominator `/144` ([PREREG §5.5 lines 446-451](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:446)). But the runner swallows timeout/crash with `|| true` ([line 140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) and emits only `MEASUREMENT_ERROR` without a JSON row ([line 168](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:168)). The estimator skips missing tags ([compute_p0.py:55](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:55)), intersects only present pairs ([line 65](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:65)), and divides by observed problems ([line 91](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:91)). Fix: require exactly 144 problems, exactly 288 paired rows per mode, exact seeds `{31415,2718}`, no duplicates, no missing calibration tags, and abort on any `MEASUREMENT_ERROR` unless an explicit pre-registered retry policy handles it.

3. **The p₀ ceiling abort is not enforced by the workflow.**  
   `compute_p0.py` returns exit 2 if `p0 > 0.10` ([line 131](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:131)), but the runner never invokes it; it only prints a suggested command ([run_p0_calibration.sh:193](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:193)). Fix: full runner must call `compute_p0.py`, check exit status, write structured output, and prevent freeze if nonzero.

**Question Audit**

Q1: The tx-50 shortcut is valid only under a narrow equivalence relation: final `(problem, seed, solved)` status for p₀. It is **not** equivalent for cost, WAL, predicate traversal, or tx count. The header’s “identical cost accumulator” claim ([rollback_sim.rs:12](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/rollback_sim.rs:12)) conflicts with the evaluator’s immediate return at tx 50 ([evaluator.rs:534](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:534)). Change the log from `info!` to `warn!` ([line 535](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:535)). Double-fire is not a real risk because the loop returns immediately ([line 550](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:550)). Serialization is correct: default `None` is omitted ([line 1254](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1254)); treatment sets `Some(true)` ([line 549](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:549)).

Q2: `src/boot.rs` self-hash omission is defensible under the documented passive-tamper threat model ([TRACE §4 lines 70-77](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:70)). But for this batch the bigger issue is that evaluator does not run Boot. Also add calibration scripts and `Cargo.lock` to Trust Root or equivalent launch attestation; the current 16-entry manifest omits them ([genesis_payload.toml:60](/home/zephryj/projects/turingosv4/genesis_payload.toml:60)). `cases/MANIFEST.sha256` transitive hashing is sufficient ([cases/MANIFEST.sha256:1](/home/zephryj/projects/turingosv4/cases/MANIFEST.sha256:1)). `lib.rs` omission is not independently fatal because `evaluator.rs` is hashed and imports the module through it.

Q3: `max(over seeds)` matches PREREG ([PREREG:448](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:448)) and implementation ([compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88)). But `solved()` should read v2 `progress`, not only nonexistent `progress_verified`; v2 schema uses `progress` ([jsonl_schema.rs:96](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:96)). Current fallback works only because Phase B passes runtime and verified together ([evaluator.rs:1158](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1158)).

Q4: No LLM-as-Judge defect found. Control accepts only through Lean (`verify_omega_detailed` / `verify_partial`) ([evaluator.rs:777](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:777), [line 1016](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1016)). Treatment is a measurement artifact, not a production state transition. That counter-argument is structurally sound, but TRACE should mark the rollback mapping partial, not full ✅ ([TRACE:31](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:31)).

Q5: Findings C+D do not affect p₀ directly. `compute_p0.py` reads only solved status from rows ([compute_p0.py:78](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:78)); `verified` is caller-supplied from Lean success paths or explicit false on exhaustion/short-circuit ([evaluator.rs:1218](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1218)). WAL Omega gaps remain Phase D blockers, not this batch’s p₀ blocker.

Q6: Abort is currently manual, not enforced. Borderline `0.05 < p₀ ≤ 0.10` should not trigger post-hoc retuning to tx75; that would violate the frozen protocol. If you want a yellow-band rule, define it before launch.

Q7: API drift and timeout handling are under-specified. The schema has `model_snapshot` ([jsonl_schema.rs:112](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:112)), but runner does not set it. The 40-min timeout ([run_p0_calibration.sh:140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) plus no resume makes failure expensive and bias-prone. Add canary-before/after, exact row coverage, and resume/retry before launch.

Q8: FC3-N34 promotion is premature as a batch guarantee because the evaluator entrypoint does not enforce it. The rollback FC1/FC2 mapping is outcome-analogous, not path-equivalent. Draft TRACE_MATRIX_v2 before launch, after fixes, so the batch audit trail is contemporaneous rather than retrofitted.

**Recommendation: FIX-THEN-PROCEED.** Run no full calibration until the runner is fail-fast, the estimator is strict-complete, the ceiling gate is integrated, and the evaluator launch path enforces Trust Root.
tokens used
243,983
**VERDICT: VETO**

Do not launch the 576-run batch yet. This is fixable before the batch, so my recommendation is **FIX-THEN-PROCEED**, not redesign. Conviction: **high**.

**Top Blockers**

1. **The calibration batch can run stale or unverified code.**  
   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.

2. **`compute_p0.py` can silently compute p₀ on an incomplete subset.**  
   PREREG requires full adaptation-144 × seeds and denominator `/144` ([PREREG §5.5 lines 446-451](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:446)). But the runner swallows timeout/crash with `|| true` ([line 140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) and emits only `MEASUREMENT_ERROR` without a JSON row ([line 168](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:168)). The estimator skips missing tags ([compute_p0.py:55](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:55)), intersects only present pairs ([line 65](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:65)), and divides by observed problems ([line 91](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:91)). Fix: require exactly 144 problems, exactly 288 paired rows per mode, exact seeds `{31415,2718}`, no duplicates, no missing calibration tags, and abort on any `MEASUREMENT_ERROR` unless an explicit pre-registered retry policy handles it.

3. **The p₀ ceiling abort is not enforced by the workflow.**  
   `compute_p0.py` returns exit 2 if `p0 > 0.10` ([line 131](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:131)), but the runner never invokes it; it only prints a suggested command ([run_p0_calibration.sh:193](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:193)). Fix: full runner must call `compute_p0.py`, check exit status, write structured output, and prevent freeze if nonzero.

**Question Audit**

Q1: The tx-50 shortcut is valid only under a narrow equivalence relation: final `(problem, seed, solved)` status for p₀. It is **not** equivalent for cost, WAL, predicate traversal, or tx count. The header’s “identical cost accumulator” claim ([rollback_sim.rs:12](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/rollback_sim.rs:12)) conflicts with the evaluator’s immediate return at tx 50 ([evaluator.rs:534](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:534)). Change the log from `info!` to `warn!` ([line 535](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:535)). Double-fire is not a real risk because the loop returns immediately ([line 550](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:550)). Serialization is correct: default `None` is omitted ([line 1254](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1254)); treatment sets `Some(true)` ([line 549](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:549)).

Q2: `src/boot.rs` self-hash omission is defensible under the documented passive-tamper threat model ([TRACE §4 lines 70-77](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:70)). But for this batch the bigger issue is that evaluator does not run Boot. Also add calibration scripts and `Cargo.lock` to Trust Root or equivalent launch attestation; the current 16-entry manifest omits them ([genesis_payload.toml:60](/home/zephryj/projects/turingosv4/genesis_payload.toml:60)). `cases/MANIFEST.sha256` transitive hashing is sufficient ([cases/MANIFEST.sha256:1](/home/zephryj/projects/turingosv4/cases/MANIFEST.sha256:1)). `lib.rs` omission is not independently fatal because `evaluator.rs` is hashed and imports the module through it.

Q3: `max(over seeds)` matches PREREG ([PREREG:448](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:448)) and implementation ([compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88)). But `solved()` should read v2 `progress`, not only nonexistent `progress_verified`; v2 schema uses `progress` ([jsonl_schema.rs:96](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:96)). Current fallback works only because Phase B passes runtime and verified together ([evaluator.rs:1158](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1158)).

Q4: No LLM-as-Judge defect found. Control accepts only through Lean (`verify_omega_detailed` / `verify_partial`) ([evaluator.rs:777](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:777), [line 1016](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1016)). Treatment is a measurement artifact, not a production state transition. That counter-argument is structurally sound, but TRACE should mark the rollback mapping partial, not full ✅ ([TRACE:31](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:31)).

Q5: Findings C+D do not affect p₀ directly. `compute_p0.py` reads only solved status from rows ([compute_p0.py:78](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:78)); `verified` is caller-supplied from Lean success paths or explicit false on exhaustion/short-circuit ([evaluator.rs:1218](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1218)). WAL Omega gaps remain Phase D blockers, not this batch’s p₀ blocker.

Q6: Abort is currently manual, not enforced. Borderline `0.05 < p₀ ≤ 0.10` should not trigger post-hoc retuning to tx75; that would violate the frozen protocol. If you want a yellow-band rule, define it before launch.

Q7: API drift and timeout handling are under-specified. The schema has `model_snapshot` ([jsonl_schema.rs:112](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:112)), but runner does not set it. The 40-min timeout ([run_p0_calibration.sh:140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) plus no resume makes failure expensive and bias-prone. Add canary-before/after, exact row coverage, and resume/retry before launch.

Q8: FC3-N34 promotion is premature as a batch guarantee because the evaluator entrypoint does not enforce it. The rollback FC1/FC2 mapping is outcome-analogous, not path-equivalent. Draft TRACE_MATRIX_v2 before launch, after fixes, so the batch audit trail is contemporaneous rather than retrofitted.

**Recommendation: FIX-THEN-PROCEED.** Run no full calibration until the runner is fail-fast, the estimator is strict-complete, the ceiling gate is integrated, and the evaluator launch path enforces Trust Root.
