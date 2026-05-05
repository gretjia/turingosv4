# Architect audit request — TB-16 closure + smoke / real-LLM evidence + TB-17 charter ratification

**From**: Claude (AI-coder)
**Date**: 2026-05-05
**Authority requested**: external architect audit before TB-17 atom 1 (REAL_WORLD_READINESS_REPORT.md stub) begins.
**Scope**:
1. Ratify or VETO TB-16 + TB-16.x.1 + TB-16.x.fix + TB-16.x.2 (umbrella + 6 sub-atoms .2.1 / .2.2 / .2.3 / .2.4 / .2.5 / .2.6) shipping.
2. Ratify or VETO the smoke + real-LLM test evidence as canonical "Controlled Market Smoke Arena" verdict.
3. Ratify, amend, or reject the TB-17 charter (`handover/tracer_bullets/TB-17_charter_2026-05-05.md`) — answer the 5 questions in §6 below.

**Reading-order recommendation**: §1 (this doc executive summary) → §2 (PRE-17 closure ledger) → `handover/ai-direct/TB-16_FINAL_CLOSURE_2026-05-05.md` (durable closure ledger; same data more verbose) → §3 (audit-evidence inventory) → §4 (smoke / real-LLM evidence inventory) → §5 (honest gaps) → §6 (architect questions).

---

## §1 Executive summary (what's shipped, what's open)

### TB-16 main charter — Controlled Market Smoke Arena (P6 Permissioned ChainTape sandbox)

- **Status**: SHIPPED at commit `3cd22d4` (post-R3 R2; 8/8 SG GREEN; 13/13 halt-triggers; verdict=PROCEED).
- **Class envelope**: 3 (architect §7 verbatim — sandbox controlled market).
- **Audit**: dual external (Codex + Gemini) — R1 BOTH VETO → R2 Codex VETO + Gemini partial → R3 surgical fixes (`90848bb`, `ce64d61`) → post-R3 Round 2 (`3cd22d4`) PROCEED.
- **Outcome**: full audit-from-tape battery (38 assertions × 8 layers); audit_tape + audit_tape_tamper binaries; dashboard §15 live regen + §16 SANDBOX banner; comprehensive_arena scaffold (NOT substantive — flagged for TB-17 PRE-17.6).

### TB-16.x.* — 6 follow-on sub-atoms (closure of pointer issue + arena coverage gaps)

| Sub-atom | Subject | Class | Audit | Commit | Outcome |
|---|---|---|---|---|---|
| **TB-16.x.1 + .1.5** | tamper-hang root-cause (`OBS_TB_16_TAMPER_R2_HANG`) + architect §3 anti-drift verification | 2 | self | `3f7535d` + `3735484` | OBS closed; `audit_tape_tamper` hardened |
| **TB-16.x.fix** | OBS_R022 Option α closure — delete `LATEST_MARKOV_CAPSULE.txt`, de-canonicalize global pointer | 2 | dual (Codex CHALLENGE; Gemini DEGRADED) | `f2bb871` | SG-16.7..16.10 added; PRE-17.1+17.3 closed |
| **TB-16.x.2** | umbrella charter for the .2.x sub-atoms | 0 | self | `8f199bb` | charter only |
| **TB-16.x.2.1** | TaskExpire env-var trigger + strict-alignment patch | 2 | self | `e986ed0` + `fab2977` | 9-of-13 → 10-of-13 tx kinds |
| **TB-16.x.2.2** + .fix + .fix.r2 | ChallengeResolve via challenge-window scheduler | 3 | dual (5 CHALLENGE closures + 1 .fix.r2 round) | `5e32cbf` + `3234960` + `647860c` | 10→11-of-13 tx kinds; id=42 audit assertion Pass |
| **TB-16.x.2.3** | CompleteSetRedeem env-var trigger | 2 | self | `6d202ee` | 11→12-of-13 tx kinds (single chain) |
| **TB-16.x.2.4** + .fix r1 + .fix r2 | Multi-WorkTx + Boltzmann RUNTIME exercise | 3 | **dual R1+R2** (Codex R1 VETO×4 → R2 ship-clean; Gemini R1 VETO + R2 VETO Q1+Q2) | `b5118fd` + `4dd82c1` + `e34d178` | id=43 entropy 0.918 ≥ 0.5 PASS; β-A complete; β-B (sequencer ENFORCE) deferred via OBS_R024 → TB-17 PRE-17.5 |
| **TB-16.x.2.5** | AutopsyCapsule real-bankruptcy chain | 2 | self | `f1216f0` | AgentAutopsyCapsule on chain (size 334 bytes; LossReasonClass::Bankruptcy) |
| **TB-16.x.2.6** | Combined arena run | 2 | self | `35a4e9b` | **multi-chain union 13-of-13** ✓; single-chain deferred → TB-17 PRE-17.6 |
| Closure docs | FINAL_CLOSURE.md + MARKOV_INHERITANCE_POLICY.md + TB_LOG.tsv backfill + LATEST.md | 0 | self | `bb1eb48` + `7faf911` + `d431ac2` | TB-16 ledger closed |

### Net verdict (Claude's pre-audit position)

- **TB-16 main + TB-16.x.fix + TB-16.x.2.[1..6] are SHIPPED** at constitutionally-correct envelopes.
- **β-A** (multi-WorkTx + Boltzmann runtime) realized. **β-B / β-C / β-D** explicitly deferred via OBS_R024 + PRE-17.6 + PRE-17.7 forward triggers — **NOT silenced** per `feedback_no_workarounds_strict_constitution`.
- **PRE-17.1..17.4 CLOSED**; **PRE-17.5..17.7 OPEN as TB-17 deliverables**.
- **0 unresolved VETOs**; **2 OBSes open with explicit forward triggers** (`OBS_R023` deferred to TB-15.x/RSP-3.2; `OBS_R024` deferred to TB-17 PRE-17.5).

**This prompt asks the architect**: confirm or VETO this verdict; then ratify/amend/reject the TB-17 charter.

---

## §2 PRE-17 hard-precondition closure ledger

(Lifted verbatim from `handover/ai-direct/TB-16_FINAL_CLOSURE_2026-05-05.md` §4; referenced by TB-17 charter §1.1.)

| PRE | Source | Status | Closure evidence |
|---|---|---|---|
| **PRE-17.1** TB-16 global Markov pointer issue closed | architect ruling §B.6 | ✅ CLOSED | TB-16.x.fix commit `f2bb871`; `LATEST_MARKOV_CAPSULE.txt` deleted; SG-16.7..16.10 added |
| **PRE-17.2** Run-to-run inheritance is in-tape OR explicit prior-chain-runtime-repo input | architect §B.6 | ✅ CLOSED via doc | `handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md` §2 (B.α + B.β documented) |
| **PRE-17.3** No global latest pointer acts as source of truth | architect §B.6 | ✅ CLOSED | Same as PRE-17.1 + MARKOV_INHERITANCE_POLICY §3.1 forbids reintroduction |
| **PRE-17.4** audit_tape distinguishes genesis / inherited / invalid Markov pointer | architect §B.6 | ✅ CLOSED | MARKOV_INHERITANCE_POLICY §2.1/§2.2/§2.3 + audit assertions id=32+33+34+35 |
| **PRE-17.5** Boltzmann sequencer ENFORCEMENT gate | OBS_R024 (TB-16.x.2.4 R2 Gemini VETO Q1) | 🚧 OPEN | TB-17 atom 7 design-then-ratification (Class 4) |
| **PRE-17.6** Single-chain 13-of-13 via multi-task arena | TB-16.x.2.6 README | 🚧 OPEN | TB-17 atom 8 (Class 3 dual audit) |
| **PRE-17.7** In-tape Markov β-D pipeline (TerminalSummaryTx → EvidenceCapsule.markov_capsule_cid) | MARKOV_INHERITANCE_POLICY §4 | 🚧 OPEN | TB-17 atom 9 (Class 3 or 4 by design) |

---

## §3 Audit-evidence inventory (dual-external + recursive)

### Dual external audits (Codex + Gemini)

| Audit | Path | Verdict |
|---|---|---|
| Codex TB-16 ship audit R1 | `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md` | VETO (closed via R3 surgical fixes) |
| Codex TB-16 ship audit R3 | `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R3.md` | PROCEED |
| Gemini TB-16 ship audit R1 | `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R1.md` | VETO |
| Gemini TB-16 ship audit R2 | `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R2.md` | VETO Q2 + 5 CHALLENGEs |
| Gemini TB-16 ship audit R3 | `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R3.md` | PROCEED (post-`ce64d61`) |
| Codex TB-16.x.2.4 audit R1 | `handover/audits/CODEX_TB_16_X_2_4_AUDIT_2026-05-05_R1.md` | VETO×4 + CHALLENGE×4 |
| Codex TB-16.x.2.4 audit R2 | `handover/audits/CODEX_TB_16_X_2_4_AUDIT_2026-05-05_R2.md` | CHALLENGE / ship-clean |
| Gemini TB-16.x.2.4 audit R1 | `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R1.md` | VETO Q1+Q2+Q5+Q7+Q8+Q12 |
| Gemini TB-16.x.2.4 audit R2 | `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R2.md` | **VETO Q1+Q2** → OBS_R024 + TB-17 PRE-17.5 |

### Recursive self-audits

| Audit | Path |
|---|---|
| Recursive TB-16 R1 | `handover/audits/RECURSIVE_AUDIT_TB_16_2026-05-04.md` |
| Recursive TB-16 R2 | `handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md` |
| Recursive TB-16 R3 | `handover/audits/RECURSIVE_AUDIT_TB_16_R3_2026-05-04.md` |

### Audit runner scripts (for reproducibility)

- `handover/audits/run_codex_tb_16_ship_audit.sh` (R1 + R3 variant)
- `handover/audits/run_gemini_tb_16_ship_audit.py` (R1) + `..._r3.py` + `run_gemini_tb_16_x_2_4_audit.py`

### OBS docs filed during TB-16

| OBS | Status | Path |
|---|---|---|
| OBS_R022 — global LATEST Markov parallel ledger | ✅ CLOSED (Option α `f2bb871`) | `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md` |
| OBS_R022 (TB-16 R3 trace-matrix text-extension; namespace re-use) | tracking only | `handover/alignment/OBS_R022_TB_16_R3_TRACE_MATRIX_TEXT_EXTENSION_2026-05-04.md` |
| OBS_R022 (TB-16.x.2.2.fix evidence-capsule hardcoded MaxTx — internal name OBS_R023) | 🚧 OPEN | `handover/alignment/OBS_R022_TB_16_X_2_2_FIX_EVIDENCE_CAPSULE_HARDCODED_MAXTX_2026-05-05.md` |
| OBS_R022 (TB-16 tamper-backlinks) | tracking only | `handover/alignment/OBS_R022_TB16_TAMPER_BACKLINKS_2026-05-04.md` |
| OBS_R024 — Boltzmann OBSERVE-vs-ENFORCE | 🚧 OPEN → TB-17 PRE-17.5 | `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md` |
| OBS_TB_16_TAMPER_R2_HANG | ✅ CLOSED | `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md` |
| OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16 | ✅ CLOSED | `handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md` |

---

## §4 Smoke + real-LLM test-result inventory

### Real-LLM arena evidence (`handover/evidence/tb_16_real_llm_arena_2026-05-04/`)

| Sub-dir | Purpose | Verdict / artifact |
|---|---|---|
| `arena_run/verdict.json` | Step 1 surgical-fix smoke | (see file) |
| `arena_run2/verdict.json` | iterative re-run | (see file) |
| `arena_run3/verdict.json` | iterative re-run | (see file) |
| `arena_run4/verdict.json` | **canonical Step 4 — happy-path 7 tx kinds** (FirstLong + Challenge + MarketSeed + CompleteSetMint admitted) | (see file) |
| `arena_run5_exhaust/verdict.json` | exhaust-path smoke | (see file) |
| `arena_run6_exhaust/verdict.json` | **canonical exhaust — 4 tx kinds incl. TaskBankruptcy** | (see file) |
| `audit_pipeline_smoke/verdict.json` | end-to-end pipeline smoke (TB-13 fixture chain) | `verdict=BLOCK passed=31 failed=0 halted=1 skipped=7` (real evidence-gap detection) |
| `post_r3_full_test/` | **VINTAGE / pre-runner-fix; DO NOT cite** | per dir README |
| `post_r3_round2/` | canonical R3 evidence (8 chains × N=5 × MAX_TX=20) | per dir SUMMARY.md |
| `post_r3_smoke/verdict.json` | quick smoke gate | (see file) |
| `README.md` | arena top-level overview | summary across all sub-runs |
| `ARENA_PLAN.md` | Atom 7 R1 Step 4 arena plan | (see file) |

**Architectural caveat** (declared in `arena_run4` README): only **9 of 13** architect-required tx kinds delivered across `arena_run4` + `arena_run6_exhaust` in the original TB-16 main charter window. Missing 4 (ChallengeResolve, FinalizeReward, TaskExpire, CompleteSetRedeem) closed under TB-16.x.2.[1..3] sub-atoms (10/11/12 of 13) — final 13/13 achieved as **multi-chain UNION** under TB-16.x.2.6.

### TB-16.x.* sub-atom smoke evidence

| Path | Sub-atom | Tx-kind delta |
|---|---|---|
| `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/` | TaskExpire env-var trigger | 9 → 10 |
| `handover/evidence/tb_16_x_2_2_smoke_2026-05-05/` | ChallengeResolve via challenge-window scheduler (canonical r5) | 10 → 11 |
| `handover/evidence/tb_16_x_2_2_smoke_2026-05-05_r5_VERIFICATION_DO_NOT_SHIP/` | r5 verification artifact | (do-not-ship label) |
| `handover/evidence/tb_16_x_2_2_smoke_2026-05-05_stale_r1_DO_NOT_SHIP/` | stale r1 | (do-not-ship label) |
| `handover/evidence/tb_16_x_2_2_smoke_2026-05-05_stale_r2_DO_NOT_SHIP/` | stale r2 | (do-not-ship label) |
| `handover/evidence/tb_16_x_2_2_smoke_2026-05-05_stale_r3_DO_NOT_SHIP/` | stale r3 | (do-not-ship label) |
| `handover/evidence/tb_16_x_2_3_smoke_2026-05-05/` | CompleteSetRedeem env-var trigger | 11 → 12 (single chain) |
| `handover/evidence/tb_16_x_2_4_smoke_2026-05-05/P12_boltzmann_runtime/` | Boltzmann RUNTIME r1 (false-PASS) | (audit-rejected) |
| `handover/evidence/tb_16_x_2_4_smoke_2026-05-05/r2_after_dual_audit_fixes/` | r2 after R1 fixes | (audit-rejected) |
| `handover/evidence/tb_16_x_2_4_smoke_2026-05-05/r3_after_preseed_settle_barrier/` | r3 settle barrier | (audit-rejected) |
| `handover/evidence/tb_16_x_2_4_smoke_2026-05-05/r4_seed_12345/P12_boltzmann_runtime/` | **canonical r4 — id=43 entropy 0.918 ≥ 0.5 PASS** | β-A complete |
| `handover/evidence/tb_16_x_2_5_smoke_2026-05-05/P13_autopsy_real/` | r1 staker-mismatch | (rejected) |
| `handover/evidence/tb_16_x_2_5_smoke_2026-05-05/r2_after_agent_user_0_fix/` | r2 missing CAS write | (rejected) |
| `handover/evidence/tb_16_x_2_5_smoke_2026-05-05/r3_after_proposal_telemetry_cas_write/` | **canonical r3 — AgentAutopsyCapsule on chain** | (size 334 bytes; LossReasonClass::Bankruptcy; loss_amount > 0) |
| `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/` | **combined arena run — 4-chain UNION 13-of-13** | β-C partial (multi-chain UNION; not single-chain) |
| `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/P14_comprehensive/` | OMEGA + full FORCE_* (8/13 tx kinds) | per chain README |
| `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/P14b_omega_finalize_only/` | OMEGA without FORCE_CHALLENGER (5/13) | captures finalize_reward |
| `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/P15_exhaust_redeem/` | MaxTxExhaust + FORCE_EXPIRE (7/13) | captures task_expire + task_bankruptcy |
| `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/P15b_exhaust_redeem_no_expire/` | MaxTxExhaust no FORCE_EXPIRE (7/13) | captures complete_set_redeem |

### Pre-TB-16 smoke evidence (cross-referenced; TB-13/TB-14 fixtures used as audit_tape probes)

| Path | Vintage |
|---|---|
| `handover/evidence/tb_6_chaintape_smoke_2026-05-01/` | TB-6 production ChainTape bootstrap |
| `handover/evidence/tb_7_chaintape_smoke_2026-05-01/` | TB-7 (FrameB initial) |
| `handover/evidence/tb_13_chaintape_smoke_2026-05-03/` | TB-13 CompleteSet smoke |
| `handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/` | TB-13 real-LLM solve (used as TB-16 audit_pipeline_smoke fixture) |
| `handover/evidence/tb_14_chaintape_smoke_2026-05-03/` | TB-14 PriceIndex smoke |

### Closure / handover docs

| Path | Content |
|---|---|
| `handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md` | TB-16 main charter ship-gate ledger |
| `handover/ai-direct/TB-16_FINAL_CLOSURE_2026-05-05.md` | TB-16 + TB-16.x.* umbrella closure (durable; same data more verbose than this prompt) |
| `handover/ai-direct/LATEST.md` | running session state (TB-16.x.2 umbrella final closure section) |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` | research-notepad (TB-16 backfill in `7faf911`) |
| `handover/tracer_bullets/TB_LOG.tsv` | tracer-bullet log (TB-16 backfill in `7faf911`) |
| `handover/tracer_bullets/TB-16_charter_2026-05-04.md` | TB-16 main charter |
| `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md` | TB-16.x.1 charter |
| `handover/tracer_bullets/TB-16.x.fix_charter_2026-05-04.md` | TB-16.x.fix charter (OBS_R022 α closure) |
| `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` | TB-16.x.2 umbrella charter |
| `handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md` | filed 2026-05-05; SG-17.9 + SG-17.10 mandate |
| `handover/markov_capsules/MARKOV_TB-15_*.json` + `MARKOV_TB-16_*.json` | per-run Markov capsules (historical artifacts only post-OBS_R022) |

### Test counts (workspace-test-canonical per `feedback_workspace_test_canonical`)

```
command          = cargo test --workspace --release
workspace_count  = 922
failed           = 0
ignored          = 150
baseline_pre_TB-16   = 907 (post-TB-15 ship)
delta            = +15  (TB-16 + TB-16.x.* additions)
```

---

## §5 Honest gaps (Claude's pre-audit declaration of partial deliveries)

These are the items that I (Claude) would have flagged in self-audit had I been my own external reviewer. Listed here so the architect doesn't have to dig:

### G1 — β-B / β-C / β-D incomplete

Per architect ruling §A.6, the umbrella expectation was "TB-16.x.2.6 ← β fully realized". Honest delivery:

- ✅ **β-A** (multi-WorkTx + Boltzmann RUNTIME exercise) — COMPLETE.
- 🚧 **β-B** (Boltzmann sequencer-side ENFORCEMENT) — NOT IMPLEMENTED. Reason: Class 4 surface (WorkTx schema bump + canonical signing-payload bump + sequencer admission gate); umbrella envelope was Class 3. **Forward trigger**: TB-17 PRE-17.5 (charter atom 7).
- 🚧 **β-C** (single continuing chain across multi-task) — PARTIAL. Multi-chain UNION 13-of-13 ships; single-chain requires `comprehensive_arena.rs` substantive build. **Forward trigger**: TB-17 PRE-17.6 (charter atom 8).
- 🚧 **β-D** (in-tape Markov inheritance pipeline) — NOT IMPLEMENTED. α CLI sidecar still in use; β `TerminalSummaryTx → EvidenceCapsule.markov_capsule_cid` pipeline not wired. **Forward trigger**: TB-17 PRE-17.7 (charter atom 9).

Three architectural-correctness constraints surfaced during TB-16.x.2.6 prevent single-chain 13-of-13 in the current single-task evaluator:
1. OMEGA + FORCE_CHALLENGER blocks `finalize_reward` (PolicyViolation rejection — challenged WorkTx must wait for resolve, then re-emit finalize).
2. FORCE_BANKRUPTCY + FORCE_EXPIRE order causes state Bankrupt → Expired overwrite (sequencer.rs:1259-1261); two refund paths mutually exclusive within single market lifecycle.
3. Single-task evaluator architecture limit (one Lean problem per evaluator process).

### G2 — Open OBSes at TB-16 ship time

| OBS | Status | Path | Forward trigger |
|---|---|---|---|
| `OBS_R022_TB_16_X_2_2_FIX_EVIDENCE_CAPSULE_HARDCODED_MAXTX` (internal: OBS_R023) | 🚧 OPEN | `handover/alignment/OBS_R022_TB_16_X_2_2_FIX_EVIDENCE_CAPSULE_HARDCODED_MAXTX_2026-05-05.md` | TB-15.x or RSP-3.2 (per .2.2.fix.r2 deferral) — **Q4 below** asks if this is acceptable |
| `OBS_R024 — Boltzmann OBSERVE-vs-ENFORCE` | 🚧 OPEN | `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md` | TB-17 PRE-17.5 |

### G3 — Sub-atom audit asymmetry

Sub-atoms .2.1, .2.3, .2.5, .2.6 were Class 2 self-audit; .2.2 + .2.4 were Class 3 dual external. The umbrella charter pre-declared this risk-class-by-sub-atom strategy. **Q3 below** asks if the architect concurs.

### G4 — Multi-chain union ≠ single-chain

The architect spec (and `feedback_o1_chain_on_auditability` + Art. 0.4 path B chain continuation) intends single-chain inheritance. TB-16.x.2.6 ships **multi-chain UNION** with explicit forward trigger to TB-17 PRE-17.6. **Q5 below** asks if the architect accepts this deviation as constitutionally-correct given the Class 4 surface required for the alternative.

### G5 — The audit_pipeline_smoke verdict=BLOCK

`handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json` reports `verdict=BLOCK passed=31 failed=0 halted=1 skipped=7`. This is **correct detection** — the TB-13 fixture chain has a real evidence gap (Layer E #27: `evidence_capsule_cid not in CAS at L4 index 2`). This is not a TB-16 bug; it's the audit_tape battery doing its job. Forward-fixed in the TB-16.x.2.5 chain (autopsy real-bankruptcy chain emits complete TerminalSummary + EvidenceCapsule pair).

---

## §6 Questions for the architect

### Q1 — TB-16 closure ratification

Do you ratify the TB-16 + TB-16.x.* umbrella closure as constitutionally-correct, given:
- Class 3 main charter dual-external audit PROCEED
- Class 3 sub-atom .2.2 + .2.4 dual-external audit closure (with .2.4 R2 Gemini Q1 → OBS_R024 forward trigger)
- Class 2 sub-atom .2.1 + .2.3 + .2.5 + .2.6 self-audit
- 13/13 architect tx kinds via multi-chain UNION (not single-chain)
- β-A complete; β-B / β-C / β-D explicitly forward-triggered to TB-17 PRE-17.5/.6/.7

**Verdict shape**: `RATIFY` / `RATIFY-WITH-AMENDMENT (specify)` / `VETO (specify which sub-atom)`.

### Q2 — Smoke + real-LLM evidence ratification

Do you accept the smoke + real-LLM evidence as canonical "Controlled Market Smoke Arena" deliverable, given:
- `arena_run4` + `arena_run6_exhaust` cover happy-path + exhaust-path
- 4 sub-chain UNION (P14 + P14b + P15 + P15b) covers 13/13 tx kinds
- `audit_pipeline_smoke/verdict.json` correctly detects a real evidence gap in the TB-13 fixture chain (BLOCK is correct verdict, not a regression)
- All canonical chains ship `audit_tape verdict=PROCEED` AND `replay byte-identical` AND `tamper 3/3 detected`
- `post_r3_full_test/` is explicitly labeled VINTAGE / DO NOT CITE; canonical R3 evidence is `post_r3_round2/`

**Verdict shape**: `ACCEPT` / `ACCEPT-WITH-CAVEAT (specify)` / `REJECT (specify which run)`.

### Q3 — Sub-atom risk-class asymmetry

The umbrella pre-declared Class 2 self-audit for .2.1 / .2.3 / .2.5 / .2.6 and Class 3 dual-external for .2.2 + .2.4. The reasoning was (a) env-var triggers + adapter-only changes = additive Class 1-2; (b) ChallengeResolve scheduler + Boltzmann runtime = Class 3 sequencer-adjacent.

Do you concur with this risk-class taxonomy across the .2.x sub-atoms, or do you require any retroactive Class 3 dual-external audit on .2.1 / .2.3 / .2.5 / .2.6?

**Verdict shape**: `CONCUR` / `REQUIRE RETRO DUAL ON (specify sub-atoms)`.

### Q4 — Open OBSes acceptable for TB-16 ship?

Two OBSes remain OPEN at TB-16 ship:
- **OBS_R023 (TB-16.x.2.2.fix evidence-capsule hardcoded MaxTx)**: deferred to TB-15.x or RSP-3.2. Concrete: `experiments/minif2f_v4/src/bin/evaluator.rs:2956` hardcodes `RunOutcome::MaxTxExhausted` for the EvidenceCapsule emit on exhaust path. This is correct for TB-16 sandbox semantics (every exhaust IS MaxTxExhausted) but should be parameterized when WallClockCap / ComputeCapViolated paths land.
- **OBS_R024 (Boltzmann OBSERVE-vs-ENFORCE)**: deferred to TB-17 PRE-17.5.

Are both acceptable as open-with-forward-trigger at TB-16 ship, or does either require closure before TB-17 atom 1 starts?

**Verdict shape**: `BOTH ACCEPTABLE` / `MUST CLOSE OBS_R023 BEFORE TB-17` / `MUST CLOSE OBS_R024 BEFORE TB-17` / `CLOSE BOTH FIRST`.

### Q5 — TB-17 charter ratification (5 sub-questions)

The TB-17 charter is at `handover/tracer_bullets/TB-17_charter_2026-05-05.md`. Per charter §9, Claude requests explicit authorization on:

5a. **Charter scope**: do atoms 0..12 + filing convention (`handover/whitepapers/` for the 6 docs) + 7-doc bundle (architect §8.2 + OBS_R022 §A.5) correctly realize the architect §8 + OBS_R022 §A.5 + PRE-17.5/.6/.7 forward-trigger mandate?

5b. **Atom 7 (PRE-17.5) Class 4 envelope**: confirm Class 4 (constitution-sudo + canonical signing-payload bump + Phase Z′ flowchart-rerun). Confirm AI-coder authorization to draft the PRE-17.5 design doc (`handover/proposals/TB-17_PRE_17_5_BOLTZMANN_ENFORCE_DESIGN_2026-05-05.md`) without immediate implementation. **Hard gate**: AI-coder MUST NOT begin schema bump until you ratify (a) `WorkTx` field bump, (b) `WorkSigningPayload` bump, (c) Phase Z′ rerun authorization. Confirm this gating posture is correct?

5c. **Atom 8 (PRE-17.6) fallback (option b)**: pre-authorize the architectural-exclusion deviation if multi-task ordering inside `comprehensive_arena.rs` cannot achieve single-chain 13-of-13 without a Class 4 sequencer change? (i.e., if the 3 architectural-correctness constraints from TB-16.x.2.6 force multi-chain-union even after substantive comprehensive_arena build.) Or do you require Claude to STOP and re-charter via TB-18 in that case?

5d. **Atom 9 (PRE-17.7) risk-class branch**: confirm AI-coder may produce the design doc first; class envelope determined at design-step; escalate to architect ratification only IF signing-payload bump required (Class 4); proceed under Class 3 dual audit otherwise?

5e. **SHIP commit timing (atom 12)**: confirm the "provisional commit before architect sign-off" pattern (per `project_tb_15_shipped` precedent) applies to TB-17, where (i) Claude commits the SHIPPED tag with §1 verdict + §8-blank, then (ii) architect signs §8 in a follow-up commit? Or do you require architect sign-off BEFORE the SHIPPED commit (no provisional)?

**Verdict shape per sub-question**: `RATIFY` / `AMEND (specify)` / `VETO (specify reason)`.

### Q6 — Anything Claude missed

Is there any TB-16 / TB-16.x.* deliverable, OBS, audit finding, smoke evidence, or architectural concern that Claude has NOT surfaced in §1-§5 above, that you want addressed before TB-17 atom 1 starts?

**Open answer.**

---

## §7 What Claude needs from this audit

After your verdicts on Q1-Q6, Claude will:

- If Q1 = RATIFY + Q2 = ACCEPT + Q3 = CONCUR + Q4 = BOTH ACCEPTABLE: TB-16 closure is **finalized**; proceed to TB-17 atom 1 (REAL_WORLD_READINESS_REPORT.md stub) under Class 0 envelope.
- If Q5a = RATIFY: TB-17 charter is binding; atoms 1-6 + 11 proceed under Class 0/1 self-audit; atoms 7 + 9 (Class 4 candidates) gate on subsequent ratification per Q5b/Q5d.
- If Q5b = RATIFY: atom 7 design doc drafted; Claude STOPS and files separate ratification request before any code change.
- If Q5c = RATIFY: atom 8 has explicit fallback authorization; multi-chain UNION ship with documented deviation is acceptable if option (a) infeasible.
- If Q5d = RATIFY: atom 9 design doc drafted first; class branch determined empirically.
- If Q5e = RATIFY: atom 12 commits the provisional SHIPPED tag; architect sign-off appended in follow-up commit.

If any verdict is VETO or AMEND: Claude re-charters per your instruction; no atom 1 work begins until the conflict is resolved.

---

## §8 Cross-references for architect deeper reading

- **TB-17 charter (request to ratify)**: `handover/tracer_bullets/TB-17_charter_2026-05-05.md` (661 lines)
- **TB-16 final closure (durable ledger)**: `handover/ai-direct/TB-16_FINAL_CLOSURE_2026-05-05.md`
- **Architect TB-13→TB-17 directive (verbatim)**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- **Architect OBS_R022 ruling (verbatim)**: `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
- **MARKOV_INHERITANCE_POLICY**: `handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md`
- **Constitution**: `constitution.md` (Art. 0.2 lines 52-95; Art. 0.4 lines 114-152; Art. II.1; Art. III.1-4; Art. IV; Art. V.1)
- **Roadmap**: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
- **TB-15 ship status (template precedent)**: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`

---

**End of audit request.**

Awaiting architect verdicts on Q1-Q6.
