use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand, SanitizedOutput,
};

const SECRET_KEY: &str = "TURINGOS_REAL_TASK_PRIVACY_SECRET";
const SECRET_VALUE: &str = "should_not_leak_real_task_fixture";

#[test]
fn real_task_privacy_secret_env_not_leaked() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let probe = compile_probe(tmp.path());
    std::env::set_var(SECRET_KEY, SECRET_VALUE);

    let good = run_probe(tmp.path(), &probe, env_allowlist_from_current(&["PATH"]));
    assert!(good.success());
    assert_eq!(privacy_decision(&good), "accept");

    let mut bad_env = env_allowlist_from_current(&["PATH"]);
    bad_env.insert(SECRET_KEY.to_string(), SECRET_VALUE.to_string());
    let bad = run_probe(tmp.path(), &probe, bad_env);
    assert!(bad.success());
    assert_eq!(privacy_decision(&bad), "reject_secret_leak");

    std::env::remove_var(SECRET_KEY);
}

fn compile_probe(cwd: &Path) -> PathBuf {
    let source = cwd.join("probe.rs");
    let bin = cwd.join("probe");
    std::fs::write(
        &source,
        format!(
            r#"
fn main() {{
    let value = std::env::var("{SECRET_KEY}").unwrap_or_else(|_| "missing".to_string());
    println!("stdout={{value}}");
    eprintln!("stderr={{value}}");
}}
"#
        ),
    )
    .expect("write probe");
    let compile = run_sanitized(SanitizedCommand {
        program: PathBuf::from("rustc"),
        args: vec![
            source.to_string_lossy().into_owned(),
            "-o".into(),
            bin.to_string_lossy().into_owned(),
        ],
        cwd: cwd.to_path_buf(),
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(20),
    })
    .expect("compile probe");
    assert!(
        compile.success(),
        "compile failed: {}",
        String::from_utf8_lossy(&compile.stderr)
    );
    bin
}

fn run_probe(cwd: &Path, probe: &Path, env: BTreeMap<String, String>) -> SanitizedOutput {
    run_sanitized(SanitizedCommand {
        program: probe.to_path_buf(),
        args: Vec::new(),
        cwd: cwd.to_path_buf(),
        env,
        stdin: None,
        timeout: Duration::from_secs(10),
    })
    .expect("run probe")
}

fn privacy_decision(output: &SanitizedOutput) -> &'static str {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stdout.contains(SECRET_VALUE) || stderr.contains(SECRET_VALUE) {
        "reject_secret_leak"
    } else {
        "accept"
    }
}
