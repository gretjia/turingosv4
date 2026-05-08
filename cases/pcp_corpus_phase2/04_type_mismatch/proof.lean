-- PCP Corpus Phase-2 / 04: type mismatch.
-- Mutation: conclusion (x + 4)^2 → ((x : ℕ) + 4)^2; forces ℝ ↦ ℕ coercion mismatch.
-- Expected: AttemptOutcome::LeanFail / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E.

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107_phase2_04_type_mismatch
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  ((x : ℕ) + 4)^2 + (y - 3)^2 = 5^2 := by
  nlinarith
