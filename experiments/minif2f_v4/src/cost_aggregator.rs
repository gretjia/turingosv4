// PPUT-CCL Phase B B2 — C_i full-cost aggregator.
//
// Spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B2
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (cost definition)
//   PREREG § 3 anti-Goodhart conformance #8 (test_failed_branches_in_total_cost)
//
// Invariant being enforced:
//   total_run_token_count = Σ over EVERY proposal in the run of
//       (api_prompt_tokens + api_completion_tokens + tool_stdout_tokens)
//   — including failed parses, vetoed appends, rejected OMEGA claims, step
//   rejects, and any other tx that incurred an LLM call. Counting ONLY the
//   winning branch is the canonical Goodhart attack against PPUT (cheap
//   golden path achieved by burning many quietly-discarded branches), so
//   this aggregator is the ground truth that the conformance test gates.
//
// Token-counter source-of-truth (plan B2 open Q1):
//   - prompt / completion: post-hoc API-reported counts (extended through
//     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
//   - tool stdout: chars / 4 approximation (plan B2 open Q2 default; the
//     PPUT C_i is for accounting, not budgeting, so precision is not load-
//     bearing — the conformance battery is what enforces honesty).

/// Per-(run_id, problem_id) running token + branch totals.
///
/// Construct one per run, feed every LLM call + every tool-stdout emission,
/// then read totals at run end and stamp them onto the emitted jsonl row.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RunCostAccumulator {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    /// Tool stdout tokens (chars/4 heuristic).
    pub tool_tokens: u64,
    /// Every LLM call that returned a parsed proposal — winning OR losing.
    pub proposal_count: u32,
    /// Subset of proposal_count whose tx did not produce a verified accept.
    pub failed_branch_count: u32,
}

impl RunCostAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one LLM call's API-reported token usage.
    /// Call this AFTER `client.generate(...)` returns Ok — both for winning
    /// proposals and for ones that will be rejected/vetoed/parse-failed
    /// downstream. The call already happened; the tokens already cost.
    pub fn record_llm_call(&mut self, prompt_tokens: u32, completion_tokens: u32) {
        self.prompt_tokens += prompt_tokens as u64;
        self.completion_tokens += completion_tokens as u64;
    }

    /// Record bytes of agent-observable tool output (search hits, rejection
    /// error message preserved for next-prompt error feedback, etc.).
    /// chars/4 heuristic per plan B2 open Q2 default.
    pub fn record_tool_stdout(&mut self, stdout: &str) {
        let approx = (stdout.chars().count() as u64 + 3) / 4;
        self.tool_tokens += approx;
    }

    /// Mark one proposal attempt. `accepted = true` for the verified-success
    /// branch (typically called once per run on OMEGA accept); all other
    /// proposals (parse fails, vetoed appends, rejected OMEGAs, step rejects)
    /// pass `false` so they accrete failed_branch_count.
    pub fn record_proposal(&mut self, accepted: bool) {
        self.proposal_count += 1;
        if !accepted {
            self.failed_branch_count += 1;
        }
    }

    /// Convert the most-recent failed proposal into an accepted one. Used at
    /// the OMEGA-accept return path: every tx records as failed at parse time
    /// (since acceptance isn't known yet), then the verified-success branch
    /// flips the last record before returning.
    ///
    /// Mid-term audit P0-E fix 2026-04-25: prior implementation saturated
    /// silently at 0, which masked over-flip wiring bugs (Codex finding).
    /// Now panics if called with no failed proposal to flip — surfaces
    /// the bug at debug time. A correctly-wired evaluator pairs every flip
    /// with a prior record_proposal(false), so this assertion can never
    /// fire on a clean code path.
    pub fn flip_last_failed_to_accepted(&mut self) {
        assert!(
            self.failed_branch_count > 0,
            "flip_last_failed_to_accepted called with no failed proposal to flip — \
             wiring bug: caller fired flip more times than record_proposal(false). \
             A correct path records every parsed proposal as failed at parse time, \
             then flips the most recent on OMEGA-accept return."
        );
        self.failed_branch_count -= 1;
    }

    /// C_i — total tokens summed across every proposal in the run.
    pub fn total_run_token_count(&self) -> u64 {
        self.prompt_tokens + self.completion_tokens + self.tool_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PREREG § 3 anti-Goodhart conformance #8.
    /// Synthesize a run with 5 failed proposals + 1 success and assert the
    /// run total equals the sum of ALL six proposals' tokens, not just the
    /// winner's. This is the Goodhart attack surface PREREG locks down:
    /// counting only the accepted branch makes "many cheap rejects + one
    /// lucky accept" look like a high-PPUT run.
    #[test]
    fn test_failed_branches_counted_in_total_cost() {
        let mut acc = RunCostAccumulator::new();

        // 5 failed proposals: each costs 100 prompt + 50 completion + 20 tool stdout
        // (asymmetric on purpose — catches a mistake that conflates the three buckets).
        for _ in 0..5 {
            acc.record_llm_call(100, 50);
            acc.record_tool_stdout(&"x".repeat(80)); // 80 chars / 4 = 20 tokens
            acc.record_proposal(false);
        }

        // 1 successful proposal: 200 prompt + 100 completion + 0 tool stdout.
        acc.record_llm_call(200, 100);
        acc.record_proposal(true);

        let expected_prompt = 5 * 100 + 200;
        let expected_completion = 5 * 50 + 100;
        let expected_tool = 5 * 20;
        let expected_total = expected_prompt + expected_completion + expected_tool;

        assert_eq!(
            acc.prompt_tokens, expected_prompt as u64,
            "prompt tokens must include all 6 proposals"
        );
        assert_eq!(
            acc.completion_tokens, expected_completion as u64,
            "completion tokens must include all 6 proposals"
        );
        assert_eq!(
            acc.tool_tokens, expected_tool as u64,
            "tool stdout tokens must include all failed branches"
        );
        assert_eq!(
            acc.total_run_token_count(),
            expected_total as u64,
            "C_i must sum across ALL 6 proposals — failed branches included"
        );

        assert_eq!(acc.proposal_count, 6);
        assert_eq!(acc.failed_branch_count, 5);
    }

    #[test]
    fn test_empty_accumulator_zero_total() {
        let acc = RunCostAccumulator::new();
        assert_eq!(acc.total_run_token_count(), 0);
        assert_eq!(acc.proposal_count, 0);
        assert_eq!(acc.failed_branch_count, 0);
    }

    #[test]
    #[should_panic(expected = "flip_last_failed_to_accepted called with no failed proposal")]
    fn test_flip_underflow_panics() {
        // Mid-term audit P0-E: over-flipping must surface as a panic, not
        // silent saturation. A test fixture that calls flip without a prior
        // record_proposal(false) simulates the wiring bug Codex flagged.
        let mut acc = RunCostAccumulator::new();
        acc.flip_last_failed_to_accepted(); // BUG: no failed proposal in flight
    }

    #[test]
    fn test_tool_stdout_chars_div_4_approximation() {
        let mut acc = RunCostAccumulator::new();
        // 4 chars → 1 token (exact)
        acc.record_tool_stdout("abcd");
        assert_eq!(acc.tool_tokens, 1);
        // 1 char → ceil(1/4) = 1 token (rounded up; better to over-count
        // than under-count for honest accounting under Goodhart pressure).
        acc.record_tool_stdout("e");
        assert_eq!(acc.tool_tokens, 2);
        // 7 chars → ceil(7/4) = 2 tokens
        acc.record_tool_stdout("1234567");
        assert_eq!(acc.tool_tokens, 4);
    }
}
