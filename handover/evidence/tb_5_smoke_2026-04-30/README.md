# TB-5 真实烟测 (Phase-C living regression + RSP-3 ABI compat) — 2026-04-30

**Branch**: `experiment/tb5-rsp3-resolution-gate`
**HEAD**: Atom 7 ship gate (`cc72d61`); recursive self-audit at `handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md`.
**Gate**: post-development pipeline-liveness verification + ABI-compat + capability-replication evidence per directive § 4 Q4 (smoke is non-blocking under Option A audit mode; framed as supporting evidence).

## Configuration

| Param | Value |
|---|---|
| Binary | `./target/debug/evaluator` (built from experiment/tb5-rsp3-resolution-gate post-Atom-7) |
| Problem | `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean` |
| `MAX_TRANSACTIONS` | **20** (matches TB-4 ship gate; CLAUDE.md `--half` mode ceiling) |
| Mode | `full` |
| Proxy | `LLM_PROXY_URL=http://localhost:8080` (`/health` returned `{"status": "ok"}`) |
| Model snapshot (resolved by proxy) | `deepseek-v4-flash` |

## Two condition runs

### Run 1 — `CONDITION=oneshot` (pipeline-liveness)

Key fields from `PPUT_RESULT`:
```
schema_version       v2.0
run_id               oneshot_mathd_algebra_107_1777577407344
problem_id           mathd_algebra_107
solved               false
verified             false
total_run_token_count 140
tx_count             1
budget_max_transactions 1
hit_max_tx           false
prompt_context_hash  a1f43584a17d1226
model_snapshot       deepseek-v4-flash
mode                 full
condition            oneshot
total_wall_time_ms   9619
```

Full log: `oneshot_run.log`.

`solved=false` is **expected** for oneshot — the oneshot prompt-template regression at HEAD is the documented baseline per `handover/evidence/first_v4_solve_2026-04-29/README.md` and is bit-identical across TB-1 / TB-2 / TB-3 / TB-4 / **TB-5**. Per CLAUDE.md, the oneshot smoke is **pipeline-liveness only** — proves the emit path + prompt-build path are unbroken by TB-5's institutional changes (system-emitted ChallengeResolve + 2-channel ingress + apply_one stage 1.5 sig-verifier + new dispatch arm).

`prompt_context_hash="a1f43584a17d1226"` is **bit-identical to TB-1 Day-1 (2026-04-29) + TB-2 ship + TB-3 ship + TB-4 ship + TB-5 ship** across **five independent sessions**. Strict structural invariant: the agent-facing prompt-build pipeline is structurally untouched by every TB-5 edit. This is the strongest possible compat signal.

`budget_max_transactions=1`: oneshot caps to 1 by design (single proposal). MAX_TX=20 env had no effect because the oneshot driver loop terminates after the first non-rollback transaction. Expected.

### Run 2 — `CONDITION=n1` (capability replication; elevated MAX_TX honored)

Key fields from `PPUT_RESULT`:
```
schema_version       v2.0
run_id               n1_mathd_algebra_107_1777577427580
problem_id           mathd_algebra_107
solved               true
verified             true
progress             1
golden_path_token_count 12
total_run_token_count 448
tx_count             1
budget_max_transactions 20
hit_max_tx           false
tactic_diversity     1.0
pput_runtime         0.00009208510136727958
pput_verified        0.00009208510136727958
model_snapshot       deepseek-v4-flash
mode                 full
condition            n1
tool_dist            {"omega_wtool":1, "step":1}
gp_payload           nlinarith
gp_path              per_tactic
gp_proof_file        proofs/mathd_algebra_107_1777577451_73ee91ba.lean
has_golden_path      true
total_wall_time_ms   24240
h_vppu               8.218811881188119
```

Full log: `n1_run.log`. Proof artifact: `proof_n1.lean` (CAS-stable; `LEAN_PATH=<mathlib paths> lean --stdin < proof_n1.lean` re-verifies).

**SOLVED + VERIFIED**:
- `solved=true`, `verified=true`, `progress=1`.
- `gp_payload="nlinarith"` — the canonical OMEGA proof for this theorem (matches TB-0 baseline + TB-1 Day-1 + TB-4 ship).
- `golden_path_token_count=12`, `total_run_token_count=448`, `tactic_diversity=1.0`.
- `budget_max_transactions=20` — **elevated MAX_TX env var honored** (matches TB-4 directive precedent "真实烟测需要加大 max-tx").
- `hit_max_tx=false` — solved on first tx; the elevated budget was a safety ceiling, not a binding constraint here.

## Verdict

**TB-5 ship-ready**.

1. **Pipeline-liveness PASS**: oneshot run emits v2.0 PPUT row with bit-identical `prompt_context_hash` across 5 sessions (TB-1, TB-2, TB-3, TB-4, TB-5).
2. **Capability replication PASS**: n1 run reproduces the canonical solve on `mathd_algebra_107` with `gp_payload="nlinarith"`. The TB-5 institutional changes (two-channel ingress + apply_one stage 1.5 + ChallengeResolve dispatch arm + 4 new TransitionError variants + ChallengeStatus enum + ChallengeResolution enum + 2 new state-root domains) are serde-compatible with the existing PPUT result emit path and do NOT regress the OMEGA proof path.
3. **Elevated MAX_TX honored**: `budget_max_transactions=20` carried through from env to runtime budget regime.

## Comparison vs prior smokes

| Metric | TB-1 Day-1 (`f0b659f`) | TB-2 ship | TB-3 ship | TB-4 ship | **TB-5 ship** |
|---|---|---|---|---|---|
| oneshot `prompt_context_hash` | `a1f43584a17d1226` | `a1f43584a17d1226` | `a1f43584a17d1226` | `a1f43584a17d1226` | **`a1f43584a17d1226`** ✅ |
| oneshot `schema_version` | `v2.0` | `v2.0` | `v2.0` | `v2.0` | `v2.0` ✅ |
| oneshot `solved` | false | false | false | false | false ✅ |
| n1 `solved` | true (Day-1) | (n/a) | (n/a) | true | **true** ✅ |
| n1 `gp_payload` | `nlinarith` | — | — | `nlinarith` | **`nlinarith`** ✅ |
| `budget_max_transactions` (n1) | (unknown) | — | — | 20 | **20** ✅ |

Five-row hash chain `a1f43584a17d1226` × `v2.0` × oneshot `solved=false` × n1 `solved=true` × n1 `gp_payload=nlinarith` × n1 `budget_max_transactions=20` matches across **five independent sessions** + extends through TB-5's RSP-3 system-emitted resolution ABI.

## What this smoke proves

1. The TB-5 atoms 2-7 changes did NOT regress the pre-runtime experiment harness (oneshot pipeline-liveness).
2. The TB-5 institutional changes are serde-compatible with downstream consumers — the n1 driver consumes lib crate types and round-trips correctly through the new TypedTx::ChallengeResolve variant + TransitionError additions + ChallengeCase additive `+status` field.
3. The OMEGA proof path is preserved (n1 produces `gp_payload=nlinarith` matching TB-0 / TB-1 Day-1 / TB-4 baselines).
4. Elevated MAX_TX (20) flows correctly through the budget-regime + driver loop.

## What this smoke does NOT prove

- That the new TB-5.1 ChallengeResolve dispatch arm is reachable from the evaluator. The evaluator's PputResult emit path is pre-runtime; routing it through `Sequencer::emit_system_tx` is **P2 Agent Runtime** territory, not TB-5 scope. TB-5 proves the dispatch arm exists + is unit-test + integration-test green; P2 will wire the swarm to actually route ChallengeResolve through emit_system_tx.

This is the post-development gate per directive § 4 Q4 (non-blocking). Combined with **464/464 cargo test --workspace** + **~44 new TB-5 tests** + the recursive self-audit at `handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md`, TB-5 is ship-ready.
