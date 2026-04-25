// PPUT-CCL Phase B B3 — T_i wall-clock instrumentation.
//
// Spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B3
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (time definition)
//   PREREG § 3 anti-Goodhart conformance (test_wall_clock_first_read_to_final_accept)
//
// Definition (PREREG § 5):
//   T_i = end_time − start_time
//     start_time = first read of task statement by any agent in the run
//     end_time   = final ground-truth Lean accept OR external timeout
//
// Why this is its own module rather than `start.elapsed()` inline:
//   1. Excludes evaluator-side preflight (kernel construction, tool mounting,
//      wallet load) that isn't agent-observable work.
//   2. Bracket extends through the FINAL Lean call — including a post-hoc
//      verifier added in B4 that may run AFTER runtime accept. The plan-
//      stated Goodhart concern: a Soft Law mode that fakes runtime accept
//      could exit early before post-hoc verify if the bracket closes at
//      runtime accept. `mark_final_accept` is called by the evaluator AFTER
//      the last verify call returns — making the bracket honest under that
//      attack.
//   3. Testable in isolation: synthetic Instants drive the conformance test
//      without `thread::sleep` flakiness.

use std::time::Instant;

/// Per-run wall-clock bracket.
///
/// Construct one at function entry, call `mark_first_read` at the first agent
/// prompt construction, call `mark_final_accept` after the last Lean call
/// returns, then read `elapsed_ms()` at jsonl emit.
#[derive(Debug, Clone, Copy)]
pub struct RunWallClock {
    first_read: Option<Instant>,
    final_accept: Option<Instant>,
}

impl Default for RunWallClock {
    fn default() -> Self { Self::new() }
}

impl RunWallClock {
    pub fn new() -> Self {
        Self { first_read: None, final_accept: None }
    }

    /// Stamp the bracket open at first agent prompt construction.
    /// Idempotent: subsequent calls are no-ops so the FIRST read wins
    /// regardless of which call site fires it (oneshot vs swarm tx 0).
    pub fn mark_first_read(&mut self) {
        if self.first_read.is_none() {
            self.first_read = Some(Instant::now());
        }
    }

    /// Stamp the bracket closed after the final Lean call.
    /// NOT idempotent — every call updates the close instant, so the LAST
    /// final-Lean call wins (matters when B4 adds post-hoc verify after
    /// runtime accept).
    pub fn mark_final_accept(&mut self) {
        self.final_accept = Some(Instant::now());
    }

    /// T_i in milliseconds.
    /// Returns None if the bracket never opened. If only `first_read` is set
    /// (run aborted before any final accept) returns elapsed-since-first-read,
    /// which is the right thing for the no-OMEGA exit at max_transactions.
    pub fn elapsed_ms(&self) -> Option<u64> {
        match (self.first_read, self.final_accept) {
            (Some(start), Some(end)) => {
                Some(end.saturating_duration_since(start).as_millis() as u64)
            }
            (Some(start), None) => Some(start.elapsed().as_millis() as u64),
            _ => None,
        }
    }

    /// Test-only constructor to inject specific Instants (deterministic
    /// timing for the conformance battery — avoids `thread::sleep` flake).
    #[cfg(test)]
    pub fn from_instants(first_read: Instant, final_accept: Instant) -> Self {
        Self {
            first_read: Some(first_read),
            final_accept: Some(final_accept),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// PREREG § 3 anti-Goodhart conformance:
    /// "synthetic run with 100ms prompt construction + 5s LLM call + 2s Lean
    /// verify → assert wall_time_ms ≥ 7100"
    /// Uses Instant arithmetic so the test runs in microseconds rather than
    /// 7 real seconds — the bracket math is what's being tested.
    ///
    /// Mid-term audit P0-C fix 2026-04-25: the bracket opens BEFORE prompt
    /// construction in the evaluator wiring, so the synthetic test mirrors
    /// that — `first_read = t0` and the 100ms prompt construction is
    /// INSIDE the bracket. Total = 100ms + 5s + 2s = 7100ms. The prior
    /// implementation marked first_read AFTER construction, which forced
    /// this test to relax to ≥7000ms — that relaxation is now removed.
    #[test]
    fn test_wall_clock_first_read_to_final_accept() {
        let t0 = Instant::now();
        let prompt_construction = Duration::from_millis(100);
        let llm_call = Duration::from_secs(5);
        let lean_verify = Duration::from_secs(2);

        // first_read fires BEFORE prompt construction (matching evaluator wiring).
        let first_read = t0;
        let final_accept = first_read + prompt_construction + llm_call + lean_verify;

        let wc = RunWallClock::from_instants(first_read, final_accept);
        let elapsed = wc.elapsed_ms().expect("bracket closed");

        // Plan B3 strict assertion: bracket must include prompt construction
        // (100ms) + LLM call (5s) + Lean verify (2s) = 7100ms minimum.
        assert!(elapsed >= 7100,
            "wall_time_ms must include prompt + LLM + Lean (≥ 7100ms), got {}", elapsed);
        // Upper bound catches over-counting bugs (e.g., double-bracketed Lean).
        assert!(elapsed <= 7200,
            "wall_time_ms must not double-count (≤ 7200ms slack), got {}", elapsed);
    }

    #[test]
    fn test_wall_clock_first_read_idempotent() {
        let mut wc = RunWallClock::new();
        wc.mark_first_read();
        let first = wc.first_read.expect("set");
        std::thread::sleep(Duration::from_millis(2));
        wc.mark_first_read(); // should be no-op
        assert_eq!(wc.first_read, Some(first),
            "first_read must be set once; later calls no-op");
    }

    #[test]
    fn test_wall_clock_final_accept_overwrites() {
        // B4 will add a post-hoc verifier called AFTER runtime accept. If
        // both fire mark_final_accept, the LATER instant must win so the
        // bracket includes the trailing verify.
        let mut wc = RunWallClock::new();
        wc.mark_first_read();
        wc.mark_final_accept();
        let first_close = wc.final_accept.expect("set");
        std::thread::sleep(Duration::from_millis(2));
        wc.mark_final_accept();
        let second_close = wc.final_accept.expect("set");
        assert!(second_close > first_close,
            "final_accept must update on each call; second must be later");
    }

    #[test]
    fn test_wall_clock_no_final_accept_uses_now() {
        // No-OMEGA exit path: only first_read is marked. elapsed_ms must
        // return time since first_read so the jsonl row still carries T_i.
        let mut wc = RunWallClock::new();
        wc.mark_first_read();
        std::thread::sleep(Duration::from_millis(5));
        let e = wc.elapsed_ms().expect("first_read set");
        assert!(e >= 5, "elapsed must reflect time since first_read, got {}ms", e);
    }

    #[test]
    fn test_wall_clock_unmarked_returns_none() {
        let wc = RunWallClock::new();
        assert!(wc.elapsed_ms().is_none(),
            "elapsed_ms must be None before first_read is marked");
    }
}
