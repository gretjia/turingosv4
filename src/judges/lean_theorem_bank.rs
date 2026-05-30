//! TRACE_MATRIX FC1a-judge_pi: Lean theorem bank — the problem set for the
//! price-routed proof market (P0-B). Retires the hardcoded `ZETA_TASK`: a JSONL
//! file of target theorems, each yielding a `LeanJudge`.
//!
//! Schema (one JSON object per line):
//! ```text
//! { "id", "source"("core"|"mathlib"), "difficulty", "needs_mathlib": bool,
//!   "preamble" (imports + open + `theorem <name> <args> : <goal> := by`),
//!   "reference_body" (a known-good proof body — HARNESS SELF-TEST ONLY, never
//!     shown to a market agent: leaking it would be benchmark contamination),
//!   "note" }
//! ```
//!
//! The `reference_body` lets the bank self-verify (every target IS provable and
//! its preamble is well-formed) before the experiment trusts it — the
//! `reference_proofs_verify` test kernel-checks each one. Class 1 (additive data
//! + parser; reuses `LeanJudge`; no §6 surface).

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::judges::lean_judge::{default_lean_bin, LeanJudge, PINNED_TOOLCHAIN};

#[derive(Debug, Clone, Deserialize)]
pub struct LeanTheorem {
    pub id: String,
    pub source: String,
    pub difficulty: String,
    pub needs_mathlib: bool,
    /// imports + `open` + `theorem <name> <args> : <goal> := by`.
    pub preamble: String,
    /// Known-good proof body. SELF-TEST ONLY — never expose to market agents.
    pub reference_body: String,
    #[serde(default)]
    pub note: String,
}

impl LeanTheorem {
    /// Build a `LeanJudge` for this theorem. `mathlib_lean_path` is required for
    /// `needs_mathlib` theorems (the colon-joined olean search path from
    /// `lake env printenv LEAN_PATH`); ignored for core theorems.
    pub fn judge(&self, lean_bin: PathBuf, mathlib_lean_path: Option<&str>) -> LeanJudge {
        let mut j = LeanJudge::new(self.preamble.clone());
        j.lean_bin = lean_bin;
        if self.needs_mathlib {
            if let Some(lp) = mathlib_lean_path {
                j.extra_env.push(("LEAN_PATH".to_string(), lp.to_string()));
            }
        }
        j
    }
}

/// Parse a JSONL theorem bank. Blank lines and `//`-prefixed comment lines skip.
pub fn load_bank(path: impl AsRef<Path>) -> Result<Vec<LeanTheorem>, String> {
    let p = path.as_ref();
    let text = std::fs::read_to_string(p).map_err(|e| format!("read bank {}: {e}", p.display()))?;
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("//") {
            continue;
        }
        let thm: LeanTheorem =
            serde_json::from_str(t).map_err(|e| format!("bank line {}: {e}", i + 1))?;
        out.push(thm);
    }
    Ok(out)
}

/// Resolve the Mathlib `LEAN_PATH` by asking lake in the Mathlib project dir.
/// Returns `None` if lake/dir is unavailable (callers then skip Mathlib theorems).
pub fn mathlib_lean_path(mathlib_dir: impl AsRef<Path>, lake_bin: &Path) -> Option<String> {
    let out = std::process::Command::new(lake_bin)
        .args(["env", "printenv", "LEAN_PATH"])
        .current_dir(mathlib_dir.as_ref())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Default lake binary (pinned toolchain), mirroring `default_lean_bin`.
pub fn default_lake_bin() -> PathBuf {
    if let Some(home) = std::env::var_os("HOME") {
        let pinned = PathBuf::from(&home)
            .join(".elan")
            .join("toolchains")
            .join(PINNED_TOOLCHAIN)
            .join("bin")
            .join("lake");
        if pinned.exists() {
            return pinned;
        }
    }
    PathBuf::from("lake")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bank_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lean_theorems.jsonl")
    }

    #[test]
    fn bank_parses_and_is_well_formed() {
        let bank = load_bank(bank_path()).expect("load bank");
        assert!(bank.len() >= 5, "expected >=5 theorems, got {}", bank.len());
        assert!(bank.iter().any(|t| t.needs_mathlib), "want at least one Mathlib entry");
        for t in &bank {
            assert!(t.preamble.contains(":= by"), "{}: preamble must end with := by", t.id);
            for tok in ["sorry", "admit", "native_decide"] {
                assert!(
                    !t.reference_body.contains(tok),
                    "{}: reference body smuggles kernel-bypass token {tok}",
                    t.id
                );
            }
        }
    }

    /// Real-run self-test: every reference body must kernel-`Verified`. Gated on
    /// the pinned toolchain (skips on CI without it). Mathlib entries additionally
    /// need a built Mathlib (`TOS_MATHLIB_DIR` env or
    /// `handover/lean_env/mathlib_dir.txt`); those entries skip if it is absent.
    #[test]
    fn reference_proofs_verify() {
        let lean_bin = default_lean_bin();
        if !(lean_bin.is_absolute() && lean_bin.exists()) {
            eprintln!("skip: pinned Lean toolchain {PINNED_TOOLCHAIN} absent");
            return;
        }
        let mathlib_lp = resolve_mathlib_dir().and_then(|d| mathlib_lean_path(d, &default_lake_bin()));
        let bank = load_bank(bank_path()).expect("load bank");
        for t in &bank {
            if t.needs_mathlib && mathlib_lp.is_none() {
                eprintln!("skip {} (no Mathlib build available)", t.id);
                continue;
            }
            let judge = t.judge(lean_bin.clone(), mathlib_lp.as_deref());
            let o = judge.verify(&t.reference_body);
            assert!(o.is_verified(), "bank {} reference body did not verify: {o:?}", t.id);
        }
    }

    fn resolve_mathlib_dir() -> Option<PathBuf> {
        if let Ok(d) = std::env::var("TOS_MATHLIB_DIR") {
            let p = PathBuf::from(d);
            if p.exists() {
                return Some(p);
            }
        }
        let pointer =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("handover/lean_env/mathlib_dir.txt");
        if let Ok(s) = std::fs::read_to_string(&pointer) {
            let p = PathBuf::from(s.trim());
            if p.exists() {
                return Some(p);
            }
        }
        None
    }
}
