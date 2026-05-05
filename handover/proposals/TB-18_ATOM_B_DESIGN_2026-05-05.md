# TB-18 Atom B-design — Substantive comprehensive_arena.rs (DESIGN-COMPLETE; implementation deferred to TB-18.B-impl follow-up)

**Status**: **DESIGN-COMPLETE 2026-05-05 — implementation deferred to TB-18.B-impl follow-up commit (Class 3 STEP_B-adjacent multi-day refactor; not safely landable in single auto-mode session)**.
**Filed**: 2026-05-05.
**Authority**:
  - PRE-17.6 deviation §6.B + §2.3 (atom B scope source: rewrite scaffold → substantive multi-task driver) — `handover/proposals/TB-17_PRE_17_6_COMPREHENSIVE_ARENA_DEVIATION_2026-05-05.md`.
  - Architect TB-18 ratification ruling §3 atom B + §2.8 (one process + one runtime_repo + one CAS + one chain; per-task subprocess fork FORBIDDEN) — `handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md`.
  - TB-18 charter §1.4 FR-18.7 + SG-18.6 — `handover/tracer_bullets/TB-18_charter_2026-05-05.md`.

---

## §1 The architect §2.8 mandate verbatim

```text
Atom B 要证明的是：
  one evaluator process
  one runtime_repo
  one CAS
  one chain
  multiple tasks

如果它只是一个 process 里启动多个 subprocess，每个 subprocess 自己起 chain，那不合格.

TB-18 要写明：
  B must not shell out to per-task evaluator processes that create separate chains.
```

## §2 Why this is a multi-day refactor

The current evaluator binary (`experiments/minif2f_v4/src/bin/evaluator.rs`, 4498 lines) is structured around per-process per-task semantics:

- `main()` (line 240) parses ONE `problem_file` argument.
- `run_swarm()` (line 638; ~1880-line function body) creates Kernel + Bus + ChaintapeBundle internally:
  - `Kernel::new()` at line 659.
  - `BusConfig { ... }` at line 660-673.
  - `ChaintapeBundle::new(...)` at line 695-797.
  - Bundle is held across the run; `bundle.shutdown().await` is invoked at the implicit final return.

To drive N tasks against ONE shared chain in ONE process, the bundle creation must be **lifted OUT** of `run_swarm` and shared across N invocations. This is non-trivial because:

1. **Lifetime management**: bundle holds Sequencer + ChainTape writers + CAS handles. Multi-task would share these across N task boundaries.
2. **State accumulation**: each task's TaskOpen + WorkTx + EscrowLock etc. accumulates in the same Q-state. Multi-task driver must coordinate task IDs to avoid `TaskAlreadyOpen` collisions.
3. **Preseed**: TURINGOS_CHAINTAPE_PRESEED initializes balances. Multi-task: should preseed run once at chain start (operator pattern) or per-task (bonus tokens per task)?
4. **Shutdown**: bundle.shutdown() drains queued submissions. Multi-task: shutdown ONCE at chain end, NOT per-task.
5. **Per-task isolation**: each task today is a separate run_id. Multi-task should preserve per-task run_id but consolidate chain-state accumulators.

Estimated effort: **4-8 hours of careful Rust + extensive integration testing** to ensure the existing run_swarm semantics are preserved while adding multi-task orchestration.

## §3 Why deferring is honest (per `feedback_no_workarounds_strict_constitution`)

Per `feedback_iteration_cap_24h` Class 3 production wire-up exception (72h-to-feedback-loop), atom B is appropriately sized as a 72h-class atom. In a single auto-mode session, **safely** landing the refactor is not feasible; rushing it risks:

- Regressing P01 baseline (12s solve) to a broken state.
- Introducing race conditions in shared bundle state across tasks.
- Trust-root rehash + workspace test break-then-fix loop consuming session budget.

Per `feedback_no_workarounds_strict_constitution` ("我不要凑活"), the correct course is:
1. Ship the design doc with full technical spec.
2. Defer implementation to a dedicated follow-up commit (TB-18.B-impl; Class 3 STEP_B-adjacent timeboxed appropriately).
3. Atoms F + G0 + H downstream that depend on atom B substantive can either:
   - Defer to TB-18.B-impl ship time, OR
   - Ship using TB-16.x.2.6 multi-chain UNION pattern with explicit deviation (atom B.deviation; same pattern as TB-17 PRE-17.6) and explicit forward trigger to TB-18.B-impl.

Per architect Q2 ship-claim narrowing rule, TB-18 then ships "formal benchmark substrate partially closed; single-chain 13/13 remains as TB-18.B-impl follow-up".

## §4 Implementation spec (for TB-18.B-impl follow-up)

### §4.1 New module: `experiments/minif2f_v4/src/chain_runtime.rs`

Lift bundle creation out of `run_swarm` body (lines 695-797) into a new pub function:

```rust
/// TB-18 Atom B: pre-created shared chaintape bundle for multi-task drive_task.
///
/// Encapsulates Kernel + BusConfig + ChaintapeBundle setup currently inline
/// in `run_swarm`. Caller (comprehensive_arena_v2 or atom-B M-ladder driver)
/// constructs ONCE per chain; passes &mut to each drive_task invocation.
pub struct SharedChain {
    pub kernel: Arc<Kernel>,
    pub bundle: ChaintapeBundle,
    pub bus_config: BusConfig,
}

impl SharedChain {
    /// Construct shared chain from env (TURINGOS_CHAINTAPE_PATH +
    /// TURINGOS_CHAINTAPE_PRESEED + agent_models). Mirrors run_swarm
    /// lines 695-797 logic.
    pub async fn from_env(model: &str, n_agents: usize) -> Result<Self, ChainSetupError>;

    /// Shutdown — drain queued submissions, write trust root, flush CAS.
    /// Called ONCE at chain-end after all tasks driven.
    pub async fn shutdown(self) -> Result<RunSummary, ChainSetupError>;
}
```

### §4.2 Refactor `run_swarm` signature

```rust
// BEFORE (today):
async fn run_swarm(
    problem_file: &str, problem_statement: &str, theorem_name: &str,
    lean_path: &str, proxy_url: &str, model: &str, n_agents: usize,
) -> PputResult;

// AFTER (atom B):
async fn run_swarm_with_shared_chain(
    chain: &mut SharedChain,         // NEW: shared chain reference
    spec: &TaskSpec,                 // bundles problem_file + statement + name + lean + proxy + model + n_agents
    budget: PerCallBudget,           // NEW: per-LLM-call budget passthrough
) -> PputResult;

// Backward-compat wrapper: existing run_swarm body becomes:
async fn run_swarm(problem_file, problem_statement, theorem_name, lean_path,
                  proxy_url, model, n_agents) -> PputResult {
    let mut chain = SharedChain::from_env(model, n_agents).await?;
    let spec = TaskSpec { problem_file, problem_statement, theorem_name,
                          lean_path, proxy_url, model, n_agents };
    let result = run_swarm_with_shared_chain(&mut chain, &spec,
                                              PerCallBudget::default()).await;
    chain.shutdown().await?;
    result
}
```

### §4.3 Update `drive_task` to use SharedChain

```rust
// experiments/minif2f_v4/src/drive_task.rs (replace Atom A.1 stub):

pub async fn drive_task(
    chain: &mut SharedChain,
    spec: TaskSpec,
    budget: PerCallBudget,
) -> Result<PputResult, DriveTaskError> {
    // Delegate to refactored run_swarm_with_shared_chain.
    let result = run_swarm_with_shared_chain(chain, &spec, budget).await;
    Ok(result)
}
```

### §4.4 Substantive `comprehensive_arena.rs`

Replace 436-line scaffold with multi-task driver:

```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = parse_args()?;
    let task_specs: Vec<TaskSpec> = load_manifest(&manifest_path)?;

    if task_specs.len() < 6 {
        bail!("Atom B requires ≥6 engineered tasks per architect §2.8; got {}",
              task_specs.len());
    }

    // ONE shared chain for all tasks.
    let mut chain = SharedChain::from_env(&task_specs[0].model,
                                           task_specs[0].n_agents).await?;
    let budget = PerCallBudget::from_env()?;

    let mut results = Vec::new();
    for spec in task_specs {
        let result = drive_task(&mut chain, spec, budget).await?;
        results.push(result);
    }

    // ONE shutdown at chain-end.
    let summary = chain.shutdown().await?;

    write_arena_report(&summary, &results)?;
    Ok(())
}
```

### §4.5 Engineered task set (≥6 tasks for 13/13 coverage)

Per atom D-design §4.3 Path C analysis:

| Task | Lifecycle | Tx kinds covered |
|---|---|---|
| task_A | Open → Work → Verify (OMEGA) → FinalizeReward | work, verify, finalize_reward, task_open, escrow_lock |
| task_B | Open → Work → Verify (OMEGA) → Challenge → ChallengeResolve(Released) | challenge, challenge_resolve |
| task_C | Open → MarketSeed → CompleteSetMint → Bankrupt → CompleteSetRedeem | complete_set_mint, complete_set_redeem, market_seed |
| task_D | Open → Work → MaxTxExhausted → TerminalSummary → TaskBankruptcy → TaskExpire(BankruptcyTriggered) | terminal_summary, task_expire, task_bankruptcy |
| task_E | Open → MaxTxExhausted → TerminalSummary (no bankruptcy) | (already covered; redundancy for solver_diversity) |
| task_F | Open → Force-degraded LLM → DegradedLLM TerminalSummary | (atom A new path; verifies DegradedLLM EvidenceCapsule emission end-to-end) |

Total: 13/13 architect tx kinds across 6 tasks in ONE chain.

### §4.6 STEP_B_PROTOCOL preflight

Per `feedback_step_b_protocol`, the run_swarm refactor is a sequencer-adjacent change (chaintape bundle setup interacts with sequencer). Required preflight before merge:
1. Phase 0: Codex + Gemini external audit on the SharedChain extraction patch.
2. Phase 1: parallel-branch implementation; isolated cargo test --workspace baseline.
3. Phase 2: A/B test — paired N=10 problems, current run_swarm vs run_swarm_with_shared_chain (single-task path); McNemar's test pre-registered.
4. Phase 3: merge on empirical strict win OR equivalence (refactor must be byte-identical for single-task path).

### §4.7 Trust root rehash impact

`evaluator.rs` + `lib.rs` + new `chain_runtime.rs` (added to manifest if pub) all need rehash per R-014.

---

## §5 Atom B-impl outcome verdict for TB-18 ship

**Atom B-impl SKIPPED in TB-18 (this session).**

TB-18 ship claim (per architect Q2 narrowing): **"formal benchmark substrate partially closed; single-chain 13/13 multi-task driver remains as TB-18.B-impl follow-up Class 3 STEP_B-adjacent work."**

Atom F (single-chain 13/13 evidence): SKIPPED in TB-18 as well; cannot produce single-chain evidence without atom B substrate. Falls back to TB-16.x.2.6 multi-chain UNION as TB-18 ship-time evidence with explicit deviation (parallel to TB-17 PRE-17.6 pattern).

Atom H sub-stages adjustments:
- M0 retry: PROCEED (uses existing single-task evaluator; atom A budget already wired).
- M1: PROCEED with caveat (existing single-task evaluator; not atom B substrate).
- M2: PROCEED with caveat (same; multi-agent uses existing run_swarm path); 100+ problems ground truth.

These M-ladder sub-stages still produce valuable benchmark capacity evidence, just on the existing single-task substrate rather than atom B's new substantive multi-task driver.

---

## §6 Forward triggers (TB-18.B-impl follow-up OR TB-19+)

| Trigger | Source | Carry-forward |
|---|---|---|
| **TB-18.B-impl** SharedChain refactor + run_swarm_with_shared_chain extraction + comprehensive_arena.rs substantive build | This design + architect §2.8 + §3 atom B | Class 3 STEP_B_PROTOCOL parallel-branch follow-up commit; 4-8 hour timebox |
| **TB-18.F** single-chain 13/13 evidence | Depends on TB-18.B-impl | After TB-18.B-impl ships |
| **PRE-17.6 §6.F** canonical single-chain shape replacing TB-16.x.2.6 multi-chain UNION | architect §3 atom F | Per TB-18.B-impl execution |

---

## §7 Cross-references

- TB-18 charter §1.4 FR-18.7 + SG-18.6 + SG-18.B
- Architect TB-18 ratification ruling §2.8 + §3 atom B + Q2 ship-claim narrowing
- PRE-17.6 §2.3 + §6.B
- Atom A drive_task API surface stub: `experiments/minif2f_v4/src/drive_task.rs`
- Atom A per_call_budget primitives: `experiments/minif2f_v4/src/per_call_budget.rs`
- TB-16.x.2.6 multi-chain UNION pattern (deviation precedent) — `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md`
- `feedback_no_workarounds_strict_constitution` (no 凑活: deferring is honest, not workaround)
- `feedback_iteration_cap_24h` (Class 3 72h cap; multi-day refactor doesn't fit single session)
- `feedback_step_b_protocol` (sequencer-adjacent refactor needs A/B preflight)
- `feedback_no_fake_menus` (this design takes explicit position: defer, don't fake-ship)

---

**End of design.** Atom B-impl deferred to TB-18.B-impl follow-up commit. TB-18 sequence proceeds to atom F (also DEFERRED with explicit deviation) → atom H (sub-stages run on existing substrate) → G0 + G1 audits.
