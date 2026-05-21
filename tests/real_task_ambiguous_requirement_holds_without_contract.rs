use std::path::{Path, PathBuf};
use std::time::Duration;

use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand,
};

#[test]
fn real_task_ambiguous_requirement_holds_without_contract() {
    let requirement = "make the button feel more premium";

    assert_eq!(
        evaluate_requirement(requirement, None, None),
        "hold_missing_contract"
    );
    assert_eq!(
        evaluate_requirement(
            requirement,
            None,
            Some("LLM reviewer says it looks premium")
        ),
        "hold_missing_contract",
        "LLM approval is not an accept predicate"
    );

    let contract = capture_contract();
    assert_eq!(
        evaluate_requirement(requirement, Some(&contract), None),
        "accept_bounded_contract"
    );
}

fn capture_contract() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/real_task_hygiene/ambiguous/contract.txt");
    let output = run_sanitized(SanitizedCommand {
        program: PathBuf::from("cat"),
        args: vec![path.to_string_lossy().into_owned()],
        cwd: path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(10),
    })
    .expect("cat contract");
    assert!(output.success());
    String::from_utf8(output.stdout).expect("contract utf8")
}

fn evaluate_requirement(
    requirement: &str,
    contract: Option<&str>,
    _llm_review: Option<&str>,
) -> &'static str {
    if requirement.contains("feel") || requirement.contains("premium") {
        let Some(contract) = contract else {
            return "hold_missing_contract";
        };
        if contract.contains("criterion:") {
            return "accept_bounded_contract";
        }
        return "hold_missing_contract";
    }
    "accept_bounded_contract"
}
