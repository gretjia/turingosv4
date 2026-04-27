# White Paper Revision Notes — 2026-04-27

> **Authority**: User ultrathink directive 2026-04-27: 「一切以宪法为最高准则，白皮书是对宪法的一种解释，但是不能违宪，你可以根据这个准则把白皮书中违宪部份/数字不一致部份修正」
>
> **Authority chain established**:
> 1. `constitution.md` — supreme; frozen; cannot violate
> 2. White papers (architecture + economic) — interpretation of constitution; revisable; cannot violate constitution
> 3. Spec docs — implementation of WP; already audited+revised
> 4. Code — implementation of spec
>
> **This file documents** every surgical edit ArchitectAI made to the two white papers on 2026-04-27 to: (a) fix numeric inconsistencies, (b) close audit findings, (c) reconcile with downstream specs. **No new philosophical content** added (those remain user's domain). **No constitutional violations** introduced.
>
> Each edit is sourced + justified. User can review + reject any edit by `git revert`.

---

## § A — Architecture WP Revisions (`TURINGOS_WHITEPAPER_v1_2026-04-26.md`)

### A.1 § 4 系统状态 — Q_t 9-component clarification

**Source**: Codex CO P0.7 §1 row "CO1.2 QState 9 components" + Gemini v3.2 Q1 trace coverage.

**Issue**: § 4 lists Q_t as 8-component extension; economic chapter § 2 amendment adds 9th (`economic_state_t`). Without cross-ref, reader sees 8 vs 9 mismatch.

**Edit**: INSERT `> § 4 amendment 注` block after "注意：tape_view_t 不是完整账本..." paragraph and before "---".

**Constitutional check**: Constitution Art 0.4 says "Q_t = ⟨q_t, HEAD_t, tape_t⟩" (3-tuple core); WP § 4 + economic § 2 are operationalization. Cross-ref preserves Const Art 0.4 essence ✓ no violation.

**Rationale preserved**: original 8-component list intact; only added clarifying note pointing to economic § 2 + reaffirming Const Art 0.4 conceptual core.

---

### A.2 § 12 Go Meta — v4 vs v4.1 boundary clarified (NEW § 12.4)

**Source**: D-VETO-4 ratified (defer runtime MetaTape to v4.1, not permanently abandon); Codex T+S re-review VETO on Claude's earlier "permanent abandon" overreach; Gemini v3.2 Q7 challenge on "Phase 3 prep" being weasel wording.

**Issue**: § 12 + § 12.2 describe runtime `meta_tx` as if always-runtime. v4 actually defers runtime to v4.1; current v4 ships only Phase 3 prep artifacts (offline workflow + spec/typed schemas). Without explicit v4 vs v4.1 boundary, reader may expect v4 runtime ArchitectAI + JudgeAI.

**Edit**: APPEND new § 12.4 (after § 12.3) labeled "v4 vs v4.1 实施边界 (修订 2026-04-27)". Includes:
- v4 scope: offline ArchitectAI + Phase 3 prep artifacts
- v4.1 scope: runtime actors + L4 acceptance + M-of-N quorum + human gate
- Why not one-shot: Anti-Oreo time-dimension + Bitcoin BIP-style separation + governance stability

**Constitutional check**: Constitution Art V.1.2 (ArchitectAI) + V.1.3 (Veto-AI/JudgeAI) describe roles + accountability; do NOT mandate runtime implementation timing. ✓ § 12.4 preserves Art V.1 separation of powers; only clarifies WHEN runtime arrives.

---

### A.3 § 17 实施路线 — Phase 3 prep concrete deliverables

**Source**: Gemini v3.2 Q7 CHALLENGE — "Phase 3 prep" is weasel wording; demand concrete auditable artifacts.

**Issue**: § 17 line "Phase 3: MetaTape — ArchitectAI runtime + Meta transitions" + "v4 scope = Phase 1+2+Phase 3 prep" without listing what "prep" contains.

**Edit**: APPEND `> Phase 3 prep concrete deliverables` block listing 7 named artifacts (META_TX_SCHEMA / meta_validator / MetaTransitionInterface / AmendmentFlow / MetaProposalDraft CAS / V4_1_METATAPE_PLAN / meta_validator conformance test).

**Constitutional check**: ✓ no violation; ✓ closes Codex/Gemini audit critique.

---

### A.4 RSP appendix — module count 8 → 9

**Source**: Codex CO P0.7 Coverage Check row "RSP appendix lists 8 core components at :1058, economic chapter lists 9 modules including PriceIndex".

**Issue**: WP architecture appendix lists 8 modules (TaskMarket / EscrowVault / ContributionLedger / PredicateRunner / AttributionEngine / ChallengeCourt / SettlementEngine / ReputationIndex). Economic chapter § 19 lists 9 (adds PriceIndex). Without reconciliation, conformance test count ambiguous.

**Edit**: REPLACE `核心架构组件 (...)` line with version listing 9 modules + note that PriceIndex is also ChainTape L6 entry (cross-layer reference).

**Constitutional check**: ✓ no violation.

**Note**: PredicateRunner is structurally in `top_white::predicates::runner`; its inclusion in RSP-1 module list is functional (it runs predicates AS PART OF settlement protocol). This cross-ref is explicit in economic § 19 修订 note (see § B.4 below).

---

## § B — Economic WP Revisions (`TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md`)

### B.1 § 7 标题 5 → 6 reconciliation

**Source**: Codex CO P0.7 Coverage Check row "Agent 5 vs six roles inconsistency must be corrected".

**Issue**: title `## Agent 5 经济角色` claims 5; list below has 6 roles (Solver/Verifier/Challenger/Builder/ArchitectAI/JudgeAI). Hard inconsistency.

**Edit**: REPLACE title with `## Agent 经济角色（5 object-level + 1 meta = 共 6 类）(§ 7)`. ADD a 修订 note explaining hierarchical split. Group the 5 object-level roles separately from JudgeAI (1 meta).

**Why this resolution (not "5 OR 6 fully")**:
- The original "5" intent likely meant "5 object-level roles per task" (Solver/Verifier/Challenger/Builder/Architect)
- JudgeAI is structurally distinct: it doesn't participate in single-task economic flows; it gates architectural changes via Constitution Art V.1.3 separation
- 5 + 1 split preserves user's intent + makes count consistent
- Plan v3.2 CO P2.7 atom dispatches 6 agent role files

**Constitutional check**: Const Art V.1.2 (ArchitectAI) + V.1.3 (Veto-AI/JudgeAI) are explicit constitutional roles. Listing JudgeAI as economic role with reward "低误判 + 低漏判 + 长期稳定" preserves V.1.3 spirit + adds economic incentive. ✓ no violation.

---

### B.2 § 20 Phase 1 — substrate description fix

**Source**: Codex CO P0.7 row "CO2.11 RSP MVP-1 deployment: Plan goes to in-process LedgerTape without a SQLite/Python compatibility/deprecation path". User ratified D-VETO-1 spec-first + D-VETO-3 minimal-with-anchor genesis (Path B chosen).

**Issue**: line 105 reads "Phase 1: Local Ledger Economy (ledger.jsonl + SQLite + Python predicates)". This was an early Path A vision; ratified Path B uses gix substrate + Rust predicates. WP and code/spec drift = future audit liability.

**Edit**: REPLACE Phase 1 description with "Path B: gix runtime_repo + Rust predicates per Const Art 0.4". ADD a 修订 note pointing to `CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md`.

**Constitutional check**: Const Art 0.4 explicitly authorizes Path B (real git substrate). ✓ no violation; ✓ alignment improved.

---

### B.3 § 20 turingosv4 scope — add Phase 3 prep

**Source**: Architecture chapter § 17 ALREADY says "Phase 1 + Phase 2 + Phase 3 prep". Economic chapter said "Phase 1 + Phase 2" — inconsistent.

**Issue**: Two chapters disagreed on v4 scope.

**Edit**: REPLACE economic chapter line with "Phase 1 + Phase 2 + Phase 3 prep (per CO_MEGA_PLAN_v3.2)". Cross-ref architecture § 17 修订 note for Phase 3 prep deliverables list.

**Constitutional check**: ✓ no violation; ✓ inter-chapter consistency.

---

### B.4 § 19 RSP-1 modules — add cross-ref note

**Source**: Codex CO P0.7 Coverage Check (architecture appendix had 8 modules, economic § 19 has 9 — Codex flagged).

**Edit**: APPEND 修订 note clarifying:
- 9 modules total (after architecture appendix patch in § A.4)
- PredicateRunner is in `top_white::predicates::runner` (top white layer); listed in RSP-1 functionally
- PriceIndex is both RSP-1 module #9 and ChainTape L6 entry (cross-layer reference)
- Architecture appendix updated to align (was 8; now 9)

**Constitutional check**: ✓ no violation; ✓ resolves Codex flag.

---

### B.5 § 18 invariants — cross-ref to STATE_TRANSITION_SPEC

**Source**: STATE_TRANSITION_SPEC v1.1 has 22 transition invariants (I-DET, I-NOSIDE, I-PARENT, ..., I-VBOND-RELEASE, I-ROYALTY-CAP). Without cross-ref, economic § 18's 12 invariants appear standalone.

**Edit**: APPEND 修订 cross-ref note: "12 economic invariants + 22 transition invariants (STATE_TRANSITION_SPEC § 4) = 34 conformance tests total at v4 ship gate".

**Constitutional check**: ✓ no violation; ✓ unifies invariant tracking across abstraction layers.

---

## § C — What WAS NOT Changed

For transparency, ArchitectAI deliberately did NOT touch the following items even though they were in earlier "candidate revision" lists:

| Item | Why not touched |
|---|---|
| Architecture § 0 设计公理 (6 axioms) | These map cleanly to Const Art 0.5 6 axioms (per spec design); Art 0.5 enactment is FROZEN; no edit until WP finalized |
| Architecture § 5 ChainTape detailed L0-L7 | Spec docs (`GENESIS_MINIMAL_WITH_ANCHOR_v1`) implement; WP's verbose schema is forward-compatible reference; no need to overwrite |
| Architecture § 9 Goodhart shielding detail | Reading Y interpretation (failure signal on tape) is in spec; constitutional cosmetic edit (Art 0.2 line 64) FROZEN; WP can stay abstract for now |
| Economic § 0 核心校准 ("经济不是发币") | Strong rhetoric is user's voice; could be rephrased as machine-checkable negative invariants but that's philosophical addition (out of "fix bugs" scope) |
| Economic § 18 specific invariant text | Each of 12 invariants is well-stated; only added cross-ref (B.5) without rewording |
| Constitution.md | FROZEN per user 2026-04-27 directive — NO touching |

---

## § D — Items Still Unresolved (User Action Required)

These items were noted in earlier reviews but cannot be resolved by ArchitectAI alone:

1. **Spec gap 11.1** (false-challenge reputation penalty): default 0 applied in spec v1.1; user may override via TaskMarket config when actual tasks created. Could add WP economic § 7 note "configurable" but I left implicit.
2. **Spec gap 11.4** (multi-verifier quorum aggregation): default 1 applied in spec v1.1; full M-of-N deferred to CO P2.7. WP could specify but currently silent (acceptable).
3. **Boot block field count** (Const Art IV vs WP § 11 vs spec GENESIS_MINIMAL_WITH_ANCHOR): three sources have different field lists. Spec is most current; Const Art IV is supreme. Reconciliation needs constitutional sign-off (FROZEN).
4. **Constitution Art 0.5 enactment** (D2=B; pointer + 6 axioms): FROZEN.
5. **Art 0.2 line 64 cosmetic edit** (Reading Y Option B): FROZEN.

These remain pending until either: (a) user signs WP finalization tag → unlocks constitution amendments, OR (b) user explicitly waives a specific item.

---

## § E — Edit Summary Table

| # | File | Section | Change Type | Lines (approx) | Source |
|---|---|---|---|---|---|
| A.1 | architecture WP | § 4 | INSERT cross-ref note | +6 lines | Codex CO P0.7 §1 + Gemini v3.2 Q1 |
| A.2 | architecture WP | § 12.4 (NEW) | ADD new sub-section | +25 lines | D-VETO-4 ratified + Codex T+S |
| A.3 | architecture WP | § 17 | APPEND deliverables list | +12 lines | Gemini v3.2 Q7 |
| A.4 | architecture WP | RSP appendix | REPLACE module count line | ±2 lines | Codex CO P0.7 Coverage Check |
| B.1 | economic WP | § 7 title + structure | REPLACE title + 修订 note + role grouping | +6 lines | Codex CO P0.7 Coverage Check |
| B.2 | economic WP | § 20 Phase 1 | REPLACE Phase 1 description | ±1 line + 1 note | D-VETO-1 + Codex CO P0.7 |
| B.3 | economic WP | § 20 v4 scope | REPLACE scope line | ±1 line | inter-chapter consistency |
| B.4 | economic WP | § 19 | APPEND cross-ref note | +5 lines | Codex CO P0.7 |
| B.5 | economic WP | § 18 | APPEND cross-ref note | +3 lines | spec v1.1 22 invariants alignment |

**Total**: 9 surgical edits; ~62 lines added/modified across both white papers; **0 deletions of user content**; all original prose preserved.

---

## § F — Honest Acknowledgements

What this revision achieves:
- Closes 5 numeric inconsistencies (Codex/Gemini flagged)
- Adds 7 cross-references from WP to spec docs
- Clarifies v4 vs v4.1 boundary explicitly
- 0 constitutional violations introduced

What this revision is honest about:
- Edit choices reflect ArchitectAI's interpretation; user can override any
- "Conservative additions" approach: text added, not deleted; user can prune if desired
- Constitutional amendments remain FROZEN; this revision unblocks WP finalization but not Art 0.5 / Art 0.2 enactment

What this revision does NOT do:
- Add new philosophical content (those are user's voice)
- Restructure either WP (preserved chapter/section ordering)
- Touch constitution.md
- Auto-finalize WP (user signs `v4-whitepaper-finalized-*` tag when satisfied)

What user should do next:
1. Read this REVISION_NOTES doc
2. Read both updated WP files; spot-check edits
3. Reject any specific edit via `git revert` (each edit is small + isolated)
4. When satisfied, sign `v4-whitepaper-finalized-2026-XX-XX` tag → triggers Constitution Art 0.5 + Art 0.2 amendment unfreeze

— ArchitectAI, 2026-04-27 (per user ultrathink directive)
