-- PCP Corpus Phase-2 / 09: parse-invalid output.
-- Mutation: unbalanced parentheses; Lean parser rejects pre-elaboration.
-- Expected: AttemptOutcome::ParseFail / LeanVerdictKind::ParseFailed / RejectionClass::ParseFailed=7 / route → L4.E.

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107_phase2_09_parse_invalid
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 5^2 := by
  nlinarith ((((
