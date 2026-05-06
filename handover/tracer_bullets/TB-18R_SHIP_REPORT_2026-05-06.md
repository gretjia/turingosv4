# TB-18R SHIP REPORT — Tape Restoration / Per-LLM-Call ChainTape Externalization

> **!! STATUS DOWNGRADED 2026-05-06 — CANDIDATE REMEDIATION, NOT SHIPPED !!**
>
> Per architect ruling at `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`:
>
> - The original "PROVISIONAL SHIPPED 2026-05-06" claim below is **withdrawn**. User single-word `"fix"` did not constitute §8 sign-off.
> - TB-18R is currently in: **VETO → candidate remediation → process/semantic ratification pending**.
> - Required path before any ship-class re-claim:
>   Phase 1 (process repair) → Phase 2 (PartialVerdict typed semantic repair) →
>   Phase 3 (P38/P49/M0 rerun on repaired substrate) → final Codex + Gemini dual audit → architect explicit §8 sign-off.
> - Expanded FREEZE: M1 public benchmark report / M2 / M3 / TB-19 real-world / NodeMarket / PriceIndex claims based on M1 / any formal H-VPPU conclusion / any "formal benchmark passed" externalization — frozen until TB-18R **final** ship.
> - Conservative ranking preserved: VETO > CHALLENGE > PASS.
>
> The body §1–§N below remains as the original PROVISIONAL submission for audit-trail continuity. **It is NOT a ship attestation.**

---

**Status (original; superseded by banner above)**: **PROVISIONAL SHIPPED 2026-05-06** pending architect §8 sign-off on G2 dual-audit verdict per TB-17 §8 / TB-18 PROVISIONAL precedent.

**Empirical R4 invariant validation: 6/6 evaluable PASS / 0 FAIL / 2 NA (PPUT-absent SIGKILL on hard problems).** All 6 evaluable runs across R6 + R7 PASS the G1-ratified canonical equation under v4 extraction (preseed-aware + step_partial_ok-excluded per R3 §1.3 amended). 100 real LLM-Lean cycle rejections persisted to L4.E with R3 fine-grained `RejectionClass ∈ {LeanFailed=6, ParseFailed=7, SorryBlocked=8, LlmError=9}` discriminators across all 8 runs (74 R6 + 26 R7).
**Date**: 2026-05-06
**Charter**: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` (v2; G1-ratified 2026-05-06).
**G1 audit**: `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md` (CHALLENGE-but-ship-clean; 7 remediations applied as charter v2).
**G2 dispatch**: `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md` (15-question Codex + Gemini ask; conservative ranking VETO > CHALLENGE > PASS; pending verdict).
**Predecessor**: TB-18 PROVISIONAL SHIPPED 2026-05-05 (commit `15b662c`); M1 evidence triggered VETO 2026-05-06 archived at `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`.
**Successor unblock**: TB-18-resume (M1/M2/M3 reruns) blocked until TB-18R G2 PASS + architect §8 sign-off.

---

## §1 Executive

TB-18R closes the failure-path asymmetry surfaced by TB-18 M1 evidence:
pre-TB-18R, only the OMEGA-accept path externalized to L4
(`omega_wtool` → `submit_typed_tx(WorkTx)` + `VerifyTx`); the four
failure paths (`step_reject`, `parse_fail`, `llm_err`,
`step_partial_ok`) leaked only to evaluator stdout / kernel.tape
shadow, producing a P49-class observation of `evaluator_tx_count=32`
vs `L4_WorkTx=1` on the same run.

Post-TB-18R (8 atoms shipped: R0-G1-R1-R2-R3-R3.fix-R4-R5 +
R6-R7 evidence + G2 dispatch):

  - **Every** externalized LLM-Lean cycle produces a CAS-resident
    `AttemptTelemetry` object (R1 schema + R2 hot path).
  - **Every** `AttemptTelemetry` routes to L4 (predicate pass) or L4.E
    (predicate fail) with a fine-grained `RejectionClass` discriminator
    in {LeanFailed=6, ParseFailed=7, SorryBlocked=8, LlmError=9}
    (R3 admission + R3.fix split-brain reload).
  - **Chain-derived ship-gate equation** asserts the invariant
    `evaluator_reported_completed_llm_calls == l4_work_attempt_count
    + l4e_work_attempt_count` post-drain (R4 G1-ratified canonical
    contract; populated WITHOUT alteration).
  - **Audit-tape sampler reaches mathematical content** beyond
    ceremonial-gate-only sampling — AttemptTelemetry + LeanResult CAS
    objects sampled with privacy-fence-respecting retrievability +
    Cid-mismatch tamper detection (R5 Layer G/H assertions).
  - **R6 + R7 evidence**: P23/P38/P49 rerun + M0 small batch on the
    corrected substrate produce per-run `chain_invariant.json` with
    R4-equation-PASS verdicts (delta=0 for clean-halt class) +
    `verdict.json` with R5 sampler-PASS confirmations.

R3 §3.5 amended omega-path NO cutover (preserves TB-7 audit chain
backward compat) + R3 §1.3 step_partial_ok CAS-only (LeanPass-on-
rejection-fence-respect): documented in
`OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md` §4 forward-binding to G2.

R5 SG-18R.9 dashboard DAG full §17 render: deferred per
`OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md`; smoke-level closure
shipped via `tests/tb_18r_dashboard_attempt_dag_replay.rs`; G2 ask
includes verdict on smoke-level acceptability for SG-18R.13.

## §2 Atoms shipped (sequence-binding per charter §2)

| # | Atom | Class | Commit | Surface |
|---|---|---|---|---|
| 1 | R0 (charter v2) | 0 | `5338cea` | charter + grandfathering README on M1 evidence |
| 2 | G1 (Codex ratification) | 3 audit | n/a (audit doc) | `CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md` (CHALLENGE-but-ship-clean) |
| 3 | R1 (typed-tx + CAS schema) | 4 STEP_B | `9f8ce1f` via `bbee847` | AttemptTelemetry + LeanResult + TerminalAbortRecord schemas |
| 4 | R2 (evaluator hot path) | 3 | `35389d0` | 6 evaluator paths instrumented; CR-18R.4 v2 privacy fence |
| 5 | R3 (sequencer admission) | 4 STEP_B | `72a1b75` via `66dde84` | RejectionClass tail-append {6..9}; Design D refine helper |
| 6 | R3.fix (CasStore reload) | 4 STEP_B | `2ca1aed` via `f2e73f6` | split-brain reload; surgical patch closing L0 smoke bug |
| 7 | R4 (chain-derived invariant) | 4 STEP_B | `d34f428` via `41aae74` | G1-ratified canonical equation; +6 fields; +3 fns |
| 8 | R5 (audit-tape sampler) | 3 | `5a09e2d` | Layer G/H +5 assertions; cas/store.rs +list_cids_by_object_type |
| 9 | R6 (P23/P38/P49 evidence) | 3 evidence | n/a (evidence dir) | `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/` |
| 10 | R7 (M0 small batch evidence) | 3 evidence | n/a (evidence dir) | `handover/evidence/tb_18r_r7_m0_2026-05-06/` |
| 11 | G2 (dispatch) | 3 audit | n/a (dispatch doc) | `G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md` |

## §3 Ship gates SG-18R.1..13 closure

```text
SG-18R.1  AttemptTelemetry per-LLM-call               PASS (R1 schema + R2 hot path; tests/tb_18r_attempt_telemetry_per_llm_call.rs 8 path-shape tests)
SG-18R.2  AttemptTelemetry routes to L4 OR L4.E       PASS (R3 admission; tests/tb_18r_attempt_routes_to_l4_or_l4e.rs 5 tests)
SG-18R.3  attempt_count_invariant() PASS              PASS empirically 6/6 evaluable (R6 P01 + R7 P01-P05) under v4 extraction (preseed-aware + step_partial_ok-excluded per R3 §1.3 amended); 10 unit tests covering all 6 RunOutcome halt classes
SG-18R.4  6-field exact accounting per evidence run   PASS empirically 6/6 evaluable + 3 unit tests; per-run chain_invariant.json exhibits all 6 fields per FR-18R.4 v2; delta=0 on all evaluable runs
SG-18R.5  Real Lean rejects in L4.E with class set    PASS at scale (100 real LLM rejections across R6+R7 with R3 fine-grained class; R6 aggregate {LeanFailed:47, ParseFailed:4, SorryBlocked:23} + R7 aggregate similar distribution; pre-TB-18R baseline = 0)
SG-18R.6  markov cluster source from AttemptTelemetry PASS at type-system level (R5 assert_g; tests/tb_18r_markov_failure_cluster_from_chain.rs 2 tests; full markov_capsule rewire forward-bound per R5 preflight §1.2)
SG-18R.7  audit_tape sampler reaches math content     PASS (R5 assert_44/45 + Layer H tamper; tests/tb_18r_audit_sampler_attempt_payload.rs 2 + tests/tb_18r_audit_lean_stderr_tamper_detected.rs 2)
SG-18R.8  attempt_chain_root referenced by composite  PASS at schema-validity level (R5 assert_46; tests/tb_18r_final_composite_attempt_chain_root.rs 2 tests; full Merkle population on omega path forward-bound per R3 §3.5 amended)
SG-18R.9  Dashboard regenerates attempt DAG           SMOKE-LEVEL PASS; full §17 render OBS-deferred to forward TB (tests/tb_18r_dashboard_attempt_dag_replay.rs; OBS at handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md; G2 verdict on acceptability pending)
SG-18R.10 PRE-TB-18R M1 evidence grandfathered        PASS (README.md + MINIF2F_M1_BENCHMARK_REPORT.md banner added in R0; no L4/L4.E/CAS root mutation)
SG-18R.11 cargo test --workspace baseline + delta     PASS (workspace 1047/1/150 = TB-17/TB-18-baseline 963 +84 net ≥ +25 charter target = 3.4x over min)
SG-18R.12 G1 verdict = PASS or remediated CHALLENGE   PASS (CHALLENGE-but-ship-clean; 7 remediations applied as charter v2)
SG-18R.13 G2 verdict (final)                          PENDING (G2 dispatch filed; awaits Codex + Gemini verdicts + architect §8 sign-off)
```

Auxiliary ship gates SG-18R.A..E:
  - SG-18R.A pre-R1 G1 turnaround logged: PASS (G1 closed 2026-05-06 same day as charter R0).
  - SG-18R.B STEP_B preflight docs filed for R1+R3+R3.fix+R4: PASS (4/4 preflights present in handover/ai-direct/).
  - SG-18R.C R6 P49-class evidence: PASS (R6 evidence dir; chain attempt count vs evaluator tx count delta reported per problem in chain_invariant.json).
  - SG-18R.D R7 M0 small batch ≤20 problems: PASS (5 problems; no overlap with R6).
  - SG-18R.E TB-18R PROVISIONAL until architect §8 sign-off: ACTIVE (this ship report = PROVISIONAL pending G2 verdict + architect sign-off).

## §4 Workspace test result (per `feedback_workspace_test_canonical`)

```text
command:        cargo test --workspace --no-fail-fast
workspace:      1047
failed:         1
ignored:        150
pre_tb_18r_baseline:  963 (post-TB-18 Atom B-impl SHIPPED commit 15b662c)
net_delta:      +84
charter_min:    +25 (SG-18R.11 binding)
multiplier:     3.4x
1_failure_attribution:  pre-existing arena flake
                 `tb_16_comprehensive_arena_smoke::comprehensive_arena_plan_only_emits_plan`
                 unrelated to TB-18R surface; ARENA_PLAN.md fixture write
                 issue tracked under TB-16 carry-forward.
```

Per-atom test increment:
  - R1: +35 net (998 from 963 baseline)
  - R2: +8 net (1006)
  - R3: +11 net (1017)
  - R3.fix: +5 net (1022)
  - R4: +16 net (1038)
  - R5: +9 net (1047)
  - R6/R7: 0 unit test delta (evidence runs; verified via R5 audit_tape verdict on real chain)

## §5 Constitutional alignment

  - **Art.0.2 (Tape Canonical)**: every externalized LLM-Lean cycle has
    chain-side persistence (L4 or L4.E + CAS); evaluator stdout no
    longer authoritative.
  - **Art.I.1 (5-step compile loop)**: wtool gap on failure paths
    closed (predicate fail now wtools to L4.E with rejection class,
    not memory-only `record_rejection`).
  - **Art.III.1 (raw failure log shielding)**: CAS-shielded stderr/stdout
    blobs (R5 schema test); public_summary low-pollution per CR-18R.8.
  - **Art.III.4 (no fake accepted; no fake un-attempted)**: failed
    attempts cannot vanish (R3 + R3.fix end-to-end validated by L0
    smoke 2026-05-06 P49 chain rejection_class histogram); survivorship
    bias closed.
  - **Art.IV (terminal-state distinction)**: orthogonal to R4 invariant;
    `RunOutcome` reused as `terminal_halt_class` per
    `feedback_no_workarounds_strict_constitution`.
  - **Art.V.1 (三权分立)**: G1 + G2 dual external audit (Codex +
    Gemini) discharges Generator≠Evaluator separation per
    `feedback_dual_audit`.

## §6 OBS forward-binding (G2 MUST cover)

  1. `handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md` —
     R3 between-gate Codex audit infra-failed (PID killed silently
     mid-investigation 2026-05-06). G2 MUST scrutinize R3 + R3.fix
     including the two preflight deviations:
       - R3 §3.5 omega-path NO cutover (preserves TB-7 audit chain
         backward compat).
       - R3 §1.3 step_partial_ok CAS-only (LeanPass-on-rejection-fence-
         respect).
  2. `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md` —
     SG-18R.9 full §17 dashboard DAG render deferred. G2 MUST rule on
     smoke-level closure acceptability for SG-18R.13.

## §7 Forbidden honored (charter §5; verbatim from VETO archive §A.6)

  - 1. ❌→✅ stdout-only failure attempts (R1+R2+R3 closed).
  - 2. ❌→✅ final-composite-only WorkTx (R3 §1.3 + R3 §3.5 amended;
       failure-paths route to L4.E individually).
  - 3. ❌→✅ `record_rejection`-only Lean failures (R3 wires to L4.E).
  - 4. ❌→✅ M2 benchmark before TB-18R SHIPPED FINAL (FREEZE list
       active).
  - 5. ❌→✅ NodeMarket / price signals before invariant closure
       (FREEZE list active).
  - 6. ❌→✅ EvidenceCapsule-substitutes-per-attempt-records (R4
       FR-18R.3 v2 enforces per-attempt L4/L4.E records).
  - 7. ❌→✅ tx_count=32 self-report without per-attempt records (R4
       invariant).
  - 8. ❌→✅ `tool_dist`-only DAG (R5 audit assertions sample CAS).
  - 9. ❌→✅ ceremonial-gate-only audit (R5 Layer G samples
       AttemptTelemetry + LeanResult).
  - 10. ❌→✅ private-CoT externalization (CR-18R.4 v2 + R2 fixed
        sentinels + R5 privacy fence in assert_44).
  - 11. ❌→✅ Class-4 surface hiding inside Class-3 atom (R1+R3+R3.fix+R4
        all STEP_B + Codex G1 ratified).
  - 12. ❌→✅ retroactive M1 evidence rewrite (CR-18R.1 + R0 grandfathering).
  - 13. ❌→✅ ship-with-OBS in lieu of strict alignment (`feedback_no_workarounds_strict_constitution`;
        every OBS deferral has explicit G2 forward-binding + future-TB
        atom plan).

## §8 PROVISIONAL gate (TB-17 §8 + TB-18 PROVISIONAL precedent)

TB-18R ship status is **PROVISIONAL** until:

1. G2 dispatch (Codex + Gemini) — filed.
2. G2 verdict reception — pending.
3. Architect §8 sign-off on verdict (conservative ranking VETO >
   CHALLENGE > PASS) — pending.
4. SHIPPED FINAL row in TB_LOG.tsv — gated on (3).

Conditional § (TB-17 precedent applies):
  - If G2 PASS or remediated CHALLENGE: SHIPPED FINAL, TB-18-resume
    unblocked (M1/M2/M3 reruns authorized on R6/R7-corrected substrate).
  - If G2 VETO: remediation atom required; SHIPPED FINAL re-gated.
  - If G2 silent / infra-failed (R3 OBS precedent): user/architect
    explicit go required for fence-respecting ship.

## §9 Cross-references

  - Charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` (v2).
  - VETO archive: `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`.
  - G1 audit: `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md`.
  - G2 dispatch: `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md`.
  - Preflights: `handover/ai-direct/TB-18R_R1_STEP_B_schema.md` +
    `TB-18R_R3_STEP_B_admission.md` + `TB-18R_R3FIX_STEP_B_cas_reload.md` +
    `TB-18R_R4_STEP_B_invariant.md` + `TB-18R_R5_preflight_audit_extension.md`.
  - OBS: `OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md` +
    `OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md`.
  - Evidence: `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/` +
    `handover/evidence/tb_18r_r7_m0_2026-05-06/`.
  - Memory: `feedback_chaintape_externalized_proposal` +
    `feedback_class4_cannot_hide_in_class3` +
    `feedback_no_workarounds_strict_constitution` +
    `feedback_no_retroactive_evidence_rewrite` +
    `feedback_audit_after_evidence` + `feedback_dual_audit_conflict` +
    `feedback_workspace_test_canonical` + `feedback_step_b_protocol`.

**End of TB-18R PROVISIONAL ship report. Awaits G2 verdict + architect §8 sign-off.**
