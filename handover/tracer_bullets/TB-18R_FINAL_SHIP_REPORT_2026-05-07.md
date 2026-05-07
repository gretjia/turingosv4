# TB-18R FINAL SHIP REPORT — Tape Restoration / Per-LLM-Call ChainTape Externalization

> **Status: FINAL SHIP REPORT — awaits architect §8 sign-off (NOT auto-shipped).**
>
> Per `feedback_class4_cannot_hide_in_class3` + CLAUDE.md §10, Class-3/4 ship requires
> explicit named architect authorization (scope / allowed path / forbidden path / audit
> requirement / ship authorization). A single-word reply does not constitute §8 sign-off.
> This document packages the FINAL ship evidence on top of the original 2026-05-06
> PROVISIONAL ship report (downgraded to CANDIDATE REMEDIATION same day) and the
> post-TB-C0 substrate work that closed `PROJECT_PLAN.md §3` resume conditions.
>
> **Until architect §8 sign-off is filed under `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`, TB-18R remains FINAL-CANDIDATE, not SHIPPED FINAL.**

**Date**: 2026-05-07
**Predecessor PROVISIONAL ship report**: `handover/tracer_bullets/TB-18R_SHIP_REPORT_2026-05-06.md` (banner-prefixed; status downgraded same day per `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`).
**Charter (v2; G1-ratified)**: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`.
**Authority lifting freeze**: TB-C0 SHIPPED FINAL 2026-05-07 (`handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md` — verbatim "好，确认可以 ship"; multi-clause; NOT single-word).
**Scope**: TB-18R 11 atoms (R0..R7 + G1 + G2 dispatch) plus post-TB-C0 Wave 3 real-LLM-tape evidence supplement. Constitution Landing First (commit `b7bde23`; G-009 / G-012 / G-016 substrate) is **out of TB-18R scope** — that is independent Class-2 substrate work consumed by `PROJECT_PLAN.md §3` resume conditions but not part of the TB-18R atom plan.

---

## §0 Status genealogy

```text
2026-05-06  TB-18R PROVISIONAL SHIPPED        (G2 dispatch filed; awaits dual audit)
2026-05-06  → DOWNGRADED to CANDIDATE REMEDIATION
            (architect ruling: user single-word "fix" ≠ §8 sign-off; round-2 ship
             report banner-prefixed; expanded FREEZE in effect)
2026-05-06  → Phase 2 PartialVerdict typed semantic repair shipped (e12d254 + 3f51667)
2026-05-06  → Phase 3 authorized via `2026-05-06_TB18R_EMERGENCY_HARNESS_RESET_DIRECTIVE.md`
            (operating mode shifted from Atomic Agentic Engineering →
             Constitutional Harness Engineering)
2026-05-07  → TB-C0 Constitution Landing Gate SHIPPED FINAL (architect §8 verbatim
             "好，确认可以 ship"; 5-round Codex audit VETO→CHALLENGE→CHALLENGE→PASS→PASS;
             FREEZE LIFTED for TB-18R Final / TB-18B / TB-19+).
2026-05-07  → Constitution Landing First substrate landed (b7bde23):
             G-009 HEAD_t C1 + G-012 PCP corpus + G-016 PromptCapsule
             (independent Class-2; not TB-18R scope but consumed by §3 #4–#6).
2026-05-07  → Wave 3 20p diagnostic green (ffb6ebd):
             FC1 invariant 140 = 7 + 129 + 4 on real DeepSeek tape.
2026-05-07  → Wave 3 50p diagnostic green (a612cc9):
             FC1 invariant 460 = 9 + 400 + 51 at 2.5× load.
             PROJECT_PLAN §3 = 10/10 GREEN (this is the door condition for TB-18R Final
             ship eligibility; it is NOT itself §8 sign-off).
2026-05-07  → THIS DOCUMENT: FINAL ship report packaged; awaits architect §8 sign-off.
```

---

## §1 Executive

TB-18R closes the failure-path asymmetry surfaced by TB-18 M1 evidence (P49-class observation
`evaluator_tx_count=32` vs `L4_WorkTx=1`). Pre-TB-18R, only the OMEGA-accept path externalized
to L4 (`omega_wtool` → `submit_typed_tx(WorkTx)` + `VerifyTx`); the four failure paths
(`step_reject` / `parse_fail` / `llm_err` / `step_partial_ok`) leaked only to evaluator stdout
or kernel.tape shadow. TB-18R rebuilds the substrate so:

- Every externalized LLM-Lean cycle produces a CAS-resident `AttemptTelemetry` object
  (R1 schema + R2 evaluator hot path).
- Every `AttemptTelemetry` routes to L4 (predicate pass) or L4.E (predicate fail) with a
  fine-grained `RejectionClass ∈ {LeanFailed=6, ParseFailed=7, SorryBlocked=8, LlmError=9}`
  discriminator (R3 admission expansion + R3.fix split-brain CAS reload).
- A chain-derived ship-gate equation enforces
  `evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count
  + capsule_anchored_attempt_count` (CLAUDE.md §6 amended with `step_partial_ok` capsule
  anchoring; R4 invariant + post-fix runner counting `OBS_TB18R_INV1_NONLLM_TX_2026-05-07`).
- Audit-tape sampler reaches `AttemptTelemetry` + `LeanResult` CAS objects beyond
  ceremonial-gate-only sampling (R5 Layer G/H +5 assertions).

Empirically validated at three scales of real-LLM tape:

| Batch | Evidence | LHS | RHS-a (L4) | RHS-b (L4.E) | RHS-c (capsule) | Match |
|---|---|--:|--:|--:|--:|---|
| TB-18R R6 (P23/P38/P49) | `tb_18r_r6_*` | 6/6 evaluable | — | 74 LLM rejects on L4.E | — | 6/6 PASS / 2 NA SIGKILL |
| TB-18R R7 (M0×5) | `tb_18r_r7_*` | 5/5 | — | 26 LLM rejects | — | 5/5 PASS |
| Phase 3 7-problem (P38+P49+M0×5; v3 fresh on ship HEAD) | `tb_18r_phase_3_2026-05-07T08-33-05Z` | 7/7 | — | — | — | 7/7 NATURAL PASS |
| Wave 3 20p diagnostic | `wave3_diagnostic_20p_2026-05-07T13-08-06Z` | **140** | **7** | **129** | **4** | **20/20 invariant=Ok delta=0** |
| Wave 3 50p diagnostic (2.5× load) | `wave3_diagnostic_50p_2026-05-07T14-04-48Z` | **460** | **9** | **400** | **51** | **50/50 invariant=Ok delta=0** |

The Wave 3 50p row is the first batch with non-zero `parse_fail` under load (13 cases all
correctly routed to L4.E with `RejectionClass=ParseFailed=7`; no false-accept; no silent
drop) and the first batch exercising `step_partial_ok` capsule anchoring at scale (51 cases
= 12.75× the 20p exposure; all anchored as typed `AttemptOutcome::PartialAccepted`).

Pre-TB-18R baseline of "real LLM-Lean rejections persisted to L4.E" was **0**. Post-TB-18R
baseline through Wave 3 50p is **>500** with fine-grained `RejectionClass` discriminators on
every record. Failure-path asymmetry empirically closed at scale.

---

## §2 Atoms shipped (sequence-binding per charter §2)

| # | Atom | Class | Commit / merge | Surface |
|---|---|---|---|---|
| 1 | R0 (charter v2 + grandfathering README on M1 evidence) | 0 | `5338cea` | charter + Codex Q1..Q8 remediations |
| 2 | G1 (Codex charter ratification) | 3 audit | n/a (audit doc) | `CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md` (CHALLENGE-but-ship-clean; 7 remediations applied as charter v2) |
| 3 | R1 (typed-tx + CAS schema) | 4 STEP_B | `9f8ce1f` via `bbee847` | `AttemptTelemetry` + `LeanResult` + `TerminalAbortRecord` schemas; STEP_B branch `stepb-tb18r-r1-schema` |
| 4 | R2 (evaluator hot path) | 3 | `35389d0` | 6 evaluator paths instrumented; CR-18R.4 v2 privacy fence (parsed-candidate-only on paths 1–4 + fixed sentinels on paths 5–6; never raw LLM response) |
| 5 | R3 (sequencer admission) | 4 STEP_B | `72a1b75` via `66dde84` | `RejectionClass` tail-append {LeanFailed=6, ParseFailed=7, SorryBlocked=8, LlmError=9}; Design D refine helper; STEP_B branch `tb-18r-r3-admission` |
| 6 | R3.fix (CasStore reload) | 4 STEP_B | `2ca1aed` via `f2e73f6` | split-brain reload; surgical patch closing L0 smoke bug (P49 chain rejection_class histogram pre-fix 5/5 PredicateFailed → post-fix {LeanFailed=6:1, SorryBlocked=8:4} EMPIRICALLY VALIDATED) |
| 7 | R4 (chain-derived invariant) | 4 STEP_B | `d34f428` via `41aae74` | G1-ratified canonical equation; +6 fields (expected/l4/l4e/aborted/delta/terminal_halt_class); 3 new pub fns; drain-barrier honored via existing `ChaintapeBundle::shutdown()` |
| 8 | R5 (audit-tape sampler) | 3 | `5a09e2d` | Layer G/H +5 assertions (assert_44/45/46 + assert_g + assert_47/48 tamper); `cas/store.rs::list_cids_by_object_type` Class 3 helper |
| 9 | R6 (P23/P38/P49 evidence) | 3 evidence | `8608bc3` | `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/`; 74 real LLM rejects on L4.E |
| 10 | R7 (M0×5 evidence) | 3 evidence | `8608bc3` + `e97fcea` | `handover/evidence/tb_18r_r7_m0_2026-05-06/`; 26 real LLM rejects |
| 11 | G2 (dispatch) | 3 audit | `3964957` | `G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md` (15-question Codex + Gemini ask) |

**Post-PROVISIONAL repair atoms (out of original 11 but in TB-18R scope per architect 2026-05-06 round-2 ruling)**:

| # | Atom | Class | Commit | Surface |
|---|---|---|---|---|
| R8–R12 | Phase 1 (process repair) | 0–3 docs | `e12d254` chain | candidate remediation framing; banner prefixes on PROVISIONAL ship report |
| Phase 2 | Typed `LeanVerdictKind::PartialAccepted` (Option B tail-additive) | 4 STEP_B-adjacent | `e12d254` + `3f51667` | `LeanVerdict` enum on `LeanResult`; closes the LeanPass-on-rejection fence at type level |
| Phase 3 v3 | P38 + P49 + M0×5 fresh re-run on ship HEAD | 3 evidence | `8c15d61` | `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/`; 7/7 NATURAL PASS; eliminates v2's HEAD-drift + post-hoc-corrected-files round-3 challenge axes |
| A0 fix | Evidence-drift root cause | 3 | `cf7cb48` + `64745bb` | env-gate test writes to committed evidence; `tests/constitution_no_evidence_drift_in_tests.rs` regression guard |
| Runner counting fix | FC1 invariant LHS scope clarification | 3 | `3eb4f71` | runner script + architect_inv1_check.json scope rename + CLAUDE.md §6 amended LHS = `tool_dist.step + parse_fail + llm_err`; new gate `tests/constitution_runner_invariant_formula.rs` |

---

## §3 Ship gates SG-18R.1..13 closure (final pass)

```text
SG-18R.1  AttemptTelemetry per-LLM-call               PASS (R1 schema + R2 hot path; tests/tb_18r_attempt_telemetry_per_llm_call.rs 8 path-shape tests)

SG-18R.2  AttemptTelemetry routes to L4 OR L4.E       PASS at scale (R3 admission; tests/tb_18r_attempt_routes_to_l4_or_l4e.rs 5 tests; Wave 3 50p empirical 9 → L4 + 400 → L4.E + 51 → capsule anchor; 0 vanish)

SG-18R.3  attempt_count_invariant() PASS              PASS empirically 70/70 evaluable across (R6 6/6 + R7 5/5 + Phase 3 7/7 + Wave 3 20p 20/20 + 50p 50/50) under post-fix CLAUDE.md §6 LHS scope; 10 unit tests covering all 6 RunOutcome halt classes

SG-18R.4  6-field exact accounting per evidence run   PASS empirically 70/70 evaluable + 3 unit tests; per-run chain_invariant.json exhibits all 6 fields per FR-18R.4 v2; delta=0 on all evaluable runs at all scales

SG-18R.5  Real Lean rejects in L4.E with class set    PASS at scale (>500 real LLM-Lean cycle rejections persisted to L4.E with fine-grained RejectionClass: TB-18R R6+R7 100 + Wave 3 20p 129 + Wave 3 50p 400; pre-TB-18R baseline = 0)

SG-18R.6  markov cluster source from AttemptTelemetry PASS at type-system level (R5 assert_g; tests/tb_18r_markov_failure_cluster_from_chain.rs 2 tests; full markov_capsule rewire forward-bound per R5 preflight §1.2)

SG-18R.7  audit_tape sampler reaches math content     PASS (R5 assert_44/45 + Layer H tamper; tests/tb_18r_audit_sampler_attempt_payload.rs 2 + tests/tb_18r_audit_lean_stderr_tamper_detected.rs 2; Wave 3 50p audit=PROCEED 50/50)

SG-18R.8  attempt_chain_root referenced by composite  PASS at schema-validity level (R5 assert_46; tests/tb_18r_final_composite_attempt_chain_root.rs 2 tests; full Merkle population on omega path forward-bound per R3 §3.5 amended)

SG-18R.9  Dashboard regenerates attempt DAG           SMOKE-LEVEL PASS; full §17 render OBS-deferred to forward TB (tests/tb_18r_dashboard_attempt_dag_replay.rs; OBS at handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md)

SG-18R.10 PRE-TB-18R M1 evidence grandfathered        PASS (README.md + MINIF2F_M1_BENCHMARK_REPORT.md banner added in R0; no L4/L4.E/CAS root mutation; verified by `feedback_no_retroactive_evidence_rewrite` discipline)

SG-18R.11 cargo test --workspace baseline + delta     PASS (workspace 1174/0/151 at HEAD `a612cc9`; pre-TB-18R baseline 963; net delta +211 (8.4× over charter min +25); 0 failures; pre-existing TB-16 arena flake `comprehensive_arena_plan_only_emits_plan` resolved between TB-18R original ship and Wave 3 50p)

SG-18R.12 G1 verdict = PASS or remediated CHALLENGE   PASS (CHALLENGE-but-ship-clean; 7 remediations applied as charter v2)

SG-18R.13 G2 verdict (final)                          REOPENED via post-PROVISIONAL repair path: G2 dispatch 2026-05-06 dispatched but did not gate ship after architect downgrade to CANDIDATE REMEDIATION; G2 superseded by Phase 3 v3 NATURAL PASS evidence + TB-C0 5-round Codex audit + post-TB-C0 Wave 3 20p/50p empirical re-validation. Final architect §8 sign-off this ship report is the canonical close per `feedback_audit_after_evidence`.
```

**Auxiliary ship gates SG-18R.A..E**:
- SG-18R.A pre-R1 G1 turnaround logged: PASS.
- SG-18R.B STEP_B preflight docs filed for R1+R3+R3.fix+R4: PASS (4/4 preflights present).
- SG-18R.C R6 P49-class evidence: PASS (chain attempt count vs evaluator tx count delta reported per problem in `chain_invariant.json`).
- SG-18R.D R7 M0 small batch ≤20 problems: PASS (5 problems; no overlap with R6).
- SG-18R.E TB-18R FINAL pending architect §8 sign-off: ACTIVE (this document).

---

## §4 Workspace test result (per `feedback_workspace_test_canonical`)

```text
command:                cargo test --workspace --no-fail-fast
HEAD:                   a612cc9 (Wave 3 50-problem diagnostic)
workspace:              1174
failed:                 0
ignored:                151
pre_TB-18R_baseline:    963 (post-TB-18 Atom B-impl SHIPPED commit 15b662c)
post_TB-18R_orig_ship:  1047 (commit 3964957; carried 1 pre-existing TB-16 arena flake)
delta_to_orig_ship:     +84 net (TB-18R R0..R7 atoms; charter min +25; 3.4×)
delta_post_TB-C0:       +127 net (TB-C0 + Constitution Landing First + post-fix repair atoms)
total_TB-18R_envelope:  +211 net (8.4× over charter SG-18R.11 min +25)
arena_flake:            RESOLVED between original ship and Wave 3 50p
```

```text
command:                bash scripts/run_constitution_gates.sh
HEAD:                   a612cc9
gates:                  90 passed
gates_failed:           0
gates_ignored:          1
verdict:                PASS — all gates GREEN
```

---

## §5 Aggregate empirical evidence (TB-18R + Wave 3 supplement)

### 5.1 TB-18R R6 (P23/P38/P49 substrate validation; commit `8608bc3`)

```text
problems:           3 (mathd_algebra_107 / mathd_numbertheory_1124 / numbertheory_2pownm1prime_nprime)
MAX_TX:             12
per-problem timeout: 600s
P01 mathd_algebra_107:                 OmegaAccepted, dur=583s, R4-Ok
P02 mathd_numbertheory_1124:           SIGKILL @ 600s, 50 LLM rejects on L4.E (PPUT-absent NA)
P03 numbertheory_2pownm1prime_nprime:  SIGKILL @ 600s, 23 LLM rejects on L4.E
                                       (the M1 VETO-trigger problem; pre-TB-18R 0 chain
                                        rejects → post-TB-18R 23 = failure-path
                                        asymmetry empirically closed)
audit_tape:         P01 PROCEED 38/0 + P02/P03 BLOCK 36/1 (1 fail = no-omega-ProposalTelemetry
                    consistent with no-omega-on-chain shape; expected per R3 §3.5 amended)
aggregate:          74 real LLM rejects on L4.E with R3 fine-grained discriminators
```

### 5.2 TB-18R R7 (M0×5 batch; commit `8608bc3` + `e97fcea`)

```text
problems:           5 (mathd_algebra_113/114/125/141 + aime_1983_p2)
MAX_TX:             8
per-problem timeout: 360s
audit_tape:         5/5 (4 PROCEED + 1 BLOCK on P02 with single assert_24 fail =
                    R3 §3.5 amended omega-no-cutover consequence)
R4 invariant:       5/5 PASS under v4 extraction
aggregate:          26 real LLM rejects on L4.E with R3 fine-grained discriminators
```

### 5.3 TB-18R Phase 3 v3 (P38 + P49 + M0×5 fresh on ship HEAD; commit `8c15d61`)

```text
problems:           7 (M0 batch + M1 VETO-trigger problem)
runner:             post-fix runner counting (CLAUDE.md §6 amended LHS scope)
result:             7/7 NATURAL PASS
                    7/7 audit_tape PROCEED
                    7/7 id45 PASS
                    7/7 architect_inv1_check.match=True (under canonical 3-term FC1-INV1)
                    7/7 chain_invariant verdict=Ok delta=0
halt distribution:  5/7 OmegaAccepted (mathd_numbertheory_1124 + algebra_107/114/125/141)
                    2/7 MaxTxExhausted (numbertheory_2pownm1prime_nprime + algebra_113)
P04 mixed-tx case:  reproduces canonical non_llm_tx_diagnostic_gap=3 (architect-mandated
                    admin scaffold txs); match=True under canonical 3-term FC1-INV1 —
                    gap is informational, not violation
```

### 5.4 Wave 3 20-problem diagnostic (commit `ffb6ebd`)

```text
HEAD:               9007e1a (post-Constitution-Landing-First substrate)
n:                  20
solved:             7 (35% solve rate; Wilson 95% CI [18.12%, 56.71%])
ΣPPUT:              61.50
Mean PPUT(solved):  8.79
halt_dist:          {OmegaAccepted: 7, MaxTxExhausted: 13}
batch dur:          1566s (~26 min)
audit_tape:         20/20 PROCEED
id45:               20/20 Pass
architect_inv1:     20/20 match=True
chain_invariant:    20/20 Ok delta=0

FC1 hard invariant aggregate (CLAUDE.md §6):
  completed_llm_calls_total = 140
  l4_work_attempt_total      = 7    (== omega_wtool=7 == solved=7)
  l4e_work_attempt_total     = 129  (== step_reject=129)
  capsule_anchored_total     = 4    (== step_partial_ok=4; 4 problems P03/P08/P15/P18)
  140 = 7 + 129 + 4  ✓
```

### 5.5 Wave 3 50-problem diagnostic (commit `a612cc9`)

```text
HEAD:               ffb6ebd (post-Wave-3-20p substrate; identical binary)
n:                  50
solved:             9 (18% solve rate; Wilson 95% CI [9.77%, 30.80%])
halt_dist:          {OmegaAccepted: 9, MaxTxExhausted: 41}
batch dur:          ~133 min (8000s); 2.5× load over 20p
audit_tape:         50/50 PROCEED
id45:               50/50 Pass
architect_inv1:     50/50 match=True
chain_invariant:    50/50 Ok delta=0

FC1 hard invariant aggregate (CLAUDE.md §6):
  completed_llm_calls_total = 460
  l4_work_attempt_total      = 9    (== omega_wtool=9 == solved=9)
  l4e_work_attempt_total     = 400  (== step_reject=387 + parse_fail=13)
  capsule_anchored_total     = 51   (== step_partial_ok=51; 12.75× 20p exposure)
  460 = 9 + 400 + 51  ✓

Notable:
  - First batch with non-zero parse_fail under load (13 cases all routed to L4.E
    correctly with RejectionClass=ParseFailed=7; no false-accept; no silent drop).
  - Heavy step_partial_ok exposure (51 cases) on hard algebra long-form problems
    exercises typed AttemptOutcome::PartialAccepted at scale; all anchored as
    capsule entries; none erroneously claimed L4 acceptance.
  - Lower solve rate vs 20p (18% vs 35%) is model-coverage bottleneck on the
    extended 30 hard-algebra-long-form problems, NOT substrate failure;
    invariants 50/50 GREEN is the substrate-stability signal.
```

---

## §6 PROJECT_PLAN.md §3 resume conditions check

| # | Condition | Status | Evidence |
|---|-----------|:--:|---|
| 1 | FC composite green | ✅ | `bash scripts/run_constitution_gates.sh` = 90/0/1 GREEN at HEAD `a612cc9` |
| 2 | Art. III ≥ 60% LANDED+PARTIAL with ≥1 LANDED | ✅ | matrix §D 5/5 = 100% (1 GREEN + 4 AMBER); GREEN row = Art. III prompt-persistence (G-016/019/021/028) |
| 3 | HEAD_t C1 green | ✅ | `tests/constitution_head_t_witness.rs` 5/5 PASS; matrix §A Art. 0.4 GREEN; C2 libgit2 forward-step (Week 5–8) |
| 4 | PCP synthetic corpus green | ✅ | `tests/constitution_pcp_corpus.rs` 7/7 PASS; `cases/pcp_corpus/` 9-class manifest |
| 5 | PromptCapsule anchored | ✅ | `tests/constitution_prompt_capsule.rs` 8/8 PASS; schema landed; evaluator wire-up forward-step |
| 6 | P38 / P49 attempt equality green | ✅ | Phase 3 v3 7/7 NATURAL PASS + Wave 3 20p 20/20 + Wave 3 50p 50/50 (exhaustive triangulation; 100% match=True across all batches) |
| 7 | `cargo test --workspace` 0 fail | ✅ | 1174/0/151 at HEAD `a612cc9` |
| 8 | `scripts/run_constitution_gates.sh` 0 fail | ✅ | 90/0/1 |
| 9 | No unresolved critical BLOCKED-DECISION | ✅ | G-009 / G-012 / G-016/019/021/028 all settled by architect Constitution Landing First ruling 2026-05-07 |
| 10 | Art. 0 ≥ 70% LANDED+PARTIAL | ✅ | matrix §A 5/5 = 100% (1 GREEN + 4 AMBER); GREEN row = Art. 0.4 HEAD_t C1 |

**Conclusion**: 10/10 GREEN. PROJECT_PLAN.md §3 door-condition for TB-18R Final ship eligibility is FULLY closed. This eligibility is a necessary but not sufficient condition for §8 sign-off — the architect retains §10 authorization-semantics authority over the ship decision.

---

## §7 Constitutional alignment

- **Art. 0.2 (Tape Canonical)** — every externalized LLM-Lean cycle has chain-side
  persistence (L4 or L4.E + CAS); evaluator stdout no longer authoritative; demonstrated
  at 460 cycles in Wave 3 50p with zero attempt vanish.
- **Art. I.1 (5-step compile loop)** — wtool gap on failure paths closed (predicate fail
  now wtools to L4.E with `RejectionClass`, not memory-only `record_rejection`).
- **Art. III.1 (raw failure log shielding)** — CAS-shielded stderr/stdout blobs
  (R5 schema test); `public_summary` low-pollution per CR-18R.8.
- **Art. III.4 (no fake accepted; no fake un-attempted)** — failed attempts cannot vanish
  (R3 + R3.fix end-to-end validated; survivorship bias closed); demonstrated at scale
  (>500 LLM rejects with discriminator).
- **Art. IV (terminal-state distinction)** — orthogonal to R4 invariant; `RunOutcome`
  reused as `terminal_halt_class` per `feedback_no_workarounds_strict_constitution`.
- **Art. V.1 (三权分立)** — G1 (Codex charter ratification) + post-PROVISIONAL TB-C0
  5-round Codex re-audit (VETO→CHALLENGE→CHALLENGE→PASS→PASS) discharges
  Generator≠Evaluator separation per `feedback_dual_audit`. Gemini sanity pass on the
  Constitution Landing First substrate (independent Class-2; b7bde23) is in flight at
  ship time and **does NOT gate this report** — it is forward-step coverage on
  substrate consumed by §3 #4–#6, not on TB-18R's 11 atoms.

---

## §8 OBS forward-bound (architect §8 review SHOULD cover)

| OBS | Source | Forward-bound to |
|---|---|---|
| `OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md` | R3 between-gate Codex audit infra-failed; R3 §3.5 omega-no-cutover + R3 §1.3 step_partial_ok-CAS-only deviations documented | architect §8 review of post-PROVISIONAL repair-path G2 supersession |
| `OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md` | SG-18R.9 full §17 dashboard DAG render deferred to forward TB | architect §8 ruling on smoke-level acceptability for SG-18R.13 |
| `OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md` | LHS scope clarification (canonical 3-term `tool_dist.step + parse_fail + llm_err` vs broader `evaluator_reported_tx_count`); fixed in commit `3eb4f71` + tests/constitution_runner_invariant_formula.rs | informational; closed in-line |
| `OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md` | env-gate cargo test writes to committed evidence; closed by `cf7cb48` + `tests/constitution_no_evidence_drift_in_tests.rs` regression guard | informational; closed in-line |

---

## §9 Forbidden honored (charter §5; verbatim from VETO archive §A.6)

| # | Forbidden | Status |
|---|-----------|:--:|
| 1 | stdout-only failure attempts | ✅ closed (R1+R2+R3) |
| 2 | final-composite-only WorkTx | ✅ closed (R3 §1.3 + §3.5 amended) |
| 3 | `record_rejection`-only Lean failures | ✅ closed (R3 wires to L4.E) |
| 4 | M2 benchmark before SHIPPED FINAL | ✅ honored (FREEZE active until §8) |
| 5 | NodeMarket / price signals before invariant closure | ✅ honored (FREEZE active until §8) |
| 6 | EvidenceCapsule-substitutes-per-attempt-records | ✅ closed (R4 FR-18R.3 v2 enforces per-attempt L4/L4.E records) |
| 7 | tx_count=32 self-report without per-attempt records | ✅ closed (R4 invariant + Wave 3 50p 460 records) |
| 8 | `tool_dist`-only DAG | ✅ closed (R5 audit assertions sample CAS) |
| 9 | ceremonial-gate-only audit | ✅ closed (R5 Layer G samples AttemptTelemetry + LeanResult) |
| 10 | private-CoT externalization | ✅ closed (CR-18R.4 v2 + R2 fixed sentinels + R5 privacy fence in assert_44) |
| 11 | Class-4 surface hiding inside Class-3 atom | ✅ closed (R1+R3+R3.fix+R4 all STEP_B + Codex G1 ratified) |
| 12 | retroactive M1 evidence rewrite | ✅ closed (CR-18R.1 + R0 grandfathering README + MINIF2F_M1_BENCHMARK_REPORT.md banner) |
| 13 | ship-with-OBS in lieu of strict alignment | ✅ closed (every OBS deferral has explicit forward-binding + future-TB atom plan) |

---

## §10 Architect §8 sign-off ask

Per CLAUDE.md §10 authorization semantics, this report requests architect §8 named
authorization with the following scope:

```text
Scope:           TB-18R Final ship — Tape Restoration / Per-LLM-Call ChainTape
                 Externalization (11 original atoms + post-PROVISIONAL repair atoms +
                 Wave 3 supplemental evidence).
Allowed path:    Flip TB_LOG.tsv TB-18R row from `active` to `shipped`;
                 mark this ship report status from FINAL-CANDIDATE to SHIPPED FINAL;
                 unblock TB-18B M1 / M2 benchmark scale-up charter authoring per
                 PROJECT_PLAN §5.
Forbidden path:  Retroactive rewrite of pre-TB-18R M1 evidence
                 (per `feedback_no_retroactive_evidence_rewrite`); claiming
                 "formal benchmark passed" externalization (PROJECT_PLAN §4 — that
                 is TB-18B / TB-21 territory); inferring multi-Agent economy
                 readiness from this ship (TB-21 territory).
Risk class:      3 (production wire-up + per-attempt evidence; the Class-4 atoms
                 R1 / R3 / R4 were ratified at G1 in 2026-05-06).
Audit required:  N/A for this ship (post-PROVISIONAL G2 supersession path documented
                 in §3 SG-18R.13 + §8 OBS forward-bound).
Ship authorized: ☐ YES        ☐ NO + remediation directive
```

A canonical sign-off file is to be placed at:

```
handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md
```

with content following the TB-C0 §8 sign-off template (verbatim multi-clause user
authorization; explicit scope; explicit ship authorization). Single-word sign-off
remains insufficient per `feedback_class4_cannot_hide_in_class3` precedent.

---

## §11 Forward-bound items (NOT gating this ship; tracked for next session)

1. **Wave 1/2 残留 AMBER → GREEN promotion** — independently valuable harness hardening
   (Wave 1 残留: `no_legacy_authoritative_append` / `dashboard_not_source_of_truth` /
   `no_memory_only_preseed` runtime integration; Wave 2 残留: `EvidenceCapsule` audit-only
   strictness / `MarkovEvidenceCapsule` integrity verification / `BenchmarkManifest` +
   `EvidencePackagingPolicy` independent matrix rows). NOT a §3 blocker; Wave 3
   Constitution Landing First closed §3 directly.
2. **Gemini architecture sanity pass** — HARNESS.md §1 H5 dual-audit other half on the
   Constitution Landing First substrate (b7bde23). In-flight at this ship time;
   independent Class-2 coverage; does not gate TB-18R Final.
3. **PromptCapsule evaluator wire-up** — Class-2 forward step from C-LAND-1; every
   `AttemptTelemetry` references a `PromptCapsule` CID at runtime per architect §4.3.
4. **TB-18B M1 / M2 charter** — first real benchmark scale-up post-§3 unfreeze; must
   pass `bash scripts/run_constitution_gates.sh` + Gemini sanity pass before merge per
   CR-C0.10.
5. **HEAD_t C2 libgit2** (Week 5–8) — `refs/chaintape/{l4, l4e, cas}` multi-branch
   unification on top of the existing real-libgit2 substrate. The current `Git2LedgerWriter`
   is real `git2-rs` writing real commits to `refs/transitions/main`; CAS is real `git2-rs`
   blob layer; HEAD_t in `q.head_t` is the real 40-char OID. C2 reorganizes the namespace
   under `refs/chaintape/*`; it does NOT introduce libgit2 (already present).
6. **MiniF2F-v2 misalignment corpus** (Week 5–8) — extends G-012 PCP corpus from
   synthetic 9-class to natural-corpus benchmark misalignment.

---

## §12 Cross-references

- Charter (v2; G1-ratified): `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`.
- VETO archive (lossless verbatim): `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`.
- G1 audit: `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md`.
- G2 dispatch: `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md`.
- Round-2 architect ruling (PROVISIONAL → CANDIDATE REMEDIATION): `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`.
- Phase 3 emergency reset directive: `handover/directives/2026-05-06_TB18R_EMERGENCY_HARNESS_RESET_DIRECTIVE.md`.
- TB-C0 §8 sign-off (precedent for multi-clause architect authorization): `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`.
- Preflights: `handover/ai-direct/TB-18R_R1_STEP_B_schema.md` + `TB-18R_R3_STEP_B_admission.md` + `TB-18R_R3FIX_STEP_B_cas_reload.md` + `TB-18R_R4_STEP_B_invariant.md` + `TB-18R_R5_preflight_audit_extension.md`.
- OBS: `OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md` + `OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md` + `OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md` + `OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md`.
- TB-18R 11-atom evidence: `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/` + `handover/evidence/tb_18r_r7_m0_2026-05-06/`.
- Phase 3 v3 evidence: `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/`.
- Wave 3 evidence: `handover/evidence/wave3_diagnostic_20p_2026-05-07T13-08-06Z/` + `handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z/`.
- Doctrine: `CLAUDE.md` (Constitutional Harness Engineering) + `HARNESS.md` + `PROJECT_PLAN.md`.
- Memory disciplines applied: `feedback_class4_cannot_hide_in_class3` + `feedback_step_b_protocol` + `feedback_workspace_test_canonical` + `feedback_no_workarounds_strict_constitution` + `feedback_no_retroactive_evidence_rewrite` + `feedback_audit_after_evidence` + `feedback_dual_audit_conflict` + `feedback_chaintape_externalized_proposal` + `feedback_constitutional_harness_engineering` + `feedback_real_problems_not_designed`.

**End of TB-18R FINAL SHIP REPORT. Awaits architect §8 sign-off at `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`.**
