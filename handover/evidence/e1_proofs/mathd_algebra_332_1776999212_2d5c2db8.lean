-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_332.lean
-- theorem: mathd_algebra_332
-- path_choice: per_tactic (alone | tape+payload)
-- accepted_by_agent: Agent_7
-- timestamp_unix: 1776999212
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

have h₂ : x*y = 19 := by
  calc
    x*y = (Real.sqrt (x*y))^2 := by
      rw [Real.pow_sqrt_eq_abs (show x*y ≥ 0 from ?_)]
    _ = (Real.sqrt 19)^2 := by rw [h₁]
    _ = 19 := by norm_num
  -- Need positivity condition from h₁ context? We can get it from h₁:
  have h_nonneg : x*y ≥ 0 := Real.sqrt_nonneg _
  calc
    x*y = (Real.sqrt (x*y))^2 := by
      rw [Real.sq_sqrt h_nonneg]
    _ = (Real.sqrt 19)^2 := by rw [h₁]
    _ = 19 := by norm_num