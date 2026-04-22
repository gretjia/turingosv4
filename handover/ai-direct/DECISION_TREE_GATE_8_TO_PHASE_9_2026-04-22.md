# Decision Tree — Gate 8→9 Variance Path + Phase 9 Pivot

**Date**: 2026-04-22
**Supersedes**: PLAN_FINAL § 4 Gate 8→9 (augments, doesn't replace)
**Driver**: dual-audit ITERATE verdict on 2026-04-22 Phase 2 A/B (reasoner run, now archived)

---

## § 1. Entry state

- Phase 8 code complete (experiment branch tip `910eb73` + `794b818` model switch)
- Phase 2 A/B on reasoner = strict Gate FAIL + CI crosses 0 + 2-outlier-driven → **archived** (scope-inappropriate, model deviation)
- Model default switched to **deepseek-chat** (all run_*.sh, matches pre-reg + memory + 26 historical runs)

---

## § 2. Decision tree

```
                          Phase 2.5 chat A/B N=20 (seed 74677)
                          main vs experiment, same sample
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
              [PPUT-PASS]      [INCONCLUSIVE]      [PPUT-FAIL]
              both criteria    paired CI           ΣPPUT <90%
              met              crosses 0           AND CI negative
                    │               │               │
                    ▼               ▼               ▼
              ┌──────────┐   ┌──────────────┐  ┌────────────┐
              │ MERGE    │   │  Step 2:     │  │ HOLD +     │
              │ experiment │   │  re-run      │  │ diagnose   │
              │ → main   │   │  N=20 seed 2 │  │ Phase 8    │
              │          │   │              │  │ regression │
              └────┬─────┘   └──────┬───────┘  └──────┬─────┘
                   │                │                 │
                   ▼                ▼                 ▼
           Phase 9 choice    ┌──────┴──────┐    Specific per-
           (see § 3)         │ 2 PASS /    │    problem debug
                             │ 1+1 /       │    (outliers first)
                             │ 2 FAIL      │
                             └──────┬──────┘
                                    │
                         ┌──────────┼──────────┐
                         ▼          ▼          ▼
                       2 PASS    1 PASS 1    2 FAIL
                         │       FAIL         │
                         ▼         │          ▼
                    MERGE        N=50      HOLD +
                    → Phase 9    seed 1 +  diagnose
                                 seed 2    (Bernoulli
                                 (Phase 9.A not explaining)
                                 early)
```

---

## § 3. Phase 9 branching (post-merge)

### § 3.1 Pre-pivot scope (Codex CHALLENGE-compliant)

**Do not pivot to full 9.M Market Bake-off until**:
1. Phase 2 Gate decisively PASSed (2 seeds on chat, same sample)
2. M4/M7/M8 have concrete Rust spec文档 + dual audit PASS
3. Paper-1 thesis framed as *hypothesis*, not *finding*

**Initial Phase 9 runs = Phase 9.A baseline (as originally pre-registered)**:
- 6 seeds × N=50 dual-mode + step-only
- Uses current Phase-8 binary (no new mechanism)
- Establishes chat-native PPUT baseline for next 2 papers
- Budget: ~$360, ~18-24h wall

### § 3.2 Concurrent M1 incremental test (only Codex-approved mechanism)

**9.M.1 only**: dynamic founder grant γ = f(tape_depth, past_yield)
- M1 is "incremental mechanism change" per Codex (existing Phase 3A Hayek infra)
- A/B: 2 × N=20 × chat, Phase-8 baseline vs Phase-8 + M1
- Budget: ~$30
- Acceptance: ΣPPUT on depth≥10 > 0 with M1 enabled; **does not need to beat baseline**
- If M1 A/B shows any signal → proceed 9.M.2 design for M4/M7/M8

### § 3.3 9.M.2/3/4 specs (deferred until 9.M.1 signal + spec dual-audit PASS)

M4 / M7 / M8 each need a pre-registration附加文档 before implementation:
- **M4 Satoshi citation rebate** spec: citation metric, settlement formula, rebate recipient selection, conservation proof
- **M7 append staking** spec: escrow lifecycle, refund trigger, slashing semantics, interaction with Law 2
- **M8 bonding curve LP** spec: curve form, fee policy, LP role registration, agent-LP interaction

Each spec → dual audit → if PASS → added to 9.M pipeline.

---

## § 4. Gate criteria (formally updated)

### § 4.1 Phase 2.5 chat A/B (Gate 8→9 retry)

**PASS (Gate passes, merge unblocked)**:
- Paired ΔPPUT CI does NOT fully lie below -0.05 (absolute PPUT units)
- AND one of:
  - ΣPPUT_exp ≥ 0.90 × ΣPPUT_main
  - Paired match-solve-count: exp ≥ main − 1 (1-solve tolerance)

**INCONCLUSIVE**:
- Paired ΔPPUT CI crosses 0 AND ΣPPUT gap > 10%
- → trigger Step 2 re-run seed

**FAIL**:
- Paired ΔPPUT CI lies entirely below -0.10 (absolute)
- OR ΣPPUT gap > 25% (severe regression)
- → HOLD + diagnose

### § 4.2 Phase 9.A baseline (Gate 9→10)

Already pre-registered in `REGISTRATION_PHASE_9_2026-04-22.md` § 4. No change.

### § 4.3 9.M.1 (M1 only)

**PASS (warrants scaling to 9.M.2+)**:
- M1 branch ΣPPUT on depth≥10 > 0.0 AND Σdepth≥10 > Phase-8 baseline's Σdepth≥10 - 0.3 (tolerance)
- AND paired ΔPPUT not severely negative (CI doesn't lie below -0.15)

**INCONCLUSIVE**: → larger N or different γ formula

**FAIL**: → M1 ineffective; reconsider mechanism framing

---

## § 5. Paper 1 thesis framing revision

**Per Codex Q2.3 over-claim verdict**:

### Old (over-claim)
> Constitutional topology enables Hayekian market mechanisms that drive emergent faster proof discovery.

### New (hypothesis-framed)
> We present constitutional topology (Phase 8) as a **testable substrate** for Hayek-style market mechanisms. We empirically test N mechanisms (Phase 9.M) and report observed effects on PPUT. Mechanism M shows +X% PPUT in our setup; we do not claim this generalizes beyond the tested mechanism/seed/benchmark.

Key changes:
- "drives emergent" → "we empirically test"
- "faster proof discovery" → "effects on PPUT"
- Added falsifiable scope + generalization disclaimer

---

## § 6. Artifact tree (post this decision)

```
handover/ai-direct/
├── DECISION_TREE_GATE_8_TO_PHASE_9_2026-04-22.md    ← this file
├── PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md         ← update § 4 Gate 8→9
├── REGISTRATION_PHASE_9_2026-04-22.md                ← append § 9.M sub-registration
├── DECISIONS_2026-04-22.md                           ← append decision 6,7,8
├── AUTO_RESEARCH_NOTEPAD.md                          ← F-2026-04-22-06 (ITERATE + over-claim lesson)
├── PAPER_1_OUTLINE_2026-04-22.md                     ← soften thesis
├── M1_DYNAMIC_GAMMA_SPEC_2026-04-22.md               ← new (Phase 9.M.1 spec)
├── M4_SATOSHI_REBATE_SPEC_2026-04-22.md              ← new (pre-reg sub-doc)
├── M7_APPEND_STAKING_SPEC_2026-04-22.md              ← new (pre-reg sub-doc)
└── M8_BONDING_CURVE_LP_SPEC_2026-04-22.md            ← new (pre-reg sub-doc)
```

---

## § 7. Immediate action queue (post this doc)

1. **Phase 2.5 chat A/B** — seed 74677, chat, sample_N20_S74677
2. Update 5 existing docs (PLAN_FINAL / REGISTRATION / DECISIONS / NOTEPAD / PAPER_OUTLINE)
3. Write 4 mechanism specs (M1 / M4 / M7 / M8) — enables future dual audit
4. Watch A/B results → fork per § 2 tree

---

## § 8. Rollback criteria

**If Phase 2.5 chat A/B FAILS entirely AND Phase 8 debug finds a real regression bug**:
1. `git revert` on the offending commit(s) in experiment branch
2. Preserve R1-α Ed25519 (VETO fix must stay)
3. Preserve constitutional infra (C-049/050/053/055/061/067 fixes — these were BLOCKERs)
4. Isolate the regressing change → re-audit → re-fix
5. Paper 1 timeline slips by 1-2 weeks

**If dual-audit on mechanism spec VETOes the pivot**:
1. Stay on 9.A baseline (original pre-reg)
2. Paper 1 thesis remains "constitutional substrate" not "market mechanism"
3. Paper 2 (zeta_sum_proof generalization) proceeds as planned
4. Market mechanism research deferred to Paper 3 (omegav4 PCP)
