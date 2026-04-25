-- Auto-extracted from E1v2_B_s141421_n8_20260424T102222.jsonl
-- problem: mathd_algebra_332
-- seed: 141421
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 12
-- gp_proof_file (original archive path): proofs/mathd_algebra_332_1777029737_e1b45ccd.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

field_simp at h₀
ring_nf at h₀
have hx_plus_y : x + y = 14 := by linarith; have hxy : x * y = 19 := by nlinarith; calc x^2 + y^2 = (x+y)^2 - 2*(x*y) := by ring _ = 14^2 - 2*19 := by rw [hx_plus_y, hxy] _ = 196 - 38 := by norm_num _ = 158 := by norm_num