//! TRACE_MATRIX FC1a-judge_pi: SWE-bench hidden-test JudgeAI.
//!
//! Verdict authority for the `turingos tdma run --judge swebench` loop. Each
//! candidate body is a model-produced unified-diff patch for a SWE-bench
//! coding-repair instance. The judge runs the REAL official `swebench` python
//! harness (hidden-test execution) on the candidate patch and returns:
//!
//!   * `Pass`  — the per-instance report marks the instance `resolved == true`.
//!   * `Fail`  — hidden FAIL_TO_PASS tests still failing; the reason carries
//!               only the failing test NAMES + counts (never gold/test patch
//!               contents — shielding per AGENTS.md §12 / CLAUDE.md §4).
//!
//! The loop feeds the `Fail` reason back into the next attempt's retry prompt
//! so the model can try again with the real failing-test signal.
//!
//! Single-stage by design: one stage `Repair` (one structural parallel to
//! `generate_judge::GenerateStage::Compile`). Multi-step here means multiple
//! loop ATTEMPTS within the single Repair stage, not multiple proof stages.
//!
//! No new crates: only `serde_json` + `std` + the in-repo sanitized runner.

use std::cell::Cell;
use std::path::PathBuf;
use std::time::Duration;

use crate::judges::math_step_judge::{JudgeVerdict, MathStepJudge};
use crate::sdk::sanitized_runner::{env_allowlist_from_current, run_sanitized, SanitizedCommand};

/// TRACE_MATRIX FC1a-judge_pi: Single-stage cursor for swebench repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwebenchStage {
    Repair,
}

impl SwebenchStage {
    /// TRACE_MATRIX FC1a-judge_pi: Human-readable stage label.
    pub fn label(self) -> &'static str {
        "Repair"
    }
}

/// TRACE_MATRIX FC1a-judge_pi: Real hidden-test verifier for one SWE-bench
/// instance. State is the per-attempt counter (interior-mutable so a `&self`
/// `verdict` call can bump it).
pub struct SwebenchTestJudge {
    pub instance_id: String,
    pub dataset_name: String,
    pub python_bin: PathBuf,
    pub work_dir: PathBuf,
    pub model_name: String,
    pub attempt: Cell<usize>,
}

impl SwebenchTestJudge {
    /// TRACE_MATRIX FC1a-judge_pi: Constructor (attempt counter starts at 0).
    pub fn new(
        instance_id: String,
        dataset_name: String,
        python_bin: PathBuf,
        work_dir: PathBuf,
        model_name: String,
    ) -> Self {
        Self {
            instance_id,
            dataset_name,
            python_bin,
            work_dir,
            model_name,
            attempt: Cell::new(0),
        }
    }

    /// TRACE_MATRIX FC1a-output_edge: Extract a unified-diff patch from the
    /// model's raw output. Accepts three shapes, in order:
    ///   (a) a JSON object with a `"patch"` (or `"diff"`) string field,
    ///   (b) a ```diff / ``` fenced block,
    ///   (c) a raw string already starting with `diff --git` / `---`.
    fn extract_patch(body: &str) -> Option<String> {
        // (a) JSON object with a `patch`/`diff` field.
        if let Some(value) = extract_json_object(body) {
            if let Some(patch) = value
                .get("patch")
                .or_else(|| value.get("diff"))
                .and_then(serde_json::Value::as_str)
            {
                let trimmed = patch.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }

        // (b) fenced block: ```diff ... ``` or a bare ``` ... ``` that holds a diff.
        if let Some(fenced) = extract_fenced_diff(body) {
            return Some(fenced);
        }

        // (c) raw unified diff.
        let trimmed = body.trim();
        if trimmed.starts_with("diff --git") || trimmed.starts_with("--- ") {
            return Some(trimmed.to_string());
        }

        None
    }
}

/// TRACE_MATRIX FC1a-output_edge: Lenient JSON-object extraction (mirrors the
/// swebench binary's `extract_json_object`): try whole string, else slice
/// between the first `{` and last `}`.
fn extract_json_object(content: &str) -> Option<serde_json::Value> {
    let trimmed = content.trim();
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Some(value);
    }
    let start = trimmed.find('{')?;
    let end = trimmed.rfind('}')?;
    if end <= start {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(&trimmed[start..=end]).ok()
}

/// TRACE_MATRIX FC1a-output_edge: Pull the contents of the first fenced code
/// block that looks like a unified diff. Handles ```diff, ```patch, or a bare
/// ``` fence whose body starts with `diff --git` / `---`.
fn extract_fenced_diff(body: &str) -> Option<String> {
    let mut lines = body.lines();
    let mut collecting = false;
    let mut buf: Vec<&str> = Vec::new();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if !collecting {
            if let Some(rest) = trimmed.strip_prefix("```") {
                let lang = rest.trim().to_ascii_lowercase();
                // Open a fence; we will validate its body looks like a diff.
                let _ = lang;
                collecting = true;
                buf.clear();
            }
        } else if trimmed.starts_with("```") {
            // Close fence.
            let candidate = buf.join("\n");
            let c = candidate.trim();
            if c.starts_with("diff --git") || c.starts_with("--- ") {
                return Some(c.to_string());
            }
            // Not a diff; keep scanning for another fence.
            collecting = false;
            buf.clear();
        } else {
            buf.push(line);
        }
    }
    None
}

impl MathStepJudge for SwebenchTestJudge {
    fn verdict(&self, _prior_steps: &[String], candidate_step: &str) -> JudgeVerdict {
        // 1. Extract the unified-diff patch.
        let patch = match Self::extract_patch(candidate_step) {
            Some(p) => p,
            None => {
                return JudgeVerdict::Fail {
                    reason: "model output contained no unified diff".into(),
                };
            }
        };

        // 2. Per-attempt bookkeeping.
        let n = self.attempt.get();
        self.attempt.set(n + 1);
        let run_id = format!("swebloop_{}_{}", std::process::id(), n);

        // 3. Write the single-line predictions file (serde_json handles escaping).
        let preds_path = self.work_dir.join(format!("preds_{}.jsonl", n));
        let pred = serde_json::json!({
            "instance_id": self.instance_id,
            "model_name_or_path": self.model_name,
            "model_patch": patch,
        });
        let pred_line = format!("{}\n", pred);
        if let Err(e) = std::fs::create_dir_all(&self.work_dir) {
            return JudgeVerdict::Fail {
                reason: format!("swebench harness error: cannot create work_dir: {}", e),
            };
        }
        if let Err(e) = std::fs::write(&preds_path, pred_line.as_bytes()) {
            return JudgeVerdict::Fail {
                reason: format!("swebench harness error: cannot write predictions: {}", e),
            };
        }
        let preds_abs = std::fs::canonicalize(&preds_path).unwrap_or(preds_path);

        // 4. Shell out to the official swebench harness via the sanitized runner.
        let cmd = SanitizedCommand {
            program: self.python_bin.clone(),
            args: vec![
                "-m".into(),
                "swebench.harness.run_evaluation".into(),
                "--dataset_name".into(),
                self.dataset_name.clone(),
                "--predictions_path".into(),
                preds_abs.to_string_lossy().into_owned(),
                "--instance_ids".into(),
                self.instance_id.clone(),
                "--run_id".into(),
                run_id.clone(),
                "--namespace".into(),
                "none".into(),
                "--max_workers".into(),
                "1".into(),
                "--cache_level".into(),
                "instance".into(),
            ],
            cwd: self.work_dir.clone(),
            // Hermetic verification env. The sanitized runner strips proxy vars
            // (HTTP(S)_PROXY), so an ONLINE HuggingFace dataset fetch 404s and the
            // hidden tests never run. Forcing HF offline makes the harness load the
            // locally-cached dataset (spec + hidden test_patch) with zero network,
            // keeping the verifier deterministic and answer-independent. NOTE:
            // instance/base Docker images must be pre-built (e.g. a gold smoke with
            // full env) before the loop — image builds need network this env lacks.
            env: {
                let mut env = env_allowlist_from_current(&[
                    "PATH", "HOME", "USER", "LANG", "DOCKER_HOST", "TMPDIR",
                ]);
                env.insert("HF_HUB_OFFLINE".to_string(), "1".to_string());
                env.insert("HF_DATASETS_OFFLINE".to_string(), "1".to_string());
                env
            },
            stdin: None,
            timeout: Duration::from_secs(60 * 60),
        };

        let output = match run_sanitized(cmd) {
            Ok(o) => o,
            Err(e) => {
                return JudgeVerdict::Fail {
                    reason: format!("swebench harness error (spawn failed): {}", e),
                };
            }
        };

        // 5. Read the per-instance report.
        let run_dir = self
            .work_dir
            .join("logs")
            .join("run_evaluation")
            .join(&run_id)
            .join(&self.model_name)
            .join(&self.instance_id);
        let report_path = run_dir.join("report.json");

        let report_str = match std::fs::read_to_string(&report_path) {
            Ok(s) => s,
            Err(_) => {
                // 6. Harness produced no report.json. The most common cause is a
                // model patch that does not apply (bad @@ hunk counts, fabricated
                // index hashes) — the harness errors the instance instead of
                // running the hidden tests. Surface the ACTUAL apply error from
                // run_instance.log as actionable retry signal (the patch is the
                // model's own output, so this leaks no gold/test content). The raw
                // stderr tail is only the offline-cache notice and is useless here.
                let reason = harness_failure_reason(
                    &run_dir.join("run_instance.log"),
                    &output.stderr,
                    output.exit_code,
                );
                return JudgeVerdict::Fail {
                    reason: truncate_chars(&reason, 600),
                };
            }
        };

        let report: serde_json::Value = match serde_json::from_str(&report_str) {
            Ok(v) => v,
            Err(e) => {
                return JudgeVerdict::Fail {
                    reason: format!("swebench report.json parse error: {}", e),
                };
            }
        };

        let inst = &report[self.instance_id.as_str()];
        let resolved = inst.get("resolved").and_then(serde_json::Value::as_bool);

        if resolved == Some(true) {
            return JudgeVerdict::Pass;
        }

        // Collect the still-failing FAIL_TO_PASS test NAMES only (shielding).
        let names: Vec<String> = inst
            .get("tests_status")
            .and_then(|ts| ts.get("FAIL_TO_PASS"))
            .and_then(|ftp| ftp.get("failure"))
            .and_then(serde_json::Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(ToString::to_string)
                    .collect()
            })
            .unwrap_or_default();

        let reason = format!(
            "hidden tests still failing — {} FAIL_TO_PASS unresolved: {}",
            names.len(),
            names.join(", ")
        );
        JudgeVerdict::Fail {
            reason: truncate_chars(&reason, 600),
        }
    }
}

/// TRACE_MATRIX FC1a-judge_pi: Build an actionable, shielded retry reason when
/// the harness produced no `report.json` (it errored the instance). Prefers the
/// patch-apply failure from `run_instance.log` (about the model's own patch, so
/// no gold/test leakage); falls back to the raw stderr tail.
fn harness_failure_reason(log_path: &std::path::Path, stderr: &[u8], exit_code: Option<i32>) -> String {
    if let Ok(log) = std::fs::read_to_string(log_path) {
        if log.contains("Patch Apply Failed") || log.contains("malformed patch") {
            // Extract the most specific `patch:`/`malformed` line for the model.
            let detail = log
                .lines()
                .find(|l| l.contains("malformed patch") || l.trim_start().starts_with("patch:"))
                .map(str::trim)
                .unwrap_or("patch could not be applied");
            return format!(
                "your unified diff FAILED TO APPLY, so the hidden tests never ran ({detail}). \
                 Re-emit a corrected git diff: ensure each @@ hunk header's line counts match \
                 the body exactly, keep unchanged context lines, and base it on the stated commit."
            );
        }
    }
    let tail = tail_chars(&String::from_utf8_lossy(stderr), 400);
    format!("swebench harness error (exit {:?}); stderr tail: {}", exit_code, tail)
}

/// TRACE_MATRIX FC1a-output_edge: char-safe truncation to at most `max` chars.
fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    s.chars().take(max).collect()
}

/// TRACE_MATRIX FC1a-output_edge: last `max` chars of a string (char-safe).
fn tail_chars(s: &str, max: usize) -> String {
    let total = s.chars().count();
    if total <= max {
        return s.to_string();
    }
    s.chars().skip(total - max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn judge() -> SwebenchTestJudge {
        SwebenchTestJudge::new(
            "astropy__astropy-12345".to_string(),
            "princeton-nlp/SWE-bench_Lite".to_string(),
            PathBuf::from("/usr/bin/python3"),
            std::env::temp_dir(),
            "turingos-loop".to_string(),
        )
    }

    #[test]
    fn stage_label_is_repair() {
        assert_eq!(SwebenchStage::Repair.label(), "Repair");
    }

    #[test]
    fn extract_patch_from_json_field() {
        let body = r#"{"patch":"diff --git a/x.py b/x.py\n--- a/x.py\n+++ b/x.py\n@@\n-old\n+new\n","rationale":"fix"}"#;
        let p = SwebenchTestJudge::extract_patch(body).expect("patch");
        assert!(p.starts_with("diff --git a/x.py"));
        assert!(p.contains("+new"));
    }

    #[test]
    fn extract_patch_from_diff_fence() {
        let body = "Here is the fix:\n```diff\ndiff --git a/x.py b/x.py\n--- a/x.py\n+++ b/x.py\n@@\n-old\n+new\n```\nDone.";
        let p = SwebenchTestJudge::extract_patch(body).expect("patch");
        assert!(p.starts_with("diff --git a/x.py"));
    }

    #[test]
    fn extract_patch_from_raw_diff() {
        let body = "diff --git a/x.py b/x.py\n--- a/x.py\n+++ b/x.py\n@@\n-old\n+new\n";
        let p = SwebenchTestJudge::extract_patch(body).expect("patch");
        assert!(p.starts_with("diff --git a/x.py"));
    }

    #[test]
    fn extract_patch_none_for_prose() {
        assert!(SwebenchTestJudge::extract_patch("I cannot produce a patch.").is_none());
    }

    #[test]
    fn verdict_no_diff_fails_without_running_harness() {
        let j = judge();
        let v = j.verdict(&[], "Sorry, no patch here.");
        match v {
            JudgeVerdict::Fail { reason } => {
                assert!(reason.contains("no unified diff"), "got {}", reason)
            }
            other => panic!("expected Fail, got {:?}", other),
        }
        // attempt counter must NOT advance when there is nothing to verify.
        assert_eq!(j.attempt.get(), 0);
    }

    #[test]
    fn truncate_and_tail_are_char_safe() {
        assert_eq!(truncate_chars("abcdef", 3), "abc");
        assert_eq!(truncate_chars("ab", 5), "ab");
        assert_eq!(tail_chars("abcdef", 3), "def");
        assert_eq!(tail_chars("ab", 5), "ab");
    }
}
