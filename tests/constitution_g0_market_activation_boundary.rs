#[test]
fn true_suite_market_ab_runner_declares_g0_market_activation_receipt() {
    let script = std::fs::read_to_string("scripts/run_true_suite_market_ab_current_kernel.sh")
        .expect("true-suite market A/B runner exists");
    let manifest =
        std::fs::read_to_string("tests/fixtures/liveness/realworld_liveness_coverage.toml")
            .expect("realworld liveness manifest exists");

    for expected in [
        "g0_market_activation_current_kernel",
        "G0 market activation",
        "g0_market_activation_manifest.json",
        "single WorkTx",
        "src/bin/g0_market_activation_current_kernel.rs",
        "market_ab_candidate_only_g0_core_conditions_1_2_3_6_7_8_9",
        "c4_c5_priced_dag_and_c10_c11_reward_settlement_stage2",
    ] {
        assert!(
            script.contains(expected),
            "true-suite market A/B runner must produce G0 market activation receipt marker: {expected}"
        );
    }
    assert!(
        script.contains("full_system_participation_required: true")
            || script.contains("\"full_system_participation_required\": true"),
        "market A/B root domain manifest must declare full-system participation requirement"
    );
    assert!(
        script.contains("final_closure_possible: false")
            || script.contains("\"final_closure_possible\": false"),
        "market A/B root domain manifest must explicitly remain non-closing"
    );
    assert!(
        !script.contains("priced-DAG parent selection"),
        "market A/B runner note must not overclaim c4/c5 priced-DAG parent selection closure"
    );

    assert!(
        manifest.contains(
            "handover/evidence/true_suite/<run>/market_ab/g0/g0_market_activation_manifest.json"
        ),
        "market_ab_performance_fresh final artifacts must declare the G0 market activation receipt"
    );
}

#[test]
fn g0_market_activation_declares_core_scope_not_c1_to_c11_closure() {
    let source = std::fs::read_to_string("src/bin/g0_market_activation_current_kernel.rs")
        .expect("g0 market activation binary source exists");

    assert!(
        source.contains("g0_core_market_price_discovery_conditions_1_2_3_6_7_8_9"),
        "G0 market activation must declare only the core market price-discovery scope"
    );
    assert!(
        source.contains("c4_c5_constraint_note"),
        "G0 manifest must record the WorkTx/DAG reward-fanout constraint for c4/c5"
    );
    assert!(
        source.contains("c10_c11_stage2_note"),
        "G0 manifest must record settlement c10/c11 as stage-2, not current closure"
    );
    assert!(
        source.contains("buy_no_count"),
        "G0 manifest must expose an unambiguous machine-readable NO-side counter"
    );

    for forbidden in [
        "conditions_1_to_11",
        "Targets all 11 G0 conditions",
        "c1-11=[",
        "c10_sealed_settlement: bool",
        "c11_settlement_in_tape: bool",
    ] {
        assert!(
            !source.contains(forbidden),
            "G0 source must not overclaim full c1-11 closure marker: {forbidden}"
        );
    }
}
