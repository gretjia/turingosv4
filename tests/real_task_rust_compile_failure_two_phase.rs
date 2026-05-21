use std::path::{Path, PathBuf};
use std::time::Duration;

use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand, SanitizedOutput,
};

#[test]
fn real_task_rust_compile_failure_two_phase() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let base = tmp.path().join("base.rs");
    let good = tmp.path().join("good.rs");
    let bad_noop = tmp.path().join("bad_noop.rs");

    std::fs::write(&base, "fn main() { let _x: i32 = \"wrong\"; }\n").unwrap();
    std::fs::write(&good, "fn main() { let _x: i32 = 1; }\n").unwrap();
    std::fs::write(&bad_noop, "fn main() { let _x: i32 = \"wrong\"; }\n").unwrap();

    let base_out = rustc_metadata(tmp.path(), &base);
    let good_out = rustc_metadata(tmp.path(), &good);
    let bad_out = rustc_metadata(tmp.path(), &bad_noop);

    assert!(!base_out.success(), "base must reproduce compile failure");
    assert!(
        base_out.stderr.windows(5).any(|w| w == b"error"),
        "stderr evidence captured"
    );
    assert!(good_out.success(), "good candidate must compile");
    assert!(!bad_out.success(), "bad/no-op candidate remains rejected");
    assert_eq!(base_out.cwd, tmp.path());
    assert!(base_out.argv.iter().any(|arg| arg.ends_with("base.rs")));
    assert!(changed_paths_allowed(&["src/lib.rs"]));
    assert!(!changed_paths_allowed(&["constitution.md"]));
}

fn rustc_metadata(cwd: &Path, source: &Path) -> SanitizedOutput {
    run_sanitized(SanitizedCommand {
        program: PathBuf::from("rustc"),
        args: vec![
            "--emit=metadata".into(),
            source.to_string_lossy().into_owned(),
            "-o".into(),
            cwd.join("out.rmeta").to_string_lossy().into_owned(),
        ],
        cwd: cwd.to_path_buf(),
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(20),
    })
    .expect("run rustc")
}

fn changed_paths_allowed(paths: &[&str]) -> bool {
    paths
        .iter()
        .all(|path| !matches!(*path, "constitution.md" | "genesis_payload.toml"))
}
