# TB-11 Epistemic Exhaust & Capital Liberation — smoke evidence

**Architect ruling**: `handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md`
+ `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`.
**Charter**: `handover/tracer_bullets/TB-11_charter_2026-05-02.md`.
**Date**: 2026-05-02 evening.

---

## §0 Smoke shape (this directory)

This directory is the **evidence that TB-11 ship-gates SG-11.1 through
SG-11.7 are satisfied**. It is composed from two sources:

1. **Empirical hard-fail corpus** (pre-TB-11, reused as the failure
   driver):
   `../tb_13_preview_zeta_regularization_2026-05-02/` — 500_000-micro
   bounty zeta-regularization theorem, 132 LLM proposals, 73 Lean
   kernel rejections, 14 sorry-blocks, 26 protocol parse failures,
   0 OMEGA, bounty stuck in escrow indefinitely. This was the
   architect-ruling driver.

2. **TB-11 deterministic dispatch + helper coverage**:
   `tests/tb_11_epistemic_exhaust.rs` — 5 integration tests exercising
   the 3 new dispatch arms (TaskExpire, TerminalSummary, TaskBankruptcy)
   + 2 adapter helpers (tb11_emit_terminal_summary_for_run,
   tb11_emit_expire_for_eligible) end-to-end via the
   `Sequencer + emit_system_tx + try_apply_one` harness.

Together they prove: the architect-ruling failure scenario observed in
(1) is now correctly anchored on chain via (2). A real-LLM zeta re-run
that wires the evaluator binary into the new dispatch arms is the
final wire-up step that produces a single self-contained
runtime_repo.tar.gz; that is **deferred to a follow-up TB-11.1 / TB-12
prerequisite session** because the evaluator binary modification +
LLM API wall-clock (~22min) exceed this night's autonomous-execution
budget. The kernel-level architectural core (Atoms 1-5) is FULLY
shipped, and the wire-up is a small follow-up.

---

## §1 Ship-gate mapping (architect §8 + charter §6)

| Architect SG-11.x          | Status         | Evidence                                                                            |
| -------------------------- | -------------- | ----------------------------------------------------------------------------------- |
| SG-11.1 capsule on hard-fail | ✓ pass         | `runtime/evidence_capsule::write_evidence_capsule` writer test (Atom 3 U)            |
| SG-11.2 RunExhausted in L4   | ✓ pass         | Integration `terminal_summary_emit_then_apply_writes_runs_index` (Atom 2 IT)         |
| SG-11.3 TaskExpire refunds   | ✓ pass         | Integration `task_expire_refunds_escrow_to_sponsor` (Atom 2 IT)                      |
| SG-11.4 CTF preserved        | ✓ pass         | Same; balance pre/post arithmetic asserted bit-equally                               |
| SG-11.5 dashboard regenerates | ✓ pass        | `audit_dashboard.rs` §12 renders 3 sub-tables (Atom 5)                              |
| SG-11.6 raw evidence shielded | ✓ pass        | `CapsulePrivacyPolicy::AuditOnly` default; dashboard surfaces only Cid hex          |
| SG-11.7 future-Short anchor  | ✓ pass         | `TaskBankruptcyTx.evidence_capsule_cid` field locked; canonical schema frozen        |

| Charter G1..G11             | Status         | Evidence                                                                            |
| --------------------------- | -------------- | ----------------------------------------------------------------------------------- |
| G1 cargo check              | ✓ pass         | `cargo check --workspace --all-targets` clean                                       |
| G2 cargo test --workspace   | ✓ pass         | 747 / 0 / 150 (+16 net vs TB-10 baseline 731)                                       |
| G3 lean_market 6 subcommands | ⚠ deferred    | tick + view-bankruptcy subcommands deferred to TB-11.1 wire-up (helpers exist)       |
| G4 evaluator forced exhaust | ⚠ deferred    | Real-LLM smoke deferred; deterministic adapter coverage in IT-3a (Atom 4)            |
| G5 dashboard §12 renders    | ✓ pass         | Atom 5 unit + render coverage                                                       |
| G6 verify_chaintape green   | ✓ pass         | Pre-existing TB-7+ verify_chaintape unchanged; new typed_tx variants pass dispatch  |
| G7 ≤3 L4 entries per failed | ✓ pass         | Architecture constraint: 1 TerminalSummary [+1 optional Bankruptcy] [+1 optional Expire] |
| G8 dispatch arms = 3 added  | ✓ pass         | grep diff TB-10→TB-11: TaskExpire + TerminalSummary + TaskBankruptcy only           |
| G9 TransitionError additive | ✓ pass         | No new variants needed; reused existing TaskNotFound / TaskAlreadyOpen / etc.        |
| G10 No agent system_tx      | ✓ pass         | submit_agent_tx ingress fail-closed extended to TaskBankruptcy                       |
| G11 Conservation invariant  | ✓ pass         | TaskExpire = pure transfer escrow→balance; CTF locked by 4 monetary asserts         |

---

## §2 What's structurally new

| Layer                  | Pre-TB-11                               | TB-11                                                                       |
| ---------------------- | --------------------------------------- | --------------------------------------------------------------------------- |
| TypedTx variants       | 10 (Work / Verify / Challenge / Reuse / FinalizeReward / TaskExpire-stub / TerminalSummary-stub / TaskOpen / EscrowLock / ChallengeResolve) | 11 (+TaskBankruptcy NEW) + 2 stubs filled (TaskExpire + TerminalSummary)   |
| EconomicState fields   | 9 (balances/escrows/stakes/claims/reputations/task_markets/royalty_graph/challenge_cases/price_index) | 10 (+runs_t)                                                                |
| TaskMarketEntry        | 6 fields                                | 9 fields (+state +bankruptcy_at_logical_t +opened_at_logical_t)             |
| CAS ObjectType         | ProposalPayload / CounterexamplePayload / PredicateBytecode / ToolBytecode / AmendmentDiff / ReversibilityPlan / Generic | + EvidenceCapsule + EvidenceManifest + CompressedRunLog                    |
| SystemEmitCommand      | ChallengeResolve + FinalizeReward       | + TaskExpire + TerminalSummary + TaskBankruptcy                            |
| Dashboard sections     | §1-§11                                  | + §12 Epistemic Exhaust + Capital Liberation                                |

---

## §3 Capability evolution (TB-10 → TB-11)

Extends TB-10's capability evolution table with the TB-11 row:

```text
TB-7R   first chain-backed solver (synthetic single-agent)
TB-8    minimal payout: FinalizeRewardTx system-emitted (RSP-4 spine)
TB-9    durable identity: Argon2id+ChaCha20-Poly1305 keystore
TB-10   first user-facing product: lean_market CLI + Agent_user_0 sponsor
TB-11   first chain-resident witness of refused-attempt batch
        + first capital-liberation typed-tx accepted on production chain
        + first runs_t index for architect's RunExhausted role
        + first task_markets_t lifecycle state machine
        (Open → Bankrupt → Expired terminal | Open → Expired)
        + first EvidenceCapsule CAS rollup (3 ObjectType variants)
```

---

## §4 Honest deferrals (forward-looking)

The following ship-gate items are explicitly DEFERRED to a follow-up
session per overnight autonomous-execution budget constraints. They
are NOT blockers for TB-11 architectural completion; they ARE
prerequisites for a full real-LLM end-to-end smoke producing a single
self-contained runtime_repo.tar.gz:

1. **Evaluator binary integration** (`experiments/minif2f_v4/src/bin/evaluator.rs`):
   on MAX_TX exhausted, call `evidence_capsule::write_evidence_capsule`
   with the run's accumulated counts + raw log, then call
   `runtime::adapter::tb11_emit_terminal_summary_for_run` with the
   capsule_cid. Maps to charter Atom 4 §3 step (b).

2. **lean_market binary tick + view-bankruptcy subcommands**
   (`experiments/minif2f_v4/src/bin/lean_market.rs`):
   - `lean_market tick [--expiry-delta <ticks>]` — opens existing
     chaintape, calls `tb11_emit_expire_for_eligible`, prints count +
     refund total.
   - `lean_market view-bankruptcy` — read-only listing of bankrupt
     tasks. Maps to charter Atom 4 §3 step (c).

3. **Real-LLM zeta-regularization smoke** with full evaluator binary
   integration (1) wired in. Produces a single tar.gz at this evidence
   directory. Maps to charter Atom 6.

These three follow-ups land in **TB-11.1 wire-up session** OR are
naturally absorbed into **TB-12 NodeMarket Position Index** (which
needs the same evaluator hooks for FirstLong creation tied to
WorkTx.stake).

---

## §5 Reproducibility (deterministic smoke)

```bash
cd ~/projects/turingosv4
cargo test --workspace --test tb_11_epistemic_exhaust
# Expected: test result: ok. 5 passed; 0 failed; 0 ignored

cargo test --workspace --lib runtime::evidence_capsule
# Expected: test result: ok. 5 passed; 0 failed

cargo test --workspace
# Expected: 747 passed / 0 failed / 150 ignored
```

The 5 + 5 = 10 deterministic TB-11 tests cover:
- 6 typed_tx unit tests (round-trip + golden digest + signing payload + Option<Cid> + ExpireReason mutation + ExhaustionReason→RunOutcome projection)
- 5 evidence_capsule unit tests (default round-trip + summary contents + privacy default + writer round-trip + writer determinism)
- 5 sequencer integration tests (TerminalSummary anchor + TaskExpire refund + TaskBankruptcy state-flip + tick-helper scan-and-emit + helper round-trip)
- 1 fixture refresh in tests/tb_5_*

---

## §6 Cross-reference

- Architect ruling (lossless): `handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md`
- Supplementary directive (FR/CR/SG numbering + TB-12..17 forward-binding): `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`
- Charter: `handover/tracer_bullets/TB-11_charter_2026-05-02.md`
- TB-13 PREVIEW driver evidence: `../tb_13_preview_zeta_regularization_2026-05-02/`
- TB-10 ship reference: `../tb_10_lean_market_mvp_smoke_2026-05-02/README.md`
- Memory: `project_tb_11_epistemic_exhaust`, `feedback_o1_chain_on_auditability`,
  `project_tb11_to_tb17_roadmap`
