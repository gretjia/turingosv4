//! G4.2 Class-4 STEP_B — model identity replay / no hidden switch.
//!
//! These tests are written directly against the 2026-05-13 architect
//! ratification preserved in
//! `handover/directives/2026-05-13_TB_G_G4_2_§8_ARCHITECT_RATIFICATION.md`.
//!
//! Fixed target chain:
//! `Agent_i -> genesis-assigned model identity -> AttemptTelemetry actual model
//! -> audit assertion: no hidden model switch -> dashboard/report divergence by
//! model family`.

use std::path::Path;

use turingosv4::runtime::genesis_report::{
    model_family_from_name, sorted_agent_model_assignment, AgentModelAssignment, GenesisReport,
    ModelAssignmentManifest,
};

fn base_report(tmp: &Path, assignment: Vec<AgentModelAssignment>) -> GenesisReport {
    GenesisReport {
        constitution_hash: Some("abc123".into()),
        runtime_repo: tmp.display().to_string(),
        cas_path: tmp.join("cas").display().to_string(),
        system_pubkey_hash: Some("def456".into()),
        agent_pubkeys_path: "agent_pubkeys.json".into(),
        initial_balances: vec![("Agent_0".into(), 1_000_000)],
        task_id: Some("task-g4-2".into()),
        task_open_tx: Some("taskopen-task-g4-2".into()),
        escrow_lock_tx: Some("escrowlock-task-g4-2".into()),
        agent_model_assignment: assignment,
        model_assignment_manifest_cid: Some("cid-model-assignment-manifest".into()),
        agent_role_assignment: vec![],
        role_assignment_manifest_cid: None,
    }
}

#[test]
fn g4_2_ratification_original_text_is_archived_locally() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("handover/directives/2026-05-13_TB_G_G4_2_§8_ARCHITECT_RATIFICATION.md");
    let text = std::fs::read_to_string(&path).expect("read archived architect ratification");

    for required in [
        "G3 已经进入 GREEN / 已足够关闭当前经济牙齿问题。",
        "下一步进入 G4.2：agent_model_assignment genesis schema + no-hidden-model-switch binding。",
        "批准 G4.2 进入 §8 ratification。",
        "模型身份必须进入 PromptCapsule / AttemptTelemetry 的一致链路",
        "run_report 必须能按 model family 做行为差异分析",
        "没有可 replay 的 model identity，",
        "就没有可审计的 multi-agent cognition。",
        "Stop immediately if implementation needs sequencer.rs, typed_tx.rs, signing payloads, kernel.rs, or bus.rs.",
    ] {
        assert!(
            text.contains(required),
            "ratification archive is missing architect source text: {required}"
        );
    }
}

#[test]
fn genesis_report_contains_sorted_agent_model_assignment() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let assignment = sorted_agent_model_assignment(vec![
        AgentModelAssignment {
            agent_id: "Agent_2".into(),
            model_name: "qwen3-coder".into(),
            model_family: "qwen".into(),
            model_provider: "qwen".into(),
            model_version: Some("coder".into()),
            temperature_milli: 700,
            prompt_template_hash: Some("template-qwen".into()),
        },
        AgentModelAssignment {
            agent_id: "Agent_0".into(),
            model_name: "claude-3-7-sonnet".into(),
            model_family: "claude".into(),
            model_provider: "anthropic".into(),
            model_version: Some("3-7-sonnet".into()),
            temperature_milli: 0,
            prompt_template_hash: Some("template-claude".into()),
        },
        AgentModelAssignment {
            agent_id: "Agent_1".into(),
            model_name: "gpt-5.2".into(),
            model_family: "openai".into(),
            model_provider: "openai".into(),
            model_version: Some("5.2".into()),
            temperature_milli: 700,
            prompt_template_hash: Some("template-openai".into()),
        },
    ]);

    assert_eq!(
        assignment
            .iter()
            .map(|a| a.agent_id.as_str())
            .collect::<Vec<_>>(),
        vec!["Agent_0", "Agent_1", "Agent_2"],
        "architect Q1 requires deterministic order sorted by agent_id"
    );
    assert_eq!(
        assignment
            .iter()
            .map(|a| a.temperature_milli)
            .collect::<Vec<_>>(),
        vec![0, 700, 700],
        "architect Q2 persists temperature_milli integers, e.g. 0.0 -> 0 and 0.7 -> 700"
    );

    let report = base_report(tmp.path(), assignment);
    let json = serde_json::to_string_pretty(&report).expect("serialize report");
    assert!(json.contains("\"agent_model_assignment\""));
    assert!(json.contains("\"temperature_milli\": 700"));
    assert!(!json.contains("temperature\": 0.7"));

    let round: GenesisReport = serde_json::from_str(&json).expect("round trip");
    assert_eq!(round.agent_model_assignment.len(), 3);
    assert_eq!(round.agent_model_assignment[0].agent_id, "Agent_0");
    assert_eq!(
        round.model_assignment_manifest_cid.as_deref(),
        Some("cid-model-assignment-manifest")
    );
}

#[test]
fn historical_genesis_report_without_model_assignment_is_grandfathered() {
    let legacy = r#"{
      "constitution_hash": null,
      "runtime_repo": "/tmp/runtime",
      "cas_path": "/tmp/runtime/cas",
      "system_pubkey_hash": null,
      "agent_pubkeys_path": "agent_pubkeys.json",
      "initial_balances": [],
      "task_id": null,
      "task_open_tx": null,
      "escrow_lock_tx": null
    }"#;
    let report: GenesisReport = serde_json::from_str(legacy)
        .expect("old genesis_report.json must decode with serde(default)");
    assert!(
        report.agent_model_assignment.is_empty(),
        "architect Q1 requires serde(default) grandfathering for historical genesis_report"
    );
    assert!(report.model_assignment_manifest_cid.is_none());
}

#[test]
fn model_assignment_manifest_records_resolver_provenance_without_secrets() {
    let manifest = ModelAssignmentManifest {
        batch_id: "g_phase_g4_2_mini_20260513T000000Z".into(),
        agent_model_assignment: sorted_agent_model_assignment(vec![
            AgentModelAssignment {
                agent_id: "Agent_0".into(),
                model_name: "claude-3-7-sonnet".into(),
                model_family: "claude".into(),
                model_provider: "anthropic".into(),
                model_version: Some("3-7-sonnet".into()),
                temperature_milli: 200,
                prompt_template_hash: Some("template-a".into()),
            },
            AgentModelAssignment {
                agent_id: "Agent_1".into(),
                model_name: "gpt-5.2".into(),
                model_family: "openai".into(),
                model_provider: "openai".into(),
                model_version: Some("5.2".into()),
                temperature_milli: 200,
                prompt_template_hash: Some("template-b".into()),
            },
            AgentModelAssignment {
                agent_id: "Agent_2".into(),
                model_name: "qwen3-coder".into(),
                model_family: "qwen".into(),
                model_provider: "qwen".into(),
                model_version: Some("coder".into()),
                temperature_milli: 200,
                prompt_template_hash: Some("template-c".into()),
            },
        ]),
        resolver_source: "AGENT_MODELS env via minif2f_v4 resolver".into(),
        agent_models_env_hash: "0123456789abcdef".repeat(4),
        phase_d_hetero_ok: true,
        created_at_head_t: 0,
        model_family_count_required: 3,
        model_family_count_observed: 3,
        fallback_behavior: "fail_closed_on_insufficient_families".into(),
        proxy_provider: "provider_from_model_assignment".into(),
        fail_closed_reason: None,
    };

    let json = serde_json::to_string_pretty(&manifest).expect("manifest json");
    assert!(json.contains("\"agent_models_env_hash\""));
    assert!(json.contains("\"phase_d_hetero_ok\": true"));
    assert!(json.contains("\"model_family_count_required\": 3"));
    assert!(json.contains("\"model_family_count_observed\": 3"));
    assert!(
        !json.contains("sk-") && !json.contains("OPENAI_API_KEY"),
        "manifest may hash AGENT_MODELS but must not store secrets"
    );
}

#[test]
fn model_family_inference_supports_minimal_three_family_smoke() {
    let families = [
        model_family_from_name("claude-3-7-sonnet"),
        model_family_from_name("gpt-5.2"),
        model_family_from_name("qwen3-coder"),
        model_family_from_name("deepseek-r1"),
    ];
    assert_eq!(families[0], "claude");
    assert_eq!(families[1], "openai");
    assert_eq!(families[2], "qwen");
    assert_eq!(families[3], "deepseek");

    let distinct = families
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        distinct.len() >= 3,
        "G4.2 smoke evidence must witness at least 3 distinct model families or fail closed"
    );
}

#[test]
fn g4_batch_script_records_model_identity_inputs() {
    let script = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/run_g_phase_batch.sh"),
    )
    .expect("read run_g_phase_batch.sh");

    for required in [
        "AGENT_MODELS",
        "PHASE_D_HETERO_OK",
        "G_PHASE_N_AGENTS",
        "G_PHASE_CONDITION",
        "agent_models_env_hash",
        "model_family_count_required",
        "model_family_count_observed",
        "assignment_summary",
        "--n-agents \"$G_PHASE_N_AGENTS\"",
        "--condition \"$G_PHASE_CONDITION\"",
    ] {
        assert!(
            script.contains(required),
            "G4.2 smoke manifest must record {required} per architect Step 10"
        );
    }

    assert!(
        !script.contains("--condition n1"),
        "G4.2 smoke must not hard-code n1 while advertising 10 agent model slots"
    );
}
