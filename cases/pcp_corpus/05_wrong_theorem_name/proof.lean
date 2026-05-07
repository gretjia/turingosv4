-- PCP Corpus 05: wrong theorem name (undefined identifier)
-- Mutation: invoke a non-existent lemma name.
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E

theorem pcp_corpus_05_wrong_theorem_name (n : Nat) : n + 0 = n := by
  exact Nat.add_zero_lemma_that_does_not_exist n
