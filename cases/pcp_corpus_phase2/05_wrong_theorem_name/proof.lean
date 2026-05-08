-- PCP Corpus Phase-2 / 05: wrong theorem name.
-- Mutation: reference Mathlib.Tactic.Nlinarith2 (does not exist).
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E.

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107_phase2_05_wrong_theorem_name
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 5^2 := by
  exact Mathlib.Tactic.Nlinarith2.proof_that_does_not_exist h₀
