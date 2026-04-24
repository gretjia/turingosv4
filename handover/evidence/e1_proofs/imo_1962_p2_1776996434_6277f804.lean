-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)
-- problem_file: /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/imo_1962_p2.lean
-- theorem: imo_1962_p2
-- path_choice: per_tactic (alone | tape+payload)
-- accepted_by_agent: Agent_3
-- timestamp_unix: 1776996434
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
nlinarith
refine ⟨?_, ?_⟩; nlinarith