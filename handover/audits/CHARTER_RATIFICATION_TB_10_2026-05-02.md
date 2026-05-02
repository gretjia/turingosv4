# Charter Ratification — TB-10 Lean Proof Task Market MVP

**Date**: 2026-05-02
**Charter**: `handover/tracer_bullets/TB-10_charter_2026-05-02.md`
**Authority**: user authorization 2026-05-02 ("authorized in auto mode until TB-10 is done with real LLM smoke test … and dual audit") + architect directive 2026-05-02 Part C ruling 12 + 13 line 1594.
**Mode**: auto-ratification under user-granted auto-mode (per-question proposed defaults from charter §7 are accepted as-is; deviations would require explicit user override).
**Predecessor ratifications**: TB-8 (`CHARTER_RATIFICATION_TB_8_2026-05-02.md`), TB-9 (`CHARTER_RATIFICATION_TB_9_2026-05-02.md`).

---

## §0 Scope ratification

**Architect spec verbatim** (line 1594):

```text
TB-10：Lean Proof Task Market MVP
目标：第一个可用产品：用户发任务，Agent 解题，系统验证，系统付款，dashboard 可审计。
必须：
TaskOpenTx
EscrowLockTx
WorkTx
VerifyTx
FinalizeRewardTx
replay
dashboard
```

**Scope interpretation**:

```text
Architect MUST list = 7 primitives, all already shipped (TB-3..TB-8).
TB-10 net-new   = thin user-facing wrapper that drives those 7 primitives end-to-end
                  from a non-evaluator entity (a human running a CLI).
TB-10 NOT scope = new typed_tx, new dispatch arm, new economic mutator,
                  NodeMarket / position / CompleteSet / MarketSeed / AMM / public-chain.
```

**Ratified**.

---

## §1 Q1-Q8 — auto-ratified per user authorization

### Q1 — User identity model

**Question**: How does the user (sponsor) get an AgentId?

**Default**: New `Agent_user_<n>` registers lazily into TB-9 durable keystore on first CLI use. Same keystore file as solver agents. `Agent_user_` prefix distinguishes sponsor role from solver role at convention level (no kernel-level enforcement; the prefix is informational for dashboards and audit clarity).

**Rationale**: TB-9 keystore (`~/.turingos/keystore/agent_keystore.enc`) is per-host; one keystore per host is the architect mandate. Adding sponsor agents to the same keystore preserves "one canonical record per host" without introducing a parallel keystore file. The `Agent_user_` prefix is observable in audit_dashboard §11 filtering (charter Atom 4) — solver vs sponsor visualization comes from naming convention, not from a kernel-level role enum.

**Risk**: any cryptographic confusion between sponsor signing context and solver signing context? **Answer**: no — the `WorkSigningPayload` and `TaskOpenSigningPayload` / `EscrowLockSigningPayload` are domain-separated. An Agent_user_0 keypair cannot be tricked into producing a signature accepted in the wrong domain because canonical_digest is payload-shape-specific.

**RATIFIED.**

---

### Q2 — User starting balance

**Question**: How does Agent_user_0 acquire Coin to lock as bounty?

**Default**: Runtime preseed factory `default_pput_preseed_pairs()` (new module `src/runtime/bootstrap.rs`) adds `Agent_user_0 = 10_000_000` micro (~10 Coin) alongside existing solver agents. This is the **same on_init mechanism the evaluator already uses** (`evaluator.rs:716-731` literal extracted into a reusable factory). NOT post-init mint — `assert_no_post_init_mint` only fires for typed_tx applied AFTER QState::genesis bootstrap. Genesis QState construction is the substrate.

**No genesis_payload.toml edit needed** (correction from charter draft §7 Q2): the preseed pattern is purely runtime (`genesis_with_balances` adapter), not toml-resident. The trust_root manifest is unaffected.

**Rationale**: minimum-surgery on existing codebase. The runtime preseed pattern is established and tested (TB-7.7 D3 ship); TB-10 just adds 1 entry to the list.

**Risk**: does adding `Agent_user_0` change replay determinism for past chains? **Answer**: no — replay reads QState from on-disk state_roots, not from the preseed factory. Past chains have their own genesis QState with their preseed list at the time of run; the factory only matters for FRESH chaintape bootstraps.

**RATIFIED.**

---

### Q3 — Solver task discovery

**Question**: How does evaluator know to work a user-posted task?

**Default**: `--task-mode <user|self|both>` (or env `TURINGOS_EVAL_TASK_MODE`). Default = `both`.

```text
user mode  : evaluator boot reads task_markets_t; for each open task with total_escrow > 0
             whose task_id matches the convention task:lean:heldout_49:<problem_id>,
             evaluator extracts problem_id and works it as a USER task. Skips its own
             self-funded TaskOpen+EscrowLock branch (evaluator.rs:864-922).
self mode  : existing TB-7.7 self-funded behavior preserved.
both mode  : tries user first; falls back to self if no open user task found. Default for
             back-compat with current smoke harness.
```

**Rationale**: minimum-impact extension. `both` default preserves all existing TB-9 smoke behavior — running evaluator without TB-10 user CLI works identically to TB-9. `user` mode is opt-in for user-driven flow.

**Risk**: if evaluator finds a user task but the task_id doesn't match any heldout-49 problem, what happens? **Answer**: skip that task (treat as not-applicable to this evaluator instance) and fall back to self-funded mode. Document as known limitation: heldout-49 namespace only.

**RATIFIED.**

---

### Q4 — Task ID naming

**Question**: How does user identify which Lean problem to bounty?

**Default**: `lean_market post-task --problem <heldout_49_id>` accepts only heldout-49 problem ids. The `task_id` is constructed as `task:lean:heldout_49:<problem_id>` (e.g. `task:lean:heldout_49:mathd_algebra_171`). This namespace prefix is forward-compatible with future arbitrary-Lean ingest in TB-13 Beta.

**Rationale**: heldout-49 is the existing test corpus that the evaluator + Lean oracle already work on. Accepting arbitrary Lean source requires a separate predicate path (oracle must validate the theorem statement is well-formed before letting solvers attempt it), which inflates scope. Defer to TB-13 Beta where arbitrary ingest is the headline feature.

**Risk**: namespace collision with evaluator's existing self-funded `task-{run_id}` naming? **Answer**: no — `task:lean:heldout_49:` prefix is distinct from `task-` prefix. Evaluator user-mode discovery filters by prefix.

**RATIFIED.**

---

### Q5 — Bounty unit

**Question**: User-facing bounty input format?

**Default**: `--bounty <micro>` integer MicroCoin. Min bounty = 100_000 micro (matches TB-8 evaluator self-fund baseline; the `escrow_micro` env default). Max bounty per CLI invocation: capped at sponsor's current balance (kernel will reject InsufficientBalance otherwise).

CLI display renders human-readable Coin float (`bounty 100000 micro = 0.1 Coin`) for view-task / view-wallet output, but inputs stay integer to avoid float-drift bugs.

**Rationale**: economic state lives in MicroCoin everywhere on-chain (i64 micro_units). User-facing CLI maintains integer parity with the chain.

**RATIFIED.**

---

### Q6 — Settlement view (passive vs user-triggerable)

**Question**: Does the user CLI have a way to trigger settlement?

**Default**: **PASSIVE ONLY**. `view-task` reads `claims_t[work_tx_id].status` and reports Open/Verified/Finalized/Failed. There is **NO** `lean_market settle`, `lean_market finalize`, `lean_market refund`, or any other settlement-triggering subcommand.

FinalizeReward stays **system-emitted** per TB-8 invariant. The evaluator's `tb8_emit_finalize_after_verify` helper in the OMEGA-Confirm branch is the SOLE path to FinalizeReward emission.

**Rationale**: Anti-Oreo (Art. III.4) — agents (including human users) cannot drive system_tx emission. Adding `lean_market settle` would expose `emit_system_tx(SystemEmitCommand::FinalizeReward)` to user, violating "agent ≠ direct state writer" + "no agent-submitted system tx".

**Constitutional gate**: this is non-negotiable.

**RATIFIED.**

---

### Q7 — Failed-task fate

**Question**: If no solver attempts the task or all attempts fail, what happens to the user's escrow?

**Default**: **INDEFINITE LOCK** in TB-10. Refund mechanism deferred to TB-12 (RSP-3.2 slash) + TB-13 (TaskExpireTx). The bounty stays locked in escrows_t indefinitely until a solver succeeds.

User CLI's `view-task` flags task as `STALE` (informational label) after 24h wall-clock with no Verify entry on chain. This is purely a CLI display state — no kernel state change.

Limitation documented at `handover/alignment/OBS_TB_10_NO_USER_REFUND_2026-05-02.md`.

**Rationale**:
- TB-10's MUST list (architect line 1594) does NOT include refund / expiry — adding either would scope-creep.
- Refund implies either `EscrowReleaseTx` (new typed_tx — forbidden) or `TaskExpireTx` system-emitted (TB-12 territory).
- Slash is RSP-3.2 (TB-12).
- For MVP first-product, indefinite lock is acceptable: real-money bounties on Lean problems with tight oracle gating ARE solvable; TB-9 evidence shows 3/3 SOLVE rate. The risk of stuck escrow is small and survivable until TB-12.

**RATIFIED.**

---

### Q8 — Audit depth

**Question**: TB-10 risk class + audit mode?

**Default**: **Class 2 primary** (production wire-up — new bin exposes existing typed_tx surface to a human user; NO new kernel surface) + **Class 3 audit** at Atom 6 (first new caller class for already-Class-3 economic mutators).

Audit mode = full triple coverage:

```text
1. Recursive self-audit       — 4-clause structure (Constitutional / Replay-deterministic /
                                 Conservation / User-minimum-contract); 11 ship gates;
                                 architect mandate checklist.
2. Codex impl-paranoid        — round-1 verdict; remediation if VETO/CHALLENGE; round-2 re-verdict.
3. Gemini architectural        — strategic tier `gemini-3.1-pro-preview`; degraded-label if exhausted
                                 (per feedback_dual_audit hybrid-by-risk-class).
```

Conservative verdict wins (VETO > CHALLENGE > PASS). If Codex round-1 VETO and Gemini round-1 PASS, perform round-2 remediation and resubmit to Codex.

**Rationale**: TB-9 deferred external audit because surface was kernel-only-additive (no new caller class). TB-10 surface adds **a new caller class** (human user via CLI), even though typed_tx surface itself is unchanged. Human-driven economic mutator invocation is the right place to deploy paranoid review.

**RATIFIED.**

---

## §2 Architectural clarifications

### §2.1 — Adapter constructor for user-signed TaskOpen+EscrowLock

The existing `make_synthetic_task_open` and `make_synthetic_escrow_lock` use `AgentSignature::from_bytes([0u8; 64])` (zero signatures). Empirical finding (charter §2): the kernel currently does NOT verify TaskOpen/EscrowLock Ed25519 signatures (the dispatch arms have no `verify_agent_signature` call).

**TB-10 decision**: user CLI signs TaskOpen+EscrowLock with **real Ed25519** signatures via `AgentKeypairRegistry::sign(&Agent_user_0, digest)`. Add new constructors `make_real_task_open_signed_by` and `make_real_escrow_lock_signed_by` in `src/runtime/adapter.rs` mirroring the existing `make_real_worktx_signed_by` pattern.

Kernel acceptance does NOT depend on signature validity today, but populating real signatures:
- Forward-compatible with kernel signature verification (planned for TB-12+ per WP roadmap)
- Demonstrates user identity binding at the chain boundary (architect TB-9 mandate spirit)
- Costs nothing (negligible CPU; same keystore primitive as TB-9)

The existing zero-signature evaluator path (preseed branch at `evaluator.rs:864-922`) is **not modified** in TB-10; only the new user CLI uses real signatures. This avoids touching evaluator self-funded mode.

### §2.2 — Concurrent chaintape access

**Concern**: lean_market CLI and evaluator may both have the chaintape open simultaneously (e.g., user invokes `lean_market view-task` while evaluator is mid-run).

**Mitigation**:
- Sequencer is per-process (`Arc<Sequencer>` constructed inside each process); two processes get two distinct in-memory sequencer instances.
- Both processes attach to the same on-disk chaintape via `build_chaintape_sequencer` with `nonempty_repo_ok` semantics (TB-7+ supports re-attach).
- Reads (q_snapshot) are race-free by serde of on-disk state_roots.
- Writes (submit_typed_tx) — concurrent writes from two processes are NOT supported in the v4 substrate (single-writer assumption). For TB-10 MVP, the user is expected to invoke CLI subcommands SERIALLY: post-task, then run evaluator, then view-task. Concurrent invocation is undefined behavior; documented as known limitation.

This matches TB-9's keystore concurrent-access limitation (file-lock deferred to TB-16+ per `feedback_kolmogorov_compression`).

### §2.3 — Audit dashboard §11 filter convention

audit_dashboard §11 User Tasks filter:

```rust
let user_sponsored: Vec<_> = q.economic_state_t.task_markets_t.0.iter()
    .filter(|(_, e)| e.publisher.0.starts_with("Agent_user_"))
    .collect();
```

Naming-convention filter is sufficient for TB-10. Future TB may introduce a kernel-level role flag if needed.

### §2.4 — Replay determinism with new preseed factory

`default_pput_preseed_pairs()` returns a deterministic Vec (insertion order: tb7-7-sponsor, Agent_user_0, Agent_0..9). Replay determinism preserved because:
- The factory is pure (no env reads, no system clock, no random).
- The factory is called only once per process at chaintape bootstrap.
- Different preseed lists produce different genesis QState → different state_root_t → different ledger_root_t. Replay always uses the on-disk genesis_report.json + state_roots, not the factory.

If the factory is changed in a future TB, past chains continue to replay correctly because they use their on-disk artifacts; only fresh bootstraps use the new factory.

---

## §3 Auto-ratification verdict

```text
ratification_authority      = user 2026-05-02 ("authorized in auto mode until TB-10 is done")
charter_path                = handover/tracer_bullets/TB-10_charter_2026-05-02.md
questions_resolved          = Q1-Q8 (8/8)
clarifications_recorded     = §2.1 adapter constructors, §2.2 concurrent access,
                              §2.3 dashboard filter, §2.4 replay determinism
deviations_from_charter     = §7 Q2 corrected — NO genesis_payload.toml edit needed
                              (preseed is runtime-pure; trust_root unaffected)
verdict                     = RATIFIED
next_action                 = Atom 1 (runtime preseed factory + Agent_user_0 budget)
```

Charter §7 Q2 is updated in the charter file to reflect the §2 finding (preseed factory, not genesis_payload).

---

**End of ratification record.**
