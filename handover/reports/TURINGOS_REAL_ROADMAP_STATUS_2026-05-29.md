# TuringOS Real Roadmap Status Correction — 2026-05-29

This report corrects the "TuringOS 的真实落地路线图" article against the
current merged baseline, `origin/main` at `1f00012d` (PR #212). It is a derived
view. ChainTape/CAS, executable gates, and merged PR receipts win on conflict.

## Summary

The article's strategic center is still right: TuringOS should be framed first
as a Git/ChainTape/CAS/replay-native verified runtime for objective work, not as
a public prediction market or a generic coding assistant.

The article is stale in three important ways:

1. `handover/ai-direct/LATEST.md` still had a 2026-05-27 / PR #206 headline,
   while merged main is now 2026-05-29 / PR #212.
2. The constitution gate count is now `164 failed=0`, not `165 failed=0`,
   because PR #213 retired the `turingos_dev` sidecar and its gate.
3. The economy/market layer is more mature than "v1 only lands money". That
   sentence remains in `src/economy/mod.rs` as an old module-level comment, but
   current merged code has CompleteSet, CPMM, router, YES/NO positions, and
   canonical generate-path market actions on ChainTape.

## Current Merged Baseline

- Local sync check: the inspected baseline is `origin/main@1f00012d`.
- Current main tip: PR #212, SWE-bench TDMA hidden-test judge and review fixes.
- `OBLIGATIONS.md`: OBL-001 through OBL-009 are all `satisfied`.
- Audit doctrine: platform-agnostic clean-context audit by one fresh capable
  agent; no vendor-specific single-Codex or dual Codex+Gemini rule remains.
- `turingos_dev`: retired in PR #213; `AGENTS.md §10` is an explicit retired
  placeholder so old section anchors still resolve.

## Verification Performed

On a clean detached worktree created from `origin/main`:

```text
bash scripts/run_constitution_gates.sh
# [k-1-5] total=164 failed=0

npm ci
# exit 0

npm run build
# exit 0

cargo test --features web --test generate_emits_work_tx_smoke -- --nocapture
# fails at compile time: src/web/dag_view.rs imports private TaskId through
# state::typed_tx; import should come from state::q_state.
```

The web-feature compile failure is a real current-state blocker for that
targeted smoke. It does not invalidate the constitution gate result, but it
should be fixed before claiming the web feature matrix is fully green.

## What The Article Still Gets Right

- TuringOS is strongest as a verified runtime for objective work: formal proof,
  patch repair, benchmark harnesses, replay, and audit trails.
- ChainTape/CAS/replay and Git substrate are the core truth machinery.
- Price and market traces are signals, not predicate truth.
- Public-money external markets should remain out of the near-term critical
  path; they require resolution, abuse controls, surveillance, compliance, and
  dispute machinery that are not software-only concerns.
- Proof Studio / Patch Arena remains a sound product wedge.

## Corrections To The Market/Economy Reading

The article underestimates the merged market substrate if it reads only
`src/economy/mod.rs`'s old "v1 lands money" comment.

Current merged evidence includes:

- CompleteSet mint/redeem/merge conservation gates.
- CPMM pool and directional swap gates.
- Atomic `BuyWithCoinRouter` gates with integer-only math.
- Generate-path real market sequence:
  `TaskOpen -> EscrowLock -> Work*3 -> MarketSeed -> CpmmPool ->
  BuyWithCoinRouter(YES) -> BuyWithCoinRouter(NO) -> Verify ->
  FinalizeReward -> EventResolve`.
- Web market view decodes router trades and records YES/NO counts.

The right claim is not "market is only a skeleton"; it is:

> TuringOS has an internal typed-transaction market substrate with replayed
> YES/NO trading and CPMM mechanics. It is not a public settlement market, and
> price remains observe-only for predicates.

## Corrections To The Judge/Benchmark Reading

The article's proof/patch direction is right, but SWE-bench is now more current:

- `src/judges/swebench_test_judge.rs` exists and is wired into
  `turingos tdma run --judge swebench`.
- The judge uses the real SWE-bench harness verifier path; gold/test patches do
  not enter the model prompt.
- The honest current experiment result is loop 0/3, bare 0/3. The TDMA loop and
  verifier are real, but no multi-step advantage has been demonstrated yet.

## Corrected One-Sentence Conclusion

TuringOS should still become the Git-native, judge-driven objective-work
runtime first; internal market mechanics are now real enough to be part of the
runtime substrate, but they should remain an internal signal/resource-allocation
layer until replay, product workflows, legal boundaries, and public settlement
rules are much stronger.
