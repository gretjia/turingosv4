# TB-7R Charter — Constitution-Aligned Frame B Repair

**Status**: **AUTHORIZED 2026-05-01** (architect verdict, post-D7-resolution).
**Date**: 2026-05-01
**Predecessor**: TB-7.7 D7 BLOCKED → resolved B′ (proposal-level DAG; per-tactic deferred to TB-8+).
**Phase**: P2 (Frame B finalization, P1/P3 carry-forward).
**Risk class**: **Class 3** (ChainTape authoritative path + CAS + signatures + economic preseed/escrow; not constitution-sudo).
**Iteration cap**: **72h exception granted with 24h checkpoints**.
**FC-trace**: `Art.I.1 + Art.III.4 + WP-§5.L3/L4 + DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29`.

**Phase declarations** (per `feedback_tb_phase_tag_required`):
```text
phase_id: P2 primary, P1/P3 carry-forward
roadmap_exit_criteria_addressed:
  P2:6
  P1:5,6,7,8,9
  P3 carry-forward (no ghost liquidity / conservation)
kill_criteria_tested:
  P1:1,2,3,4
  P3:1,2,3
```

---

## §0 Why TB-7R exists

TB-7.7 D1-D7 shipped structural Frame B work (payload-CAS, parent_tx,
VerificationResult, oracle/economic split, dashboard DAG, n5 evidence).
Architect ruling 2026-05-01 ultrathink turn flagged a constitutional
drift risk: **for the sake of DAG / golden-path analysis, the prior
plan implicitly invited Lean-failed proposals into L4 accepted via
stake/escrow gating.** Verdict withdraws that invitation. TB-7R is
the strict-constitution version of TB-7.7 closure:

```text
predicate pass  -> L4 accepted
predicate fail  -> L4.E rejection evidence
stake/escrow legitimacy ≠ predicate pass
```

TB-7R is **NOT TB-8**. TB-8 is reserved for the next phase
(Audit Dashboard / per-tactic DAG / deeper visualization).

---

## §1 One-line goal

Make every Frame B claim defensible against the four-clause acceptance:

```text
1. For every externalized LLM proposal:
     L4 accepted WorkTx OR L4.E rejected evidence — never both, never neither.

2. For every L4 accepted WorkTx:
     predicate evidence (Lean VerificationResult) exists and resolves from CAS.

3. For every failed proposal:
     in L4.E only; raw diagnostic shielded but auditable.

4. For every dashboard report:
     deletable and regeneratable from ChainTape + CAS alone.
```

---

## §2 Already-shipped work (verdict-validated)

| Commit | Verdict step | Status |
|---|---|---|
| `c0ec514` TB-7.6 | CAS race / sidecar atomic | ✓ |
| `a39c31b` D1+D2 | payload→CAS + parent_tx wire | ✓ |
| `89cd448` D4 | VerificationResult CAS | ✓ |
| `901062b` D5 | chain_oracle_verified split | ✓ |
| `07b6067` D6 | dashboard DAG + golden path | ✓ (verify materialized-view-only) |
| `e9cb023` D7 evidence | proposal-level DAG smoke | ✓ provisional under B′ |

---

## §3 New TB-7R deliverables (8 items)

### Deliverable A — L4 purity audit (read-only)

**Action**: scan existing TB-7.7 evidence (`handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/`,
`handover/evidence/tb_7_chaintape_smoke_2026-05-01/`) and any reachable L4 entries
in committed runtime_repos. For each L4 accepted WorkTx, verify:

```text
- ProposalTelemetry resolves
- verification_result_cid is Some(...) and resolves to a VerificationResult
- VerificationResult.verified == true
- proof_artifact_cid resolves
```

**Verdict**: NEVER retroactively migrate. If violations found, write
`handover/alignment/OBS_L4_PURITY_VIOLATION_2026-05-01.md` listing
(evidence_dir, tx_id, missing_field). Flag those evidence dirs as
unusable for accepted-state claim. New TB-7R runs must be correct.

**Ship gate**: purity report exists; either (a) zero violations or
(b) OBS file lists all violations and the affected evidence dirs are
README-annotated.

---

### Deliverable B — ChainTape-mode fail-closed for unsupported paths

**File**: `experiments/minif2f_v4/src/bin/evaluator.rs` + any sibling sites
in `src/runtime/`.

**Behavior**: when `TURINGOS_CHAINTAPE_*` env enables ChainTape mode AND
the executing path is `oneshot` / `OMEGA-full` / `OMEGA-pertactic` /
legacy `bus.append`, the binary MUST `error!()` + exit nonzero
(suggested code: `exit(3)` consistent with TB-7.5 fix #1).

**Priority** (per verdict B3):
- `run_swarm` append path → MUST be wired through `submit_typed_tx`.
- `OMEGA-full` → either wired or fail-closed; explicit choice required.
- `OMEGA-pertactic` → defer; fail-closed in ChainTape mode.
- `oneshot` → fail-closed in ChainTape mode until wired (TB-8.5+).

**Ship gate**: each unsupported path has a test that proves
fail-closed under ChainTape mode env. Out-of-mode (legacy mode) still
runs but emits NO ChainTape evidence and is NOT labeled chain-derived.

---

### Deliverable C — Genesis report emission

**File**: `src/runtime/mod.rs` `build_chaintape_sequencer_with_initial_q`
(or sibling bootstrap site).

**Behavior**: at chaintape bootstrap of a NEW run, write
`<runtime_repo>/genesis_report.json` containing:

```json
{
  "constitution_hash": "...",
  "runtime_repo": "...",
  "cas_path": "...",
  "system_pubkey": "...",
  "agent_pubkeys": "...",
  "initial_balances": "...",
  "task_id": "...",
  "task_open_tx": "...",
  "escrow_lock_tx": "..."
}
```

**Verdict**: going-forward only. Historical dirs receive a README note,
NEVER a fabricated genesis_report.json.

**Ship gate**: TB-7R smoke evidence directory contains
`genesis_report.json` with valid `task_open_tx` + `escrow_lock_tx`.

---

### Deliverable D — On-chain TaskOpenTx + EscrowLockTx (replace memory preseed in NEW runs)

**File**: `src/runtime/mod.rs` bootstrap path used for new TB-7R runs.

**Behavior**: instead of writing `q.economic_state_t.task_markets_t.insert(...)`
+ `q.economic_state_t.escrows_t.insert(...)` directly to memory, emit
two ChainTape transitions:

```text
TaskOpenTx (system signer)   -> L4 accepted
EscrowLockTx (sponsor signer) -> L4 accepted
```

**Verdict**: this applies to NEW TB-7R runs only. The historical
commit `054254f` (D3) memory-preseed is NOT rewritten. Both old and
new runtime_repos must be replayable; old replays use grandfather
path, new replays reconstruct task/escrow from ChainTape.

**Ship gate**: TB-7R smoke replay reconstructs `task_markets_t` +
`escrows_t` from L4 alone, with no memory-preseed dependency.

---

### Deliverable E — Historical evidence README annotations

**Targets**:
- `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md`
- `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/README.md`
- `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/README.md`

**Behavior**: append a "TB-7R grandfathering note" section stating:

```text
This evidence predates TB-7R genesis-report and on-chain
TaskOpenTx/EscrowLockTx requirements. It MAY use memory-only preseed
and SHOULD NOT be cited as TB-7R-grade ChainTape evidence.
For TB-7R-grade evidence see handover/evidence/tb_7r_*_2026-05-XX/.
Lean-result attestation in this evidence is evaluator-attested
(stdout-derived); only TB-7R-grade evidence is chain-oracle-derived
(VerificationResult CAS).
```

**Ship gate**: all three READMEs annotated; no fabricated genesis_report.

---

### Deliverable F — TB-7R smoke (single → half → full)

**Sequence**:

```text
1. single-problem smoke
     1 problem × MAX_TX small × ChainTape mode
     Pass criterion: genesis_report.json valid; ≥1 attempt node
     in ChainTape (L4 or L4.E); replay reconstructs.

2. half smoke
     3 problems × MAX_TX = 20 × ChainTape mode
     Pass criterion: same as single, plus parent_tx edges visible
     when multiple externalized proposals exist on same branch.

3. full n5 smoke
     5 problems (or CONDITION=n5) × MAX_TX ≥ 20 × ChainTape mode
     Pass criterion: ≥2 agent_ids; ≥1 parent_tx edge;
     all externalized proposals in ChainTape (L4 or L4.E);
     solved problem has chain_oracle_verified golden proposal;
     unsolved problem has L4.E failures and no fake accepted node.
```

**Evidence directory**: `handover/evidence/tb_7r_smoke_2026-05-XX/`.

**Ship gate**: each step PASSES before the next is invoked.

---

### Deliverable G — Audits (per verdict C4)

**Audit point 1** (after Deliverable A purity audit + B fail-closed +
C/D bootstrap changes): **Codex micro-audit** focused on L4 / L4.E
gate purity and ChainTape-mode fail-closed.

**Audit point 2** (before ship): **Codex implementation audit + Gemini
architecture audit**. If Gemini unavailable, label evidence
`degraded — Codex-only` per `feedback_dual_audit` rule. Do NOT pretend
full dual.

**VETO > CHALLENGE > PASS** hierarchy applies (`feedback_dual_audit_conflict`).

**Ship gate**: audit reports present; any VETO blocks ship; CHALLENGE
requires written rebuttal or fix.

---

### Deliverable H — TB-7R ship report

**File**: `handover/RECURSIVE_AUDIT_TB_7R_2026-05-XX.md`

**Content**: line-grounded against the four-clause acceptance criteria.
Includes commit list (TB-7R range), test counts (`cargo test --workspace`
canonical per `feedback_workspace_test_canonical`), audit summaries,
smoke evidence directory pointer, OBS list, kill-criterion status.

**TB_LOG.tsv row**: TB-7R row with phase_id / risk_class / exit_criteria /
kill_criteria / commit_range / ship_status.

---

## §4 Forbidden (verdict §7 hard guardrails)

```text
- NO NodeMarket / position semantics
- NO role taxonomy mutations (M / B+ / B-)
- NO whale tracking / contested-node metrics
- NO FinalizeRewardTx / SettlementTx (TB-9 territory)
- NO SlashTx
- NO MetaTape / ArchitectAI auto-mutate-rules
- NO predicate registry mutation
- NO constitution.md / RootBox / sudo touches
- NO retroactive ledger rewrite
- NO fabricated historical genesis_report.json
- NO per-tactic decomposition (deferred to TB-8+ per verdict A1)
- NO new TypedTx variant (verdict + WP rule; VerificationResult is CAS object)
```

---

## §5 Checkpoint schedule (verdict C7)

```text
Checkpoint 1: after CAS race + payload CAS
              [STATUS: already done at TB-7.6 + D1 — file as
               CHECKPOINT_TB7R_1_2026-05-01.md citing existing commits]

Checkpoint 2: after parent_tx + VerificationResult + L4/L4.E purity
              [REQUIRES: Deliverable A purity audit complete; D2/D4
               re-validated under verdict criteria]

Checkpoint 3: before n5 full smoke
              [REQUIRES: Deliverables B/C/D/E shipped; single + half
               smoke PASSED; Codex micro-audit clear]
```

Each checkpoint file: what changed / what tests passed / what remains
red / kill-criterion status.

---

## §6 24h checkpoint cadence (verdict C5)

Every 24h within the 72h exception window, commit one of:
- `cargo test --workspace` PASS evidence
- single-problem ChainTape smoke evidence
- `CHECKPOINT_TB7R_*.md`

Missing 24h beat → escalate to user before next code change.

---

## §7 Cross-references

- TB-7R authorization verdict: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`
- TB-7R ingestion analysis: `handover/directives/2026-05-01_TB7R_CONSTITUTION_ALIGNED_REPAIR.md`
- TB-7.7 charter: `handover/tracer_bullets/TB-7.7_charter_2026-05-01.md` (predecessor)
- TB-7 charter: `handover/tracer_bullets/TB-7_charter_2026-05-01.md`
- D7 close-out: `handover/ai-direct/HANDOVER_TB_7_7_D7_RESOLVED_2026-05-01.md`
- L4 / L4.E ledger decision: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- Three-node taxonomy decision: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`
- 9-phase roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`

---

## §8 Production claim (post-TB-7R ship)

> "TuringOS Frame B is constitution-aligned: every externalized LLM
> proposal lands in either L4 accepted (predicate-passed) or L4.E
> rejected evidence, never both. Predicate pass / fail (Lean
> VerificationResult) alone determines L4 vs L4.E — stake/escrow do
> not manufacture acceptance. The DAG is proposal-level (per-tactic
> deferred to TB-8+). Genesis state for new runs is established via
> on-chain TaskOpenTx + EscrowLockTx, not memory preseed. Historical
> evidence is grandfathered, not rewritten. The audit dashboard is a
> read-only materialized view from ChainTape + CAS — deleting it does
> not lose authoritative state. NodeMarket, settlement, slash, and
> per-tactic DAG remain post-TB-7R."
