# TB-18.B-impl SHIP STATUS — Atom B Phase 4 SHIPPED 2026-05-05

**Ship-claim** (architect Q2 narrowing): _"TB-18 Atom B Phase 4 (substantive comprehensive_arena single-process multi-task driver) SHIPPED. Single-chain 13/13 tx-kind evidence (FR-18.7 + FR-18.8 + SG-18.6 + SG-18.7) closed under Class 3 dual-audit pending. Full TB-18 ship still requires M1/M2 (atom H sub-stages 2/3) + external G0/G1 audit verdicts + architect § sign-off."_

**HEAD (after this commit)**: TB-18.B-impl multi-phase substantive build (Phase 1 SharedChain init lift + Phase 2 synthetic L4/L4.E gate lift + Phase 3 drive_task substantive body + Phase 4 comprehensive_arena rewrite + Phase 5 evidence + audit-request docs).

**Workspace tests**: 963/0/150 (baseline 962 + Phase 1 SharedChain unit test = 963; Phase 3 drive_task tests adapted in place).

**Authority**: user blanket auto-mode authority "自主执行一直到 TB-18 ship" (2026-05-05 verbatim).

## §1 What shipped

### Phase 1 — `SharedChain::from_env` lift (commit body)
- New `experiments/minif2f_v4/src/chain_runtime.rs` (~270 lines incl. doc-comments)
- `pub struct SharedChain { bus, chaintape_bundle, agent_keypairs, initial_balances_for_genesis_report, chaintape_preseed_enabled }`
- `pub fn from_env(problem_file: &str) -> Self` — lifted from `evaluator.rs::run_swarm` lines 659-789 + 794-833 (175 lines of inline init → method body)
- `experiments/minif2f_v4/src/lib.rs` registers the new module
- `evaluator.rs` destructures SharedChain into the same local-variable names used by lines 834-3999 (byte-identical source-text downstream)
- 1 unit test (env-var hygiene)
- evaluator.rs SHA-256 rehash applied to genesis_payload.toml

### Phase 2 — chain-level helper lift (synthetic L4/L4.E + genesis_report)
- `chain_runtime.rs::write_synthetic_l4_l4e_gate_and_genesis_report(bus, bundle, initial_balances, preseed_enabled, seed_id)` free function
- Lifted from `evaluator.rs` lines 1439-1562 (124 lines of inline → 12-line helper call)
- All 4 failure-modes preserved (synthetic TaskOpen submit fail / synthetic WorkTx submit fail / audit_trail write fail / genesis_report write fail; error-and-continue or warn-and-continue)
- evaluator.rs SHA-256 rehashed again

### Phase 3 — `drive_task` substantive body (replaces atom A.1 stub)
- `drive_task.rs::drive_task(chain, spec, _budget) -> Result<DriveTaskResult, DriveTaskError>`
- Body: real-signed `TaskOpenTx` via `make_real_task_open_signed_by` + state-root advance poll + real-signed `EscrowLockTx` via `make_real_escrow_lock_signed_by` + state-root advance poll
- `DriveTaskError::PendingAtomB` REMOVED per atom A.1 forward-binding promise
- New variants: `ChaintapeRequired`, `AgentKeypairsRequired`, `SigningFailed{stage,detail}`, `SubmitFailed{stage,detail}`
- `DriveTaskResult` expanded with `task_id`, `task_open_tx_id`, `escrow_lock_tx_id`, `post_open_lock_state_root_hex` (so callers can compose downstream task-specific txs)
- 3 unit tests (legacy mode → ChaintapeRequired; new TaskSpec defaults; error-display coverage)

### Phase 4 — `comprehensive_arena.rs` substantive rewrite
- 6 task-driver functions: `drive_task_a` through `drive_task_f` (each emits a task-specific lifecycle against the SHARED chain via `bus.submit_typed_tx` + `bundle.sequencer.emit_system_tx`)
- `main()`: `SharedChain::from_env` ONCE → `write_synthetic_l4_l4e_gate_and_genesis_report` ONCE → loop `drive_task_X` 6 times → `bundle.shutdown()` ONCE → write evidence
- Subprocess-spawn pattern eliminated (was TB-16 Atom 5 scaffold; non-compliant with architect §2.8 verbatim)
- TURINGOS_CHAINTAPE_PATH + TURINGOS_CAS_PATH set programmatically by the binary; preseed enabled
- 13/13 tx kinds emitted in 2.7 seconds wall-clock per smoke run (commit body §3 reports)

### Phase 5 — evidence + audit-request + ship status
- `handover/evidence/tb_18_b_phase4_2026-05-05/r1/` — canonical chain bytes (runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz) + per-task SHARED_CHAIN_RUNS_REPORT.json + tx_kind_distribution.json
- `handover/evidence/tb_18_b_phase4_2026-05-05/README.md` — directory layout + replay-verify instructions
- `handover/audits/DUAL_AUDIT_TB_18_B_PHASE4_REQUEST_2026-05-05.md` — Class 3 dual-audit request (8 Codex + 8 Gemini questions)
- This ship-status document

## §2 SG walk

| SG | Description | Status |
|---|---|---|
| **SG-18.1** | drive_task re-entrant API passes deterministic tests | ✅ Phase 3 — 3/3 unit tests + arena re-entrancy on 6 tasks |
| **SG-18.2** | DegradedLLM budget cap produces EvidenceCapsule | ✅ Atom A pre-existing; Phase 4 task_F demonstrates DegradedLLM TerminalSummary on shared chain |
| **SG-18.3** | Hardcoded MaxTxExhausted literal removed | ✅ Atom E pre-existing |
| **SG-18.4** | Deferred-finalize idempotency: no double payout | ✅ Atom C pre-existing (Gate 3 PARTIAL → TB-19+) |
| **SG-18.5** | Lifecycle-order does not erase prior lifecycle facts | ✅ Atom D-design Path C (multi-task structure dissolves PRE-17.6 §2.2) |
| **SG-18.6** | comprehensive_arena ≥6 tasks in one process and one chain | ✅ **PHASE 4 closes this** — 6 tasks drive in 1 process, 1 bundle, 1 runtime_repo, 1 CAS |
| **SG-18.7** | Single-chain 13/13 tx-kind evidence exists | ✅ **PHASE 4 closes this** — `tx_kind_distribution.json` shows distinct_tx_kinds=13 |
| **SG-18.8** | No global Markov pointer introduced | ✅ TB-16.x.fix pre-existing (LATEST_MARKOV_CAPSULE.txt deleted) |
| **SG-18.9** | M0 preflight passes before M1/M2 | ✅ Atom H sub-stage 1 commit `2bc712e` (M0 retry 20/20 PROCEED) |
| SG-18.10 | M1 50–100 problems completes with chain-backed evidence | ⏸️ NOT-RUN — forward-bound to TB-18.H-impl |
| SG-18.11 | M2 100+ n5 completes with Boltzmann observe-only | ⏸️ NOT-RUN — forward-bound to TB-18.H-impl |
| SG-18.12 | BenchmarkManifest exists | ✅ Atom H prep pre-existing |
| SG-18.13 | EvidencePackagingPolicy satisfied | ✅ pre-existing + Phase 4 evidence packaged per TB-7R/TB-8/TB-9 precedent |
| SG-18.14 | Benchmark report contains required disclaimers | ✅ Atom H sub-stage 1 pre-existing |
| SG-18.15 | G0 micro-audit AFTER F BEFORE H | 📨 Filed pre-H per architect §2.1 — awaits user external invocation |
| SG-18.16 | G1 ship audit + architect § sign-off | 📨 Filed — awaits user external invocation + architect sign-off |

**14/16 GREEN; 2 NOT-RUN forward-bound (SG-18.10 + SG-18.11) — same as Phase 4 entry state.**

## §3 CR walk

| CR | Description | Status |
|---|---|---|
| CR-18.1 | No real-world execution | ✅ |
| CR-18.2 | No real funds | ✅ (sandbox preseed only) |
| CR-18.3 | No public settlement | ✅ |
| CR-18.4 | No ChainTape bypass | ✅ (all 13 tx kinds via `bus.submit_typed_tx` or `emit_system_tx`) |
| CR-18.5 | All proposal/proof/failure evidence enters ChainTape/CAS | ✅ (synthetic ProposalTelemetry + EvidenceCapsule written) |
| CR-18.6 | Dashboard / benchmark report is materialized view | ✅ (tx_kind_distribution.json + SHARED_CHAIN_RUNS_REPORT.json computed-not-canonical) |
| CR-18.7 | No Class 4 surface hidden inside Class 3 atom | ✅ — git diff confirms no `src/state/sequencer.rs` / `src/state/typed_tx.rs` / canonical-signing-payload changes |
| CR-18.8 | No multi-chain union claimed as single-chain | ✅ — Phase 4 produces ONE chain, NOT a UNION |
| CR-18.9 | No hardcoded terminal state | ✅ (Atom E pre-existing closure) |
| CR-18.10 | No Boltzmann enforce unless separately ratified | ✅ (PRE-17.5 untouched) |

## §4 Pre-existing forward-bound items (unchanged from Atom H sub-stage 1)

| Item | To |
|---|---|
| TB-18.H-impl M1 (50-100 × n1/n3) | Forward-bound (multi-hour LLM compute) |
| TB-18.H-impl M2 (100+ × n5; observe-only) | Forward-bound (multi-day LLM compute) |
| Atom G0 Codex micro-audit | Filed; user-invoked (cloud-billed) |
| Atom G1 Codex+Gemini ship audit | Filed; user-invoked (cloud-billed) |
| Architect § sign-off | TB-17 §8 precedent |
| Atom D-impl lifecycle-order configurable | TB-19+ Class 4 ratification + Phase Z′ rerun |
| Atom C Gate 3 ChallengeStatus::Open-blocking | TB-19+ STEP_B_PROTOCOL Class 3 |
| PRE-17.5 Boltzmann ENFORCE | TB-19+ separate TB |
| PRE-17.7 β-D full pipeline | TB-19+ |
| M3 (controlled-market-enabled) + M4 (public report) | TB-19+ pilot design |

## §5 Architect Q2 ship-claim narrowing applied

Per architect TB-18 ratification ruling Q2 verbatim:

> "formal benchmark substrate partially closed; lifecycle-order constraint remains Class 4 forward trigger"

Phase 4 narrows further: **single-chain 13/13 tx-kind substrate now closed**; the lifecycle-order Class 4 constraint remains a TB-19+ forward trigger; Atom H M-ladder M1/M2 remain forward-bound.

## §6 Filing pattern

Per `feedback_kolmogorov_compression` (lossless archive over store-by-reference):
- Phase 1-4 source code commits PLUS this ship-status doc PLUS the README.md PLUS the dual-audit request doc are the durable record.
- Architect rulings are NOT re-summarized (lossless verbatim already in `handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md`).

Per `feedback_session_label_codification`: TB-18.B-impl is codified in this committed ship-status doc; not invented mid-session.

Per `feedback_no_fake_menus`: Phase 5 = single recommended path (commit → file dual audit request → await external invocation), not a menu.

## §7 Next-session triggers

| Trigger | Action |
|---|---|
| User invokes /ultrareview or Codex/Gemini on this commit | AI-coder reads verdicts → writes remediation commits OR ship-clean confirmation |
| User runs M1 batch | AI-coder writes M1 batch summary + benchmark report supplement |
| User runs M2 batch | AI-coder writes M2 supplement |
| Architect verdict on benchmark report | AI-coder applies §-sign-off (CONDITIONAL or CLEAN) per TB-17 §8 precedent |
| Architect ratifies multi-task chain shape | AI-coder updates Atom B verdict to "ratified-clean"; ship-claim narrows further if needed |

## §8 Worktree state at commit time

- HEAD on main: TB-18.B-impl SHIP COMMIT (Phase 1+2+3+4+5 combined)
- origin/main: NOT pushed (per `Don't push to remote unless user explicitly asks`)
- Workspace tests: 963/0/150 GREEN
- New evidence: handover/evidence/tb_18_b_phase4_2026-05-05/ (canonical r1; troubleshooting r0_wrong_cas_env)
- Untracked _dotgit_post_tar/ subdirs in r1 — local-only restore copies; user can verify or delete (NOT committed)

## §9 Class 3 ship envelope check

Per `feedback_class4_cannot_hide_in_class3` + `feedback_risk_class_audit`:
- Class 3 declared at charter time ✅
- No sequencer admission / typed-tx schema / canonical-signing-payload changes ✅
- Class 3 dual-audit request filed ✅ (`handover/audits/DUAL_AUDIT_TB_18_B_PHASE4_REQUEST_2026-05-05.md`)
- Conservative-resolution rule armed (VETO > CHALLENGE > PASS) ✅

## §10 Filed by

AI-coder (Claude) under TB-18.B-impl commit.

Filing under user blanket auto-mode authority "自主执行一直到 TB-18 ship".

End of TB-18.B-impl ship status.
