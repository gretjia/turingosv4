use std::fs;

#[test]
fn predicate_registry_mutation_surface_is_crate_private() {
    let src = fs::read_to_string("src/top_white/predicates/registry.rs").expect("registry.rs");
    for forbidden in [
        "pub fn register(",
        "pub fn upsert(",
        "pub fn merge_with(",
        "pub fn replace(",
    ] {
        assert!(
            !src.contains(forbidden),
            "PredicateRegistry mutation must not be public: {forbidden}"
        );
    }
    assert!(src.contains("pub(crate) fn register("));
    assert!(
        !src.contains("\n    pub fn new() -> Self"),
        "PredicateRegistry must not expose an empty public constructor"
    );
    assert!(
        !src.contains("pub struct PredicateRegistry")
            || !src.contains("Default)]\npub struct PredicateRegistry"),
        "PredicateRegistry must not expose public Default construction"
    );
}

#[test]
fn sequencer_consumes_registry_by_shared_reference_not_mutable_reference() {
    let src = fs::read_to_string("src/state/sequencer.rs").expect("sequencer.rs");
    assert!(
        src.contains("predicate_registry: &PredicateRegistry"),
        "dispatch_transition must consume a PredicateRegistry shared reference"
    );
    assert!(
        !src.contains("_predicate_registry: &PredicateRegistry"),
        "registry parameter must not be underscore-discarded"
    );
    assert!(
        !src.contains("predicate_registry: &mut PredicateRegistry"),
        "sequencer admission must not receive mutable registry access"
    );
}

#[test]
fn production_replay_paths_use_shared_registry_loader() {
    let files = [
        "src/runtime/mod.rs",
        "src/runtime/verify.rs",
        "src/runtime/persistence_evidence.rs",
        "src/runtime/audit_assertions.rs",
        "src/runtime/agent_pnl.rs",
        "src/runtime/risk_cap_impact_report.rs",
        "src/web/market_view.rs",
        "src/bin/audit_dashboard.rs",
        "experiments/minif2f_v4/src/bin/lean_market.rs",
    ];
    for file in files {
        let src = fs::read_to_string(file).expect(file);
        let ad_hoc_registry_new = ["PredicateRegistry::", "new()"].concat();
        let empty_boot_manifest = ["BootPredicateManifest::", "empty()"].concat();
        assert!(
            !src.contains(&ad_hoc_registry_new) && !src.contains(&empty_boot_manifest),
            "{file} must use load_replay_registry instead of constructing an ad hoc empty registry"
        );
        assert!(
            !src.contains("replay_full_transition("),
            "{file} must use replay_full_transition_with_predicate_binding for production replay"
        );
    }
}
