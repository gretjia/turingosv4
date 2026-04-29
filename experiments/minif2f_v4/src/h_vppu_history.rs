// TB-1 Day-4 (2026-04-29): per-problem rolling history of pput_verified,
// used to compute the H-VPPUT North Star metric per PREREG § 5.
//
// Authoritative spec: handover/tracer_bullets/TB-1_recharter_2026-04-29.md
// (Day 4 — P6 instrumentation: h_vppu computation).
//
// Semantics (per recharter § 2 Day 4):
//   h_vppu = current_pput_verified / mean(history N=1..3)
//
// Returns None when:
//   - no prior runs exist for this problem (first run; no signal),
//   - all prior pput_verified values sum to 0 (mean=0; ratio undefined).
//
// History is capped at 3 prior runs per problem (rolling window; oldest
// drops on push). This matches the recharter spec and keeps the per-
// problem signal fresh without unbounded growth.
//
// Persistence: caller passes a path to load_from / save_to. The store
// is JSON-encoded (one HashMap<String, VecDeque<f64>>); a missing or
// unreadable file degrades to an empty store so the first ever run
// against any environment never panics.
//
// FC-trace: FC1-N11 (∏p decision diversity) — h_vppu measures
// per-problem regression vs prior runs; runs that re-attempt with no
// learning produce h_vppu = current/mean ≈ 1.0 or below. Step-4
// Capability Compilation should drive h_vppu > 1 on heldout.

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

const HISTORY_CAPACITY: usize = 3;

/// TRACE_MATRIX orphan (P6 instrumentation; PREREG_PPUT_CCL_2026-04-26.md § 5
/// H-VPPUT North Star): per-problem rolling history of `pput_verified`
/// values used to compute the held-out verified PPUT regression ratio
/// emitted on `PputResult.h_vppu`. Not a constitutional flowchart node;
/// justified as an Epistemic Lab v0 product-line metric per the 9-phase
/// roadmap (`handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HVppuHistory {
    /// problem_id → rolling history of pput_verified values from prior runs.
    /// Newest pushed at back; oldest popped at front when len > capacity.
    by_problem: HashMap<String, VecDeque<f64>>,
}

impl HVppuHistory {
    /// TRACE_MATRIX orphan (P6 instrumentation): empty constructor;
    /// callers prefer load_from for persisted history.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// load from disk. Returns Self::default() on missing or unreadable
    /// file (graceful degradation; H-VPPUT is a non-blocking P6 metric per
    /// recharter Day 5 Tier-B). A corrupt store logs to stderr and starts
    /// fresh rather than panicking — instrumentation must not block runs.
    pub fn load_from(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
                eprintln!(
                    "[h_vppu_history] corrupt store at {:?} ({}); starting fresh",
                    path, e
                );
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// save to disk (atomic-ish: write to tmp then rename). Returns io::Error
    /// on failure; caller decides fail-loud vs log-and-continue. P6
    /// instrumentation should not block the ship path on a missing disk.
    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        let tmp_path = path.with_extension("json.tmp");
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(&tmp_path, serialized)?;
        fs::rename(&tmp_path, path)
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// append the current run's pput_verified to the per-problem history.
    /// Trims to HISTORY_CAPACITY (3) — newest at back, oldest dropped at
    /// front. Idempotent only with respect to identical values; callers
    /// that want at-most-once semantics across retries must dedupe.
    pub fn record(&mut self, problem_id: &str, pput_verified: f64) {
        let entry = self
            .by_problem
            .entry(problem_id.to_string())
            .or_default();
        entry.push_back(pput_verified);
        while entry.len() > HISTORY_CAPACITY {
            entry.pop_front();
        }
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG_PPUT_CCL_2026-04-26.md § 5
    /// H-VPPUT definition): compute h_vppu = current / mean(history) when
    /// there is at least one prior run AND that mean is non-zero. The
    /// current run's value is NOT included in the mean — h_vppu measures
    /// improvement against a held-out baseline, not against itself.
    ///
    /// Returns None when:
    ///   - no history exists for this problem (first run);
    ///   - the prior history mean is 0 (all prior runs failed; ratio
    ///     undefined — None preserves "no signal" semantics rather than
    ///     emitting NaN/inf into the JSONL row).
    pub fn h_vppu_for(&self, problem_id: &str, current_pput_verified: f64) -> Option<f64> {
        let entry = self.by_problem.get(problem_id)?;
        if entry.is_empty() {
            return None;
        }
        let n = entry.len() as f64;
        let sum: f64 = entry.iter().sum();
        let mean = sum / n;
        if mean == 0.0 {
            return None;
        }
        Some(current_pput_verified / mean)
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// number of prior runs stored for a given problem. Exposed for
    /// tests + downstream auditors that want to assert capacity-3
    /// invariants without round-tripping through JSON.
    pub fn history_len(&self, problem_id: &str) -> usize {
        self.by_problem
            .get(problem_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Generate a unique tmp path under std::env::temp_dir() without
    /// pulling in the `tempfile` crate. Each test gets its own.
    fn unique_tmp_path(label: &str) -> std::path::PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "h_vppu_history_{}_{}_{}.json",
            label, nanos, seq
        ))
    }

    #[test]
    fn test_first_run_returns_none() {
        let history = HVppuHistory::new();
        assert_eq!(
            history.h_vppu_for("mathd_algebra_107", 0.5),
            None,
            "no prior runs → None (no signal)"
        );
    }

    #[test]
    fn test_second_run_returns_ratio() {
        let mut history = HVppuHistory::new();
        history.record("mathd_algebra_107", 0.4);
        // Current run pput_verified = 0.6 → h_vppu = 0.6/0.4 = 1.5
        let h = history
            .h_vppu_for("mathd_algebra_107", 0.6)
            .expect("one prior run gives a ratio");
        assert!(
            (h - 1.5).abs() < 1e-12,
            "h_vppu = current/mean = 0.6/0.4 = 1.5, got {}",
            h
        );
    }

    #[test]
    fn test_capacity_3_rolling_window() {
        let mut history = HVppuHistory::new();
        for v in [0.1, 0.2, 0.3, 0.4, 0.5] {
            history.record("p1", v);
        }
        assert_eq!(
            history.history_len("p1"),
            HISTORY_CAPACITY,
            "rolling window keeps only last 3"
        );
        // Only the last 3 (0.3, 0.4, 0.5) survive; mean = 0.4
        let h = history.h_vppu_for("p1", 0.4).unwrap();
        assert!(
            (h - 1.0).abs() < 1e-12,
            "0.4 / mean(0.3,0.4,0.5) = 0.4/0.4 = 1.0, got {}",
            h
        );
    }

    #[test]
    fn test_zero_mean_returns_none() {
        let mut history = HVppuHistory::new();
        history.record("p1", 0.0);
        history.record("p1", 0.0);
        // mean=0 → ratio undefined → None (anti-Goodhart: never emit NaN/inf)
        assert_eq!(history.h_vppu_for("p1", 0.5), None);
    }

    #[test]
    fn test_per_problem_isolation() {
        let mut history = HVppuHistory::new();
        history.record("p1", 0.2);
        history.record("p2", 0.8);
        // p1's history must NOT pollute p2's ratio.
        let h_p1 = history.h_vppu_for("p1", 0.4).unwrap(); // 0.4 / 0.2 = 2.0
        let h_p2 = history.h_vppu_for("p2", 0.8).unwrap(); // 0.8 / 0.8 = 1.0
        assert!((h_p1 - 2.0).abs() < 1e-12);
        assert!((h_p2 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_persistence_roundtrip() {
        let path = unique_tmp_path("roundtrip");
        let mut h1 = HVppuHistory::new();
        h1.record("p1", 0.4);
        h1.record("p2", 0.7);
        h1.save_to(&path).expect("save");

        let h2 = HVppuHistory::load_from(&path);
        assert!((h2.h_vppu_for("p1", 0.6).unwrap() - 1.5).abs() < 1e-12);
        assert!((h2.h_vppu_for("p2", 1.4).unwrap() - 2.0).abs() < 1e-12);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_load_missing_file_default() {
        let path = unique_tmp_path("missing");
        // Path does not exist; load must degrade to empty.
        let h = HVppuHistory::load_from(&path);
        assert_eq!(h.h_vppu_for("any", 1.0), None);
    }

    #[test]
    fn test_corrupt_file_degrades_to_default() {
        let path = unique_tmp_path("corrupt");
        fs::write(&path, "{not valid json").unwrap();
        let h = HVppuHistory::load_from(&path);
        assert_eq!(
            h.h_vppu_for("any", 1.0),
            None,
            "corrupt store must not panic; returns empty"
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_record_before_query_does_not_self_reference() {
        // Verify h_vppu_for does NOT include the in-flight value when
        // the caller passes (current, then record). This ordering is
        // load → query → record → save (per evaluator wire site).
        let mut history = HVppuHistory::new();
        history.record("p1", 0.4);
        let h_before_record = history.h_vppu_for("p1", 0.6).unwrap();
        history.record("p1", 0.6);
        let h_after_record = history.h_vppu_for("p1", 0.6).unwrap();
        // Before record: 0.6 / 0.4 = 1.5
        // After record: 0.6 / mean(0.4, 0.6) = 0.6 / 0.5 = 1.2
        assert!((h_before_record - 1.5).abs() < 1e-12);
        assert!((h_after_record - 1.2).abs() < 1e-12);
    }
}
