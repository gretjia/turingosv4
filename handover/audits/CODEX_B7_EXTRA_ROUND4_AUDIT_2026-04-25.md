# Codex PPUT-CCL Phase B B7-extra ROUND-4 RE-AUDIT (post-VETO-fixes)
**Date**: 2026-04-25
**Predecessor**: CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md (verdict VETO)
**Fixes commits**: 15b87fb (Phase A+B+C) + 438a648 (code-simplifier) + c061450 (constitution) + df77b0a (audit packet record)
**Test baseline**: 187/187 PASS + 20 ignored
**Smoke**: control SOLVED tx=15, treatment UNSOLVED tx=50 with synthetic_short_circuit=true
**Negative test**: tamper Trust Root → evaluator panic at boot ✓
**Prompt size**: 96891 chars

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
session id: 019dc5da-ed1a-70b3-8364-8bdd9c4c2810
--------
user
# Codex ROUND-4 RE-AUDIT — Phase B B7-extra (after VETO fixes)

**Role**: skeptical adversarial reviewer. Independent of Gemini. CLAUDE.md "Audit Standard": VETO > CHALLENGE > PASS; conservative reading wins on disagreement.

**Mandate**: this is the FOURTH audit pass.
- **Round 1** VETO (B1+B2+B3 + Gemini Q2.b/Q2.e). Fixed in `15b87fb`.
- **Round 2** Codex VETO again — round-1 "synthetic UNSOLVED on any non-zero exit" silently absorbed TRUST_ROOT_TAMPERED panics. Fixed in `1df1f62` (crash-vs-timeout discrimination).
- **Round 3** Codex CHALLENGE: same silent-absorption class via `problem_file_missing` synthetic emit (verified all-missing 144×2 → p0=0.0); also boot preflight `|| true` discards exit code. Gemini CHALLENGE: EXIT=0+empty PPUT_RESULT case fell through to generic crash branch. **All addressed in commit `d0d474e`**.
- **Round 4 (this audit)**: determine whether `d0d474e` resolves the round-3 CHALLENGE findings AND whether it introduces NEW defects. **If no new P0 defect and round-3 CHALLENGE items are addressed: PASS → 576-run batch can launch overnight.** User has authorized auto-research overnight to proceed on a clean audit.

**Round-3 fix summary (commit `d0d474e`)**:
1. Pre-loop adaptation-file existence preflight (Codex P0 #1): iterates ADAPTATION_IDS, checks each `$MINIF2F_DIR/MiniF2F/Test/${pid}.lean` exists, lists missing PIDs (up to 10), aborts with exit 2 if any missing. In-loop missing-file branch downgraded to race-condition guard (exit 6) — should never trigger after preflight.
2. Boot preflight exit-code assertion (Codex P0 #2): captures PREFLIGHT_EXIT explicitly; asserts exit != 0 AND exit != 124; then content grep for TRUST_ROOT_TAMPERED. Three failure modes (exit 0, exit 124, panic with TRUST_ROOT_TAMPERED) get distinct diagnostic messages.
3. EXIT=0 + empty PPUT_RESULT explicit branch (Gemini CHALLENGE): malformed-run case now exits 5 with dedicated diagnostic, instead of falling into generic crash branch.

**Verification done**:
- 187/187 cargo test --workspace PASS + 20 ignored
- Happy-path smoke (--smoke, 40s) all SOLVED tx=1; new preflight steps print OK; no abort

**Decision rule**:
- If round-3 fix resolves all round-3 CHALLENGE findings AND no new P0: **PASS** with HIGH conviction → batch authorized.
- If new P0 introduced: **CHALLENGE** with specific items.
- If deeper architectural issue: **VETO**.

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
"handover/preregistration/scripts/run_p0_calibration.sh" = "323c7f9d23209447b253048c7017f394b6b6ac24f162057863da331e54a148bb"
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

# Audit-fix 2026-04-25 round-2 (Codex VETO) + round-3 (Codex P0 #2):
# evaluator boot preflight with EXIT-CODE assertion + content grep.
# A nonexistent problem MUST cause evaluator to exit non-zero AND not
# timeout. Round-3 Codex caught: if preflight times out (124) or exits 0
# (impossible-but-defensive), runner falsely printed "OK" because grep
# alone doesn't surface those failure modes.
echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
PREFLIGHT_EXIT=0
PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
if [ "$PREFLIGHT_EXIT" -eq 0 ]; then
    echo "PREFLIGHT FAIL: evaluator exited 0 on nonexistent problem path"
    echo "  (expected: non-zero exit due to problem-not-found OR Trust Root"
    echo "   panic). exit=0 means a code path silently succeeded with bad input."
    echo "$PREFLIGHT_PROBE" | head -c 800
    exit 2
fi
if [ "$PREFLIGHT_EXIT" -eq 124 ]; then
    echo "PREFLIGHT FAIL: evaluator timed out (exit 124) at boot."
    echo "  Trust Root verify or env_logger init may be hanging. Investigate."
    echo "$PREFLIGHT_PROBE" | head -c 800
    exit 2
fi
if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
    echo "$PREFLIGHT_PROBE" | head -c 800
    exit 2
fi
# Expected: evaluator exits with usage error or problem-not-found (non-zero,
# not 124, no Trust Root panic). Other panics indicate boot regression.
if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly (exit=$PREFLIGHT_EXIT):"
    echo "$PREFLIGHT_PROBE" | head -c 800
    exit 2
fi
echo "Evaluator boot preflight OK (exit=$PREFLIGHT_EXIT, no Trust Root panic, no timeout)."

# Round-3 Codex P0 #1 fix: pre-loop adaptation-file existence preflight.
# A missing problem file MUST abort the batch, not produce a synthetic
# UNSOLVED row. Codex verified a 144×2 all-missing dataset returned
# p0=0.0 — same silent-absorption class as the round-2 VETO. The
# correct posture: every (problem, seed, mode) coordinate that the
# strict-complete estimator expects must come from a real evaluator run
# (or a legitimate timeout). Missing problem file = setup error =
# operator must investigate before launching.
echo "[$(date -Is)] Adaptation file existence preflight..."
MISSING_FILES=()
while IFS= read -r PID; do
    [ -z "$PID" ] && continue
    PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
    if [ ! -f "$PROBLEM" ]; then
        MISSING_FILES+=("$PID")
    fi
done <<< "$ADAPTATION_IDS"
if [ "${#MISSING_FILES[@]}" -gt 0 ]; then
    echo "PREFLIGHT FAIL: ${#MISSING_FILES[@]} adaptation problem file(s) missing:"
    for pid in "${MISSING_FILES[@]:0:10}"; do
        echo "  - $pid (expected at $MINIF2F_DIR/MiniF2F/Test/${pid}.lean)"
    done
    [ "${#MISSING_FILES[@]}" -gt 10 ] && echo "  ... and $((${#MISSING_FILES[@]} - 10)) more"
    echo ""
    echo "Refuse to launch batch with incomplete adaptation set."
    echo "Investigate MINIF2F_DIR ($MINIF2F_DIR) and re-verify file presence."
    exit 2
fi
echo "Adaptation file existence preflight OK ($(echo "$ADAPTATION_IDS" | wc -l) files present)."

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
                # Round-3 Codex P0 #1 fix: this should never trigger now —
                # the pre-loop adaptation-file existence preflight (above)
                # already aborted the batch if any file was missing. Race
                # condition (file deleted mid-batch) = abort, no synthetic
                # row. Silent absorption class.
                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND mid-batch (race)"
                echo "  ✗ File disappeared between preflight and run. ABORTING."
                exit 6
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
            # Round-3 Gemini CHALLENGE fix: explicitly handle the EXIT=0 +
            # empty PPUT_JSON case (e.g., evaluator silently exits 0 without
            # emitting PPUT_RESULT — malformed run, not a legitimate UNSOLVED).
            # Without this branch the malformed run falls into the generic
            # crash branch with a misleading "exit=0" message.
            if [ "$EXIT" -eq 0 ] && [ -z "$PPUT_JSON" ]; then
                echo "MALFORMED (exit=0 but no PPUT_RESULT line) — ABORTING BATCH"
                echo ""
                echo "  ✗ Evaluator returned 0 but emitted no PPUT_RESULT line."
                echo "  ✗ This indicates a code bug (silent success path missing emit)"
                echo "  ✗ rather than a runtime failure. Calibration data NOT trusted."
                echo "  ✗ Last 5 stderr lines:"
                tail -5 "$STDERR_LOG" | sed 's/^/    /'
                exit 5
            elif [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then
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
I’ll audit against the checked-out tree, not only the pasted excerpts. I’m first loading the repo instructions and the exact post-fix files so line citations point at the current workspace state.
exec
/bin/bash -lc 'git status --short && git rev-parse HEAD && git log --oneline -8' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M rules/enforcement.log
?? handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md
?? handover/audits/run_codex_b7_extra_round4_audit.sh
?? handover/audits/run_gemini_b7_extra_round4_audit.py
d0d474e5e09790d684c6ec0ba09229368d9f20ec
d0d474e B7-extra round-3 audit + fixes (Codex CHALLENGE / Gemini CHALLENGE)
584001a B7-extra round-2 audit packet (Codex VETO / Gemini PASS)
1df1f62 B7-extra round-2 fix: Codex VETO on crash-as-data + 2 minor
438a648 B7-extra code-simplifier pass (Karpathy aesthetic, 无损压缩即智能)
15b87fb B7-extra dual-audit fixes (Phase A+B+C)
c061450 Constitutional amendment 2026-04-25 (sudo)
df77b0a B7-extra pre-batch dual audit: VETO/VETO
1875543 Thesis-v2 audit — ground-truth feedback alignment

exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'Cargo.toml'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
experiments/minif2f_v4/Cargo.toml
CLAUDE.md
Cargo.toml

codex
The checkout is at `d0d474e`, with a dirty/untracked audit artifact set already present. I’m treating those as pre-existing and will not modify them.
exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
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
/bin/bash -lc 'find /home/zephryj/projects/turingosv4 -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "find /home/zephryj -path '*/AGENTS.md' -print" in /home/zephryj/projects/turingosv4
 succeeded in 1469ms:
2026-04-25T18:16:28.256131Z ERROR codex_core::tools::router: error=write_stdin failed: stdin is closed for this session; rerun exec_command with tty=true to keep stdin open
 succeeded in 16711ms:
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.48/AGENTS.md
/home/zephryj/.local/share/uv/tools/kimi-cli/lib/python3.13/site-packages/kimi_cli/acp/AGENTS.md
/home/zephryj/.local/share/uv/tools/kimi-cli/lib/python3.13/site-packages/kimi_cli/tools/AGENTS.md
/home/zephryj/.codex/.tmp/plugins/plugins/build-web-apps/skills/react-best-practices/AGENTS.md
/home/zephryj/.codex/.tmp/plugins/plugins/build-web-apps/skills/supabase-best-practices/AGENTS.md
/home/zephryj/projects/turingos/AGENTS.md
/home/zephryj/projects/omega_kepler_discovery/AGENTS.md
/home/zephryj/projects/omega/AGENTS.md
/home/zephryj/projects/turingosv2/AGENTS.md
/home/zephryj/projects/turingclaw/AGENTS.md

codex
No applicable `AGENTS.md` exists under `/home/zephryj/projects/turingosv4`; the repo-level guidance is `CLAUDE.md`, whose audit rule matches your requested VETO > CHALLENGE > PASS standard. I’m now checking the actual line-level implementation and looking specifically for silent absorption paths.
exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '150,210p'" in /home/zephryj/projects/turingosv4
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
   134	# Audit-fix 2026-04-25 round-2 (Codex VETO) + round-3 (Codex P0 #2):
   135	# evaluator boot preflight with EXIT-CODE assertion + content grep.
   136	# A nonexistent problem MUST cause evaluator to exit non-zero AND not
   137	# timeout. Round-3 Codex caught: if preflight times out (124) or exits 0
   138	# (impossible-but-defensive), runner falsely printed "OK" because grep
   139	# alone doesn't surface those failure modes.
   140	echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
   141	PREFLIGHT_EXIT=0
   142	PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
   143	if [ "$PREFLIGHT_EXIT" -eq 0 ]; then
   144	    echo "PREFLIGHT FAIL: evaluator exited 0 on nonexistent problem path"
   145	    echo "  (expected: non-zero exit due to problem-not-found OR Trust Root"
   146	    echo "   panic). exit=0 means a code path silently succeeded with bad input."
   147	    echo "$PREFLIGHT_PROBE" | head -c 800
   148	    exit 2
   149	fi
   150	if [ "$PREFLIGHT_EXIT" -eq 124 ]; then
   151	    echo "PREFLIGHT FAIL: evaluator timed out (exit 124) at boot."
   152	    echo "  Trust Root verify or env_logger init may be hanging. Investigate."
   153	    echo "$PREFLIGHT_PROBE" | head -c 800
   154	    exit 2
   155	fi
   156	if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
   157	    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
   158	    echo "$PREFLIGHT_PROBE" | head -c 800
   159	    exit 2
   160	fi
   161	# Expected: evaluator exits with usage error or problem-not-found (non-zero,
   162	# not 124, no Trust Root panic). Other panics indicate boot regression.
   163	if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
   164	   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
   165	    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly (exit=$PREFLIGHT_EXIT):"
   166	    echo "$PREFLIGHT_PROBE" | head -c 800
   167	    exit 2
   168	fi
   169	echo "Evaluator boot preflight OK (exit=$PREFLIGHT_EXIT, no Trust Root panic, no timeout)."
   170	
   171	# Round-3 Codex P0 #1 fix: pre-loop adaptation-file existence preflight.
   172	# A missing problem file MUST abort the batch, not produce a synthetic
   173	# UNSOLVED row. Codex verified a 144×2 all-missing dataset returned
   174	# p0=0.0 — same silent-absorption class as the round-2 VETO. The
   175	# correct posture: every (problem, seed, mode) coordinate that the
   176	# strict-complete estimator expects must come from a real evaluator run
   177	# (or a legitimate timeout). Missing problem file = setup error =
   178	# operator must investigate before launching.
   179	echo "[$(date -Is)] Adaptation file existence preflight..."
   180	MISSING_FILES=()
   181	while IFS= read -r PID; do
   182	    [ -z "$PID" ] && continue
   183	    PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
   184	    if [ ! -f "$PROBLEM" ]; then
   185	        MISSING_FILES+=("$PID")
   186	    fi
   187	done <<< "$ADAPTATION_IDS"
   188	if [ "${#MISSING_FILES[@]}" -gt 0 ]; then
   189	    echo "PREFLIGHT FAIL: ${#MISSING_FILES[@]} adaptation problem file(s) missing:"
   190	    for pid in "${MISSING_FILES[@]:0:10}"; do
   191	        echo "  - $pid (expected at $MINIF2F_DIR/MiniF2F/Test/${pid}.lean)"
   192	    done
   193	    [ "${#MISSING_FILES[@]}" -gt 10 ] && echo "  ... and $((${#MISSING_FILES[@]} - 10)) more"
   194	    echo ""
   195	    echo "Refuse to launch batch with incomplete adaptation set."
   196	    echo "Investigate MINIF2F_DIR ($MINIF2F_DIR) and re-verify file presence."
   197	    exit 2
   198	fi
   199	echo "Adaptation file existence preflight OK ($(echo "$ADAPTATION_IDS" | wc -l) files present)."
   200	
   201	# Run loop. Each (mode, seed, problem) combination = 1 run.
   202	TOTAL_PROBLEMS=$(echo "$ADAPTATION_IDS" | wc -l)
   203	TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
   204	CANARY_START=$(date -Is)
   205	echo ""
   206	echo "=== p_0 calibration ==="
   207	echo "Mode count:    ${#MODES[@]} (control + treatment)"
   208	echo "Seed count:    ${#SEEDS[@]} (${SEEDS[*]})"
   209	echo "Problem count: $TOTAL_PROBLEMS"
   210	echo "Total runs:    $TOTAL_RUNS"
   211	echo "MODEL_SNAPSHOT: $MODEL_SNAPSHOT"
   212	echo "BUILD_SHA:     $BUILD_SHA"
   213	echo "Canary start:  $CANARY_START"
   214	echo ""
   215	
   216	# Audit-fix Gemini Q7.b: emit a valid UNSOLVED jsonl row on timeout/crash so
   217	# strict-completeness compute_p0 join sees every pair. The synthesized row
   218	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
   219	# disambiguator so downstream tooling can distinguish a timeout from a real
   220	# UNSOLVED.
   221	emit_synthetic_unsolved() {
   222	    # args: out_file mode seed pid reason exit_code
   223	    # Emits a v2 RunAggregate-conformant row — Codex re-audit CHALLENGE 2:
   224	    # `golden_path_token_count` is required by jsonl_schema.rs RunAggregate
   225	    # when schema_version == "v2.0". Synthetic rows MUST set it explicitly
   226	    # so downstream v2 tooling parses cleanly.
   227	    # USED ONLY for legitimate timeout (exit 124). Crash paths now ABORT
   228	    # the batch instead — see in-loop comment on the elif branch.
   229	    python3 - <<EOF >> "$1"
   230	import json, time
   231	print(json.dumps({
   232	    "schema_version": "v2.0",
   233	    "run_id": "synthetic_${2}_${4}_$(date +%s)",
   234	    "problem_id": "$4",
   235	    "solved": False,
   236	    "verified": False,
   237	    "progress": 0,
   238	    "split": "adaptation",
   239	    "calibration_mode": "$2",
   240	    "calibration_seed": $3,
   241	    "calibration_problem_id": "$4",
   242	    "synthetic_timeout_or_crash": True,
   243	    "synthetic_reason": "$5",
   244	    "synthetic_exit_code": $6,
   245	    "model_snapshot": "$MODEL_SNAPSHOT",
   246	    "build_sha": "$BUILD_SHA",
   247	    "boltzmann_seed": $3,
   248	    "tx_count": 0,
   249	    "golden_path_token_count": 0,
   250	    "total_run_token_count": 0,
   251	    "total_wall_time_ms": 0,
   252	    "pput_runtime": 0.0,
   253	    "pput_verified": 0.0,
   254	    "pput_m_verified": 0.0,
   255	    "failed_branch_count": 0,
   256	    "rollback_count": 0,
   257	    "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0,
   258	    "git_sha": "$BUILD_SHA",
   259	    "binary_sha256": "",
   260	    "mode": "full",
   261	    "problem": "${MINIF2F_DIR}/MiniF2F/Test/${4}.lean",
   262	    "condition": "$CONDITION",
   263	    "model": "$ACTIVE_MODEL",
   264	    "has_golden_path": False,
   265	    "time_secs": 0.0,
   266	    "pput": 0.0,
   267	    "gp_token_count": 0,
   268	    "gp_node_count": 0,
   269	}))
   270	EOF
   271	}
   272	
   273	BATCH_START=$(date +%s)
   274	RUN_IDX=0
   275	for MODE in "${MODES[@]}"; do
   276	    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
   277	    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
   278	    : > "$OUT_FILE"
   279	    : > "$STDERR_LOG"
   280	    case "$MODE" in
   281	        control)   ROLLBACK_FLAG="" ;;
   282	        treatment) ROLLBACK_FLAG="1" ;;
   283	    esac
   284	    for SEED in "${SEEDS[@]}"; do
   285	        while IFS= read -r PID; do
   286	            [ -z "$PID" ] && continue
   287	            RUN_IDX=$((RUN_IDX + 1))
   288	            PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
   289	            if [ ! -f "$PROBLEM" ]; then
   290	                # Round-3 Codex P0 #1 fix: this should never trigger now —
   291	                # the pre-loop adaptation-file existence preflight (above)
   292	                # already aborted the batch if any file was missing. Race
   293	                # condition (file deleted mid-batch) = abort, no synthetic
   294	                # row. Silent absorption class.
   295	                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND mid-batch (race)"
   296	                echo "  ✗ File disappeared between preflight and run. ABORTING."
   297	                exit 6
   298	            fi
   299	            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
   300	            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
   301	            # Note: `set -e` is bypassed for this single command via `|| EXIT=$?`
   302	            # so timeout/crash flows into the synthetic-UNSOLVED branch instead
   303	            # of aborting the entire batch.
   304	            EXIT=0
   305	            OUTPUT=$(timeout 2400 env \
   306	                CONDITION="$CONDITION" \
   307	                MINIF2F_DIR="$MINIF2F_DIR" \
   308	                BOLTZMANN_SEED="$SEED" \
   309	                SIMULATE_ROLLBACK_AT_TX_50="$ROLLBACK_FLAG" \
   310	                MODEL_SNAPSHOT="$MODEL_SNAPSHOT" \
   311	                BUILD_SHA="$BUILD_SHA" \
   312	                SPLIT="adaptation" \
   313	                RUST_LOG=info \
   314	                "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || EXIT=$?
   315	            PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1 || true)
   316	            # Round-3 Gemini CHALLENGE fix: explicitly handle the EXIT=0 +
   317	            # empty PPUT_JSON case (e.g., evaluator silently exits 0 without
   318	            # emitting PPUT_RESULT — malformed run, not a legitimate UNSOLVED).
   319	            # Without this branch the malformed run falls into the generic
   320	            # crash branch with a misleading "exit=0" message.
   321	            if [ "$EXIT" -eq 0 ] && [ -z "$PPUT_JSON" ]; then
   322	                echo "MALFORMED (exit=0 but no PPUT_RESULT line) — ABORTING BATCH"
   323	                echo ""
   324	                echo "  ✗ Evaluator returned 0 but emitted no PPUT_RESULT line."
   325	                echo "  ✗ This indicates a code bug (silent success path missing emit)"
   326	                echo "  ✗ rather than a runtime failure. Calibration data NOT trusted."
   327	                echo "  ✗ Last 5 stderr lines:"
   328	                tail -5 "$STDERR_LOG" | sed 's/^/    /'
   329	                exit 5
   330	            elif [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then

exec
/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
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
/bin/bash -lc "nl -ba src/boot.rs | sed -n '1,230p'" in /home/zephryj/projects/turingosv4
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
   221	    fn verify_trust_root_passes_on_intact_repo() {
   222	        verify_trust_root(&repo_root()).expect("intact repo verifies");
   223	    }
   224	
   225	    /// Write a single-entry [trust_root] manifest pointing at `only.txt`
   226	    /// with the given hex hash. Used by both tamper and match tests.
   227	    fn write_single_entry_repo(tmp: &Path, only_txt: &str, manifest_hash: &str) {
   228	        let genesis = format!(
   229	            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n\
   230	             [trust_root]\n\"only.txt\" = \"{manifest_hash}\"\n"

exec
/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '330,460p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   330	            elif [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then
   331	                ENRICHED=$(printf '%s' "$PPUT_JSON" | MODE_ENV="$MODE" SEED_ENV="$SEED" PID_ENV="$PID" python3 -c "
   332	import json, os, sys
   333	row = json.loads(sys.stdin.read())
   334	row['calibration_mode'] = os.environ['MODE_ENV']
   335	row['calibration_seed'] = int(os.environ['SEED_ENV'])
   336	row['calibration_problem_id'] = os.environ['PID_ENV']
   337	print(json.dumps(row))
   338	")
   339	                echo "$ENRICHED" >> "$OUT_FILE"
   340	                TX=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tx_count', 0))")
   341	                SOLVED_FLAG=$(echo "$ENRICHED" | python3 -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('progress', 1 if r.get('has_golden_path') else 0)))")
   342	                if [ "$SOLVED_FLAG" = "1" ]; then
   343	                    echo "SOLVED (tx=$TX)"
   344	                else
   345	                    echo "UNSOLVED (tx=$TX)"
   346	                fi
   347	            elif [ "$EXIT" -eq 124 ]; then
   348	                # Audit-fix Gemini Q7.b: timeout is a legitimate UNSOLVED outcome
   349	                # under a fixed wall-clock budget. Emit synthetic row with
   350	                # synthetic_timeout_or_crash=true disambiguator.
   351	                echo "TIMEOUT (exit=124) — emitting synthetic UNSOLVED row"
   352	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "timeout_2400s" 124
   353	            else
   354	                # Audit-fix Codex re-audit VETO 2026-04-25: any non-timeout
   355	                # non-zero exit (Rust panic 101, segfault 139, OOM 137, etc.) is
   356	                # NOT a legitimate UNSOLVED outcome. It indicates batch corruption.
   357	                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
   358	                # rather than be silently absorbed as UNSOLVED data — otherwise
   359	                # the entire calibration could complete with all-crash rows
   360	                # and produce a "valid" p_0=0 that gets frozen into Trust Root.
   361	                # No synthetic row emitted; partial calibration is forfeited.
   362	                echo "CRASH (exit=$EXIT) — ABORTING BATCH"
   363	                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
   364	                    echo ""
   365	                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
   366	                    echo "  ✗ Boot integrity check failed; investigate manifest vs filesystem state."
   367	                    echo "  ✗ Diagnostic stderr (tail):"
   368	                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
   369	                    exit 3
   370	                fi
   371	                echo ""
   372	                echo "  ✗ Evaluator crashed with exit=$EXIT (not a timeout)."
   373	                echo "  ✗ Calibration data NOT trusted; partial jsonl preserved at $OUT_FILE for diagnosis."
   374	                echo "  ✗ Last 5 stderr lines:"
   375	                tail -5 "$STDERR_LOG" | sed 's/^/    /'
   376	                exit 4
   377	            fi
   378	        done <<< "$ADAPTATION_IDS"
   379	    done
   380	done
   381	
   382	CANARY_END=$(date -Is)
   383	BATCH_END=$(date +%s)
   384	WALL_TIME=$((BATCH_END - BATCH_START))
   385	
   386	echo ""
   387	echo "╔═══════════════════════════════════════════╗"
   388	echo "║   p_0 CALIBRATION SUMMARY"
   389	echo "╠═══════════════════════════════════════════╣"
   390	echo "║ Wall time:        ${WALL_TIME}s"
   391	echo "║ Canary start:     $CANARY_START"
   392	echo "║ Canary end:       $CANARY_END"
   393	echo "║ MODEL_SNAPSHOT:   $MODEL_SNAPSHOT"
   394	echo "║ BUILD_SHA:        $BUILD_SHA"
   395	echo "║ Control jsonl:    ${OUT_PREFIX}_control.jsonl"
   396	echo "║ Treatment jsonl:  ${OUT_PREFIX}_treatment.jsonl"
   397	echo "╚═══════════════════════════════════════════╝"
   398	echo ""
   399	
   400	# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
   401	# = ceiling abort). For smoke modes we skip the estimator (sample size too
   402	# small to be meaningful) and just print the diagnostic snippet.
   403	if [ "$SMOKE" -eq 1 ] || [ "$SMOKE_HARD" -eq 1 ]; then
   404	    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
   405	    echo "Diagnostic: head 1 row from each jsonl"
   406	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
   407	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
   408	    exit 0
   409	fi
   410	
   411	# Full batch: estimator MUST run, exit code MUST propagate.
   412	echo "[$(date -Is)] Running p_0 estimator (strict-complete mode)..."
   413	P0_JSON="${OUT_PREFIX}_p0_result.json"
   414	set +e
   415	python3 "$SCRIPT_DIR/compute_p0.py" \
   416	    --control "${OUT_PREFIX}_control.jsonl" \
   417	    --treatment "${OUT_PREFIX}_treatment.jsonl" \
   418	    --out-json "$P0_JSON"
   419	P0_EXIT=$?
   420	set -e
   421	
   422	if [ "$P0_EXIT" -eq 0 ]; then
   423	    echo ""
   424	    echo "✓ p_0 PASSED ceiling. Result: $P0_JSON"
   425	    echo "  Next: ArchitectAI updates genesis_payload.toml [pput_accounting_0]"
   426	    echo "        + Trust Root manifest entry for the calibration jsonl."
   427	elif [ "$P0_EXIT" -eq 2 ]; then
   428	    echo ""
   429	    echo "✗ p_0 EXCEEDS ceiling (>0.10) — PREREG § 5.5 ABORT."
   430	    echo "  Calibration result NOT frozen into genesis_payload.toml."
   431	    echo "  Action: redesign rollback simulation (per PREREG § 5.5), redo."
   432	    exit 2
   433	else
   434	    echo ""
   435	    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
   436	    echo "  Calibration result NOT frozen. Investigate before retry."
   437	    exit "$P0_EXIT"
   438	fi

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
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/rollback_sim.rs | sed -n '1,150p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   181	        } else {
   182	            Ok(RunRecord::Legacy(serde_json::from_value(v)?))
   183	        }
   184	    }
   185	}
   186	
   187	#[cfg(test)]
   188	mod tests {
   189	    use super::*;
   190	
   191	    fn sample_run() -> RunAggregate {
   192	        RunAggregate {
   193	            run_id: "r-001".into(),
   194	            problem_id: "mathd_algebra_44".into(),
   195	            solved: true,
   196	            schema_version: SCHEMA_VERSION_V2.into(),
   197	            split: "adaptation".into(),
   198	            verified: true,
   199	            golden_path_token_count: 512,
   200	            total_run_token_count: 4096,
   201	            total_wall_time_ms: 12_000,
   202	            progress: 1,
   203	            pput_runtime: 0.5,
   204	            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
   205	            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
   206	            failed_branch_count: 3,
   207	            rollback_count: 0,
   208	            far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
   209	            model_snapshot: "deepseek-v4-flash@2026-04-26".into(),
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

exec
/bin/bash -lc "rg -n \"MODEL_SNAPSHOT|BUILD_SHA|synthetic_timeout|synthetic_short|progress|golden_path_token_count|binary_sha256|RunAggregate\" experiments/minif2f_v4/src -g '*.rs' handover/preregistration/scripts -g '*.py' -g '*.sh'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/preregistration/scripts/run_p0_calibration.sh:21:#   - MODEL_SNAPSHOT + GIT_SHA stamped in env for drift detection
handover/preregistration/scripts/run_p0_calibration.sh:66:# Drift-detection provenance (Codex Q7). MODEL_SNAPSHOT seeds the evaluator's
handover/preregistration/scripts/run_p0_calibration.sh:73:export MODEL_SNAPSHOT="${MODEL_SNAPSHOT:-${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}}"
handover/preregistration/scripts/run_p0_calibration.sh:74:export BUILD_SHA="${BUILD_SHA:-${GIT_SHA}${GIT_DIRTY}}"
handover/preregistration/scripts/run_p0_calibration.sh:211:echo "MODEL_SNAPSHOT: $MODEL_SNAPSHOT"
handover/preregistration/scripts/run_p0_calibration.sh:212:echo "BUILD_SHA:     $BUILD_SHA"
handover/preregistration/scripts/run_p0_calibration.sh:218:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
handover/preregistration/scripts/run_p0_calibration.sh:223:    # Emits a v2 RunAggregate-conformant row — Codex re-audit CHALLENGE 2:
handover/preregistration/scripts/run_p0_calibration.sh:224:    # `golden_path_token_count` is required by jsonl_schema.rs RunAggregate
handover/preregistration/scripts/run_p0_calibration.sh:237:    "progress": 0,
handover/preregistration/scripts/run_p0_calibration.sh:242:    "synthetic_timeout_or_crash": True,
handover/preregistration/scripts/run_p0_calibration.sh:245:    "model_snapshot": "$MODEL_SNAPSHOT",
handover/preregistration/scripts/run_p0_calibration.sh:246:    "build_sha": "$BUILD_SHA",
handover/preregistration/scripts/run_p0_calibration.sh:249:    "golden_path_token_count": 0,
handover/preregistration/scripts/run_p0_calibration.sh:258:    "git_sha": "$BUILD_SHA",
handover/preregistration/scripts/run_p0_calibration.sh:259:    "binary_sha256": "",
handover/preregistration/scripts/run_p0_calibration.sh:310:                MODEL_SNAPSHOT="$MODEL_SNAPSHOT" \
handover/preregistration/scripts/run_p0_calibration.sh:311:                BUILD_SHA="$BUILD_SHA" \
handover/preregistration/scripts/run_p0_calibration.sh:341:                SOLVED_FLAG=$(echo "$ENRICHED" | python3 -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('progress', 1 if r.get('has_golden_path') else 0)))")
handover/preregistration/scripts/run_p0_calibration.sh:350:                # synthetic_timeout_or_crash=true disambiguator.
handover/preregistration/scripts/run_p0_calibration.sh:393:echo "║ MODEL_SNAPSHOT:   $MODEL_SNAPSHOT"
handover/preregistration/scripts/run_p0_calibration.sh:394:echo "║ BUILD_SHA:        $BUILD_SHA"
handover/preregistration/scripts/run_p0_calibration.sh:406:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/preregistration/scripts/run_p0_calibration.sh:407:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
experiments/minif2f_v4/src/post_hoc_verifier.rs:30:use crate::jsonl_schema::RunAggregate;
experiments/minif2f_v4/src/post_hoc_verifier.rs:42:/// Use `compute_progress_verified` instead when the runtime gate already
experiments/minif2f_v4/src/post_hoc_verifier.rs:59:pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:65:pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:69:/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
experiments/minif2f_v4/src/post_hoc_verifier.rs:70:/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
experiments/minif2f_v4/src/post_hoc_verifier.rs:71:pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:72:    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/post_hoc_verifier.rs:76:pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:77:    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/post_hoc_verifier.rs:87:    ///  assert progress = 0, pput_verified = 0.0"
experiments/minif2f_v4/src/post_hoc_verifier.rs:97:        // run's progress to 0.
experiments/minif2f_v4/src/post_hoc_verifier.rs:103:        let progress_runtime = compute_progress_runtime(runtime_accepted);
experiments/minif2f_v4/src/post_hoc_verifier.rs:104:        let progress_verified =
experiments/minif2f_v4/src/post_hoc_verifier.rs:105:            compute_progress_verified(runtime_accepted, post_hoc_verified);
experiments/minif2f_v4/src/post_hoc_verifier.rs:107:        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:108:        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:109:        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:111:        assert_eq!(progress_runtime, 1u8,
experiments/minif2f_v4/src/post_hoc_verifier.rs:112:            "runtime gate fired → progress_runtime = 1");
experiments/minif2f_v4/src/post_hoc_verifier.rs:113:        assert_eq!(progress_verified, 0u8,
experiments/minif2f_v4/src/post_hoc_verifier.rs:114:            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
experiments/minif2f_v4/src/post_hoc_verifier.rs:137:        let progress_runtime = compute_progress_runtime(true);
experiments/minif2f_v4/src/post_hoc_verifier.rs:138:        let progress_verified = compute_progress_verified(true, true);
experiments/minif2f_v4/src/post_hoc_verifier.rs:140:        assert_eq!(progress_runtime, progress_verified,
experiments/minif2f_v4/src/post_hoc_verifier.rs:143:            compute_pput(progress_runtime, c_i, t_i_ms),
experiments/minif2f_v4/src/post_hoc_verifier.rs:144:            compute_pput(progress_verified, c_i, t_i_ms),
experiments/minif2f_v4/src/post_hoc_verifier.rs:154:        let progress_runtime = compute_progress_runtime(false);
experiments/minif2f_v4/src/post_hoc_verifier.rs:155:        let progress_verified = compute_progress_verified(false, false);
experiments/minif2f_v4/src/post_hoc_verifier.rs:157:        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:158:        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:162:    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
experiments/minif2f_v4/src/post_hoc_verifier.rs:164:        // fired is a wiring bug, not an honest progress signal. Progress
experiments/minif2f_v4/src/post_hoc_verifier.rs:167:        assert_eq!(compute_progress_verified(false, true), 0u8,
experiments/minif2f_v4/src/jsonl_schema.rs:10:// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
experiments/minif2f_v4/src/jsonl_schema.rs:13:// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
experiments/minif2f_v4/src/jsonl_schema.rs:82:pub struct RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:91:    pub golden_path_token_count: u64,
experiments/minif2f_v4/src/jsonl_schema.rs:97:    pub progress: u8,
experiments/minif2f_v4/src/jsonl_schema.rs:115:    pub binary_sha256: String,
experiments/minif2f_v4/src/jsonl_schema.rs:120:impl RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:122:    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
experiments/minif2f_v4/src/jsonl_schema.rs:123:    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
experiments/minif2f_v4/src/jsonl_schema.rs:125:    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/jsonl_schema.rs:126:        if progress == 0 || c_i == 0 || t_i_ms == 0 {
experiments/minif2f_v4/src/jsonl_schema.rs:130:        (progress as f64) / denom
experiments/minif2f_v4/src/jsonl_schema.rs:134:    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/jsonl_schema.rs:135:        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/jsonl_schema.rs:147:pub struct LegacyRunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:166:    V2(RunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:167:    Legacy(LegacyRunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:191:    fn sample_run() -> RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:192:        RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:199:            golden_path_token_count: 512,
experiments/minif2f_v4/src/jsonl_schema.rs:202:            progress: 1,
experiments/minif2f_v4/src/jsonl_schema.rs:204:            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:205:            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:211:            binary_sha256: "deadbeef".into(),
experiments/minif2f_v4/src/jsonl_schema.rs:220:        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
experiments/minif2f_v4/src/jsonl_schema.rs:221:        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
experiments/minif2f_v4/src/jsonl_schema.rs:227:    fn test_pput_verified_zero_when_progress_zero() {
experiments/minif2f_v4/src/jsonl_schema.rs:230:        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:231:        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:237:        r.progress = 0;
experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:244:        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:245:        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
experiments/minif2f_v4/src/rollback_sim.rs:24:// where `synthetic_short_circuit == Some(true)` MUST disclaim the
experiments/minif2f_v4/src/rollback_sim.rs:26:// `progress` field and the PREREG-frozen seed/problem grid.
experiments/minif2f_v4/src/rollback_sim.rs:79:        // progress. Short-circuit fires exactly once at tx == threshold,
handover/preregistration/scripts/compute_p0.py:38:    """PREREG § 1.3 progress = 1 iff Lean ground-truth verifies golden_path.
handover/preregistration/scripts/compute_p0.py:40:    Reads the v2 RunAggregate field `progress` (jsonl_schema.rs:96). Falls
handover/preregistration/scripts/compute_p0.py:42:    found this function was reading a non-existent `progress_verified`
handover/preregistration/scripts/compute_p0.py:45:    if "progress" in row and row["progress"] is not None:
handover/preregistration/scripts/compute_p0.py:46:        return int(row["progress"]) == 1
experiments/minif2f_v4/src/bin/evaluator.rs:14:    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
experiments/minif2f_v4/src/bin/evaluator.rs:43:/// `RunAggregate` v2 field as a non-Optional, so emitted jsonl rows are
experiments/minif2f_v4/src/bin/evaluator.rs:47:/// reads them; serde silently drops them when parsing as `RunAggregate`
experiments/minif2f_v4/src/bin/evaluator.rs:52:    // ── B1 RunAggregate v2 schema fields (all REQUIRED — non-Optional) ──
experiments/minif2f_v4/src/bin/evaluator.rs:68:    golden_path_token_count: u64,
experiments/minif2f_v4/src/bin/evaluator.rs:74:    progress: u8,
experiments/minif2f_v4/src/bin/evaluator.rs:75:    /// B4 dual-PPUT: pput_runtime = progress_runtime / (C_i × T_i / 1000).
experiments/minif2f_v4/src/bin/evaluator.rs:77:    /// B4 dual-PPUT: pput_verified = progress_verified / (C_i × T_i / 1000).
experiments/minif2f_v4/src/bin/evaluator.rs:98:    binary_sha256: String,
experiments/minif2f_v4/src/bin/evaluator.rs:110:    gp_token_count: u64,           // alias of golden_path_token_count
experiments/minif2f_v4/src/bin/evaluator.rs:148:    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
experiments/minif2f_v4/src/bin/evaluator.rs:155:    synthetic_short_circuit: Option<bool>,
experiments/minif2f_v4/src/bin/evaluator.rs:559:                   MUST honor synthetic_short_circuit=true on this row)", tx);
experiments/minif2f_v4/src/bin/evaluator.rs:570:            // analysis. See PputResult::synthetic_short_circuit doc-comment
experiments/minif2f_v4/src/bin/evaluator.rs:572:            result.synthetic_short_circuit = Some(true);
experiments/minif2f_v4/src/bin/evaluator.rs:1188:    let build_sha = std::env::var("BUILD_SHA").ok();
experiments/minif2f_v4/src/bin/evaluator.rs:1201:    let progress_runtime = compute_progress_runtime(runtime_accepted);
experiments/minif2f_v4/src/bin/evaluator.rs:1202:    let progress_verified =
experiments/minif2f_v4/src/bin/evaluator.rs:1203:        compute_progress_verified(runtime_accepted, post_hoc_verified);
experiments/minif2f_v4/src/bin/evaluator.rs:1204:    let pput_runtime = compute_pput(progress_runtime, c_i, t_i);
experiments/minif2f_v4/src/bin/evaluator.rs:1205:    let pput_verified = compute_pput(progress_verified, c_i, t_i);
experiments/minif2f_v4/src/bin/evaluator.rs:1206:    let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i);
experiments/minif2f_v4/src/bin/evaluator.rs:1216:    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
experiments/minif2f_v4/src/bin/evaluator.rs:1219:    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();
experiments/minif2f_v4/src/bin/evaluator.rs:1242:        golden_path_token_count: gp_tokens,
experiments/minif2f_v4/src/bin/evaluator.rs:1245:        progress: progress_verified,
experiments/minif2f_v4/src/bin/evaluator.rs:1255:        binary_sha256,
experiments/minif2f_v4/src/bin/evaluator.rs:1277:        synthetic_short_circuit: None,
experiments/minif2f_v4/src/bin/evaluator.rs:1375:                assert_eq!(agg.progress, 1u8);
experiments/minif2f_v4/src/bin/evaluator.rs:1395:    /// emitted v2 row must show progress=0 and pput_verified=0 even with
experiments/minif2f_v4/src/bin/evaluator.rs:1414:        assert_eq!(result.progress, 0u8,
experiments/minif2f_v4/src/bin/evaluator.rs:1415:            "Lean rejected → progress MUST be 0 (North Star truth)");

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
    72	"handover/preregistration/scripts/run_p0_calibration.sh" = "323c7f9d23209447b253048c7017f394b6b6ac24f162057863da331e54a148bb"
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
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '520,590p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1180,1285p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1180	    //
  1181	    // Phase B all callers pass `(runtime_accepted, post_hoc_verified) = (X, X)`
  1182	    // because runtime IS Lean today. Phase C diverges at the Soft Law
  1183	    // mode call site, not inside this function.
  1184	    let has_gp = runtime_accepted; // legacy `has_golden_path` field semantics
  1185	    let elapsed = start.elapsed().as_secs_f64();
  1186	    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
  1187	    // C-012 provenance: populated from env vars; None when unset (backward compat).
  1188	    let build_sha = std::env::var("BUILD_SHA").ok();
  1189	    let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
  1190	    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
  1191	        .ok().and_then(|s| s.parse::<u64>().ok());
  1192	
  1193	    // Mid-term audit P0-B fix 2026-04-25: collapse Optional accumulator/clock
  1194	    // values into required v2 fields. Phase B always has values for these
  1195	    // (B2 + B3 wire them at every emit site); the prior Option wrapping was
  1196	    // overly defensive and let the v2 schema slip from the contract.
  1197	    let c_i = total_run_token_count.unwrap_or(0);
  1198	    let t_i = total_wall_time_ms.unwrap_or(0);
  1199	    let failed_count = failed_branch_count.unwrap_or(0);
  1200	
  1201	    let progress_runtime = compute_progress_runtime(runtime_accepted);
  1202	    let progress_verified =
  1203	        compute_progress_verified(runtime_accepted, post_hoc_verified);
  1204	    let pput_runtime = compute_pput(progress_runtime, c_i, t_i);
  1205	    let pput_verified = compute_pput(progress_verified, c_i, t_i);
  1206	    let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i);
  1207	
  1208	    // V2 fields read from env (per-process globals).
  1209	    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
  1210	        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
  1211	                   (Phase B convention; pre-registration requires SPLIT \
  1212	                   for Phase C+ ablation runs)");
  1213	        "adaptation".to_string()
  1214	    });
  1215	    let mode = std::env::var("MODE").unwrap_or_else(|_| "full".to_string());
  1216	    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
  1217	        .unwrap_or_else(|_| model.to_string());
  1218	    let git_sha = build_sha.clone().unwrap_or_default();
  1219	    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();
  1220	
  1221	    // problem_id = basename without .lean
  1222	    let problem_id = std::path::Path::new(problem)
  1223	        .file_stem()
  1224	        .and_then(|s| s.to_str())
  1225	        .unwrap_or(problem)
  1226	        .to_string();
  1227	    // run_id = condition + problem_id + ts (collision-free for sequential runs)
  1228	    let ts = std::time::SystemTime::now()
  1229	        .duration_since(std::time::UNIX_EPOCH)
  1230	        .map(|d| d.as_millis())
  1231	        .unwrap_or(0);
  1232	    let run_id = format!("{}_{}_{}", condition, problem_id, ts);
  1233	
  1234	    PputResult {
  1235	        // ── B1 v2 schema fields ──
  1236	        schema_version: "v2.0".to_string(),
  1237	        run_id,
  1238	        problem_id,
  1239	        solved: runtime_accepted,
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

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// MiniF2F v4 Evaluator — oneshot and swarm modes
     2	//
     3	// Sole optimization metric: PPUT (Progress Per Unit Time)
     4	//   Progress = 100% if Golden Path exists (OMEGA reached), 0% otherwise
     5	//   PPUT = 100% / time_to_omega (seconds)
     6	//   No GP → PPUT = 0 → problem not worth attacking in current iteration
     7	//
     8	// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)
     9	
    10	use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
    11	use minif2f_v4::cost_aggregator::RunCostAccumulator;
    12	use minif2f_v4::wall_clock::RunWallClock;
    13	use minif2f_v4::post_hoc_verifier::{
    14	    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
    15	};
    16	use turingosv4::bus::{BusConfig, BusResult, TuringBus};
    17	use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
    18	use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
    19	use turingosv4::kernel::Kernel;
    20	use turingosv4::sdk::actor::{BoltzmannParams, boltzmann_select_parent};
    21	use turingosv4::sdk::prompt::build_agent_prompt;
    22	use turingosv4::sdk::prompt_guard::assert_no_metric_leak;
    23	use turingosv4::sdk::protocol::parse_agent_output;
    24	use turingosv4::sdk::tools::wallet::WalletTool;
    25	use turingosv4::sdk::tools::search::SearchTool;
    26	use turingosv4::sdk::tools::librarian::LibrarianTool;
    27	
    28	use std::collections::{HashMap, HashSet};
    29	use std::hash::{Hash, Hasher};
    30	use std::path::PathBuf;
    31	use std::time::Instant;
    32	use log::{info, warn, error};
    33	use rand::SeedableRng;
    34	use rand::rngs::StdRng;
    35	
    36	const DEFAULT_BOLTZMANN_SEED: u64 = 74677;  // same as sample seed (BTC/USD external)
    37	
    38	const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";
    39	
    40	/// PPUT result for a single problem — the only output that matters.
    41	///
    42	/// Mid-term audit P0-B fix 2026-04-25: this struct now carries every B1
    43	/// `RunAggregate` v2 field as a non-Optional, so emitted jsonl rows are
    44	/// dispatched as `RunRecord::V2` by `RunRecord::from_json` (presence of
    45	/// `schema_version` is the discriminant). Legacy diagnostic fields below
    46	/// are kept as Option/skip-if-None for downstream tooling that already
    47	/// reads them; serde silently drops them when parsing as `RunAggregate`
    48	/// (no `deny_unknown_fields`), so V2-tooling reads the v2 contract while
    49	/// PputResult-tooling sees the full diagnostic envelope.
    50	#[derive(Debug, serde::Serialize)]
    51	struct PputResult {
    52	    // ── B1 RunAggregate v2 schema fields (all REQUIRED — non-Optional) ──
    53	    /// Always "v2.0" — RunRecord::from_json discriminator.
    54	    schema_version: String,
    55	    /// Per-run identifier: condition + problem + timestamp.
    56	    run_id: String,
    57	    /// Problem identifier: theorem stem (basename of .lean without extension).
    58	    problem_id: String,
    59	    /// Legacy "did the run reach OMEGA" boolean (= runtime_accepted in B4 vocab).
    60	    /// B1 v2 mandates this as `solved: bool`.
    61	    solved: bool,
    62	    /// "adaptation" | "meta_validation" | "heldout" — read from SPLIT env;
    63	    /// default "adaptation" with stderr warning per Phase B convention.
    64	    split: String,
    65	    /// B4 dual-PPUT: post-hoc Lean verified result. Phase B == solved.
    66	    verified: bool,
    67	    /// Token count of the winning golden path (0 if no GP).
    68	    golden_path_token_count: u64,
    69	    /// B2 C_i — full-run token cost across all proposals.
    70	    total_run_token_count: u64,
    71	    /// B3 T_i — first agent prompt → final Lean call, in milliseconds.
    72	    total_wall_time_ms: u64,
    73	    /// 0 or 1 — Lean ground truth (= 1 iff runtime_accepted AND post_hoc_verified).
    74	    progress: u8,
    75	    /// B4 dual-PPUT: pput_runtime = progress_runtime / (C_i × T_i / 1000).
    76	    pput_runtime: f64,
    77	    /// B4 dual-PPUT: pput_verified = progress_verified / (C_i × T_i / 1000).
    78	    pput_verified: f64,
    79	    /// 10^6 × pput_verified — display unit per PREREG § 5.
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

exec
/bin/bash -lc 'git show --stat --oneline d0d474e && git show --name-only --oneline d0d474e' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
d0d474e B7-extra round-3 audit + fixes (Codex CHALLENGE / Gemini CHALLENGE)
 genesis_payload.toml                               |    2 +-
 handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md        |   15 +
 .../CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md      | 4708 ++++++++++++++++++++
 .../GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md     |   69 +
 handover/audits/run_codex_b7_extra_round3_audit.sh |  191 +
 .../audits/run_gemini_b7_extra_round3_audit.py     |  221 +
 .../preregistration/scripts/run_p0_calibration.sh  |   92 +-
 7 files changed, 5282 insertions(+), 16 deletions(-)
d0d474e B7-extra round-3 audit + fixes (Codex CHALLENGE / Gemini CHALLENGE)
genesis_payload.toml
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md
handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
handover/audits/run_codex_b7_extra_round3_audit.sh
handover/audits/run_gemini_b7_extra_round3_audit.py
handover/preregistration/scripts/run_p0_calibration.sh

exec
/bin/bash -lc 'git diff --stat 1df1f62..d0d474e && git diff --name-only 1df1f62..d0d474e' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 genesis_payload.toml                               |    2 +-
 handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md        |   15 +
 .../audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md    | 4502 +++++++++++++++++++
 .../CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md      | 4708 ++++++++++++++++++++
 .../audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md   |  119 +
 .../GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md     |   69 +
 handover/audits/run_codex_b7_extra_reaudit.sh      |  175 +
 handover/audits/run_codex_b7_extra_round3_audit.sh |  191 +
 handover/audits/run_gemini_b7_extra_reaudit.py     |  209 +
 .../audits/run_gemini_b7_extra_round3_audit.py     |  221 +
 .../preregistration/scripts/run_p0_calibration.sh  |   92 +-
 11 files changed, 10287 insertions(+), 16 deletions(-)
genesis_payload.toml
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md
handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md
handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md
handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
handover/audits/run_codex_b7_extra_reaudit.sh
handover/audits/run_codex_b7_extra_round3_audit.sh
handover/audits/run_gemini_b7_extra_reaudit.py
handover/audits/run_gemini_b7_extra_round3_audit.py
handover/preregistration/scripts/run_p0_calibration.sh

exec
/bin/bash -lc "rg -n \"compute_p0|p0|calibration_problem|synthetic_timeout|problem_file_missing|PREFLIGHT|TRUST_ROOT_TAMPERED|MALFORMED|EXIT=0\" -S . --glob '"'!target/**'"' --glob '"'!experiments/minif2f_v4/logs/**'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
./experiments/minif2f_v4/src/bin/evaluator.rs:151:    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./experiments/minif2f_v4/src/bin/evaluator.rs:183:        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./experiments/minif2f_v4/src/rollback_sim.rs:14:// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
./experiments/minif2f_v4/src/rollback_sim.rs:25:// non-equivalence. `compute_p0.py` honors this by reading only the
./experiments/minif2f_v4/run_subset.sh:45:if [ -z "$PFL" ]; then echo "PREFLIGHT FAIL: no Mathlib"; exit 2; fi
./experiments/minif2f_v4/run_subset.sh:47:if [ $? -ne 0 ] || echo "$OUT" | grep -q "error:"; then echo "PREFLIGHT FAIL: $OUT" | head -c 400; exit 2; fi
./experiments/minif2f_v4/tests/trust_root_immutability.rs:5:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./experiments/minif2f_v4/tests/trust_root_immutability.rs:25://   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
./experiments/minif2f_v4/tests/trust_root_immutability.rs:26://   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
./experiments/minif2f_v4/tests/trust_root_immutability.rs:109:        "handover/preregistration/scripts/run_p0_calibration.sh",
./experiments/minif2f_v4/tests/trust_root_immutability.rs:110:        "handover/preregistration/scripts/compute_p0.py",
./experiments/minif2f_v4/run_batch.sh:86:PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./experiments/minif2f_v4/run_batch.sh:89:if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./experiments/minif2f_v4/run_batch.sh:90:    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages. Run 'lake update && lake exe cache get'."; exit 2
./experiments/minif2f_v4/run_batch.sh:92:PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:ℝ) + 1 = 2 := by norm_num\n' \
./experiments/minif2f_v4/run_batch.sh:93:    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1)
./experiments/minif2f_v4/run_batch.sh:94:PREFLIGHT_CODE=$?
./experiments/minif2f_v4/run_batch.sh:95:if [ "$PREFLIGHT_CODE" -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./experiments/minif2f_v4/run_batch.sh:96:    echo "PREFLIGHT FAIL (exit=$PREFLIGHT_CODE): $PREFLIGHT_OUT" | head -c 500
./experiments/minif2f_v4/run_list.sh:38:if [ -z "$PFL" ]; then echo "PREFLIGHT FAIL: no Mathlib"; exit 2; fi
./experiments/minif2f_v4/run_list.sh:40:if [ $? -ne 0 ] || echo "$OUT" | grep -q "error:"; then echo "PREFLIGHT FAIL: $OUT" | head -c 400; exit 2; fi
./experiments/minif2f_v4/analysis/run_interleaved.sh:50:[ -z "$PFL" ] && { echo "PREFLIGHT FAIL: no Mathlib"; exit 2; }
./experiments/minif2f_v4/analysis/run_interleaved.sh:52:if [ $? -ne 0 ] || echo "$OUT" | grep -q "error:"; then echo "PREFLIGHT FAIL: $OUT" | head -c 400; exit 2; fi
./src/main.rs:11:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./src/main.rs:15:        panic!("TRUST_ROOT_TAMPERED: {e}");
./experiments/minif2f_v4/run_broad.sh:62:if [ -z "$PFL" ]; then echo "PREFLIGHT FAIL: no Mathlib"; exit 2; fi
./experiments/minif2f_v4/run_broad.sh:64:if [ $? -ne 0 ] || echo "$OUT" | grep -q "error:"; then echo "PREFLIGHT FAIL: $OUT" | head -c 400; exit 2; fi
./src/boot.rs:15:// `TRUST_ROOT_TAMPERED`.
./src/boot.rs:32:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./src/boot.rs:50:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./src/boot.rs:53:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./genesis_payload.toml:12:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./genesis_payload.toml:45:#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
./genesis_payload.toml:47:#     handover/preregistration/scripts/compute_p0.py (same — estimator is
./genesis_payload.toml:54:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./genesis_payload.toml:72:"handover/preregistration/scripts/run_p0_calibration.sh" = "323c7f9d23209447b253048c7017f394b6b6ac24f162057863da331e54a148bb"
./genesis_payload.toml:73:"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:20:    -   The manifest has expanded from 16 to 20 entries. The additions are `src/main.rs`, `Cargo.lock`, `run_p0_calibration.sh`, and `compute_p0.py`.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:21:    -   These additions directly address the VETO findings (call site, supply chain) and a key CHALLENGE (runner/estimator as qualified tools). Hashing the runner script (`run_p0_calibration.sh`) is a significant security improvement, as it freezes the entire execution protocol, including timeout values, environment variable setup, and the logic for handling crashes.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:44:-   **(RQ2.a) Confirm the synthetic row's schema is correct and `compute_p0.py` reads it as UNSOLVED.**
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:45:    -   The `emit_synthetic_unsolved` function in `run_p0_calibration.sh` correctly sets `"solved": False`, `"verified": False`, and `"progress": 0`.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:46:    -   The `solved()` function in `compute_p0.py` (line 45) reads the `progress` field. Since the synthetic row has `progress: 0`, `solved()` will correctly return `False`.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:47:    -   The fix is correct and effective. The schema includes the `synthetic_timeout_or_crash: true` flag, which is crucial for downstream analysis to distinguish these cases from natural UNSOLVED runs.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:50:    -   The runner script (`run_p0_calibration.sh`) now uses `set -euo pipefail`. This is critical.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:61:    -   Yes. `compute_p0.py` exits with 1 on data integrity errors (e.g., missing pairs, seed mismatch) via `sys.exit("ERROR...")`. It exits with 2 specifically for a ceiling violation.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:62:    -   `run_p0_calibration.sh` (lines 318-333) correctly distinguishes these:
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:69:    -   The `compute` function in `compute_p0.py` takes `expected_n_problems` and `expected_seeds` as arguments with defaults pointing to the PREREG constants.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:71:    -   Since `compute_p0.py` is now in the Trust Root manifest, its hash is fixed. Any execution of the calibration batch is guaranteed to run this specific version of the script, where the PREREG constants are effectively hard-coded into the `main` entry point. This prevents bypass.
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:81:-   **(RQ4.b) Does `compute_p0.py` honor this narrow equivalence?**
./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:82:    -   Yes. As verified in RQ2.a, `compute_p0.py`'s `solved()` function relies only on the `progress` field. It does not inspect cost, time, or any other non-equivalent field. The code honors the contract described in the `rollback_sim.rs` header.
./handover/audits/run_gemini_b7_extra_round4_audit.py:23:- **Round 3** YOU CHALLENGE on EXIT=0 + empty PPUT_RESULT non-exhaustive case. Codex CHALLENGE on `problem_file_missing` synthetic emit + boot preflight `|| true` exit-discard. **All addressed in commit `d0d474e`**.
./handover/audits/run_gemini_b7_extra_round4_audit.py:28:2. Boot preflight exit-code assertion (Codex P0 #2): captures PREFLIGHT_EXIT explicitly; asserts exit != 0 AND exit != 124; then content grep for TRUST_ROOT_TAMPERED. Three failure modes get distinct diagnostic messages.
./handover/audits/run_gemini_b7_extra_round4_audit.py:29:3. EXIT=0 + empty PPUT_RESULT explicit branch (Gemini round-3 CHALLENGE): malformed-run case now exits 5 with dedicated diagnostic.
./handover/audits/run_gemini_b7_extra_round4_audit.py:50:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_gemini_b7_extra_round4_audit.py:51:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/run_gemini_b7_extra_round4_audit.py:62:| Q3.d: silent skip in compute_p0 | CHALLENGE | 15b87fb: strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144 |
./handover/audits/run_gemini_b7_extra_round4_audit.py:64:| Q6.a: exit-2 not propagated | CHALLENGE | 15b87fb: runner has `set -euo pipefail` + invokes compute_p0 + propagates exit code |
./handover/audits/run_gemini_b7_extra_round4_audit.py:66:| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_gemini_b7_extra_round4_audit.py:80:Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.
./handover/audits/run_gemini_b7_extra_round4_audit.py:83:- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
./handover/audits/run_gemini_b7_extra_round4_audit.py:87:compute_p0.py now fails on missing tags / duplicates / seed mismatch / problem-set mismatch / N≠expected.
./handover/audits/run_gemini_b7_extra_round4_audit.py:90:- (RQ3.a) When ANY strict-completeness check fails, compute_p0.py exits 1 (sys.exit with string). The runner propagates exit codes: 0 → freeze; 2 → ABORT; * → "investigate". Is exit 1 (data integrity failure) handled distinctly from exit 2 (ceiling violation)?
./handover/audits/run_gemini_b7_extra_round4_audit.py:98:- (RQ4.b) The header references `compute_p0.py` honors the narrow equivalence by reading only `progress`. Verify compute_p0.py actually does this.
./handover/audits/run_gemini_b7_extra_round4_audit.py:117:  - handover/preregistration/scripts/compute_p0.py (strict-completeness)
./handover/audits/run_gemini_b7_extra_round4_audit.py:118:  - handover/preregistration/scripts/run_p0_calibration.sh (set -e + timeout + invocation)
./handover/audits/run_gemini_b7_extra_round4_audit.py:151:runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
./handover/audits/run_gemini_b7_extra_round4_audit.py:152:compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
./handover/audits/run_gemini_b7_extra_round4_audit.py:165:control_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_control.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_control.jsonl")) else "(no control smoke yet)"
./handover/audits/run_gemini_b7_extra_round4_audit.py:166:treatment_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl")) else "(no treatment smoke yet)"
./handover/audits/run_gemini_b7_extra_round4_audit.py:174:    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n{runner_sh}\n```\n" +
./handover/audits/run_gemini_b7_extra_round4_audit.py:175:    f"\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n{compute_py}\n```\n" +
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:42:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:79:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:80:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. Should we fail loudly?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:81:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:104:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2.
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:159:runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:160:compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:191:    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n{runner_sh}\n```\n" +
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:192:    f"\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n{compute_py}\n```\n" +
./handover/audits/run_gemini_b7_extra_prebatch_audit.py:227:          f"**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n"
./handover/audits/run_codex_b7_extra_round3_audit.sh:20:- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
./handover/audits/run_codex_b7_extra_round3_audit.sh:26:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/run_codex_b7_extra_round3_audit.sh:27:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/run_codex_b7_extra_round3_audit.sh:33:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/run_codex_b7_extra_round3_audit.sh:51:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_codex_b7_extra_round3_audit.sh:52:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/run_codex_b7_extra_round3_audit.sh:59:| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
./handover/audits/run_codex_b7_extra_round3_audit.sh:60:| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
./handover/audits/run_codex_b7_extra_round3_audit.sh:65:| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
./handover/audits/run_codex_b7_extra_round3_audit.sh:66:| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
./handover/audits/run_codex_b7_extra_round3_audit.sh:69:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_codex_b7_extra_round3_audit.sh:86:`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.
./handover/audits/run_codex_b7_extra_round3_audit.sh:89:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/run_codex_b7_extra_round3_audit.sh:90:- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?
./handover/audits/run_codex_b7_extra_round3_audit.sh:93:runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`
./handover/audits/run_codex_b7_extra_round3_audit.sh:97:- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?
./handover/audits/run_codex_b7_extra_round3_audit.sh:100:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/run_codex_b7_extra_round3_audit.sh:104:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/run_codex_b7_extra_round3_audit.sh:147:printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round3_audit.sh:148:cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round3_audit.sh:150:printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round3_audit.sh:151:cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round3_audit.sh:166:ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_control.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round3_audit.sh:169:ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_treatment.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:31:- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:37:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:38:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:44:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:62:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:63:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:70:| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:71:| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:76:| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:77:| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:80:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:97:`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:100:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:101:- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:104:runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:108:- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:111:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:115:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:160:// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:171:// non-equivalence. `compute_p0.py` honors this by reading only the
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:275:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:292:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:307:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:308:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:309:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:310:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:313:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:549:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:553:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:574:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:607:#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:609:#     handover/preregistration/scripts/compute_p0.py (same — estimator is
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:616:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:634:"handover/preregistration/scripts/run_p0_calibration.sh" = "92701c2876a69968a4f570a67d39c56e15da0a45d44720d4fe1b6174ecdbd821"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:635:"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:655:## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:675:#     of compute_p0.py requires every (problem, seed) pair present
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:676:#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:683:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:735:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:737:    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:739:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:775:PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:778:if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:779:    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:782:PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:783:    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:784:if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:785:    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:786:    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:793:# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:798:PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:799:if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:800:    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:801:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:807:if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:808:   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:809:    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly:"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:810:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:831:# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:832:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:855:    "calibration_problem_id": "$4",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:856:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:905:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:913:            EXIT=0
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:931:row['calibration_problem_id'] = os.environ['PID_ENV']
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:945:                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:952:                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:958:                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:960:                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:963:                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:995:# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:999:    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1001:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1002:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1008:P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1010:python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1030:    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1037:## handover/preregistration/scripts/compute_p0.py (audit-fixed)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1052:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1108:            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1112:                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1192:    p0 = sum(per_problem_regression.values()) / n_problems
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1201:        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1202:        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1203:        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1233:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1266:+    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1296:+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1382:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1434:- 4 from 2026-04-25 dual-audit fixes (`src/main.rs`, `Cargo.lock`, `handover/preregistration/scripts/run_p0_calibration.sh`, `handover/preregistration/scripts/compute_p0.py`)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1436:Will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step). Final size after Phase B → C exit: 21 files.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1453:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1456:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1481:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1688:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1708://   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1709://   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1792:        "handover/preregistration/scripts/run_p0_calibration.sh",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1793:        "handover/preregistration/scripts/compute_p0.py",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1887:{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777138897126", "problem_id": "aime_1983_p2", "solved": true, "split": "adaptation", "verified": true, "golden_path_token_count": 1420, "total_run_token_count": 12240, "total_wall_time_ms": 149170, "progress": 1, "pput_runtime": 5.476928766188159e-07, "pput_verified": 5.476928766188159e-07, "pput_m_verified": 0.5476928766188158, "failed_branch_count": 14, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": true, "time_secs": 149.172273801, "pput": 0.6703658625824985, "gp_token_count": 1420, "gp_node_count": 6, "tx_count": 15, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"omega_wtool": 1, "step": 15, "step_reject": 9, "step_partial_ok": 5}, "gp_payload": "rcases h\u2080 with \u27e8hp_pos, hp_lt_15\u27e9\nrcases h\u2081 with \u27e8hx_ge_p, hx_le_15\u27e9\nhave hx_nonneg : 0 \u2264 x := by linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\ncalc\n  f x = abs (x - p) + abs (x - 15) + abs (x - p - 15) := by\n    rw [h\u2082]\n  _ = abs (x - p) + abs (x - 15) + abs ((x - p) - 15) := by ring\n  _ \u2265 abs (x - p) + abs (x - 15) + abs (abs (x - p) - 15) := by\n    have h : abs ((x - p) - 15) \u2265 abs (abs (x - p) - 15) := by\n      exact abs_sub_abs_le_abs_sub _ _\n    linarith\n  _ \u2265 abs (x - p) + abs (x - 15) + (abs (x - p) - 15) := by\n    have h' : abs (abs (x - p) - 15) \u2265 abs (x - p) - 15 := by\n      nlinarith [abs_nonneg (x - p)]\n    linarith\n  _ = 2 * abs (x - p) + abs (x - 15) - 15 := by ring\n  _ \u2265 2 * abs (x - p) + (15 - x) - 15 := by\n    have hx15 : x \u2264 15 := hx_le_15\n    have : abs (x - 15) = 15 - x := by\n      rw [abs_of_nonpos (sub_nonpos.mpr hx15)]\n      ring\n    rw [this]\n    nlinarith\n  _ = 2 * abs (x - p) - x := by ring\n  _ \u2265 2 * (x - p) - x := by\n    have hxp : x - p \u2265 0 := sub_nonneg.mpr hx_ge_p\n    have : abs (x - p) = x - p := abs_of_nonneg hxp\n    rw [this]\n    nlinarith\n  _ = x - 2 * p := by ring\n  _ \u2265 15 - 2 * p := by nlinarith\n  _ \u2265 15 := by nlinarith", "gp_path": "per_tactic", "gp_proof_file": "proofs/aime_1983_p2_1777138897_a9617ab3.lean", "calibration_mode": "control", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1894:{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777139401511", "problem_id": "aime_1983_p2", "solved": false, "split": "adaptation", "verified": false, "golden_path_token_count": 0, "total_run_token_count": 40023, "total_wall_time_ms": 504246, "progress": 0, "pput_runtime": 0.0, "pput_verified": 0.0, "pput_m_verified": 0.0, "failed_branch_count": 50, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": false, "time_secs": 504.246494102, "pput": 0.0, "gp_token_count": 0, "gp_node_count": 0, "tx_count": 50, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"step_partial_ok": 3, "parse_fail": 3, "step_reject": 44, "step": 47}, "synthetic_short_circuit": true, "calibration_mode": "treatment", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1905:/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'run_p0_calibration.sh' -g 'compute_p0.py' -g 'evaluator.rs' -g 'boot.rs' -g 'trust_root_immutability.rs'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1912:handover/preregistration/scripts/run_p0_calibration.sh
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1913:handover/preregistration/scripts/compute_p0.py
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1943:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,330p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1962:    18	#     of compute_p0.py requires every (problem, seed) pair present
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1963:    19	#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1970:    26	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2022:    78	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2024:    80	    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2026:    82	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2062:   118	PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2065:   121	if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2066:   122	    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2069:   125	PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2070:   126	    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2071:   127	if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2072:   128	    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2073:   129	    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2080:   136	# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2085:   141	PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2086:   142	if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2087:   143	    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2088:   144	    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2094:   150	if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2095:   151	   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2096:   152	    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly:"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2097:   153	    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2118:   174	# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2119:   175	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2142:   198	    "calibration_problem_id": "$4",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2143:   199	    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2192:   248	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2200:   256	            EXIT=0
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2218:   274	row['calibration_problem_id'] = os.environ['PID_ENV']
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2232:   288	                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2239:   295	                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2245:   301	                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2247:   303	                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2250:   306	                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2277:/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2291:    13	    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2347:    69	            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2351:    73	                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2431:   153	    p0 = sum(per_problem_regression.values()) / n_problems
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2440:   162	        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2441:   163	        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2442:   164	        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2472:   194	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2486:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2518:   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2655:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2672:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2687:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2688:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2689:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2690:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2693:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2869:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2889:    25	//   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2890:    26	//   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2973:   109	        "handover/preregistration/scripts/run_p0_calibration.sh",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2974:   110	        "handover/preregistration/scripts/compute_p0.py",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3066:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '330,430p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3076:   338	# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3080:   342	    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3082:   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3083:   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3089:   351	P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3091:   353	python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3111:   373	    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3353:    14	// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3364:    25	// non-equivalence. `compute_p0.py` honors this by reading only the
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3463:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3496:    45	#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3498:    47	#     handover/preregistration/scripts/compute_p0.py (same — estimator is
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3505:    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3523:    72	"handover/preregistration/scripts/run_p0_calibration.sh" = "92701c2876a69968a4f570a67d39c56e15da0a45d44720d4fe1b6174ecdbd821"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3524:    73	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3543:/bin/bash -lc 'rg -n "verify_trust_root|TRUST_ROOT_TAMPERED|evaluator boot|preflight|timeout 30|synthetic_timeout_or_crash|problem_file_missing|exit 124" -S .' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3545:./src/main.rs:11:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3547:./src/main.rs:15:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3548:./experiments/minif2f_v4/tests/trust_root_immutability.rs:5:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3558:./experiments/minif2f_v4/src/bin/evaluator.rs:183:        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3560:./src/boot.rs:15:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3561:./src/boot.rs:32:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3562:./src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3563:./src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3564:./src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3565:./src/boot.rs:50:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3566:./src/boot.rs:53:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3576:./genesis_payload.toml:12:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3578:./genesis_payload.toml:54:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3581:./handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md:47:    -   The fix is correct and effective. The schema includes the `synthetic_timeout_or_crash: true` flag, which is crucial for downstream analysis to distinguish these cases from natural UNSOLVED runs.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3583:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:31:- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3586:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:37:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3587:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:38:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3588:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:44:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3589:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:62:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3591:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:80:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3596:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:100:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3597:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:111:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3598:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:115:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3599:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:275:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3600:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:292:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3601:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:307:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3602:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:308:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3603:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:309:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3604:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:310:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3605:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:313:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3614:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:549:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3616:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:553:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3618:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:574:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3620:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:616:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3625:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:793:# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3628:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:798:PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3629:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:799:if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3630:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:800:    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3632:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:832:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3634:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:856:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3635:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:905:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3636:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:945:                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3637:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:952:                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3638:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:958:                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3639:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:960:                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3640:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:963:                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3641:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1001:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3642:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1002:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3646:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1296:+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3647:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1382:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3650:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1453:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3651:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1456:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3652:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1481:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3653:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1688:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3662:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2080:   136	# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3665:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2085:   141	PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3666:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2086:   142	if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3667:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2087:   143	    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3669:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2119:   175	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3671:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2143:   199	    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3672:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2192:   248	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3673:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2232:   288	                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3674:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2239:   295	                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3675:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2245:   301	                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3676:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2247:   303	                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3677:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2250:   306	                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3680:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2518:   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3681:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2655:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3682:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2672:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3683:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2687:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3684:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2688:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3685:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2689:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3686:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2690:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3687:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2693:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3690:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:2869:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3696:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3082:   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3697:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3083:   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3699:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3463:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3701:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3505:    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3702:./handover/audits/run_codex_b7_extra_round3_audit.sh:20:- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3705:./handover/audits/run_codex_b7_extra_round3_audit.sh:26:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3706:./handover/audits/run_codex_b7_extra_round3_audit.sh:27:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3707:./handover/audits/run_codex_b7_extra_round3_audit.sh:33:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3708:./handover/audits/run_codex_b7_extra_round3_audit.sh:51:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3710:./handover/audits/run_codex_b7_extra_round3_audit.sh:69:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3715:./handover/audits/run_codex_b7_extra_round3_audit.sh:89:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3716:./handover/audits/run_codex_b7_extra_round3_audit.sh:100:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3717:./handover/audits/run_codex_b7_extra_round3_audit.sh:104:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3722:./handover/audits/run_gemini_b7_extra_round3_audit.py:22:- **Round 2** (GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned PASS — but Codex independently returned VETO catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data. Per CLAUDE.md "Audit Standard" + memory feedback_dual_audit_conflict (conservative wins), Codex's VETO triumphed; round-2 fix landed in commit `1df1f62`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3724:./handover/audits/run_gemini_b7_extra_round3_audit.py:28:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3725:./handover/audits/run_gemini_b7_extra_round3_audit.py:29:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3726:./handover/audits/run_gemini_b7_extra_round3_audit.py:35:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3727:./handover/audits/run_gemini_b7_extra_round3_audit.py:53:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3728:./handover/audits/run_gemini_b7_extra_round3_audit.py:69:| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3730:./handover/audits/run_gemini_b7_extra_round3_audit.py:83:Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3731:./handover/audits/run_gemini_b7_extra_round3_audit.py:86:- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3735:./handover/audits/run_codex_b7_extra_reaudit.sh:35:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3737:./handover/audits/run_codex_b7_extra_reaudit.sh:53:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3742:./handover/audits/run_codex_b7_extra_reaudit.sh:73:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3743:./handover/audits/run_codex_b7_extra_reaudit.sh:84:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3744:./handover/audits/run_codex_b7_extra_reaudit.sh:88:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3747:./handover/audits/run_gemini_b7_extra_reaudit.py:41:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3748:./handover/audits/run_gemini_b7_extra_reaudit.py:57:| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3750:./handover/audits/run_gemini_b7_extra_reaudit.py:71:Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3751:./handover/audits/run_gemini_b7_extra_reaudit.py:74:- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3757:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:216:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3758:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3761:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3764:./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:17:The Round-2 fix (`1df1f62`) is a substantial improvement and correctly resolves the critical VETO-level defect identified by Codex in the previous round. The new crash-discrimination logic, which aborts the batch on any non-timeout, non-zero exit, prevents the silent absorption of panics (like `TRUST_ROOT_TAMPERED`) into the calibration dataset. This restores the integrity of the Trust Root mechanism. The addition of a pre-flight boot check is a commendable "fail fast" enhancement.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3765:./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:32:    3.  `EXIT` is non-zero and not 124 → `CRASH` path, which aborts the entire batch with a specific exit code (3 for `TRUST_ROOT_TAMPERED`, 4 for others).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3766:./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:33:- **Conclusion**: This logic prevents panics from being silently converted into "valid" `UNSOLVED` data points, which was the critical flaw. The `TRUST_ROOT_TAMPERED` pre-flight check (lines 163-173) further hardens this by catching the most severe integrity failures before any API spend.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3769:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:46:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3771:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:64:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3776:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:84:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3777:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:95:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3778:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:99:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3779:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:259:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3780:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:276:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3781:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:291:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3782:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:292:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3783:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:293:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3784:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:294:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3785:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:297:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3794:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:533:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3796:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:537:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3798:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:558:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3800:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:600:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3804:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:792:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3805:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:810:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3806:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:858:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3807:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:933:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3808:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:934:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3812:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1228:+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3813:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1314:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3816:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1380:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3817:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1383:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3818:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1408:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3819:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1615:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3826:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1959:   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3830:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2439:   151	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3831:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2457:   169	    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3832:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2505:   217	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3833:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2580:   292	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3834:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2581:   293	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3835:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2607:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3836:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2624:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3837:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2639:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3838:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2640:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3839:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2641:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3840:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2642:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3841:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2645:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3844:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3443:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3850:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3651:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3852:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3693:    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3853:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3751:    19	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3856:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3817:    85	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3857:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3820:    88	- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3858:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3845:   113	1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3859:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4022:    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3861:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4026:    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3862:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4372:/bin/bash -lc 'rg -n "TRUST_ROOT_TAMPERED|synthetic_timeout_or_crash|synthetic_reason|evaluator_crash|TIMEOUT|CRASH|compute_p0|progress" handover/preregistration/scripts/compute_p0.py handover/preregistration/scripts/run_p0_calibration.sh experiments/minif2f_v4/src/bin/evaluator.rs' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3863:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4379:experiments/minif2f_v4/src/bin/evaluator.rs:183:        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3864:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4394:handover/preregistration/scripts/run_p0_calibration.sh:151:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3865:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4396:handover/preregistration/scripts/run_p0_calibration.sh:169:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3866:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4404:handover/preregistration/scripts/run_p0_calibration.sh:292:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3867:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4405:handover/preregistration/scripts/run_p0_calibration.sh:293:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3868:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4428:                    'synthetic_timeout_or_crash': True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3871:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:215:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3872:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3873:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:320:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3874:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:524:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3875:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1078:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3876:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3877:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1166:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3878:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1333:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3879:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2287:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3880:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3882:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:295:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3889:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3774:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:216:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3890:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3777:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3891:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3809:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:295:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3892:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3858:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:215:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3893:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3861:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3894:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3863:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:320:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3895:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3865:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:524:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3896:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3879:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1078:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3897:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3882:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3898:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3884:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1166:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3899:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3886:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1333:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3900:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3903:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2287:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3901:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3906:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3902:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3946:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:226:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3903:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3949:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3904:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3951:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:314:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3905:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3953:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:481:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3906:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3970:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1435:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3907:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3973:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3908:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4005:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3909:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4008:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3910:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4010:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3911:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4034:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3912:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4037:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3913:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4069:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3914:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4072:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3915:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4074:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3916:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4098:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3917:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4101:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3918:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4353:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3919:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4360:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:302:- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3920:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4374:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:152:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3921:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4377:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3922:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4379:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:257:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3923:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4381:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:461:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3924:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5504:   270	- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3925:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5536:   302	- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3926:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5680:   152	2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3927:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5696:   168	This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3928:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:226:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3929:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3930:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:314:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3931:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:481:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3932:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1435:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3933:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3935:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3936:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3937:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3938:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3939:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3941:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3942:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3943:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3944:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3945:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3950:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:286:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3951:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:303:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3952:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:318:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3953:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:319:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3954:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:320:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3955:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:321:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3956:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:324:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3965:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:572:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3967:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:576:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3969:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:597:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3970:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:630:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3974:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1131:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3977:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1197:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3978:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1200:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3979:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1225:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3981:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1472:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3982:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1541:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3983:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2127:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3984:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2144:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3985:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2159:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3986:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2160:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3987:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2161:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3988:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2162:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3989:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2165:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3992:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2307:    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3994:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2311:    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3999:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2846:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4000:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2879:    45	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4001:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3084:   461	               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4002:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3119:    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4005:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3185:    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4006:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3188:    86	- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4007:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3213:   111	1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4010:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3785:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4012:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4193:src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4013:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4194:src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4014:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4195:src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4027:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4257:handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4029:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4268:handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4031:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4279:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4032:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4291:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4033:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4303:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4036:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4323:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:318:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4037:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4324:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:319:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4038:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4325:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:320:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4049:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4367:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1131:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4052:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4374:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1197:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4054:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4386:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2159:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4055:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4387:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2160:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4056:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4388:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2161:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4061:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4414:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3119:    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4064:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4421:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3185:    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4066:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4430:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4067:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4442:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4068:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4453:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4069:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4464:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4070:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4478:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3777:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4071:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4485:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3861:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4072:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4493:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3882:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4073:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4501:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3906:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4074:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4510:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3949:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4075:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4518:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3973:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4076:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4525:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4008:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4077:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4532:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4037:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4078:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4539:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4072:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4079:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4546:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4101:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4080:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4558:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4353:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4081:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4567:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4377:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4082:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4577:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5504:   270	- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4083:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4592:handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5696:   168	This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4084:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4600:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4085:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4611:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4088:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4629:handover/ai-direct/LATEST.md:42:  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4089:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4637:handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:12:Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4092:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4642:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:17:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4095:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4649:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:83:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4096:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4654:handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4097:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4668:handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4098:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4691:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4104:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5369:   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4105:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5405:   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4114:./handover/ai-direct/LATEST.md:42:  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4115:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:152:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4116:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4117:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:257:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4118:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:461:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4119:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4120:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:302:- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4121:./handover/preregistration/scripts/run_p0_calibration.sh:115:# C-012 oracle preflight (memory feedback_oracle_preflight.md).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4122:./handover/preregistration/scripts/run_p0_calibration.sh:116:echo "[$(date -Is)] Oracle preflight..."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4123:./handover/preregistration/scripts/run_p0_calibration.sh:132:echo "Oracle preflight OK."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4124:./handover/preregistration/scripts/run_p0_calibration.sh:134:# Audit-fix 2026-04-25 round-2 (Codex re-audit VETO): evaluator boot preflight.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4125:./handover/preregistration/scripts/run_p0_calibration.sh:136:# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4126:./handover/preregistration/scripts/run_p0_calibration.sh:138:# preflight surfaces the failure with the clean diagnostic and zero wasted
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4127:./handover/preregistration/scripts/run_p0_calibration.sh:140:echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4128:./handover/preregistration/scripts/run_p0_calibration.sh:141:PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4129:./handover/preregistration/scripts/run_p0_calibration.sh:142:if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4130:./handover/preregistration/scripts/run_p0_calibration.sh:143:    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4131:./handover/preregistration/scripts/run_p0_calibration.sh:156:echo "Evaluator boot preflight OK."
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4132:./handover/preregistration/scripts/run_p0_calibration.sh:175:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4133:./handover/preregistration/scripts/run_p0_calibration.sh:184:    # USED ONLY for legitimate timeout (exit 124). Crash paths now ABORT
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4134:./handover/preregistration/scripts/run_p0_calibration.sh:199:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4135:./handover/preregistration/scripts/run_p0_calibration.sh:248:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4136:./handover/preregistration/scripts/run_p0_calibration.sh:288:                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4137:./handover/preregistration/scripts/run_p0_calibration.sh:295:                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4138:./handover/preregistration/scripts/run_p0_calibration.sh:301:                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4139:./handover/preregistration/scripts/run_p0_calibration.sh:303:                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4140:./handover/preregistration/scripts/run_p0_calibration.sh:306:                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4141:./handover/preregistration/scripts/run_p0_calibration.sh:344:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4142:./handover/preregistration/scripts/run_p0_calibration.sh:345:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4143:./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:1:# OBS — Boot failure (TRUST_ROOT_TAMPERED) is not a constitutional HALT
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4144:./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:12:Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4145:./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:16:## Why TRUST_ROOT_TAMPERED ≠ FC2-N22 HALT
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4146:./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:18:| Property | FC2-N22 HALT | TRUST_ROOT_TAMPERED panic |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4147:./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:26:FC2 has a `HALT` node (FC2-N22) that lives *inside* the boot/tick lifecycle. TRUST_ROOT_TAMPERED fires *before* the boot lifecycle — it is a precondition violation on the readonly base.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4150:./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:19:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4153:./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:90:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4154:./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:93:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4155:./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:118:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4324:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4473:/bin/bash -lc 'rg -n "test.*evaluator|Command::new|target/release/evaluator|CARGO_BIN_EXE_evaluator|TRUST_ROOT_TAMPERED" experiments/minif2f_v4/tests src tests -S' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4475:src/main.rs:11:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4476:src/main.rs:15:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4477:src/boot.rs:15:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4478:src/boot.rs:32:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4479:src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4480:src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4481:src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4482:src/boot.rs:50:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4483:src/boot.rs:53:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4486:experiments/minif2f_v4/tests/trust_root_immutability.rs:5:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4503:                base={'schema_version':'v2.0','calibration_problem_id':p,'calibration_seed':s,'progress':0,'synthetic_timeout_or_crash':True,'synthetic_reason':'problem_file_missing'}
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4506:    r=subprocess.run(['python3','handover/preregistration/scripts/compute_p0.py','--control',str(c),'--treatment',str(t),'--out-json',str(out)], cwd='/home/zephryj/projects/turingosv4', text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4522:                cf.write(json.dumps({'calibration_problem_id':p,'calibration_seed':s,'progress':1})+'\\n')
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4523:                tf.write(json.dumps({'calibration_problem_id':p,'calibration_seed':s,'progress':0})+'\\n')
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4524:    r=subprocess.run(['python3','handover/preregistration/scripts/compute_p0.py','--control',str(c),'--treatment',str(t)], cwd='/home/zephryj/projects/turingosv4', text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4530:['{', '  "n_problems": 144,', '  "n_pairs": 288,', '  "n_control_solved": 0,', '  "n_treatment_solved": 0,', '  "n_regression_pairs": 0,', '  "n_regression_problems_max_seed": 0,', '  "p0": 0.0,', '  "p0_ceiling": 0.1,', '  "ceiling_pass": true', '}', '']
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4664:   The runner claims synthetic rows are “USED ONLY” for timeout at [run_p0_calibration.sh:184](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:184), but the loop still emits a synthetic UNSOLVED row for `PROBLEM_NOT_FOUND` at [run_p0_calibration.sh:246](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:246). `compute_p0.py` only requires tags, seed set, matched problem sets, and row counts at [compute_p0.py:69](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:69), [compute_p0.py:93](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:93), [compute_p0.py:110](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:110), and [compute_p0.py:116](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:116). It does not reject `synthetic_reason=problem_file_missing`. I verified a 144-problem × 2-seed all-`problem_file_missing` synthetic dataset exits `0` with `p0=0.0`. That is the same silent data-loss class as the prior crash-as-data VETO.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4667:   The preflight runs `timeout 30 "$EVALUATOR" ... || true` at [run_p0_calibration.sh:141](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:141), then only greps output for `TRUST_ROOT_TAMPERED` or unexpected panic text at [run_p0_calibration.sh:142](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:142) and [run_p0_calibration.sh:150](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:150). A preflight timeout `124` or empty-output hang would print “OK” at [run_p0_calibration.sh:156](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:156). Since in-loop `124` becomes synthetic UNSOLVED at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285), this can still turn an evaluator-level infrastructure failure into apparently complete data.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4671:The specific round-2 VETO, `TRUST_ROOT_TAMPERED` panic absorbed as valid synthetic data, is resolved for in-loop evaluator crashes: only exit `124` emits synthetic data at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285); other exits abort at [run_p0_calibration.sh:300](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:300), with `TRUST_ROOT_TAMPERED` mapped to exit `3` at [run_p0_calibration.sh:301](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:301). The evaluator now calls `verify_trust_root` before loading problems or making LLM calls at [evaluator.rs:176](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:176) and panics on failure at [evaluator.rs:181](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:181).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4673:B2/B3 are substantially fixed: strict completeness and denominator pinning are at [compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88) and [compute_p0.py:151](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151); runner propagates estimator exit `2` as abort at [run_p0_calibration.sh:365](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:365).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4679:Targeted verification run: `cargo test -p minif2f_v4 trust_root -- --nocapture` passed. I also verified `compute_p0.py` returns exit `2` for an all-regression dataset.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4683:**Recommendation**: FIX-THEN-PROCEED again. Remove or abort the `problem_file_missing` synthetic path, add a pre-loop adaptation-file existence preflight, and make evaluator boot preflight assert an expected nonzero exit/status instead of grepping output after `|| true`.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4689:   The runner claims synthetic rows are “USED ONLY” for timeout at [run_p0_calibration.sh:184](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:184), but the loop still emits a synthetic UNSOLVED row for `PROBLEM_NOT_FOUND` at [run_p0_calibration.sh:246](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:246). `compute_p0.py` only requires tags, seed set, matched problem sets, and row counts at [compute_p0.py:69](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:69), [compute_p0.py:93](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:93), [compute_p0.py:110](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:110), and [compute_p0.py:116](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:116). It does not reject `synthetic_reason=problem_file_missing`. I verified a 144-problem × 2-seed all-`problem_file_missing` synthetic dataset exits `0` with `p0=0.0`. That is the same silent data-loss class as the prior crash-as-data VETO.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4692:   The preflight runs `timeout 30 "$EVALUATOR" ... || true` at [run_p0_calibration.sh:141](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:141), then only greps output for `TRUST_ROOT_TAMPERED` or unexpected panic text at [run_p0_calibration.sh:142](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:142) and [run_p0_calibration.sh:150](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:150). A preflight timeout `124` or empty-output hang would print “OK” at [run_p0_calibration.sh:156](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:156). Since in-loop `124` becomes synthetic UNSOLVED at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285), this can still turn an evaluator-level infrastructure failure into apparently complete data.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4696:The specific round-2 VETO, `TRUST_ROOT_TAMPERED` panic absorbed as valid synthetic data, is resolved for in-loop evaluator crashes: only exit `124` emits synthetic data at [run_p0_calibration.sh:285](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:285); other exits abort at [run_p0_calibration.sh:300](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:300), with `TRUST_ROOT_TAMPERED` mapped to exit `3` at [run_p0_calibration.sh:301](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:301). The evaluator now calls `verify_trust_root` before loading problems or making LLM calls at [evaluator.rs:176](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:176) and panics on failure at [evaluator.rs:181](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:181).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4698:B2/B3 are substantially fixed: strict completeness and denominator pinning are at [compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88) and [compute_p0.py:151](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151); runner propagates estimator exit `2` as abort at [run_p0_calibration.sh:365](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:365).
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4704:Targeted verification run: `cargo test -p minif2f_v4 trust_root -- --nocapture` passed. I also verified `compute_p0.py` returns exit `2` for an all-regression dataset.
./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4708:**Recommendation**: FIX-THEN-PROCEED again. Remove or abort the `problem_file_missing` synthetic path, add a pre-loop adaptation-file existence preflight, and make evaluator boot preflight assert an expected nonzero exit/status instead of grepping output after `|| true`.
./handover/audits/run_gemini_b7_extra_round3_audit.py:22:- **Round 2** (GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned PASS — but Codex independently returned VETO catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data. Per CLAUDE.md "Audit Standard" + memory feedback_dual_audit_conflict (conservative wins), Codex's VETO triumphed; round-2 fix landed in commit `1df1f62`.
./handover/audits/run_gemini_b7_extra_round3_audit.py:28:   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
./handover/audits/run_gemini_b7_extra_round3_audit.py:29:2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
./handover/audits/run_gemini_b7_extra_round3_audit.py:35:- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
./handover/audits/run_gemini_b7_extra_round3_audit.py:53:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_gemini_b7_extra_round3_audit.py:54:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/run_gemini_b7_extra_round3_audit.py:65:| Q3.d: silent skip in compute_p0 | CHALLENGE | 15b87fb: strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144 |
./handover/audits/run_gemini_b7_extra_round3_audit.py:67:| Q6.a: exit-2 not propagated | CHALLENGE | 15b87fb: runner has `set -euo pipefail` + invokes compute_p0 + propagates exit code |
./handover/audits/run_gemini_b7_extra_round3_audit.py:69:| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_gemini_b7_extra_round3_audit.py:83:Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.
./handover/audits/run_gemini_b7_extra_round3_audit.py:86:- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
./handover/audits/run_gemini_b7_extra_round3_audit.py:90:compute_p0.py now fails on missing tags / duplicates / seed mismatch / problem-set mismatch / N≠expected.
./handover/audits/run_gemini_b7_extra_round3_audit.py:93:- (RQ3.a) When ANY strict-completeness check fails, compute_p0.py exits 1 (sys.exit with string). The runner propagates exit codes: 0 → freeze; 2 → ABORT; * → "investigate". Is exit 1 (data integrity failure) handled distinctly from exit 2 (ceiling violation)?
./handover/audits/run_gemini_b7_extra_round3_audit.py:101:- (RQ4.b) The header references `compute_p0.py` honors the narrow equivalence by reading only `progress`. Verify compute_p0.py actually does this.
./handover/audits/run_gemini_b7_extra_round3_audit.py:120:  - handover/preregistration/scripts/compute_p0.py (strict-completeness)
./handover/audits/run_gemini_b7_extra_round3_audit.py:121:  - handover/preregistration/scripts/run_p0_calibration.sh (set -e + timeout + invocation)
./handover/audits/run_gemini_b7_extra_round3_audit.py:154:runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
./handover/audits/run_gemini_b7_extra_round3_audit.py:155:compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
./handover/audits/run_gemini_b7_extra_round3_audit.py:168:control_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_control.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_control.jsonl")) else "(no control smoke yet)"
./handover/audits/run_gemini_b7_extra_round3_audit.py:169:treatment_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl")) else "(no treatment smoke yet)"
./handover/audits/run_gemini_b7_extra_round3_audit.py:177:    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n{runner_sh}\n```\n" +
./handover/audits/run_gemini_b7_extra_round3_audit.py:178:    f"\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n{compute_py}\n```\n" +
./handover/audits/run_codex_b7_extra_reaudit.sh:35:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_codex_b7_extra_reaudit.sh:36:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/run_codex_b7_extra_reaudit.sh:43:| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
./handover/audits/run_codex_b7_extra_reaudit.sh:44:| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
./handover/audits/run_codex_b7_extra_reaudit.sh:49:| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
./handover/audits/run_codex_b7_extra_reaudit.sh:50:| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
./handover/audits/run_codex_b7_extra_reaudit.sh:53:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_codex_b7_extra_reaudit.sh:70:`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.
./handover/audits/run_codex_b7_extra_reaudit.sh:73:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/run_codex_b7_extra_reaudit.sh:74:- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?
./handover/audits/run_codex_b7_extra_reaudit.sh:77:runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`
./handover/audits/run_codex_b7_extra_reaudit.sh:81:- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?
./handover/audits/run_codex_b7_extra_reaudit.sh:84:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/run_codex_b7_extra_reaudit.sh:88:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/run_codex_b7_extra_reaudit.sh:131:printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_reaudit.sh:132:cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_reaudit.sh:134:printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_reaudit.sh:135:cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_reaudit.sh:150:ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_control.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_reaudit.sh:153:ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_treatment.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:27:-   **(RQ1.b) `genesis_payload.toml` Self-Hash:** The chicken-and-egg problem of the manifest not hashing itself is acknowledged and acceptable. The defense is sound: the call sites (`src/main.rs`, `evaluator.rs`) that invoke the verification logic are themselves in the manifest. An attacker cannot modify `genesis_payload.toml` to weaken the checks without also modifying a hashed file, which would trigger a `TRUST_ROOT_TAMPERED` failure. This is documented in `TRACE_MATRIX_v1_2026-04-25.md` § 3.
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:36:-   **(RQ2.a) Synthetic Row Correctness:** The `emit_synthetic_unsolved` function in `run_p0_calibration.sh` correctly generates a JSONL row with `"solved": false`, `"progress": 0`, and `"synthetic_timeout_or_crash": true`. The `solved()` function in `compute_p0.py` correctly interprets this as UNSOLVED by checking `int(row["progress"]) == 1`. The smoke test evidence confirms the treatment run produced a row with `solved: false` and `synthetic_short_circuit: true`, which is handled by the same logic.
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:38:    -   An evaluator crash (`exit != 0` and `exit != 124`) now aborts the entire batch (`run_p0_calibration.sh`, line 406). This correctly prevents crashes from being silently absorbed as UNSOLVED data, a P0 defect caught by Codex in Round 2.
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:39:    -   A malformed run (`exit == 0` but no `PPUT_RESULT` line) is now explicitly caught and aborts the batch with `exit 5` (`run_p0_calibration.sh`, line 385). This resolves my Round-3 CHALLENGE.
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:47:`compute_p0.py` was updated to fail loudly on incomplete or malformed data sets.
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:49:-   **(RQ3.a) Exit Code Handling:** `compute_p0.py` exits with status 1 on data integrity failures (e.g., missing problems, duplicate rows). The runner script (`run_p0_calibration.sh`, lines 463-465) correctly distinguishes this from a successful run (exit 0) and a ceiling violation (exit 2), propagating the error code and printing a specific diagnostic.
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:50:-   **(RQ3.b) Parameter Bypass:** The `compute()` function in `compute_p0.py` uses hardcoded `PREREG_` constants as default arguments. The `main()` function calls `compute()` without overriding these defaults, and the script's `argparse` does not expose them as command-line flags. This prevents a caller from accidentally or maliciously bypassing the pre-registered constraints.
./handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:59:-   **(RQ4.b) `compute_p0.py` Honoring Equivalence:** The claim that `compute_p0.py` honors this narrow equivalence is verified. Its `solved()` function depends only on the `progress` field, which directly maps to the `solved` status. It does not inspect cost, `tx_count`, or any other non-equivalent field.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:40:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:77:- (Q3.b) `compute_p0.py` per-problem uses `max(over seeds)` — worst-case framing. If only 1 of 2 seeds regressed, the problem still counts as 1. With 2 seeds, this DOUBLES the regression rate vs `mean(over seeds)`. PREREG § 5.5 line 450 explicitly says max — agreed?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:78:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. For a synthetic_short_circuit row, both will be False (correctly UNSOLVED). For a control row that SOLVED, both will be True. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:79:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. If the runner script's row-enrichment failed for any subset (e.g., python3 OOM'd on one row), would calibration silently miss data? Should we fail loudly?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:80:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:97:- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)? `compute_p0.py` joins on calibration_problem_id + calibration_seed and reads only `solved` / `progress_verified` — neither field comes from WAL or recent_rejections. Argument for "no impact": the calibration jsonl IS ground-truth-validated independently. Verify.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:99:- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50 (correctly reflects 50 tx of real activity). Does anything downstream INTERPRET these as "if they were lower than control, the treatment toggle is bug-prone"? compute_p0.py doesn't; flag if any other tooling does.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:103:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2 if ceiling violated.
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:106:- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it and continue? Test by inspection: where in the pipeline does compute_p0.py run, and what handles exit code?
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:166:printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:167:cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:169:printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:170:cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_prebatch_audit.sh:199:  printf '**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n'
./handover/audits/run_gemini_b7_extra_reaudit.py:41:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_gemini_b7_extra_reaudit.py:42:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/run_gemini_b7_extra_reaudit.py:53:| Q3.d: silent skip in compute_p0 | CHALLENGE | 15b87fb: strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144 |
./handover/audits/run_gemini_b7_extra_reaudit.py:55:| Q6.a: exit-2 not propagated | CHALLENGE | 15b87fb: runner has `set -euo pipefail` + invokes compute_p0 + propagates exit code |
./handover/audits/run_gemini_b7_extra_reaudit.py:57:| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_gemini_b7_extra_reaudit.py:71:Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.
./handover/audits/run_gemini_b7_extra_reaudit.py:74:- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
./handover/audits/run_gemini_b7_extra_reaudit.py:78:compute_p0.py now fails on missing tags / duplicates / seed mismatch / problem-set mismatch / N≠expected.
./handover/audits/run_gemini_b7_extra_reaudit.py:81:- (RQ3.a) When ANY strict-completeness check fails, compute_p0.py exits 1 (sys.exit with string). The runner propagates exit codes: 0 → freeze; 2 → ABORT; * → "investigate". Is exit 1 (data integrity failure) handled distinctly from exit 2 (ceiling violation)?
./handover/audits/run_gemini_b7_extra_reaudit.py:89:- (RQ4.b) The header references `compute_p0.py` honors the narrow equivalence by reading only `progress`. Verify compute_p0.py actually does this.
./handover/audits/run_gemini_b7_extra_reaudit.py:108:  - handover/preregistration/scripts/compute_p0.py (strict-completeness)
./handover/audits/run_gemini_b7_extra_reaudit.py:109:  - handover/preregistration/scripts/run_p0_calibration.sh (set -e + timeout + invocation)
./handover/audits/run_gemini_b7_extra_reaudit.py:142:runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
./handover/audits/run_gemini_b7_extra_reaudit.py:143:compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
./handover/audits/run_gemini_b7_extra_reaudit.py:156:control_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_control.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_control.jsonl")) else "(no control smoke yet)"
./handover/audits/run_gemini_b7_extra_reaudit.py:157:treatment_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl")) else "(no treatment smoke yet)"
./handover/audits/run_gemini_b7_extra_reaudit.py:165:    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n{runner_sh}\n```\n" +
./handover/audits/run_gemini_b7_extra_reaudit.py:166:    f"\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n{compute_py}\n```\n" +
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:19:    -   **Conclusion**: The equivalence is a convenience that is valid *only because* `compute_p0.py` ignores cost and intra-run state, joining only on the final `SOLVED`/`UNSOLVED` status. The constitutional anchor is weak; it achieves an outcome analogous to FC2-N22 HALT, but it does not *traverse* the constitutional path of FC1-E18 (repeatedly). The documentation in `rollback_sim.rs` should be amended to state this is a *functionally equivalent outcome for p_0 estimation* rather than a generally "observably equivalent" process.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:41:-   **(Q3.b) `max(over seeds)` framing**: **PASS**. `PREREG § 5.5` line 450 explicitly specifies `max over the 2 seeds`. The implementation in `compute_p0.py:84` correctly reflects this policy (`if regression > per_problem_regression[pid]: ...`). This is a consistent implementation of a pre-registered, "worst-case" policy choice.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:43:-   **(Q3.c) `solved()` predicate**: **PASS**. The `solved()` function in `compute_p0.py:44-48` correctly prioritizes `progress_verified` over the legacy `has_golden_path`, which aligns with the B4 audit findings and the `RunAggregate::V2` schema.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:45:-   **(Q3.d) Silent drop in `compute_p0.py`**: **CHALLENGE**. The script at `compute_p0.py:56-58` silently skips rows that are missing `calibration_problem_id` or `calibration_seed`. A failure in the runner script (`run_p0_calibration.sh:170-177`) to stamp a row would cause that data point to be silently dropped, biasing the `p_0` result without warning. The script should fail loudly if any row from either input file is missing the required keys.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:64:-   **(Q6.a) `exit 2` abort**: **CHALLENGE**. The runner script `run_p0_calibration.sh` does not use `set -e`. If `compute_p0.py` exits with code 2, the script will continue executing, printing the summary box. An automated runner could miss the failure. The script should use `set -e` or explicitly check the exit code of `compute_p0.py` and abort.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:71:-   **(Q7.b) Timeout impact on `p_0`**: **CHALLENGE**. A run that times out is logged as `MEASUREMENT_ERROR` (`run_p0_calibration.sh:195`). This means the (problem, seed) pair will be missing from one of the jsonl files. `compute_p0.py` will then exclude this pair from its analysis (`set(c.keys()) & set(t.keys())`). This introduces sampling bias. A timeout is a valid outcome (UNSOLVED) and should be treated as such, not as a data error. The runner script should be modified to emit a valid UNSOLVED jsonl row for timed-out runs.
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:100:3.  **CHALLENGE Fix**: Modify `compute_p0.py` to fail loudly if any row is missing the required `calibration_*` keys, preventing silent data loss and biased results. (`Q3.d`)
./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:101:4.  **CHALLENGE Fix**: Modify `run_p0_calibration.sh` to treat a timeout as a valid `UNSOLVED` outcome and emit a corresponding JSONL row, instead of a `MEASUREMENT_ERROR` that leads to data exclusion. (`Q7.b`)
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:216:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:17:The Round-2 fix (`1df1f62`) is a substantial improvement and correctly resolves the critical VETO-level defect identified by Codex in the previous round. The new crash-discrimination logic, which aborts the batch on any non-timeout, non-zero exit, prevents the silent absorption of panics (like `TRUST_ROOT_TAMPERED`) into the calibration dataset. This restores the integrity of the Trust Root mechanism. The addition of a pre-flight boot check is a commendable "fail fast" enhancement.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:28:- **Evidence**: `handover/preregistration/scripts/run_p0_calibration.sh` lines 309-333.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:30:    1.  `EXIT=0` with `PPUT_RESULT` → Success path.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:32:    3.  `EXIT` is non-zero and not 124 → `CRASH` path, which aborts the entire batch with a specific exit code (3 for `TRUST_ROOT_TAMPERED`, 4 for others).
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:33:- **Conclusion**: This logic prevents panics from being silently converted into "valid" `UNSOLVED` data points, which was the critical flaw. The `TRUST_ROOT_TAMPERED` pre-flight check (lines 163-173) further hardens this by catching the most severe integrity failures before any API spend.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:47:- **Location**: `handover/preregistration/scripts/run_p0_calibration.sh`, lines 299-333.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:48:- **Vulnerability**: If the `evaluator` process exits successfully (`EXIT=0`) but fails to print a line starting with `PPUT_RESULT:` to stdout, the run is silently dropped.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:56:- **Impact**: This constitutes a silent `MEASUREMENT_ERROR`. If this occurs, `compute_p0.py` will fail its strict-completeness check, aborting the analysis after the entire batch has run and spent the budget. Worse, if the completeness check were ever relaxed, it would bias the `p_0` estimate by selectively dropping runs (likely successful ones, as a crash would be caught).
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:57:- **Recommendation**: The `if/elif/else` structure should be made exhaustive for the `EXIT=0` case. An `else` branch should be added to the main `if` to catch the `[ "$EXIT" -eq 0 ] && [ -z "$PPUT_JSON" ]` condition, print a loud error, and abort the batch.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:64:- **RQ3 (Strict-Completeness)**: PASS. `compute_p0.py` correctly uses hardcoded PREREG constants and does not expose them as arguments, preventing bypass. The runner script correctly distinguishes the data integrity exit code (1) from the ceiling violation exit code (2).
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:65:- **RQ4 (Equivalence Claim)**: PASS. The header in `rollback_sim.rs` is accurate for the system's current feature set. `compute_p0.py` correctly honors the narrow equivalence by only inspecting the `progress` field.
./handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:69:**Final Recommendation**: The team has successfully addressed the critical VETO. The remaining issue is a standard bug, not an architectural flaw. A small patch to `run_p0_calibration.sh` to make the result-parsing logic exhaustive will resolve the CHALLENGE. After that fix is applied and verified, the batch can proceed with high confidence.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:46:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:47:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:54:| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:55:| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:60:| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:61:| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:64:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:81:`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:84:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:85:- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:88:runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:92:- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:95:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:99:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:144:// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:155:// non-equivalence. `compute_p0.py` honors this by reading only the
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:259:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:276:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:291:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:292:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:293:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:294:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:297:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:533:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:537:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:558:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:591:#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:593:#     handover/preregistration/scripts/compute_p0.py (same — estimator is
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:600:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:618:"handover/preregistration/scripts/run_p0_calibration.sh" = "9a1e216301eb5dba72351a49a5a4c799e4bbf0dee79646467d28d972f8196cf8"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:619:"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:639:## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:659:#     of compute_p0.py requires every (problem, seed) pair present
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:660:#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:667:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:719:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:721:    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:723:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:759:PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:762:if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:763:    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:766:PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:767:    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:768:if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:769:    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:770:    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:791:# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:792:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:809:    "calibration_problem_id": "$4",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:810:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:858:                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:866:            EXIT=0
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:884:row['calibration_problem_id'] = os.environ['PID_ENV']
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:927:# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:931:    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:933:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:934:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:940:P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:942:python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:962:    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:969:## handover/preregistration/scripts/compute_p0.py (audit-fixed)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:984:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1040:            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1044:                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1124:    p0 = sum(per_problem_regression.values()) / n_problems
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1133:        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1134:        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1135:        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1165:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1198:+    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1228:+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1314:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1363:**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1380:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1383:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1408:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1615:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1635://   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1636://   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1719:        "handover/preregistration/scripts/run_p0_calibration.sh",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1720:        "handover/preregistration/scripts/compute_p0.py",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1814:{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777138897126", "problem_id": "aime_1983_p2", "solved": true, "split": "adaptation", "verified": true, "golden_path_token_count": 1420, "total_run_token_count": 12240, "total_wall_time_ms": 149170, "progress": 1, "pput_runtime": 5.476928766188159e-07, "pput_verified": 5.476928766188159e-07, "pput_m_verified": 0.5476928766188158, "failed_branch_count": 14, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": true, "time_secs": 149.172273801, "pput": 0.6703658625824985, "gp_token_count": 1420, "gp_node_count": 6, "tx_count": 15, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"omega_wtool": 1, "step": 15, "step_reject": 9, "step_partial_ok": 5}, "gp_payload": "rcases h\u2080 with \u27e8hp_pos, hp_lt_15\u27e9\nrcases h\u2081 with \u27e8hx_ge_p, hx_le_15\u27e9\nhave hx_nonneg : 0 \u2264 x := by linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\ncalc\n  f x = abs (x - p) + abs (x - 15) + abs (x - p - 15) := by\n    rw [h\u2082]\n  _ = abs (x - p) + abs (x - 15) + abs ((x - p) - 15) := by ring\n  _ \u2265 abs (x - p) + abs (x - 15) + abs (abs (x - p) - 15) := by\n    have h : abs ((x - p) - 15) \u2265 abs (abs (x - p) - 15) := by\n      exact abs_sub_abs_le_abs_sub _ _\n    linarith\n  _ \u2265 abs (x - p) + abs (x - 15) + (abs (x - p) - 15) := by\n    have h' : abs (abs (x - p) - 15) \u2265 abs (x - p) - 15 := by\n      nlinarith [abs_nonneg (x - p)]\n    linarith\n  _ = 2 * abs (x - p) + abs (x - 15) - 15 := by ring\n  _ \u2265 2 * abs (x - p) + (15 - x) - 15 := by\n    have hx15 : x \u2264 15 := hx_le_15\n    have : abs (x - 15) = 15 - x := by\n      rw [abs_of_nonpos (sub_nonpos.mpr hx15)]\n      ring\n    rw [this]\n    nlinarith\n  _ = 2 * abs (x - p) - x := by ring\n  _ \u2265 2 * (x - p) - x := by\n    have hxp : x - p \u2265 0 := sub_nonneg.mpr hx_ge_p\n    have : abs (x - p) = x - p := abs_of_nonneg hxp\n    rw [this]\n    nlinarith\n  _ = x - 2 * p := by ring\n  _ \u2265 15 - 2 * p := by nlinarith\n  _ \u2265 15 := by nlinarith", "gp_path": "per_tactic", "gp_proof_file": "proofs/aime_1983_p2_1777138897_a9617ab3.lean", "calibration_mode": "control", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1821:{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777139401511", "problem_id": "aime_1983_p2", "solved": false, "split": "adaptation", "verified": false, "golden_path_token_count": 0, "total_run_token_count": 40023, "total_wall_time_ms": 504246, "progress": 0, "pput_runtime": 0.0, "pput_verified": 0.0, "pput_m_verified": 0.0, "failed_branch_count": 50, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": false, "time_secs": 504.246494102, "pput": 0.0, "gp_token_count": 0, "gp_node_count": 0, "tx_count": 50, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"step_partial_ok": 3, "parse_fail": 3, "step_reject": 44, "step": 47}, "synthetic_short_circuit": true, "calibration_mode": "treatment", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1927:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:1959:   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2081:/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2095:    13	    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2151:    69	            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2155:    73	                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2235:   153	    p0 = sum(per_problem_regression.values()) / n_problems
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2244:   162	        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2245:   163	        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2246:   164	        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2276:   194	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2287:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,300p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2306:    18	#     of compute_p0.py requires every (problem, seed) pair present
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2307:    19	#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2314:    26	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2366:    78	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2368:    80	    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2370:    82	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2406:   118	PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2409:   121	if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2410:   122	    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2413:   125	PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2414:   126	    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2415:   127	if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2416:   128	    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2417:   129	    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2438:   150	# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2439:   151	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2456:   168	    "calibration_problem_id": "$4",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2457:   169	    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2505:   217	                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2513:   225	            EXIT=0
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2531:   243	row['calibration_problem_id'] = os.environ['PID_ENV']
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2574:   286	# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2578:   290	    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2580:   292	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2581:   293	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2587:   299	P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2607:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2624:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2639:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2640:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2641:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2642:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2645:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2815:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '297,340p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2819:   299	P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2821:   301	python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2841:   321	    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2999:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3340:    14	// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3351:    25	// non-equivalence. `compute_p0.py` honors this by reading only the
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3443:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3463:    25	//   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3464:    26	//   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3547:   109	        "handover/preregistration/scripts/run_p0_calibration.sh",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3548:   110	        "handover/preregistration/scripts/compute_p0.py",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3651:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3684:    45	#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3686:    47	#     handover/preregistration/scripts/compute_p0.py (same — estimator is
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3693:    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3711:    72	"handover/preregistration/scripts/run_p0_calibration.sh" = "9a1e216301eb5dba72351a49a5a4c799e4bbf0dee79646467d28d972f8196cf8"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3712:    73	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3751:    19	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3800:    68	**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3817:    85	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3820:    88	- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:3845:   113	1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4022:    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4026:    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4045:script = pathlib.Path('handover/preregistration/scripts/compute_p0.py').resolve()
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4046:spec = importlib.util.spec_from_file_location('compute_p0', script)
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4051:    return {'calibration_problem_id': pid, 'calibration_seed': seed, 'progress': progress}
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4066:{"ceiling_pass": false, "n_control_solved": 3, "n_pairs": 4, "n_problems": 2, "n_regression_pairs": 2, "n_regression_problems_max_seed": 2, "n_treatment_solved": 1, "p0": 1.0, "p0_ceiling": 0.1}
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4079:            row={'calibration_problem_id':pid,'calibration_seed':seed,'progress':1}
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4081:            # 15 problem-level regressions => p0=15/144 > .10
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4082:            tr={'calibration_problem_id':pid,'calibration_seed':seed,'progress':0 if i<15 else 1}
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4085:python3 handover/preregistration/scripts/compute_p0.py --control \""'$control" --treatment "$treatment" --out-json "$tmp/out.json" >/tmp/compute_p0_probe.out 2>/tmp/compute_p0_probe.err; code=$?; echo code=$code; head -20 /tmp/compute_p0_probe.out; cat /tmp/compute_p0_probe.err; test -f "$tmp/out.json" && echo out_json_present' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4095:  "p0": 0.10416666666666667,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4096:  "p0_ceiling": 0.1,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4368:The estimator probe shows the intended hard failure modes: incomplete/duplicate rows exit via `SystemExit` and a p0 ceiling breach returns exit `2`. I accidentally invoked one cargo test command with multiple filters, then restarted with valid filters; they’re compiling now and I’ll wait for those results before closing the verdict.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4372:/bin/bash -lc 'rg -n "TRUST_ROOT_TAMPERED|synthetic_timeout_or_crash|synthetic_reason|evaluator_crash|TIMEOUT|CRASH|compute_p0|progress" handover/preregistration/scripts/compute_p0.py handover/preregistration/scripts/run_p0_calibration.sh experiments/minif2f_v4/src/bin/evaluator.rs' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4378:experiments/minif2f_v4/src/bin/evaluator.rs:151:    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4379:experiments/minif2f_v4/src/bin/evaluator.rs:183:        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4391:handover/preregistration/scripts/run_p0_calibration.sh:18:#     of compute_p0.py requires every (problem, seed) pair present
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4392:handover/preregistration/scripts/run_p0_calibration.sh:19:#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4393:handover/preregistration/scripts/run_p0_calibration.sh:150:# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4394:handover/preregistration/scripts/run_p0_calibration.sh:151:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4395:handover/preregistration/scripts/run_p0_calibration.sh:164:    "progress": 0,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4396:handover/preregistration/scripts/run_p0_calibration.sh:169:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4397:handover/preregistration/scripts/run_p0_calibration.sh:170:    "synthetic_reason": "$5",
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4398:handover/preregistration/scripts/run_p0_calibration.sh:248:                SOLVED_FLAG=$(echo "$ENRICHED" | python3 -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('progress', 1 if r.get('has_golden_path') else 0)))")
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4399:handover/preregistration/scripts/run_p0_calibration.sh:257:                    echo "TIMEOUT (exit=124) — emitting synthetic UNSOLVED row"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4400:handover/preregistration/scripts/run_p0_calibration.sh:260:                    echo "CRASH (exit=$EXIT) — emitting synthetic UNSOLVED row"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4401:handover/preregistration/scripts/run_p0_calibration.sh:261:                    emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "evaluator_crash" "$EXIT"
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4402:handover/preregistration/scripts/run_p0_calibration.sh:286:# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4403:handover/preregistration/scripts/run_p0_calibration.sh:290:    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4404:handover/preregistration/scripts/run_p0_calibration.sh:292:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4405:handover/preregistration/scripts/run_p0_calibration.sh:293:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4406:handover/preregistration/scripts/run_p0_calibration.sh:301:python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4407:handover/preregistration/scripts/run_p0_calibration.sh:321:    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4408:handover/preregistration/scripts/compute_p0.py:13:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4409:handover/preregistration/scripts/compute_p0.py:38:    """PREREG § 1.3 progress = 1 iff Lean ground-truth verifies golden_path.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4410:handover/preregistration/scripts/compute_p0.py:40:    Reads the v2 RunAggregate field `progress` (jsonl_schema.rs:96). Falls
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4411:handover/preregistration/scripts/compute_p0.py:42:    found this function was reading a non-existent `progress_verified`
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4412:handover/preregistration/scripts/compute_p0.py:45:    if "progress" in row and row["progress"] is not None:
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4413:handover/preregistration/scripts/compute_p0.py:46:        return int(row["progress"]) == 1
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4425:                    'calibration_problem_id': pid,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4428:                    'synthetic_timeout_or_crash': True,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4434:python3 handover/preregistration/scripts/compute_p0.py --control \""'$control" --treatment "$treatment" --out-json "$tmp/out.json" >/tmp/compute_p0_all_synth.out 2>/tmp/compute_p0_all_synth.err; code=$?; echo code=$code; cat /tmp/compute_p0_all_synth.out; cat /tmp/compute_p0_all_synth.err' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4444:  "p0": 0.0,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4445:  "p0_ceiling": 0.1,
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4456:   The evaluator now panics on Trust Root failure at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:181), but the runner captures any nonzero evaluator exit from [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:226) and emits a synthetic UNSOLVED row for all non-timeout crashes at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:254). `compute_p0.py` then reads only `progress` at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:45) and accepts the row into the regression estimator at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:131).  
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4457:   I verified with a complete synthetic `evaluator_crash` dataset: `compute_p0.py` returned exit `0`, `p0=0.0`, `ceiling_pass=true`. That means a systemic boot/integrity failure can become a complete passing calibration and reach the freeze path at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:308). This is not safe to launch.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4460:   `RunAggregate` requires `golden_path_token_count` at [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:91), and `schema_version` causes v2 parsing at [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:173). The synthetic emitter sets `schema_version: v2.0` but omits `golden_path_token_count` in [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:156). `compute_p0.py` will tolerate this, but downstream v2 tooling reasonably will not.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4466:- **RQ2 / B2:** Strict completeness is materially fixed: missing tags and duplicates fail at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:66), seed mismatch at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:90), problem count and pair count at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:110), denominator pinned to 144 at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151). Timeout-as-UNSOLVED is defensible under a fixed wall-clock budget; arbitrary crash-as-UNSOLVED is not.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4468:- **RQ3 / B3:** Ceiling enforcement is fixed locally: `compute_p0.py` returns `2` on ceiling breach at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:192), and the runner aborts at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:313). I verified a 15/144 regression fixture returns exit `2`. Compute errors are separated from ceiling errors at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:319). The remaining failure is upstream: crash rows can make the estimator pass.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4470:- **RQ5/RQ6:** `MODEL_SNAPSHOT`/`BUILD_SHA` stamping is adequate for pairing at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:68) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1216). The code-simplifier changes are semantically fine: one-shot SHA-256 at [boot.rs](/home/zephryj/projects/turingosv4/src/boot.rs:84), `String` hex write at [boot.rs](/home/zephryj/projects/turingosv4/src/boot.rs:163), and `matches!` env check at [rollback_sim.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/rollback_sim.rs:45). Minor doc drift: TRACE_MATRIX still says manifest size 16 at [TRACE_MATRIX_v1_2026-04-25.md](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:68) while `genesis_payload.toml` says 20 at [genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:50).
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4482:   The evaluator now panics on Trust Root failure at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:181), but the runner captures any nonzero evaluator exit from [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:226) and emits a synthetic UNSOLVED row for all non-timeout crashes at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:254). `compute_p0.py` then reads only `progress` at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:45) and accepts the row into the regression estimator at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:131).  
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4483:   I verified with a complete synthetic `evaluator_crash` dataset: `compute_p0.py` returned exit `0`, `p0=0.0`, `ceiling_pass=true`. That means a systemic boot/integrity failure can become a complete passing calibration and reach the freeze path at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:308). This is not safe to launch.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4486:   `RunAggregate` requires `golden_path_token_count` at [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:91), and `schema_version` causes v2 parsing at [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:173). The synthetic emitter sets `schema_version: v2.0` but omits `golden_path_token_count` in [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:156). `compute_p0.py` will tolerate this, but downstream v2 tooling reasonably will not.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4492:- **RQ2 / B2:** Strict completeness is materially fixed: missing tags and duplicates fail at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:66), seed mismatch at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:90), problem count and pair count at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:110), denominator pinned to 144 at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151). Timeout-as-UNSOLVED is defensible under a fixed wall-clock budget; arbitrary crash-as-UNSOLVED is not.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4494:- **RQ3 / B3:** Ceiling enforcement is fixed locally: `compute_p0.py` returns `2` on ceiling breach at [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:192), and the runner aborts at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:313). I verified a 15/144 regression fixture returns exit `2`. Compute errors are separated from ceiling errors at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:319). The remaining failure is upstream: crash rows can make the estimator pass.
./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4496:- **RQ5/RQ6:** `MODEL_SNAPSHOT`/`BUILD_SHA` stamping is adequate for pairing at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:68) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1216). The code-simplifier changes are semantically fine: one-shot SHA-256 at [boot.rs](/home/zephryj/projects/turingosv4/src/boot.rs:84), `String` hex write at [boot.rs](/home/zephryj/projects/turingosv4/src/boot.rs:163), and `matches!` env check at [rollback_sim.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/rollback_sim.rs:45). Minor doc drift: TRACE_MATRIX still says manifest size 16 at [TRACE_MATRIX_v1_2026-04-25.md](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:68) while `genesis_payload.toml` says 20 at [genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:50).
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:31:- **Round 2** Codex VETO again — round-1 "synthetic UNSOLVED on any non-zero exit" silently absorbed TRUST_ROOT_TAMPERED panics. Fixed in `1df1f62` (crash-vs-timeout discrimination).
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:32:- **Round 3** Codex CHALLENGE: same silent-absorption class via `problem_file_missing` synthetic emit (verified all-missing 144×2 → p0=0.0); also boot preflight `|| true` discards exit code. Gemini CHALLENGE: EXIT=0+empty PPUT_RESULT case fell through to generic crash branch. **All addressed in commit `d0d474e`**.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:37:2. Boot preflight exit-code assertion (Codex P0 #2): captures PREFLIGHT_EXIT explicitly; asserts exit != 0 AND exit != 124; then content grep for TRUST_ROOT_TAMPERED. Three failure modes (exit 0, exit 124, panic with TRUST_ROOT_TAMPERED) get distinct diagnostic messages.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:38:3. EXIT=0 + empty PPUT_RESULT explicit branch (Gemini CHALLENGE): malformed-run case now exits 5 with dedicated diagnostic, instead of falling into generic crash branch.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:59:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:60:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:67:| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:68:| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:73:| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:74:| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:77:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:94:`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:97:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:98:- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:101:runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:105:- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:108:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:112:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:157:// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:168:// non-equivalence. `compute_p0.py` honors this by reading only the
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:272:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:289:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:304:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:305:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:306:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:307:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:310:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:546:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:550:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:571:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:604:#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:606:#     handover/preregistration/scripts/compute_p0.py (same — estimator is
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:613:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:631:"handover/preregistration/scripts/run_p0_calibration.sh" = "323c7f9d23209447b253048c7017f394b6b6ac24f162057863da331e54a148bb"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:632:"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:652:## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:672:#     of compute_p0.py requires every (problem, seed) pair present
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:673:#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:680:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:732:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:734:    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:736:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:772:PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:775:if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:776:    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:779:PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:780:    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:781:if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:782:    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:783:    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:795:PREFLIGHT_EXIT=0
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:796:PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:797:if [ "$PREFLIGHT_EXIT" -eq 0 ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:798:    echo "PREFLIGHT FAIL: evaluator exited 0 on nonexistent problem path"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:801:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:804:if [ "$PREFLIGHT_EXIT" -eq 124 ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:805:    echo "PREFLIGHT FAIL: evaluator timed out (exit 124) at boot."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:807:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:810:if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:811:    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:812:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:817:if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:818:   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:819:    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly (exit=$PREFLIGHT_EXIT):"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:820:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:823:echo "Evaluator boot preflight OK (exit=$PREFLIGHT_EXIT, no Trust Root panic, no timeout)."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:828:# p0=0.0 — same silent-absorption class as the round-2 VETO. The
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:843:    echo "PREFLIGHT FAIL: ${#MISSING_FILES[@]} adaptation problem file(s) missing:"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:871:# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:872:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:895:    "calibration_problem_id": "$4",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:896:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:958:            EXIT=0
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:970:            # Round-3 Gemini CHALLENGE fix: explicitly handle the EXIT=0 +
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:976:                echo "MALFORMED (exit=0 but no PPUT_RESULT line) — ABORTING BATCH"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:990:row['calibration_problem_id'] = os.environ['PID_ENV']
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1004:                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1011:                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1017:                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1019:                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1022:                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1054:# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1058:    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1060:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1061:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1067:P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1069:python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1089:    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1096:## handover/preregistration/scripts/compute_p0.py (audit-fixed)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1111:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1167:            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1171:                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1251:    p0 = sum(per_problem_regression.values()) / n_problems
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1260:        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1261:        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1262:        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1292:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1325:+    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1355:+        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1441:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1493:- 4 from 2026-04-25 dual-audit fixes (`src/main.rs`, `Cargo.lock`, `handover/preregistration/scripts/run_p0_calibration.sh`, `handover/preregistration/scripts/compute_p0.py`)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1495:Will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step). Final size after Phase B → C exit: 21 files.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1512:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1515:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1540:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1747:// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1767://   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1768://   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1851:        "handover/preregistration/scripts/run_p0_calibration.sh",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1852:        "handover/preregistration/scripts/compute_p0.py",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1946:{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777138897126", "problem_id": "aime_1983_p2", "solved": true, "split": "adaptation", "verified": true, "golden_path_token_count": 1420, "total_run_token_count": 12240, "total_wall_time_ms": 149170, "progress": 1, "pput_runtime": 5.476928766188159e-07, "pput_verified": 5.476928766188159e-07, "pput_m_verified": 0.5476928766188158, "failed_branch_count": 14, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": true, "time_secs": 149.172273801, "pput": 0.6703658625824985, "gp_token_count": 1420, "gp_node_count": 6, "tx_count": 15, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"omega_wtool": 1, "step": 15, "step_reject": 9, "step_partial_ok": 5}, "gp_payload": "rcases h\u2080 with \u27e8hp_pos, hp_lt_15\u27e9\nrcases h\u2081 with \u27e8hx_ge_p, hx_le_15\u27e9\nhave hx_nonneg : 0 \u2264 x := by linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\nhave hx_nonneg : 0 \u2264 x := by\n  have hp_nonneg : 0 \u2264 p := by linarith\n  linarith\ncalc\n  f x = abs (x - p) + abs (x - 15) + abs (x - p - 15) := by\n    rw [h\u2082]\n  _ = abs (x - p) + abs (x - 15) + abs ((x - p) - 15) := by ring\n  _ \u2265 abs (x - p) + abs (x - 15) + abs (abs (x - p) - 15) := by\n    have h : abs ((x - p) - 15) \u2265 abs (abs (x - p) - 15) := by\n      exact abs_sub_abs_le_abs_sub _ _\n    linarith\n  _ \u2265 abs (x - p) + abs (x - 15) + (abs (x - p) - 15) := by\n    have h' : abs (abs (x - p) - 15) \u2265 abs (x - p) - 15 := by\n      nlinarith [abs_nonneg (x - p)]\n    linarith\n  _ = 2 * abs (x - p) + abs (x - 15) - 15 := by ring\n  _ \u2265 2 * abs (x - p) + (15 - x) - 15 := by\n    have hx15 : x \u2264 15 := hx_le_15\n    have : abs (x - 15) = 15 - x := by\n      rw [abs_of_nonpos (sub_nonpos.mpr hx15)]\n      ring\n    rw [this]\n    nlinarith\n  _ = 2 * abs (x - p) - x := by ring\n  _ \u2265 2 * (x - p) - x := by\n    have hxp : x - p \u2265 0 := sub_nonneg.mpr hx_ge_p\n    have : abs (x - p) = x - p := abs_of_nonneg hxp\n    rw [this]\n    nlinarith\n  _ = x - 2 * p := by ring\n  _ \u2265 15 - 2 * p := by nlinarith\n  _ \u2265 15 := by nlinarith", "gp_path": "per_tactic", "gp_proof_file": "proofs/aime_1983_p2_1777138897_a9617ab3.lean", "calibration_mode": "control", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1953:{"schema_version": "v2.0", "run_id": "n3_aime_1983_p2_1777139401511", "problem_id": "aime_1983_p2", "solved": false, "split": "adaptation", "verified": false, "golden_path_token_count": 0, "total_run_token_count": 40023, "total_wall_time_ms": 504246, "progress": 0, "pput_runtime": 0.0, "pput_verified": 0.0, "pput_m_verified": 0.0, "failed_branch_count": 50, "rollback_count": 0, "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0, "model_snapshot": "deepseek-chat@438a6481c907-dirty", "git_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "binary_sha256": "", "mode": "full", "problem": "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/aime_1983_p2.lean", "condition": "n3", "model": "deepseek-chat", "has_golden_path": false, "time_secs": 504.246494102, "pput": 0.0, "gp_token_count": 0, "gp_node_count": 0, "tx_count": 50, "build_sha": "438a6481c90746c17982abebe50d466ec4bac378-dirty", "classifier_version": "v1_2026-04-16-a", "boltzmann_seed": 31415, "tool_dist": {"step_partial_ok": 3, "parse_fail": 3, "step_reject": 44, "step": 47}, "synthetic_short_circuit": true, "calibration_mode": "treatment", "calibration_seed": 31415, "calibration_problem_id": "aime_1983_p2"}
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2078:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2110:   183	        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2140:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,330p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2159:    18	#     of compute_p0.py requires every (problem, seed) pair present
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2160:    19	#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2167:    26	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2219:    78	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2221:    80	    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2223:    82	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2259:   118	PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2262:   121	if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2263:   122	    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2266:   125	PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2267:   126	    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2268:   127	if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2269:   128	    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2270:   129	    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2282:   141	PREFLIGHT_EXIT=0
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2283:   142	PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2284:   143	if [ "$PREFLIGHT_EXIT" -eq 0 ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2285:   144	    echo "PREFLIGHT FAIL: evaluator exited 0 on nonexistent problem path"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2288:   147	    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2291:   150	if [ "$PREFLIGHT_EXIT" -eq 124 ]; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2292:   151	    echo "PREFLIGHT FAIL: evaluator timed out (exit 124) at boot."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2294:   153	    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2297:   156	if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2298:   157	    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2299:   158	    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2304:   163	if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2305:   164	   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2306:   165	    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly (exit=$PREFLIGHT_EXIT):"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2307:   166	    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2310:   169	echo "Evaluator boot preflight OK (exit=$PREFLIGHT_EXIT, no Trust Root panic, no timeout)."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2315:   174	# p0=0.0 — same silent-absorption class as the round-2 VETO. The
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2330:   189	    echo "PREFLIGHT FAIL: ${#MISSING_FILES[@]} adaptation problem file(s) missing:"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2358:   217	# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2359:   218	# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2382:   241	    "calibration_problem_id": "$4",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2383:   242	    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2445:   304	            EXIT=0
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2457:   316	            # Round-3 Gemini CHALLENGE fix: explicitly handle the EXIT=0 +
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2463:   322	                echo "MALFORMED (exit=0 but no PPUT_RESULT line) — ABORTING BATCH"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2474:/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2488:    13	    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2544:    69	            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2548:    73	                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2628:   153	    p0 = sum(per_problem_regression.values()) / n_problems
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2637:   162	        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2638:   163	        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2639:   164	        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2669:   194	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2696:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2713:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2728:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2729:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2730:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2731:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2734:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2914:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '330,460p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2922:   336	row['calibration_problem_id'] = os.environ['PID_ENV']
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2936:   350	                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2943:   357	                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2949:   363	                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2951:   365	                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2954:   368	                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2986:   400	# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2990:   404	    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2992:   406	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2993:   407	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2999:   413	P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3001:   415	python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3021:   435	    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3033:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3053:    25	//   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3054:    26	//   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3137:   109	        "handover/preregistration/scripts/run_p0_calibration.sh",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3138:   110	        "handover/preregistration/scripts/compute_p0.py",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3240:    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3244:    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3265:    14	// `(problem_id, seed, solved)` tuple consumed by `compute_p0.py` for
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3276:    25	// non-equivalence. `compute_p0.py` honors this by reading only the
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3586:/bin/bash -lc "rg -n \"MODEL_SNAPSHOT|BUILD_SHA|synthetic_timeout|synthetic_short|progress|golden_path_token_count|binary_sha256|RunAggregate\" experiments/minif2f_v4/src -g '*.rs' handover/preregistration/scripts -g '*.py' -g '*.sh'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3588:handover/preregistration/scripts/run_p0_calibration.sh:21:#   - MODEL_SNAPSHOT + GIT_SHA stamped in env for drift detection
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3589:handover/preregistration/scripts/run_p0_calibration.sh:66:# Drift-detection provenance (Codex Q7). MODEL_SNAPSHOT seeds the evaluator's
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3590:handover/preregistration/scripts/run_p0_calibration.sh:73:export MODEL_SNAPSHOT="${MODEL_SNAPSHOT:-${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3591:handover/preregistration/scripts/run_p0_calibration.sh:74:export BUILD_SHA="${BUILD_SHA:-${GIT_SHA}${GIT_DIRTY}}"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3592:handover/preregistration/scripts/run_p0_calibration.sh:211:echo "MODEL_SNAPSHOT: $MODEL_SNAPSHOT"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3593:handover/preregistration/scripts/run_p0_calibration.sh:212:echo "BUILD_SHA:     $BUILD_SHA"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3594:handover/preregistration/scripts/run_p0_calibration.sh:218:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3595:handover/preregistration/scripts/run_p0_calibration.sh:223:    # Emits a v2 RunAggregate-conformant row — Codex re-audit CHALLENGE 2:
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3596:handover/preregistration/scripts/run_p0_calibration.sh:224:    # `golden_path_token_count` is required by jsonl_schema.rs RunAggregate
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3597:handover/preregistration/scripts/run_p0_calibration.sh:237:    "progress": 0,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3598:handover/preregistration/scripts/run_p0_calibration.sh:242:    "synthetic_timeout_or_crash": True,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3599:handover/preregistration/scripts/run_p0_calibration.sh:245:    "model_snapshot": "$MODEL_SNAPSHOT",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3600:handover/preregistration/scripts/run_p0_calibration.sh:246:    "build_sha": "$BUILD_SHA",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3601:handover/preregistration/scripts/run_p0_calibration.sh:249:    "golden_path_token_count": 0,
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3602:handover/preregistration/scripts/run_p0_calibration.sh:258:    "git_sha": "$BUILD_SHA",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3603:handover/preregistration/scripts/run_p0_calibration.sh:259:    "binary_sha256": "",
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3604:handover/preregistration/scripts/run_p0_calibration.sh:310:                MODEL_SNAPSHOT="$MODEL_SNAPSHOT" \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3605:handover/preregistration/scripts/run_p0_calibration.sh:311:                BUILD_SHA="$BUILD_SHA" \
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3606:handover/preregistration/scripts/run_p0_calibration.sh:341:                SOLVED_FLAG=$(echo "$ENRICHED" | python3 -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('progress', 1 if r.get('has_golden_path') else 0)))")
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3607:handover/preregistration/scripts/run_p0_calibration.sh:350:                # synthetic_timeout_or_crash=true disambiguator.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3608:handover/preregistration/scripts/run_p0_calibration.sh:393:echo "║ MODEL_SNAPSHOT:   $MODEL_SNAPSHOT"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3609:handover/preregistration/scripts/run_p0_calibration.sh:394:echo "║ BUILD_SHA:        $BUILD_SHA"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3610:handover/preregistration/scripts/run_p0_calibration.sh:406:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3611:handover/preregistration/scripts/run_p0_calibration.sh:407:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3683:handover/preregistration/scripts/compute_p0.py:38:    """PREREG § 1.3 progress = 1 iff Lean ground-truth verifies golden_path.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3684:handover/preregistration/scripts/compute_p0.py:40:    Reads the v2 RunAggregate field `progress` (jsonl_schema.rs:96). Falls
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3685:handover/preregistration/scripts/compute_p0.py:42:    found this function was reading a non-existent `progress_verified`
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3686:handover/preregistration/scripts/compute_p0.py:45:    if "progress" in row and row["progress"] is not None:
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3687:handover/preregistration/scripts/compute_p0.py:46:        return int(row["progress"]) == 1
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3735:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3768:    45	#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3770:    47	#     handover/preregistration/scripts/compute_p0.py (same — estimator is
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3777:    54	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3795:    72	"handover/preregistration/scripts/run_p0_calibration.sh" = "323c7f9d23209447b253048c7017f394b6b6ac24f162057863da331e54a148bb"
./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3796:    73	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
./handover/audits/run_codex_b7_extra_round4_audit.sh:20:- **Round 2** Codex VETO again — round-1 "synthetic UNSOLVED on any non-zero exit" silently absorbed TRUST_ROOT_TAMPERED panics. Fixed in `1df1f62` (crash-vs-timeout discrimination).
./handover/audits/run_codex_b7_extra_round4_audit.sh:21:- **Round 3** Codex CHALLENGE: same silent-absorption class via `problem_file_missing` synthetic emit (verified all-missing 144×2 → p0=0.0); also boot preflight `|| true` discards exit code. Gemini CHALLENGE: EXIT=0+empty PPUT_RESULT case fell through to generic crash branch. **All addressed in commit `d0d474e`**.
./handover/audits/run_codex_b7_extra_round4_audit.sh:26:2. Boot preflight exit-code assertion (Codex P0 #2): captures PREFLIGHT_EXIT explicitly; asserts exit != 0 AND exit != 124; then content grep for TRUST_ROOT_TAMPERED. Three failure modes (exit 0, exit 124, panic with TRUST_ROOT_TAMPERED) get distinct diagnostic messages.
./handover/audits/run_codex_b7_extra_round4_audit.sh:27:3. EXIT=0 + empty PPUT_RESULT explicit branch (Gemini CHALLENGE): malformed-run case now exits 5 with dedicated diagnostic, instead of falling into generic crash branch.
./handover/audits/run_codex_b7_extra_round4_audit.sh:48:- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
./handover/audits/run_codex_b7_extra_round4_audit.sh:49:- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)
./handover/audits/run_codex_b7_extra_round4_audit.sh:56:| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
./handover/audits/run_codex_b7_extra_round4_audit.sh:57:| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
./handover/audits/run_codex_b7_extra_round4_audit.sh:62:| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
./handover/audits/run_codex_b7_extra_round4_audit.sh:63:| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
./handover/audits/run_codex_b7_extra_round4_audit.sh:66:| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
./handover/audits/run_codex_b7_extra_round4_audit.sh:83:`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.
./handover/audits/run_codex_b7_extra_round4_audit.sh:86:- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
./handover/audits/run_codex_b7_extra_round4_audit.sh:87:- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?
./handover/audits/run_codex_b7_extra_round4_audit.sh:90:runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`
./handover/audits/run_codex_b7_extra_round4_audit.sh:94:- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?
./handover/audits/run_codex_b7_extra_round4_audit.sh:97:`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).
./handover/audits/run_codex_b7_extra_round4_audit.sh:101:- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).
./handover/audits/run_codex_b7_extra_round4_audit.sh:144:printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round4_audit.sh:145:cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round4_audit.sh:147:printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n' >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round4_audit.sh:148:cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round4_audit.sh:163:ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_control.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"
./handover/audits/run_codex_b7_extra_round4_audit.sh:166:ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_treatment.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:293:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:295:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
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
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:479:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:481:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1435:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:215:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:320:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:522:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:524:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1078:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1166:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1331:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1333:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2287:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:84:- (Q3.b) `compute_p0.py` per-problem uses `max(over seeds)` — worst-case framing. If only 1 of 2 seeds regressed, the problem still counts as 1. With 2 seeds, this DOUBLES the regression rate vs `mean(over seeds)`. PREREG § 5.5 line 450 explicitly says max — agreed?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:85:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. For a synthetic_short_circuit row, both will be False (correctly UNSOLVED). For a control row that SOLVED, both will be True. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:86:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. If the runner script's row-enrichment failed for any subset (e.g., python3 OOM'd on one row), would calibration silently miss data? Should we fail loudly?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:87:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:104:- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)? `compute_p0.py` joins on calibration_problem_id + calibration_seed and reads only `solved` / `progress_verified` — neither field comes from WAL or recent_rejections. Argument for "no impact": the calibration jsonl IS ground-truth-validated independently. Verify.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:106:- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50 (correctly reflects 50 tx of real activity). Does anything downstream INTERPRET these as "if they were lower than control, the treatment toggle is bug-prone"? compute_p0.py doesn't; flag if any other tooling does.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:110:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2 if ceiling violated.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:113:- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it and continue? Test by inspection: where in the pipeline does compute_p0.py run, and what handles exit code?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:286:// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:303:/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:318:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:319:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:320:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:321:            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:324:                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:572:// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:576:        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:597:#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:630:# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:665:## handover/preregistration/scripts/run_p0_calibration.sh (NEW)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:683:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:725:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:727:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:757:PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:760:if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:761:    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:764:PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:765:    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:766:PREFLIGHT_CODE=$?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:767:if [ "$PREFLIGHT_CODE" -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:768:    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:769:    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:824:row['calibration_problem_id'] = '$PID'
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:857:    echo "calibration_seed + calibration_problem_id are present. Then re-run without"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:861:    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:865:    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:870:## handover/preregistration/scripts/compute_p0.py (NEW)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:885:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:921:    # Index by (problem_id, seed). calibration_problem_id and calibration_seed
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:922:    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:927:            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:964:    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:973:        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:974:        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:975:        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1005:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1038:+    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1131:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1180:**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1197:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1200:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1225:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1289:| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1297:> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1411:**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1470:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1472:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1541:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1672:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2127:    15	// `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2144:    32	/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2159:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2160:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2161:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2162:    50	            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2165:    53	                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2307:    11	// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2311:    15	        panic!("TRUST_ROOT_TAMPERED: {e}");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2485:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2502:    16	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2544:    58	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2546:    60	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2576:    90	PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2579:    93	if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2580:    94	    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2583:    97	PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2584:    98	    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2585:    99	PREFLIGHT_CODE=$?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2586:   100	if [ "$PREFLIGHT_CODE" -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2587:   101	    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2588:   102	    echo "$PREFLIGHT_OUT" | head -c 500
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2643:   157	row['calibration_problem_id'] = '$PID'
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2676:   190	    echo "calibration_seed + calibration_problem_id are present. Then re-run without"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2680:   194	    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2684:   198	    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2688:/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2702:    13	    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2738:    49	    # Index by (problem_id, seed). calibration_problem_id and calibration_seed
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2739:    50	    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2744:    55	            pid = r.get("calibration_problem_id")
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2781:    92	    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2790:   101	        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2791:   102	        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2792:   103	        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2822:   133	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2846:    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2879:    45	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3082:   459	                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3084:   461	               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3119:    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3168:    66	**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3185:    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3188:    86	- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3213:   111	1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3275:    56	| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3283:    64	> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3395:    81	**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
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
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3683:/bin/bash -lc 'rg -n "p0|compute_p0|baseline_regression|p0_calibration|MEASUREMENT_ERROR|synthetic_short_circuit|total_run_token_count" -S .' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3702:./experiments/minif2f_v4/src/bin/evaluator.rs:151:    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3735:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:42:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3739:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:79:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3740:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:80:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. Should we fail loudly?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3741:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:81:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3743:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:104:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3746:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:159:runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3747:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:160:compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3748:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:191:    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n{runner_sh}\n```\n" +
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3749:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:192:    f"\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n{compute_py}\n```\n" +
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3750:./handover/audits/run_gemini_b7_extra_prebatch_audit.py:227:          f"**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3757:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:40:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3761:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:77:- (Q3.b) `compute_p0.py` per-problem uses `max(over seeds)` — worst-case framing. If only 1 of 2 seeds regressed, the problem still counts as 1. With 2 seeds, this DOUBLES the regression rate vs `mean(over seeds)`. PREREG § 5.5 line 450 explicitly says max — agreed?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3762:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:78:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. For a synthetic_short_circuit row, both will be False (correctly UNSOLVED). For a control row that SOLVED, both will be True. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3763:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:79:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. If the runner script's row-enrichment failed for any subset (e.g., python3 OOM'd on one row), would calibration silently miss data? Should we fail loudly?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3764:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:80:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3765:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:97:- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)? `compute_p0.py` joins on calibration_problem_id + calibration_seed and reads only `solved` / `progress_verified` — neither field comes from WAL or recent_rejections. Argument for "no impact": the calibration jsonl IS ground-truth-validated independently. Verify.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3766:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:99:- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50 (correctly reflects 50 tx of real activity). Does anything downstream INTERPRET these as "if they were lower than control, the treatment toggle is bug-prone"? compute_p0.py doesn't; flag if any other tooling does.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3767:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:103:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2 if ceiling violated.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3768:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:106:- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it and continue? Test by inspection: where in the pipeline does compute_p0.py run, and what handles exit code?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3771:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:166:printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (NEW)\n\n```bash\n' >> "$TMP_PROMPT"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3772:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:167:cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3773:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:169:printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (NEW)\n\n```python\n' >> "$TMP_PROMPT"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3774:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:170:cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3775:./handover/audits/run_codex_b7_extra_prebatch_audit.sh:199:  printf '**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n'
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3778:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3780:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:19:    -   **Conclusion**: The equivalence is a convenience that is valid *only because* `compute_p0.py` ignores cost and intra-run state, joining only on the final `SOLVED`/`UNSOLVED` status. The constitutional anchor is weak; it achieves an outcome analogous to FC2-N22 HALT, but it does not *traverse* the constitutional path of FC1-E18 (repeatedly). The documentation in `rollback_sim.rs` should be amended to state this is a *functionally equivalent outcome for p_0 estimation* rather than a generally "observably equivalent" process.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3782:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:41:-   **(Q3.b) `max(over seeds)` framing**: **PASS**. `PREREG § 5.5` line 450 explicitly specifies `max over the 2 seeds`. The implementation in `compute_p0.py:84` correctly reflects this policy (`if regression > per_problem_regression[pid]: ...`). This is a consistent implementation of a pre-registered, "worst-case" policy choice.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3783:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:43:-   **(Q3.c) `solved()` predicate**: **PASS**. The `solved()` function in `compute_p0.py:44-48` correctly prioritizes `progress_verified` over the legacy `has_golden_path`, which aligns with the B4 audit findings and the `RunAggregate::V2` schema.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3784:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:45:-   **(Q3.d) Silent drop in `compute_p0.py`**: **CHALLENGE**. The script at `compute_p0.py:56-58` silently skips rows that are missing `calibration_problem_id` or `calibration_seed`. A failure in the runner script (`run_p0_calibration.sh:170-177`) to stamp a row would cause that data point to be silently dropped, biasing the `p_0` result without warning. The script should fail loudly if any row from either input file is missing the required keys.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3785:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3788:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:64:-   **(Q6.a) `exit 2` abort**: **CHALLENGE**. The runner script `run_p0_calibration.sh` does not use `set -e`. If `compute_p0.py` exits with code 2, the script will continue executing, printing the summary box. An automated runner could miss the failure. The script should use `set -e` or explicitly check the exit code of `compute_p0.py` and abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3789:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:71:-   **(Q7.b) Timeout impact on `p_0`**: **CHALLENGE**. A run that times out is logged as `MEASUREMENT_ERROR` (`run_p0_calibration.sh:195`). This means the (problem, seed) pair will be missing from one of the jsonl files. `compute_p0.py` will then exclude this pair from its analysis (`set(c.keys()) & set(t.keys())`). This introduces sampling bias. A timeout is a valid outcome (UNSOLVED) and should be treated as such, not as a data error. The runner script should be modified to emit a valid UNSOLVED jsonl row for timed-out runs.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3790:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:100:3.  **CHALLENGE Fix**: Modify `compute_p0.py` to fail loudly if any row is missing the required `calibration_*` keys, preventing silent data loss and biased results. (`Q3.d`)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3791:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:101:4.  **CHALLENGE Fix**: Modify `run_p0_calibration.sh` to treat a timeout as a valid `UNSOLVED` outcome and emit a corresponding JSONL row, instead of a `MEASUREMENT_ERROR` that leads to data exclusion. (`Q7.b`)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3800:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:522:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3803:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1331:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3806:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3810:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:- (B) cost asymmetry — short-circuit at tx 50 understates C_i vs true 150-tx vetoed loop; mitigated by synthetic_short_circuit field doc + compute_p0.py only joins on SOLVED/UNSOLVED
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3814:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:84:- (Q3.b) `compute_p0.py` per-problem uses `max(over seeds)` — worst-case framing. If only 1 of 2 seeds regressed, the problem still counts as 1. With 2 seeds, this DOUBLES the regression rate vs `mean(over seeds)`. PREREG § 5.5 line 450 explicitly says max — agreed?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3815:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:85:- (Q3.c) The `solved()` predicate in `compute_p0.py` reads `progress_verified` if present, else `has_golden_path`. For a synthetic_short_circuit row, both will be False (correctly UNSOLVED). For a control row that SOLVED, both will be True. Is the predicate correct under the v2 RunAggregate schema?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3816:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:86:- (Q3.d) `compute_p0.py` skips rows missing `calibration_problem_id` or `calibration_seed` — silent drop. If the runner script's row-enrichment failed for any subset (e.g., python3 OOM'd on one row), would calibration silently miss data? Should we fail loudly?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3817:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:87:- (Q3.e) Boltzmann seeds [31415, 2718] frozen in PREREG; `run_p0_calibration.sh` hardcodes them. If a future drift forces re-calibration, are the seeds in Trust Root manifest sufficient to detect tampering, or could a runner script tamper go undetected?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3818:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:104:- (Q5.a) Do these gaps affect p_0 calibration RESULT (not Phase D consumption)? `compute_p0.py` joins on calibration_problem_id + calibration_seed and reads only `solved` / `progress_verified` — neither field comes from WAL or recent_rejections. Argument for "no impact": the calibration jsonl IS ground-truth-validated independently. Verify.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3819:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:106:- (Q5.c) `failed_branch_count` and `total_run_token_count` for treatment-short-circuit rows are stamped at tx 50 (correctly reflects 50 tx of real activity). Does anything downstream INTERPRET these as "if they were lower than control, the treatment toggle is bug-prone"? compute_p0.py doesn't; flag if any other tooling does.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3820:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:110:PREREG § 5.5 ceiling: if measured p_0 > 0.10, ABORT. compute_p0.py exits 2 if ceiling violated.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3821:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:113:- (Q6.a) Is exit-2 actually an ABORT in the runner workflow, or could a downstream script swallow it and continue? Test by inspection: where in the pipeline does compute_p0.py run, and what handles exit code?
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3828:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:665:## handover/preregistration/scripts/run_p0_calibration.sh (NEW)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3829:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:683:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3830:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:725:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3831:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:727:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3833:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:861:    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3834:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:865:    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3835:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:870:## handover/preregistration/scripts/compute_p0.py (NEW)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3836:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:885:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3837:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:922:    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3838:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:964:    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3839:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:973:        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3840:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:974:        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3841:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:975:        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3843:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1005:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3846:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1038:+    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3853:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1180:**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3857:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1289:| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3860:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1297:> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3864:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1411:**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3867:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1470:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3870:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1672:   151	    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3882:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2485:/bin/bash -lc "nl -ba handover/preregistration/scripts/run_p0_calibration.sh | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3883:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2502:    16	#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3884:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2544:    58	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3885:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2546:    60	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3887:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2680:   194	    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3888:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2684:   198	    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3889:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2688:/bin/bash -lc "nl -ba handover/preregistration/scripts/compute_p0.py | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3890:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2702:    13	    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3891:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2739:    50	    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3892:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2781:    92	    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3893:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2790:   101	        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3894:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2791:   102	        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3895:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2792:   103	        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3897:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2822:   133	            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3907:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3082:   459	                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3908:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3168:    66	**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3912:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3275:    56	| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3915:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3283:    64	> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3919:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3395:    81	**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3921:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3605:/bin/bash -lc "rg -n \"MODEL_SNAPSHOT|ACTIVE_MODEL|BINARY_SHA256|BUILD_SHA|SPLIT|MODE|BOLTZMANN_SEED|max_transactions|MAX\" experiments/minif2f_v4/src/bin/evaluator.rs handover/preregistration/scripts/run_p0_calibration.sh experiments/minif2f_v4/src -g '*.rs'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3922:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3607:handover/preregistration/scripts/run_p0_calibration.sh:25:#   export ACTIVE_MODEL=deepseek-chat           (default)
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3923:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3608:handover/preregistration/scripts/run_p0_calibration.sh:37:export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3924:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3609:handover/preregistration/scripts/run_p0_calibration.sh:42:SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3925:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3610:handover/preregistration/scripts/run_p0_calibration.sh:53:MODES=("control" "treatment")
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3926:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3611:handover/preregistration/scripts/run_p0_calibration.sh:67:d = json.load(open('$SPLITS_JSON'))
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3927:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3612:handover/preregistration/scripts/run_p0_calibration.sh:109:TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3928:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3613:handover/preregistration/scripts/run_p0_calibration.sh:112:echo "Mode count:    ${#MODES[@]} (control + treatment)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3929:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3614:handover/preregistration/scripts/run_p0_calibration.sh:120:for MODE in "${MODES[@]}"; do
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3930:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3615:handover/preregistration/scripts/run_p0_calibration.sh:121:    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3931:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3616:handover/preregistration/scripts/run_p0_calibration.sh:122:    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3932:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3617:handover/preregistration/scripts/run_p0_calibration.sh:125:    case "$MODE" in
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3933:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3618:handover/preregistration/scripts/run_p0_calibration.sh:135:                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND, skip"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3934:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3619:handover/preregistration/scripts/run_p0_calibration.sh:138:            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3935:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3620:handover/preregistration/scripts/run_p0_calibration.sh:139:            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3936:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3621:handover/preregistration/scripts/run_p0_calibration.sh:143:                BOLTZMANN_SEED="$SEED" \
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3937:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3622:handover/preregistration/scripts/run_p0_calibration.sh:155:row['calibration_mode'] = '$MODE'
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3940:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:479:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3951:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:293:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4116:./handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md:81:**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4120:./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:56:| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4123:./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:64:> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4129:./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2166:+    let p0 = snap.get_portfolio("Agent_0").expect("Agent_0 portfolio");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4130:./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2168:+    let pos_a = p0.get(&node_a).expect("Agent_0 position on node A");
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4131:./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2173:+    assert!(p0.get(&node_b).is_none(),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4163:./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:459:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4166:./handover/ai-direct/LATEST.md:48:  - `handover/preregistration/scripts/run_p0_calibration.sh`: iterates adaptation-144 × seeds [31415, 2718] × {control, treatment} = 576 runs; `--smoke` flag = 4-run probe
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4167:./handover/ai-direct/LATEST.md:49:  - `handover/preregistration/scripts/compute_p0.py`: control/treatment pair → regression_p_seed → max-over-seeds → p_0; PREREG § 5.5 ceiling = 0.10
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4169:./handover/ai-direct/LATEST.md:51:  - **Next**: user GO → 576-run batch (~$3-5, ~8h overnight) → compute_p0.py → write p_0 to genesis_payload.toml [pput_accounting_0] → recompute Trust Root + commit jsonl into manifest → Gate B dual-audit Phase B → Phase C transition
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4172:./handover/preregistration/scripts/run_p0_calibration.sh:16:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4173:./handover/preregistration/scripts/run_p0_calibration.sh:58:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4174:./handover/preregistration/scripts/run_p0_calibration.sh:60:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4175:./handover/preregistration/scripts/run_p0_calibration.sh:169:                echo "MEASUREMENT_ERROR"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4176:./handover/preregistration/scripts/run_p0_calibration.sh:194:    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4177:./handover/preregistration/scripts/run_p0_calibration.sh:198:    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4178:./handover/preregistration/scripts/compute_p0.py:13:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4179:./handover/preregistration/scripts/compute_p0.py:50:    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4180:./handover/preregistration/scripts/compute_p0.py:92:    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4181:./handover/preregistration/scripts/compute_p0.py:101:        "p0": p0,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4182:./handover/preregistration/scripts/compute_p0.py:102:        "p0_ceiling": 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4183:./handover/preregistration/scripts/compute_p0.py:103:        "ceiling_pass": p0 <= 0.10,
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4184:./handover/preregistration/scripts/compute_p0.py:128:    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4185:./handover/preregistration/scripts/compute_p0.py:133:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4186:./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:66:**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4193:src/boot.rs:47:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4194:src/boot.rs:48:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4195:src/boot.rs:49:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4239:handover/audits/run_gemini_b7_extra_prebatch_audit.py:227:          f"**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n"
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4252:handover/audits/run_codex_b7_extra_prebatch_audit.sh:199:  printf '**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings\n'
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4257:handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4264:handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4268:handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:47:-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4279:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4291:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4303:handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4310:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3:**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4323:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:318:            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4324:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:319:            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4325:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:320:            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4367:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1131:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4374:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1197:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4386:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2159:    47	            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4387:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2160:    48	            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4388:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:2161:    49	            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4414:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3119:    17	| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4421:handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3185:    83	`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
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
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4629:handover/ai-direct/LATEST.md:42:  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4631:handover/ai-direct/LATEST.md:51:  - **Next**: user GO → 576-run batch (~$3-5, ~8h overnight) → compute_p0.py → write p_0 to genesis_payload.toml [pput_accounting_0] → recompute Trust Root + commit jsonl into manifest → Gate B dual-audit Phase B → Phase C transition
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4637:handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:12:Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4642:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:17:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4649:handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:83:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4654:handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4662:handover/preregistration/scripts/compute_p0.py:124:    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4668:handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:4691:     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5331:/bin/bash -lc 'git ls-files Cargo.lock src/main.rs experiments/minif2f_v4/src/lib.rs handover/preregistration/scripts/run_p0_calibration.sh handover/preregistration/scripts/compute_p0.py' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5335:handover/preregistration/scripts/compute_p0.py
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5336:handover/preregistration/scripts/run_p0_calibration.sh
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5369:   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5371:2. **`compute_p0.py` can silently compute p₀ on an incomplete subset.**  
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5372:   PREREG requires full adaptation-144 × seeds and denominator `/144` ([PREREG §5.5 lines 446-451](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:446)). But the runner swallows timeout/crash with `|| true` ([line 140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) and emits only `MEASUREMENT_ERROR` without a JSON row ([line 168](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:168)). The estimator skips missing tags ([compute_p0.py:55](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:55)), intersects only present pairs ([line 65](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:65)), and divides by observed problems ([line 91](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:91)). Fix: require exactly 144 problems, exactly 288 paired rows per mode, exact seeds `{31415,2718}`, no duplicates, no missing calibration tags, and abort on any `MEASUREMENT_ERROR` unless an explicit pre-registered retry policy handles it.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5375:   `compute_p0.py` returns exit 2 if `p0 > 0.10` ([line 131](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:131)), but the runner never invokes it; it only prints a suggested command ([run_p0_calibration.sh:193](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:193)). Fix: full runner must call `compute_p0.py`, check exit status, write structured output, and prevent freeze if nonzero.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5383:Q3: `max(over seeds)` matches PREREG ([PREREG:448](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:448)) and implementation ([compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88)). But `solved()` should read v2 `progress`, not only nonexistent `progress_verified`; v2 schema uses `progress` ([jsonl_schema.rs:96](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:96)). Current fallback works only because Phase B passes runtime and verified together ([evaluator.rs:1158](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1158)).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5387:Q5: Findings C+D do not affect p₀ directly. `compute_p0.py` reads only solved status from rows ([compute_p0.py:78](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:78)); `verified` is caller-supplied from Lean success paths or explicit false on exhaustion/short-circuit ([evaluator.rs:1218](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1218)). WAL Omega gaps remain Phase D blockers, not this batch’s p₀ blocker.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5391:Q7: API drift and timeout handling are under-specified. The schema has `model_snapshot` ([jsonl_schema.rs:112](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:112)), but runner does not set it. The 40-min timeout ([run_p0_calibration.sh:140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) plus no resume makes failure expensive and bias-prone. Add canary-before/after, exact row coverage, and resume/retry before launch.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5405:   `run_p0_calibration.sh` does not use `set -e` ([line 27](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:27)); `cargo build` is not checked ([line 84](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:84)); then the script runs `target/release/evaluator` regardless ([line 85](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:85)). Also, the evaluator does **not** call `verify_trust_root`; only root `src/main.rs` does ([src/main.rs:14](/home/zephryj/projects/turingosv4/src/main.rs:14)), and this batch never runs that binary. Fix: fail-fast build, verify binary hash, and call `turingosv4::boot::verify_trust_root` from evaluator startup before any calibration run.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5407:2. **`compute_p0.py` can silently compute p₀ on an incomplete subset.**  
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5408:   PREREG requires full adaptation-144 × seeds and denominator `/144` ([PREREG §5.5 lines 446-451](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:446)). But the runner swallows timeout/crash with `|| true` ([line 140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) and emits only `MEASUREMENT_ERROR` without a JSON row ([line 168](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:168)). The estimator skips missing tags ([compute_p0.py:55](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:55)), intersects only present pairs ([line 65](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:65)), and divides by observed problems ([line 91](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:91)). Fix: require exactly 144 problems, exactly 288 paired rows per mode, exact seeds `{31415,2718}`, no duplicates, no missing calibration tags, and abort on any `MEASUREMENT_ERROR` unless an explicit pre-registered retry policy handles it.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5411:   `compute_p0.py` returns exit 2 if `p0 > 0.10` ([line 131](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:131)), but the runner never invokes it; it only prints a suggested command ([run_p0_calibration.sh:193](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:193)). Fix: full runner must call `compute_p0.py`, check exit status, write structured output, and prevent freeze if nonzero.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5419:Q3: `max(over seeds)` matches PREREG ([PREREG:448](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:448)) and implementation ([compute_p0.py:88](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:88)). But `solved()` should read v2 `progress`, not only nonexistent `progress_verified`; v2 schema uses `progress` ([jsonl_schema.rs:96](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:96)). Current fallback works only because Phase B passes runtime and verified together ([evaluator.rs:1158](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1158)).
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5423:Q5: Findings C+D do not affect p₀ directly. `compute_p0.py` reads only solved status from rows ([compute_p0.py:78](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:78)); `verified` is caller-supplied from Lean success paths or explicit false on exhaustion/short-circuit ([evaluator.rs:1218](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1218)). WAL Omega gaps remain Phase D blockers, not this batch’s p₀ blocker.
./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:5427:Q7: API drift and timeout handling are under-specified. The schema has `model_snapshot` ([jsonl_schema.rs:112](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:112)), but runner does not set it. The 40-min timeout ([run_p0_calibration.sh:140](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140)) plus no resume makes failure expensive and bias-prone. Add canary-before/after, exact row coverage, and resume/retry before launch.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:72:**TL;DR**: when a Q7.b "synthetic UNSOLVED on any non-zero exit" was added in round-1 fix to address sampling bias, it silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data — neutralizing the B1 fix that the same round was supposed to deliver. **Codex caught it in round-2; Gemini missed it (PASS).** Per CLAUDE.md "Audit Standard" + memory `feedback_dual_audit_conflict`, conservative reading wins → VETO. Round-2 fix (commit `1df1f62`) discriminates exit codes: only timeout (124) emits synthetic row; any other crash ABORT BATCH with grep for TRUST_ROOT_TAMPERED. Round-3 Gemini returned CHALLENGE on a follow-up exhaustiveness gap (EXIT=0 + empty PPUT_RESULT case fell through to generic crash branch); fixed in same notepad-update cycle. **Lesson**: when fixing a sampling-bias bug, the fix itself can become a security bypass; always re-audit fixes before promoting to PASS. The dual-audit's value is exactly in this kind of cross-checking.
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:84:**TL;DR**: B7 commit shipped 4 new pub symbols (`verify_trust_root`, `parse_trust_root_section`, `TrustRootError`, panic site in main) without TRACE_MATRIX backlinks — violation of CLAUDE.md "Alignment Standard". User flagged. Fixed in commit `0cc48bc`: doc comments added with `/// TRACE_MATRIX FC3-N34: ...` etc; TRACE_MATRIX_v1 written (FC3-N34 ⚠️→✅ promoted, 15 readonly-extension orphan rows with constitutional justification); OBS_BOOT_FAIL_NOT_HALT records that TRUST_ROOT_TAMPERED panic happens before kernel/bus init exists, so it's not a FC2-N22 HALT (no QState to mark Halted) — closer to FC3-E14 immediate-abort variant. **Lesson**: every src/ pub symbol MUST get TRACE_MATRIX backlink in same commit it's introduced. Treating alignment as "follow-up cleanup" leads to drift.
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2166:+    let p0 = snap.get_portfolio("Agent_0").expect("Agent_0 portfolio");
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2168:+    let pos_a = p0.get(&node_a).expect("Agent_0 position on node A");
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2173:+    assert!(p0.get(&node_b).is_none(),
./handover/ai-direct/LATEST.md:3:**Session Summary**: B7 (Trust Root + Boot freeze) → 用户 atomic-alignment critique (3 flowcharts) → B7 alignment fix → B7-extra rollback toggle + calibration scripts → dual audit round-1 VETO/VETO → 13-fix landing → simplifier pass → **constitution amendment (sudo)**: V.1.1 sudo scope + V.1.2 ArchitectAI commit authority + V.1.3 JudgeAI→Veto-AI + V.3 amendment log → re-audit round 2 VETO/PASS (Codex caught self-inflicted regression: Q7.b silently absorbed TRUST_ROOT_TAMPERED panics) → round-2 fix → re-audit round 3 CHALLENGE/CHALLENGE (problem_file_missing absorption + boot preflight `||true` exit-discard + EXIT=0+empty PPUT_RESULT non-exhaustive) → round-3 fix (commit `d0d474e`) → **round-4 audit in flight**. **187/187 cargo test PASS** + 20 ignored. Trust Root manifest **20 files** (was 15 — added main.rs / Cargo.lock / runner.sh / compute_p0.py per audit). User authorized auto-research overnight to PROCEED on PASS/PASS.
./handover/ai-direct/LATEST.md:42:  - `src/main.rs`: pre-Boot `verify_trust_root(env!("CARGO_MANIFEST_DIR"))` panics with `TRUST_ROOT_TAMPERED: ...` on any error; replaces previous placeholder
./handover/ai-direct/LATEST.md:48:  - `handover/preregistration/scripts/run_p0_calibration.sh`: iterates adaptation-144 × seeds [31415, 2718] × {control, treatment} = 576 runs; `--smoke` flag = 4-run probe
./handover/ai-direct/LATEST.md:49:  - `handover/preregistration/scripts/compute_p0.py`: control/treatment pair → regression_p_seed → max-over-seeds → p_0; PREREG § 5.5 ceiling = 0.10
./handover/ai-direct/LATEST.md:51:  - **Next**: user GO → 576-run batch (~$3-5, ~8h overnight) → compute_p0.py → write p_0 to genesis_payload.toml [pput_accounting_0] → recompute Trust Root + commit jsonl into manifest → Gate B dual-audit Phase B → Phase C transition
./handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md:81:**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:56:| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
./handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md:64:> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:1:# OBS — Boot failure (TRUST_ROOT_TAMPERED) is not a constitutional HALT
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:12:Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:16:## Why TRUST_ROOT_TAMPERED ≠ FC2-N22 HALT
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:18:| Property | FC2-N22 HALT | TRUST_ROOT_TAMPERED panic |
./handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:26:FC2 has a `HALT` node (FC2-N22) that lives *inside* the boot/tick lifecycle. TRUST_ROOT_TAMPERED fires *before* the boot lifecycle — it is a precondition violation on the readonly base.
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:19:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:71:- 4 from 2026-04-25 dual-audit fixes (`src/main.rs`, `Cargo.lock`, `handover/preregistration/scripts/run_p0_calibration.sh`, `handover/preregistration/scripts/compute_p0.py`)
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:73:Will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step). Final size after Phase B → C exit: 21 files.
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:90:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:93:- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
./handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:118:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:302:- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:152:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:257:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:459:                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:461:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/preregistration/scripts/run_p0_calibration.sh:18:#     of compute_p0.py requires every (problem, seed) pair present
./handover/preregistration/scripts/run_p0_calibration.sh:19:#   - runner invokes compute_p0.py at end with exit-code propagation
./handover/preregistration/scripts/run_p0_calibration.sh:26:#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
./handover/preregistration/scripts/run_p0_calibration.sh:78:    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
./handover/preregistration/scripts/run_p0_calibration.sh:80:    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
./handover/preregistration/scripts/run_p0_calibration.sh:82:    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
./handover/preregistration/scripts/run_p0_calibration.sh:118:PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
./handover/preregistration/scripts/run_p0_calibration.sh:121:if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
./handover/preregistration/scripts/run_p0_calibration.sh:122:    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
./handover/preregistration/scripts/run_p0_calibration.sh:125:PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
./handover/preregistration/scripts/run_p0_calibration.sh:126:    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
./handover/preregistration/scripts/run_p0_calibration.sh:127:if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
./handover/preregistration/scripts/run_p0_calibration.sh:128:    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
./handover/preregistration/scripts/run_p0_calibration.sh:129:    echo "$PREFLIGHT_OUT" | head -c 500
./handover/preregistration/scripts/run_p0_calibration.sh:141:PREFLIGHT_EXIT=0
./handover/preregistration/scripts/run_p0_calibration.sh:142:PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
./handover/preregistration/scripts/run_p0_calibration.sh:143:if [ "$PREFLIGHT_EXIT" -eq 0 ]; then
./handover/preregistration/scripts/run_p0_calibration.sh:144:    echo "PREFLIGHT FAIL: evaluator exited 0 on nonexistent problem path"
./handover/preregistration/scripts/run_p0_calibration.sh:147:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/preregistration/scripts/run_p0_calibration.sh:150:if [ "$PREFLIGHT_EXIT" -eq 124 ]; then
./handover/preregistration/scripts/run_p0_calibration.sh:151:    echo "PREFLIGHT FAIL: evaluator timed out (exit 124) at boot."
./handover/preregistration/scripts/run_p0_calibration.sh:153:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/preregistration/scripts/run_p0_calibration.sh:156:if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
./handover/preregistration/scripts/run_p0_calibration.sh:157:    echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
./handover/preregistration/scripts/run_p0_calibration.sh:158:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/preregistration/scripts/run_p0_calibration.sh:163:if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
./handover/preregistration/scripts/run_p0_calibration.sh:164:   ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
./handover/preregistration/scripts/run_p0_calibration.sh:165:    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly (exit=$PREFLIGHT_EXIT):"
./handover/preregistration/scripts/run_p0_calibration.sh:166:    echo "$PREFLIGHT_PROBE" | head -c 800
./handover/preregistration/scripts/run_p0_calibration.sh:169:echo "Evaluator boot preflight OK (exit=$PREFLIGHT_EXIT, no Trust Root panic, no timeout)."
./handover/preregistration/scripts/run_p0_calibration.sh:174:# p0=0.0 — same silent-absorption class as the round-2 VETO. The
./handover/preregistration/scripts/run_p0_calibration.sh:189:    echo "PREFLIGHT FAIL: ${#MISSING_FILES[@]} adaptation problem file(s) missing:"
./handover/preregistration/scripts/run_p0_calibration.sh:217:# strict-completeness compute_p0 join sees every pair. The synthesized row
./handover/preregistration/scripts/run_p0_calibration.sh:218:# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
./handover/preregistration/scripts/run_p0_calibration.sh:241:    "calibration_problem_id": "$4",
./handover/preregistration/scripts/run_p0_calibration.sh:242:    "synthetic_timeout_or_crash": True,
./handover/preregistration/scripts/run_p0_calibration.sh:304:            EXIT=0
./handover/preregistration/scripts/run_p0_calibration.sh:316:            # Round-3 Gemini CHALLENGE fix: explicitly handle the EXIT=0 +
./handover/preregistration/scripts/run_p0_calibration.sh:322:                echo "MALFORMED (exit=0 but no PPUT_RESULT line) — ABORTING BATCH"
./handover/preregistration/scripts/run_p0_calibration.sh:336:row['calibration_problem_id'] = os.environ['PID_ENV']
./handover/preregistration/scripts/run_p0_calibration.sh:350:                # synthetic_timeout_or_crash=true disambiguator.
./handover/preregistration/scripts/run_p0_calibration.sh:357:                # Especially TRUST_ROOT_TAMPERED panics MUST abort the batch
./handover/preregistration/scripts/run_p0_calibration.sh:363:                if grep -q "TRUST_ROOT_TAMPERED" "$STDERR_LOG"; then
./handover/preregistration/scripts/run_p0_calibration.sh:365:                    echo "  ✗ DETECTED: TRUST_ROOT_TAMPERED in evaluator stderr"
./handover/preregistration/scripts/run_p0_calibration.sh:368:                    grep "TRUST_ROOT_TAMPERED" "$STDERR_LOG" | head -3 | sed 's/^/    /'
./handover/preregistration/scripts/run_p0_calibration.sh:400:# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
./handover/preregistration/scripts/run_p0_calibration.sh:404:    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
./handover/preregistration/scripts/run_p0_calibration.sh:406:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/preregistration/scripts/run_p0_calibration.sh:407:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
./handover/preregistration/scripts/run_p0_calibration.sh:413:P0_JSON="${OUT_PREFIX}_p0_result.json"
./handover/preregistration/scripts/run_p0_calibration.sh:415:python3 "$SCRIPT_DIR/compute_p0.py" \
./handover/preregistration/scripts/run_p0_calibration.sh:435:    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
./handover/preregistration/scripts/compute_p0.py:13:    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
./handover/preregistration/scripts/compute_p0.py:69:            pid = r.get("calibration_problem_id")
./handover/preregistration/scripts/compute_p0.py:73:                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
./handover/preregistration/scripts/compute_p0.py:153:    p0 = sum(per_problem_regression.values()) / n_problems
./handover/preregistration/scripts/compute_p0.py:162:        "p0": p0,
./handover/preregistration/scripts/compute_p0.py:163:        "p0_ceiling": 0.10,
./handover/preregistration/scripts/compute_p0.py:164:        "ceiling_pass": p0 <= 0.10,
./handover/preregistration/scripts/compute_p0.py:194:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",

codex
The core round-3 paths are visible now: missing adaptation files abort before the loop, evaluator boot preflight captures exit status, and the in-loop `EXIT=0/no PPUT_RESULT` case exits 5. I’m checking whether any remaining branch can still turn setup corruption into a valid UNSOLVED row.
exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
import hashlib, re, sys
root=Path('.').resolve()
text=(root/'genesis_payload.toml').read_text()
in_section=False
entries=[]
for line in text.splitlines():
    stripped=line.split('#',1)[0].strip()
    if not stripped:
        continue
    if stripped.startswith('[') and stripped.endswith(']'):
        in_section = stripped == '[trust_root]'
        continue
    if in_section:
        m=re.match(r'\"(["'^"]+)"'"\\s*=\\s*\"([0-9a-f]+)\""'$'"', stripped)
        if not m:
            print('parse_fail', stripped)
            sys.exit(1)
        entries.append(m.groups())
errors=[]
for rel, expected in entries:
    actual=hashlib.sha256((root/rel).read_bytes()).hexdigest()
    if actual "'!= expected:
        errors.append((rel, expected, actual))
print(f'"'entries={len(entries)} mismatches={len(errors)}')
for rel, exp, act in errors[:10]:
    print(f'{rel}: expected {exp} actual {act}')
sys.exit(1 if errors else 0)
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
entries=20 mismatches=0

exec
/bin/bash -lc 'cargo test -p minif2f_v4 --test trust_root_immutability' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'tmp=$(mktemp -d)
python3 - <<'"'PY' \""'$tmp"
import json, pathlib, sys
root=pathlib.Path(sys.argv[1])
seeds=(31415,2718)
problems=[f'"'p{i:03d}' for i in range(144)]
for name, treatment in [('control', False), ('treatment', True)]:
    with (root/f'{name}.jsonl').open('w') as f:
        for pid in problems:
            for seed in seeds:
                progress=1
                if treatment and pid == 'p000' and seed == 31415:
                    progress=0
                f.write(json.dumps({'calibration_problem_id':pid,'calibration_seed':seed,'progress':progress})+'\\n')
PY
python3 handover/preregistration/scripts/compute_p0.py --control \""'$tmp/control.jsonl" --treatment "$tmp/treatment.jsonl" --out-json "$tmp/out.json"' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'tmp=$(mktemp -d)
printf '"'%s\\n' '{\"calibration_problem_id\":\"p001\",\"calibration_seed\":31415,\"progress\":1}' > \""'$tmp/control.jsonl"
printf '"'%s\\n' '{\"calibration_problem_id\":\"p001\",\"calibration_seed\":31415,\"progress\":0}' > \""'$tmp/treatment.jsonl"
python3 handover/preregistration/scripts/compute_p0.py --control "$tmp/control.jsonl" --treatment "$tmp/treatment.jsonl"' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
ERROR: seed mismatch — expected [2718, 31415]; control=[31415], treatment=[31415]

 succeeded in 118ms:
{
  "n_problems": 144,
  "n_pairs": 288,
  "n_control_solved": 288,
  "n_treatment_solved": 287,
  "n_regression_pairs": 1,
  "n_regression_problems_max_seed": 1,
  "p0": 0.006944444444444444,
  "p0_ceiling": 0.1,
  "ceiling_pass": true
}

[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):
  8b6a6b8fed23ee149db912d8d7efe1190f82f0793b0946cf39a3f1bea5e913cf

 succeeded in 291ms:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running tests/trust_root_immutability.rs (target/debug/deps/trust_root_immutability-e3f95f3eb1354a13)

running 4 tests
test test_pput_accounting_0_section_present ... ok
test test_trust_root_simulated_write_aborts ... ok
test test_trust_root_manifest_includes_b2_b4_files ... ok
test test_trust_root_immutable_at_boot ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


codex
Focused verification matched the static read: the Trust Root test suite passes, all 20 manifest hashes match the working tree, and `compute_p0.py` rejects a partial one-problem dataset with exit 1 instead of computing. I’m adding one ceiling-path check because that is the B3 gating behavior.
exec
/bin/bash -lc 'tmp=$(mktemp -d)
python3 - <<'"'PY' \""'$tmp"
import json, pathlib, sys
root=pathlib.Path(sys.argv[1])
seeds=(31415,2718)
problems=[f'"'p{i:03d}' for i in range(144)]
for name, treatment in [('control', False), ('treatment', True)]:
    with (root/f'{name}.jsonl').open('w') as f:
        for pid in problems:
            for seed in seeds:
                progress=1
                if treatment and int(pid[1:]) < 15 and seed == 31415:
                    progress=0
                f.write(json.dumps({'calibration_problem_id':pid,'calibration_seed':seed,'progress':progress})+'\\n')
PY
python3 handover/preregistration/scripts/compute_p0.py --control \""'$tmp/control.jsonl" --treatment "$tmp/treatment.jsonl" --out-json "$tmp/out.json"' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'cargo test -p minif2f_v4 rollback_sim' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'cargo test -p turingosv4 boot' in /home/zephryj/projects/turingosv4
 exited 2 in 126ms:

ERROR: p_0 = 0.1042 > 0.10 — ABORT per PREREG § 5.5 ceiling.
{
  "n_problems": 144,
  "n_pairs": 288,
  "n_control_solved": 288,
  "n_treatment_solved": 273,
  "n_regression_pairs": 15,
  "n_regression_problems_max_seed": 15,
  "p0": 0.10416666666666667,
  "p0_ceiling": 0.1,
  "ceiling_pass": false
}

[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):
  54d7eab4433893e9320a368374b397ef3b8450c663cb3b509aed4355b24f7645

 succeeded in 323ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests src/lib.rs (target/debug/deps/minif2f_v4-ea0fb4b84b011e86)

running 6 tests
test rollback_sim::tests::does_not_fire_after_threshold ... ok
test rollback_sim::tests::does_not_fire_before_threshold ... ok
test rollback_sim::tests::fires_at_threshold_when_enabled ... ok
test rollback_sim::tests::env_var_name_matches_prereg ... ok
test rollback_sim::tests::threshold_constant_matches_prereg ... ok
test rollback_sim::tests::never_fires_when_disabled ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.00s

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

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s


 succeeded in 3545ms:
    Blocking waiting for file lock on package cache
   Compiling turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
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

warning: `turingosv4` (lib test) generated 8 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 8 suggestions)
warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 9 warnings (8 duplicates) (run `cargo fix --lib -p turingosv4` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.41s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-51250c76ee802e8d)

running 6 tests
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 122 filtered out; finished in 0.04s

     Running unittests src/main.rs (target/debug/deps/turingosv4-109432aa5e1a43ce)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/reward_pull_conservation.rs (target/debug/deps/reward_pull_conservation-343f2eb6f66d7ddd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/wal_resume.rs (target/debug/deps/wal_resume-4081e9642f73c7dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s


exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
import json
p=Path('handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json')
d=json.loads(p.read_text())
ids=d['splits']['adaptation']['problem_ids']
print(f'adaptation_count={len(ids)} unique={len(set(ids))} first={ids[0]} last={ids[-1]}')
print(f'seed_contract=[31415,2718]')
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
adaptation_count=144 unique=144 first=aime_1983_p2 last=numbertheory_x5neqy2p4
seed_contract=[31415,2718]

exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
import json, os
root=Path('/home/zephryj/projects/turingosv4')
minif=Path(os.environ.get('MINIF2F_DIR','/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4'))
ids=json.loads((root/'handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json').read_text())['splits']['adaptation']['problem_ids']
missing=[pid for pid in ids if not (minif/'MiniF2F/Test'/f'{pid}.lean').is_file()]
print(f'minif2f_dir={minif}')
print(f'checked={len(ids)} missing={len(missing)}')
for pid in missing[:10]: print(pid)
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
minif2f_dir=/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4
checked=144 missing=0

exec
/bin/bash -lc 'cargo build --release -p minif2f_v4' in /home/zephryj/projects/turingosv4
 succeeded in 233ms:
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
    Finished `release` profile [optimized] target(s) in 0.16s

exec
/bin/bash -lc 'PREFLIGHT_EXIT=0
PREFLIGHT_PROBE=$(timeout 30 /home/zephryj/projects/turingosv4/target/release/evaluator /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
printf '"'exit=%s\\n' \""'$PREFLIGHT_EXIT"
printf '"'%s\\n' \""'$PREFLIGHT_PROBE" | head -20' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exit=1
Problem file not found: /nonexistent_problem_path.lean

exec
/bin/bash -lc 'git diff --unified=80 1df1f62..d0d474e -- handover/preregistration/scripts/run_p0_calibration.sh genesis_payload.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
diff --git a/genesis_payload.toml b/genesis_payload.toml
index 0debc98..117c905 100644
--- a/genesis_payload.toml
+++ b/genesis_payload.toml
@@ -1,89 +1,89 @@
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
-"handover/preregistration/scripts/run_p0_calibration.sh" = "92701c2876a69968a4f570a67d39c56e15da0a45d44720d4fe1b6174ecdbd821"
+"handover/preregistration/scripts/run_p0_calibration.sh" = "323c7f9d23209447b253048c7017f394b6b6ac24f162057863da331e54a148bb"
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
diff --git a/handover/preregistration/scripts/run_p0_calibration.sh b/handover/preregistration/scripts/run_p0_calibration.sh
index c8b59af..7bb0aac 100755
--- a/handover/preregistration/scripts/run_p0_calibration.sh
+++ b/handover/preregistration/scripts/run_p0_calibration.sh
@@ -54,295 +54,357 @@ case "$MODE_ARG" in
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
 
-# Audit-fix 2026-04-25 round-2 (Codex re-audit VETO): evaluator boot preflight.
-# Run the evaluator binary on a trivial fixture path BEFORE the batch starts,
-# specifically to catch TRUST_ROOT_TAMPERED at boot. If Trust Root is tampered,
-# the in-loop crash-vs-timeout discrimination would still catch it, but a
-# preflight surfaces the failure with the clean diagnostic and zero wasted
-# API spend.
+# Audit-fix 2026-04-25 round-2 (Codex VETO) + round-3 (Codex P0 #2):
+# evaluator boot preflight with EXIT-CODE assertion + content grep.
+# A nonexistent problem MUST cause evaluator to exit non-zero AND not
+# timeout. Round-3 Codex caught: if preflight times out (124) or exits 0
+# (impossible-but-defensive), runner falsely printed "OK" because grep
+# alone doesn't surface those failure modes.
 echo "[$(date -Is)] Evaluator boot preflight (Trust Root verify)..."
-PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1 || true)
+PREFLIGHT_EXIT=0
+PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
+if [ "$PREFLIGHT_EXIT" -eq 0 ]; then
+    echo "PREFLIGHT FAIL: evaluator exited 0 on nonexistent problem path"
+    echo "  (expected: non-zero exit due to problem-not-found OR Trust Root"
+    echo "   panic). exit=0 means a code path silently succeeded with bad input."
+    echo "$PREFLIGHT_PROBE" | head -c 800
+    exit 2
+fi
+if [ "$PREFLIGHT_EXIT" -eq 124 ]; then
+    echo "PREFLIGHT FAIL: evaluator timed out (exit 124) at boot."
+    echo "  Trust Root verify or env_logger init may be hanging. Investigate."
+    echo "$PREFLIGHT_PROBE" | head -c 800
+    exit 2
+fi
 if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
     echo "PREFLIGHT FAIL: TRUST_ROOT_TAMPERED at evaluator boot. ABORTING."
     echo "$PREFLIGHT_PROBE" | head -c 800
     exit 2
 fi
 # Expected: evaluator exits with usage error or problem-not-found (non-zero,
-# but no panic). Anything containing "panicked at" except expected-args
-# error is suspicious.
+# not 124, no Trust Root panic). Other panics indicate boot regression.
 if echo "$PREFLIGHT_PROBE" | grep -q "panicked at" && \
    ! echo "$PREFLIGHT_PROBE" | grep -q "Usage:"; then
-    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly:"
+    echo "PREFLIGHT FAIL: evaluator panicked unexpectedly (exit=$PREFLIGHT_EXIT):"
     echo "$PREFLIGHT_PROBE" | head -c 800
     exit 2
 fi
-echo "Evaluator boot preflight OK."
+echo "Evaluator boot preflight OK (exit=$PREFLIGHT_EXIT, no Trust Root panic, no timeout)."
+
+# Round-3 Codex P0 #1 fix: pre-loop adaptation-file existence preflight.
+# A missing problem file MUST abort the batch, not produce a synthetic
+# UNSOLVED row. Codex verified a 144×2 all-missing dataset returned
+# p0=0.0 — same silent-absorption class as the round-2 VETO. The
+# correct posture: every (problem, seed, mode) coordinate that the
+# strict-complete estimator expects must come from a real evaluator run
+# (or a legitimate timeout). Missing problem file = setup error =
+# operator must investigate before launching.
+echo "[$(date -Is)] Adaptation file existence preflight..."
+MISSING_FILES=()
+while IFS= read -r PID; do
+    [ -z "$PID" ] && continue
+    PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
+    if [ ! -f "$PROBLEM" ]; then
+        MISSING_FILES+=("$PID")
+    fi
+done <<< "$ADAPTATION_IDS"
+if [ "${#MISSING_FILES[@]}" -gt 0 ]; then
+    echo "PREFLIGHT FAIL: ${#MISSING_FILES[@]} adaptation problem file(s) missing:"
+    for pid in "${MISSING_FILES[@]:0:10}"; do
+        echo "  - $pid (expected at $MINIF2F_DIR/MiniF2F/Test/${pid}.lean)"
+    done
+    [ "${#MISSING_FILES[@]}" -gt 10 ] && echo "  ... and $((${#MISSING_FILES[@]} - 10)) more"
+    echo ""
+    echo "Refuse to launch batch with incomplete adaptation set."
+    echo "Investigate MINIF2F_DIR ($MINIF2F_DIR) and re-verify file presence."
+    exit 2
+fi
+echo "Adaptation file existence preflight OK ($(echo "$ADAPTATION_IDS" | wc -l) files present)."
 
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
-                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND"
-                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
-                continue
+                # Round-3 Codex P0 #1 fix: this should never trigger now —
+                # the pre-loop adaptation-file existence preflight (above)
+                # already aborted the batch if any file was missing. Race
+                # condition (file deleted mid-batch) = abort, no synthetic
+                # row. Silent absorption class.
+                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND mid-batch (race)"
+                echo "  ✗ File disappeared between preflight and run. ABORTING."
+                exit 6
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
-            if [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then
+            # Round-3 Gemini CHALLENGE fix: explicitly handle the EXIT=0 +
+            # empty PPUT_JSON case (e.g., evaluator silently exits 0 without
+            # emitting PPUT_RESULT — malformed run, not a legitimate UNSOLVED).
+            # Without this branch the malformed run falls into the generic
+            # crash branch with a misleading "exit=0" message.
+            if [ "$EXIT" -eq 0 ] && [ -z "$PPUT_JSON" ]; then
+                echo "MALFORMED (exit=0 but no PPUT_RESULT line) — ABORTING BATCH"
+                echo ""
+                echo "  ✗ Evaluator returned 0 but emitted no PPUT_RESULT line."
+                echo "  ✗ This indicates a code bug (silent success path missing emit)"
+                echo "  ✗ rather than a runtime failure. Calibration data NOT trusted."
+                echo "  ✗ Last 5 stderr lines:"
+                tail -5 "$STDERR_LOG" | sed 's/^/    /'
+                exit 5
+            elif [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then
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
 

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '430,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_MATRIX_v1_2026-04-25.md | sed -n '60,78p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    60	| `experiments/minif2f_v4/src/bin/evaluator.rs` | the wiring; tampering with it defeats every layer above |
    61	| `constitution.md` | FC3-N3 (constitution as ground truth) — direct |
    62	| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
    63	| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | sealed heldout split — pre-registration commitment per § 2.3 |
    64	| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | the spec being committed to — pre-registration anchor |
    65	
    66	`genesis_payload.toml` itself is **not** self-hashed (chicken-and-egg). The semantic anchor is the `[pput_accounting_0]` section content, not its hash. Section 6 below records this limitation.
    67	
    68	**Total manifest size**: **20 files** as of 2026-04-25 post-audit-fix. Composition:
    69	- 15 from B7 (PREREG § 1.8 base 8 + audit accounting 6 + B6 prompt_guard)
    70	- 1 from B7-extra (`rollback_sim.rs`)
    71	- 4 from 2026-04-25 dual-audit fixes (`src/main.rs`, `Cargo.lock`, `handover/preregistration/scripts/run_p0_calibration.sh`, `handover/preregistration/scripts/compute_p0.py`)
    72	
    73	Will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step). Final size after Phase B → C exit: 21 files.
    74	
    75	---
    76	
    77	## § 4. `src/boot.rs` is **not** in the Trust Root manifest
    78	

exec
/bin/bash -lc 'rg -n "LEAN_BINARY|LEAN_PATH|lean" experiments/minif2f_v4/src/lean4_oracle.rs experiments/minif2f_v4/src/bin/evaluator.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/evaluator.rs:8:// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)
experiments/minif2f_v4/src/bin/evaluator.rs:10:use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
experiments/minif2f_v4/src/bin/evaluator.rs:17:use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
experiments/minif2f_v4/src/bin/evaluator.rs:38:const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";
experiments/minif2f_v4/src/bin/evaluator.rs:57:    /// Problem identifier: theorem stem (basename of .lean without extension).
experiments/minif2f_v4/src/bin/evaluator.rs:59:    /// Legacy "did the run reach OMEGA" boolean (= runtime_accepted in B4 vocab).
experiments/minif2f_v4/src/bin/evaluator.rs:131:    // re-run `lean --stdin` from disk artifacts alone, without trusting in-memory runtime.
experiments/minif2f_v4/src/bin/evaluator.rs:134:    // gp_proof_file = relative path to the standalone .lean archive (problem + proof).
experiments/minif2f_v4/src/bin/evaluator.rs:193:        eprintln!("Usage: evaluator <problem_file.lean>");
experiments/minif2f_v4/src/bin/evaluator.rs:212:    let lean_path = derive_lean_path(&minif2f_dir);
experiments/minif2f_v4/src/bin/evaluator.rs:218:                       &lean_path, &proxy_url, &model).await
experiments/minif2f_v4/src/bin/evaluator.rs:225:                     &lean_path, &proxy_url, &model, n).await
experiments/minif2f_v4/src/bin/evaluator.rs:274:    lean_path: &str, proxy_url: &str, model: &str,
experiments/minif2f_v4/src/bin/evaluator.rs:281:        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
experiments/minif2f_v4/src/bin/evaluator.rs:292:    // Chat models (deepseek-chat, 2026-04-22) default to ```lean fences; verifier hard-rejects
experiments/minif2f_v4/src/bin/evaluator.rs:394:    lean_path: &str, proxy_url: &str, model: &str, n_agents: usize,
experiments/minif2f_v4/src/bin/evaluator.rs:466:        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
experiments/minif2f_v4/src/bin/evaluator.rs:722:        // value at runtime even when the prompt builder is clean. Gate
experiments/minif2f_v4/src/bin/evaluator.rs:797:                                    lean_path.to_string(),
experiments/minif2f_v4/src/bin/evaluator.rs:823:                                        // verifiers can re-run lean from disk alone.
experiments/minif2f_v4/src/bin/evaluator.rs:898:                                        let class = classify_lean_error(&err_detail);
experiments/minif2f_v4/src/bin/evaluator.rs:1037:                                    lean_path.to_string(),
experiments/minif2f_v4/src/bin/evaluator.rs:1099:                                        let class = classify_lean_error(&reason);
experiments/minif2f_v4/src/bin/evaluator.rs:1221:    // problem_id = basename without .lean
experiments/minif2f_v4/src/bin/evaluator.rs:1282:/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
experiments/minif2f_v4/src/bin/evaluator.rs:1284:/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
experiments/minif2f_v4/src/bin/evaluator.rs:1303:    let fname = format!("{}_{}_{}.lean", theorem_name, ts, short);
experiments/minif2f_v4/src/bin/evaluator.rs:1312:         -- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file\n\
experiments/minif2f_v4/src/bin/evaluator.rs:1351:            "test_problem.lean", "oneshot", "deepseek-v4-flash",
experiments/minif2f_v4/src/bin/evaluator.rs:1405:            "test_problem.lean", "oneshot", "deepseek-v4-flash",
experiments/minif2f_v4/src/lean4_oracle.rs:2:// Constitutional basis: Art. I.1 (boolean predicate — pass/fail only)
experiments/minif2f_v4/src/lean4_oracle.rs:31:    lean_path: String,
experiments/minif2f_v4/src/lean4_oracle.rs:32:    lean_binary: String,
experiments/minif2f_v4/src/lean4_oracle.rs:36:    pub fn new(problem_statement: String, theorem_name: String, lean_path: String) -> Self {
experiments/minif2f_v4/src/lean4_oracle.rs:37:        // Use LEAN_BINARY env or auto-detect from MiniF2F lean-toolchain version.
experiments/minif2f_v4/src/lean4_oracle.rs:38:        // Default: v4.24.0 (matches pre-built Mathlib oleans).
experiments/minif2f_v4/src/lean4_oracle.rs:39:        let lean_binary = std::env::var("LEAN_BINARY").unwrap_or_else(|_| {
experiments/minif2f_v4/src/lean4_oracle.rs:41:            let v4_24 = format!("{}/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean", home);
experiments/minif2f_v4/src/lean4_oracle.rs:45:                "lean".to_string()
experiments/minif2f_v4/src/lean4_oracle.rs:51:            lean_path,
experiments/minif2f_v4/src/lean4_oracle.rs:52:            lean_binary,
experiments/minif2f_v4/src/lean4_oracle.rs:91:    /// OMEGA verification — the ultimate boolean predicate.
experiments/minif2f_v4/src/lean4_oracle.rs:104:    /// Callers MUST pass through classify_lean_error before broadcast (C-022).
experiments/minif2f_v4/src/lean4_oracle.rs:130:            &self.lean_binary,
experiments/minif2f_v4/src/lean4_oracle.rs:134:        // Set LEAN_PATH environment for Mathlib resolution
experiments/minif2f_v4/src/lean4_oracle.rs:135:        std::env::set_var("LEAN_PATH", &self.lean_path);
experiments/minif2f_v4/src/lean4_oracle.rs:192:            &self.lean_binary,
experiments/minif2f_v4/src/lean4_oracle.rs:195:        std::env::set_var("LEAN_PATH", &self.lean_path);
experiments/minif2f_v4/src/lean4_oracle.rs:225:                    // Clean compile, no "No goals to be solved" marker, no
experiments/minif2f_v4/src/lean4_oracle.rs:242:            Ok(SandboxResult::Timeout) => PartialVerdict::Reject("lean_timeout".into()),
experiments/minif2f_v4/src/lean4_oracle.rs:262:        "lean4_oracle"
experiments/minif2f_v4/src/lean4_oracle.rs:290:/// Derive LEAN_PATH from the MiniF2F data directory.
experiments/minif2f_v4/src/lean4_oracle.rs:291:/// Searches .lake/packages/*/.lake/build/lib/lean (Lake 4 layout).
experiments/minif2f_v4/src/lean4_oracle.rs:292:pub fn derive_lean_path(minif2f_dir: &str) -> String {
experiments/minif2f_v4/src/lean4_oracle.rs:298:            // Lake 4 layout: packages/<pkg>/.lake/build/lib/lean
experiments/minif2f_v4/src/lean4_oracle.rs:299:            let build_lib = entry.path().join(".lake/build/lib/lean");
experiments/minif2f_v4/src/lean4_oracle.rs:303:            // Fallback: packages/<pkg>/lib/lean (older layout)
experiments/minif2f_v4/src/lean4_oracle.rs:304:            let lib_lean = entry.path().join("lib").join("lean");
experiments/minif2f_v4/src/lean4_oracle.rs:305:            if lib_lean.is_dir() {
experiments/minif2f_v4/src/lean4_oracle.rs:306:                paths.push(lib_lean.display().to_string());
experiments/minif2f_v4/src/lean4_oracle.rs:312:    let project_lib = PathBuf::from(minif2f_dir).join(".lake/build/lib/lean");
experiments/minif2f_v4/src/lean4_oracle.rs:320:/// Load a problem file: read the .lean file, extract theorem name, prepare for agents.
experiments/minif2f_v4/src/lean4_oracle.rs:390:    fn test_clean_tactic_accepted() {

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/lean4_oracle.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Engine 3: Popperian Guillotine — Lean 4 Oracle
     2	// Constitutional basis: Art. I.1 (boolean predicate — pass/fail only)
     3	// V3L-01: sorry 3-layer defense (upstream filter + oracle reject + post-hoc)
     4	// V3L-02: oracle determinism (same input → same output)
     5	// V3L-07: identity theft prevention (reject new theorem declarations)
     6	
     7	use std::path::PathBuf;
     8	use std::time::Duration;
     9	use turingosv4::sdk::sandbox::{LocalProcessSandbox, SandboxEngine, SandboxResult};
    10	use turingosv4::sdk::tool::{ToolSignal, TuringTool};
    11	use std::any::Any;
    12	
    13	/// Forbidden patterns in agent-submitted code.
    14	/// These are checked BEFORE sending to Lean 4.
    15	const FORBIDDEN_PATTERNS: &[&str] = &[
    16	    "#eval", "#check", "#reduce", "#exec", "#print",  // output/reflection
    17	    "native_decide",                                     // bytecode bypass
    18	    "IO.Process", "IO.FS", "System.FilePath",           // system escape
    19	    "run_tac", "unsafe", "dbg_trace", "IO.println",    // meta/debug
    20	];
    21	
    22	/// Identity theft patterns — declarations that rename the target theorem.
    23	const DECLARATION_KEYWORDS: &[&str] = &[
    24	    "theorem ", "lemma ", "def ", "example ", "instance ",
    25	    "structure ", "class ", "inductive ", "abbrev ",
    26	];
    27	
    28	pub struct Lean4Oracle {
    29	    pub problem_statement: String,
    30	    pub theorem_name: String,
    31	    lean_path: String,
    32	    lean_binary: String,
    33	}
    34	
    35	impl Lean4Oracle {
    36	    pub fn new(problem_statement: String, theorem_name: String, lean_path: String) -> Self {
    37	        // Use LEAN_BINARY env or auto-detect from MiniF2F lean-toolchain version.
    38	        // Default: v4.24.0 (matches pre-built Mathlib oleans).
    39	        let lean_binary = std::env::var("LEAN_BINARY").unwrap_or_else(|_| {
    40	            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    41	            let v4_24 = format!("{}/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean", home);
    42	            if std::path::Path::new(&v4_24).exists() {
    43	                v4_24
    44	            } else {
    45	                "lean".to_string()
    46	            }
    47	        });
    48	        Lean4Oracle {
    49	            problem_statement,
    50	            theorem_name,
    51	            lean_path,
    52	            lean_binary,
    53	        }
    54	    }
    55	
    56	    /// Pre-append security checks (Law 1: reject-only).
    57	    pub fn check_payload(&self, payload: &str) -> Result<(), String> {
    58	        // V3L-07: identity theft — reject new declarations with different names
    59	        for keyword in DECLARATION_KEYWORDS {
    60	            if let Some(pos) = payload.find(keyword) {
    61	                let after = &payload[pos + keyword.len()..];
    62	                let declared_name: String = after.chars()
    63	                    .take_while(|c| c.is_alphanumeric() || *c == '_')
    64	                    .collect();
    65	                if !declared_name.is_empty() && declared_name != self.theorem_name {
    66	                    return Err(format!(
    67	                        "Identity theft: declared '{}' but target is '{}'",
    68	                        declared_name, self.theorem_name
    69	                    ));
    70	                }
    71	            }
    72	        }
    73	
    74	        // V3L-01: sorry firewall
    75	        for word in ["sorry", "sorryAx"] {
    76	            if has_word_boundary(payload, word) {
    77	                return Err(format!("Forbidden: '{}' detected", word));
    78	            }
    79	        }
    80	
    81	        // Forbidden patterns
    82	        for pattern in FORBIDDEN_PATTERNS {
    83	            if payload.contains(pattern) {
    84	                return Err(format!("Forbidden pattern: '{}'", pattern));
    85	            }
    86	        }
    87	
    88	        Ok(())
    89	    }
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
   191	        let sandbox = LocalProcessSandbox::new(
   192	            &self.lean_binary,
   193	            &["--stdin"],
   194	        );
   195	        std::env::set_var("LEAN_PATH", &self.lean_path);
   196	
   197	        match sandbox.execute(&full_code, timeout) {
   198	            Ok(SandboxResult::Completed { stdout, stderr, exit_code }) => {
   199	                let combined = format!("{}\n{}", stdout, stderr);
   200	                if combined.contains("declaration uses 'sorry'") {
   201	                    return PartialVerdict::Reject("declaration_uses_sorry".into());
   202	                }
   203	                if combined.contains("No goals to be solved") {
   204	                    return PartialVerdict::Complete;
   205	                }
   206	                // Lean reports "unsolved goals" when a proof compiles but does
   207	                // not discharge all obligations. This is EXACTLY the partial-
   208	                // OK case: the tactics ran without type errors, but more work
   209	                // remains. Under the old semantics this was a REJECT; under
   210	                // Phase 7, it is the signal that Q_{t+1} is well-formed.
   211	                if combined.contains("unsolved goals") && !combined.contains("error: unknown") && !combined.contains("error: type mismatch") {
   212	                    // Only accept as partial if there are no OTHER errors.
   213	                    // Distinguish unsolved-goals (partial OK) from actual bugs.
   214	                    let hard_errors: Vec<&str> = combined.lines()
   215	                        .filter(|l| l.contains("error:") && !l.contains("unsolved goals"))
   216	                        .collect();
   217	                    if hard_errors.is_empty() {
   218	                        return PartialVerdict::PartialOk;
   219	                    }
   220	                    // Has both unsolved-goals and a hard error → reject

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/lean4_oracle.rs | sed -n '197,255p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   197	        match sandbox.execute(&full_code, timeout) {
   198	            Ok(SandboxResult::Completed { stdout, stderr, exit_code }) => {
   199	                let combined = format!("{}\n{}", stdout, stderr);
   200	                if combined.contains("declaration uses 'sorry'") {
   201	                    return PartialVerdict::Reject("declaration_uses_sorry".into());
   202	                }
   203	                if combined.contains("No goals to be solved") {
   204	                    return PartialVerdict::Complete;
   205	                }
   206	                // Lean reports "unsolved goals" when a proof compiles but does
   207	                // not discharge all obligations. This is EXACTLY the partial-
   208	                // OK case: the tactics ran without type errors, but more work
   209	                // remains. Under the old semantics this was a REJECT; under
   210	                // Phase 7, it is the signal that Q_{t+1} is well-formed.
   211	                if combined.contains("unsolved goals") && !combined.contains("error: unknown") && !combined.contains("error: type mismatch") {
   212	                    // Only accept as partial if there are no OTHER errors.
   213	                    // Distinguish unsolved-goals (partial OK) from actual bugs.
   214	                    let hard_errors: Vec<&str> = combined.lines()
   215	                        .filter(|l| l.contains("error:") && !l.contains("unsolved goals"))
   216	                        .collect();
   217	                    if hard_errors.is_empty() {
   218	                        return PartialVerdict::PartialOk;
   219	                    }
   220	                    // Has both unsolved-goals and a hard error → reject
   221	                    let detail = hard_errors.iter().take(3).cloned().collect::<Vec<_>>().join(" | ");
   222	                    return PartialVerdict::Reject(format!("hard_error_with_unsolved: {}", detail));
   223	                }
   224	                if exit_code == 0 && !combined.contains("error:") {
   225	                    // Clean compile, no "No goals to be solved" marker, no
   226	                    // error. Edge case: treat as Complete (same rule as
   227	                    // verify_omega_detailed path 2).
   228	                    return PartialVerdict::Complete;
   229	                }
   230	                let err_preview: String = combined.lines()
   231	                    .filter(|l| l.contains("error") || l.contains("unexpected") || l.contains("expected"))
   232	                    .take(4)
   233	                    .collect::<Vec<_>>()
   234	                    .join(" | ");
   235	                let detail = if err_preview.is_empty() {
   236	                    combined.chars().take(400).collect::<String>()
   237	                } else {
   238	                    err_preview
   239	                };
   240	                PartialVerdict::Reject(detail)
   241	            }
   242	            Ok(SandboxResult::Timeout) => PartialVerdict::Reject("lean_timeout".into()),
   243	            Err(e) => PartialVerdict::Reject(format!("sandbox_error: {}", e)),
   244	        }
   245	    }
   246	}
   247	
   248	/// Phase 7 three-way verdict on a partial proof prefix.
   249	#[derive(Debug, Clone)]
   250	pub enum PartialVerdict {
   251	    /// All goals solved → OMEGA reached, halt the run.
   252	    Complete,
   253	    /// Tactics elaborate without type errors but goals remain → Q_{t+1} valid,
   254	    /// append this step as a tape node and continue.
   255	    PartialOk,

codex
**Findings**

No VETO or CHALLENGE findings. `d0d474e` resolves the round-3 blockers I can verify, and I do not see a new P0 introduced by the fix.

**Evidence**

- Round-3 missing-file absorption is closed: the runner preflights all adaptation files and aborts on any missing file before the loop, and the in-loop missing-file branch now exits `6` instead of emitting synthetic UNSOLVED. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:179) and [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:288).
- Round-3 boot preflight exit-code loss is closed: `PREFLIGHT_EXIT` is captured, exit `0` and `124` are explicit failures, `TRUST_ROOT_TAMPERED` aborts, and unexpected panics abort. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140).
- Gemini’s `EXIT=0` + empty `PPUT_RESULT` case is closed: it now exits `5` as malformed calibration data. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:321).

B1/B2/B3 remain resolved:

- B1: the production evaluator calls `verify_trust_root` before argument parsing, problem loading, LLM calls, or JSONL emission. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:162) and [boot.rs](/home/zephryj/projects/turingosv4/src/boot.rs:71). Note: actual code initializes `env_logger` first at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:164); I do not classify that as P0 because no calibration logic runs before Trust Root verification.
- B2: `compute_p0.py` rejects missing calibration tags, duplicates, seed mismatch, problem-set mismatch, wrong problem count, and wrong pair count; denominator is fixed at `144`. See [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:66), [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:90), and [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151).
- B3: full batch invokes `compute_p0.py`, preserves exit code, treats `2` as ceiling abort, and treats other nonzero exits as investigation-required. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:411) and [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:422). This matches PREREG §5.5 ceiling at [PREREG](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:462).

**RQ Notes**

- Timeout-as-UNSOLVED is defensible under the fixed wall-clock budget, and crash absorption is not reintroduced: only exit `124` emits a synthetic row; other nonzero exits abort. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:347) and [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:353).
- Synthetic rows include every required v2 `RunAggregate` field from [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:82); emitted fields are at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:231). `synthetic_short_circuit` being absent is not a conflict.
- `MODEL_SNAPSHOT` / `BUILD_SHA` are adequate for this batch: stamped in runner at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:68), read by evaluator at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1216).
- Manifest growth is bounded and documented: current 20 entries, final expected 21 after calibration JSONL freeze. See [TRACE_MATRIX](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:68).
- Code-simplifier changes are semantically safe: `Sha256::digest` one-shot is equivalent for these file reads, `String` formatting unwrap is sound, and `matches!(var.as_deref(), Ok("1"))` is equivalent to the prior `ok().as_deref() == Some("1")`.

**Verification Run**

I ran focused checks: Trust Root test suite, boot tests, rollback tests, release build, manifest hash verification, evaluator nonexistent-file preflight, strict-completeness negative case, and p₀ ceiling synthetic case. All behaved as expected; the ceiling case exited `2`.

**VERDICT**: PASS  
**Conviction**: high  
**Recommendation**: PROCEED with 576-run batch
tokens used
173,037
**Findings**

No VETO or CHALLENGE findings. `d0d474e` resolves the round-3 blockers I can verify, and I do not see a new P0 introduced by the fix.

**Evidence**

- Round-3 missing-file absorption is closed: the runner preflights all adaptation files and aborts on any missing file before the loop, and the in-loop missing-file branch now exits `6` instead of emitting synthetic UNSOLVED. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:179) and [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:288).
- Round-3 boot preflight exit-code loss is closed: `PREFLIGHT_EXIT` is captured, exit `0` and `124` are explicit failures, `TRUST_ROOT_TAMPERED` aborts, and unexpected panics abort. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:140).
- Gemini’s `EXIT=0` + empty `PPUT_RESULT` case is closed: it now exits `5` as malformed calibration data. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:321).

B1/B2/B3 remain resolved:

- B1: the production evaluator calls `verify_trust_root` before argument parsing, problem loading, LLM calls, or JSONL emission. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:162) and [boot.rs](/home/zephryj/projects/turingosv4/src/boot.rs:71). Note: actual code initializes `env_logger` first at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:164); I do not classify that as P0 because no calibration logic runs before Trust Root verification.
- B2: `compute_p0.py` rejects missing calibration tags, duplicates, seed mismatch, problem-set mismatch, wrong problem count, and wrong pair count; denominator is fixed at `144`. See [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:66), [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:90), and [compute_p0.py](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/compute_p0.py:151).
- B3: full batch invokes `compute_p0.py`, preserves exit code, treats `2` as ceiling abort, and treats other nonzero exits as investigation-required. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:411) and [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:422). This matches PREREG §5.5 ceiling at [PREREG](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:462).

**RQ Notes**

- Timeout-as-UNSOLVED is defensible under the fixed wall-clock budget, and crash absorption is not reintroduced: only exit `124` emits a synthetic row; other nonzero exits abort. See [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:347) and [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:353).
- Synthetic rows include every required v2 `RunAggregate` field from [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:82); emitted fields are at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:231). `synthetic_short_circuit` being absent is not a conflict.
- `MODEL_SNAPSHOT` / `BUILD_SHA` are adequate for this batch: stamped in runner at [run_p0_calibration.sh](/home/zephryj/projects/turingosv4/handover/preregistration/scripts/run_p0_calibration.sh:68), read by evaluator at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1216).
- Manifest growth is bounded and documented: current 20 entries, final expected 21 after calibration JSONL freeze. See [TRACE_MATRIX](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:68).
- Code-simplifier changes are semantically safe: `Sha256::digest` one-shot is equivalent for these file reads, `String` formatting unwrap is sound, and `matches!(var.as_deref(), Ok("1"))` is equivalent to the prior `ok().as_deref() == Some("1")`.

**Verification Run**

I ran focused checks: Trust Root test suite, boot tests, rollback tests, release build, manifest hash verification, evaluator nonexistent-file preflight, strict-completeness negative case, and p₀ ceiling synthetic case. All behaved as expected; the ceiling case exited `2`.

**VERDICT**: PASS  
**Conviction**: high  
**Recommendation**: PROCEED with 576-run batch
