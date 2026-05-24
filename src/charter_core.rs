//! TRACE_MATRIX FC2-Q_0 + FC3-constitution_binding: TDMA-Bounded CharterCore.
//!
//! CharterCore is a bounded, content-addressed distillation of the constitution
//! that fits within B_G=500 tokens. The kernel injects ONLY this CharterCore
//! into the worker prompt — NEVER the full constitution.md bytes (KILL-tdma-6).
//!
//! The drift detector ties the CharterCore to a specific constitution.md SHA-256
//! so that any constitutional revision invalidates the compiled charter and
//! forces recompilation. This prevents a CharterCore from silently diverging
//! from the canonical constitution after the latter changes (directive §9).
//!
//! The compiler is a deterministic regex extractor — it picks out
//! Art./Article anchors, FC1/FC2/FC3 mentions, and constitutional KILL/Forbidden
//! lines, packs them under the B_G ceiling, and stores the rest as retrieval
//! handles so the kernel can fetch them on demand.
//!
//! Phase E Path B (libgit2 real-git substrate per Art. 0.4) is implemented
//! for the TDMA-bounded loop in `src/git_tape_ledger.rs` and for the L4
//! canonical transition ledger in `src/bottom_white/ledger/transition_ledger.rs`.
//! The legacy `src/ledger.rs` in-memory `Tape` Path A peer remains for the
//! `run_proof` default and emergency in-process rollback.
//! See `handover/architect-insights/PHASE_E_TODO_TDMA.md` for the original
//! Phase E obligation chain.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

use crate::token_budget::B_G;
use crate::tokenizer::Tokenizer;

// ── Schema ───────────────────────────────────────────────────────

/// Bounded constitutional charter (directive §9).
/// TRACE_MATRIX FC2-Q_0 + FC3-constitution_binding: The single object that
/// carries the constitution into the kernel prompt within a hard budget.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharterCore {
    pub schema_version: String,
    pub constitution_sha256: String,
    pub compiler_version: String,
    pub charter_core_sha256: String,
    pub content: String,
    pub retrieval_handles: Vec<String>,
    pub covered_invariants: Vec<String>,
    pub omitted_sections: Vec<String>,
    pub token_count: usize,
}

/// Drift detection outcomes.
/// TRACE_MATRIX FC2-Q_0 + KILL-tdma-7: Distinguishes the bootable-clean case
/// from the stale-constitution case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharterDriftError {
    ConstitutionShaMismatch {
        expected: String,
        actual: String,
    },
}

impl std::fmt::Display for CharterDriftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharterDriftError::ConstitutionShaMismatch { expected, actual } => write!(
                f,
                "CharterCore drift: constitution sha mismatch (expected {}, actual {})",
                expected, actual
            ),
        }
    }
}

impl std::error::Error for CharterDriftError {}

// ── Helpers ──────────────────────────────────────────────────────

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

// ── Compiler ─────────────────────────────────────────────────────

/// Pull anchor lines from the constitution: Art. 0.x, Art. I.x, FC1/2/3, KILL.
/// TRACE_MATRIX FC2-Q_0: Deterministic extractor.
fn extract_anchors(constitution_text: &str) -> Vec<String> {
    let mut anchors = Vec::new();
    for line in constitution_text.lines() {
        let trimmed = line.trim();
        let is_anchor = trimmed.starts_with("Art.")
            || trimmed.starts_with("# ")
            || trimmed.starts_with("## ")
            || trimmed.starts_with("### ")
            || trimmed.contains("FC1")
            || trimmed.contains("FC2")
            || trimmed.contains("FC3")
            || trimmed.to_uppercase().contains("KILL")
            || trimmed.to_uppercase().contains("FORBIDDEN")
            || trimmed.to_uppercase().contains("INVARIANT");
        if is_anchor && !trimmed.is_empty() {
            anchors.push(trimmed.to_string());
        }
    }
    anchors
}

/// Extract `Art. X.Y` mentions for retrieval_handles + covered_invariants.
/// Tolerates both `Art.0.1` (no space) and `Art. 0.1` (with space) styles.
fn extract_article_mentions(text: &str) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i + 4 <= chars.len() {
        if chars[i..i + 4].iter().collect::<String>() == "Art." {
            let mut j = i + 4;
            // Skip whitespace after "Art."
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            // Collect the article ID: alphanumeric + dots
            let mut id = String::new();
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '.') {
                id.push(chars[j]);
                j += 1;
            }
            let id = id.trim_end_matches('.').to_string();
            if !id.is_empty() {
                let handle = format!("Art.{}", id);
                if seen.insert(handle.clone()) {
                    out.push(handle);
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

/// Compile a CharterCore from raw constitution bytes (directive §9).
/// TRACE_MATRIX FC2-Q_0 + KILL-tdma-6: Bounds content to B_G tokens and
/// records the constitution sha for drift detection. The CharterCore.content
/// is what the kernel will inject into the worker prompt — NEVER the raw
/// constitution.md bytes.
pub fn compile_charter_core(
    constitution_bytes: &[u8],
    compiler_version: &str,
    tokenizer: &Tokenizer,
) -> CharterCore {
    let constitution_sha256 = sha256_hex(constitution_bytes);
    let text = String::from_utf8_lossy(constitution_bytes).to_string();

    let anchors = extract_anchors(&text);
    let article_mentions = extract_article_mentions(&text);

    // Build content by greedy-accumulating anchors under B_G budget.
    let mut content = String::new();
    let mut omitted: Vec<String> = Vec::new();
    let mut current_tokens: usize = 0;
    for anchor in &anchors {
        let line_tokens = tokenizer.count_text(anchor) + 1; // +1 for newline
        if current_tokens + line_tokens <= B_G {
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(anchor);
            current_tokens = tokenizer.count_text(&content);
        } else {
            omitted.push(anchor.clone());
        }
    }

    // Hard-cap content tokens at B_G — clip from the end if estimator under-counted.
    while tokenizer.count_text(&content) > B_G && !content.is_empty() {
        if let Some(last_nl) = content.rfind('\n') {
            content.truncate(last_nl);
        } else {
            content.clear();
        }
    }

    let token_count = tokenizer.count_text(&content);
    let retrieval_handles = article_mentions.clone();
    let covered_invariants = article_mentions;

    // CharterCore sha is computed on the canonical fields except the sha itself.
    let payload_for_hash = serde_json::json!({
        "schema_version": "tdma-charter-core/v1",
        "constitution_sha256": constitution_sha256,
        "compiler_version": compiler_version,
        "content": content,
        "retrieval_handles": retrieval_handles,
        "covered_invariants": covered_invariants,
        "omitted_sections": omitted,
        "token_count": token_count,
    });
    let charter_core_sha256 = sha256_hex(
        serde_json::to_string(&payload_for_hash)
            .unwrap_or_default()
            .as_bytes(),
    );

    CharterCore {
        schema_version: "tdma-charter-core/v1".into(),
        constitution_sha256,
        compiler_version: compiler_version.into(),
        charter_core_sha256,
        content,
        retrieval_handles,
        covered_invariants,
        omitted_sections: omitted,
        token_count,
    }
}

// ── Drift detector ───────────────────────────────────────────────

/// Boot-time freshness check (directive §9 + KILL-tdma-7).
/// TRACE_MATRIX FC2-Q_0 + KILL-tdma-7: If the constitution bytes hash differs
/// from the stored constitution_sha256, the CharterCore is stale and the
/// kernel MUST refuse to boot (or recompile).
pub fn validate_charter_core_freshness(
    charter: &CharterCore,
    constitution_bytes: &[u8],
) -> Result<(), CharterDriftError> {
    let actual = sha256_hex(constitution_bytes);
    if charter.constitution_sha256 != actual {
        return Err(CharterDriftError::ConstitutionShaMismatch {
            expected: charter.constitution_sha256.clone(),
            actual,
        });
    }
    Ok(())
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn tk() -> Tokenizer {
        Tokenizer::new()
    }

    fn sample_constitution() -> &'static str {
        "# Constitution\n\
         ## Art. 0.1 — Information is Free\n\
         All signals must traverse the tape.\n\
         ## Art. 0.2 — Tape Canonicity\n\
         FORBIDDEN: shadow ledger source of truth.\n\
         ## Art. 0.4 — Q_t Version Control\n\
         FC1a tape_t carries proposals. FC1b wtool advances Q_{t+1}.\n\
         ## Art. I.1 — Natural-language constraints are soft\n\
         INVARIANT: every soft rule must have a CI guard.\n\
         KILL-clauses must convert into runtime asserts.\n"
    }

    // ── charter_core_budget ─────────────────────────────────────

    #[test]
    fn charter_core_budget_under_b_g() {
        let bytes = sample_constitution().as_bytes();
        let charter = compile_charter_core(bytes, "v1.0", &tk());
        assert!(
            charter.token_count <= B_G,
            "content must fit B_G={} (got {})",
            B_G,
            charter.token_count
        );
        assert_eq!(charter.schema_version, "tdma-charter-core/v1");
        assert_eq!(charter.compiler_version, "v1.0");
        assert_eq!(charter.constitution_sha256.len(), 64);
        assert_eq!(charter.charter_core_sha256.len(), 64);
    }

    #[test]
    fn charter_core_budget_preserves_anchors_when_small() {
        let bytes = sample_constitution().as_bytes();
        let charter = compile_charter_core(bytes, "v1.0", &tk());
        // Small enough to fully include all anchors
        assert!(charter.content.contains("Art. 0.4"));
        assert!(charter.content.contains("FC1a") || charter.content.contains("FC1b"));
        assert!(charter.content.contains("KILL") || charter.content.contains("FORBIDDEN"));
        assert!(charter.retrieval_handles.iter().any(|h| h.starts_with("Art.")));
    }

    #[test]
    fn charter_core_budget_clips_oversize_constitution() {
        // Build a constitution that exceeds B_G with anchors alone.
        let mut big = String::new();
        for i in 0..2000 {
            big.push_str(&format!("## Art. {}.X — anchor line {}\n", i, i));
        }
        let charter = compile_charter_core(big.as_bytes(), "v1.0", &tk());
        assert!(charter.token_count <= B_G, "must clip oversize");
        assert!(!charter.omitted_sections.is_empty(), "omitted set populated");
    }

    // ── charter_core_drift ──────────────────────────────────────

    #[test]
    fn charter_core_drift_detects_constitution_change() {
        let bytes_v1 = sample_constitution().as_bytes();
        let charter = compile_charter_core(bytes_v1, "v1.0", &tk());

        // No change yet -> Ok
        assert!(validate_charter_core_freshness(&charter, bytes_v1).is_ok());

        // Constitution mutated -> Err
        let mutated = format!("{}\n## Art. 0.5 — new clause\n", sample_constitution());
        let err = validate_charter_core_freshness(&charter, mutated.as_bytes()).unwrap_err();
        match err {
            CharterDriftError::ConstitutionShaMismatch { expected, actual } => {
                assert_ne!(expected, actual);
                assert_eq!(expected.len(), 64);
                assert_eq!(actual.len(), 64);
            }
        }
    }

    #[test]
    fn charter_core_recompile_after_drift_validates_clean() {
        let bytes_v1 = sample_constitution().as_bytes();
        let charter_v1 = compile_charter_core(bytes_v1, "v1.0", &tk());

        let bytes_v2 = format!("{}\n## Art. 0.5 — new\n", sample_constitution());
        assert!(validate_charter_core_freshness(&charter_v1, bytes_v2.as_bytes()).is_err());

        // After recompilation against the new bytes, drift goes away.
        let charter_v2 = compile_charter_core(bytes_v2.as_bytes(), "v1.0", &tk());
        assert!(validate_charter_core_freshness(&charter_v2, bytes_v2.as_bytes()).is_ok());
        assert_ne!(charter_v1.constitution_sha256, charter_v2.constitution_sha256);
    }
}
