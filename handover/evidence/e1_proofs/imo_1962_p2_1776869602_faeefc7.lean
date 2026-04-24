-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/imo_1962_p2.lean
-- theorem: imo_1962_p2
-- path_choice: alone (alone | tape+payload)
-- accepted_by_agent: oneshot
-- timestamp_unix: 1776869602
-- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file
--

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem imo_1962_p2
  (x : ℝ)
  (h₀ : 0 ≤ 3 - x)
  (h₁ : 0 ≤ x + 1)
  (h₂ : 1 / 2 < Real.sqrt (3 - x) - Real.sqrt (x + 1)) :
  -1 ≤ x ∧ x < 1 - Real.sqrt 31 / 8 := by

constructor
· linarith [h₁]
· have h3 : 0 ≤ Real.sqrt (3 - x) := Real.sqrt_nonneg _
  have h4 : 0 ≤ Real.sqrt (x + 1) := Real.sqrt_nonneg _
  have h5 : Real.sqrt (3 - x) - Real.sqrt (x + 1) > 0 := by linarith
  have h6 : Real.sqrt (3 - x) > Real.sqrt (x + 1) := by linarith
  have h7 : 3 - x > x + 1 := by
    refine (Real.strictMonoOn_sqrt (by positivity) (by positivity) h6).mp ?_
    exact sub_pos_of_lt h6
    exact by positivity
    exact by positivity
  have h8 : x < 1 := by linarith
  have h9 : Real.sqrt (3 - x) - Real.sqrt (x + 1) = 1 / 2 + ((Real.sqrt (3 - x) - Real.sqrt (x + 1)) - 1/2) := by ring
  have h10 : ((Real.sqrt (3 - x) - Real.sqrt (x + 1)) - 1/2) > 0 := by linarith
  have h11 : (Real.sqrt (3 - x) - Real.sqrt (x + 1)) ^ 2 > (1/2)^2 := by
    nlinarith? [h5, h10]
  have h12 : (Real.sqrt (3 - x) - Real.sqrt (x + 1)) ^ 2 = (3 - x) + (x + 1) - 2 * Real.sqrt ((3 - x) * (x + 1)) := by
    ring
    rw [Real.sqrt_mul (show 0 ≤ 3 - x from by linarith) _]
    ring
  rw [h12] at h11
  have h13 : 2 * Real.sqrt ((3 - x) * (x + 1)) < 4 - (1/2)^2 := by linarith
  have h14 : Real.sqrt ((3 - x) * (x + 1)) < (4 - (1/2)^2)/2 := by linarith
  have h15 : (4 - (1/2)^2)/2 = 31/16 := by ring
  rw [h15] at h14
  have h16 : 0 ≤ (3 - x) * (x + 1) := by
    have h16a : 0 ≤ 3 - x := by linarith
    have h16b : 0 ≤ x + 1 := h₁
    exact mul_nonneg h16a h16b
  have h17 : (3 - x) * (x + 1) < (31/16)^2 := by
    calc
      (3 - x) * (x + 1) = (Real.sqrt ((3 - x) * (x + 1))) ^ 2 := by
        rw [Real.pow_sqrt_eq_abs (3 - x) * (x + 1) ?_]
        exact abs_of_nonneg h16
      _ < (31/16)^2 := by nlinarith [Real.sqrt_nonneg _]
  have h18 : (3 - x) * (x + 1) = -x^2 + 2*x + 3 := by ring
  rw [h18] at h17
  have h19 : x^2 - 2*x + (31/16)^2 - 3 > 0 := by linarith
  have h20 : x^2 - 2*x + (31/16)^2 - 3 = (x - (1 - Real.sqrt 31 / 8)) * (x - (1 + Real.sqrt 31 / 8)) := by
    ring_nf
    ring
  rw [h20] at h19
  have h21 : 1 + Real.sqrt 31 / 8 > 1 := by
    have h21a : Real.sqrt 31 > 0 := Real.sqrt_pos.mpr (by norm_num)
    linarith
  have h22 : x < 1 + Real.sqrt 31 / 8 := by linarith [h8, h21]
  have h23 : x - (1 + Real.sqrt 31 / 8) < 0 := by linarith
  have h24 : (x - (1 - Real.sqrt 31 / 8)) * (x - (1 + Real.sqrt 31 / 8)) > 0 := h19
  have h25 : x - (1 - Real.sqrt 31 / 8) < 0 := by
    contrapose! h24
    nlinarith
  linarith