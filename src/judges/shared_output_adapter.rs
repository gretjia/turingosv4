//! TRACE_MATRIX FC1a-output_edge: M7 — shared output adapter + deterministic
//! diff materializer (Constitutional Market Activation G0, Class 1 additive).
//!
//! Two responsibilities, both pure / side-effect-free:
//!
//! 1. **Shared output adapter** (`parse_tdma_output`): a single entry-point that
//!    parses the worker's raw output into the pinned `tdma-state-update/v1`
//!    header + body, so every arm (Nesbitt / Swebench / future market agents)
//!    consumes output through ONE parser (parity). Wraps the existing
//!    `state_update::parse_prefix_json` + `tdma_runner::extract_body`; it adds no
//!    new schema discriminant (M7 only wraps — changing `StateStatus` /
//!    `StateUpdate` would be a Class-4 wire-schema change → STOP + re-§8).
//!
//! 2. **Deterministic diff materializer** (`parse_search_replace_blocks` +
//!    `apply_blocks` + `synthesize_unified_diff`): takes the model out of the
//!    hunk-arithmetic business. The worker emits Aider-style SEARCH/REPLACE
//!    blocks; we apply them to the (pre-fix) target-file content and synthesize a
//!    correct unified diff DETERMINISTICALLY via libgit2 (`git2::Patch`), whose
//!    output is git-apply compatible by construction. This is the apply-gate fix
//!    for a flash worker that is weak at unified-diff line counts. Raw-diff
//!    remains a first-class fallback in `swebench_test_judge::extract_patch`.
//!
//! Charter: handover/tracer_bullets/TB-MARKET-ACTIVATION-G0_charter_2026-05-29.md §3.1
//! Design:  handover/SWEBENCH_MARKET_ACTIVATION_BENCHMARK_v4_2026-05-29.md
//!
//! No §6 restricted surface; no §8 required (Class 1, additive wrap).

use std::fmt;
use std::path::Path;

use crate::state_update::{parse_prefix_json, HeaderParseError, StateUpdate};
use crate::tdma_runner::{extract_body, sha256_hex};
use crate::token_budget::{B_HEADER, B_HEADER_SCAN};

/// Single pinned schema version (mirrors `state_update.rs` doc-pin at line 31 +
/// the runtime check in `validate_state_update_schema`). A version-drift guard.
pub const SCHEMA_VERSION: &str = "tdma-state-update/v1";

// ── 1. Shared output adapter ─────────────────────────────────────────

/// The parsed worker output: pinned header + body + content hash.
/// `body_sha256` lets the caller anchor the consumed body on tape/CAS (Art. 0.2).
#[derive(Debug, Clone)]
pub struct ParsedOutput {
    pub header: StateUpdate,
    pub body: String,
    pub body_sha256: String,
    pub schema_version: &'static str,
}

/// Parse a worker's raw output into header + body through the ONE shared path.
///
/// Pure function — no side effects, does NOT touch any judge attempt counter
/// (FC1 attempt-count invariant: `verdict` is still called exactly once per LLM
/// cycle; this adapter never changes that count).
///
/// Schema pin is enforced by `parse_prefix_json` → `validate_state_update_schema`:
/// any non-`tdma-state-update/v1` header returns `HeaderParseError::SchemaInvalid`
/// before the body is consumed.
pub fn parse_tdma_output(raw: &str) -> Result<ParsedOutput, HeaderParseError> {
    let header = parse_prefix_json(raw, B_HEADER_SCAN, B_HEADER)?;
    let body = extract_body(raw);
    let body_sha256 = sha256_hex(body.as_bytes());
    Ok(ParsedOutput {
        header,
        body,
        body_sha256,
        schema_version: SCHEMA_VERSION,
    })
}

// ── 2. Deterministic diff materializer ───────────────────────────────

/// One Aider-style edit: replace `search` with `replace` inside file `path`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchReplaceBlock {
    pub path: String,
    pub search: String,
    pub replace: String,
}

/// Materialization failures. None of these advance any state; the caller routes
/// them as a shielded retry signal ("SEARCH block not found" etc.), never as a
/// hidden-test leak.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterializeError {
    /// No well-formed SEARCH/REPLACE block parsed from the body.
    NoBlocks,
    /// The SEARCH text does not occur in the target file content.
    SearchNotFound { path: String },
    /// The SEARCH text occurs more than once (ambiguous; refuse rather than guess).
    AmbiguousMatch { path: String, count: usize },
    /// libgit2 failed to synthesize the unified diff.
    DiffSynthesis(String),
}

impl fmt::Display for MaterializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterializeError::NoBlocks => write!(f, "no SEARCH/REPLACE block found in body"),
            MaterializeError::SearchNotFound { path } => {
                write!(f, "SEARCH block not found in file {}", path)
            }
            MaterializeError::AmbiguousMatch { path, count } => {
                write!(f, "SEARCH block matched {} times in file {} (ambiguous)", count, path)
            }
            MaterializeError::DiffSynthesis(e) => write!(f, "diff synthesis failed: {}", e),
        }
    }
}

impl std::error::Error for MaterializeError {}

const FENCE_SEARCH: &str = "<<<<<<< SEARCH";
const FENCE_DIVIDER: &str = "=======";
const FENCE_REPLACE: &str = ">>>>>>> REPLACE";

/// Parse Aider-style SEARCH/REPLACE blocks out of a body. Format:
///
/// ```text
/// path/to/file.py
/// <<<<<<< SEARCH
/// old line(s)
/// =======
/// new line(s)
/// >>>>>>> REPLACE
/// ```
///
/// The file path is the last non-empty, non-fence line before the opening
/// `<<<<<<< SEARCH`. Multiple blocks are supported. Robust to surrounding prose.
pub fn parse_search_replace_blocks(body: &str) -> Vec<SearchReplaceBlock> {
    #[derive(PartialEq)]
    enum Mode {
        Idle,
        InSearch,
        InReplace,
    }
    let mut mode = Mode::Idle;
    let mut candidate_path = String::new();
    let mut path = String::new();
    let mut search: Vec<&str> = Vec::new();
    let mut replace: Vec<&str> = Vec::new();
    let mut out: Vec<SearchReplaceBlock> = Vec::new();

    for line in body.lines() {
        let trimmed = line.trim();
        match mode {
            Mode::Idle => {
                if trimmed.starts_with(FENCE_SEARCH) {
                    path = candidate_path.clone();
                    search.clear();
                    mode = Mode::InSearch;
                } else if !trimmed.is_empty() && !trimmed.starts_with("```") {
                    candidate_path = trimmed.to_string();
                }
            }
            Mode::InSearch => {
                if trimmed == FENCE_DIVIDER {
                    replace.clear();
                    mode = Mode::InReplace;
                } else {
                    search.push(line);
                }
            }
            Mode::InReplace => {
                if trimmed.starts_with(FENCE_REPLACE) {
                    out.push(SearchReplaceBlock {
                        path: path.clone(),
                        search: search.join("\n"),
                        replace: replace.join("\n"),
                    });
                    candidate_path.clear();
                    mode = Mode::Idle;
                } else {
                    replace.push(line);
                }
            }
        }
    }
    out
}

/// Apply SEARCH/REPLACE blocks to `original` content. Each block's SEARCH text
/// must occur EXACTLY ONCE (else error — refuse to guess). Returns edited content.
pub fn apply_blocks(
    original: &str,
    blocks: &[SearchReplaceBlock],
) -> Result<String, MaterializeError> {
    if blocks.is_empty() {
        return Err(MaterializeError::NoBlocks);
    }
    let mut current = original.to_string();
    for b in blocks {
        if b.search.is_empty() {
            // Empty SEARCH (whole-file create/replace) is out of G0 scope; refuse.
            return Err(MaterializeError::SearchNotFound {
                path: b.path.clone(),
            });
        }
        let count = current.matches(&b.search).count();
        if count == 0 {
            return Err(MaterializeError::SearchNotFound {
                path: b.path.clone(),
            });
        }
        if count > 1 {
            return Err(MaterializeError::AmbiguousMatch {
                path: b.path.clone(),
                count,
            });
        }
        current = current.replacen(&b.search, &b.replace, 1);
    }
    Ok(current)
}

/// Synthesize a git-apply-compatible unified diff between `original` and `edited`
/// for `path`, DETERMINISTICALLY via libgit2. Empty string if no change.
pub fn synthesize_unified_diff(
    path: &str,
    original: &str,
    edited: &str,
) -> Result<String, MaterializeError> {
    if original == edited {
        return Ok(String::new());
    }
    let p = Path::new(path);
    let mut patch = git2::Patch::from_buffers(
        original.as_bytes(),
        Some(p),
        edited.as_bytes(),
        Some(p),
        None,
    )
    .map_err(|e| MaterializeError::DiffSynthesis(e.to_string()))?;
    let buf = patch
        .to_buf()
        .map_err(|e| MaterializeError::DiffSynthesis(e.to_string()))?;
    let s = buf
        .as_str()
        .ok_or_else(|| MaterializeError::DiffSynthesis("diff buffer not valid UTF-8".into()))?;
    Ok(s.to_string())
}

/// End-to-end materializer: parse blocks, apply to the target file's original
/// content, synthesize the deterministic unified diff. The single function the
/// judge calls when the worker emits SEARCH/REPLACE instead of a raw diff.
pub fn materialize_patch(
    path: &str,
    original: &str,
    body: &str,
) -> Result<String, MaterializeError> {
    let blocks = parse_search_replace_blocks(body);
    let edited = apply_blocks(original, &blocks)?;
    synthesize_unified_diff(path, original, &edited)
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_with(header: &str, body: &str) -> String {
        format!("{}\n---BODY---\n{}", header, body)
    }

    const VALID_HEADER: &str = r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"g0-flask","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}"#;

    #[test]
    fn test_version_hash_constant() {
        // Version-drift regression guard.
        assert_eq!(SCHEMA_VERSION, "tdma-state-update/v1");
    }

    #[test]
    fn test_parse_tdma_output_roundtrip() {
        let body = "some patch body";
        let raw = raw_with(VALID_HEADER, body);
        let parsed = parse_tdma_output(&raw).expect("must parse");
        assert_eq!(parsed.schema_version, "tdma-state-update/v1");
        assert_eq!(parsed.body, body);
        assert!(!parsed.body.is_empty());
        assert_eq!(parsed.body_sha256, sha256_hex(body.as_bytes()));
        assert_eq!(parsed.header.task_id, "g0-flask");
    }

    #[test]
    fn test_parse_tdma_output_schema_mismatch_errors() {
        let v2_header = r#"{"schema_version":"tdma-state-update/v2","status":"Proceed","task_id":"t","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}"#;
        let raw = raw_with(v2_header, "body");
        let err = parse_tdma_output(&raw).unwrap_err();
        assert!(matches!(err, HeaderParseError::SchemaInvalid(_)));
    }

    #[test]
    fn test_parse_search_replace_blocks() {
        let body = "src/foo.py\n<<<<<<< SEARCH\n    return 1\n=======\n    return 2\n>>>>>>> REPLACE\n";
        let blocks = parse_search_replace_blocks(body);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].path, "src/foo.py");
        assert_eq!(blocks[0].search, "    return 1");
        assert_eq!(blocks[0].replace, "    return 2");
    }

    #[test]
    fn test_parse_multiple_blocks() {
        let body = "a.py\n<<<<<<< SEARCH\nx\n=======\ny\n>>>>>>> REPLACE\nb.py\n<<<<<<< SEARCH\nm\n=======\nn\n>>>>>>> REPLACE\n";
        let blocks = parse_search_replace_blocks(body);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].path, "a.py");
        assert_eq!(blocks[1].path, "b.py");
        assert_eq!(blocks[1].search, "m");
        assert_eq!(blocks[1].replace, "n");
    }

    #[test]
    fn test_apply_blocks_replaces_once() {
        let original = "def f():\n    return 1\n";
        let blocks = vec![SearchReplaceBlock {
            path: "f.py".into(),
            search: "    return 1".into(),
            replace: "    return 2".into(),
        }];
        let edited = apply_blocks(original, &blocks).expect("apply ok");
        assert_eq!(edited, "def f():\n    return 2\n");
    }

    #[test]
    fn test_apply_blocks_search_not_found() {
        let original = "def f():\n    return 1\n";
        let blocks = vec![SearchReplaceBlock {
            path: "f.py".into(),
            search: "    return 99".into(),
            replace: "    return 2".into(),
        }];
        let err = apply_blocks(original, &blocks).unwrap_err();
        assert!(matches!(err, MaterializeError::SearchNotFound { .. }));
    }

    #[test]
    fn test_apply_blocks_ambiguous() {
        let original = "x\nx\n";
        let blocks = vec![SearchReplaceBlock {
            path: "f.py".into(),
            search: "x".into(),
            replace: "y".into(),
        }];
        let err = apply_blocks(original, &blocks).unwrap_err();
        assert!(matches!(err, MaterializeError::AmbiguousMatch { count: 2, .. }));
    }

    #[test]
    fn test_apply_blocks_empty_errors() {
        let err = apply_blocks("anything", &[]).unwrap_err();
        assert_eq!(err, MaterializeError::NoBlocks);
    }

    #[test]
    fn test_synthesize_unified_diff_structure() {
        let original = "line1\nline2\nline3\n";
        let edited = "line1\nCHANGED\nline3\n";
        let diff = synthesize_unified_diff("src/foo.py", original, edited).expect("diff ok");
        assert!(diff.contains("@@"), "diff must have a hunk header: {}", diff);
        assert!(diff.contains("-line2"), "diff must remove line2: {}", diff);
        assert!(diff.contains("+CHANGED"), "diff must add CHANGED: {}", diff);
        // git-apply -p1 compatibility: a/ and b/ path prefixes.
        assert!(diff.contains("foo.py"), "diff must name the file: {}", diff);
    }

    #[test]
    fn test_synthesize_no_change_is_empty() {
        let diff = synthesize_unified_diff("f.py", "same\n", "same\n").expect("ok");
        assert!(diff.is_empty());
    }

    #[test]
    fn test_materialize_patch_end_to_end() {
        let original = "def buggy():\n    return None\n";
        let body = "src/mod.py\n<<<<<<< SEARCH\n    return None\n=======\n    return []\n>>>>>>> REPLACE\n";
        let diff = materialize_patch("src/mod.py", original, body).expect("materialize ok");
        assert!(diff.contains("@@"));
        assert!(diff.contains("-    return None"));
        assert!(diff.contains("+    return []"));
    }
}
