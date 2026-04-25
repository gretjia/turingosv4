-- Auto-extracted from E1v2_B_s2357_n8_20260424T151426.jsonl
-- problem: algebra_bleqa_apbon2msqrtableqambsqon8b
-- seed: 2357
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 15
-- gp_proof_file (original archive path): proofs/algebra_bleqa_apbon2msqrtableqambsqon8b_1777043873_93ac52a0.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

rcases h₀ with ⟨ha, hb⟩
calc
    (a + b) / 2 - Real.sqrt (a * b)
        = ((a + b) / 2 - Real.sqrt (a * b)) * 1 := by ring
    _ ≤ ((a + b) / 2 - Real.sqrt (a * b)) * ((a + b) / 2 + Real.sqrt (a * b)) / ((a + b) / 2 + Real.sqrt (a * b)) := by
      nlinarith [ha, hb, h₁]
    _ = ((a + b)^2 / 4 - a * b) / ((a + b) / 2 + Real.sqrt (a * b)) := by ring
    _ = ((a - b)^2 / 4) / ((a + b) / 2 + Real.sqrt (a * b)) := by ring
    _ ≤ ((a - b)^2 / 4) / (2 * Real.sqrt (a * b)) := by
      refine (div_le_div_right ?_).mpr ?_
      · positivity
      · nlinarith
    _ = (a - b)^2 / (8 * Real.sqrt (a * b)) := by ring
    _ ≤ (a - b)^2 / (8 * b) := by
      refine (div_le_div_right ?_).mpr ?_
      · nlinarith
      · have hsqrt : Real.sqrt (a * b) ≥ b := by
          nlinarith [Real.sqrt_le_sqrt (by nlinarith : b * b ≤ a * b), hb]
        nlinarith