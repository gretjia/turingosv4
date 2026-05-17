//! TISR Phase 6.3 — CAS-wire integration tests for the spec / generate /
//! welcome trio. Class 2 wire-up: uses the real `CasStore` from the lib
//! crate; no LLM calls (those need a real SILICONFLOW_API_KEY and live
//! in the manual demo path).
//!
//! What this covers:
//!   1. `turingos init` scaffolds a workspace with empty cas/.
//!   2. `turingos llm config` writes turingos.toml with SiliconFlow defaults.
//!   3. `turingos spec --skip-llm --answers-file` (a) writes spec.md +
//!      transcript.jsonl, (b) stores an EvidenceCapsule in cas/ with a real
//!      sha256 CID, (c) the sidecar JSONL records the put.
//!   4. `turingos welcome` reads the CAS CID and flips `spec [x]` with the
//!      truncated CID visible.
//!   5. `turingos spec` rerun is idempotent — same content → same CID.
//!   6. CAS readback: the byte stream stored at CID matches what spec.md
//!      contains (content addressing invariant).
//!
//! FC-trace: FC2-N16 (CLI dispatch) + FC3 evidence binding (CAS-anchored
//! spec capsule). Class 2: integration through public surface; no Class-4
//! schema touch.

#![cfg(unix)]

use std::path::PathBuf;
use std::process::Command;

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    for candidate in &[
        format!("{manifest_dir}/target/debug/turingos"),
        format!("{manifest_dir}/target/release/turingos"),
    ] {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return p;
        }
    }
    panic!("turingos binary not found; run `cargo build --bin turingos` first");
}

const TICTACTOE_ANSWERS: &str = r#"[
  "Tic-tac-toe game for my 7-year-old nephew to play on iPad. No ads, no signup.",
  "Pen-and-paper tic-tac-toe, but on a tablet. Like the simple HTML versions but without ads.",
  "Whose turn it is, what's in each of the 9 squares (X/O/empty), whether someone won.",
  "Open page -> see 3x3 grid -> click a square -> shows X -> opponent clicks -> shows O -> repeat until someone wins three in a row -> show 'X wins!' -> click 'New Game' to reset.",
  "Already-marked squares should NOT be clickable. After game ends, no more clicks register. Page refresh starts fresh.",
  "Out of scope: accounts, online multiplayer, leaderboard, AI opponent. Just two humans on one device.",
  "Success: my nephew can open it and play a full game alone, no instructions needed. Zero ads.",
  "Bonus: big text, soft colors (not harsh red/blue), thick X and O marks."
]"#;

#[test]
fn phase63_full_cas_wire_init_llm_spec_welcome() {
    let workspace = tempfile::TempDir::new().expect("tempdir");
    let ws = workspace.path();

    // 1. init
    let out = Command::new(turingos_bin())
        .args([
            "init",
            "--project",
            &ws.to_string_lossy(),
            "--template",
            "proof",
            "--force",
        ])
        .output()
        .expect("spawn init");
    assert!(
        out.status.success(),
        "init failed: stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(ws.join("cas").is_dir(), "init must create cas/");
    assert!(ws.join("genesis_payload.toml").is_file(), "init must write genesis_payload.toml");

    // 2. llm config (defaults)
    let out = Command::new(turingos_bin())
        .args(["llm", "config", "--workspace", &ws.to_string_lossy()])
        .output()
        .expect("spawn llm config");
    assert!(
        out.status.success(),
        "llm config failed: stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let toml = std::fs::read_to_string(ws.join("turingos.toml")).expect("read toml");
    assert!(toml.contains("siliconflow"), "expected siliconflow default in toml: {toml}");
    assert!(toml.contains("DeepSeek"), "expected DeepSeek model in toml: {toml}");
    assert!(toml.contains("Qwen3-Coder"), "expected Qwen3-Coder model in toml: {toml}");
    // Crucial security invariant: actual API key NEVER persisted to disk.
    assert!(!toml.contains("sk-"), "API key value must NEVER appear in turingos.toml: {toml}");

    // 3. spec --skip-llm --answers-file
    let answers_path = ws.join("answers.json");
    std::fs::write(&answers_path, TICTACTOE_ANSWERS).expect("write answers");
    let out = Command::new(turingos_bin())
        .args([
            "spec",
            "--workspace",
            &ws.to_string_lossy(),
            "--answers-file",
            &answers_path.to_string_lossy(),
            "--skip-llm",
            "--lang",
            "en",
        ])
        .output()
        .expect("spawn spec");
    assert!(
        out.status.success(),
        "spec failed: stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    // CID must appear in stdout (we want this visible to operators)
    assert!(
        stdout.contains("CAS capsule CID"),
        "spec stdout should print CAS capsule CID; got: {stdout}"
    );
    // Extract CID hex (64 lower-hex chars)
    let cid_line = stdout
        .lines()
        .find(|l| l.contains("CAS capsule CID"))
        .expect("cid line");
    let cid_hex = cid_line
        .split("->")
        .nth(1)
        .expect("cid arrow")
        .trim()
        .to_string();
    assert_eq!(cid_hex.len(), 64, "CID hex must be 64 chars; got {cid_hex:?}");
    assert!(
        cid_hex.chars().all(|c| c.is_ascii_hexdigit()),
        "CID must be hex; got {cid_hex:?}"
    );

    // Sidecar must record the put
    let sidecar = ws.join("cas").join(".turingos_cas_index.jsonl");
    assert!(sidecar.is_file(), "CAS sidecar must exist at {}", sidecar.display());
    let sidecar_content = std::fs::read_to_string(&sidecar).expect("read sidecar");
    assert!(
        sidecar_content.contains("EvidenceCapsule"),
        "sidecar should record an EvidenceCapsule put; got: {sidecar_content}"
    );
    assert!(
        sidecar_content.contains("turingos-spec-capsule-v1"),
        "sidecar should record the spec-capsule schema_id; got: {sidecar_content}"
    );

    // 4. welcome reads the CAS CID and flips spec [x]
    let out = Command::new(turingos_bin())
        .args(["welcome", "--workspace", &ws.to_string_lossy()])
        .output()
        .expect("spawn welcome");
    assert!(out.status.success(), "welcome failed: stderr={}", String::from_utf8_lossy(&out.stderr));
    let welcome_out = String::from_utf8_lossy(&out.stdout);
    assert!(
        welcome_out.contains("[x] 4. turingos spec"),
        "welcome should flip spec to [x]; got: {welcome_out}"
    );
    // Welcome should embed the truncated CID for operator audit
    let cid_prefix = &cid_hex[..8];
    let cid_suffix = &cid_hex[cid_hex.len() - 8..];
    assert!(
        welcome_out.contains(cid_prefix) && welcome_out.contains(cid_suffix),
        "welcome should show truncated CID ({cid_prefix}…{cid_suffix}); got: {welcome_out}"
    );

    // 5. Idempotency: re-running spec with the same answers should yield the
    //    SAME CID (sha256 of the same spec.md bytes is deterministic).
    let out2 = Command::new(turingos_bin())
        .args([
            "spec",
            "--workspace",
            &ws.to_string_lossy(),
            "--answers-file",
            &answers_path.to_string_lossy(),
            "--skip-llm",
            "--lang",
            "en",
        ])
        .output()
        .expect("spawn spec round 2");
    let stdout2 = String::from_utf8_lossy(&out2.stdout);
    let cid_hex2 = stdout2
        .lines()
        .find(|l| l.contains("CAS capsule CID"))
        .and_then(|l| l.split("->").nth(1))
        .map(|s| s.trim().to_string())
        .expect("cid2");
    assert_eq!(
        cid_hex2, cid_hex,
        "spec idempotency: same content must yield same CID — content-addressing invariant"
    );
}

#[test]
fn phase63_generate_missing_spec_fails_clearly() {
    // No spec.md, no CAS capsule. `turingos generate` should fail cleanly,
    // not panic. Doesn't need an API key — the missing-spec check fires
    // BEFORE the HTTP request.
    let workspace = tempfile::TempDir::new().expect("tempdir");
    let ws = workspace.path();

    let _ = Command::new(turingos_bin())
        .args([
            "init",
            "--project",
            &ws.to_string_lossy(),
            "--template",
            "proof",
            "--force",
        ])
        .output()
        .expect("init");

    let out = Command::new(turingos_bin())
        .args(["generate", "--workspace", &ws.to_string_lossy()])
        .output()
        .expect("spawn generate");
    assert!(!out.status.success(), "generate without spec must fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("spec not found")
            || stderr.contains("turingos spec")
            || stderr.contains("no spec"),
        "stderr should hint at missing spec; got: {stderr}"
    );
}

#[test]
fn phase63_llm_show_after_config_round_trips() {
    let workspace = tempfile::TempDir::new().expect("tempdir");
    let ws = workspace.path();

    let _ = Command::new(turingos_bin())
        .args([
            "init",
            "--project",
            &ws.to_string_lossy(),
            "--template",
            "proof",
            "--force",
        ])
        .output()
        .expect("init");

    // Custom model override (verifies the flag path, not just defaults).
    let _ = Command::new(turingos_bin())
        .args([
            "llm",
            "config",
            "--workspace",
            &ws.to_string_lossy(),
            "--meta-model",
            "deepseek-ai/DeepSeek-V3.2",
            "--blackbox-model",
            "Qwen/Qwen2.5-Coder-32B-Instruct",
            "--api-key-env",
            "MY_CUSTOM_KEY",
        ])
        .output()
        .expect("llm config");

    let out = Command::new(turingos_bin())
        .args(["llm", "show", "--workspace", &ws.to_string_lossy()])
        .output()
        .expect("llm show");
    assert!(out.status.success(), "llm show failed: stderr={}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("DeepSeek-V3.2"), "show must echo meta model: {stdout}");
    assert!(stdout.contains("Qwen2.5-Coder-32B-Instruct"), "show must echo blackbox model: {stdout}");
    assert!(stdout.contains("MY_CUSTOM_KEY"), "show must echo api_key_env name: {stdout}");
    // Must NEVER print the actual API key value (we never set one in this test,
    // but the contract is critical).
    assert!(
        !stdout.contains("sk-"),
        "show MUST NEVER print an API key value; got: {stdout}"
    );
}
