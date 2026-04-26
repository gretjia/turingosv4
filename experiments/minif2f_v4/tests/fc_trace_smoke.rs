// Phase A atom A6 — fc_trace end-to-end smoke.
//
// Spawns a tiny child process that calls `emit_event` with FC_TRACE=1
// + FC_TRACE_FILE=<tempfile>, then asserts the file contains a
// well-formed JSON line with the expected fc_id + run_id + agent_id +
// kv keys. The child-process boundary is required because
// `fc_trace_enabled()` caches the env var read in a `OnceLock` — once
// the parent test binary has read it (cold path: not set), no in-test
// env mutation can flip the gate. A subprocess gives a fresh OnceLock.
//
// This is the only behavioral witness the production wiring depends
// on: every FC anchor site emits via `emit_event`, and `emit_event`
// is exercised here under the gate-on path.

use std::io::Write;
use std::process::Command;

fn cargo_bin_dir() -> std::path::PathBuf {
    // The integration test binary lives at target/<profile>/deps/...;
    // examples are siblings under target/<profile>/examples. We want
    // `cargo run --example fc_trace_emit_one` so we just use cargo.
    std::env::current_dir().unwrap()
}

#[test]
fn fc_trace_file_receives_well_formed_json_event() {
    let tmpdir = std::env::temp_dir().join(format!(
        "fc_trace_smoke_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos()).unwrap_or(0)
    ));
    std::fs::create_dir_all(&tmpdir).expect("mkdir tmp");
    let trace_path = tmpdir.join("fc_trace.jsonl");

    // Write a tiny Rust harness that links against minif2f_v4 and
    // emits one event. Using `cargo run` directly is too slow; we
    // instead build a one-shot binary via `cargo build --bin` of an
    // example file we drop into the experiments crate. To stay
    // self-contained and avoid touching the workspace's bin set, we
    // spawn the test process to call its OWN wrapper via env injection
    // and re-exec.

    // Fallback approach: invoke the published evaluator binary in dry
    // mode. We don't have that. Simplest working pattern: shell out
    // to `cargo run -p minif2f_v4 --quiet --example fc_trace_emit_one`
    // which we provide as a sibling example file.

    let status = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "-p",
            "minif2f_v4",
            "--example",
            "fc_trace_emit_one",
        ])
        .env("FC_TRACE", "1")
        .env("FC_TRACE_FILE", &trace_path)
        .current_dir(cargo_bin_dir())
        .status()
        .expect("spawn fc_trace_emit_one example");
    assert!(status.success(), "fc_trace_emit_one example must exit 0");

    let contents = std::fs::read_to_string(&trace_path)
        .expect("FC_TRACE_FILE must exist after emit");
    let lines: Vec<&str> = contents
        .lines()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(lines.len(), 1, "exactly one event was emitted; file:\n{}", contents);
    let line = lines[0];

    // Must be valid JSON.
    let v: serde_json::Value =
        serde_json::from_str(line).expect("emitted line must be valid JSON");

    // Stable fields per fc_trace.rs event shape.
    assert!(v.get("ts_ms").and_then(|x| x.as_u64()).is_some());
    assert_eq!(
        v.get("fc_id").and_then(|x| x.as_str()),
        Some("FC2-N22"),
        "fc_id must be the stable string from FcId::as_str"
    );
    assert_eq!(
        v.get("run_id").and_then(|x| x.as_str()),
        Some("smoke_run_001")
    );
    assert_eq!(v.get("tx").and_then(|x| x.as_u64()), Some(42));
    assert_eq!(
        v.get("agent_id").and_then(|x| x.as_str()),
        Some("Agent_2")
    );
    let kv = v.get("kv").expect("kv block present");
    assert_eq!(
        kv.get("reason").and_then(|x| x.as_str()),
        Some("OmegaAccepted")
    );
    assert_eq!(kv.get("gp_nodes").and_then(|x| x.as_u64()), Some(7));

    // Cleanup.
    let _ = std::fs::remove_dir_all(&tmpdir);
    // Suppress unused-import lint when no Write needed in path above.
    let _ = std::io::sink().write_all(b"");
}
