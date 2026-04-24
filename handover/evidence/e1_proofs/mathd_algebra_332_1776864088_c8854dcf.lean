-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_332.lean
-- theorem: mathd_algebra_332
-- path_choice: alone (alone | tape+payload)
-- accepted_by_agent: oneshot
-- timestamp_unix: 1776864088
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

have h2 : x + y = 14 := by linarith
have pos : 0 < Real.sqrt 19 := Real.sqrt_pos.mpr (by norm_num)
have nonneg_xy : 0 ≤ x * y := by
  by_contra! H
  have h4 : Real.sqrt (x * y) = 0 := Real.sqrt_eq_zero_of_nonpos (by linarith)
  rw [h4] at h₁
  linarith
have h3 : x * y = 19 := by
  calc
    x * y = (Real.sqrt (x * y)) ^ 2 := by rw [Real.sq_sqrt nonneg_xy]
    _ = (Real.sqrt 19) ^ 2 := by rw [h₁]
    _ = 19 := by rw [Real.sq_sqrt (show 0 ≤ (19 : ℝ) from by norm_num)]

calc
  x^2 + y^2 = (x + y)^2 - 2 * (x * y) := by ring
  _ = 14^2 - 2 * 19 := by rw [h2, h3]
  _ = 158 := by norm_num