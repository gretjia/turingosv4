//! Constitution gate — `BenchmarkManifest` schema is load-bearing for any
//! scaled (50+ × n>1 × multi-seed) benchmark batch per TB-18B charter
//! FR-18B.1 + CR-18B.5 + `feedback_benchmark_manifest_required`.
//!
//! These tests are the executable face of CR-18B.5 ("NO BenchmarkManifest
//! field omission. Missing fields = ship-block."). The lib unit tests in
//! `src/runtime/benchmark_manifest.rs::tests` cover per-field validation;
//! these constitution tests cross-cut the schema invariants and the
//! disk-format stability that downstream tooling depends on.
//!
//! `FC-trace: Art-0.2 Tape Canonical (manifest is per-batch metadata)`.

use turingosv4::runtime::benchmark_manifest::{
    BenchmarkManifest, BenchmarkManifestError, BENCHMARK_MANIFEST_SCHEMA_ID,
};

fn ref_manifest() -> BenchmarkManifest {
    BenchmarkManifest {
        batch_id: "tb_18b_m1_2026-05-08T00-00-00Z".into(),
        problem_ids: vec![
            "mathd_algebra_107".into(),
            "mathd_algebra_113".into(),
            "mathd_algebra_141".into(),
        ],
        model_id: "deepseek-chat".into(),
        model_ver: "v3".into(),
        temperature_decimal: "0.7".into(),
        max_tx_budget: 64,
        n_per_problem: 3,
        seeds: vec![1, 2, 3],
        lean_version: "4.7.0".into(),
        mathlib_commit: "a".repeat(40),
        turingos_commit: "b".repeat(40),
        strategy: "M1: 3p × n=3 × 3-seed = 27 runs DeepSeek baseline".into(),
        schema_id: BENCHMARK_MANIFEST_SCHEMA_ID.to_string(),
    }
}

/// CR-18B.5 — every required field on a properly-populated manifest validates.
/// This is the GREEN baseline; subsequent tests stress field-omission rejection.
#[test]
fn benchmark_manifest_well_formed_validates() {
    let m = ref_manifest();
    m.validate().expect("ref manifest validates");
    // total_runs = problems × n × seeds = 3 × 3 × 3 = 27
    assert_eq!(m.total_runs(), 27);
}

/// CR-18B.5 — schema_id pin: a manifest declaring an outdated schema id is
/// ship-block. Catches silent schema bumps.
#[test]
fn benchmark_manifest_schema_id_drift_is_blocked() {
    let mut m = ref_manifest();
    m.schema_id = "turingosv4.benchmark_manifest.v0".into();
    match m.validate() {
        Err(BenchmarkManifestError::SchemaIdMismatch(s)) => {
            assert_eq!(s, "turingosv4.benchmark_manifest.v0");
        }
        other => panic!("expected SchemaIdMismatch, got {other:?}"),
    }
}

/// FR-18B.1 — pin-fields are positional invariants; if ANY required field is
/// empty / zero / dup / non-hex, validation MUST fire. We sweep the field
/// space and assert each canonical mutation produces some error (not Ok).
#[test]
fn benchmark_manifest_no_field_omission_allowed() {
    // Each mutator below empties / zeros / breaks one required field; all
    // must be ship-block.
    let mutators: Vec<(&str, fn(&mut BenchmarkManifest))> = vec![
        ("batch_id", |m| m.batch_id.clear()),
        ("problem_ids", |m| m.problem_ids.clear()),
        ("model_id", |m| m.model_id.clear()),
        ("model_ver", |m| m.model_ver.clear()),
        ("temperature_decimal", |m| m.temperature_decimal.clear()),
        ("max_tx_budget", |m| m.max_tx_budget = 0),
        ("n_per_problem", |m| m.n_per_problem = 0),
        ("seeds", |m| m.seeds.clear()),
        ("lean_version", |m| m.lean_version.clear()),
        ("mathlib_commit", |m| m.mathlib_commit = "short".into()),
        ("turingos_commit", |m| m.turingos_commit = "short".into()),
        ("strategy", |m| m.strategy.clear()),
    ];
    for (name, mutate) in mutators {
        let mut m = ref_manifest();
        mutate(&mut m);
        assert!(
            m.validate().is_err(),
            "CR-18B.5 violation: omitting field `{name}` produced Ok validation"
        );
    }
}

/// FR-18B.1 — `total_runs()` is the contract: `n_problems × n_per_problem ×
/// n_seeds`. The aggregate runner uses this number to assert
/// `chain_invariant.json` count matches the manifest. A drift here would
/// produce a phantom "all 50/50 OK" report on a partial run.
#[test]
fn benchmark_manifest_total_runs_arithmetic_is_stable() {
    let cases = [
        (1u64, 1u32, 1usize, 1u64),
        (2, 1, 1, 2),
        (1, 3, 1, 3),
        (1, 1, 3, 3),
        (50, 3, 3, 450),     // M1 = 50 × 3 × 3 = 450
        (100, 3, 6, 1800),   // M2 = 100 × 3 × 3 × 2 (2 models = 2 seeds *2 ≈ 6 effective seeds)
        (100, 3, 3, 900),    // M2 single-model = 100 × 3 × 3 = 900
    ];
    for (n_problems, n, n_seeds, expected) in cases {
        let mut m = ref_manifest();
        m.problem_ids = (0..n_problems).map(|i| format!("p_{i}")).collect();
        m.n_per_problem = n;
        m.seeds = (0..(n_seeds as u64)).collect();
        m.validate().unwrap_or_else(|e| {
            panic!("synth manifest n_problems={n_problems} n={n} seeds={n_seeds}: {e}")
        });
        assert_eq!(
            m.total_runs(),
            expected,
            "total_runs drift: n_problems={n_problems} n={n} seeds={n_seeds}"
        );
    }
}

/// CR-18B.3 — disk format round-trip is byte-stable across write/read for any
/// well-formed manifest. Catches silent serde drift that would invalidate
/// archived manifests.
#[test]
fn benchmark_manifest_disk_round_trip_is_stable() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("BENCHMARK_MANIFEST.json");
    let m = ref_manifest();
    m.write_json(&path).expect("write");
    let m2 = BenchmarkManifest::read_validated(&path).expect("read+validate");
    assert_eq!(m, m2);
    // Re-serialize and compare to ensure the JSON pretty form is also stable.
    let body1 = std::fs::read_to_string(&path).expect("read body");
    m2.write_json(&path).expect("rewrite");
    let body2 = std::fs::read_to_string(&path).expect("re-read body");
    assert_eq!(body1, body2, "JSON pretty form drifted across rewrite");
}

/// Defense-in-depth — schema_id pin is at the constitutional gate level so a
/// future schema bump is detectable even if downstream tooling forgets to
/// validate.
#[test]
fn benchmark_manifest_schema_id_pin_is_constitutional() {
    assert_eq!(
        BENCHMARK_MANIFEST_SCHEMA_ID,
        "turingosv4.benchmark_manifest.v1",
        "Constitution gate: BENCHMARK_MANIFEST_SCHEMA_ID drift caught at gate level."
    );
}
