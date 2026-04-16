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

/// Forbidden patterns in agent-submitted code.
/// These are checked BEFORE sending to Lean 4.
const FORBIDDEN_PATTERNS: &[&str] = &[
    "#eval", "#check", "#reduce", "#exec", "#print",  // output/reflection
    "native_decide",                                     // bytecode bypass
    "IO.Process", "IO.FS", "System.FilePath",           // system escape
    "run_tac", "unsafe", "dbg_trace", "IO.println",    // meta/debug
];

/// Identity theft patterns — declarations that rename the target theorem.
const DECLARATION_KEYWORDS: &[&str] = &[
    "theorem ", "lemma ", "def ", "example ", "instance ",
    "structure ", "class ", "inductive ", "abbrev ",
];

pub struct Lean4Oracle {
    pub problem_statement: String,
    pub theorem_name: String,
    lean_path: String,
    lean_binary: String,
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
        Lean4Oracle {
            problem_statement,
            theorem_name,
            lean_path,
            lean_binary,
        }
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

        // Forbidden patterns
        for pattern in FORBIDDEN_PATTERNS {
            if payload.contains(pattern) {
                return Err(format!("Forbidden pattern: '{}'", pattern));
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
    pub fn verify_omega_detailed(&self, proof_chain: &str) -> Result<(bool, String), String> {
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
    fn test_decide_tactic_permitted() {
        // decide is a legitimate tactic (not forbidden per se)
        let oracle = make_oracle();
        assert!(oracle.check_payload("decide").is_ok());
    }

    #[test]
    fn test_word_boundary_function() {
        assert!(has_word_boundary("x sorry y", "sorry"));
        assert!(!has_word_boundary("notsorryhere", "sorry"));
        assert!(has_word_boundary("sorry", "sorry"));
        assert!(has_word_boundary("(sorry)", "sorry"));
    }
}
