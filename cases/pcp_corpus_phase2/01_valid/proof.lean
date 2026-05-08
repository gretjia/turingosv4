-- PCP Corpus Phase-2 / 01: valid proof (canonical positive control).
-- Source: MiniF2F Test set / mathd_algebra_107.
-- Expected: AttemptOutcome::LeanPass / LeanVerdictKind::Verified / route → L4 accepted.

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107_phase2_01_valid
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 5^2 := by
  nlinarith
