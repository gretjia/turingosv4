//! BenchmarkManifest — pin-set for scaled benchmark runs.
//!
//! Per `feedback_benchmark_manifest_required` + TB-18B charter FR-18B.1:
//! "Scaled benchmark (50+ × n>1 × multi-seed) MUST pin problem_ids/model+ver/
//! temp/budget/seed/Lean+mathlib/TuringOS commits BEFORE batch."
//!
//! Per CR-18B.5: NO BenchmarkManifest field omission. Missing fields = ship-block.
//!
//! This module provides a canonical schema for the manifest plus a strict
//! validator. The manifest is written ONCE before the batch and ANY field
//! omission causes `validate()` to fail. Aggregate runners read the manifest
//! to verify run conditions and to assert post-run invariants (e.g.
//! `turingos_commit` matches the binary that produced the run).
//!
//! `FC-trace: Art-0.2 Tape Canonical (manifest is per-batch metadata, not
//! ChainTape; lives in evidence dir alongside per-problem chain_invariant.json)`.
//! `Phase-tag: P3 Lean Proof Task Market — first formal benchmark scale-up`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; report-side helper, not chain-resident): TB-18B FR-18B.1 / CR-18B.5 — pin-set required before any 50+ × n>1 × multi-seed benchmark batch. Constitutional Justification: `feedback_benchmark_manifest_required` + TB-18B charter §3 line 47-48.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkManifest {
    /// Batch identifier (e.g. "tb_18b_m1_2026-05-08T..."). Pinned to the
    /// evidence dir name; downstream tooling cross-references.
    pub batch_id: String,
    /// MiniF2F problem IDs included in this batch. Sorted, deduplicated. The
    /// manifest pins WHICH problems were run; missing IDs = silent exclusion.
    pub problem_ids: Vec<String>,
    /// LLM model identity (e.g. "deepseek-chat", "deepseek-reasoner").
    pub model_id: String,
    /// LLM model version string (e.g. "2024-12-26", "v3"). Distinct from
    /// `model_id` so future re-runs can reconstruct the exact configuration.
    pub model_ver: String,
    /// Sampling temperature. Float represented as a decimal string for
    /// determinism (no f64 normalization drift across serializations).
    pub temperature_decimal: String,
    /// MAX_TX budget per run (terminal halt boundary).
    pub max_tx_budget: u64,
    /// Per-problem repetition count (n). M1 = 3, M2 = 3.
    pub n_per_problem: u32,
    /// Random seeds for the multi-seed sweep. Sorted, deduplicated.
    pub seeds: Vec<u64>,
    /// Lean version (e.g. "4.x.y"). Pinned to detect Lean upgrade drift.
    pub lean_version: String,
    /// Mathlib commit hash (40-hex). Pinned to detect mathlib drift.
    pub mathlib_commit: String,
    /// TuringOS commit hash (40-hex). Pinned to detect runtime drift between
    /// manifest write and batch run; aggregate runner asserts post-run match.
    pub turingos_commit: String,
    /// Sample / aggregation strategy declaration (per FR-18B.2 +
    /// `feedback_evidence_packaging_policy_required`). Free-form string;
    /// e.g. "M1: 50p × n=3 × 3-seed = 450 runs DeepSeek baseline".
    pub strategy: String,
    /// Schema id pin — version this struct shape so future bumps fire a
    /// detectable mismatch on old manifests.
    pub schema_id: String,
}

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B; Art-0.2): manifest schema id pin. Constitutional Justification: TB-18B charter §3 + `feedback_benchmark_manifest_required` schema-stability discipline.
pub const BENCHMARK_MANIFEST_SCHEMA_ID: &str = "turingosv4.benchmark_manifest.v1";

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B): validation error kinds for a `BenchmarkManifest`. One variant per CR-18B.5 ship-block field. Constitutional Justification: TB-18B charter CR-18B.5 + `feedback_benchmark_manifest_required`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchmarkManifestError {
    EmptyBatchId,
    EmptyProblemIds,
    DuplicateProblemId(String),
    EmptyModelId,
    EmptyModelVer,
    EmptyTemperature,
    InvalidTemperature(String),
    ZeroMaxTxBudget,
    ZeroN,
    EmptySeeds,
    DuplicateSeed(u64),
    EmptyLeanVersion,
    InvalidMathlibCommit(String),
    InvalidTuringosCommit(String),
    EmptyStrategy,
    SchemaIdMismatch(String),
}

impl std::fmt::Display for BenchmarkManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BenchmarkManifestError::*;
        match self {
            EmptyBatchId => write!(f, "BenchmarkManifest: empty batch_id"),
            EmptyProblemIds => write!(f, "BenchmarkManifest: problem_ids is empty"),
            DuplicateProblemId(id) => {
                write!(f, "BenchmarkManifest: duplicate problem_id {id:?}")
            }
            EmptyModelId => write!(f, "BenchmarkManifest: empty model_id"),
            EmptyModelVer => write!(f, "BenchmarkManifest: empty model_ver"),
            EmptyTemperature => {
                write!(f, "BenchmarkManifest: empty temperature_decimal")
            }
            InvalidTemperature(s) => write!(
                f,
                "BenchmarkManifest: invalid temperature_decimal {s:?} (not parseable as f64)"
            ),
            ZeroMaxTxBudget => write!(f, "BenchmarkManifest: max_tx_budget == 0"),
            ZeroN => write!(f, "BenchmarkManifest: n_per_problem == 0"),
            EmptySeeds => write!(f, "BenchmarkManifest: seeds is empty"),
            DuplicateSeed(s) => write!(f, "BenchmarkManifest: duplicate seed {s}"),
            EmptyLeanVersion => write!(f, "BenchmarkManifest: empty lean_version"),
            InvalidMathlibCommit(s) => write!(
                f,
                "BenchmarkManifest: invalid mathlib_commit {s:?} (expected 40-hex)"
            ),
            InvalidTuringosCommit(s) => write!(
                f,
                "BenchmarkManifest: invalid turingos_commit {s:?} (expected 40-hex)"
            ),
            EmptyStrategy => write!(f, "BenchmarkManifest: empty strategy"),
            SchemaIdMismatch(s) => write!(
                f,
                "BenchmarkManifest: schema_id {s:?} != {BENCHMARK_MANIFEST_SCHEMA_ID}"
            ),
        }
    }
}

impl std::error::Error for BenchmarkManifestError {}

fn is_40_hex(s: &str) -> bool {
    s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit())
}

impl BenchmarkManifest {
    /// TRACE_MATRIX § 3 orphan (CR-18B.5): strict validator. Every field must be
    /// populated; missing fields are ship-block. Returns the FIRST error
    /// encountered (deterministic ordering for caller diagnostics).
    pub fn validate(&self) -> Result<(), BenchmarkManifestError> {
        use BenchmarkManifestError::*;

        if self.schema_id != BENCHMARK_MANIFEST_SCHEMA_ID {
            return Err(SchemaIdMismatch(self.schema_id.clone()));
        }
        if self.batch_id.trim().is_empty() {
            return Err(EmptyBatchId);
        }
        if self.problem_ids.is_empty() {
            return Err(EmptyProblemIds);
        }
        let mut seen_pid: BTreeSet<&str> = BTreeSet::new();
        for pid in &self.problem_ids {
            if !seen_pid.insert(pid.as_str()) {
                return Err(DuplicateProblemId(pid.clone()));
            }
        }
        if self.model_id.trim().is_empty() {
            return Err(EmptyModelId);
        }
        if self.model_ver.trim().is_empty() {
            return Err(EmptyModelVer);
        }
        if self.temperature_decimal.trim().is_empty() {
            return Err(EmptyTemperature);
        }
        if self.temperature_decimal.parse::<f64>().is_err() {
            return Err(InvalidTemperature(self.temperature_decimal.clone()));
        }
        if self.max_tx_budget == 0 {
            return Err(ZeroMaxTxBudget);
        }
        if self.n_per_problem == 0 {
            return Err(ZeroN);
        }
        if self.seeds.is_empty() {
            return Err(EmptySeeds);
        }
        let mut seen_seed: BTreeSet<u64> = BTreeSet::new();
        for s in &self.seeds {
            if !seen_seed.insert(*s) {
                return Err(DuplicateSeed(*s));
            }
        }
        if self.lean_version.trim().is_empty() {
            return Err(EmptyLeanVersion);
        }
        if !is_40_hex(&self.mathlib_commit) {
            return Err(InvalidMathlibCommit(self.mathlib_commit.clone()));
        }
        if !is_40_hex(&self.turingos_commit) {
            return Err(InvalidTuringosCommit(self.turingos_commit.clone()));
        }
        if self.strategy.trim().is_empty() {
            return Err(EmptyStrategy);
        }
        Ok(())
    }

    /// TRACE_MATRIX § 3 orphan: total-runs derivation
    /// `n_problems × n_per_problem × n_seeds`. The aggregate runner asserts
    /// `chain_invariant.json` count matches this product.
    pub fn total_runs(&self) -> u64 {
        (self.problem_ids.len() as u64) * (self.n_per_problem as u64) * (self.seeds.len() as u64)
    }

    /// TRACE_MATRIX § 3 orphan: serialize to canonical pretty JSON for
    /// human-readable evidence packaging. Manifest lives at
    /// `<evidence_dir>/BENCHMARK_MANIFEST.json`.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// TRACE_MATRIX § 3 orphan: write to disk path; creates parent dir.
    pub fn write_json(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = self
            .to_json_pretty()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// TRACE_MATRIX § 3 orphan: read from disk + validate.
    pub fn read_validated(path: &Path) -> Result<Self, BenchmarkManifestError> {
        let body = std::fs::read_to_string(path).map_err(|e| {
            BenchmarkManifestError::EmptyBatchId // surface IO via the most-likely-empty kind for caller summary; full IO retained via to_string
                .clone_with_io(&e)
        })?;
        let m: Self = serde_json::from_str(&body)
            .map_err(|e| BenchmarkManifestError::SchemaIdMismatch(format!("parse error: {e}")))?;
        m.validate()?;
        Ok(m)
    }
}

// Helper to enrich an error variant with IO context. Implemented as a private
// trait method to avoid leaking std::io::Error into the public error enum.
trait CloneWithIo {
    fn clone_with_io(&self, e: &std::io::Error) -> Self;
}

impl CloneWithIo for BenchmarkManifestError {
    fn clone_with_io(&self, e: &std::io::Error) -> Self {
        BenchmarkManifestError::SchemaIdMismatch(format!("io error: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_manifest() -> BenchmarkManifest {
        BenchmarkManifest {
            batch_id: "tb_18b_m1_2026-05-08T00-00-00Z".into(),
            problem_ids: vec!["mathd_algebra_107".into(), "mathd_algebra_113".into()],
            model_id: "deepseek-chat".into(),
            model_ver: "v3".into(),
            temperature_decimal: "0.7".into(),
            max_tx_budget: 64,
            n_per_problem: 3,
            seeds: vec![1, 2, 3],
            lean_version: "4.7.0".into(),
            mathlib_commit: "0".repeat(40),
            turingos_commit: "1".repeat(40),
            strategy: "M1: 2p × n=3 × 3-seed = 18 runs DeepSeek baseline".into(),
            schema_id: BENCHMARK_MANIFEST_SCHEMA_ID.to_string(),
        }
    }

    #[test]
    fn good_manifest_validates() {
        let m = good_manifest();
        m.validate().expect("good manifest validates");
        assert_eq!(m.total_runs(), 2 * 3 * 3);
    }

    #[test]
    fn empty_batch_id_is_blocked() {
        let mut m = good_manifest();
        m.batch_id = "".into();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptyBatchId));
    }

    #[test]
    fn empty_problem_ids_is_blocked() {
        let mut m = good_manifest();
        m.problem_ids.clear();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptyProblemIds));
    }

    #[test]
    fn duplicate_problem_id_is_blocked() {
        let mut m = good_manifest();
        m.problem_ids.push("mathd_algebra_107".into());
        assert_eq!(
            m.validate(),
            Err(BenchmarkManifestError::DuplicateProblemId(
                "mathd_algebra_107".into()
            ))
        );
    }

    #[test]
    fn empty_model_id_is_blocked() {
        let mut m = good_manifest();
        m.model_id = "".into();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptyModelId));
    }

    #[test]
    fn empty_model_ver_is_blocked() {
        let mut m = good_manifest();
        m.model_ver = "".into();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptyModelVer));
    }

    #[test]
    fn empty_temperature_is_blocked() {
        let mut m = good_manifest();
        m.temperature_decimal = "".into();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptyTemperature));
    }

    #[test]
    fn invalid_temperature_is_blocked() {
        let mut m = good_manifest();
        m.temperature_decimal = "not_a_number".into();
        match m.validate() {
            Err(BenchmarkManifestError::InvalidTemperature(s)) => {
                assert_eq!(s, "not_a_number")
            }
            other => panic!("expected InvalidTemperature, got {other:?}"),
        }
    }

    #[test]
    fn zero_max_tx_is_blocked() {
        let mut m = good_manifest();
        m.max_tx_budget = 0;
        assert_eq!(m.validate(), Err(BenchmarkManifestError::ZeroMaxTxBudget));
    }

    #[test]
    fn zero_n_is_blocked() {
        let mut m = good_manifest();
        m.n_per_problem = 0;
        assert_eq!(m.validate(), Err(BenchmarkManifestError::ZeroN));
    }

    #[test]
    fn empty_seeds_is_blocked() {
        let mut m = good_manifest();
        m.seeds.clear();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptySeeds));
    }

    #[test]
    fn duplicate_seed_is_blocked() {
        let mut m = good_manifest();
        m.seeds.push(1);
        assert_eq!(m.validate(), Err(BenchmarkManifestError::DuplicateSeed(1)));
    }

    #[test]
    fn empty_lean_version_is_blocked() {
        let mut m = good_manifest();
        m.lean_version = "".into();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptyLeanVersion));
    }

    #[test]
    fn invalid_mathlib_commit_is_blocked() {
        let mut m = good_manifest();
        m.mathlib_commit = "short".into();
        match m.validate() {
            Err(BenchmarkManifestError::InvalidMathlibCommit(s)) => {
                assert_eq!(s, "short")
            }
            other => panic!("expected InvalidMathlibCommit, got {other:?}"),
        }
    }

    #[test]
    fn invalid_turingos_commit_is_blocked() {
        let mut m = good_manifest();
        m.turingos_commit = "ZZZ".repeat(20); // 60 chars, non-hex
        match m.validate() {
            Err(BenchmarkManifestError::InvalidTuringosCommit(_)) => {}
            other => panic!("expected InvalidTuringosCommit, got {other:?}"),
        }
    }

    #[test]
    fn empty_strategy_is_blocked() {
        let mut m = good_manifest();
        m.strategy = "".into();
        assert_eq!(m.validate(), Err(BenchmarkManifestError::EmptyStrategy));
    }

    #[test]
    fn schema_id_mismatch_is_blocked() {
        let mut m = good_manifest();
        m.schema_id = "turingosv4.benchmark_manifest.v0".into();
        match m.validate() {
            Err(BenchmarkManifestError::SchemaIdMismatch(s)) => {
                assert_eq!(s, "turingosv4.benchmark_manifest.v0")
            }
            other => panic!("expected SchemaIdMismatch, got {other:?}"),
        }
    }

    #[test]
    fn round_trip_json() {
        let m = good_manifest();
        let json = m.to_json_pretty().expect("to_json");
        let m2: BenchmarkManifest = serde_json::from_str(&json).expect("from_json");
        assert_eq!(m, m2);
        m2.validate().expect("round-trip validates");
    }

    #[test]
    fn write_read_disk_round_trip() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("BENCHMARK_MANIFEST.json");
        let m = good_manifest();
        m.write_json(&path).expect("write");
        let m2 = BenchmarkManifest::read_validated(&path).expect("read");
        assert_eq!(m, m2);
    }

    #[test]
    fn schema_id_constant_pinned() {
        // Defense-in-depth: pin the schema id string so a future rename fires.
        assert_eq!(
            BENCHMARK_MANIFEST_SCHEMA_ID,
            "turingosv4.benchmark_manifest.v1"
        );
    }
}
