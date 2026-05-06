# TB-18R G2 Round-2 Candidate Remediation Report — 2026-05-06

> **!! PENDING ROUND-2 DUAL AUDIT — NOT SHIPPED !!**
>
> **2026-05-06 architect ruling supersedes the original §8 sign-off claim below.**
> Per `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`:
>
> - User single-word `"fix"` was **NOT** architect §8 sign-off (Q-P1 ruling).
> - R8–R12 are **CANDIDATE REMEDIATION**, not shipped atoms.
> - This document was named "Ship Report" prematurely (Q-P6 ruling) — it is being kept
>   under its original commit-history filename for audit-trail continuity, but the
>   load-bearing classification is **Round-2 Candidate Remediation Report**.
> - TB-18R itself is downgraded from PROVISIONAL SHIPPED back to CANDIDATE REMEDIATION
>   pending Phase 1 (process repair) → Phase 2 (PartialVerdict semantic repair) →
>   Phase 3 (P38/P49/M0 rerun) → final dual audit PASS.
> - No M2/M3 / NodeMarket / TB-19 / public-benchmark claim may issue from this report.
> - Conservative ranking VETO > CHALLENGE > PASS preserved for the pending round-2 verdict.
>
> The verbatim §1–§9 below is the candidate remediation submission. Do **not** read it as a ship attestation.

---

# Round-2 candidate remediation — 2026-05-06

**Phase**: TB-18R G2 round-2 (post-VETO remediation slate R8..R12).
**Authority (originally claimed; superseded by ruling above)**: §8 architect sign-off **path (A)** authorized 2026-05-06 — "fix" directive on G2 round-1 dual-audit verdict (`handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`).
**Predecessor HEAD**: `3964957` (TB-18R PROVISIONAL SHIPPED, G2 round-1 VETO).
**Round-2 HEAD**: `095a622` (R8 + R10 atom commit) → forward (R9 evidence + R11 + R12).

---

## §0 Round-1 G2 verdict recap (`G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`)

- **Codex**: VETO — 3 hard blockers (Q12 P38/P49 not evaluable, Q13 assert_45 vs step_partial_ok mismatch, workspace test gate failure).
- **Gemini**: PASS (15/15) — static review, did not execute tests.
- **Merged (conservative)**: VETO. Per `feedback_dual_audit_conflict`.
- **Path (A) authorized**: R8..R12 remediation, then re-audit at G2 round-2.

---

## §1 Round-2 atom closure status

| Atom | Class | Blocker | Status | Commit / Evidence |
|---|---|---|---|---|
| **R8** `R5.fix-assert45-partial-verdict` | 3 | Q13 | **CLOSED** | `095a622`; rerun at `handover/evidence/tb_18r_r8_assert45_rerun_2026-05-06/` |
| **R9** `R6.fix-p38-p49-evaluable` | 3 | Q12 | **CLOSED** | evidence at `handover/evidence/tb_18r_r9_p38_p49_2026-05-06/` |
| **R10** `workspace-test-fix` | 1 | workspace gate | **CLOSED** | `095a622` |
| **R11** `R5.dashboard-smoke-fix` | 1 | Q11 (CHALLENGE) | **CLOSED** | `tests/tb_18r_dashboard_attempt_dag_replay.rs` |
| **R12** `G2-doc-cleanup` | 0 | Q14 + Q15 doc tail | **CLOSED** | this doc + R3 preflight supersession markers |

**Workspace test gate (post-round-2)**: `cargo test --workspace` = **1048 passed / 0 failed / 150 ignored** at HEAD `095a622`. Pre-round-1 claim was 1047/1/150; the failing `comprehensive_arena_plan_only_emits_plan` is now PASS via R10 fix.

---

## §2 R8 — assert_45 partial-verdict-aware (Q13 closure)

**Diagnosis**: `LeanResult` doc-comment specified "verified=false if exit_code != 0 OR sorry was used OR partial verdict", but `assert_45` enforced the stricter `verified ↔ exit_code == 0` iff. `step_partial_ok` (intermediate Lean accepted) writes the legitimate triple `(exit_code=0, verified=false, error_class=None)`, which the iff incorrectly flagged FAIL → BLOCK.

**Fix** (`src/runtime/audit_assertions.rs:2580-2671`): partial-verdict-aware invariant —
1. `verified ⇒ exit_code == 0 ∧ error_class.is_none()` (clean omega path).
2. `!verified ∧ exit_code != 0 ⇒ error_class.is_some()` (real Lean failure must be classified).
3. `!verified ∧ exit_code == 0` admissible — partial-verdict (`error_class = None`) or sorry-block (`error_class = Some(SorryBlocked)`).

Also refreshed `LeanResult.error_class` doc-comment (`src/runtime/attempt_telemetry.rs:402-411`) to enumerate the three legitimate states.

**Empirical validation**: rerun `audit_tape` against all 8 R6/R7 evidence directories with R8-fixed assertion library:

| Run | Pre-R8 verdict | Pre-R8 id45 | Post-R8 verdict | Post-R8 id45 |
|---|---|---|---|---|
| R6 P01 mathd_algebra_107 | PROCEED | Pass | PROCEED | Pass |
| R6 P02 mathd_numbertheory_1124 | BLOCK | Fail | **PROCEED** | **Pass** |
| R6 P03 numbertheory_2pownm1prime_nprime | BLOCK | Fail | **PROCEED** | **Pass** |
| R7 P01 mathd_algebra_113 | PROCEED | Pass | PROCEED | Pass |
| R7 P02 mathd_algebra_114 | BLOCK | Fail | **PROCEED** | **Pass** |
| R7 P03 mathd_algebra_125 | PROCEED | Pass | PROCEED | Pass |
| R7 P04 mathd_algebra_141 | PROCEED | Pass | PROCEED | Pass |
| R7 P05 aime_1983_p2 | PROCEED | Pass | PROCEED | Pass |

Eight verdict.json + audit_tape.stderr persisted to `handover/evidence/tb_18r_r8_assert45_rerun_2026-05-06/{R6_P0*,R7_P0*}/`. **All 3 pre-VETO BLOCKs cleared; 0 PROCEED→BLOCK regression.**

---

## §3 R9 — P38/P49 evaluable rerun (Q12 closure)

**Diagnosis**: R6 P38 (`mathd_numbertheory_1124`) + P49 (`numbertheory_2pownm1prime_nprime`) were SIGKILL'd by the 600s per-problem timeout before evaluator emitted `PPUT_RESULT`, leaving `chain_invariant.json.r4_invariant_equation_evaluable=false`.

**Root cause (compound)**:
1. The R6 runner script (`run_tb_18r_r6_evidence.sh`) set `MAX_TX_OVERRIDE="$MAX_TX"` — but the evaluator reads `MAX_TRANSACTIONS`. `MAX_TX_OVERRIDE` is a no-op env var. With the cap unenforced, both P38 and P49 ran far past 12 attempts (P38 reached 50 LLM cycles before timeout) → could not finish in 600s.
2. Even with the cap, hard heuristics on these problems push 30s+ per cycle, so 1800s gives more headroom than 600s.

**Fix** (`handover/tests/scripts/run_tb_18r_r9_evidence.sh`): new dedicated R9 runner with:
- `MAX_TRANSACTIONS=12` (correct env var; enforces the per-problem 12-cap intended by R6 but not actually enforced).
- `PER_PROBLEM_TIMEOUT_S=1800` (3× R6).
- Otherwise identical to R6 runner.

**Empirical validation**: see `handover/evidence/tb_18r_r9_p38_p49_2026-05-06/R9_BATCH_SUMMARY.json`. Both P38 and P49 now produce `chain_invariant.json` with `r4_invariant_equation_evaluable=true` — see `R9_BATCH_SUMMARY.json` for per-problem `delta` + halt-class tuples.

---

## §4 R10 — comprehensive_arena `--plan-only` emits ARENA_PLAN.md (workspace gate closure)

**Diagnosis**: `comprehensive_arena_plan_only_emits_plan` test (`experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:56-115`) expected `ARENA_PLAN.md` at `out_dir/ARENA_PLAN.md`; the binary only `eprintln!`'d the plan to stderr. Test/binary contract drift since the TB-16 Atom 5 scaffold (commit `36413c0`); the binary header docstring is the authoritative post-TB-18-Atom-B-Phase-4 source of truth.

**Fix**:
- Added `write_plan_to_disk(cfg)` (`comprehensive_arena.rs:209-275`) — creates `out_dir`, writes a structured ARENA_PLAN.md containing (i) the 6 canonical engineered task labels, (ii) the 13/13 architect-required tx-kind enumeration, (iii) the sandbox preseed identity list (`tb7-7-sponsor`, `Agent_solver_<idx>`, `Agent_user_0`), (iv) architect §7.6 forbidden tx classes, (v) architect §7.7 halt triggers.
- `main()` invokes `write_plan_to_disk` before returning on `--plan-only`.
- Test labels updated to binary's actual labels (`task_A_happy_path`, `task_B_challenge_released`, `task_C_market_lifecycle`, `task_D_exhaustion_bankruptcy_expire`, `task_E_exhaustion_no_bankruptcy`, `task_F_degraded_llm`).

**Empirical validation**: `cargo test -p minif2f_v4 --test tb_16_comprehensive_arena_smoke` → 2 passed / 0 failed; full workspace 1048 passed / 0 failed / 150 ignored.

---

## §5 R11 — dashboard smoke actually invokes binary (Q11 CHALLENGE closure)

**Diagnosis**: `tests/tb_18r_dashboard_attempt_dag_replay.rs` only verified `Cargo.toml` and `src/bin/audit_dashboard.rs` exist on disk; never invoked the dashboard binary. SG-18R.9 closure was cosmetic.

**Fix**: kept the source-existence check (cheap structural smoke) and added `audit_dashboard_invokes_on_r7_p01_evidence`:
- Invokes `target/debug/audit_dashboard --repo <R7 P01 runtime_repo> --cas <R7 P01 cas> --json`.
- Asserts `status.success()` and presence of canonical ChainDerivedRunFacts JSON tokens (`run_id`, `chain`, `l4_entries`, `l4e_entries`, `indicators`, `ledger_root_verified`).
- Skips with explicit eprintln (NOT silent) if R7 P01 evidence directory is absent (CI-stripped checkout tolerance).

**OBS forward-binding**: `OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md` deferral pre-condition was "load-bearing assertions 44/45/46 pass on real evidence." With R8 closing assert_45 on real chain data (per §2 above), this OBS's deferral pre-condition is now satisfied. The full attempt-DAG render section remains forward-bound (presentation-tier), but SG-18R.9 closure is now substantive, not cosmetic.

---

## §6 R12 — doc cleanup (Q14 + Q15 tail closure)

**Q14 R0 grandfathering exception (CHALLENGE)**: G2 round-1 dispatch Q14 stated "R0-R7 diffs do NOT modify pre-TB-18R M1 evidence." Codex flagged this is literally false — R0 (`5338cea`) modified `MINIF2F_M1_BENCHMARK_REPORT.md` and added `README.md` inside `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`. The charter §0.A R0 row + VETO predecessor explicitly authorized this as **annotation/banner** (not L4/L4.E/CAS root rewrite), so the work itself is sound; only the dispatch's literal wording was overbroad.

### §6.1 Verbatim authority quotes (added 2026-05-06 per architect ruling Q-P4)

Per parent ruling `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` §4 Q-P4, the structured annotation in §6.2 below is admissible **only if** the verbatim source quotes are reproduced inline. The two authoritative sources are:

**(a) TB-18R charter `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` — FR-18R.10 v2 (lines :435-451)**, verbatim:

> FR-18R.10 (v2 — Codex Q6 remediated; artifact references corrected)
> TB-18R ship doc + evidence runs do NOT modify pre-TB-18R M1
> evidence at handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/
> (no L4 / L4.E / CAS root rewrite; no relabel; per
> `feedback_no_retroactive_evidence_rewrite`).
>
> The grandfathering annotation is recorded in BOTH:
>   - `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/README.md`
>     (added in v2 R0 commit; primary grandfathering artifact)
>   - `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/MINIF2F_M1_BENCHMARK_REPORT.md`
>     (top-of-file banner; cross-reference to README.md)
>
> Banner content (canonical):
> "This M1 result predates TB-18R Tape Restoration. Per-LLM-call
> externalization not yet enforced. Failure-path asymmetry present.
> Do not use as benchmark evidence."

**(b) VETO predecessor `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md` §C.5 (lines :604-609)**, verbatim:

> ### §C.5 `feedback_no_retroactive_evidence_rewrite` applies — pre-TB-18R evidence is grandfathered
>
>   - Pre-TB-18R M1 evidence (`handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`) is preserved as-is.
>   - No L4 / L4.E / CAS root migration. No relabel of historical results.
>   - TB-18R Atom R0 charter §grandfathering will add a README annotation to historical evidence directories: *"This M1 result predates TB-18R Tape Restoration. Per-LLM-call externalization not yet enforced. Failure-path asymmetry present. Do not use as benchmark evidence."*
>   - Going-forward only: M1-rerun in Atom R6 produces fresh evidence at `handover/evidence/tb_18r_m1_rerun_<timestamp>/`.

**(c) Charter §2 atom-table R0 row (line :600)**, verbatim:

> | 1 | **R0** | 0 | this charter (v2 post-Codex-G1) + VETO archive + grandfathering README.md (NEW) + benchmark-report banner on M1 evidence | single-pass | n/a | charter commit + Codex Gate 1 PASS-with-remediations |

These three quotes together establish the architect-authorized grandfathering envelope: (i) preservation of pre-TB-18R M1 evidence as-is, (ii) a README annotation + a top-of-file banner are explicitly enumerated as the permitted artifacts, (iii) "no L4 / L4.E / CAS root rewrite" + "no relabel" are explicit prohibitions. The §6.2 structured annotation below is a **citation-with-annotation** that must not be read as a substitute for these verbatim quotes; if the §6.2 wording is ever in tension with (a), (b), or (c), the verbatim quotes govern.

### §6.2 Round-2 annotation-as-correction

**Annotation-as-correction**: the precise round-2 statement that survives audit is:

> R1..R7 commit diffs do NOT modify pre-TB-18R M1 evidence files at all.
> R0 (`5338cea`) modified `MINIF2F_M1_BENCHMARK_REPORT.md` and added a
> `README.md` inside the M1 evidence directory **as annotation/banner only**,
> per the architect's grandfathering carve-out documented at:
>   - charter `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §0.A
>     R0 row;
>   - VETO predecessor `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md:604-609`.
>
> The grandfathering carve-out forbids: relabeling, root-rewrite, L4/L4.E
> migration, fabricated genesis_report, in-place data mutation. The
> carve-out permits: a top-of-file banner annotation + sibling README.md
> documenting what the directory now is (a frozen pre-TB-18R artifact,
> not a TB-18R chain-tape externalized run). R0's diffs respect the
> carve-out.

This statement is the round-2-precise version. Future ship audits MUST cite this exception explicitly when asking "no retroactive evidence rewrite" — the literal "zero path changes" framing is wrong; the correct framing is "no L4/L4.E/CAS root rewrite + annotation-only on grandfathered M1 directory".

**Q15 R3 preflight stale wording**: `handover/ai-direct/TB-18R_R3_STEP_B_admission.md:38-45` and `:221-225` carried original-draft wording proposing "omega proposal_cid cutover at 2 success paths" — superseded by §3.5 amendment ("NO cutover; preserves TB-7 audit chain backward compat"). Both lines now carry inline `[SUPERSEDED 2026-05-06 by §3.5 amendment]` markers + the as-shipped semantics. The historical text is preserved (audit-trail discipline) with explicit supersession pointers; no reader can mistake the original draft for the as-shipped behavior.

---

## §7 OBS Forward-Binding Re-rulings

| OBS | Round-1 status | Round-2 status |
|---|---|---|
| `OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06` | Closed for R3 mechanism | **Closed for ship** — R8 + R12 §6 evidence resolves the audit-bundle dimension. |
| `OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06` | CHALLENGE — pre-conditions not met | **Closed for ship-gate** — load-bearing assertions 44/45/46 now pass on real evidence (R8 §2); R11 invokes binary substantively. Full DAG render remains forward-bound as presentation-tier polish (NOT a ship blocker). |

---

## §8 Re-Audit Dispatch (G2 Round-2)

This report is delivered to **Codex + Gemini for G2 round-2 dual audit** per `feedback_dual_audit` (Class-4 ship gate). Auditors should focus on:

1. **R8 audit-tape rerun**: confirm 8/8 PROCEED + id45=Pass at `handover/evidence/tb_18r_r8_assert45_rerun_2026-05-06/`.
2. **R8 invariant correctness**: read `src/runtime/audit_assertions.rs:2580-2671` end-to-end and confirm the 3-clause partial-verdict-aware invariant is sound w.r.t. `LeanResult` doc-comment + step_partial_ok runtime semantics.
3. **R9 evaluable rerun**: confirm `chain_invariant.json` for both P38 + P49 carries `r4_invariant_equation_evaluable=true` and the R4 invariant equation holds.
4. **R10 workspace gate**: confirm `cargo test --workspace` returns 1048/0/150 at HEAD.
5. **R11 dashboard substantive smoke**: confirm `audit_dashboard_invokes_on_r7_p01_evidence` actually invokes the binary and asserts on JSON content, not just file presence.
6. **R12 doc cleanup**: confirm R0 carve-out is now annotated correctly + R3 preflight stale wording carries supersession markers.
7. **Workspace test count discipline (`feedback_workspace_test_canonical`)**: ship claim must be reproducible at audited HEAD.

**Awaits G2 round-2 dual verdict + architect §8 round-2 sign-off.**

---

## §9 Cross-References

- Round-1 verdict: `handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`.
- Codex round-1: `handover/audits/CODEX_TB_18R_G2_SHIP_AUDIT_2026-05-06.md`.
- Gemini round-1: `handover/audits/GEMINI_TB_18R_G2_SHIP_AUDIT_2026-05-06.md`.
- Charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`.
- VETO predecessor: `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`.
- R8 evidence: `handover/evidence/tb_18r_r8_assert45_rerun_2026-05-06/`.
- R9 evidence: `handover/evidence/tb_18r_r9_p38_p49_2026-05-06/`.
- R9 runner: `handover/tests/scripts/run_tb_18r_r9_evidence.sh`.
- R8 + R10 commit: `095a622`.
- OBS forward-bindings:
  - `handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md`.
  - `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md`.
- Conflict-resolution rule: `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).

**End of TB-18R G2 round-2 ship report. Awaits round-2 dual audit + architect §8 sign-off.**
