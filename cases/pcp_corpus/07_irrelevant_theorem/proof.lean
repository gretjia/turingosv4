-- PCP Corpus 07: irrelevant theorem
-- Mutation: invoke a real lemma whose conclusion does not match the goal.
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E

theorem pcp_corpus_07_irrelevant_theorem (n : Nat) : n + 0 = n := by
  exact Nat.zero_add n
