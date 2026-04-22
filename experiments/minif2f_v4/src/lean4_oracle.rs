// Engine 3: Popperian Guillotine — Lean 4 Oracle
// Constitutional basis: Art. I.1 (boolean predicate — pass/fail only)
// V3L-01: sorry 3-layer defense (upstream filter + oracle reject + post-hoc)
// V3L-02: oracle determinism (same input → same output)
// V3L-07: identity theft prevention (reject new theorem declarations)

use std::path::PathBuf;
use std::time::Duration;
use turingosv4::sdk::sandbox::{LocalProcessSandbox, SandboxEngine, SandboxResult};
use turingosv4::sdk::tool::{ToolSignal, TuringTool};
use std::any::Any;

/// Forbidden patterns in agent-submitted code (substring match).
/// These are checked BEFORE sending to Lean 4.
const FORBIDDEN_PATTERNS: &[&str] = &[
    "#eval", "#check", "#reduce", "#exec", "#print",  // output/reflection
    "native_decide",                                     // bytecode bypass
    "IO.Process", "IO.FS", "System.FilePath",           // system escape
    "run_tac", "unsafe", "dbg_trace", "IO.println",    // meta/debug
];

/// Bare-tactic words forbidden as agent scratch (C-011 complete / Phase 8.D /
/// C-050). Unlike FORBIDDEN_PATTERNS (substring), these match *bare* word
/// usage only — qualified Mathlib references (e.g. `Decidable.decide`,
/// `Nat.decide_lt`, `Mathlib.Tactic.Omega.…`) are ALLOWED because they are
/// legitimate API calls, not brute-force tactic invocations.
///
/// Rationale (Codex N-3 / decision 1 option C):
///   Complete ban of the substring would violate Art. I.1.1 Completeness=1
///   (correct Mathlib-using proofs would be rejected). Allowing unqualified
///   `by decide` / `by omega` as a proof body lets agents brute-force
///   recurring number-theory lemmas (repeat of F-2026-04-20-05 class).
const BARE_TACTIC_FORBIDDEN: &[&str] = &["decide", "omega"];

/// Identity theft patterns — declarations that rename the target theorem.
const DECLARATION_KEYWORDS: &[&str] = &[
    "theorem ", "lemma ", "def ", "example ", "instance ",
    "structure ", "class ", "inductive ", "abbrev ",
];

#[derive(Clone)]
pub struct Lean4Oracle {
    pub problem_statement: String,
    pub theorem_name: String,
    lean_path: String,
    lean_binary: String,
    /// Phase 8.C v3 (C-067 R1-α): per-instance Ed25519 signing key.
    /// Private to the oracle. Bus registers `public_key()` at setup and
    /// verifies receipt signatures. Forging a receipt now requires breaking
    /// Ed25519 — in-process attackers can't inject new trusted pubkeys
    /// after `init()` freezes registration.
    ///
    /// `Clone` copies the signing key so the same oracle can be used in
    /// both tool-mount and receipt-issue contexts (single run-wide
    /// capability, registered once).
    signing_key: ed25519_dalek::SigningKey,
}

impl Lean4Oracle {
    pub fn new(problem_statement: String, theorem_name: String, lean_path: String) -> Self {
        // Use LEAN_BINARY env or auto-detect from MiniF2F lean-toolchain version.
        // Default: v4.24.0 (matches pre-built Mathlib oleans).
        let lean_binary = std::env::var("LEAN_BINARY").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
            let v4_24 = format!("{}/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean", home);
            if std::path::Path::new(&v4_24).exists() {
                v4_24
            } else {
                "lean".to_string()
            }
        });
        let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
        Lean4Oracle {
            problem_statement,
            theorem_name,
            lean_path,
            lean_binary,
            signing_key,
        }
    }

    /// The public key this oracle's receipts verify against. Register with
    /// `bus.register_oracle(oracle.public_key())` before `bus.init()`.
    pub fn public_key(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }

    /// Issue a signed `Complete` receipt for a proof the caller has already
    /// verified. Private signing key stays with the oracle — only the
    /// produced receipt leaves.
    pub fn issue_complete_receipt(
        &self,
        payload: &str,
        parent_id: Option<&str>,
    ) -> turingosv4::sdk::oracle_receipt::OracleReceipt {
        turingosv4::sdk::oracle_receipt::OracleReceipt::sign_new(
            payload, parent_id,
            turingosv4::sdk::predicate::Verdict::Complete,
            turingosv4::sdk::predicate::PredicateKind::Lean4Boolean,
            &self.signing_key,
        )
    }

    /// Issue a signed `PartialOk` receipt (step mode partial elaboration).
    pub fn issue_partial_receipt(
        &self,
        payload: &str,
        parent_id: Option<&str>,
    ) -> turingosv4::sdk::oracle_receipt::OracleReceipt {
        turingosv4::sdk::oracle_receipt::OracleReceipt::sign_new(
            payload, parent_id,
            turingosv4::sdk::predicate::Verdict::PartialOk { confidence: 1.0 },
            turingosv4::sdk::predicate::PredicateKind::Lean4Boolean,
            &self.signing_key,
        )
    }

    /// Pre-append security checks (Law 1: reject-only).
    pub fn check_payload(&self, payload: &str) -> Result<(), String> {
        // V3L-07: identity theft — reject new declarations with different names
        for keyword in DECLARATION_KEYWORDS {
            if let Some(pos) = payload.find(keyword) {
                let after = &payload[pos + keyword.len()..];
                let declared_name: String = after.chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !declared_name.is_empty() && declared_name != self.theorem_name {
                    return Err(format!(
                        "Identity theft: declared '{}' but target is '{}'",
                        declared_name, self.theorem_name
                    ));
                }
            }
        }

        // V3L-01: sorry firewall
        for word in ["sorry", "sorryAx"] {
            if has_word_boundary(payload, word) {
                return Err(format!("Forbidden: '{}' detected", word));
            }
        }

        // Forbidden patterns (substring match)
        for pattern in FORBIDDEN_PATTERNS {
            if payload.contains(pattern) {
                return Err(format!("Forbidden pattern: '{}'", pattern));
            }
        }

        // C-050 / Phase 8.D: bare decide/omega (allow qualified Mathlib form)
        for word in BARE_TACTIC_FORBIDDEN {
            if has_bare_tactic_invocation(payload, word) {
                return Err(format!("Forbidden bare tactic: '{}' (C-011 / C-050). \
                                    Qualified forms like Decidable.{} are allowed.",
                                   word, word));
            }
        }

        Ok(())
    }

    /// OMEGA verification — the ultimate boolean predicate.
    /// Feeds problem_statement + proof_chain to Lean 4 and checks:
    /// - "No goals to be solved" → OMEGA (true)
    /// - Any error → not OMEGA (false)
    ///
    /// V3L-02: deterministic — no random elements in verification.
    /// Rule 22 v2 clause 4: proof_chain passed verbatim, no byte modification.
    pub fn verify_omega(&self, proof_chain: &str) -> Result<bool, String> {
        self.verify_omega_detailed(proof_chain).map(|(ok, _)| ok)
    }

    /// Step-B v3: return (success, error_output) so callers can classify.
    /// Empty error string on success; raw combined stderr/stdout (truncated) on reject.
    /// Callers MUST pass through classify_lean_error before broadcast (C-022).
    ///
    /// F-2026-04-20-05 fix: run check_payload here too. Previously this was only
    /// enforced via on_pre_append when the payload traversed bus.append. The
    /// `complete` action calls this function directly and historically bypassed
    /// the forbidden-pattern check, allowing `native_decide` brute-force bytecode
    /// bypass to register as OMEGA. Enforcing here closes the hole without
    /// breaking bus-path callers (idempotent double-check).
    pub fn verify_omega_detailed(&self, proof_chain: &str) -> Result<(bool, String), String> {
        if let Err(reason) = self.check_payload(proof_chain) {
            log::warn!("[oracle] payload rejected pre-Lean: {}", reason);
            return Ok((false, format!("forbidden_payload: {}", reason)));
        }
        let full_code = format!("{}\n{}", self.problem_statement, proof_chain);
        if has_word_boundary(&full_code, "sorry") || has_word_boundary(&full_code, "sorryAx") {
            return Ok((false, "sorry_in_proof".into()));
        }

        // Compute gas limit based on code size.
        // Mathlib import alone takes ~50s on this VM, so base must be high.
        let lines = full_code.lines().count();
        let timeout_secs = 120 + (lines as u64);
        let timeout = Duration::from_secs(timeout_secs.min(300));

        // Execute in sandbox
        let sandbox = LocalProcessSandbox::new(
            &self.lean_binary,
            &["--stdin"],
        );

        // Set LEAN_PATH environment for Mathlib resolution
        std::env::set_var("LEAN_PATH", &self.lean_path);

        match sandbox.execute(&full_code, timeout) {
            Ok(SandboxResult::Completed { stdout, stderr, exit_code }) => {
                let combined = format!("{}\n{}", stdout, stderr);
                if combined.contains("declaration uses 'sorry'") {
                    log::warn!("oracle reject reason: declaration uses 'sorry'");
                    return Ok((false, "declaration_uses_sorry".into()));
                }
                if combined.contains("No goals to be solved") {
                    return Ok((true, String::new()));
                }
                if exit_code == 0 && !combined.contains("error:") {
                    return Ok((true, String::new()));
                }
                let err_preview: String = combined.lines()
                    .filter(|l| l.contains("error") || l.contains("unexpected") || l.contains("expected"))
                    .take(4)
                    .collect::<Vec<_>>()
                    .join(" | ");
                let detail = if err_preview.is_empty() { combined.chars().take(800).collect::<String>() } else { err_preview };
                log::warn!("oracle reject reason (exit={}): {}", exit_code, detail);
                Ok((false, detail))
            }
            Ok(SandboxResult::Timeout) => Err("Lean 4 verification timed out".into()),
            Err(e) => Err(format!("Sandbox error: {}", e)),
        }
    }

    /// Phase 7 (Turing δ-step): three-way classification for partial proofs.
    /// Elaborates `problem_statement + proof_prefix` (the accumulated tactic
    /// chain) and returns:
    ///   Complete   — Lean accepts, no remaining goals → OMEGA, halt the run
    ///   PartialOk  — tactics elaborate without type errors but goals remain →
    ///                the prefix is a valid computation path, write Q_{t+1} and
    ///                continue
    ///   Reject(r)  — any true elaboration error → prefix is invalid, do NOT
    ///                write; agent must try a different step
    ///
    /// This is the constitutional δ that Art. IV mermaid demands: a single
    /// transition rule that updates Q_t one step at a time, with ∏p as the
    /// pass/fail on that single step — not on the whole computation.
    pub fn verify_partial(&self, proof_prefix: &str) -> PartialVerdict {
        if let Err(reason) = self.check_payload(proof_prefix) {
            log::warn!("[oracle/partial] rejected pre-Lean: {}", reason);
            return PartialVerdict::Reject(format!("forbidden_payload: {}", reason));
        }
        let full_code = format!("{}\n{}", self.problem_statement, proof_prefix);
        if has_word_boundary(&full_code, "sorry") || has_word_boundary(&full_code, "sorryAx") {
            return PartialVerdict::Reject("sorry_in_proof".into());
        }

        let lines = full_code.lines().count();
        let timeout_secs = 120 + (lines as u64);
        let timeout = Duration::from_secs(timeout_secs.min(300));

        let sandbox = LocalProcessSandbox::new(
            &self.lean_binary,
            &["--stdin"],
        );
        std::env::set_var("LEAN_PATH", &self.lean_path);

        match sandbox.execute(&full_code, timeout) {
            Ok(SandboxResult::Completed { stdout, stderr, exit_code }) => {
                let combined = format!("{}\n{}", stdout, stderr);
                if combined.contains("declaration uses 'sorry'") {
                    return PartialVerdict::Reject("declaration_uses_sorry".into());
                }
                if combined.contains("No goals to be solved") {
                    return PartialVerdict::Complete;
                }
                // Lean reports "unsolved goals" when a proof compiles but does
                // not discharge all obligations. This is EXACTLY the partial-
                // OK case: the tactics ran without type errors, but more work
                // remains. Under the old semantics this was a REJECT; under
                // Phase 7, it is the signal that Q_{t+1} is well-formed.
                if combined.contains("unsolved goals") && !combined.contains("error: unknown") && !combined.contains("error: type mismatch") {
                    // Only accept as partial if there are no OTHER errors.
                    // Distinguish unsolved-goals (partial OK) from actual bugs.
                    let hard_errors: Vec<&str> = combined.lines()
                        .filter(|l| l.contains("error:") && !l.contains("unsolved goals"))
                        .collect();
                    if hard_errors.is_empty() {
                        return PartialVerdict::PartialOk;
                    }
                    // Has both unsolved-goals and a hard error → reject
                    let detail = hard_errors.iter().take(3).cloned().collect::<Vec<_>>().join(" | ");
                    return PartialVerdict::Reject(format!("hard_error_with_unsolved: {}", detail));
                }
                if exit_code == 0 && !combined.contains("error:") {
                    // Clean compile, no "No goals to be solved" marker, no
                    // error. Edge case: treat as Complete (same rule as
                    // verify_omega_detailed path 2).
                    return PartialVerdict::Complete;
                }
                let err_preview: String = combined.lines()
                    .filter(|l| l.contains("error") || l.contains("unexpected") || l.contains("expected"))
                    .take(4)
                    .collect::<Vec<_>>()
                    .join(" | ");
                let detail = if err_preview.is_empty() {
                    combined.chars().take(400).collect::<String>()
                } else {
                    err_preview
                };
                PartialVerdict::Reject(detail)
            }
            Ok(SandboxResult::Timeout) => PartialVerdict::Reject("lean_timeout".into()),
            Err(e) => PartialVerdict::Reject(format!("sandbox_error: {}", e)),
        }
    }
}

/// Phase 7 three-way verdict on a partial proof prefix.
#[derive(Debug, Clone)]
pub enum PartialVerdict {
    /// All goals solved → OMEGA reached, halt the run.
    Complete,
    /// Tactics elaborate without type errors but goals remain → Q_{t+1} valid,
    /// append this step as a tape node and continue.
    PartialOk,
    /// Elaboration failed → Q_{t+1} = Q_t, agent tries a different step.
    Reject(String),
}

impl TuringTool for Lean4Oracle {
    fn manifest(&self) -> &str {
        "lean4_oracle"
    }

    fn on_pre_append(&mut self, _author: &str, payload: &str) -> ToolSignal {
        match self.check_payload(payload) {
            Ok(()) => ToolSignal::Pass,
            Err(reason) => ToolSignal::Veto(reason),
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

/// Word boundary check — ensures we match whole words, not substrings.
fn has_word_boundary(text: &str, word: &str) -> bool {
    for (i, _) in text.match_indices(word) {
        let before = if i > 0 { text.as_bytes()[i - 1] } else { b' ' };
        let after_idx = i + word.len();
        let after = if after_idx < text.len() { text.as_bytes()[after_idx] } else { b' ' };
        if !before.is_ascii_alphanumeric() && before != b'_'
           && !after.is_ascii_alphanumeric() && after != b'_' {
            return true;
        }
    }
    false
}

/// Strip string literals and comments so `has_bare_tactic_invocation` only
/// sees real code. Preserves whitespace structure (replaces skipped content
/// with a single space) so word-boundary detection still works.
///
/// Handled:
///   - `"..."` string literals (with `\"` escape)
///   - `-- ... \n` line comments
///   - `/- ... -/` block comments, **including Lean's nested form**
///     (R3 Codex residual CHALLENGE): a depth counter tracks `/-`/`-/`
///     openings so `/- outer /- inner -/ still outer -/` is fully stripped.
///   - Doc/section comments `/-- ... -/` and `/-! ... -/` also enter the
///     `/-` branch and get stripped.
/// Not handled (out of Paper-1 scope): `⟨...⟩` antiquotations; string
/// interpolation with code; multiline `raw""` strings — rare in MiniF2F.
fn strip_strings_and_comments(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                // String literal — consume until unescaped closing quote.
                while let Some(&nc) = chars.peek() {
                    chars.next();
                    if nc == '\\' {
                        chars.next();  // skip the escaped char
                    } else if nc == '"' {
                        break;
                    }
                }
                out.push(' ');
            }
            '-' if chars.peek() == Some(&'-') => {
                // Line comment.
                chars.next();  // the 2nd `-`
                while let Some(&nc) = chars.peek() {
                    if nc == '\n' { break; }
                    chars.next();
                }
                out.push(' ');
            }
            '/' if chars.peek() == Some(&'-') => {
                // Block comment /- ... -/ with nested depth tracking.
                chars.next();  // the `-` after `/`
                let mut depth: u32 = 1;
                while depth > 0 {
                    match chars.next() {
                        Some('/') if chars.peek() == Some(&'-') => {
                            chars.next();
                            depth += 1;
                        }
                        Some('-') if chars.peek() == Some(&'/') => {
                            chars.next();
                            depth -= 1;
                        }
                        Some(_) => {}
                        None => break,  // unterminated — give up
                    }
                }
                out.push(' ');
            }
            _ => out.push(c),
        }
    }
    out
}

/// Whole-word match NOT preceded by a namespace qualifier (`.` or `:`),
/// operating on source code with string literals and comments already
/// stripped. Unicode-safe (uses char iteration, not byte indexing).
///
/// Returns true for bare tactic invocations like `by decide`, `; omega`,
/// `  decide\n`. Returns false for:
///   - qualified references (`Decidable.decide`, `Nat.decide_lt`)
///   - identifier containment (`native_decide`, `my_decide_helper`)
///   - in-string-literal occurrences (`"decide fate"`)
///   - in-comment occurrences (`-- decide this`)
///   - Unicode-neighbored word-boundaries (won't misread UTF-8 bytes)
fn has_bare_tactic_invocation(text: &str, word: &str) -> bool {
    let clean = strip_strings_and_comments(text);
    let haystack = clean.as_str();
    let mut search_from = 0;
    while let Some(rel_idx) = haystack[search_from..].find(word) {
        let idx = search_from + rel_idx;
        let after_idx = idx + word.len();

        // Char immediately before `idx` (None if at start).
        // Safe: `idx` is a char boundary returned by `find`.
        let before_char: Option<char> = haystack[..idx].chars().next_back();
        // Char immediately after the matched word (None if at end).
        let after_char: Option<char> = haystack[after_idx..].chars().next();

        let is_word_ch = |c: char| c.is_alphanumeric() || c == '_';

        let word_boundary =
            !matches!(before_char, Some(c) if is_word_ch(c))
            && !matches!(after_char, Some(c) if is_word_ch(c));

        if !word_boundary {
            search_from = idx + word.len();
            continue;
        }

        // Qualified with namespace? Dot OR colon.
        if matches!(before_char, Some('.') | Some(':')) {
            search_from = idx + word.len();
            continue;
        }

        return true;
    }
    false
}

#[cfg(test)]
mod tactic_whitelist_tests {
    use super::{has_bare_tactic_invocation, strip_strings_and_comments};

    #[test]
    fn rejects_bare_decide() {
        assert!(has_bare_tactic_invocation("by decide", "decide"));
        assert!(has_bare_tactic_invocation("  decide\n", "decide"));
        assert!(has_bare_tactic_invocation("; decide", "decide"));
        assert!(has_bare_tactic_invocation("decide", "decide"));
    }

    #[test]
    fn allows_qualified_decide() {
        assert!(!has_bare_tactic_invocation("Decidable.decide p", "decide"));
        assert!(!has_bare_tactic_invocation("Nat.decide_lt", "decide"));
        assert!(!has_bare_tactic_invocation("@Decidable.decide", "decide"));
    }

    #[test]
    fn allows_identifier_containing_decide() {
        // `native_decide` — _ kills word boundary (handled separately)
        assert!(!has_bare_tactic_invocation("native_decide", "decide"));
        // `my_decide_helper` — likewise
        assert!(!has_bare_tactic_invocation("my_decide_helper", "decide"));
        // `decideFoo` — capital after → still a word but no identifier
        assert!(!has_bare_tactic_invocation("decideFoo", "decide"));
    }

    #[test]
    fn rejects_bare_omega() {
        assert!(has_bare_tactic_invocation("by omega", "omega"));
        assert!(has_bare_tactic_invocation("; omega\n", "omega"));
    }

    #[test]
    fn allows_qualified_omega() {
        assert!(!has_bare_tactic_invocation("Mathlib.Tactic.Omega.omega", "omega"));
        assert!(!has_bare_tactic_invocation("Nat.Linear.Omega", "omega"));
    }

    #[test]
    fn allows_real_proofs() {
        // A typical linarith-based proof with no forbidden tactic.
        let proof = "by\n  have h : a + b > 0 := by linarith\n  exact h.le";
        assert!(!has_bare_tactic_invocation(proof, "decide"));
        assert!(!has_bare_tactic_invocation(proof, "omega"));
    }

    // R3: string / comment / Unicode robustness (Gemini + Codex CHALLENGE).

    #[test]
    fn ignores_string_literal_occurrence() {
        // `"decide"` inside a string literal must NOT be flagged.
        let proof = r#"have h_str : String := "decide the fate""#;
        assert!(!has_bare_tactic_invocation(proof, "decide"));
        let proof2 = r#"have msg := "please omega this""#;
        assert!(!has_bare_tactic_invocation(proof2, "omega"));
    }

    #[test]
    fn ignores_escape_sequences_in_string() {
        let proof = r#"have s := "\"decide\" is fine here""#;
        assert!(!has_bare_tactic_invocation(proof, "decide"));
    }

    #[test]
    fn ignores_line_comment_occurrence() {
        let proof = "-- try by decide first\nby linarith";
        assert!(!has_bare_tactic_invocation(proof, "decide"));
    }

    #[test]
    fn ignores_block_comment_occurrence() {
        let proof = "/- consider omega for this -/\nby linarith";
        assert!(!has_bare_tactic_invocation(proof, "omega"));
    }

    #[test]
    fn catches_bare_tactic_even_with_comments_elsewhere() {
        let proof = "-- this is a comment\nby decide\n-- end comment";
        assert!(has_bare_tactic_invocation(proof, "decide"));
    }

    #[test]
    fn handles_unicode_identifier_neighbors() {
        // `αβγ.decide` — qualified; `γ` is 2-byte UTF-8 preceding `.`
        let proof = "have h := αβγ.decide";
        assert!(!has_bare_tactic_invocation(proof, "decide"),
            "qualified reference with Unicode namespace must be allowed");
    }

    #[test]
    fn handles_unicode_in_comment_and_string() {
        // Ensure Unicode in stripped regions doesn't break stripping.
        let proof = r#"-- 中文注释 decide here
"α → β omega"
by linarith"#;
        assert!(!has_bare_tactic_invocation(proof, "decide"));
        assert!(!has_bare_tactic_invocation(proof, "omega"));
    }

    #[test]
    fn handles_nested_block_comments() {
        // R3 Codex residual: Lean nested block comments must NOT terminate
        // stripping at the inner `-/`, which would leak `decide` into the
        // scan. Depth counter must balance openings/closings.
        let proof = "/- outer /- inner -/ still outer decide -/ by linarith";
        assert!(!has_bare_tactic_invocation(proof, "decide"),
            "nested block comment must fully strip; outer 'decide' must be ignored");
        let proof2 = "/- depth 1 /- depth 2 /- depth 3 decide -/ -/ -/ by ring";
        assert!(!has_bare_tactic_invocation(proof2, "decide"),
            "3-deep nested comment must fully strip");
    }

    #[test]
    fn handles_doc_comments() {
        let proof = "/-- doc comment with decide -/\nby linarith";
        assert!(!has_bare_tactic_invocation(proof, "decide"));
    }

    #[test]
    fn strip_leaves_real_tactic_visible() {
        let proof = r#"
            -- helper
            have h : 1 = 1 := rfl
            "a string"
            by decide
        "#;
        let stripped = strip_strings_and_comments(proof);
        assert!(stripped.contains("by decide"));
        assert!(!stripped.contains("a string"));
    }
}

/// Derive LEAN_PATH from the MiniF2F data directory.
/// Searches .lake/packages/*/.lake/build/lib/lean (Lake 4 layout).
pub fn derive_lean_path(minif2f_dir: &str) -> String {
    let lake_dir = PathBuf::from(minif2f_dir).join(".lake/packages");
    let mut paths = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&lake_dir) {
        for entry in entries.flatten() {
            // Lake 4 layout: packages/<pkg>/.lake/build/lib/lean
            let build_lib = entry.path().join(".lake/build/lib/lean");
            if build_lib.is_dir() {
                paths.push(build_lib.display().to_string());
            }
            // Fallback: packages/<pkg>/lib/lean (older layout)
            let lib_lean = entry.path().join("lib").join("lean");
            if lib_lean.is_dir() {
                paths.push(lib_lean.display().to_string());
            }
        }
    }

    // Also add the project's own build output
    let project_lib = PathBuf::from(minif2f_dir).join(".lake/build/lib/lean");
    if project_lib.is_dir() {
        paths.push(project_lib.display().to_string());
    }

    paths.join(":")
}

/// Load a problem file: read the .lean file, extract theorem name, prepare for agents.
pub fn load_problem(problem_path: &str) -> Result<(String, String), String> {
    let content = std::fs::read_to_string(problem_path)
        .map_err(|e| format!("Cannot read {}: {}", problem_path, e))?;

    // Extract theorem name from "theorem <name>" line
    let theorem_name = content.lines()
        .find(|line| line.starts_with("theorem "))
        .and_then(|line| {
            line.strip_prefix("theorem ")
                .map(|rest| rest.split_whitespace().next().unwrap_or("").to_string())
        })
        .ok_or_else(|| format!("No theorem declaration found in {}", problem_path))?;

    // Replace "by sorry" with "by\n" to leave room for agent tactics
    let prepared = content.replace("by sorry", "by\n");

    Ok((prepared, theorem_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_oracle() -> Lean4Oracle {
        Lean4Oracle::new(
            "theorem test_thm : 1 + 1 = 2 := by".to_string(),
            "test_thm".to_string(),
            "".to_string(),
        )
    }

    #[test]
    fn test_sorry_rejected() {
        let oracle = make_oracle();
        assert!(oracle.check_payload("sorry").is_err());
    }

    #[test]
    fn test_sorry_in_word_not_rejected() {
        let oracle = make_oracle();
        // "sorry" inside another word should not trigger
        assert!(oracle.check_payload("notsorryhere").is_ok());
    }

    #[test]
    fn test_identity_theft_rejected() {
        let oracle = make_oracle();
        assert!(oracle.check_payload("theorem wrong_name : True := trivial").is_err());
    }

    #[test]
    fn test_correct_theorem_name_accepted() {
        let oracle = make_oracle();
        assert!(oracle.check_payload("theorem test_thm : True := trivial").is_ok());
    }

    #[test]
    fn test_forbidden_native_decide() {
        let oracle = make_oracle();
        assert!(oracle.check_payload("native_decide").is_err());
    }

    #[test]
    fn test_forbidden_io_process() {
        let oracle = make_oracle();
        assert!(oracle.check_payload("IO.Process.run").is_err());
    }

    #[test]
    fn test_clean_tactic_accepted() {
        let oracle = make_oracle();
        assert!(oracle.check_payload("simp [Nat.add_comm]").is_ok());
    }

    #[test]
    fn test_bare_decide_forbidden() {
        // Phase 8.D (C-050, Codex N-3 / decision 1 option C):
        // bare `decide` as agent tactic is forbidden (prevents brute-force
        // repeat of native_decide class abuse, F-2026-04-20-05).
        let oracle = make_oracle();
        assert!(oracle.check_payload("decide").is_err(),
            "bare decide must be forbidden (C-011 complete)");
        assert!(oracle.check_payload("by decide").is_err());
        // Qualified forms (legitimate Mathlib API) must still pass.
        assert!(oracle.check_payload("Decidable.decide p").is_ok(),
            "qualified Decidable.decide is legitimate API use");
        assert!(oracle.check_payload("Nat.decide_lt").is_ok());
    }

    #[test]
    fn test_bare_omega_forbidden() {
        // Mirrors decide policy for omega (Mathlib omega tactic).
        let oracle = make_oracle();
        assert!(oracle.check_payload("by omega").is_err());
        assert!(oracle.check_payload("Nat.Linear.Omega").is_ok(),
            "qualified Omega namespace ref is legitimate");
    }

    #[test]
    fn test_word_boundary_function() {
        assert!(has_word_boundary("x sorry y", "sorry"));
        assert!(!has_word_boundary("notsorryhere", "sorry"));
        assert!(has_word_boundary("sorry", "sorry"));
        assert!(has_word_boundary("(sorry)", "sorry"));
    }
}
