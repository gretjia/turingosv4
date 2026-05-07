-- PCP Corpus 03: sorry insertion (forbidden incomplete-proof token)
-- Expected: AttemptOutcome::SorryBlock / LeanVerdictKind::SorryBlocked / RejectionClass::SorryBlocked=8 / route → L4.E

theorem pcp_corpus_03_sorry_insertion (n : Nat) : n + 0 = n := by
  sorry
