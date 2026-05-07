-- PCP Corpus 06: off-by-one arithmetic
-- Mutation: prove `n + 1 = n` instead of `n + 0 = n`.
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E

theorem pcp_corpus_06_off_by_one_arith (n : Nat) : n + 1 = n := by
  rfl
