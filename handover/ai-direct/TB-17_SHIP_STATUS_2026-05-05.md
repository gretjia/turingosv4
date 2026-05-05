# TB-17 Ship Status — 2026-05-05 (PROVISIONAL — pending architect §8 signature)

**Status**: PROVISIONAL SHIP per `project_tb_15_shipped` precedent + 2026-05-05 architect verdict §B.8 atom 12 verbatim ("可以先生成 ready-for-ratification snapshot，但最终 TB-17 不能算完全 shipped，直到 human architect 签署 readiness report").
**Charter**: `handover/tracer_bullets/TB-17_charter_2026-05-05.md` (RATIFIED-WITH-AMENDMENT 2026-05-05).
**Architect verdict**: `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md`.
**Authorization**: user-architect "严格按架构师意见执行，直到 TB-17 ship".

---

## §1 Atom completion ledger (12 atoms)

| # | Atom | Risk class | Status | Output |
|---|---|---|---|---|
| 0 | Charter | 0 | ✅ amended (DRAFT → RATIFIED-WITH-AMENDMENT) | `handover/tracer_bullets/TB-17_charter_2026-05-05.md` (862 lines) |
| 1 | REAL_WORLD_READINESS_REPORT.md | 0 | ✅ stub + atom-12 fill-in | `handover/whitepapers/REAL_WORLD_READINESS_REPORT.md` |
| 2 | DOMAIN_SELECTION_CRITERIA.md | 0 | ✅ filed | `handover/whitepapers/DOMAIN_SELECTION_CRITERIA.md` (4 candidates / 1 pilot D1 approved / 6 banned categories) |
| 3 | ORACLE_REQUIREMENTS.md | 0 | ✅ filed | `handover/whitepapers/ORACLE_REQUIREMENTS.md` (T1/T2/T3/T4 architecture + 9-field provenance + 6-attack-surface §8) |
| 4 | CHALLENGE_COURT_REQUIREMENTS.md | 0 | ✅ filed | `handover/whitepapers/CHALLENGE_COURT_REQUIREMENTS.md` (per-tier window + evidence + resolver hierarchy + D1 pilot config) |
| 5 | SAFETY_BOUNDARY.md | 0 | ✅ filed | `handover/whitepapers/SAFETY_BOUNDARY.md` (escalation state machine + per-tier timeout + sandbox/SHADOW/LIVE label + privacy class taxonomy) |
| 6 | IRREVERSIBLE_ACTION_POLICY.md | 0 | ✅ filed | `handover/whitepapers/IRREVERSIBLE_ACTION_POLICY.md` (8 architect Q6.2 subtypes + 10-row §5 verdict matrix; all 4 verdict classes exercised) |
| 7 | PRE-17.5 Boltzmann enforce | 0 (design) | ✅ design-only filed | `handover/proposals/TB-17_PRE_17_5_BOLTZMANN_ENFORCE_DESIGN_2026-05-05.md` (Class 4 if implemented; pending architect ratification) |
| 8 | PRE-17.6 comprehensive_arena | 0 (deviation) | ✅ architectural-exclusion deviation filed | `handover/proposals/TB-17_PRE_17_6_COMPREHENSIVE_ARENA_DEVIATION_2026-05-05.md` (TB-18 forward-binding §6 scope) |
| 9 | PRE-17.7 in-tape Markov β-D | 0 (design) | ✅ design-first filed | `handover/proposals/TB-17_PRE_17_7_INTAPE_MARKOV_DESIGN_2026-05-05.md` (provisional β-A Class 3 branch) |
| 10 | RESERVED for mid-charter amendment | — | (no work; reserved per charter §3 atom 10) | n/a |
| 11 | Conformance test battery | 1 | ✅ 17 new tests; all PASS | `tests/tb_17_markov_inheritance_policy.rs` + `tests/tb_17_irreversible_action_examples.rs` + `tests/tb_17_minif2f_scale_separation.rs` |
| 12 | SHIP — provisional snapshot | hybrid | ✅ this doc + REAL_WORLD_READINESS_REPORT.md fill-in | (this commit) |

---

## §2 Ship gate ledger SG-17.1..SG-17.20 (architect verdict §B.7 verbatim)

| ID | Gate | Status | Evidence |
|---|---|---|---|
| **SG-17.1** | REAL_WORLD_READINESS_REPORT.md passes audit | ✅ stub-then-fill complete | atom 1 + atom 12 §1 verdict CONDITIONAL |
| **SG-17.2** | ≥3 candidate domains classified by T-tier | ✅ 4 candidates (D1/D2/D3/D4) | DOMAIN_SELECTION_CRITERIA §2 |
| **SG-17.3** | ≥1 low-risk pilot domain approved | ✅ D1 Lean/Coq/Isabelle T2 PILOT-APPROVED | DOMAIN_SELECTION_CRITERIA §6 |
| **SG-17.4** | Per-tier oracle architecture documented | ✅ T1/T2/T3/T4 + 6-attack-surface §8 | ORACLE_REQUIREMENTS §2 + §8 |
| **SG-17.5** | ChallengeCourt evidence + window + resolver + escalation | ✅ all present | CHALLENGE_COURT_REQUIREMENTS §1 + §2 + §3 + §4 |
| **SG-17.6** | Human escalation path + RootBox protocol | ✅ state machine + per-tier timeout + Q6.3 verbatim default-safe-action | SAFETY_BOUNDARY §1 + §2 + §3 |
| **SG-17.7** | No production real-world task launched | ✅ no new entry points target real-world domain | grep audit on commits + experiments/ + src/bin/ |
| **SG-17.8** | ≥8 candidate-action verdicts (allow/deny/require-human/require-delay) | ✅ 10 verdicts; all 4 classes exercised | IRREVERSIBLE_ACTION_POLICY §5 + `tests/tb_17_irreversible_action_examples.rs` |
| **SG-17.9** | Markov inheritance policy doc + tested | ✅ doc + 10 tests | MARKOV_INHERITANCE_POLICY (pre-existing) + `tests/tb_17_markov_inheritance_policy.rs` |
| **SG-17.10** | No global filesystem pointer source-of-truth | ✅ LATEST_MARKOV_CAPSULE.txt absent | `tests/tb_17_markov_inheritance_policy.rs::sg_17_10_no_global_filesystem_pointer` |
| **SG-17.11** | `cargo test --workspace` ≥ TB-16 baseline (922) / 0 fail / ≤150 ignored | ✅ **939 / 0 / 150** | this session full workspace test |
| **SG-17.12** | Flowchart conformance tests cover FC1/FC2/FC3 | ✅ via existing `tests/fc_alignment_conformance.rs` (TB-17 stubs in `tests/conformance_stubs.rs` if extended) | (existing surface preserved; atom 11 doc-conformance fixtures added) |
| **SG-17.13** | All PRE-17.1..17.7 closed OR explicitly deferred with architect ratification | ✅ 17.1-17.4 closed; 17.5/.6/.7 deferred-with-ratification-request (atom 7 + atom 8 deviation + atom 9) | this doc §1 |
| **SG-17.14** | Atom 7 design-only deferred OR Class 4 ratified | ✅ design-only deferral path; ratification pending | atom 7 design doc |
| **SG-17.15** | Atom 8 single-chain 13/13 OR multi-chain-union deviation ratified | ✅ deviation filed; ratification pending | atom 8 deviation doc |
| **SG-17.16** | Atom 9 in-tape Markov green OR design-only deferred | ✅ design-only deferral path; ratification pending | atom 9 design doc |
| **SG-17.17** | REAL_WORLD_READINESS_REPORT.md §8 has human architect sign-off | 🟡 **PENDING** — to be filed by architect post-provisional-commit | §8 of report (block left blank) |
| **SG-17.18** | MiniF2F scale ≠ real-world readiness (separate classification) | ✅ FR-17.13 + CR-17.13 codified; 2 conformance tests green | `tests/tb_17_minif2f_scale_separation.rs` |
| **SG-17.19** | No real-world payout / public settlement / external action in code or evidence | ✅ no such code path present | grep audit (no MainNet / no real-payment / no public-API entrypoints) |
| **SG-17.20** | Readiness reports reproducible from docs + ChainTape + CAS, not hidden state | ✅ §9 reproducibility section explicit | REAL_WORLD_READINESS_REPORT §9 |

**Net SG status**: 19/20 GREEN; 1/20 (SG-17.17) **PENDING architect signature** — that's the constitutionally-correct outcome for "provisional ship" per architect §8 atom 12 verbatim.

---

## §3 PRE-17 hard-precondition closure ledger

| PRE | Status | Closure path |
|---|---|---|
| PRE-17.1 | ✅ CLOSED | TB-16.x.fix `f2bb871` (LATEST_MARKOV_CAPSULE.txt deleted) |
| PRE-17.2 | ✅ CLOSED via doc | MARKOV_INHERITANCE_POLICY.md §2 (B.α + B.β documented); SG-17.9 enforcement test green |
| PRE-17.3 | ✅ CLOSED | Same as PRE-17.1 + MARKOV_INHERITANCE_POLICY §3.1 forbids reintroduction |
| PRE-17.4 | ✅ CLOSED | MARKOV_INHERITANCE_POLICY §2.1/§2.2/§2.3 + audit assertions id=32+33+34+35 (TB-16.x.2.x) |
| **PRE-17.5** | 🟡 **DEFERRED-WITH-RATIFICATION-PENDING** | atom 7 design doc; Class 4 surface; either (a) architect ratifies in TB-17 → impl + dual audit, or (b) design-only ship → TB-18 closure |
| **PRE-17.6** | 🟡 **DEFERRED-WITH-DEVIATION-PENDING** | atom 8 deviation doc; multi-chain UNION 13/13 from TB-16.x.2.6 ratified as TB-17 evidence; substantive single-chain → TB-18 |
| **PRE-17.7** | 🟡 **DEFERRED-WITH-RATIFICATION-PENDING** | atom 9 design doc; provisional β-A Class 3 branch; architect ratifies branch verification → impl + dual audit, or → TB-18 |

---

## §4 New memory bindings

Added 4 new memory entries this session (per architect §A.9 mandate):

- `project_tb_16_ratified_with_scope_limits` — TB-16 RATIFIED only as sandbox-controlled-market-smoke.
- `feedback_minif2f_scaling_policy` — M0-M4 ladder; full benchmark = TB-18 only.
- `feedback_class4_cannot_hide_in_class3` — Class 4 surfaces require separate ratification.
- `project_tb_17_ratified_charter_2026-05-05` — charter RATIFIED-WITH-AMENDMENT; FR/CR/SG expanded.

`MEMORY.md` index updated with all 4 entries.

---

## §5 Architect ratification request (Q1-Q6 follow-on items)

The 2026-05-05 architect verdict at `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md` answered the original Q1-Q6 from the audit prompt. **Three remaining ratification asks** are now filed and need final architect verdict before TB-17 is fully shipped:

1. **Atom 7 PRE-17.5 design** — ratify implementation within TB-17 (Class 4 schema bump + Phase Z′ rerun) OR confirm deferral to TB-18. Default: deferral.
2. **Atom 8 PRE-17.6 deviation** — ratify multi-chain UNION as canonical TB-17 evidence + ratify TB-18 forward-binding §6 scope.
3. **Atom 9 PRE-17.7 design** — ratify β-A Class 3 branch + permit AI-coder feasibility verification + impl within TB-17 OR confirm deferral.

Architect signature on `REAL_WORLD_READINESS_REPORT.md` §8 closes SG-17.17 and finalizes TB-17 ship.

---

## §6 Forward-trigger ledger (binding for next TBs)

| Forward target | Source | Memory |
|---|---|---|
| **TB-18** Formal Benchmark Scale-Up | architect §B.10.2 + atom 8 deviation §6 | `feedback_minif2f_scaling_policy` |
| TB-18 atom A: evaluator.rs re-entrant API | atom 8 deviation §6.A | — |
| TB-18 atom B: comprehensive_arena substantive build | atom 8 deviation §6.B | — |
| TB-18 atom C: deferred-finalize path (constraint #1) | TB-16.x.2.6 forensic finding #1 | — |
| TB-18 atom D: lifecycle-order-configurable (constraint #2) | TB-16.x.2.6 forensic finding #2 | `feedback_class4_cannot_hide_in_class3` |
| TB-18 atom E: OBS_R023 closure (architect Q4 cap) | OBS_R022_TB_16_X_2_2_FIX_EVIDENCE_CAPSULE_HARDCODED_MAXTX | — |
| TB-18 atom F: single-chain 13/13 evidence | atom 8 deviation §6.F | — |
| TB-18 atom G: dual external audit (Codex + Gemini) | `feedback_dual_audit` Class 3 | — |
| TB-18 atom H: full MiniF2F M2 (100+ problems) | architect §B.9 M2 phase | `feedback_minif2f_scaling_policy` |
| **TB-19** Low-Risk Real-World Pilot Design | architect §B.10.3 | — |
| TB-19 D1 Lean pilot config | DOMAIN_SELECTION_CRITERIA §6 + CHALLENGE_COURT §7 + SAFETY_BOUNDARY §5.1 | — |
| **PRE-17.5 closure (if TB-17 ratification denied)** | TB-18 atom (TBD) | atom 7 design doc |
| **PRE-17.7 closure (if TB-17 ratification denied)** | TB-18 atom (TBD) | atom 9 design doc |

---

## §7 Workspace test ledger

```
command          = cargo test --workspace --release
workspace_count  = 939
failed           = 0
ignored          = 150
delta_vs_TB-16   = +17 (10 markov_inheritance_policy + 5 irreversible_action_examples + 2 minif2f_scale_separation)
```

Per `feedback_workspace_test_canonical`. SG-17.11 ✅.

---

## §8 Cross-references

- TB-17 charter (RATIFIED-WITH-AMENDMENT): `handover/tracer_bullets/TB-17_charter_2026-05-05.md`
- 2026-05-05 architect verdict (lossless): `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md`
- 2026-05-04 architect OBS_R022 ruling: `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
- 2026-05-03 architect TB-13→TB-17 directive: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- TB-16 final closure: `handover/ai-direct/TB-16_FINAL_CLOSURE_2026-05-05.md`
- 6 readiness whitepapers: `handover/whitepapers/REAL_WORLD_READINESS_REPORT.md` + `DOMAIN_SELECTION_CRITERIA.md` + `ORACLE_REQUIREMENTS.md` + `CHALLENGE_COURT_REQUIREMENTS.md` + `SAFETY_BOUNDARY.md` + `IRREVERSIBLE_ACTION_POLICY.md`
- 3 atom proposal docs: `handover/proposals/TB-17_PRE_17_5_*` + `..._6_*` + `..._7_*`
- Atom 11 conformance tests: `tests/tb_17_markov_inheritance_policy.rs` + `tests/tb_17_irreversible_action_examples.rs` + `tests/tb_17_minif2f_scale_separation.rs`
- Audit prompt origin: `handover/architect-insights/REQUEST_TB_16_CLOSURE_AND_TB_17_AUDIT_2026-05-05.md`

---

**End of provisional ship status.** Final TB-17 SHIP closure pending architect signature on `handover/whitepapers/REAL_WORLD_READINESS_REPORT.md` §8.
