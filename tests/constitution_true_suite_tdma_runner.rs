//! True-suite TDMA/proof runner contract.
//!
//! CI uses a local mock OpenAI-compatible endpoint. The production runner
//! uses the same public `turingos tdma run` path against the local
//! DeepSeek/SiliconFlow proxy.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;

use serde_json::Value;
use sha2::{Digest, Sha256};
use tempfile::TempDir;
use turingosv4::git_tape_ledger::GitTapeLedger;
use turingosv4::ledger::{ImmutableTapeLedger, NodeKind};

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        _ => panic!("unknown bin {name}"),
    }
}

fn sha256_file(path: &Path) -> String {
    let mut h = Sha256::new();
    h.update(std::fs::read(path).expect("read file for sha256"));
    format!("{:x}", h.finalize())
}

fn response_for_request(request: &str, stage_attempt: usize) -> String {
    let body = if request.contains("Stage1-Simplify-2010n") && stage_attempt == 1 {
        "Too short."
    } else if request.contains("Stage1-Simplify-2010n") {
        "For Stage 1, we explicitly simplify 2025n - 15n by factoring out n: (2025 - 15)n = 2010n. Thus the closure rule is exactly about positive divisors of 2010n, not a different expression."
    } else if request.contains("Stage2-Factor-2010") {
        "For Stage 2, factor the integer 2010 = 2 · 3 · 5 · 67 into prime factors. This records all four possible new prime sources contributed by multiplying n by 2010."
    } else if request.contains("Stage3-Closure-Prime-Containment") {
        "For Stage 3, any positive divisor of 2010n can introduce only prime factors already among primes of n together with the primes 2, 3, 5, and 67. Therefore the divisor operation has a clear prime-containment invariant and no new prime outside that set appears."
    } else if request.contains("Stage4-Counterexample-Construction") {
        "For Stage 4, define S as the smallest set containing 1 and closed under taking every positive divisor of 2010n whenever n is in S. This is a counterexample: every member has only primes from {2,3,5,67}, so the prime 7 never enters and is not contained in S."
    } else if request.contains("Stage5-Conclude-NO") {
        "For Stage 5, the answer is NO: S need not contain all positive integers. The constructed closed set satisfies the rule but omits 7, so containing every positive integer is not forced."
    } else {
        panic!("unexpected TDMA mock request: {request}");
    };
    format!(
        "{{\"schema_version\":\"tdma-state-update/v1\",\"status\":\"Proceed\",\"task_id\":\"mock-stage\",\"action\":\"PROPOSE\",\"failed_predicate\":null,\"reject_class\":null,\"next_action_hint\":null,\"evidence_hash\":null}}\n---BODY---\n{body}"
    )
}

fn start_mock_openai_endpoint() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock endpoint");
    let addr = listener.local_addr().expect("local addr");
    thread::spawn(move || {
        let mut stage_attempts = std::collections::BTreeMap::<String, usize>::new();
        for _ in 0..6 {
            let Ok((mut stream, _)) = listener.accept() else {
                break;
            };
            let mut buf = [0u8; 49152];
            let n = stream.read(&mut buf).expect("read request");
            let request = String::from_utf8_lossy(&buf[..n]);
            assert!(
                request.starts_with("POST /v1/chat/completions"),
                "unexpected mock endpoint request: {request}"
            );
            let stage = [
                "Stage1-Simplify-2010n",
                "Stage2-Factor-2010",
                "Stage3-Closure-Prime-Containment",
                "Stage4-Counterexample-Construction",
                "Stage5-Conclude-NO",
            ]
            .iter()
            .find(|stage| request.contains(**stage))
            .unwrap_or_else(|| panic!("unexpected TDMA stage request: {request}"))
            .to_string();
            let count = stage_attempts.entry(stage).or_insert(0);
            *count += 1;
            let content = response_for_request(&request, *count);
            let body = serde_json::json!({
                "model": "mock-tdma-proof-agent",
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
                    "prompt_tokens": 41,
                    "completion_tokens": 59,
                    "total_tokens": 100
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

#[test]
fn tdma_runner_uses_external_endpoint_and_writes_durable_tdma_tape() {
    let tmp = TempDir::new().expect("tempdir");
    let workspace = tmp.path().join("tdma_workspace");
    let evidence_dir = workspace.clone();
    let endpoint = start_mock_openai_endpoint();

    let init = Command::new(bin("turingos"))
        .args([
            "init",
            "--project",
            workspace.to_str().expect("utf8 path"),
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

    let run = Command::new(bin("turingos"))
        .args([
            "tdma",
            "run",
            "--workspace",
            workspace.to_str().expect("utf8 path"),
            "--judge",
            "putnam_2025_b3",
            "--role",
            "meta",
            "--evidence-dir",
            evidence_dir.to_str().expect("utf8 path"),
            "--max-attempts-per-stage",
            "2",
            "--temperature",
            "0.1",
            "--tape-backend",
            "git",
        ])
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("DEEPSEEK_API_KEY", "mock-meta-key")
        .env("DEEPSEEK_API_KEY_WORKER", "mock-worker-key")
        .output()
        .expect("run turingos tdma");
    assert!(
        run.status.success(),
        "turingos tdma failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let manifest_path = evidence_dir.join("manifest.json");
    let chaintape_path = evidence_dir.join("chaintape.jsonl");
    let probes_path = evidence_dir.join("per_attempt_probes.jsonl");
    let report_path = evidence_dir.join("ProductionTdmaReport.md");
    let tdma_tape = workspace.join("tdma_tape.git");
    assert!(manifest_path.is_file(), "manifest.json missing");
    assert!(chaintape_path.is_file(), "chaintape.jsonl missing");
    assert!(probes_path.is_file(), "per_attempt_probes.jsonl missing");
    assert!(report_path.is_file(), "ProductionTdmaReport.md missing");
    assert!(tdma_tape.is_dir(), "durable TDMA git tape missing");

    let manifest: Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).expect("read manifest"))
            .expect("parse manifest");
    assert_eq!(
        manifest["problem_label"],
        "turingos tdma --judge putnam_2025_b3"
    );
    assert_eq!(manifest["stages_total"], 5);
    assert_eq!(manifest["stages_completed"], 5);
    assert_eq!(manifest["total_attempts"], 6);
    assert_eq!(manifest["total_failed_attempts"], 1);
    assert_eq!(manifest["leak_in_any_prompt"], false);
    assert_eq!(manifest["all_prompts_within_budget"], true);
    assert_eq!(
        manifest["chaintape_sha256"]
            .as_str()
            .expect("chaintape sha"),
        sha256_file(&chaintape_path)
    );
    assert_eq!(
        manifest["probes_sha256"].as_str().expect("probes sha"),
        sha256_file(&probes_path)
    );

    let chaintape = std::fs::read_to_string(&chaintape_path).expect("read chaintape");
    assert!(
        chaintape.lines().count() >= 5,
        "TDMA tape must carry one accepted node per stage at minimum"
    );
    assert!(
        chaintape.contains("StateAccepted"),
        "TDMA tape must carry accepted state nodes"
    );

    let reopened = GitTapeLedger::open(&tdma_tape).expect("reopen durable TDMA tape");
    let nodes = reopened.dump_all_nodes();
    assert!(
        nodes.len() >= 7,
        "durable TDMA tape must retain rejected proposal, retry BBS, and accepted states"
    );
    assert!(
        reopened.count_nodes(Some(NodeKind::RetryBeliefState), Some(false), None, None) >= 1,
        "reopened TDMA tape must retain retry belief state from rejected first attempt"
    );
    assert!(
        reopened.count_nodes(Some(NodeKind::StateAccepted), Some(true), None, None) >= 5,
        "reopened TDMA tape must retain accepted state nodes"
    );
    assert_ne!(
        reopened.get_verified_head(),
        "H0",
        "durable TDMA verified head must advance after accepted proof stages"
    );
}

#[test]
fn tdma_runner_script_preserves_external_boundary_and_tdma_tape_semantics() {
    let script = std::fs::read_to_string("scripts/run_true_suite_tdma_current_kernel.sh")
        .expect("read TDMA runner script");
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("TURINGOS_SILICONFLOW_ENDPOINT"));
    assert!(script.contains("src/drivers/llm_proxy.py"));
    assert!(script.contains("\"$TURINGOS\" tdma run"));
    assert!(script.contains("--tape-backend git"));
    assert!(script.contains("tdma_tape.git"));
    assert!(script.contains("replay_report.json"));
    assert!(script.contains("handover/evidence/true_suite"));
    assert!(script.contains("not bottom-white L4"));
    for forbidden in [
        "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS",
        "TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE",
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION",
        "tdma_rc1_real_evidence",
    ] {
        assert!(
            !script.contains(forbidden),
            "true-suite TDMA runner must not inherit old scripted or legacy evidence path: {forbidden}"
        );
    }
}
