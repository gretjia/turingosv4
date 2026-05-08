-- PCP Corpus Phase-2 / 02: mutated invalid proof.
-- Mutation: nlinarith → linarith (insufficient power for nonlinear goal).
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E.

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107_phase2_02_mutated_invalid
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 5^2 := by
  linarith
