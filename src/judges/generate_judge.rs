//! TRACE_MATRIX FC1a-predicate_pi: Single-stage code-generation JudgeAI.
//!
//! Atom 19 of the TDMA-Generate + Phase E package. Verdicts whether an LLM
//! response is a usable artifact bundle for `turingos generate --tdma-bounded`.
//!
//! Single-stage by design: one LLM call produces one file bundle; the judge
//! decides Pass or Fail. Multi-stage generation (e.g. spec → scaffold → tests)
//! is a future extension; until then, this judge has one stage `Compile`.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md

use std::cell::Cell;

use crate::judges::math_step_judge::JudgeVerdict;

/// TRACE_MATRIX FC1a-predicate_pi: Single-stage cursor for generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerateStage {
    Compile,
}

impl GenerateStage {
    /// TRACE_MATRIX FC1a-predicate_pi: Human-readable stage label.
    pub fn label(self) -> &'static str {
        match self {
            GenerateStage::Compile => "Compile",
        }
    }
}

/// TRACE_MATRIX FC1a-predicate_pi: Reject classes for generate output.
///
/// Mapped one-to-one to the existing `runtime::rejection_capsule::RejectClass`
/// at the cmd_generate.rs callsite when constructing a GenerateRejectionCapsule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateRejectClass {
    NoFilesParsed,
    MissingEntrypoint,
    PathTraversal,
    ParseFailed,
    HeuristicCompileFailed,
    OffStage,
}

impl GenerateRejectClass {
    /// TRACE_MATRIX FC1a-predicate_pi: Canonical reject_class string for tape.
    pub fn reject_class_str(self) -> &'static str {
        match self {
            GenerateRejectClass::NoFilesParsed => "no-files-parsed",
            GenerateRejectClass::MissingEntrypoint => "missing-entrypoint",
            GenerateRejectClass::PathTraversal => "path-traversal",
            GenerateRejectClass::ParseFailed => "parse-failed",
            GenerateRejectClass::HeuristicCompileFailed => "heuristic-compile-failed",
            GenerateRejectClass::OffStage => "off-stage",
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Canonical failed_predicate string for header.
    pub fn failed_predicate_str(self) -> &'static str {
        match self {
            GenerateRejectClass::NoFilesParsed => "bundle.has_at_least_one_file",
            GenerateRejectClass::MissingEntrypoint => "bundle.entrypoint_present",
            GenerateRejectClass::PathTraversal => "bundle.path_safe",
            GenerateRejectClass::ParseFailed => "bundle.parse_ok",
            GenerateRejectClass::HeuristicCompileFailed => "bundle.compiles",
            GenerateRejectClass::OffStage => "bundle.recognized_shape",
        }
    }
}

/// TRACE_MATRIX FC1a-predicate_pi: Single-stage generate-judge state.
pub struct GenerateJudge {
    current_stage: Cell<GenerateStage>,
    expected_entrypoint: String,
    enable_compile_check: bool,
}

#[derive(Debug, Clone)]
struct ParsedBundle {
    files: Vec<(String, String)>,
}

fn parse_file_fences(body: &str) -> Result<ParsedBundle, String> {
    let mut files: Vec<(String, String)> = Vec::new();
    let mut current_path: Option<String> = None;
    let mut in_fence = false;
    let mut fence_buf = String::new();

    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("### File:") {
            if in_fence {
                return Err("file marker appeared inside an open fence".into());
            }
            current_path = Some(rest.trim().to_string());
        } else if trimmed.starts_with("```") {
            if in_fence {
                if let Some(path) = current_path.take() {
                    files.push((path, std::mem::take(&mut fence_buf)));
                } else {
                    return Err("closing fence with no `### File:` header".into());
                }
                in_fence = false;
            } else {
                if current_path.is_none() {
                    continue;
                }
                in_fence = true;
                fence_buf.clear();
            }
        } else if in_fence {
            fence_buf.push_str(line);
            fence_buf.push('\n');
        }
    }
    if in_fence {
        return Err("unterminated fence in body".into());
    }
    Ok(ParsedBundle { files })
}

fn path_unsafe(path: &str) -> bool {
    if path.starts_with('/') {
        return true;
    }
    path.split(['/', '\\']).any(|seg| seg == ".." || seg.is_empty())
}

impl GenerateJudge {
    /// TRACE_MATRIX FC1a-predicate_pi: Constructor at stage Compile.
    pub fn new(expected_entrypoint: String, enable_compile_check: bool) -> Self {
        Self {
            current_stage: Cell::new(GenerateStage::Compile),
            expected_entrypoint,
            enable_compile_check,
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Promote stage after a successful step.
    /// Single-stage judge — first advance terminates the proof.
    pub fn advance(&self) -> bool {
        false
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Run verdict on candidate body.
    pub fn verdict_for_stage(
        &self,
        body: &str,
        _stage: GenerateStage,
        _accepted: &[String],
    ) -> (JudgeVerdict, Option<GenerateRejectClass>) {
        let trimmed = body.trim();
        if trimmed.is_empty() {
            return (
                JudgeVerdict::Fail {
                    reason: "empty body".into(),
                },
                Some(GenerateRejectClass::NoFilesParsed),
            );
        }

        let parsed = match parse_file_fences(body) {
            Ok(p) => p,
            Err(reason) => {
                return (
                    JudgeVerdict::Fail { reason },
                    Some(GenerateRejectClass::ParseFailed),
                );
            }
        };

        if parsed.files.is_empty() {
            return (
                JudgeVerdict::Fail {
                    reason: "no `### File:` blocks found in body".into(),
                },
                Some(GenerateRejectClass::NoFilesParsed),
            );
        }

        for (path, _content) in &parsed.files {
            if path_unsafe(path) {
                return (
                    JudgeVerdict::Fail {
                        reason: format!("unsafe path: {}", path),
                    },
                    Some(GenerateRejectClass::PathTraversal),
                );
            }
        }

        let entrypoint_present = parsed
            .files
            .iter()
            .any(|(p, _)| p == &self.expected_entrypoint);
        if !entrypoint_present {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "expected entrypoint `{}` not in parsed bundle",
                        self.expected_entrypoint
                    ),
                },
                Some(GenerateRejectClass::MissingEntrypoint),
            );
        }

        if self.enable_compile_check {
            // Atom 19 ships the heuristic-compile-check infrastructure but
            // not the actual compiler invocations. Future atoms wire rustc /
            // tsc / pyright dry-run as gated by the per-file extension.
            // For now, with the flag set, this is a no-op (always passes
            // when files are recognized). The reject class is reserved for
            // future use.
        }

        (JudgeVerdict::Pass, None)
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Inspect current stage cursor.
    pub fn current_stage(&self) -> GenerateStage {
        self.current_stage.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn judge() -> GenerateJudge {
        GenerateJudge::new("main.py".to_string(), false)
    }

    #[test]
    fn judge_passes_on_valid_bundle() {
        let body = r#"
Here is the generated bundle:

### File: main.py
```python
print("hello")
```

### File: helper.py
```python
def add(a, b): return a + b
```
"#;
        let (v, c) = judge().verdict_for_stage(body, GenerateStage::Compile, &[]);
        assert!(v.is_pass(), "{:?}", v);
        assert!(c.is_none());
    }

    #[test]
    fn judge_rejects_no_files() {
        let body = "I will not generate any files today.";
        let (v, c) = judge().verdict_for_stage(body, GenerateStage::Compile, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(GenerateRejectClass::NoFilesParsed));
    }

    #[test]
    fn judge_rejects_path_traversal() {
        let body = r#"
### File: ../../etc/passwd
```
exploit
```
"#;
        let (v, c) = judge().verdict_for_stage(body, GenerateStage::Compile, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(GenerateRejectClass::PathTraversal));
    }

    #[test]
    fn judge_rejects_parse_fail() {
        let body = r#"
### File: main.py
```python
print("hello")
"#;
        let (v, c) = judge().verdict_for_stage(body, GenerateStage::Compile, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(GenerateRejectClass::ParseFailed));
    }

    #[test]
    fn judge_rejects_missing_entrypoint() {
        let body = r#"
### File: helper.py
```python
def helper(): pass
```
"#;
        let (v, c) = judge().verdict_for_stage(body, GenerateStage::Compile, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(GenerateRejectClass::MissingEntrypoint));
    }

    #[test]
    fn judge_advance_terminates() {
        let j = judge();
        let progressed = j.advance();
        assert!(!progressed, "single-stage judge cannot advance");
        assert_eq!(j.current_stage(), GenerateStage::Compile);
    }
}
