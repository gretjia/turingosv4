# TB-7.7 Charter — DAG-capable ChainTape Probe

**Status**: **RATIFIED 2026-05-01** by architect ruling (post-TB-7.6 ship).
**Date**: 2026-05-01
**Predecessor**: TB-7.6 SHIPPED at `c0ec514` (CAS race fix + I90d/e/f tamper tests).
**Phase**: P2 (TB-7 Frame B carry-forward; NOT a separate TB-9/TB-11 territory).
**Audit class**: Class 2 (production wire-up — touches evaluator hot path + adds CAS object schema additions + ChainDerivedRunFacts schema additions).

---

## §0 Why this probe exists

The TB-7 Atoms 2/3/4/5/6/7 + TB-7.5/7.6 closure shipped **routing-layer
correctness**: every meaningful LLM proposal traverses `bus.submit_typed_tx`,
signatures verify, replay reconstructs structural facts. Phase B real-LLM
evidence (commit `a981317`) confirmed real DeepSeek output enters
ChainTape with 423/448 tokens + step_complete tactic + Agent_0 pubkey.

**But on closer inspection (architect ruling 2026-05-01 ultrathink turn),
five core capabilities remain blocked at the chain layer**:

```text
1. 真正的 chaintape           — 🟨 routing 真，artifact 字节不在 CAS
2. node append               — 🟨 全 L4.E (stake=0)
3. tree search               — 🔴 parent_tx 永远 None
4. golden path on solved题   — 🔴 不可能 (3 洞: payload bytes 缺 / Lean
                                   verdict 不在链 / chain.solved 永远 false)
5. 整体 DAG 分析             — 🟨 节点+元数据有，无边、无内容、无 oracle outcome
```

The gaps are NOT due to NodeMarket / Settlement / Slash being missing
(those are correctly deferred to TB-9 / TB-11). They're due to **wiring +
schema gaps within TB-7 charter scope**:

- (#1) `proposal_artifact_cid` is computed as `sha256(payload_bytes)` but
  the bytes themselves are never written to CAS. A hash without storage.
- (#2) `evaluator.rs` Atom 2/3 hot paths use `stake = 0` → all WorkTx route
  to L4.E. Charter §4.3 narrowed scope said "no settlement", but it
  also said "ChallengeWindow OPEN" — implying accepted L4 WorkTx is
  in-scope; only FinalizeRewardTx is out.
- (#3) `ProposalTelemetry.parent_tx` field exists but evaluator passes
  `None` always.
- (#4) Lean's actual verdict (exit code, stderr, proof file hash) has no
  chain-side record. `VerifyTx::Confirm` is a verifier *declaration*, not
  Lean *evidence*.
- (#5) `ChainDerivedRunFacts.solved` requires accepted L4 + VerifyTx::Confirm
  in L4 — impossible under current zero-stake routing.

TB-7.7 closes all five **without** introducing NodeMarket / Settlement /
Slash — i.e. **strictly within charter §4 narrowed scope**.

---

## §1 One-line goal

Given a chain-backed LLM run, `audit_dashboard` (composing `verify_chaintape`
+ `chain_derived_run_facts` + new `golden_path_extract`) must be able to:

1. List every meaningful LLM proposal as a chain node (L4 or L4.E).
2. Render parent/child edges from `ProposalTelemetry.parent_tx`.
3. Recover the full proposal payload from CAS via `proposal_artifact_cid`.
4. On solved problems, identify the golden path (root → ... →
   `chain_oracle_verified` node) using only ChainTape + CAS + replay
   report — no evaluator-stdout dependency.
5. Distinguish `chain_oracle_verified` (Lean accepted; this TB) from
   `chain_economic_finalized` (settlement complete; TB-9+ territory).

---

## §2 Seven required deliverables (architect ruling)

### Deliverable 1 — Proposal payload bytes → CAS

**File**: `src/runtime/proposal_telemetry.rs` `build_for_evaluator_append`

**Change**: take `&mut CasStore` parameter; call `cas.put(payload_bytes,
ObjectType::ProposalPayload, creator, logical_t, schema_id)` and use
returned CID as `proposal_artifact_cid` (instead of unstored `sha256(...)`).

**Ship gate**: 100% of WorkTx.proposal_cid resolves to ProposalTelemetry
AND 100% of ProposalTelemetry.proposal_artifact_cid resolves to original
payload bytes via CasStore.get.

### Deliverable 2 — `parent_tx` wired in evaluator

**File**: `experiments/minif2f_v4/src/bin/evaluator.rs` Atom 2 + Atom 3 sites

**Change**: maintain `last_tx_by_agent_branch: BTreeMap<(AgentId,
String), TxId>` in evaluator scope. Pass `parent_tx` into a new
`ProposalTelemetry::new_with_parent` constructor. After each
bus.submit_typed_tx, record `last_tx_by_agent_branch[(agent_id, branch_id)]
= submitted_tx_id`.

**Ship gate**: ≥1 `branch_lineage` edge in dashboard output for any run
with ≥2 same-agent same-branch proposals.

### Deliverable 3 — Real LLM WorkTx can enter L4 accepted

**Approach**: ENV-gated pre-seed at chaintape bootstrap + non-zero stake
in evaluator hot path.

**Files**:
- `src/runtime/mod.rs` `build_chaintape_sequencer_with_initial_q`: optionally
  pre-seed `task_markets_t` + `balances_t[Agent_*]` from env vars.
- `experiments/minif2f_v4/src/bin/evaluator.rs` Atom 2/3 hot paths:
  use stake from env (`TURINGOS_CHAINTAPE_PROPOSAL_STAKE_MICRO`, default
  1000 = 0.001 coin).

**Preserve L4.E**: at least one labeled forced rejection with explicit
`forced_rejection_for_gate_3 = true` so Gate 3 stays satisfied even on
all-accept runs.

**Ship gate**: ≥1 accepted L4 WorkTx + ≥1 L4.E rejection for any
ChainTape run with non-trivial LLM activity.

### Deliverable 4 — Lean oracle verdict as CAS evidence

**File**: NEW `src/runtime/verification_result.rs`

**Schema**:

```rust
pub struct VerificationResult {
    pub target_work_tx: TxId,
    pub verifier_agent: AgentId,
    pub lean_exit_code: i32,
    pub lean_stdout_hash: Hash,
    pub lean_stderr_hash: Hash,
    pub proof_file_hash: Hash,
    pub proof_artifact_cid: Cid,
    pub verified: bool,
}
```

`ProposalTelemetry` gains optional `verification_result_cid: Option<Cid>`
(additive; pre-existing telemetry remains valid via `Option::None`).

`evaluator.rs` writes VerificationResult to CAS for OMEGA-accept paths
BEFORE submitting WorkTx + VerifyTx (so the WorkTx's ProposalTelemetry
can include the cid). Append-branch (intermediate) proposals leave
`verification_result_cid: None`.

**Ship gate**: ≥1 VerificationResult in CAS per OMEGA-accepted run;
chain-derived `chain_oracle_verified` field returns `true` iff at least
one such accepted-L4 WorkTx + Confirm-Verify pair has a `verified=true`
VerificationResult.

### Deliverable 5 — `chain_oracle_verified` / `chain_economic_finalized` split

**File**: `src/runtime/chain_derived_run_facts.rs`

**Add fields** (additive; existing `solved` / `verified` retained for
compat with documented semantics narrowing):

```rust
pub chain_oracle_verified: bool,   // ≥1 accepted L4 WorkTx + Confirm + VerificationResult.verified=true
pub chain_economic_finalized: bool, // always false in TB-7 (settlement = TB-9+)
```

Document in module docstring: existing `solved`/`verified` reflect
**economic-level finality** (which is `false` until TB-9 settlement);
the new `chain_oracle_verified` is the **oracle-level** signal that TB-7
can produce.

**Ship gate**: on a ChainTape run with at least one Lean-verified
proposal landing in L4 accepted, `chain_oracle_verified=true` AND
`chain_economic_finalized=false`.

### Deliverable 6 — `audit_dashboard` upgrade

**File**: `src/bin/audit_dashboard.rs`

**Add sections**:
- §8 DAG nodes (each L4 + L4.E entry as a node row, with parent_tx,
  agent_id, branch_id, candidate_tactic, payload preview, oracle_verified)
- §9 DAG edges (parent_tx → child_tx via ProposalTelemetry)
- §10 Golden path (only on `chain_oracle_verified=true`: walk root → … →
  oracle-verified node; show payload bytes from CAS at each step)

**Ship gate**: dashboard outputs non-empty DAG edges on multi-attempt
runs; outputs a non-empty golden path on solved problems.

### Deliverable 7 — n5 multi-agent ChainTape smoke

**Procedure**: run evaluator with `CONDITION=n5` × `MAX_TX≥20` ×
`TURINGOS_CHAINTAPE_*` env on 1-2 problems. Capture evidence to
`handover/evidence/tb_7_7_dag_capable_smoke_2026-05-XX/`.

**Ship gate evidence**:
- ≥2 distinct agent_ids in agent_pubkeys.json
- ≥1 accepted L4 WorkTx
- ≥1 L4.E rejection (natural or labeled-forced)
- ≥1 non-empty parent_tx edge
- ≥1 oracle_verified node (Lean accepted)
- Golden path reconstructable from ChainTape + CAS

---

## §3 Out of scope (architect ruling explicit non-list)

- NodeMarket position semantics → TB-11
- Role taxonomy (M / B+ / B-) → needs architect ratify schema additions
- BUY YES / BUY NO orders → TB-11
- Whale tracking / contested-node metrics → TB-11
- FinalizeRewardTx settlement → TB-9 minimal payout
- SlashTx → TB-9 (or TB-11)
- Public chain → P7
- CONDITION=oneshot ChainTape wiring → TB-8.5 or later (oneshot + chaintape
  dual mode is a separate concern)

---

## §4 Forbidden (inherits TB-7 §6)

Inherits all TB-7 §6 #1-33. TB-7.7 additions:

34. **No new TypedTx variant** — VerificationResult is a CAS object, NOT
    a new TypedTx. ProposalTelemetry.verification_result_cid is an
    optional CID field (additive). VerifyTx ABI unchanged.
35. **No charter §4.3 settlement scope expansion** — non-zero stake in
    Deliverable 3 is for ADMISSION GATE clearance, NOT for
    FinalizeRewardTx / SlashTx / payout. Stake stays locked in
    `stakes_t`; no settlement transitions emit.
36. **No `chain_economic_finalized = true` in TB-7.7** — placeholder
    field always `false` until TB-9 ships SettlementEngine. This is
    explicit semantic separation, not a future-coupling.

---

## §5 Build order (commit-by-commit)

```text
Commit 1 (DONE @ c0ec514)  — TB-7.6 CAS race + I90d/e/f
Commit 2  — D1: payload bytes → CAS (proposal_telemetry + tests)
Commit 3  — D2: parent_tx wire (evaluator + ProposalTelemetry::new_with_parent)
Commit 4  — D3: pre-seed for L4 accept (runtime bootstrap + evaluator stake)
Commit 5  — D4: VerificationResult CAS object (new module + telemetry field)
Commit 6  — D5: chain_oracle_verified split (chain_derived_run_facts)
Commit 7  — D6: audit_dashboard DAG + golden path
Commit 8  — D7: n5 multi-agent smoke evidence + TB-7.7 ship report
```

---

## §6 Cross-references

- TB-7 charter: `handover/tracer_bullets/TB-7_charter_2026-05-01.md`
- TB-7 ARCHITECT_RULING: `handover/directives/2026-05-01_TB7_ARCHITECT_RULING.md`
- TB-7 ship-time Codex audit: `handover/audits/CODEX_TB7_FULLDIFF_AUDIT_2026-05-01.md`
- TB-7 Phase B real-LLM smoke: `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/README.md`
- TB-7.5 audit fixes commit: `f7f511b`
- TB-7.6 CAS race fix commit: `c0ec514`

---

## §7 Production claim (post-TB-7.7)

> "TuringOS chaintape is now DAG-capable: every meaningful LLM proposal
> is recorded as a chain node with full payload retrievable from CAS and
> parent/child lineage on the ProposalTelemetry side. On problems where
> Lean accepts a proof, the golden path (root → … → oracle-verified
> node) is reconstructable from ChainTape + CAS + replay report alone,
> with no evaluator-stdout dependency. Real LLM proposals can land in L4
> accepted via stake-gated admission; rejected proposals stay in L4.E
> with full ProposalTelemetry. Oracle-level acceptance
> (`chain_oracle_verified`) is distinct from economic-level finality
> (`chain_economic_finalized`), the latter remaining `false` until TB-9
> SettlementEngine. NodeMarket positions, FinalizeRewardTx, SlashTx, and
> public chain remain post-MVP per charter §13.1."
