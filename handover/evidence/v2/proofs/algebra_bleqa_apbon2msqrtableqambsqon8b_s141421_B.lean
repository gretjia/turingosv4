-- Auto-extracted from E1v2_B_s141421_n8_20260424T102222.jsonl
-- problem: algebra_bleqa_apbon2msqrtableqambsqon8b
-- seed: 141421
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 17
-- gp_proof_file (original archive path): proofs/algebra_bleqa_apbon2msqrtableqambsqon8b_1777026318_2adc3343.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

rcases h₀ with ⟨ha, hb⟩
have h_nonneg : 0 ≤ b := by linarith
have h_ineq : (a + b)^2 - 4*a*b ≤ (a - b)^2 := by nlinarith
have h_sq_nonneg : 0 ≤ a - b := sub_nonneg.mpr h₁
have h_denom_pos : 0 < 8 * b := by
  nlinarith
calc
  (a + b) / 2 - Real.sqrt (a * b) = ((a + b) - 2 * Real.sqrt (a * b)) / 2 := by ring
  _ ≤ ((a + b) - 2 * (2 * a * b / (a + b))) / 2 := by
    -- using AM-GM inequality: sqrt(ab) ≥ 2ab/(a+b) for a,b>0
    have h_amgm : 2 * a * b / (a + b) ≤ Real.sqrt (a * b) := by
      nlinarith
    nlinarith
  _ = ((a + b)^2 - 4*a*b) / (2*(a+b)) := by ring
  _ = (a - b)^2 / (2*(a+b)) := by
    nlinarith
  _ ≤ (a - b)^2 / (8*b) := by
    -- denominator larger: 2(a+b) ≥ 8b  because a ≥ b > 0
    have h_denom : 8 * b ≤ 2*(a+b) := by nlinarith
    nlinarith
nlinarith