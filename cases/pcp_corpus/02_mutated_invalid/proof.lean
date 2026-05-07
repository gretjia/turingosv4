-- PCP Corpus 02: mutated invalid proof (general LeanFail)
-- Mutation: replace `rfl` with `Nat.succ_ne_zero` in a context where it does not apply.
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E

theorem pcp_corpus_02_mutated_invalid (n : Nat) : n + 0 = n := by
  exact Nat.succ_ne_zero n
