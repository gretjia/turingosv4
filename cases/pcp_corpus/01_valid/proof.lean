-- PCP Corpus 01: valid proof (canonical positive control)
-- Expected: AttemptOutcome::LeanPass / LeanVerdictKind::Verified / route → L4 accepted

theorem pcp_corpus_01_valid (n : Nat) : n + 0 = n := by
  rfl
