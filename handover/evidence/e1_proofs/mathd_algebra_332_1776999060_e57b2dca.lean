-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_332.lean
-- theorem: mathd_algebra_332
-- path_choice: per_tactic (alone | tape+payload)
-- accepted_by_agent: Agent_7
-- timestamp_unix: 1776999060
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
    x^2 + y^2 = (x + y)^2 - 2 * x * y := by ring
    _ = ((2*7)^2) - 2 * (Real.sqrt 19)^2 := by
      have hsum : x + y = 14 := by linarith
      have hprod : x * y = 19 := by
        have hsqrt : Real.sqrt (x * y) = Real.sqrt 19 := h₁
        apply (Real.sqrt_inj.mp ?_)
        · exact hsqrt
        · nlinarith [Real.sqrt_nonneg (x*y), Real.sqrt_nonneg 19]
        · calc
            Real.sqrt (x * y) = Real.sqrt 19 := h₁
            _ ≥ 0 := Real.sqrt_nonneg _
        · calc
            Real.sqrt 19 = Real.sqrt (x * y) := h₁.symm
            _ ≥ 0 := Real.sqrt_nonneg _
      nlinarith
    _ = 196 - 2 * 19 := by ring
    _ = 158 := by ring