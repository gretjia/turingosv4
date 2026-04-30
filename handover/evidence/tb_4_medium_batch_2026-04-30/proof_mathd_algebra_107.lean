-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean
-- theorem: mathd_algebra_107
-- path_choice: per_tactic (alone | tape+payload)
-- accepted_by_agent: Agent_0
-- timestamp_unix: 1777551926
-- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file
--

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_107
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 5^2 := by

nlinarith