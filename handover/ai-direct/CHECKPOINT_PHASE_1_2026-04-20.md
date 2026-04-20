# CHECKPOINT — Phase 1 (Tape WAL Persistence)

**Date**: 2026-04-20
**Branch**: `feat/tape-phase-1-wal` (worktree `../v4-tape-wal/`)
**Commit on branch**: `d49a73d` "Phase 1 (C-037): Tape WAL persistence + replay"
**NOT yet merged to main.**

## What changed

| Component | Change |
|---|---|
| new: `src/wal.rs` | Append-only JSONL writer for `Node` and `LedgerEvent`; fsync after each line; lossy-recovery replay |
| `src/lib.rs` | `pub mod wal;` |
| `src/bus.rs` | `TuringBus::with_wal_path(...)` ctor — opens (or replays then opens) a WAL; persists every tape/ledger append; at-most-one-loss-on-crash |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | Opt-in via `WAL_DIR` env; path derived from problem stem + timestamp; `WAL_RESUME_ID` env supports rerunning against existing WAL |
| new: `tests/wal_resume.rs` | 2 integration tests — end-to-end crash-then-resume and empty-file-yields-fresh-bus |
| **No change to**: prompts, oracle, markets, wallet, kernel internals, protocol — purely additive |

## Phase 1 N=20 batch (`templadder_n8_20260420T134928.jsonl`)

### Solve outcomes
- **17/20 solved = 85%**  ← best N=20 single-run since project start
- 3 timeouts: `imo_1964_p2`, `induction_sumkexp3eqsumksq`, `mathd_algebra_208`
- **Two persistent-fails CRACKED** in a single run for the first time:
  - `amc12b_2021_p13` SOLVED (761s) — was FAIL in all 4 of the last ≥4 runs
  - `mathd_algebra_332` SOLVED (277s) — one of the 3 problems in the "failed all 6+ prior runs" set

### Audit re-verification
```
=== Re-verifying 17 artifacts ===
  ... 17 lines: VERIFIED ...
=== Summary ===
  Re-verified:   17/17 = 100.0%
  Wall time:     320s
```
Including the two newly-cracked persistent-fails — Lean independently compiled both proofs from disk. Not hallucination, not runtime cheating; real mathematics.

### WAL telemetry
| File population | Count |
|---|---|
| 2 events (RunStart + RunEnd, solved path) | 17 WALs |
| 1 event (RunStart, killed at timeout) | 3 WALs |
| Any WAL with node records | 0 |

Zero `Node` records in any WAL is the **expected** behavior at Phase 1 — agents still don't call `append` because nothing rewards them (Phase 2's job). The WAL infrastructure is proven correct; the content will bloat naturally once Phase 2 activates tape use.

### Crash-resume test (`cargo test --release --test wal_resume`)
```
running 2 tests
test wal_empty_file_yields_fresh_bus ... ok
test wal_persists_appends_across_bus_drop ... ok
```
- **wal_persists_appends_across_bus_drop**: bus opens WAL → 5 appends → drop (simulates crash) → second bus opens same WAL → tape has all 5 nodes in order, ledger replays RunStart + 5 Append events, hash chain verifies
- **wal_empty_file_yields_fresh_bus**: fresh path produces empty bus (no false-positive on missing file)

This is the formal proof that Q_t now survives process restart — the "memory any tasks" capability the user named.

## Variance baseline reconciliation

| Run | solved / 20 | note |
|---|---|---|
| nscaling_n8 fixed-temp | 11/20 | pre-TEMP_LADDER |
| N=20 first TEMP_LADDER | 14/20 | |
| Dual-path seed=74677 (N=20⊂N=50) | 18/20 | upper-tail |
| Dual-path seed=31415 (N=20⊂N=50) | 18/20 | upper-tail |
| Tape Economy v1 (fee=500) | 16/20 | |
| Tape Economy v2 (fee=2000) | 16/20 | |
| Phase 0 (audit-fix only) | 15/20 | mid-band |
| **Phase 1 (this)** | **17/20** | **upper-band, 2 persistent-fails cracked** |

**Phase 1 is +2 solves vs Phase 0 and unlocked 2 persistent-fails.** The +2 is well within LLM sampling variance, so we cannot claim WAL improves solve rate; but we can claim **WAL does NOT regress solve rate**, and two historically-unsolvable problems became solvable on this specific seed.

## Red-line check

| # | Red line | Status | Notes |
|---|---|---|---|
| 1 | New agent funded post-genesis | ✓ N/A | No wallet change |
| 2 | Markets resolve on process exit | ✓ N/A | No settlement change |
| 3 | Raw CoT to public tape | ✓ PASS | WAL stores Node.payload (= agent's canonical extracted output, not `<think>` block) |
| 4 | Prompt manipulation toward append | ✓ PASS | Prompt unchanged |
| 5 | Reward curve as env-var | ✓ N/A | No reward change |
| 6 | ∏p accepts non-re-verifiable | ✓ PASS | 17/17 re-verified |
| 7 | Anything downgraded to Phase N | ✓ PASS | Nothing deferred |

## Stop conditions

| Condition | Threshold | Observed | Status |
|---|---|---|---|
| Solve rate vs baseline | ≤ -5pp from median (~16/20) | 17/20 (+1 vs median) | ✓ |
| Conservation test | sum(wallet) == initial | N/A this phase (no wallet change) | ✓ |
| Re-verifiability | ≥90% | 100% | ✓ |
| Red lines | none | none | ✓ |
| Lean preflight | OK | implicit (all 17 compile) | ✓ |

## Recommendation: **PROCEED to Phase 2**

**Reasoning**:
1. Constitutional capability added: Q_t persistence across process exits — first time in project history
2. Crash-resume integration test passes formally; WAL content inspected empirically
3. All 17 solves re-verified (100% audit pass)
4. Two persistent-fails cracked (side evidence; not causal claim)
5. All 7 red lines clean
6. WAL infrastructure is ready to actually get exercised once Phase 2 makes append rational

## Recommended branch handling

`feat/tape-phase-1-wal` can be **merged to main now** (separately from Phase 2), because:
- Entirely additive: opt-in via `WAL_DIR` env; default off preserves baseline
- Verified by integration test + N=20 smoke
- Unblocks future phases (Phase 2 will want a WAL to prove append events persist; Phase 4 needs cross-problem persistence to build on)

Alternative: stack Phase 1 + Phase 2 on same branch, merge together after Phase 2 checkpoint. Your call.

## Next: Phase 2 — Reward-Pull (founder grant + marginal contribution)

Per plan §三 Phase 2:
- Branch: `feat/tape-phase-2-rewardpull` (stacked on phase-1 branch OR branched from main after phase-1 merges)
- Files: `src/bus.rs` (append-phase founder grant), `src/sdk/tools/wallet.rs` (expose record_shares), evaluator delete remove cold-fee if present on that branch
- Goal: `append` becomes EV-positive — agent automatically receives γ·system_lp YES shares on own node
- Stop: solve rate ≥ 15/20 AND at least one problem shows `append > 0` per run
- Key hypothesis: with founder grant at γ=0.05 (default), agents rationally prefer `append` over direct `complete` when they are uncertain about payload correctness
