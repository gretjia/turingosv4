# TB-18 M1 Formal Benchmark Evidence — 2026-05-05T18-31-55Z

> **🛑 GRANDFATHERED 2026-05-06 — DO NOT USE AS BENCHMARK EVIDENCE.**
>
> This M1 result predates **TB-18R Tape Restoration**.
>
> - Charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`
> - VETO archive (lossless verbatim): `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`
>
> Per-LLM-call externalization was **not yet enforced** at this run. Failure-path asymmetry is present: success ω-paths externalize correctly, but `step_reject` / `parse_fail` / `llm_err` / `step_partial_ok` paths leak to evaluator stdout / kernel.tape shadow without L4 / L4.E entries. P49-class heavy runs show `evaluator_tx_count=32` but `L4_WorkTx=1` and `L4.E_real_LLM_rejection=0`.
>
> Ship verdict: **VETO** (external audit 2026-05-06; user-confirmed).
>
> **TB-18 M1 / M2 / M3 / NodeMarket / PriceIndex / Polymarket-signal / public-chain / real-world-readiness all FROZEN until TB-18R SHIPPED FINAL with G2 PASS.**
>
> Per `feedback_no_retroactive_evidence_rewrite`: this evidence is preserved as-is (no L4 / L4.E / CAS root rewrite). Annotation is going-forward only. Use the post-TB-18R-ship rerun evidence at `handover/evidence/tb_18r_p23_p38_p49_rerun_*/` and `handover/evidence/tb_18r_m0_rerun_*/` instead.

---

## Contents

- `MINIF2F_M1_BENCHMARK_REPORT.md` — original M1 report (also annotated at top with the same banner)
- `M1_RUN_MANIFEST.json` — manifest pinning problem set / model / seed / commit
- `M0_RUN_MANIFEST.json` — M0 preflight manifest
- `M0_BATCH_SUMMARY.json` — M0 batch metrics
- `EVIDENCE_INDEX.json` — index of per-problem artifact paths
- `P{01..50}_*/` — per-problem artifact directories (50 total)

## Per-problem artifact layout

Each `Pnn_*/` contains:
- `verdict.json` — solved / unsolved + PPUT
- `verdict_replay.json` — replay determinism check
- `tamper_report.json` — tamper detection on chain
- `h_vppu_history.json` — H-VPPUT trace

Note: `tool_dist` failure counters (`step_reject` / `parse_fail` / `llm_err` / `step_partial_ok`) for individual problems are NOT chain-anchored at this run; they exist only in evaluator stdout. This is the precise defect TB-18R remediates.

## Cross-references

- HEAD at run time: `c9e0dc1` (G0 CHALLENGE-resolved)
- Manifest ID: `652890ecc158d7139fb699b682298605bad3857ad3fb37a74dc6fd5a83fd57af`
- TB-18R Codex Gate 1 audit: `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md`
