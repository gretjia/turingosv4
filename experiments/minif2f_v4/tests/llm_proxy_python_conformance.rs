// A8e2 fix G1 — recurring conformance gate for `src/drivers/llm_proxy.py`.
//
// Round-2 audit (Codex R2#1 + Gemini R2#1) caught: `scripts/test_llm_proxy.py`
// existed and was in Trust Root, but was only documented as a manual
// invocation. A test that does not run automatically is just
// documentation — it cannot prevent the V3L-27-class regression that
// Gemini's round-1 VETO targeted.
//
// This integration test bridges the Python proxy suite into
// `cargo test --workspace` so it runs on every Rust test invocation
// and on every CI pipeline that already exercises Rust tests. The
// test depends on the system `python3` interpreter being available;
// if not, it skips with a clear diagnostic so the absence-of-Python
// case doesn't masquerade as a real failure.
//
// Constitutional anchor: meta-witness for atom A7 (case C-027 +
// V3L-27 mitigation — multi-key round-robin avoiding single-key
// rate-limit collapse).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    // The minif2f_v4 crate's Cargo.toml lives at the repo's
    // experiments/minif2f_v4 path, so two parents up is the repo root.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root reachable from CARGO_MANIFEST_DIR")
        .to_path_buf()
}

#[test]
fn proxy_python_conformance_suite_passes() {
    let root = repo_root();
    let script = root.join("scripts").join("test_llm_proxy.py");
    assert!(
        script.is_file(),
        "scripts/test_llm_proxy.py must exist at the canonical path; got {:?}",
        script
    );

    // A8e3 fix H6 (Codex R3#3): the wrapper MUST fail closed when
    // `python3` is missing. A "soft skip" is exactly the silent-pass
    // failure mode that Gemini's round-1 VETO targeted — a gate that
    // disappears under environmental drift is not a gate. If a runner
    // environment lacks Python, that's a CI configuration bug, not an
    // acceptable-skip case. Explicit opt-out:
    // `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` — must be set deliberately,
    // never set by default. The bypass is logged loudly so the gate's
    // absence is visible in test output.
    let opt_out = std::env::var("SKIP_LLM_PROXY_PYTHON_CONFORMANCE")
        .as_deref() == Ok("1");
    if opt_out {
        eprintln!(
            "[G1] SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 — gate explicitly \
             bypassed. This is a downgraded run; the V3L-27 round-robin \
             conformance battery did NOT execute."
        );
        return;
    }
    let python_check = Command::new("python3").arg("--version").output();
    assert!(
        python_check.is_ok(),
        "python3 not found on PATH; G1 conformance gate requires it. \
         Install python3 + the openai SDK (see scripts/test_llm_proxy.py \
         header) or — only with deliberate intent — set \
         SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 to bypass."
    );

    let output = Command::new("python3")
        .arg(&script)
        .current_dir(&root)
        .output()
        .expect("spawn python3 scripts/test_llm_proxy.py");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "scripts/test_llm_proxy.py must exit 0 (round-robin + routing \
         conformance battery for V3L-27 mitigation).\n\
         status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        stdout,
        stderr
    );

    // Sanity-check the unittest summary line is present so a future
    // refactor that removes the test class definitions surfaces here
    // instead of silently skipping.
    assert!(
        stderr.contains("OK") || stdout.contains("OK"),
        "unittest output must contain the trailing 'OK' line.\n\
         stdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}
