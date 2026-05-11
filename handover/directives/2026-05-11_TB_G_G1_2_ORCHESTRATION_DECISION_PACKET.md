# TB-G G1.2 — Orchestration Architecture Decision Packet (2026-05-11)

> **Class**: 0 (this packet itself; decision affects Class-3 atom G1.2 implementation).
>
> **Status**: AWAITING ARCHITECT DECISION. G1.2 implementation HALTED until §11 questions are answered.
>
> **Reason for HALT**: TB-G G1.1 §8 packet §6 authorized "G1.2 (batch driver binary; Class 3) — autonomous after G1.1 ships". But the orchestration model itself (single-process loop vs subprocess-with-resume) is architecturally substantive — it determines whether G1.2 honors architect §2.8 verbatim ("one process / one runtime_repo / one CAS / one chain / multiple tasks") in the strict reading, OR redefines that boundary by extending the G1.1 resume-primitive precedent across N production-LLM problems. Per CLAUDE.md §9 + §10, the implementation cannot extrapolate this decision; per user 2026-05-11 verbatim "如 packet 或 charter 描述与实际代码不一致, STOP 问 user 而不是 extrapolate", impl HALTs here.

---

## §0. Header

- **TB**: TB-G atom **G1.2** (batch_evaluator binary)
- **Charter**: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §1 Module G1
- **Architect directive**: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
- **Parent §8**: G1.1 packet §6 SIGNED 2026-05-11 ("好，确认可以 ship")
- **Class**: 3 (production wire-up; binary-layer) per `feedback_risk_class_audit`. **OPEN QUESTION** (§11 Q4): does the ~3000-line lift in Option A/C push G1.2 to Class-4 STEP_B?
- **STEP_B branch (forward)**: `feat/g1-2-batch-evaluator` (not yet cut)
- **`origin/main` HEAD at packet draft time**: `5f90171` (post G1.1 ship)
- **Phase-id**: P3-G (RSP Economy Generative Arena)
- **FC-trace**: FC1 Runtime Loop (per-problem L4/L4.E externalization preserved) + FC2 Boot (cross-problem chain continuity; resume primitive role TBD per decision)

---

## §1. The decision required from the architect

**One-sentence question**: For G1.2 batch_evaluator, which of three orchestration models do you authorize?

- **Option A** — single-process in-memory loop (architect §2.8 strict reading)
- **Option B** — subprocess orchestrator with `TURINGOS_CHAINTAPE_RESUME=1` between invocations (G1.1 mini-smoke precedent)
- **Option C** — hybrid (in-process default + subprocess opt-in)

The choice is load-bearing because it determines whether the §2.8 verbatim "如果它只是一个 process 里启动多个 subprocess, 每个 subprocess 自己起 chain, 那不合格" passage applies to the G1.1-resume-primitive case (where each subprocess attaches to the **same** chain, not "起 own chain").

---

## §2. Background — what already shipped

| Atom | Status | Surfaces | Evidence |
|------|--------|----------|----------|
| **G0.1** TB-G charter | ✅ LANDED | `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` | session #39 (HEAD `2c110dc`) |
| **G0.2** G-Phase directive | ✅ LANDED | `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` (586 lines) | session #39 |
| **G0.3** Matrix §R rows | ✅ LANDED | `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R (9 rows) | session #39 |
| **G1.1** Resume-mode genesis branch | ✅ SHIPPED FINAL | `src/runtime/mod.rs` + `src/state/sequencer.rs::new_at_logical_t` + `src/runtime/agent_keypairs.rs::resume_existing_durable` + `tests/constitution_g1_resume.rs` (8 SG-G1.* gates) | sessions #39 + #40; HEAD `5f90171`; constitution gates 310/0/1; aggregate audit_tape PROCEED on 3-problem mini smoke |
| **G1.2** batch_evaluator | ⏸ AWAITING THIS DECISION | TBD per §11 Q1 | n/a |
| **G1.3** wrapper script | ⏸ blocked by G1.2 | `scripts/run_g_phase_batch.sh` (NEW) | n/a |
| **G1.4** persistence-evidence binding test | ⏸ blocked by G1.2 | `tests/constitution_g1_persistence_evidence_binding.rs` (NEW; SG-G1.11..G1.15) | n/a |

---

## §3. Constitutional evidence

### §3.1 Architect §2.8 verbatim (TB-18 ruling 2026-05-05)

Quoted at three live source-code anchors (single source of truth still in architect ruling archive; copies in code docstrings):

- `experiments/minif2f_v4/src/bin/comprehensive_arena.rs:7..11` (Phase 4 Atom B docstring)
- `experiments/minif2f_v4/src/chain_runtime.rs:7..9` (Atom B Phase 1 docstring)
- `experiments/minif2f_v4/src/drive_task.rs:11..14` (Atom A re-entrant API docstring)

Verbatim text:

> **Atom B 要证明的是: one evaluator process / one runtime_repo / one CAS / one chain / multiple tasks. 如果它只是一个 process 里启动多个 subprocess, 每个 subprocess 自己起 chain, 那不合格.**

**Critical clause for this decision**: "每个 subprocess **自己起 chain**". The forbidden pattern is subprocess-starts-its-own-chain. G1.1 resume primitive establishes a path where subprocess #N+1 does NOT start its own chain — it attaches to subprocess #1's chain via `TURINGOS_CHAINTAPE_RESUME=1`.

### §3.2 G-Phase directive §1.3 — empty-market 涌现 prerequisite

`handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md:105` verbatim:

> 每个 problem 都 fresh runtime_repo + fresh genesis → "每一轮开局都把交易员洗白、清仓、重置记忆。那市场不会涌现."

`handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md:226..230` Halt conditions:

> ```
> per-problem isolation reappears
> fresh genesis per problem
> positions reset silently
> balance reset silently
> ```

**Critical clause for this decision**: the failure mode is "fresh **genesis** per problem" — not "fresh process per problem". The architect's framing in G-Phase explicitly identifies *genesis reset* (= new on-init mint + balances clear + positions clear + state_root zero) as the kill condition, not *new process invocation*. G1.1 resume primitive prevents genesis reset across subprocess boundaries.

### §3.3 G1.1 binary-layer scope expansion authorization (user verbatim 2026-05-11)

Preserved in `handover/ai-direct/LATEST.md` session #40:

> 断点续作是本项目的核心。如果连断点续作都达不到了，那我们的图灵机，我们的tape存在的意义是什么呢？从图灵机原教旨主义角度去解决这个tape问题。首先，对齐宪法。

This authorized the **binary-layer** scope expansion in G1.1 R1.5: `AgentKeypairRegistry::resume_existing_durable` + `chain_runtime.rs` branching on `TURINGOS_CHAINTAPE_RESUME=1`. Both layers (kernel + binary) now support resume.

**Critical clause for this decision**: "Turing-machine fundamentalist" framing of FC2 §3.2 ("every real evidence run must be replayable from genesis_report + ChainTape + CAS + agent registry + system pubkeys") promotes the resume primitive to a first-class boundary, not a corner case. Option B (subprocess + resume) exercises this boundary on every problem #2..N; Option A relegates it to crash-recovery only.

### §3.4 G1.1 mini-smoke established precedent (sessions #40 ship evidence)

`handover/evidence/g_phase_g1_1_smoke_2026-05-11T13-01-12Z/` shape:

```
g_phase_g1_1_smoke_2026-05-11T13-01-12Z/
├── G1_1_SMOKE_MANIFEST.json
├── G1_1_SMOKE_SUMMARY.json
├── aggregate_verdict.json     ← audit_tape verdict=PROCEED
├── audit_tape.stderr
├── runtime_repo/              ← SHARED across P01..P03
├── cas/                       ← SHARED across P01..P03
├── P01_mathd_algebra_107/     ← subprocess #1; fresh bootstrap
├── P02_mathd_algebra_125/     ← subprocess #2; RESUME mode
└── P03_mathd_algebra_141/     ← subprocess #3; RESUME mode
```

LATEST.md §40 reports cumulative shared tape: **5 L4 + 19 L4.E + 83 CAS objects across ONE persistent runtime_repo + ONE shared CAS**; chain monotonically grew 0 → 3 → 4 → 5 across subprocess boundaries; pinned_pubkeys.json epoch 1 + 2 + 3 entries all coexist; aggregate `audit_tape verdict=PROCEED`. **Architect §8 SIGNED this evidence shape.**

**Critical clause for this decision**: the architect explicitly ratified ("好，确认可以 ship") a packet whose §6 ship-condition (b) was "3-problem mini real-LLM smoke with `TURINGOS_CHAINTAPE_RESUME=1`" — i.e., subprocess-with-resume was already in scope at §8 sign-off. This is the strongest precedent in favor of Option B's compliance.

---

## §4. Code evidence — what's already in place

### §4.1 `evaluator.rs` `run_swarm` boundaries

- File: `experiments/minif2f_v4/src/bin/evaluator.rs`
- Total lines: 5280
- `run_swarm` entry: line **829** (`async fn run_swarm(...)`)
- `run_swarm` close: line **4708** (column-0 `}` after the function body)
- Body size: **3880 lines**

The user prompt range "evaluator.rs:829..1700" only covers function entry + Phase 1 destructure + preseed setup + initial round logic — **NOT** the per-problem swarm cycle in its entirety. Lines 1700..4708 contain: Boltzmann routing, per-LLM-call dispatch, parse-error/llm-error/lean-success branches, WorkTx submission, VerifyTx submission, Lean error classification, AttemptTelemetry CAS write, ProposalTelemetry CAS write, MarketDecisionTrace emission, terminal-summary emission, and ~3500 lines of state shuttling.

### §4.2 `comprehensive_arena.rs` precedent (Phase 4)

- File: `experiments/minif2f_v4/src/bin/comprehensive_arena.rs`
- Total lines: 1071
- Class: 3 per its own docstring line 70..73 (`feedback_class4_cannot_hide_in_class3` — consumes existing APIs, no admission / typed-tx / signing-payload change)

Docstring lines 4..30 (verbatim):

> ## Why this binary exists in its current form
>
> Per architect §2.8 verbatim:
>
> > Atom B 要证明的是：one evaluator process / one runtime_repo /
> > one CAS / one chain / multiple tasks. 如果它只是一个 process 里启动多个
> > subprocess，每个 subprocess 自己起 chain，那不合格.
>
> The pre-TB-18 `comprehensive_arena.rs` (TB-16 Atom 5 scaffold) was a
> subprocess-spawn orchestrator — each task subprocessed `evaluator`,
> creating one chain per task. That is **architecturally non-compliant**
> with the §2.8 mandate.
>
> TB-18 Atom B Phase 4 (this commit) rewrites the binary as a single-process
> multi-task in-memory driver:
>
>   1. `SharedChain::from_env()` — single bundle init (Phase 1 lift; see `chain_runtime.rs`).
>   2. `write_synthetic_l4_l4e_gate_and_genesis_report` ONCE with a chain-level seed_id (Phase 2 lift).
>   3. For each of 6 engineered tasks {A,B,C,D,E,F}: call `drive_task` (TaskOpen + EscrowLock per-task scaffold; Phase 3) + emit the task-specific lifecycle txs via direct `bus.submit_typed_tx` + `bundle.sequencer.emit_system_tx` calls. All 6 tasks share the same bundle.
>   4. `bundle.shutdown()` ONCE at chain end.

Docstring lines 50..61 (verbatim, on why no LLM loop):

> Per `feedback_chaintape_externalized_proposal`: the chain records what
> the system externalized via `submit_typed_tx`, not LLM internals. The
> 6-task engineered set produces the 13/13 tx kinds via **real-signed synthetic envelopes**...

**Critical observation**: `comprehensive_arena.rs` is the canonical Phase 4 §2.8-compliant single-process driver pattern, **but it deliberately skipped the real-LLM agent loop**. G1.2 must add real-LLM agent loop on top of this pattern (or pick a different pattern entirely).

### §4.3 Phase 1+2+3 lifts already in place

| Phase | Surface | File | Lines |
|-------|---------|------|-------|
| Phase 1 | `SharedChain::from_env` — Kernel + BusConfig + ChaintapeBundle + AgentKeypairRegistry + TuringBus | `src/chain_runtime.rs` | full file (~580 lines) |
| Phase 2 | `write_synthetic_l4_l4e_gate_and_genesis_report` — synthetic TaskOpen + zero-stake WorkTx + genesis_report.json | `src/chain_runtime.rs` lines 402..533 | ~130 lines |
| Phase 3 | `drive_task(chain, spec, budget)` — TaskOpen + EscrowLock per-task scaffolder | `src/drive_task.rs::drive_task` lines 175..285 | ~110 lines |

What is **NOT** lifted yet: the LLM agent loop body of `run_swarm` (≈ lines 1100..3900 of `bin/evaluator.rs`, ≈ 2800 lines), including:
- N-agent Boltzmann routing
- Per-agent prompt build + LLM dispatch
- Response parse + error-class branch
- WorkTx real-signed submission
- VerifyTx real-signed submission
- AttemptTelemetry CAS write
- ProposalTelemetry CAS write
- Per-call budget enforcement (PerCallBudget)
- Wall-clock timeout
- TerminalSummary emission
- EvidenceCapsule emission on degraded outcomes

### §4.4 G1.1 resume primitive — both layers in place

Kernel layer (`src/runtime/mod.rs`):
- `RuntimeChaintapeConfig.resume_existing_chain: bool` field
- `from_env` reads `TURINGOS_CHAINTAPE_RESUME=="1"` strict equality
- `build_chaintape_sequencer_with_initial_q` resume branch using canonical `replay_full_transition` FC2 Boot primitive
- `bootstrap_resume_state` helper: reads existing `pinned_pubkeys.json`, generates NEW epoch keypair, appends to manifest

Binary layer (`experiments/minif2f_v4/src/chain_runtime.rs:236..298`):
- `AgentKeypairRegistry::resume_existing_durable(runtime_repo, keystore, password)` — reads existing `agent_pubkeys.json`, fail-closes on `ManifestAbsentInResume` or `ResumeKeystoreInconsistent`
- Cross-check pubkey-hex(manifest) == pubkey-hex(derive_from_keystore_secret)
- Strict env gate: `matches!(env::var("TURINGOS_CHAINTAPE_RESUME").as_deref(), Ok("1"))`

CI binding (`tests/constitution_g1_resume.rs`):
- SG-G1.1: resume on empty repo == legacy genesis (byte-equal)
- SG-G1.2: resume on N-entry chain → `Sequencer.next_logical_t == N`
- SG-G1.3: balances reconstruction matches forward replay
- SG-G1.4: `NonEmptyRuntimeRepo` only fires when resume=false (back-compat regression)
- SG-G1.5: `pinned_pubkeys.json` epoch preserved across resume
- SG-G1.6: `resume_existing_durable` fail-closed on manifest absent
- SG-G1.7: `resume_existing_durable` fail-closed on missing-secret
- SG-G1.8: `resume_existing_durable` fail-closed on pubkey mismatch

**Critical observation**: the entire kernel-side resume primitive is generic — works equally well for in-process resume (within one batch process across N problems requires NO resume; one SharedChain stays alive) and for cross-process resume (Option B's subprocess #2..N path).

---

## §5. Three options — detailed engineering specification

### §5.1 Option A — Single-process in-memory loop (§2.8 strict reading)

**Architecture diagram**:

```
batch_evaluator (1 process)
├── SharedChain::from_env()              ← fresh bootstrap (or resume on crash-restart)
├── write_synthetic_l4_l4e_gate(seed_id) ← ONCE
└── for each problem in problems:
    └── swarm_one_problem::run_one_problem(&mut chain, &spec).await
        ├── per-problem TaskOpen + EscrowLock (drive_task)
        ├── N-agent LLM swarm loop          ← NEW lift from evaluator.rs run_swarm body
        ├── Lean check
        ├── WorkTx / VerifyTx submission
        ├── AttemptTelemetry CAS write
        └── TerminalSummary emission
    chain.shutdown() ← ONCE at end
```

**File-tree delta**:

```
experiments/minif2f_v4/src/
├── lib.rs                       (+ pub mod swarm_one_problem;)
├── chain_runtime.rs             (unchanged)
├── drive_task.rs                (unchanged)
└── swarm_one_problem.rs         (NEW; ~2800-3000 lines lifted from bin/evaluator.rs:1100..3900)

experiments/minif2f_v4/src/bin/
├── evaluator.rs                 (5280 → ~1500 lines; run_swarm becomes thin wrapper
│                                 calling swarm_one_problem::run_one_problem;
│                                 main + arg parsing + oneshot path unchanged)
├── comprehensive_arena.rs       (unchanged — synthetic-tx Phase 4 driver still
│                                 lives alongside as the synthetic-coverage tool)
└── batch_evaluator.rs           (NEW; ~300 lines)
```

**Resume primitive role**: dormant in normal operation; activates only when a prior `batch_evaluator` process crashed mid-batch and the user restarts with `TURINGOS_CHAINTAPE_RESUME=1` against the existing runtime_repo. The G1.1 SG-G1.6..G1.8 fail-closed paths fire in this crash-recovery case only.

**Architect §2.8 compliance**: ✅ strict literal — one process, one runtime_repo, one CAS, one chain, multiple tasks (multiple problems in this case). Exact mirror of `comprehensive_arena.rs` Phase 4 pattern.

**Trust Root / STEP_B exposure**:
- Binary `bin/evaluator.rs` undergoes substantial refactor (5280→~1500 lines; function `run_swarm` body relocated to `swarm_one_problem.rs`)
- Binary is **NOT** on CLAUDE.md §12 STEP_B protected list (which enumerates `src/state/sequencer.rs` / `src/state/typed_tx.rs` / `src/bottom_white/cas/schema.rs` / etc. — kernel surfaces only)
- New file `src/swarm_one_problem.rs` is **NOT** on STEP_B protected list
- No Trust Root manifest (`genesis_payload.toml`) hash change

**Per-problem evidence dir shape** (G1.3 wrapper expectation):

```
<batch_run_dir>/
├── runtime_repo/                    ← single shared chain
├── cas/                             ← single shared CAS
├── BATCH_MANIFEST.json
├── PROBLEMS.txt
├── batch_summary.json
└── per_problem/
    ├── P01_mathd_algebra_107/
    │   ├── pput_result.json
    │   ├── evaluator.stdout (per-problem subset; actual stdout is unified)
    │   └── audit_telemetry.json
    └── P02_..., P03_..., ...
```

**Cross-problem PnL trajectory** (G-Phase §1.3 prerequisite):
- ✅ **Natural** — `SharedChain` stays alive; `EconomicState.balances_t` / `task_markets_t` / `cpmm_pools_t` / `conditional_share_balances_t` accumulate without interruption
- G3.1 `compute_agent_pnl` reads chain state directly; trajectory is a chain walk
- G5.1 opportunity scheduler can hold in-process state (last-chosen-action history, etc.) without serialization

**G1.1 mini-smoke evidence-shape compatibility**: ❌ DIVERGES — G1.1 smoke had P01/P02/P03 subdirs at top-level each owning a full evaluator.stdout/stderr; Option A produces unified stdout/stderr at batch level with per-problem subsections. SG-G1.6 ship-gate from G1.1 packet ("ONE runtime_repo across N") still satisfies; SG-G1.7 ("legacy evaluator binary unchanged") **needs rewording** because Option A reshapes `bin/evaluator.rs`.

**Implementation cost**:
- Lift `run_swarm` body (~2800 lines) from `bin/evaluator.rs` into `swarm_one_problem.rs::run_one_problem(chain: &mut SharedChain, spec: &TaskSpec, ...) -> PputResult`
- Need to thread `&mut SharedChain` through ~30 internal references currently using locally-scoped `bundle` / `agent_keypairs` / `bus` variables
- Rewrite `bin/evaluator.rs::run_swarm` as a 50-line thin wrapper: build SharedChain → call run_one_problem → shutdown
- Build `bin/batch_evaluator.rs` orchestrator (~300 lines: arg parsing + problems file load + per-problem loop + summary)
- Add CI tests for `run_one_problem` standalone semantics
- Risk: ~30 internal borrows in `run_swarm` body cross function boundaries (e.g., `mut last_tx_by_agent`, `chaintape_preseed_enabled`, `initial_balances_for_genesis_report`) — each needs threading review

**Effort estimate**: 2-3 sessions (Class-3 STEP_B-adjacent lift; PRE-impl packet draft + impl + dual audit + smoke + LATEST update).

**G-Phase downstream adaptation**:
- **G3.2** sequencer risk-cap admission — orthogonal (Class-4 sequencer change; works regardless of orchestrator)
- **G3.3** `=== Your Position ===` prompt block — Option A makes per-viewer PnL trivially derivable in-process from `chain.bundle.sequencer.q_snapshot()` between problems
- **G5.1** opportunity scheduler — Option A allows in-process scheduler state (Boltzmann weights / abstain history / role-bias accumulator) without external persistence layer
- **G7** structural smoke — Option A produces ONE evaluator.stdout with cross-problem markers; G7 walker reads chain (which has the same shape as Option B)

### §5.2 Option B — Subprocess orchestrator (G1.1 mini-smoke precedent)

**Architecture diagram**:

```
batch_evaluator (1 process; orchestrator only)
└── for (i, problem) in problems.iter().enumerate():
    └── Command::new("evaluator")
        .env("TURINGOS_CHAINTAPE_PATH", &shared_runtime_repo)
        .env("TURINGOS_CAS_PATH", &shared_cas)
        .env("TURINGOS_CHAINTAPE_RESUME", if i==0 {"0"} else {"1"})  ← KEY
        .env("TURINGOS_CHAINTAPE_PRESEED", if i==0 {"1"} else {"0"})
        .arg(&problem_path)
        .stdout(per_problem_stdout)
        .stderr(per_problem_stderr)
        .status()  ← subprocess #i runs full evaluator with shared chain
```

Each subprocess in Option B:
1. Reads shared `runtime_repo/` (i=0: empty/fresh; i>0: existing N entries)
2. `chain_runtime::SharedChain::from_env` bootstraps:
   - i=0: `build_chaintape_sequencer_with_initial_q` fresh path + generates initial pinned_pubkeys + new agent_pubkeys
   - i>0: `build_chaintape_sequencer_with_initial_q` resume branch (via G1.1) — loads existing pinned_pubkeys, generates new epoch keypair, reconstructs QState via `replay_full_transition`, sets `next_logical_t = chain_length`; `AgentKeypairRegistry::resume_existing_durable` loads existing agent_pubkeys + cross-checks keystore
3. Runs full `run_swarm` body unchanged
4. Calls `bundle.shutdown()` on exit
5. Subprocess #i+1 picks up from i's exit state via the same resume primitive

**File-tree delta**:

```
experiments/minif2f_v4/src/
└── (no changes to lib/ — Phase 1+2+3 already in place)

experiments/minif2f_v4/src/bin/
├── evaluator.rs               (unchanged — strict honor of "legacy evaluator unchanged")
├── comprehensive_arena.rs     (unchanged)
└── batch_evaluator.rs         (NEW; ~250 lines: arg parsing + problems file load
                                + Command::spawn loop + per-problem evidence dir
                                + aggregate summary)
```

**Resume primitive role**: **load-bearing on every problem #2..N** in normal operation. SG-G1.6 / SG-G1.7 / SG-G1.8 fail-closed paths in the resume primitive are exercised on every batch run (defense-in-depth). This is precisely the "Turing-machine fundamentalist" reading of `feedback_tape_first_real_tests` + FC2 §3.2 — the resume primitive IS the chain-to-chain continuation mechanism.

**Architect §2.8 compliance** — REQUIRES ARCHITECT RULING:
- Literal verbatim "如果它只是一个 process 里启动多个 subprocess, 每个 subprocess 自己起 chain, 那不合格" — taken word-for-word, Option B is **non-compliant** because batch_evaluator (1 process) launches N subprocesses
- Operative-clause reading: "每个 subprocess **自己起 chain**" is the forbidden specific — Option B's subprocesses do NOT start their own chains; subprocess #1 starts the only chain, #2..N attach to it via resume primitive (FC2 §3.2 canonical replay primitive `replay_full_transition`). On this reading, Option B is **compliant**
- Architect §8 SIGNED G1.1 packet whose §6 ship condition was the 3-subprocess mini-smoke pattern — this IS the precedent

**Trust Root / STEP_B exposure**:
- **Zero binary refactor**: `bin/evaluator.rs` unchanged
- Zero STEP_B file change
- Zero Trust Root manifest hash change
- Zero CI regression risk on existing `tests/` surface

**Per-problem evidence dir shape** (G1.3 wrapper expectation):

```
<batch_run_dir>/
├── runtime_repo/                    ← single shared chain (resume primitive)
├── cas/                             ← single shared CAS
├── BATCH_MANIFEST.json
├── PROBLEMS.txt
├── batch_summary.json
├── P01_mathd_algebra_107/           ← subprocess #1 evidence
│   ├── evaluator.stdout
│   ├── evaluator.stderr
│   ├── pput_result.json
│   └── chain_invariant.json (per-problem)
├── P02_mathd_algebra_125/           ← subprocess #2 evidence
│   ├── evaluator.stdout
│   ├── evaluator.stderr
│   └── ...
└── P03_..., ...
```

**Identical to G1.1 mini-smoke shape**: `handover/evidence/g_phase_g1_1_smoke_2026-05-11T13-01-12Z/`.

**Cross-problem PnL trajectory** (G-Phase §1.3 prerequisite):
- ✅ **Chain-resident** — `EconomicState.balances_t` / `task_markets_t` / `conditional_share_balances_t` survive in `runtime_repo/refs/transitions/main` across subprocess boundaries; subprocess #i+1 replays them via `replay_full_transition`
- G3.1 `compute_agent_pnl` works (reads chain state on subprocess startup)
- ⚠️ **G5.1 opportunity scheduler** — if scheduler needs cross-problem memory beyond what's in chain state (e.g., per-agent action history, exploration counts), needs chain-resident persistence (chain-resident is the constitutional posture anyway per `feedback_tape_first_real_tests`); requires explicit design as part of G5 atom
- G3.3 `=== Your Position ===` — subprocess reads chain state on prompt build (same primitive as in-process)

**G1.1 mini-smoke evidence-shape compatibility**: ✅ **identical** — Option B's batch_evaluator output is byte-for-byte the same shape as the G1.1 smoke evidence the architect just §8-signed.

**Implementation cost**:
- Build `bin/batch_evaluator.rs` orchestrator (~250 lines: arg parse, problems load, spawn loop, summary)
- Build CI integration test that runs batch_evaluator end-to-end on a 2-problem smoke fixture
- No `bin/evaluator.rs` change
- No `src/` library change
- Optional: small `swarm_one_problem.rs` extraction if user wants the symmetry; otherwise skipped (or shrunk to a thin re-export of the binary's existing `run_swarm` for library testability)

**Effort estimate**: 0.5-1 session (PRE-impl packet draft small; impl ~250 lines; dual audit small surface; smoke fast; LATEST update).

**G-Phase downstream adaptation**:
- **G3.2** sequencer risk-cap admission — orthogonal (works regardless of orchestrator)
- **G3.3** `=== Your Position ===` — prompt build reads chain state at subprocess boot; works the same as Option A
- **G5.1** opportunity scheduler — scheduler state MUST live chain-resident (CAS object or new typed_tx variant) — this is **constitutionally required anyway** per `feedback_tape_first_real_tests` ("no tape, no test"); Option B forces this discipline upfront vs. Option A where it could be deferred behind in-process state for the lifetime of one batch
- **G7** structural smoke — natural fit: G7 walker reads runtime_repo + CAS + per-problem evidence subdir; same shape as G1.1 smoke

### §5.3 Option C — Hybrid (in-process default + subprocess opt-in)

**Architecture diagram**:

```
batch_evaluator (1 process; orchestrator)
├── if env TURINGOS_BATCH_MODE=subprocess:
│   └── Option B's subprocess loop
└── else (default):
    └── Option A's in-process loop
```

**File-tree delta**: Option A's delta + Option B's delta — all changes in both options.

**Resume primitive role**: dormant (default in-process) OR load-bearing (subprocess opt-in) per env.

**Architect §2.8 compliance**: ✅ strict on default path; cf. §5.2 ruling needed on subprocess path.

**Trust Root / STEP_B exposure**: same as Option A on the in-process default path (binary refactor, ~2800-line lift).

**Implementation cost**: highest (Option A's lift cost + Option B's wrapper cost + dual-mode test matrix).

**Effort estimate**: 3-4 sessions.

**Recommendation against C**: at current G-Phase scope where the goal is empty-market 涌现 / persistent multi-agent collaboration, supporting two orchestration paths simultaneously is premature optimization. The architect should pick one path and let G2..G7 evolve against that pattern; if subprocess mode becomes needed later, it can land in a future TB once the cost/benefit is empirically clear.

---

## §6. Decision matrix

| Dimension | A — Single-process | B — Subprocess | C — Hybrid |
|-----------|---------|---------|---------|
| **Implementation LOC delta** | +3300 / -2800 | +250 | +3550 / -2800 |
| **`bin/evaluator.rs` change** | Major refactor (5280→1500) | None | Major refactor |
| **STEP_B exposure** | None (not on §12 list) | None | None |
| **Trust Root rehash** | None | None | None |
| **Architect §2.8 literal** | ✅ Compliant | ⚠️ Needs ruling (operative-clause interpretation) | ✅ Default path |
| **Architect §2.8 operative** | ✅ Compliant | ✅ Compliant (subprocess attach, not "起 own chain") | ✅ |
| **Resume primitive role** | Dormant; crash-recovery only | Load-bearing on every problem #2..N | Both |
| **`SG-G1.7 legacy evaluator unchanged`** | ❌ Conflicts; needs rewording | ✅ Strict honor | ❌ Conflicts on default path |
| **Match G1.1 mini-smoke evidence shape** | ❌ Diverges | ✅ Identical | Partial |
| **Match `comprehensive_arena.rs` Phase 4 pattern** | ✅ Mirror | ⚠️ Different family | ✅ Default path |
| **G3.1 PnL trajectory** | Natural (in-process state read) | Natural (chain state read) | Both |
| **G3.3 `=== Your Position ===`** | Trivial (q_snapshot between problems) | Trivial (subprocess reads chain on boot) | Both |
| **G5.1 opportunity scheduler** | In-process state OK | Forces chain-resident scheduler state (`feedback_tape_first_real_tests`-aligned) | Both |
| **G-Phase §1.3 "fresh genesis kill" prevention** | ✅ Genesis only at problem #0 | ✅ Genesis only at subprocess #0; #1..N use replay | ✅ |
| **PRE-§8 dual audit risk surface** | Larger (~3300 LOC) | Smaller (~250 LOC) | Largest |
| **PRE-§8 audit CHALLENGE risk** | Low (§2.8 strict; well-precedented by Phase 4) | Medium (§2.8 literal-vs-operative dispute; will need packet to cite G1.1 precedent) | Medium |
| **Total session estimate** | 2-3 | 0.5-1 | 3-4 |

---

## §7. G-Phase downstream implications by option

### §7.1 G2 / G2P — MarketDecisionTrace + Peer Verification Bridge
**Both options**: orthogonal — `MarketDecisionTrace` lives in CAS; prompt blocks read per-viewer chain state at prompt build. **No differentiator**.

### §7.2 G3 — Persistent PnL / Solvency / Bankruptcy
- **G3.1** `compute_agent_pnl` derived view: orthogonal (reads chain; works in both)
- **G3.2** sequencer risk-cap admission (Class 4): orthogonal — admission gate is in sequencer, fires regardless of orchestrator
- **G3.3** `=== Your Position ===` per-viewer prompt block: **Option A slightly cheaper** — can hold pre-computed `AgentMarketStateView` in process between problems. **Option B requires building the view at subprocess boot** from chain state (cheap; ~ms cost). Negligible difference.
- **G3.4** §G PnL trajectory report: orthogonal (audit_dashboard reads runtime_repo + CAS)

### §7.3 G4 — Multi-LLM Mix + No-Hidden-Model-Switch
- **G4.1** 3-model-family CSV: orthogonal (env-var passed to evaluator subprocess OR to in-process model dispatcher)
- **G4.2** `[agent_model_assignment]` genesis schema (Class 4): orthogonal — schema is in `genesis_payload.toml` + `bootstrap.rs`; reads same in both options
- **G4.3/G4.4** breakdown report + no-hidden-switch detector: orthogonal

### §7.4 G5 — Opportunity Scheduler + Role Classifier — **MATERIAL DIFFERENTIATOR**
- **G5.1** opportunity scheduler: needs to track agent action history, exploration counts, Boltzmann-weight smoothing across problems
  - **Option A**: scheduler state can live in `Arc<Mutex<SchedulerState>>` inside `batch_evaluator` process; simple to implement; **constitutionally questionable** because in-process state is memory-only (not chain-resident) — violates `feedback_tape_first_real_tests` "no tape, no test" if scheduler state affects future tx admission or proposal weighting
  - **Option B**: forces scheduler state to be chain-resident — either CAS object refreshed each subprocess boot OR new typed_tx variant (Class 4); aligns with `feedback_tape_first_real_tests` strictly
- **G5.2** role classifier: derived view; runs at batch-end in either option; **no differentiator**

### §7.5 G6 / G7 — Pricing feedback + Structural smoke
- **G6.1/G6.2/G6.3**: orthogonal — `market_context` lives in chain; predicate clean-grep enforced statically
- **G7.1** structural smoke 13 sub-gates: G7 walker reads runtime_repo + CAS + per-problem evidence; same shape in both options (Option B's per-problem evidence subdirs are explicit; Option A's are unified-stdout-with-markers but chain content is identical)
- **G7.2** §K mechanism-witness OR clean-negative: orthogonal

**Net G-Phase implication**: Option A makes G5.1 scheduler state-management *easier in the short term* but creates a constitutional fault line (in-process state + tape-first). Option B forces chain-resident scheduler state from day one — harder upfront but constitutionally cleaner long-term.

---

## §8. Audit risk profile by option (PRE-§8 + POST-impl Codex G2 + Gemini Pro dual audit)

### §8.1 Likely CHALLENGE surfaces

**Option A**:
- Q-Lift-Byte-Identity: did the ~2800-line lift preserve byte-identical semantics? (Codex G2 will diff old vs new run_swarm logic carefully; high risk of finding small drift)
- Q-Borrow-Reshape: are ~30 internal borrows correctly threaded through `&mut SharedChain`? (medium risk)
- Q-PerCallBudget-Plumbing: does `PerCallBudget` survive the lift correctly? (medium)
- Q-Trace-Continuity: does FC1 attempt-count equality still hold per-problem after lift? (low — chain semantics unchanged)
- Q-SG-G1.7-rewording: ship-gate text needs revision; audit will check the new wording is binding (low)

**Option B**:
- Q-§2.8-literal: Codex will cite §2.8 verbatim and ask whether subprocess pattern complies (medium — packet must cite G1.1 precedent + operative-clause reading; user/architect ruling §11 Q2 closes this definitively)
- Q-Resume-On-Every-Problem-Stress: are SG-G1.6..G1.8 fail-closed paths exercised correctly when fired N times per batch? (low — SG-G1.* tests cover; integration smoke confirms)
- Q-Subprocess-Crash-Recovery: if subprocess #k crashes mid-batch, does the orchestrator handle gracefully? (low — explicit error-handling in batch_evaluator)
- Q-Env-Var-Propagation: subprocess env passed correctly? (low — straightforward)

**Option C**: union of both option's CHALLENGE surfaces.

### §8.2 Constitutional compliance audit (matrix §R G1 row)

- **Option A**: G1.1 SG-G1.6 "ONE runtime_repo across N" satisfied trivially. SG-G1.7 needs rewording. Other G1.* SG gates unchanged.
- **Option B**: G1.1 SG-G1.6 / G1.7 / G1.8 all satisfied verbatim. SG-G1.6 ("ONE runtime_repo across N") is the load-bearing assertion that defines this whole atom; Option B exercises it directly.
- **Option C**: requires both A's and B's evidence; dual matrix-row witness.

---

## §9. Implementation effort estimate

| Option | PRE-impl packet | Implementation | Dual audit | Smoke + verification | LATEST update | **Total** |
|--------|----------------|----------------|------------|---------------------|---------------|----------|
| A | 1 session | 2 sessions | 0.5-1 session | 0.5 session | 0.5 session | **3-4 sessions** |
| B | 0.5 session | 0.5 session | 0.5 session | 0.5 session | 0.5 session | **1-1.5 sessions** |
| C | 1 session | 3 sessions | 1 session | 1 session | 0.5 session | **5-6 sessions** |

(Session ≈ one focused implementation pass with full validation cycle; assumes no major CHALLENGE rework.)

---

## §10. Recommendation (advisory; non-binding)

**Option B** is recommended on the following grounds, each individually weak but collectively load-bearing:

1. **G1.1 architectural precedent already established**. The architect §8-signed packet whose §6 condition (b) was the 3-subprocess mini-smoke. That ship event redefined §2.8's operative clause from "no subprocess" to "no subprocess starting its own chain". Option B exercises the same pattern at scale; Option A repudiates the precedent.

2. **Resume primitive load-bearing on the canonical path**. User 2026-05-11 verbatim: "断点续作是本项目的核心 ... 从图灵机原教旨主义角度去解决这个tape问题". Option B makes the resume primitive fire on every problem #2..N (defense-in-depth); Option A relegates it to crash-recovery (rarely fires; quietly bit-rots).

3. **Constitutional alignment forced upfront on G5**. Option B forces G5.1 opportunity scheduler state to be chain-resident (per `feedback_tape_first_real_tests`); Option A allows in-process state which would violate that posture in stealth.

4. **Lower audit risk surface**. ~250 LOC vs ~3300 LOC; Codex G2 + Gemini Pro dual audit complete in one round for Option B; Option A is at least 2 rounds (lift-byte-identity will surface drift).

5. **Faster shipping unblocks G2..G7 sooner**. G-Phase has 6 forward atoms behind G1.2; 1-1.5 sessions vs 3-4 sessions is a 2-3x difference at the phase budget level.

The strongest counter-argument is **§7.4 G5 scheduler state**: Option A makes G5.1 cheap in the short term. But the constitutional cost (in-process state that influences future admission) is a `feedback_tape_first_real_tests` violation that would surface in G5 audit anyway — Option B forces the right design now.

If the architect reads §2.8 literally and rules Option B non-compliant, **Option A** is the next-best fallback. **Option C is not recommended** at this phase.

---

## §11. Architect decision questions (please answer these)

> **Q1 (load-bearing)** — Which orchestration model do you authorize for G1.2?
> - [ ] Option A — single-process in-memory loop (§2.8 strict reading)
> - [ ] Option B — subprocess orchestrator with resume primitive (G1.1 precedent)
> - [ ] Option C — hybrid
> - [ ] Other (please specify)

> **Q2 (operative-clause reading; relevant if Q1 = B or C)** — Do you reaffirm that "subprocess attaching to shared runtime_repo via `TURINGOS_CHAINTAPE_RESUME=1`" is **NOT** the §2.8-forbidden pattern (i.e., the G1.1 mini-smoke precedent stands as binding precedent)?
> - [ ] Yes — G1.1 precedent stands; "每个 subprocess 自己起 chain" forbids own-chain creation, not shared-chain attach
> - [ ] No — §2.8 forbids subprocess-per-task regardless of chain sharing mechanism (this would invalidate G1.1's mini-smoke ship evidence retroactively; please flag if this is your reading)

> **Q3 (ship-gate rewording; relevant if Q1 = A or C)** — Do you authorize revising G1.1 packet §6 ship-gate text from "legacy evaluator binary unchanged" to "evaluator binary thin-wrapper preserves single-task semantics byte-identical"?
> - [ ] Yes — `bin/evaluator.rs` may be refactored as long as single-task semantics are byte-identical
> - [ ] No — `bin/evaluator.rs` must remain unchanged (forces Option B or de-scope Option A)

> **Q4 (Class boundary)** — Does the ~3000-line lift in Option A/C constitute Class-4 STEP_B (because of magnitude), or remain Class-3 (because no `src/state/sequencer.rs` / `src/state/typed_tx.rs` / `genesis_payload.toml` Trust Root surface is touched)?
> - [ ] Class-3 — magnitude alone doesn't trigger Class-4; only §12 STEP_B file list does
> - [ ] Class-4 — magnitude triggers Class-4 STEP_B; requires its own §8 packet (NOT covered by G1.1 packet §6)
> - [ ] N/A — Q1 = B

> **Q5 (audit cadence)** — Confirm PRE-impl packet draft (Codex G2 + Gemini Pro design review) + POST-impl dual audit (round-cap 2) is correct for G1.2?
> - [ ] Yes — proceed with PRE-impl packet draft once Q1..Q4 answered
> - [ ] POST-impl only — skip PRE-impl design review (faster but loses early CHALLENGE catching)
> - [ ] Other (please specify)

> **Q6 (forward §8 implications)** — If Q1 = A or C and Q4 = Class-4, do you authorize drafting a new §8 packet for G1.2 (parallel structure to G1.1 §8 packet), with new architect §8 sign-off step?
> - [ ] Yes — proceed with G1.2 §8 packet drafting
> - [ ] No — Q1 = B (no Class-4 trigger; G1.1 §8 covers)
> - [ ] No — proceed Class-3 even at ~3000-line magnitude

---

## §12. Forward implications by decision outcome

### If Q1 = B (subprocess; recommended):

1. Cut branch `feat/g1-2-batch-evaluator` (Class 3)
2. Build `bin/batch_evaluator.rs` (~250 lines)
3. Build `scripts/run_g_phase_batch.sh` (G1.3; ~200 lines bash; sibling of `run_stage_b3.sh`)
4. Build `tests/constitution_g1_persistence_evidence_binding.rs` (G1.4; SG-G1.11..G1.15)
5. PRE-§8 dual audit (Codex G2 + Gemini Pro; round-cap 2) covering Q1..Q9
6. Real-LLM mini smoke (3-problem batch with persistence witness)
7. LATEST.md session #41 update; matrix §R G1 row 🟡 AMBER → 🟢 GREEN if all 15 SG-G1.* gates GREEN
8. G2 / G2P parallel priorities unlocked (Class 2 — autonomous)
9. G3.2 / G4.2 §8 packets drafted (separate Class-4 ratification per `feedback_no_batch_class4_signoff`)

### If Q1 = A (single-process):

1. **If Q4 = Class-4**: HALT — draft new G1.2 §8 packet; submit to architect for §8 sign-off; only proceed after sign-off
2. **If Q4 = Class-3**: proceed
3. Cut branch `feat/g1-2-batch-evaluator-inprocess`
4. Lift `run_swarm` body to `src/swarm_one_problem.rs::run_one_problem` (~2800 lines)
5. Refactor `bin/evaluator.rs::run_swarm` as thin wrapper (50 lines)
6. Build `bin/batch_evaluator.rs` (~300 lines)
7. Update G1.1 packet §6 SG-G1.7 wording per Q3
8. Same G1.3 / G1.4 / dual audit / smoke / LATEST update as Option B (but larger surface)

### If Q1 = C (hybrid):

Union of Option A + Option B forward paths; not recommended.

---

## §13. Status

- 2026-05-11 — Packet drafted by Claude (Session #41 autonomous work; G1.2 implementation HALTED pending architect ruling on §11 Q1..Q6).
- Forward HALT condition cleared by architect's written response to §11.
- Once cleared, implementation proceeds per §12 forward path matching Q1.
- Packet itself is Class-0 documentation; no code change; no Trust Root impact; no constitution gate impact.

---

## §14. References

- **Architect ruling 2026-05-05 (TB-18 §2.8)** — verbatim copied to `experiments/minif2f_v4/src/bin/comprehensive_arena.rs:7..11` + `experiments/minif2f_v4/src/chain_runtime.rs:7..9` + `experiments/minif2f_v4/src/drive_task.rs:11..14`
- **G-Phase architect directive 2026-05-11** — `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` §1.3 + §G1 halt conditions
- **G1.1 §8 packet (SIGNED)** — `handover/directives/2026-05-11_TB_G_G1_1_§8_PACKET.md` §6
- **G1.1 mini-smoke evidence** — `handover/evidence/g_phase_g1_1_smoke_2026-05-11T13-01-12Z/` (3 subprocess invocations; aggregate audit_tape PROCEED)
- **TB-G charter** — `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §1 Module G1
- **Session #40 close** — `handover/ai-direct/LATEST.md` session #40 entry
- **CLAUDE.md** — §2.1 Constitutional Harness Engineering / §9 Class boundary / §10 authorization semantics / §12 STEP_B protected list / §13 economy laws / §18 benchmark rules
- **Feedback rules** — `feedback_no_batch_class4_signoff`, `feedback_tape_first_real_tests`, `feedback_class4_cannot_hide_in_class3`, `feedback_dual_audit`, `feedback_no_workarounds_strict_constitution`, `feedback_audit_after_evidence`
