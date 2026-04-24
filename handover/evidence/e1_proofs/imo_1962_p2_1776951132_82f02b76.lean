-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/imo_1962_p2.lean
-- theorem: imo_1962_p2
-- path_choice: per_tactic (alone | tape+payload)
-- accepted_by_agent: Agent_3
-- timestamp_unix: 1776951132
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

rcases em (x < -1) with (hx | hx)
rcases em (x < -1) with (hx | hx)
rcases em (x < -1) with (hx | hx)
nlinarith
nlinarith
constructor
nlinarith
nlinarith
constructor
rcases em (x < -1) with (hx | hx)
refine ⟨by linarith, ?_⟩
· have hpos_sqrt3 : 0 ≤ Real.sqrt (3 - x) := Real.sqrt_nonneg _
  have hpos_sqrt1 : 0 ≤ Real.sqrt (x + 1) := Real.sqrt_nonneg _
  have h_nonneg_diff : Real.sqrt (3 - x) - Real.sqrt (x + 1) > 1/2 := h₂
  have h_pos_under : 0 ≤ Real.sqrt (3 - x) - Real.sqrt (x + 1) := by linarith
  have h_ineq_clean : 1 < 2*(Real.sqrt (3 - x) - Real.sqrt (x + 1)) := by linarith
  have : (Real.sqrt (3 - x) + Real.sqrt (x + 1)) > 0 := by
    nlinarith [hpos_sqrt3, hpos_sqrt1]
  have h_eq : (Real.sqrt (3 - x) - Real.sqrt (x + 1))*(Real.sqrt (3 - x) + Real.sqrt (x + 1)) = (3-x) - (x+1) := by ring
  nlinarith
refine ⟨by
  have : 0 ≤ x + 1 := h₁
  linarith, ?_⟩
· have h_sq : (Real.sqrt (3 - x) - Real.sqrt (x + 1)) > 1/2 := h₂
  have h_nonneg_sq3 : 0 ≤ Real.sqrt (3 - x) := Real.sqrt_nonneg _
  have h_nonneg_sq1 : 0 ≤ Real.sqrt (x + 1) := Real.sqrt_nonneg _
  have h_sum_pos : Real.sqrt (3 - x) + Real.sqrt (x + 1) > 0 := by
    nlinarith
  have h_diff_sq : (Real.sqrt (3 - x) - Real.sqrt (x + 1)) * (Real.sqrt (3 - x) + Real.sqrt (x + 1)) = (3 - x) - (x + 1) := by
    ring
    nlinarith
  have h_ineq : 1 < 2*(Real.sqrt (3 - x) - Real.sqrt (x + 1)) := by linarith
  have h_goal : (Real.sqrt (3 - x) + Real.sqrt (x + 1)) < (3 - x) - (x + 1) := by
    nlinarith
  have h_sq_sum : (Real.sqrt (3 - x) + Real.sqrt (x + 1))^2 = (3 - x) + (x + 1) + 2*Real.sqrt ((3 - x)*(x + 1)) := by
    ring
    nlinarith
  have h_rhs : (3 - x) - (x + 1) = 2 - 2*x := by ring
  have h_lhs_sq : (Real.sqrt (3 - x) + Real.sqrt (x + 1))^2 = 4 + 2*Real.sqrt ((3 - x)*(x + 1)) := by
    nlinarith
  have h_sq_ineq : 4 + 2*Real.sqrt ((3 - x)*(x + 1)) < (2 - 2*x)^2 := by
    nlinarith
  have h_sq_ineq' : 2*Real.sqrt ((3 - x)*(x + 1)) < (2 - 2*x)^2 - 4 := by
    nlinarith
  have h_nonneg_sqrt_prod : 0 ≤ Real.sqrt ((3 - x)*(x + 1)) := Real.sqrt_nonneg _
  have h_pos_rhs : 0 < (2 - 2*x)^2 - 4 := by
    nlinarith
  have h_sq_rhs : (2 - 2*x)^2 - 4 = 4*x*(x - 2) := by ring
  have h_x_lt_2 : x < 2 := by
    nlinarith
  have h_x_lt_1 : x < 1 := by
    nlinarith
  have h_ineq_final : x < 1 - Real.sqrt 31 / 8 := by
    nlinarith
  exact h_ineq_final