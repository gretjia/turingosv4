# TB-FLOWCHART-FC2-FC3-CLOSURE — Class 4 Charter Draft v1

Date: 2026-05-25
Risk class: Class 4
Status: DRAFT — not ratified, not implemented
Source of truth: `constitution.md` only
User obligation: OBL-005

This charter specifies the closure design for the two remaining constitutional
liveness gaps that block the claim "the three constitution flowcharts are live
in production":

- FC2 map-reduce tick: `clock -> mr -> tape0/tape1`
- FC3 meta loop: `logs -> feedback -> architectAI` and
  `init -> error -> re-init -> boot`

It does not modify `constitution.md`. It does not remove active modules that
are outside the three flowcharts but needed by the working system. Those modules
remain classified in
`handover/audits/TURINGOSV4_ARCHITECTURE_LIVENESS_MAP_2026-05-25.md` as
required substrate, support invariant, product workload, or legacy candidate.

## 1. Constitutional Anchors

FC2 authority is `constitution.md:571-660`:

- `Q_t = <q_t, HEAD_t, tape_t>` and `Q_{t+1}` are the state carrier
  (`constitution.md:587-597`).
- `InitAI --x|once| mr` and `InitAI --x|once| Q0` are once-only boot edges
  (`constitution.md:632-638`).
- Runtime loop reads `tape0 & HEAD0 -> s_i`, sends `q_i & s_i -> delta`,
  then commits output to `HEAD1/tape1` through predicates/write tool
  (`constitution.md:640-653`).
- FC2 tick is explicit: `clock --> mr`, `mr ==>|map| tape0`,
  `mr ==>|reduce| tape1` (`constitution.md:656-659`).

FC3 authority is `constitution.md:826-870`:

- `constitution` and `logs` are read-only ground truth for the init/meta layer
  (`constitution.md:836-844`).
- `tools ==>|write| log` and `log ====>|archive| logs`
  (`constitution.md:855-857`, `constitution.md:865`).
- `logs -->|feedback| architectAI` (`constitution.md:866`).
- `init ==> error ==========>|re-init| boot` (`constitution.md:867`).
- `constitution -->|abide| vetoAI & architectAI` and
  `vetoAI -->|veto| architectAI` (`constitution.md:868-869`).

## 2. Current Production Reality

Boot/replay/resume are live. `build_chaintape_sequencer` writes the initial
state seed and boot-time activation; resume uses the same replay primitive as
`verify_chaintape`. L4 and L4.E are real tape surfaces.

FC2 tick is not live. The current `turingos task tick` command is a wrapper
around `TASK_RUNNER_BIN`; it is not a ChainTape-visible map/reduce transition.
The current typed tx enum has no `MapReduceTick`, the ledger `TxKind` enum has
no tick kind, and `SystemEmitCommand` has no map-reduce tick command.

FC3 feedback/reinit is not live in-process. Current ArchitectAI/Veto-AI evidence
is external governance/audit artifacts. That is useful and must be preserved as
process evidence, but it is not the runtime edge
`logs -> feedback -> architectAI`, nor the runtime edge
`error -> re-init -> boot`.

## 3. Closure Strategy

The closure must be tape-first:

- accepted FC2/FC3 closure events are typed L4 transactions;
- rejected or forged events route to L4.E and do not consume L4 logical time;
- replay reconstructs final state from `initial_q_state.json`, L4, L4.E, CAS,
  and pinned system pubkeys;
- no closure proof may depend on stdout, dashboards, handover docs, global
  latest pointers, wall clock, env, RNG, or memory-only state.

The closure should be split unless the user explicitly ratifies an all-in-one
Class 4 atom:

1. **FC2-TICK**: map-reduce tick typed tx, signing, sequencer emission,
   replay validation, and LiveNow gates.
2. **FC3-META**: meta-feedback and reinit typed txs, signing, sequencer
   emission, replay validation, and LiveNow gates.

## 4. Locked Interface Contracts

These shapes are contracts for implementation review. Field names may be
adjusted only by a new charter revision before §8 sign-off.

### 4.0 Shared Prefix Invariant Context

FC2 and FC3 both need facts that are not available inside the existing pure
`dispatch_transition(q, tx, registry, tools, ...)` function. They therefore
MUST use a shared pre-dispatch invariant layer that is called by both live
`apply_one` and replay before the candidate tx payload is written to CAS.

```rust
pub struct SystemInvariantContext<'a> {
    pub l4_prefix: &'a [LedgerEntry],
    pub l4_root_before: Hash,
    pub l4_len_before: u64,
    pub l4e_root_before: Hash,
    pub l4e_len_before: u64,
    pub cas_metadata_root_before: Hash,
    pub constitution_hash: Hash,
}
```

Live construction:

- `l4_prefix`, `l4_root_before`, and `l4_len_before` are read from the
  accepted ChainTape prefix before the candidate tx is appended.
- `l4e_root_before` and `l4e_len_before` are read from the L4.E archive prefix
  before the candidate tx is processed. The implementation atom must expose
  the L4.E chain length/root as a deterministic reader; stdout or dashboard
  counters are invalid.
- `cas_metadata_root_before` folds sorted `CasObjectMetadata::canonical_hash()`
  values visible before the candidate tx payload is stored.
- `constitution_hash` is taken from the verified trust-root manifest /
  genesis report for `constitution.md`, not from an unverified live workspace
  file.

Replay construction:

- The verifier builds the same context from `entries[..candidate_index]`,
  the L4.E JSONL prefix count/root recorded in the candidate tx, the CAS
  metadata snapshot visible before the candidate payload CID, and the pinned
  trust-root constitution hash.
- A zero-test or grep-only replay proof is invalid. Replay must recompute the
  roots and reject any candidate whose signed fields do not match the context.

### 4.1 FC2 Map-Reduce Tick

```rust
pub enum TypedTx {
    // Existing variants unchanged.
    MapReduceTick(MapReduceTickTx),
}

#[repr(u8)]
pub enum TxKind {
    // Existing discriminants unchanged.
    MapReduceTick = 20,
}

#[repr(u8)]
pub enum TickKind {
    Scheduled = 0,
}

pub struct MapReduceTickTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub tape0_root: Hash,
    pub tape0_len: u64,
    pub clock_t: u64,
    pub map_root: Hash,
    pub reduce_root: Hash,
    pub tick_kind: TickKind,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

pub struct MapReduceTickSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub tape0_root: Hash,
    pub tape0_len: u64,
    pub clock_t: u64,
    pub map_root: Hash,
    pub reduce_root: Hash,
    pub tick_kind: TickKind,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

pub enum SystemEmitCommand {
    // Existing commands unchanged.
    MapReduceTick { tick_kind: TickKind },
}
```

Rules:

- `MapReduceTick` is system-emitted only. Agent ingress rejects it.
- `tape0_root == q.ledger_root_t`.
- `tape0_len == current L4 length before the tick`.
- `clock_t == q.q_t.current_round + 1`.
- `map_root` is recomputed from the accepted L4 prefix and CAS payload
  references. It is not trusted from the tx body.
- `reduce_root` is recomputed from `(map_root, tape0_root, tape0_len,
  clock_t, tick_kind)`. It is not trusted from the tx body.
- `dispatch_transition` stays pure. The prefix-dependent verification happens
  in a pre-dispatch system-invariant layer used by both live `apply_one` and
  replay.
- FC2 consumes only the L4/CAS subset of `SystemInvariantContext` needed to
  recompute `tape0_root`, `tape0_len`, `map_root`, and `reduce_root`. The
  wider L4.E / CAS-metadata / constitution fields exist for the shared
  verifier shape and FC3; they are not FC2 tick signing fields in this
  charter revision.
- If implementation needs FC2 to sign L4.E root, CAS metadata root, or
  constitution hash directly, this charter must be revised before §8.
- The tx does not contain `tape1_root`, because that would create a
  signing/root cycle. The concrete `tape1` witness is the accepted
  `LedgerEntry.resulting_ledger_root`.
- The state mutation is narrow: increment `q.q_t.current_round` to `clock_t`
  and advance `state_root_t` through a domain-separated tick state-root helper.

Minimal root formulas:

```rust
fn map_reduce_tick_map_root(prefix: &[LedgerEntry], cas: &dyn LedgerCasView) -> Hash;

fn map_reduce_tick_reduce_root(
    map_root: Hash,
    tape0_root: Hash,
    tape0_len: u64,
    clock_t: u64,
    tick_kind: TickKind,
) -> Hash;
```

### 4.2 FC3 Meta Feedback and Reinit

The first closure keeps ArchitectAI/Veto-AI role execution honest as
`ExternalOnly` while making the feedback/reinit edges tape-visible. A future
runtime-role implementation can switch the mode to `Runtime` under a separate
Class 4 atom.

```rust
pub enum TypedTx {
    // Existing variants unchanged.
    MetaFeedback(MetaFeedbackTx),
    ReinitRequest(ReinitRequestTx),
    ReinitBoot(ReinitBootTx),
}

#[repr(u8)]
pub enum TxKind {
    // Existing discriminants unchanged.
    MetaFeedback = 21,
    ReinitRequest = 22,
    ReinitBoot = 23,
}

#[repr(u8)]
pub enum MetaRoleMode {
    ExternalOnly = 0,
    Runtime = 1,
}

#[repr(u8)]
pub enum VetoVerdict {
    Pass = 0,
    Veto = 1,
}

pub struct BootProfileId(pub String);

pub struct MetaFeedbackTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub constitution_hash: Hash,
    pub source_log_root: Hash,
    pub l4_root_before: Hash,
    pub l4_len_before: u64,
    pub l4e_root_before: Hash,
    pub l4e_len_before: u64,
    pub cas_metadata_root_before: Hash,
    pub architect_input_cid: Cid,
    pub veto_verdict: VetoVerdict,
    pub role_mode: MetaRoleMode,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

pub struct ReinitRequestTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub trigger_entry: u64,
    pub reason_cid: Cid,
    pub target_boot_profile: BootProfileId,
    pub role_mode: MetaRoleMode,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

pub struct ReinitBootTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub request_tx_id: TxId,
    pub replayed_state_root: Hash,
    pub boot_profile: BootProfileId,
    pub role_mode: MetaRoleMode,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

pub enum SystemEmitCommand {
    // Existing commands unchanged.
    MetaFeedback {
        architect_input_cid: Cid,
        veto_verdict: VetoVerdict,
        role_mode: MetaRoleMode,
    },
    ReinitRequest {
        trigger_entry: u64,
        reason_cid: Cid,
        target_boot_profile: BootProfileId,
        role_mode: MetaRoleMode,
    },
    ReinitBoot {
        request_tx_id: TxId,
        boot_profile: BootProfileId,
        role_mode: MetaRoleMode,
    },
}
```

Rules:

- All three FC3 meta txs are system-emitted only.
- `MetaFeedbackTx.source_log_root` is recomputed from the pre-candidate logs
  archive: L4 root/length, L4.E root/length, CAS metadata root, and the pinned
  constitution hash. It is not trusted from the tx body.
- The pre-candidate boundary is strict: `source_log_root` must exclude the
  candidate `MetaFeedbackTx` payload CID itself. Including post-candidate CAS
  metadata would create a signing/root cycle and is a charter violation.
- `architect_input_cid` must exist in CAS as `ObjectType::Generic` with schema
  id `fc3.architect_input.v1`, and its canonical bytes must decode to
  `ArchitectInputCapsule`.
- `ReinitRequestTx.trigger_entry` must point to an accepted tape fact that can
  justify reinit, initially `TerminalSummaryTx { run_outcome: ErrorHalt }` or
  a failed boot/replay anchor ratified in the implementation atom.
- `reason_cid` must exist in CAS as `ObjectType::Generic` with schema id
  `fc3.reinit_reason.v1`, and its canonical bytes must decode to
  `ReinitReasonCapsule`.
- `ReinitBootTx.request_tx_id` must point to a prior accepted
  `ReinitRequestTx`.
- `ReinitBootTx.replayed_state_root` is not caller-supplied through
  `SystemEmitCommand`. The live builder and replay verifier both recompute it
  by deterministic replay through the referenced `ReinitRequestTx` prefix; any
  mismatch in the signed tx body is rejected.
- `initial_q_state.json` is never rewritten. Reinit is a new tape fact and a
  resume/boot action, not a retroactive genesis rewrite.
- Ordinary agent read views must not expose raw logs, raw autopsy detail, or
  deep history by accepting these meta txs.

Minimal FC3 CAS capsule shapes:

```rust
pub struct ArchitectInputCapsule {
    pub schema_version: String, // exactly "fc3.architect_input.v1"
    pub source_log_root: Hash,
    pub public_summary: String,
    pub private_detail_cid: Option<Cid>,
}

pub struct ReinitReasonCapsule {
    pub schema_version: String, // exactly "fc3.reinit_reason.v1"
    pub trigger_entry: u64,
    pub public_summary: String,
    pub private_detail_cid: Option<Cid>,
}
```

If implementation needs first-class CAS `ObjectType` variants instead of
`ObjectType::Generic + schema_id`, this charter must be revised before §8
because that becomes an explicit CAS schema bump.

Minimal log-root formula:

```rust
fn fc3_source_log_root(
    l4_root_before: Hash,
    l4_len_before: u64,
    l4e_root_before: Hash,
    l4e_len_before: u64,
    cas_metadata_root_before: Hash,
    constitution_hash: Hash,
) -> Hash;

fn cas_metadata_root(cas: &CasStore) -> Hash {
    // fold sorted CasObjectMetadata::canonical_hash() values
}
```

## 5. Required Implementation Atoms

### Atom A — FC2-TICK Schema and Replay Skeleton

Touch scope:

- `src/state/typed_tx.rs`
- `src/bottom_white/ledger/transition_ledger.rs`
- `src/bottom_white/ledger/system_keypair.rs`
- replay helpers/tests

Acceptance:

- new discriminants are tail-added only;
- round-trip and canonical digest tests pass;
- old golden digests/discriminants are unchanged;
- forged/tampered tick signatures fail.

### Atom B — FC2-TICK Sequencer and Runtime Live Path

Touch scope:

- `src/state/sequencer.rs`
- `src/runtime/mod.rs` or a narrow runtime helper
- `tests/constitution_flowchart_livenow.rs`
- `tests/fc_alignment_conformance.rs`

Acceptance:

- `fc2_map_reduce_tick_is_livenow` creates an accepted
  `TxKind::MapReduceTick` L4 entry;
- `q.q_t.current_round` advances exactly by one;
- replay recomputes prefix roots;
- agent ingress is forbidden;
- the old ignored pending test is replaced by an active gate.

### Atom C — FC3-META Schema and Replay Skeleton

Touch scope:

- `src/state/typed_tx.rs`
- `src/bottom_white/ledger/transition_ledger.rs`
- `src/bottom_white/ledger/system_keypair.rs`
- replay helpers/tests

Acceptance:

- `MetaFeedback`, `ReinitRequest`, and `ReinitBoot` are tail-added only;
- role mode is explicit (`ExternalOnly` or `Runtime`);
- signing payloads exclude signatures and have domain-separated golden digests;
- replay rejects missing CAS references, wrong schema ids, wrong
  `ObjectType`, decode failures, and wrong pre-candidate root fields.

### Atom D — FC3-META Sequencer and Runtime Live Path

Touch scope:

- `src/state/sequencer.rs`
- `src/runtime/mod.rs` or a narrow runtime helper
- FC3 live tests
- architecture/liveness docs

Acceptance:

- `fc3_logs_feedback_to_architect_ai_is_typed_l4_fact` emits
  `MetaFeedbackTx` and verifies CAS-backed input;
- `fc3_error_reinit_request_links_errorhalt_to_next_boot` links
  `TerminalSummary(ErrorHalt)` to `ReinitRequest` and `ReinitBoot`;
- `verify_chaintape` reconstructs the same roots;
- no handover document or global latest pointer is accepted as canonical
  runtime input.

## 6. Mandatory Gates

Pre-implementation failing gates:

Before implementation starts, add active non-ignored failing stubs for each new
LiveNow gate. The failure proof must show that the named test exists and fails;
zero-test `--exact` runs are invalid.

```bash
rg -n "fn fc2_map_reduce_tick_is_livenow" tests/constitution_flowchart_livenow.rs
rg -n "fn fc3_logs_feedback_to_architect_ai_is_typed_l4_fact" tests/constitution_flowchart_livenow.rs
rg -n "fn fc3_error_reinit_request_links_errorhalt_to_next_boot" tests/constitution_flowchart_livenow.rs
cargo test --test constitution_flowchart_livenow fc2_map_reduce_tick_is_livenow -- --exact
cargo test --test constitution_flowchart_livenow fc3_logs_feedback_to_architect_ai_is_typed_l4_fact -- --exact
cargo test --test constitution_flowchart_livenow fc3_error_reinit_request_links_errorhalt_to_next_boot -- --exact
```

Post-implementation gates:

```bash
cargo check --workspace
cargo test --test constitution_flowchart_livenow
cargo test --test constitution_flowchart_source_alignment
cargo test --test fc_alignment_conformance
cargo test --test constitution_matrix_drift
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast
```

Structural grep gates:

```bash
rg -n "MapReduceTick|MetaFeedback|ReinitRequest|ReinitBoot" src/state/typed_tx.rs src/state/sequencer.rs src/bottom_white/ledger/transition_ledger.rs src/bottom_white/ledger/system_keypair.rs src/runtime/verify.rs
rg -n "TASK_RUNNER_BIN|handover/ai-direct/LATEST|FC_ELEMENTS|TRACE_MATRIX_v" src/state src/runtime tests/constitution_flowchart_livenow.rs
```

Expected:

- first grep finds the new typed surfaces in the canonical places only;
- second grep has no canonical-input hits for the closure path.

## 7. Kill Criteria

Abort or revise the charter if any of these become necessary:

- FC2 tick is implemented as CLI-only, runner-only, dashboard-only, or stdout
  evidence.
- FC2 tick cannot be replayed from L4/CAS/pinned pubkeys.
- FC2 tick requires changing existing tx discriminants or old golden digests.
- FC3 feedback accepts handover docs, `LATEST`, or old trace matrices as
  canonical runtime input.
- FC3 feedback computes `source_log_root` from post-candidate CAS metadata or
  any boundary that replay cannot reconstruct exactly.
- FC3 CAS capsules cannot be decoded canonically or cannot be verified against
  `ObjectType::Generic + schema_id` without a new CAS schema bump.
- FC3 `ReinitBoot` trusts a caller-supplied `replayed_state_root` instead of
  recomputing it from the referenced prefix.
- FC3 reinit rewrites old genesis, old ChainTape, old L4.E, old CAS evidence,
  or trust-root history.
- A new system tx can be submitted through agent ingress.
- `dispatch_transition` gains I/O, wall-clock, env, RNG, or writer side
  effects.
- Ordinary agent prompts receive raw logs, raw autopsy detail, or deep-history
  material from FC3 meta txs.
- Implementation removes active non-violating modules merely to simplify
  constitutional closure.

## 8. Extra Active Modules Retained

The following are outside the literal flowchart nodes but are necessary for the
current system to both obey the constitution and pass real-world tests:

- CAS store: reconstructable evidence and payload bytes.
- L4 ChainTape and L4.E: accepted/rejected tape truth.
- PredicateRegistry and boot activation: executable predicate ground truth.
- System and agent key registries: signature authority separation.
- ProposalTelemetry, PromptCapsule, AttemptTelemetry, EvidenceCapsule,
  MarkovEvidenceCapsule: reconstructable evidence and shielding substrate.
- TDMA/generate/spec-grill/Polymarket market modules: product workloads that
  exercise the constitutional substrate.
- Source-alignment and liveness gates: support invariants preventing derived
  files from becoming false authority.

They remain legal only while they are active, replayable or derived from tape,
and do not override constitution/ChainTape/CAS truth.

## 9. §8 Ratification Options

The user/architect must sign exactly one option before implementation:

```text
[ ] APPROVED-SPLIT-FC2-FIRST — implement Atom A+B first; FC3 follows in a second §8/PR series. Overall three-flowchart closure and OBL-005 remain BLOCKED until FC3 also lands or is constitutionally superseded.
[ ] APPROVED-SPLIT-FC3-FIRST — implement Atom C+D first; FC2 follows in a second §8/PR series. Overall three-flowchart closure and OBL-005 remain BLOCKED until FC2 also lands or is constitutionally superseded.
[ ] APPROVED-ALL-IN-ONE — implement Atom A+B+C+D in one Class 4 PR; larger review and higher integration risk.
[ ] APPROVED-FC2-ONLY — close FC2 tick now; FC3 remains explicitly blocked.
[ ] APPROVED-FC3-ONLY — close FC3 feedback/reinit now; FC2 tick remains explicitly blocked.
[ ] DEFER — keep OBL-005 blocked; no implementation.
[ ] REVISE-v2 — return with modifications before §8.
```

Recommended option: `APPROVED-SPLIT-FC2-FIRST`.

Rationale: FC2 tick is the smaller closure and establishes the shared
pre-dispatch/replay system-invariant pattern that FC3 can reuse.

Sign-off line:

```text
Option selected: ______________________________
Signed by: ____________________________________
Date signed: __________________________________
```

## 10. Research Witnesses

Read-only researcher outputs used for this draft:

- FC2 tick researcher: `FC2-TICK-DESIGN-READY`
- FC3 feedback/reinit researcher: `FC3-DESIGN-READY`
- Class 4 boundary researcher: `CLASS4-CONTRACT-READY`

All three researchers were instructed to treat `constitution.md` as authority
and derived alignment files as evidence only.
