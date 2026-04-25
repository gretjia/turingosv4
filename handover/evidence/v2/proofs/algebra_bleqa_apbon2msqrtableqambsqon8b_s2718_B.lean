-- Auto-extracted from E1v2_B_s2718_n8_20260424T140411.jsonl
-- problem: algebra_bleqa_apbon2msqrtableqambsqon8b
-- seed: 2718
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 10
-- gp_proof_file (original archive path): proofs/algebra_bleqa_apbon2msqrtableqambsqon8b_1777039575_2608e2a2.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

rcases h₀ with ⟨ha, hb⟩
have h_nonneg : 0 ≤ (a - b)^2 := by nlinarith
have h_sq : (a + b) / 2 - Real.sqrt (a * b) ≤ (a - b)^2 / (8 * b) := by
  have h_am_gm : Real.sqrt (a * b) ≤ (a + b) / 2 := by
    apply Real.sqrt_mul_self_le_add_div_two a b
    exact ha
    exact hb
  have h_diff : (a + b) / 2 - Real.sqrt (a * b) ≤ (a + b) / 2 - (a + b) / 2 + (a - b)^2 / (8 * b) := by
    nlinarith
  nlinarith