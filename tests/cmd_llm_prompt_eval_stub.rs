//! A2 atom (Phase 6.3.y) — `turingos llm prompt-eval` CLI surface contract tests.
//!
//! These tests do NOT make real LLM calls. They exercise the CLI argument
//! parsing, fixture loading, prompt loading, and JSON-shape error reporting.
//!
//! TRACE_MATRIX FC2-N16 A2: prompt-eval surface contract.

use std::process::Command;

fn bin_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target/debug/turingos");
    p
}

#[test]
fn help_lists_prompt_eval_action() {
    let output = Command::new(bin_path())
        .arg("llm")
        .output()
        .expect("failed to spawn");
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(
        combined.contains("prompt-eval"),
        "help should mention 'prompt-eval' action; got: {}",
        combined
    );
}

#[test]
fn prompt_eval_without_args_fails_args_exit5() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("prompt-eval")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "prompt-eval with no args should fail"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"ok\":false") || stdout.contains("\"ok\": false"),
        "stdout should be error JSON; got: {}",
        stdout
    );
    // Exit code 5 = args.
    assert_eq!(output.status.code(), Some(5), "expected exit 5 (args)");
}

#[test]
fn prompt_eval_missing_workspace_fails() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("prompt-eval")
        .arg("--prompt-file")
        .arg("/tmp/p.md")
        .arg("--role")
        .arg("meta")
        .arg("--fixture")
        .arg("/tmp/f.jsonl")
        .output()
        .expect("failed to spawn");
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout must be valid JSON");
    assert_eq!(v["ok"], serde_json::json!(false));
    assert_eq!(v["error"]["kind"], serde_json::json!("args"));
}

#[test]
fn prompt_eval_unknown_role_fails() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("prompt-eval")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--prompt-file")
        .arg("/tmp/p.md")
        .arg("--role")
        .arg("not_a_role")
        .arg("--fixture")
        .arg("/tmp/f.jsonl")
        .output()
        .expect("failed to spawn");
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("unknown --role"), "got: {}", stdout);
}

#[test]
fn prompt_eval_missing_fixture_file_fails_io_exit4() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("prompt-eval")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--prompt-file")
        .arg("/nonexistent/prompt.md")
        .arg("--role")
        .arg("blackbox")
        .arg("--fixture")
        .arg("/nonexistent/fixture.jsonl")
        .output()
        .expect("failed to spawn");
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout must be valid JSON");
    assert_eq!(v["error"]["kind"], serde_json::json!("io"));
    assert_eq!(output.status.code(), Some(4), "expected exit 4 (io)");
}

#[test]
fn prompt_eval_unknown_flag_fails() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("prompt-eval")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--this-flag-does-not-exist")
        .output()
        .expect("failed to spawn");
    assert!(!output.status.success());
}

#[test]
fn prompt_eval_invalid_lang_fails() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("prompt-eval")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--prompt-file")
        .arg("/tmp/p.md")
        .arg("--role")
        .arg("meta")
        .arg("--fixture")
        .arg("/tmp/f.jsonl")
        .arg("--lang")
        .arg("fr")
        .output()
        .expect("failed to spawn");
    assert!(!output.status.success());
}

#[test]
fn starter_fixture_parses_as_valid_jsonl() {
    // Verify the shipped starter fixture is well-formed JSONL.
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/grill_prompt_eval_fixture.jsonl");
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read fixture at {}: {e}", path.display()));
    let mut row_count = 0usize;
    let mut ids = std::collections::HashSet::new();
    for (lineno, raw) in content.lines().enumerate() {
        let t = raw.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(t)
            .unwrap_or_else(|e| panic!("fixture line {} is invalid JSON: {e}: {}", lineno + 1, t));
        let id = v["id"].as_str().expect("each row must have a string id");
        assert!(
            !id.is_empty(),
            "fixture row id must be non-empty (line {})",
            lineno + 1
        );
        assert!(ids.insert(id.to_string()), "duplicate fixture row id: {id}");
        row_count += 1;
    }
    assert!(
        row_count >= 8,
        "starter fixture must have at least 8 rows; got {}",
        row_count
    );
}

#[test]
fn starter_fixture_covers_required_tag_categories() {
    // The starter fixture must include rows in every category the A2 spec calls
    // out: mrs_chen (meta), register (triage f8_win), gibberish (m8_regression),
    // playback (no_hallucination).
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/grill_prompt_eval_fixture.jsonl");
    let content = std::fs::read_to_string(&path).expect("read fixture");
    let mut have_mrs_chen = false;
    let mut have_register = false;
    let mut have_gibberish = false;
    let mut have_playback = false;
    for raw in content.lines() {
        let t = raw.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(t).expect("valid JSONL");
        let tags: Vec<String> = v["tags"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        if tags.iter().any(|t| t == "mrs_chen") {
            have_mrs_chen = true;
        }
        if tags.iter().any(|t| t == "register") {
            have_register = true;
        }
        if tags.iter().any(|t| t == "gibberish") {
            have_gibberish = true;
        }
        if tags.iter().any(|t| t == "playback") {
            have_playback = true;
        }
    }
    assert!(have_mrs_chen, "fixture must include mrs_chen meta rows");
    assert!(have_register, "fixture must include register triage rows");
    assert!(
        have_gibberish,
        "fixture must include gibberish negative-control rows (M8 regression detector)"
    );
    assert!(
        have_playback,
        "fixture must include playback no-hallucination rows"
    );
}
