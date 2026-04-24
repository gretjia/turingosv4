-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/algebra_apbon2pownleqapownpbpowon2.lean
-- theorem: algebra_apbon2pownleqapownpbpowon2
-- path_choice: per_tactic (alone | tape+payload)
-- accepted_by_agent: Agent_7
-- timestamp_unix: 1777010656
-- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file
--

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem algebra_apbon2pownleqapownpbpowon2
  (a b : ℝ)
  (n : ℕ)
  (h₀ : 0 < a ∧ 0 < b)
  (h₁ : 0 < n) :
  ((a + b) / 2)^n ≤ (a^n + b^n) / 2 := by

rcases h₀ with ⟨ha, hb⟩
have hpos : 0 < (a + b) / 2 := by nlinarith
rcases h₁ with hn
by_contra hneg; push_neg at hneg; have h : (a^n + b^n) / 2 < ((a + b) / 2)^n := by nlinarith; have hpos : 0 < a^n + b^n := by positivity; nlinarith [pow_mul_add_le_add_pow (a := a) (b := b) (n := n) ha hb hn]