-- Auto-extracted from E1v2_B_s31415_n8_20260424T112916.jsonl
-- problem: mathd_algebra_270
-- seed: 31415
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 15
-- gp_proof_file (original archive path): proofs/mathd_algebra_270_1777033391_90f5a064.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

calc
  f (f 1) = f (1 / (1 + 2)) := by
    rw [h₀ 1 (by norm_num : (1 : ℝ) ≠ -2)]
  _ = f (1/3) := by ring
  _ = 1 / ((1/3) + 2) := by
    rw [h₀ (1/3) (by
      intro h; have : (1/3 : ℝ) = -2 := h; linarith)]
  _ = 1 / (7/3) := by ring
  _ = 3/7 := by norm_num