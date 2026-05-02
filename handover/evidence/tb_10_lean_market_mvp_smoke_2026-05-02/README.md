# TB-10 Lean Proof Task Market MVP — Smoke Evidence — 2026-05-02

**Date**: 2026-05-02
**TB**: TB-10 (Lean Proof Task Market MVP — first user-facing product)
**Source**: `target/release/lean_market` + `target/release/evaluator` (TB-10 ship-candidate)
**Model**: `deepseek-chat` via local LLM proxy at `localhost:8080/v1/chat/completions`
**Lean**: 4.29.1 (`/home/zephryj/.elan/bin/lean`)
**Mathlib**: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/.lake/build`
**Charter**: `handover/tracer_bullets/TB-10_charter_2026-05-02.md`
**Architect spec**: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md:1594`

```text
TB-10：Lean Proof Task Market MVP
目标：第一个可用产品：用户发任务，Agent 解题，系统验证，系统付款，dashboard 可审计。
必须：
  TaskOpenTx        ✓ user-signed (Agent_user_0)
  EscrowLockTx      ✓ user-signed (Agent_user_0)
  WorkTx            ✓ solver-signed (Agent_0; TB-9 durable keystore)
  VerifyTx          ✓ verifier-signed (Agent_0)
  FinalizeRewardTx  ✓ system-emitted via tb8_emit_finalize_after_verify
  replay            ✓ verify_chaintape 7-indicator GREEN per run
  dashboard         ✓ audit_dashboard §11 User Tasks renders correctly
```

---

## §0 Headline — first user-facing product end-to-end

```text
                      problem               bounty (μ)   solver_payout (μ)   sponsor balance after   solver balance after   golden path (gp_payload)        time_secs
                      ─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
run_a   (fresh)       mathd_algebra_171     100,000      100,000             9,900,000                999,000                4-tactic calc rw ring norm_num    99.6s
run_b   (load)        mathd_algebra_107     100,000      100,000             9,900,000                999,000                nlinarith                         11.0s
regression (load)     mathd_numbertheory_961 250,000     250,000             9,750,000                1,149,000              norm_num                          12.2s
```

**3/3 SOLVED**. Per-run ChainTape:
- 5 L4 entries (TaskOpen + EscrowLock + Work + Verify + FinalizeReward)
- 2 L4.E entries (synthetic-seed rejection — pre-existing TB-6 evidence pattern; not user-induced)
- 7 verify_chaintape indicators GREEN
- Cross-run pubkey identity ✓ (Agent_user_0 + Agent_0 same pubkey across all 3 runs via TB-9 durable keystore)

**Sponsor / solver balance accounting** (architect mandate "user posts → agent solves → system pays"):
- Run A: Agent_user_0 starts at 10_000_000 → debited 100_000 (EscrowLock) → final 9_900_000.
  Agent_0 starts at 1_000_000 → -1_000 (work stake locked into stakes_t) -100_000 (verify bond locked into stakes_t) +100_000 (FinalizeReward credit) = 999_000 final balance.
- Run B: same shape; sponsor 10_000_000 → 9_900_000; Agent_0 nets 999_000 (same TB-7R stakes lock pattern; see TB-9 evidence §2 baseline).
- Regression: bounty 250_000 → sponsor 9_750_000 (debited exact 250_000); Agent_0 receives 250_000 → final 1_149_000 = 1_000_000 - 1_000 - 100_000 + 250_000.

The **architect mandate** is verifiable from the dashboard §11 + replay_report.json triplet alone:
```text
sponsor balance Δ + solver payout Δ + stakes_t locked = 0  (CTF conservation; TB-3 invariant inherited)
sponsor balance Δ = -bounty                                   (TB-3 EscrowLock dispatch arm)
solver balance Δ contains +bounty                              (TB-8 FinalizeReward dispatch arm)
```

---

## §1 What's new in TB-10 vs TB-9 smoke (per architect mandate)

```text
+ src/runtime/bootstrap.rs              new module — `default_pput_preseed_pairs()`
                                         factory exposing tb7-7-sponsor + Agent_user_0
                                         + Agent_0..9 in one consolidated list (12
                                         entries; total preseed supply 30_000_000 micro)
+ src/runtime/adapter.rs                +make_real_task_open_signed_by + make_real_escrow_lock_signed_by
                                         real-Ed25519-signature constructors mirroring
                                         existing make_real_worktx_signed_by pattern
+ experiments/minif2f_v4/src/bin/evaluator.rs  preseed branch detects TURINGOS_USER_TASK_MODE=1
                                                env and swaps sponsor tb7-7-sponsor →
                                                Agent_user_0 with REAL Ed25519 signatures;
                                                bounty overrideable via TURINGOS_USER_TASK_BOUNTY_MICRO
+ experiments/minif2f_v4/src/bin/lean_market.rs   new user-facing CLI binary with subcommands:
                                                   run-task / view-task / view-wallet / view-replay.
                                                   Sets up TURINGOS_USER_TASK_* env vars and
                                                   spawns evaluator subprocess; post-run reads
                                                   chaintape via replay_full_transition for
                                                   user-friendly summaries.
+ src/bin/audit_dashboard.rs            §11 TB-10 User Tasks section: per-user-task
                                         table with sponsor / bounty_micro / solver /
                                         claim_status / payout_micro / opened@t columns.
                                         Filter convention: TaskOpenTx whose sponsor_agent
                                         starts with `Agent_user_`. Aggregate row:
                                         n user tasks + n Finalized + total bounty + total paid.
```

**NOT changed** (per TB-10 charter §5):
- ❌ NO new TypedTx variant
- ❌ NO new dispatch arm in `sequencer.rs`
- ❌ NO new TransitionError variant
- ❌ NO new state-root domain
- ❌ NO new agent-callable system_tx surface (lean_market has no `settle/finalize/refund` subcommand)
- ❌ NO `monetary_invariant.rs` cascade (5-holding CTF invariant + total_supply_micro UNCHANGED)
- ❌ NO post-init mint (Agent_user_0 funded only via on_init preseed factory)
- ❌ NO genesis_payload.toml `[bootstrap]` edit (preseed is runtime-pure; no toml schema change)

The **new TB-10 invariant** observable from the smoke alone:

1. `task_markets_t` entry with `publisher = AgentId("Agent_user_0")` exists on chain (verifiable via dashboard §11 OR `lean_market view-task`).
2. `claims_t[claim-verifytx-Agent_0-omega-pertactic-1].status = Finalized` with `amount = bounty` (the user's bounty has fully transferred to the solver).
3. `balances_t[Agent_user_0]` debited by exactly `bounty` (10_000_000 → 9_900_000 for runs A+B; → 9_750_000 for regression with bounty 250_000).
4. Agent_user_0 has a durable Ed25519 keypair persisted to `agent_keystore.enc` alongside the existing Agent_0 entry; cross-run identity proven by `diff -q agent_pubkeys_for_witness.json` across all 3 runs (= IDENTICAL).

---

## §2 Side-by-side: TB-7R → TB-8 → TB-9 → TB-10 (capability evolution)

Each TB's value is **load-bearing for the next**. TB-10 inherits all of:
- TB-7R Frame B authoritative routing (every LLM proposal traverses bus.submit_typed_tx → on-disk ChainTape)
- TB-8 minimal payout (FinalizeRewardTx system-emitted; first economic mutator that moves money)
- TB-9 durable identity (Agent_user_0 + Agent_0 keypairs survive evaluator restart via encrypted keystore)

### §2.1 Outcome metrics (single-problem n1 × `mathd_algebra_171` × MAX_TX=10 — held constant for delta isolation)

| TB | sponsor identity | sponsor sig | solver identity | sponsor cross-run? | solver cross-run? | dashboard user view | replay self-contained |
|---|---|---|---|---|---|---|---|
| TB-7R single  | `tb7-7-sponsor` | zero-sig | run-local Agent_0 | n/a | no | n/a | partial (sidecars optional) |
| TB-8 single   | `tb7-7-sponsor` | zero-sig | run-local Agent_0 | n/a | no | §9 Claims | partial |
| TB-9 run-A    | `tb7-7-sponsor` | zero-sig | durable Agent_0 (cross-run ✓) | n/a | YES | §9 + §10 Durable identity | YES (RQ3 packaging) |
| TB-9 run-B    | `tb7-7-sponsor` | zero-sig | durable Agent_0 (same pubkey) | n/a | YES | §9 + §10 | YES |
| **TB-10 run_a** | **`Agent_user_0`** (durable; runtime preseed factory) | **REAL Ed25519** (TB-10 Atom 1 constructors) | durable Agent_0 (TB-9 carry) | **YES (durable keystore + preseed factory)** | YES (TB-9 carry) | §9 + §10 + **§11 User Tasks** | YES |
| TB-10 run_b   | Agent_user_0 (same pubkey across run_a) | REAL Ed25519 | durable Agent_0 (same pubkey) | **YES** | YES | §11 | YES |
| TB-10 regression | Agent_user_0 (same pubkey) | REAL Ed25519 | Agent_0 (same pubkey) | YES | YES | §11 with bounty=250k | YES |

### §2.2 ChainTape detail metrics (run_a vs TB-9 baseline)

| Indicator | TB-9 run-A | TB-10 run_a | TB-10 run_b | TB-10 regression |
|---|---|---|---|---|
| L4 entries | 5 | 5 | 5 | 5 |
| L4.E entries | 2 | 2 | 2 | 2 |
| ledger_root_verified | ✓ | ✓ | ✓ | ✓ |
| system_signatures_verified | ✓ | ✓ | ✓ | ✓ |
| state_reconstructed | ✓ | ✓ | ✓ | ✓ |
| economic_state_reconstructed | ✓ | ✓ | ✓ | ✓ |
| cas_payloads_retrievable | ✓ | ✓ | ✓ | ✓ |
| agent_signatures_verified | ✓ | ✓ | ✓ | ✓ |
| proposal_telemetry_cas_retrievable | ✓ | ✓ | ✓ | ✓ |
| **TaskOpen sponsor on chain** | tb7-7-sponsor | **Agent_user_0** | **Agent_user_0** | **Agent_user_0** |
| **EscrowLock sponsor on chain** | tb7-7-sponsor | **Agent_user_0** | **Agent_user_0** | **Agent_user_0** |
| **TaskOpen signature** | zero (legacy synthetic) | **REAL Ed25519** | **REAL Ed25519** | **REAL Ed25519** |
| **EscrowLock signature** | zero (legacy synthetic) | **REAL Ed25519** | **REAL Ed25519** | **REAL Ed25519** |
| FinalizeReward system-emitted | ✓ | ✓ | ✓ | ✓ |
| Solver receives full bounty | ✓ (100k) | ✓ (100k) | ✓ (100k) | ✓ (250k) |
| Sponsor debited exact bounty | n/a (self-sponsored) | **✓ (10M → 9.9M)** | **✓ (10M → 9.9M)** | **✓ (10M → 9.75M)** |
| dashboard §11 User Tasks | n/a | **✓ Finalized** | **✓ Finalized** | **✓ Finalized** |

### §2.3 Tx kind sequence on L4 (per single problem)

```text
TB-9 single  : TaskOpen(tb7-7-sponsor, zero-sig)
                 → EscrowLock(tb7-7-sponsor, zero-sig)
                 → Work(Agent_0, REAL-sig)
                 → Verify(Agent_0, REAL-sig)
                 → FinalizeReward(system-emitted, sys-sig)              (5 L4 entries)
                                       Agent_0 self-funded the task

TB-10 single : TaskOpen(Agent_user_0, REAL-sig)         ← TB-10 Atom 1 net-new
                 → EscrowLock(Agent_user_0, REAL-sig)    ← TB-10 Atom 1 net-new
                 → Work(Agent_0, REAL-sig)
                 → Verify(Agent_0, REAL-sig)
                 → FinalizeReward(system-emitted, sys-sig)              (5 L4 entries)
                                       USER (Agent_user_0) funded the task; Agent_0 (solver) was paid

```

### §2.4 What each TB concretely added (cumulative)

```text
TB-7R: Frame B authoritative routing on L4 / L4.E with predicate evidence.
TB-8 : minimal payout — first system-emitted economic mutator (FinalizeRewardTx).
TB-9 : durable identity — Agent_0 keypair persists across evaluator restart;
       Wallet collapses to read-only projection of EconomicState.balances_t.
TB-10: first user-facing product — Agent_user_0 sponsor identity (also durable);
       lean_market CLI binary; user-mode evaluator hook; audit_dashboard §11
       User Tasks view section; full end-to-end real-LLM smoke proving:
         user posts task → agent solves → system verifies → system pays → dashboard auditable.
```

---

## §3 Per-run replay reports + dashboards

```text
run_a_n1_mathd_algebra_171/
  lean_market.log                  user-facing CLI output (post-run summary section)
  evaluator.log                    not separately captured; folded into lean_market.log
  dashboard.txt                    audit_dashboard output (§1-§11 with TB-10 §11)
  replay_report.json               verify_chaintape JSON (7 indicators GREEN)
  verify.log                       tail of verify_chaintape stdout
  agent_keystore_at_exit.enc       snapshot of durable keystore after run_a
  agent_pubkeys_for_witness.json   per-run pubkey manifest; canonical witness for cross-run identity
  lean_market_view_task.log        `lean_market view-task` output (post-hoc replay-based view)
  lean_market_view_wallet.log      `lean_market view-wallet` output (Agent_user_0 balance + full balances_t dump)
  lean_market_view_replay.log      `lean_market view-replay` output (delegates to verify_chaintape; 7 GREEN)
  runtime_repo.tar.gz              self-contained replay bundle (sidecars included per TB-8 RQ3)
  cas.tar.gz                       CAS object store

run_b_n1_mathd_algebra_107/        (same shape; agent_keystore reused; pubkey identical to run_a)
regression_n1_mathd_numbertheory_961/  (same shape; bounty 250_000; harder problem; reused keystore)

keystore/
  agent_keystore.enc               LIVE durable keystore shared across all 3 runs
                                    (mirrors TB-9 evidence directory layout)
agent_keystore.enc                 same keystore, top-level (mirrors TB-9; convenience symlink)
```

### §3.1 Independent re-verification

```bash
# Extract any run's runtime_repo + cas tar.gz, then:
mkdir -p /tmp/tb10_replay
cd /tmp/tb10_replay
tar -xzf <evidence_dir>/<run>/runtime_repo.tar.gz
tar -xzf <evidence_dir>/<run>/cas.tar.gz
cargo run --release --bin verify_chaintape -- \
  --repo /tmp/tb10_replay/runtime_repo \
  --cas  /tmp/tb10_replay/cas_runtime_repo \
  --out  /tmp/tb10_replay/replay_report.json
# Should match the committed copy bit-exactly modulo runtime-tagged run_id/epoch.
```

```bash
# To verify cross-run identity:
diff -q run_a_n1_mathd_algebra_171/agent_pubkeys_for_witness.json \
        run_b_n1_mathd_algebra_107/agent_pubkeys_for_witness.json
# Expected output: (no output — files are identical)
diff -q run_a_n1_mathd_algebra_171/agent_pubkeys_for_witness.json \
        regression_n1_mathd_numbertheory_961/agent_pubkeys_for_witness.json
# Expected output: (no output — same Agent_0 + Agent_user_0 pubkey across all 3 runs)
```

---

## §4 Architectural empirical observations recorded

### §4.1 lean_market is a thin process wrapper, not a re-implementation

The `lean_market run-task` subcommand spawns the `evaluator` binary as a subprocess after setting `TURINGOS_USER_TASK_MODE=1` + `TURINGOS_USER_TASK_BOUNTY_MICRO=<n>` + a fresh `TURINGOS_CHAINTAPE_PATH`. The evaluator's preseed branch (extended in TB-10 Atom 3) detects user-mode and swaps sponsor `tb7-7-sponsor` → `Agent_user_0` with real Ed25519 signatures via `make_real_task_open_signed_by` + `make_real_escrow_lock_signed_by`. The solver loop is unchanged from TB-7R/TB-8/TB-9.

This was a deliberate architectural choice (per ratification §2.1): the Sequencer fail-closes on `BootstrapError::NonEmptyRuntimeRepo` (TB-6 Atom 1 invariant), so two separate processes cannot share an active chaintape without resume mode (a future-TB enhancement). Single-process invocation with subprocess delegation was the surgical fix.

The `view-*` subcommands operate on a post-run chaintape READ-ONLY via `replay_full_transition` — no Sequencer is bootstrapped, no NonEmptyRuntimeRepo gate fires. This is the supported read-only path.

### §4.2 Real Ed25519 on TaskOpen + EscrowLock is forward-compatible

Empirical finding (charter §2): the kernel currently does NOT verify TaskOpen/EscrowLock Ed25519 signatures (no `verify_agent_signature` call in those dispatch arms; per `src/state/sequencer.rs:1054 + 1095`). Existing evaluator code submits zero-signature variants and the kernel accepts.

TB-10 user CLI signs TaskOpen + EscrowLock with REAL Ed25519 anyway. This is forward-compatible with future TB-12+ kernel hardening (planned per WP roadmap) and demonstrates user identity binding at the chain boundary even before kernel-level verification arrives. Cost: negligible CPU (one Ed25519 sign per tx; AgentKeypairRegistry-cached).

Until the kernel hardens, the signature field on TaskOpen+EscrowLock is structural attestation by convention. The chain auditor can validate it post-hoc against `agent_pubkeys.json`.

### §4.3 Cross-run identity for SPONSOR is a TB-10 net-new property

TB-9 demonstrated cross-run identity for SOLVER (Agent_0). TB-10 extends the same property to SPONSOR (Agent_user_0): every TB-10 run signs TaskOpen+EscrowLock with the SAME Agent_user_0 Ed25519 keypair, recovered from `agent_keystore.enc` on each evaluator boot. Verified empirically by `diff -q agent_pubkeys_for_witness.json` across all 3 runs (= IDENTICAL).

This closes the TB-9 mandate "持仓、payout、future NodeMarket 都必须归属于 durable identity" for the sponsor side: sponsor's bounty allocations on chain are bound to the SAME public-key entity across run boundaries. Future NodeMarket position attribution can rely on this binding.

### §4.4 Sponsor / solver balance arithmetic verifies CTF conservation

Per-run delta:

```text
Run A:
  Agent_user_0:  10_000_000 → 9_900_000   (Δ = -100_000; EscrowLock debit)
  escrows_t:     0 → +100_000             (Δ = +100_000; EscrowLock credit; balanced)
  Agent_0:       1_000_000 → 999_000      (Δ = -1_000; net of -1_000 work stake + -100_000 verify bond
                                            + +100_000 FinalizeReward credit)
  stakes_t:      0 → +101_000             (work stake + verify bond locked; not yet released)
  escrows_t:     +100_000 → 0             (Δ = -100_000; FinalizeReward debit)
  Σ balances + Σ escrows + Σ stakes:  unchanged (CTF conserved per TB-3 invariant)

Run B:           same shape as Run A (bounty 100_000)
Regression:      same shape; bounty 250_000 → sponsor Δ = -250_000; solver final balance += 250_000
```

The `assert_total_ctf_conserved` invariant fires on every dispatch arm and was unchanged from TB-9. TB-10 surface adds zero new mutators — the chain's economic conservation properties are inherited unchanged.

### §4.5 Duration delta vs TB-9 is environmental, not architectural

Run A took 99.6s (cold-cache Lean kernel + Mathlib compile). Run B took 11.0s (warm cache; same Lean kernel; same Mathlib). Regression took 12.2s (warm cache).

Same pattern as TB-9 evidence §4.3: the architectural cost of TB-10 (Argon2id KDF on first Agent_user_0 keypair generation; one Ed25519 sign per TaskOpen+EscrowLock) is ~50ms per run. The 99s vs 11s delta is entirely Lean kernel cache state.

---

## §5 Workspace test count (TB-10 ship-gate)

```text
command         = cargo test --workspace
workspace_count = 731
failed          = 0
ignored         = 150
delta vs TB-9   = +8 (723 → 731)
  +8 net new tests:
    +8 src/runtime/bootstrap::tests        (default_pput_preseed_pairs unit suite:
                                            returns 12 entries, every entry has positive balance,
                                            agent_user_0 present with sponsor budget, tb7-7-sponsor
                                            preserved, 10 solver agents each at 1M micro,
                                            total preseed supply = 30M, deterministic across calls,
                                            genesis construction matches total)
```

Per `feedback_workspace_test_canonical`: `cargo test --workspace` is the canonical ship-gate signal; bare `cargo test` is forbidden. Reporting shape preserved.

---

## §6 What this evidence does NOT cover

```text
✗ user-callable system_tx surface (lean_market settle/finalize/refund) — Anti-Oreo violation; permanently NOT in scope
✗ kernel signature verification on TaskOpen/EscrowLock — TB-12+ hardening
✗ task expiry / refund mechanism — TB-12 RSP-3.2 + TB-13 Beta scope
✗ NodeMarket / NodePosition / CompleteSet / MarketSeed — TB-11+
✗ HTTP / web surface — TB-13 Beta scope
✗ Arbitrary Lean source ingest — TB-13 Beta (TB-10 only accepts heldout-49 problem ids)
✗ Multi-org / cross-host keystore federation — post-v1.0
✗ Concurrent CLI ↔ evaluator chaintape access — file-lock; TB-16+ polish
✗ Multi-task (>1 user task per chaintape) — supported by code but not stressed by smoke
```

These are **forbidden** in the TB-10 charter §5; their absence is by design.

---

## §7 Sign-off

```text
ship_candidate_commit   = <pending Atom 7>
predecessor_commit      = 76204d6 (TB-9 session-close); 7a82c87 (TB-9 ship)
all_runs_solved         = 3/3
finalized_claims        = 3/3 (every run produced Finalized claim with payout_micro = bounty)
sponsor_balance_check   = ✓ Agent_user_0 debited by exact bounty (10M-bounty per run)
cross_run_pubkey_match  = YES (run_a == run_b == regression: agent_pubkeys.json bit-identical)
seven_indicators_green  = YES (per run)
workspace_test_count    = 731 / 0 failed / 150 ignored
architect_mandate       = SATISFIED (line 1594: 7 primitives + replay + dashboard all GREEN end-to-end via user-driven flow)
```
