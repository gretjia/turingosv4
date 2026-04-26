# TFR_MASTER_PLAN_2026-04-26.md

> ⚠️ **LEGACY / SUPERSEDED — preserve for history only** ⚠️
>
> **As of 2026-04-26 night** (per CO_P0_AMENDMENT_v1, D3=A): this TFR v1 plan is **deprecated**.
> Successor: `handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md`
> Authoritative blueprint: `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md`
> Tri-model orchestration: `handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md`
>
> **Why deprecated**: TFR v1 covered ~20% of the 2026-04-26 white paper scope (only tape/git substrate + Anti-Oreo partial). v3.1 expands to full 反奥利奥 + 6-layer ChainTape + 9-component Q_t + 12 economic invariants + RSP-1 9 modules + DO-178C bidirectional trace matrix.
>
> **Do not implement against this plan.** Read v3.1 instead.
>
> Original content preserved below for archaeological reference.

---

# TuringOS Foundation Refactor — Master Plan (Path B: Real Git Substrate)
> **Status**: Architectural design, awaiting external dual-audit gate (Codex + Gemini) before sprint S0 commit.
> **Author**: ArchitectAI (Claude Opus 4.7, 1M context) — v1 draft, 2026-04-26.
> **Scope**: 6–8 wall-clock weeks; freezes Phase C C2 batch; reopens at S3 exit.
> **Constitutional anchor**: Art. 0.1 / 0.2 / 0.3 / 0.4 (273b362) + Art. IV flowchart (lines 540–610) + Art. V.1 separation of powers.

---

## Table of Contents

1. § 1 — Mission + Path B confirmation
2. § 2 — Architectural design
   - 2.1 Decision: gix vs git2-rs
   - 2.2 Per-cell runtime repo layout
   - 2.3 Node ↔ git mapping (proposed + challenged)
   - 2.4 Tape trait API (minimum interface)
   - 2.5 GitTape implementation contract
   - 2.6 MemTape (legacy) contract
   - 2.7 Q_t = ⟨q_t, HEAD_t, tape_t⟩ runtime mapping
   - 2.8 Π_p as pre-commit hook
   - 2.9 rtool / wtool signature unification
   - 2.10 Boltzmann routing as git branch (deferred to S5+)
   - 2.11 Backward-compat shim strategy
3. § 3 — Sprint structure with timeline
4. § 4 — Per-sprint atom enumeration
   - S0, S1, S2, S3, S4, S5, S6 atoms with file-path / dependency / cargo-test / dual-audit gates
5. § 5 — Team organization (internal + external)
6. § 6 — DO-178C TRACE_MATRIX_v3 strategy
7. § 7 — Trust Root migration plan
8. § 8 — Risk register
9. § 9 — PREREG amendment text
10. § 10 — Decision log + open questions for the user

---

## § 1 — Mission + Path B Confirmation

### 1.1 Mission

> Replace TuringOS's in-memory `Vec<Node>` tape with a real git substrate (per-cell runtime repo) so that Q_t = ⟨q_t, HEAD_t, tape_t⟩ is *literally* a git working tree, every `bus.append` is *literally* a `git commit`, and every Π_p evaluation is *literally* a pre-commit hook. The goal is to pay 6–8 weeks of refactor cost once, in exchange for: 30 years of battle-tested content-addressable storage, a free Merkle-DAG hash chain (closes Art. 0.3 reservation), an audit-time `git log` / `git diff` / `git fsck` / `git verify-pack` toolchain, and constitutional alignment with Art. 0.4's verbatim "version control" framing.

### 1.2 Why Path B (recap of the 2026-04-26 user decision)

| | Path A (semantic) | Path B (real git) | Path C (hybrid defer) |
|---|---|---|---|
| Effort | ~3 weeks | **~6–8 weeks** | ~3 + ~5 = ~8 weeks (worse than B) |
| Constitutional alignment | partial — keeps `Vec<Node>`, fakes HEAD_t | **full** — Q_t literally is git | full @ Phase E (deferred technical debt) |
| Phase E auditor toolchain | bespoke replay code | **stock git** | one-time migration cost |
| Hash chain (Art. 0.3) | self-implemented; hand-audited | **free, sha256 commit hashes** | self-implemented then replaced |
| Risk profile | re-implements wheels poorly | **steeper learning curve, well-trodden ground** | every Phase C/D commit accumulates A→B migration debt |
| Effective cumulative cost | 3 wk + permanent maintenance | **6–8 wk one-shot** | 3 + 5 wk + sunk-cost-pain |

User chose **B**. This document operationalizes that choice.

### 1.3 What is *not* in scope

- Merkle root + heldout-sealed-hash double-lock (Art. 0.3 Phase E+ extension) — sprint S6 *prepares* for this but does not ship it.
- Phase D ArchitectAI ↔ ledger cost-attribution UI — S6 ensures the substrate is *capable* of `git diff`-based attribution; the dashboard is post-TFR.
- Multi-cell shared repos (cross-problem cooperation, federation across runs) — explicitly out-of-scope; TFR commits to per-cell isolated repos.
- Boltzmann-routing-as-git-branch (S5+ stretch goal; falls back to "single-branch, parent-pointer-only" if S5 schedule overruns).

### 1.4 What freezes during TFR

- **Phase C C2 batch** — frozen the moment S0 commits land (no new C2 runs). Restart gating: S3 (Bus integration) cargo-test green + dual-audit PASS/PASS.
- **PREREG 30-day arc clock** — see § 9. Two options proposed; user must explicitly pick.
- **Phase D shadow-mode runs** — paused. ArchitectAI can read tape but cannot write user_space artifacts during S0–S3 (the substrate is in flux).
- **Trust Root manifest** — *changes* during TFR (every sprint adds / removes files; § 7 spells out the migration). The manifest is updated through `architect-ingest` skill at each sprint exit, *not* mid-sprint.

---

## § 2 — Architectural Design

### 2.1 Decision: `gix` vs `git2-rs`

| Axis | `gix` (gitoxide, pure Rust) | `git2-rs` (libgit2 FFI) |
|---|---|---|
| Language safety | pure Rust, no FFI panics across ABI | C library, segfault risk on bad input |
| Build complexity | `cargo build` works on any Rust target | needs `libgit2-sys` build, OpenSSL/zlib system deps |
| API stability | active development, breaking changes 2024–2026 | stable, mature (since 2014) |
| Feature completeness | ~85% (commit, branch, diff, refs OK; some advanced merge missing) | 100% — wraps full libgit2 |
| Performance | comparable (gix sometimes faster due to no FFI overhead) | comparable |
| Audit-time tooling parity | `git` CLI works on either repo (both produce stock object format) | same |
| Bug surface for our use case (commit + branch + diff + ref-walk) | well-covered in gix-stable APIs | trivially covered |
| TuringOS dep policy | **CLAUDE.md "压缩即智能"** — prefer compression; avoid FFI when avoidable | adds C tooling to bootstrap |

**Recommendation: `gix` (gitoxide) crate version `^0.66`** at S0 commit time. Pin to a specific minor release in `Cargo.toml`. Rationale: pure-Rust avoids the libgit2/OpenSSL build matrix that has historically broken our docker images; the features we need (commit / branch / index / diff / ref-walk / hash-object) are all in stable `gix` APIs; if we hit a `gix` capability gap we *fall back* to shelling out to system `git` via `tokio::process::Command` — this is constitutionally fine because we are *manipulating* a real git repo either way.

**Fallback (locked-in)**: if S0 spike (atom S0.4) reveals a gix blocker, switch to `git2-rs` *with* explicit dual-audit re-approval. The Tape trait (§ 2.4) is library-agnostic; switching costs ~1 day of work.

**Forbidden**: shelling out to `git` CLI as the *primary* mechanism. Reason: process-spawn overhead at every `bus.append` is unacceptable (Phase C C2 has ~200 appends/run × 1k runs = 200k commits; at ~30ms per `git` exec that's 100 minutes of pure shell overhead per batch). Library API is mandatory; CLI is debug-only.

### 2.2 Per-cell runtime repo layout

```
experiments/<problem_id>/<run_id>/
  runtime_repo/                         <-- newly created at run start, fresh
    .git/                               <-- gix-managed
      HEAD                              <-- HEAD_t (constitutional)
      objects/                          <-- content-addressable Node payloads
      refs/heads/main                   <-- canonical chain
      refs/heads/proposal-<author>-<tx> <-- (S5+) per-Boltzmann-pick branch
      hooks/pre-commit                  <-- Π_p (S2 lands shim, S3 wires real preds)
    .turingos/
      nodes/<node_id>.json              <-- typed Node metadata (kind / cost / kind_payload)
      ledger.jsonl                      <-- LedgerEvent append-only mirror (legacy compat)
    .gitignore                          <-- excludes nothing yet; may exclude .turingos/cache/
  wal/                                  <-- legacy WAL JSONL (deprecated end of S5)
  proofs/                               <-- accepted .lean artifacts (sidecar; unchanged)
  jsonl_logs/                           <-- run-level aggregate (unchanged)
```

**Lifecycle**:
1. **Run start**: `gix::init(runtime_repo)` → empty repo, `HEAD` points to unborn `main`. Initial commit at boot writes `genesis.json` (run_id, problem_id, model_snapshot, seeds) and stamps it as the run's *root commit* — this commit's SHA is the run's de facto Trust Root anchor.
2. **Per-append**: `bus.append(...)` writes `.turingos/nodes/<id>.json` to working tree, stages it, commits with `kind`/`author`/`cost` in commit-message body, advances `main`. `HEAD` after the commit ≡ `HEAD_{t+1}`.
3. **Run end**: optional `gix::archive(format="tar", HEAD)` → `<run_id>.tar` for archival + Phase E reproducibility audit. Repo can be left in place; size budget below.
4. **Post-mortem auditor pass**: external auditor runs `git log --all --format="%H %s" runtime_repo` to walk the entire causal history. `git diff <commit_before>..<commit_after>` gives per-commit `kind_payload` deltas — solves Phase D ArchitectAI cost-attribution-to-golden-path (Art. 0.2 §4).

**Size budget**: at Phase B C2 hard-10 × 200 tx × ~1 KB per commit ≈ 200 KB working tree + ~400 KB pack file = **~600 KB per run × 100 runs/batch = 60 MB**. Acceptable; deletable after archival.

**Cleanup**: `runtime_repo/` is *not* in TR; deletable post-archival. Genesis hashing rule (S6) records the genesis-commit SHA into the run-level JSONL row, so even after deletion the run's tape root is forensically pinned.

### 2.3 Node ↔ git mapping (challenge to user's proposal)

**User's proposal**: each Node = one git commit + one `.turingos/nodes/<id>.json` file in working tree.

**My counter-proposal (refinement, not replacement)**:

| Node component | Storage location | Rationale |
|---|---|---|
| `node.id` | commit message header `Node-Id: tx_<n>_by_<author>` + filename `.turingos/nodes/<id>.json` | dual-source; debuggable via `git log --grep="Node-Id:"` |
| `node.author` | commit author (`Author: <agent_id> <agent_id@turingos.local>`) | git-native, surfaces in `git log` automatically |
| `node.created_at` | commit committer-timestamp (millisecond UNIX ms) | git supports up-to-second natively; we override env var `GIT_COMMITTER_DATE` to inject ms-precision via the JSON sidecar (closes V-06 properly) |
| `node.payload` | working-tree file `.turingos/nodes/<id>.json` field `payload` | full text, deterministic encoding |
| `node.citations` | git **parents** (multi-parent merge commits for multi-cite) | **this is the key constitutional win** — Tape DAG topology IS git DAG topology |
| `node.kind` (V-01) | commit-message body `Kind: <variant>` + sidecar field | both for discoverability + parsing |
| `node.kind_payload` | sidecar JSON field `kind_payload` | structured per-kind data |
| `node.cost` (V-01 fix) | sidecar JSON field `cost: { prompt_tokens, completion_tokens, tool_stdout_tokens, latency_ms }` | structured |
| `node.verified` (V-03 fix) | sidecar `verified: true|false` + commit message `Verified: <bool>` | failed-branch nodes commit with `verified: false`, pre-commit hook permits this iff Π_p reports the failure (rejects only un-recorded silent drops) |
| `node.hash` (Art. 0.3) | **commit SHA itself** | NO additional field needed; Art. 0.3 reservation closes by design |

**Why merge commits for citations**: when a Node cites N parents (rare but possible — a synthesis step), git represents this as an N-parent merge commit. `git log --graph` then renders the proof DAG natively. This is the single highest-leverage architectural decision in the plan.

**Edge case**: a Node with 0 citations (root) maps to git's *root commit* (no parents). Handled natively.

**Edge case**: same `node.id` re-asserted (V6 spacetime paradox protection) — git rejects via "commit already exists" if the SHA collides; we add a programmatic check: if `.turingos/nodes/<id>.json` already exists in HEAD's tree, return `TapeError::DuplicateId` *before* attempting commit.

### 2.4 Tape trait API (minimum interface)

```rust
// src/tape/mod.rs (new module, S1)
pub trait Tape {
    /// Append a node. On success, returns the node's storage handle.
    /// For GitTape: handle is the commit SHA.
    /// For MemTape: handle is the legacy NodeId.
    fn append(&mut self, node: Node) -> Result<NodeHandle, TapeError>;

    /// Get a node by ID.
    fn get(&self, id: &str) -> Option<Node>;

    /// Time arrow: nodes in commit (= insertion) order.
    /// For GitTape: `git log --reverse --first-parent main`.
    /// For MemTape: `self.time_arrow.clone()`.
    fn time_arrow(&self) -> Vec<NodeId>;

    /// Walk the primary causal chain from `node_id` back to root.
    /// For GitTape: `git rev-list <node_commit_sha>` along first parent.
    /// For MemTape: legacy `trace_ancestors`.
    fn get_chain(&self, node_id: &str) -> Vec<NodeId>;

    /// Current HEAD. Closes Art. 0.4 HEAD_t gap.
    /// For GitTape: `git rev-parse HEAD`.
    /// For MemTape: `time_arrow.last()`.
    fn head(&self) -> Option<NodeId>;

    /// Children of a node (reverse citations).
    /// For GitTape: `git rev-list --children <id>`.
    /// For MemTape: `reverse_citations[id]`.
    fn children(&self, id: &str) -> Vec<NodeId>;

    /// Total node count.
    fn len(&self) -> usize;

    /// Existence check.
    fn contains(&self, id: &str) -> bool;
}

pub enum NodeHandle {
    GitCommit { sha: String },
    MemId { id: NodeId },
}

impl NodeHandle {
    pub fn as_node_id(&self) -> &str { /* normalize */ }
}
```

**Critical design notes**:

1. **Trait, not enum dispatch**: `Box<dyn Tape>` allows runtime swap (env-gated `TURINGOS_TAPE=git|mem`); both impls coexist forever in tests, prod uses Git after S3.
2. **No `&mut` on read methods**: enables snapshot pattern for `bus.snapshot()` without lock thrashing.
3. **`Node` stays a serializable struct** (sprint S1 redefines it per V-01; sprint S2 adds `kind`/`cost`/`kind_payload`). Both impls share the same `Node` definition — only storage backend differs.
4. **No async**: gix supports sync APIs; we keep sync to avoid contagion. The bus is V3L-11 serial-reactor anyway.
5. **Returns `NodeHandle`, not `()`**: caller learns commit SHA without re-querying. Critical for ledger emission pairing.

### 2.5 `GitTape` implementation contract

```rust
// src/tape/git_tape.rs (new, S1)
pub struct GitTape {
    repo: gix::Repository,
    main_branch: gix::refs::FullName,
    runtime_dir: PathBuf,
    /// In-memory cache of (id → commit-SHA). Persistable but rebuildable from
    /// `git log` (a derived view per Art. 0.2).
    id_to_sha: HashMap<String, gix::ObjectId>,
}

impl GitTape {
    pub fn init(runtime_dir: impl AsRef<Path>) -> Result<Self, TapeError> {
        // gix::init(runtime_dir)
        // create initial commit with genesis.json
        // set HEAD → refs/heads/main
    }

    pub fn open(runtime_dir: impl AsRef<Path>) -> Result<Self, TapeError> {
        // gix::open(runtime_dir)
        // walk all of main, populate id_to_sha cache
    }
}

impl Tape for GitTape {
    fn append(&mut self, node: Node) -> Result<NodeHandle, TapeError> {
        // 1. Resolve parent commit SHAs from node.citations via id_to_sha
        // 2. Write `.turingos/nodes/<node.id>.json` to working tree
        // 3. Stage (gix index add)
        // 4. RUN PRE-COMMIT HOOK (Π_p) — if fails, abort, return Veto
        // 5. Commit with author/timestamp/message metadata; gix returns commit SHA
        // 6. Update id_to_sha cache
        // 7. Return NodeHandle::GitCommit { sha }
    }
    // ... other methods walk the git DAG via gix
}
```

**Constitutional citation**: this struct is `FC1-N4` (`tape_t` as files) literalized. Every line of `GitTape::append` maps to a constitutional clause; the TRACE_MATRIX_v3 (§ 6) cross-references each step.

### 2.6 `MemTape` (legacy) contract

```rust
// src/tape/mem_tape.rs (renamed from current src/ledger.rs::Tape, S1)
pub struct MemTape {
    nodes: HashMap<NodeId, Node>,
    reverse_citations: HashMap<NodeId, Vec<NodeId>>,
    time_arrow: Vec<NodeId>,
}

impl Tape for MemTape { /* current ledger.rs logic, reorganized */ }
```

**Lives forever** as the in-memory fast-path test fixture. Production switches to `GitTape` after S3; tests use `MemTape` for unit-testing kernel topology / market math without paying gix init cost.

### 2.7 Q_t = ⟨q_t, HEAD_t, tape_t⟩ runtime mapping

```rust
// src/q_state.rs (new at S2; consolidates ad-hoc state)
pub struct QState {
    /// q_t — agent cognitive state (RunCostAccumulator, search history,
    /// boltzmann seed, etc.) — promoted from JSONL-only to a tape-derivable
    /// struct over sprints S4 + S5.
    pub q_t: AgentSwarmState,

    /// HEAD_t — canonical "where are we" pointer. Equals tape.head().
    /// Stored explicitly so downstream code never has to ask the tape twice.
    pub head_t: Option<NodeId>,

    /// tape_t — the Tape trait object. Real impl behind dyn dispatch.
    pub tape_t: Box<dyn Tape>,
}

impl QState {
    /// rtool — Art. IV line 599: rtool(⟨q_t, tape_t, HEAD_t⟩) → ⟨q_i, s_i⟩
    pub fn rtool(&self) -> AgentInput { /* ... */ }

    /// wtool — Art. IV line 627: wtool(output | tape_t, HEAD_t, tools_other)
    pub fn wtool(&mut self, output: AgentOutput) -> Result<QState, BusError> { /* ... */ }
}
```

S2 introduces `QState` as the *thing the bus owns* (replacing the tangled bus state today). All state mutations route through `QState::wtool`. Closes the Art. 0.4 gap top-to-bottom.

### 2.8 Π_p as pre-commit hook

**Two-tier implementation**:

1. **In-process pre-commit gate** (`Tape::append` runs Π_p *before* invoking `gix::commit`):
   - Forbidden patterns (current `forbidden_patterns` check)
   - Payload size limits
   - Tool veto signal
   - Π_p returns `PredicateResult::Pass | Reject(reason)`. On Reject, we **still commit** but with `verified: false` and `kind_payload.reject_class: "veto:..."` (closes V-03 — failed branches on tape).

2. **On-disk `.git/hooks/pre-commit` shell hook** (S3 lands; Phase E gate):
   - Belt-and-suspenders. Independently verifies that the commit being made is consistent with sidecar JSON.
   - Detects out-of-band tampering (someone manually `git commit`-ing into the runtime repo).
   - Phase E auditor can `git verify-pack` + re-run pre-commit hooks on every commit to confirm Π_p was active during the run.

**Constitutional citation**: this is the literal mechanism Art. 0.1 names "**strict discipline**" and Art. 0.4 names "Π_p as pre-commit hook".

### 2.9 rtool / wtool signature unification

**Current** (broken, per Art. 0.4 violation table):
```rust
bus.snapshot() -> UniverseSnapshot          // no HEAD axis
bus.append(author, payload, parent_id)      // doesn't operate on HEAD or path
```

**Target** (S2/S3):
```rust
QState::rtool() -> AgentInput { q_i, s_i }                 // matches Art. IV line 599
QState::wtool(output: AgentOutput) -> Result<QState, _>    // matches Art. IV line 627
                                                            // (mutates tape, advances HEAD,
                                                            //  returns Q_{t+1})
```

`bus.append` becomes a **wrapper** around `QState::wtool` for back-compat in tests. New code paths call `QState::wtool` directly.

### 2.10 Boltzmann routing as git branch (deferred to S5+)

**Stretch design** (does NOT block sprint exits S0–S4):

- Each Boltzmann parent-pick decision = `git branch proposal-<author>-<tx>` from the picked parent.
- Failed branches stay on their proposal branch (orphaned tip).
- Successful branches merge back to `main`.
- The seed used for Boltzmann selection becomes a tape Node (`kind: BoltzmannPick`, `kind_payload: { seed_state, scores }` — closes V-08b).

**Fallback** (single-branch mode, S0–S4 default): all commits on `refs/heads/main` with multi-parent merges representing the DAG. `git log --graph` shows everything. Boltzmann decisions recorded as `kind: BoltzmannPick` Nodes but no actual branch is created.

S5 promotes from fallback → real branches if schedule permits; if not, S6 explicitly defers to a future TFR2 mini-arc.

### 2.11 Backward-compat shim strategy (cargo test green at every commit)

The hard constraint: **cargo test passes at every commit**. The challenge: `bus.append`'s signature changes (V-01 adds cost; S2 adds kind; S3 returns NodeHandle). Approach:

1. **S1 atom 1.3**: introduce `bus.append_v2(node: NodeBuilder) -> Result<NodeHandle>`. Old `bus.append(author, payload, parent)` stays, internally calls `append_v2(NodeBuilder::default()...)`. **No test breakage**.
2. **Sprints S1–S3**: every new caller writes `append_v2`; existing callers untouched.
3. **S4 atom 4.6**: migrate evaluator + tests off `append` to `append_v2`. Once green, **delete `append`** in atom 4.7.
4. **S5 atom 5.x**: do the same for `bus.snapshot` → `QState::rtool`.

Result: not a single test breaks across the entire arc; the old API simply dies at end of S4. This is the same pattern used in production codebases for major refactors and works exceptionally well with TuringOS's serial-reactor architecture.

---

## § 3 — Sprint Structure with Timeline

The user's proposed structure (S0 → S1 substrate → S2 schema → S3 bus → S4 economy → S5 side-state → S6 Phase E gate prep) is **correct in spirit** but I refine for parallelizability and dual-audit cadence:

| Sprint | Name | Duration (wall-clock) | Effective work | Dual-audit gate |
|---|---|---|---|---|
| **S0** | Design + spike + decision freeze | 5–7 days | Plan + gix spike + dual-audit-of-plan | PASS/PASS gates S1 entry |
| **S1** | Tape trait + GitTape skeleton + MemTape rename | 7–10 days | trait-only refactor; both impls compile | PASS/PASS gates S2 entry |
| **S2** | Node schema v2 (kind / cost / kind_payload) + WAL v2 + QState struct | 7–10 days | schema upgrade end-to-end | PASS/PASS gates S3 entry |
| **S3** | Bus integration (real GitTape in production) + pre-commit hooks | 10–14 days | the heart of the migration | **PASS/PASS unfreezes Phase C C2 batch** |
| **S4** | Economy on tape (markets / wallet / invest as derived views) | 7–10 days | market events on tape | PASS/PASS gates S5 entry |
| **S5** | Side-state on tape (search / board / FC trace / mr tick / Boltzmann) | 7–10 days | parallel ledgers → derived views | PASS/PASS gates S6 entry |
| **S6** | Phase E gate prep (heldout ops sealing on git substrate + Trust Root migration + arc reopen) | 5–7 days | hardening | **PASS/PASS reopens 30-day Phase E arc** |
| **Total** | | **48–68 days = ~7–10 weeks** | | |

**Schedule risk**: my honest estimate is **8 weeks** (56 days), with 2 weeks buffer accepted up to 10. The user's 6–8 week mandate is achievable at the lower bound only if every dual-audit converges in one round; round-2 fix-then-proceed cycles (which is the *normal* pattern from PREREG-arc evidence — see A8 had 14 rounds) can each add 2–3 days. **This plan budgets for the median, not the optimistic case.**

**Mitigation**: S0 atom 0.6 (the gix-spike sub-atom) is a real working code prototype, not just paper design. If the spike reveals gix is unworkable for our case, S0 extends 3 days for a `git2-rs` switch — the Tape trait is library-agnostic, so this is a localized cost.

---

## § 4 — Per-Sprint Atom Enumeration

### Sprint S0 — Design + Spike + Decision Freeze (5–7 days)

| Atom | Scope | File paths touched | Dependencies | Cargo test gate | Dual-audit gate |
|---|---|---|---|---|---|
| **S0.1** | Land this master plan | `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md` | none | n/a (doc only) | required PASS/PASS — Codex + Gemini both review § 2–§ 4 |
| **S0.2** | Update `genesis_payload.toml` to mark `pput_accounting_0` calibration as deferred-during-TFR; update PREREG amendment per § 9 | `genesis_payload.toml`, `handover/preregistration/PREREG_AMENDMENT_TFR_2026-04-26.md` | S0.1 | `cargo test --workspace` green | PASS/PASS (audit verifies amendment text matches plan § 9) |
| **S0.3** | Cargo: add `gix = "0.66"` (or current stable) as workspace dep (feature-gated `tape-git`); CI builds both feature-on and feature-off | `Cargo.toml`, `experiments/minif2f_v4/Cargo.toml` | S0.1 | `cargo test --workspace` green; `cargo test --workspace --features tape-git` green | PASS/PASS |
| **S0.4** | gix spike: standalone bin `tools/gix_spike.rs` that creates a runtime repo, commits 100 fake nodes, walks the chain via gix, prints the SHAs. Validates gix supports our needs in <1 day. | `tools/gix_spike.rs` (new) | S0.3 | `cargo run --bin gix_spike` exits 0 | optional review (informational) |
| **S0.5** | TRACE_MATRIX_v3 seed — Art. 0 → code-symbol mapping | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md` | S0.1 | n/a (doc only) | PASS/PASS |
| **S0.6** | Trust Root manifest update — add this plan + amendment + TRACE_MATRIX_v3 to TR; remove no entries (yet) | `genesis_payload.toml [trust_root]` | S0.1, S0.2, S0.5 | `cargo test trust_root_immutability --workspace` green | PASS/PASS |
| **S0.7** | Mark Phase C C2 batch FROZEN — add commit-stop file `handover/freeze/C2_FROZEN_TFR_2026-04-26.md` that any C2-attempt script reads + halts on | `handover/freeze/C2_FROZEN_TFR_2026-04-26.md` (new), `experiments/minif2f_v4/src/bin/evaluator.rs` (add startup check) | S0.6 | `cargo test --workspace` green; manual: `RUN_MODE=c2_batch` exits with FROZEN message | PASS/PASS |

**Sprint S0 dependency graph**:
```
S0.1 ──┬──> S0.2 ──┐
       ├──> S0.3 ──> S0.4 (informational)
       └──> S0.5 ──┘
              └────> S0.6 ──> S0.7 ──> [GATE: dual-audit; S1 entry]
```

**S0 exit gate**: dual-audit on (1) the master plan (S0.1), (2) the gix-vs-git2 decision (with spike S0.4 evidence attached), (3) the PREREG amendment (S0.2). All three must PASS/PASS before S1 begins.

---

### Sprint S1 — Tape Trait + GitTape Skeleton + MemTape Rename (7–10 days)

**STEP_B_PROTOCOL applies**: this sprint touches `src/ledger.rs` (renamed) and creates new modules; every atom in S1 is a STEP_B-protocol commit (parallel branch, dual-audit per atom, no direct main edits).

| Atom | Scope | File paths | Dependencies | Cargo test gate | Dual-audit gate |
|---|---|---|---|---|---|
| **S1.1** | Define `Tape` trait + `NodeHandle` enum + `TapeError` extension (no impls yet) | `src/tape/mod.rs` (new) | S0 | `cargo build --workspace` green | PASS/PASS |
| **S1.2** | Move existing `Tape` struct from `src/ledger.rs` to `src/tape/mem_tape.rs` as `MemTape`; impl `Tape` trait for it | `src/tape/mem_tape.rs` (new), `src/ledger.rs` (delete struct, keep `Node`/`NodeId`/`Ledger`/etc.), `src/lib.rs` (re-export) | S1.1 | full `cargo test --workspace` — every existing test must pass; symbols `Tape`/`time_arrow`/`children`/`get`/`append`/`trace_ancestors` continue to resolve via re-exports | PASS/PASS |
| **S1.3** | Implement `GitTape::init` + `GitTape::append` (single-parent only) + `GitTape::head` + `GitTape::time_arrow`. Behind `#[cfg(feature = "tape-git")]`. | `src/tape/git_tape.rs` (new) | S1.1, S0.3 | `cargo test --workspace --features tape-git tape::git_tape` green; **MemTape tests still green without the feature** | PASS/PASS |
| **S1.4** | Implement `GitTape::get` + `GitTape::children` + `GitTape::get_chain` + `GitTape::contains` + `GitTape::len` | `src/tape/git_tape.rs` | S1.3 | new `tests/git_tape_parity.rs` — parameterized over both impls, asserts identical observable behavior | PASS/PASS |
| **S1.5** | Multi-parent commit support in `GitTape::append` (citations.len() > 1 → merge commit) | `src/tape/git_tape.rs` | S1.3 | `tests/git_tape_dag_branching.rs` mirrors existing `mem_tape::test_tape_dag_branching` behavior | PASS/PASS |
| **S1.6** | Add `head() -> Option<NodeId>` method on Tape trait + both impls; expose via `Kernel::head()` and `TuringBus::head()` (closes Art. 0.4 HEAD_t gap signature; not yet wired into bus.append flow) | `src/tape/mod.rs`, `src/tape/mem_tape.rs`, `src/tape/git_tape.rs`, `src/kernel.rs`, `src/bus.rs` | S1.4 | `cargo test --workspace [--features tape-git]` green | PASS/PASS |
| **S1.7** | TRACE_MATRIX_v3 update — every new pub symbol in S1 backlinked to FC1-N3/N4/N13 | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md`, source files (doc-comments) | S1.6 | `cargo test fc_alignment_conformance --workspace` green | PASS/PASS |

**Sprint S1 dependency graph**:
```
S1.1 ──> S1.2 ──> S1.3 ──> S1.4 ──> S1.5
                              └────> S1.6 ──> S1.7 ──> [GATE: dual-audit; S2 entry]
```

**S1 exit gate**: PASS/PASS dual-audit on S1.7 + a green `cargo test --workspace` AND `cargo test --workspace --features tape-git` AND `tests/git_tape_parity.rs` showing both impls produce identical results on a 50-commit fixture.

**STEP_B_PROTOCOL files touched in S1**: `src/ledger.rs` (destructive rename), `src/bus.rs` (S1.6 head method add), `src/kernel.rs` (S1.6 head method add). All require parallel branch + per-atom dual-audit.

---

### Sprint S2 — Node Schema v2 + WAL v2 + QState (7–10 days)

| Atom | Scope | File paths | Dependencies | Cargo test gate | Dual-audit gate |
|---|---|---|---|---|---|
| **S2.1** | `NodeKind` enum (12 variants per AUDITOR audit § Commit 1) + `NodeCost` struct (4 fields) + `Node` schema v2 with `kind: NodeKind`, `cost: NodeCost`, `kind_payload: serde_json::Value`, `created_at: u128` ms, `verified: bool` (default true) | `src/ledger.rs` (Node redef) | S1 exit | `cargo test --workspace` — existing tests use `NodeBuilder::default()` shim (S2.2) so don't break | PASS/PASS |
| **S2.2** | `NodeBuilder` with sensible defaults; `Node::legacy_from(author, payload, parent_id)` shim for old call sites | `src/ledger.rs`, `src/tape/mod.rs` | S2.1 | full `cargo test --workspace` green; old `Node { id, author, payload, citations, created_at, completion_tokens }` literals in tests rewritten to builder via `find/replace` (mechanical) | PASS/PASS |
| **S2.3** | WAL v2 schema with per-line SHA-256 hash chain (closes V-18); `wal::Wal::write_node` computes `prev_hash + serialize(node)` → `hash`; replay verifies | `src/wal.rs` | S2.1 | new `tests/wal_v2_hash_chain.rs` — tampering detection; mid-line corruption fails replay | PASS/PASS |
| **S2.4** | `QState` struct landed; **inert** (not yet wired into bus) | `src/q_state.rs` (new) | S2.1, S1.6 | `cargo test --workspace q_state` green | PASS/PASS |
| **S2.5** | `tape_canonical_round_trip` conformance test scaffold (closes audit recommendation 1) — synthesizes 5 NodeKind variants, writes through WAL v2, replays, asserts bit-identical reconstruction | `tests/tape_canonical_round_trip.rs` (new) | S2.3 | passes with both `MemTape` and `GitTape --features tape-git` | PASS/PASS |
| **S2.6** | TRACE_MATRIX_v3 — every new pub symbol backlinked; `NodeKind` variants each cited to AUDITOR § Commit-3..9 | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md` + source doc-comments | S2.5 | `cargo test fc_alignment_conformance --workspace` green | PASS/PASS |

**Sprint S2 dependency graph**:
```
S2.1 ──┬──> S2.2 ──> S2.5 ──> S2.6 ──> [GATE: dual-audit; S3 entry]
       ├──> S2.3 ────┘
       └──> S2.4 ────┘
```

**STEP_B_PROTOCOL files touched in S2**: `src/wal.rs` (S2.3), `src/ledger.rs` (S2.1, S2.2). Per-atom dual-audit on these two atoms specifically.

---

### Sprint S3 — Bus Integration (real GitTape in production) + Pre-commit Hooks (10–14 days)

**This is the heart of the migration. Highest risk sprint. STEP_B_PROTOCOL on every atom.**

| Atom | Scope | File paths | Dependencies | Cargo test gate | Dual-audit gate |
|---|---|---|---|---|---|
| **S3.1** | `TuringBus::with_runtime_repo(path)` constructor — initializes GitTape, replaces `kernel.tape` semantics. **Default path**: in-memory MemTape; explicit env `TURINGOS_TAPE=git` opts in. | `src/bus.rs`, `src/kernel.rs` | S2 exit | full `cargo test --workspace` green (default path unchanged); new `tests/bus_with_runtime_repo.rs` exercises git path | PASS/PASS |
| **S3.2** | `bus.append_v2(NodeBuilder) -> Result<NodeHandle>` — new API; old `bus.append` delegates internally | `src/bus.rs` | S3.1 | full `cargo test --workspace` — every existing append test passes via the shim | PASS/PASS |
| **S3.3** | Π_p as in-process pre-commit gate: forbidden patterns + size limits + tool veto run inside `bus.append_v2`'s GitTape path BEFORE `gix::commit`. Failed-branch path: commit with `verified: false` (closes V-03). | `src/bus.rs`, `src/tape/git_tape.rs` | S3.2 | new `tests/predicate_pre_commit_gate.rs` — verifies vetoed payloads do appear on tape with `verified: false` | PASS/PASS |
| **S3.4** | `.git/hooks/pre-commit` shell hook installer — `GitTape::init` writes the hook script; sanity-checks sidecar JSON consistency | `src/tape/git_tape.rs`, `src/tape/hooks/pre-commit.sh` (new) | S3.3 | `tests/git_pre_commit_hook.rs` — hook fires on shell `git commit` attempt | PASS/PASS |
| **S3.5** | Migrate `evaluator.rs::run_swarm` to construct bus via `with_runtime_repo` when `TURINGOS_TAPE=git`; legacy MemTape path retained for back-compat. Phase B baseline runs unchanged when env unset. | `experiments/minif2f_v4/src/bin/evaluator.rs` | S3.4 | full `cargo test --workspace`; full `cargo test --workspace --features tape-git`; manual: smoke a 1-problem oneshot under both modes; results identical except for tape backing store | PASS/PASS |
| **S3.6** | `QState` becomes the bus's *primary* state (S2.4 was inert; S3.6 wires it). `bus.append_v2` calls `QState::wtool` internally. | `src/bus.rs`, `src/q_state.rs` | S3.5 | full `cargo test --workspace [--features tape-git]` green | PASS/PASS |
| **S3.7** | `every_llm_call_has_tape_node` conformance test scaffold (closes audit V-22) — instruments `ResilientLLMClient::generate` and asserts 1:1 tape-node correspondence | `experiments/minif2f_v4/tests/every_llm_call_has_tape_node.rs` (new) | S3.6 | passes under both tape backends | PASS/PASS |
| **S3.8** | TRACE_MATRIX_v3 — Π_p / wtool / rtool / append_v2 all backlinked to FC1-N5 / N11 / N13 / N14 | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md` | S3.7 | `cargo test fc_alignment_conformance --workspace` green | PASS/PASS |
| **S3.9** | **Sprint exit ceremony**: full dual-audit + Trust Root manifest update + **Phase C C2 batch FROZEN file deletion** (unfreezes C2). | `handover/freeze/C2_FROZEN_TFR_2026-04-26.md` (delete), `genesis_payload.toml [trust_root]` | S3.8 | full `cargo test --workspace` ; full `cargo test --workspace --features tape-git`; `cargo test --workspace --release` (perf sanity); a 10-problem oneshot batch under TURINGOS_TAPE=git completes in <2× the in-memory runtime baseline | **PASS/PASS unfreezes Phase C C2 batch** |

**Sprint S3 dependency graph**:
```
S3.1 ──> S3.2 ──> S3.3 ──> S3.4 ──> S3.5 ──> S3.6 ──> S3.7 ──> S3.8 ──> S3.9 [UNFREEZE C2]
```

**STEP_B_PROTOCOL files**: every atom in S3 touches `src/bus.rs` or `src/kernel.rs` or `src/wal.rs` — every commit is parallel-branch + per-atom dual-audit.

---

### Sprint S4 — Economy on Tape (markets / wallet / invest as derived views) (7–10 days)

| Atom | Scope | File paths | Dependencies | Cargo test gate | Dual-audit gate |
|---|---|---|---|---|---|
| **S4.1** | `EventType::MarketCreate` + `MarketResolve` actually emitted from `bus`/`kernel` (closes V-04) | `src/bus.rs`, `src/kernel.rs` | S3 exit | new `tests/market_event_completeness.rs` (audit suggestion 4) | PASS/PASS |
| **S4.2** | `Invest` event detail with structured (amount, direction, shares) (closes V-05) | `src/bus.rs`, `src/ledger.rs` | S4.1 | new `tests/invest_event_provenance.rs` | PASS/PASS |
| **S4.3** | `evaluator.rs:1318-1336` direct `bus.kernel.buy_yes/buy_no` calls migrate through `bus.invest()` API which goes through tape (closes V-05 path 2) | `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/bus.rs` | S4.2 | `cargo test --workspace` green | PASS/PASS |
| **S4.4** | `WalletTool` becomes derived view — `wallet::reconstruct_from_tape(tape)` returns identical state to in-memory; conformance test asserts equality (closes V-14) | `src/sdk/tools/wallet.rs`, `tests/wallet_replay_invariance.rs` (new) | S4.3 | both backends green | PASS/PASS |
| **S4.5** | Founder grant + bounty market open/resolve become typed Node kinds (closes V-15 + V-16) | `src/bus.rs`, `src/kernel.rs` | S4.4 | `cargo test --workspace` green | PASS/PASS |
| **S4.6** | `bus.append` (legacy 3-arg) deleted; all callers on `append_v2`. **HARD MIGRATION POINT** — last sprint with legacy shims. | `src/bus.rs`, all callers | S4.5 | full `cargo test --workspace` green | PASS/PASS |
| **S4.7** | TRACE_MATRIX_v3 — economy events backlinked | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md` | S4.6 | `cargo test fc_alignment_conformance --workspace` green | PASS/PASS |

**STEP_B_PROTOCOL files**: every S4 atom touches `src/bus.rs`, `src/kernel.rs`, or `src/sdk/tools/wallet.rs` (a wallet-restricted file per Codex audit § 5 step 4).

---

### Sprint S5 — Side-State on Tape (search / board / FC trace / mr tick / Boltzmann) (7–10 days)

| Atom | Scope | File paths | Dependencies | Cargo test gate | Dual-audit gate |
|---|---|---|---|---|---|
| **S5.1** | mr tick → tape Node (closes V-08a) | `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/bus.rs` | S4 exit | new `tests/mr_tick_count_matches.rs` | PASS/PASS |
| **S5.2** | Synthetic treatment → tape Node (closes V-07) | `experiments/minif2f_v4/src/bin/evaluator.rs` | S5.1 | new `tests/synthetic_treatment_provenance.rs` | PASS/PASS |
| **S5.3** | Boltzmann pick → tape Node (closes V-08b) | `experiments/minif2f_v4/src/bin/evaluator.rs` | S5.2 | new `tests/boltzmann_provenance.rs` | PASS/PASS |
| **S5.4** | Search hit injection → tape Node (closes V-10); `search_cache` becomes derived view | `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/sdk/tools/search.rs` | S5.3 | `cargo test --workspace` green | PASS/PASS |
| **S5.5** | Librarian board / learned.md → derived projection (closes V-11) | `src/sdk/tools/librarian.rs` | S5.4 | new `tests/librarian_replay_invariance.rs` | PASS/PASS |
| **S5.6** | Lean error string + Halt detail on tape (closes V-19, V-21); legacy WAL deprecated (still readable for replay, no longer written) | `experiments/minif2f_v4/src/lean4_oracle.rs`, `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/wal.rs` | S5.5 | new `tests/halt_provenance.rs` | PASS/PASS |
| **S5.7** | Audit guard provenance on tape (closes V-24); `assert_no_metric_leak` writes `kind: AuditCheck` Node | `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/sdk/prompt_guard.rs` | S5.6 | new `tests/metric_leak_guard_provably_executed.rs` | PASS/PASS |
| **S5.8** | TRACE_MATRIX_v3 — all V-* violations now have closing-commit references | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md` | S5.7 | `cargo test fc_alignment_conformance --workspace` green | PASS/PASS |

---

### Sprint S6 — Phase E Gate Prep + Trust Root Migration + Arc Reopen (5–7 days)

| Atom | Scope | File paths | Dependencies | Cargo test gate | Dual-audit gate |
|---|---|---|---|---|---|
| **S6.1** | Heldout operational sealing on git substrate — verify L1–L5 layers from PREREG § 2.3 still hold under `runtime_repo` (e.g., agent prompt context cannot include git refs/objects/etc.) | `experiments/minif2f_v4/tests/heldout_operational_sealing.rs` extension | S5 exit | green | PASS/PASS |
| **S6.2** | Per-cell runtime repo lifecycle — full integration test: 1 problem × 50 tx → tarball → checksum the tarball SHA → reproduce by rerun → asserts identical SHA | `experiments/minif2f_v4/tests/runtime_repo_lifecycle.rs` (new) | S6.1 | green; runtime ≤ 2× in-memory baseline | PASS/PASS |
| **S6.3** | Trust Root migration — final manifest: add new TFR files (S0.1 plan, amendment, TRACE_MATRIX_v3, all new src/ files); remove now-deleted shims; `tests/trust_root_immutability.rs` updated | `genesis_payload.toml [trust_root]`, `tests/trust_root_immutability.rs` | S6.2 | green | PASS/PASS |
| **S6.4** | Phase C C2 unfreeze re-validation — run full hard-10 × 2-seed C2 batch with `TURINGOS_TAPE=git`; results dual-audited against the pre-TFR baseline (which is preserved in `discarded_12way_run_2026-04-24/`); accepts NEGATIVE finding if PPUT shifts (audit logs the diff) | `experiments/minif2f_v4/logs/pput_ccl_C2_post_TFR_*.jsonl` | S6.3 | n/a (data run) | external dual-audit on the comparison report |
| **S6.5** | PREREG arc clock restart per § 9 amendment text | `handover/preregistration/PREREG_AMENDMENT_TFR_2026-04-26.md` (status: ACTIVATED) | S6.4 | n/a (governance) | PASS/PASS |
| **S6.6** | Sprint S6 + TFR exit ceremony — close-out doc, lessons learned to `cases/C-XXX.yaml`, restart Phase C with the C2 batch | `cases/C-XXX_TFR_lessons.yaml` (new), `handover/architect-insights/TFR_EXIT_REPORT_2026-04-XX.md` (new) | S6.5 | full `cargo test --workspace [--features tape-git]` green | **PASS/PASS reopens 30-day Phase E arc** |

---

## § 5 — Team Organization (Internal + External)

### 5.1 Internal (Claude main + ArchitectAI subagents)

- **ArchitectAI** (Claude Opus 4.7, 1M context) — sole code-writer for `src/` and `experiments/minif2f_v4/src/`. Per Art. V.1.2, has commit authority on non-constitution files.
- **Claude main session (this thread)** — orchestration: invokes ArchitectAI for atoms, runs `validate` skill, runs `dev-cycle` skill, dispatches dual-audit packets.
- **`auditor` subagent** (Claude `auditor` mode, read-only) — internal first-pass audit on each atom before external dual-audit. Catches obvious regressions cheap.
- **`codex:rescue` subagent** — local fallback when Codex CLI is the easier path for a specific investigation.

### 5.2 External (Codex + Gemini)

Per CLAUDE.md "Audit Standard": every sprint exit + every commit before merge gets dual-audit packet.

- **Codex** (latest GPT-5-codex / GPT-5.2 family) — driven via `handover/audits/run_codex_TFR_audit.sh` (S0.1 ships the script). Verdict domain: `{PASS, CHALLENGE, VETO}`.
- **Gemini 2.5 Pro** — driven via `handover/audits/run_gemini_TFR_audit.py`. Same verdict domain.
- **Conservative merge**: VETO > CHALLENGE > PASS. CHALLENGE → fix-then-proceed. VETO → halt sprint, redesign.

### 5.3 Sprint exit roles (responsibility matrix)

| Sprint exit | ArchitectAI generates | auditor subagent (internal) | Codex (external) | Gemini (external) | User (`gretjia`) |
|---|---|---|---|---|---|
| S0 | plan + amendment + spike | reads, finds doc bugs | full review | full review | reads only; sudo on amendment text |
| S1–S5 | code + tests + TRACE_MATRIX update | per-atom diff review | per-sprint exit packet | per-sprint exit packet | reads sprint exit reports |
| S6 | TFR exit report | reads | full review | full review | sudo on Phase C unfreeze + arc reopen |

### 5.4 The "every commit before merge" cadence

Per the user mandate "every sprint design exit + every commit before merge". Practically:

- **Atom-level commits**: internal `auditor` subagent only. External dual-audit triggers at *sprint* exit (every 3–7 atoms), not per-atom.
- **STEP_B_PROTOCOL atoms** (those touching `src/{bus,kernel,wal,ledger}.rs`): external dual-audit per-atom mandatory.
- **Trust Root commits**: external dual-audit always.

Math: ~50 atoms across the arc. STEP_B atoms = ~25. Per-atom external audits = ~25; per-sprint exit audits = 7. Total external audit invocations ≈ 32, at ~30min each = 16 wall-hours of audit-runtime, plus fix-cycle latency. Budget envelope per audit-cycle is ~$8–15 USD at our backbone pricing → ~$300–500 total external audit spend across TFR. Within PREREG § 11 USD-500 cap, but tight — this is one of the open questions in § 10.

---

## § 6 — DO-178C TRACE_MATRIX_v3 Strategy

### 6.1 Goal

Every constitutional clause maps to ≥1 code symbol. Every public code symbol has a backlink to a constitutional clause OR is marked `orphan + justification`. No exceptions. This is DO-178C tool-qualification level discipline.

### 6.2 v3 baseline

S0.5 produces `TRACE_MATRIX_v3_2026-04-26.md` with two new top-level sections relative to v2:

- **§ Art. 0 mapping**: Art. 0.1 four-elements → code symbols
  - Paper → `src/tape/git_tape.rs::GitTape` (FC1-N4 v3)
  - Pencil → `src/bus.rs::TuringBus::append_v2` (FC1-N13 v3)
  - Rubber → `gix::commit` immutability + `verified: false` failed-branch nodes (FC1-N15 v3)
  - Discipline → `src/bus.rs::evaluate_predicates` + `.git/hooks/pre-commit` (FC1-N11 + new FC1-N11b "on-disk")
- **§ Art. 0.4 mapping**: Q_t = ⟨q_t, HEAD_t, tape_t⟩ → `src/q_state.rs::QState`
  - q_t → `QState::q_t` (FC1-N2)
  - HEAD_t → `QState::head_t` + `Tape::head()` (FC1-N3)
  - tape_t → `QState::tape_t: Box<dyn Tape>` (FC1-N4)
  - rtool → `QState::rtool` (FC1-N5)
  - wtool → `QState::wtool` (FC1-N13)

### 6.3 Per-sprint TRACE_MATRIX update protocol

Each sprint's last atom (S1.7, S2.6, S3.8, S4.7, S5.8, S6.6) is a TRACE_MATRIX update. The conformance test `tests/fc_alignment_conformance.rs` MUST go green at every commit; ignored stubs (`#[ignore]`) cover deferred rows. The matrix's `Status` column transitions:

- 📅 → 🔨 when a sprint claims the row as in-scope
- 🔨 → ✅ when the atom lands and conformance test green
- ✅ → ⚠️ if a regression detected (forces immediate fix-then-proceed)

### 6.4 Tooling

- **Backlink lint**: `rules/active/R-015_trace_matrix_pub_symbol.yaml` already enforces `/// TRACE_MATRIX <FC-id>: <role>` doc-comments on pub symbols. S0.5 extends this for the new `tape::*` and `q_state::*` modules.
- **Bidirectional check**: a new `tests/trace_matrix_bidirectional.rs` parses TRACE_MATRIX_v3, the rules manifest, and source doc-comments, and asserts:
  - For every constitutional clause in TRACE_MATRIX → ≥1 code symbol cites it.
  - For every code symbol → at most one TRACE_MATRIX row (no orphans without explicit justification).
- **Update at every atom**: per the rule, an atom that adds/removes a pub symbol MUST update the matrix in the same commit; `cargo test fc_alignment_conformance` enforces.

---

## § 7 — Trust Root Migration Plan

### 7.1 Pre-TFR Trust Root (post-A8e14, ~38 entries)

Already documented in `genesis_payload.toml [trust_root]`. Includes `src/{kernel,wal,bus}.rs`, `src/sdk/prompt_guard.rs`, `experiments/minif2f_v4/src/lean4_oracle.rs`, etc.

### 7.2 Per-sprint TR delta

| Sprint | Adds to TR | Removes from TR | Re-hashes (existing entries modified) |
|---|---|---|---|
| **S0** | `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md`, `handover/preregistration/PREREG_AMENDMENT_TFR_2026-04-26.md`, `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md`, `handover/freeze/C2_FROZEN_TFR_2026-04-26.md` | none | `Cargo.toml`, `experiments/minif2f_v4/Cargo.toml` (gix dep), `experiments/minif2f_v4/src/bin/evaluator.rs` (FROZEN check) |
| **S1** | `src/tape/mod.rs`, `src/tape/mem_tape.rs`, `src/tape/git_tape.rs`, `tests/git_tape_parity.rs` | none (ledger.rs renames stay; struct moves) | `src/ledger.rs`, `src/lib.rs`, `src/bus.rs`, `src/kernel.rs` (head method) |
| **S2** | `src/q_state.rs`, `tests/wal_v2_hash_chain.rs`, `tests/tape_canonical_round_trip.rs` | none | `src/ledger.rs` (Node v2), `src/wal.rs` (hash chain), `genesis_payload.toml [pput_accounting_0]` (TFR-deferred-calibration note) |
| **S3** | `src/tape/hooks/pre-commit.sh`, `tests/predicate_pre_commit_gate.rs`, `tests/git_pre_commit_hook.rs`, `tests/bus_with_runtime_repo.rs`, `experiments/minif2f_v4/tests/every_llm_call_has_tape_node.rs` | `handover/freeze/C2_FROZEN_TFR_2026-04-26.md` (S3.9) | `src/bus.rs`, `src/kernel.rs`, `experiments/minif2f_v4/src/bin/evaluator.rs` |
| **S4** | `tests/market_event_completeness.rs`, `tests/invest_event_provenance.rs`, `tests/wallet_replay_invariance.rs` | none | `src/bus.rs`, `src/kernel.rs`, `src/sdk/tools/wallet.rs`, `experiments/minif2f_v4/src/bin/evaluator.rs` |
| **S5** | `tests/mr_tick_count_matches.rs`, `tests/synthetic_treatment_provenance.rs`, `tests/boltzmann_provenance.rs`, `tests/librarian_replay_invariance.rs`, `tests/halt_provenance.rs`, `tests/metric_leak_guard_provably_executed.rs` | none | `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/sdk/tools/search.rs`, `src/sdk/tools/librarian.rs`, `experiments/minif2f_v4/src/lean4_oracle.rs`, `src/wal.rs` (deprecation note), `src/sdk/prompt_guard.rs` |
| **S6** | `tests/runtime_repo_lifecycle.rs`, `cases/C-XXX_TFR_lessons.yaml`, `handover/architect-insights/TFR_EXIT_REPORT_2026-04-XX.md` | none | `genesis_payload.toml [trust_root]` (final shape), `tests/trust_root_immutability.rs`, `experiments/minif2f_v4/tests/heldout_operational_sealing.rs` |

### 7.3 Tampering protection during TFR

The Trust Root manifest is a moving target during TFR (~50 add+modify operations). Two protections:

1. **Per-sprint dual-audit on the manifest delta**: at every sprint exit, the dual-audit packet *includes* the TR diff. Auditors verify the additions are TFR-relevant and the re-hashes match the actual file SHAs.
2. **Boot-time `verify_trust_root` always green**: `cargo test trust_root_immutability --workspace` green at every commit. This is the same gate that prevents in-flight tampering; it just runs more often.

### 7.4 Post-TFR final TR shape

After S6.3, the TR is ~50 entries (38 + ~15 new − ~3 removed). The new entries cluster around the new `src/tape/` module + the conformance tests + the TFR governance docs.

---

## § 8 — Risk Register

| ID | Risk | Probability | Impact | Mitigation | Trigger / monitor |
|---|---|---|---|---|---|
| **R1** | gix capability gap discovered mid-S1 | Med | High (1–2 day slip + library swap) | S0.4 spike validates the API surface BEFORE S1 begins. Fallback to git2-rs is library-localized. | S0.4 spike outcome |
| **R2** | gix performance is materially worse than in-memory | Med | High (S3.9 perf gate fails, blocks unfreeze) | S3 atom 3.5 includes a perf-baseline run; if `>2× slowdown`, S3.9 demands either gix optimization or batch commit (multi-Node-per-commit) refactor | S3 perf benchmarks |
| **R3** | dual-audit cycles each take longer than 30min (Codex + Gemini) and budget overrun | Med | Med | per-sprint audit not per-atom (except STEP_B). Conservative atom batching. Budget tracking via `handover/audits/USD_BUDGET_TFR_2026-04-26.md` updated weekly | weekly USD spend check |
| **R4** | A novel `kind` variant added in S2 / S5 forces a v3 schema → breaking change | Low | High | `kind: NodeKind` is an open enum; new variants are non-breaking. `kind_payload: serde_json::Value` is schemaless. v2 → v3 only if Node *struct fields* change, which is post-TFR territory. | code review |
| **R5** | Phase C C2 batch unfreeze (S3.9) reveals architectural drift between pre/post-TFR results | Med | Med | S6.4 explicitly includes a comparison report. NEGATIVE finding accepted (per PREREG § 11 stopping rule). User decides whether to proceed. | S6.4 audit |
| **R6** | TRACE_MATRIX_v3 maintenance burden becomes unsustainable | Low | Med | bidirectional conformance test (S0.5) catches drift mechanically; doc updates are atom-level (not separate sprints). Worst case: defer non-critical rows to v4. | per-sprint conformance test green |
| **R7** | Phase E heldout operational sealing breaks under git substrate (e.g., commit messages leak heldout problem IDs) | Med | Catastrophic (arc invalid) | S6.1 explicitly tests this. Pre-flight grep over runtime repo content for heldout IDs at boot. | S6.1 dual-audit |
| **R8** | The 8-week budget overruns | High | Low–Med | This plan budgets 8 weeks honestly with a 2-week buffer. Per-sprint timeline slip is expected; user-visible reporting via `handover/architect-insights/TFR_PROGRESS_<week>.md` weekly | weekly progress doc |
| **R9** | A constitutional amendment surfaces during TFR forcing TFR-itself amendment | Low | High | Art. V.3 amendment process is explicit. If constitution edits are needed mid-TFR, sprint S0 atom 0.X "constitutional addendum" added on-the-fly with user sudo. | every architect-ingest cycle |
| **R10** | gix does not support multi-parent commits in a way that round-trips cleanly | Low | Med | Verified in S0.4 spike. Fallback: single-parent only, citations stored only in sidecar JSON (looses some `git log --graph` ergonomics but functional) | S0.4 spike output |
| **R11** | The runtime-repo per-cell scheme accumulates disk space over long arcs | Low | Low | size budget § 2.2 calculated; tarball + delete after archival | weekly disk-space monitoring during S3.5 onwards |
| **R12** | A subtle race / non-determinism introduced by gix commit-author/timestamp env-var handling | Low | High (reproducibility break) | S2.5 round-trip conformance test exercises this on every commit. CI sets `GIT_AUTHOR_DATE` + `GIT_COMMITTER_DATE` deterministically per Node. | per-atom CI green |
| **R13** | Boltzmann-as-branch (S5+) doesn't ship in time | High | Low | Single-branch fallback design covers S0–S4. Boltzmann-on-tape is sufficient for Phase E reproducibility; branch ergonomics are post-TFR2 stretch. | S5 entry |
| **R14** | Codex / Gemini disagree on a key architectural decision | Med | Med | Conservative merge: VETO > CHALLENGE > PASS. If both VETO, halt sprint, redesign. Historical data shows ~2 of 14 A8 rounds had structural disagreement and converged within fix-then-proceed. | every dual-audit |

---

## § 9 — PREREG Amendment Text (proposing arc extension)

The user mandate explicitly named this. I provide **two** proposals — the user must explicitly choose between them, OR sudo-amend with their own variant.

### 9.1 Proposal A: Arc Extension (recommended)

> **PREREG_AMENDMENT_TFR_2026-04-26.md**
>
> § 1. Trigger: 2026-04-26 architectural amendment (commit `273b362`) added Art. 0.4 declaring Q_t a version-controlled triple. The substrate refactor required to land Art. 0.4 (the TuringOS Foundation Refactor / TFR) is estimated at 6–8 wall-clock weeks per `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md`. The 30-day arc clock cannot reasonably proceed during TFR because (i) Phase C C2 batch is FROZEN through TFR sprint S3, (ii) Phase D shadow runs are paused through TFR sprint S5, (iii) Phase E heldout operational sealing must be re-validated under the new substrate per TFR sprint S6.1.
>
> § 2. Amendment scope: extend the 30-day arc clock by the wall-clock duration of TFR (sprints S0 through S6.4, capped at **10 weeks = 70 days**). Phase A5 commit gate's "30 days from PASS/PASS" timer pauses at TFR commit S0.7 (C2 batch FROZEN file landing) and resumes at TFR commit S6.5 (arc-clock-restart governance landing). All Phase A through Phase D work performed before TFR remains valid (no re-do); Phase C C2 batch results pre-TFR are preserved (`discarded_12way_run_2026-04-24/`) but not used for inferential decisions; the post-TFR re-run (S6.4) produces the canonical Phase C C2 dataset.
>
> § 3. Pause-condition explicit: any TFR sprint dual-audit VETO halts both the TFR clock and the arc clock. Resumption requires VETO-fix-then-proceed dual-audit PASS/PASS.
>
> § 4. Family-size invariant: TFR does not modify N_max = 34 or k_max = 10 (per § 9 of PREREG). Hypotheses H1–H4 + per-artifact j-PPUT/j-FAR/j-CPR remain frozen. The substrate change is statistically transparent: nothing about Holm-Bonferroni stepwise procedure or the inferential family construction is affected.
>
> § 5. Heldout integrity: heldout-54 list is NOT re-drawn during TFR. The heldout SHA-256 (`51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b`) remains the canonical seal. TFR sprint S6.1 verifies that the new git substrate does not leak heldout IDs into prompt context, tool-call args, or commit messages (extending PREREG § 2.3 L1–L5 layers).
>
> § 6. USD budget extension: PREREG § 11 USD-500 cap raised by **+$300** (to $800) to cover TFR per-sprint dual-audit invocations (~$300 budgeted in TFR_MASTER_PLAN § 5.4). Arc-original Phase A–E budget envelope unchanged at $500.
>
> § 7. Trust Root migration: `genesis_payload.toml [trust_root]` is mutated 7 times during TFR (one per sprint exit). Each mutation is dual-audited. Boot's `verify_trust_root` continues to pass at every commit. Post-TFR final TR shape (~50 entries) is the new canonical baseline.
>
> § 8. Reporting discipline: weekly progress reports in `handover/architect-insights/TFR_PROGRESS_<week>.md` documenting (i) atoms shipped, (ii) audit verdicts, (iii) USD spend, (iv) schedule slip. Public to user; user has standing veto authority.
>
> § 9. Fallback: if TFR overruns 70 days, this amendment is voided and the arc reverts to a NEGATIVE-finding declaration per PREREG § 11 wall-clock cap. User has sudo to amend further.
>
> § 10. Activation: this amendment activates at TFR sprint S0.2 commit. It deactivates at TFR sprint S6.5 commit. Period.

### 9.2 Proposal B: Scope Reduction (not recommended, but presented for explicit user decision)

> Alternative if the user wishes to preserve the original 30-day arc clock: declare Phase E artifact-collection arc as a **NEGATIVE finding ("infrastructure-pivot, capability-not-demonstrated")** at the moment TFR begins (S0.2 commit). Publish CCL-1 negative paper per PREREG § 7 Gate H fallback. Treat TFR as a separate post-publication arc with its own, fresh PREREG.
>
> **Cost**: paper publishes a non-result; but TFR begins immediately with no arc-clock pressure.
>
> **Benefit**: preserves Phase E integrity strictly; no amendment needed.
>
> **My architectural recommendation: Proposal A**. The Phase E hypothesis is premature without Art. 0.4 substrate; declaring NEGATIVE before testing is statistically wasteful.

---

## § 10 — Decision Log + Open Questions for User

### 10.1 Decisions captured by this plan

| # | Decision | Status |
|---|---|---|
| D1 | Path B confirmed | User decision 2026-04-26, captured in Art. 0.4 |
| D2 | gix > git2-rs (with git2-rs fallback) | ArchitectAI design; **needs S0.4 spike to confirm** |
| D3 | Per-cell runtime repo at `experiments/<problem>/<run_id>/runtime_repo/` | ArchitectAI design |
| D4 | Node ↔ git mapping: commit + sidecar JSON; citations as multi-parent | ArchitectAI refined design (challenges user proposal mildly) |
| D5 | `Tape` trait with `MemTape` + `GitTape` implementations | ArchitectAI design |
| D6 | `QState` struct landed in S2; wired in S3 | ArchitectAI design |
| D7 | Π_p as in-process gate AND on-disk hook (belt-and-suspenders) | ArchitectAI design |
| D8 | 7-sprint structure (S0–S6) | ArchitectAI refines user proposal |
| D9 | STEP_B_PROTOCOL on every atom touching `src/{bus,kernel,wal,ledger}.rs` | per CLAUDE.md, mandatory |
| D10 | Per-sprint dual-audit + per-STEP_B-atom dual-audit | per CLAUDE.md, mandatory |
| D11 | TRACE_MATRIX_v3 with bidirectional conformance | per Art. IV / DO-178C |
| D12 | Phase C C2 batch FROZEN at S0.7, unfrozen at S3.9 | per user mandate |

### 10.2 Open questions — user must explicitly decide

| # | Question | Options | My recommendation |
|---|---|---|---|
| **Q1** | PREREG amendment | A (arc extension +70d; rec.) / B (scope reduction; declare NEGATIVE now) / C (sudo your own variant) | **A** |
| **Q2** | gix vs git2-rs at S0.4 spike outcome | (a) accept gix and proceed / (b) switch to git2-rs / (c) defer to user review | (a) if spike green; auto-(b) if spike fails |
| **Q3** | USD budget extension (+$300 total to $800) | yes / no / different amount | yes — $300 is the median estimate, +50% buffer if pre-spent |
| **Q4** | TFR exit ceremony (S6.6) — restart Phase C with C2 batch immediately, or insert a "Phase C-prime" review week first | (a) restart C2 immediately / (b) 1-week review week / (c) wait for user review | **(a)**, with the proviso that S6.4 already includes a pre/post comparison report |
| **Q5** | Boltzmann-as-branch (S5.x) — pursue in S5 or defer to TFR2 | (a) pursue / (b) defer | **(b) defer**; Boltzmann-on-tape via `kind: BoltzmannPick` Nodes is sufficient for Phase E reproducibility |
| **Q6** | Per-cell repo cleanup policy | (a) keep all forever / (b) tarball + delete after run / (c) tarball + 30-day retention then delete | **(c)** — disk efficient, archival-safe |
| **Q7** | Path A fallback if TFR overruns 70 days | (a) declare NEGATIVE per PREREG § 11 / (b) sudo-amend further / (c) salvage as Path A semantic version | (a) per PREREG; (b) is escape-hatch if the data justifies |
| **Q8** | Heldout list rotation if TFR-induced changes affect operational sealing | (a) keep current heldout-54 / (b) re-draw with new seed / (c) defer decision to S6.1 | **(c) defer**; S6.1 will produce evidence either way |
| **Q9** | Multi-key SiliconFlow + heterogeneous-LLM (Phase D) — paused or continues during TFR | (a) paused (consistent with C2 freeze) / (b) continues independently / (c) up to user | **(a)** — TFR substrate change might affect provider integration; Phase D shadow has nothing to gain meanwhile |
| **Q10** | TRACE_MATRIX_v3 living vs frozen during TFR | (a) updated per-atom (current plan) / (b) frozen at S0.5, single update at S6.6 | **(a)** — DO-178C requires per-change matrix update |

### 10.3 Items that DO NOT need user decision (ArchitectAI proceeds with defaults)

- All implementation details inside § 2.4–§ 2.10 (Tape trait API surface, internal structures)
- Per-atom file-path choices in § 4 (these are mechanical)
- Conformance test names + locations (mechanical)
- Internal `auditor` subagent invocation cadence
- Documentation style + commit message phrasing
- Cargo feature flag naming (`tape-git`)
- Per-sprint TRACE_MATRIX update wording

---

## Final Check: Plan Self-Audit

- [x] § 1 mission + Path B confirmation — done
- [x] § 2 architectural design — Tape trait + GitTape impl + Q_t mapping + Π_p as pre-commit hook + Node↔commit mapping — done with refinements
- [x] § 3 sprint structure with timeline — 7 sprints, 48–68 days, ~7–10 weeks
- [x] § 4 per-sprint atom enumeration — ~50 atoms with file paths + dependency graph + cargo test gate + dual audit gate
- [x] § 5 team organization — internal Claude + external Codex/Gemini, audit cadence, budget envelope
- [x] § 6 DO-178C TRACE_MATRIX_v3 strategy — bidirectional conformance, per-sprint update protocol
- [x] § 7 Trust Root migration plan — per-sprint TR delta, tampering protection
- [x] § 8 risk register — 14 risks with probability + impact + mitigation + monitor
- [x] § 9 PREREG amendment text — Proposal A (recommended) + Proposal B (alternative)
- [x] § 10 decision log + open questions — 12 captured + 10 open

End of master plan v1. Awaiting:
1. User Q1–Q10 decisions
2. External dual-audit (Codex + Gemini) on this document — gate for S0.1 commit and S0 sprint kickoff.

---

### Critical Files for Implementation

The five files most critical for implementing this plan (absolute paths):

- /home/zephryj/projects/turingosv4/src/ledger.rs (Node schema + MemTape extraction at S1; v2 schema at S2)
- /home/zephryj/projects/turingosv4/src/bus.rs (the integration heart; STEP_B every atom in S3+)
- /home/zephryj/projects/turingosv4/src/kernel.rs (tape ownership + market events on tape, S3+S4)
- /home/zephryj/projects/turingosv4/src/wal.rs (v2 hash chain at S2; deprecated at S5.6)
- /home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs (the ultimate caller; migrates through S3.5–S5.7)
---

## Appendix A — User's U1..U9 Atomization Integration

User initially proposed a 9-atom Path-A-shaped atomization (`U1..U9`) before choosing Path B. Those atoms are **subsumed but extended** by the Path-B sprint structure. Mapping for traceability:

| User U-atom | Path-B sprint atom | Notes |
|---|---|---|
| **U1** Node.completion_tokens 字段 mandatory + bus.append* 签名扩展 | **S2.1** (Node v2 with `cost: NodeCost`) + **S3.2** (`append_v2`) | Path B replaces flat `completion_tokens: u32` with structured `NodeCost { prompt_tokens, completion_tokens, tool_stdout_tokens, latency_ms }`; signature change mandated under STEP_B. U2 closed by U1 in Path B. |
| **U2** Node.prompt_tokens 字段新增 | **S2.1** (NodeCost includes `prompt_tokens`) | Same as U1 — folded into NodeCost. |
| **U3** evaluator.rs `tape_tokens = payload.len()` hack 替换 | **S3.5** (evaluator migration) + **S4.6** (legacy bus.append delete; payload.len() hack dies with it) | The hack gets deleted entirely when GitTape is canonical and `Σ n.cost.completion_tokens` becomes the source of truth. |
| **U4** RunCostAccumulator → derived view + assert_eq | **S2.5** (tape_canonical_round_trip test) + **S4.4** (wallet_replay_invariance — same pattern for wallet derived view) | The "derived view + cross-validation" pattern propagates across all parallel ledgers (cost, wallet, market, search, librarian). |
| **U5** 新 conformance test tape_canonical_ledger_invariant.rs | **S2.5** (`tests/tape_canonical_round_trip.rs`) + S2 onwards conformance suite | Each derived-view atom adds its own conformance test (10 total per audit recommendation). |
| **U6** WAL fixtures + replay 测试更新 | **S2.3** (WAL v2 with hash chain) + **S5.6** (WAL deprecation when GitTape becomes canonical) | git's commit DAG IS the WAL after S5; legacy WAL kept readable for replay of pre-TFR jsonl. |
| **U7** jsonl schema doc + gp_token_count 语义澄清 | **S2.6** (TRACE_MATRIX) + **S4.6** (legacy bus.append delete cleans gp_token_count proxy) | The `gp_token_count = Σ tape n.cost.completion_tokens` definition is constitutional after S3.5. |
| **U8** TRACE_MATRIX_v3 — Art. 0 → src/ledger.rs::Node | **S0.5** (TRACE_MATRIX_v3 seed) + per-sprint exit atom (S1.7, S2.6, S3.8, S4.7, S5.8, S6.6) | Bidirectional conformance test enforces every commit. |
| **U9** Node.hash 字段 schema 语义槽位 | **CLOSED BY DESIGN** in Path B — Plan § 2.3 row "node.hash → commit SHA itself" | git's commit SHA *is* the Merkle hash. Art. 0.3 reservation closes by structural fact rather than added field. The hash chain (Art. 0.3 Phase E+) is the git commit DAG. |

**Conclusion**: every U-atom maps into the S* sprint structure with clear closure. The Path B plan is a strict super-set of U1..U9; nothing is dropped.

---

## Appendix B — Documents to be created during S0 (atom-by-atom)

| S0 atom | New file path | Purpose |
|---|---|---|
| S0.1 | `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md` (this doc) | canonical plan |
| S0.2 | `handover/preregistration/PREREG_AMENDMENT_TFR_2026-04-26.md` | arc extension; per § 9 Proposal A |
| S0.3 | `Cargo.toml` (modify) + `experiments/minif2f_v4/Cargo.toml` (modify) | gix dep |
| S0.4 | `tools/gix_spike.rs` | spike validation |
| S0.5 | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md` | bidirectional mapping seed |
| S0.6 | `genesis_payload.toml` (modify; manifest delta per § 7.2) | TR migration step 1 |
| S0.7 | `handover/freeze/C2_FROZEN_TFR_2026-04-26.md` + `experiments/minif2f_v4/src/bin/evaluator.rs` (startup check) | Phase C C2 freeze enforcement |

S0 sprint exit ceremony: dual external audit on (S0.1, S0.2, S0.4 spike outcome, S0.5) → PASS/PASS gates S1.
