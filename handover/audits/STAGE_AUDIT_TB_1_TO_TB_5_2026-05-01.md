# Stage Audit — TB-1 through TB-5 cumulative — 2026-05-01

**Scope**: cumulative audit of TuringOS v4 P1 + P3 RSP-N work shipped between 2026-04-29 and 2026-04-30. Single-AI self-audit per user instruction.
**Branch / HEAD**: `main` @ `c472823`
**Test suite**: 617/617 passing (`cargo test --workspace`, 46 suites, 0 failed; `--workspace` flag mandatory per D4 in architect review request).

---

## §1 Per-TB summary

| TB | Date shipped | RSP version | Production claim added | Tests added (workspace count delta) | Smoke evidence | On-chain? |
|---|---|---|---|---|---|---|
| TB-0 | 2026-04-29 | N/A (pre-RSP) | First v4-native solve `pput=0→0.000215` on `mathd_algebra_107` via nlinarith | (no rust tests; capability anchor only) | `first_v4_solve_2026-04-29/` | ❌ |
| TB-1 | 2026-04-30 | RSP-0 primitives | Monetary invariant scaffolding + L4.E rejection-evidence; Tier-A/B split; runtime enforcement deferred TB-2 | +9 Tier-A | `tb_1_day4_h_vppu/` (Day-4 spike; pre-`prompt_context_hash`) | ❌ |
| TB-2 | 2026-04-30 | + Runtime spine | Accepted WorkTx → canonical L4 (state_root_t / ledger_root_t / logical_t advance); rejected → L4.E with submit_id; replay from L4 alone reaches same state | +16 (3 in-crate + 13 integration) | `tb_2_phase1_smoke/` (oneshot only) | ❌ |
| TB-3 | 2026-04-30 | RSP-1 formal-tx-surface | TaskOpen + EscrowLock first-class; bridge deletion (CI-enforced); WorkTx admission structural via `task_markets_t.total_escrow > 0` + solver solvency; lock-on-accept | +29 (5 typed + 8 in-crate + 11 integration + 2 bridge invariant + 3 monetary) | `tb_3_smoke/` (oneshot only) | ❌ |
| TB-4 | 2026-04-30 | RSP-2 admission | VerifyTx + ChallengeTx admission; verifier bond + challenger NO stake atomically transferred; opened_at_round window-anchor; target_work_tx backref; 3-class error taxonomy | +30 (5 typed + 10 in-crate + 12 integration + 3 control/replay) | `tb_4_smoke/` (oneshot + n1 SOLVED) + `tb_4_medium_batch/` (4/5 SOLVED at MAX_TX=30) | ❌ |
| TB-5 | 2026-04-30 | RSP-3.0 + RSP-3.1 | Anti-Oreo agent-vs-system ingress separation **structurally enforced**; emit_system_tx constructs+signs internally; apply_one stage 1.5 PinnedSystemPubkeys verify; ChallengeResolve dispatch arm Released + UpheldDeferred paths | +46 (5 typed + 13 in-crate + 10 + 13 integration + 3 anti-drift + 2 misc) | `tb_5_smoke/` (oneshot + n1 SOLVED) | ❌ |

**Cumulative TB-1..TB-5**: 130 net new rust tests; ~571 + ~46 = 617 workspace count at TB-5 ship.

---

## §2 What is structurally green at TB-5 ship

### §2.1 Runtime kernel (Sequencer + dispatch + ledger)

- `Sequencer::apply_one` exhaustively dispatches over the 10 TypedTx variants (TB-5 added ChallengeResolve as the 10th).
- Stage 1.5 (TB-5) + Stage 2 (TB-2) + Stage 3-9 (TB-2) all closed.
- Two L4 paths: accepted → `LedgerWriter::commit(LedgerEntry)`; rejected → `RejectionEvidenceWriter::append_rejected(RejectedSubmissionRecord)`. Mutually exclusive per K1 invariant.
- `LedgerEntry` carries `parent_ledger_root` chain + `system_signature` over `LedgerEntrySigningPayload::canonical_digest()`. Replay from L4 alone reconstructs `economic_state_t` (tests I29 + I80).
- `Git2LedgerWriter` (line `transition_ledger.rs:642`) exists for on-disk persistence; `InMemoryLedgerWriter` (line 243) is the test-harness path.

### §2.2 Anti-Oreo enforcement (TB-5 atomic)

- `Sequencer::submit_agent_tx` rejects 4 system-emitted TypedTx variants pre-queue with `SubmitError::SystemTxForbiddenOnAgentIngress` (FinalizeReward + TaskExpire + TerminalSummary + ChallengeResolve).
- `Sequencer::emit_system_tx` is the sole system-ingress path; constructs the typed tx + signs internally with `Ed25519Keypair`; callers cannot pass forged signatures because they do not construct the typed tx.
- apply_one stage 1.5 (`system_message_for_verification` exhaustive helper) re-verifies system_signature against `PinnedSystemPubkeys` for current epoch; failure → `TransitionError::InvalidSystemSignatureLive` + 1 L4.E PolicyViolation row + no logical_t advance (K1).
- Defense-in-depth: emit-time `verify_emitted_system_tx_signature` + apply_time stage 1.5 verify. Forged-sig replay paths covered by U22-U28 unit tests.

### §2.3 RSP-1 formal tx surface (TB-3)

- `TaskOpen` + `EscrowLock` first-class TypedTx variants. Bridge at `sequencer.rs:197-215` (TB-2 P0-B option (a)) DELETED at TB-3 Atom 6 commit `fa85350`; CI-enforced by `tests/tb_3_bridge_deletion_invariant.rs`.
- WorkTx admission structural: `q.economic_state_t.task_markets_t[work.task_id].total_escrow > 0` AND `q.economic_state_t.balances_t[work.agent_id] >= work.stake`. No bridges.
- Lock-on-accept: accepted WorkTx atomically debits `balances_t[agent]` by `work.stake` + inserts `stakes_t[work.tx_id] = StakeEntry { amount, staker, task_id }`. Rejected WorkTx leaves `economic_state_t` bit-identical (L4.E never mutates economic state per user verdict #14).
- `task_market.total_escrow` is a **derived cache**, NOT a money holding (`monetary_invariant.total_supply_micro` deliberately does not count it; would double-count on every lock).

### §2.4 RSP-2 admission spine (TB-4)

- `VerifyTx` admission: atomic balance debit → stakes_t[verify.tx_id] credit; verifier puts skin in the game; verdict rides L4 only, never mutates Q_t (charter § 3.10 signal-not-judge).
- `ChallengeTx` admission: atomic balance debit → challenge_cases_t[challenge.tx_id] credit with `opened_at_round = q.q_t.current_round` + `target_work_tx` back-ref. Multi-challenger representable (each ChallengeTx keys a distinct entry).
- Window-only-anchor rule: TB-4 emits `opened_at_round`; does NOT compute closure / deadline / auto-finalize. RSP-3 owns closure.

### §2.5 RSP-3.0 + RSP-3.1 resolution gate (TB-5)

- `ChallengeResolve` dispatch arm Released path: refund challenger += case.bond + zero bond + flip status to Released (entry preserved for audit trail per directive § 7 Q6).
- UpheldDeferred path: marker only — flip status to UpheldDeferred; bond preserved for TB-6 RSP-3.2 slash routing.
- Idempotency: `AlreadyResolved` gate. Unknown target: `ChallengeNotFound`. Both map to L4ERejectionClass::PolicyViolation.
- `ChallengeStatus` enum (q_state.rs:385) is the Q-side single source of truth (3 states: Open / Released / UpheldDeferred + serde-default Open). On-wire `ChallengeResolution` enum (typed_tx.rs:465) is distinct payload type (Released / UpheldDeferred only — Open is not on-wire).
- 5-holding CTF invariant + 9-sub-field EconomicState UNCHANGED: Released is balanced transfer between holding 5 (challenge_cases.bond) and holding 1 (balances_t); UpheldDeferred touches no holding.

### §2.6 Anti-drift CI (cumulative)

- TB-3: `tests/tb_3_bridge_deletion_invariant.rs` — bridge pattern must not resurrect.
- TB-4: `tests/tb_4_rsp2_admission_surface.rs::no_no_stake_tx_or_verifier_bond_tx_variant_in_src` — 4 forbidden variant names.
- TB-5: `tests/tb_5_anti_drift.rs` — extends with SlashTx + SettlementTx + ProvisionalAcceptTx + ReputationUpdateTx; charter hygiene + P6-touch git-diff guard.
- All anti-drift CI tests skip comments (only flag actual code references).

---

## §3 What is gap at TB-5 ship

### §3.1 Production-binary chaintape wire-up — NOT GREEN

Per `SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` § 3:
- No production binary drives `Sequencer::apply_one`.
- `bus.rs` `sequencer: Option<Arc<Sequencer>>` is `None` in `main.rs` (`TuringBus::new_legacy()`).
- `experiments/minif2f_v4/src/bin/evaluator.rs` does not import `turingosv4::state::sequencer` at all.
- No on-disk chain (`Git2LedgerWriter` instance) has ever been produced from an LLM-driven run.
- Cumulative chaintape debt: 5 TBs of kernel functionality with no production wire-up.

This is a **structural gap** flagged by user 2026-05-01 question "现在 TuringOS 具有真正的 chaintape 了吗？".

### §3.2 Smoke evidence is paper trail, not chain audit

Per § 1.3 of architect review request:
- `*_run.log` files are stdout dumps with no signature, no parent_ledger_root, no replay path.
- `prompt_context_hash` invariance across 5 sessions is a structural compat signal for the **prompt-build pipeline**, NOT for the kernel.
- "Smoke tape" naming is metaphor inherited from v3 PaperTape; not a structural property.

### §3.3 RSP-3.2 slash + RSP-4 settlement + RSP-5/6/7 — RED

- RSP-3.2 (slash execution; SlashTx system-emitted; balances/stakes/challenge_cases mutations conditional on UpheldDeferred status) — TB-6 candidate per current ROADMAP, OR pivot to P2 Agent Runtime per architect review D1.
- RSP-4 (settlement_engine) — TB-7+ territory.
- RSP-5 (contribution_dag) + RSP-6 (reputation_update_tx + price index) + RSP-7 (production market dynamics) — RED.

### §3.4 P2 Agent Runtime end-to-end role separation — RED

Solver / Verifier / Challenger / Planner role-separation Exit criteria cannot be demonstrated until evaluator → Sequencer wire-up exists (depends on §3.1 gap closure). Currently demonstrated only in `cargo test` fixture-level role distinction.

### §3.5 P4 Information Loom — RED

Depends on real L4.E rejection-evidence records + reputation events; both exist as test-fixture rows, not LLM-driven production rows.

### §3.6 P5/P6/P7/P8 — RED

Out of TB-1..TB-5 scope by design.

---

## §4 Open debts surfaced at this stage gate

| # | Debt | Severity | Remedy candidate |
|---|---|---|---|
| 1 | Test-count under-report 464→617 across 5 docs | medium | Mechanical patch commit on main |
| 2 | "Smoke tape" naming implies cryptographic chain it does not have | medium | Rename to "smoke evidence" in templates + retroactive |
| 3 | No production binary drives Sequencer (5-TB cumulative debt) | high | TB-6 = P2 Agent Runtime atom (recommended) OR explicit "P2 = TB-7" target |
| 4 | Audit-mode standard not codified for system-emitted economic mutators (TB-5 Codex-only with grep self-verify fallback was ad-hoc) | medium | Architect ruling D3 in review request |
| 5 | Workspace-test convention not codified (`cargo test --workspace` mandatory) | low | Architect ruling D4 + memory update `feedback_phased_checkpoint` |

---

## §5 Production claims rolling forward to TB-6

After TB-5 ship + this stage audit, the following claims are operative for any TB-6+ work:

1. **Runtime kernel honors L4 / L4.E split** (TB-2 production claim, still green).
2. **RSP-1 formal tx surface is on canonical L4** (TB-3, still green).
3. **WorkTx.stake commits real money on accept; rejected WorkTx leaves economic state untouched** (TB-3, still green).
4. **task_market.total_escrow is derived cache, not money holding** (TB-3, still green).
5. **VerifyTx.bond + ChallengeTx.stake stay inline; no VerifierBondTx / NoStakeTx variants** (TB-4, still green; CI-enforced by I44 + tb_5_anti_drift scanner).
6. **ChallengeResolve is system-emitted only; agent forging structurally impossible** (TB-5, GREEN; this is the constitutional Anti-Oreo claim).
7. **emit_system_tx constructs+signs internally; apply_one stage 1.5 verifies via PinnedSystemPubkeys** (TB-5, GREEN).
8. **5-holding CTF invariant + 9-sub-field EconomicState preserved across all admission + resolution paths** (TB-3..TB-5 cumulative, GREEN).

All 8 claims are demonstrated by `cargo test --workspace` (617/617). None are demonstrated by any LLM-driven production run.

---

## §6 Summary

**TB-1..TB-5 stage gate**: kernel-side structural progress is genuine and substantively defensible at the test-harness level. The 5 production claims above hold under `cargo test --workspace` audit.

**Outstanding**: production binary wire-up gap (§3.1) is the single largest debt at this gate. It does not invalidate TB-5 ship — but it does mean the "smoke tape" gate in TB-3/TB-4/TB-5 ship docs has been overstated as a structural guarantee. Honest restatement: smoke evidence proves runs happened + capability is replicable + prompt-build pipeline is unchanged; it does NOT prove the kernel was exercised at runtime.

**Architect review** (`handover/directives/2026-05-01_TB6_ARCHITECT_REVIEW_REQUEST.md`) is the appropriate forum to rule on whether TB-6 closes the gap (P2 Agent Runtime) or extends RSP-N micro-versions (RSP-3.2 slash). 5 decision items D1-D5 await architect ruling.
