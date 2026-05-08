//! Wilson score 95% confidence interval for binomial proportions.
//!
//! Closes Art. I.2 PPUT report discipline AMBER (kill: "report missing ΣPPUT +
//! Mean-PPUT(solved) + Wilson 95% CI"). Per CLAUDE.md §17 Report Standard:
//! "95% CI if reporting aggregate".
//!
//! Wilson score interval is preferred over Wald (normal approximation) because
//! it handles edge cases (k=0, k=n) without producing nonsensical intervals
//! that extend beyond [0, 1]. For solve-count proportions in MiniF2F
//! benchmarks where k can be 0/n at small batch sizes, Wilson is mandatory.
//!
//! `FC-trace: Art.I.2 — Statistical signal (PPUT / reputation / consensus)`.

/// Z-score for 95% two-sided confidence (1.96 standard).
const Z_95: f64 = 1.959_963_984_540_054_0;

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08): Wilson 95% confidence interval for a binomial proportion. Closes Art. I.2 PPUT report discipline AMBER (CLAUDE.md §17 Report Standard).
///
/// `successes` and `trials` are non-negative counts; `successes <= trials` is
/// required (panics in debug, clamped in release to `successes = trials`).
///
/// Returns `None` when `trials == 0` (no inference possible). Otherwise returns
/// `Some(WilsonCi { lower, upper, point })` where the interval covers the true
/// proportion with ~95% probability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WilsonCi {
    pub successes: u32,
    pub trials: u32,
    pub point: f64,
    pub lower: f64,
    pub upper: f64,
}

impl WilsonCi {
    /// TRACE_MATRIX § 3 orphan (Art. I.2): compute Wilson 95% CI. Returns `None` when `trials == 0`.
    pub fn new_95(successes: u32, trials: u32) -> Option<Self> {
        if trials == 0 {
            return None;
        }
        let k = successes.min(trials) as f64;
        let n = trials as f64;
        let z = Z_95;
        let z2 = z * z;
        let p_hat = k / n;
        let denom = 1.0 + z2 / n;
        let center = (p_hat + z2 / (2.0 * n)) / denom;
        let margin =
            z * ((p_hat * (1.0 - p_hat) / n + z2 / (4.0 * n * n)).sqrt()) / denom;
        let lower = (center - margin).max(0.0);
        let upper = (center + margin).min(1.0);
        Some(Self {
            successes,
            trials,
            point: p_hat,
            lower,
            upper,
        })
    }

    /// TRACE_MATRIX § 3 orphan (Art. I.2): format as a human-readable percentage interval, e.g. "25.0% (CI 16.2%–36.4%)".
    pub fn format_percent(&self) -> String {
        format!(
            "{:.1}% (CI {:.1}%–{:.1}%)",
            self.point * 100.0,
            self.lower * 100.0,
            self.upper * 100.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_trials_returns_none() {
        assert!(WilsonCi::new_95(0, 0).is_none());
    }

    #[test]
    fn zero_successes_lower_bound_is_zero() {
        let ci = WilsonCi::new_95(0, 50).expect("trials > 0");
        assert_eq!(ci.point, 0.0);
        assert!((ci.lower - 0.0).abs() < 1e-9);
        assert!(ci.upper > 0.0 && ci.upper < 0.1);
    }

    #[test]
    fn full_successes_upper_bound_is_one() {
        let ci = WilsonCi::new_95(50, 50).expect("trials > 0");
        assert_eq!(ci.point, 1.0);
        assert!((ci.upper - 1.0).abs() < 1e-9);
        assert!(ci.lower < 1.0 && ci.lower > 0.9);
    }

    #[test]
    fn balanced_50_of_100_centered_at_half() {
        let ci = WilsonCi::new_95(50, 100).expect("trials > 0");
        assert_eq!(ci.point, 0.5);
        assert!((ci.lower - 0.402).abs() < 0.005);
        assert!((ci.upper - 0.598).abs() < 0.005);
    }

    #[test]
    fn small_sample_25_of_100_known_interval() {
        let ci = WilsonCi::new_95(25, 100).expect("trials > 0");
        assert_eq!(ci.point, 0.25);
        assert!((ci.lower - 0.175).abs() < 0.005);
        assert!((ci.upper - 0.343).abs() < 0.005);
    }

    #[test]
    fn interval_width_shrinks_with_n() {
        let small = WilsonCi::new_95(5, 10).expect("trials > 0");
        let large = WilsonCi::new_95(500, 1000).expect("trials > 0");
        let small_width = small.upper - small.lower;
        let large_width = large.upper - large.lower;
        assert!(large_width < small_width);
    }

    #[test]
    fn lower_is_never_above_point_and_upper_never_below() {
        for &(k, n) in &[(0, 1), (1, 50), (10, 100), (99, 100), (100, 100)] {
            let ci = WilsonCi::new_95(k, n).expect("trials > 0");
            assert!(ci.lower <= ci.point, "lower > point at k={} n={}", k, n);
            assert!(ci.point <= ci.upper, "point > upper at k={} n={}", k, n);
            assert!(ci.lower >= 0.0 && ci.upper <= 1.0);
        }
    }

    #[test]
    fn format_percent_renders_human_readable() {
        let ci = WilsonCi::new_95(25, 100).expect("trials > 0");
        let s = ci.format_percent();
        assert!(s.starts_with("25.0%"), "got {}", s);
        assert!(s.contains("CI "));
        assert!(s.contains("%"));
    }

    #[test]
    fn successes_clamped_when_exceeds_trials() {
        // Defensive: in release mode, clamp instead of producing NaN.
        let ci = WilsonCi::new_95(150, 100).expect("trials > 0");
        assert_eq!(ci.point, 1.0);
    }
}
