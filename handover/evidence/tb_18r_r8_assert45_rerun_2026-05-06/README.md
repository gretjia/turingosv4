# TB-18R G2 round-2 R8 — assert_45 partial-verdict-aware rerun

**Phase**: TB-18R G2 round-2 R8 evidence (Q13 VETO closure).
**Authority**: G2 round-1 verdict §3 Blocker 1 + §6 R8.
**Date**: 2026-05-06.
**Git HEAD at rerun**: `095a622` (TB-18R G2 round-2 R8 + R10 atom commit).
**Predecessor evidence**: `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/` + `handover/evidence/tb_18r_r7_m0_2026-05-06/`.

---

## §1 What changed

`assert_45` (`src/runtime/audit_assertions.rs:2580-2671`) was loosened from the strict `verified ↔ exit_code == 0` iff to a partial-verdict-aware invariant per `LeanResult` doc-comment:
1. `verified ⇒ exit_code == 0 ∧ error_class.is_none()` (clean omega path).
2. `!verified ∧ exit_code != 0 ⇒ error_class.is_some()` (real Lean failure must be classified).
3. `!verified ∧ exit_code == 0` admissible — partial-verdict (`error_class = None`) or sorry-block (`error_class = Some(SorryBlocked)`).

## §2 Per-run rerun results (`audit_tape` on chain unchanged)

| Run | Source evidence | Pre-R8 verdict | Pre-R8 id45 | **Post-R8 verdict** | **Post-R8 id45** |
|---|---|---|---|---|---|
| R6 P01 | tb_18r_r6_p23_p38_p49_2026-05-06/P01_mathd_algebra_107 | PROCEED | Pass | PROCEED | Pass |
| R6 P02 | tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124 | BLOCK | Fail | **PROCEED** | **Pass** |
| R6 P03 | tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime | BLOCK | Fail | **PROCEED** | **Pass** |
| R7 P01 | tb_18r_r7_m0_2026-05-06/P01_mathd_algebra_113 | PROCEED | Pass | PROCEED | Pass |
| R7 P02 | tb_18r_r7_m0_2026-05-06/P02_mathd_algebra_114 | BLOCK | Fail | **PROCEED** | **Pass** |
| R7 P03 | tb_18r_r7_m0_2026-05-06/P03_mathd_algebra_125 | PROCEED | Pass | PROCEED | Pass |
| R7 P04 | tb_18r_r7_m0_2026-05-06/P04_mathd_algebra_141 | PROCEED | Pass | PROCEED | Pass |
| R7 P05 | tb_18r_r7_m0_2026-05-06/P05_aime_1983_p2 | PROCEED | Pass | PROCEED | Pass |

**Result**: 8/8 PROCEED, 8/8 id45=Pass, 0 PROCEED→BLOCK regression. **Q13 VETO closed**.

## §3 No chain rewrite

The chains under `tb_18r_r6_*` and `tb_18r_r7_*` were NOT modified — only the audit_tape verdict.json + audit_tape.stderr were re-emitted under the post-R8 assertion library and persisted here in this separate evidence directory (per `feedback_no_retroactive_evidence_rewrite`).

## §4 Cross-References

- R8 source change: `095a622` commit (`src/runtime/audit_assertions.rs:2580-2671` + `src/runtime/attempt_telemetry.rs:402-411`).
- G2 round-1 verdict: `handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`.
- G2 round-2 ship report: `handover/audits/TB-18R_G2_ROUND_2_SHIP_REPORT_2026-05-06.md`.
