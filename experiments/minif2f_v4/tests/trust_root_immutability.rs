// PPUT-CCL Phase B7 — Trust Root immutability (PREREG § 1.8 + § 7 Gate B).
//
// Boot computes SHA-256 of every Trust Root file at process start and
// compares against the genesis_payload.toml [trust_root] manifest. Any
// mismatch = `TRUST_ROOT_TAMPERED` abort.
//
// Trust Root manifest (PREREG § 1.8 + B2-B4 mid-term audit recommendation):
//   src/kernel.rs
//   src/wal.rs
//   src/bus.rs
//   src/drivers/llm_http.rs                           (B2-B4 audit add)
//   experiments/minif2f_v4/src/lean4_oracle.rs
//   experiments/minif2f_v4/src/cost_aggregator.rs     (B2)
//   experiments/minif2f_v4/src/wall_clock.rs          (B3)
//   experiments/minif2f_v4/src/post_hoc_verifier.rs   (B4)
//   experiments/minif2f_v4/src/jsonl_schema.rs        (B1)
//   experiments/minif2f_v4/src/bin/evaluator.rs       (the wiring)
//   constitution.md
//   handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
//   genesis_payload.toml [pput_accounting_0] section
//   cases/*.yaml (via cases/MANIFEST.sha256)
//
// Phase B7 scope; #[ignore] until Boot integration + genesis_payload.toml
// land.

#[test]
#[ignore = "Phase B7 — Boot integration + genesis_payload.toml not yet implemented"]
fn test_trust_root_immutable_at_boot() {
    // Cold-start with intact files: Boot computes SHA-256s, all match
    // genesis manifest, process continues. No abort.
    panic!("Phase B7 not implemented");
}

#[test]
#[ignore = "Phase B7 — Boot integration + genesis_payload.toml not yet implemented"]
fn test_trust_root_simulated_write_aborts() {
    // Simulated tampering: modify a Trust Root file byte (in a tempdir
    // copy), recompute SHA-256, assert mismatch → TRUST_ROOT_TAMPERED
    // abort path fires.
    panic!("Phase B7 not implemented");
}

#[test]
#[ignore = "Phase B7 — Boot integration + genesis_payload.toml not yet implemented"]
fn test_trust_root_manifest_includes_b2_b4_files() {
    // Mid-term audit recommendation: B2 (cost_aggregator), B3 (wall_clock),
    // B4 (post_hoc_verifier), B1 (jsonl_schema), evaluator.rs, llm_http.rs
    // MUST be in the Trust Root manifest. Boot will hash them at startup.
    //
    // Phase B7 wiring: load genesis_payload.toml [trust_root]; assert each
    // path above is a key.
    panic!("Phase B7 not implemented");
}

#[test]
#[ignore = "Phase B7 — Boot integration + genesis_payload.toml not yet implemented"]
fn test_pput_accounting_0_section_present() {
    // genesis_payload.toml must contain [pput_accounting_0] with:
    //   schema_version, progress_definition, cost_definition,
    //   time_definition, verified_predicate, heldout_sealed_hash,
    //   source_pool_sha256, baseline_regression_rate (B7-extra),
    //   baseline_regression_jsonl_sha256, k_max=10, n_max=34.
    panic!("Phase B7 not implemented");
}
