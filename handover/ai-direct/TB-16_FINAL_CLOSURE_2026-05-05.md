# TB-16 FINAL CLOSURE — Controlled Market Smoke Arena (umbrella)

**Date**: 2026-05-05
**Author**: TB-16.x.2 umbrella shipping closure (final sub-atom 2.6 commit `35a4e9b` on `main`)
**Status**: ALL 6 SUB-ATOMS SHIPPED — ready for architect sign-off

---

## §0 Why this document exists

Per architect ruling `2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md` §A.6, TB-16 closes when the umbrella charter `TB-16.x.2_charter_2026-05-04.md` finishes all 6 sub-atoms and β architectural status is honestly reported. This document IS the closure ledger. After architect sign-off, TB-17 charter writing can begin.

---

## §1 Sub-atom completion ledger

| # | Sub-atom | Surface | Class | Smoke iters | Audit | Commit | Ship gate |
|---|---|---|---|---|---|---|---|
| 0 | TB-16 main charter (Atoms 0..7 + R3 closure) | full audit_tape battery + dashboard + comprehensive_arena scaffold | 3 | n/a | dual (Codex+Gemini R1..R3 closure) | `3cd22d4` (post-R3 R2) | 8/8 SG GREEN; 13 halt-triggers; verdict=PROCEED |
| 0.x.1 | TB-16.x.1 + .1.5 (tamper R2 hang) | audit_tape_tamper hardening | 2 | 1 | self | `3f7535d + 3735484` | OBS_TB_16_TAMPER_R2_HANG resolved |
| 0.x.fix | TB-16.x.fix (OBS_R022 α closure) | LATEST_MARKOV_CAPSULE.txt deletion + audit_tape Optional pointer | 2 | 1 | dual (Codex CHALLENGE; Gemini DEGRADED) | `f2bb871` | SG-16.7..16.10 added |
| 2.1 | TaskExpire env-var trigger | evaluator.rs FORCE_EXPIRE + adapter.rs tb11_emit_expire_for_eligible | 2 | 1 (e986ed0) | self | `e986ed0` | 9-of-13 → **10-of-13** tx kinds |
| 2.2 | ChallengeResolve via challenge-window scheduler | evaluator.rs FORCE_CHALLENGE_RESOLVE + adapter.rs tb16_emit_challenge_resolve_for_eligible + sequencer.rs untouched (STEP_B-spirit worktree) | 3 | 5 (r1..r5; pre-session r1..r2 stale; this session r3..r5 clean) | dual (Codex CHALLENGE; Gemini CHALLENGE; 5 fixes; 1 .fix.r2) | `5e32cbf+3234960+647860c` | 10→**11-of-13** tx kinds; id=42 audit assertion Pass |
| 2.3 | CompleteSetRedeem env-var trigger | evaluator.rs FORCE_REDEEM + adapter.rs make_real_complete_set_redeem_signed_by | 2 | 1 | self | `6d202ee` (this session) | 11→**12-of-13** tx kinds (single-chain) |
| 2.4 | Multi-WorkTx + Boltzmann RUNTIME exercise | evaluator.rs FORCE_BOLTZMANN_SEED_WORKTXS + audit_assertions.rs id=43 | 3 | 4 (r1 false-PASS via ROOT-counted; r2 fix r1 with VETO; r3 settle barrier; r4 seed=12345 PASS) | **dual R1+R2** (Codex R1 VETO×4 + R2 ship-clean; Gemini R1 VETO + R2 VETO Q1+Q2 → OBS_R024 + TB-17 PRE-17.5) | `b5118fd + 4dd82c1 + e34d178` (this session, 3 commits) | id=43 entropy 0.918 ≥ 0.5 PASS; chain ≥3 admitted WorkTx; β-A complete |
| 2.5 | AutopsyCapsule real-bankruptcy chain | evaluator.rs FORCE_BANKRUPTCY_AFTER_ACCEPTED + audit_assertions.rs id=43 stub | 2 | 3 (r1 staker mismatch; r2 missing CAS write; r3 ship-gate fix) | self | `f1216f0` (this session) | AgentAutopsyCapsule on chain (size 334 bytes; LossReasonClass::Bankruptcy; loss_amount > 0) |
| 2.6 | Combined arena run | smoke script + multi-chain evidence | 2 | 4 chains (P14 + P14b + P15 + P15b) | self | `35a4e9b` (this session) | **multi-chain union 13-of-13** ✓; single-chain deferred to TB-17 PRE-17.6 |

**Total commits this session**: 7 (`6d202ee`, `f1216f0`, `b5118fd`, `4dd82c1`, `e34d178`, `35a4e9b`, +1 forthcoming for stage 2 docs).
**Total commits across TB-16 lifetime**: 30+ (see `git log --oneline | grep -E TB-16 | wc -l`).

---

## §2 13-of-13 architect tx kinds — UNION achievement evidence

```
P14_comprehensive (handover/evidence/tb_16_x_2_6_smoke_2026-05-05/P14_comprehensive/):
  work=6, verify=1, challenge=1, task_open=1, escrow_lock=1,
  complete_set_mint=1, market_seed=1, challenge_resolve=1
  (8/13; OMEGA-Confirm path; full FORCE_*)

P14b_omega_finalize_only (.../P14b_omega_finalize_only/):
  work=1, verify=1, task_open=1, escrow_lock=1, finalize_reward=1
  (5/13; captures finalize_reward — blocked by FORCE_CHALLENGER in P14)

P15_exhaust_redeem (.../P15_exhaust_redeem/):
  task_open=1, escrow_lock=1, complete_set_mint=1, market_seed=1,
  terminal_summary=1, task_expire=1, task_bankruptcy=1
  (7/13; redeem rejected because FORCE_EXPIRE overwrote Bankrupt → Expired)

P15b_exhaust_redeem_no_expire (.../P15b_exhaust_redeem_no_expire/):
  task_open=1, escrow_lock=1, complete_set_mint=1, complete_set_redeem=1,
  market_seed=1, terminal_summary=1, task_bankruptcy=1
  (7/13; captures complete_set_redeem)

UNION across 4 chains: 13/13 ✓  (per per-chain audit_verdict=PROCEED)
```

Forensic findings preserved (architectural-correctness, NOT bugs to fix):
1. OMEGA + FORCE_CHALLENGER blocks finalize_reward (PolicyViolation rejection — challenged WorkTx must wait for resolve, then re-emit finalize)
2. FORCE_BANKRUPTCY + FORCE_EXPIRE order (state Bankrupt → Expired overwrite at sequencer.rs:1259-1261; the two refund paths are mutually exclusive within a single market lifecycle by design)
3. Single-task evaluator architecture limit (one Lean problem per evaluator process; multi-task arena = TB-17 scope per PRE-17.6)

---

## §3 β architectural status declaration (per architect ruling §A.6)

The architect ruling §A.6 mapped β-progression to:
- TB-16.x.2.4 — multi-WorkTx attempt + Boltzmann runtime ← begin β
- TB-16.x.2.6 — combined arena run, single continuing chain ← β fully realized

**Honest β closure** (this session's net delivery):

| β component | Status | Forward trigger |
|---|---|---|
| **β-A**: multi-WorkTx + Boltzmann RUNTIME exercise | ✅ COMPLETE (commit `e34d178`) | n/a |
| **β-B**: Boltzmann sequencer-side ENFORCEMENT (vs proposal-side OBSERVE) | 🚧 NOT IMPLEMENTED — Class 4 surface | OBS_R024 + **TB-17 PRE-17.5** |
| **β-C**: single continuing chain across multi-task | 🚧 PARTIAL — multi-chain UNION only | **TB-17 PRE-17.6** |
| **β-D**: in-tape Markov inheritance pipeline | 🚧 NOT IMPLEMENTED — α CLI sidecar still | **TB-17 PRE-17.7** (NEW) |

The "TB-16.x.2.6 ← β fully realized" expectation is **PARTIALLY** met. β-A is realized; β-B/C/D are deferred to TB-17 with concrete forward triggers.

The umbrella charter Class 3 risk envelope for .2.4 + .2.6 forbade the Class 4 surface required for β-B/C/D substantive build; deferral is the constitutionally-correct outcome per `feedback_no_workarounds_strict_constitution` ("no 凑活 workaround — OBS is concrete forward-trigger, not a silencing").

---

## §4 TB-17 hard preconditions ledger (PRE-17.1..17.7)

| PRE | Source | Status | Closure evidence |
|---|---|---|---|
| **PRE-17.1** TB-16 global Markov pointer issue closed | architect §B.6 | ✅ CLOSED | TB-16.x.fix `f2bb871`; LATEST_MARKOV_CAPSULE.txt deleted |
| **PRE-17.2** run-to-run inheritance is in-tape OR explicit prior-chain-runtime-repo input | architect §B.6 | ✅ CLOSED via doc | `handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md` §2 (filed this session) |
| **PRE-17.3** no global latest pointer acts as source of truth | architect §B.6 | ✅ CLOSED | Same as 17.1 + MARKOV_INHERITANCE_POLICY §3.1 forbids reintroduction |
| **PRE-17.4** audit_tape distinguishes genesis / inherited / invalid Markov pointer | architect §B.6 | ✅ CLOSED | MARKOV_INHERITANCE_POLICY §2.1/2.2/2.3 + audit assertions id=32+33+34+35 |
| **PRE-17.5** Boltzmann sequencer ENFORCEMENT gate | OBS_R024 (this session via Gemini R2 Q1 VETO) | 🚧 OPEN | TB-17 charter MUST add WorkTx parent_tx schema bump + admission gate; closes OBSERVE→ENFORCE gap |
| **PRE-17.6** single-chain 13-of-13 via multi-task arena (`comprehensive_arena.rs` substantive build) | TB-16.x.2.6 README (this session) | 🚧 OPEN | TB-17 charter MUST build out comprehensive_arena from current scaffold to 6-task driver; closes multi-chain-union deviation |
| **PRE-17.7** in-tape Markov β-D pipeline (TerminalSummaryTx → EvidenceCapsule.markov_capsule_cid) | MARKOV_INHERITANCE_POLICY §4 (this session) | 🚧 OPEN | TB-17 charter MUST wire TerminalSummaryTx to carry markov_capsule_cid; deprecates α CLI sidecar resolver |

---

## §5 Open OBSes at TB-16 ship time

| OBS | Status | Path | Forward trigger |
|---|---|---|---|
| OBS_R022 — global LATEST Markov parallel ledger | ✅ CLOSED (Option α) | `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md` | n/a (closed by `f2bb871`) |
| OBS_R023 — evaluator.rs:2956 hardcoded RunOutcome::MaxTxExhausted | 🚧 OPEN | `handover/alignment/OBS_R022_TB_16_X_2_2_FIX_EVIDENCE_CAPSULE_HARDCODED_MAXTX_2026-05-05.md` | TB-15.x or RSP-3.2 (per .2.2.fix.r2 deferral) |
| OBS_R024 — TB-16.x.2.4 Boltzmann OBSERVE-vs-ENFORCE | 🚧 OPEN | `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md` | **TB-17 PRE-17.5** |
| OBS_TB_16_TAMPER_R2_HANG | ✅ CLOSED | `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md` | n/a (closed by `3f7535d + 3735484`) |
| OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16 | ✅ CLOSED | (per TB-16 Atom 4) | n/a |

---

## §6 Test counts (workspace-test-canonical)

```
command          = cargo test --workspace --release
workspace_count  = 922  (+7 from .2.4.fix r1 id=43 unit tests vs 915 pre-session baseline)
failed           = 0
ignored          = 150
```

---

## §7 Trust Root rehash chain (this session, evaluator.rs)

```
12489ab4 (.2.2.fix r2 baseline, session start)
  → e1c4d057 (.2.3 commit 6d202ee)
  → d39c67d1 (.2.5 commit f1216f0)
  → 5a989d15 (.2.4 commit b5118fd)
  → fada36b4 (.2.4.fix r1 commit 4dd82c1)
  → 346a6a3c (.2.4.fix r1 supplemental — preseed-settle barrier)
  → 5dfd5142 (.2.4.fix r2 commit e34d178)
```

src/runtime/adapter.rs:
```
c1360a73 (.2.2.fix.r2 baseline)
  → 48da399a (.2.3 commit 6d202ee — make_real_complete_set_redeem_signed_by helper)
```

src/runtime/audit_assertions.rs: NOT in trust root manifest.

genesis_payload.toml hash chain documented inline (each rehash carries annotation citing predecessor + rationale).

---

## §8 Hygiene #15 forensic finding (preserved)

TB-16.x.2.1 commit `e986ed0` substantive smoke is real (binary contained FORCE_EXPIRE; stderr `TaskExpire batch: count=1`; verdict.json `task_expire=1`); ONLY the ship-gate `grep '"task_expire"'` was field-name false-positive (always passes regardless of count). Per `feedback_no_retroactive_evidence_rewrite`, historical .2.1 evidence stays untouched. Forward fix: all this-session smoke scripts (.2.3, .2.5, .2.4 + r4, .2.6) use python3 JSON count guard + secondary trace witness pattern.

---

## §9 Architect sign-off pending

This document IS the comprehensive closure ledger required for architect sign-off per architect ruling §A.6 ("TB-17 — Real-World Readiness Gate (preconditions + new SGs)"). The ledger:

1. Demonstrates all 6 sub-atoms shipped + smoke-verified on real-LLM substrate.
2. Honestly declares β-A complete + β-B/C/D deferred with concrete TB-17 PRE forward triggers.
3. Maps 13-of-13 architect tx kinds to multi-chain union evidence.
4. Files MARKOV_INHERITANCE_POLICY.md as the architect-mandated artifact.
5. Maps all 7 PRE-17 hard preconditions to closure status / forward triggers.

**Ready for architect ratification. TB-17 charter writing unblocked upon sign-off.**

The user-driven autonomous-mode authorization for TB-16 (2026-05-04: "auto mode on until dual audit and real comprehensive LLM smoke test passed") was met at TB-16 main charter Atom 7 R3 closure (`ce64d61`) and re-met at this TB-16.x.2 umbrella shipping. Continuing to TB-17 requires fresh user authorization (Class 4 surface implications per PRE-17.5 + PRE-17.7).

---

## §10 Cross-references

- TB-16 main charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- TB-16.x.2 umbrella charter: `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
- TB-16.x.fix charter: `handover/tracer_bullets/TB-16.x.fix_charter_2026-05-04.md`
- Architect OBS_R022 ruling (PRE-17.1..17.4 + α/β/γ): `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
- MARKOV_INHERITANCE_POLICY: `handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md`
- LATEST.md (running state): `handover/ai-direct/LATEST.md`
- Multi-chain evidence ledger: `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md`
- Codex audits: `handover/audits/CODEX_TB_16_X_2_4_AUDIT_2026-05-05_R{1,2}.md`
- Gemini audits: `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R{1,2}.md`
- OBS_R024 (Boltzmann ENFORCE deferral): `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md`
- Project memories updated: `project_tb_16_x_fix_shipped`, `feedback_markov_inheritance_tape_derived`, `feedback_no_workarounds_strict_constitution`
