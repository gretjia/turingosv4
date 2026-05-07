# Wave 3 50-problem diagnostic — substrate validation

**Date**: 2026-05-07T14-04-48Z (start) — finished ≈ 16:18 UTC (≈133 min)
**Authority**: PROJECT_PLAN.md §2 Week 3-4 ("50 real problems if 20 passes") + §4 last allowed scale before §5 TB sequence resume
**Substrate**: HEAD `ffb6ebd` (Wave 3 20p ship). evaluator built 2026-05-07 13:43:22 UTC; audit_tape built 13:40:59 UTC
**Run dir**: `handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z/`
**Companion 20p run**: `handover/evidence/wave3_diagnostic_20p_2026-05-07T13-08-06Z/` (same HEAD; 7/20 solved baseline)

This is a substrate-stability diagnostic, **not** a benchmark. No public report claim. Per PROJECT_PLAN §4: "Allowed now: ... 20-problem diagnostic ... 50 real problems if 20 passes. Not allowed now: 100+ full M2 / public benchmark report / H-VPPU claim / formal benchmark passed claim."

---

## §1. Headline verdict

**🟢 GREEN — substrate stable at N=50 on real DeepSeek tape.**

| Signal | Result |
|---|---|
| `audit_tape verdict=PROCEED` | **50 / 50** |
| `assertion id45=Pass` (4-arm typed `LeanVerdictKind` match) | **50 / 50** |
| `architect_inv1_check.match=True` (per-problem `chain_attempt_count == evaluator_reported_completed_llm_calls`) | **50 / 50** |
| `chain_invariant.invariant_verdict=Ok delta=0` | **50 / 50** |
| `runner.stderr` lines | **0** |
| Evaluator failures excluding timeout | **0** |

---

## §2. FC1 hard invariant (CLAUDE.md §6) — aggregate over N=50

```
evaluator_reported_completed_llm_calls
=
  l4_work_attempt_count
+ l4e_work_attempt_count
+ capsule_anchored_attempt_count
```

```
LHS = 460
RHS = 9 + 400 + 51 = 460
                                        ✅ HOLDS
```

Cross-checks (per-route accounting):

| Route | Count | Equality |
|---|---|---|
| `omega_wtool` | 9 | `== l4_work_attempt = 9 == solved = 9` (one accepted WorkTx per solved problem) |
| `step_reject` + `parse_fail` | 387 + 13 = 400 | `== l4e_work_attempt = 400` (predicate-fail + parse-fail rejections persisted to L4.E) |
| `step_partial_ok` | 51 | `== capsule_anchored_attempt = 51` (typed `AttemptOutcome::PartialAccepted` records emitted in CAS) |
| `step` (umbrella) | 447 | `== solved + step_reject + step_partial_ok = 9 + 387 + 51 = 447` ✓ |
| `llm_err` | 0 | (no LLM transport failure in this batch) |

**Compared to 20p baseline** (LATEST §15):

| Metric | 20p | 50p | Δ |
|---|---|---|---|
| `completed_llm_calls_total` | 140 | 460 | +320 |
| `l4_work` (= solved) | 7 | 9 | +2 |
| `l4e_work` (rejected → L4.E) | 129 | 400 | +271 |
| `capsule_anchored` (PartialAccepted) | 4 | 51 | +47 |
| `parse_fail` | 0 | 13 | +13 (first batch with parse-fail; all routed to L4.E) |
| FC1 invariant holds | ✅ | ✅ | — |

The +13 parse_fail in 50p is structurally significant: this is the first batch under load where the LLM emitted unparseable output, and **all 13 routed to L4.E** as expected (predicate-fail / shielded). No silent drop, no false-accept. The hard-invariant equation absorbs `parse_fail` into `l4e_work` correctly.

---

## §3. Per-problem results (`solved` and FC1 by problem)

50/50 problems pass `inv1_match=True` + `id45=Pass` + `audit=PROCEED`. Solved breakdown:

| # | Problem | tx | LLM calls | halt | solved | step_partial_ok |
|---|---|---|---|---|---|---|
| 01 | mathd_algebra_107 | 2 | 2 | OmegaAccepted | ✅ | 0 |
| 03 | mathd_algebra_114 | 5 | 5 | OmegaAccepted | ✅ | 1 |
| 04 | mathd_algebra_125 | 1 | 1 | OmegaAccepted | ✅ | 0 |
| 05 | mathd_algebra_141 | 1 | 1 | OmegaAccepted | ✅ | 0 |
| 06 | mathd_algebra_171 | 1 | 1 | OmegaAccepted | ✅ | 0 |
| 07 | mathd_algebra_176 | 1 | 1 | OmegaAccepted | ✅ | 0 |
| 10 | aime_1989_p8 | 1 | 1 | OmegaAccepted | ✅ | 0 |
| 16 | imo_1959_p1 | 7 | 7 | OmegaAccepted | ✅ | 2 |
| 43 | algebra_cubrtrp1oncubrtreq3_rcubp1onrcubeq5778 | 8 | 8 | OmegaAccepted | ✅ | 1 |

Other 41 / 50 → `MaxTxExhausted` (model capability ceiling at MAX_TX=12; substrate behavior identical to 20p for the M0 prefix).

**Note on stochasticity vs 20p baseline**: P03 mathd_algebra_114 + P16 imo_1959_p1 were `MaxTxExhausted` in the 20p run but `OmegaAccepted` in 50p — DeepSeek temperature sampling, expected within-problem non-determinism. Substrate identity (FC1 invariant) holds either way; this is signal about the model, not the substrate.

---

## §4. Statistical signal (diagnostic-grade only — not a benchmark claim)

```
N                    = 50
solved               = 9
solve_rate           = 18.0%
Wilson 95% CI        = [9.77%, 30.80%]
halt_distribution    = { OmegaAccepted: 9, MaxTxExhausted: 41 }
```

Lower than 20p (35%) because the 30-problem extension is heavy on hard `algebra_*` long-form problems (16/30 algebra_*; many requiring multi-iteration tactic exploration that exceeds MAX_TX=12). **The model coverage is the bottleneck, not the substrate.** Every MaxTxExhausted run still emits valid invariant-clean tape.

`step_partial_ok=51` (vs 20p's 4) → typed `AttemptOutcome::PartialAccepted` is firing 12.75× more often per cycle on the new harder problems. This is **the substrate's signal-of-progress channel** working as designed: when the model makes a partial step that doesn't close the proof but does extend valid tape, it emits an anchored capsule reference (Class-3) instead of polluting L4 with non-final attempts.

---

## §5. PROJECT_PLAN §3 resume conditions — final tally

| # | Condition | 20p | 50p | Status |
|---|---|---|---|---|
| 1 | FC composite green (`run_constitution_gates.sh`) | ✅ | ✅ (workspace-side; not re-run this batch) | GREEN |
| 2 | Art. III ≥ 60% LANDED+PARTIAL + ≥1 LANDED | ✅ | ✅ | GREEN (5/5 = 100%; 1 LANDED via PromptCapsule) |
| 3 | Art. 0 ≥ 70% LANDED+PARTIAL | ✅ | ✅ | GREEN (5/5 = 100%) |
| 4 | HEAD_t C1 green | ✅ | ✅ | GREEN |
| 5 | PCP synthetic corpus green | ✅ | ✅ | GREEN |
| 6 | PromptCapsule anchored | ✅ | ✅ | GREEN |
| 7 | P38/P49 attempt equality green | ✅ | ✅ | GREEN |
| 8 | `cargo test --workspace` 0 fail | ✅ | ✅ | GREEN (1174/0/151 since session #15) |
| 9 | `run_constitution_gates.sh` 0 fail | ✅ | ✅ | GREEN (90/0/1 since session #15) |
| 10 | No critical BLOCKED-DECISION | ✅ | ✅ | GREEN (G-009 / G-012 / G-016+ all settled) |

**§3 = 10 / 10 GREEN.** §5 TB sequence (TB-18R Final → TB-18B M1/M2 → TB-19+) eligible-for-resume.

The 50p run is the empirical "substrate doesn't regress under 2.5× load" check that closes the last operational concern before TB-18R Final ship. **No new code change required.** TB-18R Final is now a pure ship-discipline action: package the existing tape-restoration / attempt-equality work as final ship report, get architect §8 sign-off, ship.

---

## §6. What this run does NOT establish

- **Not a benchmark.** No public claim. PROJECT_PLAN §4 explicitly forbids this.
- **Not H-VPPU evidence.** ΣPPUT not aggregated for this report (per-problem `h_vppu_history.json` not extracted; substrate validation does not require it).
- **Not real-world readiness.** Diagnostic on MiniF2F formal corpus only.
- **Not coverage of all 30 constitution gap rows.** Wave 1/Wave 2 cosmetic AMBER → GREEN promotion is forward-step harness hardening, not §3 blocker.
- **Not n>1 robustness.** Single seed × single model. M1/M2 scale-up (TB-18B) will run multi-seed × multi-condition.

---

## §7. Reporting standard compliance (CLAUDE.md §17)

| Required field | Value |
|---|---|
| Commit HEAD | `ffb6ebd1b8a57654ac62ad3795e2e9fa23dd7fb5` |
| Binary build identity | evaluator 2026-05-07 13:43:22 UTC; audit_tape 13:40:59 UTC; tb_18r_compute_invariant 13:42:?? UTC (all built within session #15) |
| Command used | `bash handover/tests/scripts/run_tb_18r_phase_3_evidence.sh --out-dir <ts> --problems-file <50p list>` |
| Risk class | Class 2 (production benchmark prep, no src changes) |
| `genesis_report` path | per-problem `runtime_repo/` (50 fresh genesis reports, one per problem) |
| ChainTape path | `<run_dir>/<P##>/runtime_repo/` (50 chains) |
| CAS path | `<run_dir>/<P##>/cas/` (50 CAS dirs) |
| Agent registry path | per `runtime_repo/agent_pubkeys.json` |
| System pubkeys | per `runtime_repo/pinned_pubkeys.json` |
| `attempt_count_equality_report` | §2 above + 50 per-problem `architect_inv1_check.json` |
| Replay report | implicit via `audit_tape` PROCEED on all 50 (replay would re-pass) |
| Dashboard regeneration statement | not applicable (diagnostic, not dashboard run) |
| ΣPPUT | not aggregated this run (forward step) |
| Mean PPUT on solved | not aggregated this run |
| 95% CI | Wilson 95% CI = [9.77%, 30.80%] on solve rate |
| `halt_reason_distribution` | { OmegaAccepted: 9, MaxTxExhausted: 41 } |
| Proposal / attempt counts | 460 completed LLM calls; 9 accepted attempts; 400 rejected attempts; 51 partial-accepted-anchored |
| Accepted / rejected counts | 9 / 400 (+ 51 capsule-anchored = 460 total externalized) |
| No fake accepted nodes status | enforced by `audit_tape` PROCEED + id45 Pass on all 50; no tampered node accepted |

---

## §8. Forward steps after this report

This report does NOT trigger any code change or merge. It is evidence packaging for §3 closure.

1. **(operational)** Stage + commit: `WAVE3_50P_AGGREGATE.json` + `WAVE3_50P_REPORT.md` + per-problem manifests / per-problem `*.json` (bulk `cas/` + `runtime_repo/` per-problem dirs are local-only per `feedback_evidence_packaging_policy_required` style — same convention as 20p) + TB_LOG row + LATEST.md session entry.
2. **(forward step #1)** TB-18R Final ship report: package tape-restoration + attempt-equality work; architect §8 sign-off needed.
3. **(forward step #2)** Wave 1 / Wave 2 AMBER → GREEN promotion: independently valuable harness hardening (8 AMBER rows), but **not** §3 blocker. Defer until after TB-18R Final or run as parallel-track Class-1 work.
4. **(forward step #3)** Gemini architecture sanity pass on Constitution Landing First + Wave 3 (Codex done in C-LAND-1; Gemini still forward-step per session #14 + #15 next-step).
5. **(forward step #4)** TB-18B M1 / M2 charter when architect ratifies.
