# Whitepaper v2 — Tactical Constitutional-Level Alignment Note

**Date**: 2026-04-27
**Status**: ratified by user (sole human architect)
**Subject document**: `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md`

---

## § 1 What this note declares

The document `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` is hereby designated **tactical constitutional-level alignment**, which means:

- **Authority above** all derivative documents — Plan v3.2, Blueprint v1, Whitepaper v1, Deepthink draft, AUTO_RESEARCH_NOTEPAD — when text conflicts.
- **Authority below** `constitution.md`, which remains the unique formal alignment document and cannot be amended without a Phase Z′ 6-stage rerun (Art. IV + C-069).
- **No flowchart edit** is performed. No FC element renumbering. No TRACE_MATRIX rewrite. No conformance test rebuild.

Net effect: forward-looking decisions are checked against (constitution.md ∧ v2 whitepaper). Past code that complies with constitution but predates v2 framing is **grandfathered** unless v2 reveals a constitutional violation (none detected at ratification time).

---

## § 2 Why "tactical" instead of full constitutional merge

A full merge would trigger:
- Phase Z′ 6-stage rerun (per Art. IV + C-069)
- TRACE_MATRIX_v3 → v4 rewrite (FC elements added/renumbered)
- `tests/fc_alignment_conformance.rs` rebuild for new FC nodes
- Dual external audit on every src/ pub symbol's flowchart re-mapping
- Plan v3.2 → v3.3 atom renumbering

User explicitly chose to skip these heavy gates ("先不合并"). v2 instead acts as **the highest校准 mirror** for derivative documents, without forcing the alignment chain to be re-derived.

This is consistent with C-069 spirit (constitution edits are sacred) while still elevating v2 above the day-to-day planning docs.

---

## § 3 Conflict-resolution table (v2 vs derivative docs)

When a derivative doc disagrees with v2, v2 wins. Concrete points where this matters:

| # | Derivative doc | Conflict | v2 ruling |
|---|---|---|---|
| 1 | Whitepaper v1 (Deepthink) — chain-as-primary framing | "TuringOS is a verifiable state-ledger OS" | **v2 § 公理 5**: "区块链不是本体，只是 tape 的一种实现" — chain is implementation, not body |
| 2 | Blueprint v1 — file-level v4 spec emphasizes ledger | Two-layer (Agent + Ledger) implicit | **v2 § 3**: three-layer Anti-Oreo (top-white predicates / middle-black agents / bottom-white tools) — bottom tools are first-class, not subordinate to ledger |
| 3 | CO_MEGA_PLAN_v3.2 — 170 atom flat list | No explicit phase narrative | **v2 § 17**: 5-phase roadmap (GitTape → LedgerTape → MetaTape → PermissionedChainTape → PublicSettlement) becomes the narrative overlay; atoms group under phases without renumbering |
| 4 | INV8_DAG_DETERMINISM_SPEC_v1 (currently VETOED) | Algorithm-design level | **v2 § 5.1 Layer 4 + § 13.5**: re-checked under "transition ledger is implementation, not the body" lens — INV8 v2 must serve Layer 4 contract, not stand alone |
| 5 | PCP framing in older docs | "Probabilistically Checkable Proofs" complexity-theoretic | **v2 § 6.4**: engineering PCP — "正确候选应尽量不被误杀，错误候选应高概率被拦截" — no math overcommitment |

---

## § 4 Substantive new doctrine (sedimented as OBS files)

Four items in v2 introduce framing not previously explicit. Each gets an OBS file rather than direct case/rule promotion (per C-069 hygiene: observe first, codify later):

| Item | OBS file | Future ratification path |
|---|---|---|
| 创造域 vs 安全域 dual rejection mode | `OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md` | Could become a new case (C-077?) if pattern recurs in audit |
| Public / Private / Commit-Reveal predicate trinity | `OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md` | Already partly implemented (CO1.5 enum); commit-reveal path scheduled for later wave |
| Q_t five-root extension (state/ledger/budget/predicate-registry/tool-registry) | `OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md` | May open CO1.2-v2 atom (Q_t struct extension); current Q_t backwards-compatible via stub-default |
| InitAI as system component | `OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md` | Conceptual placeholder; no implementation atom yet |

---

## § 5 ChainTape directive — "项目全面向区块链前进" interpreted

User's directive accompanying ratification: **"项目全面向区块链前进"** ("project comprehensively advances toward blockchain").

**Officially interpreted as**: ChainTape vertical (**Trust Anchor Layer 0 + ChainTape Layers 1–6** per v2 § 5.1) becomes the primary engineering thrust for Wave 6+. Note: Layer 0 (Constitution Root) is the trust anchor outside the six ChainTape implementation layers; ChainTape proper is L1 PredicateRegistry → L2 ToolRegistry → L3 CAS → L4 TransitionLedger → L5 MaterializedView → L6 SignalIndices.

**Officially NOT interpreted as**: "blockchain becomes the body of TuringOS" — that reading was explicitly rejected and would invalidate v2 § 公理 5 + § 13.

User confirmation phrase: **"按白皮书和宪法以最高准则，区块链 chaintape 也是在反奥利奥架构中的"** — ChainTape lives within the Anti-Oreo architecture.

### 5.1 Wave 6 priorities re-ordered under ChainTape lens

Pre-v2 Wave 6 candidate ordering (from LATEST.md):
1. INV8 spec v2 revision
2. CO1.7 transition_ledger
3. CO1.1.4-pre1.b fixture corpus
4. CO1.1.4 bus.rs split (STEP_B)
5. CO1.1.5 kernel.rs split (STEP_B)
6. F ceremonies

Post-v2 ChainTape-lens ordering (recommended):
1. **CO1.7 transition_ledger** (Layer 4) — **promoted** because v2 makes Layer 4 the central artifact connecting agents to state
2. **CO1.1.4-pre1.b fixture corpus** — engineering pre-req for STEP_B byte comparison; unblocks STEP_B
3. **INV8 spec v2 revision** — required to clear CO P2.4.0 attribution gate; now scoped under Layer 4 (transition ledger DAG)
4. **CO1.1.4 / CO1.1.5 STEP_B** — pair with #2 fixtures
5. **F ceremonies** — user-led; independent of Wave 6 critical path

User retains override authority on this ordering.

---

## § 6 Audit posture

Under v2 ratification + ChainTape directive, the dual external audit (Codex + Gemini) on v2 itself is being run in this same ratification cycle. Verdicts feed into AUDIT_LEDGER. Conservative-wins (VETO > CHALLENGE > PASS) per `feedback_dual_audit_conflict` memory.

Even a CHALLENGE/CHALLENGE outcome does not retract v2's tactical-alignment status — it produces v2.1 patches. Only a VETO finding that v2 contradicts constitution.md would force a retraction.

---

## § 7 Preserve / retire

- **Preserve**: Whitepaper v1, Whitepaper v1 Economic, Blueprint v1, Plan v3.2, all historical audit transcripts, all OBS files, AUDIT_LEDGER. Nothing is deleted.
- **Retire (semantically, not physically)**: any standalone phrase in v1 / Blueprint / Deepthink that asserts "ledger / blockchain is the body of TuringOS." Such phrases are now **historical drafting language** superseded by v2 § 公理 5.

---

## § 8 References

- `constitution.md` — supreme; unchanged at v2 ratification
- `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` — subject of this note (v2.1 patched 2026-04-27 same day per dual-audit round-1 must-fix)
- `handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md` — predecessor; preserved
- `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` — file-level spec; preserved
- `handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md` — atom plan; v2 overlays narrative without renumbering
- `handover/audits/AUDIT_LEDGER.md` — v2 audit rows recorded here (round-1 + round-2)
- `cases/V3_LESSONS.md` — historical lessons; some now reframable under v2 lens

---

## § 9 Sunset Clause — Tactical Status is Time-Bounded

(Added 2026-04-27 per Gemini round-1 audit Q10 CHALLENGE: tactical-alignment dual authority creates governance debt; must declare sunset triggers.)

The "tactical constitutional-level alignment" status of v2 is **explicitly time-bounded and condition-bounded**. v2 cannot remain in this dual-authority state indefinitely. One of the following sunset triggers MUST fire to retire the tactical status:

### § 9.1 Hard date trigger
**Mandatory review by 2027-01-01** (≈ 9 months from ratification): human architect (user) reviews whether v2 still serves its 校准 purpose. Outcomes:
- (a) **Full constitutional merge** — v2 content folded into constitution.md via Phase Z′ 6-stage rerun (Art. IV + C-069); tactical status retired.
- (b) **Retire as historical** — v2 archived; constitution.md continues solo.
- (c) **Renew tactical status** — explicit re-ratification with new sunset date (max +6 months).

### § 9.2 Phase trigger
**Before Phase 4 (Permissioned ChainTape) entry**: any pre-Phase-4 wave touches multi-org governance, which v2 § 13.3 endorses. Multi-org introduction without resolved constitutional supremacy = governance ambiguity at the worst time. **Phase 4 entry is BLOCKED until tactical status is retired** (either by full merge or formal retirement).

### § 9.3 Conflict-count trigger
**N=3 v2-only doctrine arbitrations**: if § 10 (conflict-resolution) escalates to human architect three times within any 90-day window, tactical status is automatically suspended pending review. This bounds the "drift cost" of dual authority empirically.

### § 9.4 Sunset audit trail
Each trigger event is recorded in AUDIT_LEDGER as a §-9.x row. No sunset trigger may be silently ignored. If hardware date trigger (§ 9.1) is missed, v2 enters automatic suspension on 2027-01-02 until human architect formally addresses it.

---

## § 10 Conflict-Resolution Process — When v2 Disagrees With Constitution or Plan v3.2

(Added 2026-04-27 per Gemini round-1 Q10 + Codex round-1 Q9/Q10: dual authority needs explicit arbitration mechanics.)

When a forward-looking decision needs to be made and v2 conflicts with another authoritative document, the following process applies:

### § 10.1 v2 vs constitution.md
**constitution.md ALWAYS wins.** If v2 text appears to authorize an action that constitution.md forbids, OR forbid an action that constitution.md authorizes, the answer is determined by constitution.md. v2 is treated as having a drafting error in that section, which must be patched in v2.x.

**Detection mechanism**: Codex / Gemini external audits (per Tri-Model Protocol) MUST flag v2-vs-constitution conflicts as VETO. The Codex round-1 audit on v2 (2026-04-27) demonstrated this mechanism worked as designed — Q3 identified the sudo-scope drift (v2 § 16.4 vs Art V.1.1 line 715) and triggered v2.1 patch.

**Required action on flag**: v2.x patch within current session if doc-only fix; escalate to human architect if structural; never accept "v2 says X" as a defense for breaching constitution.

### § 10.2 v2 vs CO_MEGA_PLAN_v3.2
**Domain-split, not winner-take-all** (per v2 § 17.6 Plan-of-Record boundary):
- **§ 17 (narrative phase)**: v2 wins on phase ordering, phase naming, narrative groupings.
- **Atom-level (CO-id, dependencies, scheduling, budget, audit cadence)**: Plan v3.2 wins.
- **Conflict on which is which**: escalate to human architect; do NOT auto-resolve by either side claiming priority.

### § 10.3 v2 vs Blueprint / Whitepaper v1 / Deepthink
**v2 wins** (per § 1 of this note). These derivative documents are explicitly subordinated to v2 in the alignment hierarchy. Conflicting passages in v1 / Blueprint / Deepthink are treated as historical drafting language superseded by v2 § 公理 5 + § 13 + § 18.

### § 10.4 Mandatory escalation triggers
Even within v2's authority scope, the following situations REQUIRE escalation to human architect (no autonomous v2-based resolution):
- Any action that would touch `constitution.md` (always escalate, regardless of what v2 says).
- Any action that would touch `genesis_payload.toml` Trust Root authored under v2-derived motivation (escalate; per Art V.1.2 ArchitectAI may commit, but here v2 is the author of the proposal, creating self-review risk).
- Any sudo-related operation (per § 10.1, scope is constitutional only; if v2 wording suggests broader scope, escalate first).
- Any action that would invoke multi-org / cross-trust-domain governance (Phase 4 territory; per § 9.2, blocked entirely until sunset).

### § 10.5 Telemetry
Every § 10 invocation is logged in AUDIT_LEDGER with category `v2-arbitration`. § 9.3 conflict-count trigger uses these rows.

---

— ArchitectAI (orchestrator), 2026-04-27, ratified by user; v2.1 patches applied 2026-04-27 same day per Gemini Q10 + Codex Q3/Q9/Q10
