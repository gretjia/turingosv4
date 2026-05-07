-- PCP Corpus 09: parse-invalid output (malformed Lean source / no recognizable code block)
-- Expected: AttemptOutcome::ParseFail / LeanVerdictKind::Failed / RejectionClass::ParseFailed=7 / route → L4.E

theorem pcp_corpus_09_parse_invalid (n : Nat) : n + 0 = n := by
  this is not lean syntax @@@@
