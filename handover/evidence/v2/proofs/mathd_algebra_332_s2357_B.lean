-- Auto-extracted from E1v2_B_s2357_n8_20260424T151426.jsonl
-- problem: mathd_algebra_332
-- seed: 2357
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 5
-- gp_proof_file (original archive path): proofs/mathd_algebra_332_1777047305_11259f7.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

have hsum : x + y = 14 := by linarith
have hprod : x * y = 19 := by
  calc
    x * y = (Real.sqrt (x * y))^2 := by
      rw [Real.pow_sqrt_eq_abs (show 0 ≤ x * y from by
        have : 0 ≤ Real.sqrt (x * y) := Real.sqrt_nonneg _
        nlinarith [Real.sqrt (x * y) = Real.sqrt 19] )]
    _ = (Real.sqrt 19)^2 := by rw [h₁]
    _ = 19 := by norm_num
calc
  x^2 + y^2 = (x + y)^2 - 2*(x*y) := by ring
  _ = 14^2 - 2*19 := by rw [hsum, hprod]
  _ = 196 - 38 := by norm_num
  _ = 158 := by norm_num
calc
  x^2 + y^2 = (x + y)^2 - 2*(x*y) := by ring
  _ = 14^2 - 2*19 := by rw [hsum, hprod]
  _ = 196 - 38 := by norm_num
  _ = 158 := by norm_num