# TuringOS Market-Economy Mission — Final Executable Task-Package

**Status:** DECOMPOSITION + ACCEPTANCE-CRITERIA ONLY. No counted run authorized by this document.
**Repo:** `/Users/zephryj/work/turingosv4` · **Branch:** `claude/emerge-stage2` @ `103b239347` · **Diff base:** `git merge-base HEAD origin/main` = `a663ed4d` (NOT literal `main`=`1f00012d` nor `origin/main`=`a68ba081` — using literal main drowns scope-guards in unrelated emerge-stage2 deltas).
**Date:** 2026-06-01

---

## 1. Mission — the ONE compressed verifiable proposition

> **Under SAME models, SAME budget B, SAME verifier (Lean `#print-axioms` ⊆ {propext, Classical.choice, Quot.sound}), SAME task set: does TuringOS's price + ledger + predicate allocation convert compute into verification-PASSING paths at strictly greater efficiency (banked@B primary; cost_of_pass_micro_usd / PPUT secondary) than a coordinator-swarm — AND does that advantage survive price-destruction ablation (RealPrice > ShuffledPrice AND RealPrice ≠ FlatBid), proving the gain is *price coordination*, not parallelism/harness — with every headline arm replayable from a frozen tape that a standalone verifier reconstructs to integer/byte equality with the run manifest?**

It is ONE proposition with three conjoined clauses that must all hold for a price claim: (a) market ≥ coordinator-swarm on PPUT; (b) market > shuffle AND market ≠ flatbid (the causal firewall, `lean_hayek_market.rs:916,975,988` — flatbid = constant bids = "THE causal firewall"); (c) the arm is tape-replayable or it is excluded. NOT "more agents," NOT "smarter than a single model" — a new agent-organization *institution*.

---

## 2. Defensible claim framing — what is and is NOT claimed

**REJECTED:** "world-first agent market economy." Falsified by prior art: Contract Net Protocol (Smith 1980, IEEE Trans. Computers C29:1104), market-based multi-robot task allocation (Zlot/MURDOCH), COALESCE (arXiv 2506.01900), Shapley-Coop (arXiv 2506.07388). For the Art.V branch, also rejected: "world-first self-evolving agent" — prior art Meta-Harness (arXiv 2603.28052), AHE (arXiv 2604.25850), Darwin-Gödel.

**DEFENSIBLE (OS-level combination):** TuringOS is **a verifiable, replayable, price-coordinated OS for LLM agent economies** — the *unified combination* of: LLM-agent task decomposition + compute allocation + on-tape failure ledger + price signals + verification predicates + replayable hash-chained tape + constitutionally-vetoed ArchitectAI self-modification. The novelty is not any single mechanism; it is the institutional substrate that makes price-routed allocation **auditable, Sybil-resistant, and Goodhart-shielded** end to end.

**Prior-art boundary (explicit):** Coordinator-swarms (Kimi K2.6 300-subagent shared-state, Grok Build worktrees, Anthropic orchestrator-worker) differentiate by shared-state registry + conflict resolution. None operationalize (i) failure-branch ledger on a canonical tape, (ii) a loss-bearing asset market with a settlement oracle, (iii) Goodhart-shielded price broadcast, (iv) replayable price-routing as a *causal* allocator. TuringOS's line is exactly there. A TIE vs coordinator-swarm on solve-rate is still a win **iff** it buys accountability (replay + Sybil-resistance + shielding) the baseline lacks.

**Honest regime caveat (binding, from the repo's own evidence):** price is a near-NULL in single-shot *aggregation* of correlated judgments (`MULTI_AGENT_ECONOMY_FINAL_VERDICT`, 5× null; `PRICE_ECONOMY_DEFINITIVE_VERDICT`; H0 GoNoGo `H0_HARD_LEAN_MARKET_GONOGO_RESULT_2026-05-30.md:16-23` — single **7/12** ≥ shuffled **6/12** > market **5/12**, i.e. the signal-destroyed shuffle BEAT the real market) and is causal in *adversary-robust sequential allocation* (`PRICE_ECONOMY_VALIDATED_REPUTATION`, Sybil-defunding, 10/10 seeds). The mission must locate every price claim in the regime where the repo's own data says it can win.

---

## 3. Dependency DAG, lines & resource split

```
                          ┌─────────────────────────────────────────┐
   PREREQUISITE (gates all counted runs)                            │
   ┌──────────────────────────────────────┐                         │
   │ TP-0  Tape-Canonical Semantic-Ver-A   │  ◄── HARD GATE          │
   │       (MarketTape-lite → replayable)  │      a market arm that  │
   │       schema-aware verify_market_tape │      cannot replay from │
   └───────────────┬──────────────────────┘      frozen tape is     │
                   │ BLOCKS every counted T2/T3/Art.V cell           │
                   ▼                                                 │
   MAIN LINE (70%) ─────────────────────────────────────────────────┘
   ┌──────────────────────────┐        ┌──────────────────────────────┐
   │ TP-1  Microstructure Spec│───────►│ TP-2  Coordinator-Swarm arm  │
   │  (5-pillar charter +     │ feeds  │  + full PPUT arm-set + the   │
   │   conformance predicates)│ prereg │  counted T2 causal sweep     │
   └──────────┬───────────────┘        └──────────────────────────────┘
              │                              ▲ also gated on EMERGE Stage-2 (#28)
              │ TP-1 prereg consumed by branches
              ▼
   BRANCHES
   ┌───────────────────────────┐    ┌──────────────────────────────────┐
   │ TP-3 (20%) Art.V meta-mkt │    │ TP-4 (10%) Heterogeneous N_eff   │
   │  ArchitectAI as priced    │    │  GATED on Stage-2 Q1; demotes to │
   │  participant; depends TP-1 │    │  reputation+cost if JUST_SAMPLING│
   │  reuses TP-2 settle/PPUT   │    │  depends TP-1, TP-2, TP-3        │
   └───────────────────────────┘    └──────────────────────────────────┘
```

**Resource split (architect-binding): 70% main line (TP-0 prerequisite + TP-1 + TP-2) · 20% TP-3 (Art.V) · 10% TP-4 (T3).**

**Blocks-what (hard edges):**
- **TP-0 → everything.** No counted T2/T3/Art.V cell runs until TP-0 lands AND its prereg is locked. Exclusion rule: any arm whose `verify_market_tape` exits ≠0 is excluded from headlines.
- **TP-1 → TP-2, TP-3, TP-4.** The 5-pillar conformance contract is the eligibility gate; an arm that fails any pillar predicate is headline-ineligible.
- **TP-2 → TP-4.** Arm B (coordinator-swarm) is *consumed* by TP-4, never built there (re-confirmed absent: grep `coordinator|orchestrat|decompo|swarm` over `src/bin/lean_market_agent.rs` = 0 hits; Policy enum at `lean_market_agent.rs:80-105` has no Coordinator).
- **EMERGE Stage-2 (#28) → TP-2, TP-4.** Stage-2 is `in_progress`; the live partial trend points at `Q1=JUST_SAMPLING` (v4-pro@k8 alone recovers 5 of 8 "combination-target" theorems by extra draws), which demotes TP-4.

---

## 4. Task Packages

### TP-0 — Tape-Canonical Semantic-Version-A: make PPUT replay-auditable (prerequisite, risk 1–2)

**Objective.** Promote the existing in-bin `MarketTape` (`src/bin/lean_hayek_market.rs:107-170`) to a shared, replay-reconstructable module and close exactly the audit gaps that block PPUT auditability **for ONE pinned reference arm**, without attempting the real-git Q_t/HEAD_t rebuild (Art. 0.4 path B is Phase-E-only, 6–8wk, out of scope). A `head_commit_sha` string in GenesisPin is the MINIMAL provenance surrogate.

**Verifier-mandated correction (blocker, now folded in): the bin has FOUR run_* functions with FOUR schemas, not one substrate.** Confirmed in code:
| fn | task trigger | schema | wallets? | micro_usd? |
|---|---|---|---|---|
| `run_probe_alloc` (`:919`) | `het*` (dispatch `:1378-1379`) | `lean_probe_alloc.v1` (`:1049`) | YES (`realized_pnl` `:1017/1020`) | **NO** |
| `run_alloc` (`:1076`) | `pool:` (dispatch `:1382-1383`) | `lean_hayek_alloc.v2` (`:1220`) | **NO** (pool-banking loop) | YES (cost_of_pass headline, `finish_alloc:1210-1233`) |
| `run_compete` (`:1241`) | `cmp_*` | `lean_hayek_compete.v1` (`:1338`) | no | tokens only |
| fall-through `run` | inline het | `lean_hayek_market.v1` (`:1486`) | YES | — |

**Resolution: PIN the reference arm to `run_alloc` (`pool:` task, schema `lean_hayek_alloc.v2`)** — the one that computes the `cost_of_pass` headline. On that arm `derive_wallets` is OUT OF SCOPE (no per-agent wallets); `derive_cost` reconstructs micro_usd from `LlmCall`+`MODEL_RATES`. A SECOND, schema-aware wallet check runs on a `run_probe_alloc`/het4 arm where `derive_pnl` applies but `cost_match` is dropped (het4 manifest has no micro_usd). The verifier MUST read `manifest['schema']` and apply only the `derive_*` that schema exposes.

**Files.**
- `src/bin/lean_hayek_market.rs:107-170` (MarketTape/MarketEvent — substrate to extract), `:85` (`call_micro_usd`), `:69-83` (`MODEL_RATES` — move into shared module so verifier links identical rates), `:101` (`stake_from_confidence`), `:1017/1020` (probe settle formula for the wallet arm), `:1210-1233` (`finish_alloc` — cost headline for the pinned arm), `:29-32` (now-false header comment "lib.rs untouched … all helpers inline" — MUST update if lib.rs is touched).
- `src/market_tape.rs` (NEW shared module: MarketTape + GenesisPin + Settle + `derive_pnl`/`derive_closed`/`derive_cost` + `verify_chain` returning `Result`).
- `src/lib.rs` (`pub mod market_tape;` — **trust-root/constitution touch per user-memory "adding a mod = constitution touch"; Class-2-bordering-Class-4, needs per-atom §8 OR the bin-local `#[path]` alternative**).
- `src/bin/verify_market_tape.rs` (NEW standalone replay verifier — NOT a pure "mirror" of `verify_chaintape.rs`: that 110-line wrapper self-derives indicators and exits 0/1/2 with NO `--manifest` byte-compare; `verify_market_tape` adds a manifest-equality contract, a *stricter* dimension).
- `tests/market_tape_canonical_roundtrip.rs` (NEW conformance gate — one fixture PER schema).
- `handover/preregistration/T2_TAPE_CANONICAL_PREREG_TP0.json` (NEW — locks prereg before any counted run).

**Subtasks + risk.**
1. **(TP-0.1, risk 2)** Extract MarketTape/MarketEvent → `src/market_tape.rs`, re-point bin via `use`. Behavior-preserving move so existing P4-lite/LEAN-ALLOC/BENCH tapes still verify-green. **Corrected framing:** `verify_chain` ALREADY aborts — confirmed `:149 Err(_) => return false`, `:150 return false` on broken prev-hash. The real (smaller) change is `bool → Result<(), VerifyErr>` so the verifier reports WHICH line index broke; do NOT claim it closes a silent-skip violation (the silent-skip is in `src/wal.rs:88-92`, which is out of scope here).
2. **(TP-0.2, risk 1)** GenesisPin as mandatory first record: `{run_id, seed, model_roster, budget_B, verify_budget, axiom_whitelist:[propext,Classical.choice,Quot.sound], policy/arm, head_commit_sha (40-hex git HEAD at run start), prereg_path}`. `append()` Err/panics if first event ≠ GenesisPin.
3. **(TP-0.3, risk 1)** `derive_cost(tape)` recomputes total micro_usd + tokens from `LlmCall` events alone via the shared `MODEL_RATES`; integer-only (`AGENTS.md §12`).
4. **(TP-0.4, risk 1)** Settle event for the wallet arm. **Anti-pattern guard:** `derive_pnl` must RECOMPUTE from GenesisPin budget + Invest debits + Resolve outcome + the settlement RULE (formula at `:1017/1020`), NOT read `Settle.pnl_delta`; the conformance test cross-checks recomputed == manifest AND == `Settle.pnl_delta`.
5. **(TP-0.5, risk 1)** `FailedProposal` event `{agent, claim, reject_class, output_hash}` so parse-fail/veto proposals that never reached the verifier become `verified=false` tape nodes (constitution.md:71 item-5 analog). Do NOT touch `bus.rs:66` graveyard (legacy path, unused by T2).
6. **(TP-0.6, risk 2)** Build `verify_market_tape.rs` — schema-aware, read-only, emits `replay_report.json`, exit 0/1.
7. **(TP-0.7, risk 2)** Conformance gate `market_tape_canonical_roundtrip.rs`, one fixture per schema; must be able to FAIL.
8. **(TP-0.8, risk 2)** Author + lock the prereg JSON; run the pinned `pool:` arm end-to-end through the upgraded module + `verify_market_tape`; re-run `cargo test --workspace --no-fail-fast` + `bash scripts/run_constitution_gates.sh`; clean-context audit handoff.

**ACCEPTANCE (verifier-revised; `[machine]` = machine-checkable):**
- **[machine]** Reference arm PINNED to `run_alloc`/`pool:`/`lean_hayek_alloc.v2`. Schema-aware verifier reconstructs banked, micro_usd, tokens from frozen tape alone, byte/integer-equal to manifest:
  `cargo run --bin verify_market_tape -- --tape <run>.tape --manifest <run>.json --out /tmp/rr.json; test $? -eq 0 && jq -e '.schema=="lean_hayek_alloc.v2" and .banked_match and .cost_match and .tokens_match' /tmp/rr.json`
- **[machine]** `derive_cost` recomputed (not read-back), integer-only:
  `jq -e '.cost_match and .tokens_match' /tmp/rr.json && awk '/fn derive_(cost|pnl|wallets)/{f=1} f&&/^}/{f=0} f' src/market_tape.rs | grep -n 'f64\|f32' | wc -l | grep -qx 0` (per_rk f64 at `finish_alloc:1218` and routing softmax are NOT in derive_* and are exempt).
- **[machine]** GenesisPin mandatory-first + negative test:
  `head -1 <run>.tape | jq -e '.kind=="GenesisPin" and (.body.model_roster|length>0) and (.body.budget_B!=null) and (.body.axiom_whitelist|index("Classical.choice")) and (.body.head_commit_sha|length==40)'` AND `cargo test --test market_tape_canonical_roundtrip genesis_pin_must_be_first -- --exact`.
- **[machine]** Failure branches on the FIXTURE (not a live run — live het4 is reliably-solvable so a live criterion is vacuously skipped): `cargo test --test market_tape_canonical_roundtrip failed_branches_appear_on_tape -- --exact` asserts `derive_failures(fixture).len()==EXPECTED` (hardcoded integer) and a `FailedProposal{verified:false}` node exists for the non-verifier-reached case.
- **[machine]** Wallet arm (`run_probe_alloc`/het4, schema `lean_probe_alloc.v1`): `derive_pnl` recompute equals `manifest.realized_pnl_micro`; `cost_match` ABSENT-by-design (het4 has no micro_usd):
  `cargo run --bin verify_market_tape -- --tape <het4>.tape --manifest <het4>.json --out /tmp/rrw.json; jq -e '.schema=="lean_probe_alloc.v1" and .pnl_match' /tmp/rrw.json`
- **[machine]** Negative control (suite can FAIL): `cargo test --test market_tape_canonical_roundtrip wal_hash_chain_uninterrupted -- --exact`; documented manual one-byte tamper → non-zero exit → revert.
- **[machine]** Prereg FORWARD-VALID for the confound (reserved, not tested in TP-0):
  `jq -e '.confound_reservation.shuffle_must_roundtrip==true and .headline_statistic=="delta_realprice_minus_shuffledprice_paired" and .binding_scarcity=="verify_budget_lt_k" and (.exclusion_rule|test("!=0"))' handover/preregistration/T2_TAPE_CANONICAL_PREREG_TP0.json`
- **[machine]** Scope guard against the ACTUAL branch base:
  `B=$(git merge-base HEAD origin/main); git diff $B --name-only | grep -E 'src/(kernel|bus|state/sequencer|state/typed_tx|sdk/tools/wallet)\.rs|src/bottom_white/cas/schema\.rs' | wc -l | grep -qx 0`; lib.rs delta is only `pub mod market_tape;`; no `git2|libgit2|Repository::` anywhere in the diff; header comment `:29-32` updated.
- **[judged]** lib.rs edit carries a per-atom §8 ratification reference (`handover/directives/<date>_TP0_lib_mod_s8.md`) OR the diff shows `src/lib.rs` unchanged and market_tape is shared bin-local via `#[path]`. Clean-context auditor confirms which.
- **[machine]** No regression across all schemas the module feeds (one fixture per schema): `cargo test --workspace --no-fail-fast` exit 0; `scripts/run_constitution_gates.sh` exit 0; real pinned-arm run → `jq -e '.tape_chain_ok==true'`.

**Prereg fields (locked before any counted run):** arm_set (MARKET / NO-PRICE-MULTI / SHUFFLED-PRICE / BEST-FIRST / COORDINATOR-SWARM-once-it-exists); model_roster pinned to `MODEL_RATES` rows (`:69-77`), SAME across arms; budget_B (binding unit + value, SAME per arm); verify_budget (< k — the regime P4-lite proved required); seeds (≥12); axiom_whitelist {propext,Classical.choice,Quot.sound}; head_commit_sha; manifest_fields_under_audit (closed, tokens, micro_usd, realized_pnl_micro[], failures count); **exclusion_rule (verify_market_tape exit≠0 ⇒ excluded — pre-committed so a failed replay cannot be retconned)**; wal_mandatory_decision (T2 counted runs: `--tape-out` MANDATORY, scoped to this bin only).

**Traps avoided:** no real-git rebuild (uses head_commit_sha surrogate); zero capability/novelty claim (pure plumbing); silent on price efficacy (P4-lite already showed het4 negative — fix is *prerequisite, not evidence*); correct substrate (MarketTape-lite, not the stale `bus.rs/kernel.rs` path excised per `kernel.rs:10-12`); §6 surfaces untouched; integer-money invariant held; exclusion rule pre-registered; historical tapes re-verified never rewritten.

---

### TP-1 — Verifiable Market Microstructure Spec (institutional core, risk 1–2)

**Objective.** Pin the five-question microstructure SPEC the T2 experiment instantiates, in code-grounded pre-registered form, as a charter + machine-checkable conformance predicates. Doc + tests only; NO §6 edit (quarantined to a future Class-3/4 atom under per-atom §8).

**Files.** `handover/preregistration/T2_MICROSTRUCTURE_SPEC_PREREG_2026-06-01.json` (NEW); READ-ONLY cites: `typed_tx.rs:753-764` (NodePosition immutable exposure = traded asset), `:700-716` (PositionKind FirstLong/ChallengeShort; MarketBuy/Sell forbidden §9.4), `sequencer.rs:1225-1280` (settlement gate), `lean_hayek_market.rs:99-103` (stake_from_confidence i64), `:69-83` (MODEL_RATES integer), `:1210-1231` (banked_at_B/cost_of_pass), `reputation_constitutional.rs:149,191-194` (reputation = accepted-WorkTx, Sybil never accrues), `price_index.rs:7-11` (price signal-not-truth), `:316` (`from_env` admits epsilon=0 — the REAL loader), `:514-523` (compute_mask_set: one dominating child masks parent), `constitution.md:413-424` (Art.III.4 Goodhart); `tests/t2_microstructure_conformance.rs` (NEW, Class 1).

**Subtasks + risk.**
1. **(TP-1.1, r0)** ASSET pillar → NodePosition; CompleteSet/MarketBuy/Sell OUT-OF-SCOPE; conservation = stake-return + escrow-release, not share redemption.
2. **(TP-1.2, r0)** ORACLE pillar → Lean `#print-axioms` settlement bundle verified BEFORE WorkTx accept; whitelist POSITIVE-asserted; native_decide (Lean.ofReduceBool) must FAIL (per user-memory Lean-axiom gate); offline-replay flagged as the auditability gap (TP-0-owned).
3. **(TP-1.3, r1)** HONEST-BIDDING pillar → stake + reputation + escrow; **states HONESTLY it is BELIEF-driven, not a proper scoring rule**; pre-registers Brier-by-bin [0,20,40,60,80,100] + Sybil-domination prediction (honest specialist out-bids Sybil within K=<int> rounds at equal wealth).
4. **(TP-1.4, r1)** GOODHART pillar → **(i) reuse existing green test as conformance ANCHOR, do NOT re-implement**: `tests/tb_14_halt_triggers.rs::price_does_not_affect_predicate_result` already greps sequencer for compute_price_index/NodeMarketEntry/RationalPrice/mask_set == 0 (confirmed `grep -ncE 'price' src/state/sequencer.rs` = **0**). **(ii) spend the new-test budget on the UNCOVERED falsifier** — the predicate-id leak remap. Confirmed leak surface is **11 sites** (TP undercounted at 4): `grep -nE 'PredicateFailed\(' src/state/sequencer.rs` = 1234,1239,1269,1275,**2465,2478,2483,2488,2496,2502,2712** (the second cluster includes hardcoded pids like `challenge_window_closed_with_no_upheld_challenge`).
5. **(TP-1.5, r1)** EXPLORATION pillar → epsilon_min > 0 as a BINDING run-validity gate (cite the REAL loader `price_index.rs:316`, not the unit-test line); action-entropy non-collapse = headline exclusion; the TP-2 env wrapper must REJECT epsilon<floor since `from_env` itself accepts 0.
6. **(TP-1.6, r1)** Conformance harness — one predicate per pillar; price-absence DELEGATES to `tb_14_halt_triggers`; `assert_agent_diagnostic_strips_pid` is genuinely new; live-dependent ones `#[ignore]` with named evidence paths.
7. **(TP-1.7, r1)** Assemble + freeze the JSON; record its SHA-256 in an EXTERNAL lock (OBLIGATIONS.md or sibling `.sha256`, per `PPUT_CCL_HARD10_2026-04-26.json` precedent — NO self-referential `self_sha256` field).

**ACCEPTANCE (verifier-revised):**
- **[machine]** Five pillar objects keyed asset/oracle/honest_bidding/goodhart_shield/exploration, each with citation + falsifiable_predicate: `python3 -c "import json; d=json.load(open('handover/preregistration/T2_MICROSTRUCTURE_SPEC_PREREG_2026-06-01.json')); p=d['pillars']; assert set(p)=={'asset','oracle','honest_bidding','goodhart_shield','exploration'}; [p[x]['citation'] and p[x]['falsifiable_predicate'] for x in p]; print('OK')"`
- **[machine]** ORACLE positive axiom assertion FAILS on native_decide/Lean.ofReduceBool (not grep-only); settlement gate before Ok(()): `grep -nE 'PredicateFailed\(' src/state/sequencer.rs` shows 1234,1239,1269,1275 inside `verify_work_predicates`.
- **[machine]** GOODHART price-blindness uses the EXISTING anchor: `cargo test --test tb_14_halt_triggers price_does_not_affect_predicate_result` (exit 0); `grep -cE 'price' src/state/sequencer.rs == 0`.
- **[machine]** GOODHART leak remap (the new falsifier): `goodhart_shield.known_leak` lists the FULL 11-site grep; `tests/t2_microstructure_conformance.rs::assert_agent_diagnostic_strips_pid` asserts each variant's agent-facing projection has an opaque class label and NOT the PredicateId string.
- **[machine]** HONEST-BIDDING integer-only + Brier bins + Sybil-K locked + "belief-driven not proper-scoring" stated: `sed -n '99,103p' src/bin/lean_hayek_market.rs` shows `-> i64`, no f64.
- **[machine]** EXPLORATION epsilon_min>0 is a binding run-validity gate citing `price_index.rs:316` (loader) + `:514-523` (mask); entropy-collapse = exclusion.
- **[machine]** CAUSAL CONTROL pre-registers BOTH ablations: shuffled-price (`lean_hayek_market.rs` shuffled route) AND flatbid firewall (`:916,975,988`). Kill condition: real>shuffle but ==flatbid ⇒ "win is structure/parallelism, not price," NOT headline-eligible.
- **[machine]** Harness compiles, static predicates pass, live ones `#[ignore]`: `cargo check --test t2_microstructure_conformance` (0) AND `cargo test --test t2_microstructure_conformance -- --skip live` (0).
- **[machine]** Frozen JSON tamper-evident via EXTERNAL lock: `shasum -a 256 …` matches a hash in OBLIGATIONS.md / sibling `.sha256`; no `self_sha256` field.
- **[machine]** NO §6 surface edited: `git diff --name-only HEAD` ∩ §6 list = ∅ (price_index.rs is NOT §6; its READ-ONLY cites are safe).

**Prereg fields:** as above + T2 budget B (reasoner tokens, EQUAL per arm) + seed set (12) + task pool (44-theorem EMERGE band) LOCKED; eligibility_gate = arm excluded unless all five conformance predicates pass AND `verify_chain` + `reconstruct_state` (`economy/ledger.rs:287` returns state_root) AND economic_state reconstructed (`tests/economic_state_reconstruct.rs`); non-goals: NO world-first; market-efficiency NOT a theorem; tape-canonical is TP-0-owned, NOT assumed done.

**Traps avoided:** "world-first" rejected (CNP-1980 cited, OS-combination claim); market-efficiency-not-theorem (real-price must beat shuffle); Goodhart pinned with machine-grep + leak flagged; exploration-collapse → binding epsilon_min gate; honest-bidding honestly belief-driven; coordinator arm confirmed ABSENT (0 hits) so TP-1 does not pretend the baseline exists.

---

### TP-2 — Coordinator-Swarm baseline + full PPUT arm-set + counted T2 sweep (main, risk 2–3)

**Objective.** Build the single missing baseline — **COORDINATOR-SWARM** (central decomposer → on-tape `TaskOpen×K` → reputation-routed workers → deterministic synthesis → one Lean verify) as a new `Coordinator` Policy in `lean_market_agent.rs`, reusing g1 tx machinery + LeanJudge at SAME model/budget/task/verifier. Lock the full arm-set, emit five headline metrics per arm, run the counted sweep, report the finding-tree HONESTLY against the live null-prior.

**Files.** `lean_market_agent.rs:80-128` (Policy enum/parse/label/emits_challenges — add Coordinator; confirmed enum has market/random_bear/fixed_bear/shuffled_price/no_price/single/parallel/majority/best_first/skeptic_rerank, NO Coordinator), `:715-740` (golden_path + PPUT), `lean_hayek_market.rs:1209-1233` (finish_alloc — add cost_of_first_valid + wasted_cost_invalid), `:107-180` (MarketEvent — add TaskOpen/RouteAssign/Synthesis + derive_dag), `reputation_constitutional.rs:145-200` (reputation route — reuse as coordinator worker-routing), `price_index.rs:316` (epsilon loader — NOT §6; a `BOLTZMANN_MIN_EPSILON` clamp is a clean Class-1 add), `sequencer.rs:1235-1242` (leak surface — read-only scrub target), `wallet.rs:1-35` (TB-9 MicroCoin), prereg `TP2_COORDINATOR_SWARM_PREREG_2026-06-01.json` (NEW), template `ECONOMY_BENCHMARK_PREREG_2026-05-31.json`, `emerge_stage1_cells_2026-05-31.json` (8 combination-targets), `tests/fixtures/lean_theorems_pool.jsonl`.

**Subtasks + risk.**
1. **(TP-2.1, r0)** Lock prereg cloning the ECONOMY discipline block.
2. **(TP-2.2, r1)** Add `Coordinator` variant + parse + label + emits_challenges (additive; Class 1).
3. **(TP-2.3, r2)** Coordinator serial-then-parallel loop: 1 decomposer call → TaskSpecTree CAS object → K TaskOpen events (each carrying `parent_task_id`+`subtask_spec_hash`, pre-constructed so a worker WorkTx never precedes its TaskOpen) → route via reputation rule → workers bid MicroCoin → DETERMINISTIC synthesis (concat + single verify, no merge-LLM) → Synthesis event. All decomposer/worker/synthesis tokens into the SAME micro_usd accumulator (Class 2). **State the coordinator's routing as a first-class design choice (reputation-weighted ASSIGNMENT, the decomposer's argmax), not "reuses market price routing"** — else the market-vs-coordinator contrast blurs (both end reputation-routed).
4. **(TP-2.4, r2)** Extend MarketEvent + `derive_dag()` rebuilding the subtask DAG from TaskOpen alone; failed subtasks append as `Verify{verdict:false}` (Class 2).
5. **(TP-2.5, r1)** Add cost_of_first_valid + wasted_cost_on_invalid_branches to finish_alloc, both tape-derived integer micro_usd (Class 1).
6. **(TP-2.6, risk 3) — RE-CLASSED Class 4 CANDIDATE.** The Goodhart-shield reject_class scrub MUST default to ZERO `sequencer.rs` edits (it is a §6 restricted surface, `AGENTS.md:216`: "Class 4 cannot hide inside a Class 3 umbrella"). Do the shielding entirely in the agent-facing read-view / prompt-assembly in the bin. If any sequencer touch is truly unavoidable → HALT for a per-atom §8 directive + PRE-§8 clean-context audit before implementation.
7. **(TP-2.7, r2)** Smoke gate before the sweep: coordinator vs market vs single on the 8 combination-targets × 2 seeds; verify_chain green, derive_dag reconstructs, dynamic range, B binds for coordinator (Class 2).
8. **(TP-2.8, r2)** Counted sweep 5 arms + 3 ablations × 12 seeds × {44-pool AND 8-subset}, interleaved per-seed; Wilcoxon-paired + Holm-Bonferroni; **honest finding-tree**: on the monolithic 44-pool the pre-registered prior is market ≤ shuffle (single 7 ≥ shuffled 6 > market 5); the decisive test is the decomposable 8-subset; if market does not beat shuffle, state plainly the win is parallelism/structure/accountability, not price (Class 2).

**ACCEPTANCE (verifier-revised — note the two unrunnable criteria the verifier caught are fixed):**
- **[machine]** PREREG-LOCKED-FIRST + every arm resolves to a parseable Policy (kills the 3 invented vaporware arms): `test "$(git log --diff-filter=A --format=%ct -1 -- …TP2_…json)" -lt "$(git log --diff-filter=A --format=%ct -1 -- …TP2_AGGREGATE_RESULT.json)"`; for each arm name, `grep -qE "\"$a\" *=>|Policy::…"`; `jq -e '.arms|length==8'`.
- **[machine]** COORDINATOR-ARM-RUNS (the `--help` path is REMOVED — confirmed `parse_args` has no `--help` branch and requires `--runtime-repo/--cas/--run-id/--problem`, so the old `--help` criterion could never pass): `grep -nE 'Policy::Coordinator'` + `grep -nE '"coordinator" *=>'` + real smoke `cargo run -q --bin lean_market_agent -- … --policy coordinator --out /tmp/coord.json && jq -e '.policy=="coordinator" and (.schema|startswith("lean_hayek_alloc")) and .tape_chain_ok==true' /tmp/coord.json`.
- **[machine]** FIVE-HEADLINE-METRICS + TYPED-INTEGER MONEY (structural check, NOT the co-occurrence grep that false-failed on `fn finish_alloc(… micro_usd: i64, … wall: f64)`): `jq -e 'has("banked_at_B") and has("cost_of_pass_micro_usd") and has("cost_of_first_valid") and has("wasted_cost_on_invalid_branches") and has("pput") and (.cost_of_pass_micro_usd|type=="number" and floor==.cost_of_pass_micro_usd)'`; `grep -nE '(micro_usd|amount_micro|cost_of_first_valid|wasted_cost): *(i64|i128|u64)'`; `! grep -nE '\b(micro_usd|cost_of_pass|cost_of_first_valid|wasted_cost)\b[^;]*\bas f(64|32)\b'`.
- **[machine]** SINGLE-CANONICAL-TAPE + cross-ledger conformance (resolves the two-tape conflation): declare the ONE authoritative tape (recommend the in-bin MarketTape that finish_alloc/golden_path already read; any g1 TaskOpen is a derived shadow); `wasted_cost` recomputed from THAT frozen tape == in-run accumulator; if both ledgers carry the chain, TaskOpen DAG from each is byte-equal: `cargo test -q --bin lean_hayek_market -- wasted_cost_equals_tape derive_dag_from_taskopen_only cross_ledger_taskopen_byte_equal`.
- **[machine]** FAILED-BRANCHES-ON-TAPE precondition (discharge the TP-0 hard gate before any counted cell): `cargo test -q -- failed_branches_appear_on_tape every_llm_call_has_tape_node`; `jq -e '[.tape_events[]?|select(.kind=="Verify" and .body.verdict==false)]|length >= 1'`.
- **[machine]** CAUSAL-GATE-DECIDED (no free escape hatch — verdict bound to data): `jq -e '.comparisons.market_vs_price_shuffle as $c | ($c.p_holm<0.05 and $c.market_banked>$c.shuffle_banked and .verdict=="price_causal") or (($c.p_holm>=0.05 or $c.market_banked<=$c.shuffle_banked) and .verdict=="price_not_causal_win_is_structure")'`.
- **[machine]** SAME-BUDGET-PARITY (enforced knob = reasoner tokens): every cell `reasoner_completion_tokens <= B`; coordinator decomposer+worker+synthesis split within tolerance of locked overhead; cells over B EXCLUDED + reported.
- **[machine]** GOODHART-SHIELD WITHOUT §6 EDIT: `! grep -rnE 'SettlementPredicateFailed\([^)]*\)|predicate_id' src/bin/lean_market_agent.rs src/bin/lean_hayek_market.rs | grep -iE 'prompt|reject_class|agent_view|broadcast'`; `( ! git diff --name-only origin/main | grep -qE 'src/state/sequencer.rs' ) || ls handover/directives/2026-06-01_TP2*SECTION8*`.
- **[machine]** EXPLORATION-FLOOR enforced-or-logged: every cell records epsilon on tape; `jq -e '.epsilon_den>0 and .epsilon_num>=1 and (.epsilon_num*10 >= .epsilon_den)'`; optional `cargo test -q -- min_epsilon_clamp_rejects_zero`.
- **[machine]** CONSTITUTION + §6 UNTOUCHED: FC1/FC2/FC3 byte-identical; no §6 surface modified for an admission change absent per-atom §8.

**Prereg fields (corrected — arm names map to REAL Policies, no vaporware):** arms (8: single | parallel | best_first | coordinator | market | shuffled_price | random_bear | a third real ablation — `delayed_price`/`random_price` each need a NEW Policy variant + dispatch like `shuffled_price`, OR drop and map to existing names); seeds (12 explicit); binding_budget_B_reasoner_completion_tokens (single integer, e.g. 4000 per ECONOMY pilot, same all arms); coordinator_decomposer_overhead_pct (LOCKED split); coordinator_K_max (≤5); task_set (44-pool) AND combination_target_subset (the 8: lm_commute_pow,lm_det_3x3,lm_det_mul,lm_f,lm_ineq2,lm_monotone_glue,lm_sum_cubes,lm_trace_prod); OMEGA (Lean `Verified`, whitelist {propext,Classical.choice,Quot.sound}, sorry/admit/native_decide rejected); MODEL_RATES pin date (`:69-77`, 2026-05-31, identical roster); primary=banked@B; secondary=cost_of_pass/cost_of_first_valid/wasted_cost/pput; causal_gate (market>shuffle at Holm-p<0.05 REQUIRED for any price claim); GO/NOGO; exploration_floor (BOLTZMANN_MIN_EPSILON, locked 1/10); replay_exclusion_rule (verify_chain=false OR non-whitelist axioms ⇒ excluded).

**Traps avoided + corrected facts:** PRICE-WITHOUT-CAUSAL-GATE encoded as a hard pre-registered gate; **H0 numbers corrected to single 7 ≥ shuffled 6 > market 5** (shuffle, signal-destroyed, BEAT market — counter-evidence is STRONGER than first stated); world-first overclaim rejected; cost-as-latency-artifact closed by SAME-budget + locked overhead; two-tape conflation resolved by naming ONE authoritative ledger; TP-2.6 re-classed Class-4-candidate; epsilon floor downgraded to a prereg-config + tape-logged gate (no MIN_EPSILON symbol exists in src/ today — `price_index.rs` accepts epsilon=0). **Strategic note:** the whole value of TP-2 is the decomposable 8-subset (Hong-Page E=M−D diversity non-zero); the 44-pool run is a pre-registered confirmation of the null, not a test.

---

### TP-3 — Art.V Self-Evolving Harness MARKET: ArchitectAI as priced participant (branch, 20%, risk 2–3)

**Objective.** Instantiate the FC3 meta-loop (constitution.md:826-870) as a PRICED, REPLAYABLE, VETO-GATED market whose tradable good is a HARNESS-IMPROVEMENT PROPOSAL — TuringOS's answer to Meta-Harness/AHE. Each self-modification is a loss-bearing market position settled by SEALED held-out eval, gated by a constitutional Veto-AI ({PASS,VETO} only, Art.V.1.3, `constitution.md:740-765`), reconstructable from a frozen tape. INSTANTIATES FC3, does NOT alter it (its hash is a Class-4 must-not-change contract).

**Re-anchored CORE PROPOSITION (verifier blocker — the single-shot sealed-Δ market is the AGGREGATION near-null regime; the validated regime is adversary-robust sequential allocation):** the falsifiable HEADLINE is the **Sybil-defunding / calibration-honesty** property — capital-at-risk permanently ejects an over-claiming proposer that GREEDY-BEST-PREDICTED keeps funding — measured as realized-uplift-per-meta-token UNDER an adversarial proposer fraction across repeated rounds (mirror `reputation_constitutional.rs:147-198`). Raw realized-uplift is demoted to TIE-acceptable; a META=GREEDY tie on raw uplift is PREDICTED (the repo's own aggregation-regime null), not a surprise. A TIE vs GREEDY is a win because the cost bought is accountability (sealed eval + Veto + tape), not raw uplift.

**Files.** `constitution.md:704-716/719-737/740-765/826-870` (Art.V triple-defense / ArchitectAI authority / Veto-AI / FC3 — READ-ONLY), `lean_hayek_market.rs:107-145` (MarketEvent — extend in the NEW bin), `:1209-1232` (finish_alloc uplift metric), `:69-97` (MODEL_RATES B_meta), `reputation_constitutional.rs:147-198` (Sybil-resistant reputation), `typed_tx.rs:687/700-716` (PositionSide — **do NOT reuse as a YES/NO market side**, §9.4 forbids MarketBuy/Sell; keep meta-bet in bin-local `Invest{side:"YES"|"NO"}` string shape, or model as a WorkTx-stake FirstLong bond), `sequencer.rs:1225-1280/1848-1866` (§6 — settles THROUGH, unchanged), `economy/ledger.rs:56-91` (L4 AcceptedEntry), `price_index.rs:7-11` (meta-price signal-only), `cmd_verify_chaintape.rs:40-48`, prereg `ARTV_META_MARKET_PREREG_<date>.json` (NEW), `src/bin/artv_meta_market.rs` (NEW).

**Subtasks + risk.**
1. **(TP3-S0, r0)** PREREG lock: 7 change-classes (verifier/prompt-routing/tool-API/tape-schema/cost-attribution/error-abstraction/memory-retrieval) each tagged with AGENTS.md risk-class; **≥5 arms {META-MARKET, GREEDY-BEST-PREDICTED, FIFO-UNPRICED, META-PRICE-SHUFFLE, META-FLATBID}**; B_meta INCLUSIVE of sealed-settlement cost; frozen held-out split SHA-256 + FIXED size (8–12 theorems, e.g. the combination-target set); settlement method declared; planted-Sybil id; epsilon floor; Veto article-set; NON-CLAIM statement.
2. **(TP3-S1, r1)** ASSET + 5-microstructure for the META good (proposal = CAS-addressed diff vs pinned baseline carrying predicted sealed-uplift; oracle = sealed held-out eval; honest bidding = reputation accrues only on sealed-confirmed uplift; Goodhart = sealed disjoint split + opaque scoring; exploration = meta-price signal-only, epsilon floor).
3. **(TP3-S2, r1)** META-TAPE schema (MetaProposal/MetaPredict/MetaInvest/MetaSeal/MetaVeto/MetaResolve); VETOED or sealed-Δ≤0 lands `verified=false`; integer-money.
4. **(TP3-S3, r2→2-3)** VETO-AI gate BEFORE sealed settlement: VETO if change touches a §6 surface without §8, or disables a verifier / widens budget / swaps model (the AHE-blocked shortcuts); VETO ⇒ REJECTED + verified=false, NO sealed run, NO bank, regardless of price. **Price NEVER overrides Veto.**
5. **(TP3-S4, risk 2→Class 3) — RE-CLASSED net-new infra.** Sealed-eval engine (checkout baseline → apply patch → run FROZEN split → compute Δ → emit MetaSeal). Confirmed NO existing bin does git-checkout+apply+rerun. **Default to FROZEN-COMPETENCE-MATRIX replay** (the repo's proven deterministic+cheap method: collect agent×task Lean matrix ONCE, replay all arms) — this simultaneously fixes determinism, the ~999-call/3.6h cost, AND replayability. Sandbox NEVER touches the live tree; any §6-class real apply requires §8.
6. **(TP3-S5, r2)** Three+ arm dispatcher sharing proposer pool / B_meta / Veto / sealed-eval; headline = realized-uplift-per-meta-token + calibration error |predicted−sealed| per arm.
7. **(TP3-S6, r2)** Goodhart + correlation shielding audit (proposers see only abstracted signals + meta-price, never held-out bodies / coefficients / other patches; eval_split_hash proves no mid-run swap).
8. **(TP3-S7, r1)** Replay + tape-canonical conformance.

**ACCEPTANCE (verifier-revised):**
- **[machine]** PREREG includes the price-destruction arms + sealed-eval cost bound: `python3 …` asserts 7 change_classes, `{META-MARKET,META-PRICE-SHUFFLE,META-FLATBID,GREEDY-BEST-PREDICTED,FIFO-UNPRICED} ⊆ arms`, `B_meta_includes_settlement_cost`, `held_out_split_hash`, `held_out_split_size`, `settlement_method`, `sybil_proposer_id`, `epsilon_floor`; mtime precedes earliest counted output.
- **[machine]** PRICE-IS-CAUSAL FIREWALL (replaces FIFO-only headline): META-MARKET realized-uplift-per-meta-token STRICTLY exceeds BOTH META-PRICE-SHUFFLE and META-FLATBID, paired CI excluding 0 on each delta; else registered verdict = "meta-uplift comes from STRUCTURE not price" reported as the headline. (Flatbid firewall reused from `lean_hayek_market.rs:916,975,988`.)
- **[machine]** CALIBRATION-HONESTY AS PRIMARY: with planted over-claiming Sybil at the registered adversary fraction over the repeated-round horizon, META-MARKET defunds the Sybil (`reputation[sybil]==0` all rounds AND `sybil_final_wallet < honest_final_wallet`) while GREEDY keeps funding it; `calib_err[META] CI-upper < calib_err[GREEDY] CI-lower`: `python3 scripts/check_sybil_calibration.py …`.
- **[machine]** NO-VETO-BYPASS + POSITIVE test: every BANKED has a preceding MetaVeto=PASS; ZERO VETO banked; AND ≥1 proposal touching a §6 surface without §8 is actually VETOED (gate fires): `python3 scripts/check_no_veto_bypass.py …`.
- **[machine]** TAPE SUBSTRATE INTERNALLY CONSISTENT (resolves the category error — `verify_chaintape` only checks L4 ChainTape, NOT MarketTape-lite JSONL): pick ONE — (A) real Sequencer/L4 ⇒ `verify_chaintape … exits 0` AND declare typed-tx additions Class-4 + per-atom §8; OR (B) MarketTape-lite ⇒ bin-local `verify_chain()` + bespoke `artv_meta_conformance` ONLY, with `grep -c verify_chaintape over the TP claims == 0`.
- **[machine]** ALL META-BRANCHES ON TAPE + SETTLEMENT RE-DERIVABLE FROM RECORDED INPUTS (NOT byte-identical stochastic re-execution): `cargo test --test artv_meta_conformance` {every_meta_proposal_on_tape, meta_price_derived_from_invests_only, sealed_verdict_rederivable_from_recorded_inputs, proof_body_cid_present_on_seal}.
- **[machine]** NO §6 MUTATED BY THE META-MARKET + SANDBOX BOUNDARY: diff writes none of kernel/bus/wallet/sequencer/typed_tx/economy-ledger unless substrate A + cited §8 directive; every BANKED §6-touching change carries `sandbox_only=true`, never auto-merged.
- **[machine]** FC3 / flowchart hashes UNCHANGED: `bash scripts/run_constitution_gates.sh` (constitution_matrix_drift green) AND `cargo test constitution_flowchart_source_alignment`; flowchart-hash diff pre/post == empty.

**Prereg fields:** 7 change_classes (each AGENTS.md-risk-tagged); arms {META-MARKET, GREEDY-BEST-PREDICTED, FIFO-UNPRICED, META-PRICE-SHUFFLE, META-FLATBID}; B_meta (equal per arm, integer micro-USD, INCLUDES settlement cost); baseline_commit; held_out_split + hash + size; uplift_metric (Δbanked@B primary, Δcost_of_pass secondary); headline = realized-uplift-per-meta-token; **primary endpoint = Sybil-defunding + calibration error with its own CI + MDE + required N**; planted-Sybil id; epsilon floor; Veto article-set; sandbox-vs-live boundary per change-class; explicit NON-CLAIM (Meta-Harness/AHE/Darwin-Gödel prior art; claim = OS-combination + accountability).

**Traps avoided:** world-first-self-evolving rejected; META-MARKET must beat shuffle+flatbid; sealed self-serving-eval shielded (frozen hash-pinned disjoint split); Veto-bypass invariant; tape-canonical on the meta-layer; self-modifier cannot silently edit §6 (Veto + sandbox-only); correlation collapse shielded; capital-at-risk calibration; FC3 instantiated-not-altered; reuses T2's settle/PPUT plumbing, does not gate the 70% line. **Cost realism:** frozen-matrix settlement is mandatory or a handful of proposals × arms exhausts the compute the 70% T2 line needs.

---

### TP-4 — Heterogeneous EFFECTIVE-INDEPENDENT-N, controlled (branch, 10%, risk 2–3)

**Objective.** Prove or falsify, as a CONTROLLED experiment, that under SAME total budget B / SAME verifier / SAME task set, TuringOS-market over a HETEROGENEOUS pool converts model DIVERSITY into verified-pass-rate more efficiently than (A) closed-strong repeated-sampling, (B) coordinator-swarm [consumed from TP-2], (C) single-open swarm, (D) multi-open no-price, vs (E) multi-open market. Deliverable = the decentralization quintuple per arm: pairwise failure correlation, unique coverage, marginal Δ_i, price-weighted reputation, N_eff. The reframe: not "many open > one closed" but "does the price/ledger/predicate INSTITUTION extract more verified-pass per token from a diverse pool than sampling/coordinator/un-priced swarm."

**HARD GATE + modal-outcome warning (verifier blocker — the live Stage-2 data predicts the collapse branch):** TP-4's N_eff deliverable is gated on EMERGE Stage-2 Q1 firing REAL (`v4-pro@k8 ≤ union@k4 − 3 = ≤23`). Task #28 is `in_progress`; the deciding A_control is only a partial (`/tmp/emerge_stage2_v4pro_k8.json.partial.json`, ~31/44 scored) and ALREADY shows v4-pro@k8 alone recovering 5 of the 8 "combination-target" theorems it missed at k4 — pure extra sampling (the confound `EMERGE_STAGE1_FINDINGS:59-66` named). Separately, qwen3.7-max@k4 alone solves 23/44 incl 6/8 combination-targets (`emerge_stage2_qwen_cells_2026-06-01.json`), so a single strong cross-family model subsumes most of the "diversity" the quintuple measures. **Expect Q1=JUST_SAMPLING; plan TP-4 as the DEMOTED reputation+cost-accounting exercise, not the N_eff headline.** Do NOT author the TP-4 prereg or build until #28 lands a COMPLETE A_control and a written Q1 verdict.

**Files.** `EMERGE_STAGE2_PREREG_2026-06-01.json`, `emerge_stage1_cells_2026-05-31.json`, `EMERGE_STAGE1_FINDINGS_2026-05-31.md`, `lean_market_agent.rs:80-130/719-785` (Policy::Market/NoPrice/ShuffledPrice — real ChainTape path via `build_chaintape_sequencer_with_initial_q`), `lean_hayek_market.rs:43-97/107-180/1210-1231` (LITE MarketTape — DIFFERENT schema), `lean_hetero_market.rs:1-25` (**DISQUALIFIED as base**: single-model tactic-family specialization, "Diagnostic-grade, no chain/CAS/replay, allows native_decide" — violates TP-4's OMEGA-only + verify_chain gates), `reputation_constitutional.rs:83-86/145-194` (TokenCounts{1,1} — proof BODIES not persisted), `wallet.rs:1-55`, `price_index.rs:7-11`, `P4_LITE_PRICE_CAUSALITY_2026-05-31.md:73-119`, prereg `TP4_EFFN_PREREG_<date>.json` (NEW), `src/bin/effn_aggregator.rs` (NEW), `tests/effn_metric_conformance.rs` (NEW).

**Subtasks + risk.**
1. **(TP4-S0, r0)** GATE CHECK: block until Stage-2 Q1 resolves on a COMPLETE A_control; if JUST_SAMPLING ⇒ STRIKE (not caveat) the N_eff/marginal-Δ/failure-correlation quintuple, prereg primary = reputation+cost-only.
2. **(TP4-S1, r0)** PRE-REGISTRATION (after the gate).
3. **(TP4-S2, r0)** TAPE-CANONICAL precondition: pin ONE substrate; state plainly the strong tape-canonical claim is unavailable (Commits 1-4 not landed — only `constitution_tape_canonical_gate.rs` exists, not the per-violation suite); scope reputation to success-rate-only unless proof bodies are persisted.
4. **(TP4-S3, risk 2→Class 2-3) — RE-SCOPED net-new.** Both candidate bins take ONE model per process (`Args.model:String`). BUILD a multi-model roster runner from scratch (extend `Args.model:String → roster:Vec<String>` stamping per-agent model_id on every Invest/LlmCall) on the chosen substrate; **add a pinned, dated, sourced MODEL_RATES row for qwen3.7-max (currently FALLBACK at `:80-81`)**; smoke gate: 1-theorem run emits ≥2 distinct model_ids on the tape + passes the chain check BEFORE any sweep.
5. **(TP4-S4, r1)** `effn_aggregator.rs` — 5 metrics as pure functions over the pinned-substrate tape; integer money; f64 confined to N_eff fit + softmax.
6. **(TP4-S5, r1)** `effn_metric_conformance.rs` — formula tests on Stage-1 ground truth; **fixture derivation corrected: Δ_V3.2 = |union(v4pro,V3.2)| − |union(v4pro)| = 26 − 18 = +8** (equals the 3-model union because Qwen is strictly dominated, a_only=0), Δ_Qwen=0.
7. **(TP4-S6, risk 3)** Counted runs (real money/evidence/verifier); arm B BLOCKED-PENDING-TP2 if unshipped; verify_chain or EXCLUDED.
8. **(TP4-S7, r0)** Aggregate + honest report (N_eff with failure-correlation matrix + marginal-contribution vector + explicit "no open model almost-as-good-as a closed flagship").

**ACCEPTANCE (verifier-revised — the two ill-posed criteria are fixed):**
- **[machine]** S0 gate resolves on a COMPLETE (not `.partial.`) Stage-2 A_control: `jq -e '.schema=="lean_emergence_stage1.v1" and (.per_model["deepseek-v4-pro"].solved_count_at_k|type=="number")' …`; then q1_branch ∈ {REAL,JUST_SAMPLING}; if JUST_SAMPLING the prereg deliverable matches `reputation_cost_only`.
- **[machine]** Prereg git-committed before any arm result.
- **[machine]** SINGLE canonical substrate pinned; every counted arm passes ITS chain check on that one substrate; no arm mixes ChainTape + lite-MarketTape signals.
- **[machine]** Roster runner emits ≥2 distinct model_ids; qwen3.7-max priced by a REAL row (no FALLBACK): `grep -qE '"qwen3.7-max"' src/bin/lean_hayek_market.rs && … --roster … && jq -rs '…|unique|length' /tmp/smoke.jsonl | grep -qE '[2-9]' && ! grep -q FALLBACK /tmp/smoke_pricing.log`.
- **[machine]** CONTROLLED BUDGET BASIS = equal reasoner-TOKEN budget (the only knob the harness enforces — micro_usd is a DERIVED output of tokens×rate, so equal-dollar across heterogeneous rates is unsatisfiable as a gate): `jq -s 'map(.reasoner_budget_tok)|(max==min)'` MUST be true; `jq -s 'map(.micro_usd)|(max-min)'` reported vs `prereg.micro_usd_report_band` (NOT a pass/fail gate).
- **[machine]** Metric formulas correct on Stage-1 ground truth (Δ_V3.2=+8 as the 2-model-union marginal, Δ_Qwen=0, price-weighted reputation Σ=1, N_eff finite for p̄=26/44 & p_single=18/44): `cargo test --test effn_metric_conformance`.
- **[machine]** Falsifier = RealPrice vs ShuffledPrice on the SAME hetero roster (not only vs NoPrice): arms include E(market), E_shuffled, D(no-price); verdict requires E to beat BOTH shuffle and random or be reported non-causal.
- **[machine]** Integer-only money in new code; **[machine]** no §6 surface touched; **[machine]** arm B BLOCKED-PENDING-TP2 with grep receipt or consumed from a cited TP-2 binary.
- **[judged]** N_eff honesty caveat adjacent to every figure; report states qwen3.7-max@k4 alone reaches 6/8 combination-targets and that this compresses N_eff toward 1.

**Prereg fields:** stage2_gate.q1_branch + result_path; arms {A,B,C,D,E} fixed; heterogeneous_roster [deepseek-v4-pro, DeepSeek-V3.2, qwen3.7-max] (Qwen3-32B EXCLUDED, a_only=0); budget = equal TOTAL reasoner tokens; seeds 12; task_set (8 combination-targets primary, 26-union optional); OMEGA-only whitelist; the 5 metric defs verbatim; reputation_signal_basis (success-rate-only unless proof bodies persisted); price_causality_interpretation (capability vs fair-resource-allocation — P4-lite found price NOT causal for Lean solving); qwen3.7-max pricing row (pinned/dated/sourced); dashscope_viability_fallback (Qwen2.5-72B if >5% failure / 2× latency); statistical_test (McNemar paired or bootstrap-CI on seed-paired diff); exclusion_rule (verify_chain=false ⇒ excluded).

**Traps avoided:** "many open > one closed" reframed to controlled N_eff with mandatory failure-correlation + "summary statistic not independence" caveat; world-first rejected; market-efficiency-not-theorem (E must beat shuffle AND random); cost-as-latency closed by equal-token budget; sampling confound GATED on Stage-2; strictly-dominated model excluded; un-replayable PPUT excluded; reputation scoped to success-rate; coordinator NOT built here; DashScope fallback pre-locked. **Strategic:** TP-4 is triple-blocked (TP-1/2/3) + gated on a Stage-2 trending negative + needs a non-existent multi-model tape-canonical runner — PARK it behind Stage-2; if JUST_SAMPLING, rewrite from scratch as a price-weighted-reputation + cost study (which CAN stand on `reputation_constitutional.rs` today).

---

## 5. CAMPAIGN-LEVEL final acceptance (pre-registered, falsifiable)

Locked BEFORE any counted run. Primary headline metric = **banked@B**; secondary = cost_of_pass_micro_usd. The verdict is computed only on cells that pass the TP-0 replay gate (failed replay ⇒ excluded, pre-committed).

| Verdict | Pre-registered condition (all clauses binding) |
|---|---|
| **GO** | On banked@B over ≥12 seeds, paired (Wilcoxon signed-rank, Holm-Bonferroni): **MARKET arm beats COORDINATOR-SWARM** AND **MARKET > SHUFFLED-PRICE** AND **MARKET ≠ FLATBID** (the causal firewall — not statistically matched by constant-bid), all at **p<0.05**, on the **banked@B** headline at fixed `verify_budget < k`, with **every headline arm replay-GREEN** (`verify_market_tape` exit 0) and **#print-axioms ⊆ whitelist** on every banked proof. The win is *price coordination*. |
| **WEAK-GO** | MARKET **ties** COORDINATOR-SWARM and/or SHUFFLE on solve-rate (banked@B CI overlaps 0) **but strictly wins cost-of-pass_micro_usd** (lower cost-of-pass at equal banked, paired, p<0.05), AND is replay-GREEN + Sybil-resistant (TP-1 honest-bidding falsifier passes). The accountability/efficiency cost is bought even without a capability win. Reported as "auditable governance substrate," not "clever market." |
| **NO-GO** | **MARKET ≈ SHUFFLE** (banked@B and cost-of-pass CIs both overlap 0) OR **MARKET == FLATBID** (constant-bid matches real price). Pre-registered conclusion: **the win, if any, was parallelism/structure/harness, NOT price** — exactly the architect's trap, and consistent with the repo's monolithic-pool prior (single 7 ≥ shuffled 6 > market 5). Price is NOT the causal allocator on this substrate; the institution's defensible value collapses to replay + Sybil-resistance accounting only. |

**Falsifiability anchors:** the NO-GO lane is reachable and is the *prior* on the 44-pool monolithic run (the repo's own 5× null + the H0 table where shuffle beat market). The decisive discriminating test is the **decomposable 8-theorem combination-target subset** (where Hong-Page E=M−D diversity is non-zero); the 44-pool run is a pre-registered confirmation of the null, not the test. The verdict string is machine-bound to the data (TP-2 CAUSAL-GATE-DECIDED criterion): a mislabeled verdict fails the check. For TP-3 the same GO/WEAK-GO/NO-GO logic applies with META-MARKET vs META-PRICE-SHUFFLE/META-FLATBID and the Sybil-defunding axis as the primary endpoint.

---

## 6. Objections & new insights raised during research (architect-invited dissent)

**Structural / blocker-class:**
1. **Four schemas, not one substrate** (TP-0): the bin has `run_probe_alloc`→`lean_probe_alloc.v1` (wallets, NO micro_usd), `run_alloc`→`lean_hayek_alloc.v2` (micro_usd/cost_of_pass, NO wallets), `run_compete`→`lean_hayek_compete.v1`, fall-through `run`→`lean_hayek_market.v1`. A single `derive_cost/derive_pnl/derive_wallets` cannot match all; the verifier MUST be schema-aware. Acceptance #1/#5 were unsatisfiable on the het4 arm the TP named.
2. **Two incompatible tape systems, silently conflated** (TP-0/2/3/4): MarketTape-lite (in-bin JSONL prev-hash, `verify_chain` checks only JSONL) vs the real g1 ChainTape/L4 (Sequencer, `verify_chaintape`-green). "verify_chain green" ≠ Art.0.2 canonical replay. The 24 known tape-canonical violations all live on the g1/bus side. Every PPUT/auditability claim must name WHICH tape is authoritative.
3. **§6 mis-classification** (TP-2.6, TP-3-S4, TP-4-S3): the sequencer reject_class scrub, the sealed-apply engine, and the multi-model roster runner were undersold as Class 2 wire-up; they are Class-4-candidate / Class-3 net-new. "Class 4 cannot hide inside a Class 3 umbrella" (`AGENTS.md:216`).
4. **Regime mismatch** (TP-3): single-shot sealed-Δ aggregation is the repo's near-NULL regime; the validated regime is adversary-robust sequential allocation. The headline must be Sybil-defunding/calibration, not raw uplift.
5. **TP-4 premise being killed by live data**: v4-pro@k8 recovers 5/8 combination-targets by extra draws (sampling confound); qwen3.7-max alone subsumes 6/8. Expect Q1=JUST_SAMPLING ⇒ demote TP-4.

**Correctness corrections (story-fication caught):**
6. `verify_chain` ALREADY aborts (`:149 Err(_) => return false`) — it does NOT silent-skip; the real change is bool→Result ergonomics. The genuine silent-skip is `wal.rs:88-92` (out of scope).
7. H0 numbers were backwards: real table is single 7 ≥ shuffled 6 > market 5 — the signal-destroyed shuffle BEAT the real market (counter-evidence is STRONGER than first claimed).
8. The Goodhart price-absence test ALREADY exists and passes (`tb_14_halt_triggers.rs::price_does_not_affect_predicate_result`); a duplicate is a criterion-you-cannot-fail. Redirect to the UNCOVERED leak remap.
9. Predicate-id leak is ~2.5× larger than enumerated: 11 sites confirmed (4 in `verify_work_predicates` + a 7-site settlement-window cluster at 2465–2712 with hardcoded pids).
10. epsilon=0 is admitted by the REAL loader `price_index.rs:316` (not the unit-test line); no `MIN_EPSILON` symbol exists in src/ — "locked" is a prereg-config promise, not a code invariant (and price_index.rs is NOT §6, so a clamp is a clean Class-1 add).
11. `--policy realprice` is valid only for the n_rounds betting path (`route()` `:452`), NOT the het4 probe-alloc arms (market/shuffled/uniform/roundrobin/flatbid/single_strong, `:988`) — a latent command bug for any het counted run.
12. `--policy coordinator --help` acceptance was unrunnable (`parse_args` has no `--help` branch and requires 4 args) — replaced with a real smoke cell.
13. The f64-money grep false-failed on `fn finish_alloc(… micro_usd: i64, … wall: f64)` (different tokens, same line) — replaced with a structural integer-type check.
14. Budget knob is reasoner-TOKENS; micro_usd is derived — equal-dollar across heterogeneous rates is unsatisfiable as a gate (one-line architect decision needed before TP-2/TP-4).
15. PositionSide is a "responsibility bond, not market side"; MarketBuy/Sell forbidden §9.4 — do not reuse it as a YES/NO meta-bet side (TP-3).
16. `self_sha256` is a self-reference paradox; repo convention hashes EXTERNAL inputs (`PPUT_CCL_HARD10`) — use an external `.sha256` / OBLIGATIONS.md lock.
17. Diff base must be `git merge-base HEAD origin/main` = `a663ed4d`, NOT literal main (`1f00012d`) nor origin/main (`a68ba081`) — else scope-guards drown in unrelated emerge-stage2 deltas.

**Strongly-recommended insights:**
18. **Frozen-competence-matrix** is the repo's proven cheap+deterministic+replayable method (`reputation_constitutional`, het_emergence); porting TP-3's sealed eval to a frozen patch×eval-split matrix fixes determinism, the ~3.6h/run cost, AND replayability at once.
19. The **flatbid firewall** is already implemented and was the decisive control (`:916,975,988`) — any market design omitting a price-destruction arm repeats the exact mistake the architect already corrected on the T2 line.
20. A standalone `BOLTZMANN_MIN_EPSILON` clamp on `price_index.rs` (NOT §6) is a clean Class-1 hardening the architect could land independently to protect every future price experiment from exploration collapse — candidate for a spin-off.

---

## 7. Scope note

This document is the **decomposition + acceptance-criteria step only**. It produces: (a) the dependency DAG and resource split, (b) per-package objectives/files/subtasks/risk, (c) machine-checkable per-package acceptance criteria with exact commands, (d) the pre-registered campaign GO/WEAK-GO/NO-GO. **No counted experiment is authorized by this document.** No counted run begins until **TP-0 lands** (the tape-canonical semantic-version-A prerequisite, replay-GREEN on the pinned arm) **AND each downstream TP's pre-registration JSON is locked and externally hash-pinned** before any arm result is read. Per the 70/20/10 split, work proceeds TP-0 → TP-1 → TP-2 (main, 70%); TP-3 (20%) reuses TP-2 plumbing and may proceed in parallel after TP-1; TP-4 (10%) is PARKED behind a COMPLETE EMERGE Stage-2 Q1 verdict and TP-2's coordinator-swarm, and is rewritten to a reputation+cost study if Stage-2 returns JUST_SAMPLING (the modal expected outcome on current evidence). The lib.rs touch in TP-0 and any sequencer/typed_tx touch in TP-2/TP-3 are gated to per-atom §8 ratification + PRE-§8 clean-context audit before implementation; FC1/FC2/FC3 are not changed by any package.

**Key files referenced (absolute):** `/Users/zephryj/work/turingosv4/src/bin/lean_hayek_market.rs`, `/Users/zephryj/work/turingosv4/src/bin/lean_market_agent.rs`, `/Users/zephryj/work/turingosv4/src/bin/reputation_constitutional.rs`, `/Users/zephryj/work/turingosv4/src/state/sequencer.rs`, `/Users/zephryj/work/turingosv4/src/state/typed_tx.rs`, `/Users/zephryj/work/turingosv4/src/state/price_index.rs`, `/Users/zephryj/work/turingosv4/src/economy/ledger.rs`, `/Users/zephryj/work/turingosv4/src/lib.rs`, `/Users/zephryj/work/turingosv4/src/wal.rs`, `/Users/zephryj/work/turingosv4/tests/tb_14_halt_triggers.rs`, `/Users/zephryj/work/turingosv4/constitution.md`, `/Users/zephryj/work/turingosv4/AGENTS.md`, `/Users/zephryj/work/turingosv4/handover/preregistration/` (NEW prereg JSONs land here), `/Users/zephryj/work/turingosv4/handover/reports/`.