-- PCP Corpus 08: partial tactic accepted, final invalid
-- Mutation: first tactic produces step_partial_ok progress, but the final composite fails.
-- Expected (sequence):
--   - intermediate AttemptOutcome::PartialAccepted / LeanVerdictKind::PartialAccepted / CAS-only (no L4, no L4.E)
--   - final AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E

theorem pcp_corpus_08_partial_then_final_invalid (n : Nat) : (n + 0) + 0 = n + 1 := by
  rw [Nat.add_zero]   -- partial: this rewrite makes progress (step_partial_ok)
  rfl                 -- final: fails because n + 0 ≠ n + 1
