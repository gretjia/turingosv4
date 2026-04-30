# STEP_B Preflight — TB-3 RSP-1 Formal Tx Surface (WP-canonical)

**Date**: 2026-04-30
**TB**: TB-3 ("P3 RSP-1 Formal Tx Surface")
**Charter**: `handover/tracer_bullets/TB-3_charter_2026-04-30.md` (DRAFT v2)
**Protocol**: `handover/ai-direct/STEP_B_PROTOCOL.md`
**Predecessor STEP_B**: `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md` (v3, audit cycle complete)
**Status**: Phase-0 brief — pending external dual audit (Codex + Gemini). User scope review of charter v2 must precede launch.

---

## 0. Why STEP_B applies here

`STEP_B_PROTOCOL.md` § 0 (verbatim):
> any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`, `src/state/sequencer.rs`). Also applicable to any proposal that touches "institution" per C-031.

TB-3 changes `src/state/sequencer.rs` (now in CLAUDE.md restricted list per TB-2 P1-A). Two additional files are non-restricted but C-031 institutional-class:

- `src/state/q_state.rs` — Q_t schema. TaskMarketsIndex key migration `BTreeMap<TxId, _>` → `BTreeMap<TaskId, _>` is a constitutional shape change.
- `src/economy/monetary_invariant.rs` — CTF conservation gate. `total_supply_micro` migrates from 6 holdings (TB-1) to 5 holdings (drops the `bounty` term). This touches the supply-conservation invariant directly; per `cases/C-001_post_genesis_minting.yaml` it is the highest-criticality enforcement code in the project.

Treat all three as full STEP_B scope. Disagreement between auditors → conservative verdict (require) per `feedback_dual_audit_conflict`.

---

## 1. Target

| File | Role | Touched by TB-3 |
|---|---|---|
| `src/state/sequencer.rs` | L4 sequencer + dispatch arms + apply_one driver | **YES** (primary). Adds: 2 new `dispatch_transition` arms (`TaskOpen`, `EscrowLock`); refactors WorkTx arm. Adds: `TASK_OPEN_DOMAIN_V1` + `ESCROW_LOCK_DOMAIN_V1` const. **DELETES**: bridge code at `:197-215` (incl. `// TB-2 P0-B option (a): drop this when task_open_tx lands in TB-3` comment at `:205`). No `Sequencer` struct field changes; no `Sequencer::new` signature change. |
| `src/state/typed_tx.rs` | TypedTx ABI + signing payloads + TransitionError | **YES**. Adds 2 new variants (`TaskOpen(TaskOpenTx)`, `EscrowLock(EscrowLockTx)`); 2 new structs + 2 SigningPayloads + 2 HasSubmitter impls + 2 new `DOMAIN_AGENT_*` consts. Adds **3** new `TransitionError` variants (`TaskAlreadyOpen`, `TaskNotOpen`, `InsufficientBalance`) + 3 Display arms. **`WorkTx` schema is UNCHANGED** (per charter § 3.1 WP-canonical decision — no `stake_lock_tx_id`; no `YesStakeTx`). |
| `src/state/q_state.rs` | Q_t schema | **YES**. `TaskMarketsIndex(BTreeMap<TxId, _>)` → `(BTreeMap<TaskId, _>)`. `TaskMarketEntry`: REMOVE `bounty`, ADD `total_escrow` + `escrow_lock_tx_ids: BTreeSet<TxId>` + `settlement_rule_hash: Hash`. `EscrowEntry`: ADD `task_id: TaskId` (additive serde-default). `StakeEntry`: ADD `task_id: TaskId` (additive serde-default). 9-sub-field `EconomicState` invariant preserved. |
| `src/economy/monetary_invariant.rs` | CTF conservation gate | **YES**. `total_supply_micro` drops the `bounty` term (`:90-94`). 6 → 5 holdings. Adds new public predicate `assert_task_market_total_escrow_matches_locks` per charter § 3.2 cache=truth invariant. TB-1 test `ctf_counts_all_six_holding_subindexes` (`:404-435`) renamed + rewritten to `_five_`. |
| `src/bottom_white/ledger/transition_ledger.rs` | TxKind enum + canonical L4 | invoked through existing API; `TxKind` enum gains 2 variants (`TaskOpen`, `EscrowLock`). |
| `src/bottom_white/ledger/rejection_evidence.rs` | L4.E writer | `RejectionClass` enum gains 1 variant (`InsufficientBalance = 4`). `public_summary_for` arm `"insufficient_balance"`. |
| `src/sdk/tools/wallet.rs` / `src/bus.rs` / `src/kernel.rs` | wtool / bus / kernel | **NO**. |
| `src/economy/ledger.rs::AcceptedLedger` | TB-1 RSP-0 primitive | **NO** (stays primitive; not used as production accepted spine — TB-2 #6 inherited red line). |
| `src/economy/escrow_vault.rs` | task-keyed EscrowVault | **NO** (TB-3 reads `q.economic_state_t.escrows_t.0` and `task_markets_t.0` only; `EscrowVault` remains the future RSP-2+ unification target). |
| `tests/tb_3_rsp1_formal_surface.rs` | TB-3 integration acceptance battery (~11 tests) | **NEW**. |
| `tests/tb_3_bridge_deletion_invariant.rs` | bridge-resurrection CI invariant | **NEW** (Rust-native recursive scanner). |
| `src/state/sequencer.rs` `#[cfg(test)] mod tb3_runtime_boundary` | TB-3 in-crate unit tests (8 tests) | **NEW**. |
| `src/state/typed_tx.rs::tests` | TB-3 ABI unit tests | **NEW** (T1-T5). |
| `src/economy/monetary_invariant.rs::tests` | 6→5 migration + cache=truth + double-count regression | **MODIFIED + NEW** (3 tests). |

---

## 2. Why the change is necessary (Phase-0 brief for external audit)

**Observable institutional debt at HEAD `a82f73e`** (post-TB-2 ship):

1. **The bridge** at `src/state/sequencer.rs:197-215` is a synthetic-ID-from-TaskId construction:
   ```rust
   let lookup_tx_id = TxId(work.task_id.0.clone());
   let has_escrow = q.economic_state_t.escrows_t.0.contains_key(&lookup_tx_id)
                  || q.economic_state_t.task_markets_t.0.contains_key(&lookup_tx_id);
   ```
   This:
   - violates Art. 0.2 Tape Canonical (the lookup key is not derivable from any tape transition; it is a runtime cast);
   - is explicitly marked as a TB-3 deletion target with the comment `// TB-2 P0-B option (a): drop this when task_open_tx lands in TB-3` at `:205`;
   - was admitted by TB-2 only because there was no formal `TaskOpenTx` to populate `task_markets_t` legitimately.
2. **No code path emits accepted L4 rows that populate `task_markets_t` or `escrows_t`**. Test fixtures do this by direct `EconomicState` mutation. This is precisely the "ghost liquidity" pattern forbidden by ROADMAP § 3 P3 Forbidden CF-2.
3. **`task_markets_t.bounty`** at `src/state/q_state.rs:228-239` is currently treated as a money holding (counted in `monetary_invariant.rs::total_supply_micro:90-94`; named "bounty" in the test at `:405`). Once `EscrowLockTx` lands, the same money is also in `escrows_t.amount` — counting both would mint phantom money on every escrow lock. Even before TB-3 lands, the schema is set up to encourage double-counting: any `EscrowLockTx`-shaped helper that updates `task_markets_t.bounty` AND `escrows_t.amount` simultaneously without dropping the bounty term would trigger this.
4. **`WorkTx.stake` admission is event-bound by WP § 18 Inv 5 but not realized at runtime**: TB-2's accepted WorkTx leaves `economic_state_t` unchanged (`src/state/sequencer.rs:217-221` docstring: "TB-2 does not yet move stake/escrow balances; RSP-1 lifecycle is TB-3+"). Solver could submit infinite work_txs each declaring large stake without any balance commitment; the "1 Coin = 1 YES + 1 NO" CTF promise is structurally unredeemed.

**Failure mode if we don't change**:

- The bridge stays as institutional debt forever. Any future TB that adds new admission predicates has to compose against the bridge's synthetic-ID semantics.
- Test fixtures continue seeding `EconomicState` directly, which reproduces ghost-liquidity patterns in tests and risks the same anti-pattern leaking into production code paths.
- The WP § 14.1 / § 18 Inv 5 promise that stake is event-bound risk commitment remains a paper claim, not a runtime invariant.
- P4 Information Loom blocks: failure clusterer needs separable rejection classes (no-balance vs no-task vs no-escrow vs invariant-violation). Without the new `InsufficientBalance` L4E class, "余额不够" and "其他 policy 违规" appear identical to clusterer.

**Less-invasive alternatives considered and rejected** (per `STEP_B_PROTOCOL.md` Phase-0 gate):

- *(Alt A)* Keep the bridge; introduce only `TaskOpenTx` + `EscrowLockTx` and DON'T migrate `TaskMarketsIndex` key from TxId to TaskId. Rejected: the bridge's `TxId(task_id.0.clone())` cast still abuses TxId as a name-space carrier. Even if `TaskOpenTx` happens to assign `task_open_tx.tx_id == TxId(task_id.0)`, that's an implicit invariant (TxId-of-task-open-tx == namespace-cast-of-TaskId) that any future tx allocator can break silently. WP § 19 RSP-1 names TaskMarket as a per-task concept; key migration is the constitutional fit.
- *(Alt B)* Introduce a separate `YesStakeTx` variant, with `WorkTx` carrying `stake_lock_tx_id: TxId`. Rejected: contradicts WP § 14.1 + § 5 Layer 4 + § 18 Inv 5 + economic § 7 + § 19. The user verdict 2026-04-30 explicitly chose WP-canonical inline-stake. See charter § 3.1.
- *(Alt C)* Keep `task_markets_t.bounty` as a holding alongside introducing `total_escrow`. Rejected: double-counts every locked bounty. The clean invariant separates source-of-truth (`escrows_t.amount`) from cache (`task_markets_t.total_escrow`). See charter § 3.2.
- *(Alt D)* Stake admission-only (don't lock balance on accept). Rejected: violates WP § 18 Inv 5 (YES/NO event-bound) + Law 2 (Only Investment Costs Money — investment is now). Renders WorkTx.stake structurally unredeemed; "phantom stake" is the rigorous twin of ghost liquidity.

**Audit gate**: if both Codex and Gemini say "less-invasive alternative exists" or "scope wrong", revise. If both say "necessary as scoped", proceed to Phase 1. Disagreement → conservative verdict (block).

---

## 3. Minimum sufficient version (scope ceiling)

Day-1 of any production-code work must NOT exceed the items below.

### 3.1 `q_state.rs` schema migration (Atom 2)

**TaskMarketsIndex key migration**:

```rust
// src/state/q_state.rs:222-224 — BEFORE
pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);

// AFTER
pub struct TaskMarketsIndex(pub BTreeMap<TaskId, TaskMarketEntry>);
```

**TaskMarketEntry field migration** (`src/state/q_state.rs:228-239`):

```rust
// AFTER
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskMarketEntry {
    #[serde(default)]
    pub publisher: AgentId,
    // REMOVED: pub bounty: MicroCoin
    #[serde(default = "MicroCoin::zero")]
    pub total_escrow: MicroCoin,                    // NEW — derived aggregate, NOT a holding
    #[serde(default)]
    pub escrow_lock_tx_ids: BTreeSet<TxId>,         // NEW — replay-deterministic provenance
    #[serde(default = "task_market_default_quorum")]
    pub verifier_quorum: u32,
    #[serde(default = "task_market_default_royalty_bp")]
    pub max_reuse_royalty_fraction_basis_points: u16,
    #[serde(default)]
    pub settlement_rule_hash: Hash,                  // NEW — RSP-3/4 hook
}
```

**EscrowEntry additive field** (`src/state/q_state.rs:166-178`):

```rust
// AFTER
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscrowEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub depositor: AgentId,
    #[serde(default)]
    pub task_id: TaskId,    // NEW — backref to enable cache=truth invariant check
}
```

**StakeEntry additive field** (`src/state/q_state.rs:184-197`): mirror — add `task_id: TaskId`.

Why both `EscrowEntry.task_id` and `StakeEntry.task_id` are additive: serde-default (`#[serde(default)]`) means any pre-TB-3 deserialized row gets `TaskId::default()`. TB-3 is pre-anchor (no anchored Q_t to migrate); existing tests that don't populate `task_id` continue to work. Cache=truth invariant test populates the field explicitly.

**`#[serde(default)]` on EscrowEntry/StakeEntry default to TaskId::default() = TaskId("")**. The cache=truth invariant test must verify that a non-default task_id round-trips correctly — covered by U6 + I22 in §5.

### 3.2 `monetary_invariant.rs` 6 → 5 holding migration (Atom 2)

```rust
// src/economy/monetary_invariant.rs:80-94 — BEFORE
fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
    let mut total: i64 = 0;
    // sum over balances + escrows + stakes + claims + bounty + bond
    // ... 6 terms ...
    Ok(total)
}

// AFTER (Atom 2)
fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
    let mut total: i64 = 0;
    for (_, v) in &s.balances_t.0 { total = total.checked_add(v.micro_units()).ok_or(MonetaryError::Overflow)?; }
    for (_, e) in &s.escrows_t.0 { total = total.checked_add(e.amount.micro_units()).ok_or(...)?; }
    for (_, e) in &s.stakes_t.0 { total = total.checked_add(e.amount.micro_units()).ok_or(...)?; }
    for (_, e) in &s.claims_t.0 { total = total.checked_add(e.amount.micro_units()).ok_or(...)?; }
    for (_, c) in &s.challenge_cases_t.0 { total = total.checked_add(c.bond.micro_units()).ok_or(...)?; }
    // task_markets_t.total_escrow is INTENTIONALLY OMITTED — derived cache, not a holding.
    // Counting it would double-mint every locked bounty (Σ escrows.amount also counts it).
    Ok(total)
}
```

**New public predicate** (Atom 2):

```rust
/// Cache=truth invariant for derived `task_markets_t[task_id].total_escrow` field.
/// Equals the sum of `escrows_t[e].amount` for all escrows whose `task_id == task_id`.
/// MUST hold across every accepted state transition that touches escrows or task_markets.
pub fn assert_task_market_total_escrow_matches_locks(
    s: &EconomicState,
    task_id: &TaskId,
) -> Result<(), MonetaryError> {
    let cached = s.task_markets_t.0.get(task_id)
        .map(|m| m.total_escrow.micro_units())
        .unwrap_or(0);
    let derived: i64 = s.escrows_t.0.values()
        .filter(|e| &e.task_id == task_id)
        .map(|e| e.amount.micro_units())
        .sum();
    if cached != derived {
        return Err(MonetaryError::DerivedCacheMismatch {
            cached,
            derived,
            // ... task_id payload ...
        });
    }
    Ok(())
}
```

The new `MonetaryError::DerivedCacheMismatch` variant is additive to the error enum.

**Renamed test**: `ctf_counts_all_six_holding_subindexes` (`:404-435`) → `ctf_counts_all_five_holding_subindexes`. Fixture rewritten to drop the `task_markets_t.bounty = X` setup and replace with `escrows_t.insert(TxId(...), EscrowEntry { amount: X, depositor, task_id })` for the same X. Total stays identical.

**New test**: `total_supply_does_not_double_count_total_escrow` — explicitly constructs an EconomicState with both `escrows_t[e].amount = K` and `task_markets_t[t].total_escrow = K`; asserts `total_supply_micro` returns K (not 2K).

### 3.3 New TypedTx variants (Atom 3)

`TaskOpenTx` schema per charter § 4.1. `TaskOpenSigningPayload` excludes the `signature` field (8 fields → 8 fields without sig). New domain prefix `b"turingosv4.agent_sig.task_open.v1"`.

`EscrowLockTx` schema per charter § 4.1. `EscrowLockSigningPayload` excludes the `signature` field (7 fields → 6 fields without sig). New domain prefix `b"turingosv4.agent_sig.escrow_lock.v1"`.

`TypedTx` enum (`src/state/typed_tx.rs:608-616`) extension:

```rust
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),
    TaskOpen(TaskOpenTx),         // NEW
    EscrowLock(EscrowLockTx),     // NEW
}
```

`TxKind` enum at `src/bottom_white/ledger/transition_ledger.rs` gains `TaskOpen` + `EscrowLock` variants (additive — repr-u8 indexing must NOT shift existing variant numbers; new ones tail-append).

`HasSubmitter` impls: both new variants return `Some(self.sponsor_agent.clone())`.

`canonical_hash`'s exhaustive match (`worktx_canonical_hash` is local; no impact). All other exhaustive matches across the workspace (`tx_kind()`, `to_signing_payload()`, replay arms) close compile errors at TB-3 build time — Atom 3's first compile failure is the test that no consumer is missed.

### 3.4 Three new `TransitionError` variants (Atom 3)

```rust
// src/state/typed_tx.rs::TransitionError additions
TaskAlreadyOpen,
TaskNotOpen,
InsufficientBalance,
```

Display arms (`src/state/typed_tx.rs:790-816`):

```rust
TransitionError::TaskAlreadyOpen => write!(f, "task market already open for task_id"),
TransitionError::TaskNotOpen => write!(f, "no open task market for task_id"),
TransitionError::InsufficientBalance => write!(f, "balance below required debit amount"),
```

`L4ERejectionClass` mapping (`src/state/sequencer.rs::rejection_class_for`):

```rust
TE::TaskAlreadyOpen => RC::PolicyViolation,
TE::TaskNotOpen => RC::EscrowMissing,           // semantic re-use: no open task = no funded admission
TE::InsufficientBalance => RC::InsufficientBalance,  // NEW L4E variant — see §3.5
```

`public_summary_for` arms:

```rust
TransitionError::TaskAlreadyOpen => Some("task_already_open".into()),
TransitionError::TaskNotOpen => Some("task_not_open".into()),
TransitionError::InsufficientBalance => Some("insufficient_balance".into()),
```

### 3.5 New `L4ERejectionClass::InsufficientBalance` variant (Atom 3)

```rust
// src/bottom_white/ledger/rejection_evidence.rs:56-66
#[repr(u8)]
pub enum RejectionClass {
    PredicateFailed = 0,
    PolicyViolation = 1,
    EscrowMissing = 2,
    InvariantViolation = 3,
    InsufficientBalance = 4,    // NEW (additive; preserves canonical encoding for {0..3})
}
```

Add a `public_summary` arm `"insufficient_balance"` if the writer's summary table at `:316` enumerates classes.

P4 Information Loom rationale (Art. III.4 Goodhart shielding via discriminator richness): "余额不足" / "无任务 escrow" / "无 stake" / "违反守恒" / "其他 policy 违规" must surface as 5 separable signals. Folding `InsufficientBalance` into `PolicyViolation` collapses 2 → 1 and silently de-discriminates clusterer signal.

### 3.6 `TaskOpen` dispatch arm (Atom 4)

```rust
// src/state/sequencer.rs::dispatch_transition — TaskOpen arm (NEW)
TypedTx::TaskOpen(open) => {
    // Step 1: parent-root match
    if open.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent);
    }
    // Step 2: idempotency
    if q.economic_state_t.task_markets_t.0.contains_key(&open.task_id) {
        return Err(TransitionError::TaskAlreadyOpen);
    }
    // Step 3: q_next — insert TaskMarketEntry with total_escrow=0
    let mut q_next = q.clone();
    let entry = TaskMarketEntry {
        publisher: open.sponsor_agent.clone(),
        total_escrow: MicroCoin::zero(),
        escrow_lock_tx_ids: BTreeSet::new(),
        verifier_quorum: open.verifier_quorum,
        max_reuse_royalty_fraction_basis_points: open.max_reuse_royalty_fraction_basis_points,
        settlement_rule_hash: open.settlement_rule_hash,
    };
    q_next.economic_state_t.task_markets_t.0.insert(open.task_id.clone(), entry);

    // Step 4: monetary invariants (trivial — no money moved)
    assert_no_post_init_mint(tx, q).map_err(|_| TransitionError::MonetaryInvariantViolation)?;
    assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
        .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

    // Step 5: state_root advance
    q_next.state_root_t = task_open_accept_state_root(&q.state_root_t, tx);

    Ok((q_next, SignalBundle::default()))
}
```

New domain const + helper:

```rust
pub(crate) const TASK_OPEN_DOMAIN_V1: &[u8] = b"turingosv4.task_open.accept.v1";

pub fn task_open_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let work_digest = worktx_canonical_hash(tx);  // generic over TypedTx
    let mut h = Sha256::new();
    h.update(TASK_OPEN_DOMAIN_V1);
    h.update(prev.0);
    h.update(work_digest.0);
    Hash::from_bytes(h.finalize().into())
}
```

(Note: `worktx_canonical_hash` is mis-named; it's actually a generic `TypedTx` canonical hash. TB-3 keeps the function as-is — renaming is a future hygiene patch — but the domain prefix `b"turingosv4.worktx.canonical_hash.v1"` is correct because it signs the entire wire-encoded TypedTx, not just WorkTx specifically.)

### 3.7 `EscrowLock` dispatch arm (Atom 5)

```rust
TypedTx::EscrowLock(lock) => {
    // Step 1: parent-root match
    if lock.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent);
    }
    // Step 2: target task exists
    if !q.economic_state_t.task_markets_t.0.contains_key(&lock.task_id) {
        return Err(TransitionError::TaskNotOpen);
    }
    // Step 3: sponsor solvency
    let sponsor_bal = q.economic_state_t.balances_t.0.get(&lock.sponsor_agent)
        .copied().unwrap_or(MicroCoin::zero());
    if sponsor_bal.micro_units() < lock.amount.micro_units() {
        return Err(TransitionError::InsufficientBalance);
    }
    // Step 4: q_next — atomic balance → escrow transfer + cache update
    let mut q_next = q.clone();
    let new_bal = MicroCoin::from_micro_units(sponsor_bal.micro_units() - lock.amount.micro_units())
        .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
    q_next.economic_state_t.balances_t.0.insert(lock.sponsor_agent.clone(), new_bal);
    q_next.economic_state_t.escrows_t.0.insert(
        lock.tx_id.clone(),
        EscrowEntry {
            amount: lock.amount,
            depositor: lock.sponsor_agent.clone(),
            task_id: lock.task_id.clone(),
        },
    );
    // Cache update — total_escrow + escrow_lock_tx_ids
    let entry = q_next.economic_state_t.task_markets_t.0.get_mut(&lock.task_id)
        .expect("task verified to exist at step 2");
    entry.total_escrow = MicroCoin::from_micro_units(
        entry.total_escrow.micro_units() + lock.amount.micro_units()
    ).map_err(|_| TransitionError::MonetaryInvariantViolation)?;
    entry.escrow_lock_tx_ids.insert(lock.tx_id.clone());

    // Step 5: monetary invariants (debit = credit; CTF conserved)
    assert_no_post_init_mint(tx, q).map_err(|_| TransitionError::MonetaryInvariantViolation)?;
    assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
        .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
    assert_task_market_total_escrow_matches_locks(&q_next.economic_state_t, &lock.task_id)
        .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

    // Step 6: state_root advance
    q_next.state_root_t = escrow_lock_accept_state_root(&q.state_root_t, tx);

    Ok((q_next, SignalBundle::default()))
}
```

### 3.8 `WorkTx` arm refactor — bridge deletion + structural admission + lock-on-accept (Atom 6)

**Delete**: `src/state/sequencer.rs:197-215` (the bridge body AND the deletion-target comment at `:205`).

**Replace** Step 5 escrow gate:

```rust
// NEW (replacing the bridge):
let market = q.economic_state_t.task_markets_t.0.get(&work.task_id);
let has_escrow = market.map_or(false, |m| m.total_escrow.micro_units() > 0);
if !has_escrow {
    return Err(TransitionError::EscrowMissing);
}
```

**Insert NEW Step 6 (solver solvency)**:

```rust
let solver_bal = q.economic_state_t.balances_t.0.get(&work.agent_id)
    .copied().unwrap_or(MicroCoin::zero());
if solver_bal.micro_units() < work.stake.micro_units() {
    return Err(TransitionError::InsufficientBalance);
}
```

**Modify q_next construction** (was: `let mut q_next = q.clone();` then only state_root_t changed):

```rust
let mut q_next = q.clone();
// NEW: lock-on-accept stake commitment per WP § 18 Inv 5
let new_bal = MicroCoin::from_micro_units(solver_bal.micro_units() - work.stake.micro_units())
    .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
q_next.economic_state_t.balances_t.0.insert(work.agent_id.clone(), new_bal);
q_next.economic_state_t.stakes_t.0.insert(
    work.tx_id.clone(),
    StakeEntry {
        amount: work.stake,
        staker: work.agent_id.clone(),
        task_id: work.task_id.clone(),
    },
);
// Existing: state_root advance via WORKTX_ACCEPT_DOMAIN_V1
q_next.state_root_t = worktx_accept_state_root(&q.state_root_t, tx);
```

**Conservation check** (existing call site at `:238-243`) now does real work — debit-to-stakes invariant. With economic state mutating, `assert_total_ctf_conserved(q, q_next, &[])` is the primary CTF gate on the runtime spine, no longer a no-op.

**Rejection semantics** (per charter § 3.4 + user verdict #14): `apply_one`'s rejection path (TB-2 Atom 4) was already correct — a rejected WorkTx returns `Err(TransitionError::*)`, the q_next constructed above is discarded, no L4 row is appended, only L4.E. **Verify**: rejected path takes `q` (not `q_next`) into the rejection writer call, so no economic mutation leaks.

### 3.9 Untouched arms

`Verify`, `Challenge`, `Reuse`, `FinalizeReward`, `TaskExpire`, `TerminalSummary` all remain `NotYetImplemented`. RSP-2/3+ scope.

---

## 4. Parallel-branch plan (Phase 1)

### A branch — `experiment/tb3-rsp1-formal-tx-surface`

```bash
git worktree add .claude/worktrees/stepb-tb3-rsp1-formal-tx-surface -b experiment/tb3-rsp1-formal-tx-surface
```

Atomic commits per § 7 of charter (Atoms 2 → 8). Each atom: compile-green + named tests pass + `--smoke` passes.

### B branch — baseline (control)

main HEAD `a82f73e` (TB-2 ship merge).

### Acceptance gate (end of Atom 7)

- All 22+ TB-3 tests green (T1-T5 + U4-U11 + I20-I30 + bridge invariant + 3 monetary tests).
- `cargo test --workspace` zero failures.
- `cargo test --lib boot` PASS (Trust Root manifest rehashed for sequencer.rs / typed_tx.rs / q_state.rs / monetary_invariant.rs).
- `--smoke` PASS (Phase-C living regression — pipeline-liveness on `mathd_algebra_107` × oneshot × deepseek-chat; expect `prompt_context_hash="a1f43584a17d1226"` bit-identical to TB-1 Day-1 + TB-2 ship).
- `bridge_pattern_does_not_resurrect_in_src` PASS.

---

## 5. Acceptance battery (~22 tests)

### 5.1 In-crate unit tests — `src/state/typed_tx.rs::tests` (5 tests)

| ID | Name | Asserts |
|---|---|---|
| T1 | `task_open_tx_canonical_digest_is_deterministic` | re-encode of same TaskOpenTx yields identical 32-byte digest |
| T2 | `escrow_lock_tx_canonical_digest_is_deterministic` | same, EscrowLockTx |
| T3 | `task_open_signing_payload_excludes_signature` | `TaskOpenSigningPayload` has 8 fields (vs 9 in TaskOpenTx) |
| T4 | `escrow_lock_signing_payload_excludes_signature` | `EscrowLockSigningPayload` has 6 fields (vs 7 in EscrowLockTx) |
| T5 | `transition_error_display_covers_3_new_variants` | Display impl produces non-empty unique strings for `TaskAlreadyOpen` / `TaskNotOpen` / `InsufficientBalance` |

### 5.2 In-crate unit tests — `src/state/sequencer.rs::tests` mod tb3_runtime_boundary (8 tests)

| ID | Name | Asserts |
|---|---|---|
| U4 | `dispatch_task_open_inserts_task_market_entry` | post-dispatch q_next has new entry; balances unchanged; total_escrow=0 |
| U5 | `dispatch_task_open_rejects_when_already_open` | second TaskOpen for same task_id returns `Err(TaskAlreadyOpen)` |
| U6 | `dispatch_escrow_lock_debits_balance_credits_escrow_updates_total` | balance, escrow, total_escrow, escrow_lock_tx_ids all coherent post-dispatch |
| U7 | `dispatch_escrow_lock_rejects_when_task_not_open` | EscrowLock to unknown task → `Err(TaskNotOpen)` |
| U8 | `dispatch_escrow_lock_rejects_when_insufficient_balance` | sponsor balance < amount → `Err(InsufficientBalance)` |
| U9 | `dispatch_worktx_admission_via_formal_surface_no_bridge` | predicate-passing WorkTx after open+lock+balance setup is admitted; no bridge pattern in admission path |
| U10 | `dispatch_worktx_rejects_when_solver_balance_lt_stake` | balance < stake → `Err(InsufficientBalance)` |
| U11 | `dispatch_worktx_accept_debits_balance_credits_stakes` | post-accept: balance debited by stake; stakes_t[work.tx_id] populated with task_id; CTF conserved |

### 5.3 Integration tests — `tests/tb_3_rsp1_formal_surface.rs` (11 tests)

| ID | Name | Asserts |
|---|---|---|
| I20 | `submit_task_open_tx_appends_to_canonical_l4` | Sequencer::submit + run → 1 canonical L4 row; 0 L4.E |
| I21 | `submit_escrow_lock_tx_appends_to_canonical_l4` | ditto for EscrowLock |
| I22 | `escrow_lock_atomic_balance_to_escrow_transfer` | Σ balances + Σ escrows is invariant; cache=truth holds |
| I23 | `submit_worktx_via_formal_surface_advances_state_root_and_locks_stake` | full happy path: open→lock→work; state_root + ledger_root + logical_t advance; balance debited; stakes_t populated |
| I24 | `submit_worktx_without_task_open_appends_l4e_task_not_open` | WorkTx with no TaskOpen → 1 L4.E TaskNotOpen-class row, 0 L4 row |
| I25 | `submit_worktx_without_escrow_lock_appends_l4e_escrow_missing` | TaskOpen + WorkTx (no EscrowLock) → L4.E EscrowMissing |
| I26 | `submit_worktx_with_insufficient_solver_balance_appends_l4e_insufficient_balance` | solver under-funded → L4.E InsufficientBalance |
| I27 | `submit_worktx_with_zero_stake_appends_l4e_stake_insufficient` | TB-2 inheritance: stake==0 still routes to L4.E StakeInsufficient |
| I28 | `rejected_worktx_does_not_change_balances_escrows_stakes` | per charter § 3.4 + user verdict #14: full pre/post Q_t economic-state equality on rejection |
| I29 | `replay_from_l4_only_reconstructs_economic_state` | extends TB-2 I13 to 5-holding economic state including total_escrow re-derivation |
| I30 | `property_no_sequence_violates_total_ctf_conservation` | proptest 1000 sequences over {TaskOpen, EscrowLock, WorkTx (predicate-passing or failing)} preserve CTF + cache=truth at every accepted step |

### 5.4 Bridge invariant test — `tests/tb_3_bridge_deletion_invariant.rs` (1 test)

```rust
#[test]
fn bridge_pattern_does_not_resurrect_in_src() {
    let forbidden = "TxId(work.task_id.0.clone())";
    let hits = scan_rs_files_recursive("src/", forbidden);
    assert!(
        hits.is_empty(),
        "bridge pattern resurrected in src/:\n{}",
        hits.join("\n")
    );
}
// Helper: `std::fs::read_dir` recursive walk; reads each `.rs` file; collects
// lines containing the forbidden literal. Excludes `tests/` (this file lives
// there; self-match would trigger). Excludes nothing else.
```

Positive control: a separate `#[test]` constructs a known-clean snippet ("`if true { 1 } else { 0 }`") and asserts the scanner finds it in a temp file — verifies the scanner works.

### 5.5 Monetary invariant tests — `src/economy/monetary_invariant.rs::tests` (3 tests)

| ID | Name | Asserts |
|---|---|---|
| M1 | `ctf_counts_all_five_holding_subindexes` | rename + rewrite of TB-1's `_six_` test; bounty term dropped; total = 5 holdings |
| M2 | `task_market_total_escrow_matches_sum_of_escrow_locks` | NEW invariant per § 3.2 |
| M3 | `total_supply_does_not_double_count_total_escrow` | NEW; constructs paired `escrows_t.amount=K` + `task_markets_t.total_escrow=K`; asserts `total_supply_micro` = K not 2K |

### 5.6 Test fixtures

All TB-3 fixtures construct EconomicState via accepted Sequencer::submit (TaskOpen + EscrowLock) — never by direct mutation. Per charter § 5 #13 ghost-liquidity red line.

The TB-2 `tests/tb_2_runtime_boundary.rs` fixtures that seed `task_markets_t` directly need ONE-TIME migration: replace each direct mutation with a paired `TaskOpenTx` + `EscrowLockTx` submission. This is part of Atom 6 scope (the WorkTx arm refactor breaks pre-existing fixtures that relied on bridge semantics; the cleanest fix is to migrate them through the formal surface).

---

## 6. Frozen analyzer (Phase 2)

Phase 2 statistical A/B is **not** required for TB-3 — there is no MiniF2F-level capability surface change. TB-3 is a pure structural / institutional change. Phase 2 reduces to:

- TB-3 acceptance battery (§ 5) on experiment branch — required green.
- TB-3 acceptance battery on main (control) — N/A (tests don't exist on main; new test surface).
- `--smoke` on both branches — required identical `prompt_context_hash`.
- `cargo test --workspace` on both branches — both must be zero-failure.

If any TB-3 acceptance test passes on experiment branch but `--smoke` regresses or main `cargo test --workspace` regresses → revert.

---

## 7. Verdict and merge path (Phase 3)

- Treatment win (acceptance battery green + smoke identical + cargo test green + Phase-1c dual external audit PASS/PASS or substance-PASS per `feedback_dual_audit_conflict`):
  - `git merge experiment/tb3-rsp1-formal-tx-surface --no-ff` on main.
  - Update `handover/tracer_bullets/TB_LOG.tsv` row TB-3: `active → shipped` with `ship_commits` range.
  - Update `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` "TB-3 SHIPPED" log section.
  - Update `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` P3 `Current state` to reflect formal-surface RSP-1.
  - Update `docs/economics.md` to remove "New tx: yes_stake_tx" wording (replace with WP-canonical mapping note).
- Treatment lose / inconclusive:
  - `git tag archive/tb3-rsp1-formal-tx-surface_2026-04-30 experiment/tb3-rsp1-formal-tx-surface`.
  - `OBS_TB-3_FAILED.md` per TB methodology v2.
  - No commit to main; charter must change before retry.

---

## 8. Forbidden in this STEP_B (red lines — operational; charter § 5 is authoritative)

The 15 charter § 5 red lines apply verbatim. Operational re-statement of the highest-criticality ones for the auditor:

1. **No `YesStakeTx` variant** under any name (per charter § 3.1; auditor MUST verify the diff has exactly 2 new TypedTx variants, not 3).
2. **No `WorkTx` schema bump** (`src/state/typed_tx.rs:222-236` field count unchanged at 11 wire fields + TxStatus elision; `WorkSigningPayload` field count unchanged).
3. **No `task_markets_t.total_escrow` term in `total_supply_micro`** (auditor MUST grep the diff for the symbol; absence is the gate).
4. **No L4.E mutation of economic_state** (auditor MUST verify rejection path in `apply_one` does not call `q_w.write()` to mutate balances/escrows/stakes/task_markets; only the canonical-accept path mutates Q_t).
5. **No bridge resurrection** (auditor MUST verify `bridge_pattern_does_not_resurrect_in_src` passes; spot-check the diff for `TxId(work.task_id.0.clone())` or any morally-equivalent synthetic-ID-from-TaskId construction).
6. **No `economy::ledger::AcceptedLedger` use as production accepted spine** (TB-2 #6 inherited).
7. **No non-empty `exempt_tx_kinds`** at runtime — production `assert_total_ctf_conserved(before, after, &[])` only.
8. **No new EconomicState sub-fields** (the 9-sub-field invariant `q_state.rs::tests::economic_state_has_nine_sub_fields` MUST still pass).
9. **No direct `EconomicState` mutation in test fixtures** for setup (per charter § 5 #13 ghost-liquidity red line; fixtures use accepted-tx submission).

---

## 9. Pointers

- Charter v2: `handover/tracer_bullets/TB-3_charter_2026-04-30.md`
- Predecessor STEP_B (TB-2 v3, completed cycle): `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`
- Predecessor merged audit: `handover/audits/DUAL_AUDIT_TB_2_PHASE1C_VERDICT_2026-04-30.md`
- Roadmap canonical (P3 + § 6 RSP-N + § 11 deps + § 17.6 layered ordering): `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
- Whitepaper canonical (§ 14.1 inline-stake + § 5 Layer 4 + § 18 Inv 5 + § 19 RSP-1 + § 17.6): `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md`
- Whitepaper economic supplement (§ 7 + § 18 + § 19 + § 21): `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md`
- L4 / L4.E decision record: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- STEP_B protocol: `handover/ai-direct/STEP_B_PROTOCOL.md`
- C-031 (institution > tuning): `cases/C-031_institution_over_tuning.yaml`
- C-001 (zero-tolerance post-init mint): `cases/C-001_post_genesis_minting.yaml`
- Constitution Art 0.2 (Tape Canonical / derived-view 守恒测试 contract): `constitution.md` § 0.2

---

## 10. v1 (this revision) — change log against charter v2

This is preflight v1 (no prior preflight revisions for TB-3). It mirrors charter v2 structure 1:1:
- charter § 0 → preflight § 0 + § 2
- charter § 1 → preflight § 5 (test mapping)
- charter § 3 (decision blocks) → preflight § 2 (Why necessary) + § 3 (snippets)
- charter § 4 (build surface) → preflight § 3 (line-grounded snippets)
- charter § 5 (forbidden) → preflight § 8 (operational restatement)
- charter § 6 (Day-1) → preflight § 4 (Phase 1 plan)
- charter § 7 (atom sequence) → preflight § 4 + § 7
- charter § 8 (proofs) → preflight § 5 + § 7
- charter § 9 (audit gate) → preflight § 7
- charter § 10 (resolved scope-review) → preflight implicit via § 3 snippets

Two charter decisions worth highlighting for the auditor:

- **lock-on-accept** (charter § 3.4): the WorkTx accept arm now mutates economic_state (balance debit + stakes_t insert). This is a NEW q_next mutation vs TB-2's no-op. The `assert_total_ctf_conserved` call at `sequencer.rs:238-243` becomes the primary runtime CTF gate; it was effectively a guard-against-future-regressions in TB-2 but is now the live invariant check.
- **5-holding migration** (charter § 3.2): `monetary_invariant.rs` reduces holding count from 6 to 5. This is independently verifiable: any pre-TB-3 `total_supply_micro` invocation that depended on `task_markets_t.bounty` being nonzero will see a different total. The TB-1 boot test that asserts genesis `total_supply_micro == 0` (`monetary_invariant.rs:355`) is unaffected because genesis has no `task_markets_t` entries.
