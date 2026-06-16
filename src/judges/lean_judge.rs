//! TRACE_MATRIX FC1a-judge_pi: Lean-kernel JudgeAI — a pure, strict verifier for
//! the price-routed proof market (Hard Lean Market Go/No-Go).
//!
//! Unlike the heuristic `zeta_judge` the G1 market currently uses (a substring
//! matcher), `LeanJudge` settles OMEGA on the REAL Lean kernel. A candidate is a
//! proof BODY (tactic block) for a FIXED target theorem; the statement lives in
//! `preamble` (ending `... := by`), so an agent cannot weaken the goal.
//!
//! ## Empirically-pinned verdict contract (Lean v4.24.0, 2026-05-30, verified by
//! real runs — see prereg §3):
//!
//! `lean -DwarningAsError=true <file>`:
//!   * clean valid proof   -> exit 0  => Verified
//!   * `sorry` / `admit`   -> exit 1  (the "declaration uses 'sorry'" WARNING is
//!                                      promoted to an error) => rejected
//!   * wrong proof         -> exit 1  (type error / unsolved goals) => Failed
//!   * `native_decide`     -> exit 0  (NOT a warning; compiles to native code and
//!                                      BYPASSES the kernel) => MUST source-reject
//!
//! So a candidate is `Verified` IFF (a) it contains none of the kernel-trust-bypass
//! tokens [`sorry`, `admit`, `native_decide`, `unsafe`] (source scan, comments
//! stripped; mirrors constitution bus rule C-011), (b) `lean -DwarningAsError=true`
//! exits 0, and (c) an appended `#print axioms <theorem>` probe reports only
//! explicitly whitelisted axioms. This is STRICTER than `run_lean_checker`
//! (registry.rs:1220), which treats a bare exit 0 as pass and would therefore
//! accept a `sorry`-bearing proof — exactly the weak-judge inflation the
//! constitution (CLAUDE.md §4) and the prereg forbid.
//!
//! Substrate-agnostic: verifies whatever the `preamble` imports. Lean-core / Std
//! proofs verify offline today; Mathlib proofs verify once a Mathlib olean build +
//! `LEAN_PATH` exist (set via `extra_env` / `cwd`). Class 2 (additive verifier;
//! reuses the in-repo sanitized runner; no §6 surface).

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::judges::math_step_judge::{JudgeVerdict, MathStepJudge};
// De-Lean migration (2026-06-15, §8): the kernel verdict/error enums were renamed
// generic (LeanVerdictKind -> VerifierVerdictKind, LeanErrorClass -> VerifierErrorClass,
// variant SorryBlocked -> IncompleteProofBlocked, LeanFailed -> VerifierFailed). This
// math-domain judge keeps its own Lean-named local types but consumes the renamed kernel
// enums. Discriminant numbers + serde wire-names are pinned in attempt_telemetry.rs.
use crate::runtime::attempt_telemetry::{VerifierErrorClass, VerifierVerdictKind};
use crate::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand, SanitizedOutput,
};

/// TRACE_MATRIX FC1a-judge_pi: pinned Lean toolchain for the JudgeAI verifier.
/// Toolchain that the existing minif2f proofs pin to (elan layout name).
pub const PINNED_TOOLCHAIN: &str = "leanprover--lean4---v4.24.0";

/// TRACE_MATRIX FC1a-judge_pi: kernel-trust-bypass tokens the JudgeAI rejects.
/// Tokens that close a goal without a real kernel proof or bypass kernel trust.
/// `sorry`/`admit` also surface as warnings (caught by `-DwarningAsError`), but we
/// reject them at the source so the verdict is `SorryBlocked` (not `Failed`), and so
/// that `native_decide` — which is NOT a warning and would otherwise exit 0 — is also
/// blocked. Mirrors constitution bus rule C-011 (forbidden scratch-work tactics).
pub const KERNEL_BYPASS_TOKENS: &[&str] = &["sorry", "admit", "native_decide", "unsafe"];

/// TRACE_MATRIX FC1-N12: explicit default axiom whitelist for standalone LeanJudge.
///
/// Aligned with `AXIOM_WHITELIST` (the documented banked classical base):
/// `{propext, Classical.choice, Quot.sound}`. Het det-family proofs use
/// `Classical.choice` (a BANKED axiom that Lean's standard classical lemmas
/// depend on); excluding it from the default caused those proofs to be rejected
/// by `verify_axioms_after_success` even when the Lean kernel accepted them.
///
/// Callers that need a STRICTER (constructive-only) gate must explicitly
/// override `allowed_axioms` after construction — the default is NOT narrowed
/// here, only aligned to the banked base. Non-banked axioms (`sorryAx`,
/// `Lean.ofReduceBool` / `Lean.trustCompiler`, any hand-declared axiom) remain
/// fail-closed because they are absent from both this constant and `AXIOM_WHITELIST`.
pub const DEFAULT_ALLOWED_AXIOMS: &[&str] = &["propext", "Classical.choice", "Quot.sound"];

/// Max bytes of (shielded) Lean error text fed back into a retry prompt. The error
/// is the public compiler diagnostic on the agent's OWN candidate (legitimate retry
/// signal, like the swebench judge's failing-test names), bounded and never a raw
/// full-stderr dump (CLAUDE.md §4 raw-Lean-stderr shielding).
const FEEDBACK_MAX: usize = 240;

/// TRACE_MATRIX FC1a-judge_pi: the classical trust base a Verified proof may depend on.
/// A proof that merely COMPILES (`lean -DwarningAsError=true` exit 0) can still smuggle a
/// kernel-trust bypass that is NOT an `error:` and so slips past the exit-0 check:
///   * `sorryAx`                              — from `sorry` / `admit`,
///   * `Lean.ofReduceBool` / `Lean.trustCompiler` — from `native_decide` (native-compiled),
///   * any hand-declared `axiom`.
/// The only honest soundness certificate is Lean's own `#print axioms <name>`: the transitive
/// axiom set must be ⊆ this classical base. Mirrors the proven whitelists in
/// `src/bin/lean_emergence.rs` (AXIOM_WHITELIST) and `src/bin/lean_hayek_market.rs`
/// (AXIOM_ALLOWLIST). Empirically pinned on Lean v4.24.0: clean proof = `does not depend on
/// any axioms` (empty), `simp` = `[propext]`, `native_decide` = `[Lean.ofReduceBool,
/// Lean.trustCompiler]` (∉ whitelist → rejected).
pub const AXIOM_WHITELIST: &[&str] = &["propext", "Classical.choice", "Quot.sound"];

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// TRACE_MATRIX FC1-N12: A08 Lean axiom-probe status for standalone JudgeAI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxiomCheckStatus {
    PassedWhitelisted,
    RejectedNonWhitelisted,
    AxiomProbeFailed,
    SourceForbiddenPattern,
    LeanFailed,
}

/// TRACE_MATRIX FC1-N12: parsed `#print axioms` report, never raw Lean authority by itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AxiomReport {
    pub status: AxiomCheckStatus,
    pub rejected_axioms: Vec<String>,
}

/// TRACE_MATRIX FC1a-judge_pi: typed JudgeAI verdict for one candidate proof.
/// Strict Lean outcome for one candidate proof against the fixed target theorem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeanOutcome {
    pub verdict_kind: VerifierVerdictKind,
    pub error_class: Option<VerifierErrorClass>,
    pub exit_code: i32,
    pub timed_out: bool,
    pub axiom_check_status: AxiomCheckStatus,
    pub rejected_axioms: Vec<String>,
    /// Bounded, shielded failure summary for the retry prompt (empty on Verified).
    pub feedback: String,
    /// True ONLY when the kernel compiled (exit 0) but `#print axioms` exposed a
    /// non-whitelist axiom (sorryAx / native_decide trust / a hand-declared axiom). Such an
    /// outcome is NOT Verified — `verdict_kind` is `Failed` (so the CAS sidecar stays
    /// assert_45-consistent), but this flag lets a reader see it was a soundness reject, not
    /// a compile failure.
    pub axiom_rejected: bool,
    /// The parsed transitive axiom set (`#print axioms <name>`) on a Verified or
    /// axiom-rejected outcome; empty Vec otherwise (and on the clean "does not depend on any
    /// axioms" case). The audit-grade soundness footprint the bin persists into the manifest.
    pub axioms: Vec<String>,
}

impl LeanOutcome {
    /// TRACE_MATRIX FC1a-judge_pi: true iff the JudgeAI verdict is a clean OMEGA.
    pub fn is_verified(&self) -> bool {
        matches!(self.verdict_kind, VerifierVerdictKind::Verified)
    }
}

/// TRACE_MATRIX FC1a-judge_pi: pure Lean-kernel JudgeAI verifier (one target theorem).
/// A pure Lean verifier bound to ONE fixed target theorem.
#[derive(Debug, Clone)]
pub struct LeanJudge {
    /// `imports + set_option + open + "theorem <name> <args> : <goal> := by"`.
    /// The candidate proof body is appended after it.
    pub preamble: String,
    /// Lean binary. Pin to a concrete toolchain bin to avoid elan auto-download
    /// (a bare `lean` shim tries to fetch the latest toolchain — fatal offline).
    pub lean_bin: PathBuf,
    /// cwd for the lean process (repo root for core; the lake project dir for Mathlib).
    pub cwd: PathBuf,
    /// Extra env beyond the PATH+HOME allowlist (e.g. `("LEAN_PATH", "<oleans>")`).
    pub extra_env: Vec<(String, String)>,
    /// Axioms allowed by the standalone A08 `#print axioms` gate.
    pub allowed_axioms: BTreeSet<String>,
    /// Per-verify wall-clock timeout.
    pub timeout: Duration,
}

impl LeanJudge {
    /// TRACE_MATRIX FC1a-judge_pi: construct the JudgeAI verifier with defaults.
    /// Construct with sane defaults: the pinned toolchain bin, repo-root cwd, 60s.
    pub fn new(preamble: impl Into<String>) -> Self {
        Self {
            preamble: preamble.into(),
            lean_bin: default_lean_bin(),
            cwd: std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir()),
            extra_env: Vec::new(),
            allowed_axioms: DEFAULT_ALLOWED_AXIOMS
                .iter()
                .map(|axiom| (*axiom).to_string())
                .collect(),
            timeout: Duration::from_secs(60),
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: assemble a candidate proof body into a checkable file.
    /// Assemble the full `.lean` source for a candidate proof body.
    ///
    /// The body is `dedent`-ed (NOT flat-trimmed): the body's first tactic anchors the
    /// Lean `by` tactic block at column 0, so a uniformly-indented body (common when one
    /// is sliced out of a fuller `theorem … := by` block) must have its SHARED leading
    /// indent stripped from EVERY line — a flat `.trim()` strips only line 1's indent,
    /// leaving later siblings deeper than the anchor and thus OUTSIDE the block, which
    /// mislabels a correct proof `Failed`. See `dedent`.
    pub fn assemble(&self, candidate_body: &str) -> String {
        let body = dedent(candidate_body);
        let mut s = String::with_capacity(self.preamble.len() + body.len() + 2);
        s.push_str(&self.preamble);
        if !self.preamble.ends_with('\n') && !self.preamble.ends_with(' ') {
            s.push('\n');
        }
        s.push_str(&body);
        s.push('\n');
        s
    }

    /// TRACE_MATRIX FC1a-judge_pi: the JudgeAI verdict — verify a candidate proof body.
    /// Verify a candidate proof body and return the strict Lean outcome.
    pub fn verify(&self, candidate_body: &str) -> LeanOutcome {
        // 1. Source-scan the CANDIDATE (the preamble is trusted/fixed). Strip
        //    comments first so a `sorry` mentioned in a comment is not a false reject.
        if let Some(tok) = first_bypass_token(candidate_body) {
            return LeanOutcome {
                verdict_kind: VerifierVerdictKind::IncompleteProofBlocked,
                error_class: Some(VerifierErrorClass::IncompleteProofBlocked),
                exit_code: 0,
                timed_out: false,
                axiom_check_status: AxiomCheckStatus::SourceForbiddenPattern,
                rejected_axioms: Vec::new(),
                feedback: format!("kernel-bypass token `{tok}` is forbidden"),
                axiom_rejected: false,
                axioms: Vec::new(),
            };
        }

        // 2. Assemble + verify the candidate proof with Lean.
        let src = self.assemble(candidate_body);
        let theorem_name = match theorem_name_from_preamble(&self.preamble) {
            Some(name) => name,
            None => {
                return failed_with_axioms(
                    -1,
                    false,
                    AxiomCheckStatus::AxiomProbeFailed,
                    Vec::new(),
                    "could not identify theorem name for #print axioms".into(),
                )
            }
        };
        let out = self.run_lean_source(&src, "candidate");

        match out {
            Ok(o) if o.success() => self.verify_axioms_after_success(&src, &theorem_name),
            Ok(o) => {
                let timed_out = o.timed_out;
                let feedback = if timed_out {
                    "lean timed out".to_string()
                } else {
                    shield_lean_diagnostic(&o.stderr, &o.stdout)
                };
                failed(o.exit_code.unwrap_or(-1), timed_out, feedback)
            }
            Err(e) => failed(-1, false, format!("lean spawn failed: {e}")),
        }
    }

    fn run_lean_source(&self, source: &str, label: &str) -> std::io::Result<SanitizedOutput> {
        let n = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "turingos-leanjudge-{label}-{}-{}.lean",
            std::process::id(),
            n
        ));
        std::fs::write(&path, source.as_bytes())?;

        let mut env = env_allowlist_from_current(&["PATH", "HOME"]);
        for (k, v) in &self.extra_env {
            env.insert(k.clone(), v.clone());
        }
        let out = run_sanitized(SanitizedCommand {
            program: self.lean_bin.clone(),
            args: vec![
                "-DwarningAsError=true".into(),
                path.to_string_lossy().into_owned(),
            ],
            cwd: self.cwd.clone(),
            env,
            stdin: None,
            timeout: self.timeout,
        });
        let _ = std::fs::remove_file(&path);
        out
    }

    fn verify_axioms_after_success(&self, source: &str, theorem_name: &str) -> LeanOutcome {
        let probe_source = format!("{source}\n#print axioms {theorem_name}\n");
        match self.run_lean_source(&probe_source, "axioms") {
            Ok(o) if o.success() => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                let stderr = String::from_utf8_lossy(&o.stderr);
                let report = classify_axiom_report(&stdout, &stderr, &self.allowed_axioms);
                match report.status {
                    AxiomCheckStatus::PassedWhitelisted => LeanOutcome {
                        verdict_kind: VerifierVerdictKind::Verified,
                        error_class: None,
                        exit_code: 0,
                        timed_out: false,
                        axiom_check_status: report.status,
                        rejected_axioms: report.rejected_axioms,
                        feedback: String::new(),
                        axiom_rejected: false,
                        axioms: Vec::new(),
                    },
                    AxiomCheckStatus::RejectedNonWhitelisted => failed_with_axioms(
                        0,
                        false,
                        report.status,
                        report.rejected_axioms.clone(),
                        format!(
                            "non-whitelisted axioms: {}",
                            report.rejected_axioms.join(", ")
                        ),
                    ),
                    AxiomCheckStatus::AxiomProbeFailed => failed_with_axioms(
                        0,
                        false,
                        report.status,
                        report.rejected_axioms,
                        "axiom probe failed".into(),
                    ),
                    AxiomCheckStatus::SourceForbiddenPattern | AxiomCheckStatus::LeanFailed => {
                        failed_with_axioms(
                            0,
                            false,
                            AxiomCheckStatus::AxiomProbeFailed,
                            Vec::new(),
                            "unexpected axiom probe status".into(),
                        )
                    }
                }
            }
            Ok(o) => {
                let feedback = if o.timed_out {
                    "axiom probe timed out".to_string()
                } else {
                    shield_lean_diagnostic(&o.stderr, &o.stdout)
                };
                failed_with_axioms(
                    o.exit_code.unwrap_or(-1),
                    o.timed_out,
                    AxiomCheckStatus::AxiomProbeFailed,
                    Vec::new(),
                    feedback,
                )
            }
            Err(e) => failed_with_axioms(
                -1,
                false,
                AxiomCheckStatus::AxiomProbeFailed,
                Vec::new(),
                format!("axiom probe spawn failed: {e}"),
            ),
        }
    }

    /// `#print axioms` whitelist gate — fires ONLY after a clean exit-0 compile (caller
    /// guarantees `o.success()`). A second `lean` invocation on `<assembled> +
    /// "#print axioms <name>"` exposes the transitive axiom set; the candidate is Verified
    /// IFF that set ⊆ `AXIOM_WHITELIST`. FAIL-CLOSED everywhere a soundness fact is missing
    /// (no theorem name, re-run does not compile, no axiom line) → `axiom_rejected`, never
    /// Verified. Same sanitized command shape / cwd / env (incl. LEAN_PATH) as the first run.
    fn axiom_gate(&self, candidate_body: &str) -> LeanOutcome {
        // (1) Locate the theorem name (needed by `#print axioms <name>`). Fail-closed.
        let name = match extract_theorem_name(&self.preamble) {
            Some(n) => n,
            None => {
                return axiom_rejected(
                    "could not locate theorem name in preamble for #print axioms".into(),
                    Vec::new(),
                )
            }
        };

        // (2) Second source = the SAME assembled proof + the print-axioms query.
        let src = format!("{}\n#print axioms {name}\n", self.assemble(candidate_body));
        let n = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "turingos-leanjudge-axck-{}-{}.lean",
            std::process::id(),
            n
        ));
        if std::fs::write(&path, src.as_bytes()).is_err() {
            return axiom_rejected(
                "could not write temp lean file for #print axioms".into(),
                Vec::new(),
            );
        }

        // (3) Run with the SAME sanitized command (program, args modulo file, cwd, env).
        let mut env = env_allowlist_from_current(&["PATH", "HOME"]);
        for (k, v) in &self.extra_env {
            env.insert(k.clone(), v.clone());
        }
        let out = run_sanitized(SanitizedCommand {
            program: self.lean_bin.clone(),
            args: vec![
                "-DwarningAsError=true".into(),
                path.to_string_lossy().into_owned(),
            ],
            cwd: self.cwd.clone(),
            env,
            stdin: None,
            timeout: self.timeout,
        });
        let _ = std::fs::remove_file(&path);

        // (4) The print-axioms re-run must itself compile exit-0 and emit an axiom line.
        let o = match out {
            Ok(o) if o.success() => o,
            Ok(o) => {
                let fb = if o.timed_out {
                    "lean timed out on #print axioms".to_string()
                } else {
                    shield_lean_diagnostic(&o.stderr, &o.stdout)
                };
                return axiom_rejected(fb, Vec::new());
            }
            Err(e) => {
                return axiom_rejected(
                    format!("lean spawn failed on #print axioms: {e}"),
                    Vec::new(),
                )
            }
        };
        let stdout = String::from_utf8_lossy(&o.stdout);
        let parsed = match parse_axiom_set(&stdout) {
            Some(set) => set,
            None => {
                return axiom_rejected("no `#print axioms` line in lean output".into(), Vec::new())
            }
        };

        // (5) Subset check against the classical trust base.
        let bad: Vec<String> = parsed
            .iter()
            .filter(|a| !AXIOM_WHITELIST.contains(&a.as_str()))
            .cloned()
            .collect();
        let axioms: Vec<String> = parsed.into_iter().collect();
        if bad.is_empty() {
            LeanOutcome {
                verdict_kind: VerifierVerdictKind::Verified,
                error_class: None,
                exit_code: 0,
                timed_out: false,
                axiom_check_status: AxiomCheckStatus::PassedWhitelisted,
                rejected_axioms: Vec::new(),
                feedback: String::new(),
                axiom_rejected: false,
                axioms,
            }
        } else {
            axiom_rejected(
                format!(
                    "non-whitelist axiom(s): {bad:?} (kernel-bypass: sorryAx/native_decide trust)"
                ),
                axioms,
            )
        }
    }
}

/// TRACE_MATRIX FC1-N12: classify a `#print axioms` output against an explicit whitelist.
pub fn classify_axiom_report(
    stdout: &str,
    stderr: &str,
    allowed_axioms: &BTreeSet<String>,
) -> AxiomReport {
    let text = format!("{stdout}\n{stderr}");
    if text.trim().is_empty() {
        return AxiomReport {
            status: AxiomCheckStatus::AxiomProbeFailed,
            rejected_axioms: Vec::new(),
        };
    }
    if text.contains("does not depend on any axioms") {
        return AxiomReport {
            status: AxiomCheckStatus::PassedWhitelisted,
            rejected_axioms: Vec::new(),
        };
    }
    let Some(axioms) = parse_axiom_list(&text) else {
        return AxiomReport {
            status: AxiomCheckStatus::AxiomProbeFailed,
            rejected_axioms: Vec::new(),
        };
    };
    let rejected_axioms: Vec<String> = axioms
        .into_iter()
        .filter(|axiom| !allowed_axioms.contains(axiom))
        .collect();
    let status = if rejected_axioms.is_empty() {
        AxiomCheckStatus::PassedWhitelisted
    } else {
        AxiomCheckStatus::RejectedNonWhitelisted
    };
    AxiomReport {
        status,
        rejected_axioms,
    }
}

fn parse_axiom_list(text: &str) -> Option<Vec<String>> {
    let start = text.find('[')?;
    let end = text[start + 1..].find(']')? + start + 1;
    let body = &text[start + 1..end];
    let axioms: Vec<String> = body
        .split(',')
        .map(|part| part.trim().trim_matches('`').trim_matches('"'))
        .filter(|part| !part.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    Some(axioms)
}

fn theorem_name_from_preamble(preamble: &str) -> Option<String> {
    let mut tokens = preamble.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "theorem" || token == "lemma" {
            return tokens
                .next()
                .map(|name| name.trim_matches(|c: char| c == ':' || c == '{' || c == '('))
                .filter(|name| !name.is_empty())
                .map(ToOwned::to_owned);
        }
    }
    None
}

/// `MathStepJudge` impl — the verifier-agnostic product seam. `candidate_step` is a
/// full proof BODY for the fixed theorem; `Pass` IFF kernel-`Verified`. `prior_steps`
/// is unused in the whole-proof model.
impl MathStepJudge for LeanJudge {
    fn verdict(&self, _prior_steps: &[String], candidate_step: &str) -> JudgeVerdict {
        let o = self.verify(candidate_step);
        if o.is_verified() {
            JudgeVerdict::Pass
        } else {
            JudgeVerdict::Fail { reason: o.feedback }
        }
    }
}

fn failed(exit_code: i32, timed_out: bool, feedback: String) -> LeanOutcome {
    failed_with_axioms(
        exit_code,
        timed_out,
        AxiomCheckStatus::LeanFailed,
        Vec::new(),
        feedback,
    )
}

fn failed_with_axioms(
    exit_code: i32,
    timed_out: bool,
    axiom_check_status: AxiomCheckStatus,
    rejected_axioms: Vec<String>,
    feedback: String,
) -> LeanOutcome {
    LeanOutcome {
        verdict_kind: VerifierVerdictKind::Failed,
        error_class: Some(VerifierErrorClass::VerifierFailed),
        exit_code,
        timed_out,
        axiom_check_status,
        rejected_axioms,
        feedback,
        axiom_rejected: false,
        axioms: Vec::new(),
    }
}

/// Soundness reject: the candidate COMPILED (exit 0) but its `#print axioms` set is not a
/// subset of `AXIOM_WHITELIST` (or the name/axiom line could not be obtained). Modeled as the
/// canonical `Failed` arm (exit_code=1, error_class=LeanFailed, !verified) so the CAS
/// `LeanResult` sidecar stays assert_45-consistent — `VerifierVerdictKind` is NOT extended (that
/// enum is an out-of-scope, repr-stable, CAS-hash-bearing surface). `axiom_rejected=true`
/// distinguishes it from a plain compile failure; `axioms` carries the offending set.
fn axiom_rejected(feedback: String, axioms: Vec<String>) -> LeanOutcome {
    let mut s: String = feedback.chars().take(FEEDBACK_MAX).collect();
    if feedback.chars().count() > FEEDBACK_MAX {
        s.push('…');
    }
    LeanOutcome {
        verdict_kind: VerifierVerdictKind::Failed,
        error_class: Some(VerifierErrorClass::VerifierFailed),
        exit_code: 1,
        timed_out: false,
        axiom_check_status: AxiomCheckStatus::RejectedNonWhitelisted,
        rejected_axioms: axioms.clone(),
        feedback: s,
        axiom_rejected: true,
        axioms,
    }
}

/// TRACE_MATRIX FC1a-judge_pi: resolve the pinned Lean toolchain bin for the verifier.
/// Resolve the pinned Lean toolchain binary; fall back to a bare `lean` (PATH) when
/// the pinned toolchain is absent (e.g. CI without v4.24.0 — callers gate on
/// `lean_bin.exists()` for real-run tests).
pub fn default_lean_bin() -> PathBuf {
    if let Some(home) = std::env::var_os("HOME") {
        let pinned = PathBuf::from(&home)
            .join(".elan")
            .join("toolchains")
            .join(PINNED_TOOLCHAIN)
            .join("bin")
            .join("lean");
        if pinned.exists() {
            return pinned;
        }
    }
    PathBuf::from("lean")
}

/// Dedent a candidate proof body so its shallowest line sits at column 0 while the
/// RELATIVE indentation of deeper lines (genuine nesting: `·` foci, `have … := by`
/// sub-blocks, `case` arms) is preserved byte-for-byte.
///
/// Why this exists: `assemble` appends the body right after the preamble's `:= by`, so
/// the body's first tactic lands at column 0 and that column ANCHORS the Lean `by`
/// tactic block. A naive `.trim()` on a uniformly-indented body (e.g. the 2-space block
/// `"  simp [Matrix.det_fin_three]\n  ring"` sliced from a fuller `theorem … := by`
/// block) strips ONLY the first line's indent, leaving later siblings deeper than the
/// anchor — Lean then parses them OUTSIDE the block (`unsolved goals` + `unexpected
/// identifier; expected command`) and a CORRECT proof is mislabeled `Failed`. Stripping
/// the longest common leading-whitespace prefix re-aligns every sibling to the same
/// column without disturbing genuine nesting. Empirically pinned (Lean v4.24.0 +
/// mathlib4): the de-aligned body fails; the dedented body verifies.
///
/// Conservative by construction: it strips only whitespace SHARED by all non-blank
/// lines, so it can never flatten real nesting, and for a single-line or already-col-0
/// body it is equivalent to a trim. A body whose FIRST line is already shallower than a
/// later line (e.g. `"simp\n  ring"` — a body a prior `.trim()` already de-aligned) is
/// NOT recoverable here; that is why this normalization must run at the FIRST point a
/// body is captured, before any lossy trim, not only at the end of the pipeline.
pub fn dedent(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    // Longest common leading-whitespace prefix over the non-blank lines.
    let mut common: Option<&str> = None;
    for line in &lines {
        if line.trim().is_empty() {
            continue; // blank lines do not constrain the shared indent
        }
        let lead = &line[..line.len() - line.trim_start().len()];
        common = Some(match common {
            None => lead,
            Some(prev) => {
                let n = prev
                    .bytes()
                    .zip(lead.bytes())
                    .take_while(|(a, b)| a == b)
                    .count();
                &prev[..n]
            }
        });
    }
    let cut = common.map_or(0, str::len);
    // Strip the shared prefix from each line; blank lines collapse to empty.
    let stripped: Vec<&str> = lines
        .iter()
        .map(|line| {
            if line.trim().is_empty() {
                ""
            } else {
                line[cut..].trim_end()
            }
        })
        .collect();
    // Drop leading / trailing blank lines, keep the interior verbatim.
    let start = stripped.iter().position(|l| !l.is_empty()).unwrap_or(0);
    let end = stripped
        .iter()
        .rposition(|l| !l.is_empty())
        .map_or(0, |i| i + 1);
    stripped[start..end].join("\n")
}

/// Re-align an extracted proof body for `:= by\n<body>` assembly. A FLAT tactic
/// sequence (no nested block openers) has every sibling flushed to column 0, curing the
/// "first line shallower than a sibling" de-alignment that the conservative [`dedent`]
/// cannot recover — e.g. a model `proof_body` `"simp\n  ring"` (common prefix `""`) or an
/// inline `:= by tac\n  tac` slice (common prefix `" "`). A body that CONTAINS genuine
/// nesting (a line that opens a child block) is handed to [`dedent`], preserving relative
/// nesting byte-for-byte.
///
/// SOUND: flattening a flat sequence cannot restructure tactics (no nesting to destroy),
/// and Lean still verifies the real goal — `realign` can only cure a false NEGATIVE, never
/// manufacture a false positive against the theorem statement. Apply at the point a
/// model's proof body is extracted (the het probe and `lean_market_agent`); keep
/// [`dedent`] conservative for the assemble-time path.
pub fn realign(body: &str) -> String {
    let expanded = body.replace('\t', "  ");
    if opens_nested_block(&expanded) {
        return dedent(&expanded);
    }
    let lines: Vec<&str> = expanded.lines().map(str::trim).collect();
    let start = lines.iter().position(|l| !l.is_empty()).unwrap_or(0);
    let end = lines
        .iter()
        .rposition(|l| !l.is_empty())
        .map_or(0, |i| i + 1);
    if start >= end {
        return String::new();
    }
    lines[start..end].join("\n")
}

/// True iff any non-blank line opens a nested tactic/term block, so the body has genuine
/// relative nesting that [`realign`] must preserve (defer to conservative [`dedent`])
/// rather than flush. Conservative: a missed opener only risks a false NEGATIVE (same
/// class as the bug being fixed), never a false positive — Lean is the final arbiter.
fn opens_nested_block(body: &str) -> bool {
    body.lines().any(|raw| {
        let l = raw.trim();
        if l.is_empty() {
            return false;
        }
        let ends_open = l == "by"
            || l.ends_with(" by")
            || l.ends_with("=>")
            || l == "do"
            || l.ends_with(" do")
            || l == "with"
            || l.ends_with(" with")
            || l.ends_with(" from")
            || l.ends_with(":=");
        let starts_block = l.starts_with('·')
            || l.starts_with('•')
            || l == "|"
            || l.starts_with("| ")
            || l.starts_with("case ")
            || l.starts_with("next ")
            || l.starts_with("calc")
            || l.starts_with('{');
        ends_open || starts_block
    })
}

/// Strip Lean line (`-- ...`) and block (`/- ... -/`) comments, then return the
/// first kernel-bypass token that appears as a whole word in code.
fn first_bypass_token(candidate: &str) -> Option<&'static str> {
    let code = strip_lean_comments(candidate);
    KERNEL_BYPASS_TOKENS
        .iter()
        .copied()
        .find(|tok| contains_word(&code, tok))
}

/// Remove `--` line comments and `/- ... -/` (non-nested) block comments.
fn strip_lean_comments(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'-' && i + 1 < bytes.len() && bytes[i + 1] == b'-' {
            // line comment to end of line
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
        } else if bytes[i] == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'-' {
            // block comment to `-/`
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'-' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i += 2;
            out.push(' ');
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

/// True iff `needle` occurs in `hay` bounded by non-identifier chars (Lean
/// identifiers are alphanumeric + `_` + `'` + `.`). Avoids matching `sorry` inside a
/// larger identifier.
fn contains_word(hay: &str, needle: &str) -> bool {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_' || c == '\'' || c == '.';
    let mut start = 0;
    while let Some(rel) = hay[start..].find(needle) {
        let at = start + rel;
        let before_ok = at == 0 || !hay[..at].chars().next_back().map(is_ident).unwrap_or(false);
        let after = at + needle.len();
        let after_ok =
            after >= hay.len() || !hay[after..].chars().next().map(is_ident).unwrap_or(false);
        if before_ok && after_ok {
            return true;
        }
        start = at + needle.len();
    }
    false
}

/// Extract the theorem/lemma name from the preamble so `#print axioms <name>` can target it.
/// Verbatim from `src/bin/lean_emergence.rs` (`extract_theorem_name`): scan for `theorem `
/// then `lemma `, take chars until whitespace or one of `( { [ :`. `None` for a nameless
/// `example` (the axiom gate fail-closes such a preamble).
fn extract_theorem_name(preamble: &str) -> Option<String> {
    for kw in ["theorem ", "lemma "] {
        if let Some(i) = preamble.find(kw) {
            let after = &preamble[i + kw.len()..];
            let name: String = after
                .chars()
                .take_while(|c| !c.is_whitespace() && !matches!(c, '(' | '{' | '[' | ':'))
                .collect();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

/// Parse the dependency set printed by `#print axioms <name>` out of Lean's raw output.
/// Verbatim semantics from `src/bin/lean_hayek_market.rs` (`parse_axiom_set`). Lean emits
/// exactly one of:
///   `'<name>' depends on axioms: [propext, Classical.choice, Quot.sound]`
///   `'<name>' does not depend on any axioms`
/// Returns the axiom names (empty set for the "no axioms" case), or `None` if no such line is
/// present (co-occurs with a hard compile error). Case-sensitive (`sorryAx`, `Quot.sound`).
fn parse_axiom_set(raw: &str) -> Option<BTreeSet<String>> {
    if raw.contains("does not depend on any axioms") {
        return Some(BTreeSet::new());
    }
    let after = &raw[raw.find("depends on axioms:")? + "depends on axioms:".len()..];
    let lb = after.find('[')?;
    let rb = after[lb..].find(']')? + lb;
    Some(
        after[lb + 1..rb]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
    )
}

/// Bounded, shielded diagnostic: the first `error:` line (or first non-empty line)
/// from Lean, truncated. Never the full stderr dump.
fn shield_lean_diagnostic(stderr: &[u8], stdout: &[u8]) -> String {
    let text = if stderr.is_empty() {
        String::from_utf8_lossy(stdout)
    } else {
        String::from_utf8_lossy(stderr)
    };
    let line = text
        .lines()
        .find(|l| l.contains("error:"))
        .or_else(|| text.lines().find(|l| !l.trim().is_empty()))
        .unwrap_or("lean failed")
        .trim();
    let mut s: String = line.chars().take(FEEDBACK_MAX).collect();
    if line.chars().count() > FEEDBACK_MAX {
        s.push('…');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Pure-logic tests (always run; no toolchain) ──────────────────

    #[test]
    fn assemble_appends_body_after_preamble() {
        let j = LeanJudge::new("theorem t : 1 = 1 := by");
        let src = j.assemble("  rfl  ");
        assert_eq!(src, "theorem t : 1 = 1 := by\nrfl\n");
    }

    #[test]
    fn dedent_realigns_uniformly_indented_block() {
        // The bug shape: a 2-space block sliced from a fuller `:= by` body. A flat trim
        // would leave `simp [...]\n  ring` (line 2 deeper than the col-0 anchor → outside
        // the by-block). Dedent strips the SHARED 2-space prefix → both tactics at col 0.
        assert_eq!(
            dedent("  simp [Matrix.det_fin_three]\n  ring"),
            "simp [Matrix.det_fin_three]\nring"
        );
    }

    #[test]
    fn dedent_preserves_relative_nesting() {
        // Genuine nesting (inner `have … := by` step 2 deeper) must survive: strip only
        // the shared outer prefix, keep the inner step relatively indented.
        assert_eq!(
            dedent("  have h : True := by\n    trivial\n  exact h"),
            "have h : True := by\n  trivial\nexact h"
        );
    }

    #[test]
    fn dedent_is_trim_for_single_line_and_col0() {
        assert_eq!(dedent("  rfl  "), "rfl");
        // already col-0 → unchanged (no JSON-body regression)
        assert_eq!(dedent("simp\nnorm_num"), "simp\nnorm_num");
        // leading / trailing blank lines dropped, interior kept
        assert_eq!(dedent("\n\n  exact h\n\n"), "exact h");
    }

    #[test]
    fn dedent_does_not_recover_already_dealigned_body() {
        // Documents the boundary: once line 1 is shallower than a sibling (a prior trim
        // already destroyed the shared prefix), dedent cannot re-align — which is WHY the
        // normalization must run before the first lossy trim, not only in `assemble`.
        assert_eq!(dedent("simp\n  ring"), "simp\n  ring");
    }

    #[test]
    fn bypass_tokens_detected_in_code() {
        assert_eq!(first_bypass_token("exact sorry"), Some("sorry"));
        assert_eq!(first_bypass_token("by admit"), Some("admit"));
        assert_eq!(
            first_bypass_token("by native_decide"),
            Some("native_decide")
        );
    }

    #[test]
    fn bypass_token_in_comment_is_ignored() {
        // `sorry` only in a comment must NOT be flagged (the code is clean).
        assert_eq!(first_bypass_token("-- todo: not a sorry here\n  rfl"), None);
        assert_eq!(first_bypass_token("/- sorry in block -/ rfl"), None);
    }

    #[test]
    fn bypass_token_not_matched_as_substring() {
        // identifiers that merely CONTAIN the token are not bypasses
        assert_eq!(first_bypass_token("exact sorryLemma"), None);
        assert_eq!(first_bypass_token("exact my_admit_helper"), None);
    }

    #[test]
    fn contains_word_boundaries() {
        assert!(contains_word("by sorry", "sorry"));
        assert!(!contains_word("sorryX", "sorry"));
        assert!(!contains_word("Xsorry", "sorry"));
        assert!(contains_word("a; sorry; b", "sorry"));
    }

    #[test]
    fn source_scan_rejects_before_running_lean() {
        // Even with a bogus lean_bin, a sorry candidate is SorryBlocked at the
        // source-scan stage (lean is never invoked).
        let mut j = LeanJudge::new("theorem t : True := by");
        j.lean_bin = PathBuf::from("/nonexistent/lean");
        let o = j.verify("exact sorry");
        assert_eq!(o.verdict_kind, VerifierVerdictKind::IncompleteProofBlocked);
        assert_eq!(
            o.error_class,
            Some(VerifierErrorClass::IncompleteProofBlocked)
        );
    }

    // ── Real-run tests (gated on the pinned toolchain being present) ──

    fn toolchain_or_skip() -> Option<PathBuf> {
        let bin = default_lean_bin();
        if bin.is_absolute() && bin.exists() {
            Some(bin)
        } else {
            eprintln!("skip: pinned Lean toolchain {PINNED_TOOLCHAIN} not present");
            None
        }
    }

    #[test]
    fn real_lean_verifies_valid_core_proof() {
        let Some(bin) = toolchain_or_skip() else {
            return;
        };
        let mut j = LeanJudge::new("theorem t (n : Nat) : n + 0 = n := by");
        j.lean_bin = bin;
        let o = j.verify("simp");
        assert!(o.is_verified(), "expected Verified, got {o:?}");
    }

    #[test]
    fn real_lean_rejects_wrong_core_proof() {
        let Some(bin) = toolchain_or_skip() else {
            return;
        };
        let mut j = LeanJudge::new("theorem t : (2 : Nat) + 2 = 5 := by");
        j.lean_bin = bin;
        let o = j.verify("rfl");
        assert_eq!(o.verdict_kind, VerifierVerdictKind::Failed);
        assert!(!o.feedback.is_empty());
    }

    #[test]
    fn real_lean_verifies_indented_multitactic_body() {
        // Regression for the het_capability_probe de-alignment bug. A uniformly-indented
        // multi-tactic body (the shape sliced from a fuller `:= by` block) MUST verify,
        // not be mislabeled Failed. Pre-dedent, `assemble`'s flat trim left the 2nd/3rd
        // tactics deeper than the col-0 anchor → `unsolved goals` + `unexpected
        // identifier; expected command`. Mathlib-free (core `And`/`constructor`) so it
        // runs fast on the pinned toolchain alone.
        let Some(bin) = toolchain_or_skip() else {
            return;
        };
        let mut j = LeanJudge::new("theorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by");
        j.lean_bin = bin;
        // 2-space uniform indent on every tactic — the exact shape that used to fail.
        let o = j.verify("  constructor\n  exact hp\n  exact hq");
        assert!(
            o.is_verified(),
            "indented multi-tactic body must verify, got {o:?}"
        );
    }

    // ── F5 axiom-gate: pure parse/extract (always run; no toolchain) ──

    #[test]
    fn parse_axiom_set_shapes() {
        // clean proof → empty set
        assert_eq!(
            parse_axiom_set("'t' does not depend on any axioms"),
            Some(BTreeSet::new())
        );
        // whitelist axioms
        assert_eq!(
            parse_axiom_set("'t' depends on axioms: [propext, Classical.choice]"),
            Some(
                ["propext".to_string(), "Classical.choice".to_string()]
                    .into_iter()
                    .collect()
            )
        );
        // native_decide trust axioms (both present, ∉ whitelist)
        let nd = parse_axiom_set("'t' depends on axioms: [Lean.ofReduceBool, Lean.trustCompiler]")
            .expect("axiom line");
        assert!(nd.contains("Lean.ofReduceBool"));
        assert!(nd.contains("Lean.trustCompiler"));
        // no axiom line at all → None (fail-closed)
        assert_eq!(parse_axiom_set("random error text"), None);
    }

    #[test]
    fn extract_theorem_name_shapes() {
        assert_eq!(
            extract_theorem_name("theorem tos_add_zero (n : Nat) : n + 0 = n := by"),
            Some("tos_add_zero".to_string())
        );
        assert_eq!(
            extract_theorem_name(
                "import Mathlib\nopen Real\ntheorem tos_sq_add (a b : R) : a = a := by"
            ),
            Some("tos_sq_add".to_string())
        );
        // lemma keyword
        assert_eq!(
            extract_theorem_name("lemma helper : True := by"),
            Some("helper".to_string())
        );
        // nameless example → None (axiom gate fail-closes)
        assert_eq!(extract_theorem_name("example : True := by"), None);
    }

    // ── F5 axiom-gate: real Lean (gated on the pinned toolchain) ──

    #[test]
    fn axiom_gate_accepts_clean_proof() {
        let Some(bin) = toolchain_or_skip() else {
            return;
        };
        let mut j = LeanJudge::new("theorem t (n : Nat) : n + 0 = n := by");
        j.lean_bin = bin;
        let o = j.verify("simp");
        assert!(o.is_verified(), "expected Verified, got {o:?}");
        assert!(!o.axiom_rejected);
        // this proof's footprint is `[propext]` ⊆ whitelist
        assert!(
            o.axioms
                .iter()
                .all(|a| AXIOM_WHITELIST.contains(&a.as_str())),
            "axioms {:?} must all be whitelisted",
            o.axioms
        );
    }

    #[test]
    fn axiom_gate_rejects_hand_declared_axiom() {
        // native_decide is caught FIRST by the source token-scan (SorryBlocked), so it never
        // reaches the axiom gate. To exercise the gate's REJECT path on a NON-source-detectable
        // leak, hand-declare an axiom: it compiles exit-0, passes the token scan (no
        // sorry/admit/native_decide), but `#print axioms t` prints `[evil]` ∉ whitelist.
        let Some(bin) = toolchain_or_skip() else {
            return;
        };
        let mut j =
            LeanJudge::new("axiom evil : (2 : Nat) + 2 = 5\ntheorem t : (2 : Nat) + 2 = 5 := by");
        j.lean_bin = bin;
        let o = j.verify("exact evil");
        assert!(
            !o.is_verified(),
            "axiom-dirty proof must NOT be Verified: {o:?}"
        );
        assert!(o.axiom_rejected, "expected axiom_rejected, got {o:?}");
        assert!(
            o.axioms.contains(&"evil".to_string()),
            "axioms {:?}",
            o.axioms
        );
        // CAS sidecar consistency: Failed arm shape (exit_code=1, !verified).
        assert_eq!(o.verdict_kind, VerifierVerdictKind::Failed);
        assert_eq!(o.exit_code, 1);
    }
}
