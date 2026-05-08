-- PCP Corpus Phase-2 / 08: partial tactic accepted but final invalid.
-- Mutation: ring_nf normalizes the LHS, then a deliberately-wrong tactic (rfl) fails.
-- Expected: AttemptOutcome::LeanFail (final) / LeanVerdictKind::Failed / RejectionClass::LeanFailed=6 / route → L4.E.
-- The first tactic (ring_nf) succeeds; the failure must occur on the SECOND tactic, exercising
-- the partial-accepted-but-final-invalid routing path explicitly.

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107_phase2_08_partial_then_final_invalid
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 5^2 := by
  ring_nf
  rfl
