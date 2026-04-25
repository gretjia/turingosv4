# Codex PPUT-CCL Phase B B7-extra ROUND-3 RE-AUDIT (post-VETO-fixes)
**Date**: 2026-04-25
**Predecessor**: CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md (verdict VETO)
**Fixes commits**: 15b87fb (Phase A+B+C) + 438a648 (code-simplifier) + c061450 (constitution) + df77b0a (audit packet record)
**Test baseline**: 187/187 PASS + 20 ignored
**Smoke**: control SOLVED tx=15, treatment UNSOLVED tx=50 with synthetic_short_circuit=true
**Negative test**: tamper Trust Root → evaluator panic at boot ✓
**Prompt size**: 93570 chars

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
session id: 019dc5d2-d7e1-7c53-a011-070572c99dc6
--------
user
# Codex ROUND-3 RE-AUDIT — Phase B B7-extra (after VETO fixes)

**Role**: skeptical adversarial reviewer. Independent of Gemini. CLAUDE.md "Audit Standard": VETO > CHALLENGE > PASS; conservative reading wins on disagreement.

**Mandate**: this is the THIRD audit pass.
- **Round 1** (CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md): VETO with B1+B2+B3 top blockers. Fixed in commit `15b87fb`.
- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
- **Round 3 (this audit)**: determine whether the round-2 fix resolves your previous VETO AND whether it introduces NEW defects. Specifically scrutinize the new crash-vs-timeout discrimination logic and the boot preflight.

**Round-2 fix summary (commit `1df1f62`, on top of `15b87fb`+`438a648`)**:
1. Crash discrimination in runner loop:
   - exit 124 (timeout) → synthetic UNSOLVED row (only this case emits synthetic data)
   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
3. Synthetic row schema: `golden_path_token_count: 0` added (was missing per your CHALLENGE 2).
4. TRACE_MATRIX_v1 manifest size doc-drift fixed (16 → 20 with composition breakdown).

**Verification done**:
- 187/187 cargo test --workspace PASS + 20 ignored
- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
- Happy path smoke: --smoke (mathd_algebra_107, 4 runs, 40s) all SOLVED tx=1; jsonl V2 + calibration tags + MODEL_SNAPSHOT all stamped; no abort.

**Decision rule**:
- If the round-2 fix RESOLVES your prior VETO (no silent absorption path) AND introduces no new P0: **PASS** with HIGH conviction → batch authorized.
- If any new P0 defect or the round-2 fix is incomplete: **CHALLENGE** with specific items.
- If a deeper architectural issue surfaces: **VETO** with explicit "not safe to launch 576-run batch".

**Constitutional amendments since last audit** (commit `c061450`, sudo-authorized):
- Art. V.1.1: human sudo applies *only* to constitution.md (not other Trust Root files)
- Art. V.1.2: ArchitectAI has commit authority on non-constitution files (no sudo needed post-Veto-AI PASS)
- Art. V.1.3: JudgeAI renamed to **Veto-AI**; output domain narrowed to {PASS, VETO} only — explicit white-list exclusion of subjective quality judgments
- FC3 mermaid: judgeAI[JudgeAI] → vetoAI[Veto-AI]
- Art. V.3 (NEW): Constitutional Amendment Log

**State of the code (re-audit baseline)**:
- 187/187 cargo test --workspace PASS + 20 deferred-stub `#[ignore]`
- Hard-problem smoke (aime_1983_p2): control SOLVED tx=15 (~2.5 min); treatment UNSOLVED tx=50 with `synthetic_short_circuit=true` ✓
- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)

## Fix summary by previous finding

| Previous finding | Severity | Fix commit + location |
|---|---|---|
| B1: evaluator doesn't call verify_trust_root | **VETO** | 15b87fb: evaluator.rs:163-183 — `verify_trust_root` called at top of `async fn main`; panic on Err. Negative-test verified end-to-end. |
| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
| Q1.b: info!→warn! at short-circuit log | CHALLENGE | 15b87fb: evaluator.rs short-circuit log now `warn!` with cost-asymmetry warning |
| Q1.a: equivalence claim wrong | CHALLENGE | 15b87fb: rollback_sim.rs header rewritten — narrow equivalence on (problem, seed, solved) only; explicit non-equivalence list |
| Q2.b: src/main.rs not in manifest | **VETO (Gemini)** | 15b87fb: added to genesis_payload.toml |
| Q2.e: Cargo.lock not in manifest | **VETO (Gemini)** | 15b87fb: added |
| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
| Q4 + Q8: TRACE_MATRIX rollback_sim ✅ → ⚠️ | CHALLENGE | 15b87fb: TRACE_MATRIX_v1 § 2 downgraded to ⚠️ partial with annotation |
| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
| Q7.c: no resume mode | CHALLENGE (deferred) | Not addressed in this round — 576 runs are 8h, full restart cost is bounded; resume adds complexity not justified pre-batch |
| Q3.e: runner script seeds not in Trust Root | CHALLENGE | Resolved transitively: runner.sh now in Trust Root (Q2 fix) so seed array is hash-protected |
| Q6.c: yellow-band rule for borderline p_0 | CHALLENGE (open) | Codex previously said "don't post-hoc retune" — accepted; no yellow band defined; passive accept on (0, 0.10] |

## Specific re-audit questions

### RQ1 — B1 fix completeness
`evaluator.rs:163-183` calls `turingosv4::boot::verify_trust_root(&repo_root)` at the top of `async fn main`. The repo_root is resolved from `env!("CARGO_MANIFEST_DIR")` (= `experiments/minif2f_v4`) joined with `..` × 2 + canonicalize.

Vulnerabilities:
- (RQ1.a) Is the canonicalize() correct under all deployment scenarios (e.g., binary copied to a different directory)? PREREG runs the binary from the project tree; deployment-elsewhere is out of scope, agreed?
- (RQ1.b) The verify_trust_root call is BEFORE env_logger::init(). Does that matter for diagnostics? (Panic message goes to stderr regardless.)
- (RQ1.c) Could a future evaluator addition place ANY logic before verify_trust_root that would compromise the order? Defensive structure?
- (RQ1.d) The negative-test (tampered manifest → panic) was verified by the implementer. Is there a corresponding automated test (cargo test) that catches accidental removal of the verify_trust_root call? Do you require one?

### RQ2 — B2 fix completeness
`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.

Vulnerabilities:
- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?

### RQ3 — B3 fix completeness
runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`

Vulnerabilities:
- (RQ3.a) When P0_EXIT=2 (ceiling violated), the runner prints "p_0 EXCEEDS ceiling" and exits 2. Does it ALSO ensure the calibration jsonl files are NOT silently absorbed into Trust Root? Manifest update is a separate manual ArchitectAI step — is that documented?
- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?

### RQ4 — Synthetic timeout row schema
`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).

Vulnerabilities:
- (RQ4.a) The synthetic row schema fields match v2 RunAggregate. Is there a field MISSING that downstream tooling would reasonably expect (e.g., `verified` field — present, set false; `binary_sha256` — empty string; etc.)?
- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).

### RQ5 — New surfaces introduced by fix
- (RQ5.a) `MODEL_SNAPSHOT` env now drift-detection-stamped at runner level. Evaluator was already reading `MODEL_SNAPSHOT` env before B7-extra fix (jsonl_schema.rs:112). The runner's `${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}` format works for downstream pairing? `-dirty` suffix when working tree has unstaged changes.
- (RQ5.b) `BUILD_SHA` env: same. Is `git rev-parse HEAD` the right baseline (vs e.g., `git describe --always`)?
- (RQ5.c) Trust Root manifest now 20 entries. Is this growing unbounded? PREREG § 1.8 is meant to be relatively stable.

### RQ6 — Did the fix introduce code-simplifier regressions?
A code-simplifier pass (commit `438a648`) ran AFTER the audit fix commit (`15b87fb`) but BEFORE this re-audit. Modifications: `Sha256::digest` one-shot in boot.rs, `matches!` macro in rollback_sim.rs, dropped redundant `: PathBuf` annotation in evaluator.rs.

Vulnerabilities:
- (RQ6.a) `Sha256::digest(&bytes)` — semantic-equivalent to `Sha256::new() / update(&bytes) / finalize()`? Does it handle large inputs (genesis_payload.toml: ~3KB; constitution.md: ~25KB; cases/MANIFEST.sha256: ~5KB) correctly without intermediate state issues?
- (RQ6.b) `hex_lower` now uses `std::fmt::Write` + `write!(out, "{b:02x}").unwrap()`. The `.unwrap()` on a `String` write — write to a String never errs by spec (infallible). Sound.
- (RQ6.c) `rollback_simulation_enabled()` reads env via `matches!(... .as_deref(), Ok("1"))`. Earlier was `... .ok().as_deref() == Some("1")`. Equivalent?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED with 576-run batch / FIX-THEN-PROCEED again / REDESIGN

If PASS: cite the specific evidence that the three previous top blockers (B1+B2+B3) are now resolved, AND that no new P0 defect was introduced by the fix.

---

# Appended source files (post-fix state)


## experiments/minif2f_v4/src/rollback_sim.rs (post code-simplifier)

```rust
// PPUT-CCL Phase B B7-extra — synthetic rollback simulation.
//
// Constitutional anchor (TRACE_MATRIX_v1 § 7.2, status ⚠️ partial after
// 2026-04-25 dual-audit re-review): the `--simulate-rollback-at-tx-50`
// toggle (PREREG § 5.5) MAPS TO the conjunction of **FC1-E18** (∏p=0 →
// Q_t preservation) repeated for tx 50..max_transactions and the
// resulting natural **FC2-N22 HALT** with `HaltReason::MaxTxExhausted`.
// No new HaltReason variant is introduced and no new constitutional
// surface is created.
//
// **Equivalence is narrow** (audit-fix 2026-04-25, Codex Q1 + Gemini
// Q1.a): the short-circuit at tx == threshold is equivalent to the
// 150-tx vetoed loop on a *single* observable — the final
// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
// the PREREG § 5.5 estimator. It is **NOT** equivalent on:
//   - cost accumulator C_i (skips ~150 LLM calls × ~250 tokens each)
//   - wall-clock T_i (skips ~150 × per-tx wall-clock contribution)
//   - WAL ledger event sequence (skips ~150 Append/Reject events)
//   - bus-level predicate traversal (the synthetic ∏p=0 is asserted at
//     evaluator layer, not registered with bus.evaluate_predicates)
//   - `tx_count` field (stamped at threshold = 50, not at max_transactions)
//
// Consumers that touch any non-(problem, seed, solved) field on rows
// where `synthetic_short_circuit == Some(true)` MUST disclaim the
// non-equivalence. `compute_p0.py` honors this by reading only the
// `progress` field and the PREREG-frozen seed/problem grid.
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
    matches!(std::env::var(ROLLBACK_ENV_VAR).as_deref(), Ok("1"))
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

## src/boot.rs (post code-simplifier)

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
    for (rel_path, expected) in &manifest {
        let full = repo_root.join(rel_path);
        let bytes = fs::read(&full).map_err(|err| TrustRootError::FileRead {
            path: full.clone(),
            err,
        })?;
        let actual = hex_lower(&Sha256::digest(&bytes));
        if actual != *expected {
            return Err(TrustRootError::Tampered {
                path: full,
                expected: expected.clone(),
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
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(out, "{b:02x}").unwrap();
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

    /// Write a single-entry [trust_root] manifest pointing at `only.txt`
    /// with the given hex hash. Used by both tamper and match tests.
    fn write_single_entry_repo(tmp: &Path, only_txt: &str, manifest_hash: &str) {
        let genesis = format!(
            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n\
             [trust_root]\n\"only.txt\" = \"{manifest_hash}\"\n"
        );
        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
        fs::write(tmp.join("only.txt"), only_txt).unwrap();
    }

    #[test]
    fn verify_trust_root_detects_tamper_in_tempdir() {
        // Manifest claims a zero hash; on-disk content "tampered" hashes to
        // anything else, so verify must surface Tampered.
        let tmp = tempdir();
        write_single_entry_repo(&tmp, "tampered", &"0".repeat(64));
        match verify_trust_root(&tmp).expect_err("tamper must be detected") {
            TrustRootError::Tampered { path, expected, actual } => {
                assert!(path.ends_with("only.txt"));
                assert_eq!(expected, "0".repeat(64));
                assert_ne!(actual, expected);
            }
            other => panic!("expected Tampered, got {other:?}"),
        }
    }

    #[test]
    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
        let tmp = tempdir();
        let payload = "hello";
        let hash = hex_lower(&Sha256::digest(payload.as_bytes()));
        write_single_entry_repo(&tmp, payload, &hash);
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

## src/main.rs

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

## genesis_payload.toml (20 entries)

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
#   2026-04-25 dual-audit fixes (4):
#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
#       in manifest; comment-out = silent bypass)
#     Cargo.lock (audit Q2.e VETO — supply-chain dep-version swap defense)
#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
#       DO-178C tool qualification — runner is a frozen production tool)
#     handover/preregistration/scripts/compute_p0.py (same — estimator is
#       a frozen production tool)
#
# Total: 20 files. genesis_payload.toml itself is conceptually frozen but
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
"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
"Cargo.lock" = "577446e8fe11e91bc8751bf13e5ddca6c5faa64d3309b878768c550d3e6feb98"
"handover/preregistration/scripts/run_p0_calibration.sh" = "92701c2876a69968a4f570a67d39c56e15da0a45d44720d4fe1b6174ecdbd821"
"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
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
"experiments/minif2f_v4/src/rollback_sim.rs" = "9ccdf177813452bb83221bd780f2f7f10da619117aca8e44d3ecb355505fe561"
"experiments/minif2f_v4/src/bin/evaluator.rs" = "2caf3f66ce26a0ef8233e21f6c730a34e369d308671c195c5abcbe7c46034665"
"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
"cases/MANIFEST.sha256" = "3759631afae97c631be9a4038479b8d3681af25885f9758c32ad88c22f17fc3f"
"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"

```

## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)

```bash
#!/usr/bin/env bash
# PPUT-CCL B7-extra — p_0 calibration runner (audit-fixed 2026-04-25).
#
# PREREG § 5.5 protocol:
#   - control:    evaluator on adaptation-144 × seeds [31415, 2718]
#   - treatment:  same + SIMULATE_ROLLBACK_AT_TX_50=1
#   - 288 + 288 = 576 runs total.
#   - regression_p = 1 iff control SOLVED && treatment UNSOLVED, same (problem, seed)
#   - p_0 = sum_p max_seed(regression_p) / 144
#
# Constitutional anchor: see experiments/minif2f_v4/src/rollback_sim.rs header.
#
# Audit-fix 2026-04-25 (dual VETO):
#   - set -e (Codex B1 + Gemini Q6.a) — any subprocess failure aborts batch
#   - cargo build exit checked (Codex B1)
#   - timeout / crash emits a valid UNSOLVED jsonl row instead of dropping
#     to MEASUREMENT_ERROR (Gemini Q7.b + Codex B2) — strict-completeness
#     of compute_p0.py requires every (problem, seed) pair present
#   - runner invokes compute_p0.py at end with exit-code propagation
#     (Codex B3) — p_0 > 0.10 ceiling triggers ABORT
#   - MODEL_SNAPSHOT + GIT_SHA stamped in env for drift detection
#     (Codex Q7) — feeds into evaluator's existing model_snapshot field
#   - canary timestamps logged at batch start + end
#
# Usage:
#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
#     --smoke        1 mathd_algebra problem × 4 runs (~5 min, ~$0.05) — infra check
#     --smoke-hard   1 aime problem × 2 runs (control + treatment, seed=31415,
#                                             ~20 min, ~$0.20) — toggle-fire check
#     (no flag)      full 576-run batch (~$3-5, ~8h — needs explicit user GO)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Auto-load v3 .env for API keys if not already set.
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    # shellcheck disable=SC1090
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$PROJECT_ROOT/experiments/minif2f_v4/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"

MODE_ARG="${1:-}"
SMOKE=0
SMOKE_HARD=0
case "$MODE_ARG" in
    --smoke)        SMOKE=1 ;;
    --smoke-hard)   SMOKE_HARD=1 ;;
    "")             ;;
    *)              echo "Unknown arg: $MODE_ARG"; exit 1 ;;
esac

# PREREG § 5.5: condition fixed at n3 (3-agent swarm — needs >=50 tx capacity).
# Boltzmann seeds frozen at PREREG values. Audit-fix: no seed override path.
CONDITION="n3"
SEEDS=(31415 2718)
MODES=("control" "treatment")

# Drift-detection provenance (Codex Q7). MODEL_SNAPSHOT seeds the evaluator's
# existing model_snapshot jsonl field; GIT_SHA stamps the build commit.
GIT_SHA=$(cd "$PROJECT_ROOT" && git rev-parse HEAD)
GIT_DIRTY=""
if ! (cd "$PROJECT_ROOT" && git diff --quiet HEAD); then
    GIT_DIRTY="-dirty"
fi
export MODEL_SNAPSHOT="${MODEL_SNAPSHOT:-${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}}"
export BUILD_SHA="${BUILD_SHA:-${GIT_SHA}${GIT_DIRTY}}"

mkdir -p "$LOG_DIR"
if [ "$SMOKE" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
elif [ "$SMOKE_HARD" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
else
    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
fi

# Resolve adaptation-144 problem list from frozen splits.
ADAPTATION_IDS=$(python3 -c "
import json
d = json.load(open('$SPLITS_JSON'))
for pid in d['splits']['adaptation']['problem_ids']:
    print(pid)
")

if [ "$SMOKE" -eq 1 ]; then
    SMOKE_ID=$(echo "$ADAPTATION_IDS" | grep "^mathd_algebra" | head -1)
    [ -z "$SMOKE_ID" ] && SMOKE_ID=$(echo "$ADAPTATION_IDS" | head -1)
    ADAPTATION_IDS="$SMOKE_ID"
    echo "[smoke] using single problem: $SMOKE_ID"
elif [ "$SMOKE_HARD" -eq 1 ]; then
    HARD_ID=$(echo "$ADAPTATION_IDS" | grep "^aime_" | head -1)
    [ -z "$HARD_ID" ] && HARD_ID=$(echo "$ADAPTATION_IDS" | tail -1)
    ADAPTATION_IDS="$HARD_ID"
    SEEDS=(31415)  # smoke-hard uses single seed to bound cost
    echo "[smoke-hard] using single problem: $HARD_ID (seed 31415 only)"
fi

# Audit-fix Codex B1: build must succeed; failure aborts.
echo "[$(date -Is)] Building evaluator (release)..."
( cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 ) 2>&1 | tail -3
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"
if [ ! -x "$EVALUATOR" ]; then
    echo "BUILD FAIL: $EVALUATOR not produced. ABORT."
    exit 2
fi

# C-012 oracle preflight (memory feedback_oracle_preflight.md).
echo "[$(date -Is)] Oracle preflight..."
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
    \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) \
    -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
    exit 2
fi
PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
    echo "$PREFLIGHT_OUT" | head -c 500
    exit 2
fi
echo "Oracle preflight OK."

# Audit-fix 2026-04-25 round-2 (Codex re-audit VETO): evaluator boot preflight.
# Run the evaluator binary on a trivial fixture path BEFORE the batch starts,
# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
# the in-loop crash-vs-timeout discrimination would still catch it, but a
# preflight surfaces the failure with the clean diagnostic and zero wasted
# API spend.
echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
    echo "$PREFLIGHT_PROBE" | head -c 800
    exit 2
fi
# Expected: evaluator exits with usage error or problem-not-found (non-zero,
# but no panic). Anything containing "panicked at" except expected-args
# error is suspicious.
if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly:"
    echo "$PREFLIGHT_PROBE" | head -c 800
    exit 2
fi
echo "Evaluator boot preflight OK."

# Run loop. Each (mode, seed, problem) combination = 1 run.
TOTAL_PROBLEMS=$(echo "$ADAPTATION_IDS" | wc -l)
TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
CANARY_START=$(date -Is)
echo ""
echo "=== p_0 calibration ==="
echo "Mode count:    ${#MODES[@]} (control + treatment)"
echo "Seed count:    ${#SEEDS[@]} (${SEEDS[*]})"
echo "Problem count: $TOTAL_PROBLEMS"
echo "Total runs:    $TOTAL_RUNS"
echo "MODEL_SNAPSHOT: $MODEL_SNAPSHOT"
echo "BUILD_SHA:     $BUILD_SHA"
echo "Canary start:  $CANARY_START"
echo ""

# Audit-fix Gemini Q7.b: emit a valid UNSOLVED jsonl row on timeout/crash so
# strict-completeness compute_p0 join sees every pair. The synthesized row
# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
# disambiguator so downstream tooling can distinguish a timeout from a real
# UNSOLVED.
emit_synthetic_unsolved() {
    # args: out_file mode seed pid reason exit_code
    # Emits a v2 RunAggregate-conformant row — Codex re-audit CHALLENGE 2:
    # `golden_path_token_count` is required by jsonl_schema.rs RunAggregate
    # when schema_version == "v2.0". Synthetic rows MUST set it explicitly
    # so downstream v2 tooling parses cleanly.
    # USED ONLY for legitimate timeout (exit 124). Crash paths now ABORT
    # the batch instead — see in-loop comment on the elif branch.
    python3 - <<EOF >> "$1"
import json, time
print(json.dumps({
    "schema_version": "v2.0",
    "run_id": "synthetic_${2}_${4}_$(date +%s)",
    "problem_id": "$4",
    "solved": False,
    "verified": False,
    "progress": 0,
    "split": "adaptation",
    "calibration_mode": "$2",
    "calibration_seed": $3,
    "calibration_problem_id": "$4",
    "synthetic_timeout_or_crash": True,
    "synthetic_reason": "$5",
    "synthetic_exit_code": $6,
    "model_snapshot": "$MODEL_SNAPSHOT",
    "build_sha": "$BUILD_SHA",
    "boltzmann_seed": $3,
    "tx_count": 0,
    "golden_path_token_count": 0,
    "total_run_token_count": 0,
    "total_wall_time_ms": 0,
    "pput_runtime": 0.0,
    "pput_verified": 0.0,
    "pput_m_verified": 0.0,
    "failed_branch_count": 0,
    "rollback_count": 0,
    "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0,
    "git_sha": "$BUILD_SHA",
    "binary_sha256": "",
    "mode": "full",
    "problem": "${MINIF2F_DIR}/MiniF2F/Test/${4}.lean",
    "condition": "$CONDITION",
    "model": "$ACTIVE_MODEL",
    "has_golden_path": False,
    "time_secs": 0.0,
    "pput": 0.0,
    "gp_token_count": 0,
    "gp_node_count": 0,
}))
EOF
}

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
                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND"
                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
                continue
            fi
            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
            # Note: `set -e` is bypassed for this single command via `|| EXIT=$?`
            # so timeout/crash flows into the synthetic-UNSOLVED branch instead
            # of aborting the entire batch.
            EXIT=0
            OUTPUT=$(timeout 2400 env \
                CONDITION="$CONDITION" \
                MINIF2F_DIR="$MINIF2F_DIR" \
                BOLTZMANN_SEED="$SEED" \
                SIMULATE_ROLLBACK_AT_TX_50="$ROLLBACK_FLAG" \
                MODEL_SNAPSHOT="$MODEL_SNAPSHOT" \
                BUILD_SHA="$BUILD_SHA" \
                SPLIT="adaptation" \
                RUST_LOG=info \
                "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || EXIT=$?
            PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1 || true)
            if [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then
                ENRICHED=$(printf '%s' "$PPUT_JSON" | MODE_ENV="$MODE" SEED_ENV="$SEED" PID_ENV="$PID" python3 -c "
import json, os, sys
row = json.loads(sys.stdin.read())
row['calibration_mode'] = os.environ['MODE_ENV']
row['calibration_seed'] = int(os.environ['SEED_ENV'])
row['calibration_problem_id'] = os.environ['PID_ENV']
print(json.dumps(row))
")
                echo "$ENRICHED" >> "$OUT_FILE"
                TX=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tx_count', 0))")
                SOLVED_FLAG=$(echo "$ENRICHED" | python3 -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('progress', 1 if r.get('has_golden_path') else 0)))")
                if [ "$SOLVED_FLAG" = "1" ]; then
                    echo "SOLVED (tx=$TX)"
                else
                    echo "UNSOLVED (tx=$TX)"
                fi
            elif [ "$EXIT" -eq 124 ]; then
                # Audit-fix Gemini Q7.b: timeout is a legitimate UNSOLVED outcome
                # under a fixed wall-clock budget. Emit synthetic row with
                # synthetic_timeout_or_crash=true disambiguator.
                echo "TIMEOUT (exit=124) — emitting synthetic UNSOLVED row"
                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "timeout_2400s" 124
            else
                # Audit-fix Codex re-audit VETO 2026-04-25: any non-timeout
                # non-zero exit (Rust panic 101, segfault 139, OOM 137, etc.) is
                # NOT a legitimate UNSOLVED outcome. It indicates batch corruption.
                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
                # rather than be silently absorbed as UNSOLVED data — otherwise
                # the entire calibration could complete with all-crash rows
                # and produce a "valid" p_0=0 that gets frozen into Trust Root.
                # No synthetic row emitted; partial calibration is forfeited.
                echo "CRASH (exit=$EXIT) — ABORTING BATCH"
                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
                    echo ""
                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
                    echo "  ✗ Boot integrity check failed; investigate manifest vs filesystem state."
                    echo "  ✗ Diagnostic stderr (tail):"
                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
                    exit 3
                fi
                echo ""
                echo "  ✗ Evaluator crashed with exit=$EXIT (not a timeout)."
                echo "  ✗ Calibration data NOT trusted; partial jsonl preserved at $OUT_FILE for diagnosis."
                echo "  ✗ Last 5 stderr lines:"
                tail -5 "$STDERR_LOG" | sed 's/^/    /'
                exit 4
            fi
        done <<< "$ADAPTATION_IDS"
    done
done

CANARY_END=$(date -Is)
BATCH_END=$(date +%s)
WALL_TIME=$((BATCH_END - BATCH_START))

echo ""
echo "╔═══════════════════════════════════════════╗"
echo "║   p_0 CALIBRATION SUMMARY"
echo "╠═══════════════════════════════════════════╣"
echo "║ Wall time:        ${WALL_TIME}s"
echo "║ Canary start:     $CANARY_START"
echo "║ Canary end:       $CANARY_END"
echo "║ MODEL_SNAPSHOT:   $MODEL_SNAPSHOT"
echo "║ BUILD_SHA:        $BUILD_SHA"
echo "║ Control jsonl:    ${OUT_PREFIX}_control.jsonl"
echo "║ Treatment jsonl:  ${OUT_PREFIX}_treatment.jsonl"
echo "╚═══════════════════════════════════════════╝"
echo ""

# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
# = ceiling abort). For smoke modes we skip the estimator (sample size too
# small to be meaningful) and just print the diagnostic snippet.
if [ "$SMOKE" -eq 1 ] || [ "$SMOKE_HARD" -eq 1 ]; then
    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
    echo "Diagnostic: head 1 row from each jsonl"
    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
    exit 0
fi

# Full batch: estimator MUST run, exit code MUST propagate.
echo "[$(date -Is)] Running p_0 estimator (strict-complete mode)..."
P0_JSON="${OUT_PREFIX}_p0_result.json"
set +e
python3 "$SCRIPT_DIR/compute_p0.py" \
    --control "${OUT_PREFIX}_control.jsonl" \
    --treatment "${OUT_PREFIX}_treatment.jsonl" \
    --out-json "$P0_JSON"
P0_EXIT=$?
set -e

if [ "$P0_EXIT" -eq 0 ]; then
    echo ""
    echo "✓ p_0 PASSED ceiling. Result: $P0_JSON"
    echo "  Next: ArchitectAI updates genesis_payload.toml [pput_accounting_0]"
    echo "        + Trust Root manifest entry for the calibration jsonl."
elif [ "$P0_EXIT" -eq 2 ]; then
    echo ""
    echo "✗ p_0 EXCEEDS ceiling (>0.10) — PREREG § 5.5 ABORT."
    echo "  Calibration result NOT frozen into genesis_payload.toml."
    echo "  Action: redesign rollback simulation (per PREREG § 5.5), redo."
    exit 2
else
    echo ""
    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
    echo "  Calibration result NOT frozen. Investigate before retry."
    exit "$P0_EXIT"
fi

```

## handover/preregistration/scripts/compute_p0.py (audit-fixed)

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

    Reads the v2 RunAggregate field `progress` (jsonl_schema.rs:96). Falls
    back to legacy `has_golden_path` for pre-v2 rows. The earlier audit
    found this function was reading a non-existent `progress_verified`
    field — Codex Q3, fixed 2026-04-25.
    """
    if "progress" in row and row["progress"] is not None:
        return int(row["progress"]) == 1
    return bool(row.get("has_golden_path", False))


PREREG_SEEDS = (31415, 2718)
PREREG_N_PROBLEMS = 144


def compute(
    control_rows: list[dict],
    treatment_rows: list[dict],
    *,
    expected_n_problems: int = PREREG_N_PROBLEMS,
    expected_seeds: tuple[int, ...] = PREREG_SEEDS,
) -> dict:
    """PREREG § 5.5 estimator. Strict-complete: requires every (problem, seed)
    pair present in BOTH control and treatment, exact seed set, no missing
    `calibration_*` tags. Audit-fix 2026-04-25 (Codex B2 + Gemini Q3.d): the
    prior silently-skip behaviour biased p_0 by dropping incomplete pairs.
    """
    def index(rows: list[dict], mode: str) -> dict[tuple[str, int], dict]:
        out: dict[tuple[str, int], dict] = {}
        for i, r in enumerate(rows):
            pid = r.get("calibration_problem_id")
            seed = r.get("calibration_seed")
            if pid is None or seed is None:
                sys.exit(
                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
                    "runner stamping bug; refuse to compute p_0 on incomplete data"
                )
            key = (pid, seed)
            if key in out:
                sys.exit(
                    f"ERROR: {mode} duplicate row for (problem={pid}, seed={seed}) — "
                    "runner emitted twice; refuse to compute p_0 on duplicated data"
                )
            out[key] = r
        return out

    c = index(control_rows, "control")
    t = index(treatment_rows, "treatment")

    # Strict completeness: control and treatment key sets must be identical
    # AND must equal expected pre-registered (problem × seed) grid.
    expected_seed_set = set(expected_seeds)
    c_seeds = {seed for _, seed in c.keys()}
    t_seeds = {seed for _, seed in t.keys()}
    if c_seeds != expected_seed_set or t_seeds != expected_seed_set:
        sys.exit(
            f"ERROR: seed mismatch — expected {sorted(expected_seed_set)}; "
            f"control={sorted(c_seeds)}, treatment={sorted(t_seeds)}"
        )

    c_problems = {pid for pid, _ in c.keys()}
    t_problems = {pid for pid, _ in t.keys()}
    if c_problems != t_problems:
        only_c = c_problems - t_problems
        only_t = t_problems - c_problems
        sys.exit(
            f"ERROR: problem set mismatch between control and treatment — "
            f"only_in_control={sorted(only_c)[:5]}{'...' if len(only_c) > 5 else ''}, "
            f"only_in_treatment={sorted(only_t)[:5]}{'...' if len(only_t) > 5 else ''}"
        )

    if len(c_problems) != expected_n_problems:
        sys.exit(
            f"ERROR: expected exactly {expected_n_problems} problems per PREREG § 5.5; "
            f"got {len(c_problems)}. Refuse to compute p_0 on partial batch."
        )

    expected_pair_count = expected_n_problems * len(expected_seed_set)
    if len(c) != expected_pair_count or len(t) != expected_pair_count:
        sys.exit(
            f"ERROR: expected exactly {expected_pair_count} pairs per mode; "
            f"got control={len(c)}, treatment={len(t)}."
        )

    pairs = sorted(c.keys())

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

    # Denominator is the pre-registered count (audit-fix 2026-04-25 Codex
    # B2): if strict-completeness above passed, len(pairs)/len(seeds) ==
    # expected_n_problems by construction. Using the PREREG constant
    # makes the divide-by intent unambiguous.
    n_problems = expected_n_problems
    assert len({pid for pid, _ in pairs}) == n_problems
    p0 = sum(per_problem_regression.values()) / n_problems

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

## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff)

```diff
diff --git a/experiments/minif2f_v4/src/bin/evaluator.rs b/experiments/minif2f_v4/src/bin/evaluator.rs
index 1bdb807..649d87f 100644
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
@@ -147,6 +162,27 @@ struct PputResult {
 #[tokio::main]
 async fn main() {
     env_logger::init();
+
+    // Audit-fix 2026-04-25 (Codex B1 + Q2 — both auditors flagged): the
+    // production batch runs *this* binary, not `src/main.rs`. Without a
+    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
+    // InitAI Trust Root enforcement does NOT actually fire on the calibration
+    // batch. Boot must happen here, at the production entry point, before
+    // any LLM call or jsonl emit.
+    //
+    // Repo root: CARGO_MANIFEST_DIR is `experiments/minif2f_v4`; repo root
+    // is two levels up. canonicalize so a deployed binary still resolves
+    // the genesis path it was built against.
+    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
+        .join("..")
+        .join("..")
+        .canonicalize()
+        .expect("evaluator: repo root resolves at build time");
+    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
+        // FC3-E14 immediate-abort variant. See OBS_BOOT_FAIL_NOT_HALT.
+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
+    }
+
     // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
     // Control binary (main branch) has no such set_var → classifier_version serializes as None.
     // This makes it impossible to mistake one binary for the other in post-hoc analysis.
@@ -497,8 +533,46 @@ async fn run_swarm(
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
+            warn!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
+                   short-circuit to MaxTxExhausted exit (cost-asymmetric: skips \
+                   ~150 LLM calls vs honest vetoed loop; downstream PPUT analysis \
+                   MUST honor synthetic_short_circuit=true on this row)", tx);
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
@@ -1198,6 +1272,9 @@ fn make_pput(
         gp_payload,
         gp_path,
         gp_proof_file,
+        // B7-extra: only the calibration-treatment short-circuit site mutates
+        // this to Some(true). Default = None (most callers).
+        synthetic_short_circuit: None,
     }
 }
 

```

## handover/alignment/TRACE_MATRIX_v1_2026-04-25.md (post-fix)

# TRACE_MATRIX v1 — Constitutional Flowchart ↔ Rust Code (2026-04-25)

**Predecessor**: `TRACE_MATRIX_v0_2026-04-22.md`
**Trigger**: Phase B B7 (Trust Root + Boot freeze) shipped runtime code that (a) implements the Phase 11+ deferred FC3-N34 row and (b) introduces new files to the readonly base. Per CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素", v1 documents the new mappings before downstream work piles on top.

> **2026-04-25 amendment** (post-constitution V.3 修订日志, mid-session): the constitution renamed **JudgeAI → Veto-AI** (Art. V.1.3 + FC3 mermaid `judgeAI` → `vetoAI`). All TRACE_MATRIX v0 references to `JudgeAI` / `judgeAI` (rows FC3-N32, FC3-N42, FC3-N43, FC3-E4/E5/E15, edge `FC3-Veto`) should be read forward-compatibly as Veto-AI / vetoAI. v0 + `FC_ELEMENTS_2026-04-22.md` are immutable audit-trail baselines and are NOT backfilled. Constitutional clarifications also added at V.1.1 (sudo scope = constitution.md only) + V.1.2 (ArchitectAI commit authority on non-constitution files); these reframe how Trust Root is *enforced* (Veto-AI proposal gate + Boot manifest runtime gate) without changing what's *in* the manifest.

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
| `rollback_sim::should_simulate_rollback` | `experiments/minif2f_v4/src/rollback_sim.rs:48` | FC1-E18 (∏p=0 → Q_t) repeated · FC2-N22 HALT (existing `MaxTxExhausted` variant) — **outcome-equivalent only on (problem, seed, solved)** | Y (file header + fn doc) | ⚠️ partial (audit-fix 2026-04-25) |
| `rollback_sim::rollback_simulation_enabled` | `experiments/minif2f_v4/src/rollback_sim.rs:39` | same FC1-E18 + FC2-N22 anchor (env-var read for the predicate); narrow equivalence per above | Y | ⚠️ partial |
| `rollback_sim::ROLLBACK_TX_THRESHOLD` | `experiments/minif2f_v4/src/rollback_sim.rs:34` | PREREG § 5.5 frozen constant (calibration anchor — not a runtime parameter) | Y | ✅ |
| `rollback_sim::ROLLBACK_ENV_VAR` | `experiments/minif2f_v4/src/rollback_sim.rs:38` | env-var name (mirrors PREREG § 5.5 `--simulate-rollback-at-tx-50`) | Y | ✅ |
| `evaluator.rs` short-circuit at line 503-518 | `experiments/minif2f_v4/src/bin/evaluator.rs:503` | FC1-E18 + FC2-N22 (call-site of the synthetic predicate); **path-equivalent NOT verified — bus's evaluate_predicates is not exercised in calibration treatment** | Y (block comment) | ⚠️ partial |

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

**Total manifest size**: **20 files** as of 2026-04-25 post-audit-fix. Composition:
- 15 from B7 (PREREG § 1.8 base 8 + audit accounting 6 + B6 prompt_guard)
- 1 from B7-extra (`rollback_sim.rs`)
- 4 from 2026-04-25 dual-audit fixes (`src/main.rs`, `Cargo.lock`, `handover/preregistration/scripts/run_p0_calibration.sh`, `handover/preregistration/scripts/compute_p0.py`)

Will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step). Final size after Phase B → C exit: 21 files.

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

## constitution.md Art. V (post-amendment)

# 五、Go Meta：架构的架构 [Art. V]

所谓元智慧（meta-intelligence），就是"智慧的智慧"。同理，要让反奥利奥架构生生不息，我们就必须构建"元架构（meta-architecture）"——也就是**架构的架构** [2](#)。

在传统 Boot 过程中，InitAI 只是一个"高级翻译官"：

- 它负责把人类工程师编写的规范（spec）
- 机械地翻译成机器谓词（predicates）等白盒代码

但这里存在一个致命瓶颈：**人类工程师的认知瓶颈**。

当人类无法清晰描述复杂环境规则，或者人类编写的规范本身不够详细、甚至存在逻辑漏洞时，机械的 InitAI 只会把这些缺陷忠实地实例化到顶层白盒中。

这会导致整个系统显得机械而死板，系统能力上限也会被死死锁定在人类边界之内 [3](#)。

要打破这个天花板，系统必须掌握一种新的能力：

> **自己给自己搭架构。**
> 

所有过去的黑盒经验——包括试错与教训——都应当被提取并转化为反奥利奥架构中显式的白盒知识，例如：

- 更明确的提示词文本
- 更清晰的工具设计
- 更完备的验证代码

---

## 5.1 三权分立：元架构层的内部博弈 [Art. V.1]

为了让系统安全地实现自我进化，InitAI 不能是一个单一独裁的黑盒。它内部必须实现严格的"三权分立"机制。

系统演化的本质是：

- 机制
- 突变
- 选择

这恰恰对应元架构层中的三个角色及其永恒博弈。

### 1. 宪法（Constitution）——唯一的基准真相 [Art. V.1.1]

当人类工程师把设计工具、编写测试、搭建环境的权力全部下放给 AI 后，人类在系统中的位置退到了哪里？

答案是：

> **价值观与物理法则的确立者。**
> 

人类不再规定"系统应该怎么做"，而是规定"最顶层的目标与价值观"。这构成了整个系统的绝对底层根基。

> [2026-04-25 架构师补充] **sudo 权限的精确范围**：人类 sudo 权限**仅且只**作用于 `constitution.md` 本身。Trust Root 清单中的其他载荷文件（如 kernel.rs / lean4_oracle.rs / 预注册 / 评分管线 / cases/* 等）属于 ArchitectAI 的合法升级范围 —— 见 V.1.2。系统因此采用两层防御：(i) **提案时**由 Veto-AI 校核违宪与否（V.1.3）；(ii) **运行时**由 Boot 的 SHA-256 manifest 保证"实际加载的字节 == 已 commit 的字节"。这两层与本节的 sudo 形成**三段守护**：宪法 sudo（少数）+ Veto-AI 提案闸（多数）+ Boot 整合性闸（全部）。

---

### 2. ArchitectAI（架构师 AI）——提出者 [Art. V.1.2]

它是系统中的"激进改革派"。

当系统在运行中发现现有白盒存在缺陷时，例如：

- 工具（tools）不够用
- 当前顶层谓词（predicates）无法有效过滤新的幻觉

ArchitectAI 会主动分析系统日志，进而：

- 编写新的验证谓词函数
- 发明新的底层工具
- 重新设计文件系统（tape）的存储结构

它是系统熵减的引擎。

> [2026-04-25 架构师补充] **ArchitectAI 拥有架构升级的 commit 权限**，而非仅止于"提出"。任何**不涉及 `constitution.md`** 的修改 —— 包括 Trust Root 清单（`genesis_payload.toml`）本身的载荷条目增减与 SHA-256 重算 —— 经 Veto-AI（V.1.3）校核未发现违宪后，由 ArchitectAI 直接落盘并更新 manifest 对应字段。此过程**不需要人类 sudo**。理由：宪法已经是 ground truth，ArchitectAI 的修改若未违宪则属于宪法允许的演化空间；强制 sudo 反而把人类拉回工程细节，违反 V.1.1 "人类只立宪法"的 Meta 原则。

---

### 3. Veto-AI（违宪否决 AI）——验证者 [Art. V.1.3]

> [2026-04-25 架构师补充] 此前称 JudgeAI；本次重命名为 **Veto-AI**，以精确反映其唯一职责：单一的"违宪否决权"，无任何主观评判维度。命名变更已贯穿至下方 FC3 流程图节点 `vetoAI`。

它是系统中的"保守守门人"。

ArchitectAI 提出的任何架构变更，都**不能**直接上线。必须经过 Veto-AI 的冷酷审查。

Veto-AI 唯一的工作是：

> 拿着宪法，反复逐条校验 ArchitectAI 生成的新架构代码是否"违宪"。
> 

它**不**做（白名单严格排除）：

- 主观质量评价（代码可读性 / 工程口味）
- 性能或效率评判
- 测试覆盖率主观打分
- 任何与"违宪与否"**无关**的判定

它只做一件事：

> 否决违宪提案。
> 

也就是说，它只负责（偏）客观的违宪判定 —— 输出域 = `{PASS, VETO}` —— 不承担其他主观评价。任何把 Veto-AI 用于"代码评审"或"质量审计"的尝试都属于范畴越界，应由独立的非 Veto-AI 审计者承担。

---

> **Meta 的本质**
> 

> 
> 

> 在终极 Meta 形态中，人类架构师的唯一意义是：
> 

> 
> 

> **设立总架构的 Ground Truth——"宪法"。**
> 

> 
> 

> 这部宪法被存放在只读文件系统上，只有人类架构师才拥有修改它的 sudo 权限。
> 

---

## 5.2 宪法界限与示例 [Art. V.2]

下面给出一些可能的"宪法级"约束：

- 系统的总算力消耗不得超过 $10000$
- 必须在 24 小时内给出结果
- 任何状态变更必须具有可逆性（总是能够回滚到 $Q_{t-1}$）
- 核心谓词逻辑必须是确定性的，禁止引入概率模型
- ……

---

## 5.3 宪法修订日志 [Art. V.3]

> 宪法的修改唯一触发条件 = 人类架构师的显式 sudo 授权。每次修订必须在此处留痕，注明日期、变更摘要、动机，以便所有后续 ArchitectAI / Veto-AI / 审计者可独立复原"宪法在某时刻的形态"。

| 日期 | 触发者 | 章节 | 摘要 |
|---|---|---|---|
| 2026-04-25 | 人类架构师 | V.1.1 | 明确 sudo 权限**仅**作用于 `constitution.md` 本身；Trust Root 清单中其他文件归 ArchitectAI 升级范围；说明"sudo + Veto-AI + Boot manifest"三段守护结构。 |
| 2026-04-25 | 人类架构师 | V.1.2 | 明确 ArchitectAI 拥有 commit 权限（不止"提出"）；任何不动 `constitution.md` 的修改经 Veto-AI PASS 后由 ArchitectAI 直接落盘并更新 Trust Root manifest，无需人类 sudo。 |
| 2026-04-25 | 人类架构师 | V.1.3 + FC3 | JudgeAI 重命名为 **Veto-AI**；增加白名单严格排除（不做主观质量、性能、可读性评判）；FC3 流程图节点 `judgeAI` 同步重命名为 `vetoAI`。 |

---

> "损之又损，以至于无为，无为而无不为……"
> 

> 
> 

> —— 老子《道德经》
> 

    graph TB
        classDef white fill:#fff,stroke:#333,stroke-width:2px,color:#900
        classDef black fill:#111,stroke:#333,stroke-width:2px,color:#900
        classDef human fill:#fff4e6,stroke:#a85d00,stroke-width:2px,color:#5c3200
        classDef note fill:#fff8cc,stroke:#8a6d00,stroke-width:1px,color:#4d3d00
    
        boot
        human:::human
        human -->|maintain| constitution
    
        subgraph system
            subgraph init["InitAI"]
                subgraph readonly
                    constitution:::white@{ shape: doc, label: "constitution as ground truth" }
                    logs:::white@{ shape: docs, label: "logs archive as ground truth" }
                end
                vetoAI[Veto-AI]:::black
                architectAI[ArchitectAI]:::black
            end
    
            subgraph anti_oreo["anti-oreo"]
                top:::white
                agents:::black
                tools:::white
            end
    


---

## experiments/minif2f_v4/tests/trust_root_immutability.rs (post-fix)

```rust
// PPUT-CCL Phase B7 — Trust Root immutability (PREREG § 1.8 + § 7 Gate B).
//
// Boot computes SHA-256 of every Trust Root file at process start and
// compares against the genesis_payload.toml [trust_root] manifest. Any
// mismatch = `TRUST_ROOT_TAMPERED` abort.
//
// Trust Root manifest (PREREG § 1.8 + audit additions through 2026-04-25):
//   src/main.rs                                       (audit-fix Q2.b)
//   src/kernel.rs
//   src/wal.rs
//   src/bus.rs
//   src/drivers/llm_http.rs                           (B2-B4 audit add)
//   src/sdk/prompt_guard.rs                           (B6 add)
//   Cargo.lock                                        (audit-fix Q2.e)
//   experiments/minif2f_v4/src/lean4_oracle.rs
//   experiments/minif2f_v4/src/cost_aggregator.rs     (B2)
//   experiments/minif2f_v4/src/wall_clock.rs          (B3)
//   experiments/minif2f_v4/src/post_hoc_verifier.rs   (B4)
//   experiments/minif2f_v4/src/jsonl_schema.rs        (B1)
//   experiments/minif2f_v4/src/rollback_sim.rs        (B7-extra)
//   experiments/minif2f_v4/src/bin/evaluator.rs       (the wiring)
//   constitution.md
//   handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
//   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
//   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
//   cases/MANIFEST.sha256                             (proxy for cases/*.yaml)

use std::fs;
use std::path::{Path, PathBuf};
use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR for this test crate is experiments/minif2f_v4 — repo
    // root is two levels up.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read_genesis() -> String {
    fs::read_to_string(repo_root().join("genesis_payload.toml")).expect("genesis exists")
}

#[test]
fn test_trust_root_immutable_at_boot() {
    // Cold-start with intact files: Boot computes SHA-256s, all match
    // genesis manifest, process continues. No abort.
    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
}

#[test]
fn test_trust_root_simulated_write_aborts() {
    // Simulated tampering: build a self-contained fake-repo in a tempdir
    // with a single Trust Root entry whose recorded hash does not match
    // the file content; assert verify_trust_root returns Tampered.
    let tmp = make_tempdir("trust_root_tamper");
    let zero_hash = "0".repeat(64);
    let genesis = format!(
        "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{zero_hash}\"\n"
    );
    fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
    fs::write(tmp.join("only.txt"), "tampered content").unwrap();

    match verify_trust_root(&tmp) {
        Err(TrustRootError::Tampered { path, expected, actual }) => {
            assert!(path.ends_with("only.txt"));
            assert_eq!(expected, zero_hash);
            assert_ne!(actual, expected);
        }
        other => panic!("expected Tampered, got {other:?}"),
    }
}

#[test]
fn test_trust_root_manifest_includes_b2_b4_files() {
    // Mid-term audit recommendation: B2 (cost_aggregator), B3 (wall_clock),
    // B4 (post_hoc_verifier), B1 (jsonl_schema), evaluator.rs, llm_http.rs
    // MUST be in the Trust Root manifest. B6 added prompt_guard.rs.
    let entries = parse_trust_root_section(&read_genesis()).expect("parse trust_root");
    let keys: Vec<&str> = entries.iter().map(|(k, _)| k.as_str()).collect();

    let required = [
        // PREREG § 1.8 base
        "src/kernel.rs",
        "src/wal.rs",
        "src/bus.rs",
        "experiments/minif2f_v4/src/lean4_oracle.rs",
        "constitution.md",
        "cases/MANIFEST.sha256",
        "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json",
        "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
        // Mid-term audit accounting layer
        "src/drivers/llm_http.rs",
        "experiments/minif2f_v4/src/cost_aggregator.rs",
        "experiments/minif2f_v4/src/wall_clock.rs",
        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
        "experiments/minif2f_v4/src/jsonl_schema.rs",
        "experiments/minif2f_v4/src/bin/evaluator.rs",
        // B6 add
        "src/sdk/prompt_guard.rs",
        // B7-extra add
        "experiments/minif2f_v4/src/rollback_sim.rs",
        // 2026-04-25 dual-audit fixes
        "src/main.rs",
        "Cargo.lock",
        "handover/preregistration/scripts/run_p0_calibration.sh",
        "handover/preregistration/scripts/compute_p0.py",
    ];

    for path in required {
        assert!(
            keys.contains(&path),
            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
        );
    }
}

#[test]
fn test_pput_accounting_0_section_present() {
    // genesis_payload.toml must contain [pput_accounting_0] with the PREREG
    // § 1.8 keys.
    let genesis = read_genesis();
    let body = extract_section(&genesis, "pput_accounting_0").expect("section present");
    let body = body.as_str();

    let required_keys = [
        "schema_version",
        "progress_definition",
        "cost_definition",
        "time_definition",
        "verified_predicate",
        "heldout_sealed_hash",
        "source_pool_sha256",
        "baseline_regression_rate",
        "baseline_regression_jsonl_sha256",
        "k_max",
        "n_max",
    ];
    for key in required_keys {
        let needle = format!("{key} =");
        assert!(
            body.contains(&needle),
            "[pput_accounting_0] missing key: {key}"
        );
    }

    // Frozen invariants from PREREG § 1.8: heldout sealed hash, k_max, n_max.
    assert!(body.contains(
        "\"51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b\""
    ), "heldout_sealed_hash diverges from PREREG § 2.3");
    assert!(body.contains("k_max = 10"), "k_max must be 10 per PREREG");
    assert!(body.contains("n_max = 34"), "n_max must be 34 per PREREG");
}

// --- helpers ---

fn extract_section(text: &str, name: &str) -> Option<String> {
    // Line-anchored scan: skip commented-out section headers (e.g. inside
    // the file's leading docstring) and only match real headers in column 0.
    let mut in_section = false;
    let mut body = String::new();
    let target = format!("[{name}]");
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_section = trimmed == target;
            continue;
        }
        if in_section {
            body.push_str(line);
            body.push('\n');
        }
    }
    if body.is_empty() {
        None
    } else {
        Some(body)
    }
}

fn make_tempdir(tag: &str) -> PathBuf {
    let pid = std::process::id();
    let nano = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir: PathBuf = std::env::temp_dir().join(format!("turingosv4-{tag}-{pid}-{nano}"));
    fs::create_dir_all(&dir).unwrap();
    let _: &Path = dir.as_path();
    dir
}

```

## smoke evidence — control row (aime_1983_p2)

```json
{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777138897126", "problem_id": "aime_1983_p2", "solved": true, "split": "adaptation", "verified": true, "golden_path_token_count": 1420, "total_run_token_count": 12240, "total_wall_time_ms": 149170, "progress": 1, "pput_runtime": 5.476928766188159e-07, "pput_verified": 5.476928766188159e-07, "pput_m_verified": 0.5476928766188158, "failed_branch_count": 14, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": true, "time_secs": 149.172273801, "pput": 0.6703658625824985, "gp_token_count": 1420, "gp_node_count": 6, "tx_count": 15, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"omega_wtool": 1, "step": 15, "step_reject": 9, "step_partial_ok": 5}, "gp_payload": "rcases h\u2080 with \u27e8hp_pos, hp_lt_15\u27e9\nrcases h\u2081 with \u27e8hx_ge_p, hx_le_15\u27e9\nhave hx_nonneg : 0 \u2264 x := by linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\ncalc\n  f x = abs (x - p) + abs (x - 15) + abs (x - p - 15) := by\n    rw [h\u2082]\n  _ = abs (x - p) + abs (x - 15) + abs ((x - p) - 15) := by ring\n  _ \u2265 abs (x - p) + abs (x - 15) + abs (abs (x - p) - 15) := by\n    have h : abs ((x - p) - 15) \u2265 abs (abs (x - p) - 15) := by\n      exact abs_sub_abs_le_abs_sub _ _\n    linarith\n  _ \u2265 abs (x - p) + abs (x - 15) + (abs (x - p) - 15) := by\n    have h' : abs (abs (x - p) - 15) \u2265 abs (x - p) - 15 := by\n      nlinarith [abs_nonneg (x - p)]\n    linarith\n  _ = 2 * abs (x - p) + abs (x - 15) - 15 := by ring\n  _ \u2265 2 * abs (x - p) + (15 - x) - 15 := by\n    have hx15 : x \u2264 15 := hx_le_15\n    have : abs (x - 15) = 15 - x := by\n      rw [abs_of_nonpos (sub_nonpos.mpr hx15)]\n      ring\n    rw [this]\n    nlinarith\n  _ = 2 * abs (x - p) - x := by ring\n  _ \u2265 2 * (x - p) - x := by\n    have hxp : x - p \u2265 0 := sub_nonneg.mpr hx_ge_p\n    have : abs (x - p) = x - p := abs_of_nonneg hxp\n    rw [this]\n    nlinarith\n  _ = x - 2 * p := by ring\n  _ \u2265 15 - 2 * p := by nlinarith\n  _ \u2265 15 := by nlinarith", "gp_path": "per_tactic", "gp_proof_file": "proofs/aime_1983_p2_1777138897_a9617ab3.lean", "calibration_mode": "control", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}

```

## smoke evidence — treatment row (aime_1983_p2)

```json
{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777139401511", "problem_id": "aime_1983_p2", "solved": false, "split": "adaptation", "verified": false, "golden_path_token_count": 0, "total_run_token_count": 40023, "total_wall_time_ms": 504246, "progress": 0, "pput_runtime": 0.0, "pput_verified": 0.0, "pput_m_verified": 0.0, "failed_branch_count": 50, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": false, "time_secs": 504.246494102, "pput": 0.0, "gp_token_count": 0, "gp_node_count": 0, "tx_count": 50, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"step_partial_ok": 3, "parse_fail": 3, "step_reject": 44, "step": 47}, "synthetic_short_circuit": true, "calibration_mode": "treatment", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}

```

---

Now give your INDEPENDENT re-audit. Cite §/file:line for every finding.

codex
I’ll re-audit from the checked-out tree rather than relying only on the appended snippets, with focus on the runner’s crash/timeout branches, boot ordering, and strict-completeness estimator behavior.
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'run_p0_calibration.sh' -g 'compute_p0.py' -g 'evaluator.rs' -g 'boot.rs' -g 'trust_root_immutability.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
src/boot.rs
experiments/minif2f_v4/tests/trust_root_immutability.rs
CLAUDE.md
experiments/minif2f_v4/src/bin/evaluator.rs
handover/preregistration/scripts/run_p0_calibration.sh
handover/preregistration/scripts/compute_p0.py

exec
/bin/bash -lc 'git status --short && git log --oneline -8' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M rules/enforcement.log
?? handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
?? handover/audits/run_codex_b7_extra_round3_audit.sh
?? handover/audits/run_gemini_b7_extra_round3_audit.py
584001a B7-extra round-2 audit packet (Codex VETO / Gemini PASS)
1df1f62 B7-extra round-2 fix: Codex VETO on crash-as-data + 2 minor
438a648 B7-extra code-simplifier pass (Karpathy aesthetic, 无损压缩即智能)
15b87fb B7-extra dual-audit fixes (Phase A+B+C)
c061450 Constitutional amendment 2026-04-25 (sudo)
df77b0a B7-extra pre-batch dual audit: VETO/VETO
1875543 Thesis-v2 audit — ground-truth feedback alignment
3868b1d B7-extra: architect-insights — abstraction-depth findings

codex
The working tree has unrelated audit artifacts and `rules/enforcement.log` modified; I’m treating those as out of scope and not touching them. I found no repo `AGENTS.md`, so I’m using the prompt-level instructions plus `CLAUDE.md`.
exec
/bin/bash -lc 'rg -n "Audit Standard|VETO|CHALLENGE|PASS|反奥利奥|flowchart|pub symbol" CLAUDE.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
8:- 唯一对齐文档: `constitution.md` (反奥利奥架构)
17:## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
19:- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
37:- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
41:- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage

exec
/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,330p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env bash
     2	# PPUT-CCL B7-extra — p_0 calibration runner (audit-fixed 2026-04-25).
     3	#
     4	# PREREG § 5.5 protocol:
     5	#   - control:    evaluator on adaptation-144 × seeds [31415, 2718]
     6	#   - treatment:  same + SIMULATE_ROLLBACK_AT_TX_50=1
     7	#   - 288 + 288 = 576 runs total.
     8	#   - regression_p = 1 iff control SOLVED && treatment UNSOLVED, same (problem, seed)
     9	#   - p_0 = sum_p max_seed(regression_p) / 144
    10	#
    11	# Constitutional anchor: see experiments/minif2f_v4/src/rollback_sim.rs header.
    12	#
    13	# Audit-fix 2026-04-25 (dual VETO):
    14	#   - set -e (Codex B1 + Gemini Q6.a) — any subprocess failure aborts batch
    15	#   - cargo build exit checked (Codex B1)
    16	#   - timeout / crash emits a valid UNSOLVED jsonl row instead of dropping
    17	#     to MEASUREMENT_ERROR (Gemini Q7.b + Codex B2) — strict-completeness
    18	#     of compute_p0.py requires every (problem, seed) pair present
    19	#   - runner invokes compute_p0.py at end with exit-code propagation
    20	#     (Codex B3) — p_0 > 0.10 ceiling triggers ABORT
    21	#   - MODEL_SNAPSHOT + GIT_SHA stamped in env for drift detection
    22	#     (Codex Q7) — feeds into evaluator's existing model_snapshot field
    23	#   - canary timestamps logged at batch start + end
    24	#
    25	# Usage:
    26	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
    27	#     --smoke        1 mathd_algebra problem × 4 runs (~5 min, ~$0.05) — infra check
    28	#     --smoke-hard   1 aime problem × 2 runs (control + treatment, seed=31415,
    29	#                                             ~20 min, ~$0.20) — toggle-fire check
    30	#     (no flag)      full 576-run batch (~$3-5, ~8h — needs explicit user GO)
    31	
    32	set -euo pipefail
    33	
    34	SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    35	PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
    36	
    37	# Auto-load v3 .env for API keys if not already set.
    38	if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    39	    # shellcheck disable=SC1090
    40	    source "$HOME/projects/turingosv3/.env"
    41	fi
    42	export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
    43	export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
    44	
    45	MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
    46	LOG_DIR="$PROJECT_ROOT/experiments/minif2f_v4/logs"
    47	TIMESTAMP=$(date +%Y%m%dT%H%M%S)
    48	SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"
    49	
    50	MODE_ARG="${1:-}"
    51	SMOKE=0
    52	SMOKE_HARD=0
    53	case "$MODE_ARG" in
    54	    --smoke)        SMOKE=1 ;;
    55	    --smoke-hard)   SMOKE_HARD=1 ;;
    56	    "")             ;;
    57	    *)              echo "Unknown arg: $MODE_ARG"; exit 1 ;;
    58	esac
    59	
    60	# PREREG § 5.5: condition fixed at n3 (3-agent swarm — needs >=50 tx capacity).
    61	# Boltzmann seeds frozen at PREREG values. Audit-fix: no seed override path.
    62	CONDITION="n3"
    63	SEEDS=(31415 2718)
    64	MODES=("control" "treatment")
    65	
    66	# Drift-detection provenance (Codex Q7). MODEL_SNAPSHOT seeds the evaluator's
    67	# existing model_snapshot jsonl field; GIT_SHA stamps the build commit.
    68	GIT_SHA=$(cd "$PROJECT_ROOT" && git rev-parse HEAD)
    69	GIT_DIRTY=""
    70	if ! (cd "$PROJECT_ROOT" && git diff --quiet HEAD); then
    71	    GIT_DIRTY="-dirty"
    72	fi
    73	export MODEL_SNAPSHOT="${MODEL_SNAPSHOT:-${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}}"
    74	export BUILD_SHA="${BUILD_SHA:-${GIT_SHA}${GIT_DIRTY}}"
    75	
    76	mkdir -p "$LOG_DIR"
    77	if [ "$SMOKE" -eq 1 ]; then
    78	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
    79	elif [ "$SMOKE_HARD" -eq 1 ]; then
    80	    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
    81	else
    82	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
    83	fi
    84	
    85	# Resolve adaptation-144 problem list from frozen splits.
    86	ADAPTATION_IDS=$(python3 -c "
    87	import json
    88	d = json.load(open('$SPLITS_JSON'))
    89	for pid in d['splits']['adaptation']['problem_ids']:
    90	    print(pid)
    91	")
    92	
    93	if [ "$SMOKE" -eq 1 ]; then
    94	    SMOKE_ID=$(echo "$ADAPTATION_IDS" | grep "^mathd_algebra" | head -1)
    95	    [ -z "$SMOKE_ID" ] && SMOKE_ID=$(echo "$ADAPTATION_IDS" | head -1)
    96	    ADAPTATION_IDS="$SMOKE_ID"
    97	    echo "[smoke] using single problem: $SMOKE_ID"
    98	elif [ "$SMOKE_HARD" -eq 1 ]; then
    99	    HARD_ID=$(echo "$ADAPTATION_IDS" | grep "^aime_" | head -1)
   100	    [ -z "$HARD_ID" ] && HARD_ID=$(echo "$ADAPTATION_IDS" | tail -1)
   101	    ADAPTATION_IDS="$HARD_ID"
   102	    SEEDS=(31415)  # smoke-hard uses single seed to bound cost
   103	    echo "[smoke-hard] using single problem: $HARD_ID (seed 31415 only)"
   104	fi
   105	
   106	# Audit-fix Codex B1: build must succeed; failure aborts.
   107	echo "[$(date -Is)] Building evaluator (release)..."
   108	( cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 ) 2>&1 | tail -3
   109	EVALUATOR="$PROJECT_ROOT/target/release/evaluator"
   110	if [ ! -x "$EVALUATOR" ]; then
   111	    echo "BUILD FAIL: $EVALUATOR not produced. ABORT."
   112	    exit 2
   113	fi
   114	
   115	# C-012 oracle preflight (memory feedback_oracle_preflight.md).
   116	echo "[$(date -Is)] Oracle preflight..."
   117	LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
   118	PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
   119	    \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) \
   120	    -type d 2>/dev/null | tr '\n' ':')
   121	if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
   122	    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
   123	    exit 2
   124	fi
   125	PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
   126	    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
   127	if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
   128	    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
   129	    echo "$PREFLIGHT_OUT" | head -c 500
   130	    exit 2
   131	fi
   132	echo "Oracle preflight OK."
   133	
   134	# Audit-fix 2026-04-25 round-2 (Codex re-audit VETO): evaluator boot preflight.
   135	# Run the evaluator binary on a trivial fixture path BEFORE the batch starts,
   136	# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
   137	# the in-loop crash-vs-timeout discrimination would still catch it, but a
   138	# preflight surfaces the failure with the clean diagnostic and zero wasted
   139	# API spend.
   140	echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
   141	PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
   142	if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
   143	    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
   144	    echo "$PREFLIGHT_PROBE" | head -c 800
   145	    exit 2
   146	fi
   147	# Expected: evaluator exits with usage error or problem-not-found (non-zero,
   148	# but no panic). Anything containing "panicked at" except expected-args
   149	# error is suspicious.
   150	if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
   151	   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
   152	    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly:"
   153	    echo "$PREFLIGHT_PROBE" | head -c 800
   154	    exit 2
   155	fi
   156	echo "Evaluator boot preflight OK."
   157	
   158	# Run loop. Each (mode, seed, problem) combination = 1 run.
   159	TOTAL_PROBLEMS=$(echo "$ADAPTATION_IDS" | wc -l)
   160	TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
   161	CANARY_START=$(date -Is)
   162	echo ""
   163	echo "=== p_0 calibration ==="
   164	echo "Mode count:    ${#MODES[@]} (control + treatment)"
   165	echo "Seed count:    ${#SEEDS[@]} (${SEEDS[*]})"
   166	echo "Problem count: $TOTAL_PROBLEMS"
   167	echo "Total runs:    $TOTAL_RUNS"
   168	echo "MODEL_SNAPSHOT: $MODEL_SNAPSHOT"
   169	echo "BUILD_SHA:     $BUILD_SHA"
   170	echo "Canary start:  $CANARY_START"
   171	echo ""
   172	
   173	# Audit-fix Gemini Q7.b: emit a valid UNSOLVED jsonl row on timeout/crash so
   174	# strict-completeness compute_p0 join sees every pair. The synthesized row
   175	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
   176	# disambiguator so downstream tooling can distinguish a timeout from a real
   177	# UNSOLVED.
   178	emit_synthetic_unsolved() {
   179	    # args: out_file mode seed pid reason exit_code
   180	    # Emits a v2 RunAggregate-conformant row — Codex re-audit CHALLENGE 2:
   181	    # `golden_path_token_count` is required by jsonl_schema.rs RunAggregate
   182	    # when schema_version == "v2.0". Synthetic rows MUST set it explicitly
   183	    # so downstream v2 tooling parses cleanly.
   184	    # USED ONLY for legitimate timeout (exit 124). Crash paths now ABORT
   185	    # the batch instead — see in-loop comment on the elif branch.
   186	    python3 - <<EOF >> "$1"
   187	import json, time
   188	print(json.dumps({
   189	    "schema_version": "v2.0",
   190	    "run_id": "synthetic_${2}_${4}_$(date +%s)",
   191	    "problem_id": "$4",
   192	    "solved": False,
   193	    "verified": False,
   194	    "progress": 0,
   195	    "split": "adaptation",
   196	    "calibration_mode": "$2",
   197	    "calibration_seed": $3,
   198	    "calibration_problem_id": "$4",
   199	    "synthetic_timeout_or_crash": True,
   200	    "synthetic_reason": "$5",
   201	    "synthetic_exit_code": $6,
   202	    "model_snapshot": "$MODEL_SNAPSHOT",
   203	    "build_sha": "$BUILD_SHA",
   204	    "boltzmann_seed": $3,
   205	    "tx_count": 0,
   206	    "golden_path_token_count": 0,
   207	    "total_run_token_count": 0,
   208	    "total_wall_time_ms": 0,
   209	    "pput_runtime": 0.0,
   210	    "pput_verified": 0.0,
   211	    "pput_m_verified": 0.0,
   212	    "failed_branch_count": 0,
   213	    "rollback_count": 0,
   214	    "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0,
   215	    "git_sha": "$BUILD_SHA",
   216	    "binary_sha256": "",
   217	    "mode": "full",
   218	    "problem": "${MINIF2F_DIR}/MiniF2F/Test/${4}.lean",
   219	    "condition": "$CONDITION",
   220	    "model": "$ACTIVE_MODEL",
   221	    "has_golden_path": False,
   222	    "time_secs": 0.0,
   223	    "pput": 0.0,
   224	    "gp_token_count": 0,
   225	    "gp_node_count": 0,
   226	}))
   227	EOF
   228	}
   229	
   230	BATCH_START=$(date +%s)
   231	RUN_IDX=0
   232	for MODE in "${MODES[@]}"; do
   233	    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
   234	    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
   235	    : > "$OUT_FILE"
   236	    : > "$STDERR_LOG"
   237	    case "$MODE" in
   238	        control)   ROLLBACK_FLAG="" ;;
   239	        treatment) ROLLBACK_FLAG="1" ;;
   240	    esac
   241	    for SEED in "${SEEDS[@]}"; do
   242	        while IFS= read -r PID; do
   243	            [ -z "$PID" ] && continue
   244	            RUN_IDX=$((RUN_IDX + 1))
   245	            PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
   246	            if [ ! -f "$PROBLEM" ]; then
   247	                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND"
   248	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
   249	                continue
   250	            fi
   251	            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
   252	            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
   253	            # Note: `set -e` is bypassed for this single command via `|| EXIT=$?`
   254	            # so timeout/crash flows into the synthetic-UNSOLVED branch instead
   255	            # of aborting the entire batch.
   256	            EXIT=0
   257	            OUTPUT=$(timeout 2400 env \
   258	                CONDITION="$CONDITION" \
   259	                MINIF2F_DIR="$MINIF2F_DIR" \
   260	                BOLTZMANN_SEED="$SEED" \
   261	                SIMULATE_ROLLBACK_AT_TX_50="$ROLLBACK_FLAG" \
   262	                MODEL_SNAPSHOT="$MODEL_SNAPSHOT" \
   263	                BUILD_SHA="$BUILD_SHA" \
   264	                SPLIT="adaptation" \
   265	                RUST_LOG=info \
   266	                "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || EXIT=$?
   267	            PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1 || true)
   268	            if [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then
   269	                ENRICHED=$(printf '%s' "$PPUT_JSON" | MODE_ENV="$MODE" SEED_ENV="$SEED" PID_ENV="$PID" python3 -c "
   270	import json, os, sys
   271	row = json.loads(sys.stdin.read())
   272	row['calibration_mode'] = os.environ['MODE_ENV']
   273	row['calibration_seed'] = int(os.environ['SEED_ENV'])
   274	row['calibration_problem_id'] = os.environ['PID_ENV']
   275	print(json.dumps(row))
   276	")
   277	                echo "$ENRICHED" >> "$OUT_FILE"
   278	                TX=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tx_count', 0))")
   279	                SOLVED_FLAG=$(echo "$ENRICHED" | python3 -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('progress', 1 if r.get('has_golden_path') else 0)))")
   280	                if [ "$SOLVED_FLAG" = "1" ]; then
   281	                    echo "SOLVED (tx=$TX)"
   282	                else
   283	                    echo "UNSOLVED (tx=$TX)"
   284	                fi
   285	            elif [ "$EXIT" -eq 124 ]; then
   286	                # Audit-fix Gemini Q7.b: timeout is a legitimate UNSOLVED outcome
   287	                # under a fixed wall-clock budget. Emit synthetic row with
   288	                # synthetic_timeout_or_crash=true disambiguator.
   289	                echo "TIMEOUT (exit=124) — emitting synthetic UNSOLVED row"
   290	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "timeout_2400s" 124
   291	            else
   292	                # Audit-fix Codex re-audit VETO 2026-04-25: any non-timeout
   293	                # non-zero exit (Rust panic 101, segfault 139, OOM 137, etc.) is
   294	                # NOT a legitimate UNSOLVED outcome. It indicates batch corruption.
   295	                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
   296	                # rather than be silently absorbed as UNSOLVED data — otherwise
   297	                # the entire calibration could complete with all-crash rows
   298	                # and produce a "valid" p_0=0 that gets frozen into Trust Root.
   299	                # No synthetic row emitted; partial calibration is forfeited.
   300	                echo "CRASH (exit=$EXIT) — ABORTING BATCH"
   301	                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
   302	                    echo ""
   303	                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
   304	                    echo "  ✗ Boot integrity check failed; investigate manifest vs filesystem state."
   305	                    echo "  ✗ Diagnostic stderr (tail):"
   306	                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
   307	                    exit 3
   308	                fi
   309	                echo ""
   310	                echo "  ✗ Evaluator crashed with exit=$EXIT (not a timeout)."
   311	                echo "  ✗ Calibration data NOT trusted; partial jsonl preserved at $OUT_FILE for diagnosis."
   312	                echo "  ✗ Last 5 stderr lines:"
   313	                tail -5 "$STDERR_LOG" | sed 's/^/    /'
   314	                exit 4
   315	            fi
   316	        done <<< "$ADAPTATION_IDS"
   317	    done
   318	done
   319	
   320	CANARY_END=$(date -Is)
   321	BATCH_END=$(date +%s)
   322	WALL_TIME=$((BATCH_END - BATCH_START))
   323	
   324	echo ""
   325	echo "╔═══════════════════════════════════════════╗"
   326	echo "║   p_0 CALIBRATION SUMMARY"
   327	echo "╠═══════════════════════════════════════════╣"
   328	echo "║ Wall time:        ${WALL_TIME}s"
   329	echo "║ Canary start:     $CANARY_START"
   330	echo "║ Canary end:       $CANARY_END"

exec
/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
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
    40	    Reads the v2 RunAggregate field `progress` (jsonl_schema.rs:96). Falls
    41	    back to legacy `has_golden_path` for pre-v2 rows. The earlier audit
    42	    found this function was reading a non-existent `progress_verified`
    43	    field — Codex Q3, fixed 2026-04-25.
    44	    """
    45	    if "progress" in row and row["progress"] is not None:
    46	        return int(row["progress"]) == 1
    47	    return bool(row.get("has_golden_path", False))
    48	
    49	
    50	PREREG_SEEDS = (31415, 2718)
    51	PREREG_N_PROBLEMS = 144
    52	
    53	
    54	def compute(
    55	    control_rows: list[dict],
    56	    treatment_rows: list[dict],
    57	    *,
    58	    expected_n_problems: int = PREREG_N_PROBLEMS,
    59	    expected_seeds: tuple[int, ...] = PREREG_SEEDS,
    60	) -> dict:
    61	    """PREREG § 5.5 estimator. Strict-complete: requires every (problem, seed)
    62	    pair present in BOTH control and treatment, exact seed set, no missing
    63	    `calibration_*` tags. Audit-fix 2026-04-25 (Codex B2 + Gemini Q3.d): the
    64	    prior silently-skip behaviour biased p_0 by dropping incomplete pairs.
    65	    """
    66	    def index(rows: list[dict], mode: str) -> dict[tuple[str, int], dict]:
    67	        out: dict[tuple[str, int], dict] = {}
    68	        for i, r in enumerate(rows):
    69	            pid = r.get("calibration_problem_id")
    70	            seed = r.get("calibration_seed")
    71	            if pid is None or seed is None:
    72	                sys.exit(
    73	                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
    74	                    "runner stamping bug; refuse to compute p_0 on incomplete data"
    75	                )
    76	            key = (pid, seed)
    77	            if key in out:
    78	                sys.exit(
    79	                    f"ERROR: {mode} duplicate row for (problem={pid}, seed={seed}) — "
    80	                    "runner emitted twice; refuse to compute p_0 on duplicated data"
    81	                )
    82	            out[key] = r
    83	        return out
    84	
    85	    c = index(control_rows, "control")
    86	    t = index(treatment_rows, "treatment")
    87	
    88	    # Strict completeness: control and treatment key sets must be identical
    89	    # AND must equal expected pre-registered (problem × seed) grid.
    90	    expected_seed_set = set(expected_seeds)
    91	    c_seeds = {seed for _, seed in c.keys()}
    92	    t_seeds = {seed for _, seed in t.keys()}
    93	    if c_seeds != expected_seed_set or t_seeds != expected_seed_set:
    94	        sys.exit(
    95	            f"ERROR: seed mismatch — expected {sorted(expected_seed_set)}; "
    96	            f"control={sorted(c_seeds)}, treatment={sorted(t_seeds)}"
    97	        )
    98	
    99	    c_problems = {pid for pid, _ in c.keys()}
   100	    t_problems = {pid for pid, _ in t.keys()}
   101	    if c_problems != t_problems:
   102	        only_c = c_problems - t_problems
   103	        only_t = t_problems - c_problems
   104	        sys.exit(
   105	            f"ERROR: problem set mismatch between control and treatment — "
   106	            f"only_in_control={sorted(only_c)[:5]}{'...' if len(only_c) > 5 else ''}, "
   107	            f"only_in_treatment={sorted(only_t)[:5]}{'...' if len(only_t) > 5 else ''}"
   108	        )
   109	
   110	    if len(c_problems) != expected_n_problems:
   111	        sys.exit(
   112	            f"ERROR: expected exactly {expected_n_problems} problems per PREREG § 5.5; "
   113	            f"got {len(c_problems)}. Refuse to compute p_0 on partial batch."
   114	        )
   115	
   116	    expected_pair_count = expected_n_problems * len(expected_seed_set)
   117	    if len(c) != expected_pair_count or len(t) != expected_pair_count:
   118	        sys.exit(
   119	            f"ERROR: expected exactly {expected_pair_count} pairs per mode; "
   120	            f"got control={len(c)}, treatment={len(t)}."
   121	        )
   122	
   123	    pairs = sorted(c.keys())
   124	
   125	    # Per-problem worst-case regression (max over seeds).
   126	    per_problem_regression: dict[str, int] = defaultdict(int)
   127	    n_pairs = 0
   128	    n_control_solved = 0
   129	    n_treatment_solved = 0
   130	    n_regression_pairs = 0
   131	    for pid, seed in pairs:
   132	        cr = c[(pid, seed)]
   133	        tr = t[(pid, seed)]
   134	        cs = solved(cr)
   135	        ts = solved(tr)
   136	        n_pairs += 1
   137	        if cs:
   138	            n_control_solved += 1
   139	        if ts:
   140	            n_treatment_solved += 1
   141	        regression = 1 if (cs and not ts) else 0
   142	        if regression:
   143	            n_regression_pairs += 1
   144	        if regression > per_problem_regression[pid]:
   145	            per_problem_regression[pid] = regression
   146	
   147	    # Denominator is the pre-registered count (audit-fix 2026-04-25 Codex
   148	    # B2): if strict-completeness above passed, len(pairs)/len(seeds) ==
   149	    # expected_n_problems by construction. Using the PREREG constant
   150	    # makes the divide-by intent unambiguous.
   151	    n_problems = expected_n_problems
   152	    assert len({pid for pid, _ in pairs}) == n_problems
   153	    p0 = sum(per_problem_regression.values()) / n_problems
   154	
   155	    return {
   156	        "n_problems": n_problems,
   157	        "n_pairs": n_pairs,
   158	        "n_control_solved": n_control_solved,
   159	        "n_treatment_solved": n_treatment_solved,
   160	        "n_regression_pairs": n_regression_pairs,
   161	        "n_regression_problems_max_seed": sum(per_problem_regression.values()),
   162	        "p0": p0,
   163	        "p0_ceiling": 0.10,
   164	        "ceiling_pass": p0 <= 0.10,
   165	    }
   166	
   167	
   168	def main() -> int:
   169	    ap = argparse.ArgumentParser()
   170	    ap.add_argument("--control", required=True, type=Path)
   171	    ap.add_argument("--treatment", required=True, type=Path)
   172	    ap.add_argument("--out-json", type=Path, default=None,
   173	                    help="Write structured result to this path")
   174	    args = ap.parse_args()
   175	
   176	    control_rows = load_jsonl(args.control)
   177	    treatment_rows = load_jsonl(args.treatment)
   178	
   179	    result = compute(control_rows, treatment_rows)
   180	    print(json.dumps(result, indent=2))
   181	
   182	    if args.out_json:
   183	        args.out_json.write_text(json.dumps(result, indent=2) + "\n")
   184	
   185	    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
   186	    h = hashlib.sha256()
   187	    for path in (args.control, args.treatment):
   188	        h.update(path.read_bytes())
   189	    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
   190	    print(f"  {h.hexdigest()}")
   191	
   192	    if not result["ceiling_pass"]:
   193	        print(
   194	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
   195	            file=sys.stderr,
   196	        )
   197	        return 2
   198	    return 0
   199	
   200	
   201	if __name__ == "__main__":
   202	    sys.exit(main())

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '150,230p'; nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '500,570p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   165	
   166	    // Audit-fix 2026-04-25 (Codex B1 + Q2 — both auditors flagged): the
   167	    // production batch runs *this* binary, not `src/main.rs`. Without a
   168	    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
   169	    // InitAI Trust Root enforcement does NOT actually fire on the calibration
   170	    // batch. Boot must happen here, at the production entry point, before
   171	    // any LLM call or jsonl emit.
   172	    //
   173	    // Repo root: CARGO_MANIFEST_DIR is `experiments/minif2f_v4`; repo root
   174	    // is two levels up. canonicalize so a deployed binary still resolves
   175	    // the genesis path it was built against.
   176	    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
   177	        .join("..")
   178	        .join("..")
   179	        .canonicalize()
   180	        .expect("evaluator: repo root resolves at build time");
   181	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
   182	        // FC3-E14 immediate-abort variant. See OBS_BOOT_FAIL_NOT_HALT.
   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
   184	    }
   185	
   186	    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
   187	    // Control binary (main branch) has no such set_var → classifier_version serializes as None.
   188	    // This makes it impossible to mistake one binary for the other in post-hoc analysis.
   189	    std::env::set_var("CLASSIFIER_VERSION", CLASSIFIER_VERSION);
   190	
   191	    let args: Vec<String> = std::env::args().collect();
   192	    if args.len() < 2 {
   193	        eprintln!("Usage: evaluator <problem_file.lean>");
   194	        eprintln!("  CONDITION env: oneshot|n1|n3 (default: oneshot)");
   195	        eprintln!("  MINIF2F_DIR, LLM_PROXY_URL, ACTIVE_MODEL env vars");
   196	        std::process::exit(1);
   197	    }
   198	
   199	    let problem_file = &args[1];
   200	    let condition = std::env::var("CONDITION").unwrap_or_else(|_| "oneshot".into());
   201	    let minif2f_dir = std::env::var("MINIF2F_DIR").unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into());
   202	    let proxy_url = std::env::var("LLM_PROXY_URL").unwrap_or_else(|_| "http://localhost:8080".into());
   203	    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-reasoner".into());
   204	
   205	    // Resolve problem path
   206	    let problem_path = resolve_problem_path(problem_file, &minif2f_dir);
   207	    let (problem_statement, theorem_name) = match load_problem(&problem_path) {
   208	        Ok(v) => v,
   209	        Err(e) => { eprintln!("Failed to load: {}", e); std::process::exit(1); }
   210	    };
   211	
   212	    let lean_path = derive_lean_path(&minif2f_dir);
   213	    info!("Problem: {} | Condition: {} | Model: {}", problem_file, condition, model);
   214	
   215	    let result = match condition.as_str() {
   216	        "oneshot" => {
   217	            run_oneshot(problem_file, &problem_statement, &theorem_name,
   218	                       &lean_path, &proxy_url, &model).await
   219	        }
   220	        // Generic nN: parse any "n<digits>" → run_swarm with N agents.
   221	        // Supports N-scaling experiment (percolation curve mapping).
   222	        c if c.starts_with('n') && c[1..].parse::<usize>().is_ok() => {
   223	            let n: usize = c[1..].parse().unwrap();
   224	            run_swarm(problem_file, &problem_statement, &theorem_name,
   225	                     &lean_path, &proxy_url, &model, n).await
   226	        }
   227	        "hybrid_v1" => {
   228	            // Mid-term audit P0-D fix 2026-04-25: hybrid_v1 was a Paper 1 era
   229	            // condition that ran run_oneshot, then on failure ran run_swarm,
   230	            // and merged via `..r2` field-spread. Codex flagged that the spread
   500	    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);
   501	    let max_transactions = 200;
   502	
   503	    // Art. IV map-reduce tick: periodic tape statistics (clock → mr → map/reduce)
   504	    let tick_interval: usize = std::env::var("TICK_INTERVAL")
   505	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   506	
   507	    // C-036 startup echo: per-agent (skill, temp) so debugging never grep-source.
   508	    let temp_ladder_on = std::env::var("TEMP_LADDER").ok().as_deref() == Some("1");
   509	    let agent_cfg: Vec<String> = (0..n_agents).map(|i| {
   510	        let s = i % agent_skills.len();
   511	        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
   512	        format!("Agent_{}:skill{}:t={:.2}", i, s, t)
   513	    }).collect();
   514	    info!("[swarm/{}] {}", condition, agent_cfg.join(" "));
   515	
   516	    // C-036 telemetry counters.
   517	    let mut tool_dist: HashMap<String, u32> = HashMap::new();
   518	    let mut omega_payload_hashes: HashSet<u64> = HashSet::new();
   519	    let mut omega_attempts: u32 = 0;
   520	    let mut zero_ticks_run: u32 = 0;
   521	    let mut zero_tick_warned = false;
   522	    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
   523	    // across all proposals (winning + failed branches). Read at terminal
   524	    // make_pput sites and stamped on the emitted jsonl row.
   525	    let mut acc = RunCostAccumulator::new();
   526	    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
   527	    // call. Opened on first tx's prompt build, closed before each return.
   528	    let mut wc = RunWallClock::new();
   529	    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
   530	    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
   531	    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
   532	    // from the tool list so agents stop wasting budget on name-match misses.
   533	    let search_cap: u32 = std::env::var("SEARCH_CAP")
   534	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   535	    let mut search_count: HashMap<String, u32> = HashMap::new();
   536	    // PPUT-CCL B7-extra (PREREG § 5.5): calibration treatment toggle.
   537	    // When enabled, every proposal at tx >= ROLLBACK_TX_THRESHOLD is
   538	    // synthetically vetoed. Constitutionally that is FC1-E18 (∏p=0 → Q_t)
   539	    // applied repeatedly; the run then exhausts at FC2-N22 HALT via
   540	    // `HaltReason::MaxTxExhausted`. We short-circuit at the threshold tx
   541	    // for efficiency — see `rollback_sim.rs` module header for why this
   542	    // is observably equivalent to running the loop to natural exhaustion.
   543	    let rollback_sim_on = minif2f_v4::rollback_sim::rollback_simulation_enabled();
   544	    if rollback_sim_on {
   545	        info!("[rollback_sim] PREREG § 5.5 calibration treatment ON \
   546	               (synthetic veto at tx >= {})", minif2f_v4::rollback_sim::ROLLBACK_TX_THRESHOLD);
   547	    }
   548	
   549	    for tx in 0..max_transactions {
   550	        // PPUT-CCL B7-extra: short-circuit guard. Constitutional anchor
   551	        // FC1-E18 + FC2-N22 (existing MaxTxExhausted variant). Stamps
   552	        // tx_count at the threshold, not at max_transactions, so jsonl
   553	        // analysis can distinguish a calibration treatment exit from a
   554	        // real natural exhaustion.
   555	        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
   556	            warn!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
   557	                   short-circuit to MaxTxExhausted exit (cost-asymmetric: skips \
   558	                   ~150 LLM calls vs honest vetoed loop; downstream PPUT analysis \
   559	                   MUST honor synthetic_short_circuit=true on this row)", tx);
   560	            wc.mark_final_accept();
   561	            let mut result = make_pput(problem_file, &condition, model,
   562	                                       false, false, start, 0, 0,
   563	                                       tx as u64, Some(tool_dist), None,
   564	                                       None, None, None,
   565	                                       Some(acc.total_run_token_count()),
   566	                                       Some(acc.failed_branch_count),
   567	                                       wc.elapsed_ms());
   568	            // B7-extra disambiguator: distinguish this calibration-treatment
   569	            // exit from a natural max-tx exhaustion in downstream PPUT
   570	            // analysis. See PputResult::synthetic_short_circuit doc-comment

exec
/bin/bash -lc "nl -ba src/boot.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
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
    78	    for (rel_path, expected) in &manifest {
    79	        let full = repo_root.join(rel_path);
    80	        let bytes = fs::read(&full).map_err(|err| TrustRootError::FileRead {
    81	            path: full.clone(),
    82	            err,
    83	        })?;
    84	        let actual = hex_lower(&Sha256::digest(&bytes));
    85	        if actual != *expected {
    86	            return Err(TrustRootError::Tampered {
    87	                path: full,
    88	                expected: expected.clone(),
    89	                actual,
    90	            });
    91	        }
    92	    }
    93	    Ok(())
    94	}
    95	
    96	/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
    97	/// the trust_root_immutability conformance battery (Phase B7) reads the
    98	/// manifest directly to assert it includes the audit-recommended PPUT
    99	/// accounting layer.
   100	///
   101	/// Parses the `[trust_root]` section of `genesis_payload.toml` into ordered
   102	/// `(path, sha256)` pairs. Hand-rolled — accepts the narrow subset we emit
   103	/// (quoted-key = quoted-value, comments, blank lines).
   104	pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
   105	    let mut in_section = false;
   106	    let mut entries = Vec::new();
   107	    for (lineno, raw) in text.lines().enumerate() {
   108	        let line = strip_comment(raw).trim();
   109	        if line.is_empty() {
   110	            continue;
   111	        }
   112	        if let Some(header) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
   113	            in_section = header.trim() == "trust_root";
   114	            continue;
   115	        }
   116	        if !in_section {
   117	            continue;
   118	        }
   119	        let (key, value) = line.split_once('=').ok_or_else(|| {
   120	            TrustRootError::GenesisParse(format!("line {}: missing '=' in [trust_root]", lineno + 1))
   121	        })?;
   122	        let key = unquote(key.trim()).ok_or_else(|| {
   123	            TrustRootError::GenesisParse(format!("line {}: key not quoted", lineno + 1))
   124	        })?;
   125	        let value = unquote(value.trim()).ok_or_else(|| {
   126	            TrustRootError::GenesisParse(format!("line {}: value not quoted", lineno + 1))
   127	        })?;
   128	        entries.push((key.to_string(), value.to_string()));
   129	    }
   130	    if entries.is_empty() {
   131	        return Err(TrustRootError::SectionMissing("trust_root"));
   132	    }
   133	    Ok(entries)
   134	}
   135	
   136	fn has_section(text: &str, name: &str) -> bool {
   137	    text.lines().any(|raw| {
   138	        let line = strip_comment(raw).trim();
   139	        line
   140	            .strip_prefix('[')
   141	            .and_then(|s| s.strip_suffix(']'))
   142	            .map(|h| h.trim() == name)
   143	            .unwrap_or(false)
   144	    })
   145	}
   146	
   147	fn strip_comment(line: &str) -> &str {
   148	    let mut in_string = false;
   149	    for (i, c) in line.char_indices() {
   150	        match c {
   151	            '"' => in_string = !in_string,
   152	            '#' if !in_string => return &line[..i],
   153	            _ => {}
   154	        }
   155	    }
   156	    line
   157	}
   158	
   159	fn unquote(s: &str) -> Option<&str> {
   160	    s.strip_prefix('"').and_then(|s| s.strip_suffix('"'))
   161	}
   162	
   163	fn hex_lower(bytes: &[u8]) -> String {
   164	    use std::fmt::Write;
   165	    let mut out = String::with_capacity(bytes.len() * 2);
   166	    for b in bytes {
   167	        write!(out, "{b:02x}").unwrap();
   168	    }
   169	    out
   170	}
   171	
   172	#[cfg(test)]
   173	mod tests {
   174	    use super::*;
   175	
   176	    fn repo_root() -> PathBuf {
   177	        // turingosv4 lib is at repo root; CARGO_MANIFEST_DIR == repo root.
   178	        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
   179	    }
   180	
   181	    #[test]
   182	    fn parse_strips_inline_comment_and_blanks() {
   183	        let toml = r#"
   184	            [pput_accounting_0]
   185	            schema_version = "1.0"
   186	
   187	            [trust_root]
   188	            # leading comment
   189	            "a/b.rs" = "deadbeef"   # trailing comment
   190	            "c/d.md" = "cafebabe"
   191	        "#;
   192	        let entries = parse_trust_root_section(toml).unwrap();
   193	        assert_eq!(
   194	            entries,
   195	            vec![
   196	                ("a/b.rs".to_string(), "deadbeef".to_string()),
   197	                ("c/d.md".to_string(), "cafebabe".to_string()),
   198	            ]
   199	        );
   200	    }
   201	
   202	    #[test]
   203	    fn parse_errors_on_unquoted_key() {
   204	        let toml = "[trust_root]\nfoo = \"deadbeef\"\n";
   205	        assert!(matches!(
   206	            parse_trust_root_section(toml),
   207	            Err(TrustRootError::GenesisParse(_))
   208	        ));
   209	    }
   210	
   211	    #[test]
   212	    fn parse_errors_when_section_missing() {
   213	        let toml = "[pput_accounting_0]\nschema_version = \"1.0\"\n";
   214	        assert!(matches!(
   215	            parse_trust_root_section(toml),
   216	            Err(TrustRootError::SectionMissing("trust_root"))
   217	        ));
   218	    }
   219	
   220	    #[test]

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/tests/trust_root_immutability.rs | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL Phase B7 — Trust Root immutability (PREREG § 1.8 + § 7 Gate B).
     2	//
     3	// Boot computes SHA-256 of every Trust Root file at process start and
     4	// compares against the genesis_payload.toml [trust_root] manifest. Any
     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
     6	//
     7	// Trust Root manifest (PREREG § 1.8 + audit additions through 2026-04-25):
     8	//   src/main.rs                                       (audit-fix Q2.b)
     9	//   src/kernel.rs
    10	//   src/wal.rs
    11	//   src/bus.rs
    12	//   src/drivers/llm_http.rs                           (B2-B4 audit add)
    13	//   src/sdk/prompt_guard.rs                           (B6 add)
    14	//   Cargo.lock                                        (audit-fix Q2.e)
    15	//   experiments/minif2f_v4/src/lean4_oracle.rs
    16	//   experiments/minif2f_v4/src/cost_aggregator.rs     (B2)
    17	//   experiments/minif2f_v4/src/wall_clock.rs          (B3)
    18	//   experiments/minif2f_v4/src/post_hoc_verifier.rs   (B4)
    19	//   experiments/minif2f_v4/src/jsonl_schema.rs        (B1)
    20	//   experiments/minif2f_v4/src/rollback_sim.rs        (B7-extra)
    21	//   experiments/minif2f_v4/src/bin/evaluator.rs       (the wiring)
    22	//   constitution.md
    23	//   handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
    24	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
    25	//   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
    26	//   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
    27	//   cases/MANIFEST.sha256                             (proxy for cases/*.yaml)
    28	
    29	use std::fs;
    30	use std::path::{Path, PathBuf};
    31	use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
    32	
    33	fn repo_root() -> PathBuf {
    34	    // CARGO_MANIFEST_DIR for this test crate is experiments/minif2f_v4 — repo
    35	    // root is two levels up.
    36	    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    37	        .join("..")
    38	        .join("..")
    39	        .canonicalize()
    40	        .expect("repo root resolves")
    41	}
    42	
    43	fn read_genesis() -> String {
    44	    fs::read_to_string(repo_root().join("genesis_payload.toml")).expect("genesis exists")
    45	}
    46	
    47	#[test]
    48	fn test_trust_root_immutable_at_boot() {
    49	    // Cold-start with intact files: Boot computes SHA-256s, all match
    50	    // genesis manifest, process continues. No abort.
    51	    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
    52	}
    53	
    54	#[test]
    55	fn test_trust_root_simulated_write_aborts() {
    56	    // Simulated tampering: build a self-contained fake-repo in a tempdir
    57	    // with a single Trust Root entry whose recorded hash does not match
    58	    // the file content; assert verify_trust_root returns Tampered.
    59	    let tmp = make_tempdir("trust_root_tamper");
    60	    let zero_hash = "0".repeat(64);
    61	    let genesis = format!(
    62	        "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{zero_hash}\"\n"
    63	    );
    64	    fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
    65	    fs::write(tmp.join("only.txt"), "tampered content").unwrap();
    66	
    67	    match verify_trust_root(&tmp) {
    68	        Err(TrustRootError::Tampered { path, expected, actual }) => {
    69	            assert!(path.ends_with("only.txt"));
    70	            assert_eq!(expected, zero_hash);
    71	            assert_ne!(actual, expected);
    72	        }
    73	        other => panic!("expected Tampered, got {other:?}"),
    74	    }
    75	}
    76	
    77	#[test]
    78	fn test_trust_root_manifest_includes_b2_b4_files() {
    79	    // Mid-term audit recommendation: B2 (cost_aggregator), B3 (wall_clock),
    80	    // B4 (post_hoc_verifier), B1 (jsonl_schema), evaluator.rs, llm_http.rs
    81	    // MUST be in the Trust Root manifest. B6 added prompt_guard.rs.
    82	    let entries = parse_trust_root_section(&read_genesis()).expect("parse trust_root");
    83	    let keys: Vec<&str> = entries.iter().map(|(k, _)| k.as_str()).collect();
    84	
    85	    let required = [
    86	        // PREREG § 1.8 base
    87	        "src/kernel.rs",
    88	        "src/wal.rs",
    89	        "src/bus.rs",
    90	        "experiments/minif2f_v4/src/lean4_oracle.rs",
    91	        "constitution.md",
    92	        "cases/MANIFEST.sha256",
    93	        "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json",
    94	        "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
    95	        // Mid-term audit accounting layer
    96	        "src/drivers/llm_http.rs",
    97	        "experiments/minif2f_v4/src/cost_aggregator.rs",
    98	        "experiments/minif2f_v4/src/wall_clock.rs",
    99	        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
   100	        "experiments/minif2f_v4/src/jsonl_schema.rs",
   101	        "experiments/minif2f_v4/src/bin/evaluator.rs",
   102	        // B6 add
   103	        "src/sdk/prompt_guard.rs",
   104	        // B7-extra add
   105	        "experiments/minif2f_v4/src/rollback_sim.rs",
   106	        // 2026-04-25 dual-audit fixes
   107	        "src/main.rs",
   108	        "Cargo.lock",
   109	        "handover/preregistration/scripts/run_p0_calibration.sh",
   110	        "handover/preregistration/scripts/compute_p0.py",
   111	    ];
   112	
   113	    for path in required {
   114	        assert!(
   115	            keys.contains(&path),
   116	            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
   117	        );
   118	    }
   119	}
   120	
   121	#[test]
   122	fn test_pput_accounting_0_section_present() {
   123	    // genesis_payload.toml must contain [pput_accounting_0] with the PREREG
   124	    // § 1.8 keys.
   125	    let genesis = read_genesis();
   126	    let body = extract_section(&genesis, "pput_accounting_0").expect("section present");
   127	    let body = body.as_str();
   128	
   129	    let required_keys = [
   130	        "schema_version",
   131	        "progress_definition",
   132	        "cost_definition",
   133	        "time_definition",
   134	        "verified_predicate",
   135	        "heldout_sealed_hash",
   136	        "source_pool_sha256",
   137	        "baseline_regression_rate",
   138	        "baseline_regression_jsonl_sha256",
   139	        "k_max",
   140	        "n_max",
   141	    ];
   142	    for key in required_keys {
   143	        let needle = format!("{key} =");
   144	        assert!(
   145	            body.contains(&needle),
   146	            "[pput_accounting_0] missing key: {key}"
   147	        );
   148	    }
   149	
   150	    // Frozen invariants from PREREG § 1.8: heldout sealed hash, k_max, n_max.
   151	    assert!(body.contains(
   152	        "\"51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b\""
   153	    ), "heldout_sealed_hash diverges from PREREG § 2.3");
   154	    assert!(body.contains("k_max = 10"), "k_max must be 10 per PREREG");
   155	    assert!(body.contains("n_max = 34"), "n_max must be 34 per PREREG");
   156	}
   157	
   158	// --- helpers ---
   159	
   160	fn extract_section(text: &str, name: &str) -> Option<String> {
   161	    // Line-anchored scan: skip commented-out section headers (e.g. inside
   162	    // the file's leading docstring) and only match real headers in column 0.
   163	    let mut in_section = false;
   164	    let mut body = String::new();
   165	    let target = format!("[{name}]");
   166	    for line in text.lines() {
   167	        let trimmed = line.trim_start();
   168	        if trimmed.starts_with('#') {
   169	            continue;
   170	        }
   171	        if trimmed.starts_with('[') && trimmed.ends_with(']') {
   172	            in_section = trimmed == target;
   173	            continue;
   174	        }
   175	        if in_section {
   176	            body.push_str(line);
   177	            body.push('\n');
   178	        }
   179	    }
   180	    if body.is_empty() {
   181	        None
   182	    } else {
   183	        Some(body)
   184	    }
   185	}
   186	
   187	fn make_tempdir(tag: &str) -> PathBuf {
   188	    let pid = std::process::id();
   189	    let nano = std::time::SystemTime::now()
   190	        .duration_since(std::time::UNIX_EPOCH)
   191	        .unwrap()
   192	        .as_nanos();
   193	    let dir: PathBuf = std::env::temp_dir().join(format!("turingosv4-{tag}-{pid}-{nano}"));
   194	    fs::create_dir_all(&dir).unwrap();
   195	    let _: &Path = dir.as_path();
   196	    dir
   197	}

codex
The first pass shows the round-2 crash branch does now separate exit `124` from other non-zero exits, but I’m checking whether the preflight itself has any silent-success edge cases before deciding verdict.
exec
/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '330,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   330	echo "║ Canary end:       $CANARY_END"
   331	echo "║ MODEL_SNAPSHOT:   $MODEL_SNAPSHOT"
   332	echo "║ BUILD_SHA:        $BUILD_SHA"
   333	echo "║ Control jsonl:    ${OUT_PREFIX}_control.jsonl"
   334	echo "║ Treatment jsonl:  ${OUT_PREFIX}_treatment.jsonl"
   335	echo "╚═══════════════════════════════════════════╝"
   336	echo ""
   337	
   338	# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
   339	# = ceiling abort). For smoke modes we skip the estimator (sample size too
   340	# small to be meaningful) and just print the diagnostic snippet.
   341	if [ "$SMOKE" -eq 1 ] || [ "$SMOKE_HARD" -eq 1 ]; then
   342	    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
   343	    echo "Diagnostic: head 1 row from each jsonl"
   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
   346	    exit 0
   347	fi
   348	
   349	# Full batch: estimator MUST run, exit code MUST propagate.
   350	echo "[$(date -Is)] Running p_0 estimator (strict-complete mode)..."
   351	P0_JSON="${OUT_PREFIX}_p0_result.json"
   352	set +e
   353	python3 "$SCRIPT_DIR/compute_p0.py" \
   354	    --control "${OUT_PREFIX}_control.jsonl" \
   355	    --treatment "${OUT_PREFIX}_treatment.jsonl" \
   356	    --out-json "$P0_JSON"
   357	P0_EXIT=$?
   358	set -e
   359	
   360	if [ "$P0_EXIT" -eq 0 ]; then
   361	    echo ""
   362	    echo "✓ p_0 PASSED ceiling. Result: $P0_JSON"
   363	    echo "  Next: ArchitectAI updates genesis_payload.toml [pput_accounting_0]"
   364	    echo "        + Trust Root manifest entry for the calibration jsonl."
   365	elif [ "$P0_EXIT" -eq 2 ]; then
   366	    echo ""
   367	    echo "✗ p_0 EXCEEDS ceiling (>0.10) — PREREG § 5.5 ABORT."
   368	    echo "  Calibration result NOT frozen into genesis_payload.toml."
   369	    echo "  Action: redesign rollback simulation (per PREREG § 5.5), redo."
   370	    exit 2
   371	else
   372	    echo ""
   373	    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
   374	    echo "  Calibration result NOT frozen. Investigate before retry."
   375	    exit "$P0_EXIT"
   376	fi

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '570,610p'; nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '1,180p'; nl -ba experiments/minif2f_v4/src/rollback_sim.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   570	            // analysis. See PputResult::synthetic_short_circuit doc-comment
   571	            // for the cost-asymmetry note.
   572	            result.synthetic_short_circuit = Some(true);
   573	            return result;
   574	        }
   575	
   576	        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
   577	        // bracket at the top of the FIRST tx (before chain/skill/board build
   578	        // and before build_agent_prompt). Idempotent — only the first tx's
   579	        // call sticks; subsequent calls no-op. PREREG § 5 / plan B3 define
   580	        // T_i as "first agent prompt construction"; this is the earliest
   581	        // moment the agent begins constructing its prompt.
   582	        wc.mark_first_read();
   583	
   584	        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)
   585	        if tick_interval > 0 && tx > 0 && tx % tick_interval == 0 {
   586	            let tape_len = bus.kernel.tape.time_arrow().len();
   587	            let market_count = bus.kernel.markets.len();
   588	            let ticker = bus.kernel.market_ticker(5);
   589	            let top_prices: Vec<String> = ticker.iter()
   590	                .map(|(id, p)| format!("{}:{:.0}%", id, p * 100.0))
   591	                .collect();
   592	            info!("[tick@tx{}] tape={} markets={} top={}", tx, tape_len, market_count,
   593	                top_prices.join(", "));
   594	            // Phase 6-emergent: refresh shared team board from facts only.
   595	            // Per-agent cumulative balance + recent tape-node authorship counts
   596	            // + top market prices. No instructions, no "should" — just state.
   597	            if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
   598	                let agents_sorted: Vec<String> = agent_ids.clone();
   599	                let mut author_counts: std::collections::HashMap<String, u32> =
   600	                    std::collections::HashMap::new();
   601	                for nid in bus.kernel.tape.time_arrow() {
   602	                    if let Some(n) = bus.kernel.tape.get(nid) {
   603	                        *author_counts.entry(n.author.clone()).or_insert(0) += 1;
   604	                    }
   605	                }
   606	                let wallet_balances: std::collections::HashMap<String, f64> =
   607	                    bus.tools.iter()
   608	                        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
   609	                        .map(|w| w.balances.clone())
   610	                        .unwrap_or_default();
     1	// PPUT-CCL JSONL schema v2 — proposal-level + run-level records.
     2	//
     3	// Authoritative spec:
     4	//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B1
     5	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (definitions)
     6	//
     7	// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
     8	// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
     9	// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
    10	// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
    11	// by this commit; downstream tooling is the upgrade boundary.
    12	//
    13	// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
    14	// invariant. B2/B3/B4 wire the new fields into evaluator emission paths.
    15	
    16	use serde::{Deserialize, Serialize};
    17	
    18	pub const SCHEMA_VERSION_V2: &str = "v2.0";
    19	
    20	/// Per-proposal row (one per LLM call / append / complete attempt).
    21	///
    22	/// Currently no evaluator emit path produces these — B2 (cost aggregator) and
    23	/// B3 (wall-time) will add the emit sites. This struct is the contract.
    24	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    25	pub struct ProposalRow {
    26	    pub run_id: String,
    27	    pub problem_id: String,
    28	    pub agent_id: String,
    29	    pub role: String,
    30	    pub branch_id: String,
    31	    pub proposal_hash: String,
    32	    pub accepted: bool,
    33	
    34	    /// "adaptation" | "meta_validation" | "heldout"
    35	    pub split: String,
    36	    pub schema_version: String,
    37	    /// SHA-256 of input prompt (retrieval-equivalence audit).
    38	    pub context_hash: String,
    39	    /// Runtime predicate accept = 1, reject = 0.
    40	    pub predicate_result: i32,
    41	    /// Lean post-hoc verify: 1 / 0 / null = not yet checked.
    42	    #[serde(skip_serializing_if = "Option::is_none")]
    43	    pub ground_truth_result: Option<i32>,
    44	    #[serde(skip_serializing_if = "Option::is_none")]
    45	    pub lean_error_category: Option<String>,
    46	    #[serde(skip_serializing_if = "Option::is_none")]
    47	    pub raw_error_hash: Option<String>,
    48	    /// Hash of Q^world snapshot to roll back to (PREREG ArtifactState).
    49	    #[serde(skip_serializing_if = "Option::is_none")]
    50	    pub rollback_to: Option<String>,
    51	
    52	    pub prompt_tokens: u64,
    53	    pub completion_tokens: u64,
    54	    /// Length of all tool stdout summed (B2).
    55	    pub tool_tokens: u64,
    56	    /// = prompt + completion + tool.
    57	    pub total_tokens: u64,
    58	    pub wall_time_ms: u64,
    59	    /// ISO 8601 UTC.
    60	    pub start_time: String,
    61	    pub end_time: String,
    62	    pub ast_depth: u32,
    63	    pub peer_agents_in_branch: Vec<String>,
    64	    /// SHA-256 of concatenated tool stdout.
    65	    #[serde(skip_serializing_if = "Option::is_none")]
    66	    pub tool_stdout_hash: Option<String>,
    67	    pub is_on_golden_path: bool,
    68	    #[serde(skip_serializing_if = "Option::is_none")]
    69	    pub golden_path_id: Option<String>,
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
   116	    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous".
   117	    pub mode: String,
   118	}
   119	
   120	impl RunAggregate {
   121	    /// Compute pput_verified per PREREG § 5:
   122	    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
   123	    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
   124	    /// (synthetic / degenerate runs; real runs always have positive cost+time).
   125	    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
   126	        if progress == 0 || c_i == 0 || t_i_ms == 0 {
   127	            return 0.0;
   128	        }
   129	        let denom = (c_i as f64) * (t_i_ms as f64) / 1000.0;
   130	        (progress as f64) / denom
   131	    }
   132	
   133	    /// Display unit: 10^6 × pput_verified.
   134	    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
   135	        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
   136	    }
   137	}
   138	
   139	/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
   140	/// evaluator before this commit (Paper 1 era, e.g.
   141	/// `discarded_12way_run_2026-04-24/E1v2_Abl_*.jsonl`).
   142	///
   143	/// All v3-era extension fields (reputation_at_end, halt_reason, gp_*) are
   144	/// captured by `extra` so a legacy line round-trips losslessly through
   145	/// serde_json::Value.
   146	#[derive(Debug, Clone, Deserialize, Serialize)]
   147	pub struct LegacyRunAggregate {
   148	    pub problem: String,
   149	    pub condition: String,
   150	    pub model: String,
   151	    pub has_golden_path: bool,
   152	    pub time_secs: f64,
   153	    pub pput: f64,
   154	    pub gp_token_count: u64,
   155	    pub gp_node_count: usize,
   156	    pub tx_count: u64,
   157	    /// Catch-all for v3.x optional fields (reputation_at_end, halt_reason,
   158	    /// gp_payload, gp_path, gp_proof_file, classifier_version, build_sha, ...).
   159	    #[serde(flatten)]
   160	    pub extra: serde_json::Map<String, serde_json::Value>,
   161	}
   162	
   163	/// Discriminated record for backward-compatible reading.
   164	#[derive(Debug)]
   165	pub enum RunRecord {
   166	    V2(RunAggregate),
   167	    Legacy(LegacyRunAggregate),
   168	}
   169	
   170	impl RunRecord {
   171	    /// Parse one jsonl line. v2 if `schema_version` present, else legacy.
   172	    /// Returns the raw serde error for genuinely malformed input.
   173	    pub fn from_json(line: &str) -> Result<Self, serde_json::Error> {
   174	        let v: serde_json::Value = serde_json::from_str(line)?;
   175	        let is_v2 = v.get("schema_version")
   176	            .and_then(|s| s.as_str())
   177	            .map(|s| s.starts_with("v2"))
   178	            .unwrap_or(false);
   179	        if is_v2 {
   180	            Ok(RunRecord::V2(serde_json::from_value(v)?))
     1	// PPUT-CCL Phase B B7-extra — synthetic rollback simulation.
     2	//
     3	// Constitutional anchor (TRACE_MATRIX_v1 § 7.2, status ⚠️ partial after
     4	// 2026-04-25 dual-audit re-review): the `--simulate-rollback-at-tx-50`
     5	// toggle (PREREG § 5.5) MAPS TO the conjunction of **FC1-E18** (∏p=0 →
     6	// Q_t preservation) repeated for tx 50..max_transactions and the
     7	// resulting natural **FC2-N22 HALT** with `HaltReason::MaxTxExhausted`.
     8	// No new HaltReason variant is introduced and no new constitutional
     9	// surface is created.
    10	//
    11	// **Equivalence is narrow** (audit-fix 2026-04-25, Codex Q1 + Gemini
    12	// Q1.a): the short-circuit at tx == threshold is equivalent to the
    13	// 150-tx vetoed loop on a *single* observable — the final
    14	// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
    15	// the PREREG § 5.5 estimator. It is **NOT** equivalent on:
    16	//   - cost accumulator C_i (skips ~150 LLM calls × ~250 tokens each)
    17	//   - wall-clock T_i (skips ~150 × per-tx wall-clock contribution)
    18	//   - WAL ledger event sequence (skips ~150 Append/Reject events)
    19	//   - bus-level predicate traversal (the synthetic ∏p=0 is asserted at
    20	//     evaluator layer, not registered with bus.evaluate_predicates)
    21	//   - `tx_count` field (stamped at threshold = 50, not at max_transactions)
    22	//
    23	// Consumers that touch any non-(problem, seed, solved) field on rows
    24	// where `synthetic_short_circuit == Some(true)` MUST disclaim the
    25	// non-equivalence. `compute_p0.py` honors this by reading only the
    26	// `progress` field and the PREREG-frozen seed/problem grid.
    27	//
    28	// Threat model: the threshold is fixed at 50 per PREREG § 5.5 frozen
    29	// spec. The env var `SIMULATE_ROLLBACK_AT_TX_50` is a binary toggle
    30	// (`"1"` to enable). The threshold is intentionally not exposed as a
    31	// runtime parameter — pre-registration discipline (C-070) requires that
    32	// what we calibrate is exactly what is committed in genesis_payload.toml.
    33	
    34	/// PREREG § 5.5: the synthetic rollback fires at this transaction index
    35	/// in the swarm loop. Frozen — must match the value committed in the
    36	/// pre-registration hash chain.
    37	pub const ROLLBACK_TX_THRESHOLD: u64 = 50;
    38	
    39	/// Env var name read by the evaluator. `"1"` enables the toggle; any
    40	/// other value (or absence) is "off".
    41	pub const ROLLBACK_ENV_VAR: &str = "SIMULATE_ROLLBACK_AT_TX_50";
    42	
    43	/// True iff the calibration treatment toggle is enabled in the current
    44	/// process environment.
    45	pub fn rollback_simulation_enabled() -> bool {
    46	    matches!(std::env::var(ROLLBACK_ENV_VAR).as_deref(), Ok("1"))
    47	}
    48	
    49	/// True iff the swarm loop should short-circuit at this `tx` index. The
    50	/// short-circuit is constitutionally equivalent to "synthetic ∏p=0 from
    51	/// here, naturally exhaust at `max_transactions`" — see module header.
    52	///
    53	/// `enabled` is a parameter (not read from env) so unit tests can drive
    54	/// the predicate without process-global state.
    55	pub fn should_simulate_rollback(tx: u64, enabled: bool) -> bool {
    56	    enabled && tx == ROLLBACK_TX_THRESHOLD
    57	}
    58	
    59	#[cfg(test)]
    60	mod tests {
    61	    use super::*;
    62	
    63	    #[test]
    64	    fn fires_at_threshold_when_enabled() {
    65	        assert!(should_simulate_rollback(50, true));
    66	    }
    67	
    68	    #[test]
    69	    fn does_not_fire_before_threshold() {
    70	        for tx in [0_u64, 1, 25, 49] {
    71	            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
    72	        }
    73	    }
    74	
    75	    #[test]
    76	    fn does_not_fire_after_threshold() {
    77	        // Constitutional reading: at tx > 50, the synthetic ∏p has already
    78	        // begun returning Reject; the loop continues but accumulates no
    79	        // progress. Short-circuit fires exactly once at tx == threshold,
    80	        // not on every tx after.
    81	        for tx in [51_u64, 60, 100, 199] {
    82	            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
    83	        }
    84	    }
    85	
    86	    #[test]
    87	    fn never_fires_when_disabled() {
    88	        for tx in [0_u64, 49, 50, 51, 199] {
    89	            assert!(!should_simulate_rollback(tx, false), "tx={tx}");
    90	        }
    91	    }
    92	
    93	    #[test]
    94	    fn threshold_constant_matches_prereg() {
    95	        // PREREG § 5.5 freezes the threshold at 50. If this assertion ever
    96	        // fails, the codebase has drifted from the pre-registration hash
    97	        // chain — recompute Trust Root and dual-audit before continuing.
    98	        assert_eq!(ROLLBACK_TX_THRESHOLD, 50);
    99	    }
   100	
   101	    #[test]
   102	    fn env_var_name_matches_prereg() {
   103	        // PREREG § 5.5 names the toggle `--simulate-rollback-at-tx-50`;
   104	        // the env-var equivalent (the v4 evaluator does not use clap)
   105	        // mirrors that name uppercased + underscored.
   106	        assert_eq!(ROLLBACK_ENV_VAR, "SIMULATE_ROLLBACK_AT_TX_50");
   107	    }
   108	}

exec
/bin/bash -lc "nl -ba genesis_payload.toml | sed -n '1,130p'" in /home/zephryj/projects/turingosv4
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
    41	#   2026-04-25 dual-audit fixes (4):
    42	#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
    43	#       in manifest; comment-out = silent bypass)
    44	#     Cargo.lock (audit Q2.e VETO — supply-chain dep-version swap defense)
    45	#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
    46	#       DO-178C tool qualification — runner is a frozen production tool)
    47	#     handover/preregistration/scripts/compute_p0.py (same — estimator is
    48	#       a frozen production tool)
    49	#
    50	# Total: 20 files. genesis_payload.toml itself is conceptually frozen but
    51	# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
    52	# values are the semantic anchor.
    53	#
    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
    55	
    56	[pput_accounting_0]
    57	schema_version = "1.0"
    58	progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
    59	cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
    60	time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
    61	verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
    62	heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
    63	source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
    64	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
    65	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
    66	k_max = 10
    67	n_max = 34
    68	
    69	[trust_root]
    70	"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
    71	"Cargo.lock" = "577446e8fe11e91bc8751bf13e5ddca6c5faa64d3309b878768c550d3e6feb98"
    72	"handover/preregistration/scripts/run_p0_calibration.sh" = "92701c2876a69968a4f570a67d39c56e15da0a45d44720d4fe1b6174ecdbd821"
    73	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
    74	"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
    75	"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
    76	"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
    77	"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
    78	"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
    79	"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
    80	"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
    81	"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
    82	"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
    83	"experiments/minif2f_v4/src/jsonl_schema.rs" = "7647a2ea842f4394d55e1ce85032642c18ba61f962bd2c5c8714c618e743777c"
    84	"experiments/minif2f_v4/src/rollback_sim.rs" = "9ccdf177813452bb83221bd780f2f7f10da619117aca8e44d3ecb355505fe561"
    85	"experiments/minif2f_v4/src/bin/evaluator.rs" = "2caf3f66ce26a0ef8233e21f6c730a34e369d308671c195c5abcbe7c46034665"
    86	"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
    87	"cases/MANIFEST.sha256" = "3759631afae97c631be9a4038479b8d3681af25885f9758c32ad88c22f17fc3f"
    88	"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
    89	"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"

exec
/bin/bash -lc 'rg -n "verify_trust_root|TRUST_ROOT_TAMPERED|evaluator boot|preflight|timeout 30|synthetic_timeout_or_crash|problem_file_missing|exit 124" -S .' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
./src/main.rs:11:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./src/main.rs:14:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./src/main.rs:15:        panic!("TRUST_ROOT_TAMPERED: {e}");
./experiments/minif2f_v4/tests/trust_root_immutability.rs:5:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./experiments/minif2f_v4/tests/trust_root_immutability.rs:31:use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
./experiments/minif2f_v4/tests/trust_root_immutability.rs:51:    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
./experiments/minif2f_v4/tests/trust_root_immutability.rs:58:    // the file content; assert verify_trust_root returns Tampered.
./experiments/minif2f_v4/tests/trust_root_immutability.rs:67:    match verify_trust_root(&tmp) {
./experiments/minif2f_v4/run_batch.sh:82:# C-012 preflight: oracle health check before burning API budget.
./experiments/minif2f_v4/run_batch.sh:84:echo "Oracle preflight..."
./experiments/minif2f_v4/run_batch.sh:100:echo "Oracle preflight OK."
./experiments/minif2f_v4/src/bin/evaluator.rs:168:    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
./experiments/minif2f_v4/src/bin/evaluator.rs:181:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./experiments/minif2f_v4/src/bin/evaluator.rs:183:        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./experiments/minif2f_v4/src/wall_clock.rs:14://   1. Excludes evaluator-side preflight (kernel construction, tool mounting,
./src/boot.rs:15:// `TRUST_ROOT_TAMPERED`.
./src/boot.rs:32:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./src/boot.rs:50:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./src/boot.rs:53:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./src/boot.rs:71:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./src/boot.rs:96:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./src/boot.rs:221:    fn verify_trust_root_passes_on_intact_repo() {
./src/boot.rs:222:        verify_trust_root(&repo_root()).expect("intact repo verifies");
./src/boot.rs:237:    fn verify_trust_root_detects_tamper_in_tempdir() {
./src/boot.rs:242:        match verify_trust_root(&tmp).expect_err("tamper must be detected") {
./src/boot.rs:253:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
./src/boot.rs:258:        verify_trust_root(&tmp).expect("matching hash verifies");
./genesis_payload.toml:11:#                         (`turingosv4::boot::verify_trust_root`) recomputes
./genesis_payload.toml:12:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./genesis_payload.toml:42:#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
./genesis_payload.toml:54:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:32:-   **(RQ1.c) Does evaluator.rs's call to `verify_trust_root` happen BEFORE any other state-modifying action?**
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:35:        2.  `verify_trust_root(&repo_root)` block (lines 155-160)
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:47:    -   The fix is correct and effective. The schema includes the `synthetic_timeout_or_crash: true` flag, which is crucial for downstream analysis to distinguish these cases from natural UNSOLVED runs.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:92:    -   The `write_single_entry_repo` helper in `boot.rs` (line 211) consolidates file I/O setup. The core logic of the tests—providing either a matching or a mismatching hash—remains within the respective test functions (`verify_trust_root_detects_tamper_in_tempdir` and `verify_trust_root_passes_when_hash_matches_in_tempdir`). No test coverage has been dropped. This is a safe and clean refactoring.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:31:- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:32:- **Round 3 (this audit)**: determine whether the round-2 fix resolves your previous VETO AND whether it introduces NEW defects. Specifically scrutinize the new crash-vs-timeout discrimination logic and the boot preflight.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:36:   - exit 124 (timeout) → synthetic UNSOLVED row (only this case emits synthetic data)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:37:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:38:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:44:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:62:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:69:| B1: evaluator doesn't call verify_trust_root | **VETO** | 15b87fb: evaluator.rs:163-183 — `verify_trust_root` called at top of `async fn main`; panic on Err. Negative-test verified end-to-end. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:80:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:88:`evaluator.rs:163-183` calls `turingosv4::boot::verify_trust_root(&repo_root)` at the top of `async fn main`. The repo_root is resolved from `env!("CARGO_MANIFEST_DIR")` (= `experiments/minif2f_v4`) joined with `..` × 2 + canonicalize.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:92:- (RQ1.b) The verify_trust_root call is BEFORE env_logger::init(). Does that matter for diagnostics? (Panic message goes to stderr regardless.)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:93:- (RQ1.c) Could a future evaluator addition place ANY logic before verify_trust_root that would compromise the order? Defensive structure?
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:94:- (RQ1.d) The negative-test (tampered manifest → panic) was verified by the implementer. Is there a corresponding automated test (cargo test) that catches accidental removal of the verify_trust_root call? Do you require one?
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:100:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:111:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:115:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:275:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:292:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:307:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:308:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:309:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:310:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:313:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:331:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:356:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:481:    fn verify_trust_root_passes_on_intact_repo() {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:482:        verify_trust_root(&repo_root()).expect("intact repo verifies");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:497:    fn verify_trust_root_detects_tamper_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:502:        match verify_trust_root(&tmp).expect_err("tamper must be detected") {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:513:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:518:        verify_trust_root(&tmp).expect("matching hash verifies");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:549:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:552:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:553:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:573:#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:574:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:604:#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:616:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:772:# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:773:echo "[$(date -Is)] Oracle preflight..."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:789:echo "Oracle preflight OK."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:791:# Audit-fix 2026-04-25 round-2 (Codex re-audit VETO): evaluator boot preflight.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:793:# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:795:# preflight surfaces the failure with the clean diagnostic and zero wasted
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:797:echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:798:PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:799:if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:800:    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:813:echo "Evaluator boot preflight OK."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:832:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:841:    # USED ONLY for legitimate timeout (exit 124). Crash paths now ABORT
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:856:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:905:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:945:                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:952:                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:958:                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:960:                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:963:                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1001:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1002:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1245:## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1281:+    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1294:+    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1296:+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1382:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1392:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1445:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1453:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1456:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1481:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1688:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1714:use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1734:    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1741:    // the file content; assert verify_trust_root returns Tampered.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1750:    match verify_trust_root(&tmp) {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2059:   115	# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2060:   116	echo "[$(date -Is)] Oracle preflight..."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2076:   132	echo "Oracle preflight OK."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2078:   134	# Audit-fix 2026-04-25 round-2 (Codex re-audit VETO): evaluator boot preflight.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2080:   136	# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2082:   138	# preflight surfaces the failure with the clean diagnostic and zero wasted
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2084:   140	echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2085:   141	PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2086:   142	if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2087:   143	    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2100:   156	echo "Evaluator boot preflight OK."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2119:   175	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2128:   184	    # USED ONLY for legitimate timeout (exit 124). Crash paths now ABORT
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2143:   199	    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2192:   248	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2232:   288	                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2239:   295	                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2245:   301	                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2247:   303	                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2250:   306	                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2503:   168	    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2516:   181	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2518:   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2655:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2672:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2687:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2688:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2689:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2690:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2693:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2711:    71	pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2736:    96	/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2869:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2895:    31	use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2915:    51	    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2922:    58	    // the file content; assert verify_trust_root returns Tampered.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2931:    67	    match verify_trust_root(&tmp) {
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3064:The first pass shows the round-2 crash branch does now separate exit `124` from other non-zero exits, but I’m checking whether the preflight itself has any silent-success edge cases before deciding verdict.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3082:   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3083:   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3462:    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3463:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3493:    42	#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3505:    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/run_codex_b7_extra_round3_audit.sh:20:- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
./handover/audits/run_codex_b7_extra_round3_audit.sh:21:- **Round 3 (this audit)**: determine whether the round-2 fix resolves your previous VETO AND whether it introduces NEW defects. Specifically scrutinize the new crash-vs-timeout discrimination logic and the boot preflight.
./handover/audits/run_codex_b7_extra_round3_audit.sh:25:   - exit 124 (timeout) → synthetic UNSOLVED row (only this case emits synthetic data)
./handover/audits/run_codex_b7_extra_round3_audit.sh:26:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/run_codex_b7_extra_round3_audit.sh:27:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/run_codex_b7_extra_round3_audit.sh:33:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/run_codex_b7_extra_round3_audit.sh:51:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_codex_b7_extra_round3_audit.sh:58:| B1: evaluator doesn't call verify_trust_root | **VETO** | 15b87fb: evaluator.rs:163-183 — `verify_trust_root` called at top of `async fn main`; panic on Err. Negative-test verified end-to-end. |
./handover/audits/run_codex_b7_extra_round3_audit.sh:69:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_codex_b7_extra_round3_audit.sh:77:`evaluator.rs:163-183` calls `turingosv4::boot::verify_trust_root(&repo_root)` at the top of `async fn main`. The repo_root is resolved from `env!("CARGO_MANIFEST_DIR")` (= `experiments/minif2f_v4`) joined with `..` × 2 + canonicalize.
./handover/audits/run_codex_b7_extra_round3_audit.sh:81:- (RQ1.b) The verify_trust_root call is BEFORE env_logger::init(). Does that matter for diagnostics? (Panic message goes to stderr regardless.)
./handover/audits/run_codex_b7_extra_round3_audit.sh:82:- (RQ1.c) Could a future evaluator addition place ANY logic before verify_trust_root that would compromise the order? Defensive structure?
./handover/audits/run_codex_b7_extra_round3_audit.sh:83:- (RQ1.d) The negative-test (tampered manifest → panic) was verified by the implementer. Is there a corresponding automated test (cargo test) that catches accidental removal of the verify_trust_root call? Do you require one?
./handover/audits/run_codex_b7_extra_round3_audit.sh:89:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/run_codex_b7_extra_round3_audit.sh:100:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/run_codex_b7_extra_round3_audit.sh:104:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/run_codex_b7_extra_round3_audit.sh:153:printf '\n```\n\n## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff)\n\n```diff\n' >> "$TMP_PROMPT"
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:67:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:119:- (Q7.d) Oracle preflight (C-012) only checks `(1:ℝ) + 1 = 2`. Is preflight depth sufficient?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:129:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
./handover/audits/run_gemini_b7_extra_round3_audit.py:22:- **Round 2** (GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned PASS — but Codex independently returned VETO catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data. Per CLAUDE.md "Audit Standard" + memory feedback_dual_audit_conflict (conservative wins), Codex's VETO triumphed; round-2 fix landed in commit `1df1f62`.
./handover/audits/run_gemini_b7_extra_round3_audit.py:27:   - exit 124 (timeout) → synthetic UNSOLVED row (only this case emits synthetic data)
./handover/audits/run_gemini_b7_extra_round3_audit.py:28:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/run_gemini_b7_extra_round3_audit.py:29:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/run_gemini_b7_extra_round3_audit.py:35:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/run_gemini_b7_extra_round3_audit.py:53:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_gemini_b7_extra_round3_audit.py:69:| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_gemini_b7_extra_round3_audit.py:80:- (RQ1.c) Does evaluator.rs's call to `verify_trust_root` happen BEFORE any other state-modifying action (env_logger init, env::set_var for CLASSIFIER_VERSION)? Walk the order.
./handover/audits/run_gemini_b7_extra_round3_audit.py:83:Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.
./handover/audits/run_gemini_b7_extra_round3_audit.py:86:- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
./handover/audits/run_gemini_b7_extra_round3_audit.py:115:  - experiments/minif2f_v4/src/bin/evaluator.rs (added verify_trust_root call)
./handover/audits/run_gemini_b7_extra_round3_audit.py:179:    f"\n## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff fa93943..HEAD)\n\n```diff\n{evaluator_diff}\n```\n" +
./handover/audits/run_codex_b7_extra_reaudit.sh:18:**Mandate**: this is a RE-AUDIT. Your previous verdict (in `handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md`) was **VETO** with three top blockers (B1 evaluator-not-calling-verify_trust_root, B2 estimator-incomplete-subset, B3 ceiling-not-enforced) plus several CHALLENGE items. The user authorized FIX-THEN-PROCEED. The fixes have landed. Determine whether the original VETO is resolved AND whether the fix introduces new defects.
./handover/audits/run_codex_b7_extra_reaudit.sh:35:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_codex_b7_extra_reaudit.sh:42:| B1: evaluator doesn't call verify_trust_root | **VETO** | 15b87fb: evaluator.rs:163-183 — `verify_trust_root` called at top of `async fn main`; panic on Err. Negative-test verified end-to-end. |
./handover/audits/run_codex_b7_extra_reaudit.sh:53:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_codex_b7_extra_reaudit.sh:61:`evaluator.rs:163-183` calls `turingosv4::boot::verify_trust_root(&repo_root)` at the top of `async fn main`. The repo_root is resolved from `env!("CARGO_MANIFEST_DIR")` (= `experiments/minif2f_v4`) joined with `..` × 2 + canonicalize.
./handover/audits/run_codex_b7_extra_reaudit.sh:65:- (RQ1.b) The verify_trust_root call is BEFORE env_logger::init(). Does that matter for diagnostics? (Panic message goes to stderr regardless.)
./handover/audits/run_codex_b7_extra_reaudit.sh:66:- (RQ1.c) Could a future evaluator addition place ANY logic before verify_trust_root that would compromise the order? Defensive structure?
./handover/audits/run_codex_b7_extra_reaudit.sh:67:- (RQ1.d) The negative-test (tampered manifest → panic) was verified by the implementer. Is there a corresponding automated test (cargo test) that catches accidental removal of the verify_trust_root call? Do you require one?
./handover/audits/run_codex_b7_extra_reaudit.sh:73:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/run_codex_b7_extra_reaudit.sh:84:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/run_codex_b7_extra_reaudit.sh:88:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/run_codex_b7_extra_reaudit.sh:137:printf '\n```\n\n## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff)\n\n```diff\n' >> "$TMP_PROMPT"
./handover/audits/run_gemini_b7_extra_reaudit.py:21:- Q2.b: src/main.rs not in Trust Root manifest (call site of verify_trust_root)
./handover/audits/run_gemini_b7_extra_reaudit.py:41:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_gemini_b7_extra_reaudit.py:57:| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_gemini_b7_extra_reaudit.py:68:- (RQ1.c) Does evaluator.rs's call to `verify_trust_root` happen BEFORE any other state-modifying action (env_logger init, env::set_var for CLASSIFIER_VERSION)? Walk the order.
./handover/audits/run_gemini_b7_extra_reaudit.py:71:Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.
./handover/audits/run_gemini_b7_extra_reaudit.py:74:- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
./handover/audits/run_gemini_b7_extra_reaudit.py:103:  - experiments/minif2f_v4/src/bin/evaluator.rs (added verify_trust_root call)
./handover/audits/run_gemini_b7_extra_reaudit.py:167:    f"\n## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff fa93943..HEAD)\n\n```diff\n{evaluator_diff}\n```\n" +
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:66:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:118:- (Q7.d) The runner's oracle preflight (C-012) only checks `(1:ℝ) + 1 = 2 := by norm_num`. If Mathlib breaks for a more complex tactic mid-batch, calibration silently produces FALSE for problems that should have SOLVED. Is preflight depth sufficient?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:128:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:216:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1441:- Recovery: `lake exe cache get` (memorialized as feedback_oracle_preflight)
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:29:-   **(Q2.b) `src/main.rs` omission**: **VETO**. This is a critical omission. `TRACE_MATRIX_v1 § 4` defends omitting `boot.rs`, but `main.rs` is the *caller* that enforces the check. If `main.rs:12` is commented out, the entire Trust Root verification is silently bypassed. An attacker with passive file system access (the stated threat model) could make this one-line change without recompiling (if source is deployed alongside binary) or as part of a malicious recompile. The call to `verify_trust_root` is the lynchpin of the entire security model; its call site must be immutable. `src/main.rs` must be added to the Trust Root manifest.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:73:-   **(Q7.d) Oracle preflight depth**: **PASS**. The preflight check is minimal but sufficient for its purpose: to detect a completely broken Lean environment before starting an 8-hour batch.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:90:The discovery of two critical omissions from the Trust Root manifest (`src/main.rs` and `Cargo.lock`) represents a fundamental failure of the system's integrity guarantee. These are not minor issues; they undermine the entire premise of the `boot::verify_trust_root` check. The batch cannot proceed until these are fixed, as the resulting `genesis_payload.toml` would be based on a compromised verification process.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:17:The Round-2 fix (`1df1f62`) is a substantial improvement and correctly resolves the critical VETO-level defect identified by Codex in the previous round. The new crash-discrimination logic, which aborts the batch on any non-timeout, non-zero exit, prevents the silent absorption of panics (like `TRUST_ROOT_TAMPERED`) into the calibration dataset. This restores the integrity of the Trust Root mechanism. The addition of a pre-flight boot check is a commendable "fail fast" enhancement.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:32:    3.  `EXIT` is non-zero and not 124 → `CRASH` path, which aborts the entire batch with a specific exit code (3 for `TRUST_ROOT_TAMPERED`, 4 for others).
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:33:- **Conclusion**: This logic prevents panics from being silently converted into "valid" `UNSOLVED` data points, which was the critical flaw. The `TRUST_ROOT_TAMPERED` pre-flight check (lines 163-173) further hardens this by catching the most severe integrity failures before any API spend.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:63:- **RQ1 (Manifest Completeness)**: PASS. The 20-entry manifest is comprehensive for the stated threat model. The exclusion of `boot.rs` is a documented and reasonable trade-off (`TRACE_MATRIX_v1 § 4`). The `verify_trust_root` call in `evaluator.rs` happens correctly before any other action.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:29:**Mandate**: this is a RE-AUDIT. Your previous verdict (in `handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md`) was **VETO** with three top blockers (B1 evaluator-not-calling-verify_trust_root, B2 estimator-incomplete-subset, B3 ceiling-not-enforced) plus several CHALLENGE items. The user authorized FIX-THEN-PROCEED. The fixes have landed. Determine whether the original VETO is resolved AND whether the fix introduces new defects.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:46:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:53:| B1: evaluator doesn't call verify_trust_root | **VETO** | 15b87fb: evaluator.rs:163-183 — `verify_trust_root` called at top of `async fn main`; panic on Err. Negative-test verified end-to-end. |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:64:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:72:`evaluator.rs:163-183` calls `turingosv4::boot::verify_trust_root(&repo_root)` at the top of `async fn main`. The repo_root is resolved from `env!("CARGO_MANIFEST_DIR")` (= `experiments/minif2f_v4`) joined with `..` × 2 + canonicalize.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:76:- (RQ1.b) The verify_trust_root call is BEFORE env_logger::init(). Does that matter for diagnostics? (Panic message goes to stderr regardless.)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:77:- (RQ1.c) Could a future evaluator addition place ANY logic before verify_trust_root that would compromise the order? Defensive structure?
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:78:- (RQ1.d) The negative-test (tampered manifest → panic) was verified by the implementer. Is there a corresponding automated test (cargo test) that catches accidental removal of the verify_trust_root call? Do you require one?
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:84:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:95:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:99:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:259:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:276:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:291:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:292:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:293:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:294:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:297:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:315:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:340:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:465:    fn verify_trust_root_passes_on_intact_repo() {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:466:        verify_trust_root(&repo_root()).expect("intact repo verifies");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:481:    fn verify_trust_root_detects_tamper_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:486:        match verify_trust_root(&tmp).expect_err("tamper must be detected") {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:497:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:502:        verify_trust_root(&tmp).expect("matching hash verifies");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:533:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:536:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:537:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:557:#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:558:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:588:#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:600:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:756:# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:757:echo "[$(date -Is)] Oracle preflight..."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:773:echo "Oracle preflight OK."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:792:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:810:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:858:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:933:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:934:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1177:## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1213:+    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1226:+    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1228:+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1314:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1324:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1372:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1380:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1383:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1408:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1615:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1641:use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1661:    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1668:    // the file content; assert verify_trust_root returns Tampered.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1677:    match verify_trust_root(&tmp) {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1944:   168	    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1957:   181	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1959:   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2403:   115	# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2404:   116	echo "[$(date -Is)] Oracle preflight..."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2420:   132	echo "Oracle preflight OK."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2439:   151	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2457:   169	    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2505:   217	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2580:   292	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2581:   293	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2607:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2624:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2639:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2640:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2641:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2642:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2645:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2663:    71	pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2688:    96	/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3443:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3469:    31	use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3489:    51	    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3496:    58	    // the file content; assert verify_trust_root returns Tampered.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3505:    67	    match verify_trust_root(&tmp) {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3650:    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3651:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3681:    42	#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3693:    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3751:    19	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3761:    29	| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3809:    77	- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3817:    85	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3820:    88	- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3845:   113	1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4022:    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4025:    14	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4026:    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4372:/bin/bash -lc 'rg -n "TRUST_ROOT_TAMPERED|synthetic_timeout_or_crash|synthetic_reason|evaluator_crash|TIMEOUT|CRASH|compute_p0|progress" handover/preregistration/scripts/compute_p0.py handover/preregistration/scripts/run_p0_calibration.sh experiments/minif2f_v4/src/bin/evaluator.rs' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4379:experiments/minif2f_v4/src/bin/evaluator.rs:183:        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4394:handover/preregistration/scripts/run_p0_calibration.sh:151:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4396:handover/preregistration/scripts/run_p0_calibration.sh:169:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4404:handover/preregistration/scripts/run_p0_calibration.sh:292:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4405:handover/preregistration/scripts/run_p0_calibration.sh:293:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4428:                    'synthetic_timeout_or_crash': True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4464:- **RQ1 / B1:** Code-level fix is present: evaluator calls `verify_trust_root` before args, problem loading, LLM calls, or JSONL emission at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:163). `env_logger::init()` is actually before the check at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:164), contrary to the prompt; that is not itself a P0. `canonicalize()` is acceptable for PREREG project-tree runs at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:176), but copied-binary deployment remains out of scope. There is no automated binary-level regression test that proves the evaluator entrypoint panics on tamper; current tests call `verify_trust_root` directly at [trust_root_immutability.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/trust_root_immutability.rs:47). More importantly, the runner currently neutralizes the panic, so B1 is not launch-complete.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4490:- **RQ1 / B1:** Code-level fix is present: evaluator calls `verify_trust_root` before args, problem loading, LLM calls, or JSONL emission at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:163). `env_logger::init()` is actually before the check at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:164), contrary to the prompt; that is not itself a P0. `canonicalize()` is acceptable for PREREG project-tree runs at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:176), but copied-binary deployment remains out of scope. There is no automated binary-level regression test that proves the evaluator entrypoint panics on tamper; current tests call `verify_trust_root` directly at [trust_root_immutability.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/trust_root_immutability.rs:47). More importantly, the runner currently neutralizes the panic, so B1 is not launch-complete.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:215:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:320:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:524:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1078:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1166:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1333:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2287:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3512:- Recovery: `lake exe cache get` (memorialized as feedback_oracle_preflight)
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:295:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:511://   1. Excludes evaluator-side preflight (kernel construction, tool mounting,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:882:+    // → final Lean call returns. Excludes evaluator preflight (kernel ctor,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:916:+    // (just before the API call goes out). Excludes prior preflight cost.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1333:    87	    // → final Lean call returns. Excludes evaluator preflight (kernel ctor,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1645:    14	//   1. Excludes evaluator-side preflight (kernel construction, tool mounting,
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2192:   233	    // (just before the API call goes out). Excludes prior preflight cost.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3774:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:216:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3777:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3809:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:295:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3858:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:215:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3861:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3863:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:320:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3865:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:524:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3879:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1078:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3882:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3884:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1166:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3886:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1333:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3903:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2287:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3906:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3946:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:226:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3949:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3951:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:314:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3953:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:481:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3970:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1435:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3973:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4005:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4008:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4010:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4034:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4037:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4069:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4072:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4074:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4098:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4101:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4353:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4360:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:302:- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4374:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:152:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4377:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4379:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:257:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4381:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:461:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5504:   270	- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5536:   302	- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5680:   152	2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5696:   168	This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:226:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:314:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:481:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1435:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2660:- Recovery: `lake exe cache get` (memorialized as feedback_oracle_preflight)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6939:- Recovery: `lake exe cache get` (memorialized as feedback_oracle_preflight)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2939:- Recovery: `lake exe cache get` (memorialized as feedback_oracle_preflight)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:73:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:125:- (Q7.d) The runner's oracle preflight (C-012) only checks `(1:ℝ) + 1 = 2 := by norm_num`. If Mathlib breaks for a more complex tactic mid-batch, calibration silently produces FALSE for problems that should have SOLVED. Is preflight depth sufficient?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:135:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:286:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:303:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:318:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:319:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:320:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:321:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:324:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:342:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:369:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:493:    fn verify_trust_root_passes_on_intact_repo() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:494:        verify_trust_root(&repo_root()).expect("intact repo verifies");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:498:    fn verify_trust_root_detects_tamper_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:516:        let err = verify_trust_root(&tmp).expect_err("tamper must be detected");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:530:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:541:        verify_trust_root(&tmp).expect("matching hash verifies");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:572:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:575:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:576:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:596:#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:597:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:630:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:754:# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:755:echo "Oracle preflight..."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:772:echo "Oracle preflight OK."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1131:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1141:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1189:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1197:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1200:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1225:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1356:- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1472:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1541:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2127:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2144:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2159:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2160:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2161:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2162:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2165:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2183:    71	pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2210:    98	/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2307:    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2310:    14	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2311:    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2573:    87	# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2574:    88	echo "Oracle preflight..."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2591:   105	echo "Oracle preflight OK."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2845:    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2846:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2879:    45	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3084:   461	               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3119:    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3129:    27	| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3177:    75	- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3185:    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3188:    86	- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3213:   111	1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3340:    26	- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3685:/bin/bash -lc 'rg -n "trust_root|verify_trust_root|src/main.rs|Cargo.lock|cases/MANIFEST|genesis_payload" tests src handover experiments/minif2f_v4/src -S' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3785:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4189:src/main.rs:14:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4193:src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4194:src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4195:src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4198:src/boot.rs:71:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4201:src/boot.rs:98:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4214:src/boot.rs:222:    fn verify_trust_root_passes_on_intact_repo() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4215:src/boot.rs:223:        verify_trust_root(&repo_root()).expect("intact repo verifies");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4216:src/boot.rs:227:    fn verify_trust_root_detects_tamper_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4222:src/boot.rs:245:        let err = verify_trust_root(&tmp).expect_err("tamper must be detected");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4223:src/boot.rs:259:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4226:src/boot.rs:270:        verify_trust_root(&tmp).expect("matching hash verifies");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4230:handover/audits/run_gemini_b7_extra_prebatch_audit.py:67:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4233:handover/audits/run_gemini_b7_extra_prebatch_audit.py:129:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4243:handover/audits/run_codex_b7_extra_prebatch_audit.sh:66:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4246:handover/audits/run_codex_b7_extra_prebatch_audit.sh:128:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4257:handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4265:handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:29:-   **(Q2.b) `src/main.rs` omission**: **VETO**. This is a critical omission. `TRACE_MATRIX_v1 § 4` defends omitting `boot.rs`, but `main.rs` is the *caller* that enforces the check. If `main.rs:12` is commented out, the entire Trust Root verification is silently bypassed. An attacker with passive file system access (the stated threat model) could make this one-line change without recompiling (if source is deployed alongside binary) or as part of a malicious recompile. The call to `verify_trust_root` is the lynchpin of the entire security model; its call site must be immutable. `src/main.rs` must be added to the Trust Root manifest.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4268:handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4269:handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:90:The discovery of two critical omissions from the Trust Root manifest (`src/main.rs` and `Cargo.lock`) represents a fundamental failure of the system's integrity guarantee. These are not minor issues; they undermine the entire premise of the `boot::verify_trust_root` check. The batch cannot proceed until these are fixed, as the resulting `genesis_payload.toml` would be based on a compromised verification process.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4279:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4291:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4303:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4314:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:73:- (Q2.b) The 16-file list omits `src/main.rs` (which calls verify_trust_root). If main.rs is tampered to NOT call verify_trust_root, the entire chain bypasses. Should main.rs be in Trust Root?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4317:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:135:| `genesis_payload.toml [trust_root]` | FC3-S3 readonly subgraph extension | boot::verify_trust_root reads | ? |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4323:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:318:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4324:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:319:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4325:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:320:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4328:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:342:pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4331:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:369:/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4344:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:493:    fn verify_trust_root_passes_on_intact_repo() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4345:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:494:        verify_trust_root(&repo_root()).expect("intact repo verifies");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4346:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:498:    fn verify_trust_root_detects_tamper_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4352:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:516:        let err = verify_trust_root(&tmp).expect_err("tamper must be detected");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4353:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:530:    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4356:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:541:        verify_trust_root(&tmp).expect("matching hash verifies");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4358:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:575:    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4361:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:596:#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4367:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1131:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4368:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1141:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4373:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1189:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4374:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1197:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4376:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1356:- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4386:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2159:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4387:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2160:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4388:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2161:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4391:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2183:    71	pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4394:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2210:    98	/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4402:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2310:    14	    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4406:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2845:    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4414:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3119:    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4415:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3129:    27	| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4420:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3177:    75	- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4421:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3185:    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4423:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3340:    26	- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4430:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4442:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4453:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4464:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4478:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3777:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4485:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3861:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4493:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3882:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4501:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3906:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4510:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3949:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4518:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3973:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4525:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4008:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4532:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4037:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4539:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4072:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4546:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4101:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4558:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4353:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4567:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4377:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4577:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5504:   270	- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4592:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5696:   168	This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4600:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4611:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4618:handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md:26:- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4628:handover/ai-direct/LATEST.md:41:  - `src/boot.rs` (new, +pub in lib.rs): `verify_trust_root(repo_root)` parses [trust_root] section (hand-rolled minimal TOML parser, no new dep — compression principle), recomputes SHA-256 per path, returns `TrustRootError::Tampered{path,expected,actual}` on mismatch; 6 unit tests (parse/blank/comment/missing-section/intact-repo/tempdir-tamper)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4629:handover/ai-direct/LATEST.md:42:  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4637:handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:12:Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4639:handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:52:If/when Phase 11+ lands an in-process re-init mechanism (TRACE_MATRIX FC3-N41), it can use `Result<(), TrustRootError>` from `verify_trust_root` instead of letting the panic propagate. At that point the panic becomes the *uncaught* path and structured retry becomes the *caught* path. Constitution still does not need to change — both paths land at FC3-E14.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4641:handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:57:- `src/boot.rs:62` — `verify_trust_root` itself, TRACE_MATRIX backlink to FC3-N34
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4642:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:17:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4643:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:27:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4648:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:75:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4649:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:83:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4654:handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4668:handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4691:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4714:    28	use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4734:    48	    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4741:    55	    // the file content; assert verify_trust_root returns Tampered.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4750:    64	    match verify_trust_root(&tmp) {
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5340:/bin/bash -lc "nl -ba experiments/minif2f_v4/Cargo.toml | sed -n '1,80p' && rg -n \"verify_trust_root|turingosv4::boot\" experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5369:   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5405:   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.
./handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md:26:- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
./handover/ai-direct/CHECKPOINT_PHASE_1_2026-04-20.md:93:| Lean preflight | OK | implicit (all 17 compile) | ✓ |
./handover/ai-direct/CHECKPOINT_PHASE_0_2026-04-20.md:66:| Lean preflight | OK | Implicit (proof artifacts compile) | ✓ |
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:103:- Lean + Mathlib toolchain drift (preflight lock)
./handover/ai-direct/CHECKPOINT_PHASE_2_1c_2026-04-21.md:68:| Lean preflight | OK | all 17 compile independently | ✓ |
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:291:- Recovery: `lake exe cache get` (memorialized as feedback_oracle_preflight)
./handover/ai-direct/PLAN_V3_2026-04-15.md:141:- oracle preflight 失败
./handover/ai-direct/LATEST.md:41:  - `src/boot.rs` (new, +pub in lib.rs): `verify_trust_root(repo_root)` parses [trust_root] section (hand-rolled minimal TOML parser, no new dep — compression principle), recomputes SHA-256 per path, returns `TrustRootError::Tampered{path,expected,actual}` on mismatch; 6 unit tests (parse/blank/comment/missing-section/intact-repo/tempdir-tamper)
./handover/ai-direct/LATEST.md:42:  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:152:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:257:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:461:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:302:- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/preregistration/scripts/run_p0_calibration.sh:115:# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/preregistration/scripts/run_p0_calibration.sh:116:echo "[$(date -Is)] Oracle preflight..."
./handover/preregistration/scripts/run_p0_calibration.sh:132:echo "Oracle preflight OK."
./handover/preregistration/scripts/run_p0_calibration.sh:134:# Audit-fix 2026-04-25 round-2 (Codex re-audit VETO): evaluator boot preflight.
./handover/preregistration/scripts/run_p0_calibration.sh:136:# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/preregistration/scripts/run_p0_calibration.sh:138:# preflight surfaces the failure with the clean diagnostic and zero wasted
./handover/preregistration/scripts/run_p0_calibration.sh:140:echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
./handover/preregistration/scripts/run_p0_calibration.sh:141:PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/preregistration/scripts/run_p0_calibration.sh:142:if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/preregistration/scripts/run_p0_calibration.sh:143:    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/preregistration/scripts/run_p0_calibration.sh:156:echo "Evaluator boot preflight OK."
./handover/preregistration/scripts/run_p0_calibration.sh:175:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/preregistration/scripts/run_p0_calibration.sh:184:    # USED ONLY for legitimate timeout (exit 124). Crash paths now ABORT
./handover/preregistration/scripts/run_p0_calibration.sh:199:    "synthetic_timeout_or_crash": True,
./handover/preregistration/scripts/run_p0_calibration.sh:248:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/preregistration/scripts/run_p0_calibration.sh:288:                # synthetic_timeout_or_crash=true disambiguator.
./handover/preregistration/scripts/run_p0_calibration.sh:295:                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/preregistration/scripts/run_p0_calibration.sh:301:                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/preregistration/scripts/run_p0_calibration.sh:303:                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/preregistration/scripts/run_p0_calibration.sh:306:                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/preregistration/scripts/run_p0_calibration.sh:344:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/preregistration/scripts/run_p0_calibration.sh:345:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:1:# OBS — Boot failure (TRUST_ROOT_TAMPERED) is not a constitutional HALT
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:12:Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:16:## Why TRUST_ROOT_TAMPERED ≠ FC2-N22 HALT
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:18:| Property | FC2-N22 HALT | TRUST_ROOT_TAMPERED panic |
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:26:FC2 has a `HALT` node (FC2-N22) that lives *inside* the boot/tick lifecycle. TRUST_ROOT_TAMPERED fires *before* the boot lifecycle — it is a precondition violation on the readonly base.
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:52:If/when Phase 11+ lands an in-process re-init mechanism (TRACE_MATRIX FC3-N41), it can use `Result<(), TrustRootError>` from `verify_trust_root` instead of letting the panic propagate. At that point the panic becomes the *uncaught* path and structured retry becomes the *caught* path. Constitution still does not need to change — both paths land at FC3-E14.
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:57:- `src/boot.rs:62` — `verify_trust_root` itself, TRACE_MATRIX backlink to FC3-N34
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:19:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:29:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:82:- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:90:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:93:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:118:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.

exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,60p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4
     2	
     3	## What
     4	Silicon-Native Microkernel for LLM Formal Verification Swarm.
     5	Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.
     6	
     7	## Why
     8	- 唯一对齐文档: `constitution.md` (反奥利奥架构)
     9	- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
    10	- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)
    11	
    12	## Code Standard (Art. I.1 + C-004 + C-027)
    13	- `cargo check` / `cargo test` 必过；`.env` 永不 commit
    14	- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
    15	- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码
    16	
    17	## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
    18	- Generator ≠ Evaluator：代码作者不可是唯一审计者
    19	- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
    20	- 宪法违规立即 BLOCKER，不可延期、不可"可接受"
    21	
    22	## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
    23	- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
    24	- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
    25	- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
    26	- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
    27	- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪
    28	
    29	## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
    30	- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
    31	- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
    32	- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）
    33	
    34	## Alignment Standard (Art. IV + C-069)
    35	- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
    36	  (后续 rev: `TRACE_MATRIX_vN.md`)
    37	- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
    38	  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
    39	- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
    40	  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
    41	- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
    42	- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法
    43	
    44	## Common Law (宪法 + 判例)
    45	宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
    46	- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
    47	- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
    48	- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）
    49	
    50	## Docs (按需加载)
    51	| 文档 | 何时加载 |
    52	|------|---------|
    53	| `docs/architecture.md` | 修改 src/ 核心模块时 |
    54	| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
    55	| `docs/hardware.md` | SSH/部署/远程操作时 |
    56	| `docs/experiments.md` | 创建或运行实验时 |
    57	| `docs/rules.md` | 触发规则或修改规则时 |
    58	
    59	## User
    60	独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.

exec
/bin/bash -lc 'rg -n "fn make_pput|struct PputResult|model_snapshot|build_sha|binary_sha256|golden_path_token_count|schema_version|PPUT_RESULT" experiments/minif2f_v4/src/bin/evaluator.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
45:/// `schema_version` is the discriminant). Legacy diagnostic fields below
51:struct PputResult {
54:    schema_version: String,
68:    golden_path_token_count: u64,
94:    model_snapshot: String,
98:    binary_sha256: String,
110:    gp_token_count: u64,           // alias of golden_path_token_count
116:    build_sha: Option<String>,
249:    println!("PPUT_RESULT:{}", json);
382:            // Do not emit PPUT_RESULT — batch runner must retry on resume.
1157:fn make_pput(
1188:    let build_sha = std::env::var("BUILD_SHA").ok();
1216:    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
1218:    let git_sha = build_sha.clone().unwrap_or_default();
1219:    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();
1236:        schema_version: "v2.0".to_string(),
1242:        golden_path_token_count: gp_tokens,
1253:        model_snapshot,
1255:        binary_sha256,
1267:        build_sha,
1339:    /// no `schema_version` field, which forced B1's dispatcher to classify
1342:    /// `schema_version` stamp or any required v2 field.
1362:            line.contains("\"schema_version\":\"v2.0\""),
1363:            "v2 emit must stamp schema_version=v2.0; got: {}",
1370:                assert_eq!(agg.schema_version, "v2.0");

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '80,160p'; nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1240,1305p'; nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '260,330p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    80	    pput_m_verified: f64,
    81	    /// B2 C_i sub-counter: count of proposals that did NOT verify.
    82	    failed_branch_count: u32,
    83	    /// Phase B always 0; Phase C+ when ArtifactState rollbacks land.
    84	    rollback_count: u32,
    85	    /// FAR guardrail (Phase B not yet computed; emit 0.0 placeholder).
    86	    far: f64,
    87	    /// ERR guardrail (Phase B not yet computed).
    88	    err: f64,
    89	    /// IAC guardrail (Phase B not yet computed).
    90	    iac: f64,
    91	    /// CPR guardrail (Phase B not yet computed).
    92	    cpr: f64,
    93	    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
    94	    model_snapshot: String,
    95	    /// Trust Root provenance — git commit SHA at boot.
    96	    git_sha: String,
    97	    /// Trust Root binary fingerprint — Phase B placeholder; B7 fills.
    98	    binary_sha256: String,
    99	    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous" — from
   100	    /// MODE env, default "full" Phase B.
   101	    mode: String,
   102	
   103	    // ── Legacy diagnostic fields (preserved for downstream tooling) ──
   104	    problem: String,
   105	    condition: String,
   106	    model: String,
   107	    has_golden_path: bool,         // alias of `solved`; legacy field name
   108	    time_secs: f64,                // wall time elapsed (function-entry bracket; legacy)
   109	    pput: f64,                     // 100/time if GP, 0 otherwise (legacy display)
   110	    gp_token_count: u64,           // alias of golden_path_token_count
   111	    gp_node_count: usize,          // nodes on golden path (0 if no GP)
   112	    tx_count: u64,                 // total transactions attempted
   113	    // C-012 provenance: stamp per-row commit SHA + classifier version + RNG seed.
   114	    // All Optional; serialize-skip when None (backward compat with v3.1/v3.2 artifacts).
   115	    #[serde(skip_serializing_if = "Option::is_none")]
   116	    build_sha: Option<String>,
   117	    #[serde(skip_serializing_if = "Option::is_none")]
   118	    classifier_version: Option<String>,
   119	    #[serde(skip_serializing_if = "Option::is_none")]
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
  1240	        split,
  1241	        verified: post_hoc_verified,
  1242	        golden_path_token_count: gp_tokens,
  1243	        total_run_token_count: c_i,
  1244	        total_wall_time_ms: t_i,
  1245	        progress: progress_verified,
  1246	        pput_runtime,
  1247	        pput_verified,
  1248	        pput_m_verified,
  1249	        failed_branch_count: failed_count,
  1250	        // Phase B placeholders — Phase C+ wires these as the modes activate.
  1251	        rollback_count: 0,
  1252	        far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
  1253	        model_snapshot,
  1254	        git_sha,
  1255	        binary_sha256,
  1256	        mode,
  1257	        // ── Legacy diagnostic fields ──
  1258	        problem: problem.to_string(),
  1259	        condition: condition.to_string(),
  1260	        model: model.to_string(),
  1261	        has_golden_path: has_gp,
  1262	        time_secs: elapsed,
  1263	        pput,
  1264	        gp_token_count: gp_tokens,
  1265	        gp_node_count: gp_nodes,
  1266	        tx_count,
  1267	        build_sha,
  1268	        classifier_version,
  1269	        boltzmann_seed,
  1270	        tool_dist,
  1271	        unique_payload_ratio,
  1272	        gp_payload,
  1273	        gp_path,
  1274	        gp_proof_file,
  1275	        // B7-extra: only the calibration-treatment short-circuit site mutates
  1276	        // this to Some(true). Default = None (most callers).
  1277	        synthetic_short_circuit: None,
  1278	    }
  1279	}
  1280	
  1281	/// Phase 0 (C-039 candidate): persist a self-contained, re-verifiable proof artifact.
  1282	/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
  1283	/// the exact code that the Lean oracle accepted. An external verifier can run
  1284	/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
  1285	/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
  1286	fn persist_proof_artifact(
  1287	    problem_file: &str, theorem_name: &str, problem_statement: &str,
  1288	    full_proof: &str, path_choice: &str, agent_id: &str,
  1289	) -> Option<String> {
  1290	    use std::collections::hash_map::DefaultHasher;
  1291	    use std::hash::{Hash, Hasher};
  1292	    let exp_dir = std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into());
  1293	    let proofs_dir = std::path::Path::new(&exp_dir).join("proofs");
  1294	    if let Err(e) = std::fs::create_dir_all(&proofs_dir) {
  1295	        log::warn!("[audit] cannot create proofs dir {:?}: {}", proofs_dir, e);
  1296	        return None;
  1297	    }
  1298	    let ts = std::time::SystemTime::now()
  1299	        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
  1300	    let mut h = DefaultHasher::new();
  1301	    full_proof.hash(&mut h);
  1302	    let short = format!("{:x}", h.finish() & 0xFFFFFFFF);
  1303	    let fname = format!("{}_{}_{}.lean", theorem_name, ts, short);
  1304	    let path = proofs_dir.join(&fname);
  1305	    let header = format!(
   260	    if PathBuf::from(problem_file).exists() {
   261	        return problem_file.to_string();
   262	    }
   263	    let test_path = format!("{}/MiniF2F/Test/{}", minif2f_dir, problem_file);
   264	    if PathBuf::from(&test_path).exists() { return test_path; }
   265	    let valid_path = format!("{}/MiniF2F/Valid/{}", minif2f_dir, problem_file);
   266	    if PathBuf::from(&valid_path).exists() { return valid_path; }
   267	    eprintln!("Problem file not found: {}", problem_file);
   268	    std::process::exit(1);
   269	}
   270	
   271	/// Oneshot: single LLM call → verify → PPUT.
   272	async fn run_oneshot(
   273	    problem_file: &str, problem_statement: &str, theorem_name: &str,
   274	    lean_path: &str, proxy_url: &str, model: &str,
   275	) -> PputResult {
   276	    let start = Instant::now();
   277	    let mut acc = RunCostAccumulator::new();
   278	    let mut wc = RunWallClock::new();
   279	
   280	    let oracle = Lean4Oracle::new(
   281	        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
   282	    );
   283	
   284	    // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
   285	    // bracket BEFORE prompt construction. PREREG § 5 / plan B3 define T_i
   286	    // as "first agent prompt construction → final Lean call". Marking after
   287	    // the construction (prior wiring) under-counted prompt-build time and
   288	    // forced the conformance test to relax its 7100ms assertion.
   289	    wc.mark_first_read();
   290	
   291	    // R-22 v2 clause 4 stays reject-only; the prompt must prevent fences at the source.
   292	    // Chat models (deepseek-chat, 2026-04-22) default to ```lean fences; verifier hard-rejects
   293	    // any response containing ``` so the instruction must be explicit. See F-2026-04-22-08.
   294	    let prompt = format!(
   295	        "Complete the following Lean 4 proof. Output ONLY the tactic proof body as raw Lean \
   296	         tokens. DO NOT wrap in markdown code fences (no ```). No prose, no backticks.\n\n{}",
   297	        problem_statement
   298	    );
   299	
   300	    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
   301	    // Model-aware max_tokens: deepseek-chat caps at 8192; reasoner needs 16000 for thinking.
   302	    let max_toks = if model.contains("chat") { 8000 } else { 16000 };
   303	    let request = GenerateRequest {
   304	        model: model.to_string(),
   305	        messages: vec![Message { role: "user".into(), content: prompt }],
   306	        temperature: Some(0.2),
   307	        max_tokens: Some(max_toks),
   308	    };
   309	
   310	    // PPUT-CCL B6 runtime gate: scan the assembled prompt for PPUT scalars
   311	    // before the call goes out. Any leak aborts deterministically — Goodhart
   312	    // shield at the LLM-call boundary.
   313	    assert_no_metric_leak(&request.messages[0].content);
   314	    match client.generate(&request).await {
   315	        Ok(response) => {
   316	            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
   317	            acc.record_proposal(false);
   318	            // Rule 22 v2 clause 4: reject markdown fences
   319	            if response.content.contains("```") {
   320	                wc.mark_final_accept();
   321	                // P0-A: caller declares both runtime + post-hoc legs.
   322	                // Fence reject = neither leg fired.
   323	                return make_pput(problem_file, "oneshot", model,
   324	                                 false, false, start, 0, 0, 1,
   325	                                 None, None, None, None, None,
   326	                                 Some(acc.total_run_token_count()),
   327	                                 Some(acc.failed_branch_count),
   328	                                 wc.elapsed_ms());
   329	            }
   330	

exec
/bin/bash -lc 'rg -n "test.*evaluator|Command::new|target/release/evaluator|CARGO_BIN_EXE_evaluator|TRUST_ROOT_TAMPERED" experiments/minif2f_v4/tests src tests -S' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/main.rs:11:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
src/main.rs:15:        panic!("TRUST_ROOT_TAMPERED: {e}");
src/boot.rs:15:// `TRUST_ROOT_TAMPERED`.
src/boot.rs:32:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
src/boot.rs:50:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
src/boot.rs:53:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
src/sdk/sandbox.rs:80:        let mut child = Command::new(&self.command)
experiments/minif2f_v4/tests/mode_flag_binary_purity.rs:26:    // this test will exercise the end-to-end evaluator binary once Phase C
experiments/minif2f_v4/tests/trust_root_immutability.rs:5:// mismatch = `TRUST_ROOT_TAMPERED` abort.

codex
I found one concrete mismatch with the stated round-2 invariant: the runner comments now say synthetic rows are used only for timeout, but the full loop still emits a synthetic UNSOLVED row when the MiniF2F problem file is missing. I’m checking whether that can pass strict completeness and bias `p_0`, because that would be the same class of silent data-loss problem as the prior VETO.
exec
/bin/bash -lc 'cargo test -p minif2f_v4 trust_root -- --nocapture' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "python3 - <<'PY'
import json, tempfile, subprocess, pathlib
seeds=(31415,2718)
problems=[f'p{i:03d}' for i in range(144)]
with tempfile.TemporaryDirectory() as d:
    d=pathlib.Path(d)
    c=d/'control.jsonl'; t=d/'treatment.jsonl'; out=d/'out.json'
    with c.open('w') as cf, t.open('w') as tf:
        for p in problems:
            for s in seeds:
                base={'schema_version':'v2.0','calibration_problem_id':p,'calibration_seed':s,'progress':0,'synthetic_timeout_or_crash':True,'synthetic_reason':'problem_file_missing'}
                cf.write(json.dumps(base|{'calibration_mode':'control'})+'\\n')
                tf.write(json.dumps(base|{'calibration_mode':'treatment'})+'\\n')
    r=subprocess.run(['python3','handover/preregistration/scripts/compute_p0.py','--control',str(c),'--treatment',str(t),'--out-json',str(out)], cwd='/home/zephryj/projects/turingosv4', text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print('exit', r.returncode)
    print(r.stdout.splitlines()[:12])
    print('stderr', r.stderr.strip())
PY" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "python3 - <<'PY'
import json, tempfile, subprocess, pathlib
seeds=(31415,2718)
problems=[f'p{i:03d}' for i in range(144)]
with tempfile.TemporaryDirectory() as d:
    d=pathlib.Path(d)
    c=d/'control.jsonl'; t=d/'treatment.jsonl'
    with c.open('w') as cf, t.open('w') as tf:
        for p in problems:
            for s in seeds:
                cf.write(json.dumps({'calibration_problem_id':p,'calibration_seed':s,'progress':1})+'\\n')
                tf.write(json.dumps({'calibration_problem_id':p,'calibration_seed':s,'progress':0})+'\\n')
    r=subprocess.run(['python3','handover/preregistration/scripts/compute_p0.py','--control',str(c),'--treatment',str(t)], cwd='/home/zephryj/projects/turingosv4', text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print('exit', r.returncode)
    print('stderr_tail', r.stderr.strip().splitlines()[-1] if r.stderr.strip() else '')
PY" in /home/zephryj/projects/turingosv4
 succeeded in 142ms:
exit 0
['{', '  "n_problems": 144,', '  "n_pairs": 288,', '  "n_control_solved": 0,', '  "n_treatment_solved": 0,', '  "n_regression_pairs": 0,', '  "n_regression_problems_max_seed": 0,', '  "p0": 0.0,', '  "p0_ceiling": 0.1,', '  "ceiling_pass": true', '}', '']
stderr 

 succeeded in 152ms:
exit 2
stderr_tail ERROR: p_0 = 1.0000 > 0.10 — ABORT per PREREG § 5.5 ceiling.

 succeeded in 453ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: `turingosv4` (lib) generated 9 warnings (run `cargo fix --lib -p turingosv4` to apply 9 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/minif2f_v4-ea0fb4b84b011e86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 31 filtered out; finished in 0.00s

     Running unittests src/bin/evaluator.rs (target/debug/deps/evaluator-d9a8d7c0016f58a1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/architect_sole_lt_reader.rs (target/debug/deps/architect_sole_lt_reader-1614be9f7cc074ba)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/artifact_content_predicates.rs (target/debug/deps/artifact_content_predicates-7d9bb67b5ca57998)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/artifact_lookup_evasion.rs (target/debug/deps/artifact_lookup_evasion-f48ca58ba5d0e691)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/auditor_sees_candidate_only.rs (target/debug/deps/auditor_sees_candidate_only-e73e0ba91a257b13)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/heldout_operational_sealing.rs (target/debug/deps/heldout_operational_sealing-7399950e0458ee16)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/mode_flag_binary_purity.rs (target/debug/deps/mode_flag_binary_purity-8055ec99ba077c4e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/pput_anti_goodhart.rs (target/debug/deps/pput_anti_goodhart-b6a795b44d55732e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/trust_root_immutability.rs (target/debug/deps/trust_root_immutability-e3f95f3eb1354a13)

running 3 tests
test test_trust_root_simulated_write_aborts ... ok
test test_trust_root_manifest_includes_b2_b4_files ... ok
test test_trust_root_immutable_at_boot ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.05s


codex
**Findings**

1. **P0: missing MiniF2F problem files are still silently converted into valid calibration data.**  
   The runner claims synthetic rows are “USED ONLY” for timeout at [run_p0_calibration.sh:184](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:184), but the loop still emits a synthetic UNSOLVED row for `PROBLEM_NOT_FOUND` at [run_p0_calibration.sh:246](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:246). `compute_p0.py` only requires tags, seed set, matched problem sets, and row counts at [compute_p0.py:69](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:69), [compute_p0.py:93](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:93), [compute_p0.py:110](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:110), and [compute_p0.py:116](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:116). It does not reject `synthetic_reason=problem_file_missing`. I verified a 144-problem × 2-seed all-`problem_file_missing` synthetic dataset exits `0` with `p0=0.0`. That is the same silent data-loss class as the prior crash-as-data VETO.

2. **P0-adjacent: evaluator boot preflight discards the evaluator exit code.**  
   The preflight runs `timeout 30 "$EVALUATOR" ... || true` at [run_p0_calibration.sh:141](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:141), then only greps output for `TRUST_ROOT_TAMPERED` or unexpected panic text at [run_p0_calibration.sh:142](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:142) and [run_p0_calibration.sh:150](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:150). A preflight timeout `124` or empty-output hang would print “OK” at [run_p0_calibration.sh:156](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:156). Since in-loop `124` becomes synthetic UNSOLVED at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285), this can still turn an evaluator-level infrastructure failure into apparently complete data.

**Resolved From Prior VETO**

The specific round-2 VETO, `TRUST_ROOT_TAMPERED` panic absorbed as valid synthetic data, is resolved for in-loop evaluator crashes: only exit `124` emits synthetic data at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285); other exits abort at [run_p0_calibration.sh:300](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:300), with `TRUST_ROOT_TAMPERED` mapped to exit `3` at [run_p0_calibration.sh:301](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:301). The evaluator now calls `verify_trust_root` before loading problems or making LLM calls at [evaluator.rs:176](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:176) and panics on failure at [evaluator.rs:181](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:181).

B2/B3 are substantially fixed: strict completeness and denominator pinning are at [compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88) and [compute_p0.py:151](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151); runner propagates estimator exit `2` as abort at [run_p0_calibration.sh:365](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:365).

**Other Notes**

`env_logger::init()` actually precedes Trust Root verification at [evaluator.rs:164](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:164), contrary to RQ1.b’s wording. I do not treat that as a blocker: no LLM call, JSONL emit, or problem load occurs before boot verification.

Targeted verification run: `cargo test -p minif2f_v4 trust_root -- --nocapture` passed. I also verified `compute_p0.py` returns exit `2` for an all-regression dataset.

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED again. Remove or abort the `problem_file_missing` synthetic path, add a pre-loop adaptation-file existence preflight, and make evaluator boot preflight assert an expected nonzero exit/status instead of grepping output after `|| true`.
tokens used
103,791
**Findings**

1. **P0: missing MiniF2F problem files are still silently converted into valid calibration data.**  
   The runner claims synthetic rows are “USED ONLY” for timeout at [run_p0_calibration.sh:184](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:184), but the loop still emits a synthetic UNSOLVED row for `PROBLEM_NOT_FOUND` at [run_p0_calibration.sh:246](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:246). `compute_p0.py` only requires tags, seed set, matched problem sets, and row counts at [compute_p0.py:69](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:69), [compute_p0.py:93](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:93), [compute_p0.py:110](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:110), and [compute_p0.py:116](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:116). It does not reject `synthetic_reason=problem_file_missing`. I verified a 144-problem × 2-seed all-`problem_file_missing` synthetic dataset exits `0` with `p0=0.0`. That is the same silent data-loss class as the prior crash-as-data VETO.

2. **P0-adjacent: evaluator boot preflight discards the evaluator exit code.**  
   The preflight runs `timeout 30 "$EVALUATOR" ... || true` at [run_p0_calibration.sh:141](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:141), then only greps output for `TRUST_ROOT_TAMPERED` or unexpected panic text at [run_p0_calibration.sh:142](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:142) and [run_p0_calibration.sh:150](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:150). A preflight timeout `124` or empty-output hang would print “OK” at [run_p0_calibration.sh:156](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:156). Since in-loop `124` becomes synthetic UNSOLVED at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285), this can still turn an evaluator-level infrastructure failure into apparently complete data.

**Resolved From Prior VETO**

The specific round-2 VETO, `TRUST_ROOT_TAMPERED` panic absorbed as valid synthetic data, is resolved for in-loop evaluator crashes: only exit `124` emits synthetic data at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285); other exits abort at [run_p0_calibration.sh:300](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:300), with `TRUST_ROOT_TAMPERED` mapped to exit `3` at [run_p0_calibration.sh:301](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:301). The evaluator now calls `verify_trust_root` before loading problems or making LLM calls at [evaluator.rs:176](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:176) and panics on failure at [evaluator.rs:181](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:181).

B2/B3 are substantially fixed: strict completeness and denominator pinning are at [compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88) and [compute_p0.py:151](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151); runner propagates estimator exit `2` as abort at [run_p0_calibration.sh:365](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:365).

**Other Notes**

`env_logger::init()` actually precedes Trust Root verification at [evaluator.rs:164](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:164), contrary to RQ1.b’s wording. I do not treat that as a blocker: no LLM call, JSONL emit, or problem load occurs before boot verification.

Targeted verification run: `cargo test -p minif2f_v4 trust_root -- --nocapture` passed. I also verified `compute_p0.py` returns exit `2` for an all-regression dataset.

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED again. Remove or abort the `problem_file_missing` synthetic path, add a pre-loop adaptation-file existence preflight, and make evaluator boot preflight assert an expected nonzero exit/status instead of grepping output after `|| true`.
