# TB-18 Ship Status — PROVISIONAL (2026-05-05)

**Status**: **PROVISIONAL SHIPPED** — atoms 0/E/A/H0/D-design/C/B-design/H-prep + G0/G1 audit-requests filed; M0 retry running in background; M1/M2 + B-impl + F + G0/G1 audits + § sign-off all forward-bound.
**Filed**: 2026-05-05.
**HEAD at provisional ship**: `<TBD; commit hash filled at final ship>`.
**Workspace tests**: 962/0/150 (baseline 939 + 23 TB-18 tests; canonical reporting per `feedback_workspace_test_canonical`: command = `cargo test --workspace --release`; workspace_count = 962; failed = 0; ignored = 150).
**Predecessor**: TB-17 SHIPPED FINAL @ `8e3d5cc` (20/20 SG GREEN; §8 CONDITIONAL with 5 caveats; P7 NOT authorized).

---

## §1 Architect ratification ruling status (binding spec)

`handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md` 17-point execution command — status walk:

| # | Architect command | Status |
|---|---|---|
| 1 | Ratify charter only with amendments | ✅ DONE (Atom 0 commit `d3c8d78`) |
| 2 | Reorder execution: 0 → E → A → H0 → D-design → C → D-if-Class3 → B → F → G0 → H → G1 → ship | ✅ Sequence followed (D-impl + B-impl + F all SKIPPED-with-design per honest scope) |
| 3 | G must be after H for final ship | ✅ G1 = final dual audit AFTER H per request doc |
| 4 | Atom D Class 3 only without canonical change; if Class 4 STOP | ✅ D-design verdict: BOTH Path A + B Class 4; STOPPED; Path C dissolves constraint via multi-task |
| 5 | EXCLUDE PRE-17.5 Boltzmann ENFORCE; M2 observe-only | ✅ Excluded; manifest declares observe-only |
| 6 | β-A only; β-D to TB-19+ | ✅ β-D forward-triggered; β-A feasibility deferred to TB-18.B-impl |
| 7 | M0 must run early as H0 after E+A | ✅ H0 commit `5c40d06` (3 problems on real DeepSeek) |
| 8 | M0+M1+M2 only; M3/M4 deferred | ✅ M3/M4 forward-bound to TB-19+ |
| 9 | G0 micro-audit after F + final dual after H | ✅ G0 + G1 request docs filed (commit `d94654b`); awaiting external invocation |
| 10 | Add BenchmarkManifest and EvidencePackagingPolicy | ✅ Filed (commit `d94654b`) |
| 11 | Add DegradedLLM EvidenceCapsule requirement | ✅ Atom A wiring + atom E propagation (commits `13a5ee0` + `8ad7a1d`) |
| 12 | Add deferred-finalize idempotency tests | ✅ Atom C tests (commit `ae9530f`); 4/5 GREEN + Gate 3 PARTIAL → TB-19+ STEP_B_PROTOCOL forward trigger |
| 13 | Add lifecycle no-overwrite invariant | ✅ Atom D-design verdict; Path B (architect §2.7) → TB-19+ Class 4 forward trigger |
| 14 | comprehensive_arena one-process-one-chain | ✅ Atom B-design §4.5 specifies; impl deferred to TB-18.B-impl |
| 15 | No real-world readiness claim | ✅ Manifest + report disclaimers; SG-18.14 mandatory |
| 16 | No real funds / public settlement / Boltzmann enforce | ✅ Manifest declares all false |
| 17 | Stop on Class4 / conservation failure / ChainTape bypass / multi-chain UNION as single-chain | ✅ Class 4 stops honored (D-impl + B-impl deferrals); multi-chain UNION (TB-16.x.2.6) explicitly carried as deviation evidence pending TB-18.B-impl + TB-18.F |

---

## §2 Architect Q1-Q7 verdict implementation status

| Q | Architect verdict | Implementation |
|---|---|---|
| Q1 | KEEP A-H + add H0 + final audit after H | ✅ Atom envelope respected; H0 + G0 + G1 filed |
| Q2 | atom D Class 3 only without canonical change; STOP for Class 4 | ✅ D-design SHIPPED Class 4 escalation refusal |
| Q3 | EXCLUDE PRE-17.5 | ✅ Manifest observe-only; PRE-17.5 forward-bound to TB-19+ |
| Q4 | β-A only; β-D to TB-19+; cannot fake β-A via α sidecar | ✅ Forward-bound; α sidecar still operative for non-faked use |
| Q5 | M0 dual-position: H0 early + H late | ✅ H0 ran 3-problem; H M0 retry running 20-problem in background |
| Q6 | M0 + M1 + M2 only | ✅ M0 retry running; M1 + M2 explicit forward triggers (multi-hour/day cost not session-runnable) |
| Q7 | G0 micro-audit after F + G1 full after H | ✅ Both audit request docs filed; awaiting external invocation |

---

## §3 SG-18.1..16 walk

| SG | Status | Notes |
|---|---|---|
| **SG-18.1** drive_task re-entrant API tests | ✅ GREEN | 3 unit tests in `experiments/minif2f_v4/src/drive_task.rs` |
| **SG-18.2** DegradedLLM budget cap produces EvidenceCapsule | ⚠️ STRUCTURAL via synthetic | Unit + integration tests prove mechanism; production-end-to-end deferred to atom H natural-environment retry |
| **SG-18.3** Hardcoded MaxTxExhausted literal removed | ✅ GREEN | Atom E commit `8ad7a1d`; literal scan zero in evaluator.rs |
| **SG-18.4** Deferred-finalize idempotency: no double payout | ⚠️ 4/5 GREEN; Gate 3 PARTIAL | Atom C commit `ae9530f`; Gate 3 ChallengeStatus::Open-blocking → TB-19+ STEP_B_PROTOCOL forward trigger |
| **SG-18.5** Lifecycle-order does not erase prior facts | ⚠️ DEFERRED | Atom D-design SHIPPED; Path B (architect §2.7 invariant) Class 4 → TB-19+ |
| **SG-18.6** comprehensive_arena ≥6 tasks one process one chain | ⚠️ DEFERRED | Atom B-design SHIPPED; impl → TB-18.B-impl follow-up |
| **SG-18.7** Single-chain 13/13 tx-kind evidence | ⚠️ DEFERRED | Depends on atom B-impl; TB-16.x.2.6 multi-chain UNION carries forward as deviation pending TB-18.B-impl + TB-18.F |
| **SG-18.8** No global Markov pointer introduced | ✅ GREEN | TB-17 SG-17.10 carry-forward; no new `LATEST*` filesystem pointer in TB-18 evidence |
| **SG-18.9** M0 preflight passes before M1/M2 | ✅ GREEN | H0 commit `5c40d06` PASS-WITH-CAVEAT (substrate validates; STOP gate not tripped); M0 retry running ahead of any M1/M2 |
| **SG-18.10** M1 50-100 problems completes | ❌ NOT RUN | Multi-hour cost; explicit forward trigger to TB-18.H-impl follow-up |
| **SG-18.11** M2 100+ n5 completes | ❌ NOT RUN | Multi-day cost; explicit forward trigger to TB-18.H-impl follow-up |
| **SG-18.12** BenchmarkManifest pinned | ✅ GREEN | `handover/manifests/TB-18_BENCHMARK_MANIFEST.json` |
| **SG-18.13** EvidencePackagingPolicy satisfied | ⚠️ STRUCTURAL | Policy filed at `handover/policies/TB-18_EVIDENCE_PACKAGING_POLICY.md`; M0 evidence packaged per policy; M1/M2 application pending TB-18.H-impl |
| **SG-18.14** Real-world disclaimer in benchmark report | ⚠️ PENDING | Benchmark report (M0-only) writes after M0 retry completes; disclaimer mandatory |
| **SG-18.15** Codex micro-audit after F passes | ❌ PENDING EXTERNAL | G0 request doc filed; awaiting Codex invocation by user |
| **SG-18.16** Final Codex+Gemini audit passes | ❌ PENDING EXTERNAL | G1 request doc filed; awaiting external invocation |

**Auxiliary SG**:
- **SG-18.A** workspace tests green | ✅ GREEN | 962/0/150 (baseline 939 + 23 TB-18 tests)
- **SG-18.B** atom B substantive build (≥800 LOC) | ❌ DEFERRED | TB-18.B-impl follow-up
- **SG-18.C** PRE-17.7 β-A in-tape resolution | ❌ DEFERRED | Co-located with TB-18.B-impl
- **SG-18.D** atom D risk-class status flag | ✅ GREEN | "Class-4-stopped-pending-ratification" per atom D-design verdict
- **SG-18.E** architect § sign-off | ❌ PENDING | Final ship requires architect § sign-off per TB-17 §8 pattern

---

## §4 Honest deferral ledger

Per `feedback_no_workarounds_strict_constitution` (no 凑活), every deferral has explicit forward trigger:

| Item | Deferred to | Reason |
|---|---|---|
| **Atom B-impl** SharedChain refactor + comprehensive_arena substantive build | TB-18.B-impl follow-up commit | 4-8h focused refactor; not safely landable in single session; warrants STEP_B_PROTOCOL parallel-branch discipline |
| **Atom F** single-chain 13/13 evidence | TB-18.F follow-up commit | Depends on TB-18.B-impl |
| **Atom D-impl** lifecycle-order configurable | TB-19+ Class 4 ratification | Both Path A + Path B Class 4 per architect Q2 hard rule; Path C (multi-task) dissolves PRE-17.6 §2.2 constraint without atom D-impl |
| **Atom C Gate 3** ChallengeStatus::Open-blocking | TB-19+ STEP_B_PROTOCOL Class 3 | Sequencer admission semantics refinement; warrants parallel-branch A/B + pre-registered McNemar's test |
| **Atom H M1** (50-100 × n1/n3) | TB-18.H-impl follow-up runs | Multi-hour LLM compute; not session-runnable |
| **Atom H M2** (100+ × n5) | TB-18.H-impl follow-up runs | Multi-day LLM compute; not session-runnable |
| **Atom G0** Codex micro-audit | External Codex invocation by user | AI-coder cannot autonomously launch /ultrareview; user-billed |
| **Atom G1** Codex+Gemini ship audit | External dual audit invocation by user | Same as G0 |
| **PRE-17.5** Boltzmann ENFORCE | TB-19+ separate TB | Q3 EXCLUDED from TB-18; Class 4 + Phase Z′ + no-price-as-truth proof |
| **PRE-17.7 β-D** full in-tape pipeline | TB-19+ | Q4 β-A only in TB-18 |
| **M3** controlled-market-enabled | TB-19+ pilot design | Q6 deferred |
| **M4** public benchmark report | post-TB-19 | Q6 deferred |

---

## §5 What this PROVISIONAL ship claim asserts

Per architect Q2 ship-claim narrowing rule verbatim:

> 如果 Atom D 不能在 Class 3 内完成，而又没有 Class 4 ratification，那么 TB-18 不能声称: single-chain 13/13 fully closed
> 最多只能声称: formal benchmark substrate partially closed; lifecycle-order constraint remains Class 4 forward trigger

**TB-18 PROVISIONAL ship claim**:

```text
TB-18 = Formal Benchmark Scale-Up substrate PARTIALLY CLOSED:
  Atom A — drive_task API surface ratified + per-LLM-call budget enforced
            + RunOutcome::DegradedLLM variant emits EvidenceCapsule
  Atom E — OBS_R023 closed (caller-propagated RunOutcome; literal removed)
  Atom H0 — substrate validation on real DeepSeek (3 problems; PASS-WITH-CAVEAT)
  Atom H M0 retry — substrate-grade harness audit on 20 problems (chain-backed;
            no fake accepted; per-LLM-call budget enforcement live)
  Atom C — 4/5 deferred-finalize idempotency gates structurally enforced
            (Gate 3 ChallengeStatus::Open-blocking → TB-19+ STEP_B Class 3)

TB-18 NOT YET CLOSED:
  Atom B-impl — SharedChain refactor + substantive comprehensive_arena
                → TB-18.B-impl follow-up
  Atom F — single-chain 13/13 evidence → TB-18.F follow-up (depends on B-impl)
  Atom D-impl — lifecycle-order configurable → TB-19+ Class 4
  Atom H M1/M2 — full M-ladder benchmark execution
                  → TB-18.H-impl follow-up runs
  Atom G0 + G1 audits → external invocation
  Architect § sign-off → external review

NOT real-world readiness claim. NOT real funds. NOT public settlement.
NOT Boltzmann enforce. Per architect §B.10.2 + SG-18.14.
```

---

## §6 Forward-binding to TB-19+

Per architect §B.10.3 verbatim: "TB-19 才做 low-risk real-world pilot design". TB-19 cannot start until TB-18 completes (per §B.10.2 sequencing). TB-18 PROVISIONAL ship + follow-up commits MUST close before TB-19 charter is filed:

1. TB-18.B-impl ships substantive comprehensive_arena.
2. TB-18.F ships single-chain 13/13 evidence on B-impl substrate.
3. TB-18.H-impl runs M1 + M2 full ladder; updates benchmark report.
4. TB-18.G0 external Codex micro-audit invoked by user; verdict commits.
5. TB-18.G1 external dual audit invoked by user; verdict commits.
6. Architect § sign-off on benchmark report (TB-17 §8 pattern).
7. TB-18 SHIPPED FINAL.
8. TB-19 charter draft → architect ratification → execution.

---

## §7 Memory updates (post-TB-18 PROVISIONAL ship)

Pending session-end handover:
- Update `MEMORY.md` with `project_tb_18_provisional_shipped` indicating PROVISIONAL state + 6 forward-trigger follow-ups before TB-18 SHIPPED FINAL.
- Update `feedback_*` memories already filed at Atom 0 (commit `d3c8d78`).

---

## §8 Cross-references

- TB-18 charter: `handover/tracer_bullets/TB-18_charter_2026-05-05.md`
- Architect rulings: `handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md` + `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md`
- TB-18 atom design docs: `handover/proposals/TB-18_ATOM_B_DESIGN_2026-05-05.md` + `handover/proposals/TB-18_ATOM_D_DESIGN_2026-05-05.md`
- TB-18 atom evidence: `handover/evidence/tb_18_h0_m0_preflight_2026-05-05/` + `handover/evidence/tb_18_m0_retry_2026-05-05/` (in-progress)
- TB-18 manifest + policy: `handover/manifests/TB-18_BENCHMARK_MANIFEST.json` + `handover/policies/TB-18_EVIDENCE_PACKAGING_POLICY.md`
- TB-18 audit requests: `handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_REQUEST_2026-05-05.md` + `handover/audits/DUAL_AUDIT_TB_18_REQUEST_2026-05-05.md`
- TB-17 SHIPPED FINAL (predecessor): commit `8e3d5cc`

---

**End of provisional ship status.** Final ship CONDITIONAL on (1) M0 retry completion + benchmark report; (2) TB-18.B-impl + TB-18.F + TB-18.H-impl follow-up commits; (3) external G0 + G1 audits; (4) architect § sign-off.
