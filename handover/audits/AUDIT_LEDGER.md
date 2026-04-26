# TuringOS v4 — External Audit Ledger

> **Purpose**: real-time tracking of every Codex / Gemini invocation per `TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md` § 6.
>
> **Update protocol**: every invocation appends one row at completion. Phase exits compute running totals; user weekly check shows cumulative.
>
> **Budget**: $435-950 over CO P0+P1+P2 (per `CO_P0_AMENDMENT_v1_2026-04-26.md` § 2). Mid-point $700; gates 80% ($560), 100% ($700).

---

## § 1 Running Totals

| Phase | Atom-level | Phase-exit | Sub-total |
|---|---|---|---|
| CO P0 | $0 | (in flight) | (in flight) |
| CO P1 | — | — | — |
| CO P2 | — | — | — |
| **Cumulative** | **$0** | **(in flight)** | **(in flight)** |

Pre-CO sunk cost (Phase A+B): ~$100 (carried from prior arc; not in $435-950 budget — that budget covers v4 refactor only).

---

## § 2 Invocation Log

| Timestamp | Atom / Phase | Model | Role | Cost ($) | Verdict | Notes |
|---|---|---|---|---|---|---|
| 2026-04-26 night | CO P0.7 | Codex | full audit on Blueprint + Plan v3.1 + Protocol + Amendment v1 | (pending) | (pending) | spawned via codex-rescue subagent |
| 2026-04-26 night | CO P0.7 | Gemini 2.5-pro | full audit on same packet | (pending) | (pending) | spawned via Bash + curl |

(Rows append as invocations complete.)

---

## § 3 Cost Breakdown Convention

Per Protocol § 5:
- Standard atom Codex review: $2-5
- STEP_B atom Codex implement+review: $5-10
- Gemini per-atom heavy review: $1-2
- Phase exit Codex full audit: $15-25
- Phase exit Gemini full audit: $10-15

Costs above are **estimates**; actual API spend logged when invocation returns. Discrepancy tracked in § 4.

---

## § 4 Estimate vs Actual Variance

| Cost class | Est avg | Actual avg | Δ |
|---|---|---|---|
| (data accumulates) | — | — | — |

---

## § 5 Escalation Triggers

- **80% threshold ($560 cumulative)**: ArchitectAI auto-escalates to user; proposes scope reduction or dual-audit cadence reduction
- **100% threshold ($700)**: hard pause; user sudo required to proceed
- **Single-atom audit fail rate > 30%**: signals atom design is unclear; ArchitectAI revises spec methodology
- **Codex / Gemini divergence rate > 20%** on PASS/CHALLENGE/VETO: signals atom specs ambiguous; ArchitectAI tightens spec template

---

— ArchitectAI, 2026-04-26 night (seeded)
