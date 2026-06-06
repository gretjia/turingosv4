use turingosv4::workloads::benchmark_boundary::{
    validate_adapter_claim_text, AdapterResultClassification, ClaimBoundaryError,
    WorkloadAdapterKind, WorkloadAdapterResult,
};

fn base_result(classification: AdapterResultClassification) -> WorkloadAdapterResult {
    WorkloadAdapterResult {
        workload_id: "lean-mini-smoke".to_string(),
        run_id: "a14-run-001".to_string(),
        adapter_kind: WorkloadAdapterKind::Lean,
        evidence_manifest_cid:
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        result_classification: classification,
        verifier_backed_task_pass_count: 0,
        structural_smoke_count: 1,
        participation_canary_count: 0,
        unsupported_claim_count: 0,
    }
}

#[test]
fn structural_smoke_cannot_claim_task_pass() {
    let result = base_result(AdapterResultClassification::StructuralSmoke);
    let err = validate_adapter_claim_text(&result, "TASK-PASS on heldout benchmark", false)
        .expect_err("structural smoke cannot claim task pass");
    assert_eq!(err, ClaimBoundaryError::TaskPassWithoutVerifier);
}

#[test]
fn verifier_backed_result_may_claim_task_pass() {
    let mut result = base_result(AdapterResultClassification::RealVerifierBacked);
    result.verifier_backed_task_pass_count = 1;
    result.structural_smoke_count = 0;

    validate_adapter_claim_text(&result, "verifier-backed TASK-PASS", false)
        .expect("real verifier-backed result may claim task pass");
}

#[test]
fn market_victory_claim_requires_preregistered_evidence() {
    let mut result = base_result(AdapterResultClassification::RealVerifierBacked);
    result.verifier_backed_task_pass_count = 3;
    result.structural_smoke_count = 0;

    let err = validate_adapter_claim_text(&result, "market beats baseline on this workload", false)
        .expect_err("market victory headline must be preregistered");
    assert_eq!(err, ClaimBoundaryError::StrongClaimWithoutPreregistration);
}

#[test]
fn unsupported_claim_counter_is_ship_block() {
    let mut result = base_result(AdapterResultClassification::ParticipationCanary);
    result.unsupported_claim_count = 1;

    let err = result
        .validate()
        .expect_err("unsupported adapter claims must block");
    assert_eq!(err, ClaimBoundaryError::UnsupportedClaimCountNonZero);
}

#[test]
fn adapter_kind_names_stay_workload_scoped() {
    assert_eq!(
        turingosv4::workloads::lean::adapter_kind(),
        WorkloadAdapterKind::Lean
    );
    assert_eq!(
        turingosv4::workloads::swebench::adapter_kind(),
        WorkloadAdapterKind::Swebench
    );
}
