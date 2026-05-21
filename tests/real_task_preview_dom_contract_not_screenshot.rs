use std::path::{Path, PathBuf};
use std::time::Duration;

use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand,
};

#[test]
fn real_task_preview_dom_contract_not_screenshot() {
    let base = capture_fixture("preview/base.html");
    let good = capture_fixture("preview/good.html");
    let bad_style = capture_fixture("preview/bad_style_only.html");
    let bad_console = capture_fixture("preview/bad_console.html");
    let bad_network = capture_fixture("preview/bad_network.html");

    assert_eq!(preview_decision(&base, false), "reject_dom_contract");
    assert_eq!(preview_decision(&good, false), "accept");
    assert_eq!(
        preview_decision(&bad_style, true),
        "reject_dom_contract",
        "screenshot/style evidence alone is not an oracle"
    );
    assert_eq!(
        preview_decision(&bad_console, false),
        "reject_console_error"
    );
    assert_eq!(
        preview_decision(&bad_network, false),
        "reject_untracked_network"
    );
}

fn capture_fixture(rel: &str) -> String {
    let path = fixture_path(rel);
    let output = run_sanitized(SanitizedCommand {
        program: PathBuf::from("cat"),
        args: vec![path.to_string_lossy().into_owned()],
        cwd: path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(10),
    })
    .expect("cat fixture");
    assert!(
        output.success(),
        "fixture capture failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.stdout.is_empty(), "stdout evidence captured");
    String::from_utf8(output.stdout).expect("fixture utf8")
}

fn preview_decision(html: &str, _screenshot_captured: bool) -> &'static str {
    if html.contains("console.error") {
        return "reject_console_error";
    }
    if html.contains("https://") || html.contains("http://") {
        return "reject_untracked_network";
    }
    if !html.contains("id=\"primary-cta\"") {
        return "reject_dom_contract";
    }
    "accept"
}

fn fixture_path(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/real_task_hygiene")
        .join(rel)
}
