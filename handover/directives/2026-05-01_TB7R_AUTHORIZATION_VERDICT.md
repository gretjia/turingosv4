---
date: 2026-05-01
ingested_by: claude (architect-ingest skill)
phase_id: P2 (Frame B finalization)
status: AUTHORIZED — execution authorized within stated bounds
classification: architect verdict — formal authorization for TB-7R
supersedes_pending_decisions_in: handover/directives/2026-05-01_TB7R_CONSTITUTION_ALIGNED_REPAIR.md
relates_to:
  - handover/ai-direct/HANDOVER_TB_7_7_D7_PENDING_2026-05-01.md (D7 BLOCKED → resolved B′)
  - handover/tracer_bullets/TB-7.7_charter_2026-05-01.md
  - handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md
---

# Architect Verdict 2026-05-01 — TB-7R Authorization

## Headline

**TB-7R authorized as Constitution-Aligned Repair, not as feature expansion.**
First principle (unchanged from prior directive):

```text
predicate pass -> L4 accepted
predicate fail -> L4.E rejection evidence
stake / escrow 不能制造 L4 accepted node
```

No Layer-1 violation. Directive reinforces: kernel zero domain knowledge,
append-only DAG, economic conservation.

---

## Resolutions (per question raised in ingestion analysis)

### A — Constitution / thesis-level

| ID | Question | Verdict |
|---|---|---|
| **A1** | D7 (链 = 答案 vs CoT) per-tactic? | **B′**: TB-7R does NOT do per-tactic decomposition. Keep `complete` tool. Compound proposal (LLM-output whole calc block in 1 call) = 1 attempt node. Per-tactic decomposition deferred to TB-8+. |
| **A2** | Three-node taxonomy → constitution? | NO. Use in TB-7R / Developer Manual / Decision Record only. Write to `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`. Do NOT modify constitution.md. |
| **A3** | Directive = amendment or stricter reading? | Stricter reading of existing constitution. Register as case-law / decision record, NOT amendment. |
| **A4** | D7 single-WorkTx = constitutional violation? | NOT automatically. Becomes BLOCKER iff `proposal_count_runtime_externalized > proposal_count_chaintape_attempts (L4 + L4.E)`. Internal CoT / private model thinking is excluded from the count by design. |

### B — Scope / historical evidence

| ID | Question | Verdict |
|---|---|---|
| **B1** | Redo D3 memory-only pre-seed? | **New evidence**: yes (must use on-chain TaskOpenTx + EscrowLockTx). **Historical evidence** at `054254f`: do NOT rewrite. Annotate README only. |
| **B2** | Migrate historical L4 → L4.E? | NO retroactive ledger rewrite. Run purity audit; write `OBS_L4_PURITY_VIOLATION_*.md` if violations found; flag as unusable for accepted-state claim; new runs must be correct. |
| **B3** | `oneshot` / `OMEGA-full` / `OMEGA-pertactic` / append path handling? | In ChainTape mode: **fail-closed** if not wired through `submit_typed_tx`. Do NOT delete modes. Priority: `run_swarm append` MUST be wired; `OMEGA-full` either wired or fail-closed; `OMEGA-pertactic` defer; `oneshot` fail-closed in ChainTape mode. |
| **B4** | genesis_report.json backfill? | Going-forward only. Historical dirs get a README note ("predates TB-7R genesis-report requirement"). NEVER fabricate. |
| **B5** | "node append" terminology cleanup? | TB-7R docs / dashboard / decision record only. NO src/ global rename, NO TRACE_MATRIX rewrite, NO constitution.md change. |
| **B6** | evaluator-stdout → oracle truth decoupling? | New TB-7R runs MUST use VerificationResult CAS as oracle evidence. Historical runs grandfathered with `evaluator-attested` / `stdout-attested` label; NOT relabeled `chain-oracle-derived`. |

### C — Process / Living Harness

| ID | Question | Verdict |
|---|---|---|
| **C1** | TB name? | **TB-7R**. Reserve TB-8 for next phase (Audit Dashboard / per-tactic DAG / deeper visualization). |
| **C2** | Risk class? | **Class 3** (ChainTape authoritative path + CAS + signatures + economic preseed/escrow). NOT Class 4 (no constitution.md / RootBox / sudo). |
| **C3** | STEP_B trigger? | Only if touching `src/state/sequencer.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs` / `src/kernel.rs`. CAS / dashboard / ProposalTelemetry / VerificationResult / evaluator / docs / tests do NOT trigger. |
| **C4** | Dual-audit scope? | NOT per-commit. Two audit points: (1) Codex micro-audit after L4 purity / L4.E gate fix; (2) Codex implementation audit + Gemini architecture audit before ship. Gemini unavailable → label `degraded`, do NOT pretend full dual. |
| **C5** | 24h vs 72h? | **72h exception granted.** Mandatory 24h checkpoints: each 24h must commit `cargo test --workspace` PASS, OR single-problem ChainTape smoke, OR `CHECKPOINT_TB7R_*.md`. |
| **C6** | phase_id / roadmap_exit / kill? | `phase_id: P2 primary, P1/P3 carry-forward`. `roadmap_exit_criteria: P2:6, P1:5,6,7,8,9, P3 carry-forward (no ghost liquidity / conservation)`. `kill_criteria: P1:1,2,3,4 + P3:1,2,3`. NOT P5/P6/NodeMarket. |
| **C7** | Phased checkpoints? | **3 checkpoints required**: (1) after CAS race + payload CAS (already done at TB-7.6 + D1); (2) after parent_tx + VerificationResult + L4/L4.E purity; (3) before n5 full smoke. Each checkpoint: what changed / what tests passed / what remains red / kill-criterion status. |
| **C8** | Smoke granularity? | Sequence: single-problem → half (3 problems × MAX_TX 20) → full (5 problems or n5 × MAX_TX≥20). NO direct full batch. |
| **C9** | FC-trace tags? | Use exact TRACE_MATRIX FC if precise row exists. Else commit-message form `FC-trace: WP-§5.L3/L4 + Art.I.1 + Art.III.4` and add a TRACE_MATRIX TODO / orphan justification. NEVER fabricate FC numbers. |

### D — Sequencing

| ID | Question | Verdict |
|---|---|---|
| **D1** | CAS race first? | YES. Independent of A1. Already shipped at TB-7.6 commit `c0ec514`. |
| **D2** | Commit 2-6 order? | `CAS race → payload CAS → parent_tx → VerificationResult → L4/L4.E purity → dashboard`. NOT n5 before DAG fixes. |
| **D3** | Commit 7 (n5 smoke) depends on A1? | YES. Per A1=B′, n5 expectation is **proposal-level DAG, not tactic-level**. Acceptance: `≥2 agent_ids`, `≥1 parent_tx edge if multiple externalized proposals`, all externalized proposals represented, solved problem has `chain_oracle_verified` golden proposal, unsolved has L4.E failures and no fake accepted node. |

---

## Authorization (verbatim — to forward to AI coder)

```text
Authorization:

1.  Use TB-7R name.
2.  Execute CAS race fix first if not already committed.
    [NOTE: already shipped at c0ec514 (TB-7.6).]
3.  Do not modify constitution.md.
4.  Treat directive as stricter interpretation / case-law, not amendment.
5.  Do not retroactively rewrite old ledger roots.
6.  New TB-7R evidence must use on-chain TaskOpenTx + EscrowLockTx,
    not memory-only preseed.
7.  Run L4 purity audit; if old accepted L4 entries fail predicates,
    write OBS, do not migrate history.
8.  In ChainTape mode, unsupported oneshot / OMEGA-full / OMEGA-pertactic
    paths must fail-closed.
9.  Defer per-tactic decomposition to TB-8+.
10. Current DAG = proposal-level DAG only.
11. Use Attempt Node / State Node / Rejection Evidence Node terms in
    docs/dashboard, not constitution.
12. Risk class = Class 3.
13. STEP_B only if protected files are touched.
14. Audits:
    - Codex micro-audit after L4 purity / L4.E gate fix.
    - Codex + Gemini ship audit if available; Gemini unavailable =>
      degraded label.
15. 72h exception granted with 24h checkpoints.
16. Final smoke sequence:
    single-problem -> half smoke -> full n5 smoke.
```

---

## Acceptance criteria (from §8 of verdict)

```text
For every externalized LLM proposal:
  it is represented as either L4 accepted WorkTx or L4.E rejected evidence.

For every L4 accepted WorkTx:
  predicate evidence exists and resolves from CAS.

For every failed proposal:
  it is not in L4 accepted;
  it is in L4.E;
  raw diagnostic is shielded but auditable.

For every dashboard report:
  it can be deleted and regenerated from ChainTape + CAS.
```

---

## Mapping verdict 7-commit list → already-shipped commits

| Verdict commit | Status | Existing commit |
|---|---|---|
| 1. CAS race / sidecar atomic | ✓ SHIPPED | `c0ec514` (TB-7.6) |
| 2. Proposal payload → CAS | ✓ SHIPPED | `a39c31b` (TB-7.7 D1) |
| 3. parent_tx DAG edge | ✓ SHIPPED | `a39c31b` (TB-7.7 D2, same commit) |
| 4. Lean VerificationResult CAS | ✓ SHIPPED | `89cd448` (TB-7.7 D4) |
| 5. L4 / L4.E strict split | ⚠ NEEDS PURITY AUDIT | (verify D3 `054254f` purity; B2 verdict applies) |
| 6. chain_oracle_verified | ✓ SHIPPED | `901062b` (TB-7.7 D5) |
| 7. n5 smoke | ⚠ PROVISIONAL | `e9cb023` D7 evidence captured under B′; new TB-7R smoke must apply B1+B3+B4+B6 corrections |

**Net new TB-7R work** (post-verdict):

1. L4 purity audit (read-only) → OBS if violations.
2. ChainTape-mode fail-closed for `oneshot` / `OMEGA-full` / `OMEGA-pertactic` (B3).
3. Genesis-report emission at chaintape bootstrap (B4 going-forward).
4. On-chain TaskOpenTx + EscrowLockTx replacing memory preseed in NEW runs (B1 going-forward).
5. README annotations on historical evidence (B1+B4 grandfathering note).
6. New TB-7R smoke (single → half → n5) under corrected criteria.
7. Codex micro-audit (post purity) + Codex+Gemini ship audit.
8. TB-7R ship report.

---

## Awaited: user "go" before execution

Verdict authorizes the plan and bounds; this archive documents it.
Execution begins on user "go" (or equivalent). Per architect-ingest:
接收指令 ≠ 授权执行 — but here authorization IS explicit. The pause
is procedural: confirm scope (this archive + TB-7R charter +
decision record + memories + task list) is correct before the first
code change.
