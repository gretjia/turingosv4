//! A2 atom (Phase 6.3.y) — golden-fixture A/B test for triage v1 vs v2.
//!
//! Verifies that `prompt-eval` on the starter fixture's M8 gibberish rows
//! against the v2 triage prompt would catch the M8 regression (one of the
//! M8 rows MUST be tagged as a `m8_regression` negative control). The real
//! v1-vs-v2 LLM A/B is gated behind `#[ignore]` because it requires a live
//! SiliconFlow API key; the un-ignored tests assert the fixture/prompt-asset
//! invariants that make that A/B meaningful.
//!
//! TRACE_MATRIX FC2-N16 A2: prompt-eval v1-vs-v2 triage regression contract.

use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn bin_path() -> PathBuf {
    let mut p = manifest_dir();
    p.push("target/debug/turingos");
    p
}

#[test]
fn triage_v1_and_v2_prompt_assets_both_exist() {
    let v1 = manifest_dir().join("assets/prompts/grill_triage_blackbox_v1.md");
    let v2 = manifest_dir().join("assets/prompts/grill_triage_blackbox_v2.md");
    assert!(
        v1.exists(),
        "triage v1 prompt asset missing at {}",
        v1.display()
    );
    assert!(
        v2.exists(),
        "triage v2 prompt asset missing at {}",
        v2.display()
    );
}

#[test]
fn fixture_contains_m8_regression_negative_controls() {
    // The fixture MUST contain gibberish rows tagged `m8_regression` — these
    // are the negative controls that any candidate triage prompt must NOT
    // mis-classify as `relevant`. Without these rows, prompt-eval cannot
    // catch the M8-class non-local-effect failure F8 introduced.
    let fixture_path = manifest_dir().join("tests/fixtures/grill_prompt_eval_fixture.jsonl");
    let content =
        std::fs::read_to_string(&fixture_path).unwrap_or_else(|e| panic!("read fixture: {e}"));
    let mut found_m8_gibberish = 0usize;
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
        let expected_class = v["expected_class"].as_str().unwrap_or("");
        if tags.iter().any(|t| t == "m8_regression")
            && tags.iter().any(|t| t == "gibberish")
            && expected_class == "gibberish"
        {
            found_m8_gibberish += 1;
        }
    }
    assert!(
        found_m8_gibberish >= 2,
        "fixture must contain ≥2 M8 gibberish negative-control rows; found {}",
        found_m8_gibberish
    );
}

#[test]
fn fixture_contains_f8_register_positive_controls() {
    // The fixture MUST also contain register-tolerance rows that v1 fails
    // and v2 passes — without these, the v1-vs-v2 baseline-delta cannot
    // demonstrate the F8 win that motivated v2 in the first place.
    let fixture_path = manifest_dir().join("tests/fixtures/grill_prompt_eval_fixture.jsonl");
    let content = std::fs::read_to_string(&fixture_path).expect("read fixture");
    let mut found_register_relevant = 0usize;
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
        let expected_class = v["expected_class"].as_str().unwrap_or("");
        if tags.iter().any(|t| t == "register")
            && tags.iter().any(|t| t == "f8_win")
            && expected_class == "relevant"
        {
            found_register_relevant += 1;
        }
    }
    assert!(
        found_register_relevant >= 2,
        "fixture must contain ≥2 F8 register positive-control rows; found {}",
        found_register_relevant
    );
}

#[test]
fn prompt_eval_against_triage_v2_smoke_args_only() {
    // Smoke test that the CLI accepts the canonical invocation shape for the
    // v2 triage A/B run, even if the LLM call itself can't be made in CI.
    // We point at a workspace that does NOT have a configured API key so the
    // call exits at the require_api_key step (exit code 2), but the args
    // parser must accept everything cleanly first.
    let workspace = tempfile::tempdir().expect("tempdir");
    let prompt_v2 = manifest_dir().join("assets/prompts/grill_triage_blackbox_v2.md");
    let fixture = manifest_dir().join("tests/fixtures/grill_prompt_eval_fixture.jsonl");
    let output = Command::new(bin_path())
        .env_remove("SILICONFLOW_API_KEY")
        .arg("llm")
        .arg("prompt-eval")
        .arg("--workspace")
        .arg(workspace.path())
        .arg("--prompt-file")
        .arg(&prompt_v2)
        .arg("--role")
        .arg("blackbox")
        .arg("--fixture")
        .arg(&fixture)
        .output()
        .expect("failed to spawn");
    // Exit MUST NOT be 5 (args error). 2 = http (no api key), 1 = some
    // rows failed, 0 = all passed (unlikely without API). Any of {0,1,2}
    // is acceptable; 5 means our CLI surface is broken.
    let code = output.status.code().unwrap_or(-1);
    assert_ne!(
        code,
        5,
        "prompt-eval CLI rejected canonical v2 invocation as args error; \
         stdout={}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
#[ignore = "live LLM A/B; requires SILICONFLOW_API_KEY + network. Run manually \
            with: SILICONFLOW_API_KEY=... cargo test --test cmd_llm_prompt_eval_v1_vs_v2_triage \
            -- --ignored --nocapture"]
fn prompt_eval_v2_catches_m8_gibberish_regression() {
    // The headline A/B: run prompt-eval on the M8 fixture rows against v2.
    // If v2 has the F8 register-tolerance fix but breaks gibberish detection
    // (which it does, per the universality campaign findings), at least one
    // of the M8 gibberish rows will fail. This test asserts that the eval
    // mechanism correctly *reports* that failure (exit 1, fail_ids non-empty,
    // and at least one of the failing ids carries the m8_regression tag).
    let workspace = tempfile::tempdir().expect("tempdir");
    let prompt_v2 = manifest_dir().join("assets/prompts/grill_triage_blackbox_v2.md");
    let fixture = manifest_dir().join("tests/fixtures/grill_prompt_eval_fixture.jsonl");
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("prompt-eval")
        .arg("--workspace")
        .arg(workspace.path())
        .arg("--prompt-file")
        .arg(&prompt_v2)
        .arg("--role")
        .arg("blackbox")
        .arg("--fixture")
        .arg(&fixture)
        .output()
        .expect("failed to spawn");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("stdout not valid JSON: {e}; stdout={stdout}"));
    let pass = v["pass"].as_u64().expect("pass count");
    let fail = v["fail"].as_u64().expect("fail count");
    let total = v["total"].as_u64().expect("total count");
    assert_eq!(pass + fail + v["error"].as_u64().unwrap_or(0), total);
    let fail_ids: Vec<String> = v["fail_ids"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    // If v2 truly regressed M8, ≥1 m8_regression-tagged row should fail.
    // (If a future v3 fixes both, this assertion can be flipped to all-pass.)
    eprintln!("v2 fail_ids: {fail_ids:?}");
    let any_m8_failed = fail_ids
        .iter()
        .any(|id| id.contains("gibberish") || id.contains("s9") || id.contains("a9"));
    assert!(
        any_m8_failed,
        "expected at least one M8 gibberish row to fail on v2 triage; \
         got fail_ids={fail_ids:?}"
    );
}
