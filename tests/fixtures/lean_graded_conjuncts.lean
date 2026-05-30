import Mathlib

-- grad_nt: number theory (6 conjuncts)
theorem c_nt_1 : Nat.gcd 48 36 = 12 := by decide
theorem c_nt_2 : Nat.Coprime 25 14 := by decide
theorem c_nt_3 : Nat.Prime 101 := by norm_num
theorem c_nt_4 : (15 : ℕ) ∣ 225 := by norm_num
theorem c_nt_5 : Nat.gcd 0 7 = 7 := by simp
theorem c_nt_6 : ∀ n : ℕ, n ∣ n ^ 2 := fun n => ⟨n, by ring⟩

-- grad_alg: algebra (6 conjuncts)
theorem c_alg_1 : ∀ a b : ℤ, (a + b) ^ 2 = a ^ 2 + 2 * a * b + b ^ 2 := by intro a b; ring
theorem c_alg_2 : ∀ a b : ℤ, a - b = -(b - a) := by intro a b; ring
theorem c_alg_3 : (2 : ℤ) ^ 10 = 1024 := by norm_num
theorem c_alg_4 : ∀ n : ℕ, Even (n + n) := fun n => ⟨n, by ring⟩
theorem c_alg_5 : ∀ a b c : ℤ, a * (b + c) = a * b + a * c := by intro a b c; ring
theorem c_alg_6 : ∀ a b : ℤ, (a - b) * (a + b) = a ^ 2 - b ^ 2 := by intro a b; ring

-- grad_ord: order/misc (6 conjuncts)
theorem c_ord_1 : ∀ a b : ℕ, a ≤ max a b := fun a b => le_max_left a b
theorem c_ord_2 : ∀ a b : ℕ, min a b ≤ a := fun a b => min_le_left a b
theorem c_ord_3 : ∀ a : ℕ, a ≤ a + 1 := fun a => Nat.le_succ a
theorem c_ord_4 : ∀ a b : ℕ, a ≤ b → a ≤ b + 1 := fun a b h => Nat.le_succ_of_le h
theorem c_ord_5 : (List.length [1, 2, 3, 4] = 4) := by decide
theorem c_ord_6 : ∀ a b : ℝ, a ≤ b ∨ b ≤ a := fun a b => le_total a b
