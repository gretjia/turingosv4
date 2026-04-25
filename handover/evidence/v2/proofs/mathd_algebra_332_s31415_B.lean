-- Auto-extracted from E1v2_B_s31415_n8_20260424T112916.jsonl
-- problem: mathd_algebra_332
-- seed: 31415
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 20
-- gp_proof_file (original archive path): proofs/mathd_algebra_332_1777033583_5698ef8c.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

have h2 : (x + y)^2 = 196 := by
  nlinarith
have h3 : x * y = 19 := by
  calc
    x * y = (Real.sqrt (x * y))^2 := by
      rw [Real.pow_sqrt_eq_abs (show 0 ≤ x * y from ?_), abs_of_nonneg (show 0 ≤ x * y from ?_)]
    _ = (Real.sqrt 19)^2 := by rw [h₁]
    _ = 19 := by norm_num
  -- This path is messy; let's use nlinarith instead
  nlinarith
nlinarith