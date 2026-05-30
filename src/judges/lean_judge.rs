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
//! tokens [`sorry`, `admit`, `native_decide`] (source scan, comments stripped;
//! mirrors constitution bus rule C-011) AND (b) `lean -DwarningAsError=true` exits 0.
//! This is STRICTER than `run_lean_checker` (registry.rs:1220), which treats a bare
//! exit 0 as pass and would therefore accept a `sorry`-bearing proof — exactly the
//! weak-judge inflation the constitution (CLAUDE.md §4) and the prereg forbid.
//!
//! Substrate-agnostic: verifies whatever the `preamble` imports. Lean-core / Std
//! proofs verify offline today; Mathlib proofs verify once a Mathlib olean build +
//! `LEAN_PATH` exist (set via `extra_env` / `cwd`). Class 2 (additive verifier;
//! reuses the in-repo sanitized runner; no §6 surface).

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::judges::math_step_judge::{JudgeVerdict, MathStepJudge};
use crate::runtime::attempt_telemetry::{LeanErrorClass, LeanVerdictKind};
use crate::sdk::sanitized_runner::{env_allowlist_from_current, run_sanitized, SanitizedCommand};

/// Toolchain that the existing minif2f proofs pin to (elan layout name).
pub const PINNED_TOOLCHAIN: &str = "leanprover--lean4---v4.24.0";

/// Tokens that close a goal without a real kernel proof or bypass kernel trust.
/// `sorry`/`admit` also surface as warnings (caught by `-DwarningAsError`), but we
/// reject them at the source so the verdict is `SorryBlocked` (not `Failed`), and so
/// that `native_decide` — which is NOT a warning and would otherwise exit 0 — is also
/// blocked. Mirrors constitution bus rule C-011 (forbidden scratch-work tactics).
pub const KERNEL_BYPASS_TOKENS: &[&str] = &["sorry", "admit", "native_decide"];

/// Max bytes of (shielded) Lean error text fed back into a retry prompt. The error
/// is the public compiler diagnostic on the agent's OWN candidate (legitimate retry
/// signal, like the swebench judge's failing-test names), bounded and never a raw
/// full-stderr dump (CLAUDE.md §4 raw-Lean-stderr shielding).
const FEEDBACK_MAX: usize = 240;

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Strict Lean outcome for one candidate proof against the fixed target theorem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeanOutcome {
    pub verdict_kind: LeanVerdictKind,
    pub error_class: Option<LeanErrorClass>,
    pub exit_code: i32,
    pub timed_out: bool,
    /// Bounded, shielded failure summary for the retry prompt (empty on Verified).
    pub feedback: String,
}

impl LeanOutcome {
    pub fn is_verified(&self) -> bool {
        matches!(self.verdict_kind, LeanVerdictKind::Verified)
    }
}

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
    /// Per-verify wall-clock timeout.
    pub timeout: Duration,
}

impl LeanJudge {
    /// Construct with sane defaults: the pinned toolchain bin, repo-root cwd, 60s.
    pub fn new(preamble: impl Into<String>) -> Self {
        Self {
            preamble: preamble.into(),
            lean_bin: default_lean_bin(),
            cwd: std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir()),
            extra_env: Vec::new(),
            timeout: Duration::from_secs(60),
        }
    }

    /// Assemble the full `.lean` source for a candidate proof body.
    pub fn assemble(&self, candidate_body: &str) -> String {
        let mut s = String::with_capacity(self.preamble.len() + candidate_body.len() + 2);
        s.push_str(&self.preamble);
        if !self.preamble.ends_with('\n') && !self.preamble.ends_with(' ') {
            s.push('\n');
        }
        s.push_str(candidate_body.trim());
        s.push('\n');
        s
    }

    /// Verify a candidate proof body and return the strict Lean outcome.
    pub fn verify(&self, candidate_body: &str) -> LeanOutcome {
        // 1. Source-scan the CANDIDATE (the preamble is trusted/fixed). Strip
        //    comments first so a `sorry` mentioned in a comment is not a false reject.
        if let Some(tok) = first_bypass_token(candidate_body) {
            return LeanOutcome {
                verdict_kind: LeanVerdictKind::SorryBlocked,
                error_class: Some(LeanErrorClass::SorryBlocked),
                exit_code: 0,
                timed_out: false,
                feedback: format!("kernel-bypass token `{tok}` is forbidden"),
            };
        }

        // 2. Assemble + write a temp .lean file.
        let src = self.assemble(candidate_body);
        let n = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "turingos-leanjudge-{}-{}.lean",
            std::process::id(),
            n
        ));
        if std::fs::write(&path, src.as_bytes()).is_err() {
            return failed(-1, false, "could not write temp lean file".into());
        }

        // 3. Run `lean -DwarningAsError=true <file>` under the sanitized runner.
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

        match out {
            Ok(o) if o.success() => LeanOutcome {
                verdict_kind: LeanVerdictKind::Verified,
                error_class: None,
                exit_code: 0,
                timed_out: false,
                feedback: String::new(),
            },
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
    LeanOutcome {
        verdict_kind: LeanVerdictKind::Failed,
        error_class: Some(LeanErrorClass::LeanFailed),
        exit_code,
        timed_out,
        feedback,
    }
}

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
        let after_ok = after >= hay.len()
            || !hay[after..].chars().next().map(is_ident).unwrap_or(false);
        if before_ok && after_ok {
            return true;
        }
        start = at + needle.len();
    }
    false
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
    fn bypass_tokens_detected_in_code() {
        assert_eq!(first_bypass_token("exact sorry"), Some("sorry"));
        assert_eq!(first_bypass_token("by admit"), Some("admit"));
        assert_eq!(first_bypass_token("by native_decide"), Some("native_decide"));
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
        assert_eq!(o.verdict_kind, LeanVerdictKind::SorryBlocked);
        assert_eq!(o.error_class, Some(LeanErrorClass::SorryBlocked));
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
        let Some(bin) = toolchain_or_skip() else { return };
        let mut j = LeanJudge::new("theorem t (n : Nat) : n + 0 = n := by");
        j.lean_bin = bin;
        let o = j.verify("simp");
        assert!(o.is_verified(), "expected Verified, got {o:?}");
    }

    #[test]
    fn real_lean_rejects_wrong_core_proof() {
        let Some(bin) = toolchain_or_skip() else { return };
        let mut j = LeanJudge::new("theorem t : (2 : Nat) + 2 = 5 := by");
        j.lean_bin = bin;
        let o = j.verify("rfl");
        assert_eq!(o.verdict_kind, LeanVerdictKind::Failed);
        assert!(!o.feedback.is_empty());
    }
}
