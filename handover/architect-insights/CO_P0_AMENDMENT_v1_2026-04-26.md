# CO Phase 0 — Amendment v1 (D-Decisions + Tri-Model Cost)

> **Date**: 2026-04-26 night shift
> **Authority**: User 2026-04-26 — "本项目由你负责组织 codex 和 gemini 共同完成" + "我要睡了，你以 auto research 方式执行" → ArchitectAI granted autonomous CO P0 doc-only execution authority. Source code remains frozen until user wakes + approves CO P1 launch.
>
> **Supersedes**: CO_MEGA_PLAN_v3.1 § 2 CO P0 atom assignments are amended by this doc.

---

## § 1 D1-D6 Resolutions (PROVISIONAL — auto-research recommendations, NOT user approval)

> **Codex CO P0.7 audit fix**: original wording said "auto-research = all-rec" which Codex flagged as treating user-asleep state as user approval. Reframed: these are **PROVISIONAL** ArchitectAI recommendations applied to tonight's doc-only artifacts. **NONE are user-confirmed**. CO P1 entry is gated on explicit user confirmation (or override) of each D-decision.

| # | Decision | ArchitectAI rec (PROVISIONAL) | Rationale |
|---|---|---|---|
| **D1** | PREREG / PPUT-CCL fate | **C — MVP-pivot** | Preserves some Phase C output; abbreviated run after CO P1 (no full RSP); pragmatic balance vs total abandon (B) or 4-month freeze (A) |
| **D2** | Constitution Art. 0.5 | **B — pointer + 6 公理 only** | Keeps constitution.md compact; full white paper text stays in `handover/whitepapers/` |
| **D3** | TFR v1 disposition | **A — deprecate but preserve** | History preservation; LEGACY banner |
| **D4** | CO P3 (MetaTape) in v4 | **B — defer to v4.1** | **⚠️ Codex flagged contradiction**: WP architecture § 17 (lines 1013-1024) says v4 scope incl Phase 3 prep; Blueprint defers MetaTape to v4.1. User must reconcile or override. |
| **D5** | RSP depth | **A — full** | 12 invariants are interdependent; partial coverage violates Inv 5/7/8 by construction |
| **D6** | External audit cadence | **A — full (per phase + per STEP_B atom)** | Trust restoration via redundancy; tri-model co-execution mandates this |

**Wake action required**: user must explicitly confirm or override each D before CO P1 launch. "Silent acceptance" does not count. ArchitectAI commits to all-rec **only for tonight's reversible doc artifacts**; no irreversible actions taken.

---

## § 2 Cost Budget Amendment (Plan v3.1 § 6)

Plan v3.1 § 6 estimated $250-500 with Codex/Gemini as **auditors only**. TRI_MODEL_ORCHESTRATION_PROTOCOL § 5 reframes them as **co-executors**, costs:

| Cost class | Plan v3.1 (auditor) | Protocol (co-executor) |
|---|---|---|
| Atom-level | implicit in phase audit | $360-830 (110 standard × $2-5 + 22 STEP_B × $5-10 + 30 Gemini reviews × $1-2) |
| Phase exit | $250-500 | $75-120 |
| **TOTAL** | **$250-500** | **$435-950** |

**Amendment**: v4 budget is **$435-950** (mid-point estimate $700). Burn rate gates:
- Weekly check: cumulative spend reported in user check-in
- 80% threshold ($560 spent): escalate to user; consider scope reduction
- 100% threshold ($700): hard pause; user sudo required to proceed

Real-time tracking: `handover/audits/AUDIT_LEDGER.md` (seeded tonight; first entries from CO P0.7 dual audit).

---

## § 3 Tonight's Auto-Research Execution Scope

ArchitectAI will execute (DOC-ONLY, no src/ changes):

| Atom | Action | Reversible? |
|---|---|---|
| CO0.1 | FINAL_BLUEPRINT saved (committed earlier 2c3fd84) | ✅ done |
| CO0.2 | Plan v3.1 saved (committed earlier 2c3fd84) | ✅ done |
| **CO0.3** | DRAFT Constitution Art. 0.5 → `CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md` | ✅ DRAFT (user enacts on wake via cp) |
| **CO0.4** | DRAFT PREREG_AMENDMENT_v2 → `handover/preregistration/PREREG_AMENDMENT_v2_2026-04-26.md` | ✅ DRAFT (user enacts on wake) |
| **CO0.5** | TFR v1 deprecate banner → `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md` | ✅ direct edit (D3=A authorized) |
| **CO0.6** | Trust Root manifest **43→49 entries (+6)** → `genesis_payload.toml` | ⚠️ direct edit (TR mutation; recoverable via git revert if dual audit VETOes; Codex CO P0.7 audit flagged earlier "47/48/49" drift across docs as governance integrity issue — corrected post-audit) |
| **CO0.7** | Spawn Codex + Gemini dual audit on Blueprint + Plan v3.1 + Protocol + this amendment | ⚠️ external API calls (irreversible spend ~$30-50) |
| **AUDIT_LEDGER seed** | `handover/audits/AUDIT_LEDGER.md` | ✅ doc creation |
| **LATEST.md update** | `handover/ai-direct/LATEST.md` | ✅ doc edit |
| **ScheduleWakeup** | check audit results in ~30min | ✅ schedule entry |

Items NOT executed tonight (require user awake):
- Constitution.md cp workflow enactment (Art. 0.5 stays DRAFT)
- PREREG_v2 enactment (stays DRAFT)
- CO P1 entry (any src/ change, including the bus.rs/kernel.rs splits)
- Any STEP_B atom

---

## § 4 Wake-up Briefing Items for User

When user wakes, ArchitectAI will surface (in LATEST.md + first chat message):

1. **Audit verdicts** — Codex + Gemini PASS/CHALLENGE/VETO on Blueprint + Plan v3.1 + Protocol + Amendment v1
2. **Constitution Art. 0.5 enactment request** — formal sudo + cp workflow, OR override D2 to A (full text)
3. **PREREG_v2 enactment request** — formal sudo, OR override D1 to A (PAUSE) / B (NEGATIVE)
4. **TR manifest review** — 5 new entries; if dual audit CHALLENGE/VETO, partial revert possible
5. **CO P1 launch GO/NOGO** — assuming PASS/PASS, ArchitectAI proposes CO P1.0 entry (Constitution Root formalization, 2 atoms, 1-2 days)
6. **Cost ledger** — first entries: tonight's spend + projected P1 spend
7. **Sleep-shift incidents** — anything unexpected (e.g., Gemini API error → no audit available)

---

## § 5 Self-Audit on This Amendment

What this amendment commits to:
- **all-rec D1-D6** with explicit user override path
- **$435-950 tri-model budget** with weekly check + 80%/100% escalation gates
- **DOC-ONLY night shift** — no src/ changes; no irreversible architectural moves
- **Audit-first** — dual audit on full v3.1 stack before CO P1 entry

What this amendment does NOT do:
- Enact Art. 0.5 (still DRAFT)
- Enact PREREG_v2 (still DRAFT)
- Touch src/{bus,kernel,wal,ledger}.rs in any way
- Spend more than ~$30-50 tonight (one Codex + one Gemini invocation)

What this amendment is honest about:
- "auto-research mode" is a strong delegation but ArchitectAI deliberately stays conservative for irreversible actions
- D-decisions are taken on user's behalf; reversal on wake is normal and not a failure
- Tri-model cost projection has wide uncertainty band ($435-950); first weekly burn rate calibrates

— ArchitectAI, 2026-04-26 night
