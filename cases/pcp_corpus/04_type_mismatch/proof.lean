-- PCP Corpus 04: type mismatch
-- Mutation: return a `String` where `Nat` is required.
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E

theorem pcp_corpus_04_type_mismatch (n : Nat) : n + 0 = n := by
  exact "not a Nat"
