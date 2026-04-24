-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_44.lean
-- theorem: mathd_algebra_44
-- path_choice: alone (alone | tape+payload)
-- accepted_by_agent: oneshot
-- timestamp_unix: 1776873440
-- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file
--

import Mathlib

set_option maxHeartbeats 0

open BigOperators Real Nat Topology Rat

theorem mathd_algebra_44
  (s t : ℝ)
  (h₀ : s = 9 - 2 * t)
  (h₁ : t = 3 * s + 1) :
  s = 1 ∧ t = 4 := by

constructor
· rw [h₀] at h₁
  linarith
· rw [h₀] at h₁
  linarith