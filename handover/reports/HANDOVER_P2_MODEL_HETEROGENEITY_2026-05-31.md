# Handover prompt — P2: real-model-heterogeneity market emergence

> Paste everything below the line into a fresh session as its opening prompt. It is self-contained.
> Repo: `/Users/zephryj/work/turingosv4` · branch `claude/lean-market-baselines` · 2026-05-31.

---

You are continuing a TuringOS v4 investigation. Read this entire prompt before acting. **Do not re-derive
what is already proven below — build on it.** Your standing goal (a `/goal` Stop-hook may be active):
**produce a REAL test demonstrating multi-agent intelligence emergence (a price-routed market beating a
single agent), without violating the three constitution flowcharts FC1/FC2/FC3.**

## 0. The standard you are held to (non-negotiable)
**守宪法 gates + 真题真跑** — real runs decide; review/analysis/static-reasoning is only a witness, never
the judge. Concretely:
- Every claimed proof MUST be independently re-run under the real Lean kernel AND `#print axioms`-checked
  to contain NO `sorryAx`. A binary's own "solved=true" is not enough — re-verify the emitted proof.
- Every capability claim MUST be an **aggregate over seeds**, never a single lucky run. (Twice in this
  investigation a single run showed fake emergence, e.g. market 3/5 > single 0/5, that the aggregate
  revealed as noise. A fabricated "71%" number was once written before data landed and had to be
  discarded — NEVER write a result number you have not measured. If you catch yourself about to state a
  number from expectation rather than a frozen data file, stop.)
- PR-only. Never merge to main. Never edit FC1/FC2/FC3 or §6 restricted surfaces (`src/state/sequencer.rs`,
  `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, `src/kernel.rs`, `src/bus.rs`,
  `src/sdk/tools/wallet.rs`). Integer-only money paths. Read `CLAUDE.md` + `AGENTS.md` + `OBLIGATIONS.md`.

## 1. What is already PROVEN (do not redo — read `handover/reports/MARKET_EMERGENCE_FULL_REPORT_2026-05-31.md`)
1. **Constitutional defect found + fixed.** The shipped `boltzmann_select_parent_v2` (`src/sdk/actor.rs`)
   was argmax-by-price = pure exploitation → the work-DAG collapsed to one chain → multi-agent ≡
   single-agent (the `过度利用→集体平庸` failure **Art. II.2.1** forbids). The fix
   `boltzmann_softmax_select_parent` (same file, FC1-N5 backlink) samples node ∝ `exp(price/T)`, giving
   true price-routed attention distribution incl. non-local backtracking. **Additive** — old fn retained
   for g0/g1 replay.
2. **Honest NEGATIVE (homogeneous pool):** on single-deep-sequence theorems with ONE model, the market
   does NOT beat a single agent — tree-search market **0/24** vs single **2/24** (depth problem; breadth
   wastes budget). This bound is real; preserve it.
3. **Emergence APPEARS with heterogeneity (`src/bin/lean_hetero_market.rs`):** a pool of genuinely-limited
   specialists (harness-locked to one tactic family each, self-select via SKIP) routed by the market
   beats any single agent. `het4` deterministic across all 8 seeds: **market 3/4 > single 2/4 >
   single_spec 1/4**; 16-seed aggregate **market 3.81 > single 3.00 > single_spec 1.50** avg
   Lean-verified sub-goals. The whole exceeds the best part; a lone specialist is capped at 1/4 by
   construction → real complementary combination.
4. This is all on PR **#221** (CI all-green: constitution gate suite, r022, liveness, no-sidecar). 19
   commits. Lead commits: `03336c20` (softmax) → `8f011310` (tree) → `40b5c20a` (negative) → `bbaf4629`
   (hetero emergence) → `e7fec921` (full report).

## 2. The KNOWN LIMIT you must remove = YOUR TASK (P2)
The §1.3 emergence uses **artificial** heterogeneity: agents differ by a harness-enforced *tactic-family
lock*, all calling the SAME model (`deepseek-chat`). A skeptic will say "you manufactured the
specialists." The strongest, least-contestable form of the claim — and the architect's **Thesis-B** ("a
cheap diverse swarm beats one strong expensive model") — is:

> **At equal total token budget, does a market of genuinely-different MODELS beat the single strongest
> model alone, on hard Lean theorems, with every solve Lean-kernel-verified?**

Concretely:
- Replace tactic-family locks with **real model heterogeneity**: agents drawn from
  `{deepseek-chat, deepseek-reasoner}` (both route through the local proxy on `:8123`;
  `deepseek-reasoner` is the stronger/slower CoT model). FIRST verify reasoner actually responds:
  `curl -s http://127.0.0.1:8123/v1/chat/completions -d '{"model":"deepseek-reasoner","messages":[{"role":"user","content":"reply OK"}],"max_tokens":20}'` (unset `ALL_PROXY/https_proxy` first; the
  proxy is the provider-abstraction layer — do NOT modify it or the Rust client).
- **Arms (equal token budget — this is load-bearing):**
  - `market` = mixed pool {chat, reasoner, chat, reasoner} routed by the softmax price market.
  - `single_strong` = `deepseek-reasoner` alone (the honest hardest baseline — the strongest single agent).
  - `single_cheap` = `deepseek-chat` alone.
  - (optional) `homogeneous_market` = 4× chat market (isolates "diverse models" from "more agents").
- **GUARD against the trivial confound:** the market must NOT win merely by spending more tokens.
  `deepseek-reasoner` emits far more tokens/call (CoT). Count ACTUAL tokens (the manifest already records
  `tokens`; the proxy `/stats` endpoint gives ground truth) and **equalize on total tokens, not call
  count** — give `single_strong` enough calls to match the market's total token spend. If the market only
  wins because reasoner-in-the-market spent more, that is a NO-GO, report it honestly.
- **Decision:** market full-solve-rate (or avg sub-goals at equal tokens) significantly > single_strong →
  Thesis-B emergence confirmed (the strongest result available). market ≈ single_strong at equal tokens →
  honest negative; the §1.3 specialist emergence still stands as the headline, report both.

## 3. How to run (operational — verified working today)
- **Proxy:** runs detached (nohup) on `:8123`, routes `deepseek-*` → `api.deepseek.com` directly (China,
  no VPN). Check `curl -s http://127.0.0.1:8123/health`. If down, restart with
  `bash scripts/start_proxy.sh` (it sources keys from `/Users/zephryj/work/turingosv4-probe-gpqa/.env` —
  never print or commit keys). The `:8123` proxy IS the constitutional provider-abstraction layer; keep
  the Rust client pointed at it, unchanged.
- **Mathlib:** built at `/Users/zephryj/work/mathlib4` (Lean v4.24.0, ~7000 oleans). Pass
  `--mathlib-dir /Users/zephryj/work/mathlib4`. LEAN_PATH resolves via `lake env`.
- **Hetero binary (your starting point):** `src/bin/lean_hetero_market.rs`. Build:
  `cargo build --release --bin lean_hetero_market`. Run:
  `./target/release/lean_hetero_market --task het4 --policy market --n-rounds N --seed S --model deepseek-chat --mathlib-dir /Users/zephryj/work/mathlib4 --out /tmp/x.json`.
  It currently takes ONE `--model`; your job adds per-agent model assignment (a small Class-1/2 change to
  the agent-construction loop near line 167-185 + a `--models` flag or a policy that interleaves models).
  Tactic-family lock is enforced at `lean_hetero_market.rs:225` (`if let Some(f) = fam`); for P2 you
  REMOVE/relax the family lock and instead vary the model per agent.
- **Independent verification pattern (reuse this):** after a run, read `omega_proof`/proof from the
  manifest, write it to a `.lean` file with `#print axioms tm` appended, run
  `LEAN_PATH=$(lake env printenv LEAN_PATH) <lean-bin> file.lean`, accept only exit 0 with no `sorryAx`.
  See `/tmp/vcell.sh` pattern in the prior session (recreate it — it parallelizes cells with `xargs -P 3`
  and re-verifies each).

## 4. Pitfalls that cost me time (avoid them)
- **Background sweeps started with bare `&` get KILLED when the tool turn returns.** Use the tracked
  background mechanism (run_in_background) OR an explicit `until`-loop that waits for a `DONE` marker, so
  the job survives the turn. Several sweeps only ran half their cells because of this.
- **The auto-classifier flaps ("temporarily unavailable" / "Stage 2 error").** It is transient — simply
  retry the exact command. Don't redesign around it. Read-only ops never need it.
- **deepseek returns whole multi-line proofs as one "tactic"** and ignores prompt instructions to be
  atomic. Do NOT try to force atomicity by rejecting compound tactics — I tried, it broke solving (0/6),
  reverted in `86b74f83`. Work WITH compound tactics.
- **VPN on `:1080` for git push is intermittent.** Push with
  `git -c http.proxy=socks5://127.0.0.1:1080 push origin claude/lean-market-baselines`; if it fails,
  commits are safe locally, retry later. (DeepSeek API does NOT need the VPN; git push does.)
- **CI gates that bit me twice (pre-check locally before every PR):** (a) `r022_check` — every new library
  `pub` symbol needs a `/// TRACE_MATRIX <FC-id>: <role>` backlink; (b) liveness — every new `src/bin/*.rs`
  must be added to `tests/fixtures/liveness/production_module_liveness.toml`. Run
  `python3 scripts/check_trace_matrix.py --mode ci --base-ref origin/main` and
  `cargo test --test constitution_production_module_liveness` + `bash scripts/run_constitution_gates.sh`
  (expect `[k-1-5] total=164 failed=0`) BEFORE opening a PR.

## 5. After P2 (the longer plan — for context, not now)
P1 push emergence to full-solve (not just graded sub-goals) · **P4** price-attribution ablation
(shuffled-price arm: if the market still wins with the price signal destroyed, the win is parallelism not
PRICE — run this before the expensive P3) · **P3** promote the proof-state tree node model into the real
ChainTape/CPMM/replay market so emergent runs are `verify_chaintape`-green (turns "a real test showed
emergence" into "a constitutional run proves emergence"). Recommended order overall: **P2 → P1 → P4 → P3**.

## 6. First actions for you
1. Read `handover/reports/MARKET_EMERGENCE_FULL_REPORT_2026-05-31.md` (code + data + analysis in one file)
   and `src/bin/lean_hetero_market.rs`.
2. Verify proxy health + that `deepseek-reasoner` responds.
3. Check PR #221 status (`gh pr checks 221 --repo gretjia/turingosv4`); if merged, rebranch from main.
4. Implement per-agent model assignment in the hetero market (Class 1-2, additive). Then run the equal-token
   market vs single_strong test, independently re-verifying every solve, aggregating over ≥8 seeds.
5. Write a frozen-data report + open a PR (gates pre-checked). State the honest result either way.
