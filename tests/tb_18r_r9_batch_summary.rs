//! Regression test for the TB-18R R9 batch summary writer.
//!
//! FC trace: FC3 evidence feedback view. The summary is a materialized view,
//! but it must remain parseable so real-problem evidence can be audited from
//! per-problem ChainTape/CAS artifacts without hand repair.

use std::fs;
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

#[test]
fn r9_batch_summary_is_valid_json_and_does_not_parse_digits_from_field_names() {
    let tmp = TempDir::new().expect("tempdir");
    let out_dir = tmp.path();

    let problems_file = out_dir.join("r9_problems.txt");
    fs::write(
        &problems_file,
        "mathd_numbertheory_1124\nnumbertheory_2pownm1prime_nprime\n",
    )
    .expect("write problems");

    write_invariant(
        &out_dir.join("P01_mathd_numbertheory_1124"),
        r#"{
  "delta": -1,
  "expected_completed_attempts": 4,
  "invariant_verdict": "Err(r4 field name must not pollute numeric extraction)",
  "l4_work_attempt_count": 1,
  "l4e_work_attempt_count": 2,
  "r4_invariant_equation_evaluable": false
}"#,
    );
    write_invariant(
        &out_dir.join("P02_numbertheory_2pownm1prime_nprime"),
        r#"{
  "delta": 0,
  "expected_completed_attempts": 3,
  "invariant_verdict": "Ok",
  "l4_work_attempt_count": 3,
  "l4e_work_attempt_count": 0,
  "r4_invariant_equation_evaluable": true
}"#,
    );

    let status = Command::new("python3")
        .arg("handover/tests/scripts/r9_batch_summary.py")
        .arg("--out-dir")
        .arg(out_dir)
        .arg("--problems-file")
        .arg(&problems_file)
        .arg("--max-tx")
        .arg("12")
        .arg("--per-problem-timeout-s")
        .arg("1800")
        .arg("--git-head")
        .arg("abc123")
        .arg("--run-timestamp-utc")
        .arg("2026-05-17T00-00-00Z")
        .status()
        .expect("run summary writer");
    assert!(status.success(), "summary writer failed: {status}");

    let summary_path = out_dir.join("R9_BATCH_SUMMARY.json");
    let summary: Value =
        serde_json::from_str(&fs::read_to_string(summary_path).expect("read summary"))
            .expect("summary is valid json");
    assert_eq!(summary["problem_count"], 2);
    assert_eq!(summary["per_problem_results"][0]["l4"], 1);
    assert_eq!(summary["per_problem_results"][0]["l4e"], 2);
    assert_eq!(summary["per_problem_results"][0]["delta"], -1);
    assert_eq!(summary["per_problem_results"][0]["evaluable"], false);
    assert_eq!(
        summary["per_problem_results"][0]["invariant_verdict"],
        "Err(r4 field name must not pollute numeric extraction)"
    );
    assert_eq!(summary["per_problem_results"][1]["l4"], 3);
    assert_eq!(summary["per_problem_results"][1]["evaluable"], true);
}

fn write_invariant(dir: &std::path::Path, body: &str) {
    fs::create_dir_all(dir).expect("create problem dir");
    fs::write(dir.join("chain_invariant.json"), body).expect("write invariant");
}
