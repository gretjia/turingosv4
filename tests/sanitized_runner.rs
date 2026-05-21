use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use turingosv4::sdk::sanitized_runner::{run_sanitized, NetworkPolicyClaim, SanitizedCommand};

fn workspace(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be after epoch")
        .as_nanos();
    path.push(format!("turingos-sanitized-runner-{name}-{nanos}"));
    std::fs::create_dir_all(&path).expect("workspace dir can be created");
    path
}

fn bash_command(cwd: &Path, script: &str) -> SanitizedCommand {
    SanitizedCommand {
        program: "bash".into(),
        args: vec!["-c".into(), script.into()],
        cwd: cwd.to_path_buf(),
        env: BTreeMap::new(),
        stdin: None,
        timeout: Duration::from_secs(2),
    }
}

#[test]
fn sandbox_env_allowlist_only() {
    let cwd = workspace("env-allowlist");
    let mut command = bash_command(&cwd, "env | sort");
    command.env.insert("KEEP_ME".into(), "yes".into());
    command.env.insert("ANOTHER_KEY".into(), "42".into());

    let output = run_sanitized(command).expect("command runs");
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf8");

    assert!(stdout.contains("KEEP_ME=yes"));
    assert!(stdout.contains("ANOTHER_KEY=42"));
    assert_eq!(
        output.allowed_env_keys,
        vec!["ANOTHER_KEY".to_string(), "KEEP_ME".to_string()]
    );
}

#[test]
fn sandbox_env_secret_not_inherited() {
    let cwd = workspace("env-secret");
    std::env::set_var("TURINGOS_SANITIZED_RUNNER_SECRET", "do-not-leak");

    let output = run_sanitized(bash_command(
        &cwd,
        "printf '%s' \"${TURINGOS_SANITIZED_RUNNER_SECRET:-}\"",
    ))
    .expect("command runs");

    assert_eq!(output.stdout, b"");
}

#[test]
fn sandbox_cwd_is_explicit_workspace() {
    let cwd = workspace("cwd");

    let output = run_sanitized(bash_command(&cwd, "pwd")).expect("command runs");
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf8");

    assert_eq!(stdout.trim(), cwd.to_string_lossy());
    assert_eq!(output.cwd, cwd);
}

#[test]
fn sandbox_stdout_stderr_are_captured() {
    let cwd = workspace("stdio");

    let output =
        run_sanitized(bash_command(&cwd, "printf out; printf err >&2")).expect("command runs");

    assert_eq!(output.stdout, b"out");
    assert_eq!(output.stderr, b"err");
}

#[test]
fn sandbox_large_stdout_does_not_deadlock() {
    let cwd = workspace("large-stdout");

    let output = run_sanitized(bash_command(
        &cwd,
        "python3 - <<'PY'\nprint('x' * 200000)\nPY",
    ))
    .expect("command runs");

    assert!(!output.timed_out);
    assert_eq!(output.exit_code, Some(0));
    assert!(output.stdout.len() >= 200000);
}

#[test]
fn sandbox_timeout_kills_child() {
    let cwd = workspace("timeout");
    let mut command = bash_command(&cwd, "sleep 5");
    command.timeout = Duration::from_millis(100);

    let output = run_sanitized(command).expect("timeout is reported as output");

    assert!(output.timed_out);
    assert_eq!(output.exit_code, None);
}

#[test]
fn sandbox_exit_code_is_recorded() {
    let cwd = workspace("exit-code");

    let output = run_sanitized(bash_command(&cwd, "exit 37")).expect("command runs");

    assert!(!output.timed_out);
    assert_eq!(output.exit_code, Some(37));
}

#[test]
fn sandbox_result_contains_command_argv() {
    let cwd = workspace("argv");

    let output = run_sanitized(bash_command(&cwd, "true")).expect("command runs");

    assert_eq!(
        output.argv,
        vec!["bash".to_string(), "-c".to_string(), "true".to_string()]
    );
}

#[test]
fn sandbox_rejects_false_network_denyall_claim() {
    let cwd = workspace("network-claim");

    let output = run_sanitized(bash_command(&cwd, "true")).expect("command runs");

    assert_eq!(output.network_policy_claim, NetworkPolicyClaim::NotEnforced);
}
