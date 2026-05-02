# TRACE_FLOWCHART_MATRIX — TB ↔ Constitution Flowchart Mapping

**Authority**: architect directive 2026-05-02, ruling 9 of Part C ("Add TRACE_FLOWCHART_MATRIX.md mapping future TBs to Flowchart 1/2/3").
- Source: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md`
- Insight summary: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`
**Status**: ratified by user authorization 2026-05-02 (option D1, "create skeleton now with TB-1..TB-7R back-fill + TB-8 forward row").
**Companion**: `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` (per-symbol map; this matrix is per-TB).

---

## §1 Why this matrix exists

The lossless constitution integrated edition 2026-05-02 elevated three flowcharts from explanatory diagrams to **SHA256-anchored architectural contracts**. Each TB charter (current + future) must declare which flowchart element(s) it touches; deviation between code behavior and the four canonical hashes is now provably auditable.

This matrix is the cross-reference. It is updated at every TB ship.

---

## §2 The four canonical flowchart hashes

```text
Flowchart 1a — Runtime loop, page 8
  rtool / input / Agent δ / output / predicates ∏p / write tool path
  SHA256: a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5

Flowchart 1b — Runtime loop continuation, page 9
  predicates branch / write tool / Q_{t+1} / map-reduce tick
  SHA256: b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d

Flowchart 2 — Boot + full architecture, page 13
  Initialization (human → InitAI → predicates / Q0 / mr) + runtime loop + Finalization
  SHA256: 6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333

Flowchart 3 — Meta-architecture, page 17
  Constitution + logs archive (read-only) → JudgeAI / ArchitectAI →
  anti-oreo runtime (top / agents / tools) → log → archive → feedback → re-init
  SHA256: c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd
```

---

## §3 TB ↔ Flowchart matrix

Legend: ✅ touched (TB closes or extends this flowchart element) · ◯ adjacent (TB references but does not modify) · — not relevant.

| TB ID | Flowchart 1<br>(runtime loop) | Flowchart 2<br>(boot) | Flowchart 3<br>(meta) | Notes |
|---|---|---|---|---|
| TB-1 | ◯ | — | — | TypedTx skeleton (pre-flowchart-anchoring) |
| TB-2 | ◯ | — | — | predicate registry skeleton |
| TB-3 | ✅ economic mutator (escrow / RSP) | — | — | EscrowLockTx + WorkTx.stake inline |
| TB-4 | ✅ challenge path | — | — | ChallengeTx + VerifyTx |
| TB-5 | ✅ predicate gate | — | — | RSP-3 challenge resolution + system-tx ingress |
| TB-6 | ✅ runtime loop closure (production binary) | ✅ Boot wire-up (Q_0 from prod binary) | — | Production ChainTape wire-up; binary now drives kernel |
| TB-7 | ✅ Frame B (proposal-level DAG) | — | — | Per-tactic deferred to TB-8+ |
| TB-7R | ✅ runtime loop FULL closure: every externalized proposal → L4 / L4.E; predicate evidence resolves from CAS; dashboard regeneratable | ✅ Boot continuity: genesis_report.json + on-chain TaskOpen / EscrowLock | — | Constitution-Aligned Frame B Repair; Class 3 dual ship audit PASS; 712 tests / 0 fail |
| TB-8 | ✅ settlement node CLOSED (every accepted L4 WorkTx with closed challenge window + no upheld challenge → exactly 1 L4 FinalizeRewardTx that atomically debits escrow + credits solver + flips claims_t.status = Finalized) | ✅ Boot continuity preserved (no new artifact; TB-7R genesis_report.json carries forward) | — | Minimal payout / FinalizeRewardTx SHIPPED 2026-05-02; Class 3 dual audit (Codex + Gemini both PASS strategic-tier; Codex round-1 VETO RQ3+RQ4 → round-2 PASS); 725 tests / 0 fail / 150 ignored (+13 net TB-8) |
| TB-9 (planned) | ✅ identity in input/output of Agent δ | ✅ persistent registry initialized at boot | — | Durable AgentRegistry + read-only WalletTool projection |
| TB-10 (planned) | ✅ task-market path through Agent δ | ✅ task lifecycle observable from boot | — | Lean Proof Task Market MVP |
| TB-11 (planned) | ✅ price-as-statistical-signal in output (not in predicates) | — | — | NodePosition + PriceIndex v0; no trading |
| TB-12 (planned) | ✅ economic mutator (CompleteSet) | — | — | CompleteSet + MarketSeedTx; CTF semantics in code |
| TB-13 (planned) | ✅ economic mutator (CPMM Router) | — | — | CPMM Router; constant-product invariant |
| TB-14 (planned) | ◯ scheduler / read-view (NOT predicate / NOT ledger) | — | — | Boltzmann masking + two-axis P_accept / P_progress |
| TB-15 (planned) | — | — | ✅ logs archive → ArchitectAI feedback → re-init | Markov Log Loom + EvidenceCapsule; first Flowchart 3 closure |
| TB-16 (planned) | ✅ all loops live | ✅ boot fully observable | ✅ EvidenceCapsule per session | Beta with market signals |
| TB-17 (planned) | ✅ trade ledger | — | — | Full market trading (post-v1.0) |

---

## §4 Validation tests by flowchart

### 4.1 Flowchart 1 (runtime loop)

For any TB that touches the runtime loop, the following invariants must hold:

```text
1. Every externalized proposal lands in L4 (accepted) or L4.E (rejected).
   - No "third place" for failed proposals.
   - No accepted node without predicate-passing evidence.

2. Predicate evidence resolves from CAS.
   - L4 or L4.E entry → CID → CAS → evidence blob → sha256 verifies.

3. Dashboard is materialized view.
   - Dashboard is regeneratable from ChainTape + CAS alone.
   - Dashboard does not have authoritative state.

4. Predicate failure does not advance Q_t.
   - q_state.ledger_root unchanged on rejected proposals.
   - L4.E append is a separate ledger slot, not a Q_t mutation.
```

Reference: TB-7R 4-clause acceptance + 7-condition ship gate (`handover/audits/RECURSIVE_AUDIT_TB_7R_2026-05-02.md`).

### 4.2 Flowchart 2 (boot)

For any TB that touches boot or Q_0:

```text
1. Boot artifact exists and is replayable.
   - genesis_report.json (or successor) lives in run evidence dir.
   - Replay reconstructs Q_0 byte-exactly from the artifact.

2. on_init is the sole legal mint point.
   - No post-init mint may appear in any TypedTx variant.
   - MarketMakerBudget (when introduced TB-12+) is allocated AT on_init,
     consumed thereafter, never refilled by future mint.

3. TaskOpen / EscrowLock are observable from L4 (post-TB-7R).
   - Memory-only preseed is forbidden as production evidence.
```

Reference: TB-7R Atom C+D commit `392a516`; `handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md`.

### 4.3 Flowchart 3 (meta)

For any TB that touches the meta loop (logs / ArchitectAI / JudgeAI / re-init):

```text
1. EvidenceCapsule produced at session end.
   - Per Markov rule: latest capsule + constitution = default context.
   - Historical logs preserved in archive but not loaded by default.

2. No raw log fragment leaks into Agent prompt.
   - Per Art. III.1 屏蔽错误 + DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN.

3. Permission changes route through ArchitectAI → JudgeAI/VetoAI → canary.
   - Even Autopsy-suggested policy patches must traverse this loop.

4. Markov override (deep-history read) only on persistent-disease problems.
   - Override requires explicit reason + signature.
```

Reference: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`; directive Part C §11 (TB-15 EvidenceCapsule structure).

---

## §5 Update protocol

```text
1. Every new TB charter MUST add a row to §3 declaring which flowchart
   element(s) it touches. Missing declaration = reject before commit
   (per feedback_tb_phase_tag_required pattern, extended to flowcharts).

2. At TB ship, validation tests in §4 are checked for the touched flowcharts.
   Failing test = ship-gate violation.

3. If a TB touches no flowchart element (rare; e.g., pure docs TB), declare
   "—" across all three columns and explain in Notes.

4. This matrix is updated in the same commit as the TB ship.
   No "I'll update it later" — the matrix is part of the ship gate.

5. Flowchart hashes are immutable. If the constitution canonical flowcharts
   change, that is a Class 4 sudo event; this matrix is then re-rebased.
```

---

## §6 Status

```text
Created:               2026-05-02
Last TB ship:          TB-7R (commits 55680bb + 46716ae + 17d69de)
Next TB:               TB-8 (Minimal Payout / FinalizeRewardTx) — charter rewritten 2026-05-02
TB-1..TB-7R back-fill: from existing TB_LOG.tsv + commit history; mappings are
                       reconstructed, not original-author-declared.
                       Original authorship of these TBs predates this matrix.
TB-8 forward row:      declared in TB-8 charter rewrite 2026-05-02.
```

Reconstruction caveat for back-filled rows: TB-1..TB-7R rows in §3 are best-effort declarations based on what each TB demonstrably touched. Original TB charters did not declare flowchart traces. Future TBs will declare directly.
