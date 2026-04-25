-- Auto-extracted from E1v2_B_s31415_n8_20260424T112916.jsonl
-- problem: numbertheory_2pownm1prime_nprime
-- seed: 31415
-- condition: B (heterogeneous, n=8 swarm)
-- build_sha: 29ab43a
-- tx_count: 3
-- gp_proof_file (original archive path): proofs/numbertheory_2pownm1prime_nprime_1777033610_5f93fa48.lean
-- Verify: pipe through `lean --stdin` after prepending the
-- corresponding MiniF2F problem statement; or use
-- `python3 tools/audit_proof.py <this file>` if that tool is added in v2.2.

have hprime : n ≠ 1 := by
  intro hn
  have : 2^n - 1 = 1 := by
    calc
      2^n - 1 = 2^1 - 1 := by simpa [hn]
      _ = 1 := by norm_num
    
  have : ¬ Nat.Prime 1 := by
    norm_num [Nat.prime_def_lt']
  exact this (h₁.trans ?_)
  -- Actually h₁ says 2^n-1 is prime, but we derived it equals 1, contradiction
  -- Let's redo:
  have h2 : 2^n - 1 = 1 := by
    calc
      2^n - 1 = 2^1 - 1 := by simpa [hn]
      _ = 1 := by norm_num
  have : ¬ Nat.Prime (1 : ℕ) := by norm_num
  rw [h2] at h₁
  exact this h₁