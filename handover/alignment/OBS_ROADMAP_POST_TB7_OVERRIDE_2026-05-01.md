# OBS — ROADMAP_9_PHASE_2026-04-29 post-TB-7 sequencing override

**Date**: 2026-05-01
**Type**: Observation / forwarding note (architect-doc untouched per hygiene pattern)
**Authority**: `handover/directives/2026-05-01_TB7_ARCHITECT_RULING.md` §6
**Anchor**: `handover/tracer_bullets/TB-7_charter_2026-05-01.md` §13
**Status**: Active observation — controls forward TB sequencing decisions.

---

## §0 Why this OBS exists

`handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` is the architect-authored 9-phase sequencing axis (P0-P9) ratified 2026-04-29. Its post-TB-7 sequencing branch was further specified by `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md` §4.5, which sequenced:

```
TB-7:  P2 Agent proposal trail OR RSP-M0/M1 NodePosition
TB-8:  RSP-M2 NodeMarketEntry + PriceIndex v0
TB-9:  RSP-3.2 Slash execution
TB-10: RSP-M3 CompleteSet accounting
TB-11: RSP-4 SettlementEngine / ContributionDAG
TB-12: RSP-M4 MarketOrder / trading layer
```

The architect ruling 2026-05-01 (TB-7 ratification) **explicitly supersedes** this post-TB-7 sequencing per ruling §6.1 / §6.2 and TB-7 charter §13.1. This OBS captures the supersession without editing the original ROADMAP doc.

---

## §1 Hygiene pattern (why ROADMAP is not edited)

Per `feedback_alignment_*` conventions and the constitutional-hygiene pattern documented in `CLAUDE.md` Alignment Standard:

> "constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法"

The ROADMAP is not the constitution but is architect-authored at the same authority tier. We extend the same pattern: forward-redirect via OBS, not in-place edit. This:

1. Preserves the original ROADMAP as written (audit-trail integrity).
2. Routes future readers to the binding sequencing authority.
3. Lets the architect re-ratify the ROADMAP in a future pass if desired.

---

## §2 Authoritative new post-TB-7 sequence

The binding sequence is the one in TB-7 charter §13.1, NOT the one in ROADMAP_9_PHASE §3 / TB-6 ruling §4.5:

```
TB-7  (THIS)  — P2 Frame B — per-LLM-proposal WorkTx routing
TB-8          — Audit dashboard
TB-9          — Minimal payout (single solver/verifier; no royalty; no NodeMarket)
TB-10         — Beta launch (narrow Lean problem set; real ChainTape + payout)
TB-10.5       — Persistent AgentRegistry + agent keystore (durable identity)
TB-11         — NodeMarket v0 (FirstLong/Short positions; PriceIndex v0; not tradable)
TB-12+        — Polymarket-like full market
```

**Deferred post-MVP**: NodeMarket trading, AMM, public chain, MetaTape, multi-org, full RSP-4 settlement (royalty / ContributionDAG), P6 PPUT research expansion, h_vppu polish.

(Long-term reputation identity is NOT deferred — it lands at TB-10.5 because TB-11 NodeMarket v0 cannot ship without persistent owner identity. This was added at user direction post-ratification 2026-05-01 to close a gap in the ruling text.)

---

## §3 Why the override

Per ruling §6.2 / §8: each item in the old sequence (NodeMarket, Slash, Settlement, Polymarket) adds structural surface that REQUIRES real-proposal-on-chain anchor + real-payout grounding to mean anything economic. Building these on synthetic anchor or no-payout context expands kernel-only debt instead of closing it — the same trap that produced the "5-TB ChainTape production debt" pre-TB-6.

The MVP is the smallest vertical slice that delivers a launchable product:

```
正式上线 MVP = Lean Proof Task Market on ChainTape
```

NOT full TuringOS final form. The MVP is the gate that any further mechanism work passes through.

---

## §4 Phase mapping under the new sequence

The TB → phase_id mapping is preserved; only TB ordering changes within phases:

| TB | phase_id (primary) | phase_id (carry-forward) |
|---|---|---|
| TB-7  | P2 Agent Runtime | P1 / P3 |
| TB-8  | P2 Agent Runtime (UX layer) | P1 |
| TB-9  | P3 RSP (RSP-4 minimal) | P1 / P2 |
| TB-10 | P2 / P3 (Beta integration) | — |
| TB-10.5 | P2 Agent Runtime (durable identity) | — |
| TB-11 | P3 RSP (RSP-M v0 NodePosition) | P1 |
| TB-12+ | P3 RSP (RSP-M trading) | P1 |

P4 Loom, P5 MetaTape, P6 multi-org / Epistemic Lab, P7 Public, P8 Autonomous remain post-MVP.

---

## §5 Conformance / future-proofing

Any future TB charter that proposes one of the deferred-post-MVP items **before MVP ships** MUST:

1. Cite this OBS in §0 of the charter.
2. Justify the override in §13 of the charter (per `feedback_launch_priority`).
3. Trigger architect review (Class 3+ if it touches money movement).

Any future TB charter that follows the new sequence MUST:

1. Reference TB-7 charter §13 in §11 cross-references.
2. Include phase_id + roadmap_exit_criteria_addressed + kill_criteria_tested per `feedback_tb_phase_tag_required`.

---

## §6 Update / supersession

This OBS itself can be superseded by:

- A future architect ruling that re-ratifies the original ROADMAP §3 sequencing.
- A direct architect amendment to ROADMAP_9_PHASE_2026-04-29.md (which would re-render this OBS stale).
- MVP launch closure (TB-10 ship) → at that point the post-MVP phase ordering becomes the active question and this OBS may be archived.

Until then, this OBS is the active routing.
