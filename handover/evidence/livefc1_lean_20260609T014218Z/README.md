# LIVE-FC1 Lean-tier — REAL Lean-oracle Verified PPUT (A5-closing run)

Run id: `livefc1-lean-20260609T014218Z` (UTC 2026-06-09).
Worktree: `turingosv4-ln` branch `claude/livefc1-lean`, base `origin/main 1addeea0`
(all of LIVE-FC1 Phase 1-7 merged).

## Why this run exists

The Phase-7 swarm run (`livefc1_swarm_20260608T233701Z`) had VPPUT `progress=0` for
every task — correctly — because a **math benchmark has no external oracle**: no
`VerificationResult.verified` ground-truth witness fires, so the VPPUT numerator is
honestly gated to 0. **Lean is a real external oracle** (the Lean kernel = ground
truth). A Lean-verified proof produces an accepted golden path WITH a verified
`VerificationResult` on the canonical tape, so `reconstruct_vpput_from_tape` yields
`progress=1` and a **NON-ZERO** `verified_pput_micro`. That is the one step that
closes acceptance **A5** (a non-zero held-out Verified PPUT).

## Oracle preflight (BEFORE any paid call, per `feedback_oracle_preflight`)

- Pinned Lean toolchain `leanprover--lean4---v4.24.0` present at
  `~/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean`.
- Trivial GOOD core theorem (`n + 0 = n := by simp`) → exit 0; `#print axioms` →
  `[propext]` (whitelisted). Wrong theorem (`2 + 2 = 5 := by rfl`) → exit 1 (real
  type error). The oracle discriminates correctly.
- `judges::lean_theorem_bank::tests::reference_proofs_verify` (real-run, gated on the
  pinned toolchain) → **ok**: all 5 CORE reference proofs Verified through the actual
  `LeanJudge` path; the Mathlib entry correctly skipped (no Mathlib build — this run
  uses **core-only** theorems, no Mathlib cache needed).

## What ran (REAL, on the canonical ChainTape)

A thin **UNPINNED** orchestrator (`src/bin/livefc1_lean_runner.rs`, genesis pin-count
0) drove a real DeepSeek agent over **3 EASY CORE Lean theorems** through the local
`llm_proxy.py`, on ONE shared canonical ChainTape. Per theorem:

1. Phase-5 budget check (live tape spend vs signed ceiling, 20 000 micro-units).
2. Phase-6 brand-GENERIC `ProviderHandleCapsule` anchored on canonical CAS (opaque
   sha256 handle; brand→handle mapping stays external-only).
3. REAL LLM call → a Lean proof BODY; **REAL Lean kernel** (`LeanJudge`) verdicts it.
4. On a kernel-VERIFIED proof: a CAS `VerificationResult{verified:true}` (the oracle
   witness) is **linked into `ProposalTelemetry.verification_result_cid`**, an accepted
   L4 spine `TaskOpen→EscrowLock→WorkTx` + a `VerifyTx(Confirm)` lands, and a
   `TerminalSummary{run_outcome=OmegaAccepted}` is emitted for that task.

Those are EXACTLY the two gates `reconstruct_vpput_from_tape` requires for
`progress=1` (`omega_terminal` AND `oracle_verified`).

### Theorems attempted → which VERIFIED (real Lean)

| theorem id | LLM proof body (DeepSeek) | Lean verdict | progress | vppu_micro |
|---|---|---|---|---|
| `calib_core_add_zero` | `exact Nat.add_zero n` | **Verified** | 1 | 1862 |
| `calib_core_le_succ`  | `omega` | **Verified** | 1 | 1904 |
| `calib_core_decide_le`| `decide` | **Verified** | 1 | 1904 |

All 3 verified on the first attempt. Note `add_zero` was proven with
`exact Nat.add_zero n`, NOT the bank reference body `simp` — genuine model output,
not a leaked reference.

## On-tape verification (`livefc1_swarm_verify`, pure read of the canonical tape)

- **VPPUT: `ground_truth_solved = 3`**; per-task `progress=1`, `verified_pput_micro` =
  **1862 / 1904 / 1904** (all > 0).
- **NON-ZERO held-out VPPUT**: H-VPPUT (held-out mean over the 3 tasks) = **1890
  micro-units**; best single-task = **1904 micro-units**.
- Observer: FC2 boot=LIVE tick=LIVE terminal=LIVE; **zombie_count=0**; L4 entries=17,
  L4.E=0.
- **`replay_roots_match_genesis = true`** (from-genesis replay roots match).
- `distinct_provider_handles = 1` (capsules=3) — single-provider (DeepSeek) Lean run;
  ≥2 heterogeneity is the **already-merged swarm run**'s job, not A5.

## Cost (eat-our-own-dogfood, BOUNDED)

- **529 real tokens** total (proxy `estimated_count=0` ⇒ real DeepSeek tokens), 3
  requests, 0 errors. Wall-clock 6.7 s.
- Blended pricing (~$0.1–0.6 / 1M tokens) ⇒ **well under US$0.01 (sub-cent)**.
- Budget ceiling armed at 20 000 micro-units; real spend stayed under, so the
  budget gate returned WITHIN on every attempt (FC2-HALT arm not triggered this run).

## A5 verdict

**A5 CLOSED — non-zero VPPUT, Lean-verified.** A real Lean-oracle-verified golden
path on the canonical ChainTape yields a NON-ZERO held-out Verified PPUT
(H-VPPUT = 1890 micro-units; per-task up to 1904). Honest, bounded, sub-cent.

## Files

- `metrics.json` — consolidated A5 metrics (per-task progress + cost + ticks +
  vppu_micro, H-VPPUT, observer summary, replay, handle count, budget, honesty notes).
- `verify_metrics.json` — raw output of `livefc1_swarm_verify` (the canonical on-tape
  reconstruction).
- `lean_run_sidecar.json` — the run sidecar (attempts, LLM proof bodies, brand
  sidecar, VerificationResult CIDs).
- `budget_manifest.toml` — the signed/approved Phase-5 budget ceiling (unpinned).
- `tape_bundle.tar.gz` — the canonical tape: git-backed L4 ChainTape (`runtime_repo`
  incl. `.git` objects), `rejections.jsonl` (L4.E), and 327 CAS objects.

## Honesty / anti-overclaim

- `progress=1` is on tape ONLY because the REAL Lean kernel returned success
  (`LeanJudge.is_verified()`), not a fabricated witness. The VPPUT reconstruction
  gates `progress` on a CAS `VerificationResult{verified:true}` resolvable via the
  telemetry link AND a `TerminalSummary{OmegaAccepted}` — both present here.
- A FAILED Lean attempt would land as an L4.E `LeanFailed` row (token-spent, counted),
  never as a verified path; the runner has that arm wired, it simply did not fire
  because all 3 easy theorems verified first try.
- No pinned (genesis) file changed. `src/bin/livefc1_lean_runner.rs` is a new UNPINNED
  bin (genesis pin-count 0); it reuses `LeanJudge`, the swarm canonical-spine helpers,
  and the Phase-1..6 observe-only mechanisms.
- `verify_metrics.json` carries one stale hardcoded printf string
  (`"progress=0 for all math tasks…"`) inherited verbatim from the swarm-context
  `livefc1_swarm_verify` binary; it is contradicted by that same file's actual
  reconstructed data (`vpput_ground_truth_solved=3`, per-task `progress=1`). The
  canonical reconstruction values are authoritative.
