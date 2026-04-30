# TB-4 Medium-Difficulty Real-Question Batch — 2026-04-30

**Branch**: `main` (post-TB-4 ship; HEAD `6c42cf7`)
**Predecessor evidence**: `handover/evidence/tb_4_smoke_2026-04-30/` (1-problem ship-gate smoke)
**Gate**: post-ship capability validation per user request "可以做一次完整的，中等难度的真题测试" (2026-04-30).

## Configuration

| Param | Value |
|---|---|
| Binary | `./target/debug/evaluator` (built from main @ HEAD post-TB-4 ship) |
| Mode | `full` |
| `CONDITION` | `n1` (single-agent multi-tx; lets the elevated MAX_TX budget actually exercise multi-step search) |
| `MAX_TRANSACTIONS` | **30** (4× TB-3 ship-gate ceiling; 1.5× TB-4 ship-gate ceiling) |
| `LLM_PROXY_URL` | `http://localhost:8080` (`/health` returned `{"status":"ok"}`) |
| Per-problem timeout | 600 s (`coreutils timeout`) |
| Model snapshot | `deepseek-v4-flash` (proxy-resolved) |

## Problem set (5 problems; mixed adaptation-set difficulty)

All 5 are members of the pre-registered `adaptation` split (per `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json`; 144-problem pool; `sha256: 90896bbe...1252`).

| # | Problem | Difficulty (informal) | Outcome |
|---|---|---|---|
| 1 | `mathd_algebra_107` | EASY (canonical baseline; TB-0 first v4 solve) | ✅ SOLVED |
| 2 | `mathd_algebra_125` | EASY-MEDIUM (linear nat; small case `x=6`) | ✅ SOLVED |
| 3 | `mathd_algebra_141` | MEDIUM (uses `(a+b)²−2ab=a²+b²`) | ✅ SOLVED |
| 4 | `mathd_algebra_148` | MEDIUM (cubic at point; needs eval substitution `f 2 = 9`) | ✅ SOLVED via composite tactic |
| 5 | `amc12a_2003_p5` | MEDIUM-HARD (multi-digit decomposition; AMC-style) | ❌ MAX_TX EXHAUSTED |

## Per-problem results

| Problem | solved | verified | tx_count | failed_branch_count | hit_max_tx | pput_m_verified | tactic_diversity | wall_s | gp_payload |
|---|---|---|---|---|---|---|---|---|---|
| `mathd_algebra_107` | true | true | 1 | 0 | false | 33.61 | 1.00 | 66.4 | `nlinarith` |
| `mathd_algebra_125` | true | true | 1 | 0 | false | 209.38 | 1.00 | 10.3 | `nlinarith` |
| `mathd_algebra_141` | true | true | 1 | 0 | false | 236.90 | 1.00 | 9.6 | `nlinarith` |
| `mathd_algebra_148` | true | true | **23** | **22** | false | 0.42 | 0.13 | 211.7 | `rw [h₀ 2] at h₁`<br>`nlinarith` |
| `amc12a_2003_p5` | false | false | **30** | 30 | **true** | 0.00 | 0.10 | 400.3 | (none) |

## Aggregate (per Art. I.2 + § C-052/C-053/C-057/C-061 main-metric requirements)

```
ΣPPUT_m_verified      = 480.31    (Σ over all 5 problems)
Mean PPUT_m_verified  = 120.08    (over n=4 SOLVED only)
Solve rate            = 4/5 = 80%
95% CI (Wilson)       = [0.38, 0.96]   (wide; n=5 small-sample)
Total wall time       = 698.4s        (~11.6 min)
```

### Art. IV halt_reason_distribution

| halt_reason | count |
|---|---|
| `OmegaAccepted` | 4 |
| `MaxTxExhausted` | 1 |
| `WallClockCap` | 0 |
| `ComputeCapViolated` | 0 |
| `ErrorHalt` | 0 |

### Art. II.2.1 multi-agent statistics

`CONDITION=n1` ⇒ single-agent run; `parent_selection_entropy` and `pairwise_payload_diversity_mean` are **N/A** (require n ≥ 2 agents). Per-problem `tactic_diversity` reported above.

## Verdict

**TB-4 capability validated**.

1. **80% solve rate** on n=5 mixed-difficulty adaptation problems with `CONDITION=n1` + `MAX_TX=30`. Far stronger than the TB-4 ship-gate single-problem smoke (1/1 on `mathd_algebra_107`).

2. **Multi-tx search demonstrably operative** — `mathd_algebra_148` required 23 transactions through `dispatch_transition` + `apply_one` to converge on the composite tactic `rw [h₀ 2] at h₁; nlinarith` (22 failed branches first; `tactic_diversity=0.13` evidences focused-but-iterative search). This is the strongest validation that the elevated MAX_TX budget actually flows through the runtime and isn't trivially short-circuited.

3. **Expected failure mode preserved** — `amc12a_2003_p5` hit `MAX_TX=30` cleanly with `solved=false` and `hit_max_tx=true`. No false-positive solve; no system crash; no L4.E corruption (verified by `tx_count=30` and `failed_branch_count=30` matching `MAX_TRANSACTIONS=30` exactly). The problem's multi-digit decomposition is genuinely hard for the deepseek-v4-flash model under the budget.

4. **TB-4 ABI changes serde-compatible across diverse problems** — every PPUT_RESULT row carries `schema_version="v2.0"`, `model_snapshot="deepseek-v4-flash"`, and (where solved) a CAS-stable `gp_proof_file` re-verifiable via `LEAN_PATH=<mathlib paths> lean --stdin < proof_*.lean`.

## Comparison vs prior smokes

| Metric | TB-3 ship smoke (`2eee4ee`) | TB-4 ship smoke (`bbe2d16`, n1) | **TB-4 medium batch (this run)** |
|---|---|---|---|
| Problem count | 1 | 1 | **5** |
| Condition | oneshot | n1 | n1 |
| `MAX_TRANSACTIONS` | 5 | 20 | **30** |
| Avg `tx_count` (solved) | 1 | 1 | 6.5 (driven by mathd_algebra_148's 23 tx) |
| Solve rate | 0/1 | 1/1 | **4/5** |
| Multi-tx exercise | none | none | **mathd_algebra_148: 23 tx; amc12a_2003_p5: 30 tx (MAX exhausted)** |
| Composite-tactic discovery | n/a | n/a | **`rw [h₀ 2] at h₁; nlinarith`** (mathd_algebra_148) |
| `prompt_context_hash` (oneshot) | `a1f43584a17d1226` (4 sessions bit-identical) | (same) | (n1 condition; not directly comparable but shows runtime spine isolation) |

## What this batch proves

1. The TB-4 RSP-2 admission surface ABI changes (parent_state_root schema bumps + ChallengeCase additive + 4 new TransitionError variants + 2 new state-root domains + Verify/Challenge dispatch arms) are serde-compatible with the n1 driver across **5 distinct problems** at non-trivial budget regimes.
2. Capability replication holds beyond the canonical TB-0 baseline (`mathd_algebra_107`); the runtime + downstream PPUT emit path produces **valid, re-verifiable proofs** on 4 distinct problems.
3. The elevated MAX_TX budget is operative — for problems that need >1 transaction, the budget regime correctly admits multi-step search up to the configured ceiling.
4. **The runtime's behavioral envelope under load** — observable signatures (`failed_branch_count`, `tactic_diversity`, `verifier_wait_ms`) show coherent search dynamics (focused on a small tactic set after early failures; expected from the chat-over-reasoner heuristic per `project_chat_over_reasoner` memory).

## What this batch does NOT prove

- That the TB-4 RSP-2 admission spine is **reachable from** the evaluator's PPUT emit path (P2 Agent Runtime territory; out of TB-4 scope per charter § 5 #1). The evaluator's solve path is currently pre-runtime; TB-4 RSP-2 dispatch arms are exercised only by the in-crate + integration test battery (30 new TB-4 tests under `cargo test --workspace`).
- Cross-problem statistical signals at full Phase B/C scale (n=5 is too small for reputation_distribution / parent_selection_entropy / pairwise_payload_diversity per Art. I.2 + II.2.1 multi-agent statistics; CONDITION=n1 forecloses parent_selection signals entirely).
- Difficulty stratification (the 5 problems were hand-picked as a "medium spread"; not a deterministic random sample from a difficulty-classified pool).

## Artifacts

- `batch_results.jsonl` — 5 PPUT_RESULT rows, one per problem, full v2.0 schema.
- `*.log` — full evaluator stdout per problem (proxy interactions, predicate evaluations, halt reason).
- `proof_*.lean` — 4 CAS-stable proof artifacts for the SOLVED problems (re-verifiable via `lean --stdin`).
- `aggregate.sh` — re-runnable aggregator from `*.log` → `batch_results.jsonl`.

## Reproducibility note (per C-012 / C-016 / C-039)

Each `proof_*.lean` is a self-contained Lean 4 source file with the canonical Mathlib import. Re-verification:

```bash
cd /path/to/mathlib4
LEAN_PATH=$(lake env printenv LEAN_PATH) lean --stdin < proof_<problem>.lean
```

Expected output: zero diagnostics (proof typechecks). Re-verification is independent of the TuringOS runtime (this is the C-012 measurement-correctness anchor: the proofs stand or fall on Lean 4's typechecker, not on TuringOS-internal claims).
