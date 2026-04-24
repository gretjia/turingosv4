-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_332.lean
-- theorem: mathd_algebra_332
-- path_choice: per_tactic (alone | tape+payload)
-- accepted_by_agent: Agent_2
-- timestamp_unix: 1776994561
-- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file
--

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_332
  (x y : ℝ)
  (h₀ : (x + y) / 2 = 7)
  (h₁ : Real.sqrt (x * y) = Real.sqrt 19) :
  x^2 + y^2 = 158 := by

calc
  x^2 + y^2 = ((x + y)^2 - 2 * x * y) := by ring
  _ = ((x + y)^2 - 2 * (x * y)) := rfl
  _ = ((2*7)^2 - 2 * (Real.sqrt (x * y))^2) := by
    rw [h₀, h₁]
    ring
  _ = (14^2 - 2 * (Real.sqrt 19)^2) := rfl
  _ = (196 - 2 * 19) := by
    norm_num [Real.pow_sqrt_eq_abs, Real.sqrt_mul_self]
    -- Actually simpler: (Real.sqrt 19)^2 = 19
    norm_num
  _ = 196 - 38 := by ring
  _ = 158 := by norm_num