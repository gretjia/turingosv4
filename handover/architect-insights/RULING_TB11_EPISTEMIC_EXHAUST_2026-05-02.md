# Architect ruling annotation — TB-11 Epistemic Exhaust & Capital Liberation

**Authoritative archive**: `handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md`
(this file is the annotation layer; the directive archive is the source of truth)

**Issued**: 2026-05-02 evening, post-TB-10 ship.

**Status**: ratified by user authorization 2026-05-02 evening
("make it your own understanding but always align with the constitution.
then you can make it all the way to the end of the developing TB phases").

## One-line summary

TB-11 is **redirected** from the previously-planned "NodeMarket Decision +
Position Index" to **Epistemic Exhaust & Capital Liberation**:
EvidenceCapsule (CAS) + RunExhaustedTx (L4 anchor) + TaskExpireTx (escrow
refund) + TaskBankruptcyTx (NO/Short death certificate). NodeMarket moves to
TB-12+.

## Core principle introduced

> **O(1) chain cost, O(N) auditability.**
> State facts → L4. Rejected transactions → L4.E. High-dimensional evidence
> → CAS. Evidence summaries → dashboard / Information Loom. NodeMarket prices
> only after death certificates and proof certificates both exist.

## Why this came up

TB-13 PREVIEW (zeta-regularization run, 2026-05-02 mid-day) empirically
demonstrated the "Invisible Graveyard" failure mode: 132 LLM proposals + 73
Lean errors + 14 sorry-blocks + 26 parse failures + 0 OMEGA = bounty stuck in
escrow with no chain witness of why. Ledger has 2 L4 entries (TaskOpen +
EscrowLock) and stops there. The chain proves "no fake accepted" but cannot
prove "every fake attempt was refused" — one-sided epistemics.

## What changes for the next-TB queue

| Before (LATEST.md TB-10 ship) | After (this ruling) |
|---|---|
| TB-11 = NodeMarket M0/M1 (Decision Record + Position Index) | TB-11 = Epistemic Exhaust & Capital Liberation |
| TB-12 = MarketSeed (CompleteSet) | TB-12 = NodeMarket M0/M1 (was TB-11) |
| TB-13 = CPMM | TB-13 = CompleteSet (was TB-12) |
| TB-14 = AMM math | TB-14 = CPMM (was TB-13) |
| TB-15 = price index | TB-15 = PriceIndex / Boltzmann (was TB-14) |
| TB-16 = autopsy | TB-16 = Lamarckian Autopsy / Markov Loom |
| TB-17 = (n/a) | TB-17 = Pre-Real-World PCP Gate |

The architect explicitly states "先埋葬失败，释放资本；再允许市场围绕成功/失败进行定价"
(bury failure first, free up capital; then permit market pricing around
success/failure).

## What this DOES NOT change

- TB-10 ship stands (`6ab165c`). TB-10 user-facing CLI (lean_market) is the
  caller class for TB-11's lean_market tick / view-bankruptcy subcommands.
- All shipped invariants (CTF conservation, Anti-Oreo, no post-init mint,
  Ed25519 sponsor + solver durable identity) carry forward unchanged.
- Constitution unchanged.

## Implementation map (charter atom plan; full charter in
`handover/tracer_bullets/TB-11_charter_2026-05-02.md`)

```
Atom 0  Ratify charter + auto-resolve §7 open questions
Atom 1  Schema atom — typed_tx variants + EvidenceCapsule CAS schema
        - Extend TerminalSummaryTx additively with parent_state_root +
          solver_agent: Option<AgentId> + evidence_capsule_cid: Option<Cid>
          (architect §6.2 RunExhaustedTx ≡ TerminalSummaryTx in code; no
          production rows exist on this variant; additive bump is safe).
        - Extend TaskExpireTx additively with sponsor_agent + escrow_tx_id +
          reason: ExpireReason (architect §6.2; no production rows exist).
        - NEW TypedTx::TaskBankruptcy variant + signing payload + domain
          prefix.
        - NEW EvidenceCapsule struct in src/bottom_white/cas/ (CAS schema —
          NOT a TypedTx; lives entirely in L3 CAS, anchored from L4 by Cid).
        - NEW BankruptcyReason + ExpireReason + ExhaustionReason + 
          CapsulePrivacyPolicy enums.
Atom 2  Sequencer wire — dispatch arms + emit_system_tx commands
        - Implement TaskExpire dispatch (refund: escrows_t[escrow_tx_id]
          credits balances_t[sponsor_agent] by amount; debits total_escrow
          cache; CTF preserved).
        - Implement TerminalSummary dispatch (writes RunsIndex entry keyed
          by run_id; anchors evidence_capsule_cid on L4).
        - Implement TaskBankruptcy dispatch (writes
          task_markets_t[task_id].bankruptcy_at_logical_t = Some(t); blocks
          future EscrowLock for that task_id).
        - Add SystemEmitCommand::TaskExpire { task_id } /
          SystemEmitCommand::TerminalSummary { run_id, ... } /
          SystemEmitCommand::TaskBankruptcy { task_id, ... }.
        - Extend agent-ingress fail-closed match to include TaskBankruptcy.
Atom 3  EvidenceCapsule writer — src/runtime/evidence_capsule.rs
        - Compresses run.log + lean errors into .tar.gz; stores in CAS.
        - Returns EvidenceCapsule struct with capsule_id (Cid).
        - Privacy policy enum: AuditOnly / PublicSummaryBroadcast /
          AuthorizedCAS.
        - public_summary string redaction — never raw log, never agent prompts.
Atom 4  Runtime emission wiring
        - Evaluator on MAX_TX exhausted / timeout / solver give-up:
          1. build EvidenceCapsule, write to CAS.
          2. emit_system_tx TerminalSummary { run_id, evidence_capsule_cid,
             RunOutcome::MaxTxExhausted, ... }.
          3. if N consecutive RunExhausted on same task_id (N=3 default):
             emit_system_tx TaskBankruptcy.
        - lean_market new subcommand "tick":
          - scans task_markets_t for tasks past TASK_EXPIRY_LOGICAL_T_DELTA
            (default 1000) without a Finalized claim;
          - for each, emit_system_tx TaskExpire (refunds via
            tb11_emit_expire_for_eligible runtime helper);
          - returns count of expired tasks + total refunded.
        - lean_market new subcommand "view-bankruptcy":
          - reads task_markets_t for bankruptcy_at_logical_t Some;
          - prints task_id, bankruptcy_reason, evidence_capsule_cid.
Atom 5  Dashboard surface
        - audit_dashboard §12 Exhausted / Bankrupt / Expired runs section.
        - Per-row: task_id, run_id, attempt_count, terminal_reason, refund.
        - evidence_capsule_cid printed as Cid (clickable in future TB-13 web).
        - Raw evidence access requires --include-evidence flag (audit-only).
Atom 6  Smoke + replay validation
        - Reuse zeta-regularization corpus from tb_13_preview_zeta_*; this
          is the canonical hard-fail problem.
        - Drive evaluator with MAX_TX=10 forced exhaustion; assert:
          a) EvidenceCapsule in CAS (compressed log_cid resolves to data);
          b) RunExhaustedTx (= TerminalSummary) in L4 with capsule_cid;
          c) After expiry tick: TaskExpireTx in L4 + balances_t[sponsor]
             credited by exactly bounty;
          d) verify_chaintape 7 indicators GREEN;
          e) CTF conservation pre vs. post-refund.
Atom 7  Recursive self-audit + (optional Class 3) Codex / Gemini external
        - 4-clause: Constitutional / Replay-deterministic / Conservation /
          Negative-truth-completeness.
        - 11 ship gates per architect §8 (mapped 1-to-1).
        - Conservative verdict per feedback_dual_audit_conflict.
Atom 8  Ship — LATEST.md, TB_LOG.tsv row 33, TRACE_FLOWCHART_MATRIX TB-11
        row, evidence README, ship commit.
```

## Risk class

**Class 3** (capital-mover via TaskExpireTx escrow refund + system-emitted
sig path). Per `feedback_risk_class_audit` and `feedback_dual_audit`:
recursive self-audit + Codex impl-paranoid (mandatory) + Gemini architectural
strategic (degraded label OK if exhausted).

## Iteration cap

**72h** (production wire-up exception per `feedback_iteration_cap_24h`),
24h checkpoints. If Atom 5 dashboard or Atom 6 smoke slips past 72h, escalate
back to user before continuing.

## Forbidden in TB-11 (architect §11 + §5)

- ❌ NodeMarket trading
- ❌ CompleteSet
- ❌ CPMM / AMM
- ❌ ShortPosition payout
- ❌ Slash logic (RSP-3.2 territory)
- ❌ Per-attempt L4 spam (the explicit anti-pattern)
- ❌ Raw failure log broadcast to agent read view
- ❌ Ghost liquidity
- ❌ constitution.md edit

## Cross-references

- Constitution: Art. 0 (tape canonical) + Art. I.1 (5-step loop closure for
  negative path) + Art. II.2.1 (entropy on failure cohort) + Art. III.4
  (no fake accepted) + Art. IV (halt_reason taxonomy) + Art. V (Anti-Oreo)
- Memory: `feedback_kolmogorov_compression`, `feedback_no_retroactive_evidence_rewrite`,
  `feedback_chaintape_externalized_proposal`, `feedback_dual_audit`,
  `feedback_iteration_cap_24h`, `feedback_risk_class_audit`,
  `feedback_no_fake_menus`, `feedback_workspace_test_canonical`,
  `feedback_smoke_evidence_naming`
- TB-13 PREVIEW evidence (driver for this ruling):
  `handover/evidence/tb_13_preview_zeta_regularization_2026-05-02/`
- TB-10 ship state:
  `handover/evidence/tb_10_lean_market_mvp_smoke_2026-05-02/README.md`
- 9-phase roadmap (re-ordered post-this-ruling):
  `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
