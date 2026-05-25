//! True-suite generate/artifact runner contract.
//!
//! CI uses a local mock OpenAI-compatible endpoint. The production runner
//! uses the same CLI path against the local DeepSeek/SiliconFlow proxy.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;

use serde_json::Value;
use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::artifact_bundle::{ArtifactBundleManifest, ARTIFACT_BUNDLE_SCHEMA_ID};
use turingosv4::runtime::generation_attempt::GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID;
use turingosv4::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        _ => panic!("unknown bin {name}"),
    }
}

fn bin_dir(path: &str) -> &Path {
    Path::new(path).parent().expect("bin has parent")
}

fn cid_from_hex(hex: &str) -> Cid {
    assert_eq!(hex.len(), 64, "cid must be 64 hex chars: {hex}");
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("cid hex byte");
    }
    Cid(bytes)
}

fn start_mock_openai_endpoint() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock endpoint");
    let addr = listener.local_addr().expect("local addr");
    thread::spawn(move || {
        for _ in 0..12 {
            let Ok((mut stream, _)) = listener.accept() else {
                break;
            };
            let mut buf = [0u8; 32768];
            let n = stream.read(&mut buf).expect("read request");
            let request = String::from_utf8_lossy(&buf[..n]);
            assert!(
                request.starts_with("POST /v1/chat/completions"),
                "unexpected mock endpoint request: {request}"
            );
            let content = if request.contains("TuringOS Blackbox AI")
                || request.contains("OUTPUT FORMAT")
                || request.contains("Generate the working code")
            {
                "### File: index.html\n```html\n<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Launch Plan Matrix</title><style>body{font-family:'IBM Plex Sans',system-ui,sans-serif;background:#f8f6f1;color:#1a1a1a}main{max-width:920px;margin:0 auto;padding:32px}h1{font-family:'Fraunces',Georgia,serif;color:#4e8b7a}.grid{display:grid;grid-template-columns:repeat(3,1fr);gap:12px}.card{border:1px solid #4e8b7a;padding:12px}</style></head><body><main><h1>Launch Plan Matrix</h1><div class=\"grid\"><section class=\"card\"><h2>Plan A</h2><p>Total: <strong>17</strong></p></section><section class=\"card\"><h2>Plan B</h2><p>Total: <strong>19</strong></p></section><section class=\"card\"><h2>Plan C</h2><p>Total: <strong>16</strong></p></section></div><p id=\"recommendation\">Recommended: Plan B</p><script>document.querySelector('#recommendation').dataset.ready='true';</script></main></body></html>\n```"
            } else {
                "# Launch Plan Decision Matrix\n\nBuild a self-contained HTML page that compares three launch plans with editable weighted scores, handles blank or invalid scores without crashing, and recommends the highest total. No backend, account, or external runtime dependency."
            };
            let body = serde_json::json!({
                "model": "mock-generate-artifact-agent",
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": content
                        },
                        "finish_reason": "stop"
                    }
                ],
                "usage": {
                    "prompt_tokens": 31,
                    "completion_tokens": 47,
                    "total_tokens": 78
                }
            })
            .to_string();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .expect("write response");
        }
    });
    format!("http://{addr}/v1/chat/completions")
}

fn extract_bundle_cid(stdout: &[u8]) -> String {
    let stdout = String::from_utf8_lossy(stdout);
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("artifact_bundle_cid="))
        .unwrap_or_else(|| panic!("artifact_bundle_cid missing from stdout:\n{stdout}"))
        .to_string()
}

#[test]
fn generate_artifact_runner_uses_external_endpoint_and_replays_artifact_chain() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("generate_artifact");
    let endpoint = start_mock_openai_endpoint();

    let init = Command::new(bin("turingos"))
        .args([
            "init",
            "--project",
            run_dir.to_str().expect("utf8 path"),
            "--template",
            "proof",
            "--provider",
            "deepseek",
        ])
        .output()
        .expect("run turingos init");
    assert!(
        init.status.success(),
        "turingos init failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&init.stdout),
        String::from_utf8_lossy(&init.stderr)
    );

    let answers = run_dir.join("answers.json");
    std::fs::write(
        &answers,
        r#"[
          "I need to compare launch plans without losing the tradeoffs.",
          "A small decision matrix is similar.",
          "Remember plans, weights, scores, and totals while the page is open.",
          "I open a page, edit scores, and see the best plan.",
          "Blank values, equal totals, and long names should not break it.",
          "No login, backend, cloud sync, payments, or external packages.",
          "Success means choosing a launch plan in under five minutes.",
          "Build a self-contained HTML launch-plan matrix."
        ]"#,
    )
    .expect("write answers");

    let spec = Command::new(bin("turingos"))
        .args([
            "spec",
            "--workspace",
            run_dir.to_str().expect("utf8 path"),
            "--answers-file",
            answers.to_str().expect("utf8 path"),
            "--lang",
            "en",
        ])
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("DEEPSEEK_API_KEY", "mock-meta-key")
        .env("DEEPSEEK_API_KEY_WORKER", "mock-worker-key")
        .output()
        .expect("run turingos spec");
    assert!(
        spec.status.success(),
        "turingos spec failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&spec.stdout),
        String::from_utf8_lossy(&spec.stderr)
    );
    assert!(run_dir.join("spec.md").is_file(), "spec.md missing");

    let generate = Command::new(bin("turingos"))
        .args([
            "generate",
            "--workspace",
            run_dir.to_str().expect("utf8 path"),
            "--from-capsule",
            "--entrypoint",
            "index.html",
            "--n-parallel-workers",
            "1",
        ])
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("DEEPSEEK_API_KEY", "mock-meta-key")
        .env("DEEPSEEK_API_KEY_WORKER", "mock-worker-key")
        .output()
        .expect("run turingos generate");
    assert!(
        generate.status.success(),
        "turingos generate failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&generate.stdout),
        String::from_utf8_lossy(&generate.stderr)
    );
    let bundle_cid = extract_bundle_cid(&generate.stdout);

    let cas_dir = run_dir.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas");
    let bundle_cid_typed = cid_from_hex(&bundle_cid);
    let bundle_meta = store
        .metadata(&bundle_cid_typed)
        .expect("artifact bundle metadata");
    assert_eq!(
        bundle_meta.schema_id.as_deref(),
        Some(ARTIFACT_BUNDLE_SCHEMA_ID)
    );
    let bundle_bytes = store.get(&bundle_cid_typed).expect("read bundle");
    let bundle: ArtifactBundleManifest =
        serde_json::from_slice(&bundle_bytes).expect("decode bundle manifest");
    assert_eq!(bundle.entrypoint, "index.html");
    assert_eq!(
        store
            .metadata(&cid_from_hex(&bundle.generation_attempt_cid))
            .and_then(|m| m.schema_id.clone()),
        Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string())
    );
    for file in &bundle.files {
        assert!(
            store.metadata(&cid_from_hex(&file.cid)).is_some(),
            "bundle file CID must resolve in CAS: {}",
            file.cid
        );
    }

    let pinned: Value = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("runtime_repo").join("pinned_pubkeys.json"))
            .expect("read pinned_pubkeys.json"),
    )
    .expect("parse pinned_pubkeys.json");
    let chain_run_id = pinned
        .get("run_id")
        .and_then(Value::as_str)
        .expect("pinned_pubkeys run_id");

    let replay_report = run_dir.join("replay_report.json");
    let verify = Command::new(bin("turingos"))
        .env("TURINGOS_BIN_DIR", bin_dir(bin("verify_chaintape")))
        .args([
            "verify",
            "chaintape",
            "--repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            chain_run_id,
            "--out",
            replay_report.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run turingos verify chaintape");
    assert!(
        verify.status.success(),
        "turingos verify chaintape failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&verify.stdout),
        String::from_utf8_lossy(&verify.stderr)
    );

    let replay: Value = serde_json::from_str(
        &std::fs::read_to_string(&replay_report).expect("read replay_report.json"),
    )
    .expect("parse replay report");
    assert!(
        replay
            .get("l4_entries")
            .and_then(Value::as_u64)
            .map(|n| n >= 9)
            .unwrap_or(false),
        "generate true-suite should include boot ticks plus TaskOpen/Escrow/Work/MarketSeed/Verify/Finalize/EventResolve; got {replay}"
    );
    for key in [
        "ledger_root_verified",
        "system_signatures_verified",
        "state_reconstructed",
        "economic_state_reconstructed",
        "cas_payloads_retrievable",
        "agent_signatures_verified",
        "proposal_telemetry_cas_retrievable",
    ] {
        assert_eq!(
            replay.get(key).and_then(Value::as_bool),
            Some(true),
            "replay indicator `{key}` must pass: {replay}"
        );
    }

    let proposal_seen = store
        .list_cids_by_object_type(ObjectType::Generic)
        .into_iter()
        .filter(|cid| {
            store
                .metadata(cid)
                .and_then(|m| m.schema_id.clone())
                .as_deref()
                == Some("turingosv4.proposal_telemetry.v1")
        })
        .any(|cid| read_proposal_telemetry(&store, &cid).is_ok());
    assert!(
        proposal_seen,
        "accepted artifact must be linked into WorkTx through ProposalTelemetry CAS"
    );
}

#[test]
fn generate_artifact_runner_script_preserves_external_agent_boundary() {
    let script =
        std::fs::read_to_string("scripts/run_true_suite_generate_artifact_current_kernel.sh")
            .expect("read generate runner script");
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("TURINGOS_SILICONFLOW_ENDPOINT"));
    assert!(script.contains("src/drivers/llm_proxy.py"));
    assert!(script.contains("\"$TURINGOS\" spec"));
    assert!(script.contains("generate"));
    assert!(script.contains("--from-capsule"));
    assert!(script.contains("artifact_bundle_cid.json"));
    assert!(script.contains("verify chaintape"));
    assert!(script.contains("handover/evidence/true_suite"));
    for forbidden in [
        "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS",
        "TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE",
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION",
    ] {
        assert!(
            !script.contains(forbidden),
            "true-suite generate runner must not inherit old scripted REAL fixtures: {forbidden}"
        );
    }
}
