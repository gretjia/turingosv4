-- PCP Corpus Phase-2 / 06: off-by-one arithmetic.
-- Mutation: conclusion 5^2 → 6^2 (true statement broken; LHS still equals 25, NOT 36).
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E.

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107_phase2_06_off_by_one_arith
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 6^2 := by
  nlinarith
