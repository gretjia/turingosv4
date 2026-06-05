# TuringOS-TC Operationalization Full Execution Plan

Status: active orchestration contract for OBL-014.

Authority: user request on 2026-06-04 to implement the TC-000..TC-021
operationalization plan from a clean base, using orchestrated multi-agent
execution and independent audits.

Clean base: `origin/main@39233aa7c868f0e9b37a7a29eb426279f41cf032`.

Working branch: `codex/turingos-tc-operationalization`.

Working tree: `/Users/zephryj/work/turingosv4-tc-operationalization`.

Dirty-tree source: `/Users/zephryj/work/turingosv4`, treated only as evidence
quarry and hypothesis reservoir. Do not use the dirty tree as the TC base.

## 0. Non-Negotiable Architecture Locks

- TuringOS-TC is an OS substrate program, not a Lean program.
- Lean is not part of the kernel. Lean theorem proving is one workload,
  verifier, and benchmark family.
- Kernel-facing TC abstractions must stay domain-general and close to the
  constitution plus the three constitutional flowcharts: tape, refs, CAS,
  transitions, predicates, replay, capabilities, scheduling, and governance.
- Path B is locked: Git-backed `Q_t = <q_t, HEAD_t, tape_t>`.
- Stage implementation must not start from the dirty branch.
- Avoid typed-transaction Class 4 schema changes unless a later explicit
  per-atom section-8 ratification is issued.
- Side-effect gateway first version uses a durable outbox around existing
  evidence capsules and existing transaction surfaces.
- TC-011 Lean micro-state may provide a feature-layer step adapter, but final
  truth for proof acceptance remains the existing final judge path and axiom
  report. No pre-final micro-state is a verified truth source.
- Claim B wording is bounded G0 completeness only: finite grammar, stable rank,
  and strict dovetailing. Do not turn heuristic success into a substrate claim.

## 1. Locked Contracts

### C1 Dirty-Tree Preservation Manifest

Required manifest fields:

```yaml
snapshot_id: string
source_branch: string
source_head: string
origin_main: "39233aa7c868f0e9b37a7a29eb426279f41cf032"
tracked_patch_sha256: string
ahead_patch_sha256: string
untracked_tgz_sha256: string
status_txt_sha256: string
classifications:
  PORT_NOW: [string]
  PORT_SEPARATE: [string]
  ARCHIVE_ONLY: [string]
  DROP_JUNK: [string]
```

Invariant: dirty historical residue is preserved, hashed, classified, and
requalified before any salvage. It is never silently trusted as TC source.

### C2 Path B Ref Contract

Required Rust contract:

```rust
pub struct TcHeadRefs {
    pub accepted_l4: &'static str,
    pub rejected_l4e: &'static str,
    pub cas_root: &'static str,
    pub tdma_verified: &'static str,
    pub tdma_tail: &'static str,
}
```

Required values:

```text
accepted_l4   = refs/chaintape/l4
rejected_l4e  = refs/chaintape/l4e
cas_root      = refs/chaintape/cas
tdma_verified = refs/tdma/verified_head
tdma_tail     = refs/tdma/ledger_tail
```

Invariant: authoritative ref movement cannot ignore failure. A failed
authoritative ref update returns an error or enters an explicitly detected
recovery state.

### C3 External Side-Effect Record

Required data shape:

```rust
pub struct ExternalCallIntent {
    pub intent_id: String,
    pub logical_call_id: String,
    pub call_site: String,
    pub run_id: String,
    pub request_hash: String,
    pub provider: String,
    pub model: Option<String>,
    pub redacted_request_cid: String,
    pub idempotency_key: String,
    pub timeout_ms: u64,
    pub logical_t: u64,
}

pub enum ExternalCallTerminal {
    Result {
        result_hash: String,
        usage: Usage,
        status: u16,
        provider_request_id: Option<String>,
    },
    Failure {
        class: String,
        retryable: bool,
        public_summary: String,
    },
    Abandoned {
        reason: String,
        may_have_spent: bool,
    },
}
```

Invariant: `Intent count == Result + Failure + Abandoned`. Clean completion
does not allow unresolved pending side effects.

### C4 Lean Micro-State Contract

Required data shape:

```rust
pub struct GoalState {
    pub theorem_id: String,
    pub state_id: String,
    pub parent_state_id: Option<String>,
    pub goals: Vec<GoalView>,
    pub imports_hash: String,
    pub preamble_hash: String,
    pub lean_version: String,
    pub mathlib_rev: Option<String>,
}

pub struct TacticAttempt {
    pub attempt_id: String,
    pub parent_state_id: String,
    pub tactic: String,
    pub timeout_ms: u64,
    pub input_goal_hash: String,
}

pub enum LeanStepOutcome {
    Advanced { next: GoalState },
    Complete { proof_script: String },
    Failed { class: String, feedback: String },
    Timeout,
    Rejected { class: String },
}
```

Invariant: no micro-state `Verified` state exists. Final proof acceptance is
only after the final judge accepts the assembled proof and axiom report.

### C5 G0 Completeness Contract

Required data shape:

```rust
pub struct G0Manifest {
    pub version: String,
    pub tactic_atoms: Vec<String>,
    pub lemma_atoms: Vec<String>,
    pub productions_hash: String,
    pub manifest_hash: String,
}

pub struct Candidate {
    pub ast_canonical: String,
    pub digest: String,
    pub rank: u64,
    pub lean_text: String,
}

pub struct SchedulerTrace {
    pub tick: u64,
    pub lane: Lane,
    pub candidate_digest: Option<String>,
    pub rank: Option<u64>,
    pub action: String,
    pub verdict: String,
}
```

Default G0 excludes unrestricted high-level automation, native compute escape
paths, raw tactic strings, and hidden automation. Rank is pure over canonical
AST data.

## 2. Phase Plan

### Phase Q: Quarantine Before TC-000

Atoms: TC-Q000 dirty quarantine; TC-Q001 clean worktree.

Execution:

- Snapshot dirty branch outside the repo.
- Hash tracked diff, ahead diff, untracked tarball, and status output.
- Create a clean worktree from the locked base.
- Classify salvage items as `PORT_NOW`, `PORT_SEPARATE`, `ARCHIVE_ONLY`, or
  `DROP_JUNK`.

Ship gate:

- Snapshot manifest verifies.
- No destructive command used.
- Clean worktree has no preexisting dirty files.
- `rules/enforcement.log` conflict residue is classified `DROP_JUNK`.

Audit:

- Dirty-Tree Steward verifies preservation and classifications.

### Phase 0: Constitutional Decision Lock

Atoms: TC-000 Path B decision; TC-001 Veto-AI scope lock.

Execution:

- Commit explicit Path B decision and ref topology.
- Lock Veto-AI verdict domain to `{PASS,VETO}`.
- Separate constitutional veto role from engineering review roles.

Ship gate:

- Docs state Path A and Path C are rejected.
- No strong public completion language.
- No restricted code touched.

Audit:

- Constitution Auditor.
- Karpathy Architect Auditor before high-risk code starts.

### Phase 1: Boot Trust Root

Atom: TC-002 Boot trust-root manifest.

Execution:

- Verify constitution hash.
- Verify trust-root payload hashes.
- Verify predicate hash list.
- Verify the current ref contract.

Ship gate:

- `turingos boot --verify-manifest` passes.
- `turingos boot --verify-constitution-hash` passes.
- `turingos boot --verify-predicates` passes.
- One SHA mismatch fixture fails closed.

### Phase 2: Path B Substrate

Atoms: TC-003 Git QState substrate; TC-004 rtool/wtool triple; TC-005 l4/l4e
split.

Execution:

- Harden `src/git_tape_ledger.rs`.
- Harden transition-ledger interaction only if required and ratified by risk
  class.
- No swallowed authoritative ref movement.
- Reopen append resumes sequence identity.
- Accepted and rejected refs reconstruct independently.

Ship gate:

- `git fsck --full` passes on test repos.
- Reopen append produces `tn-N+1`.
- Rejected predicate path advances only L4.E.
- Accepted world is unchanged after rejection.

### Phase 3: Art. 0.2 Tape Canonical Repair

Sub-atoms: TC-101 through TC-110.

Execution:

- Add structured transaction and cost schema where allowed by risk class.
- Route failed proposals to L4.E.
- Put gateway, WAL, map-reduce tick, LLM call, wallet, search, board, Lean
  error, halt, and boot provenance facts on tape or in CAS with tape reference.
- Ensure sidecars are derived views only.

Ship gate:

- Every derived view has a deterministic reconstruction assertion.
- No sidecar is authoritative.
- Replay does not call network or LLM.

### Phase 4: External Side-Effect Gateway

Atom: TC-007.

Execution:

- Implement durable outbox around LLM calls first.
- Route, proof, and challenge calls emit pre-call intent and one terminal
  record.

Ship gate:

- Crash before send, after intent, after HTTP success, and after parse failure
  maps to deterministic terminal states.
- Clean halt fails on unresolved pending records.

### Phase 5: Universal Witnesses

Atoms: TC-009 Minsky; TC-010 Brainfuck.

Execution:

- Implement two independent universal-machine witnesses.
- Every machine step is reconstructible from tape/CAS.

Ship gate:

- Replay from genesis is byte-identical.
- Tamper test fails as expected.
- Capped non-halting run resumes deterministically.

### Phase 6: Lean Micro-State

Atom: TC-011.

Execution:

- Implement feature-layer LeanREPL/RPC-style step adapter.
- Keep final certification on the existing final judge path.
- Keep Lean outside the kernel and outside the general OS truth substrate.

Ship gate:

- `intro` and `simp` fixtures advance or complete.
- Backtracking works.
- Raw stderr is absent from prompt view.
- Final proof recertifies via the final judge.

### Phase 7: Completeness Spine

Atoms: TC-012 G0; TC-013 fair dovetail.

Execution:

- Freeze G0 manifest.
- Implement canonical parser, rank, enumerator, and strict 1:1 dovetail
  scheduler.

Ship gate:

- Known-rank corpus count is exact.
- Witness index `i` is attempted by even tick `2*i`.
- Market on/off/shuffled produces byte-identical even-lane trace.

### Phase 8: Queue Isolation

Atom: TC-014.

Execution:

- Hard-separate even enumerator queue and odd heuristic queue.
- Duplicate suppression is digest-only with trace pointer.

Ship gate:

- Poisoned high-price odd queue cannot skip, pop, reorder, or mask even-lane
  candidates.

### Phase 9: Market / Autonomous Legalization

Atoms: TC-016; TC-017.

Execution:

- Market, price, and autonomous routing affect odd-lane priority only.
- They cannot affect predicate authority, verifier authority, rank, or
  enumerator coverage.

Ship gate:

- Price toggle changes odd trace only.
- Verifier acceptance comes only from final verifier.
- Parity metrics include proposal, route, challenge, token, verifier,
  scheduler, enumerator, and wall-clock metrics.

### Phase 10: Agent View Shielding

Atom: TC-018.

Execution:

- Prompt views expose scoped summaries only.
- Hidden theorem bodies, unshielded verifier output, private diagnostics, and full
  landscape leakage are blocked.

Ship gate:

- Sentinel tests prove route/proof prompts do not leak hidden bodies or raw
  verifier errors.

### Phase 11: RAM Statelessness Crash Test

Atom: TC-015.

Execution:

- Kill-after-every-committed-transition matrix for substrate, gateway, Lean
  step, and scheduler.

Ship gate:

- Restart from Git/CAS only.
- No RAM cache is required for correctness.
- Snapshots are acceleration-only.

### Phase 12: Difficulty Ladder

Atom: TC-019.

Execution:

- Freeze L0-L5 theorem/task bank.
- Gate hard benchmark entry on L0-L4 receipts.

Ship gate:

- No hard-set Claim C run begins before L0-L4 are green.

### Phase 13: Price Calibration

Atom: TC-020.

Execution:

- Preregister finite-budget Claim C experiments.
- Record arm semantics and parity metrics.

Ship gate:

- If token, verifier, scheduler, or enumerator parity fails, report remains
  descriptive and non-promotional.

### Phase 14: Full Audit Packet

Atom: TC-021.

Execution:

- Export source SHAs, constitution hash, boot manifest, ref schema, replay
  commands, crash matrix, witness traces, G0 manifest, scheduler traces, parity
  tables, and audit verdicts.

Ship gate:

- Clean-context audit has no unresolved violation.
- Obligation witness returns `OBL-ALL-CLOSED`.
- Final reliability audit excludes metadata artifacts `RUN_STATUS.json`,
  `STAGE_A_POWER_GATE.json`, and `prereg.json`.

## 3. Multi-Agent Execution Schedule

Use at most three implementer lanes at once and keep ownership disjoint:

- Substrate lane: Phase Q, Phase 0, Phase 1, Phase 2, Phase 3.
- Gateway/reliability lane: Phase 4, Phase 11, audit packet reliability
  receipts.
- Lean/search lane: Phase 6, Phase 7, Phase 8, Phase 10, and feature-level
  theorem workload surfaces.

Do not dispatch broad coding before the relevant contract is locked. Each
high-risk phase gets an independent audit with the appropriate role:

- Constitution Auditor for trust-root, Path B, and authority surfaces.
- Data-Integrity Auditor for tape/canonical-view and side-effect records.
- Reliability Auditor for crash and replay behavior.
- Formal-Methods Auditor for bounded G0, scheduler, and Lean feature-layer
  semantics.
- Karpathy Architect Auditor before major architecture coding and Karpathy
  Simple Code Auditor before final branch completion.
- Obligation witness before final completion.

## 4. Required Verification Gates

Broad gates:

```bash
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift
```

Structural checks:

```bash
changed_files="$(mktemp)"
(git diff --name-only origin/main...HEAD; git ls-files -o --exclude-standard) | sort -u > "$changed_files"

grep -E 'src/(kernel|bus|state/sequencer|state/typed_tx|sdk/tools/wallet|bottom_white/cas/schema)\.rs' "$changed_files" || true

grep -E '^(src|tests)/' "$changed_files" |
  while IFS= read -r f; do
    grep -nE 'raw.*std[e]rr|Lean.*std[e]rr|api[_-]?ke[y]|Authori[z]ation|Bear[e]r' "$f"
  done

grep -E '^(handover|src|tests)/' "$changed_files" |
  grep -v '^handover/directives/tc_taskpackets_2026-06-04/' |
  while IFS= read -r f; do
    grep -nE 'PRO[V]EN|DEFIN[I]TIVE|caus[a]l|isolated[[:space:]]lever|X[[:space:]]>[[:space:]]Y' "$f"
  done

rm -f "$changed_files"
```

Note: if the restricted-surface grep returns matches, stop and classify before
continuing. If either content scan prints a match, stop and classify before
continuing. These checks are changed-file scoped; do not scan historical
handover wholesale and treat old archived text as a new ship blocker.

## 5. Current Local Progress Snapshot

Completed in the first local slice:

- Phase Q preservation manifest.
- TC-000 Path B decision.
- TC-001 Veto-AI scope lock.
- TC-002 boot trust-root verifier slice.
- Path B ref contract scaffold and fail-closed ledger hardening tests.
- External-call record contract scaffold and invariant tests.
- Lean micro-state record contract scaffold with no pre-final verified state.
- Bounded G0 manifest/rank/dovetail scaffold tests.

Not yet complete:

- Production gateway tape integration.
- Universal witnesses.
- LeanREPL/RPC adapter.
- Production scheduler and queue isolation.
- Crash matrix.
- Difficulty ladder.
- Price calibration.
- Final audit packet.
- Clean-context audits for later ship-path phases.
- Final obligation witness.
