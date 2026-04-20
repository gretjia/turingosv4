# Tape Economy v1 — Design

**Author**: Claude (auto-research, flight window 2026-04-20)
**Status**: DESIGN — not yet implemented. Will be built on branch `feat/tape-economy-v1`.

## Problem

Observed in the 43/50 dual-path run: `tool_dist` aggregate `append: 1` across
all 43 solved problems. The tape-verification fallback (F-2026-04-19-08) makes
Q_t *reachable* at ∏p, but there is no economic force pulling agents to
actually fill Q_t. Law 2 says markets drive behavior — right now no market
exists on "did you use tape?", so agents rationally skip it.

Persistent failures (`mathd_algebra_293/332`, `induction_sumkexp3eqsumksq`,
plus the occasional timeout on hard problems) all look like cases where
decomposition would help but agents never attempt it.

## Hypothesis

If `complete` on an empty tape costs Coins, agents will choose between:
(a) accept the fee and one-shot (keeps current-best behavior on easy problems)
(b) `append` a few partial tactics first (tape non-empty → complete is free)

This is a *mechanism* (C-034), not a rule: agents remain free to one-shot
easy problems; economics only pushes them toward tape on the margin.

## Design

### Rule
- `complete` attempt with `tape.time_arrow().len() == 0` debits the caller
  `COMPLETE_COLD_FEE` (default 500 Coins) at the point of attempt.
- `complete` attempt with tape non-empty is free (the tape itself is evidence
  of Q_t contribution).
- Debited Coins flow to a per-problem `bounty_pool` field on the kernel.
- At `halt_and_settle`, if a golden path exists, the bounty_pool is split
  among authors of nodes on the golden path, proportional to their node count.
  If no golden path, the pool is distributed back to the original fee-payers
  (no minting, Law 2 conservation).

### Invariants
- Conservation: all debits have matching credits (fee pool → GP authors OR
  refunded). Never mint.
- Non-negative balances: deduct only if wallet has ≥ fee. If not, skip the
  cold fee (can't make the problem unsolvable for broke agents).
- Reentrancy: pool state is mutated only inside bus.complete / halt_and_settle.

### Constitutional Alignment
- Law 2 (markets drive Coin flow): the cold fee is itself a market signal —
  "the system charges for bare complete because it believes tape use is
  valuable." Matches Art. II.2.
- Art. IV (Q_t → ∏p): reinforces F-2026-04-19-07 fix — Q_t is not just
  available to ∏p, it is *incentivized* at the economic layer.
- C-034 (mechanism > prompt): the prompt does not say "please use tape";
  the economics make it rational.

### Touched Files
- `src/kernel.rs`: add `bounty_pool: HashMap<ProblemId, f64>` (or per-run scalar
  since each evaluator run is one problem). Add `record_cold_fee`,
  `distribute_bounty`.
- `src/bus.rs`: `complete` surfaces a hook; evaluator calls into it before
  verification.
- `src/sdk/tools/wallet.rs`: expose `deduct_if_sufficient(agent, amount) ->
  Option<f64>` returning the amount actually taken (0 if insufficient).
- `experiments/minif2f_v4/src/bin/evaluator.rs`: invoke cold-fee gate on
  complete; call `distribute_bounty` on halt_and_settle.
- Env: `COMPLETE_COLD_FEE` (default 500), `TAPE_ECONOMY_V1=1` toggle so we can
  run A/B with main.

### A/B Plan
- Branch: `feat/tape-economy-v1`.
- Worktree: `../v4-tape-economy/` (separate `target/`, no cross-contamination
  with the variance run on main).
- N=20 sample (`sample_N20_S74677.txt`) — shorter cycle than N=50 to fit the
  5h window.
- Both arms: same binary from the worktree, toggled by `TAPE_ECONOMY_V1` env.
- Primary metric: solve rate. Success floor: not regressing below 14/20
  (main's typical N=20 pace).
- Secondary: `append`, `invest`, `complete_via_tape` in aggregate tool_dist.

### Failure Modes We Expect / Accept
- If the cold fee is too high, easy problems that *should* one-shot get
  priced out → solve rate drops. Mitigation: `COMPLETE_COLD_FEE` is tunable;
  start at 500, try 100 if 500 tanks.
- If agents just tape-garbage to dodge the fee, `tape_chain + payload` will
  fail verification and they'll eventually hit the search cap. We'll see
  unique_payload_ratio drop — C-036 catches it.
- If the mechanism works but solve rate only matches main, that's still a
  constitutional win (Q_t is now load-bearing in economics). Ship anyway.

## Open Questions (for user review when they return)

1. Is `COMPLETE_COLD_FEE = 500` the right order of magnitude? 10000 starting
   balance means one cold fee is ~5% of capital; ten is 50%. Want pressure
   but not bankruptcy.
2. Bounty distribution: proportional to nodes on GP, or flat per author?
   Proportional rewards graph-shapers; flat rewards participation.
3. Should there be a secondary cold fee on `complete_via_tape` too (smaller),
   or is the current free pass fine?
4. This is an economic change on restricted files (`wallet.rs`, `bus.rs`,
   `kernel.rs`). User authorized kernel edits via branch — merge decision
   still needs user sign-off before main.

## Next Action
After variance run completes and its findings are committed to main, set up
the worktree and implement this design. Report results in LATEST.md.
