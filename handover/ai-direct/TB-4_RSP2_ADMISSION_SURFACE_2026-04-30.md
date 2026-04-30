# TB-4 STEP_B Phase-0 Preflight — RSP-2 Admission Surface (line-grounded)

**Date**: 2026-04-30
**Status**: Atom 1 deliverable; line-grounded vs main HEAD `da4c67a` + experiment HEAD `cfc81de` (charter + book-keeping)
**Charter**: `handover/tracer_bullets/TB-4_charter_2026-04-30.md` DRAFT v2
**Directive**: `handover/directives/2026-04-30_TB4_directive.md`
**Templated on**: `handover/ai-direct/TB-3_RSP1_FORMAL_TX_SURFACE_2026-04-30.md` (TB-3 preflight v1 shape)

---

## §0 Why this preflight exists

Per CLAUDE.md "Code Standard" + STEP_B_PROTOCOL.md, any change to `src/state/sequencer.rs` (added to STEP_B-restricted list per TB-2 P1-A) requires a parallel-branch experimental write with a Phase-0 necessity audit. TB-4 also touches `src/state/typed_tx.rs` (institutional change per C-031: 4 new TransitionError variants + 2 schema bumps) and `src/state/q_state.rs` (additive `target_work_tx` on `ChallengeCase`). All three are bundled in this preflight.

This preflight pins **exact line refs against main HEAD `da4c67a`** so that Phase-1c diff audit + Atom 8 self-audit can verify each change is at its declared site, no scope creep.

---

## §1 Scope summary (binding to charter v2)

```text
Touched files (3):
  src/state/typed_tx.rs        — 2 schema bumps + 4 new TransitionError variants + Display arms + 2 SigningPayload field-count bumps + 2 new fixture fns + 5 new tests + 2 golden digest constant rotations
  src/state/q_state.rs         — ChallengeCase: +target_work_tx (additive serde-default); Default impl extension
  src/state/sequencer.rs       — 2 new state-root domain constants + 2 new helpers + Verify dispatch arm + Challenge dispatch arm + 10 new in-crate unit tests; rejection_class_for / public_summary_for table extension

Untouched (Phase-1c verifies absence of touch):
  src/economy/monetary_invariant.rs      (5-holding count stays)
  src/bottom_white/ledger/transition_ledger.rs   (TxKind has Verify + Challenge variants from CO1.1.4-pre1)
  src/bottom_white/ledger/rejection_evidence.rs  (no new RejectionClass variants)
  src/bottom_white/cas/*                          (no schema changes)
  src/kernel.rs / src/bus.rs / src/sdk/tools/wallet.rs   (no edits)

New files (3 + smoke evidence dir):
  tests/tb_4_rsp2_admission_surface.rs    (12 integration tests I31-I40 + I43 + I44)
  tests/tb_4_replay_property.rs           (I41 replay extension + I42 proptest)
  handover/audits/RECURSIVE_AUDIT_TB_4_2026-04-30.md   (Atom 8 self-audit)
  handover/evidence/tb_4_smoke_2026-04-30/             (Atom 8 真实烟测 with elevated MAX_TX)
```

---

## §2 typed_tx.rs — schema bump (Atom 2)

### §2.1 VerifyTx — add `parent_state_root`

**Current shape** (`src/state/typed_tx.rs:240-248`):

```rust
pub struct VerifyTx {
    pub tx_id: TxId,                       //  1
    pub target_work_tx: TxId,              //  2
    pub verifier_agent: AgentId,           //  3
    pub bond: StakeMicroCoin,              //  4
    pub verdict: VerifyVerdict,            //  5
    pub signature: AgentSignature,         //  6
    pub timestamp_logical: u64,            //  7
}
```

**Atom 2 shape** (insert `parent_state_root` as field #2):

```rust
pub struct VerifyTx {
    pub tx_id: TxId,                       //  1
    pub parent_state_root: Hash,           //  2  TB-4 NEW
    pub target_work_tx: TxId,              //  3
    pub verifier_agent: AgentId,           //  4
    pub bond: StakeMicroCoin,              //  5
    pub verdict: VerifyVerdict,            //  6
    pub signature: AgentSignature,         //  7
    pub timestamp_logical: u64,            //  8
}
```

**Why field #2**: matches WorkTx field-order convention (WorkTx field#3 = parent_state_root, but WorkTx has task_id at #2; for VerifyTx there is no task_id — target_work_tx serves a similar anchoring role, but the parent_state_root must precede it for canonical-encoding determinism per spec §1.2 and the StaleParent gate is the most upstream check). Field-renumbering is wire-breaking, but pre-TB-4 has zero accepted L4 rows of VerifyTx (dispatch arm was `NotYetImplemented`), so the bump is harmless.

### §2.2 ChallengeTx — add `parent_state_root`

**Current shape** (`src/state/typed_tx.rs:259-267`):

```rust
pub struct ChallengeTx {
    pub tx_id: TxId,                       //  1
    pub target_work_tx: TxId,              //  2
    pub challenger_agent: AgentId,         //  3
    pub stake: StakeMicroCoin,             //  4
    pub counterexample_cid: Cid,           //  5
    pub signature: AgentSignature,         //  6
    pub timestamp_logical: u64,            //  7
}
```

**Atom 2 shape**:

```rust
pub struct ChallengeTx {
    pub tx_id: TxId,                       //  1
    pub parent_state_root: Hash,           //  2  TB-4 NEW
    pub target_work_tx: TxId,              //  3
    pub challenger_agent: AgentId,         //  4
    pub stake: StakeMicroCoin,             //  5
    pub counterexample_cid: Cid,           //  6
    pub signature: AgentSignature,         //  7
    pub timestamp_logical: u64,            //  8
}
```

### §2.3 SigningPayload field-count bumps (6 → 7)

**Current `VerifySigningPayload`** (`src/state/typed_tx.rs:480-489`):

```rust
pub struct VerifySigningPayload {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,
    pub verdict: VerifyVerdict,
    pub timestamp_logical: u64,
}
```

**Atom 2 shape**:

```rust
pub struct VerifySigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,           // TB-4 NEW
    pub target_work_tx: TxId,
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,
    pub verdict: VerifyVerdict,
    pub timestamp_logical: u64,
}
```

Same shape mod for `ChallengeSigningPayload` (`:497-506`).

`VerifyTx::to_signing_payload()` (`:623-633`) + `ChallengeTx::to_signing_payload()` (`:636-646`) gain the field copy.

### §2.4 Domain prefixes — UNBUMPED

`DOMAIN_AGENT_VERIFY` and `DOMAIN_AGENT_CHALLENGE` (defined elsewhere in typed_tx.rs as `b"turingosv4.agent_sig.verify.v1"` / `b"turingosv4.agent_sig.challenge.v1"`) stay at `.v1`. Justification: pre-TB-4 has no signed VerifyTx/ChallengeTx in any production tape (their dispatch was `NotYetImplemented`), so the v1 prefix is semantically unused-historical; reusing it after the schema bump does NOT violate any signature-replay assumption (an attacker would need a private key controlling a `verifier_agent` AgentId to forge any message; the schema layout determines which message is signed, and there are no historical signatures to confuse with).

### §2.5 Golden digest rotation

Two `EXPECTED_HEX_*` constants (`src/state/typed_tx.rs:1674-1677`) rotate:

```rust
const EXPECTED_HEX_VERIFY: &str =
    "425b9bd7e99c427b3b7934d45a00dee3d66fc346deed72ec307de01bb3f1db99";  // pre-TB-4 (6-field signing payload)
const EXPECTED_HEX_CHALLENGE: &str =
    "c90be7617e9aba5a70dc8d625e654c1c712403aaf47e7734497fc0e909e8f788";  // pre-TB-4
```

Atom 2 recomputes both via the new fixture (with `parent_state_root` set) and updates the constants. The `golden_verify_tx_digest` and `golden_challenge_tx_digest` tests at `:1694-1704` assert against the new values; the tests themselves are unchanged in shape.

### §2.6 Fixture updates

**`fixture_verify_tx`** (`:1131-1141`) gains:

```rust
parent_state_root: h(0x66),  // TB-4 NEW
```

**`fixture_challenge_tx`** (`:1143-1153`) gains:

```rust
parent_state_root: h(0x77),  // TB-4 NEW
```

(Distinct nibbles per fixture so digests diverge from each other.)

### §2.7 New tests T1-T4 (Atom 2)

Mirror the TB-3 T1-T4 shape at `:1758-1793`. Tests added:

```rust
fn verify_tx_canonical_digest_includes_parent_state_root() { /* T1 */ }
fn challenge_tx_canonical_digest_includes_parent_state_root() { /* T2 */ }
fn verify_signing_payload_excludes_signature_field_count_7() { /* T3 — 8-field tx → 7-field payload */ }
fn challenge_signing_payload_excludes_signature_field_count_7() { /* T4 — 8-field tx → 7-field payload */ }
```

T1/T2 each build two fixtures with different `parent_state_root` values and assert their digests differ (proves the field is in the canonical bytes).

---

## §3 typed_tx.rs — TransitionError extension (Atom 3)

### §3.1 4 new variants + 1 reserved

**Current** (`src/state/typed_tx.rs:855-953`):

```rust
pub enum TransitionError {
    StaleParent,
    SignatureInvalid,
    InvalidSystemSignature,
    StakeInsufficient,
    TargetWorkTxNotFound,             // ← reserved (declared since CO1.1.4-pre1, never emitted)
    TargetWorkTxNotVerifiable,        // ← reserved
    ParentNotAcceptedYet,
    AcceptancePredicateFailed(PredicateId),
    VerificationPredicateFailed(PredicateId),
    SettlementPredicateFailed(PredicateId),
    ChallengeWindowClosed,
    ChallengeWindowStillOpen,
    AlreadySlashed,
    CounterexampleInsufficient,
    ToolNotInRegistry,
    ToolCreatorMismatch,
    ClaimNotFound,
    TaskNotFound,
    TaskNotExpired,
    TaskHasOpenClaim,
    TerminalSummaryNotApplicable,
    EscrowMissing,                   // TB-2
    MonetaryInvariantViolation,      // TB-2
    TaskAlreadyOpen,                 // TB-3
    TaskNotOpen,                     // TB-3
    InsufficientBalance,             // TB-3
    NotYetImplemented,
}
```

**Atom 3 additions** (4 new actively-emitted + 0 reserved-rename — existing reserved variants reused):

```rust
    // ── TB-4 RSP-2 admission (charter § 3.8 + directive Q3) ─────────────────
    /// VerifyTx.bond == 0. Distinct from StakeInsufficient (used for ChallengeTx
    /// stake==0 to keep WP-economic § 7 "Verifier 抵押 bond" naming honest).
    /// Maps to L4ERejectionClass::PolicyViolation per charter § 4.5.
    BondInsufficient,
    /// VerifyTx / ChallengeTx target_work_tx exists in canonical L4 but is
    /// no longer in stakes_t (target was never accepted as live, OR has been
    /// resolved/finalized in a future RSP-3 path; in TB-4 minimum scope the
    /// two cases collapse since RSP-3 has not yet introduced finalize-removes-
    /// stakes_t logic). Distinct from TargetWorkTxNotFound (reserved for the
    /// "tx_id has no L4 row at all" case, which TB-4 dispatch_transition cannot
    /// distinguish from a Q_t-only viewpoint and so is unreachable in TB-4).
    /// Distinct from TargetWorkTxNotVerifiable (reserved for "target tx_id
    /// exists but is not a WorkTx type", e.g. points at a TaskOpen — also
    /// unreachable in TB-4 since dispatch arm reads stakes_t[target] directly).
    /// Maps to L4ERejectionClass::PolicyViolation per charter § 4.5.
    TargetWorkInactive,
    /// ChallengeTx.counterexample_cid == Cid::ZERO. Sanity gate against empty
    /// challenges. Distinct from MalformedPayload (which would reject earlier
    /// at deserialize time). P4 Information Loom needs this discriminator
    /// per directive Q7.
    EmptyCounterexample,
```

Plus reuse:
- `TargetWorkTxNotFound` — declared but unused in TB-4 (reserved per § 3.8); kept for RSP-3 distinguish.
- `TargetWorkTxNotVerifiable` — declared but unused in TB-4 (reserved per § 3.8); kept for RSP-3 distinguish.
- `StakeInsufficient` — re-used for ChallengeTx.stake == 0 (existing variant; semantic match).
- `InsufficientBalance` — re-used for VerifyTx/ChallengeTx balance debit failure (TB-3 variant).
- `StaleParent` — re-used (pre-TB-2).

**Display arm additions** (`src/state/typed_tx.rs:957-985`):

```rust
    Self::BondInsufficient => write!(f, "verifier bond insufficient"),
    Self::TargetWorkInactive => write!(f, "target work_tx not in stakes_t (never accepted live, or already resolved)"),
    Self::EmptyCounterexample => write!(f, "challenge counterexample_cid is empty / zero"),
```

### §3.2 Test T5

Mirrors TB-3 T5 (`:1799-1813`):

```rust
fn transition_error_display_covers_3_new_variants_plus_reserved() {
    // 3 new actively-emitted variants
    let s_bond = format!("{}", TransitionError::BondInsufficient);
    let s_inactive = format!("{}", TransitionError::TargetWorkInactive);
    let s_empty = format!("{}", TransitionError::EmptyCounterexample);
    // 2 reserved variants whose Display already exists
    let s_not_found = format!("{}", TransitionError::TargetWorkTxNotFound);
    let s_not_verif = format!("{}", TransitionError::TargetWorkTxNotVerifiable);
    // distinct strings
    assert!(!s_bond.is_empty() && !s_inactive.is_empty() && !s_empty.is_empty());
    assert_ne!(s_bond, s_inactive);
    assert_ne!(s_inactive, s_empty);
    assert_ne!(s_bond, s_empty);
    assert_ne!(s_not_found, s_inactive);
    assert_ne!(s_not_verif, s_inactive);
    assert!(s_bond.contains("bond"));
    assert!(s_inactive.contains("stakes_t"));
    assert!(s_empty.contains("counterexample"));
}
```

---

## §4 q_state.rs — ChallengeCase additive (Atom 3)

### §4.1 ChallengeCase shape change

**Current** (`src/state/q_state.rs:336-350`):

```rust
pub struct ChallengeCase {
    #[serde(default)]
    pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bond: MicroCoin,
    #[serde(default)]
    pub opened_at_round: u64,
}

impl Default for ChallengeCase {
    fn default() -> Self {
        Self { challenger: AgentId::default(), bond: MicroCoin::zero(), opened_at_round: 0 }
    }
}
```

**Atom 3 shape**:

```rust
pub struct ChallengeCase {
    #[serde(default)]
    pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bond: MicroCoin,
    #[serde(default)]
    pub opened_at_round: u64,
    /// TB-4 charter § 3.3 — replay-deterministic backref to target WorkTx.
    /// Required by RSP-3 settlement (routing slash/release on resolution) and
    /// by multi-challenger representability (multiple ChallengeCase rows with
    /// distinct keys may share `target_work_tx`).
    #[serde(default)]
    pub target_work_tx: TxId,
}

impl Default for ChallengeCase {
    fn default() -> Self {
        Self {
            challenger: AgentId::default(),
            bond: MicroCoin::zero(),
            opened_at_round: 0,
            target_work_tx: TxId::default(),
        }
    }
}
```

This is **entry-shape additive** (TB-3's pattern for `EscrowEntry.task_id` and `StakeEntry.task_id`), not a new EconomicState sub-field. The 9-sub-field invariant `economic_state_has_nine_sub_fields` (q_state.rs::tests) remains green.

`TxId` is already in scope (q_state.rs:69) — no import needed.

---

## §5 sequencer.rs — Verify + Challenge dispatch arms (Atoms 4-5)

### §5.1 New state-root domain constants

Insert after the TB-3 `ESCROW_LOCK_DOMAIN_V1` block at `src/state/sequencer.rs:107`:

```rust
// ────────────────────────────────────────────────────────────────────────────
// TB-4 RSP-2 — Verify + Challenge state-root domains (charter § 4.3)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-4 charter § 4.3 — Verify-accept state-root domain.
pub(crate) const VERIFY_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.verify.accept.v1";

/// TRACE_MATRIX TB-4 charter § 4.3 — Challenge-accept state-root domain.
pub(crate) const CHALLENGE_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.challenge.accept.v1";

/// TRACE_MATRIX TB-4 charter § 4.3 — interim state-root mutator on
/// `VerifyTx` accept. Mirror of `task_open_accept_state_root` shape.
pub fn verify_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(VERIFY_ACCEPT_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-4 charter § 4.3 — interim state-root mutator on
/// `ChallengeTx` accept. Mirror of `verify_accept_state_root`.
pub fn challenge_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(CHALLENGE_ACCEPT_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}
```

### §5.2 `rejection_class_for` extension

Current at `src/state/sequencer.rs:154-177`. Atom 3 extends with explicit arms:

```rust
    // TB-4 RSP-2 admission mapping (charter § 4.5):
    TE::BondInsufficient => RC::PolicyViolation,
    TE::TargetWorkInactive => RC::PolicyViolation,
    TE::EmptyCounterexample => RC::PolicyViolation,
    // TargetWorkTxNotFound + TargetWorkTxNotVerifiable already fall through
    // to PolicyViolation via the wildcard arm; explicit arms added for clarity.
    TE::TargetWorkTxNotFound => RC::PolicyViolation,
    TE::TargetWorkTxNotVerifiable => RC::PolicyViolation,
```

### §5.3 `public_summary_for` extension

Current at `src/state/sequencer.rs:185-199`:

```rust
    TransitionError::BondInsufficient => Some("bond_insufficient".into()),
    TransitionError::TargetWorkInactive => Some("target_work_inactive".into()),
    TransitionError::EmptyCounterexample => Some("empty_counterexample".into()),
    TransitionError::TargetWorkTxNotFound => Some("target_work_not_found".into()),
    TransitionError::TargetWorkTxNotVerifiable => Some("target_work_not_verifiable".into()),
```

### §5.4 Verify dispatch arm (Atom 4)

**Replace** `src/state/sequencer.rs:324`:

```rust
        TypedTx::Verify(_) => Err(TransitionError::NotYetImplemented),
```

with:

```rust
        // ──────────────────────────────────────────────────────────────────
        // TB-4 Atom 4 — Verify arm (charter § 3.4 + § 4.3).
        // Verifier puts skin in the game: bond debited from balances, locked
        // into stakes_t[verify.tx_id]. Target liveness via stakes_t lookup.
        // No verdict mutation in Q_t (verdict rides L4 only — § 3.10).
        // ──────────────────────────────────────────────────────────────────
        TypedTx::Verify(verify) => {
            // Step 1: parent-root match.
            if verify.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: bond positivity.
            if verify.bond.micro_units() == 0 {
                return Err(TransitionError::BondInsufficient);
            }
            // Step 3: liveness — target must be in stakes_t (live YES stake).
            // TB-4 minimum scope collapses TargetNotFound + TargetWorkInactive
            // to this single check (charter § 4.3 "open atom-1 design question"
            // resolution: TB-4 emits TargetWorkInactive; TargetNotFound +
            // TargetNotVerifiable are reserved for RSP-3 finalize logic).
            let target_stake = q.economic_state_t.stakes_t.0.get(&verify.target_work_tx);
            let target_stake = match target_stake {
                Some(s) => s,
                None => return Err(TransitionError::TargetWorkInactive),
            };
            // Step 4: verifier solvency.
            let verifier_bal = q.economic_state_t.balances_t.0
                .get(&verify.verifier_agent)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if verifier_bal.micro_units() < verify.bond.micro_units() {
                return Err(TransitionError::InsufficientBalance);
            }
            // Step 5: q_next — atomic balance → stakes_t transfer.
            let mut q_next = q.clone();
            let new_bal_micro = verifier_bal.micro_units() - verify.bond.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                verify.verifier_agent.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            q_next.economic_state_t.stakes_t.0.insert(
                verify.tx_id.clone(),
                crate::state::q_state::StakeEntry {
                    amount: verify.bond.0,
                    staker: verify.verifier_agent.clone(),
                    task_id: target_stake.task_id.clone(),
                },
            );
            // Step 6: monetary invariants (debit = credit).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 7: state_root advance via VERIFY_ACCEPT_DOMAIN_V1.
            q_next.state_root_t = verify_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
```

### §5.5 Challenge dispatch arm (Atom 5)

**Replace** `src/state/sequencer.rs:325`:

```rust
        TypedTx::Challenge(_) => Err(TransitionError::NotYetImplemented),
```

with:

```rust
        // ──────────────────────────────────────────────────────────────────
        // TB-4 Atom 5 — Challenge arm (charter § 3.5 + § 4.3 + § 3.9).
        // Challenger puts NO position in the market: stake debited from
        // balances, locked into challenge_cases_t[challenge.tx_id].
        // opened_at_round = q.q_t.current_round is the structural anchor (§ 3.9);
        // closure / slash / resolve are RSP-3 (§ 3.7 + § 5 #11-12).
        // ──────────────────────────────────────────────────────────────────
        TypedTx::Challenge(challenge) => {
            // Step 1: parent-root match.
            if challenge.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: stake positivity.
            if challenge.stake.micro_units() == 0 {
                return Err(TransitionError::StakeInsufficient);
            }
            // Step 3: liveness — target in stakes_t.
            if !q.economic_state_t.stakes_t.0.contains_key(&challenge.target_work_tx) {
                return Err(TransitionError::TargetWorkInactive);
            }
            // Step 4: challenger solvency.
            let challenger_bal = q.economic_state_t.balances_t.0
                .get(&challenge.challenger_agent)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if challenger_bal.micro_units() < challenge.stake.micro_units() {
                return Err(TransitionError::InsufficientBalance);
            }
            // Step 5: counterexample non-empty.
            if challenge.counterexample_cid == Cid::ZERO {
                return Err(TransitionError::EmptyCounterexample);
            }
            // Step 6: q_next — atomic balance → challenge_cases_t transfer.
            let mut q_next = q.clone();
            let new_bal_micro = challenger_bal.micro_units() - challenge.stake.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                challenge.challenger_agent.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            q_next.economic_state_t.challenge_cases_t.0.insert(
                challenge.tx_id.clone(),
                crate::state::q_state::ChallengeCase {
                    challenger: challenge.challenger_agent.clone(),
                    bond: challenge.stake.0,
                    opened_at_round: q.q_t.current_round,    // ← § 3.9 anchor
                    target_work_tx: challenge.target_work_tx.clone(),
                },
            );
            // Step 7: monetary invariants (debit = credit; challenge_cases.bond
            // is the 5th holding term).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 8: state_root advance via CHALLENGE_ACCEPT_DOMAIN_V1.
            q_next.state_root_t = challenge_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
```

### §5.6 Imports needed

At the top of sequencer.rs, after existing `use` block:

- `Cid` for the counterexample_cid comparison: `use crate::bottom_white::cas::schema::Cid;`
- `q.q_t.current_round` is the canonical Q-derived monotonic clock available from `QState` (via `AgentSwarmState.current_round` per `src/state/q_state.rs:98`). [TB-5 directive 2026-04-30 § 4 A1 patch correction: original wording said "q.q_t.current_round is already a field on QState"; that was incorrect — Q_t has no top-level `logical_t`. Atom 5 implementation correctly used `q.q_t.current_round`; this preflight wording is now reconciled with shipped code at `src/state/sequencer.rs:480`.]

---

## §6 In-crate unit tests (Atoms 4-5)

Mirror TB-3's U4-U11 shape at `src/state/sequencer.rs::tests`. 10 new tests:

```rust
// Atom 4 (Verify)
fn dispatch_verify_locks_bond_in_stakes_t_at_verify_tx_id() { /* U12 */ }
fn dispatch_verify_rejects_when_bond_zero() { /* U13 — BondInsufficient */ }
fn dispatch_verify_rejects_when_target_not_found() { /* U14 — TargetWorkInactive */ }
fn dispatch_verify_rejects_when_target_inactive() { /* U15 — same TargetWorkInactive class; same condition in TB-4 minimum scope */ }
fn dispatch_verify_rejects_when_verifier_balance_lt_bond() { /* U16 — InsufficientBalance */ }

// Atom 5 (Challenge)
fn dispatch_challenge_opens_case_with_target_back_ref_and_logical_t_anchor() { /* U17 */ }
fn dispatch_challenge_rejects_when_stake_zero() { /* U18 — StakeInsufficient */ }
fn dispatch_challenge_rejects_when_target_not_found() { /* U19 — TargetWorkInactive */ }
fn dispatch_challenge_rejects_when_counterexample_cid_zero() { /* U20 — EmptyCounterexample */ }
fn dispatch_challenge_rejects_when_challenger_balance_lt_stake() { /* U21 — InsufficientBalance */ }
```

Each test uses `dispatch_transition` directly (NOT `apply_one`) per TB-3's U4-U11 pattern — keeps unit tests fast and isolated from CAS/keypair/writer setup. Helper `seed_qstate_with_live_worktx(stakes_amount, balances)` builds a Q_t with one stakes_t entry so the liveness gate has something to find.

---

## §7 Integration tests (Atoms 4-7)

### §7.1 `tests/tb_4_rsp2_admission_surface.rs`

Mirrors `tests/tb_3_rsp1_formal_surface.rs` shape (Harness + fresh_harness). 12 tests:

```rust
// Atom 4 (Verify)
fn submit_verify_tx_appends_to_canonical_l4_and_locks_bond() { /* I31 */ }
fn verify_admission_atomic_balance_to_stakes_transfer() { /* I33 */ }
fn verify_against_inactive_target_appends_l4e_target_inactive() { /* I35 */ }
fn verify_with_zero_bond_appends_l4e_bond_insufficient() { /* I37 */ }

// Atom 5 (Challenge)
fn submit_challenge_tx_appends_to_canonical_l4_and_opens_case() { /* I32 */ }
fn challenge_admission_atomic_balance_to_challenge_cases_transfer() { /* I34 */ }
fn challenge_against_inactive_target_appends_l4e_target_inactive() { /* I36 */ }
fn challenge_with_zero_stake_appends_l4e_stake_insufficient() { /* I38 */ }

// Atom 6 (multi-challenger + window-anchor + L4.E-no-mutation)
fn multiple_challengers_same_target_all_accepted_distinct_case_rows() { /* I39 — directive Q4 binding */ }
fn rejected_verify_or_challenge_does_not_change_economic_state() { /* I40 */ }
fn challenge_window_anchor_equals_q_logical_t_at_accept() { /* I43 */ }

// Atom 7 (no-drift CI)
fn no_NoStakeTx_or_VerifierBondTx_variant_in_src() { /* I44 — directive § 5.1 */ }
```

### §7.2 `tests/tb_4_replay_property.rs`

Mirrors I29 + I30 shape from TB-3. 2 tests:

```rust
fn replay_from_l4_only_reconstructs_economic_state_with_verify_and_challenge() { /* I41 */ }
fn property_no_sequence_violates_total_ctf_conservation_with_verify_challenge() { /* I42 — proptest 1000 sequences */ }
```

---

## §8 Resolved Atom-1 design questions

### Q1 — `TargetNotFound` vs `TargetWorkInactive` in TB-4 minimum scope

**Resolution**: TB-4 emits **`TargetWorkInactive`** for the not-in-stakes_t case (only check available from pure dispatch_transition reading Q_t). `TargetWorkTxNotFound` and `TargetWorkTxNotVerifiable` stay declared as RESERVED variants in TransitionError; their Display arms exist; no code path in TB-4 emits them. RSP-3 will distinguish them when finalize-removes-stakes_t logic creates the distinction.

Charter § 4.4 already documents this resolution; preflight § 5.4 step 3 commit makes it operative.

### Q2 — L4.E `details` payload carries TransitionError variant name?

**Resolution**: confirmed by reading `src/state/sequencer.rs:808-821`:

```rust
let diag_bytes = transition_err.to_string().into_bytes();
let raw_diagnostic_cid = {
    let mut cas_w = self.cas.write()...
    Some(cas_w.put(
        &diag_bytes,
        ObjectType::Generic,
        &creator,
        rejection_logical_t,
        Some("TransitionError.display.v1".to_string()),
    )?)
};
```

The TransitionError's Display string IS persisted to CAS as `raw_diagnostic_cid` on the L4.E row. `RejectedSubmissionRecord.raw_diagnostic_cid` carries the CID; `#[serde(skip_serializing, default)]` (TB-1 P0-3) shields it from agent read view but P4 Information Loom can read it via direct CAS access.

So TB-4's clustering of multiple distinct TransitionError variants under L4ERejectionClass::PolicyViolation does NOT lose signal — P4 can re-distinguish them via CAS-fetched `raw_diagnostic_cid` payload. The L4ERejectionClass enum stays at its existing 5-variant taxonomy (PredicateFailed / PolicyViolation / EscrowMissing / InvariantViolation / InsufficientBalance) without TB-4 adding a 6th.

---

## §9 Forbidden file touches (CI-verifiable)

Atom 1 commits this preflight only; Atoms 2-7 land code. Phase-1c diff audit verifies these touch budgets:

| File | Allowed touch | Phase-1c verification |
|---|---|---|
| `src/state/typed_tx.rs` | 2 schema bumps + 4 new TransitionError variants + 2 SigningPayload field-count bumps + 5 new tests + 2 golden constant rotations | `git diff main..HEAD -- src/state/typed_tx.rs \| wc -l` ≤ 250 lines net add |
| `src/state/q_state.rs` | ChallengeCase: + target_work_tx field + Default impl extension | `git diff main..HEAD -- src/state/q_state.rs \| wc -l` ≤ 30 lines net add |
| `src/state/sequencer.rs` | 2 state-root domain consts + 2 helpers + Verify arm + Challenge arm + 5 imports + rejection_class_for + public_summary_for extension + 10 new in-crate unit tests | `git diff main..HEAD -- src/state/sequencer.rs \| wc -l` ≤ 600 lines net add |
| `src/economy/monetary_invariant.rs` | ZERO (preflight § 1) | `git diff main..HEAD -- src/economy/monetary_invariant.rs` empty |
| `src/bottom_white/ledger/rejection_evidence.rs` | ZERO | empty |
| `src/bottom_white/ledger/transition_ledger.rs` | ZERO | empty |
| `src/kernel.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs` | ZERO | empty |
| `tests/tb_4_*.rs` | NEW files (2 — admission_surface + replay_property) | new |

---

## §10 Atom-by-atom deliverables (executable)

| Atom | Files touched | Tests added | Commit subject |
|---|---|---|---|
| 0 (DONE) | charter v2, directive, TB_LOG, NOTEPAD | none | `Atom 0 — Charter v2 + PLAN sync` |
| 1 (THIS) | preflight doc | none | `Atom 1 — STEP_B Phase-0 preflight` |
| 2 | typed_tx.rs (schema bump + signing payload + golden rotations + fixture updates + T1-T4) | T1, T2, T3, T4 | `Atom 2 — VerifyTx + ChallengeTx parent_state_root schema bump` |
| 3 | typed_tx.rs (4 TransitionError variants + Display arms + T5) + q_state.rs (ChallengeCase additive) | T5 | `Atom 3 — TransitionError +4 variants + ChallengeCase additive target_work_tx` |
| 4 | sequencer.rs (Verify arm + VERIFY_ACCEPT_DOMAIN_V1 + helper + rejection_class_for/public_summary_for arms + U12-U16 + I31, I33, I35, I37) | U12, U13, U14, U15, U16, I31, I33, I35, I37 | `Atom 4 — Verify dispatch arm + apply_one accepted-spine` |
| 5 | sequencer.rs (Challenge arm + CHALLENGE_ACCEPT_DOMAIN_V1 + helper + U17-U21 + I32, I34, I36, I38) | U17, U18, U19, U20, U21, I32, I34, I36, I38 | `Atom 5 — Challenge dispatch arm + apply_one accepted-spine` |
| 6 | tests/tb_4_rsp2_admission_surface.rs (I39, I40, I43) | I39, I40, I43 | `Atom 6 — Multi-challenger + window-anchor + L4.E-no-mutation tests` |
| 7 | tests/tb_4_replay_property.rs (I41, I42) + tests/tb_4_rsp2_admission_surface.rs (I44 anti-drift) | I41, I42, I44 | `Atom 7 — Replay + property + no-drift CI tests` |
| 8 | handover/audits/RECURSIVE_AUDIT_TB_4_2026-04-30.md + handover/evidence/tb_4_smoke_2026-04-30/ | none (audit + smoke) | `Atom 8 — Recursive self-audit + 真实烟测 evidence` |
| Ship | (--no-ff merge) + post-merge book-keeping | none | `TB-4 SHIPPED — merge experiment/tb4-rsp2-admission-surface` |

Total new test count: **5 typed_tx + 10 sequencer in-crate + 12 integration = 27 new TB-4 tests**. Target post-ship: 568+/568+ cargo test green.

---

## §11 Cross-references

- Charter v2 (binding): `handover/tracer_bullets/TB-4_charter_2026-04-30.md`
- Architect directive (binding): `handover/directives/2026-04-30_TB4_directive.md`
- TB-3 preflight (template): `handover/ai-direct/TB-3_RSP1_FORMAL_TX_SURFACE_2026-04-30.md`
- TB-3 charter (precedent for atom-by-atom shape): `handover/tracer_bullets/TB-3_charter_2026-04-30.md`
- STEP_B protocol: `STEP_B_PROTOCOL.md`
- WP-vs-Roadmap reconciliation memory: `feedback_wp_vs_roadmap_reconciliation`
- 9-phase roadmap directive: `handover/directives/2026-04-29_9_phase_roadmap.md`
