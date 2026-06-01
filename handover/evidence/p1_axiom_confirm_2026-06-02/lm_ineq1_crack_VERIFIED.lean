import Mathlib

theorem ineq_amgm_concrete (a b : ℝ) (ha : 0 ≤ a) (hb : 0 ≤ b) :
    3 * a^2 + 5 * b^2 ≥ 2 * Real.sqrt 15 * (a * b) := by
have ha' : a^2 ≥ 0 := pow_two_nonneg a
have hb' : b^2 ≥ 0 := pow_two_nonneg b
have h_nonneg_sqrt : 0 ≤ Real.sqrt 15 := Real.sqrt_nonneg _
-- use AM-GM inequality: (3a^2 + 5b^2)/2 ≥ sqrt(3a^2 * 5b^2) = sqrt(15)*|a*b|
-- Since a,b ≥ 0, |a*b| = a*b.
-- We can apply the inequality (x^2 + y^2)/2 ≥ x*y for x = sqrt(3)*a, y = sqrt(5)*b
set x := Real.sqrt 3 * a with hx_def
set y := Real.sqrt 5 * b with hy_def
have hx_sq : x^2 = 3*a^2 := by
  dsimp [x]
  calc
    (Real.sqrt 3 * a)^2 = (Real.sqrt 3)^2 * a^2 := by ring
    _ = (3 : ℝ) * a^2 := by
      rw [Real.sq_sqrt (show 0 ≤ (3 : ℝ) from by norm_num)]
    _ = 3*a^2 := by ring
have hy_sq : y^2 = 5*b^2 := by
  dsimp [y]
  calc
    (Real.sqrt 5 * b)^2 = (Real.sqrt 5)^2 * b^2 := by ring
    _ = (5 : ℝ) * b^2 := by
      rw [Real.sq_sqrt (show 0 ≤ (5 : ℝ) from by norm_num)]
    _ = 5*b^2 := by ring
have h_nonneg_x : 0 ≤ x := mul_nonneg (Real.sqrt_nonneg _) ha
have h_nonneg_y : 0 ≤ y := mul_nonneg (Real.sqrt_nonneg _) hb
have h_ineq : x^2 + y^2 ≥ 2*x*y := by
  nlinarith [sq_nonneg (x - y)]
calc
  3*a^2 + 5*b^2 = x^2 + y^2 := by rw [hx_sq, hy_sq]
  _ ≥ 2*x*y := h_ineq
  _ = 2*(Real.sqrt 3 * a)*(Real.sqrt 5 * b) := rfl
  _ = 2*(Real.sqrt 3 * Real.sqrt 5)*(a*b) := by ring
  _ = 2*Real.sqrt (3*5)*(a*b) := by rw [Real.sqrt_mul (show 0 ≤ (3:ℝ) from by norm_num) 5]
  _ = 2*Real.sqrt 15*(a*b) := by norm_num