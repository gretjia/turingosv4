# Market-emergence investigation — constitutional root-cause + tactic-tree redesign

> 2026-05-31 · branch `claude/lean-market-baselines` · architect mandate: study the constitution,
> find where the code diverges from it, make any constitution-aligned change (kernel-level OK; the
> 3 flowcharts FC1/FC2/FC3 are the only red line), until the market shows the expected emergence.

## The question
Earlier H0/C runs found market ≈/< single-agent (NO-GO). The architect's challenge: that is an
artifact of NOT implementing the constitution's real loop — the market's value is **price-routed
NON-LOCAL tree search**: agents see ALL node prices and may jump back to ANY (incl earliest) node
to start a NEW branch (MCTS-style backtracking), not just extend the latest. If that is missing,
multi-agent collapses to one chain ≈ single agent.

## Finding 1 — CONFIRMED constitutional violation: argmax routing (Art. II.2.1)
`src/sdk/actor.rs::boltzmann_select_parent_v2` is **argmax-by-price (+ epsilon-uniform)**, NOT a
Boltzmann distribution. That is pure EXPLOITATION → every agent piles onto the single highest-price
node → the work-DAG degenerates to one chain. Constitution **Art. II.2.1** explicitly forbids this:
"如果中层黑盒对最高分信号过度敏感(过度利用),所有中层黑盒会迅速收敛到同一个局部最优,导致群体失去
多样性,甚至陷入集体平庸." Art. II.2 says price must guide the "注意力**分布**" (a distribution,
not an argmax point). The architect's intuition was correct.

**Fix (committed `03336c20`, pushed):** added `boltzmann_softmax_select_parent` (true softmax,
samples node ∝ exp(price/T); temperature = explore/exploit knob; f64 in the POLICY only, not a
money path — the chosen parent is on tape so replay reconstructs from L4). Wired into the
market/random_bear/fixed_bear/shuffled arms (`--boltzmann-temp`, default 0.15). Verified it now
produces a BRANCHING tree (lean_market_agent lm_mono: 5 nodes, 4 parents) vs the old argmax chain.

## Finding 2 — softmax NECESSARY but NOT SUFFICIENT (on the full-attempt model)
H1 (3 headroom theorems × 4 seeds, 16-attempt budget): market 4/12 ≈ shuffled 5/12 ≈ single 5/12.
H2 (× 6 seeds, 54 cells): market 14/36 ≈ single 15/36 ≈ shuffled 13/36. Still market ≈ single.

**Root cause (Finding 2):** in `lean_market_agent` a NODE = a FULL proof attempt; "expanding" a node
= refining that whole attempt with Lean feedback. So branching across full-attempts ≈ a single
agent's sequential retries, and these monolithic proofs are solved fast-or-never (no partial
gradient to route on). The architect's "go back to an early node and start a new branch" needs nodes
to be **partial proof STATES**, not full attempts — constitution §B: "Node = git commit / branch =
git branch / Boltzmann routing = git branch."

## Fix 2 — tactic-level proof-TREE market: `src/bin/lean_tree_market.rs` (NEW, this commit)
Models the proof as a real tree of PARTIAL Lean tactic-states:
- node = (tactic script so far, parsed remaining Lean goals);
- expand = LLM proposes the NEXT tactic given the remaining goals → Lean → child state;
- branch = different next-tactics from the SAME state; OMEGA = a script that closes all goals;
- **market** = Boltzmann-softmax over node VALUE (goals-closed progress − stuck-penalty) → routes
  attention across promising partial states incl early ones (backtrack / new branch);
- **single** = one DFS chain (extend own deepest node; retry same node on failure, no branch jump).
Diagnostic-grade (no chain/CAS/replay; allows decide/native_decide — sound, irrelevant to the
routing question; the no-native_decide rule stays in LeanJudge for the real OMEGA market).
`eval_proof` classification (Omega / Partial-"unsolved goals" / Invalid) validated against real
Lean output; multi-line tactics indented correctly (deepseek returns `induction … with …` blocks).

**Status: built + WORKS** — solved `tm_sumodd` (but in 1 shot → that theorem is too easy to need a
tree). Liveness-registered.

## NEXT STEP (the actual emergence test) — hand-off
Run `lean_tree_market` market vs single on the HARD multi-step theorems where 1-shot fails so tree
search + backtracking matters: `tm_cube4` (∑i³·4 = (n(n+1))², induction+ring), `tm_mono` (StrictMono
x³+x), `tm_sqrt2` (Irrational √2). Budget-match (market N×R vs single 1×(N·R)), seeds, e.g.:
```
./target/release/lean_tree_market --theorem tm_cube4 --policy market --n-agents 4 --n-rounds 8 \
   --mathlib-dir /Users/zephryj/work/mathlib4 --seed S --out ...
./target/release/lean_tree_market --theorem tm_cube4 --policy single --n-agents 1 --n-rounds 32 ...
```
Metric: solved-rate + best_goals_closed (partial credit) at equal budget. If market > single here →
the architect's vision is validated (price-routed tactic-tree search with backtracking beats a single
DFS chain even with homogeneous agents) → port to the full constitutional market (ChainTape/CPMM/
replay). If still ≈ → next gaps: (a) curate `tm_*` theorems with genuine dead-ends + multiple viable
tactic paths (tm_sumodd was 1-shot); (b) value backprop (a node's price should reflect its SUBTREE
potential, MCTS-style, not just local progress); (c) heterogeneous agents (different models).

## Open infra notes
- DeepSeek proxy `:8123` runs detached (nohup) and is up. The 1080 VPN (git push) is intermittent —
  `git -c http.proxy=socks5://127.0.0.1:1080 push` when it's reachable.
- deepseek proposes imperfect single tactics (e.g. malformed `induction n with k ih`) — the tree
  tolerates this (bad tactic → Invalid → stuck-penalty → routing moves on), which is the point.
