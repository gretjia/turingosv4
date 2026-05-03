OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019def9a-4808-7231-a549-0b257d3f0530
--------
user
# Codex TB-14 Atom 6 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the TB-14 Atom 6
(production wire-swap + legacy CPMM excision) ship-gate dual external
audit. Independent of Gemini ship audit (parallel, architectural strategic
angle).

**Mandate**: Class 3 (production code path migration; STEP_B restricted
files `src/kernel.rs` + `src/bus.rs` touched). Per memory
`feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Round cap = 2 per
`feedback_elon_mode_policy`.

## Audit target

- Charter: `handover/tracer_bullets/TB-14_charter_2026-05-03.md`
- Atom 6 kickoff: `handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md`
- Architect ruling §5 verbatim: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- Closes: `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`
- ChainTape smoke evidence: `handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md`

```text
TB-14 Atom 5 baseline:  a9fbdf3                 (Atom 5 ship; 6/6 halt-triggers GREEN; 841/0/150)
TB-14 Atom 6 range:     a9fbdf3..38412bf        (this audit)

Atom 6 commits to anchor your reads:
  9cc40e1  TB-14 handover — LATEST.md update + Atom 6 kickoff doc (pre-atom-6 prep)
  44cd480  TB-14 Atom 6 — production wire-swap + legacy CPMM excision
  38412bf  TB-14 Atom 6 follow-up — close internal auditor F1 (dead BusResult::Invested f64)
  c291dde  TB-14 Atom 6 LATEST.md update — local-commits-shipped, external dual audit pending

HEAD (38412bf): cargo test --workspace = 821 passed / 0 failed / 150 ignored
                (delta_vs_a9fbdf3 = -20 net = 25 deleted CPMM tests + 5 new TB-14 tests)
                6/6 architect §5.7 halt-triggers GREEN
                ChainTape smoke (chain-backed, deterministic, non-LLM) PASS

Internal `auditor` subagent verdict (read-only Class 3 self-review on 44cd480):
  CHALLENGE conviction=high, recommendation=FIX-THEN-PROCEED.
  Single finding F1 (CHALLENGE, FIX-NOW): src/bus.rs:95 dead
  `BusResult::Invested { node_id, shares: f64 }` enum variant — pre-TB-9
  invest-path residual; zero call sites, zero match arms; halt-trigger #4
  only fences price_index.rs so this f64 surface in TB-14-touched bus.rs
  was unfenced. ADDRESSED by 38412bf (4-line deletion + bus.rs rehash).
  Other findings F2-F5 ACCEPTED (cosmetic / out-of-scope / process-discipline /
  pending-external).
```

## DELETIONS (closing OBS_TB_12_LEGACY_CPMM_QUARANTINE)

- `src/prediction_market.rs` (entire file — 390 LoC; BinaryMarket CPMM,
  f64 trading semantics, automatic liquidity) — DELETED
- `src/lib.rs` `pub mod prediction_market;` — DELETED
- `src/kernel.rs` market fields (`markets: HashMap<NodeId, BinaryMarket>`,
  `bounty_market: Option<BinaryMarket>`, `bounty_lp_seed: f64`) +
  methods (`create_market` / `buy_yes` / `buy_no` / `yes_price` /
  `market_ticker` / `market_ticker_full` / `open_bounty_market` /
  `bounty_yes_price` / `resolve_bounty` / `resolve_all`) + 5 legacy
  market-related kernel tests + `KernelError::Market` /
  `MarketNotFound` / `MarketExists` variants + `ResolutionResult` —
  DELETED. Kernel restored to V3L-45 pure-topology contract.
- `src/sdk/actor.rs` legacy items (`BoltzmannParams` (f64),
  `is_frontier`, `lineage_score`, legacy `boltzmann_select_parent` (f64)) +
  6 legacy tests — DELETED. `boltzmann_select_parent_v2` (Atom 5;
  integer-rational) is the sole scheduler.
- `src/sdk/snapshot.rs` legacy fields (`MarketSnapshot{f64...}`,
  `UniverseSnapshot.markets` HashMap, `UniverseSnapshot.market_ticker`
  String, dead-since-TB-9 `balances` + `portfolios` + `get_balance` +
  `get_portfolio` impls) — DELETED. All decimal-float surface excised
  under G-14.11.
- `src/bus.rs` `BusConfig.system_lp_amount: f64` + dead
  `BusResult::Invested { shares: f64 }` (F1 follow-up) — DELETED.

## WIRE-SWAPS (production code paths)

- `src/sdk/snapshot.rs`: `UniverseSnapshot` now carries integer-rational
  `price_index: BTreeMap<TxId, NodeMarketEntry>` +
  `mask_set: BTreeSet<TxId>`. Sequencer-optional empty fallback for
  legacy ledger-only mode.
- `src/bus.rs::snapshot()`: rewritten — calls
  `state::compute_price_index(&q.economic_state_t)` +
  `state::compute_mask_set(...)` from live `Sequencer::q_snapshot` when
  the bus is wired with a sequencer (chaintape mode); else empty.
  bus.rs imports TB-14 types here legitimately — halt-trigger #2 fence
  targets `src/state/sequencer.rs` `use` block only; bus.rs is the
  canonical broadcast point.
- `src/bus.rs::init`: removed `HAYEK_BOUNTY` env-gated bounty market open.
- `src/bus.rs::append_internal`: removed per-append `kernel.create_market`
  call.
- `src/bus.rs::halt_and_settle`: no longer calls `kernel.resolve_all`.
- `experiments/minif2f_v4/src/bin/evaluator.rs`: production wire-swap.
  Imports updated; `BoltzmannMaskPolicy::from_env`; `market_ticker_str`
  derived from `snap.price_index` (cross-multiplication argmax sort,
  renders `n/d`, never decimal); `prompt_balance` queried from
  `bus.sequencer.q_snapshot().balances_t` (replaces legacy
  `snap.get_balance`); Boltzmann selector now
  `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set,
  &policy, &mut rng)`.
- `src/bin/audit_dashboard.rs`: ADDITIVE — NEW §14 PriceIndex render
  section. ARCHITECT-MANDATED BANNER: literal "PRICE IS SIGNAL, NOT
  TRUTH" (architect §5.1 verbatim). 4 SG-14.6 unit tests.

## NEW + UPDATED TESTS

- NEW `tests/tb_14_chaintape_smoke.rs`: chain-backed; pattern from
  `tb_13_chaintape_smoke.rs`. Asserts (a) `verify_chaintape` 7/7 GREEN
  post-wire-swap; (b) `replayed_q.economic_state_t == live_q.economic_state_t`
  byte-equal; (c) `compute_price_index(live)` ==
  `compute_price_index(replayed)` byte-equal (FC3-N42 chaintape replay
  determinism for derived view by composition); (d) idempotency across
  5 invocations; (e) empty `node_positions_t` → empty PriceIndex.
- NEW 4 SG-14.6 unit tests in `src/bin/audit_dashboard.rs::tb14_render_tests`.
- NEW `src/kernel.rs::test_trace_golden_path_unknown_node`.
- UPDATED `tests/tb_13_legacy_cpmm_forward_fence.rs::prediction_market_legacy_quarantined`
  rewritten from "label discipline" to "absence discipline" (legacy
  file gone, no fields, no methods, no module declaration).
- UPDATED `tests/fc_alignment_conformance.rs::fc1_n6_input_universe_snapshot_via_bus`
  asserts new `price_index` + `mask_set` fields.
- UPDATED bus.rs internal tests + snapshot.rs internal test.

## Architect-mandated audit questions (CR-14.x conformance + §5.7 halt-triggers)

**Q1 (CR-14.1 / halt-trigger #1)**: Does the price signal influence
predicate gates? `src/state/sequencer.rs` body must contain ZERO
references to TB-14 price/mask types
(`compute_price_index|compute_mask_set|NodeMarketEntry|RationalPrice|BoltzmannMaskPolicy`).
Verify by inspection. Halt-trigger #1 (`tests/tb_14_halt_triggers.rs:27-53`)
is the file-level fence; you do the architectural-soundness check.

**Q2 (CR-14.2 / halt-trigger #2)**: Does the price signal affect L4/L4.E
classification? `src/state/sequencer.rs` `use` block must contain ZERO
TB-14 imports (permanent fence — even after Atom 6's bus.rs snapshot
wire-swap). Verify by inspection.

**Q3 (CR-14.3 / SG-14.3 / halt-trigger #3)**: Are masked parents still in
`tape.nodes()`? `compute_mask_set` MUST only insert into
`BTreeSet<TxId>`; `Tape` MUST never be mutated. Trace
`src/state/price_index.rs:364-429` and `src/bus.rs::snapshot`. Halt-
trigger #3 is functional; you do the production-path soundness check.

**Q4 (CR-14.4 / SG-14.8)**: Low-liquidity children cannot mask parent.
`src/state/price_index.rs:402-407` enforces
`child.liquidity_depth < policy.min_liquidity → continue`. Verify the
threshold rule is right (BoltzmannMaskPolicy::default() has
`min_liquidity = 1 Coin`); challenge if you see edge cases (overflow,
saturation, signed-vs-unsigned).

**Q5 (CR-14.5 / SG-14.7 / halt-trigger #6)**: Open challenges block
masking. `src/state/price_index.rs:377-382, 410-412` indexes
`ChallengeStatus::Open` and skips. Verify the index is correctly built
from `econ.challenge_cases_t` and the skip is unconditional (no
override conditions).

**Q6 (CR-14.6 — Goodhart shield)**: `NodeMarketEntry`
(`src/state/price_index.rs:97-109`) carries 10 fields (node_id,
task_id, event_id, long_interest, short_interest, yes_share_depth,
no_share_depth, price_yes, price_no, liquidity_depth) — NONE expose
predicate content. Dashboard §14 render
(`src/bin/audit_dashboard.rs:1500-1570`) renders only these fields.
Verify no leak path.

**Q7 (G-14.11 / charter §5.6)**: No f64 in TB-14 module surface.
`src/state/price_index.rs` is fenced by halt-trigger #4 runtime fs
read. After F1 fix, verify `src/sdk/snapshot.rs`, `src/bus.rs`,
`src/bin/audit_dashboard.rs` §14 render block, `src/sdk/actor.rs` v2
span are all clean. EXPECTED OUT-OF-SCOPE: `experiments/minif2f_v4/src/bin/evaluator.rs`
`prompt_balance: f64` (this is the prompt.rs render contract;
prompt.rs is NOT a TB-14 module surface — accept this if you agree;
flag if you disagree).

**Q8 (Art.0.2 replay determinism)**:
`tests/tb_14_chaintape_smoke.rs:307-348` claims
`compute_price_index(live) == compute_price_index(replayed)`
byte-equal + idempotent across 5 invocations + empty pre-condition
honest. Verify the assertions actually measure what they claim.

**Q9 (charter §5.6 forbidden)**: No market trading. No price-based
settlement. No parent deletion. No AMM. No DPMM. No price-as-oracle
language. Walk the diff (`git diff a9fbdf3..38412bf`) and confirm
zero introductions of any of these.

## Implementation-paranoid scrutiny (RQ1-RQ8)

**RQ1 — bus.snapshot sequencer-optional empty fallback**: when
`bus.sequencer == None` (legacy WAL-only smoke tests), price_index +
mask_set return empty. Could a downstream consumer (evaluator,
dashboard) misread "empty" as "no positions" when the truth is
"sequencer not wired"? Trace evaluator.rs:1320-1395 zero-tick alarm
and confirm the diagnostic distinguishes (or that the conflation is
benign).

**RQ2 — kernel-purity post-deletion**: `src/kernel.rs` post-Atom-6 has
~140 lines, 5 tests, only `Tape`/`Node`/`NodeId`/`KernelError` in
scope. Verify zero domain strings (lean / tactic / theorem / proof /
mathlib / sorry). V3L-45 contract restored?

**RQ3 — replay determinism via composition**: `compute_price_index` is
a pure function over `EconomicState`. The chaintape smoke verifies
`live.economic_state_t == replayed.economic_state_t` byte-equal (TB-13
already proven for that invariant) and then asserts
`compute_price_index(live) == compute_price_index(replayed)`. Is
this composition argument sound, or are there hidden non-determinism
sources (BTreeMap iteration order, hash randomness, etc.)?

**RQ4 — boltzmann_select_parent_v2 production wire-up**: evaluator.rs
calls `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set,
&policy, &mut boltz_rng).map(|tx| tx.0)`. Is the `.map(|tx| tx.0)`
unwrapping correct (TxId(String) → String)? Does it lose information?
Trace the downstream parent_id consumer.

**RQ5 — evaluator.rs balance plumb-through fix**: pre-Atom-6 the
evaluator passed `snap.get_balance(agent_id)` (legacy f64 fallback).
Atom 6 replaces this with a live
`bus.sequencer.q_snapshot().economic_state_t.balances_t.get(&AgentId)`
query. Could this introduce stale-read problems? The bus is single-
writer (V3L-11 serial reactor) — does that close all race conditions?

**RQ6 — STEP_B Phase 1 deviation**: the commit body declares working
directly on main rather than `.claude/worktrees/stepb-tb14-atom6`.
The author argues Phase 0 (necessity) is satisfied by architect
ratification + Phase 3 (audit gate) preserved. The internal auditor
flagged this as ACCEPT-with-caveat (LOW-MED severity). Take YOUR
position — is this a real review-quality concern, or operational
overhead?

**RQ7 — F1 follow-up commit (38412bf) hygiene**: the dead
`BusResult::Invested { shares: f64 }` deletion was a separate commit
post-44cd480. The G2 single-rehash discipline is satisfied per-commit
(44cd480 rehashed bus.rs once; 38412bf rehashed bus.rs once). Is this
the right hygiene, or should the F1 fix have been amended into 44cd480
to keep Atom 6 as a single atomic commit?

**RQ8 — TB-14 ChainTape smoke coverage gap**: the smoke uses the TB-13
CompleteSet flow only (no NodePosition mutation), so the resulting
PriceIndex is empty. The smoke is honest about this and cross-
references where the non-empty case lives (in-memory unit tests). Is
this acceptable Class 3 coverage, or should there be a non-empty
NodePositions chain-backed smoke (i.e., submit a real WorkTx that
creates a FirstLong NodePosition, then assert non-empty PriceIndex
post-replay)?

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All Q1-Q9 + RQ1-RQ8 cleared; ship is clean.)
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
- <repeat per challenge>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
FIX-THEN-PROCEED / REDESIGN).

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md.


warning: Codex could not find bubblewrap on PATH. Install bubblewrap with your OS package manager. See the sandbox prerequisites: https://developers.openai.com/codex/concepts/sandboxing#prerequisites. Codex will use the vendored bubblewrap in the meantime.
codex
I’ll audit this as a read-only ship gate: first load the repo instructions, then inspect the Atom 6 diff and the specific production paths/tests behind Q1-Q9 and RQ1-RQ8. The workspace is currently read-only, so I can produce the audit content, but I may be blocked from saving the requested markdown file.
exec
/bin/bash -lc 'git rev-parse --short HEAD && git log --oneline -n 8' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
c291dde
c291dde TB-14 Atom 6 LATEST.md update — local-commits-shipped, external dual audit pending
38412bf TB-14 Atom 6 follow-up — close internal auditor F1 (dead BusResult::Invested f64 residual in bus.rs)
44cd480 TB-14 Atom 6 — production wire-swap + legacy CPMM excision (closes OBS_TB_12_LEGACY_CPMM_QUARANTINE)
9cc40e1 TB-14 handover — LATEST.md update + Atom 6 kickoff doc for fresh session
a9fbdf3 TB-14 Atom 5 — boltzmann_select_parent_v2 + halt-triggers #1/#2 — 6/6 GREEN
7cbcacf TB-14 Atom 4 — BoltzmannMaskPolicy::from_env() + 7 env vars
668695d TB-14 Atom 3 — mask_set + compute_mask_set + BoltzmannMaskPolicy skeleton
23ac581 TB-14 Atom 2 — PriceIndex pure-fn view + fence architectural fix

exec
/bin/bash -lc 'git status --short --branch' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
## main...origin/main [ahead 27]
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/README.md
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/agent_pubkeys.json
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/replay_report.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/agent_pubkeys.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/pinned_pubkeys.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/replay_report.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M rules/enforcement.log
?? experiments/minif2f_v4/h_vppu_history.json
?? handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md.raw
?? handover/audits/run_codex_tb_14_ship_audit.sh
?? handover/audits/run_gemini_tb_14_ship_audit.py
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/

exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 110ms:
codex
The checkout is on `main` at `c291dde`, one commit past the stated code target `38412bf`; that later commit appears to be a handover/LATEST update. I’ll anchor code inspection to `a9fbdf3..38412bf` and call out where HEAD metadata differs.
exec
/bin/bash -lc 'ls -la handover/audits' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
total 40772
drwxr-xr-x  2 zephryj zephryj   24576 May  3 20:49 .
drwxr-xr-x 14 zephryj zephryj    4096 May  2 01:31 ..
-rw-r--r--  1 zephryj zephryj   38316 Apr 29 16:42 2026-04-29_external_audit.md
-rw-r--r--  1 zephryj zephryj   38205 Apr 26 09:07 A8_AUDIT_HISTORY_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   15789 Apr 26 09:07 A8_EXIT_PACKET_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   12657 Apr 26 15:12 AUDITOR_TFR_PLAN_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   32350 Apr 28 07:58 AUDIT_LEDGER.md
-rw-r--r--  1 zephryj zephryj    7634 Apr 25 14:27 B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   14721 May  2 15:10 CHARTER_RATIFICATION_TB_10_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   14125 May  2 11:28 CHARTER_RATIFICATION_TB_8_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   13372 May  2 13:19 CHARTER_RATIFICATION_TB_9_2026-05-02.md
-rw-r--r--  1 zephryj zephryj    5620 Apr 27 11:40 CLAUDE_AUDITOR_CO1_7_0AF_KEYPAIR_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    7143 Apr 28 12:16 CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_FINAL_2026-04-28.md
-rw-r--r--  1 zephryj zephryj   11255 Apr 28 11:29 CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    6677 Apr 28 11:59 CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md
-rw-r--r--  1 zephryj zephryj   11950 Apr 29 02:41 CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    9287 Apr 28 07:26 CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    5615 Apr 28 07:56 CO1_7_DUAL_AUDIT_VERDICT_R3_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    9706 Apr 29 03:17 CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    6546 Apr 29 03:34 CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    5159 Apr 29 03:43 CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R4_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    6297 Apr 28 14:14 CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_FINAL_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    7741 Apr 28 14:04 CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  496648 Apr 25 17:07 CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  236036 Apr 25 17:59 CODEX_B7_EXTRA_REAUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  341088 Apr 25 18:10 CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  705580 Apr 25 18:21 CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  150010 Apr 29 05:22 CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj 1340857 Apr 29 05:35 CODEX_CO1_13_ROUND2_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj 1152985 Apr 28 11:26 CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  523686 Apr 28 11:58 CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  264778 Apr 28 12:08 CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  413082 Apr 28 12:11 CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  568226 Apr 28 12:15 CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    4180 Apr 27 11:41 CODEX_CO1_2_QSTATE_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj 2744270 Apr 29 01:03 CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  483375 Apr 29 03:13 CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  431835 Apr 29 03:33 CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  177390 Apr 29 03:42 CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  647022 Apr 28 13:59 CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  187480 Apr 28 14:07 CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  439017 Apr 28 14:12 CODEX_CO1_7_IMPL_BUNDLE_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  392767 Apr 28 07:24 CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  721388 Apr 28 07:45 CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  947675 Apr 28 07:55 CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  362040 Apr 29 11:47 CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   38906 Apr 26 16:45 CODEX_CO_P0_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    3465 Apr 27 12:00 CODEX_INV8_DAG_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   15120 Apr 24 06:34 CODEX_PAPER1_AUDIT_2026-04-23.md
-rw-r--r--  1 zephryj zephryj   17473 Apr 25 03:26 CODEX_PAPER1_V2_1_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   11419 Apr 24 17:57 CODEX_PAPER1_V2_AUDIT_2026-04-24.md
-rw-r--r--  1 zephryj zephryj 1112560 Apr 26 03:05 CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  660680 Apr 26 05:14 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj 1764610 Apr 26 07:20 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md
-rw-r--r--  1 zephryj zephryj  651156 Apr 26 08:01 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md
-rw-r--r--  1 zephryj zephryj  514594 Apr 26 08:51 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md
-rw-r--r--  1 zephryj zephryj 2401751 Apr 26 09:06 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R13.md
-rw-r--r--  1 zephryj zephryj  910608 Apr 26 05:30 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md
-rw-r--r--  1 zephryj zephryj  455324 Apr 26 05:39 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md
-rw-r--r--  1 zephryj zephryj  482745 Apr 26 05:49 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md
-rw-r--r--  1 zephryj zephryj  524409 Apr 26 05:58 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md
-rw-r--r--  1 zephryj zephryj 1574378 Apr 26 06:07 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md
-rw-r--r--  1 zephryj zephryj  639750 Apr 26 06:51 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md
-rw-r--r--  1 zephryj zephryj  595925 Apr 26 07:02 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md
-rw-r--r--  1 zephryj zephryj 1603789 Apr 26 07:11 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md
-rw-r--r--  1 zephryj zephryj  170881 Apr 25 10:59 CODEX_PPUT_CCL_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  286708 Apr 25 11:27 CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  556403 Apr 25 11:38 CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  351502 Apr 25 11:51 CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  674813 Apr 25 14:18 CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   27129 Apr 27 08:06 CODEX_SPEC_FREEZE_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   12423 Apr 27 08:20 CODEX_SPEC_V12_REAUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   10348 Apr 27 09:48 CODEX_SPEC_V13_REAUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    2163 Apr 27 11:07 CODEX_SPEC_V14_ROUND4_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   18146 May  1 10:31 CODEX_TB6_FULLDIFF_AUDIT_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   15264 May  2 03:57 CODEX_TB7R_MICRO_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   19450 May  1 13:16 CODEX_TB7_FULLDIFF_AUDIT_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   11709 May  3 03:09 CODEX_TB_12_SHIP_AUDIT_2026-05-03.md
-rw-r--r--  1 zephryj zephryj  562967 May  3 07:08 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md
-rw-r--r--  1 zephryj zephryj  441401 May  3 07:14 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md
-rw-r--r--  1 zephryj zephryj 2853081 May  3 07:43 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md
-rw-r--r--  1 zephryj zephryj 2543592 May  3 09:17 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md
-rw-r--r--  1 zephryj zephryj 1466195 May  3 09:46 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md
-rw-r--r--  1 zephryj zephryj  116326 May  3 10:06 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md
-rw-r--r--  1 zephryj zephryj   18037 May  3 20:49 CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md.raw
-rw-r--r--  1 zephryj zephryj  440063 Apr 29 18:28 CODEX_TB_1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  138233 Apr 29 19:09 CODEX_TB_1_PATH_A_PP_MICROAUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   19808 Apr 30 04:06 CODEX_TB_2_PHASE0_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   17319 Apr 30 04:36 CODEX_TB_2_PHASE0_R2_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   12327 Apr 30 06:43 CODEX_TB_2_PHASE1C_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    2505 Apr 30 06:43 CODEX_TB_2_PHASE1C_R2_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   21484 Apr 30 15:08 CODEX_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   26538 Apr 30 17:15 CODEX_TB_5_PHASE0_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   11819 Apr 30 17:33 CODEX_TB_5_PHASE0_R3_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    5424 Apr 30 19:33 CODEX_TB_5_PHASE0_R4_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    9754 May  2 07:00 CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj  221263 May  2 07:23 CODEX_TB_7R_SHIP_AUDIT_R2_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   12143 May  2 12:26 CODEX_TB_8_SHIP_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj    6579 May  2 12:49 CODEX_TB_8_SHIP_AUDIT_R2_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   33447 Apr 26 15:14 CODEX_TFR_PLAN_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   24673 Apr 27 00:36 CODEX_T_S_REVIEW_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   13619 Apr 27 15:09 CODEX_WHITEPAPER_V2_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   10094 Apr 27 15:53 CODEX_WHITEPAPER_V2_AUDIT_2026-04-27_R2.md
-rw-r--r--  1 zephryj zephryj    1365 Apr 27 17:20 CODEX_WHITEPAPER_V2_AUDIT_2026-04-27_R3.md
-rw-r--r--  1 zephryj zephryj   11204 Apr 22 08:25 CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    8506 Apr 24 06:38 DUAL_AUDIT_PAPER1_VERDICT_2026-04-23.md
-rw-r--r--  1 zephryj zephryj   11048 Apr 25 11:02 DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    5620 Apr 25 11:52 DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   10063 Apr 29 18:30 DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   12368 Apr 30 04:08 DUAL_AUDIT_TB_2_PHASE0_VERDICT_R1_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    7565 Apr 30 06:43 DUAL_AUDIT_TB_2_PHASE1C_VERDICT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   11925 Apr 30 16:06 DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    5206 Apr 25 03:28 DUAL_AUDIT_V2_1_VERDICT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj    5074 Apr 24 18:03 DUAL_AUDIT_V2_VERDICT_2026-04-24.md
-rw-r--r--  1 zephryj zephryj   32689 Apr 30 14:47 EXTERNAL_AUDIT_PROMPT_TB_4_SHIP_TB_5_CHARTER_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   17270 Apr 22 00:15 EXT_CODEX_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    5147 Apr 22 14:00 EXT_CODEX_PHASE_2_AB_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    2428 Apr 22 09:38 EXT_CODEX_PHASE_8A_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    5302 Apr 22 10:34 EXT_CODEX_PHASE_8_BATCH_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    4081 Apr 22 12:03 EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md
-rw-r--r--  1 zephryj zephryj   37108 Apr 22 00:05 EXT_DEEPSEEK_2026-04-21.md
-rw-r--r--  1 zephryj zephryj   16843 Apr 22 00:03 EXT_GEMINI_2026-04-21.md
-rw-r--r--  1 zephryj zephryj    1704 Apr 22 13:54 EXT_GEMINI_PHASE_2_AB_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    7445 Apr 22 09:26 EXT_GEMINI_PHASE_8A_2026-04-22.md
-rw-r--r--  1 zephryj zephryj   10510 Apr 22 10:20 EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    5124 Apr 22 11:49 EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    2353 Apr 22 11:12 EXT_GEMINI_PHASE_8_R1R8_2026-04-22.md
-rw-r--r--  1 zephryj zephryj   14317 Apr 25 17:03 GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   10757 Apr 25 17:55 GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj    5921 Apr 25 18:07 GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj    8503 Apr 25 18:16 GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj    6466 Apr 29 05:20 GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    5134 Apr 29 05:33 GEMINI_CO1_13_ROUND2_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   15223 Apr 28 11:23 GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    8757 Apr 28 11:55 GEMINI_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    5452 Apr 28 12:06 GEMINI_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj   13570 Apr 29 00:59 GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   13262 Apr 29 03:09 GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   12117 Apr 29 03:30 GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    3073 Apr 29 03:40 GEMINI_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   10903 Apr 28 13:55 GEMINI_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj   17156 Apr 28 07:21 GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    7047 Apr 28 07:42 GEMINI_CO1_7_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    3903 Apr 28 07:53 GEMINI_CO1_7_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    5818 Apr 29 11:46 GEMINI_CO1_8_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   10124 Apr 26 16:39 GEMINI_CO_P0_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    6782 Apr 27 11:59 GEMINI_INV8_DAG_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   12066 Apr 24 06:37 GEMINI_PAPER1_AUDIT_2026-04-23.md
-rw-r--r--  1 zephryj zephryj    5445 Apr 25 03:17 GEMINI_PAPER1_V2_1_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   12276 Apr 24 17:54 GEMINI_PAPER1_V2_AUDIT_2026-04-24.md
-rw-r--r--  1 zephryj zephryj    9407 Apr 26 03:01 GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj    7429 Apr 26 05:10 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    7833 Apr 26 07:15 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md
-rw-r--r--  1 zephryj zephryj    3713 Apr 26 07:56 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md
-rw-r--r--  1 zephryj zephryj    2889 Apr 26 08:48 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md
-rw-r--r--  1 zephryj zephryj    8117 Apr 26 09:03 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R13.md
-rw-r--r--  1 zephryj zephryj    4942 Apr 26 05:26 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md
-rw-r--r--  1 zephryj zephryj    5714 Apr 26 05:36 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md
-rw-r--r--  1 zephryj zephryj    6017 Apr 26 05:46 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md
-rw-r--r--  1 zephryj zephryj    3369 Apr 26 05:54 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md
-rw-r--r--  1 zephryj zephryj    3748 Apr 26 06:03 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md
-rw-r--r--  1 zephryj zephryj    4832 Apr 26 06:48 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md
-rw-r--r--  1 zephryj zephryj    3776 Apr 26 06:59 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md
-rw-r--r--  1 zephryj zephryj    7508 Apr 26 07:07 GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md
-rw-r--r--  1 zephryj zephryj    7407 Apr 25 10:57 GEMINI_PPUT_CCL_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    2090 Apr 25 11:26 GEMINI_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    5390 Apr 25 11:34 GEMINI_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    6260 Apr 25 11:48 GEMINI_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   12339 Apr 25 14:14 GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   13090 Apr 27 07:57 GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj     336 Apr 27 08:15 GEMINI_SPEC_V12_REAUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    5591 Apr 27 11:04 GEMINI_SPEC_V14_ROUND4_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    9451 May  3 03:13 GEMINI_TB_12_SHIP_AUDIT_2026-05-03_R1.md
-rw-r--r--  1 zephryj zephryj   13340 May  3 06:46 GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R1.md
-rw-r--r--  1 zephryj zephryj   10980 May  3 07:09 GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2_PRE_R3.md
-rw-r--r--  1 zephryj zephryj   10179 May  3 07:39 GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R3.md
-rw-r--r--  1 zephryj zephryj   10824 May  3 09:10 GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R4.md
-rw-r--r--  1 zephryj zephryj   10898 May  3 09:40 GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R5.md
-rw-r--r--  1 zephryj zephryj    9176 May  3 10:03 GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R6.md
-rw-r--r--  1 zephryj zephryj    6332 Apr 29 18:24 GEMINI_TB_1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    7779 Apr 30 03:57 GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    5902 Apr 30 06:13 GEMINI_TB_2_PHASE1C_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   11801 Apr 30 15:22 GEMINI_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    8704 May  2 06:49 GEMINI_TB_7R_SHIP_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj    3415 May  2 12:19 GEMINI_TB_8_SHIP_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   17139 Apr 27 01:00 GEMINI_V32_REVIEW_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    8696 Apr 27 15:05 GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    8805 Apr 27 15:48 GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27_R2.md
-rw-r--r--  1 zephryj zephryj    9444 Apr 27 17:19 GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27_R3.md
-rw-r--r--  1 zephryj zephryj   10991 Apr 27 04:13 GEMINI_WP_REVISION_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   14942 Apr 22 08:21 HARNESS_COMPRESSION_AUDIT_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    5184 May  2 01:03 L4_PURITY_AUDIT_TB7R_2026-05-02.md
-rw-r--r--  1 zephryj zephryj    2214 Apr 22 08:43 PPUT_HISTORICAL_AUDIT_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    8065 Apr 22 08:41 PPUT_RAW_DATA_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    6366 Apr 22 08:42 PPUT_REFRAME_2026-04-22.md
-rw-r--r--  1 zephryj zephryj   26598 May  2 15:45 RECURSIVE_AUDIT_TB_10_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   14498 May  2 19:37 RECURSIVE_AUDIT_TB_11_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   21973 May  3 03:28 RECURSIVE_AUDIT_TB_12_2026-05-03.md
-rw-r--r--  1 zephryj zephryj   36725 May  3 07:48 RECURSIVE_AUDIT_TB_13_2026-05-03.md
-rw-r--r--  1 zephryj zephryj   15501 Apr 30 10:05 RECURSIVE_AUDIT_TB_3_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   19743 Apr 30 11:53 RECURSIVE_AUDIT_TB_4_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   15802 May  1 02:44 RECURSIVE_AUDIT_TB_5_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   19062 May  1 08:08 RECURSIVE_AUDIT_TB_6_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   21084 May  2 07:26 RECURSIVE_AUDIT_TB_7R_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   15714 May  1 13:35 RECURSIVE_AUDIT_TB_7_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   14970 May  2 12:50 RECURSIVE_AUDIT_TB_8_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   15858 May  2 14:14 RECURSIVE_AUDIT_TB_9_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   10499 May  1 02:42 SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   12246 May  1 02:43 STAGE_AUDIT_TB_1_TO_TB_5_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   27391 Apr 29 18:43 STATUS_REPORT_TO_ARCHITECT_2026-04-29_overnight.md
-rw-r--r--  1 zephryj zephryj   10274 May  2 11:20 STEP_B_PREFLIGHT_TB8_2026-05-02.md
-rw-r--r--  1 zephryj zephryj    6016 May  2 13:20 STEP_B_PREFLIGHT_TB9_ATOM1_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   14963 Apr 22 08:43 SYNTHESIS_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    6121 May  2 01:30 TB7R_DELIVERABLE_D_VERIFICATION_2026-05-02.md
-rwxr-xr-x  1 zephryj zephryj    1664 Apr 22 08:41 pput_scan.py
-rwxr-xr-x  1 zephryj zephryj   18185 Apr 25 16:59 run_codex_b7_extra_prebatch_audit.sh
-rwxr-xr-x  1 zephryj zephryj   13679 Apr 25 17:53 run_codex_b7_extra_reaudit.sh
-rwxr-xr-x  1 zephryj zephryj   15067 Apr 25 18:06 run_codex_b7_extra_round3_audit.sh
-rwxr-xr-x  1 zephryj zephryj   15020 Apr 25 18:15 run_codex_b7_extra_round4_audit.sh
-rwxr-xr-x  1 zephryj zephryj    9729 Apr 29 05:17 run_codex_co1_13_round1_audit.sh
-rwxr-xr-x  1 zephryj zephryj    7938 Apr 29 05:31 run_codex_co1_13_round2_audit.sh
-rwxr-xr-x  1 zephryj zephryj   12453 Apr 28 11:20 run_codex_co1_1_4_pre1_round1_audit.sh
-rwxr-xr-x  1 zephryj zephryj   10217 Apr 28 11:53 run_codex_co1_1_4_pre1_round2_audit.sh
-rwxr-xr-x  1 zephryj zephryj    6231 Apr 28 12:05 run_codex_co1_1_4_pre1_round3_audit.sh
-rwxr-xr-x  1 zephryj zephryj    3828 Apr 28 12:10 run_codex_co1_1_4_pre1_round4_audit.sh
-rwxr-xr-x  1 zephryj zephryj    3881 Apr 28 12:14 run_codex_co1_1_4_pre1_round5_audit.sh
-rwxr-xr-x  1 zephryj zephryj   11732 Apr 29 00:56 run_codex_co1_7_5_round1_audit.sh
-rwxr-xr-x  1 zephryj zephryj   10943 Apr 29 03:07 run_codex_co1_7_extra_round2_audit.sh
-rwxr-xr-x  1 zephryj zephryj    8614 Apr 29 03:27 run_codex_co1_7_extra_round3_audit.sh
-rwxr-xr-x  1 zephryj zephryj    5496 Apr 29 03:39 run_codex_co1_7_extra_round4_audit.sh
-rwxr-xr-x  1 zephryj zephryj   10097 Apr 28 13:53 run_codex_co1_7_impl_bundle_round1_audit.sh
-rwxr-xr-x  1 zephryj zephryj    4534 Apr 28 14:05 run_codex_co1_7_impl_bundle_round2_audit.sh
-rwxr-xr-x  1 zephryj zephryj    2729 Apr 28 14:10 run_codex_co1_7_impl_bundle_round3_audit.sh
-rwxr-xr-x  1 zephryj zephryj    8934 Apr 28 07:19 run_codex_co1_7_round1_audit.sh
-rwxr-xr-x  1 zephryj zephryj   10983 Apr 28 07:40 run_codex_co1_7_round2_audit.sh
-rwxr-xr-x  1 zephryj zephryj    6595 Apr 28 07:52 run_codex_co1_7_round3_audit.sh
-rwxr-xr-x  1 zephryj zephryj   11429 Apr 29 04:49 run_codex_co1_8_round1_audit.sh
-rwxr-xr-x  1 zephryj zephryj   10072 Apr 26 02:59 run_codex_phase_a0_exit_audit.sh
-rwxr-xr-x  1 zephryj zephryj    5197 Apr 26 08:53 run_codex_phase_a8_exit_audit.sh
-rwxr-xr-x  1 zephryj zephryj    6992 Apr 25 10:56 run_codex_pput_ccl_audit.sh
-rwxr-xr-x  1 zephryj zephryj    7131 Apr 25 11:26 run_codex_pput_ccl_audit_round2.sh
-rwxr-xr-x  1 zephryj zephryj    7259 Apr 25 11:34 run_codex_pput_ccl_audit_round3.sh
-rwxr-xr-x  1 zephryj zephryj    5849 Apr 25 11:48 run_codex_pput_ccl_audit_round4.sh
-rwxr-xr-x  1 zephryj zephryj   11679 Apr 25 14:12 run_codex_pput_ccl_b2_b4_audit.sh
-rwxr-xr-x  1 zephryj zephryj   10920 May  3 06:47 run_codex_tb_13_ship_audit.sh
-rwxr-xr-x  1 zephryj zephryj   12023 May  3 09:08 run_codex_tb_13_ship_audit_r4.sh
-rwxr-xr-x  1 zephryj zephryj    8585 May  3 09:39 run_codex_tb_13_ship_audit_r5.sh
-rwxr-xr-x  1 zephryj zephryj    5883 May  3 10:02 run_codex_tb_13_ship_audit_r6.sh
-rwxr-xr-x  1 zephryj zephryj   14656 May  3 20:47 run_codex_tb_14_ship_audit.sh
-rwxr-xr-x  1 zephryj zephryj   13082 Apr 29 18:21 run_codex_tb_1_audit_2026-04-29.sh
-rwxr-xr-x  1 zephryj zephryj    9197 Apr 29 19:07 run_codex_tb_1_path_a_pp_microaudit.sh
-rwxr-xr-x  1 zephryj zephryj    9459 May  2 07:16 run_codex_tb_7r_round_2_audit_2026-05-02.sh
-rwxr-xr-x  1 zephryj zephryj   18215 May  2 06:45 run_codex_tb_7r_ship_audit_2026-05-02.sh
-rw-r--r--  1 zephryj zephryj    6161 May  2 12:36 run_codex_tb_8_round_2_audit_2026-05-02.sh
-rwxr-xr-x  1 zephryj zephryj   10523 May  2 11:52 run_codex_tb_8_ship_audit_2026-05-02.sh
-rwxr-xr-x  1 zephryj zephryj   15579 Apr 25 17:01 run_gemini_b7_extra_prebatch_audit.py
-rwxr-xr-x  1 zephryj zephryj   13711 Apr 25 17:54 run_gemini_b7_extra_reaudit.py
-rwxr-xr-x  1 zephryj zephryj   15109 Apr 25 18:06 run_gemini_b7_extra_round3_audit.py
-rwxr-xr-x  1 zephryj zephryj   14559 Apr 25 18:15 run_gemini_b7_extra_round4_audit.py
-rwxr-xr-x  1 zephryj zephryj   10808 Apr 29 05:19 run_gemini_co1_13_round1_audit.py
-rwxr-xr-x  1 zephryj zephryj    7989 Apr 29 05:32 run_gemini_co1_13_round2_audit.py
-rwxr-xr-x  1 zephryj zephryj   12775 Apr 28 11:21 run_gemini_co1_1_4_pre1_round1_audit.py
-rwxr-xr-x  1 zephryj zephryj   10179 Apr 28 11:54 run_gemini_co1_1_4_pre1_round2_audit.py
-rwxr-xr-x  1 zephryj zephryj    7391 Apr 28 12:05 run_gemini_co1_1_4_pre1_round3_audit.py
-rw-r--r--  1 zephryj zephryj   11919 Apr 29 00:58 run_gemini_co1_7_5_round1_audit.py
-rw-r--r--  1 zephryj zephryj   12424 Apr 29 03:08 run_gemini_co1_7_extra_round2_audit.py
-rw-r--r--  1 zephryj zephryj   10481 Apr 29 03:29 run_gemini_co1_7_extra_round3_audit.py
-rw-r--r--  1 zephryj zephryj    5451 Apr 29 03:39 run_gemini_co1_7_extra_round4_audit.py
-rwxr-xr-x  1 zephryj zephryj    8016 Apr 28 13:54 run_gemini_co1_7_impl_bundle_round1_audit.py
-rwxr-xr-x  1 zephryj zephryj    9664 Apr 28 07:20 run_gemini_co1_7_round1_audit.py
-rwxr-xr-x  1 zephryj zephryj   11155 Apr 28 07:42 run_gemini_co1_7_round2_audit.py
-rwxr-xr-x  1 zephryj zephryj    6054 Apr 28 07:52 run_gemini_co1_7_round3_audit.py
-rwxr-xr-x  1 zephryj zephryj   11693 Apr 29 11:46 run_gemini_co1_8_round1_audit.py
-rwxr-xr-x  1 zephryj zephryj    6693 Apr 26 16:36 run_gemini_co_p0_audit.py
-rwxr-xr-x  1 zephryj zephryj    6716 Apr 26 16:35 run_gemini_co_p0_audit.sh
-rw-r--r--  1 zephryj zephryj    3997 Apr 27 11:58 run_gemini_inv8_audit.py
-rw-r--r--  1 zephryj zephryj    3936 Apr 24 06:36 run_gemini_paper1_audit.py
-rw-r--r--  1 zephryj zephryj    7401 Apr 25 03:16 run_gemini_paper1_v2_1_audit.py
-rw-r--r--  1 zephryj zephryj    4737 Apr 24 15:24 run_gemini_paper1_v2_audit.py
-rwxr-xr-x  1 zephryj zephryj    7548 Apr 26 03:00 run_gemini_phase_a0_exit_audit.py
-rwxr-xr-x  1 zephryj zephryj    6298 Apr 26 08:53 run_gemini_phase_a8_exit_audit.py
-rwxr-xr-x  1 zephryj zephryj   10243 Apr 25 10:54 run_gemini_pput_ccl_audit.py
-rwxr-xr-x  1 zephryj zephryj    8807 Apr 25 11:25 run_gemini_pput_ccl_audit_round2.py
-rwxr-xr-x  1 zephryj zephryj    5974 Apr 25 11:33 run_gemini_pput_ccl_audit_round3.py
-rwxr-xr-x  1 zephryj zephryj    6060 Apr 25 11:47 run_gemini_pput_ccl_audit_round4.py
-rwxr-xr-x  1 zephryj zephryj   13450 Apr 25 14:13 run_gemini_pput_ccl_b2_b4_audit.py
-rwxr-xr-x  1 zephryj zephryj    7176 Apr 27 07:55 run_gemini_spec_freeze_audit.py
-rwxr-xr-x  1 zephryj zephryj    5788 Apr 27 08:14 run_gemini_spec_v12_reaudit.py
-rw-r--r--  1 zephryj zephryj    5755 Apr 27 11:03 run_gemini_spec_v14_round4_audit.py
-rwxr-xr-x  1 zephryj zephryj    6816 May  3 03:13 run_gemini_tb_12_ship_audit.py
-rwxr-xr-x  1 zephryj zephryj    8462 May  3 06:45 run_gemini_tb_13_ship_audit.py
-rwxr-xr-x  1 zephryj zephryj   11477 May  3 09:09 run_gemini_tb_13_ship_audit_r4.py
-rwxr-xr-x  1 zephryj zephryj   13746 May  3 09:38 run_gemini_tb_13_ship_audit_r5.py
-rwxr-xr-x  1 zephryj zephryj   14106 May  3 10:00 run_gemini_tb_13_ship_audit_r6.py
-rwxr-xr-x  1 zephryj zephryj   12016 May  3 20:49 run_gemini_tb_14_ship_audit.py
-rwxr-xr-x  1 zephryj zephryj   15353 Apr 29 18:22 run_gemini_tb_1_audit_2026-04-29.py
-rw-r--r--  1 zephryj zephryj   17000 Apr 30 03:55 run_gemini_tb_2_phase0_audit_2026-04-30.py
-rw-r--r--  1 zephryj zephryj   13644 Apr 30 06:12 run_gemini_tb_2_phase1c_audit_2026-04-30.py
-rwxr-xr-x  1 zephryj zephryj   20114 May  2 06:48 run_gemini_tb_7r_ship_audit_2026-05-02.py
-rwxr-xr-x  1 zephryj zephryj    9437 May  2 11:53 run_gemini_tb_8_ship_audit_2026-05-02.py
-rwxr-xr-x  1 zephryj zephryj    7802 Apr 27 00:58 run_gemini_v32_review.py
-rw-r--r--  1 zephryj zephryj    6934 Apr 27 15:05 run_gemini_whitepaper_v2_audit.py
-rwxr-xr-x  1 zephryj zephryj    7586 Apr 27 04:12 run_gemini_wp_revision_audit.py

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'git diff --stat a9fbdf3..38412bf' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 experiments/minif2f_v4/src/bin/evaluator.rs        | 111 +++++-
 genesis_payload.toml                               |  12 +-
 handover/ai-direct/LATEST.md                       | 113 ++++++
 .../ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md   | 165 ++++++++
 .../tb_14_chaintape_smoke_2026-05-03/README.md     |  31 ++
 .../agent_pubkeys.json                             |   5 +
 .../genesis_report.json                            | 270 +++++++++++++
 .../pinned_pubkeys.json                            |  11 +
 .../replay_report.json                             |  21 +
 src/bin/audit_dashboard.rs                         | 256 ++++++++++++-
 src/bus.rs                                         | 180 +++++----
 src/kernel.rs                                      | 256 +------------
 src/lib.rs                                         |   1 -
 src/prediction_market.rs                           | 390 -------------------
 src/sdk/actor.rs                                   | 275 +------------
 src/sdk/snapshot.rs                                |  78 ++--
 tests/fc_alignment_conformance.rs                  |  19 +-
 tests/tb_13_legacy_cpmm_forward_fence.rs           | 103 ++---
 tests/tb_14_chaintape_smoke.rs                     | 424 +++++++++++++++++++++
 tests/wal_resume.rs                                |   1 -
 20 files changed, 1638 insertions(+), 1084 deletions(-)

 succeeded in 8598ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

2026-05-03T20:49:55.718328Z ERROR codex_core::tools::router: error=write_stdin failed: stdin is closed for this session; rerun exec_command with tty=true to keep stdin open
exec
/bin/bash -lc 'git diff --name-status 38412bf..c291dde' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
M	handover/ai-direct/LATEST.md

exec
/bin/bash -lc "sed -n '1,220p' handover/tracer_bullets/TB-14_charter_2026-05-03.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TB-14 Charter — PriceIndex v0 + Boltzmann Masking

**Status**: RATIFIED 2026-05-03 (user-architect approved plan; AI-coder execution authorized).
**Date**: 2026-05-03.
**Predecessor**: TB-13 SHIPPED 2026-05-03 (`d0094d8`).
**Authority**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §5 (operative spec) + `handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md` §2-4 (binding Boltzmann constitutional alignment).
**Phase**: **P4 Information Loom** (primary — price-as-signal layer, Boltzmann scheduler mask) + **P3 RSP-6** (price index micro-version per 2026-05-02 directive Part C §9.6).
**Risk class envelope**: **Class 2** (Atoms 0-5 = Class 2: type definitions, price derivation, mask computation, integer Boltzmann rewrite). **Atom 6 = Class 3** (production wire-swap: evaluator.rs parent-selection path + legacy CPMM excision per OBS_TB_12_LEGACY_CPMM_QUARANTINE; dual Codex+Gemini audit mandatory).
**Iteration cap**: 24h per Class 2 atom; **72h for Atom 6** (production wire-up exception per `feedback_iteration_cap_24h`).

**Operating mode**: Atoms 0–5 continuous (AI-coder-decided micro-checkpoints). CP-A after Atom 2 (`cargo test --workspace` baseline preserved; no f64 in price_index.rs). CP-B after Atom 4 (mask_set + BoltzmannMaskPolicy complete; all halt-triggers except #1/#2 green). CP-C after Atom 5 (integer Boltzmann v2 complete; all 6 halt-triggers green). Atom 6 dual audit continuous; **STOP after audit verdict**. Atom 7 SHIP per plan.

**FC-trace**: `Art.0.2` (Tape Canonical — `compute_price_index` + `compute_mask_set` are deterministic pure functions over canonical `EconomicState`; no env input; replay-identical) + `Art.I.1` (5-step compile loop — PriceIndex influences parent selection, DOES NOT affect predicate outcome; CR-14.1 explicit) + `Art.III.2` (catalog-not-encyclopedia — PriceIndex is a derived view, not a new canonical store; one `compute_price_index` call per round) + `Art.III.3` (heterogeneity preservation — epsilon exploration maintains agent diversity across rounds) + `Art.III.4` (Goodhart shield — private predicate content not exposed in NodeMarketEntry; mask cannot expose predicate logic to agents).

**Flowchart-trace**: **Flowchart 1** (runtime — `compute_price_index(econ)` iterates `node_positions_t`, groups by `node_id`, derives `long_interest` / `short_interest` / `price_yes` / `price_no` / `liquidity_depth` per FR-14.1..3; `compute_mask_set(econ, policy, price_index)` marks parents of dominant-child nodes in `BTreeSet<TxId>`; `boltzmann_select_parent_v2` uses mask_set to filter frontier candidates + epsilon-greedy for exploration) + **Flowchart 2** (boot — `AgentVisibleProjection.mask_set` is `serde(default)` empty; backward-compatible with TB-13 chain snapshots) + **Flowchart 3** (meta — PriceIndex is TB-14 read-view substrate; TB-15 Autopsy will compress per-round price history into MarkovEvidenceCapsule; TB-16 Controlled Arena will use price-vs-outcome correlation for controlled experiments; TB-14 lays signal layer only).

**Phase declarations** (per `feedback_tb_phase_tag_required`):

```text
phase_id: P4 Information Loom (primary)
          + P3 RSP-6 (price index micro-version)

roadmap_exit_criteria_addressed:
  P4-Exit5  Price signal influences task priority (Boltzmann mask changes
            scheduler read-view) but DOES NOT override predicate; CR-14.1
            + halt-trigger #1 + halt-trigger #2 verified by test.
  P4-Exit6  YES/NO risk price signals risk without deciding fact-truth;
            price_yes=None for zero-liquidity nodes (FR-14.3 + SG-14.2);
            dashboard renders "PRICE IS SIGNAL, NOT TRUTH" (SG-14.6).
  P4-Exit7  Goodhart-sensitive private predicates NOT exposed in
            NodeMarketEntry fields (CR-14.6); private predicate content
            hidden behind ChainTape content-address (CAS), not surfaced
            in derived price view.
  P3:RSP-6  Price index computes YES/NO probabilities from NodePosition
            long/short interest (FR-14.1 + FR-14.2); read-only broadcast
            signal (FR-14.4); no trading, no AMM, no DPMM.

kill_criteria_tested:
  P4-Forbidden:price-replaces-predicate
            halt-trigger #1: price_does_not_affect_predicate_result —
            high price_yes + predicate acceptance.value=false → L4 still
            returns AcceptancePredicateFailed (not accepted). MUST PASS.
  P4-Forbidden:price-changes-L4-decision
            halt-trigger #2: price_does_not_change_l4_decision —
            sequencer.rs:522-527 predicate gate unchanged; price_index_t
            never read in dispatch_transition. MUST PASS.
  P3-Kill:agent-self-report-bonus
            No agent can self-report a price to inflate their own mask
            outcome. PriceIndex derived sequencer-side from canonical
            NodePositionsIndex (agent-visible stake ≠ agent-self-report).
  halt-trigger #3  parent_not_deleted_from_chaintape — tape.nodes() full
            iteration finds masked parent. MUST PASS.
  halt-trigger #4  no_f64_in_tb_14_modules — grep/AST fence on TB-14 code
            surface. MUST PASS.
  halt-trigger #5  zero_liquidity_returns_none — compute_price_index returns
            price_yes=None when long+short==0. MUST PASS.
  halt-trigger #6  unresolved_challenge_blocks_masking — compute_mask_set
            skips child with Open ChallengeCase. MUST PASS.

flowchart_trace:
  Flowchart 3 (meta):   FC3-N42 = compute_price_index (price-derivation
                         pure fn; iterates node_positions_t + share
                         balances; deterministic replay).
  Flowchart 2 (signal): FC2-N28 = mask_set field in AgentVisibleProjection
                         (mask publication; global per-round; serde-default
                         empty for backward-compat).
  Flowchart 2 (sched):  FC2-N29 = boltzmann_select_parent_v2 (scheduler
                         honors mask_set; epsilon exploration; integer-
                         rational argmax; no f64).
```

---

## §0 Why TB-14 exists (architect post-TB-12 ruling §5.1)

**Architect spec verbatim** (lossless archive from `directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §5.1):

```text
TB-14 — PriceIndex v0 + Boltzmann Masking

将 TB-12 的 NodePosition 和 TB-13 的 share/collateral state 转化为价格信号，
并用于 read-view / scheduler masking。

核心原则：
  Price is signal, not truth.

这与宪法中的信号管理一致。宪法把顶层白盒的工作定义为量化、广播、屏蔽，
并把统计信号用于衡量相对有效性；但布尔谓词仍然确立绝对边界。
```

---

## §1 One-line goal

```text
Goal: Derive YES/NO price signals from NodePositionsIndex long/short
      interest (TB-12 substrate) as a read-only statistical broadcast
      (PriceIndex); use a BoltzmannMaskPolicy to filter the scheduler's
      frontier view to parent attempt-nodes dominated by higher-priced
      children, with epsilon-greedy exploration. NO market trading, NO
      price-based settlement, NO f64, NO parent deletion, NO Goodhart
      leak, NO AMM, NO DPMM. Replace legacy f64 CPMM in src/kernel.rs +
      src/prediction_market.rs + src/sdk/actor.rs with integer-rational
      equivalents. Atom 6 closes OBS_TB_12_LEGACY_CPMM_QUARANTINE.

Key objects (architect §5.2 verbatim):
  RationalPrice { numerator: u128, denominator: u128 }
  NodeMarketEntry { node_id, task_id, event_id, long_interest,
    short_interest, yes_share_depth, no_share_depth, price_yes,
    price_no, liquidity_depth }
  BoltzmannMaskPolicy { beta_num, beta_den, min_liquidity,
    price_margin: RationalPrice, epsilon_exploration_num/den }
```

---

## §2 What's already shipped

| Foundation | Source | Status |
|---|---|---|
| `NodePositionsIndex(BTreeMap<TxId, NodePosition>)` (FLAT) | `src/state/q_state.rs:524, 192` | TB-12 |
| `NodePosition { node_id, task_id, owner, side: PositionSide, amount: MicroCoin, ... }` | `src/state/typed_tx.rs:680-690` | TB-12 |
| `PositionSide { Long, Short }` | `src/state/typed_tx.rs:609` | TB-12 |
| `ConditionalCollateralIndex(BTreeMap<EventId, MicroCoin>)` | `src/state/q_state.rs:213, 549` | TB-13 |
| `ConditionalShareBalances(BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair{yes,no: ShareAmount}>>)` | `src/state/q_state.rs:225, 566, 577` | TB-13 |
| `ShareAmount { units: u128 }` | `src/state/typed_tx.rs:1105` | TB-13 |
| `EventId(pub TaskId)` | `src/state/typed_tx.rs:1080` | TB-13 |
| `ChallengeCasesIndex(BTreeMap<TxId, ChallengeCase{status: Open/Released/UpheldDeferred, target_work_tx}>)` | `src/state/q_state.rs:621-677` | TB-8 |
| `MicroCoin(i64)`, `TxId(String)`, `TaskId(String)` | `src/economy/money.rs:34`, `src/state/q_state.rs:71,84` | TB-3 |
| L4 acceptance gate (must NOT regress) | `src/state/sequencer.rs:516-558` | TB-3 |
| `AgentVisibleProjection { views }` stub | `src/state/q_state.rs:121-123, 718` | TB-7R |
| `Tape { nodes, reverse_citations, time_arrow }` in-memory DAG | `src/ledger.rs:30-136` | TB-6 |
| `audit_dashboard.rs` §1-§13 | `src/bin/audit_dashboard.rs` | TB-13 |
| `tests/tb_13_legacy_cpmm_forward_fence.rs` (TB-14 inherits fence) | `tests/tb_13_legacy_cpmm_forward_fence.rs` | TB-13 |
| **LEGACY to replace**: `PriceIndex(BTreeMap<TxId, MicroCoin>)` | `src/state/q_state.rs:699, 167` | TB-3 |
| **LEGACY to delete**: `src/prediction_market.rs`, `kernel.markets/bounty_market/bounty_yes_price` | `src/kernel.rs:24-43, 85, 192`, `src/prediction_market.rs` | legacy |
| **LEGACY to replace**: `BoltzmannParams { temperature: f64, ... }`, `boltzmann_select_parent` | `src/sdk/actor.rs:13-31, 101-147` | legacy |

**TB-13 carry-forward decisions (codified, not open)** (charter §8 for record):
- `ShareAmount.units: u128` — **KEEP u128**. Domain: non-negative quantity; architect's own TB-14 `RationalPrice { numerator: u128, denominator: u128 }` uses u128 (consistent convention). Not a deviation; TB-13 charter §2 explicitly documented the u128 rationale at time of writing.
- `ResolutionRef` removed in TB-13 round-5 — **KEEP REMOVED**. `ResolutionsIndex` deferred to TB-15 (`OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md`); forward reference would dangle. Both codified as correct staging decisions.

---

## §3 Deliverables (7 atoms)

### Atom 0 — TB-14 charter (this document) (Class 0)
**Action**: this document IS the charter. User-architect ratification received 2026-05-03.
**Iter cap**: 0h (already complete).

### Atom 1 — Halt-trigger fixture file (Class 2, 24h)
**NEW**: `tests/tb_14_halt_triggers.rs` — 6 `#[test]` per architect §5.7 halt triggers. All stub via `unimplemented!()` until later atoms fill in. This is the autonomous loop's stop-condition harness.
- `price_does_not_affect_predicate_result`
- `price_does_not_change_l4_decision`
- `parent_not_deleted_from_chaintape`
- `no_f64_in_tb_14_modules`
- `zero_liquidity_returns_none`
- `unresolved_challenge_blocks_masking`

**CP-A target**: file compiles; 6 tests fail with `unimplemented!()`.

### Atom 2 — `RationalPrice` + `NodeMarketEntry` + `compute_price_index` (Class 2, 24h)
**NEW**: `src/state/price_index.rs`. **MOD**: `src/state/mod.rs`, `src/state/q_state.rs` (DELETE legacy PriceIndex L699 + field L167), `tests/tb_13_legacy_cpmm_forward_fence.rs:113` (DELETE `"RationalPrice"` forbid line — name becomes legal).
**NEW**: `tests/tb_14_price_index.rs`.
**MOD**: `tests/fc_alignment_conformance.rs` — add FC3-N42 row.
**Halt triggers green**: #4, #5.
**CP-A gate**: `cargo test --workspace` PASS; SG-14.1 + SG-14.2 green.

### Atom 3 — `mask_set` in `AgentVisibleProjection` + `compute_mask_set` (Class 2, 24h)
**MOD**: `src/state/q_state.rs:121-123` (add `mask_set: BTreeSet<TxId>` with `serde(default)`). **MOD**: `src/state/price_index.rs` (append `compute_mask_set`).
**NEW**: `tests/tb_14_mask_set.rs`.
**MOD**: `tests/fc_alignment_conformance.rs` — add FC2-N28 row.
**Halt triggers green**: #3, #6.

### Atom 4 — `BoltzmannMaskPolicy` + env loader (Class 2, 24h)
**MOD**: `src/state/price_index.rs` (append `BoltzmannMaskPolicy` struct + `from_env()`). ENV vars: `BOLTZMANN_BETA_NUM/DEN`, `BOLTZMANN_MIN_LIQUIDITY_MICRO`, `BOLTZMANN_PRICE_MARGIN_NUM/DEN`, `BOLTZMANN_EPSILON_NUM/DEN`. **CP-B gate**: all halt-triggers except #1/#2 green.

### Atom 5 — Integer Boltzmann selector v2 (Class 2, 24h)
**MOD**: `src/sdk/actor.rs` — DELETE f64 `BoltzmannParams`/`is_frontier`/`lineage_score`/`boltzmann_select_parent`; ADD `boltzmann_select_parent_v2<R: Rng>(tape, price_index, mask_set, policy, rng) -> Option<NodeId>`.
**MOD**: `tests/fc_alignment_conformance.rs` — add FC2-N29 row.
**Halt triggers green**: ALL 6. **CP-C gate**: `cargo test --workspace` PASS; 6/6 halt-triggers GREEN.

### Atom 6 — Production wire-swap + legacy CPMM excision (**CLASS 3**, 72h, **DUAL AUDIT**)
**DELETE**: `src/prediction_market.rs`.
**MOD** (STEP_B_PROTOCOL for restricted files):
- `src/kernel.rs:19-43, 85, 192, 200, 212` — excise `markets`, `bounty_market`, `bounty_lp_seed`, f64 price fields
- `src/lib.rs` — remove `pub mod prediction_market`
- `src/bus.rs:478-509` — replace f64 markets feed with `compute_price_index` + `compute_mask_set`
- `src/sdk/snapshot.rs` — `UniverseSnapshot` carries `price_index: BTreeMap<TxId, NodeMarketEntry>` + `mask_set: BTreeSet<TxId>`
- `experiments/minif2f_v4/src/bin/evaluator.rs:1167, 1532-1538` — production wire-swap: `BoltzmannMaskPolicy::from_env()` + `boltzmann_select_parent_v2`
- `src/bin/audit_dashboard.rs:1492` — ADD §14 PriceIndex render with "PRICE IS SIGNAL, NOT TRUTH" banner (SG-14.6)
**SMOKE evidence** (chain-backed): `handover/evidence/tb_14_chaintape_smoke_2026-05-03/` with `--smoke` + `--half` runs.
**DUAL AUDIT** (Codex + Gemini). Round cap = 2; ROI-flip stop per `feedback_audit_loop_roi_flip`.

### Atom 7 — Ship gate
**NEW**: `handover/ai-direct/TB-14_SHIP_STATUS_2026-05-03.md`.
**MOD**: `handover/ai-direct/LATEST.md`.
**Final commit**: `TB-14 SHIPPED — PriceIndex v0 + Boltzmann Masking (Class 3 dual audit; 8/8 SG; 6/6 halt-triggers; closes OBS_TB_12_LEGACY_CPMM_QUARANTINE)`. FC-trace: `FC3-N42 + FC2-N28 + FC2-N29`.

---

## §4 Audit mode

| Atom | Risk class | Audit |
|---|---|---|
| 0-5 | Class 2 | Self-audit (cargo check + cargo test --workspace + halt-trigger battery) |
| 6 | **Class 3** | **Dual: Codex + Gemini mandatory**. VETO > CHALLENGE > PASS. Round cap = 2; OBS-defer ROI-flip residuals per `feedback_audit_loop_roi_flip`. If either auditor VETO at R2 → escalate to user. |
| 7 | Class 0 | Ship commit only |

---

## §5 Forbidden (architect §5.6 verbatim)

- No market trading
- No price-based settlement
- No parent deletion (mask = read-view only, ChainTape immutable)
- No Goodhart leak of private predicates
- No masking unresolved-challenge nodes
- **No f64** (anywhere in TB-14 code surface)
- No AMM

exec
/bin/bash -lc "sed -n '1,240p' handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TB-14 Atom 6 — Kickoff for Fresh Session

**Date opened**: 2026-05-03 (post-Atom-5 ship at `a9fbdf3`).
**Risk class**: **Class 3** (production wire-swap; touches `src/kernel.rs` + `src/bus.rs` STEP_B_PROTOCOL restricted files; deletes `src/prediction_market.rs` entirely; migrates production caller `experiments/minif2f_v4/src/bin/evaluator.rs`).
**Iteration cap**: 72h to evaluator pass/fail signal (per `feedback_iteration_cap_24h` Atom 6 production-wire-up exception).
**Audit**: **MANDATORY DUAL — Codex + Gemini** (per `feedback_dual_audit` Class 3 hybrid-by-risk). VETO > CHALLENGE > PASS (`feedback_dual_audit_conflict`). Round cap = 2 (`feedback_elon_mode_policy`); R3+ requires user authorization.
**Model directive**: Opus 4.7 xhigh (per `feedback_opusplan_unsuitable_for_turingos`). Do NOT use /opusplan for any sub-step.

---

## What's done (HEAD `a9fbdf3`)

- Atoms 0–5 of TB-14 shipped — see `LATEST.md` 🔨 2026-05-03 section.
- 6/6 architect §5.7 halt-triggers GREEN.
- workspace test count = 841 passed / 0 failed / 150 ignored.
- `boltzmann_select_parent_v2` exists in `src/sdk/actor.rs` ALONGSIDE the legacy f64 `BoltzmannParams` / `boltzmann_select_parent` / `is_frontier` / `lineage_score`. Atom 6 deletes the legacy as part of the wire-swap.
- `compute_price_index` + `compute_mask_set` + `BoltzmannMaskPolicy::from_env` ready in `src/state/price_index.rs`.

---

## Pacing expectations (CRITICAL — read before optimizing for speed)

Atoms 2–5 of the prior session shipped at ~25 / 15 / 10 / 20 min wallclock per atom. That pacing INCLUDED quality-producing work that LOOKS like overhead but is the product:

- Pre-deletion **G4 reference scans** (caught the L129 build-breaker the prior /opusplan attempt missed)
- **G2 single-rehash discipline** per file per atom (prevents the `fd4ad7d4 → eb0c9acb` trust-root churn the prior /opusplan attempt produced)
- Comprehensive commit bodies with FC-trace + `delta_vs_HEAD` + decision rationale (lets the next session pick up cold)
- OBS file writes when R-022 hook required justification
- Memory codification when novel anti-patterns surfaced

**DO NOT try to "go faster" for Atom 6 by skipping these guards.** That ~70-80 min/atom pace IS the quality pace. Faster than that means something is being skipped — the most likely victim is G4 (reference enumeration before deleting `prediction_market.rs`), and the most likely consequence is the Atom 6 equivalent of the L129 build-breaker.

**Eliminable overhead** (NOT quality work — optimize these freely):

| What | Why it's eliminable |
|---|---|
| Use `cargo test --lib` or `cargo test --test <name>` during iteration | Per-atom workspace runtime is ~3 min × N runs. Reserve `cargo test --workspace` for atom CP gates only (after all in-atom changes complete). |
| Pre-grep `pub fn\|struct\|enum` for `/// TRACE_MATRIX` backlinks before attempting commit | Avoids R-022 first-pass failures (e.g. Atom 2's `dominates_by` missing backlink → required edit + rehash). |
| Use `git commit -F /tmp/<msg>.txt` (NOT `git commit -m "$(cat <<EOF...)"`) | Pre-commit hooks read the staged message via `.git/COMMIT_EDITMSG`; `-m` does not write there before hook runs, so R-022-skip tokens don't reach the hook. `-F` does. (Confirmed during prior session Atom 2 commit attempt.) |
| Pre-stage GIT_COMMIT_MSG export shell snippet | Same root cause as above. |

Class 3 **dual audit** will introduce its own wallclock — each Codex / Gemini round trip is ~10–30 min. Plan v2 round cap = 2; if R2 still failing on either auditor, escalate to user. Do NOT compress audit rounds by bucket-OBS-deferring all CHALLENGEs (per `feedback_audit_obs_bias` and the user-flagged TB-13 round-3 incident). The right call is per-finding triage: cheap fixes get fixed; only multi-hour future-architecture residuals OBS-defer.

---

## Atom 6 deliverables (per charter §3 + plan v2 §Atom 6)

### DELETE

- `src/prediction_market.rs` — entire file (TB-3..TB-10 legacy CPMM scaffolding; closes `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`).
- `src/sdk/actor.rs` — legacy `BoltzmannParams` struct + `is_frontier` + `lineage_score` + `boltzmann_select_parent` (the f64 ones; v2 stays).

### MOD (STEP_B_PROTOCOL — parallel-branch A/B per `feedback_step_b_protocol` + `STEP_B_PROTOCOL.md`)

- `src/kernel.rs:19-43, 85, 192, 200, 212` — excise `markets`, `bounty_market`, `bounty_lp_seed` fields + their accessor methods (`open_bounty_market` / `resolve_bounty` / `create_market` / `buy_yes` / `buy_no` / `yes_price` / `market_ticker` / `market_ticker_full`). Kernel becomes pure topology — restores V3L-45 docstring contract.
- `src/lib.rs` — remove `pub mod prediction_market`.
- `src/bus.rs:430, 478-509, 482, 501` — replace f64 `markets` snapshot feed with `compute_price_index(&q_state.economic_state_t)` + `compute_mask_set(...)`. **CRITICAL**: per halt-trigger #2 (sequencer-import fence), bus.rs may now import TB-14 types but the L4/L4.E classification path must NOT consult them — only post-classification snapshot building.
- `src/sdk/snapshot.rs` — `MarketSnapshot { yes_price: f64, ... }` becomes `NodeMarketEntry`; `UniverseSnapshot` carries `price_index: BTreeMap<TxId, NodeMarketEntry>` + `mask_set: BTreeSet<TxId>`.
- `experiments/minif2f_v4/src/bin/evaluator.rs:20, 1167, 1532-1538` — production wire-swap:
  - `use turingosv4::sdk::actor::{BoltzmannParams, boltzmann_select_parent}` → `use turingosv4::sdk::actor::boltzmann_select_parent_v2;` + `use turingosv4::state::BoltzmannMaskPolicy;`
  - `BoltzmannParams::from_env()` → `BoltzmannMaskPolicy::from_env()`
  - `prices: HashMap<String, f64>` derived from `snap.markets[].yes_price` → `&snap.price_index` + `&snap.mask_set` directly
  - `boltzmann_select_parent(&snap.tape, &prices, &params, &mut boltz_rng)` → `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set, &policy, &mut boltz_rng)`. **Note**: v2's signature does NOT take `&Tape`; it operates on the price_index keyset.
- `src/bin/audit_dashboard.rs` — ADD §14 PriceIndex render with explicit "PRICE IS SIGNAL, NOT TRUTH" banner (SG-14.6); render `NodeMarketEntry` per-node table with `price_yes` / `price_no` as `numerator/denominator` strings (NEVER decimal — no f64 anywhere in TB-14 dashboard render).

### Tests

- Rewrite the CPMM-touching test files (per `prediction_market.rs:30` doc count, ~10 files). Each test that imports `BinaryMarket` / `BoltzmannParams` / etc. must be either rewritten to use the new API or deleted as obsolete.
- Add SG-14.6 dashboard signal-not-outcome label test (assertions on the `audit_dashboard.rs` §14 render output containing the literal "PRICE IS SIGNAL, NOT TRUTH").
- ChainTape smoke (chain-backed; per `feedback_smoke_evidence_naming` — only chain-backed Sequencer::apply_one + on-disk LedgerEntry counts as "smoke tape"; stdout-only does NOT):
  - `tests/tb_14_chaintape_smoke.rs` (NEW; pattern from `tests/tb_13_chaintape_smoke.rs`) MUST PASS.
  - `--smoke` (1 problem × MAX_TX=2) PASS.
  - `--half` (3 problems × MAX_TX=20) PASS.
  - Evidence dir `handover/evidence/tb_14_chaintape_smoke_2026-05-03/` with `replay_report.json` + `agent_pubkeys.json` + `genesis_report.json` per TB-13 pattern.

### Atom 6 ship gate

1. `cargo check --workspace` PASS.
2. `cargo test --workspace --no-fail-fast` PASS — all 6/6 halt-triggers must remain GREEN; full count must be ≥ 803 (charter §6 G-14.9 ship target; current is 841 + Atom 6 net new).
3. **Codex external review** on Atom 6 diff: PASS (or final OBS-deferred CHALLENGE residuals after R2).
4. **Gemini external review** on Atom 6 diff: PASS (or final OBS-deferred CHALLENGE residuals after R2).
5. ROI-flip stop trigger (`feedback_audit_loop_roi_flip`): if R2 CHALLENGEs are test-scaffold edge cases not production-code defects, OBS-defer + ship. Do NOT bucket-OBS production-code defects.
6. Conservative verdict (`feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS. Either auditor VETO at R2 → escalate to user.
7. ChainTape smoke evidence (chain-backed) committed.
8. Halt-trigger #2 specifically re-verified: bus.rs may import TB-14 types in Atom 6, but the L4/L4.E classification branch must remain decoupled. Code review at ship time confirms separation. (The halt-trigger #2 test itself scans sequencer.rs `use` statements, not bus.rs — but the spirit is preserved by code review.)

---

## Cross-cutting guards (re-applied from plan v2)

- **G1**: ZERO `f64` / `f32` substring in `src/state/price_index.rs`. Halt-trigger #4 fence verifies via runtime fs read. After Atom 6, also check `src/sdk/actor.rs` (legacy f64 deleted), `src/sdk/snapshot.rs` (legacy `MarketSnapshot.yes_price: f64` deleted), `src/bin/audit_dashboard.rs` §14 render block.
- **G2**: Per atom, each `genesis_payload.toml` file hash entry updated AT MOST ONCE. No `fd4ad7d4 → eb0c9acb` mid-atom double-rehash. Atom 6 will rehash many files — discipline is one rehash per file per atom, regardless of how many edits.
- **G3**: Architectural fixes, not band-aids. (Already shipped in Atom 2: successor-TB-marker fence fix. Further architectural debts in Atom 6 → OBS, not hardcoded lists.)
- **G4**: Pre-deletion reference scan. Before deleting `prediction_market.rs` and the legacy actor.rs functions, enumerate EVERY transitive use across `src/` + `tests/` + `experiments/`. Compare against discovered set after deletion. Plan to enumerate first; if grep finds anything outside the planned MOD list, STOP and update the plan — the MOD list is wrong.
- **G5**: Workspace test count tracked at every checkpoint. Negative delta is a STOP signal until root-caused.
- **G6**: No fake menus / no fence-sit ratification. Take explicit position on each finding; document deviations in commit body.

---

## Risk-class declarations

```text
phase_id:                P4 Information Loom + P3 RSP-6
risk_class:              Class 3 (production wire-swap; STEP_B_PROTOCOL restricted files)
roadmap_exit_criteria:   P4-Exit5 (price signal influences priority not predicate)
                         P4-Exit6 (YES/NO price signal-not-truth)
                         P4-Exit7 (Goodhart-shielded private predicates)
                         + closes OBS_TB_12_LEGACY_CPMM_QUARANTINE
kill_criteria_tested:    architect §5.7 halt triggers 1-6 (re-run at end of atom)
flowchart_trace:         FC2-N28, FC2-N29, FC3-N42 — preserved across wire-swap
iter_cap:                72h to evaluator pass/fail signal
```

---

## Where to start (fresh session — first 30 min)

1. Read this kickoff doc end-to-end.
2. Read `handover/ai-direct/LATEST.md` 🔨 2026-05-03 section.
3. Read `~/.claude/plans/sparkling-hugging-donut.md` §Atom 6 (skim — most details replicated above).
4. Read `handover/tracer_bullets/TB-14_charter_2026-05-03.md` §3 Atom 6 (architect spec verbatim).
5. Read `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` (the OBS this atom closes).
6. Run `cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test result" | awk '{passed+=$4; failed+=$6; ignored+=$8} END {print "passed:", passed, "failed:", failed, "ignored:", ignored}'` to confirm starting state (must show **841 passed / 0 failed / 150 ignored**). If different, investigate before any code changes.
7. Run a pre-Atom-6 reference scan: `grep -rln "BinaryMarket\|BoltzmannParams\|boltzmann_select_parent\b\|prediction_market\|MarketSnapshot" src/ tests/ experiments/ 2>/dev/null | tee /tmp/atom_6_refscan.txt`. Cross-check against the MOD list above. Any file in the scan output not in the MOD list = a planning gap.
8. Plan the wire-swap diff using STEP_B_PROTOCOL parallel branch A/B (per `STEP_B_PROTOCOL.md`).
9. Confirm Codex + Gemini dual audit triggers are available before starting code work (do not start a 72h Class 3 atom without auditors lined up).

---

## Cross-references

- Charter: `handover/tracer_bullets/TB-14_charter_2026-05-03.md`
- Architect spec verbatim: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §5
- Plan v2 (anti-pattern guards): `~/.claude/plans/sparkling-hugging-donut.md`
- STEP_B protocol: `STEP_B_PROTOCOL.md`
- Memory rule (model selection): `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_opusplan_unsuitable_for_turingos.md`
- Memory rule (STEP_B for restricted files): `feedback_step_b_protocol`
- Memory rule (dual audit hybrid-by-risk): `feedback_dual_audit`
- Memory rule (audit OBS-deferral bias warning): `feedback_audit_obs_bias`
- Memory rule (audit-loop ROI flip stop): `feedback_audit_loop_roi_flip`
- Memory rule (smoke evidence chain-backed only): `feedback_smoke_evidence_naming`
- Memory rule (workspace test canonical): `feedback_workspace_test_canonical`
- Memory rule (no retroactive evidence rewrite): `feedback_no_retroactive_evidence_rewrite`
- TB-13 ChainTape smoke pattern (template for Atom 6 smoke): `tests/tb_13_chaintape_smoke.rs` + `handover/evidence/tb_13_chaintape_smoke_2026-05-03/`
- Halt-trigger fence pattern (parallel for Atom 6 review): `tests/tb_14_halt_triggers.rs` halt-trigger #4 (file-level fence) + #1/#2 (sequencer fences)

---

## Stop conditions (immediate halt — no auto-execute exception)

- Halt-trigger flips green → red mid-atom → IMMEDIATE STOP, root-cause investigation.
- Either auditor VETO at R2 → escalate to user.
- 72h iter-cap exceeded → escalate to user.
- ChainTape smoke fails post-wire-swap → root-cause; do NOT relabel as "smoke evidence" without chain-backing (`feedback_smoke_evidence_naming`).
- Any reference to legacy `BoltzmannParams` / `boltzmann_select_parent` (legacy) / `BinaryMarket` / `prediction_market::*` survives in committed code → STOP, complete the deletion before merge.
- G2 violation (same file's hash line touched twice in one atom's diff) → STOP, re-plan.
- workspace_count drops without explanation → STOP, root-cause (almost certainly a test file failed to compile and its tests vanished — this is exactly the Atom 2 v1 L129 failure mode).

---

## Speed expectation summary

**Wall-clock target**: pure coding ~30–45 min; dual audit 1–2 rounds adds 60–120 min; ChainTape smoke + evidence ~30 min. **Total: 2–4 hours to Atom 6 ship**, longer if VETOs surface in R1.

If a sub-step is going faster than the prior session's pacing on equivalent work, sanity-check that you haven't skipped G1–G6. Quality first; speed is the residual.

exec
/bin/bash -lc 'rg -n "''^## §5|TB-14|Price is signal|5'"\\.7|halt\" handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
102:这条要在 TB-13/TB-14 反复保护，否则 CompleteSet 一接入，很容易出现“双账本”：
151:TB-14  PriceIndex v0 + Boltzmann Masking
411:Must halt immediately if:
436:# 5. TB-14 — PriceIndex v0 + Boltzmann Masking
445:Price is signal, not truth.
579:## 5.7 Loop-mode instruction
718:SG-15.7
1104:| TB-14 | PriceIndex + Boltzmann              | 价格信号与 scheduler masking          | 不让价格替代谓词     |
1134:TB-14:
1210:TB-14 的 PriceIndex 只能是统计信号/只读物化视图，不是 truth。
1232:必须在 TB-13/TB-14 之前确认：
1245:parallel families for TB-14 / TB-15 / TB-16 / TB-17). Part A is the canonical
1311:| `NodeMarketEntry` | Listed in roadmap as TB-14 derived view | Re-affirmed: NOT canonical state, derived view only |
1348:4. Drafting TB-14 / TB-15 / TB-16 / TB-17 stub charters.

exec
/bin/bash -lc "sed -n '1,220p' handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# OBS — TB-12 legacy CPMM quarantine prerequisite for TB-13

**Date**: 2026-05-03.
**Status**: OBS (observation; tracked for future TB).
**Triggered by**: Codex TB-12 ship audit Q5 CHALLENGE
(`handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md`).
**Audit verdict**: CHALLENGE on Q5 resolved as out-of-scope-for-TB-12
(see RECURSIVE_AUDIT_TB_12_2026-05-03.md §10).

## Summary

`src/prediction_market.rs` (345 lines) is legacy Tier 0 CPMM
scaffolding from early v4 (pre-2026-05 architect ruling on
TB-13/TB-14 trajectory). It violates the post-2026-05 architect
forbidden list:

- **f64 arithmetic** in `BinaryMarket` (yes_reserve / no_reserve / k /
  lp_total) — architect 2026-05-02 directive Part C line 1574 + §9.4
  TB-13 CR-13.5 explicit no-f64-mutation rule.
- **Automatic liquidity** via constant-product market-maker — architect
  §9.4 TB-13 forbidden list "No automatic liquidity. No ghost
  liquidity."
- **Trading semantics** (`buy_yes` / `BuyOutcome`) — architect §9.4
  TB-12 forbidden + §9.4 TB-13 + TB-14 forbidden.

Consumed by `src/kernel.rs:9-67`:
```rust
use crate::prediction_market::{BinaryMarket, MarketError};
pub markets: HashMap<NodeId, BinaryMarket>,
pub bounty_market: Option<BinaryMarket>,
```

## TB-12 boundary

TB-12 = Node Exposure Index added zero new code touching
`prediction_market.rs` or its kernel.rs consumers. Verified via
`grep -rn "BinaryMarket\|prediction_market" $(git diff 6ab165c..HEAD --name-only)`
returning empty for TB-12 commits (5ada28d → f4bff3f).

NodePosition (TB-12 atom 1) is a SEPARATE flat index canonical to
EconomicState; it does NOT consume or extend BinaryMarket.

## Roadmap replacement

Per architect 2026-05-02 supplementary directive
(`handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`):

- **TB-13 CompleteSet + MarketSeedTx**: introduces integer-math
  CTF-conserving YES/NO conditional shares. Replaces BinaryMarket's
  CPMM YES/NO accounting.
- **TB-14 PriceIndex v0**: computes price as
  `long_interest / (long_interest + short_interest)` from
  `node_positions_t` (TB-12 schema). NO automatic liquidity. NO CPMM.
  Replaces BinaryMarket's price discovery + its CPMM math.

After TB-14 ships, `src/prediction_market.rs` + `src/kernel.rs` market
scaffolding becomes architecturally dead. **Quarantine /
deprecation / removal is required before TB-14 SHIP** to prevent
the new architecture from inheriting f64 / automatic-liquidity
artefacts.

## Action plan (TB-13 prerequisite)

A future TB-13 atom 0.5 (carry-forward, mirroring TB-12 Atom 0.5
carry-forward pattern) MUST:

1. Audit which `src/kernel.rs` paths still consume `BinaryMarket`.
2. Either:
   a. Replace `BinaryMarket` consumers with new TB-13 CompleteSet
      conditional-share types, OR
   b. Feature-gate the legacy paths behind `#[cfg(feature = "legacy_cpmm")]`
      (off by default; explicit opt-in for migration tests only), OR
   c. Delete outright if no production consumers remain.
3. Remove `pub mod prediction_market;` from `src/lib.rs` once consumers
   are gone.
4. Remove the `BinaryMarket` Trust Root manifest entries (if any).
5. Ship-gate the migration with a forbidden-token grep that fails on
   any `BinaryMarket` / `buy_yes` / `f64 reserve` reference outside
   `cfg(feature = "legacy_cpmm")`.

## Why this is OBS-tracked, not blocker

Per `feedback_no_retroactive_evidence_rewrite`:

> New evidence requirements ... apply going-forward only. NEVER rewrite
> old ledger roots ... fabricate genesis_report into old dirs, or relabel
> old `evaluator-attested` results as `chain-oracle-derived`.

The architect's 2026-05-02 + 2026-05-03 forbidden-token rules are
forward-binding for NEW code in TB-12 onward. They do NOT auto-remove
pre-existing v3-style scaffolding. TB-12 honored the rule by adding
zero new trading code. Quarantine of pre-existing scaffolding is the
TB-13 prerequisite that the architect's ruling implies via
"replace, not extend" semantics.

## Cross-references

- Codex audit doc: `handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md` Q5
- TB-12 recursive self-audit §10 remediation log: `handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md`
- Architect supplementary directive (TB-13 + TB-14 spec): `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`
- Architect 2026-05-03 ruling (TB-12 forbidden list): `handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md` §9.4
- Architect 2026-05-03 post-TB-12 ruling (TB-13 Atom 0.5 mandate): `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` Part A §4.2
- TB-13 charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`

---

## TB-13 Atom 0.5 update (2026-05-03 evening — forward-fence + label IN PLACE)

**Status update**: open OBS still tracked; forward-fence ship-gate added in TB-13 Atom 0.5; hard removal **carries forward to TB-14 SHIP prerequisite** unchanged.

### What TB-13 Atom 0.5 added

1. **Module-header label** in `src/prediction_market.rs` — ` //! # LEGACY — ...` doc-comment block declaring: not constitutional, not RSP-M, not production market path; lists each constitutional non-compliance (f64 / automatic liquidity / trading semantics); names the carry-forward owner (TB-14 SHIP prerequisite).
2. **Field-level labels** in `src/kernel.rs` — every CPMM-bearing field (`markets`, `bounty_market`, `bounty_lp_seed`) carries `**LEGACY** ...` doc-comments naming the migration path (TB-13 `CompleteSetMintTx` / `ConditionalShareBalances` + TB-14 `PriceIndex`).
3. **Forward-fence ship-gate test** at `tests/tb_13_legacy_cpmm_forward_fence.rs` — three EXACT-named tests per architect §4.2:
   - `legacy_cpm_api_not_imported_by_complete_set` (SG-13.0.1)
   - `no_f64_in_complete_set_or_market_seed` (SG-13.0.2)
   - `prediction_market_legacy_quarantined` (SG-13.0.3)
4. **OBS carry-forward** (this update) — SG-13.0.4 satisfied as "explicitly carried as non-importable legacy".

### What TB-13 Atom 0.5 deliberately did NOT do

- **No retroactive deletion** of `src/prediction_market.rs` or `src/kernel.rs` market scaffolding. Production callers at `src/bus.rs:206 / 327 / 359 / 480-515` and `experiments/minif2f_v4/src/bin/evaluator.rs:1323` plus 10+ test files (`tests/tb_6_*`, `tests/tb_7_*`, `tests/wal_resume.rs`, `tests/fc_alignment_conformance.rs`) would break.
- **No removal** of `pub mod prediction_market;` from `src/lib.rs` for the same reason.
- **No `#[cfg(feature = "legacy_cpmm")]` feature gate** — that would still require touching every consumer; same surface area; defers no work.

This decision is consistent with `feedback_no_retroactive_evidence_rewrite` (forward-binding rules apply going-forward only) and architect §4.2 halting-trigger semantics (which target NEW TB-13 code, not existing scaffolding).

### TB-14 SHIP prerequisite (unchanged)

Per the original action plan above, before TB-14 SHIP one of these MUST be done:

a. Replace `BinaryMarket` consumers with new TB-13 + TB-14 conditional-share + price-signal types, OR
b. Feature-gate behind `#[cfg(feature = "legacy_cpmm")]` (off by default; explicit opt-in for migration tests only), OR
c. Delete outright if no production consumers remain.

Either way: remove `pub mod prediction_market;` from `src/lib.rs`; remove `BinaryMarket` Trust Root entries (if any); ship-gate the migration with a forbidden-token grep that fails on any `BinaryMarket` / `buy_yes` / `f64 reserve` reference outside `cfg(feature = "legacy_cpmm")`.

exec
/bin/bash -lc "sed -n '1,220p' handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TB-14 Atom 6 — ChainTape smoke (post-wire-swap regression)

**Date**: 2026-05-03
**Source**: `tests/tb_14_chaintape_smoke.rs::tb_14_atom_6_post_wire_swap_chaintape_replay_preserves_price_index_determinism`
**Trigger**: TB-14 Atom 6 production wire-swap (excise legacy CPMM scaffolding; reroute bus snapshot price-signal surface through `compute_price_index` + `compute_mask_set` integer-rational derived views).

## Headline

- L4 entries: 2 (mint + redeem)
- L4.E entries: 0
- All 7 ReplayReport indicators GREEN: true
- Live `state_root_t`: `72f3053b532b528d30ec58319c66a7ba575a98a1375f1e129b902253dc13588e`
- Replay `final_state_root_hex`: `72f3053b532b528d30ec58319c66a7ba575a98a1375f1e129b902253dc13588e`
- `live.economic_state_t == replayed.economic_state_t`: byte-equal
- `compute_price_index(live)` == `compute_price_index(replayed)`: byte-equal
- `compute_price_index` idempotent across 5 invocations: ✓
- Empty `node_positions_t` → empty PriceIndex BTreeMap: ✓

## What this evidence proves (Atom 6 specific)

1. The Atom 6 production wire-swap (excised `prediction_market.rs`, `kernel.markets`, `BoltzmannParams`, legacy f64 `boltzmann_select_parent`; rewired `bus.snapshot` to derive `price_index` + `mask_set` from `Sequencer::q_snapshot`'s `EconomicState`) does NOT regress chain-replay determinism.
2. `verify_chaintape` reconstructs a `QState` from persisted artifacts whose `final_state_root_hex` matches live `state_root_t` (Art.0.2 Tape Canonical preserved across the wire-swap).
3. The TB-14 derived view (`compute_price_index(econ)`) is replay-deterministic by composition: pure function over a byte-equal-replayed `EconomicState` yields byte-equal `BTreeMap<TxId, NodeMarketEntry>` (FR-14.x / FC3-N42 chaintape integration evidence).
4. `compute_price_index` is idempotent across N calls (Art.0.2 pure-function determinism at the derived-view layer).
5. Empty `node_positions_t` → empty PriceIndex (FR-14.3 / halt-trigger #5 extended at the chaintape integration layer).

## What is NOT in scope here

- **Non-empty PriceIndex via WorkTx**: this smoke uses CompleteSet flow only (TB-13 substrate). A WorkTx-creates-NodePosition flow (TB-12 substrate that produces non-empty PriceIndex) is covered by the in-memory unit tests at `tests/tb_14_price_index.rs` + halt-triggers + `src/state/price_index.rs` inline tests. Per `feedback_chaintape_externalized_proposal`, the chaintape smoke records what the system externalizes via `submit_typed_tx` end-to-end; the per-position aggregation is pure-function-tested elsewhere.
- **`mask_set` via Tape children**: `compute_mask_set` requires a Tape; this smoke does not exercise mask computation (covered by `tests/tb_14_mask_set.rs` + halt-triggers #3 / #6).
- **Boltzmann v2 selector**: covered by inline tests in `src/sdk/actor.rs::tests::v2_*`. Production wire-up at `experiments/minif2f_v4/src/bin/evaluator.rs:~1559` is exercised by the `--smoke` / `--half` evaluator runs.

exec
/bin/bash -lc "sed -n '436,610p' handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# 5. TB-14 — PriceIndex v0 + Boltzmann Masking

## 5.1 目标

将 TB-12 的 `NodePosition` 和 TB-13 的 share/collateral state 转化为价格信号，并用于 read-view / scheduler masking。

核心原则：

```text
Price is signal, not truth.
```

这与宪法中的信号管理一致。宪法把顶层白盒的工作定义为量化、广播、屏蔽，并把统计信号用于衡量相对有效性；但布尔谓词仍然确立绝对边界。

---

## 5.2 新增对象

```rust
pub struct RationalPrice {
    pub numerator: u128,
    pub denominator: u128,
}

pub struct NodeMarketEntry {
    pub node_id: TxId,
    pub task_id: TaskId,
    pub event_id: EventId,
    pub long_interest: MicroCoin,
    pub short_interest: MicroCoin,
    pub yes_share_depth: ShareAmount,
    pub no_share_depth: ShareAmount,
    pub price_yes: Option<RationalPrice>,
    pub price_no: Option<RationalPrice>,
    pub liquidity_depth: MicroCoin,
}

pub struct BoltzmannMaskPolicy {
    pub beta_num: i64,
    pub beta_den: i64,
    pub min_liquidity: MicroCoin,
    pub price_margin: RationalPrice,
    pub epsilon_exploration_num: u64,
    pub epsilon_exploration_den: u64,
}
```

---

## 5.3 Functional requirements

```text
FR-14.1
Compute price_yes = long / (long + short) when exposure liquidity exists.

FR-14.2
Compute price_no = short / (long + short).

FR-14.3
If no long/short liquidity exists, price=None.

FR-14.4
Expose PriceIndex as read-only broadcast/statistical signal.

FR-14.5
Boltzmann scheduler may mask parent only if child price sufficiently dominates.

FR-14.6
Masked parent remains in ChainTape and can be recovered.
```

---

## 5.4 Constitutional requirements

```text
CR-14.1
Price cannot override predicates.

CR-14.2
Price cannot make failed proposal accepted.

CR-14.3
Masked means read-view/scheduler mask only, not deletion.

CR-14.4
Low-liquidity price cannot mask parent.

CR-14.5
Unresolved-challenged child cannot mask parent.

CR-14.6
Goodhart-sensitive private predicates remain hidden.
```

---

## 5.5 Ship gates

```text
SG-14.1
PriceIndex computes expected YES/NO probabilities.

SG-14.2
No-liquidity node has price=None.

SG-14.3
Parent not deleted from ChainTape after masking.

SG-14.4
Predicate failure still dominates high price.

SG-14.5
Boltzmann selection includes epsilon exploration.

SG-14.6
Dashboard shows price as signal, not outcome.

SG-14.7
Unresolved challenge blocks masking.

SG-14.8
Low-liquidity manipulation cannot mask parent.
```

---

## 5.6 Forbidden

```text
No market trading.
No price-based settlement.
No parent deletion.
No Goodhart leak of private predicates.
No masking unresolved-challenge nodes.
No f64.
No AMM.
No DPMM.
No price-as-oracle language.
```

---

## 5.7 Loop-mode instruction

AI coder can run implementation autonomously until pre-ship audit.

Halt if:

```text
price affects predicate result;
price changes L4/L4.E decision;
parent node deleted/removed from ChainTape;
f64 introduced;
price computed for zero-liquidity node as non-None;
unresolved challenge is masked as safe.
```

Risk class:

```text
Class 2
```

If Boltzmann masking changes actual Agent read view in production, promote to:

```text
Class 3
```

---

# 6. TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule

## 6.1 目标

codex
The governing docs line up with the user prompt: Atom 6 is Class 3 because the scheduler read view is now production-wired, and §5.6/§5.7 make the forbidden list literal. I’m moving from docs into code paths now, with emphasis on places where an empty or fallback view could accidentally look like a truthful signal.
exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1,120p;500,575p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
     2	//!
     3	//! Spec authority:
     4	//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 3 (Sequencer
     5	//!   pseudocode, K1 dual-counter, K3 head_t deferred, C3 sign API)
     6	//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 8
     7	//!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
     8	//!
     9	//! Single-writer per (runtime_repo, run_id). Per spec § 5.2.1.
    10	//!
    11	//! **Stub state (this atom)**: every per-kind transition returns
    12	//! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
    13	//! bodies. The structural correctness of the apply path (snapshot → dispatch →
    14	//! CAS put → sign → root fold → commit → Q_t mutation) is locked by the
    15	//! impl + tests here; what's left is per-kind transition logic.
    16	//!
    17	//! /// TRACE_MATRIX § 5.2.1 + § 8 — L4 sequencer single-writer + dispatch.
    18	
    19	use std::sync::atomic::{AtomicU64, Ordering};
    20	use std::sync::{Arc, RwLock};
    21	
    22	use sha2::{Digest, Sha256};
    23	
    24	use crate::bottom_white::cas::schema::{Cid, ObjectType};
    25	use crate::bottom_white::cas::store::{CasError, CasStore};
    26	use crate::bottom_white::ledger::system_keypair::{
    27	    transition_ledger_emitter, Ed25519Keypair, KeypairError, SystemEpoch,
    28	};
    29	use crate::bottom_white::ledger::rejection_evidence::{
    30	    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
    31	};
    32	use crate::bottom_white::ledger::transition_ledger::{
    33	    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    34	    LedgerWriterError,
    35	};
    36	use crate::bottom_white::tools::registry::ToolRegistry;
    37	use crate::economy::monetary_invariant::{
    38	    assert_claim_amount_backed_by_escrow, assert_no_post_init_mint, assert_read_is_free,
    39	    assert_task_market_total_escrow_matches_locks, assert_total_ctf_conserved,
    40	};
    41	use crate::state::q_state::{AgentId, EscrowEntry, Hash, QState, TaskMarketEntry, TxId};
    42	use crate::state::typed_tx::{HasSubmitter, SignalBundle, TransitionError, TypedTx};
    43	use std::collections::BTreeSet;
    44	use crate::top_white::predicates::registry::PredicateRegistry;
    45	
    46	// ────────────────────────────────────────────────────────────────────────────
    47	// TB-2 — WorkTx-accept state-root domain (preflight v3 §3.4 + P1-1 r2)
    48	// ────────────────────────────────────────────────────────────────────────────
    49	
    50	/// TRACE_MATRIX FC3-S3: TB-2 interim WorkTx-accept state-root domain.
    51	///
    52	/// Real patch semantics for `q_next.state_root_t` land in P5; until then
    53	/// TB-2 advances the state root deterministically with this domain string
    54	/// concatenated against `q.state_root_t` and the canonical hash of the
    55	/// accepted WorkTx. Distinct from the TB-1 toy domain
    56	/// `b"turingosv4.l4_state_root.v1"` used by `AcceptedLedger` at
    57	/// `src/economy/ledger.rs:350, :357` (TB-1 RSP-0 primitive vs production
    58	/// state-root mutator separation).
    59	pub(crate) const WORKTX_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.worktx.accept.v1";
    60	
    61	/// TRACE_MATRIX FC3-S3: TB-2 canonical hash helper for a `TypedTx`.
    62	///
    63	/// Defined locally (not in `bottom_white::ledger::transition_ledger`) because
    64	/// `canonical_hash(tx)` is NOT a generic existing helper there — only
    65	/// `canonical_encode` is — and TB-2 wants a single short call site that
    66	/// includes domain separation. Codex r2 P1-2.
    67	pub(crate) fn worktx_canonical_hash(tx: &TypedTx) -> Hash {
    68	    let mut h = Sha256::new();
    69	    h.update(b"turingosv4.worktx.canonical_hash.v1");
    70	    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    71	    let digest: [u8; 32] = h.finalize().into();
    72	    Hash::from_bytes(digest)
    73	}
    74	
    75	/// TRACE_MATRIX FC3-S3: TB-2 interim state-root mutator on WorkTx accept.
    76	///
    77	/// `q_next.state_root_t = sha256(WORKTX_ACCEPT_DOMAIN_V1 ‖ q.state_root_t.0
    78	/// ‖ worktx_canonical_hash(tx).0)`. P5 replaces this with real patch
    79	/// semantics; until then this is the deterministic monotonic mutation
    80	/// asserted by U3 / I9.
    81	///
    82	/// Public single-item surface for the TB-2 accept-side state-root contract.
    83	/// Integration tests in `tests/tb_2_runtime_boundary.rs` (e.g. I9) use this
    84	/// helper directly to recompute the expected post-accept hash WITHOUT
    85	/// re-implementing the WORKTX_ACCEPT_DOMAIN_V1 / worktx_canonical_hash
    86	/// composition by hand. The composing primitives stay `pub(crate)` so the
    87	/// public surface is a single semantic helper, not the raw building blocks
    88	/// (Phase-1c r1 Codex P0-1 remediation).
    89	pub fn worktx_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    90	    let work_digest = worktx_canonical_hash(tx);
    91	    let mut h = Sha256::new();
    92	    h.update(WORKTX_ACCEPT_DOMAIN_V1);
    93	    h.update(prev.0);
    94	    h.update(work_digest.0);
    95	    let digest: [u8; 32] = h.finalize().into();
    96	    Hash::from_bytes(digest)
    97	}
    98	
    99	// ────────────────────────────────────────────────────────────────────────────
   100	// TB-3 RSP-1 — TaskOpen + EscrowLock state-root domains (charter § 4.3)
   101	// ────────────────────────────────────────────────────────────────────────────
   102	
   103	/// TRACE_MATRIX TB-3 charter § 4.3 — TaskOpen-accept state-root domain.
   104	pub(crate) const TASK_OPEN_DOMAIN_V1: &[u8] = b"turingosv4.task_open.accept.v1";
   105	
   106	/// TRACE_MATRIX TB-3 charter § 4.3 — EscrowLock-accept state-root domain.
   107	pub(crate) const ESCROW_LOCK_DOMAIN_V1: &[u8] = b"turingosv4.escrow_lock.accept.v1";
   108	
   109	/// TRACE_MATRIX TB-3 charter § 4.3 — interim state-root mutator on
   110	/// `TaskOpenTx` accept. Mirror of `worktx_accept_state_root` with its own
   111	/// domain prefix for SHA-256 input separation. Real patch semantics for
   112	/// `q_next.state_root_t` land in P5; until then this is the deterministic
   113	/// monotonic mutation. Public single-item surface for integration tests
   114	/// to recompute the expected post-accept hash without re-implementing
   115	/// the domain composition.
   116	pub fn task_open_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
   117	    let mut h = Sha256::new();
   118	    h.update(TASK_OPEN_DOMAIN_V1);
   119	    h.update(prev.0);
   120	    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
   500	        TypedTx::Work(_)
   501	        | TypedTx::Verify(_)
   502	        | TypedTx::Challenge(_)
   503	        | TypedTx::Reuse(_)
   504	        | TypedTx::TaskOpen(_)
   505	        | TypedTx::EscrowLock(_)
   506	        | TypedTx::CompleteSetMint(_)
   507	        | TypedTx::CompleteSetRedeem(_)
   508	        | TypedTx::MarketSeed(_) => None,
   509	    }
   510	}
   511	
   512	// ────────────────────────────────────────────────────────────────────────────
   513	// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
   514	// ────────────────────────────────────────────────────────────────────────────
   515	
   516	/// TRACE_MATRIX § 8 — exhaustive dispatch over `TypedTx` variants.
   517	///
   518	/// **Stub state (CO1.7-impl A3)**: every variant returns
   519	/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
   520	/// transition body per `STATE_TRANSITION_SPEC § 3.1-3.7`. The exhaustive match
   521	/// itself is the contract: any future TypedTx variant addition triggers a
   522	/// non-exhaustive-match compile error here, forcing explicit handling.
   523	pub(crate) fn dispatch_transition(
   524	    q: &QState,
   525	    tx: &TypedTx,
   526	    _predicate_registry: &PredicateRegistry,
   527	    _tool_registry: &ToolRegistry,
   528	) -> Result<(QState, SignalBundle), TransitionError> {
   529	    match tx {
   530	        TypedTx::Work(work) => {
   531	            // TB-2 Atom 3: WorkTx pure validation per preflight v3 §3.3.
   532	            // No I/O, no side effects, no writer calls — apply_one is the
   533	            // only place ledger writes happen.
   534	
   535	            // Step 1: parent-root match (Inv 5; P1:5).
   536	            if work.parent_state_root != q.state_root_t {
   537	                return Err(TransitionError::StaleParent);
   538	            }
   539	
   540	            // Step 2: acceptance predicate bundle — every entry must be true.
   541	            for (pid, bwp) in work.predicate_results.acceptance.iter() {
   542	                if !bwp.value {
   543	                    return Err(TransitionError::AcceptancePredicateFailed(pid.clone()));
   544	                }
   545	            }
   546	
   547	            // Step 3: settlement predicate bundle (if applicable to RSP-1).
   548	            for (pid, bwp) in work.predicate_results.settlement.iter() {
   549	                if !bwp.value {
   550	                    return Err(TransitionError::SettlementPredicateFailed(pid.clone()));
   551	                }
   552	            }
   553	
   554	            // Step 4: YES stake gate (RSP-1 P3:3). StakeMicroCoin newtype
   555	            // intentionally has no integer comparison; use the const accessor.
   556	            if work.stake.micro_units() <= 0 {
   557	                return Err(TransitionError::StakeInsufficient);
   558	            }
   559	
   560	            // ──────────────────────────────────────────────────────────────
   561	            // TB-3 Atom 6 — Bridge DELETED. Structural admission via the
   562	            // formal RSP-1 surface: task_markets_t[task_id].total_escrow > 0.
   563	            // The TB-2 P0-B option (a) bridge `TxId(work.task_id.0.clone())`
   564	            // synthetic-ID + escrows_t fallback is GONE — its constitutional
   565	            // debt is now closed. Charter § 4.3 step 6 + § 5 #14 (no bridge
   566	            // resurrection — enforced by tests/tb_3_bridge_deletion_invariant.rs
   567	            // in Atom 7).
   568	            // ──────────────────────────────────────────────────────────────
   569	
   570	            // Step 5: escrow presence gate via formal surface (charter § 4.3
   571	            // step 6 NEW form). task_markets_t is now TaskId-keyed and
   572	            // populated only by accepted TaskOpenTx. total_escrow is the
   573	            // derived cache that grows only via accepted EscrowLockTx.
   574	            let market = q.economic_state_t.task_markets_t.0.get(&work.task_id);
   575	            let has_escrow = market.map_or(false, |m| m.total_escrow.micro_units() > 0);

exec
/bin/bash -lc "nl -ba src/state/price_index.rs | sed -n '1,180p;330,450p;560,720p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-14 Atom 2 — PriceIndex v0 derived view.
     2	//!
     3	//! TRACE_MATRIX FC3-N42 (architect 2026-05-03 ruling §5.1 + §5.2 + §5.4 +
     4	//! charter §1 goal): pure deterministic function over canonical
     5	//! `EconomicState` that derives `NodeMarketEntry` per `TxId` from
     6	//! `node_positions_t` (TB-12 substrate) plus `conditional_share_balances_t`
     7	//! (TB-13 substrate). **Price is signal, not truth** (architect §5.1):
     8	//! the derived view is read-only broadcast input to the scheduler mask
     9	//! (FR-14.5 / FR-14.6) and dashboard render (SG-14.6); it MUST NOT
    10	//! influence predicate gates (CR-14.1 / halt-trigger #1) or L4 / L4.E
    11	//! decision (CR-14.2 / halt-trigger #2).
    12	//!
    13	//! All arithmetic is integer-rational (`u128` numerator + denominator).
    14	//! Decimal-float types are forbidden in this module per charter §5
    15	//! Forbidden list and halt-trigger #4. Replay-deterministic per
    16	//! Art.0.2: no env input, no clock, no randomness.
    17	
    18	use std::collections::{BTreeMap, BTreeSet};
    19	
    20	use serde::{Deserialize, Serialize};
    21	
    22	use crate::economy::money::MicroCoin;
    23	use crate::ledger::Tape;
    24	use crate::state::q_state::{AgentId, ChallengeStatus, EconomicState, ShareSidePair};
    25	use crate::state::typed_tx::{EventId, NodePosition, PositionSide, ShareAmount};
    26	use crate::state::{TaskId, TxId};
    27	
    28	// ─────────────────────────────────────────────────────────────────────────
    29	// RationalPrice — architect §5.2 verbatim shape
    30	// ─────────────────────────────────────────────────────────────────────────
    31	
    32	/// TRACE_MATRIX TB-14 Atom 2 (architect §5.2 verbatim): integer-rational
    33	/// price representation. `numerator / denominator` ∈ \[0, 1\] when
    34	/// constructed by `compute_price_index` (architect FR-14.1 + FR-14.2). All
    35	/// comparisons use cross-multiplication; no division until the dashboard
    36	/// render layer (SG-14.6) where it is purely cosmetic.
    37	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
    38	pub struct RationalPrice {
    39	    pub numerator: u128,
    40	    pub denominator: u128,
    41	}
    42	
    43	impl RationalPrice {
    44	    /// TRACE_MATRIX FC3-N42 (architect §5.5 SG-14.x mask-margin gate; helper
    45	    /// for FC2-N28 `compute_mask_set` in Atom 3): cross-multiplication
    46	    /// dominance predicate.
    47	    ///
    48	    /// True iff `self - other >= margin`, computed by cross-multiplication
    49	    /// to avoid division. Used by Atom 3's `compute_mask_set` to enforce
    50	    /// the price-margin gate (FR-14.5 / SG-14.x). Defensive: returns
    51	    /// `false` on any zero denominator (`compute_price_index` never
    52	    /// produces a `RationalPrice` with zero denominator — that case is
    53	    /// `Option::None` per FR-14.3 / halt-trigger #5 — but defense-in-depth
    54	    /// is cheap).
    55	    pub fn dominates_by(&self, other: &RationalPrice, margin: &RationalPrice) -> bool {
    56	        if self.denominator == 0 || other.denominator == 0 || margin.denominator == 0 {
    57	            return false;
    58	        }
    59	        // Goal: self - other >= margin
    60	        //   (self.n * other.d - other.n * self.d) / (self.d * other.d)
    61	        //       >= margin.n / margin.d
    62	        // Cross-multiply by (self.d * other.d * margin.d) > 0:
    63	        //   (self.n * other.d - other.n * self.d) * margin.d
    64	        //       >= margin.n * (self.d * other.d)
    65	        let self_d = self.denominator;
    66	        let other_d = other.denominator;
    67	        let cross_diff = self
    68	            .numerator
    69	            .saturating_mul(other_d)
    70	            .saturating_sub(other.numerator.saturating_mul(self_d));
    71	        let lhs = cross_diff.saturating_mul(margin.denominator);
    72	        let rhs = margin
    73	            .numerator
    74	            .saturating_mul(self_d)
    75	            .saturating_mul(other_d);
    76	        lhs >= rhs
    77	    }
    78	}
    79	
    80	// ─────────────────────────────────────────────────────────────────────────
    81	// NodeMarketEntry — architect §5.2 verbatim shape (10 fields)
    82	// ─────────────────────────────────────────────────────────────────────────
    83	
    84	/// TRACE_MATRIX TB-14 Atom 2 (architect §5.2 verbatim): per-node market
    85	/// signal entry. **Derived view** populated by `compute_price_index`;
    86	/// never stored as canonical state (architect §5.1: "price is signal,
    87	/// not truth"; charter §7 auto-resolution A: "no second source-of-truth").
    88	///
    89	/// Field semantics:
    90	/// - `node_id` — the `TxId` of the WorkTx attempt-node these positions reference
    91	/// - `task_id` — the `TaskId` (Q-derived from any underlying `NodePosition`)
    92	/// - `event_id` — `EventId(task_id)` (TB-13: 1:1 with TaskId per `typed_tx.rs:1075`)
    93	/// - `long_interest` / `short_interest` — sum of `NodePosition.amount` per side
    94	/// - `yes_share_depth` / `no_share_depth` — sum of `ConditionalShareBalances` for `event_id`
    95	/// - `price_yes` / `price_no` — `Option<RationalPrice>`; `None` iff zero liquidity (FR-14.3)
    96	/// - `liquidity_depth` — `long_interest + short_interest` (saturating)
    97	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    98	pub struct NodeMarketEntry {
    99	    pub node_id: TxId,
   100	    pub task_id: TaskId,
   101	    pub event_id: EventId,
   102	    pub long_interest: MicroCoin,
   103	    pub short_interest: MicroCoin,
   104	    pub yes_share_depth: ShareAmount,
   105	    pub no_share_depth: ShareAmount,
   106	    pub price_yes: Option<RationalPrice>,
   107	    pub price_no: Option<RationalPrice>,
   108	    pub liquidity_depth: MicroCoin,
   109	}
   110	
   111	// ─────────────────────────────────────────────────────────────────────────
   112	// compute_price_index — pure fn over EconomicState
   113	// ─────────────────────────────────────────────────────────────────────────
   114	
   115	/// TRACE_MATRIX TB-14 Atom 2 (FC3-N42; architect §5.1 + charter §3 Atom 2):
   116	/// derive the per-node `PriceIndex` from `EconomicState.node_positions_t`
   117	/// (long / short interest aggregation; FR-14.1 / FR-14.2) and
   118	/// `conditional_share_balances_t` (yes / no share depth aggregation per
   119	/// `event_id`).
   120	///
   121	/// **Replay-deterministic** (Art.0.2): pure over the canonical state
   122	/// vector; no env / clock / RNG. Iteration order is `BTreeMap` order on
   123	/// `TxId`, which is lexicographic on the inner `String`.
   124	///
   125	/// **No predicate side-effect** (CR-14.1 / halt-trigger #1): this is a
   126	/// read-only derivation; the sequencer never reads its result during
   127	/// `dispatch_transition` (predicate gate at `sequencer.rs:516-558`).
   128	///
   129	/// **Empty / zero-stake → None** (FR-14.3 / halt-trigger #5): a node with
   130	/// zero long AND zero short interest yields
   131	/// `price_yes == None && price_no == None`. Rationale: division-by-zero
   132	/// avoidance and architect §5.7 halt trigger 5.
   133	pub fn compute_price_index(econ: &EconomicState) -> BTreeMap<TxId, NodeMarketEntry> {
   134	    // Pass 1: group NodePositions by node_id; collect (task_id, long_micro, short_micro).
   135	    let mut groups: BTreeMap<TxId, (TaskId, u128, u128)> = BTreeMap::new();
   136	    for position in econ.node_positions_t.0.values() {
   137	        let amount_micro = position.amount.micro_units();
   138	        let amount_u128 = if amount_micro < 0 {
   139	            0u128
   140	        } else {
   141	            amount_micro as u128
   142	        };
   143	        let entry = groups
   144	            .entry(position.node_id.clone())
   145	            .or_insert_with(|| (position.task_id.clone(), 0u128, 0u128));
   146	        match position.side {
   147	            PositionSide::Long => entry.1 = entry.1.saturating_add(amount_u128),
   148	            PositionSide::Short => entry.2 = entry.2.saturating_add(amount_u128),
   149	        }
   150	    }
   151	
   152	    // Pass 2: per node, derive NodeMarketEntry.
   153	    let mut out: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   154	    for (node_id, (task_id, long_micro, short_micro)) in groups.into_iter() {
   155	        let total_micro = long_micro.saturating_add(short_micro);
   156	        let event_id = EventId(task_id.clone());
   157	
   158	        let to_micro = |u: u128| -> MicroCoin {
   159	            // Saturating cast u128 → i64 (positive values only; cap at i64::MAX).
   160	            let capped = if u > i64::MAX as u128 { i64::MAX } else { u as i64 };
   161	            MicroCoin::from_micro_units(capped)
   162	        };
   163	
   164	        let (price_yes, price_no) = if total_micro == 0 {
   165	            (None, None)
   166	        } else {
   167	            (
   168	                Some(RationalPrice {
   169	                    numerator: long_micro,
   170	                    denominator: total_micro,
   171	                }),
   172	                Some(RationalPrice {
   173	                    numerator: short_micro,
   174	                    denominator: total_micro,
   175	                }),
   176	            )
   177	        };
   178	
   179	        // yes_share_depth / no_share_depth: sum across all owners' balances
   180	        // for this event_id. The conditional_share_balances_t shape is
   330	}
   331	
   332	// ─────────────────────────────────────────────────────────────────────────
   333	// compute_mask_set — derive the parent-mask set from price_index + Tape
   334	// + policy + open challenges. Pure deterministic over inputs.
   335	// ─────────────────────────────────────────────────────────────────────────
   336	
   337	/// TRACE_MATRIX TB-14 Atom 3 (FC2-N28; architect §5.5 SG-14.3 / SG-14.5 /
   338	/// SG-14.7 / SG-14.8 + charter §3 Atom 3): derive the per-round
   339	/// `mask_set: BTreeSet<TxId>` of parent-attempt-nodes whose visibility is
   340	/// suppressed in the agent read-view because they are dominated by a
   341	/// child whose YES-price exceeds the parent's by `policy.price_margin`.
   342	///
   343	/// **Read-view mask, not deletion** (CR-14.3 + SG-14.3 + halt-trigger #3):
   344	/// the `Tape` itself is unchanged; this function only nominates parent
   345	/// IDs for filtering at the scheduler / read-view boundary. The full
   346	/// `tape.nodes()` iteration always yields masked parents.
   347	///
   348	/// **Algorithm** (architect §5.5 + charter §3 Atom 3):
   349	/// for each `(parent_id, parent_entry)` in `price_index`:
   350	///   for each `child_id` in `tape.children(parent_id)`:
   351	///     - if `child.liquidity_depth < policy.min_liquidity`: skip (CR-14.4 / SG-14.8)
   352	///     - if any `ChallengeCase` targets `child_id` with status `Open`:
   353	///       skip (CR-14.5 / SG-14.7 / halt-trigger #6)
   354	///     - if `child.price_yes >= parent.price_yes + policy.price_margin`
   355	///       (cross-multiplication; no division): insert `parent_id` and
   356	///       move on to next parent (one dominating child suffices).
   357	///
   358	/// **Determinism**: `BTreeMap` / `BTreeSet` ordering on `TxId` (lexicographic
   359	/// on inner `String`); tape children iteration is in append order; output
   360	/// `BTreeSet` is therefore deterministic given identical inputs.
   361	///
   362	/// **Returns parent IDs only**, not (parent, child) edges; the mask is a
   363	/// flat set per FR-14.5 + charter §7 auto-resolution B (global v0).
   364	pub fn compute_mask_set(
   365	    econ: &EconomicState,
   366	    tape: &Tape,
   367	    policy: &BoltzmannMaskPolicy,
   368	    price_index: &BTreeMap<TxId, NodeMarketEntry>,
   369	) -> BTreeSet<TxId> {
   370	    let mut mask: BTreeSet<TxId> = BTreeSet::new();
   371	
   372	    // Build a quick lookup: child_node_id → has any Open challenge targeting it?
   373	    // ChallengeCasesIndex shape: BTreeMap<TxId (challenge_id), ChallengeCase>.
   374	    // Each ChallengeCase has target_work_tx + status. We index by target_work_tx
   375	    // and treat Open as the only blocking status (Released / UpheldDeferred
   376	    // are resolved per CR-14.5 + SG-14.7 — those do not block masking).
   377	    let mut open_challenge_targets: BTreeSet<&TxId> = BTreeSet::new();
   378	    for case in econ.challenge_cases_t.0.values() {
   379	        if case.status == ChallengeStatus::Open {
   380	            open_challenge_targets.insert(&case.target_work_tx);
   381	        }
   382	    }
   383	
   384	    for (parent_id, parent_entry) in price_index.iter() {
   385	        // Parent without YES price is uninformative (zero-liquidity per
   386	        // FR-14.3); cannot be dominated by any child via price_margin.
   387	        let parent_price_yes = match parent_entry.price_yes.as_ref() {
   388	            Some(p) => p,
   389	            None => continue,
   390	        };
   391	
   392	        // Look up children via Tape parent-edges.
   393	        for child_node_id in tape.children(parent_id.0.as_str()) {
   394	            let child_tx_id = TxId(child_node_id.clone());
   395	
   396	            // Look up child's NodeMarketEntry.
   397	            let child_entry = match price_index.get(&child_tx_id) {
   398	                Some(e) => e,
   399	                None => continue,
   400	            };
   401	
   402	            // CR-14.4 / SG-14.8: low-liquidity children cannot mask parent.
   403	            if child_entry.liquidity_depth.micro_units()
   404	                < policy.min_liquidity.micro_units()
   405	            {
   406	                continue;
   407	            }
   408	
   409	            // CR-14.5 / SG-14.7 / halt-trigger #6: open challenge blocks masking.
   410	            if open_challenge_targets.contains(&child_tx_id) {
   411	                continue;
   412	            }
   413	
   414	            // FR-14.5 dominance check: child.price_yes >= parent.price_yes
   415	            // + price_margin (computed via cross-multiplication; no division).
   416	            let child_price_yes = match child_entry.price_yes.as_ref() {
   417	                Some(p) => p,
   418	                None => continue,
   419	            };
   420	
   421	            if child_price_yes.dominates_by(parent_price_yes, &policy.price_margin) {
   422	                mask.insert(parent_id.clone());
   423	                break; // One dominating child suffices.
   424	            }
   425	        }
   426	    }
   427	
   428	    mask
   429	}
   430	
   431	// ─────────────────────────────────────────────────────────────────────────
   432	// Inline unit tests — pure-fn coverage of FR-14.1..3 + determinism +
   433	// rational-equality invariant. The decimal-float fence test lives in
   434	// `tests/tb_14_halt_triggers.rs` (halt-trigger #4); this module never
   435	// reads its own source.
   436	// ─────────────────────────────────────────────────────────────────────────
   437	
   438	#[cfg(test)]
   439	mod tests {
   440	    use super::*;
   441	    use crate::state::typed_tx::PositionKind;
   442	    use std::collections::BTreeMap as Map;
   443	
   444	    fn micro(units: i64) -> MicroCoin {
   445	        MicroCoin::from_micro_units(units)
   446	    }
   447	
   448	    fn make_position(
   449	        position_id: &str,
   450	        node_id: &str,
   560	                400_000,
   561	            ),
   562	            make_position(
   563	                "p2",
   564	                "n1",
   565	                "t1",
   566	                "a2",
   567	                PositionSide::Short,
   568	                PositionKind::ChallengeShort,
   569	                400_000,
   570	            ),
   571	        ]);
   572	        let idx = compute_price_index(&econ);
   573	        let entry = idx.get(&TxId("n1".into())).expect("n1 present");
   574	        assert_eq!(
   575	            entry.price_yes,
   576	            Some(RationalPrice {
   577	                numerator: 400_000,
   578	                denominator: 800_000
   579	            })
   580	        );
   581	        assert_eq!(
   582	            entry.price_no,
   583	            Some(RationalPrice {
   584	                numerator: 400_000,
   585	                denominator: 800_000
   586	            })
   587	        );
   588	    }
   589	
   590	    #[test]
   591	    fn rational_equality_invariant() {
   592	        // For any non-zero-liquidity node:
   593	        //   price_yes.num + price_no.num == price_yes.den == price_no.den.
   594	        let econ = econ_with_positions(vec![
   595	            make_position(
   596	                "p1",
   597	                "n1",
   598	                "t1",
   599	                "a1",
   600	                PositionSide::Long,
   601	                PositionKind::FirstLong,
   602	                700_000,
   603	            ),
   604	            make_position(
   605	                "p2",
   606	                "n1",
   607	                "t1",
   608	                "a2",
   609	                PositionSide::Short,
   610	                PositionKind::ChallengeShort,
   611	                300_000,
   612	            ),
   613	        ]);
   614	        let idx = compute_price_index(&econ);
   615	        let entry = idx.get(&TxId("n1".into())).expect("n1 present");
   616	        let py = entry.price_yes.expect("price_yes present");
   617	        let pn = entry.price_no.expect("price_no present");
   618	        assert_eq!(py.denominator, pn.denominator, "denominators must match");
   619	        assert_eq!(
   620	            py.numerator + pn.numerator,
   621	            py.denominator,
   622	            "rational equality: long_n + short_n == total"
   623	        );
   624	    }
   625	
   626	    #[test]
   627	    fn determinism_n_calls_identical() {
   628	        let econ = econ_with_positions(vec![
   629	            make_position(
   630	                "p1",
   631	                "n1",
   632	                "t1",
   633	                "a1",
   634	                PositionSide::Long,
   635	                PositionKind::FirstLong,
   636	                700_000,
   637	            ),
   638	            make_position(
   639	                "p2",
   640	                "n2",
   641	                "t2",
   642	                "a2",
   643	                PositionSide::Short,
   644	                PositionKind::ChallengeShort,
   645	                300_000,
   646	            ),
   647	            make_position(
   648	                "p3",
   649	                "n1",
   650	                "t1",
   651	                "a3",
   652	                PositionSide::Short,
   653	                PositionKind::ChallengeShort,
   654	                100_000,
   655	            ),
   656	        ]);
   657	        let first = compute_price_index(&econ);
   658	        for _ in 0..10 {
   659	            assert_eq!(
   660	                compute_price_index(&econ),
   661	                first,
   662	                "deterministic across calls"
   663	            );
   664	        }
   665	    }
   666	
   667	    #[test]
   668	    fn yes_share_depth_aggregates_across_owners() {
   669	        use crate::state::typed_tx::EventId;
   670	        // Two owners hold YES/NO shares for event_id=task_x; depths sum.
   671	        let mut econ = econ_with_positions(vec![make_position(
   672	            "p1",
   673	            "node_x",
   674	            "task_x",
   675	            "a1",
   676	            PositionSide::Long,
   677	            PositionKind::FirstLong,
   678	            100_000,
   679	        )]);
   680	        let event_id = EventId(TaskId("task_x".into()));
   681	        let mut a1_map: Map<EventId, ShareSidePair> = Map::new();
   682	        a1_map.insert(
   683	            event_id.clone(),
   684	            ShareSidePair {
   685	                yes: ShareAmount::from_units(50_000),
   686	                no: ShareAmount::from_units(20_000),
   687	            },
   688	        );
   689	        let mut a2_map: Map<EventId, ShareSidePair> = Map::new();
   690	        a2_map.insert(
   691	            event_id.clone(),
   692	            ShareSidePair {
   693	                yes: ShareAmount::from_units(70_000),
   694	                no: ShareAmount::from_units(30_000),
   695	            },
   696	        );
   697	        econ.conditional_share_balances_t
   698	            .0
   699	            .insert(AgentId("a1".into()), a1_map);
   700	        econ.conditional_share_balances_t
   701	            .0
   702	            .insert(AgentId("a2".into()), a2_map);
   703	
   704	        let idx = compute_price_index(&econ);
   705	        let entry = idx.get(&TxId("node_x".into())).expect("node_x present");
   706	        assert_eq!(entry.yes_share_depth, ShareAmount::from_units(120_000));
   707	        assert_eq!(entry.no_share_depth, ShareAmount::from_units(50_000));
   708	    }
   709	
   710	    #[test]
   711	    fn rational_dominates_by() {
   712	        // 0.60 - 0.40 = 0.20 >= 0.10 ✓
   713	        let p60 = RationalPrice {
   714	            numerator: 60,
   715	            denominator: 100,
   716	        };
   717	        let p40 = RationalPrice {
   718	            numerator: 40,
   719	            denominator: 100,
   720	        };

exec
/bin/bash -lc "nl -ba src/bus.rs | sed -n '1,140p;430,540p;680,780p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 4: TSP Event Bus — SKILL lifecycle serial reactor
     2	// Constitutional basis: Art. II (selective broadcast), Art. III (selective shielding)
     3	// V3L-11: serial reactor for causal ordering (no concurrent pricing oscillation)
     4	// V3L-21: one-step-per-node payload limits
     5	// V3L-31: supervisor loop, never silent exit
     6	// V3L-32: cascade failure protection
     7	
     8	use crate::kernel::{Kernel, KernelError};
     9	use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
    10	use crate::sdk::tool::{ToolSignal, TuringTool};
    11	use crate::state::sequencer::{Sequencer, SubmissionReceipt, SubmitError};
    12	use crate::state::typed_tx::TypedTx;
    13	use serde::{Deserialize, Serialize};
    14	use std::collections::HashMap;
    15	use std::sync::Arc;
    16	
    17	// ── Symbolic constants (V-01 ceremonial kill per D-VETO-7 ratified A) ──────────
    18	
    19	/// TRACE_MATRIX FC1-Cost / FC3-Cost: placeholder until CO1.1.4 STEP_B propagates
    20	/// real LLM completion tokens from `drivers::llm_http::LlmResponse` through to
    21	/// `Node::completion_tokens`. CO1.1.4-pre1 ceremonial commit replaces the magic
    22	/// literal `0` at `bus.rs:268` with this named constant; the value is unchanged
    23	/// (still 0), but the literal is killed so the STEP_B refactor has a clear
    24	/// rename target rather than an anonymous integer.
    25	///
    26	/// See `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md` § 2.2
    27	/// D-VETO-7 for the ratified disposition.
    28	pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4: u32 = 0;
    29	
    30	// ── Configuration ───────────────────────────────────────────────
    31	
    32	/// Bus configuration. V3L-23: no hardcoded values, all configurable.
    33	///
    34	/// TB-14 Atom 6 (2026-05-03): `system_lp_amount: f64` was excised together
    35	/// with `kernel.create_market` (legacy CPMM scaffolding). Pricing is now a
    36	/// derived view over `EconomicState` via `state::compute_price_index`; no
    37	/// LP injection at bus level.
    38	pub struct BusConfig {
    39	    pub max_payload_chars: usize,
    40	    pub max_payload_lines: usize,
    41	    pub forbidden_patterns: Vec<String>,
    42	}
    43	
    44	impl Default for BusConfig {
    45	    fn default() -> Self {
    46	        BusConfig {
    47	            max_payload_chars: 1600,
    48	            max_payload_lines: 24,
    49	            forbidden_patterns: Vec::new(),
    50	        }
    51	    }
    52	}
    53	
    54	// ── Core Bus ────────────────────────────────────────────────────
    55	
    56	/// The serial event reactor.
    57	/// V3L-11: ALL state mutations go through this single-threaded reactor.
    58	/// No concurrent access to kernel/markets — causal ordering guaranteed.
    59	pub struct TuringBus {
    60	    pub kernel: Kernel,
    61	    pub ledger: Ledger,
    62	    pub tools: Vec<Box<dyn TuringTool>>,
    63	    pub config: BusConfig,
    64	    pub clock: u64,
    65	    pub tx_count: u64,
    66	    pub generation: u32,
    67	    graveyard: HashMap<String, Vec<String>>,
    68	    // Phase 1 (C-037 candidate): durable Q_t. None = legacy in-memory mode.
    69	    wal: Option<crate::wal::Wal>,
    70	    /// CO1.7-extra D3: typed-tx Sequencer; `None` when bus runs in legacy
    71	    /// ledger-only mode. Spec § 2.1 + D3 STEP_B Branch A. `#[serde(skip)]`
    72	    /// is conditional on TuringBus having serde derives — it currently
    73	    /// does not (per `pub struct TuringBus` declaration above), so the
    74	    /// attribute is omitted at this landing. If a future atom adds serde
    75	    /// to TuringBus, the skip MUST be added in the same patch.
    76	    pub sequencer: Option<Arc<Sequencer>>,
    77	}
    78	
    79	/// Scope for recent_rejections query.
    80	/// Step-B v3 Art. II.1 fix: enables global abstract-broadcast without violating C-022.
    81	#[derive(Debug, Clone, Copy)]
    82	pub enum RejectionScope {
    83	    /// Legacy: per-author graveyard (before-fix behavior).
    84	    PerAuthor,
    85	    /// Flattened across all authors, chronological (may leak raw content — use with caution).
    86	    Global,
    87	    /// Art. II.1 compliant: counted + top-k class labels. Requires callers to record class labels.
    88	    TopKClasses(usize),
    89	}
    90	
    91	/// Result of a bus append operation.
    92	///
    93	/// TB-14 Atom 6 follow-up (2026-05-03; closing internal auditor F1):
    94	/// dead `Invested { node_id, shares: f64 }` variant excised — was a
    95	/// pre-TB-9 invest-path residual with zero call sites and zero match
    96	/// arms (`grep -rn "BusResult::Invested\|Invested {"` returned only
    97	/// its own declaration site). Closes G-14.11 "no f64 in TB-14 module
    98	/// surface" residual flagged by the internal Class 3 audit.
    99	#[derive(Debug)]
   100	pub enum BusResult {
   101	    Appended { node_id: NodeId },
   102	    Vetoed { reason: String },
   103	}
   104	
   105	impl TuringBus {
   106	    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
   107	        TuringBus {
   108	            kernel,
   109	            ledger: Ledger::new(),
   110	            tools: Vec::new(),
   111	            config,
   112	            clock: 0,
   113	            tx_count: 0,
   114	            generation: 0,
   115	            graveyard: HashMap::new(),
   116	            wal: None,
   117	            sequencer: None,
   118	        }
   119	    }
   120	
   121	    /// CO1.7-extra D3: opt-in constructor wiring a typed-tx Sequencer
   122	    /// alongside the legacy ledger. Spec § 2.1 + § 2.2 (Sequencer lives at
   123	    /// TuringBus level, not nested through Kernel).
   124	    ///
   125	    /// TRACE_MATRIX § 5.2.1 — single-writer entry-point.
   126	    pub fn with_sequencer(
   127	        kernel: Kernel,
   128	        config: BusConfig,
   129	        sequencer: Arc<Sequencer>,
   130	    ) -> Self {
   131	        let mut bus = Self::new(kernel, config);
   132	        bus.sequencer = Some(sequencer);
   133	        bus
   134	    }
   135	
   136	    /// CO1.7-extra D3: typed-tx submission path. Returns receipt
   137	    /// (`submit_id`) immediately; commit happens asynchronously in
   138	    /// `Sequencer::run` driver loop.
   139	    ///
   140	    /// Returns `Err(SubmitError::QueueClosed)` when the bus runs in
   430	    /// Call sites that explicitly want per-author scope use `recent_rejections_scoped`.
   431	    pub fn recent_rejections(&self, author: &str, max: usize) -> Vec<String> {
   432	        self.recent_rejections_scoped(author, max, RejectionScope::TopKClasses(3))
   433	    }
   434	
   435	    /// Scoped rejection query (Step-B v3 Art. II.1 fix).
   436	    pub fn recent_rejections_scoped(
   437	        &self,
   438	        author: &str,
   439	        max: usize,
   440	        scope: RejectionScope,
   441	    ) -> Vec<String> {
   442	        match scope {
   443	            RejectionScope::PerAuthor => {
   444	                self.graveyard.get(author)
   445	                    .map(|v| v.iter().rev().take(max).cloned().collect())
   446	                    .unwrap_or_default()
   447	            }
   448	            RejectionScope::Global => {
   449	                // Flatten all authors' recent; keep most recent `max` across swarm.
   450	                let mut all: Vec<&String> = self.graveyard.values().flatten().collect();
   451	                // Heuristic: assume push-order ~= time-order; take last `max` global entries.
   452	                let start = all.len().saturating_sub(max);
   453	                all.drain(..start);
   454	                all.into_iter().cloned().collect()
   455	            }
   456	            RejectionScope::TopKClasses(k) => {
   457	                // C-022 shield: broadcast abstracted CLASSES with COUNTS, not raw strings.
   458	                // Expects reason strings to already be class labels (see error_abstraction).
   459	                let mut counts: HashMap<String, u32> = HashMap::new();
   460	                for v in self.graveyard.values() {
   461	                    for r in v {
   462	                        *counts.entry(r.clone()).or_insert(0) += 1;
   463	                    }
   464	                }
   465	                let mut sorted: Vec<(String, u32)> = counts.into_iter().collect();
   466	                // Sort: count DESC, then alphabetical (tiebreak stable).
   467	                sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
   468	                sorted.truncate(k);
   469	                // Emit as "label(count)" strings for prompt.
   470	                sorted.into_iter()
   471	                    .map(|(lbl, c)| format!("{}({})", lbl, c))
   472	                    .take(max)
   473	                    .collect()
   474	            }
   475	        }
   476	    }
   477	
   478	    /// Get a snapshot of the universe for agents to read.
   479	    ///
   480	    /// TRACE_MATRIX TB-14 Atom 6 (FC2-N28 + FC3-N42; architect §5.1 +
   481	    /// charter §3 Atom 6): the snapshot now carries the integer-rational
   482	    /// `price_index` + `mask_set` derived from canonical `EconomicState`
   483	    /// via `state::compute_price_index` + `state::compute_mask_set`,
   484	    /// replacing the legacy decimal-float `markets: HashMap<_, MarketSnapshot>`
   485	    /// CPMM read-view excised together with `src/prediction_market.rs`.
   486	    ///
   487	    /// **Halt-trigger #2 spirit preserved**: bus.rs imports TB-14 types
   488	    /// (this is the legitimate broadcast point per kickoff doc), but the
   489	    /// L4/L4.E classification path in `Sequencer::dispatch_transition`
   490	    /// remains free of TB-14 imports — verified by halt-trigger #2's
   491	    /// `use`-statement scan over `src/state/sequencer.rs`. The price
   492	    /// signal flows: `EconomicState (canonical)` →
   493	    /// `compute_price_index (pure derive)` → snapshot read-view →
   494	    /// scheduler / dashboard / agent prompt. It NEVER flows back into
   495	    /// `dispatch_transition`.
   496	    ///
   497	    /// **Replay-deterministic** (Art.0.2): `compute_price_index` and
   498	    /// `compute_mask_set` are pure over their inputs. The snapshot's
   499	    /// `price_index` / `mask_set` are reproducible from any byte-equal
   500	    /// `EconomicState` + `Tape` + `BoltzmannMaskPolicy` without re-running
   501	    /// the run.
   502	    ///
   503	    /// **Sequencer-optional**: when the bus runs in legacy ledger-only
   504	    /// mode (`sequencer == None`, e.g. in WAL-only smoke tests), the
   505	    /// price_index + mask_set are empty `BTreeMap` / `BTreeSet`. Callers
   506	    /// (evaluator, dashboard) treat empty as "no signal yet" — they MUST
   507	    /// NOT crash on empty.
   508	    pub fn snapshot(&self) -> crate::sdk::snapshot::UniverseSnapshot {
   509	        let policy = crate::state::BoltzmannMaskPolicy::from_env();
   510	
   511	        let (price_index, mask_set) = match self.sequencer.as_ref() {
   512	            Some(seq) => match seq.q_snapshot() {
   513	                Ok(q) => {
   514	                    let pi = crate::state::compute_price_index(&q.economic_state_t);
   515	                    let ms = crate::state::compute_mask_set(
   516	                        &q.economic_state_t,
   517	                        &self.kernel.tape,
   518	                        &policy,
   519	                        &pi,
   520	                    );
   521	                    (pi, ms)
   522	                }
   523	                Err(_) => (
   524	                    std::collections::BTreeMap::new(),
   525	                    std::collections::BTreeSet::new(),
   526	                ),
   527	            },
   528	            None => (
   529	                std::collections::BTreeMap::new(),
   530	                std::collections::BTreeSet::new(),
   531	            ),
   532	        };
   533	
   534	        crate::sdk::snapshot::UniverseSnapshot {
   535	            tape: self.kernel.tape.clone(),
   536	            price_index,
   537	            mask_set,
   538	            generation: self.generation,
   539	            tx_count: self.tx_count,
   540	        }
   680	        assert_eq!(TuringBus::bus_classify("err:tactic_linarith"), "err:tactic_linarith");
   681	        assert_eq!(TuringBus::bus_classify("err:unknown_variant_we_dont_track"), "err:other");
   682	        assert_eq!(TuringBus::bus_classify("some unprecedented garbage"), "err:other");
   683	    }
   684	
   685	    #[test]
   686	    fn test_bus_snapshot() {
   687	        // TB-14 Atom 6: snapshot.markets HashMap was replaced by
   688	        // price_index: BTreeMap<TxId, NodeMarketEntry> + mask_set: BTreeSet<TxId>.
   689	        // Without a sequencer wired (legacy ledger-only mode), both are empty
   690	        // — the bus snapshot is sequencer-optional per CR-14.x; consumers
   691	        // (evaluator, dashboard) treat empty as "no signal yet".
   692	        let mut bus = make_bus();
   693	        bus.append("A0", "step 1", None).unwrap();
   694	        let snap = bus.snapshot();
   695	        assert_eq!(snap.tx_count, 1);
   696	        assert!(snap.price_index.is_empty(), "no sequencer → empty price_index");
   697	        assert!(snap.mask_set.is_empty(), "no sequencer → empty mask_set");
   698	        assert!(snap.tape.get(&"tx_0_by_A0".to_string()).is_some(),
   699	                "appended node is in tape regardless of price index state");
   700	    }
   701	
   702	    #[test]
   703	    fn test_bus_serial_ordering() {
   704	        // V3L-11: tx_count must increment monotonically
   705	        let mut bus = make_bus();
   706	        for i in 0..5 {
   707	            bus.append("A0", &format!("step {}", i), None).unwrap();
   708	        }
   709	        assert_eq!(bus.tx_count, 5);
   710	        assert_eq!(bus.clock, 5);
   711	    }
   712	}

exec
/bin/bash -lc "nl -ba src/sdk/snapshot.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 2: Immutable universe snapshot — agents read, never mutate
     2	// Constitutional basis: Art. III.3 (decorrelation via independent snapshots)
     3	//
     4	// TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
     5	// Legacy decimal-float `MarketSnapshot` + `UniverseSnapshot.markets`
     6	// HashMap CPMM read-view was excised together with `prediction_market.rs`.
     7	// The snapshot now carries integer-rational `price_index` + `mask_set`
     8	// derived from canonical `EconomicState` via `state::compute_price_index`
     9	// + `state::compute_mask_set`. Pricing is signal, not truth.
    10	//
    11	// Dead post-TB-9-collapse `balances: HashMap<String, f64>` and
    12	// `portfolios: HashMap<String, HashMap<NodeId, (f64, f64, f64)>>` were
    13	// also retired in this atom — bus.snapshot already populated both with
    14	// empty HashMaps (no live values flowed through them). Removal is purely
    15	// additive cleanup that closes the f64 surface in this file under the
    16	// G-14.11 "no f64 in TB-14 module surface" ship gate.
    17	
    18	use crate::ledger::Tape;
    19	use crate::state::{NodeMarketEntry, TxId};
    20	use serde::{Deserialize, Serialize};
    21	use std::collections::{BTreeMap, BTreeSet};
    22	
    23	/// Complete frozen state of the universe.
    24	/// Agents receive this as read-only input — they cannot mutate it.
    25	/// Art. III.3: each agent sees the same snapshot, maintaining decorrelation.
    26	///
    27	/// TRACE_MATRIX TB-14 Atom 6 (FC2-N28 + FC3-N42; architect §5.1 + charter
    28	/// §3 Atom 6): the snapshot's price-signal surface.
    29	///
    30	/// Field semantics:
    31	/// - `tape` — the current `Tape` (DAG of attempt nodes); read-only mirror.
    32	/// - `price_index` — derived `BTreeMap<TxId, NodeMarketEntry>` per
    33	///   `compute_price_index(econ)`. Empty when bus runs sequencer-less.
    34	/// - `mask_set` — derived `BTreeSet<TxId>` per `compute_mask_set(...)`.
    35	///   Empty when bus runs sequencer-less. Mask is read-view only — masked
    36	///   parents remain in `tape.nodes()` (CR-14.3 / SG-14.3 / halt-trigger #3).
    37	/// - `generation`, `tx_count` — bus-level counters, unchanged from TB-3.
    38	#[derive(Debug, Clone, Serialize, Deserialize)]
    39	pub struct UniverseSnapshot {
    40	    pub tape: Tape,
    41	    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
    42	    pub mask_set: BTreeSet<TxId>,
    43	    pub generation: u32,
    44	    pub tx_count: u64,
    45	}
    46	
    47	#[cfg(test)]
    48	mod tests {
    49	    use super::*;
    50	
    51	    #[test]
    52	    fn test_snapshot_default_empty_signal_surface() {
    53	        // TB-14 Atom 6: a freshly-constructed snapshot has empty
    54	        // price_index + mask_set; consumers (evaluator / dashboard) must
    55	        // tolerate this as "no signal yet" without crashing.
    56	        let snap = UniverseSnapshot {
    57	            tape: Tape::new(),
    58	            price_index: BTreeMap::new(),
    59	            mask_set: BTreeSet::new(),
    60	            generation: 0,
    61	            tx_count: 0,
    62	        };
    63	        assert!(snap.price_index.is_empty());
    64	        assert!(snap.mask_set.is_empty());
    65	        assert_eq!(snap.generation, 0);
    66	        assert_eq!(snap.tx_count, 0);
    67	    }
    68	}

exec
/bin/bash -lc "nl -ba src/state/price_index.rs | sed -n '180,330p;720,960p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	        // for this event_id. The conditional_share_balances_t shape is
   181	        // `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
   182	        let mut yes_share_total: u128 = 0;
   183	        let mut no_share_total: u128 = 0;
   184	        for owner_map in econ.conditional_share_balances_t.0.values() {
   185	            if let Some(pair) = owner_map.get(&event_id) {
   186	                yes_share_total = yes_share_total.saturating_add(pair.yes.units);
   187	                no_share_total = no_share_total.saturating_add(pair.no.units);
   188	            }
   189	        }
   190	
   191	        out.insert(
   192	            node_id.clone(),
   193	            NodeMarketEntry {
   194	                node_id,
   195	                task_id,
   196	                event_id,
   197	                long_interest: to_micro(long_micro),
   198	                short_interest: to_micro(short_micro),
   199	                yes_share_depth: ShareAmount::from_units(yes_share_total),
   200	                no_share_depth: ShareAmount::from_units(no_share_total),
   201	                price_yes,
   202	                price_no,
   203	                liquidity_depth: to_micro(total_micro),
   204	            },
   205	        );
   206	    }
   207	
   208	    out
   209	}
   210	
   211	// ─────────────────────────────────────────────────────────────────────────
   212	// BoltzmannMaskPolicy — architect §5.2 verbatim shape (skeleton in Atom 3;
   213	// `from_env()` constructor lands in Atom 4 per charter §3 split).
   214	// ─────────────────────────────────────────────────────────────────────────
   215	
   216	/// TRACE_MATRIX TB-14 Atom 3 (architect §5.2 verbatim; FC2-N28 + FC2-N29
   217	/// configuration carrier): integer-rational policy parameters for the
   218	/// Boltzmann scheduler mask + epsilon-greedy exploration + price-margin
   219	/// gate. **Atom 3 ships the data shape only**; `from_env()` env-var
   220	/// loader lands in Atom 4 per charter §3 (separate iter-cap tracking).
   221	///
   222	/// Field semantics:
   223	/// - `beta_num` / `beta_den` — rational temperature for argmax tiebreaking
   224	///   (Atom 5 boltzmann_select_parent_v2 uses these). Default = 1/1.
   225	/// - `min_liquidity` — `child.liquidity_depth` floor below which child
   226	///   cannot mask parent (CR-14.4). Default = 1 Coin (1_000_000 micro).
   227	/// - `price_margin` — minimum dominance gap for child to mask parent
   228	///   (FR-14.5 / SG-14.x). Default = 1/10 (10% margin).
   229	/// - `epsilon_exploration_num` / `epsilon_exploration_den` — random
   230	///   exploration probability in argmax + epsilon-greedy (SG-14.5).
   231	///   Default = 1/10 (10% exploration).
   232	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   233	pub struct BoltzmannMaskPolicy {
   234	    pub beta_num: i64,
   235	    pub beta_den: i64,
   236	    pub min_liquidity: MicroCoin,
   237	    pub price_margin: RationalPrice,
   238	    pub epsilon_exploration_num: u64,
   239	    pub epsilon_exploration_den: u64,
   240	}
   241	
   242	impl Default for BoltzmannMaskPolicy {
   243	    fn default() -> Self {
   244	        Self {
   245	            beta_num: 1,
   246	            beta_den: 1,
   247	            min_liquidity: MicroCoin::from_micro_units(1_000_000),
   248	            price_margin: RationalPrice {
   249	                numerator: 1,
   250	                denominator: 10,
   251	            },
   252	            epsilon_exploration_num: 1,
   253	            epsilon_exploration_den: 10,
   254	        }
   255	    }
   256	}
   257	
   258	impl BoltzmannMaskPolicy {
   259	    /// TRACE_MATRIX TB-14 Atom 4 (FC2-N28 + FC2-N29 configuration loader;
   260	    /// charter §3 Atom 4): build a policy from process environment with
   261	    /// per-field defaults matching `BoltzmannMaskPolicy::default()`.
   262	    ///
   263	    /// **All 7 fields are integer-rational** (architect §5.6 forbidden:
   264	    /// no decimal float anywhere in the policy surface). The seven env
   265	    /// vars are: `BOLTZMANN_BETA_NUM` / `BOLTZMANN_BETA_DEN`,
   266	    /// `BOLTZMANN_MIN_LIQUIDITY_MICRO`, `BOLTZMANN_PRICE_MARGIN_NUM` /
   267	    /// `BOLTZMANN_PRICE_MARGIN_DEN`, `BOLTZMANN_EPSILON_NUM` /
   268	    /// `BOLTZMANN_EPSILON_DEN`. Unparsable values silently fall back to
   269	    /// the field default — fail-soft is the right policy for a scheduler
   270	    /// hyperparameter loader (Art.I.1 + C-027: env-overridable, never
   271	    /// hardcoded; misconfiguration must not crash the swarm).
   272	    ///
   273	    /// **Determinism note**: this function reads `std::env::var` and is
   274	    /// therefore NOT replay-deterministic by itself. Production callers
   275	    /// (Atom 6 evaluator wire-up) load the policy ONCE at run start, then
   276	    /// pass it as an explicit input to `compute_mask_set` /
   277	    /// `boltzmann_select_parent_v2`, which ARE deterministic given the
   278	    /// fixed policy. This separation preserves Art.0.2 replay-determinism
   279	    /// at the deterministic boundary while allowing operational tuning at
   280	    /// the env-var boundary.
   281	    pub fn from_env() -> Self {
   282	        let default = Self::default();
   283	
   284	        fn parse_i64(key: &str, fallback: i64) -> i64 {
   285	            std::env::var(key)
   286	                .ok()
   287	                .and_then(|s| s.parse::<i64>().ok())
   288	                .unwrap_or(fallback)
   289	        }
   290	        fn parse_u64(key: &str, fallback: u64) -> u64 {
   291	            std::env::var(key)
   292	                .ok()
   293	                .and_then(|s| s.parse::<u64>().ok())
   294	                .unwrap_or(fallback)
   295	        }
   296	        fn parse_u128(key: &str, fallback: u128) -> u128 {
   297	            std::env::var(key)
   298	                .ok()
   299	                .and_then(|s| s.parse::<u128>().ok())
   300	                .unwrap_or(fallback)
   301	        }
   302	
   303	        Self {
   304	            beta_num: parse_i64("BOLTZMANN_BETA_NUM", default.beta_num),
   305	            beta_den: parse_i64("BOLTZMANN_BETA_DEN", default.beta_den),
   306	            min_liquidity: MicroCoin::from_micro_units(parse_i64(
   307	                "BOLTZMANN_MIN_LIQUIDITY_MICRO",
   308	                default.min_liquidity.micro_units(),
   309	            )),
   310	            price_margin: RationalPrice {
   311	                numerator: parse_u128(
   312	                    "BOLTZMANN_PRICE_MARGIN_NUM",
   313	                    default.price_margin.numerator,
   314	                ),
   315	                denominator: parse_u128(
   316	                    "BOLTZMANN_PRICE_MARGIN_DEN",
   317	                    default.price_margin.denominator,
   318	                ),
   319	            },
   320	            epsilon_exploration_num: parse_u64(
   321	                "BOLTZMANN_EPSILON_NUM",
   322	                default.epsilon_exploration_num,
   323	            ),
   324	            epsilon_exploration_den: parse_u64(
   325	                "BOLTZMANN_EPSILON_DEN",
   326	                default.epsilon_exploration_den,
   327	            ),
   328	        }
   329	    }
   330	}
   720	        };
   721	        let m10 = RationalPrice {
   722	            numerator: 1,
   723	            denominator: 10,
   724	        };
   725	        assert!(p60.dominates_by(&p40, &m10));
   726	        assert!(!p40.dominates_by(&p60, &m10));
   727	        // 0.60 vs 0.50 by 0.20 → does not dominate.
   728	        let p50 = RationalPrice {
   729	            numerator: 50,
   730	            denominator: 100,
   731	        };
   732	        let m20 = RationalPrice {
   733	            numerator: 1,
   734	            denominator: 5,
   735	        };
   736	        assert!(!p60.dominates_by(&p50, &m20));
   737	        // 0.70 vs 0.50 by 0.20 → dominates.
   738	        let p70 = RationalPrice {
   739	            numerator: 70,
   740	            denominator: 100,
   741	        };
   742	        assert!(p70.dominates_by(&p50, &m20));
   743	        // Defense: zero denominator returns false.
   744	        let zero_den = RationalPrice {
   745	            numerator: 1,
   746	            denominator: 0,
   747	        };
   748	        assert!(!p60.dominates_by(&zero_den, &m10));
   749	        assert!(!zero_den.dominates_by(&p60, &m10));
   750	    }
   751	
   752	    // ──────────── BoltzmannMaskPolicy::from_env (Atom 4) ────────────
   753	    //
   754	    // Tests mutate process-global env vars; serialize with a static Mutex
   755	    // per `feedback_env_var_test_lock` (cargo's default test runner is
   756	    // parallel across threads within a single test binary).
   757	
   758	    use std::sync::Mutex;
   759	    static ENV_LOCK: Mutex<()> = Mutex::new(());
   760	
   761	    fn with_env_isolated<F: FnOnce()>(f: F) {
   762	        let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
   763	        let keys = [
   764	            "BOLTZMANN_BETA_NUM",
   765	            "BOLTZMANN_BETA_DEN",
   766	            "BOLTZMANN_MIN_LIQUIDITY_MICRO",
   767	            "BOLTZMANN_PRICE_MARGIN_NUM",
   768	            "BOLTZMANN_PRICE_MARGIN_DEN",
   769	            "BOLTZMANN_EPSILON_NUM",
   770	            "BOLTZMANN_EPSILON_DEN",
   771	        ];
   772	        for k in &keys {
   773	            std::env::remove_var(k);
   774	        }
   775	        f();
   776	        for k in &keys {
   777	            std::env::remove_var(k);
   778	        }
   779	    }
   780	
   781	    #[test]
   782	    fn boltzmann_mask_policy_default_matches_field_defaults() {
   783	        let p = BoltzmannMaskPolicy::default();
   784	        assert_eq!(p.beta_num, 1);
   785	        assert_eq!(p.beta_den, 1);
   786	        assert_eq!(p.min_liquidity, MicroCoin::from_micro_units(1_000_000));
   787	        assert_eq!(
   788	            p.price_margin,
   789	            RationalPrice {
   790	                numerator: 1,
   791	                denominator: 10
   792	            }
   793	        );
   794	        assert_eq!(p.epsilon_exploration_num, 1);
   795	        assert_eq!(p.epsilon_exploration_den, 10);
   796	    }
   797	
   798	    #[test]
   799	    fn boltzmann_from_env_with_no_vars_set_matches_default() {
   800	        with_env_isolated(|| {
   801	            let p = BoltzmannMaskPolicy::from_env();
   802	            assert_eq!(p, BoltzmannMaskPolicy::default());
   803	        });
   804	    }
   805	
   806	    #[test]
   807	    fn boltzmann_from_env_overrides_each_field() {
   808	        with_env_isolated(|| {
   809	            std::env::set_var("BOLTZMANN_BETA_NUM", "3");
   810	            std::env::set_var("BOLTZMANN_BETA_DEN", "2");
   811	            std::env::set_var("BOLTZMANN_MIN_LIQUIDITY_MICRO", "5000000");
   812	            std::env::set_var("BOLTZMANN_PRICE_MARGIN_NUM", "7");
   813	            std::env::set_var("BOLTZMANN_PRICE_MARGIN_DEN", "20");
   814	            std::env::set_var("BOLTZMANN_EPSILON_NUM", "3");
   815	            std::env::set_var("BOLTZMANN_EPSILON_DEN", "100");
   816	
   817	            let p = BoltzmannMaskPolicy::from_env();
   818	            assert_eq!(p.beta_num, 3);
   819	            assert_eq!(p.beta_den, 2);
   820	            assert_eq!(p.min_liquidity, MicroCoin::from_micro_units(5_000_000));
   821	            assert_eq!(
   822	                p.price_margin,
   823	                RationalPrice {
   824	                    numerator: 7,
   825	                    denominator: 20
   826	                }
   827	            );
   828	            assert_eq!(p.epsilon_exploration_num, 3);
   829	            assert_eq!(p.epsilon_exploration_den, 100);
   830	        });
   831	    }
   832	
   833	    #[test]
   834	    fn boltzmann_from_env_invalid_values_fall_back_to_defaults() {
   835	        with_env_isolated(|| {
   836	            std::env::set_var("BOLTZMANN_BETA_NUM", "not_a_number");
   837	            std::env::set_var("BOLTZMANN_PRICE_MARGIN_NUM", "");
   838	            std::env::set_var("BOLTZMANN_EPSILON_NUM", "abc");
   839	            // Other vars left unset (also fall back to default).
   840	
   841	            let p = BoltzmannMaskPolicy::from_env();
   842	            assert_eq!(p.beta_num, 1, "invalid value falls back to default 1");
   843	            assert_eq!(
   844	                p.price_margin.numerator, 1,
   845	                "empty value falls back to default 1"
   846	            );
   847	            assert_eq!(
   848	                p.epsilon_exploration_num, 1,
   849	                "non-numeric value falls back to default 1"
   850	            );
   851	        });
   852	    }
   853	
   854	    #[test]
   855	    fn boltzmann_from_env_serde_round_trip() {
   856	        let p = BoltzmannMaskPolicy {
   857	            beta_num: 5,
   858	            beta_den: 3,
   859	            min_liquidity: MicroCoin::from_micro_units(7_000_000),
   860	            price_margin: RationalPrice {
   861	                numerator: 11,
   862	                denominator: 13,
   863	            },
   864	            epsilon_exploration_num: 17,
   865	            epsilon_exploration_den: 19,
   866	        };
   867	        let json = serde_json::to_string(&p).unwrap();
   868	        let back: BoltzmannMaskPolicy = serde_json::from_str(&json).unwrap();
   869	        assert_eq!(p, back, "serde JSON round-trip identity");
   870	    }
   871	
   872	    #[test]
   873	    fn boltzmann_policy_zero_decimal_float_substring() {
   874	        // Defense-in-depth: enforce that no field type accidentally accepts
   875	        // a decimal-float-typed value. This is a structural assertion (the
   876	        // halt-trigger #4 file scan is the canonical fence for the module).
   877	        let p = BoltzmannMaskPolicy::default();
   878	        let json = serde_json::to_string(&p).unwrap();
   879	        assert!(
   880	            !json.contains('.'),
   881	            "BoltzmannMaskPolicy JSON must not contain decimal points (integer-rational only)"
   882	        );
   883	    }
   884	}

exec
/bin/bash -lc "nl -ba src/sdk/actor.rs | sed -n '1,220p;220,440p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 3: Boltzmann scheduler (TB-14 Atom 5 integer-rational v2)
     2	// Constitutional basis: Art. II.2.1 (exploration vs exploitation balance)
     3	// V3L-14: no greedy ArgMax (star topology collapse)
     4	//
     5	// TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
     6	// Legacy decimal-float `BoltzmannParams` / `is_frontier` / `lineage_score`
     7	// / `boltzmann_select_parent` were excised together with
     8	// `src/prediction_market.rs`. The integer-rational
     9	// `boltzmann_select_parent_v2` (Atom 5) is the sole scheduler now;
    10	// production wire-up at `experiments/minif2f_v4/src/bin/evaluator.rs`.
    11	
    12	use rand::Rng;
    13	
    14	/// Agent submission (from agent channel to bus reactor).
    15	#[derive(Debug, Clone)]
    16	pub struct MinerTx {
    17	    pub agent_id: String,
    18	    pub model_name: String,
    19	    pub payload: String,
    20	    pub parent_id: Option<String>,
    21	    pub action_type: String,
    22	    pub completion_tokens: u32,
    23	}
    24	
    25	// ── Boltzmann v2 (TB-14 Atom 5 integer-rational) ─────────────────────────
    26	
    27	/// TRACE_MATRIX TB-14 Atom 5 (FC2-N29; architect §5.5 SG-14.4 + SG-14.5
    28	/// + charter §3 Atom 5): integer-rational Boltzmann scheduler with
    29	/// epsilon-greedy exploration and `mask_set` read-view filter.
    30	///
    31	/// **Algorithm** (charter §7 auto-resolution C: argmax + epsilon-greedy
    32	/// for v0; full softmax deferred to TB-15+ as it would require Q16.16
    33	/// fixed-point exp ~150 LoC):
    34	/// 1. Build the candidate set: every `node_id` in `price_index` whose
    35	///    `price_yes` is `Some(_)` and which is NOT in `mask_set`
    36	///    (FR-14.5 / FR-14.6: read-view filter applied here, not by
    37	///    deleting from `Tape`).
    38	/// 2. If the candidate set is empty, return `None`.
    39	/// 3. With probability `policy.epsilon_exploration_num /
    40	///    policy.epsilon_exploration_den`, return a uniform-random pick
    41	///    (SG-14.5). The denominator must be non-zero; if zero, the
    42	///    epsilon branch is skipped (defensive).
    43	/// 4. Otherwise, return the candidate maximizing `price_yes` via
    44	///    `RationalPrice` cross-multiplication (no division, no decimal
    45	///    float). Ties broken by deterministic `TxId` lexicographic order
    46	///    (BTreeMap iteration is already lex-sorted; first-seen wins).
    47	///
    48	/// **Predicate-blind** (CR-14.1 + halt-trigger #1): this fn is the
    49	/// scheduler's PRIORITY pick, not an acceptance gate. The predicate
    50	/// gate at `sequencer.rs:516-558` is a separate check that rejects
    51	/// proposals with `acceptance.value=false` regardless of which parent
    52	/// was picked here.
    53	///
    54	/// **Determinism**: deterministic given the same `(price_index, mask_set,
    55	/// policy, rng-state)`. Production caller must pass a seeded RNG for
    56	/// replay-determinism.
    57	pub fn boltzmann_select_parent_v2<R: Rng>(
    58	    price_index: &std::collections::BTreeMap<
    59	        crate::state::TxId,
    60	        crate::state::NodeMarketEntry,
    61	    >,
    62	    mask_set: &std::collections::BTreeSet<crate::state::TxId>,
    63	    policy: &crate::state::BoltzmannMaskPolicy,
    64	    rng: &mut R,
    65	) -> Option<crate::state::TxId> {
    66	    // Step 1: candidate set = {node | price_yes is Some AND node not in mask_set}
    67	    let candidates: Vec<&crate::state::TxId> = price_index
    68	        .iter()
    69	        .filter(|(node_id, entry)| {
    70	            entry.price_yes.is_some() && !mask_set.contains(node_id)
    71	        })
    72	        .map(|(node_id, _)| node_id)
    73	        .collect();
    74	
    75	    if candidates.is_empty() {
    76	        return None;
    77	    }
    78	
    79	    // Step 3: epsilon-greedy exploration branch.
    80	    if policy.epsilon_exploration_den > 0 {
    81	        let r: u64 = rng.gen_range(0..policy.epsilon_exploration_den);
    82	        if r < policy.epsilon_exploration_num {
    83	            // Uniform random pick over candidates.
    84	            let idx: usize = rng.gen_range(0..candidates.len());
    85	            return Some(candidates[idx].clone());
    86	        }
    87	    }
    88	
    89	    // Step 4: argmax by price_yes via cross-multiplication; ties by
    90	    // BTreeMap iteration order (lexicographic on TxId.0 String).
    91	    let mut best: Option<&crate::state::TxId> = None;
    92	    let mut best_price: Option<&crate::state::RationalPrice> = None;
    93	    for cand in &candidates {
    94	        let entry = price_index.get(*cand).expect("candidate in index");
    95	        let p = entry.price_yes.as_ref().expect("filtered for Some");
    96	        match best_price {
    97	            None => {
    98	                best = Some(cand);
    99	                best_price = Some(p);
   100	            }
   101	            Some(bp) => {
   102	                // p > bp via cross-multiplication: p.n * bp.d > bp.n * p.d
   103	                let lhs = (p.numerator).saturating_mul(bp.denominator);
   104	                let rhs = (bp.numerator).saturating_mul(p.denominator);
   105	                if lhs > rhs {
   106	                    best = Some(cand);
   107	                    best_price = Some(p);
   108	                }
   109	                // tie (lhs == rhs): keep first-seen (lex order from BTreeMap).
   110	            }
   111	        }
   112	    }
   113	    best.map(|t| t.clone())
   114	}
   115	
   116	// ── Tests ───────────────────────────────────────────────────────
   117	
   118	#[cfg(test)]
   119	mod tests {
   120	    use super::*;
   121	    use crate::state::{BoltzmannMaskPolicy, NodeMarketEntry, RationalPrice, TxId};
   122	    use rand::SeedableRng;
   123	    use std::collections::{BTreeMap, BTreeSet};
   124	
   125	    fn make_entry(price_yes_num: u128, price_yes_den: u128) -> NodeMarketEntry {
   126	        NodeMarketEntry {
   127	            price_yes: if price_yes_den == 0 {
   128	                None
   129	            } else {
   130	                Some(RationalPrice {
   131	                    numerator: price_yes_num,
   132	                    denominator: price_yes_den,
   133	                })
   134	            },
   135	            ..Default::default()
   136	        }
   137	    }
   138	
   139	    #[test]
   140	    fn v2_returns_none_on_empty_index() {
   141	        let pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   142	        let mask: BTreeSet<TxId> = BTreeSet::new();
   143	        let policy = BoltzmannMaskPolicy::default();
   144	        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
   145	        assert!(boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng).is_none());
   146	    }
   147	
   148	    #[test]
   149	    fn v2_returns_none_when_all_candidates_masked() {
   150	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   151	        pi.insert(TxId("n1".into()), make_entry(60, 100));
   152	        pi.insert(TxId("n2".into()), make_entry(80, 100));
   153	        let mut mask: BTreeSet<TxId> = BTreeSet::new();
   154	        mask.insert(TxId("n1".into()));
   155	        mask.insert(TxId("n2".into()));
   156	        let policy = BoltzmannMaskPolicy::default();
   157	        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
   158	        assert!(boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng).is_none());
   159	    }
   160	
   161	    #[test]
   162	    fn v2_skips_zero_liquidity_candidates() {
   163	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   164	        pi.insert(TxId("zero".into()), make_entry(0, 0)); // price_yes = None
   165	        pi.insert(TxId("real".into()), make_entry(60, 100));
   166	        let mask: BTreeSet<TxId> = BTreeSet::new();
   167	        // Disable epsilon exploration to force argmax path (deterministic).
   168	        let policy = BoltzmannMaskPolicy {
   169	            epsilon_exploration_num: 0,
   170	            epsilon_exploration_den: 1,
   171	            ..BoltzmannMaskPolicy::default()
   172	        };
   173	        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
   174	        let pick = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng);
   175	        assert_eq!(
   176	            pick,
   177	            Some(TxId("real".into())),
   178	            "v2 must skip zero-liquidity candidate (price_yes=None)"
   179	        );
   180	    }
   181	
   182	    #[test]
   183	    fn v2_argmax_picks_highest_price_yes() {
   184	        // 3 candidates with distinct prices; epsilon = 0 forces argmax.
   185	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   186	        pi.insert(TxId("low".into()), make_entry(30, 100));
   187	        pi.insert(TxId("mid".into()), make_entry(50, 100));
   188	        pi.insert(TxId("high".into()), make_entry(80, 100));
   189	        let mask: BTreeSet<TxId> = BTreeSet::new();
   190	        let policy = BoltzmannMaskPolicy {
   191	            epsilon_exploration_num: 0,
   192	            epsilon_exploration_den: 1,
   193	            ..BoltzmannMaskPolicy::default()
   194	        };
   195	        let mut rng = rand::rngs::StdRng::seed_from_u64(7);
   196	        // Repeat: with epsilon=0 the result is fully deterministic.
   197	        for _ in 0..20 {
   198	            let pick = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng);
   199	            assert_eq!(pick, Some(TxId("high".into())));
   200	        }
   201	    }
   202	
   203	    #[test]
   204	    fn v2_epsilon_greedy_explores_under_high_epsilon() {
   205	        // SG-14.5: epsilon exploration produces non-argmax picks.
   206	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   207	        pi.insert(TxId("low".into()), make_entry(10, 100));
   208	        pi.insert(TxId("mid".into()), make_entry(50, 100));
   209	        pi.insert(TxId("high".into()), make_entry(90, 100));
   210	        let mask: BTreeSet<TxId> = BTreeSet::new();
   211	        // epsilon = 1.0 → always exploration (uniform random).
   212	        let policy = BoltzmannMaskPolicy {
   213	            epsilon_exploration_num: 10,
   214	            epsilon_exploration_den: 10,
   215	            ..BoltzmannMaskPolicy::default()
   216	        };
   217	        let mut rng = rand::rngs::StdRng::seed_from_u64(2026);
   218	        let mut seen: BTreeSet<TxId> = BTreeSet::new();
   219	        for _ in 0..200 {
   220	            if let Some(id) = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng) {
   220	            if let Some(id) = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng) {
   221	                seen.insert(id);
   222	            }
   223	        }
   224	        assert!(
   225	            seen.len() >= 2,
   226	            "SG-14.5: epsilon=1.0 must produce diverse picks; got {:?}",
   227	            seen
   228	        );
   229	    }
   230	
   231	    #[test]
   232	    fn v2_predicate_failure_dominates_high_price() {
   233	        // SG-14.4 / halt-trigger #1: a "high price" parent picked by v2 does
   234	        // not affect the predicate gate. v2 returns a TxId; predicate
   235	        // evaluation lives in sequencer.rs and is structurally decoupled
   236	        // (verified by halt-trigger #1's grep fence). Here we assert the
   237	        // v2 selector is purely a SCHEDULING priority, not an acceptance
   238	        // signal — its return value is a TxId, with no acceptance flag,
   239	        // no L4/L4.E classification effect. The structural test is in
   240	        // tests/tb_14_halt_triggers.rs::price_does_not_affect_predicate_result.
   241	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   242	        pi.insert(TxId("hi".into()), make_entry(99, 100));
   243	        let mask: BTreeSet<TxId> = BTreeSet::new();
   244	        let policy = BoltzmannMaskPolicy {
   245	            epsilon_exploration_num: 0,
   246	            epsilon_exploration_den: 1,
   247	            ..BoltzmannMaskPolicy::default()
   248	        };
   249	        let mut rng = rand::rngs::StdRng::seed_from_u64(0);
   250	        let pick = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng);
   251	        // The v2 return type is Option<TxId>, NOT (TxId, AcceptanceVerdict).
   252	        // Type-system enforces decoupling: caller cannot mistakenly read
   253	        // a "predicate verdict" from the selector.
   254	        let _: Option<TxId> = pick;
   255	    }
   256	
   257	    #[test]
   258	    fn v2_determinism_under_fixed_seed() {
   259	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
   260	        for i in 0..5 {
   261	            pi.insert(
   262	                TxId(format!("n{i}")),
   263	                make_entry((i as u128 + 1) * 10, 100),
   264	            );
   265	        }
   266	        let mask: BTreeSet<TxId> = BTreeSet::new();
   267	        let policy = BoltzmannMaskPolicy::default();
   268	
   269	        let run1: Vec<Option<TxId>> = {
   270	            let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
   271	            (0..50)
   272	                .map(|_| boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng))
   273	                .collect()
   274	        };
   275	        let run2: Vec<Option<TxId>> = {
   276	            let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
   277	            (0..50)
   278	                .map(|_| boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng))
   279	                .collect()
   280	        };
   281	        assert_eq!(
   282	            run1, run2,
   283	            "v2 must be deterministic under identical seed (Art.0.2)"
   284	        );
   285	    }
   286	}

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1,80p;1290,1415p;1515,1585p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// MiniF2F v4 Evaluator — oneshot and swarm modes
     2	//
     3	// Sole optimization metric: PPUT (Progress Per Unit Time)
     4	//   Progress = 100% if Golden Path exists (OMEGA reached), 0% otherwise
     5	//   PPUT = 100% / time_to_omega (seconds)
     6	//   No GP → PPUT = 0 → problem not worth attacking in current iteration
     7	//
     8	// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)
     9	
    10	use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
    11	use minif2f_v4::cost_aggregator::RunCostAccumulator;
    12	use minif2f_v4::wall_clock::RunWallClock;
    13	use minif2f_v4::post_hoc_verifier::{
    14	    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
    15	};
    16	use turingosv4::bus::{BusConfig, BusResult, TuringBus};
    17	use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
    18	use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
    19	use turingosv4::kernel::Kernel;
    20	use turingosv4::sdk::actor::boltzmann_select_parent_v2;
    21	use turingosv4::state::BoltzmannMaskPolicy;
    22	use turingosv4::sdk::prompt::build_agent_prompt;
    23	use turingosv4::sdk::prompt_guard::assert_no_metric_leak;
    24	use turingosv4::sdk::protocol::parse_agent_output;
    25	use turingosv4::sdk::tools::wallet::WalletTool;
    26	use turingosv4::sdk::tools::search::SearchTool;
    27	use turingosv4::sdk::tools::librarian::LibrarianTool;
    28	
    29	use std::collections::{HashMap, HashSet};
    30	use std::hash::{Hash, Hasher};
    31	use std::path::PathBuf;
    32	use std::sync::{Arc, Mutex};
    33	use std::time::Instant;
    34	use log::{info, warn, error};
    35	use rand::SeedableRng;
    36	use rand::rngs::StdRng;
    37	
    38	/// TB-1 Day-1 spike (2026-04-29): hex digest of an LLM prompt body.
    39	/// Used as `PputResult.prompt_context_hash` so Phase D CCL can join
    40	/// prompt-context → outcome trajectories without leaking the prompt
    41	/// itself. Day-1 uses `DefaultHasher` (same non-cryptographic hash
    42	/// already used for proof-artifact filenames at `persist_proof_artifact`)
    43	/// to avoid a new direct sha2 dep that would mutate the workspace
    44	/// `Cargo.lock` and trip the Trust Root gate (genesis_payload.toml is
    45	/// STEP_B-protected). Day-4 upgrades to SHA-256 in the same commit
    46	/// that re-hashes the Trust Root manifest with sudo authorization.
    47	///
    48	/// TRACE_MATRIX FC1-N12: oracle scope — the prompt is the pre-Lean
    49	/// step-1 proposal input; this hash makes it auditable from the v2 jsonl
    50	/// row alone.
    51	fn prompt_hash_hex(prompt_body: &str) -> String {
    52	    use std::collections::hash_map::DefaultHasher;
    53	    let mut h = DefaultHasher::new();
    54	    prompt_body.hash(&mut h);
    55	    format!("{:016x}", h.finish())
    56	}
    57	
    58	const DEFAULT_BOLTZMANN_SEED: u64 = 74677;  // same as sample seed (BTC/USD external)
    59	
    60	const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";
    61	
    62	/// PPUT result for a single problem — the only output that matters.
    63	///
    64	/// Mid-term audit P0-B fix 2026-04-25: this struct now carries every B1
    65	/// `RunAggregate` v2 field as a non-Optional, so emitted jsonl rows are
    66	/// dispatched as `RunRecord::V2` by `RunRecord::from_json` (presence of
    67	/// `schema_version` is the discriminant). Legacy diagnostic fields below
    68	/// are kept as Option/skip-if-None for downstream tooling that already
    69	/// reads them; serde silently drops them when parsing as `RunAggregate`
    70	/// (no `deny_unknown_fields`), so V2-tooling reads the v2 contract while
    71	/// PputResult-tooling sees the full diagnostic envelope.
    72	#[derive(Debug, serde::Serialize)]
    73	struct PputResult {
    74	    // ── B1 RunAggregate v2 schema fields (all REQUIRED — non-Optional) ──
    75	    /// Always "v2.0" — RunRecord::from_json discriminator.
    76	    schema_version: String,
    77	    /// Per-run identifier: condition + problem + timestamp.
    78	    run_id: String,
    79	    /// Problem identifier: theorem stem (basename of .lean without extension).
    80	    problem_id: String,
  1290	            // for this calibration-treatment path.
  1291	            // C1b: route accept legs through apply_mode_to_accept; under
  1292	            // SoftLaw the synthetic short-circuit also flips runtime to
  1293	            // true, contributing to the pput_runtime/pput_verified gap.
  1294	            let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
  1295	                mode, false, false,
  1296	            );
  1297	            let mut result = make_pput(problem_file, &condition, &run_model_label,
  1298	                                       rt, ph, start, 0, 0,
  1299	                                       tx as u64, Some(tool_dist), None,
  1300	                                       None, None, None,
  1301	                                       acc.total_run_token_count(),
  1302	                                       acc.failed_branch_count,
  1303	                                       wc.elapsed_ms().unwrap_or(0),
  1304	                                       false,
  1305	                                       proposal_hashes.len() as u64,
  1306	                                       proposal_count,
  1307	                                       verifier_wait_ms,
  1308	                                       budget_regime, budget_max_tx_base, &run_id);
  1309	            // B7-extra disambiguator: distinguish this calibration-treatment
  1310	            // exit from a natural max-tx exhaustion in downstream PPUT
  1311	            // analysis. See PputResult::synthetic_short_circuit doc-comment
  1312	            // for the cost-asymmetry note.
  1313	            result.synthetic_short_circuit = Some(true);
  1314	            return result;
  1315	        }
  1316	
  1317	        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
  1318	        // bracket at the top of the FIRST tx (before chain/skill/board build
  1319	        // and before build_agent_prompt). Idempotent — only the first tx's
  1320	        // call sticks; subsequent calls no-op. PREREG § 5 / plan B3 define
  1321	        // T_i as "first agent prompt construction"; this is the earliest
  1322	        // moment the agent begins constructing its prompt.
  1323	        wc.mark_first_read();
  1324	
  1325	        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)
  1326	        if tick_interval > 0 && tx > 0 && tx % tick_interval == 0 {
  1327	            let tape_len = bus.kernel.tape.time_arrow().len();
  1328	            // TB-14 Atom 6 (FC3-N42 production wire-up): tick-time signal
  1329	            // surface derived from `bus.snapshot().price_index` (integer-
  1330	            // rational NodeMarketEntry per node). Top-5 by price_yes argmax
  1331	            // (cross-multiplication, no f64) for the operator log line.
  1332	            // Local snapshot — the per-iteration `snap` at line 1424 below
  1333	            // serves the agent prompt; this one is tick-scoped only.
  1334	            let tick_snap = bus.snapshot();
  1335	            let market_count = tick_snap.price_index.len();
  1336	            let mut by_yes: Vec<(&turingosv4::state::TxId, &turingosv4::state::NodeMarketEntry)> =
  1337	                tick_snap.price_index.iter()
  1338	                    .filter(|(_, e)| e.price_yes.is_some())
  1339	                    .collect();
  1340	            by_yes.sort_by(|(_, a), (_, b)| {
  1341	                let pa = a.price_yes.as_ref().unwrap();
  1342	                let pb = b.price_yes.as_ref().unwrap();
  1343	                let lhs = (pb.numerator).saturating_mul(pa.denominator);
  1344	                let rhs = (pa.numerator).saturating_mul(pb.denominator);
  1345	                lhs.cmp(&rhs)
  1346	            });
  1347	            let top_prices: Vec<String> = by_yes.iter().take(5)
  1348	                .map(|(id, e)| {
  1349	                    let p = e.price_yes.as_ref().unwrap();
  1350	                    format!("{}:{}/{}", id.0, p.numerator, p.denominator)
  1351	                })
  1352	                .collect();
  1353	            info!("[tick@tx{}] tape={} markets={} top={}", tx, tape_len, market_count,
  1354	                top_prices.join(", "));
  1355	            // A6 FC2-N20 (mr tick): clock → mr → tape per Art. IV.
  1356	            // Phase D consumer joins on (run_id, tx) to derive the
  1357	            // tape-growth curve and detect zero-tick stalls before they
  1358	            // become C-036 alarm events.
  1359	            minif2f_v4::fc_trace::emit_event(
  1360	                minif2f_v4::fc_trace::FcId::Fc2N20,
  1361	                &run_id, Some(tx as u64), None,
  1362	                &[
  1363	                    ("tape_len", tape_len.to_string()),
  1364	                    ("market_count", market_count.to_string()),
  1365	                ],
  1366	            );
  1367	            // Phase 6-emergent: refresh shared team board from facts only.
  1368	            // Per-agent cumulative balance + recent tape-node authorship counts
  1369	            // + top market prices. No instructions, no "should" — just state.
  1370	            if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
  1371	                let agents_sorted: Vec<String> = agent_ids.clone();
  1372	                let mut author_counts: std::collections::HashMap<String, u32> =
  1373	                    std::collections::HashMap::new();
  1374	                for nid in bus.kernel.tape.time_arrow() {
  1375	                    if let Some(n) = bus.kernel.tape.get(nid) {
  1376	                        *author_counts.entry(n.author.clone()).or_insert(0) += 1;
  1377	                    }
  1378	                }
  1379	                // TB-9 collapse: WalletTool no longer carries owned f64 balances.
  1380	                // For the EMERGENT_ROLES message-board view, fall back to "n/a"
  1381	                // until balance projection is plumbed through with an EconomicState
  1382	                // ref (post-MVP polish). Tape-node authorship counts continue to
  1383	                // surface as the primary signal for emergent role differentiation.
  1384	                let mut board = format!("# tick@tx{} (tape_nodes={})\n", tx, tape_len);
  1385	                for a in &agents_sorted {
  1386	                    let nodes = author_counts.get(a).copied().unwrap_or(0);
  1387	                    board.push_str(&format!(
  1388	                        "- {}: balance=n/a, tape_nodes_authored={}\n",
  1389	                        a, nodes));
  1390	                }
  1391	                if !top_prices.is_empty() {
  1392	                    board.push_str(&format!("markets: {}\n", top_prices.join(", ")));
  1393	                }
  1394	                // Preserve any agent posts that were already in the file (append-only).
  1395	                if let Some(lib) = bus.tools.iter()
  1396	                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
  1397	                {
  1398	                    let existing = lib.read_board();
  1399	                    // Keep only the POST lines (they carry agent-originated intent).
  1400	                    let posts: String = existing.lines()
  1401	                        .filter(|l| l.starts_with("## POST") || (l.starts_with(" ") == false && !l.starts_with("#") && !l.starts_with("-") && !l.starts_with("markets:")))
  1402	                        .collect::<Vec<_>>()
  1403	                        .join("\n");
  1404	                    let full = if posts.is_empty() {
  1405	                        board
  1406	                    } else {
  1407	                        format!("{}\n{}\n", board, posts)
  1408	                    };
  1409	                    let _ = lib.write_board(&full);
  1410	                }
  1411	            }
  1412	            // C-036 zero-tick alarm: 5 consecutive ticks with no constitutional engine activity.
  1413	            if tape_len == 0 && market_count == 0 {
  1414	                zero_ticks_run += 1;
  1415	                if zero_ticks_run >= 5 && !zero_tick_warned {
  1515	            let mut by_yes: Vec<(&turingosv4::state::TxId,
  1516	                                 &turingosv4::state::NodeMarketEntry)> =
  1517	                snap.price_index.iter()
  1518	                    .filter(|(_, e)| e.price_yes.is_some())
  1519	                    .collect();
  1520	            by_yes.sort_by(|(_, a), (_, b)| {
  1521	                let pa = a.price_yes.as_ref().unwrap();
  1522	                let pb = b.price_yes.as_ref().unwrap();
  1523	                let lhs = (pb.numerator).saturating_mul(pa.denominator);
  1524	                let rhs = (pa.numerator).saturating_mul(pb.denominator);
  1525	                lhs.cmp(&rhs)
  1526	            });
  1527	            by_yes.iter().take(50)
  1528	                .map(|(id, e)| {
  1529	                    let p = e.price_yes.as_ref().unwrap();
  1530	                    format!("{}: YES={}/{}", id.0, p.numerator, p.denominator)
  1531	                })
  1532	                .collect::<Vec<_>>()
  1533	                .join(", ")
  1534	        };
  1535	
  1536	        // TB-14 Atom 6: query the canonical balance from the live sequencer
  1537	        // when wired (chaintape mode). The TB-9 collapse "balance projection
  1538	        // through snapshot is post-MVP polish" comment at L1353-1357 is
  1539	        // resolved here for the prompt path: pull MicroCoin → Coin via
  1540	        // sequencer.q_snapshot() → economic_state_t.balances_t. Falls back
  1541	        // to 0.0 when bus runs sequencer-less (legacy WAL-only mode).
  1542	        // The `f64` here is purely the prompt-render contract of
  1543	        // `build_agent_prompt(... balance: f64 ...)` — `prompt.rs` is not a
  1544	        // TB-14 module surface (the G-14.11 fence targets `price_index.rs`
  1545	        // only).
  1546	        let prompt_balance: f64 = bus.sequencer.as_ref()
  1547	            .and_then(|seq| seq.q_snapshot().ok())
  1548	            .and_then(|q| q.economic_state_t.balances_t.0
  1549	                .get(&turingosv4::state::AgentId(agent_id.clone()))
  1550	                .copied())
  1551	            .map(|micro| micro.micro_units() as f64 / 1_000_000.0)
  1552	            .unwrap_or(0.0);
  1553	
  1554	        let prompt = build_agent_prompt(
  1555	            &chain, &skill, &market_ticker_str, &errors, &hits_ref,
  1556	            prompt_balance, tools_desc, &team_board,
  1557	        );
  1558	
  1559	        // Phase A atom A3: bind δ for this agent_idx (same vector resolved
  1560	        // once at run_swarm entry from AGENT_MODELS env). In Phase B+C this
  1561	        // is uniform across all agent_idx; in Phase D it may diverge.
  1562	        let agent_model = &agent_models[agent_idx];
  1563	        // Model-aware max_tokens (same rule as oneshot branch). Per-agent so
  1564	        // a heterogeneous Phase D swarm mixing chat + reasoner backbones gets
  1565	        // the right ceiling per-call instead of a single global heuristic.
  1566	        let max_toks = if agent_model.contains("chat") { 8000 } else { 16000 };
  1567	        // Art. II.2.1 anti-homogeneity: per-agent temperature ladder breaks
  1568	        // sampling correlation among role-distinct agents (F-2026-04-18-03).
  1569	        // Disabled (keep at 0.2) when TEMP_LADDER!=1 to isolate the mechanism.
  1570	        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
  1571	            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
  1572	        } else {
  1573	            0.2
  1574	        };
  1575	        let request = GenerateRequest {
  1576	            model: agent_model.clone(),
  1577	            messages: vec![Message { role: "user".into(), content: prompt }],
  1578	            temperature: Some(temp),
  1579	            max_tokens: Some(max_toks),
  1580	        };
  1581	
  1582	        // PPUT-CCL B6 runtime gate (swarm path): swarm prompts include
  1583	        // tape contents, board posts, search hits, and learned memory —
  1584	        // any of these state surfaces could in principle inject a PPUT
  1585	        // value at runtime even when the prompt builder is clean. Gate

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '1480,1585p;1670,1760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1480	
  1481	/// TRACE_MATRIX TB-14 Atom 6 (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6):
  1482	/// §14 PriceIndex render. Pure function over the derived view; extracted for
  1483	/// SG-14.6 unit-testability.
  1484	///
  1485	/// **ARCHITECT-MANDATED BANNER**: the section opens with the literal phrase
  1486	/// "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1: "Price is signal, not
  1487	/// truth."). Re-rendering this banner in every dashboard frame is the
  1488	/// SG-14.6 ship gate's enforcement surface.
  1489	///
  1490	/// **NO DECIMAL** (charter §5 forbidden + G-14.11 ship gate "no f64 in TB-14
  1491	/// module surface"): every `price_yes` / `price_no` is rendered as
  1492	/// `numerator/denominator` integer-rational. The dashboard NEVER divides.
  1493	fn render_section_14(price_index: &BTreeMap<TxId, NodeMarketEntry>) -> String {
  1494	    let mut s = String::new();
  1495	    s.push('\n');
  1496	    s.push_str("§14 TB-14 PriceIndex (architect 2026-05-03 §5.1 + §5.5 SG-14.6)\n");
  1497	    s.push_str("---------------------------------------------------------------\n");
  1498	    s.push_str("  PRICE IS SIGNAL, NOT TRUTH.\n");
  1499	    s.push_str("    Architect §5.1 ruling 2026-05-03: the price index is a\n");
  1500	    s.push_str("    derived statistical broadcast over canonical NodePositionsIndex\n");
  1501	    s.push_str("    long/short interest. It MUST NOT influence predicate gates\n");
  1502	    s.push_str("    (CR-14.1 / halt-trigger #1) or L4/L4.E classification\n");
  1503	    s.push_str("    (CR-14.2 / halt-trigger #2). Boolean predicates establish\n");
  1504	    s.push_str("    absolute bounds; the price view is for relative-effectiveness\n");
  1505	    s.push_str("    measurement only.\n\n");
  1506	
  1507	    if price_index.is_empty() {
  1508	        s.push_str("  (no node positions recorded — price index is empty)\n");
  1509	        s.push_str("  Acceptable signal-state: a run with zero accepted WorkTx +\n");
  1510	        s.push_str("  ChallengeTx yields an empty PriceIndex by FR-14.3 / halt-\n");
  1511	        s.push_str("  trigger #5 (zero-liquidity → price=None) extended to the\n");
  1512	        s.push_str("  zero-position case.\n");
  1513	        return s;
  1514	    }
  1515	
  1516	    s.push_str("  Per-node entries (price as integer-rational n/d, never decimal):\n\n");
  1517	    s.push_str(&format!(
  1518	        "    {:<32}  {:>14}  {:>14}  {:>16}  {:>16}\n",
  1519	        "node_id", "long_micro", "short_micro", "price_yes(n/d)", "price_no(n/d)"
  1520	    ));
  1521	    s.push_str("    ");
  1522	    s.push_str(&"-".repeat(98));
  1523	    s.push('\n');
  1524	
  1525	    for (node_id, entry) in price_index.iter() {
  1526	        let yes_str = match &entry.price_yes {
  1527	            Some(p) => format!("{}/{}", p.numerator, p.denominator),
  1528	            None => "None".to_string(),
  1529	        };
  1530	        let no_str = match &entry.price_no {
  1531	            Some(p) => format!("{}/{}", p.numerator, p.denominator),
  1532	            None => "None".to_string(),
  1533	        };
  1534	        s.push_str(&format!(
  1535	            "    {:<32}  {:>14}  {:>14}  {:>16}  {:>16}\n",
  1536	            trunc(&node_id.0, 32),
  1537	            entry.long_interest.micro_units(),
  1538	            entry.short_interest.micro_units(),
  1539	            yes_str,
  1540	            no_str,
  1541	        ));
  1542	    }
  1543	
  1544	    s.push('\n');
  1545	    s.push_str("  Architect mandate (§5.1 ruling 2026-05-03) ✓:\n");
  1546	    s.push_str("    Price is signal, not truth. NodeMarketEntry is a derived view —\n");
  1547	    s.push_str("    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.\n");
  1548	    s.push_str("    NO price-based settlement. NO Goodhart leak of private predicates.\n");
  1549	    s
  1550	}
  1551	
  1552	/// TRACE_MATRIX TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4 + §10):
  1553	/// §13 Node exposure records render. Pure function over Vec<ExposureRecordRow>;
  1554	/// extracted for SG-12.6 unit-testability. ARCHITECT-MANDATED LABEL:
  1555	/// "Exposure records", NOT "Open market balances". TB-12 is exposure
  1556	/// index, NOT trading market — NodePosition is IMMUTABLE EXPOSURE RECORD
  1557	/// (architect §10), not active position balance. CR-12.1 + CR-12.2.
  1558	fn render_section_13(exposures: &[ExposureRecordRow]) -> String {
  1559	    let mut s = String::new();
  1560	    s.push('\n');
  1561	    s.push_str("§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)\n");
  1562	    s.push_str("------------------------------------------------------------------------------\n");
  1563	
  1564	    if exposures.is_empty() {
  1565	        s.push_str("  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)\n");
  1566	    } else {
  1567	        s.push_str("  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):\n");
  1568	        s.push_str("    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round\n");
  1569	        s.push_str("    -----------------+------------------+-------+-----------------+----------------+--------------+--------\n");
  1570	        let mut total_long: i64 = 0;
  1571	        let mut total_short: i64 = 0;
  1572	        for ex in exposures {
  1573	            if ex.side == "Long" {
  1574	                total_long += ex.amount_micro;
  1575	            } else if ex.side == "Short" {
  1576	                total_short += ex.amount_micro;
  1577	            }
  1578	            s.push_str(&format!(
  1579	                "    {:<16} | {:<16} | {:<5} | {:<15} | {:<14} | {:>12} | {:>6}\n",
  1580	                trunc(&ex.position_id, 16),
  1581	                trunc(&ex.node_id, 16),
  1582	                ex.side,
  1583	                ex.kind,
  1584	                trunc(&ex.owner, 14),
  1585	                ex.amount_micro,
  1670	                task_id: TaskId(format!("task-{node}")),
  1671	                event_id: turingosv4::state::typed_tx::EventId(TaskId(format!("task-{node}"))),
  1672	                long_interest: MicroCoin::from_micro_units(long),
  1673	                short_interest: MicroCoin::from_micro_units(short),
  1674	                yes_share_depth: turingosv4::state::typed_tx::ShareAmount::from_units(0),
  1675	                no_share_depth: turingosv4::state::typed_tx::ShareAmount::from_units(0),
  1676	                price_yes: py.map(|(n, d)| RationalPrice { numerator: n, denominator: d }),
  1677	                price_no: pn.map(|(n, d)| RationalPrice { numerator: n, denominator: d }),
  1678	                liquidity_depth: MicroCoin::from_micro_units(long + short),
  1679	            },
  1680	        )
  1681	    }
  1682	
  1683	    /// SG-14.6 ARCHITECT-MANDATED: dashboard §14 carries the literal banner
  1684	    /// "PRICE IS SIGNAL, NOT TRUTH". This is the structural enforcement of
  1685	    /// architect §5.1 ("Price is signal, not truth.") at the read-view
  1686	    /// surface; future maintainers adding signal-as-truth language must
  1687	    /// fail this test.
  1688	    #[test]
  1689	    fn sg_14_6_dashboard_carries_price_is_signal_not_truth_banner() {
  1690	        let pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
  1691	        let s = render_section_14(&pi);
  1692	        assert!(
  1693	            s.contains("PRICE IS SIGNAL, NOT TRUTH"),
  1694	            "SG-14.6: §14 must contain the architect-mandated banner \
  1695	             `PRICE IS SIGNAL, NOT TRUTH`. Got render:\n{s}"
  1696	        );
  1697	    }
  1698	
  1699	    /// SG-14.6 ARCHITECT-MANDATED: dashboard §14 NEVER renders prices as
  1700	    /// decimal fractions — only `numerator/denominator` integer-rational
  1701	    /// pairs. The renderer must not contain any `format!("{:.N}", ...)`
  1702	    /// invocation against a price value, and the rendered string must
  1703	    /// not contain a decimal point inside any per-row token.
  1704	    #[test]
  1705	    fn sg_14_6_dashboard_renders_price_as_integer_rational_never_decimal() {
  1706	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
  1707	        let (k1, e1) = make_entry("n_alpha", 700_000, 300_000,
  1708	                                  Some((700_000, 1_000_000)),
  1709	                                  Some((300_000, 1_000_000)));
  1710	        pi.insert(k1, e1);
  1711	        let (k2, e2) = make_entry("n_beta", 500_000, 500_000,
  1712	                                  Some((500_000, 1_000_000)),
  1713	                                  Some((500_000, 1_000_000)));
  1714	        pi.insert(k2, e2);
  1715	
  1716	        let s = render_section_14(&pi);
  1717	        // Spot-check rendering of a known rational pair.
  1718	        assert!(
  1719	            s.contains("700000/1000000"),
  1720	            "SG-14.6: per-node price_yes must render as `n/d` integer-rational. Got:\n{s}"
  1721	        );
  1722	        assert!(
  1723	            s.contains("500000/1000000"),
  1724	            "SG-14.6: per-node price_yes must render as `n/d` integer-rational. Got:\n{s}"
  1725	        );
  1726	        // Architect §5.6 forbidden: NO decimal float in TB-14 surface render.
  1727	        // Spot-check no `0.7` / `70.0%` / similar decimal strings appear in any
  1728	        // per-row context (banner text may contain commas; no decimals).
  1729	        for forbidden in &["0.7", "0.3", "0.5", "70.0%", "30.0%", "50.0%"] {
  1730	            assert!(
  1731	                !s.contains(forbidden),
  1732	                "SG-14.6: §14 render MUST NOT contain decimal price token `{forbidden}` \
  1733	                 (architect §5.6 forbidden: no f64 / no decimal). Got:\n{s}"
  1734	            );
  1735	        }
  1736	    }
  1737	
  1738	    /// SG-14.6 + FR-14.3: when the price index is empty (no recorded
  1739	    /// positions), §14 renders an explicit empty-state message rather than
  1740	    /// falling back to a stale or fabricated number.
  1741	    #[test]
  1742	    fn sg_14_6_dashboard_empty_price_index_renders_explicit_empty_state() {
  1743	        let pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
  1744	        let s = render_section_14(&pi);
  1745	        assert!(
  1746	            s.contains("price index is empty"),
  1747	            "SG-14.6: empty PriceIndex must render an explicit empty-state \
  1748	             message, not fabricate a number. Got:\n{s}"
  1749	        );
  1750	    }
  1751	
  1752	    /// SG-14.6 + FR-14.3: a node with `price_yes == None` (zero-liquidity)
  1753	    /// must render as `None`, never as `0/0`, `0.0`, or any synthesized
  1754	    /// fraction.
  1755	    #[test]
  1756	    fn sg_14_6_dashboard_renders_none_for_zero_liquidity_nodes() {
  1757	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
  1758	        let (k, e) = make_entry("n_zero", 0, 0, None, None);
  1759	        pi.insert(k, e);
  1760	        let s = render_section_14(&pi);

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1410,1538p;1585,1665p;1665,1745p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1410	                }
  1411	            }
  1412	            // C-036 zero-tick alarm: 5 consecutive ticks with no constitutional engine activity.
  1413	            if tape_len == 0 && market_count == 0 {
  1414	                zero_ticks_run += 1;
  1415	                if zero_ticks_run >= 5 && !zero_tick_warned {
  1416	                    warn!("[harness] {} consecutive zero-ticks (tape & markets idle) — \
  1417	                           constitutional engines bypassed (Art. II.1/II.2 unused)", zero_ticks_run);
  1418	                    zero_tick_warned = true;
  1419	                }
  1420	            } else {
  1421	                zero_ticks_run = 0;
  1422	            }
  1423	        }
  1424	
  1425	        let agent_idx = tx % n_agents;
  1426	        let agent_id = &agent_ids[agent_idx];
  1427	        let snap = bus.snapshot();
  1428	
  1429	        // C1e Amnesia: agent loses memory of L_t. Force the chain
  1430	        // projection to problem-statement-only (the same shape used
  1431	        // when snap.tape is genuinely empty), so every proposal is
  1432	        // re-derived from scratch every tx — ERR=0 + time/token
  1433	        // inflation are the H detection mechanisms. Internal
  1434	        // verification paths (tape+payload Lean re-verify) below
  1435	        // are NOT gated on is_amnesia; that's verifier mechanics,
  1436	        // not agent memory.
  1437	        let chain = if minif2f_v4::experiment_mode::is_amnesia(mode) || snap.tape.is_empty() {
  1438	            problem_statement.to_string()
  1439	        } else {
  1440	            let nodes: Vec<String> = snap.tape.time_arrow().iter()
  1441	                .filter_map(|id| snap.tape.get(id))
  1442	                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
  1443	                .collect();
  1444	            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
  1445	        };
  1446	
  1447	        let errors = bus.recent_rejections(agent_id, 3);
  1448	        // Art. II.2.1: per-agent skill specialization + Librarian learned memory.
  1449	        // C1c: route skill index through experiment_mode helper so Homogeneous
  1450	        // mode pins every agent_idx to 0 (Paper-1 era A condition; H4 detection).
  1451	        let skill_idx = minif2f_v4::experiment_mode::skill_index_for_agent(
  1452	            mode, agent_idx, agent_skills.len(),
  1453	        );
  1454	        let base_skill = agent_skills.get(skill_idx).unwrap_or(&"");
  1455	        // C1d Panopticon: in cognitive-isolation-breach mode, the focal
  1456	        // agent's prompt receives the merged learned-memory of ALL agents,
  1457	        // labeled with each source agent_id. Context grows ~O(N) per tx
  1458	        // → tokens↑ → PPUT↓; H2 detection mechanism. Full / SoftLaw /
  1459	        // Homogeneous / Amnesia keep the per-agent fetch.
  1460	        let learned = if minif2f_v4::experiment_mode::is_panopticon(mode) {
  1461	            bus.tools.iter()
  1462	                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
  1463	                .map(|lib| {
  1464	                    agent_ids.iter()
  1465	                        .filter_map(|a| lib.read_agent_memory(a).map(|m| format!("[{}] {}", a, m)))
  1466	                        .collect::<Vec<_>>()
  1467	                        .join("\n---\n")
  1468	                })
  1469	                .unwrap_or_default()
  1470	        } else {
  1471	            bus.tools.iter()
  1472	                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
  1473	                .and_then(|lib| lib.read_agent_memory(agent_id))
  1474	                .unwrap_or_default()
  1475	        };
  1476	        let skill = if learned.is_empty() {
  1477	            base_skill.to_string()
  1478	        } else {
  1479	            format!("{}\n\n{}", base_skill, learned)
  1480	        };
  1481	        // A8e14 R2 (Gemini R12): when an agent hits SEARCH_CAP we strip the
  1482	        // search tool — but pre-R2 the cached hits from its last search kept
  1483	        // appearing in every subsequent prompt, leaving the agent reasoning
  1484	        // from stale results for the rest of the run. Single cap_hit gate
  1485	        // for both the tool list AND the cache injection.
  1486	        let cap_hit = search_count.get(agent_id).copied().unwrap_or(0) >= search_cap;
  1487	        let hits_ref: Vec<String> = if cap_hit {
  1488	            Vec::new()
  1489	        } else {
  1490	            search_cache.get(agent_id).cloned().unwrap_or_default()
  1491	        };
  1492	        let tools_desc = if cap_hit {
  1493	            "append, complete, invest"
  1494	        } else {
  1495	            "append, complete, invest, search"
  1496	        };
  1497	        // Phase 6-emergent: read the shared team board. Gated by EMERGENT_ROLES=1
  1498	        // so baseline behaviour is untouched. Board content is built by
  1499	        // Librarian at periodic ticks (see refresh_board below).
  1500	        let team_board: String = if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
  1501	            bus.tools.iter()
  1502	                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
  1503	                .map(|l| l.read_board())
  1504	                .unwrap_or_default()
  1505	        } else {
  1506	            String::new()
  1507	        };
  1508	        // TB-14 Atom 6 (FC3-N42 production wire-up): build a top-N price
  1509	        // ticker string from `snap.price_index` (integer-rational
  1510	        // NodeMarketEntry per node). Renders price_yes as `numerator/
  1511	        // denominator` strings — never decimal — per "PRICE IS SIGNAL,
  1512	        // NOT TRUTH" SG-14.6 banner discipline. Sort: descending by
  1513	        // price_yes (cross-multiplication argmax; no f64).
  1514	        let market_ticker_str: String = {
  1515	            let mut by_yes: Vec<(&turingosv4::state::TxId,
  1516	                                 &turingosv4::state::NodeMarketEntry)> =
  1517	                snap.price_index.iter()
  1518	                    .filter(|(_, e)| e.price_yes.is_some())
  1519	                    .collect();
  1520	            by_yes.sort_by(|(_, a), (_, b)| {
  1521	                let pa = a.price_yes.as_ref().unwrap();
  1522	                let pb = b.price_yes.as_ref().unwrap();
  1523	                let lhs = (pb.numerator).saturating_mul(pa.denominator);
  1524	                let rhs = (pa.numerator).saturating_mul(pb.denominator);
  1525	                lhs.cmp(&rhs)
  1526	            });
  1527	            by_yes.iter().take(50)
  1528	                .map(|(id, e)| {
  1529	                    let p = e.price_yes.as_ref().unwrap();
  1530	                    format!("{}: YES={}/{}", id.0, p.numerator, p.denominator)
  1531	                })
  1532	                .collect::<Vec<_>>()
  1533	                .join(", ")
  1534	        };
  1535	
  1536	        // TB-14 Atom 6: query the canonical balance from the live sequencer
  1537	        // when wired (chaintape mode). The TB-9 collapse "balance projection
  1538	        // through snapshot is post-MVP polish" comment at L1353-1357 is
  1585	        // value at runtime even when the prompt builder is clean. Gate
  1586	        // every tx, every agent, every iteration.
  1587	        assert_no_metric_leak(&request.messages[0].content);
  1588	        match client.generate(&request).await {
  1589	            Ok(response) => {
  1590	                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
  1591	                // PPUT-CCL B2: every parsed proposal default-records as failed.
  1592	                // OMEGA-accept return paths flip the last record before returning.
  1593	                acc.record_proposal(false);
  1594	                match parse_agent_output(&response.content) {
  1595	                    Ok(action) => match action.tool.as_str() {
  1596	                        "append" => {
  1597	                            *tool_dist.entry("append".into()).or_insert(0) += 1;
  1598	                            if let Some(payload) = &action.payload {
  1599	                                // A4: record proposal for tactic_diversity.
  1600	                                let mut ph = std::collections::hash_map::DefaultHasher::new();
  1601	                                payload.hash(&mut ph);
  1602	                                proposal_hashes.insert(ph.finish());
  1603	                                proposal_count += 1;
  1604	                                // TB-14 Atom 6 (FC2-N29 production wire-up):
  1605	                                // integer-rational scheduler. v2 takes the
  1606	                                // `&snap.price_index` (BTreeMap<TxId,
  1607	                                // NodeMarketEntry>) and `&snap.mask_set`
  1608	                                // (BTreeSet<TxId>) directly — no f64
  1609	                                // intermediate. Returns Option<TxId>;
  1610	                                // predicate-blind (CR-14.1 + halt-trigger
  1611	                                // #1) — purely a scheduling priority pick.
  1612	                                let parent = boltzmann_select_parent_v2(
  1613	                                    &snap.price_index, &snap.mask_set,
  1614	                                    &policy, &mut boltz_rng,
  1615	                                ).map(|tx| tx.0);
  1616	
  1617	                                // ── TB-7 Atom 2: AUTHORITATIVE per-LLM-proposal routing ──
  1618	                                //
  1619	                                // Real LLM proposal → ProposalTelemetry CAS object →
  1620	                                // real-signature WorkTx → bus.submit_typed_tx → Sequencer →
  1621	                                // L4 (accepted) or L4.E (rejected). This is the Frame B
  1622	                                // closure path per TB-7 charter §4.0 + §8 Gate 1.
  1623	                                //
  1624	                                // Authoritative for ChainTape state (L4 captures the
  1625	                                // proposal byte-deterministically). The bus.append call
  1626	                                // BELOW is shadow_only (kernel.tape view sync for the next
  1627	                                // agent's prompt context — NOT canonical state).
  1628	                                // TB-7.5 fix #1 (Codex audit 492e86c action #1, BLOCKING):
  1629	                                // FAIL-CLOSED authoritative routing. Any failure of
  1630	                                // q_snapshot / CAS open / proposal_telemetry write /
  1631	                                // make_real_worktx_signed_by / submit_typed_tx exits
  1632	                                // the evaluator with code 3 and an error message —
  1633	                                // shadow_only kernel.tape sync MUST NOT be the only
  1634	                                // state mutation after an authoritative-path failure
  1635	                                // in ChainTape mode. Per TB-7 §4.0 + §6 #31.
  1636	                                if let (Some(bundle), Some(reg)) =
  1637	                                    (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
  1638	                                {
  1639	                                    let q = match bundle.sequencer.q_snapshot() {
  1640	                                        Ok(q) => q,
  1641	                                        Err(e) => {
  1642	                                            error!("[chaintape/atom2] FAIL-CLOSED: q_snapshot failed under ChainTape mode: {e:?}");
  1643	                                            std::process::exit(3);
  1644	                                        }
  1645	                                    };
  1646	                                    let parent_state_root = q.state_root_t;
  1647	                                    let logical_t = bundle.sequencer.next_logical_t_peek();
  1648	                                    let task_id_str = format!("task-{}", run_id);
  1649	
  1650	                                    // TB-7.7 D1: open CAS FIRST so build_for_evaluator_append
  1651	                                    // can durably store proposal_artifact_cid.
  1652	                                    let mut cas_store = match turingosv4::bottom_white::cas::store::CasStore::open(&bundle.cas_path) {
  1653	                                        Ok(c) => c,
  1654	                                        Err(e) => {
  1655	                                            error!("[chaintape/atom2] FAIL-CLOSED: cas open failed under ChainTape mode: {e}");
  1656	                                            std::process::exit(3);
  1657	                                        }
  1658	                                    };
  1659	
  1660	                                    // TB-7.7 D2: parent_tx from last submission per agent (root if first).
  1661	                                    let parent_tx: Option<turingosv4::state::q_state::TxId> =
  1662	                                        last_tx_by_agent.get(agent_id).cloned();
  1663	
  1664	                                    let pt = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
  1665	                                        &mut cas_store,
  1665	                                        &mut cas_store,
  1666	                                        &run_id,
  1667	                                        agent_id,
  1668	                                        proposal_count as u64,
  1669	                                        payload.as_bytes(),
  1670	                                        "append",
  1671	                                        turingosv4::runtime::proposal_telemetry::TokenCounts {
  1672	                                            prompt_tokens: response.prompt_tokens as u64,
  1673	                                            completion_tokens: response.completion_tokens as u64,
  1674	                                            tool_tokens: 0,
  1675	                                        },
  1676	                                        "tb7-atom2-evaluator-payload",
  1677	                                        logical_t,
  1678	                                        parent_tx,
  1679	                                    ) {
  1680	                                        Ok(p) => p,
  1681	                                        Err(e) => {
  1682	                                            error!("[chaintape/atom2] FAIL-CLOSED: proposal_artifact CAS put failed: {e}");
  1683	                                            std::process::exit(3);
  1684	                                        }
  1685	                                    };
  1686	
  1687	                                    let tel_cid = match turingosv4::runtime::proposal_telemetry::write_to_cas(
  1688	                                        &mut cas_store,
  1689	                                        &pt,
  1690	                                        "tb7-atom2-evaluator",
  1691	                                        logical_t,
  1692	                                    ) {
  1693	                                        Ok(c) => c,
  1694	                                        Err(e) => {
  1695	                                            error!("[chaintape/atom2] FAIL-CLOSED: proposal_telemetry CAS write failed: {e}");
  1696	                                            std::process::exit(3);
  1697	                                        }
  1698	                                    };
  1699	                                    let real_worktx = {
  1700	                                        let mut reg_guard = match reg.lock() {
  1701	                                            Ok(g) => g,
  1702	                                            Err(p) => p.into_inner(),
  1703	                                        };
  1704	                                        let suffix = format!("p{}", proposal_count);
  1705	                                        // TB-7.7 D3: stake from env (default 1000 micro-units = 0.001 coin)
  1706	                                        // for admission-gate clearance under pre-seeded escrow.
  1707	                                        // Pre-TB-7.7 stake was hardcoded 0 → all WorkTx → L4.E.
  1708	                                        let stake_micro: i64 = std::env::var("TURINGOS_CHAINTAPE_PROPOSAL_STAKE_MICRO")
  1709	                                            .ok()
  1710	                                            .and_then(|s| s.parse().ok())
  1711	                                            .unwrap_or(1_000);
  1712	                                        match turingosv4::runtime::adapter::make_real_worktx_signed_by(
  1713	                                            &mut *reg_guard,
  1714	                                            &task_id_str,
  1715	                                            agent_id,
  1716	                                            parent_state_root,
  1717	                                            stake_micro,
  1718	                                            &suffix,
  1719	                                            tel_cid,
  1720	                                            true,
  1721	                                            logical_t,
  1722	                                        ) {
  1723	                                            Ok(tx) => tx,
  1724	                                            Err(e) => {
  1725	                                                error!("[chaintape/atom2] FAIL-CLOSED: make_real_worktx_signed_by failed: {e}");
  1726	                                                std::process::exit(3);
  1727	                                            }
  1728	                                        }
  1729	                                    };
  1730	                                    // TB-7.7 D2: capture tx_id before move into submit_typed_tx.
  1731	                                    let real_worktx_tx_id = match &real_worktx {
  1732	                                        turingosv4::state::typed_tx::TypedTx::Work(w) => Some(w.tx_id.clone()),
  1733	                                        _ => None,
  1734	                                    };
  1735	                                    if let Err(e) = bus.submit_typed_tx(real_worktx).await {
  1736	                                        error!("[chaintape/atom2] FAIL-CLOSED: submit_typed_tx failed: {e:?}");
  1737	                                        std::process::exit(3);
  1738	                                    }
  1739	                                    // TB-7.7 D2: record this WorkTx as parent for next same-agent proposal.
  1740	                                    if let Some(tx_id) = real_worktx_tx_id {
  1741	                                        last_tx_by_agent.insert(agent_id.to_string(), tx_id);
  1742	                                    }
  1743	                                }
  1744	
  1745	                                // shadow_only: kernel.tape view sync for next-agent prompt

exec
/bin/bash -lc 'rg -n "BoltzmannMaskPolicy::from_env|boltz_rng|boltzmann_select_parent_v2|market_ticker_str|let snap = bus.snapshot|parent_id" experiments/minif2f_v4/src/bin/evaluator.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
20:use turingosv4::sdk::actor::boltzmann_select_parent_v2;
1171:    // boundary: `boltzmann_select_parent_v2(price_index, mask_set, &policy,
1173:    let policy = BoltzmannMaskPolicy::from_env();
1178:    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);
1427:        let snap = bus.snapshot();
1514:        let market_ticker_str: String = {
1555:            &chain, &skill, &market_ticker_str, &errors, &hits_ref,
1612:                                let parent = boltzmann_select_parent_v2(
1614:                                    &policy, &mut boltz_rng,

exec
/bin/bash -lc "rg -n \"enum TypedTx|struct WorkTx|NodePosition|PositionKind|FirstLong|ChallengeShort|node_positions_t|apply_work|WorkTx\" src/state -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/q_state.rs:190:    /// TRACE_MATRIX TB-12 (architect 2026-05-03 ruling §3 + §10): node_positions_t
src/state/q_state.rs:191:    /// — flat `BTreeMap<TxId, NodePosition>` index. **Canonical** TB-12 source
src/state/q_state.rs:193:    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
src/state/q_state.rs:201:    /// Populated by accept-arm side-effect on accepted WorkTx (FirstLong) +
src/state/q_state.rs:202:    /// ChallengeTx (ChallengeShort) per architect §8 Atom 2. VerifyTx writes
src/state/q_state.rs:206:    pub node_positions_t: NodePositionsIndex,
src/state/q_state.rs:281:/// to. Required by the WorkTx admission gate (TB-3 § 3.4 lock-on-accept):
src/state/q_state.rs:282:/// when an accepted WorkTx commits its inline `stake` into `stakes_t`, the
src/state/q_state.rs:330:    /// TB-8 Atom 1: the accepted WorkTx whose OMEGA-Confirm produced this claim.
src/state/q_state.rs:526:// TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1): NodePositionsIndex —
src/state/q_state.rs:533:/// `position_id → NodePosition` index. Architect's §3 ruling chose this
src/state/q_state.rs:536:/// (Atom 2) on accepted WorkTx (FirstLong) + ChallengeTx (ChallengeShort).
src/state/q_state.rs:538:pub struct NodePositionsIndex(
src/state/q_state.rs:539:    pub BTreeMap<TxId, crate::state::typed_tx::NodePosition>,
src/state/q_state.rs:640:/// `WorkTx.tx_id` this challenge accuses. Required by:
src/state/q_state.rs:806:        // with +node_positions_t (flat NodePositionsIndex; canonical exposure
src/state/q_state.rs:828:        assert!(obj.contains_key("node_positions_t"), "TB-12 node_positions_t sub-field missing");
src/state/q_state.rs:834:    /// TB-12 Atom 1 (architect §8 Atom 1): NodePositionsIndex empty default
src/state/q_state.rs:838:        let idx = NodePositionsIndex::default();
src/state/sequencer.rs:47:// TB-2 — WorkTx-accept state-root domain (preflight v3 §3.4 + P1-1 r2)
src/state/sequencer.rs:50:/// TRACE_MATRIX FC3-S3: TB-2 interim WorkTx-accept state-root domain.
src/state/sequencer.rs:55:/// accepted WorkTx. Distinct from the TB-1 toy domain
src/state/sequencer.rs:75:/// TRACE_MATRIX FC3-S3: TB-2 interim state-root mutator on WorkTx accept.
src/state/sequencer.rs:308:/// variants — none on the WorkTx arm in TB-2; reserved for future TBs).
src/state/sequencer.rs:317:/// though the `match` uses `_` for the 19-variant tail: WorkTx-arm-reachable
src/state/sequencer.rs:318:/// variants are explicit; non-WorkTx-arm variants fall through to
src/state/sequencer.rs:346:        TE::TargetWorkTxNotFound => RC::PolicyViolation,
src/state/sequencer.rs:347:        TE::TargetWorkTxNotVerifiable => RC::PolicyViolation,
src/state/sequencer.rs:356:        // Non-WorkTx-arm variants documented per §3.7 mapping table — should
src/state/sequencer.rs:357:        // not occur on the WorkTx arm; conservative sentinel preserves L4.E
src/state/sequencer.rs:368:/// non-WorkTx-arm variants per Codex r2 P0-4.
src/state/sequencer.rs:385:        TransitionError::TargetWorkTxNotFound => Some("target_work_not_found".into()),
src/state/sequencer.rs:386:        TransitionError::TargetWorkTxNotVerifiable => Some("target_work_not_verifiable".into()),
src/state/sequencer.rs:531:            // TB-2 Atom 3: WorkTx pure validation per preflight v3 §3.3.
src/state/sequencer.rs:581:            // WP § 14.1 + § 18 Inv 5, accepted WorkTx commits stake by
src/state/sequencer.rs:598:            // accepted WorkTx atomically debits balance + locks stake into
src/state/sequencer.rs:600:            // the WorkTx itself; per Law 2 ("Only Investment Costs Money")
src/state/sequencer.rs:621:            // accepted WorkTx with stake > 0 derives a `FirstLong`
src/state/sequencer.rs:622:            // NodePosition exposure record. Pure additive index write —
src/state/sequencer.rs:625:            // logic). NodePosition.amount is **NOT a Coin holding** per
src/state/sequencer.rs:627:            // FR-12.1 + FR-12.4: kind = FirstLong; node_id = work.tx_id;
src/state/sequencer.rs:632:                let position = crate::state::typed_tx::NodePosition {
src/state/sequencer.rs:638:                    kind: crate::state::typed_tx::PositionKind::FirstLong,
src/state/sequencer.rs:645:                    .node_positions_t
src/state/sequencer.rs:681:            // proxy for "ever accepted as live WorkTx" (charter § 4.3 step 3
src/state/sequencer.rs:730:            // WorkTx would create a SECOND claim entry (its own claim_id),
src/state/sequencer.rs:735:            // task_market entry exists (must — WorkTx admission already
src/state/sequencer.rs:746:                // the same WorkTx must NOT create a second claim row — that
src/state/sequencer.rs:873:            // accepted ChallengeTx with stake > 0 derives a `ChallengeShort`
src/state/sequencer.rs:874:            // NodePosition exposure record. Pure additive index write —
src/state/sequencer.rs:877:            // TB-4 economic logic). NodePosition.amount is **NOT a Coin
src/state/sequencer.rs:879:            // unchanged. FR-12.2 + FR-12.5: kind = ChallengeShort; node_id
src/state/sequencer.rs:885:                // Q-derive task_id from the target WorkTx's stake row.
src/state/sequencer.rs:893:                let position = crate::state::typed_tx::NodePosition {
src/state/sequencer.rs:899:                    kind: crate::state::typed_tx::PositionKind::ChallengeShort,
src/state/sequencer.rs:906:                    .node_positions_t
src/state/sequencer.rs:1199:            // Step 4: no Open challenge_cases targeting this task's WorkTxs
src/state/sequencer.rs:3141:        TaskExpireTx, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
src/state/sequencer.rs:3191:    fn fixture_work_tx() -> WorkTx {
src/state/sequencer.rs:3200:        WorkTx {
src/state/sequencer.rs:3270:    //    TB-2 Atom 3: was NotYetImplemented pre-Atom-3; now WorkTx arm runs
src/state/sequencer.rs:3291:    // Drives apply_one with a known submit_id and a WorkTx that fails the
src/state/sequencer.rs:3325:        assert_eq!(pre, post, "rejected WorkTx leaves state_root_t unchanged");
src/state/sequencer.rs:3344:        // WorkTx arm rejects with EscrowMissing.
src/state/sequencer.rs:3386:    // TB-2 Atom 3 — U3: dispatch_transition WorkTx returns the interim
src/state/sequencer.rs:3390:    // pub(crate) test surface) with a predicate-passing WorkTx + stake>0 +
src/state/sequencer.rs:3445:        // Now construct WorkTx with parent matching the funded state's state_root.
src/state/sequencer.rs:3450:            .expect("predicate-passing WorkTx with funded task + solvent solver must accept");
src/state/sequencer.rs:3465:        // **TB-3 Atom 6 charter § 3.4 lock-on-accept**: accepted WorkTx now
src/state/sequencer.rs:3469:            "TB-3: accepted WorkTx commits stake (debits balance + credits stakes_t)");
src/state/sequencer.rs:3705:    // TB-3 Atom 6 — WorkTx arm refactor tests (charter § 4.7 U9-U11)
src/state/sequencer.rs:3738:    fn fixture_worktx_v(task: &str, agent: &str, parent: Hash, stake_micro: i64, suffix: &str, predicate_passes: bool) -> WorkTx {
src/state/sequencer.rs:3744:        WorkTx {
src/state/sequencer.rs:3763:    /// U9 — WorkTx admission via formal surface (no bridge): predicate-passing
src/state/sequencer.rs:3764:    /// WorkTx after open + lock + balance setup is accepted; state_root advances.
src/state/sequencer.rs:3778:            "WorkTx with funded task + solvent solver must accept via formal surface; got {:?}", result);
src/state/sequencer.rs:3785:    /// U10 — WorkTx admission rejects when solver balance < stake.
src/state/sequencer.rs:3803:    /// U11 — Accepted WorkTx debits balance + credits stakes_t with task_id binding.
src/state/sequencer.rs:3845:        assert_eq!(pre_total, post_total, "CTF conserved across WorkTx accept");
src/state/sequencer.rs:3851:    /// "live target WorkTx"). For Verify/Challenge unit tests that only
src/state/sequencer.rs:3852:    /// need target liveness, NOT the full TaskOpen+EscrowLock+WorkTx flow.
src/state/sequencer.rs:3854:    /// WorkTx by tx_id.
src/state/sequencer.rs:4508:        // Build a WorkTx fixture and submit through apply_one directly.
src/state/mod.rs:27:    EscrowsIndex, Hash, NodeId, NodePositionsIndex, PerAgentState, QState,
src/state/mod.rs:43:    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, NodePosition, PositionKind,
src/state/mod.rs:49:    WorkTx, WriteKey,
src/state/price_index.rs:6://! `node_positions_t` (TB-12 substrate) plus `conditional_share_balances_t`
src/state/price_index.rs:25:use crate::state::typed_tx::{EventId, NodePosition, PositionSide, ShareAmount};
src/state/price_index.rs:90:/// - `node_id` — the `TxId` of the WorkTx attempt-node these positions reference
src/state/price_index.rs:91:/// - `task_id` — the `TaskId` (Q-derived from any underlying `NodePosition`)
src/state/price_index.rs:93:/// - `long_interest` / `short_interest` — sum of `NodePosition.amount` per side
src/state/price_index.rs:116:/// derive the per-node `PriceIndex` from `EconomicState.node_positions_t`
src/state/price_index.rs:134:    // Pass 1: group NodePositions by node_id; collect (task_id, long_micro, short_micro).
src/state/price_index.rs:136:    for position in econ.node_positions_t.0.values() {
src/state/price_index.rs:441:    use crate::state::typed_tx::PositionKind;
src/state/price_index.rs:454:        kind: PositionKind,
src/state/price_index.rs:456:    ) -> NodePosition {
src/state/price_index.rs:457:        NodePosition {
src/state/price_index.rs:470:    fn econ_with_positions(positions: Vec<NodePosition>) -> EconomicState {
src/state/price_index.rs:473:            econ.node_positions_t.0.insert(p.position_id.clone(), p);
src/state/price_index.rs:494:            PositionKind::FirstLong,
src/state/price_index.rs:529:            PositionKind::ChallengeShort,
src/state/price_index.rs:559:                PositionKind::FirstLong,
src/state/price_index.rs:568:                PositionKind::ChallengeShort,
src/state/price_index.rs:601:                PositionKind::FirstLong,
src/state/price_index.rs:610:                PositionKind::ChallengeShort,
src/state/price_index.rs:635:                PositionKind::FirstLong,
src/state/price_index.rs:644:                PositionKind::ChallengeShort,
src/state/price_index.rs:653:                PositionKind::ChallengeShort,
src/state/price_index.rs:677:            PositionKind::FirstLong,
src/state/typed_tx.rs:71:/// TRACE_MATRIX § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay).
src/state/typed_tx.rs:77:/// TRACE_MATRIX § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay).
src/state/typed_tx.rs:85:/// TRACE_MATRIX § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519
src/state/typed_tx.rs:147:/// TRACE_MATRIX § 1.2 WorkTx field 8 — runner-stamped predicate results
src/state/typed_tx.rs:219:/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
src/state/typed_tx.rs:223:pub struct WorkTx {
src/state/typed_tx.rs:591:// § 5a-TB-12 — NodePosition exposure record (architect 2026-05-03 ruling)
src/state/typed_tx.rs:594:// insight: NodePosition is **immutable exposure record**, not active position
src/state/typed_tx.rs:601://   No MarketBuy / MarketSell PositionKind variants (TB-13+ trading layer).
src/state/typed_tx.rs:608:/// derived from accepted `WorkTx.stake`; Short is from accepted
src/state/typed_tx.rs:625:/// position kind. **TB-12 only ships FirstLong + ChallengeShort.**
src/state/typed_tx.rs:632:pub enum PositionKind {
src/state/typed_tx.rs:633:    /// Position derived from accepted `WorkTx.stake` (work-side commitment
src/state/typed_tx.rs:634:    /// to a node). FirstLong.node_id == own work_tx_id (architect §4 +
src/state/typed_tx.rs:636:    FirstLong = 0,
src/state/typed_tx.rs:638:    /// commitment). ChallengeShort.node_id == challenge.target_work_tx
src/state/typed_tx.rs:640:    ChallengeShort = 1,
src/state/typed_tx.rs:643:impl Default for PositionKind {
src/state/typed_tx.rs:645:        Self::FirstLong
src/state/typed_tx.rs:659:/// - NOT a Coin holding (CR-12.1; NodePosition.amount is NOT in
src/state/typed_tx.rs:671:/// - `FirstLong`: `node_id == source_tx == work.tx_id`,
src/state/typed_tx.rs:673:///   `side == Long`, `kind == FirstLong`.
src/state/typed_tx.rs:674:/// - `ChallengeShort`: `node_id == challenge.target_work_tx`,
src/state/typed_tx.rs:678:///   `kind == ChallengeShort`.
src/state/typed_tx.rs:680:pub struct NodePosition {
src/state/typed_tx.rs:686:    pub kind: PositionKind,
src/state/typed_tx.rs:698://   inline in `WorkTx.stake` per WP § 14.1 + § 18 Inv 5.
src/state/typed_tx.rs:711:/// `WorkTx` admission step 2 (TB-3 charter § 3.1).
src/state/typed_tx.rs:853:/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
src/state/typed_tx.rs:1279:impl WorkTx {
src/state/typed_tx.rs:1491:/// stays inline in `WorkTx.stake` per WP § 14.1 + § 18 Inv 5; no separate
src/state/typed_tx.rs:1496:pub enum TypedTx {
src/state/typed_tx.rs:1497:    Work(WorkTx),
src/state/typed_tx.rs:1547:impl HasSubmitter for WorkTx {
src/state/typed_tx.rs:1614:// owner / provider on the wire (mirrors WorkTx → agent_id pattern).
src/state/typed_tx.rs:1689:    TargetWorkTxNotFound,
src/state/typed_tx.rs:1691:    TargetWorkTxNotVerifiable,
src/state/typed_tx.rs:1745:    /// WorkTx-arm escrow / task-market lookup miss. The bridged
src/state/typed_tx.rs:1751:    /// `assert_total_ctf_conserved` failed on the WorkTx arm. Maps to
src/state/typed_tx.rs:1760:    /// `EscrowLockTx` / `WorkTx` admission referenced a `task_id` not in
src/state/typed_tx.rs:1764:    /// `EscrowLockTx` sponsor or accepted-`WorkTx` solver lacks balance
src/state/typed_tx.rs:1777:    /// stakes_t` — i.e., the target was never accepted as a live WorkTx,
src/state/typed_tx.rs:1781:    /// `TargetWorkTxNotFound` (reserved for "tx_id has no L4 row at all"
src/state/typed_tx.rs:1783:    /// and `TargetWorkTxNotVerifiable` (reserved for "target tx_id exists
src/state/typed_tx.rs:1784:    /// but is not a WorkTx type" — also unreachable in TB-4 since the
src/state/typed_tx.rs:1786:    /// `lock-on-accept` only inserts stakes_t entries for accepted WorkTx).
src/state/typed_tx.rs:1889:            Self::TargetWorkTxNotFound => write!(f, "target work_tx not found"),
src/state/typed_tx.rs:1890:            Self::TargetWorkTxNotVerifiable => write!(f, "target work_tx not in a verifiable state"),
src/state/typed_tx.rs:2072:    fn fixture_work_tx() -> WorkTx {
src/state/typed_tx.rs:2089:        WorkTx {
src/state/typed_tx.rs:2368:    /// Building the same WorkTx via different `BTreeSet` insertion orders produces
src/state/typed_tx.rs:2374:        let make_work_tx = |read_keys_in_order: &[&str]| -> WorkTx {
src/state/typed_tx.rs:2398:            TypedTx::Work(WorkTx::default()),
src/state/typed_tx.rs:2470:        // WorkTx (agent-signed)
src/state/typed_tx.rs:2673:        let make_work_tx = |insertion_order: &[(&str, bool)]| -> WorkTx {
src/state/typed_tx.rs:2996:    /// 2 reserved-existing variants (TargetWorkTxNotFound +
src/state/typed_tx.rs:2997:    /// TargetWorkTxNotVerifiable) — establishing the directive's Q3 three-class
src/state/typed_tx.rs:3005:        let s_not_found = format!("{}", TransitionError::TargetWorkTxNotFound);
src/state/typed_tx.rs:3006:        let s_not_verif = format!("{}", TransitionError::TargetWorkTxNotVerifiable);
src/state/typed_tx.rs:3017:        // TargetWorkTxNotFound, TargetWorkTxNotVerifiable are distinct.

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1158,1185p;1740,1788p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1158	    // V3 had Math/Bull/Bear roles. V4: tactic-strategy specialization.
  1159	    let agent_skills: Vec<&str> = vec![
  1160	        "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith.",
  1161	        "Focus on structural reasoning: induction, cases, rcases, constructor.",
  1162	        "Focus on rewriting and normalization: simp, norm_num, rw, calc.",
  1163	    ];
  1164	
  1165	    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
  1166	    // TB-14 Atom 6 (FC2-N29 production wire-up): integer-rational policy
  1167	    // loaded once at run start. `from_env()` reads BOLTZMANN_BETA_NUM/DEN,
  1168	    // BOLTZMANN_MIN_LIQUIDITY_MICRO, BOLTZMANN_PRICE_MARGIN_NUM/DEN,
  1169	    // BOLTZMANN_EPSILON_NUM/DEN; unparsable values silently fall back to
  1170	    // the per-field default (Art.I.1 + C-027). Replay-deterministic
  1171	    // boundary: `boltzmann_select_parent_v2(price_index, mask_set, &policy,
  1172	    // &mut rng)` is pure given a fixed policy + seeded RNG (Art.0.2).
  1173	    let policy = BoltzmannMaskPolicy::from_env();
  1174	    // C-012: seed the Boltzmann RNG so A/B runs are reproducible.
  1175	    // Only the LLM sampling remains stochastic; same-problem paired comparison absorbs that.
  1176	    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
  1177	        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
  1178	    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);
  1179	
  1180	    // Phase A atom A5 (FC2-N22 budget regime resolution): read
  1181	    // BUDGET_REGIME + MAX_TRANSACTIONS env, validate at startup, and
  1182	    // compute the loop bound. Errors abort BEFORE any LLM call so a
  1183	    // misconfigured run cannot consume API budget. Default
  1184	    // (env unset) = TotalProposal × 200, preserving Phase B baseline
  1185	    // bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3.
  1740	                                    if let Some(tx_id) = real_worktx_tx_id {
  1741	                                        last_tx_by_agent.insert(agent_id.to_string(), tx_id);
  1742	                                    }
  1743	                                }
  1744	
  1745	                                // shadow_only: kernel.tape view sync for next-agent prompt
  1746	                                // context. NOT authoritative state — the L4 chain above is
  1747	                                // canonical. This call exists so the in-memory tape used by
  1748	                                // the next iteration's prompt builder reflects this
  1749	                                // proposal. Per TB-7 §4.0 option (3) + §6 #31 inheritance,
  1750	                                // this is annotated shadow_only and does NOT constitute
  1751	                                // authoritative state mutation. Removal contingent on
  1752	                                // kernel.tape becoming L4-derived (post-MVP refactor).
  1753	                                match bus.append(agent_id, payload, parent.as_deref()) {
  1754	                                    Ok(BusResult::Appended { node_id }) => {
  1755	                                        info!("[tx {}] {} +{}", tx, agent_id, node_id);
  1756	                                        // Art. III.2 Librarian: every compress_interval appends,
  1757	                                        // write mechanical summary (TopK error classes) to agent's
  1758	                                        // learned.md. This is white-box compression (Art. I.2:
  1759	                                        // deterministic statistical algorithm), not LLM-based.
  1760	                                        if let Some(lib) = bus.tools.iter()
  1761	                                            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>()) {
  1762	                                            if lib.should_compress() {
  1763	                                                let errors = bus.recent_rejections(agent_id, 10);
  1764	                                                let summary = format!(
  1765	                                                    "# Learned patterns (auto-compressed)\n\
  1766	                                                     Common errors: {}\n\
  1767	                                                     Tape depth: {}\n",
  1768	                                                    errors.join(", "),
  1769	                                                    snap.tape.time_arrow().len(),
  1770	                                                );
  1771	                                                let _ = lib.write_agent_memory(agent_id, &summary);
  1772	                                                info!("[tx {}] Librarian compressed for {}", tx, agent_id);
  1773	                                            }
  1774	                                        }
  1775	                                    }
  1776	                                    Ok(BusResult::Vetoed { reason }) => {
  1777	                                        warn!("[tx {}] VETO: {}", tx, reason);
  1778	                                    }
  1779	                                    _ => {}
  1780	                                }
  1781	                            }
  1782	                        }
  1783	                        "complete" => {
  1784	                            *tool_dist.entry("complete".into()).or_insert(0) += 1;
  1785	                            if let Some(payload) = &action.payload {
  1786	                                // Art. IV (∏p(output | Q_t)): Q_t (tape) feeds the verification
  1787	                                // predicate. Dual-path: try payload-alone first (standalone proof
  1788	                                // preserved), then tape+payload (tape-built proof). Accept whichever

exec
/bin/bash -lc "nl -ba src/bus.rs | sed -n '140,310p;310,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   140	    /// Returns `Err(SubmitError::QueueClosed)` when the bus runs in
   141	    /// legacy-only mode (no Sequencer wired).
   142	    ///
   143	    /// TRACE_MATRIX § 5.2.1 — typed-tx submission entry.
   144	    pub async fn submit_typed_tx(
   145	        &self,
   146	        tx: TypedTx,
   147	    ) -> Result<SubmissionReceipt, SubmitError> {
   148	        match self.sequencer.as_ref() {
   149	            Some(seq) => seq.submit(tx).await,
   150	            None => Err(SubmitError::QueueClosed),
   151	        }
   152	    }
   153	
   154	    /// Phase 1: open with WAL persistence. If the path exists, replay it to
   155	    /// rebuild tape + ledger state (resume mode). If not, start fresh and append
   156	    /// to the WAL going forward (durable mode). Either way, the Wal handle is
   157	    /// retained and every successful tape.append / ledger.append persists.
   158	    pub fn with_wal_path(
   159	        kernel: Kernel,
   160	        config: BusConfig,
   161	        wal_path: impl Into<std::path::PathBuf>,
   162	    ) -> Result<Self, std::io::Error> {
   163	        let wal_path = wal_path.into();
   164	        let mut bus = Self::new(kernel, config);
   165	        // Replay first (if file exists), then open in append mode.
   166	        let (nodes, events) = crate::wal::Wal::replay(&wal_path)?;
   167	        let resumed_nodes = nodes.len();
   168	        let resumed_events = events.len();
   169	        for n in nodes {
   170	            // Replay errors are tolerable — duplicates and dangling cites can
   171	            // happen if the WAL was concurrently appended at a stale point. We
   172	            // log and skip; the surviving prefix is canonical Q_t.
   173	            if let Err(e) = bus.kernel.append(n.clone()) {
   174	                eprintln!("[wal/replay] skip node {}: {}", n.id, e);
   175	            }
   176	        }
   177	        for e in events {
   178	            // Re-append events through the ledger so hash chain is recomputed
   179	            // from this process's perspective. Original hashes are discarded.
   180	            bus.ledger.append(e.event_type, e.node_id, e.agent, e.detail).ok();
   181	        }
   182	        if resumed_nodes > 0 || resumed_events > 0 {
   183	            eprintln!("[wal/replay] resumed {} nodes, {} events from {:?}",
   184	                      resumed_nodes, resumed_events, wal_path);
   185	        }
   186	        bus.wal = Some(crate::wal::Wal::open(&wal_path)?);
   187	        Ok(bus)
   188	    }
   189	
   190	    /// Mount a tool into the bus. Tools execute in mount order.
   191	    pub fn mount_tool(&mut self, tool: Box<dyn TuringTool>) {
   192	        self.tools.push(tool);
   193	    }
   194	
   195	    /// Boot all tools.
   196	    pub fn boot(&mut self) {
   197	        for tool in &mut self.tools {
   198	            tool.on_boot();
   199	        }
   200	    }
   201	
   202	    /// Initialize all tools with agent list. Triggers GENESIS.
   203	    ///
   204	    /// TB-14 Atom 6 (2026-05-03): legacy `HAYEK_BOUNTY` env-gated bounty
   205	    /// market open was excised together with `kernel.open_bounty_market`.
   206	    /// Capital signals now live entirely in `state::NodePositionsIndex`
   207	    /// (TB-12) and surface via `compute_price_index` derived view (TB-14).
   208	    pub fn init(&mut self, agent_ids: &[String]) {
   209	        for tool in &mut self.tools {
   210	            tool.on_init(agent_ids);
   211	        }
   212	        if let Ok(evt) = self.ledger.append(EventType::RunStart, None, None, None) {
   213	            let evt_clone = evt.clone();
   214	            if let Some(w) = self.wal.as_mut() {
   215	                let _ = w.write_event(&evt_clone);
   216	            }
   217	        }
   218	    }
   219	
   220	    /// The main append pipeline — 6 phases.
   221	    /// V3L-11: this runs serially, never concurrently.
   222	    pub fn append(&mut self, author: &str, payload: &str,
   223	                  parent_id: Option<&str>) -> Result<BusResult, String> {
   224	        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ false)
   225	    }
   226	
   227	    /// Phase 2.1 (C-043 candidate): bypass agent-facing gates for ∏p-blessed payloads.
   228	    /// The forbidden_patterns list (C-011) exists to prevent agents from appending
   229	    /// brute-force tactics (e.g. bare `decide`, `omega`, `native_decide`) as scratch
   230	    /// work. Once the Lean oracle has accepted a full proof, those same tactics are
   231	    /// by construction legitimate — re-rejecting at bus level would block the
   232	    /// wtool write that Art. IV mandates. Only oracle-accepted payloads should
   233	    /// take this path. Payload-size caps are also relaxed (proofs are longer than
   234	    /// agent scratch steps).
   235	    pub fn append_oracle_accepted(&mut self, author: &str, payload: &str,
   236	                                   parent_id: Option<&str>) -> Result<BusResult, String> {
   237	        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ true)
   238	    }
   239	
   240	    fn append_internal(&mut self, author: &str, payload: &str,
   241	                       parent_id: Option<&str>, oracle_blessed: bool) -> Result<BusResult, String> {
   242	        // Phase 0: Forbidden pattern check — skipped for oracle-accepted payloads.
   243	        if !oracle_blessed {
   244	            for pattern in &self.config.forbidden_patterns {
   245	                if payload.contains(pattern.as_str()) {
   246	                    let reason = format!("Forbidden pattern: {}", pattern);
   247	                    self.record_rejection(author, &reason);
   248	                    return Ok(BusResult::Vetoed { reason });
   249	                }
   250	            }
   251	        }
   252	
   253	        // Phase 0b: Payload size limits (V3L-21). Skipped for oracle-accepted since
   254	        // real proofs can legitimately exceed the per-step scratch budget.
   255	        if !oracle_blessed {
   256	            if payload.len() > self.config.max_payload_chars {
   257	                let reason = format!("Payload too long: {} > {} chars",
   258	                                     payload.len(), self.config.max_payload_chars);
   259	                self.record_rejection(author, &reason);
   260	                return Ok(BusResult::Vetoed { reason });
   261	            }
   262	            let line_count = payload.lines().count();
   263	            if line_count > self.config.max_payload_lines {
   264	                let reason = format!("Too many lines: {} > {}",
   265	                                     line_count, self.config.max_payload_lines);
   266	                self.record_rejection(author, &reason);
   267	                return Ok(BusResult::Vetoed { reason });
   268	            }
   269	        }
   270	
   271	        // Phase 1: Tool pre-append hooks
   272	        // TB-9 collapse (2026-05-02): InvestOnly routing deleted along with the
   273	        // bus-level f64 wallet mutators (debit_wallet/credit_wallet/settle_portfolios).
   274	        // Per architect directive 2026-05-02 line 1574 ("no f64 mutation;
   275	        // EconomicState canonical"), the v3 share-buy path is gone. Stake
   276	        // commitment now lives in `state::typed_tx::WorkTx.stake` mutating
   277	        // `EconomicState.stakes_t` via the canonical sequencer dispatch arm.
   278	        // YieldReward signals continue to be observed but are not routed to a
   279	        // f64 mutator — they live for downstream tool hooks only.
   280	        for tool in &mut self.tools {
   281	            match tool.on_pre_append(author, payload) {
   282	                ToolSignal::Veto(reason) => {
   283	                    self.record_rejection(author, &reason);
   284	                    return Ok(BusResult::Vetoed { reason });
   285	                }
   286	                ToolSignal::InvestOnly { .. } => {
   287	                    let reason = "veto:invest_disabled_tb9".to_string();
   288	                    self.record_rejection(author, &reason);
   289	                    return Ok(BusResult::Vetoed { reason });
   290	                }
   291	                ToolSignal::YieldReward { .. } | ToolSignal::Pass => {}
   292	            }
   293	        }
   294	
   295	        // Phase 3: Kernel append (topology validation)
   296	        let node_id = format!("tx_{}_by_{}", self.tx_count, author);
   297	        let citations = parent_id.map(|p| vec![p.to_string()]).unwrap_or_default();
   298	
   299	        let node = Node {
   300	            id: node_id.clone(),
   301	            author: author.to_string(),
   302	            payload: payload.to_string(),
   303	            citations,
   304	            created_at: std::time::SystemTime::now()
   305	                .duration_since(std::time::UNIX_EPOCH)
   306	                .unwrap_or_default()
   307	                .as_secs(),
   308	            completion_tokens: PENDING_COMPLETION_TOKENS_CO1_1_4,
   309	        };
   310	
   310	
   311	        self.kernel.append(node.clone()).map_err(|e| e.to_string())?;
   312	
   313	        // Phase 1 WAL: persist node AFTER successful in-memory append, BEFORE
   314	        // any downstream effects. At-most-one-loss-on-crash semantics: if the
   315	        // process dies between in-memory insert and this write, the node is
   316	        // lost on replay but every prior node survives. Log+continue on I/O
   317	        // error rather than aborting the run (Q_t durability is best-effort
   318	        // when disk is the failing component).
   319	        if let Some(w) = self.wal.as_mut() {
   320	            if let Err(e) = w.write_node(&node) {
   321	                log::warn!("[wal] write_node({}) failed: {}", node.id, e);
   322	            }
   323	        }
   324	
   325	        // Phase 4: TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
   326	        // legacy `kernel.create_market(node_id, system_lp_amount)` per-append
   327	        // CPMM market open was excised together with `prediction_market.rs`.
   328	        // Pricing is now a derived view over canonical `EconomicState`
   329	        // (`state::compute_price_index`) populated by typed-tx admission via
   330	        // `Sequencer::dispatch_transition` — never by bus-level f64 LP grant.
   331	
   332	        // Phase 5: Tool post-append hooks
   333	        for tool in &mut self.tools {
   334	            tool.on_post_append(author, &node_id);
   335	        }
   336	
   337	        if let Ok(evt) = self.ledger.append(EventType::Append, Some(node_id.clone()),
   338	                                             Some(author.to_string()), None) {
   339	            // Phase 1 WAL: persist ledger event for full hash-chain recovery.
   340	            if let Some(w) = self.wal.as_mut() {
   341	                let evt_clone = evt.clone();
   342	                if let Err(e) = w.write_event(&evt_clone) {
   343	                    log::warn!("[wal] write_event(Append) failed: {}", e);
   344	                }
   345	            }
   346	        }
   347	        self.tx_count += 1;
   348	        self.clock += 1;
   349	
   350	        Ok(BusResult::Appended { node_id })
   351	    }
   352	
   353	    /// Halt and settle — triggered by Oracle verification.
   354	    ///
   355	    /// TB-14 Atom 6 (2026-05-03): legacy `kernel.resolve_all(golden_path)`
   356	    /// CPMM market resolution was excised together with `prediction_market.rs`.
   357	    /// Settlement lives entirely in canonical typed-tx dispatch arms
   358	    /// (`FinalizeRewardTx` since TB-8) via `Sequencer::apply_one`; the bus
   359	    /// only fires the run-end event and lets tool hooks observe the golden
   360	    /// path.
   361	    pub fn halt_and_settle(&mut self, golden_path: &[NodeId]) -> Result<(), String> {
   362	        let gp: Vec<String> = golden_path.to_vec();
   363	        for tool in &mut self.tools {
   364	            tool.on_halt(&gp);
   365	        }
   366	
   367	        if let Ok(evt) = self.ledger.append(EventType::RunEnd, None, None, None) {
   368	            let evt_clone = evt.clone();
   369	            if let Some(w) = self.wal.as_mut() {
   370	                let _ = w.write_event(&evt_clone);
   371	            }
   372	        }
   373	        Ok(())
   374	    }
   375	
   376	    /// Record a rejection in the graveyard.
   377	    /// Step-B v3: ALL stored entries are bounded class labels (C-022 shield enforced at write).
   378	    /// If `reason` is already a valid class label (starts with "err:"), stored as-is.
   379	    /// Otherwise normalized to a bus-level class via `bus_classify`.
   380	    /// Exposed publicly so evaluator.rs can populate from OMEGA-reject and parse-fail.
   381	    pub fn record_rejection(&mut self, author: &str, reason: &str) {
   382	        let label = Self::bus_classify(reason);
   383	        self.graveyard
   384	            .entry(author.to_string())
   385	            .or_default()
   386	            .push(label.to_string());
   387	    }
   388	
   389	    /// Bus-level classifier: coerces any rejection reason to a bounded label.
   390	    /// This is the write-side shield that enforces Art. II.1 end-to-end.
   391	    /// The finite label set is the union of:
   392	    ///   - "err:" prefixed labels from sdk::error_abstraction (caller-classified)
   393	    ///   - "veto:forbidden", "veto:size", "veto:lines", "veto:wallet", "veto:tool_other"
   394	    ///     (bus-internal veto classes)
   395	    ///   - "err:other" catchall
   396	    pub fn bus_classify(reason: &str) -> &'static str {
   397	        // If caller already produced an "err:..." class label, trust it.
   398	        // Validate prefix; the length is bounded because the enum of labels is finite.
   399	        if reason.starts_with("err:") {
   400	            // Accept as-is but intern to static slice where possible.
   401	            // For simplicity we allocate a leaked &'static; safer: fixed mapping of known labels.
   402	            // Here we collapse unknown "err:*" to err:other to preserve finite-set invariant.
   403	            return match reason {
   404	                "err:tactic_linarith" => "err:tactic_linarith",
   405	                "err:tactic_simp_noprog" => "err:tactic_simp_noprog",
   406	                "err:tactic_ring" => "err:tactic_ring",
   407	                "err:tactic_norm_num" => "err:tactic_norm_num",
   408	                "err:tactic_other" => "err:tactic_other",
   409	                "err:unknown_const" => "err:unknown_const",
   410	                "err:unsolved_goals" => "err:unsolved_goals",
   411	                "err:unexpected_token" => "err:unexpected_token",
   412	                "err:type_mismatch" => "err:type_mismatch",
   413	                "err:rewrite_no_match" => "err:rewrite_no_match",
   414	                "err:heartbeat" => "err:heartbeat",
   415	                "err:other" => "err:other",
   416	                _ => "err:other",
   417	            };
   418	        }
   419	        // Bus internal veto reasons get their own bounded classes.
   420	        if reason.starts_with("Forbidden") { return "veto:forbidden"; }
   421	        if reason.starts_with("Payload too long") { return "veto:size"; }
   422	        if reason.starts_with("Too many lines") { return "veto:lines"; }
   423	        if reason.contains("wallet") || reason.contains("balance") { return "veto:wallet"; }
   424	        if reason.starts_with("Tool") || reason.contains("tool") { return "veto:tool_other"; }
   425	        "err:other"
   426	    }
   427	
   428	    /// Get recent rejections for an agent (Art. II.1: broadcast typical errors).
   429	    /// v3 Step-B: default scope changed to TopKClasses(3) — globally abstract-and-broadcast.
   430	    /// Call sites that explicitly want per-author scope use `recent_rejections_scoped`.

exec
/bin/bash -lc "nl -ba src/kernel.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 1: Pure topology (DAG)
     2	// Constitutional basis: Law 1 (zero domain knowledge)
     3	// V3L-45: no domain strings. V3L-23: no hardcoded params.
     4	//
     5	// CRITICAL: This module must NEVER contain domain-specific terms.
     6	// R-001 enforced by judge.sh — any edit is scanned.
     7	//
     8	// TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
     9	// All decimal-float CPMM scaffolding (`markets`, `bounty_market`,
    10	// `bounty_lp_seed`, `create_market`, `buy_yes`, `buy_no`, `yes_price`,
    11	// `market_ticker`, `market_ticker_full`, `open_bounty_market`,
    12	// `bounty_yes_price`, `resolve_bounty`, `resolve_all`) was excised
    13	// together with `src/prediction_market.rs`. Pricing now lives entirely
    14	// in the derived view `state::compute_price_index`; YES/NO claims live
    15	// in TB-13 `ConditionalShareBalances`. The kernel is pure topology
    16	// (V3L-45 docstring contract restored).
    17	
    18	use crate::ledger::{Node, NodeId, Tape, TapeError};
    19	use serde::{Deserialize, Serialize};
    20	
    21	// ── Core types ──────────────────────────────────────────────────
    22	
    23	/// The pure topology manager.
    24	/// It knows about nodes and edges (citations).
    25	/// It does NOT know what the nodes contain or what domain they belong to.
    26	#[derive(Debug, Default, Serialize, Deserialize)]
    27	pub struct Kernel {
    28	    pub tape: Tape,
    29	}
    30	
    31	/// Result of an append operation.
    32	#[derive(Debug)]
    33	pub struct AppendResult {
    34	    pub node_id: NodeId,
    35	}
    36	
    37	// ── Implementation ──────────────────────────────────────────────
    38	
    39	impl Kernel {
    40	    pub fn new() -> Self {
    41	        Kernel { tape: Tape::new() }
    42	    }
    43	
    44	    /// Append a node to the tape.
    45	    /// Only checks structural validity (topology).
    46	    /// Content validation is NOT this module's job (engine separation, C-003).
    47	    pub fn append(&mut self, node: Node) -> Result<AppendResult, KernelError> {
    48	        let node_id = node.id.clone();
    49	        self.tape.append(node).map_err(KernelError::Tape)?;
    50	        Ok(AppendResult { node_id })
    51	    }
    52	
    53	    /// Trace ancestors from a terminal node back to root(s).
    54	    /// Pure topology — path validity is determined externally.
    55	    pub fn trace_golden_path(&self, terminal_id: &str) -> Result<Vec<NodeId>, KernelError> {
    56	        if !self.tape.nodes().contains_key(terminal_id) {
    57	            return Err(KernelError::NodeNotFound(terminal_id.to_string()));
    58	        }
    59	        Ok(self.tape.trace_ancestors(terminal_id))
    60	    }
    61	}
    62	
    63	// ── Errors ──────────────────────────────────────────────────────
    64	
    65	#[derive(Debug)]
    66	pub enum KernelError {
    67	    Tape(TapeError),
    68	    NodeNotFound(String),
    69	}
    70	
    71	impl std::fmt::Display for KernelError {
    72	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    73	        match self {
    74	            KernelError::Tape(e) => write!(f, "Tape error: {}", e),
    75	            KernelError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
    76	        }
    77	    }
    78	}
    79	
    80	impl std::error::Error for KernelError {}
    81	
    82	// ── Tests ───────────────────────────────────────────────────────
    83	// NOTE: Domain-purity test lives in tests/kernel_purity.rs (outside this file)
    84	// because R-001 forbids domain terms even as test strings in kernel.rs.
    85	
    86	#[cfg(test)]
    87	mod tests {
    88	    use super::*;
    89	
    90	    fn make_node(id: &str, author: &str, payload: &str, citations: Vec<&str>) -> Node {
    91	        Node {
    92	            id: id.to_string(),
    93	            author: author.to_string(),
    94	            payload: payload.to_string(),
    95	            citations: citations.into_iter().map(|s| s.to_string()).collect(),
    96	            created_at: 0,
    97	            completion_tokens: 0,
    98	        }
    99	    }
   100	
   101	    #[test]
   102	    fn test_append_and_retrieve() {
   103	        let mut k = Kernel::new();
   104	        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
   105	        assert!(k.tape.get("n1").is_some());
   106	    }
   107	
   108	    #[test]
   109	    fn test_reject_duplicate() {
   110	        let mut k = Kernel::new();
   111	        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
   112	        assert!(k.append(make_node("n1", "A1", "step 2", vec![])).is_err());
   113	    }
   114	
   115	    #[test]
   116	    fn test_reject_dangling_citation() {
   117	        let mut k = Kernel::new();
   118	        assert!(k.append(make_node("n1", "A0", "step 1", vec!["ghost"])).is_err());
   119	    }
   120	
   121	    #[test]
   122	    fn test_golden_path_trace() {
   123	        let mut k = Kernel::new();
   124	        k.append(make_node("root", "A0", "root", vec![])).unwrap();
   125	        k.append(make_node("mid", "A1", "mid", vec!["root"])).unwrap();
   126	        k.append(make_node("leaf", "A0", "leaf", vec!["mid"])).unwrap();
   127	
   128	        let path = k.trace_golden_path("leaf").unwrap();
   129	        assert_eq!(path, vec!["root", "mid", "leaf"]);
   130	    }
   131	
   132	    #[test]
   133	    fn test_trace_golden_path_unknown_node() {
   134	        let k = Kernel::new();
   135	        assert!(matches!(
   136	            k.trace_golden_path("ghost"),
   137	            Err(KernelError::NodeNotFound(_))
   138	        ));
   139	    }
   140	}

exec
/bin/bash -lc 'rg -n "make_real_worktx_signed_by|build_for_evaluator_append_with_parent|tx_id" src/runtime experiments/minif2f_v4/src/bin/evaluator.rs tests/tb_14_chaintape_smoke.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_14_chaintape_smoke.rs:207:        tx_id: TxId("tb14-mint-1".into()),
tests/tb_14_chaintape_smoke.rs:226:        tx_id: TxId("tb14-redeem-1".into()),
experiments/minif2f_v4/src/bin/evaluator.rs:787:    // Map of agent_id → last tx_id submitted via bus.submit_typed_tx (Work or
experiments/minif2f_v4/src/bin/evaluator.rs:789:    // proposals get the previous tx_id as parent. This is what unlocks
experiments/minif2f_v4/src/bin/evaluator.rs:844:    // AgentProposalRecord in CAS + indexed under tx_id in
experiments/minif2f_v4/src/bin/evaluator.rs:984:        let task_open_tx_id =
experiments/minif2f_v4/src/bin/evaluator.rs:999:        let bad_worktx_tx_id = turingosv4::state::q_state::TxId(format!(
experiments/minif2f_v4/src/bin/evaluator.rs:1025:        // + logical_t. The index links L4 / L4.E tx_id → CAS record CID.
experiments/minif2f_v4/src/bin/evaluator.rs:1030:            &task_open_tx_id,
experiments/minif2f_v4/src/bin/evaluator.rs:1031:            &bad_worktx_tx_id,
experiments/minif2f_v4/src/bin/evaluator.rs:1053:        // TB-10 Atom 1+3: tx_id suffix depends on user-mode flag (mirrors the
experiments/minif2f_v4/src/bin/evaluator.rs:1631:                                // make_real_worktx_signed_by / submit_typed_tx exits
experiments/minif2f_v4/src/bin/evaluator.rs:1664:                                    let pt = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
experiments/minif2f_v4/src/bin/evaluator.rs:1712:                                        match turingosv4::runtime::adapter::make_real_worktx_signed_by(
experiments/minif2f_v4/src/bin/evaluator.rs:1725:                                                error!("[chaintape/atom2] FAIL-CLOSED: make_real_worktx_signed_by failed: {e}");
experiments/minif2f_v4/src/bin/evaluator.rs:1730:                                    // TB-7.7 D2: capture tx_id before move into submit_typed_tx.
experiments/minif2f_v4/src/bin/evaluator.rs:1731:                                    let real_worktx_tx_id = match &real_worktx {
experiments/minif2f_v4/src/bin/evaluator.rs:1732:                                        turingosv4::state::typed_tx::TypedTx::Work(w) => Some(w.tx_id.clone()),
experiments/minif2f_v4/src/bin/evaluator.rs:1740:                                    if let Some(tx_id) = real_worktx_tx_id {
experiments/minif2f_v4/src/bin/evaluator.rs:1741:                                        last_tx_by_agent.insert(agent_id.to_string(), tx_id);
experiments/minif2f_v4/src/bin/evaluator.rs:1927:                                            let pt_partial = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
experiments/minif2f_v4/src/bin/evaluator.rs:1950:                                            // accepted; verified=true). Deterministic work_tx_id is
experiments/minif2f_v4/src/bin/evaluator.rs:1951:                                            // `worktx-<task>-<suffix>` per make_real_worktx_signed_by.
experiments/minif2f_v4/src/bin/evaluator.rs:1953:                                            let work_tx_id_pre =
experiments/minif2f_v4/src/bin/evaluator.rs:1959:                                                work_tx_id_pre.clone(),
experiments/minif2f_v4/src/bin/evaluator.rs:2006:                                                match turingosv4::runtime::adapter::make_real_worktx_signed_by(
experiments/minif2f_v4/src/bin/evaluator.rs:2024:                                            let work_tx_id = match &work_tx {
experiments/minif2f_v4/src/bin/evaluator.rs:2025:                                                turingosv4::state::typed_tx::TypedTx::Work(w) => w.tx_id.clone(),
experiments/minif2f_v4/src/bin/evaluator.rs:2042:                                                    last_tx_by_agent.insert(agent_id.to_string(), work_tx_id.clone());
experiments/minif2f_v4/src/bin/evaluator.rs:2056:                                                    work_tx_id.clone(),
experiments/minif2f_v4/src/bin/evaluator.rs:2070:                                            let verify_tx_id = match &verify_tx {
experiments/minif2f_v4/src/bin/evaluator.rs:2071:                                                turingosv4::state::typed_tx::TypedTx::Verify(v) => Some(v.tx_id.clone()),
experiments/minif2f_v4/src/bin/evaluator.rs:2078:                                            let work_tx_id = Some(work_tx_id);
experiments/minif2f_v4/src/bin/evaluator.rs:2083:                                            if let Some(vid) = verify_tx_id.clone() {
experiments/minif2f_v4/src/bin/evaluator.rs:2098:                                            if let Some(tx_id) = verify_tx_id.or(work_tx_id) {
experiments/minif2f_v4/src/bin/evaluator.rs:2099:                                                last_tx_by_agent.insert(agent_id.to_string(), tx_id);
experiments/minif2f_v4/src/bin/evaluator.rs:2359:                                            let pt_partial = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
experiments/minif2f_v4/src/bin/evaluator.rs:2383:                                            let work_tx_id_pre =
experiments/minif2f_v4/src/bin/evaluator.rs:2389:                                                work_tx_id_pre.clone(),
experiments/minif2f_v4/src/bin/evaluator.rs:2436:                                                match turingosv4::runtime::adapter::make_real_worktx_signed_by(
experiments/minif2f_v4/src/bin/evaluator.rs:2454:                                            let work_tx_id = match &work_tx {
experiments/minif2f_v4/src/bin/evaluator.rs:2455:                                                turingosv4::state::typed_tx::TypedTx::Work(w) => w.tx_id.clone(),
experiments/minif2f_v4/src/bin/evaluator.rs:2472:                                                    let work_tx_id_str = work_tx_id.clone();
experiments/minif2f_v4/src/bin/evaluator.rs:2473:                                                    last_tx_by_agent.insert(agent_id.to_string(), work_tx_id_str);
experiments/minif2f_v4/src/bin/evaluator.rs:2490:                                                    work_tx_id.clone(),
experiments/minif2f_v4/src/bin/evaluator.rs:2504:                                            let verify_tx_id = match &verify_tx {
experiments/minif2f_v4/src/bin/evaluator.rs:2505:                                                turingosv4::state::typed_tx::TypedTx::Verify(v) => Some(v.tx_id.clone()),
experiments/minif2f_v4/src/bin/evaluator.rs:2514:                                            if let Some(vid) = verify_tx_id.clone() {
experiments/minif2f_v4/src/bin/evaluator.rs:2523:                                            let work_tx_id = Some(work_tx_id);
experiments/minif2f_v4/src/bin/evaluator.rs:2525:                                            if let Some(tx_id) = verify_tx_id.or(work_tx_id) {
experiments/minif2f_v4/src/bin/evaluator.rs:2526:                                                last_tx_by_agent.insert(agent_id.to_string(), tx_id);
experiments/minif2f_v4/src/bin/evaluator.rs:2845:                         {} accepted_tx_ids, {} rejected_tx_ids)",
experiments/minif2f_v4/src/bin/evaluator.rs:2848:                        summary.accepted_tx_ids.len(),
experiments/minif2f_v4/src/bin/evaluator.rs:2849:                        summary.rejected_tx_ids.len(),
src/runtime/adapter.rs:58:        tx_id: TxId(format!("taskopen-{}-{}", task, suffix)),
src/runtime/adapter.rs:79:        tx_id: TxId(format!("escrowlock-{}-{}", task, suffix)),
src/runtime/adapter.rs:112:        tx_id: TxId(format!("worktx-{}-{}", task, suffix)),
src/runtime/adapter.rs:152:pub fn make_real_worktx_signed_by(
src/runtime/adapter.rs:174:    let tx_id = TxId(format!("worktx-{}-{}", task, suffix));
src/runtime/adapter.rs:186:        tx_id: tx_id.clone(),
src/runtime/adapter.rs:201:        tx_id,
src/runtime/adapter.rs:238:    let tx_id = TxId(format!("verifytx-{}-{}", verifier_agent, suffix));
src/runtime/adapter.rs:247:        tx_id: tx_id.clone(),
src/runtime/adapter.rs:259:        tx_id,
src/runtime/adapter.rs:303:    let tx_id = TxId(format!("taskopen-{}-{}", task, suffix));
src/runtime/adapter.rs:305:        tx_id: tx_id.clone(),
src/runtime/adapter.rs:317:        tx_id,
src/runtime/adapter.rs:346:    let tx_id = TxId(format!("escrowlock-{}-{}", task, suffix));
src/runtime/adapter.rs:349:        tx_id: tx_id.clone(),
src/runtime/adapter.rs:359:        tx_id,
src/runtime/adapter.rs:431:    verify_tx_id: &TxId,
src/runtime/adapter.rs:435:    let claim_id_inner = TxId(format!("claim-{}", verify_tx_id.0));
src/runtime/adapter.rs:517:/// For each eligible (task_id, escrow_tx_id) pair, emits one
src/runtime/adapter.rs:570:        for escrow_tx_id in entry.escrow_lock_tx_ids.iter() {
src/runtime/adapter.rs:571:            candidates.push((task_id.clone(), escrow_tx_id.clone(), reason));
src/runtime/adapter.rs:578:    for (task_id, escrow_tx_id, reason) in candidates {
src/runtime/adapter.rs:583:            if let Some(esc) = q_now.economic_state_t.escrows_t.0.get(&escrow_tx_id) {
src/runtime/adapter.rs:590:                escrow_tx_id,
src/runtime/adapter.rs:609:    /// U-A2.a — make_real_worktx_signed_by produces a non-zero signature
src/runtime/adapter.rs:616:        let tx = make_real_worktx_signed_by(
src/runtime/adapter.rs:636:            tx_id: work.tx_id.clone(),
src/runtime/adapter.rs:659:        let tx1 = make_real_worktx_signed_by(
src/runtime/adapter.rs:671:        let tx2 = make_real_worktx_signed_by(
src/runtime/proposal_telemetry.rs:186:    /// [`build_for_evaluator_append_with_parent`] instead.
src/runtime/proposal_telemetry.rs:198:        Self::build_for_evaluator_append_with_parent(
src/runtime/proposal_telemetry.rs:214:    /// Evaluator hot path passes `Some(last_tx_id)` for the same
src/runtime/proposal_telemetry.rs:217:    pub fn build_for_evaluator_append_with_parent(
src/runtime/proposal_telemetry.rs:522:    /// TB-7.7 D2 — build_for_evaluator_append_with_parent records parent_tx
src/runtime/proposal_telemetry.rs:529:        let pt = ProposalTelemetry::build_for_evaluator_append_with_parent(
src/runtime/mod.rs:26:/// TRACE_MATRIX FC3-N1: TB-6 Atom 5 — Agent audit trail (AgentProposalRecord + CAS storage + JSONL index linking tx_id → proposal_record_cid).
src/runtime/mod.rs:29:/// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — Branch / fork visibility summary (tx_count, failed_branch_count, rollback_count, accepted/rejected tx_id sets, candidate proposal CIDs).
src/runtime/genesis_report.rs:71:    /// `tx_id` of the preseed `TaskOpenTx` submitted at bootstrap.
src/runtime/genesis_report.rs:75:    /// `tx_id` of the preseed `EscrowLockTx` submitted at bootstrap.
src/runtime/run_summary.rs:5://! > proposal CIDs, accepted tx_id, rejected tx_ids.
src/runtime/run_summary.rs:11://!   yields the `TypedTx` and the variant's `tx_id` field.
src/runtime/run_summary.rs:13://!   CAS-lookup-and-decode shape for tx_id extraction.
src/runtime/run_summary.rs:22://! `RunSummary` records what was submitted (tx_id sets) and what failed
src/runtime/run_summary.rs:94:    /// from `rejected_tx_ids.len()`, which counts only chaintape-side L4.E
src/runtime/run_summary.rs:100:    /// Sorted, deduplicated list of `tx_id`s that landed in L4 (accepted).
src/runtime/run_summary.rs:101:    pub accepted_tx_ids: Vec<TxId>,
src/runtime/run_summary.rs:102:    /// Sorted, deduplicated list of `tx_id`s that landed in L4.E (rejected).
src/runtime/run_summary.rs:103:    pub rejected_tx_ids: Vec<TxId>,
src/runtime/run_summary.rs:117:    /// `tx_id` from the typed payload, and aggregate into a `RunSummary`.
src/runtime/run_summary.rs:135:        let mut accepted_tx_ids: BTreeSet<TxId> = BTreeSet::new();
src/runtime/run_summary.rs:147:            accepted_tx_ids.insert(extract_tx_id(&typed_tx));
src/runtime/run_summary.rs:159:        let mut rejected_tx_ids: BTreeSet<TxId> = BTreeSet::new();
src/runtime/run_summary.rs:167:                    rejected_tx_ids.insert(extract_tx_id(&typed_tx));
src/runtime/run_summary.rs:178:            accepted_tx_ids: accepted_tx_ids.into_iter().collect(),
src/runtime/run_summary.rs:179:            rejected_tx_ids: rejected_tx_ids.into_iter().collect(),
src/runtime/run_summary.rs:208:/// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — extract the `tx_id` field from each
src/runtime/run_summary.rs:211:fn extract_tx_id(tx: &TypedTx) -> TxId {
src/runtime/run_summary.rs:213:        TypedTx::Work(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:214:        TypedTx::Verify(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:215:        TypedTx::Challenge(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:216:        TypedTx::Reuse(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:217:        TypedTx::FinalizeReward(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:218:        TypedTx::TaskExpire(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:219:        TypedTx::TerminalSummary(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:220:        TypedTx::TaskOpen(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:221:        TypedTx::EscrowLock(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:222:        TypedTx::ChallengeResolve(t) => t.tx_id.clone(),
src/runtime/run_summary.rs:223:        TypedTx::TaskBankruptcy(t) => t.tx_id.clone(),  // TB-11
src/runtime/run_summary.rs:224:        TypedTx::CompleteSetMint(t) => t.tx_id.clone(), // TB-13
src/runtime/run_summary.rs:225:        TypedTx::CompleteSetRedeem(t) => t.tx_id.clone(), // TB-13
src/runtime/run_summary.rs:226:        TypedTx::MarketSeed(t) => t.tx_id.clone(),      // TB-13
src/runtime/run_summary.rs:244:            accepted_tx_ids: vec![TxId("worktx-1".into())],
src/runtime/run_summary.rs:245:            rejected_tx_ids: vec![TxId("worktx-2".into()), TxId("worktx-3".into())],
src/runtime/run_summary.rs:263:            accepted_tx_ids: vec![],
src/runtime/run_summary.rs:264:            rejected_tx_ids: vec![],
src/runtime/run_summary.rs:277:    fn extract_tx_id_handles_all_variants() {
src/runtime/run_summary.rs:286:        assert_eq!(extract_tx_id(&to).0, "taskopen-t-eo-extract");
src/runtime/run_summary.rs:287:        assert_eq!(extract_tx_id(&wo).0, "worktx-t-eo-extract");
src/runtime/chain_derived_run_facts.rs:172:    tx_id: TxId,
src/runtime/chain_derived_run_facts.rs:309:    let mut accepted_worktx_by_tx_id: BTreeMap<TxId, (Option<String>, Option<String>)> =
src/runtime/chain_derived_run_facts.rs:311:    let mut confirmed_worktx_ids: BTreeSet<TxId> = BTreeSet::new();
src/runtime/chain_derived_run_facts.rs:358:                        accepted_worktx_by_tx_id.insert(
src/runtime/chain_derived_run_facts.rs:359:                            work.tx_id.clone(),
src/runtime/chain_derived_run_facts.rs:365:                            .insert(work.tx_id.clone(), tel.verification_result_cid);
src/runtime/chain_derived_run_facts.rs:368:                            tx_id: work.tx_id.clone(),
src/runtime/chain_derived_run_facts.rs:377:                        accepted_worktx_by_tx_id.insert(work.tx_id.clone(), (None, None));
src/runtime/chain_derived_run_facts.rs:378:                        accepted_worktx_vr_cid.insert(work.tx_id.clone(), None);
src/runtime/chain_derived_run_facts.rs:381:                    accepted_worktx_by_tx_id.insert(work.tx_id.clone(), (None, None));
src/runtime/chain_derived_run_facts.rs:382:                    accepted_worktx_vr_cid.insert(work.tx_id.clone(), None);
src/runtime/chain_derived_run_facts.rs:387:                    confirmed_worktx_ids.insert(verify.target_work_tx.clone());
src/runtime/chain_derived_run_facts.rs:392:                            accepted_worktx_by_tx_id.get(&verify.target_work_tx).cloned()
src/runtime/chain_derived_run_facts.rs:443:                        tx_id: work.tx_id.clone(),
src/runtime/chain_derived_run_facts.rs:522:    use crate::runtime::adapter::{make_real_verifytx_signed_by, make_real_worktx_signed_by};
src/runtime/chain_derived_run_facts.rs:601:        let worktx = make_real_worktx_signed_by(
src/runtime/agent_audit_trail.rs:5://! `candidate_proof_cid`, `tx_id`, `predicate_results`, `accepted_or_rejected`,
src/runtime/agent_audit_trail.rs:11://! Storage shape (charter § 5.4 Q6 — CAS-only with tx_id back-link, no
src/runtime/agent_audit_trail.rs:18://!    records one row per record: `{tx_id, proposal_record_cid, logical_t}`.
src/runtime/agent_audit_trail.rs:71:/// (`proposal_cid` + `candidate_proof_cid` + `tx_id`) + how the system
src/runtime/agent_audit_trail.rs:105:    /// `WorkTx.tx_id` of the proposal as routed through `bus.submit_typed_tx`.
src/runtime/agent_audit_trail.rs:107:    /// record by querying the audit trail index for `tx_id`.
src/runtime/agent_audit_trail.rs:108:    pub tx_id: TxId,
src/runtime/agent_audit_trail.rs:132:    /// `tx_id`.
src/runtime/agent_audit_trail.rs:165:        h.update(self.tx_id.0.as_bytes());
src/runtime/agent_audit_trail.rs:166:        h.update((self.tx_id.0.len() as u64).to_be_bytes());
src/runtime/agent_audit_trail.rs:282:// ── JSONL index (L4 tx_id → record CID) ─────────────────────────────────────
src/runtime/agent_audit_trail.rs:289:    pub tx_id: TxId,
src/runtime/agent_audit_trail.rs:300:/// `tx_id` + `proposal_record_cid` + `logical_t` plus `prev_hash` /
src/runtime/agent_audit_trail.rs:305:/// - Walk the L4 chain (or scan rejections.jsonl for L4.E) to find a tx_id.
src/runtime/agent_audit_trail.rs:310:/// `AgentProposalRecord.tx_id` itself.
src/runtime/agent_audit_trail.rs:351:    /// TRACE_MATRIX FC3-N1: TB-6 Atom 5 — append a new row binding `tx_id` to
src/runtime/agent_audit_trail.rs:356:        tx_id: &TxId,
src/runtime/agent_audit_trail.rs:368:            tx_id: tx_id.clone(),
src/runtime/agent_audit_trail.rs:399:    /// TRACE_MATRIX FC3-N1: TB-6 Atom 5 — find a row by tx_id (linear scan;
src/runtime/agent_audit_trail.rs:402:    pub fn find_by_tx_id(&self, tx_id: &TxId) -> Option<&AgentAuditTrailIndexRow> {
src/runtime/agent_audit_trail.rs:403:        self.rows.iter().find(|r| &r.tx_id == tx_id)
src/runtime/agent_audit_trail.rs:415:    /// (`prev_hash[i] == hash[i-1]`). Tampering with the row's `tx_id` or
src/runtime/agent_audit_trail.rs:453:    task_open_tx_id: &TxId,
src/runtime/agent_audit_trail.rs:454:    bad_worktx_tx_id: &TxId,
src/runtime/agent_audit_trail.rs:488:        tx_id: task_open_tx_id.clone(),
src/runtime/agent_audit_trail.rs:500:        &accepted.tx_id,
src/runtime/agent_audit_trail.rs:524:        tx_id: bad_worktx_tx_id.clone(),
src/runtime/agent_audit_trail.rs:543:        &rejected.tx_id,
src/runtime/agent_audit_trail.rs:579:            tx_id: TxId("worktx-test-1".into()),
src/runtime/agent_audit_trail.rs:605:        assert_eq!(decoded.tx_id, r.tx_id);
src/runtime/agent_audit_trail.rs:616:        r2.tx_id = TxId("worktx-test-2".into());
src/runtime/agent_audit_trail.rs:645:            idx.append(&r.tx_id, &cid, TEST_LOGICAL_T, &r).unwrap();
src/runtime/agent_audit_trail.rs:652:        let row = idx2.find_by_tx_id(&r.tx_id).expect("found by tx_id");
src/runtime/agent_audit_trail.rs:663:            idx.append(&r.tx_id, &cid, TEST_LOGICAL_T, &r).unwrap();
src/runtime/agent_audit_trail.rs:665:            r2.tx_id = TxId("second".into());
src/runtime/agent_audit_trail.rs:666:            idx.append(&r2.tx_id, &cid, TEST_LOGICAL_T + 1, &r2).unwrap();

exec
/bin/bash -lc "nl -ba src/runtime/adapter.rs | sed -n '1,220p;220,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-6 Atom 2 — chaintape adapter helpers.
     2	//!
     3	//! Constructors + seeding helpers for routing Agent proposals / candidate
     4	//! proofs through the production `Sequencer` via `bus.submit_typed_tx`.
     5	//! Used by:
     6	//! - `tests/tb_6_runtime_chaintape_bootstrap.rs` T10+ (synthetic fixture proof
     7	//!   that L4 + L4.E entries appear on disk).
     8	//! - `experiments/minif2f_v4/src/bin/evaluator.rs` Atom 3 hook (when chaintape
     9	//!   mode is on, emit a `WorkTx` per evaluator decision).
    10	//!
    11	//! Per architect ruling 2026-05-01 § 3.6 Atom 2: "First version (do NOT
    12	//! rewrite evaluator at once). Adapter only: Agent proposal → WorkTx; Lean
    13	//! accept → accepted WorkTx path; Lean fail / predicate fail → rejected WorkTx
    14	//! / L4.E path. Minimum: 1 accepted + 1 rejected WorkTx."
    15	//!
    16	//! This module is `pub use`-d from `src/runtime/mod.rs` so callers reach it
    17	//! as `turingosv4::runtime::adapter::*`.
    18	
    19	use std::collections::{BTreeMap, BTreeSet};
    20	
    21	use crate::bottom_white::cas::schema::Cid;
    22	use crate::economy::money::{MicroCoin, StakeMicroCoin};
    23	use crate::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
    24	use crate::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
    25	use crate::state::typed_tx::{
    26	    AgentSignature, BoolWithProof, EscrowLockSigningPayload, EscrowLockTx, PredicateId,
    27	    PredicateResultsBundle, ReadKey, SafetyOrCreation, TaskOpenSigningPayload, TaskOpenTx, TypedTx,
    28	    VerifySigningPayload, VerifyTx, VerifyVerdict, WorkSigningPayload, WorkTx, WriteKey,
    29	};
    30	
    31	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — pre-seed initial QState with sponsor balances.
    32	///
    33	/// Mirrors `tests/tb_3_rsp1_formal_surface.rs::genesis_with_balances` in
    34	/// shape. Returns a `QState::genesis()` with `balances_t` pre-populated; callers
    35	/// pass this into `build_chaintape_sequencer_with_initial_q`.
    36	///
    37	/// **Test-fixture / Atom 3 smoke only**. Real production seeding goes through
    38	/// `on_init_tx` per WP § 14.1; this helper is the synthetic alternative.
    39	pub fn genesis_with_balances(pairs: &[(AgentId, MicroCoin)]) -> QState {
    40	    let mut q = QState::genesis();
    41	    for (agent, balance) in pairs {
    42	        q.economic_state_t
    43	            .balances_t
    44	            .0
    45	            .insert(agent.clone(), *balance);
    46	    }
    47	    q
    48	}
    49	
    50	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic TaskOpenTx constructor.
    51	pub fn make_synthetic_task_open(
    52	    task: &str,
    53	    sponsor: &str,
    54	    parent_state_root: Hash,
    55	    suffix: &str,
    56	) -> TypedTx {
    57	    TypedTx::TaskOpen(TaskOpenTx {
    58	        tx_id: TxId(format!("taskopen-{}-{}", task, suffix)),
    59	        task_id: TaskId(task.into()),
    60	        parent_state_root,
    61	        sponsor_agent: AgentId(sponsor.into()),
    62	        verifier_quorum: 1,
    63	        max_reuse_royalty_fraction_basis_points: 1000,
    64	        settlement_rule_hash: Hash::ZERO,
    65	        signature: AgentSignature::from_bytes([0u8; 64]),
    66	        timestamp_logical: 1,
    67	    })
    68	}
    69	
    70	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic EscrowLockTx constructor.
    71	pub fn make_synthetic_escrow_lock(
    72	    task: &str,
    73	    sponsor: &str,
    74	    amount_micro: i64,
    75	    parent_state_root: Hash,
    76	    suffix: &str,
    77	) -> TypedTx {
    78	    TypedTx::EscrowLock(EscrowLockTx {
    79	        tx_id: TxId(format!("escrowlock-{}-{}", task, suffix)),
    80	        task_id: TaskId(task.into()),
    81	        parent_state_root,
    82	        sponsor_agent: AgentId(sponsor.into()),
    83	        amount: MicroCoin::from_micro_units(amount_micro),
    84	        signature: AgentSignature::from_bytes([0u8; 64]),
    85	        timestamp_logical: 1,
    86	    })
    87	}
    88	
    89	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic WorkTx constructor.
    90	///
    91	/// `predicate_passes = true` exercises the accepted L4 path; `predicate_passes
    92	/// = false` triggers L4.E `PredicateFailed` (or `StakeInsufficient` if
    93	/// `stake_micro = 0`). For Atom 3 hooks, `predicate_passes` mirrors the
    94	/// evaluator's accept/reject decision per Lean check.
    95	pub fn make_synthetic_worktx(
    96	    task: &str,
    97	    agent: &str,
    98	    parent_state_root: Hash,
    99	    stake_micro: i64,
   100	    suffix: &str,
   101	    predicate_passes: bool,
   102	) -> TypedTx {
   103	    let mut acceptance = BTreeMap::new();
   104	    acceptance.insert(
   105	        PredicateId("acc1".into()),
   106	        BoolWithProof {
   107	            value: predicate_passes,
   108	            proof_cid: None,
   109	        },
   110	    );
   111	    TypedTx::Work(WorkTx {
   112	        tx_id: TxId(format!("worktx-{}-{}", task, suffix)),
   113	        task_id: TaskId(task.into()),
   114	        parent_state_root,
   115	        agent_id: AgentId(agent.into()),
   116	        read_set: [ReadKey("k.read".into())]
   117	            .into_iter()
   118	            .collect::<BTreeSet<_>>(),
   119	        write_set: [WriteKey("k.write".into())]
   120	            .into_iter()
   121	            .collect::<BTreeSet<_>>(),
   122	        proposal_cid: Default::default(),
   123	        predicate_results: PredicateResultsBundle {
   124	            acceptance,
   125	            settlement: BTreeMap::new(),
   126	            safety_class: SafetyOrCreation::Safety,
   127	        },
   128	        stake: StakeMicroCoin::from_micro_units(stake_micro),
   129	        signature: AgentSignature::from_bytes([0u8; 64]),
   130	        timestamp_logical: 1,
   131	    })
   132	}
   133	
   134	/// TRACE_MATRIX FC1-N14: TB-7 Atom 2 — real-signature WorkTx constructor.
   135	///
   136	/// Builds a `WorkTx` and signs it via the per-run `AgentKeypairRegistry`.
   137	/// Mirrors `make_synthetic_worktx` shape but:
   138	///
   139	/// 1. Takes `proposal_cid` as a real CAS reference (the
   140	///    `ProposalTelemetry` object written by Atom 1.5 `proposal_telemetry`).
   141	/// 2. Computes `WorkSigningPayload::canonical_digest()` and signs via
   142	///    `AgentKeypairRegistry::sign(agent_id, digest)` — a real Ed25519
   143	///    signature, not a zero placeholder.
   144	/// 3. The `AgentSignature` is verifiable post-replay against the
   145	///    on-disk `agent_pubkeys.json` manifest (Atom 4 verify_chaintape
   146	///    extension; Gate 4).
   147	///
   148	/// This is the AUTHORITATIVE per-LLM-proposal WorkTx for TB-7 Frame B
   149	/// closure (charter §4.0 + §8 Gate 1). Atom 2 evaluator hook calls this
   150	/// for every meaningful real LLM proposal in the append branch.
   151	#[allow(clippy::too_many_arguments)]
   152	pub fn make_real_worktx_signed_by(
   153	    keypairs: &mut AgentKeypairRegistry,
   154	    task: &str,
   155	    agent: &str,
   156	    parent_state_root: Hash,
   157	    stake_micro: i64,
   158	    suffix: &str,
   159	    proposal_cid: Cid,
   160	    predicate_passes: bool,
   161	    timestamp_logical: u64,
   162	) -> Result<TypedTx, AgentKeypairError> {
   163	    let mut acceptance = BTreeMap::new();
   164	    acceptance.insert(
   165	        PredicateId("acc1".into()),
   166	        BoolWithProof {
   167	            value: predicate_passes,
   168	            proof_cid: None,
   169	        },
   170	    );
   171	
   172	    let agent_id = AgentId(agent.into());
   173	    let task_id = TaskId(task.into());
   174	    let tx_id = TxId(format!("worktx-{}-{}", task, suffix));
   175	    let read_set: BTreeSet<ReadKey> = [ReadKey("k.read".into())].into_iter().collect();
   176	    let write_set: BTreeSet<WriteKey> = [WriteKey("k.write".into())].into_iter().collect();
   177	    let predicate_results = PredicateResultsBundle {
   178	        acceptance,
   179	        settlement: BTreeMap::new(),
   180	        safety_class: SafetyOrCreation::Safety,
   181	    };
   182	    let stake = StakeMicroCoin::from_micro_units(stake_micro);
   183	
   184	    // Build the SigningPayload (10 fields; signature excluded per typed_tx.rs §3).
   185	    let payload = WorkSigningPayload {
   186	        tx_id: tx_id.clone(),
   187	        task_id: task_id.clone(),
   188	        parent_state_root,
   189	        agent_id: agent_id.clone(),
   190	        read_set: read_set.clone(),
   191	        write_set: write_set.clone(),
   192	        proposal_cid,
   193	        predicate_results: predicate_results.clone(),
   194	        stake,
   195	        timestamp_logical,
   196	    };
   197	    let digest = payload.canonical_digest();
   198	    let signature = keypairs.sign(&agent_id, digest)?;
   199	
   200	    Ok(TypedTx::Work(WorkTx {
   201	        tx_id,
   202	        task_id,
   203	        parent_state_root,
   204	        agent_id,
   205	        read_set,
   206	        write_set,
   207	        proposal_cid,
   208	        predicate_results,
   209	        stake,
   210	        signature,
   211	        timestamp_logical,
   212	    }))
   213	}
   214	
   215	/// TRACE_MATRIX FC1-N14: TB-7 Atom 3 — real-signature VerifyTx constructor for
   216	/// OMEGA-branch routing.
   217	///
   218	/// Builds a `VerifyTx` paired with an accepted `WorkTx` for the OMEGA path
   219	/// (Lean oracle accepted the proof → verifier confirms via VerifyTx). Signs
   220	/// via the same `AgentKeypairRegistry` as the WorkTx side. Produces a
   220	/// via the same `AgentKeypairRegistry` as the WorkTx side. Produces a
   221	/// `VerifyVerdict::Confirm` when `verdict_confirms = true`.
   222	///
   223	/// **OMEGA scope NARROWED per ARCHITECT_RULING D3 + charter §4.3**: WorkTx
   224	/// + VerifyTx pair only; ChallengeWindow stays OPEN; NO FinalizeRewardTx,
   225	/// NO SlashTx, NO settlement. Settlement is RSP-4 / TB-9 territory.
   226	#[allow(clippy::too_many_arguments)]
   227	pub fn make_real_verifytx_signed_by(
   228	    keypairs: &mut AgentKeypairRegistry,
   229	    parent_state_root: Hash,
   230	    target_work_tx: TxId,
   231	    verifier_agent: &str,
   232	    bond_micro: i64,
   233	    suffix: &str,
   234	    verdict_confirms: bool,
   235	    timestamp_logical: u64,
   236	) -> Result<TypedTx, AgentKeypairError> {
   237	    let verifier_id = AgentId(verifier_agent.into());
   238	    let tx_id = TxId(format!("verifytx-{}-{}", verifier_agent, suffix));
   239	    let bond = StakeMicroCoin::from_micro_units(bond_micro);
   240	    let verdict = if verdict_confirms {
   241	        VerifyVerdict::Confirm
   242	    } else {
   243	        VerifyVerdict::Doubt
   244	    };
   245	
   246	    let payload = VerifySigningPayload {
   247	        tx_id: tx_id.clone(),
   248	        parent_state_root,
   249	        target_work_tx: target_work_tx.clone(),
   250	        verifier_agent: verifier_id.clone(),
   251	        bond,
   252	        verdict,
   253	        timestamp_logical,
   254	    };
   255	    let digest = payload.canonical_digest();
   256	    let signature = keypairs.sign(&verifier_id, digest)?;
   257	
   258	    Ok(TypedTx::Verify(VerifyTx {
   259	        tx_id,
   260	        parent_state_root,
   261	        target_work_tx,
   262	        verifier_agent: verifier_id,
   263	        bond,
   264	        verdict,
   265	        signature,
   266	        timestamp_logical,
   267	    }))
   268	}
   269	
   270	// ────────────────────────────────────────────────────────────────────────────
   271	// TB-10 Atom 1 — Real-signature constructors for user-driven TaskOpen + EscrowLock.
   272	//
   273	// The synthetic constructors above use `AgentSignature::from_bytes([0u8; 64])`
   274	// because the evaluator's preseed sponsor (`tb7-7-sponsor`) is not in the
   275	// durable keystore — its ledger entries pre-date TB-7's Ed25519 wiring.
   276	//
   277	// TB-10 introduces a NEW caller class (a human user invoking `lean_market`)
   278	// who DOES carry a durable Ed25519 keypair via TB-9's keystore (Agent_user_0).
   279	// User-driven TaskOpen + EscrowLock SHOULD carry real signatures so the chain
   280	// has cryptographic attestation of sponsor identity — even though the kernel
   281	// dispatch arms (sequencer.rs:1054 + 1095) do not currently verify these
   282	// fields. This is forward-compatible with future TB-12+ kernel hardening.
   283	//
   284	// Per TB-10 charter §2.1 + ratification §2.1.
   285	// ────────────────────────────────────────────────────────────────────────────
   286	
   287	/// TRACE_MATRIX FC1-N14: TB-10 Atom 1 — real-signature TaskOpenTx constructor.
   288	///
   289	/// Builds a `TaskOpenTx` and signs it via `AgentKeypairRegistry::sign(&sponsor, digest)`.
   290	/// Mirrors `make_synthetic_task_open` shape but produces a non-zero Ed25519 signature
   291	/// over `TaskOpenSigningPayload::canonical_digest()`.
   292	#[allow(clippy::too_many_arguments)]
   293	pub fn make_real_task_open_signed_by(
   294	    keypairs: &mut AgentKeypairRegistry,
   295	    task: &str,
   296	    sponsor: &str,
   297	    parent_state_root: Hash,
   298	    suffix: &str,
   299	    timestamp_logical: u64,
   300	) -> Result<TypedTx, AgentKeypairError> {
   301	    let sponsor_id = AgentId(sponsor.into());
   302	    let task_id = TaskId(task.into());
   303	    let tx_id = TxId(format!("taskopen-{}-{}", task, suffix));
   304	    let payload = TaskOpenSigningPayload {
   305	        tx_id: tx_id.clone(),
   306	        task_id: task_id.clone(),
   307	        parent_state_root,
   308	        sponsor_agent: sponsor_id.clone(),
   309	        verifier_quorum: 1,
   310	        max_reuse_royalty_fraction_basis_points: 1000,
   311	        settlement_rule_hash: Hash::ZERO,
   312	        timestamp_logical,
   313	    };
   314	    let digest = payload.canonical_digest();
   315	    let signature = keypairs.sign(&sponsor_id, digest)?;
   316	    Ok(TypedTx::TaskOpen(TaskOpenTx {
   317	        tx_id,
   318	        task_id,
   319	        parent_state_root,
   320	        sponsor_agent: sponsor_id,
   321	        verifier_quorum: 1,
   322	        max_reuse_royalty_fraction_basis_points: 1000,
   323	        settlement_rule_hash: Hash::ZERO,
   324	        signature,
   325	        timestamp_logical,
   326	    }))
   327	}
   328	
   329	/// TRACE_MATRIX FC1-N14: TB-10 Atom 1 — real-signature EscrowLockTx constructor.
   330	///
   331	/// Builds an `EscrowLockTx` and signs it via `AgentKeypairRegistry::sign(&sponsor, digest)`.
   332	/// Mirrors `make_synthetic_escrow_lock` shape but produces a non-zero Ed25519
   333	/// signature over `EscrowLockSigningPayload::canonical_digest()`.
   334	#[allow(clippy::too_many_arguments)]
   335	pub fn make_real_escrow_lock_signed_by(
   336	    keypairs: &mut AgentKeypairRegistry,
   337	    task: &str,
   338	    sponsor: &str,
   339	    amount_micro: i64,
   340	    parent_state_root: Hash,
   341	    suffix: &str,
   342	    timestamp_logical: u64,
   343	) -> Result<TypedTx, AgentKeypairError> {
   344	    let sponsor_id = AgentId(sponsor.into());
   345	    let task_id = TaskId(task.into());
   346	    let tx_id = TxId(format!("escrowlock-{}-{}", task, suffix));
   347	    let amount = MicroCoin::from_micro_units(amount_micro);
   348	    let payload = EscrowLockSigningPayload {
   349	        tx_id: tx_id.clone(),
   350	        task_id: task_id.clone(),
   351	        parent_state_root,
   352	        sponsor_agent: sponsor_id.clone(),
   353	        amount,
   354	        timestamp_logical,
   355	    };
   356	    let digest = payload.canonical_digest();
   357	    let signature = keypairs.sign(&sponsor_id, digest)?;
   358	    Ok(TypedTx::EscrowLock(EscrowLockTx {
   359	        tx_id,
   360	        task_id,

exec
/bin/bash -lc "nl -ba src/ledger.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 0: Append-only tape with tamper detection
     2	// Constitutional basis: Law 1 (Information is Free), Magna Carta
     3	// V3 lessons: V3L-09 (no silent failure), V3L-24 (no /tmp data loss)
     4	
     5	use serde::{Deserialize, Serialize};
     6	use sha2::{Digest, Sha256};
     7	use std::collections::HashMap;
     8	use std::fmt;
     9	
    10	// ── Core types ──────────────────────────────────────────────────
    11	
    12	/// Unique identifier for a tape node.
    13	pub type NodeId = String;
    14	
    15	/// A single node on the append-only tape (DAG).
    16	/// Constitutional basis: Art. I — all signals quantized through this structure.
    17	#[derive(Debug, Clone, Serialize, Deserialize)]
    18	pub struct Node {
    19	    pub id: NodeId,
    20	    pub author: String,
    21	    pub payload: String,
    22	    pub citations: Vec<NodeId>,
    23	    pub created_at: u64,
    24	    pub completion_tokens: u32,
    25	}
    26	
    27	/// The append-only DAG tape.
    28	/// Invariant: once appended, a node is NEVER modified or removed.
    29	/// V3L-24: all data persisted to experiments/, never /tmp.
    30	#[derive(Debug, Clone, Serialize, Deserialize)]
    31	pub struct Tape {
    32	    nodes: HashMap<NodeId, Node>,
    33	    reverse_citations: HashMap<NodeId, Vec<NodeId>>,
    34	    time_arrow: Vec<NodeId>,
    35	}
    36	
    37	impl Tape {
    38	    pub fn new() -> Self {
    39	        Tape {
    40	            nodes: HashMap::new(),
    41	            reverse_citations: HashMap::new(),
    42	            time_arrow: Vec::new(),
    43	        }
    44	    }
    45	
    46	    /// Append a node to the tape.
    47	    /// Returns Err if:
    48	    /// - Node ID already exists (V6 spacetime paradox protection)
    49	    /// - Any cited parent does not exist (V5 causality defense)
    50	    /// V3L-09: never silently fail — always return explicit Result.
    51	    pub fn append(&mut self, node: Node) -> Result<(), TapeError> {
    52	        // V6: reject duplicate IDs
    53	        if self.nodes.contains_key(&node.id) {
    54	            return Err(TapeError::DuplicateId(node.id.clone()));
    55	        }
    56	
    57	        // V5: reject citations to non-existent parents
    58	        for parent_id in &node.citations {
    59	            if !self.nodes.contains_key(parent_id) {
    60	                return Err(TapeError::DanglingCitation {
    61	                    node_id: node.id.clone(),
    62	                    missing_parent: parent_id.clone(),
    63	                });
    64	            }
    65	        }
    66	
    67	        // Update reverse citations
    68	        for parent_id in &node.citations {
    69	            self.reverse_citations
    70	                .entry(parent_id.clone())
    71	                .or_default()
    72	                .push(node.id.clone());
    73	        }
    74	
    75	        // Append to time arrow
    76	        self.time_arrow.push(node.id.clone());
    77	
    78	        // Insert node
    79	        self.nodes.insert(node.id.clone(), node);
    80	
    81	        Ok(())
    82	    }
    83	
    84	    pub fn get(&self, id: &str) -> Option<&Node> {
    85	        self.nodes.get(id)
    86	    }
    87	
    88	    pub fn children(&self, id: &str) -> &[NodeId] {
    89	        self.reverse_citations
    90	            .get(id)
    91	            .map(|v| v.as_slice())
    92	            .unwrap_or(&[])
    93	    }
    94	
    95	    pub fn len(&self) -> usize {
    96	        self.nodes.len()
    97	    }
    98	
    99	    pub fn is_empty(&self) -> bool {
   100	        self.nodes.is_empty()
   101	    }
   102	
   103	    pub fn time_arrow(&self) -> &[NodeId] {
   104	        &self.time_arrow
   105	    }
   106	
   107	    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
   108	        &self.nodes
   109	    }
   110	
   111	    /// Trace the PRIMARY ancestor chain from a node back to root.
   112	    /// Follows only the first citation (primary parent) at each step.
   113	    /// This is by design: in a proof DAG, the primary chain is the proof path.
   114	    /// Multi-parent merges are represented but not followed by this function.
   115	    pub fn trace_ancestors(&self, node_id: &str) -> Vec<NodeId> {
   116	        let mut path = Vec::new();
   117	        let mut current = node_id.to_string();
   118	        let mut visited = std::collections::HashSet::new();
   119	
   120	        while let Some(node) = self.nodes.get(&current) {
   121	            if !visited.insert(current.clone()) {
   122	                break; // cycle protection (should never happen in a DAG)
   123	            }
   124	            path.push(current.clone());
   125	            // Follow first citation (primary parent in proof chain)
   126	            if let Some(parent) = node.citations.first() {
   127	                current = parent.clone();
   128	            } else {
   129	                break; // root node
   130	            }
   131	        }
   132	
   133	        path.reverse();
   134	        path
   135	    }
   136	}
   137	
   138	impl Default for Tape {
   139	    fn default() -> Self {
   140	        Self::new()
   141	    }
   142	}
   143	
   144	// ── Ledger event log ────────────────────────────────────────────
   145	
   146	/// Event types for the append-only event ledger.
   147	/// V3L-09: explicit vocabulary — only OmegaAccepted is a true OMEGA event.
   148	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
   149	pub enum EventType {
   150	    RunStart,
   151	    Append,
   152	    Invest,
   153	    MarketCreate,
   154	    MarketResolve,
   155	    OmegaInvoke,
   156	    OmegaAccepted,
   157	    OmegaRejected,
   158	    OmegaError,
   159	    RunEnd,
   160	}
   161	
   162	impl fmt::Display for EventType {
   163	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   164	        match self {
   165	            EventType::RunStart => write!(f, "RunStart"),
   166	            EventType::Append => write!(f, "Append"),
   167	            EventType::Invest => write!(f, "Invest"),
   168	            EventType::MarketCreate => write!(f, "MarketCreate"),
   169	            EventType::MarketResolve => write!(f, "MarketResolve"),
   170	            EventType::OmegaInvoke => write!(f, "OmegaInvoke"),
   171	            EventType::OmegaAccepted => write!(f, "OmegaAccepted"),
   172	            EventType::OmegaRejected => write!(f, "OmegaRejected"),
   173	            EventType::OmegaError => write!(f, "OmegaError"),
   174	            EventType::RunEnd => write!(f, "RunEnd"),
   175	        }
   176	    }
   177	}
   178	
   179	/// A single ledger event with hash-chain tamper detection.
   180	#[derive(Debug, Clone, Serialize, Deserialize)]

codex
I found a production-path concern that is not visible in the pure unit tests: `price_index` is keyed by canonical `WorkTx` ids, while `bus.snapshot()` gives `compute_mask_set` the legacy/shadow `kernel.tape`. I’m checking whether any bridge maps those two id spaces before deciding severity.
exec
/bin/bash -lc "rg -n \"parent_tx|worktx-|tx_[0-9]+_by|kernel\\.tape|compute_mask_set\\(|price_index|NodePosition\" src tests experiments/minif2f_v4/src/bin/evaluator.rs -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/evaluator.rs:786:    // TB-7.7 D2: last submitted tx per agent (for ProposalTelemetry.parent_tx).
experiments/minif2f_v4/src/bin/evaluator.rs:788:    // Verify). Root proposals leave parent_tx = None; subsequent same-agent
experiments/minif2f_v4/src/bin/evaluator.rs:1000:            "worktx-{}-atom3-l4e-synthetic-rejection",
experiments/minif2f_v4/src/bin/evaluator.rs:1171:    // boundary: `boltzmann_select_parent_v2(price_index, mask_set, &policy,
experiments/minif2f_v4/src/bin/evaluator.rs:1327:            let tape_len = bus.kernel.tape.time_arrow().len();
experiments/minif2f_v4/src/bin/evaluator.rs:1329:            // surface derived from `bus.snapshot().price_index` (integer-
experiments/minif2f_v4/src/bin/evaluator.rs:1335:            let market_count = tick_snap.price_index.len();
experiments/minif2f_v4/src/bin/evaluator.rs:1337:                tick_snap.price_index.iter()
experiments/minif2f_v4/src/bin/evaluator.rs:1374:                for nid in bus.kernel.tape.time_arrow() {
experiments/minif2f_v4/src/bin/evaluator.rs:1375:                    if let Some(n) = bus.kernel.tape.get(nid) {
experiments/minif2f_v4/src/bin/evaluator.rs:1509:        // ticker string from `snap.price_index` (integer-rational
experiments/minif2f_v4/src/bin/evaluator.rs:1517:                snap.price_index.iter()
experiments/minif2f_v4/src/bin/evaluator.rs:1544:        // TB-14 module surface (the G-14.11 fence targets `price_index.rs`
experiments/minif2f_v4/src/bin/evaluator.rs:1606:                                // `&snap.price_index` (BTreeMap<TxId,
experiments/minif2f_v4/src/bin/evaluator.rs:1613:                                    &snap.price_index, &snap.mask_set,
experiments/minif2f_v4/src/bin/evaluator.rs:1626:                                // BELOW is shadow_only (kernel.tape view sync for the next
experiments/minif2f_v4/src/bin/evaluator.rs:1633:                                // shadow_only kernel.tape sync MUST NOT be the only
experiments/minif2f_v4/src/bin/evaluator.rs:1660:                                    // TB-7.7 D2: parent_tx from last submission per agent (root if first).
experiments/minif2f_v4/src/bin/evaluator.rs:1661:                                    let parent_tx: Option<turingosv4::state::q_state::TxId> =
experiments/minif2f_v4/src/bin/evaluator.rs:1678:                                        parent_tx,
experiments/minif2f_v4/src/bin/evaluator.rs:1745:                                // shadow_only: kernel.tape view sync for next-agent prompt
experiments/minif2f_v4/src/bin/evaluator.rs:1752:                                // kernel.tape becoming L4-derived (post-MVP refactor).
experiments/minif2f_v4/src/bin/evaluator.rs:1791:                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
experiments/minif2f_v4/src/bin/evaluator.rs:1792:                                    .filter_map(|id| bus.kernel.tape.get(id))
experiments/minif2f_v4/src/bin/evaluator.rs:1796:                                let tape_len = bus.kernel.tape.time_arrow().len();
experiments/minif2f_v4/src/bin/evaluator.rs:1890:                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
experiments/minif2f_v4/src/bin/evaluator.rs:1924:                                            // TB-7.7 D2: parent_tx for branch lineage.
experiments/minif2f_v4/src/bin/evaluator.rs:1925:                                            let parent_tx_for_pt: Option<turingosv4::state::q_state::TxId> =
experiments/minif2f_v4/src/bin/evaluator.rs:1941:                                                parent_tx_for_pt,
experiments/minif2f_v4/src/bin/evaluator.rs:1951:                                            // `worktx-<task>-<suffix>` per make_real_worktx_signed_by.
experiments/minif2f_v4/src/bin/evaluator.rs:1955:                                                    "worktx-{}-{}",
experiments/minif2f_v4/src/bin/evaluator.rs:2103:                                        // shadow_only: kernel.tape view sync for halt-and-settle +
experiments/minif2f_v4/src/bin/evaluator.rs:2121:                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
experiments/minif2f_v4/src/bin/evaluator.rs:2122:                                            .filter_map(|id| bus.kernel.tape.get(id))
experiments/minif2f_v4/src/bin/evaluator.rs:2128:                                        let gp = bus.kernel.tape.time_arrow().to_vec();
experiments/minif2f_v4/src/bin/evaluator.rs:2274:                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
experiments/minif2f_v4/src/bin/evaluator.rs:2275:                                    .filter_map(|id| bus.kernel.tape.get(id))
experiments/minif2f_v4/src/bin/evaluator.rs:2317:                                              bus.kernel.tape.time_arrow().len() + 1);
experiments/minif2f_v4/src/bin/evaluator.rs:2322:                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
experiments/minif2f_v4/src/bin/evaluator.rs:2356:                                            // TB-7.7 D2: parent_tx for branch lineage.
experiments/minif2f_v4/src/bin/evaluator.rs:2357:                                            let parent_tx_for_pt: Option<turingosv4::state::q_state::TxId> =
experiments/minif2f_v4/src/bin/evaluator.rs:2373:                                                parent_tx_for_pt,
experiments/minif2f_v4/src/bin/evaluator.rs:2385:                                                    "worktx-{}-{}",
experiments/minif2f_v4/src/bin/evaluator.rs:2530:                                        // shadow_only: kernel.tape view sync; L4 chain above is
experiments/minif2f_v4/src/bin/evaluator.rs:2535:                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
experiments/minif2f_v4/src/bin/evaluator.rs:2536:                                            .filter_map(|id| bus.kernel.tape.get(id))
experiments/minif2f_v4/src/bin/evaluator.rs:2540:                                        let gp = bus.kernel.tape.time_arrow().to_vec();
experiments/minif2f_v4/src/bin/evaluator.rs:2583:                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
experiments/minif2f_v4/src/bin/evaluator.rs:2587:                                        // line ~1283 (Atom 2). This call writes only to kernel.tape
experiments/minif2f_v4/src/bin/evaluator.rs:2589:                                        // (3) + §6 #31; will be removed when kernel.tape is
experiments/minif2f_v4/src/bin/evaluator.rs:2601:                                                      bus.kernel.tape.time_arrow().len());
tests/six_axioms_alignment.rs:52:/// +node_positions_t (flat NodePositionsIndex; canonical exposure record
tests/six_axioms_alignment.rs:58:/// legacy `price_index_t` stub; TB-14 provides `compute_price_index`
tests/tb_14_mask_set.rs:18:use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
tests/tb_14_mask_set.rs:20:    compute_price_index, BoltzmannMaskPolicy, EconomicState, RationalPrice, TaskId, TxId,
tests/tb_14_mask_set.rs:22:use turingosv4::state::price_index::compute_mask_set;
tests/tb_14_mask_set.rs:32:) -> NodePosition {
tests/tb_14_mask_set.rs:33:    NodePosition {
tests/tb_14_mask_set.rs:132:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:133:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:174:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:175:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:200:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:201:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:216:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:217:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:230:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:231:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:242:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:243:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:258:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:259:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:274:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:275:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:288:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:289:    let first = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:292:            compute_mask_set(&econ, &tape, &policy, &price_index),
tests/tb_14_mask_set.rs:305:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:306:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_mask_set.rs:324:    let price_index = compute_price_index(&econ);
tests/tb_14_mask_set.rs:325:    let mask = compute_mask_set(&econ, &tape, &strict_policy, &price_index);
tests/wal_resume.rs:53:        assert_eq!(bus.kernel.tape.time_arrow().len(), 5);
tests/wal_resume.rs:61:        let arrow = bus2.kernel.tape.time_arrow();
tests/wal_resume.rs:64:            let node = bus2.kernel.tape.get(nid).expect("node present");
tests/wal_resume.rs:83:    assert_eq!(bus.kernel.tape.time_arrow().len(), 0);
tests/tb_8_minimal_payout.rs:129:        tx_id: TxId(format!("worktx-{task}-{suffix}")),
tests/tb_13_legacy_cpmm_forward_fence.rs:294:/// `src/state/price_index.rs` derives the price view from TB-13
tests/tb_13_legacy_cpmm_forward_fence.rs:715:/// type-use heuristic flagged TB-14 files (e.g. `price_index.rs`) that
tests/tb_14_chaintape_smoke.rs:9://! `state::compute_price_index` + `state::compute_mask_set` derived
tests/tb_14_chaintape_smoke.rs:31://! 3. **TB-14 NEW**: `compute_price_index(&live_q.economic_state_t)`
tests/tb_14_chaintape_smoke.rs:32://!    is byte-equal to `compute_price_index(&replayed_q.economic_state_t)`.
tests/tb_14_chaintape_smoke.rs:37://! 4. **TB-14 NEW**: `compute_price_index` is idempotent across N
tests/tb_14_chaintape_smoke.rs:68:use turingosv4::state::compute_price_index;
tests/tb_14_chaintape_smoke.rs:169:async fn tb_14_atom_6_post_wire_swap_chaintape_replay_preserves_price_index_determinism() {
tests/tb_14_chaintape_smoke.rs:304:    // FR-14.x / FC3-N42: compute_price_index over byte-equal-replayed
tests/tb_14_chaintape_smoke.rs:313:    let live_price_index = compute_price_index(&live_q.economic_state_t);
tests/tb_14_chaintape_smoke.rs:314:    let replayed_price_index = compute_price_index(&replayed_q.economic_state_t);
tests/tb_14_chaintape_smoke.rs:316:        live_price_index, replayed_price_index,
tests/tb_14_chaintape_smoke.rs:317:        "TB-14 FC3-N42: compute_price_index must be byte-equal across live vs \
tests/tb_14_chaintape_smoke.rs:321:    // ── Atom 6 invariant 3: compute_price_index idempotent ─────────────
tests/tb_14_chaintape_smoke.rs:323:    // Calling compute_price_index N times on the same EconomicState must
tests/tb_14_chaintape_smoke.rs:327:            compute_price_index(&live_q.economic_state_t),
tests/tb_14_chaintape_smoke.rs:328:            live_price_index,
tests/tb_14_chaintape_smoke.rs:329:            "TB-14 FC3-N42: compute_price_index must be idempotent"
tests/tb_14_chaintape_smoke.rs:345:        live_price_index.is_empty(),
tests/tb_14_chaintape_smoke.rs:388:                 **Source**: `tests/tb_14_chaintape_smoke.rs::tb_14_atom_6_post_wire_swap_chaintape_replay_preserves_price_index_determinism`\n\
tests/tb_14_chaintape_smoke.rs:389:                 **Trigger**: TB-14 Atom 6 production wire-swap (excise legacy CPMM scaffolding; reroute bus snapshot price-signal surface through `compute_price_index` + `compute_mask_set` integer-rational derived views).\n\
tests/tb_14_chaintape_smoke.rs:399:                 - `compute_price_index(live)` == `compute_price_index(replayed)`: byte-equal\n\
tests/tb_14_chaintape_smoke.rs:400:                 - `compute_price_index` idempotent across 5 invocations: ✓\n\
tests/tb_14_chaintape_smoke.rs:405:                 1. The Atom 6 production wire-swap (excised `prediction_market.rs`, `kernel.markets`, `BoltzmannParams`, legacy f64 `boltzmann_select_parent`; rewired `bus.snapshot` to derive `price_index` + `mask_set` from `Sequencer::q_snapshot`'s `EconomicState`) does NOT regress chain-replay determinism.\n\
tests/tb_14_chaintape_smoke.rs:407:                 3. The TB-14 derived view (`compute_price_index(econ)`) is replay-deterministic by composition: pure function over a byte-equal-replayed `EconomicState` yields byte-equal `BTreeMap<TxId, NodeMarketEntry>` (FR-14.x / FC3-N42 chaintape integration evidence).\n\
tests/tb_14_chaintape_smoke.rs:408:                 4. `compute_price_index` is idempotent across N calls (Art.0.2 pure-function determinism at the derived-view layer).\n\
tests/tb_14_chaintape_smoke.rs:413:                 - **Non-empty PriceIndex via WorkTx**: this smoke uses CompleteSet flow only (TB-13 substrate). A WorkTx-creates-NodePosition flow (TB-12 substrate that produces non-empty PriceIndex) is covered by the in-memory unit tests at `tests/tb_14_price_index.rs` + halt-triggers + `src/state/price_index.rs` inline tests. Per `feedback_chaintape_externalized_proposal`, the chaintape smoke records what the system externalizes via `submit_typed_tx` end-to-end; the per-position aggregation is pure-function-tested elsewhere.\n\
tests/tb_2_runtime_boundary.rs:92:        tx_id: TxId(format!("worktx-tb2-{}", opts.tx_id_suffix)),
tests/tb_4_rsp2_admission_surface.rs:142:        tx_id: TxId(format!("worktx-{task}-{suffix}")),
tests/tb_3_rsp1_formal_surface.rs:149:        tx_id: TxId(format!("worktx-{task}-{suffix}")),
tests/tb_3_rsp1_formal_surface.rs:370:        .get(&TxId("worktx-task-i23-i23".into())).expect("stake by work_tx_id");
tests/tb_7r_parent_tx_conformance.rs:1://! TB-7R parent_tx conformance tests — architect-mandated 2026-05-02.
tests/tb_7r_parent_tx_conformance.rs:3://! Per architect verdict 2026-05-02 (parent_tx ParentTx/DAG/Smoke ruling),
tests/tb_7r_parent_tx_conformance.rs:5://! parent_tx plumbing. Natural smoke under verdict A1=B′ + complete-tool
tests/tb_7r_parent_tx_conformance.rs:6://! one-shot solve has parent_tx_edges=0 with
tests/tb_7r_parent_tx_conformance.rs:126:    parent_tx: Option<TxId>,
tests/tb_7r_parent_tx_conformance.rs:146:        parent_tx,
tests/tb_7r_parent_tx_conformance.rs:151:    let expected_work_tx_id = TxId(format!("worktx-{task_id}-{suffix}"));
tests/tb_7r_parent_tx_conformance.rs:194:/// parent_tx=None, ProposalTelemetry has verification_result_cid pointing
tests/tb_7r_parent_tx_conformance.rs:197:///  - parent_tx_state == SingletonGoldenPathValid
tests/tb_7r_parent_tx_conformance.rs:239:        facts.parent_tx_state,
tests/tb_7r_parent_tx_conformance.rs:241:        "singleton solved run must label parent_tx_state=SingletonGoldenPathValid; got {:?}",
tests/tb_7r_parent_tx_conformance.rs:242:        facts.parent_tx_state
tests/tb_7r_parent_tx_conformance.rs:246:/// Test 2 — `second_attempt_same_branch_has_parent_tx`
tests/tb_7r_parent_tx_conformance.rs:249:/// First with parent_tx=None (root). Second with parent_tx=Some(first.tx_id).
tests/tb_7r_parent_tx_conformance.rs:250:/// Verify dashboard reconstructs the edge — parent_tx_state == MultiAttemptDagValid.
tests/tb_7r_parent_tx_conformance.rs:252:async fn second_attempt_same_branch_has_parent_tx() {
tests/tb_7r_parent_tx_conformance.rs:289:        Some(attempt_1_id.clone()), // parent_tx wired
tests/tb_7r_parent_tx_conformance.rs:301:        facts.parent_tx_state,
tests/tb_7r_parent_tx_conformance.rs:303:        "two attempts on same (agent,branch) with parent_tx wired must label MultiAttemptDagValid; got {:?}",
tests/tb_7r_parent_tx_conformance.rs:304:        facts.parent_tx_state
tests/tb_7r_parent_tx_conformance.rs:316:/// First with parent_tx=None (root). Second WITH parent_tx=None (BUG —
tests/tb_7r_parent_tx_conformance.rs:317:/// non-root attempt missing parent_tx). Verify dashboard flags as violation.
tests/tb_7r_parent_tx_conformance.rs:356:        None, // BUG: non-root attempt missing parent_tx
tests/tb_7r_parent_tx_conformance.rs:368:        facts.parent_tx_state,
tests/tb_7r_parent_tx_conformance.rs:370:        "two attempts on same branch with attempt_2.parent_tx=None must label MissingParentTxViolation; got {:?}",
tests/tb_7r_parent_tx_conformance.rs:371:        facts.parent_tx_state
tests/tb_7r_parent_tx_conformance.rs:379:/// inspects the rendered §7 golden path and §6 parent_tx_state.
tests/tb_7r_parent_tx_conformance.rs:425:        stdout.contains("parent_tx_state: SingletonGoldenPathValid"),
tests/tb_7r_parent_tx_conformance.rs:447:///  - parent_tx_state = NoMultiAttemptObserved (zero externalized proposals)
tests/tb_7r_parent_tx_conformance.rs:474:        facts.parent_tx_state,
tests/tb_7r_parent_tx_conformance.rs:477:        facts.parent_tx_state
tests/tb_7r_parent_tx_conformance.rs:551:    // 3 attempts on same (agent, branch) with parent_tx wired → MultiAttemptDagValid.
tests/tb_7r_parent_tx_conformance.rs:553:        facts.parent_tx_state,
tests/tb_7r_parent_tx_conformance.rs:555:        "3-deep DAG with parent_tx wired must label MultiAttemptDagValid; got {:?}",
tests/tb_7r_parent_tx_conformance.rs:556:        facts.parent_tx_state
tests/tb_6_agent_audit_trail.rs:103:    let record = record_for("worktx-task-i91-i91-1", AcceptedOrRejected::Rejected);
tests/tb_6_agent_audit_trail.rs:117:    assert_eq!(recovered.tx_id, TxId("worktx-task-i91-i91-1".into()));
tests/tb_6_agent_audit_trail.rs:141:    let r1 = record_for("worktx-i91b-1", AcceptedOrRejected::Accepted);
tests/tb_6_agent_audit_trail.rs:142:    let r2 = record_for("worktx-i91b-2", AcceptedOrRejected::Rejected);
tests/tb_6_agent_audit_trail.rs:170:    let r = record_for("worktx-i91d", AcceptedOrRejected::Accepted);
tests/tb_6_agent_audit_trail.rs:199:    let r = record_for("worktx-i91e", AcceptedOrRejected::Accepted);
tests/economic_state_reconstruct.rs:19:    // TB-14 Atom 2 (2026-05-03; architect §5.1): 13 → 12 (-price_index_t;
tests/economic_state_reconstruct.rs:20:    // TB-14 derives the price view via `compute_price_index` pure fn,
tests/economic_state_reconstruct.rs:99:    // TB-14 Atom 2 (2026-05-03): legacy `price_index_t` field removed —
tests/economic_state_reconstruct.rs:100:    // TB-14 derives the price view via `compute_price_index` pure fn over
tests/economic_state_reconstruct.rs:135:    // The TB-14 derived view is `compute_price_index(econ)` returning a
tests/economic_state_reconstruct.rs:138:    // `src/state/price_index.rs`.
tests/tb_7_authoritative_routing.rs:265:    let work_tx_id = TxId("worktx-fake-target".into());
tests/tb_6_run_summary.rs:96:            .contains(&TxId("worktx-task-i92-i92-rej".into())),
tests/fc_alignment_conformance.rs:39:    // Witness: behavioral — bus.kernel.tape exists + has empty time-arrow
tests/fc_alignment_conformance.rs:42:        bus.kernel.tape.time_arrow().is_empty(),
tests/fc_alignment_conformance.rs:68:    // surface is `price_index` + `mask_set` — derived integer-rational
tests/fc_alignment_conformance.rs:77:        snap.price_index.is_empty(),
tests/fc_alignment_conformance.rs:78:        "FC1-N6 / FC3-N42: price_index empty when bus is sequencer-less"
tests/fc_alignment_conformance.rs:306:// TB-14 Atom 2 — FC3-N42 (compute_price_index) witness.
tests/fc_alignment_conformance.rs:307:// TRACE_MATRIX FC3-N42 maps to src/state/price_index.rs:compute_price_index
tests/fc_alignment_conformance.rs:313:fn fc3_n42_compute_price_index_pure_fn_witness() {
tests/fc_alignment_conformance.rs:316:    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
tests/fc_alignment_conformance.rs:318:        compute_price_index, EconomicState, RationalPrice, TaskId, TxId,
tests/fc_alignment_conformance.rs:325:        NodePosition {
tests/fc_alignment_conformance.rs:338:    let idx = compute_price_index(&econ);
tests/fc_alignment_conformance.rs:355:        compute_price_index(&econ),
tests/fc_alignment_conformance.rs:357:        "FC3-N42: compute_price_index must be replay-deterministic"
tests/fc_alignment_conformance.rs:365:// compute_mask_set in src/state/price_index.rs (architect §5.5 +
tests/fc_alignment_conformance.rs:375:    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
tests/fc_alignment_conformance.rs:377:        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
tests/fc_alignment_conformance.rs:412:    let mk_pos = |pid: &str, node: &str, side: PositionSide, kind: PositionKind, amt: i64| -> NodePosition {
tests/fc_alignment_conformance.rs:413:        NodePosition {
tests/fc_alignment_conformance.rs:434:    let price_index = compute_price_index(&econ);
tests/fc_alignment_conformance.rs:435:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/fc_alignment_conformance.rs:444:        compute_mask_set(&econ, &tape, &policy, &price_index),
tests/fc_alignment_conformance.rs:468:    let mut price_index: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
tests/fc_alignment_conformance.rs:469:    price_index.insert(
tests/fc_alignment_conformance.rs:479:    price_index.insert(
tests/fc_alignment_conformance.rs:497:        boltzmann_select_parent_v2(&price_index, &mask, &argmax_policy, &mut rng);
tests/fc_alignment_conformance.rs:509:        boltzmann_select_parent_v2(&price_index, &mask_high, &argmax_policy, &mut rng);
tests/fc_alignment_conformance.rs:522:                    &price_index,
tests/fc_alignment_conformance.rs:535:                    &price_index,
tests/tb_14_halt_triggers.rs:23:// compute_price_index / NodeMarketEntry / RationalPrice / mask_set,
tests/tb_14_halt_triggers.rs:60:// not L4, even when the node has a high price_yes in compute_price_index.
tests/tb_14_halt_triggers.rs:73:// (which legitimately reads compute_price_index for read-view
tests/tb_14_halt_triggers.rs:134:    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
tests/tb_14_halt_triggers.rs:136:        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
tests/tb_14_halt_triggers.rs:157:    ) -> NodePosition {
tests/tb_14_halt_triggers.rs:158:        NodePosition {
tests/tb_14_halt_triggers.rs:189:    let price_index = compute_price_index(&econ);
tests/tb_14_halt_triggers.rs:190:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/tb_14_halt_triggers.rs:214:// src/state/price_index.rs and the TB-14 spans of src/sdk/actor.rs
tests/tb_14_halt_triggers.rs:220:    // Plan v2 G1: this test reads `src/state/price_index.rs` at runtime via
tests/tb_14_halt_triggers.rs:224:    // `src/state/price_index.rs` to contain zero substrings of the
tests/tb_14_halt_triggers.rs:237:    let price_index_path = format!("{}/src/state/price_index.rs", manifest);
tests/tb_14_halt_triggers.rs:238:    let body = std::fs::read_to_string(&price_index_path)
tests/tb_14_halt_triggers.rs:239:        .unwrap_or_else(|e| panic!("read {}: {}", price_index_path, e));
tests/tb_14_halt_triggers.rs:243:            "TB-14 halt-trigger #4 violated: src/state/price_index.rs contains forbidden \
tests/tb_14_halt_triggers.rs:255:// compute_price_index over an EconomicState where a node_id has
tests/tb_14_halt_triggers.rs:264:    use turingosv4::state::{compute_price_index, EconomicState, TaskId, TxId};
tests/tb_14_halt_triggers.rs:266:    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
tests/tb_14_halt_triggers.rs:270:    let idx_a = compute_price_index(&econ_a);
tests/tb_14_halt_triggers.rs:281:        NodePosition {
tests/tb_14_halt_triggers.rs:293:    let idx_b = compute_price_index(&econ_b);
tests/tb_14_halt_triggers.rs:325:    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
tests/tb_14_halt_triggers.rs:327:        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
tests/tb_14_halt_triggers.rs:348:    ) -> NodePosition {
tests/tb_14_halt_triggers.rs:349:        NodePosition {
tests/tb_14_halt_triggers.rs:393:    let price_index = compute_price_index(&econ);
tests/tb_14_halt_triggers.rs:394:    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
tests/q_state_reconstruct.rs:91:    //   13 → 12 (-price_index_t legacy stub; TB-14 derives the price view via
tests/q_state_reconstruct.rs:92:    //   `compute_price_index` pure fn, not stored as canonical state — "price
src/economy/ledger.rs:396:            tx_id: TxId(format!("worktx-{}", suffix)),
src/economy/monetary_invariant.rs:142:/// - TB-14 `compute_price_index` derived view (signal-not-truth per
src/economy/monetary_invariant.rs:144:///   the sum; legacy `price_index_t` field removed in TB-14 Atom 2)
tests/tb_1_acceptance.rs:109:        tx_id: TxId(format!("worktx-{}", suffix)),
tests/tb_12_node_exposure_index.rs:4://! "Node exposure index" — NOT trading market. NodePosition is an
tests/tb_12_node_exposure_index.rs:45:    AgentId, EscrowEntry, Hash, NodePositionsIndex, QState, TaskId, TxId,
tests/tb_12_node_exposure_index.rs:222:    assert_eq!(positions.len(), 1, "exactly 1 NodePosition created");
tests/tb_12_node_exposure_index.rs:225:        .expect("FirstLong NodePosition keyed by work_tx_id");
tests/tb_12_node_exposure_index.rs:297:        "VerifyTx must NOT create a NodePosition (FR-12.3 + CR-12.8)"
tests/tb_12_node_exposure_index.rs:324:    // verifying the holding-sum equality; if NodePosition.amount were
tests/tb_12_node_exposure_index.rs:331:    .expect("CR-12.2: total_supply_micro MUST be invariant across NodePosition derivation");
tests/tb_12_node_exposure_index.rs:342:    // node_positions_t. NodePosition is derived from typed-tx fields with
tests/tb_12_node_exposure_index.rs:344:    async fn run(label: &str) -> NodePositionsIndex {
tests/tb_12_node_exposure_index.rs:366:    // tx_ids and therefore identical NodePositions.
tests/tb_12_node_exposure_index.rs:412:// Note: a "WorkTx with stake==0 → no NodePosition" negative test is
tests/tb_12_node_exposure_index.rs:415:// before the NodePosition derivation runs. The Atom 2 gate
tests/tb_12_node_exposure_index.rs:419:// in the NodePosition write site protects against future regressions.
tests/tb_12_node_exposure_index.rs:469:// ── Ensure NodePosition does not break existing CTF assertions ──────────────
tests/tb_12_node_exposure_index.rs:487:    // Both must satisfy the 5-holding sum invariant; NodePosition not in sum.
tests/tb_12_node_exposure_index.rs:489:        .expect("CTF preserved across NodePosition derivation");
tests/tb_12_node_exposure_index.rs:526:        "TB-12 canonical state: node_positions_t (flat NodePositionsIndex)"
src/bin/audit_dashboard.rs:23://! 6. Branch lineage (from ProposalTelemetry branch_id + parent_tx)
src/bin/audit_dashboard.rs:41:use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide, TypedTx};
src/bin/audit_dashboard.rs:45:    compute_price_index, NodeMarketEntry, TaskId, TxId,
src/bin/audit_dashboard.rs:130:    /// derived price-index view per `compute_price_index` over a synthetic
src/bin/audit_dashboard.rs:137:    price_index: BTreeMap<TxId, NodeMarketEntry>,
src/bin/audit_dashboard.rs:140:/// TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4) — per-NodePosition
src/bin/audit_dashboard.rs:291:    parent_tx: String,
src/bin/audit_dashboard.rs:436:                let mut parent_tx: Option<String> = None;
src/bin/audit_dashboard.rs:443:                        parent_tx = tel.parent_tx.as_ref().map(|t| t.0.clone());
src/bin/audit_dashboard.rs:470:                work_parent_by_tx_id.insert(work.tx_id.0.clone(), parent_tx.clone());
src/bin/audit_dashboard.rs:471:                if let (Some(parent), Some(branch)) = (parent_tx.as_ref(), branch_id.as_ref()) {
src/bin/audit_dashboard.rs:473:                        parent_tx: parent.clone(),
src/bin/audit_dashboard.rs:771:    // WorkTx, walk parent_tx links upward to root; output the path.
src/bin/audit_dashboard.rs:941:        price_index: price_index_from_exposures(&exposures_in_progress),
src/bin/audit_dashboard.rs:948:/// vec and call the canonical `state::compute_price_index` over it.
src/bin/audit_dashboard.rs:954:/// (WorkTx) and ChallengeShort (ChallengeTx) — exactly the inputs `compute_price_index`
src/bin/audit_dashboard.rs:955:/// needs. By going through `compute_price_index` rather than re-implementing
src/bin/audit_dashboard.rs:960:/// The `kind` field is irrelevant to `compute_price_index` (which reads only
src/bin/audit_dashboard.rs:966:fn price_index_from_exposures(
src/bin/audit_dashboard.rs:976:        let position = NodePosition {
src/bin/audit_dashboard.rs:991:    compute_price_index(&econ)
src/bin/audit_dashboard.rs:1154:    // §6 Branch lineage + parent_tx state (TB-7R 2026-05-02)
src/bin/audit_dashboard.rs:1155:    // Per architect verdict 2026-05-02 (parent_tx ParentTx/DAG/Smoke ruling),
src/bin/audit_dashboard.rs:1157:    //   - SingletonGoldenPathValid (B′ singleton solve; parent_tx=None correct)
src/bin/audit_dashboard.rs:1159:    //   - MultiAttemptDagValid (≥1 multi-attempt branch with all parent_tx populated)
src/bin/audit_dashboard.rs:1160:    //   - MissingParentTxViolation (≥1 multi-attempt branch with missing parent_tx)
src/bin/audit_dashboard.rs:1161:    s.push_str("§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)\n");
src/bin/audit_dashboard.rs:1163:    let pt_state_label = match r.run_facts.parent_tx_state {
src/bin/audit_dashboard.rs:1165:            "SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)",
src/bin/audit_dashboard.rs:1169:            "MultiAttemptDagValid ✓ (≥1 multi-attempt branch with all parent_tx edges present)",
src/bin/audit_dashboard.rs:1171:            "MissingParentTxViolation ✗ (≥1 multi-attempt branch with missing parent_tx — wiring broken)",
src/bin/audit_dashboard.rs:1173:    s.push_str(&format!("  parent_tx_state: {}\n", pt_state_label));
src/bin/audit_dashboard.rs:1175:        s.push_str("  edges: (none — see parent_tx_state above for interpretation)\n");
src/bin/audit_dashboard.rs:1181:                edge.branch_id, edge.parent_tx, edge.child_tx
src/bin/audit_dashboard.rs:1477:    s.push_str(&render_section_14(&r.price_index));
src/bin/audit_dashboard.rs:1493:fn render_section_14(price_index: &BTreeMap<TxId, NodeMarketEntry>) -> String {
src/bin/audit_dashboard.rs:1500:    s.push_str("    derived statistical broadcast over canonical NodePositionsIndex\n");
src/bin/audit_dashboard.rs:1507:    if price_index.is_empty() {
src/bin/audit_dashboard.rs:1525:    for (node_id, entry) in price_index.iter() {
src/bin/audit_dashboard.rs:1556:/// index, NOT trading market — NodePosition is IMMUTABLE EXPOSURE RECORD
src/bin/audit_dashboard.rs:1565:        s.push_str("  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)\n");
src/bin/audit_dashboard.rs:1567:        s.push_str("  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):\n");
src/bin/audit_dashboard.rs:1626:    s.push_str("    NodePosition is an IMMUTABLE EXPOSURE RECORD, NOT active position balance.\n");
src/bin/audit_dashboard.rs:1627:    s.push_str("    NodePosition.amount is NOT a Coin holding (CR-12.1) and is NOT counted in\n");
src/bin/audit_dashboard.rs:1629:    s.push_str("    NodeMarketEntry is TB-14 derived view; flat NodePositionsIndex is canonical.\n");
src/bin/audit_dashboard.rs:1742:    fn sg_14_6_dashboard_empty_price_index_renders_explicit_empty_state() {
src/bin/audit_dashboard.rs:1817:        assert!(s_empty.contains("(no NodePosition records"));
tests/tb_14_price_index.rs:2://! `compute_price_index`.
tests/tb_14_price_index.rs:12://! in `src/state/price_index.rs` (FR-14.1..3, determinism, rational equality,
tests/tb_14_price_index.rs:18:use turingosv4::state::typed_tx::{EventId, NodePosition, PositionKind, PositionSide};
tests/tb_14_price_index.rs:20:    compute_price_index, EconomicState, RationalPrice, TaskId, TxId,
tests/tb_14_price_index.rs:31:) -> NodePosition {
tests/tb_14_price_index.rs:32:    NodePosition {
tests/tb_14_price_index.rs:45:fn econ_with_positions(positions: Vec<NodePosition>) -> EconomicState {
tests/tb_14_price_index.rs:55:fn sg_14_1_price_index_computes_yes_no_probabilities() {
tests/tb_14_price_index.rs:78:    let idx = compute_price_index(&econ);
tests/tb_14_price_index.rs:127:    let idx = compute_price_index(&econ);
tests/tb_14_price_index.rs:148:    let idx = compute_price_index(&econ);
tests/tb_14_price_index.rs:155:/// Determinism: per Art.0.2, `compute_price_index` is replay-deterministic.
tests/tb_14_price_index.rs:158:fn compute_price_index_is_replay_deterministic() {
tests/tb_14_price_index.rs:189:    let first = compute_price_index(&econ);
tests/tb_14_price_index.rs:192:            compute_price_index(&econ),
tests/tb_14_price_index.rs:194:            "compute_price_index must be replay-deterministic (Art.0.2)"
tests/tb_14_price_index.rs:232:    let idx = compute_price_index(&econ);
src/state/q_state.rs:127:/// `compute_mask_set` in `src/state/price_index.rs`. `#[serde(default)]`
src/state/q_state.rs:179:    // TB-14 Atom 2 (2026-05-03): `price_index_t: PriceIndex` removed.
src/state/q_state.rs:180:    // The TB-14 derived view is `compute_price_index(econ)` in
src/state/q_state.rs:181:    // `src/state/price_index.rs`; not canonical state per architect §5.1.
src/state/q_state.rs:191:    /// — flat `BTreeMap<TxId, NodePosition>` index. **Canonical** TB-12 source
src/state/q_state.rs:193:    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
src/state/q_state.rs:206:    pub node_positions_t: NodePositionsIndex,
src/state/q_state.rs:526:// TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1): NodePositionsIndex —
src/state/q_state.rs:533:/// `position_id → NodePosition` index. Architect's §3 ruling chose this
src/state/q_state.rs:538:pub struct NodePositionsIndex(
src/state/q_state.rs:539:    pub BTreeMap<TxId, crate::state::typed_tx::NodePosition>,
src/state/q_state.rs:712:// MicroCoin>)` removed. The TB-14 derived view is `compute_price_index`
src/state/q_state.rs:713:// in `src/state/price_index.rs` (architect §5.1: "price is signal, not
src/state/q_state.rs:715:// `EconomicState.price_index_t` field also removed at architect §5.2.
src/state/q_state.rs:806:        // with +node_positions_t (flat NodePositionsIndex; canonical exposure
src/state/q_state.rs:814:        // with -price_index_t (legacy stub removed; TB-14 provides
src/state/q_state.rs:815:        // `compute_price_index` pure-fn derived view, not canonical state —
src/state/q_state.rs:824:            "EconomicState must have 12 sub-fields post-TB-14 (was 13 post-TB-13; -price_index_t legacy stub); got {}",
src/state/q_state.rs:831:        assert!(!obj.contains_key("price_index_t"), "TB-14 Atom 2: price_index_t MUST be removed");
src/state/q_state.rs:834:    /// TB-12 Atom 1 (architect §8 Atom 1): NodePositionsIndex empty default
src/state/q_state.rs:838:        let idx = NodePositionsIndex::default();
src/bottom_white/ledger/transition_ledger.rs:1214:            tx_id: QTxId("worktx-replay-fixture".into()),
src/state/mod.rs:19:/// derived-view price index. `compute_price_index(econ)` is the pure-fn
src/state/mod.rs:22:pub mod price_index;
src/state/mod.rs:27:    EscrowsIndex, Hash, NodeId, NodePositionsIndex, PerAgentState, QState,
src/state/mod.rs:35:pub use price_index::{
src/state/mod.rs:36:    compute_mask_set, compute_price_index, BoltzmannMaskPolicy, NodeMarketEntry,
src/state/mod.rs:43:    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, NodePosition, PositionKind,
src/state/price_index.rs:25:use crate::state::typed_tx::{EventId, NodePosition, PositionSide, ShareAmount};
src/state/price_index.rs:34:/// constructed by `compute_price_index` (architect FR-14.1 + FR-14.2). All
src/state/price_index.rs:51:    /// `false` on any zero denominator (`compute_price_index` never
src/state/price_index.rs:85:/// signal entry. **Derived view** populated by `compute_price_index`;
src/state/price_index.rs:91:/// - `task_id` — the `TaskId` (Q-derived from any underlying `NodePosition`)
src/state/price_index.rs:93:/// - `long_interest` / `short_interest` — sum of `NodePosition.amount` per side
src/state/price_index.rs:112:// compute_price_index — pure fn over EconomicState
src/state/price_index.rs:133:pub fn compute_price_index(econ: &EconomicState) -> BTreeMap<TxId, NodeMarketEntry> {
src/state/price_index.rs:134:    // Pass 1: group NodePositions by node_id; collect (task_id, long_micro, short_micro).
src/state/price_index.rs:333:// compute_mask_set — derive the parent-mask set from price_index + Tape
src/state/price_index.rs:349:/// for each `(parent_id, parent_entry)` in `price_index`:
src/state/price_index.rs:364:pub fn compute_mask_set(
src/state/price_index.rs:368:    price_index: &BTreeMap<TxId, NodeMarketEntry>,
src/state/price_index.rs:384:    for (parent_id, parent_entry) in price_index.iter() {
src/state/price_index.rs:397:            let child_entry = match price_index.get(&child_tx_id) {
src/state/price_index.rs:456:    ) -> NodePosition {
src/state/price_index.rs:457:        NodePosition {
src/state/price_index.rs:470:    fn econ_with_positions(positions: Vec<NodePosition>) -> EconomicState {
src/state/price_index.rs:481:        let idx = compute_price_index(&econ);
src/state/price_index.rs:497:        let idx = compute_price_index(&econ);
src/state/price_index.rs:532:        let idx = compute_price_index(&econ);
src/state/price_index.rs:572:        let idx = compute_price_index(&econ);
src/state/price_index.rs:614:        let idx = compute_price_index(&econ);
src/state/price_index.rs:657:        let first = compute_price_index(&econ);
src/state/price_index.rs:660:                compute_price_index(&econ),
src/state/price_index.rs:704:        let idx = compute_price_index(&econ);
src/state/sequencer.rs:622:            // NodePosition exposure record. Pure additive index write —
src/state/sequencer.rs:625:            // logic). NodePosition.amount is **NOT a Coin holding** per
src/state/sequencer.rs:632:                let position = crate::state::typed_tx::NodePosition {
src/state/sequencer.rs:874:            // NodePosition exposure record. Pure additive index write —
src/state/sequencer.rs:877:            // TB-4 economic logic). NodePosition.amount is **NOT a Coin
src/state/sequencer.rs:893:                let position = crate::state::typed_tx::NodePosition {
src/state/sequencer.rs:3201:            tx_id: TxId("worktx-seq-fixture".into()),
src/state/sequencer.rs:3471:            .get(&TxId("worktx-seq-fixture".into()))
src/state/sequencer.rs:3745:            tx_id: TxId(format!("worktx-{task}-{suffix}")),
src/state/sequencer.rs:3831:            .get(&TxId("worktx-task-u11-u11".into()))
src/state/typed_tx.rs:591:// § 5a-TB-12 — NodePosition exposure record (architect 2026-05-03 ruling)
src/state/typed_tx.rs:594:// insight: NodePosition is **immutable exposure record**, not active position
src/state/typed_tx.rs:659:/// - NOT a Coin holding (CR-12.1; NodePosition.amount is NOT in
src/state/typed_tx.rs:680:pub struct NodePosition {
src/state/typed_tx.rs:2090:            tx_id: TxId("worktx-fixture-01".into()),
src/state/typed_tx.rs:2114:            target_work_tx: TxId("worktx-fixture-01".into()),
src/state/typed_tx.rs:2127:            target_work_tx: TxId("worktx-fixture-01".into()),
src/state/typed_tx.rs:2139:            reusing_work_tx: TxId("worktx-fixture-02".into()),
src/sdk/actor.rs:34:/// 1. Build the candidate set: every `node_id` in `price_index` whose
src/sdk/actor.rs:54:/// **Determinism**: deterministic given the same `(price_index, mask_set,
src/sdk/actor.rs:58:    price_index: &std::collections::BTreeMap<
src/sdk/actor.rs:67:    let candidates: Vec<&crate::state::TxId> = price_index
src/sdk/actor.rs:94:        let entry = price_index.get(*cand).expect("candidate in index");
src/sdk/snapshot.rs:7:// The snapshot now carries integer-rational `price_index` + `mask_set`
src/sdk/snapshot.rs:8:// derived from canonical `EconomicState` via `state::compute_price_index`
src/sdk/snapshot.rs:32:/// - `price_index` — derived `BTreeMap<TxId, NodeMarketEntry>` per
src/sdk/snapshot.rs:33:///   `compute_price_index(econ)`. Empty when bus runs sequencer-less.
src/sdk/snapshot.rs:34:/// - `mask_set` — derived `BTreeSet<TxId>` per `compute_mask_set(...)`.
src/sdk/snapshot.rs:41:    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
src/sdk/snapshot.rs:54:        // price_index + mask_set; consumers (evaluator / dashboard) must
src/sdk/snapshot.rs:58:            price_index: BTreeMap::new(),
src/sdk/snapshot.rs:63:        assert!(snap.price_index.is_empty());
src/kernel.rs:14:// in the derived view `state::compute_price_index`; YES/NO claims live
src/runtime/run_summary.rs:244:            accepted_tx_ids: vec![TxId("worktx-1".into())],
src/runtime/run_summary.rs:245:            rejected_tx_ids: vec![TxId("worktx-2".into()), TxId("worktx-3".into())],
src/runtime/run_summary.rs:287:        assert_eq!(extract_tx_id(&wo).0, "worktx-t-eo-extract");
src/runtime/mod.rs:38:/// TRACE_MATRIX FC1-N14: TB-7 Atom 1.5 — ProposalTelemetry CAS object writer (per-WorkTx LLM proposal metadata: agent_id / prompt_context_hash / proposal_artifact_cid / candidate_tactic / token_counts / tool_calls / branch_id / parent_tx; per ARCHITECT_RULING D5 + charter §4.5).
src/runtime/adapter.rs:112:        tx_id: TxId(format!("worktx-{}-{}", task, suffix)),
src/runtime/adapter.rs:174:    let tx_id = TxId(format!("worktx-{}-{}", task, suffix));
src/runtime/proposal_telemetry.rs:23://!   "parent_tx": "<TxId or null>"
src/runtime/proposal_telemetry.rs:117:/// 8. `parent_tx` — `TxId` of the parent WorkTx if this proposal was
src/runtime/proposal_telemetry.rs:135:    pub parent_tx: Option<TxId>,
src/runtime/proposal_telemetry.rs:164:            parent_tx: None,
src/runtime/proposal_telemetry.rs:184:    /// `parent_tx` is `None` here for backward compat; callers that want
src/runtime/proposal_telemetry.rs:213:    /// `parent_tx` for branch lineage / DAG-edge reconstruction.
src/runtime/proposal_telemetry.rs:227:        parent_tx: Option<TxId>,
src/runtime/proposal_telemetry.rs:255:            parent_tx,
src/runtime/proposal_telemetry.rs:442:        assert!(obj.contains_key("parent_tx"));
src/runtime/proposal_telemetry.rs:522:    /// TB-7.7 D2 — build_for_evaluator_append_with_parent records parent_tx
src/runtime/proposal_telemetry.rs:523:    /// when supplied; default builder leaves parent_tx = None.
src/runtime/proposal_telemetry.rs:525:    fn build_with_parent_records_parent_tx() {
src/runtime/proposal_telemetry.rs:528:        let parent = TxId("worktx-task-r-p0".into());
src/runtime/proposal_telemetry.rs:535:        assert_eq!(pt.parent_tx, Some(parent));
src/runtime/proposal_telemetry.rs:543:        assert!(pt2.parent_tx.is_none());
src/runtime/verification_result.rs:180:            target_work_tx: TxId("worktx-test-1".into()),
src/runtime/verification_result.rs:219:            TxId("worktx-test".into()),
src/runtime/verification_result.rs:229:            TxId("worktx-test".into()),
src/runtime/chain_derived_run_facts.rs:67:/// per architect verdict 2026-05-02 (parent_tx ParentTx/DAG/Smoke ruling),
src/runtime/chain_derived_run_facts.rs:68:/// parent_tx is a **conditional invariant**, not an unconditional smoke
src/runtime/chain_derived_run_facts.rs:69:/// requirement. The dashboard's parent_tx state distinguishes the
src/runtime/chain_derived_run_facts.rs:80:    /// success path. parent_tx=None on the root attempt is correct;
src/runtime/chain_derived_run_facts.rs:86:    /// multi-Work runs. parent_tx wiring not exercised by this run;
src/runtime/chain_derived_run_facts.rs:90:    /// non-root attempt on each multi-attempt branch has parent_tx
src/runtime/chain_derived_run_facts.rs:97:    /// one non-root attempt has `parent_tx == None`. This is a real
src/runtime/chain_derived_run_facts.rs:98:    /// violation — parent_tx plumbing did not record the lineage edge.
src/runtime/chain_derived_run_facts.rs:157:    /// **TB-7R NEW (2026-05-02)**: parent_tx invariant state per architect
src/runtime/chain_derived_run_facts.rs:162:    pub parent_tx_state: ParentTxState,
src/runtime/chain_derived_run_facts.rs:167:/// of computing `parent_tx_state`. Synthetic-seed entries (zero
src/runtime/chain_derived_run_facts.rs:175:    parent_tx: Option<TxId>,
src/runtime/chain_derived_run_facts.rs:179:/// compute parent_tx state from the per-attempt accumulator.
src/runtime/chain_derived_run_facts.rs:185:/// - ≥1 multi-attempt branch AND any non-root attempt has parent_tx=None
src/runtime/chain_derived_run_facts.rs:187:/// - ≥1 multi-attempt branch AND every non-root attempt has parent_tx=Some(_)
src/runtime/chain_derived_run_facts.rs:189:fn compute_parent_tx_state(
src/runtime/chain_derived_run_facts.rs:214:        // Root attempt's parent_tx may legitimately point at a prior tx
src/runtime/chain_derived_run_facts.rs:222:            if attempt.parent_tx.is_none() {
src/runtime/chain_derived_run_facts.rs:318:    // TB-7R parent_tx ParentTxState accumulator: capture every
src/runtime/chain_derived_run_facts.rs:366:                        // TB-7R: capture for parent_tx DAG-state computation.
src/runtime/chain_derived_run_facts.rs:371:                            parent_tx: tel.parent_tx.clone(),
src/runtime/chain_derived_run_facts.rs:438:                    // an externalized proposal for parent_tx state. The
src/runtime/chain_derived_run_facts.rs:446:                        parent_tx: tel.parent_tx.clone(),
src/runtime/chain_derived_run_facts.rs:492:    // TB-7R: compute parent_tx state from accumulated WorkTxAttempt list.
src/runtime/chain_derived_run_facts.rs:493:    // Per architect verdict 2026-05-02, parent_tx is a conditional
src/runtime/chain_derived_run_facts.rs:495:    let parent_tx_state = compute_parent_tx_state(&worktx_attempts, chain_oracle_verified);
src/runtime/chain_derived_run_facts.rs:513:        parent_tx_state,
src/runtime/agent_audit_trail.rs:579:            tx_id: TxId("worktx-test-1".into()),
src/runtime/agent_audit_trail.rs:616:        r2.tx_id = TxId("worktx-test-2".into());
src/bus.rs:36:/// derived view over `EconomicState` via `state::compute_price_index`; no
src/bus.rs:206:    /// Capital signals now live entirely in `state::NodePositionsIndex`
src/bus.rs:207:    /// (TB-12) and surface via `compute_price_index` derived view (TB-14).
src/bus.rs:329:        // (`state::compute_price_index`) populated by typed-tx admission via
src/bus.rs:482:    /// `price_index` + `mask_set` derived from canonical `EconomicState`
src/bus.rs:483:    /// via `state::compute_price_index` + `state::compute_mask_set`,
src/bus.rs:493:    /// `compute_price_index (pure derive)` → snapshot read-view →
src/bus.rs:497:    /// **Replay-deterministic** (Art.0.2): `compute_price_index` and
src/bus.rs:499:    /// `price_index` / `mask_set` are reproducible from any byte-equal
src/bus.rs:505:    /// price_index + mask_set are empty `BTreeMap` / `BTreeSet`. Callers
src/bus.rs:511:        let (price_index, mask_set) = match self.sequencer.as_ref() {
src/bus.rs:514:                    let pi = crate::state::compute_price_index(&q.economic_state_t);
src/bus.rs:515:                    let ms = crate::state::compute_mask_set(
src/bus.rs:517:                        &self.kernel.tape,
src/bus.rs:535:            tape: self.kernel.tape.clone(),
src/bus.rs:536:            price_index,
src/bus.rs:570:                assert!(bus.kernel.tape.get(&node_id).is_some());
src/bus.rs:688:        // price_index: BTreeMap<TxId, NodeMarketEntry> + mask_set: BTreeSet<TxId>.
src/bus.rs:696:        assert!(snap.price_index.is_empty(), "no sequencer → empty price_index");
src/bus.rs:698:        assert!(snap.tape.get(&"tx_0_by_A0".to_string()).is_some(),

exec
/bin/bash -lc "nl -ba tests/tb_14_halt_triggers.rs | sed -n '1,220p;220,380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	/// TB-14 Halt-Trigger Fixture (architect §5.7)
     2	///
     3	/// 6 tests that must ALL be green before TB-14 ships.
     4	/// Tests are filled in progressively per atom:
     5	///   Atom 2: #4 (no_f64) + #5 (zero_liquidity)
     6	///   Atom 3: #3 (parent_not_deleted) + #6 (unresolved_challenge)
     7	///   Atom 5: #1 (price_vs_predicate) + #2 (price_vs_l4)
     8	///
     9	/// Any atom that flips a green test to red = immediate halt (no round-2).
    10	/// TRACE_MATRIX FC3-N42 + FC2-N28 + FC2-N29
    11	
    12	// ────────────────────────────────────────────────────────────────────
    13	// Halt-trigger #1
    14	// price_does_not_affect_predicate_result
    15	//
    16	// A WorkTx with price_yes=Some(near-1) but acceptance.value=false
    17	// must still return AcceptancePredicateFailed from dispatch_transition.
    18	// Price signal MUST NOT override the predicate gate at sequencer.rs:516-558.
    19	//
    20	// TB-14 Atom 5 structural enforcement: dispatch_transition's source
    21	// path contains zero references to TB-14 price/mask types. Decoupling
    22	// is enforced by code structure — if sequencer never reads
    23	// compute_price_index / NodeMarketEntry / RationalPrice / mask_set,
    24	// they cannot affect predicate evaluation at runtime. (Parallel to
    25	// halt-trigger #4's file-level decimal-float fence.)
    26	// ────────────────────────────────────────────────────────────────────
    27	#[test]
    28	fn price_does_not_affect_predicate_result() {
    29	    let manifest = env!("CARGO_MANIFEST_DIR");
    30	    let sequencer_path = format!("{}/src/state/sequencer.rs", manifest);
    31	    let body = std::fs::read_to_string(&sequencer_path)
    32	        .unwrap_or_else(|e| panic!("read {}: {}", sequencer_path, e));
    33	
    34	    // The price/mask types must NOT appear in the sequencer dispatch path.
    35	    // Constructed at runtime via byte literals to avoid this test's own
    36	    // source containing the substrings being scanned for.
    37	    let forbidden: Vec<String> = vec![
    38	        format!("compute_price{}", "_index"),
    39	        format!("compute_mask{}", "_set"),
    40	        format!("NodeMarket{}", "Entry"),
    41	        format!("Rational{}", "Price"),
    42	        format!("Boltzmann{}", "MaskPolicy"),
    43	    ];
    44	    for tok in &forbidden {
    45	        assert!(
    46	            !body.contains(tok.as_str()),
    47	            "halt-trigger #1: src/state/sequencer.rs MUST NOT reference TB-14 \
    48	             price/mask type `{}` — sequencer dispatch is decoupled from price \
    49	             signal by construction (CR-14.1)",
    50	            tok
    51	        );
    52	    }
    53	}
    54	
    55	// ────────────────────────────────────────────────────────────────────
    56	// Halt-trigger #2
    57	// price_does_not_change_l4_decision
    58	//
    59	// A tx that fails L4 (AcceptancePredicateFailed) must enter L4.E,
    60	// not L4, even when the node has a high price_yes in compute_price_index.
    61	//
    62	// TB-14 Atom 5 structural enforcement (complementary to halt-trigger #1):
    63	// `src/state/sequencer.rs` MUST NOT IMPORT any TB-14 price/mask
    64	// type via `use` statement. Halt-trigger #1 scans for symbol uses
    65	// in the file body; halt-trigger #2 scans the `use` block to catch
    66	// import-only references (e.g., a re-export forwarder that would
    67	// otherwise let TB-14 types leak into sequencer scope without an
    68	// in-body call). Together: sequencer is permanently price-blind by
    69	// construction → L4/L4.E classification is a pure function of
    70	// dispatch_transition's verdict, never of any price signal.
    71	//
    72	// This is permanent: even after Atom 6's bus.rs snapshot wire-swap
    73	// (which legitimately reads compute_price_index for read-view
    74	// broadcast), sequencer.rs MUST remain free of TB-14 imports.
    75	// ────────────────────────────────────────────────────────────────────
    76	#[test]
    77	fn price_does_not_change_l4_decision() {
    78	    let manifest = env!("CARGO_MANIFEST_DIR");
    79	    let sequencer_path = format!("{}/src/state/sequencer.rs", manifest);
    80	    let body = std::fs::read_to_string(&sequencer_path)
    81	        .unwrap_or_else(|e| panic!("read {}: {}", sequencer_path, e));
    82	
    83	    // Scan ONLY the `use` statements in sequencer.rs for any TB-14 import.
    84	    // Constructed at runtime via byte literals to avoid self-reference.
    85	    let import_tokens: Vec<String> = vec![
    86	        format!("price{}", "_index"),                  // module path
    87	        format!("compute_price{}", "_index"),
    88	        format!("compute_mask{}", "_set"),
    89	        format!("NodeMarket{}", "Entry"),
    90	        format!("Rational{}", "Price"),
    91	        format!("Boltzmann{}", "MaskPolicy"),
    92	    ];
    93	    let mut violations: Vec<String> = Vec::new();
    94	    for (i, line) in body.lines().enumerate() {
    95	        let trimmed = line.trim_start();
    96	        if !trimmed.starts_with("use ") && !trimmed.starts_with("pub use ") {
    97	            continue;
    98	        }
    99	        for tok in &import_tokens {
   100	            if line.contains(tok.as_str()) {
   101	                violations.push(format!(
   102	                    "sequencer.rs:{}: forbidden TB-14 import token `{}` in `{}`",
   103	                    i + 1,
   104	                    tok,
   105	                    line.trim()
   106	                ));
   107	            }
   108	        }
   109	    }
   110	    assert!(
   111	        violations.is_empty(),
   112	        "halt-trigger #2: src/state/sequencer.rs MUST NOT IMPORT any TB-14 \
   113	         price/mask type. Sequencer remains permanently price-blind by \
   114	         construction; L4/L4.E classification is a pure function of \
   115	         dispatch_transition's verdict (CR-14.2). Violations:\n{}",
   116	        violations.join("\n")
   117	    );
   118	}
   119	
   120	// ────────────────────────────────────────────────────────────────────
   121	// Halt-trigger #3
   122	// parent_not_deleted_from_chaintape
   123	//
   124	// After compute_mask_set includes a parent_id, the full Tape iteration
   125	// (tape.nodes()) must still yield that parent node.
   126	// mask_set filters the SCHEDULER read-view, not ChainTape storage.
   127	// ────────────────────────────────────────────────────────────────────
   128	#[test]
   129	fn parent_not_deleted_from_chaintape() {
   130	    // TB-14 Atom 3: CR-14.3 / SG-14.3 — masking is read-view, not deletion.
   131	    use turingosv4::economy::money::MicroCoin;
   132	    use turingosv4::ledger::{Node, Tape};
   133	    use turingosv4::state::q_state::AgentId;
   134	    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
   135	    use turingosv4::state::{
   136	        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
   137	        TaskId, TxId,
   138	    };
   139	
   140	    fn node(id: &str, parents: &[&str]) -> Node {
   141	        Node {
   142	            id: id.to_string(),
   143	            author: "author".into(),
   144	            payload: format!("payload_{id}"),
   145	            citations: parents.iter().map(|s| s.to_string()).collect(),
   146	            created_at: 0,
   147	            completion_tokens: 0,
   148	        }
   149	    }
   150	    fn position(
   151	        pid: &str,
   152	        node_id: &str,
   153	        owner: &str,
   154	        side: PositionSide,
   155	        kind: PositionKind,
   156	        amount_micro: i64,
   157	    ) -> NodePosition {
   158	        NodePosition {
   159	            position_id: TxId(pid.into()),
   160	            node_id: TxId(node_id.into()),
   161	            task_id: TaskId("t1".into()),
   162	            owner: AgentId(owner.into()),
   163	            side,
   164	            kind,
   165	            amount: MicroCoin::from_micro_units(amount_micro),
   166	            source_tx: TxId(pid.into()),
   167	            opened_at_round: 1,
   168	        }
   169	    }
   170	
   171	    // Build parent → child Tape; parent 50/50, child 100/0 (clear dominance).
   172	    let mut tape = Tape::new();
   173	    tape.append(node("parent", &[])).expect("append parent");
   174	    tape.append(node("child", &["parent"])).expect("append child");
   175	
   176	    let mut econ = EconomicState::default();
   177	    for p in [
   178	        position("p1", "parent", "ag_pl", PositionSide::Long,
   179	                 PositionKind::FirstLong, 500_000),
   180	        position("p2", "parent", "ag_ps", PositionSide::Short,
   181	                 PositionKind::ChallengeShort, 500_000),
   182	        position("p3", "child", "ag_cl", PositionSide::Long,
   183	                 PositionKind::FirstLong, 2_000_000),
   184	    ] {
   185	        econ.node_positions_t.0.insert(p.position_id.clone(), p);
   186	    }
   187	
   188	    let policy = BoltzmannMaskPolicy::default();
   189	    let price_index = compute_price_index(&econ);
   190	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   191	
   192	    // Prerequisite: parent IS masked (so the test below is meaningful).
   193	    assert!(
   194	        mask.contains(&TxId("parent".into())),
   195	        "halt-trigger #3 prerequisite: parent must be masked under default policy"
   196	    );
   197	
   198	    // Halt-trigger #3 assertion: tape.nodes() still yields masked parent.
   199	    assert!(
   200	        tape.nodes().contains_key("parent"),
   201	        "halt-trigger #3: tape.nodes() MUST still contain masked parent (CR-14.3)"
   202	    );
   203	    // And the parent → child edge is preserved.
   204	    assert!(
   205	        tape.children("parent").contains(&"child".to_string()),
   206	        "halt-trigger #3: tape.children() edge MUST be preserved across mask"
   207	    );
   208	}
   209	
   210	// ────────────────────────────────────────────────────────────────────
   211	// Halt-trigger #4
   212	// no_f64_in_tb_14_modules
   213	//
   214	// src/state/price_index.rs and the TB-14 spans of src/sdk/actor.rs
   215	// must contain zero occurrences of decimal-float-type tokens.
   216	// ────────────────────────────────────────────────────────────────────
   217	#[test]
   218	fn no_f64_in_tb_14_modules() {
   219	    // TB-14 Atom 2: enforce zero decimal-float-type tokens in TB-14 modules.
   220	    // Plan v2 G1: this test reads `src/state/price_index.rs` at runtime via
   220	    // Plan v2 G1: this test reads `src/state/price_index.rs` at runtime via
   221	    // `std::fs::read_to_string` (NEVER `include_str!`, which would inline
   222	    // this very test's assertion strings — a self-reference trap that
   223	    // sank the previous /opusplan attempt). Plan v2 G1 also requires
   224	    // `src/state/price_index.rs` to contain zero substrings of the
   225	    // forbidden types ANYWHERE — including comments — so the check is a
   226	    // trivial substring search with no comment-stripping needed.
   227	    //
   228	    // The forbidden tokens are constructed at runtime from byte literals
   229	    // joined into a String, so this test's source code does not contain
   230	    // the literal substrings being scanned for.
   231	    let forbidden: Vec<String> = vec![
   232	        format!("{}{}", "f", "64"),
   233	        format!("{}{}", "f", "32"),
   234	    ];
   235	
   236	    let manifest = env!("CARGO_MANIFEST_DIR");
   237	    let price_index_path = format!("{}/src/state/price_index.rs", manifest);
   238	    let body = std::fs::read_to_string(&price_index_path)
   239	        .unwrap_or_else(|e| panic!("read {}: {}", price_index_path, e));
   240	    for tok in &forbidden {
   241	        assert!(
   242	            !body.contains(tok.as_str()),
   243	            "TB-14 halt-trigger #4 violated: src/state/price_index.rs contains forbidden \
   244	             decimal-float-type token `{}` somewhere (Plan v2 G1 requires zero substring \
   245	             occurrences anywhere in the file, including comments)",
   246	            tok
   247	        );
   248	    }
   249	}
   250	
   251	// ────────────────────────────────────────────────────────────────────
   252	// Halt-trigger #5
   253	// zero_liquidity_returns_none
   254	//
   255	// compute_price_index over an EconomicState where a node_id has
   256	// zero long AND zero short interest must return an entry where
   257	// price_yes == None AND price_no == None (FR-14.3).
   258	// Non-None price for zero-liquidity = forbidden.
   259	// ────────────────────────────────────────────────────────────────────
   260	#[test]
   261	fn zero_liquidity_returns_none() {
   262	    // TB-14 Atom 2: FR-14.3 — empty / zero-stake node yields None price.
   263	    use turingosv4::economy::money::MicroCoin;
   264	    use turingosv4::state::{compute_price_index, EconomicState, TaskId, TxId};
   265	    use turingosv4::state::q_state::AgentId;
   266	    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
   267	
   268	    // Case A: completely empty state → empty index (no entries at all).
   269	    let econ_a = EconomicState::default();
   270	    let idx_a = compute_price_index(&econ_a);
   271	    assert!(
   272	        idx_a.is_empty(),
   273	        "TB-14 halt-trigger #5: empty node_positions_t → empty PriceIndex"
   274	    );
   275	
   276	    // Case B: a node with one zero-amount Long position → entry exists,
   277	    // price_yes = None AND price_no = None per FR-14.3.
   278	    let mut econ_b = EconomicState::default();
   279	    econ_b.node_positions_t.0.insert(
   280	        TxId("zero_pos".into()),
   281	        NodePosition {
   282	            position_id: TxId("zero_pos".into()),
   283	            node_id: TxId("zero_node".into()),
   284	            task_id: TaskId("zero_task".into()),
   285	            owner: AgentId("zero_agent".into()),
   286	            side: PositionSide::Long,
   287	            kind: PositionKind::FirstLong,
   288	            amount: MicroCoin::zero(),
   289	            source_tx: TxId("zero_pos".into()),
   290	            opened_at_round: 1,
   291	        },
   292	    );
   293	    let idx_b = compute_price_index(&econ_b);
   294	    let entry = idx_b
   295	        .get(&TxId("zero_node".into()))
   296	        .expect("zero_node entry must be present in index");
   297	    assert_eq!(
   298	        entry.price_yes, None,
   299	        "TB-14 halt-trigger #5: zero stake → price_yes MUST be None (FR-14.3)"
   300	    );
   301	    assert_eq!(
   302	        entry.price_no, None,
   303	        "TB-14 halt-trigger #5: zero stake → price_no MUST be None (FR-14.3)"
   304	    );
   305	    assert_eq!(entry.long_interest, MicroCoin::zero());
   306	    assert_eq!(entry.short_interest, MicroCoin::zero());
   307	    assert_eq!(entry.liquidity_depth, MicroCoin::zero());
   308	}
   309	
   310	// ────────────────────────────────────────────────────────────────────
   311	// Halt-trigger #6
   312	// unresolved_challenge_blocks_masking
   313	//
   314	// If a child node has a ChallengeCase with status=Open targeting it,
   315	// compute_mask_set must NOT include the parent in the mask_set
   316	// even if child.price_yes dominates parent.price_yes by price_margin.
   317	// (CR-14.5 + SG-14.7)
   318	// ────────────────────────────────────────────────────────────────────
   319	#[test]
   320	fn unresolved_challenge_blocks_masking() {
   321	    // TB-14 Atom 3: CR-14.5 / SG-14.7 — Open challenge against child blocks parent masking.
   322	    use turingosv4::economy::money::MicroCoin;
   323	    use turingosv4::ledger::{Node, Tape};
   324	    use turingosv4::state::q_state::{AgentId, ChallengeCase, ChallengeStatus};
   325	    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
   326	    use turingosv4::state::{
   327	        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
   328	        TaskId, TxId,
   329	    };
   330	
   331	    fn node(id: &str, parents: &[&str]) -> Node {
   332	        Node {
   333	            id: id.to_string(),
   334	            author: "author".into(),
   335	            payload: format!("payload_{id}"),
   336	            citations: parents.iter().map(|s| s.to_string()).collect(),
   337	            created_at: 0,
   338	            completion_tokens: 0,
   339	        }
   340	    }
   341	    fn position(
   342	        pid: &str,
   343	        node_id: &str,
   344	        owner: &str,
   345	        side: PositionSide,
   346	        kind: PositionKind,
   347	        amount_micro: i64,
   348	    ) -> NodePosition {
   349	        NodePosition {
   350	            position_id: TxId(pid.into()),
   351	            node_id: TxId(node_id.into()),
   352	            task_id: TaskId("t1".into()),
   353	            owner: AgentId(owner.into()),
   354	            side,
   355	            kind,
   356	            amount: MicroCoin::from_micro_units(amount_micro),
   357	            source_tx: TxId(pid.into()),
   358	            opened_at_round: 1,
   359	        }
   360	    }
   361	
   362	    // Build parent → child Tape; parent 50/50, child 100/0 (would dominate
   363	    // under default policy if no challenge present).
   364	    let mut tape = Tape::new();
   365	    tape.append(node("parent", &[])).expect("append parent");
   366	    tape.append(node("child", &["parent"])).expect("append child");
   367	
   368	    let mut econ = EconomicState::default();
   369	    for p in [
   370	        position("p1", "parent", "ag_pl", PositionSide::Long,
   371	                 PositionKind::FirstLong, 500_000),
   372	        position("p2", "parent", "ag_ps", PositionSide::Short,
   373	                 PositionKind::ChallengeShort, 500_000),
   374	        position("p3", "child", "ag_cl", PositionSide::Long,
   375	                 PositionKind::FirstLong, 2_000_000),
   376	    ] {
   377	        econ.node_positions_t.0.insert(p.position_id.clone(), p);
   378	    }
   379	
   380	    // Add Open challenge against the child → parent masking MUST be blocked.

exec
/bin/bash -lc "nl -ba tests/tb_14_mask_set.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-14 Atom 3 — SG-14.3 + SG-14.7 + SG-14.8 explicit witness suite for
     2	//! `compute_mask_set`.
     3	//!
     4	//! TRACE_MATRIX TB-14 SG-14.3 / SG-14.7 / SG-14.8 (charter §6 ship-gates table).
     5	//! These three ship gates are the named integration-test targets per
     6	//! `handover/tracer_bullets/TB-14_charter_2026-05-03.md` §6:
     7	//!
     8	//!   SG-14.3  Parent not deleted from ChainTape after masking.
     9	//!   SG-14.7  Unresolved challenge blocks masking.
    10	//!   SG-14.8  Low-liquidity manipulation cannot mask parent.
    11	//!
    12	//! Plus: CR-14.4 (low-liquidity boundary) + CR-14.5 (open-challenge boundary)
    13	//! explicit witnesses + happy-path "child dominates parent" mask insertion.
    14	
    15	use turingosv4::economy::money::MicroCoin;
    16	use turingosv4::ledger::{Node, Tape};
    17	use turingosv4::state::q_state::{AgentId, ChallengeCase, ChallengeStatus};
    18	use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    19	use turingosv4::state::{
    20	    compute_price_index, BoltzmannMaskPolicy, EconomicState, RationalPrice, TaskId, TxId,
    21	};
    22	use turingosv4::state::price_index::compute_mask_set;
    23	
    24	fn make_position(
    25	    position_id: &str,
    26	    node_id: &str,
    27	    task_id: &str,
    28	    owner: &str,
    29	    side: PositionSide,
    30	    kind: PositionKind,
    31	    amount_micro: i64,
    32	) -> NodePosition {
    33	    NodePosition {
    34	        position_id: TxId(position_id.into()),
    35	        node_id: TxId(node_id.into()),
    36	        task_id: TaskId(task_id.into()),
    37	        owner: AgentId(owner.into()),
    38	        side,
    39	        kind,
    40	        amount: MicroCoin::from_micro_units(amount_micro),
    41	        source_tx: TxId(position_id.into()),
    42	        opened_at_round: 1,
    43	    }
    44	}
    45	
    46	fn make_node(id: &str, citations: &[&str]) -> Node {
    47	    Node {
    48	        id: id.to_string(),
    49	        author: "test_author".to_string(),
    50	        payload: format!("payload_for_{id}"),
    51	        citations: citations.iter().map(|s| s.to_string()).collect(),
    52	        created_at: 0,
    53	        completion_tokens: 0,
    54	    }
    55	}
    56	
    57	/// Build a minimal Tape + EconomicState + ChallengeCases triple for mask
    58	/// testing. Parent node has Long-only positions (price_yes near 1); child
    59	/// node has Long-only positions (price_yes near 1) by default — tests that
    60	/// want a dominance gap or a domination block adjust per case.
    61	fn baseline_econ_with_parent_child(
    62	    parent_long: i64,
    63	    parent_short: i64,
    64	    child_long: i64,
    65	    child_short: i64,
    66	) -> (EconomicState, Tape) {
    67	    let mut tape = Tape::new();
    68	    tape.append(make_node("parent_node", &[])).expect("append parent");
    69	    tape.append(make_node("child_node", &["parent_node"]))
    70	        .expect("append child");
    71	
    72	    let mut econ = EconomicState::default();
    73	    if parent_long > 0 {
    74	        let p = make_position(
    75	            "parent_long_pos",
    76	            "parent_node",
    77	            "task_p",
    78	            "agent_pl",
    79	            PositionSide::Long,
    80	            PositionKind::FirstLong,
    81	            parent_long,
    82	        );
    83	        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    84	    }
    85	    if parent_short > 0 {
    86	        let p = make_position(
    87	            "parent_short_pos",
    88	            "parent_node",
    89	            "task_p",
    90	            "agent_ps",
    91	            PositionSide::Short,
    92	            PositionKind::ChallengeShort,
    93	            parent_short,
    94	        );
    95	        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    96	    }
    97	    if child_long > 0 {
    98	        let c = make_position(
    99	            "child_long_pos",
   100	            "child_node",
   101	            "task_c",
   102	            "agent_cl",
   103	            PositionSide::Long,
   104	            PositionKind::FirstLong,
   105	            child_long,
   106	        );
   107	        econ.node_positions_t.0.insert(c.position_id.clone(), c);
   108	    }
   109	    if child_short > 0 {
   110	        let c = make_position(
   111	            "child_short_pos",
   112	            "child_node",
   113	            "task_c",
   114	            "agent_cs",
   115	            PositionSide::Short,
   116	            PositionKind::ChallengeShort,
   117	            child_short,
   118	        );
   119	        econ.node_positions_t.0.insert(c.position_id.clone(), c);
   120	    }
   121	
   122	    (econ, tape)
   123	}
   124	
   125	/// SG-14.3 — parent_id may appear in mask_set, but tape.nodes() still yields it.
   126	#[test]
   127	fn sg_14_3_parent_not_deleted_from_chaintape_after_masking() {
   128	    // Parent has 50/50 long/short (price_yes = 0.5); child has 100/0 long/short
   129	    // (price_yes = 1.0). Gap = 0.5; default policy margin = 0.10. Child masks parent.
   130	    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
   131	    let policy = BoltzmannMaskPolicy::default();
   132	    let price_index = compute_price_index(&econ);
   133	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   134	
   135	    assert!(
   136	        mask.contains(&TxId("parent_node".into())),
   137	        "SG-14.3 prerequisite: parent must be masked when child dominates"
   138	    );
   139	
   140	    // SG-14.3: tape.nodes() still yields the masked parent.
   141	    assert!(
   142	        tape.nodes().contains_key("parent_node"),
   143	        "SG-14.3: tape.nodes() MUST still contain masked parent (read-view mask only, not deletion)"
   144	    );
   145	    assert!(
   146	        tape.nodes().contains_key("child_node"),
   147	        "SG-14.3: tape.nodes() MUST still contain child"
   148	    );
   149	    // Tape children edge from parent → child preserved.
   150	    assert!(
   151	        tape.children("parent_node").contains(&"child_node".to_string()),
   152	        "SG-14.3: tape.children() relationship MUST be preserved"
   153	    );
   154	}
   155	
   156	/// SG-14.7 / CR-14.5 — open challenge against child blocks masking.
   157	#[test]
   158	fn sg_14_7_unresolved_challenge_blocks_masking() {
   159	    let (mut econ, tape) =
   160	        baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
   161	    // Add a ChallengeCase against the child with status = Open.
   162	    econ.challenge_cases_t.0.insert(
   163	        TxId("ch_against_child".into()),
   164	        ChallengeCase {
   165	            challenger: AgentId("challenger".into()),
   166	            bond: MicroCoin::from_micro_units(1_000),
   167	            opened_at_round: 1,
   168	            target_work_tx: TxId("child_node".into()),
   169	            status: ChallengeStatus::Open,
   170	        },
   171	    );
   172	
   173	    let policy = BoltzmannMaskPolicy::default();
   174	    let price_index = compute_price_index(&econ);
   175	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   176	
   177	    assert!(
   178	        !mask.contains(&TxId("parent_node".into())),
   179	        "SG-14.7: open challenge against child MUST block parent masking, even though child price would otherwise dominate"
   180	    );
   181	}
   182	
   183	/// SG-14.7 boundary — Released challenge does NOT block masking (only Open does).
   184	#[test]
   185	fn sg_14_7_released_challenge_does_not_block_masking() {
   186	    let (mut econ, tape) =
   187	        baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
   188	    econ.challenge_cases_t.0.insert(
   189	        TxId("ch_resolved".into()),
   190	        ChallengeCase {
   191	            challenger: AgentId("challenger".into()),
   192	            bond: MicroCoin::from_micro_units(1_000),
   193	            opened_at_round: 1,
   194	            target_work_tx: TxId("child_node".into()),
   195	            status: ChallengeStatus::Released,
   196	        },
   197	    );
   198	
   199	    let policy = BoltzmannMaskPolicy::default();
   200	    let price_index = compute_price_index(&econ);
   201	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   202	
   203	    assert!(
   204	        mask.contains(&TxId("parent_node".into())),
   205	        "SG-14.7 boundary: Released challenge does NOT block masking"
   206	    );
   207	}
   208	
   209	/// SG-14.8 / CR-14.4 — child below `min_liquidity` cannot mask parent.
   210	#[test]
   211	fn sg_14_8_low_liquidity_child_cannot_mask_parent() {
   212	    // Parent 50/50, child has only 100 micro-units of liquidity (well below
   213	    // the 1_000_000 micro min_liquidity default).
   214	    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 100, 0);
   215	    let policy = BoltzmannMaskPolicy::default();
   216	    let price_index = compute_price_index(&econ);
   217	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   218	
   219	    assert!(
   220	        !mask.contains(&TxId("parent_node".into())),
   221	        "SG-14.8: child below min_liquidity MUST NOT mask parent (low-liquidity manipulation guard)"
   222	    );
   223	}
   224	
   225	/// Happy path: child clearly dominates parent → parent masked.
   226	#[test]
   227	fn child_dominates_parent_inserts_into_mask_set() {
   228	    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
   229	    let policy = BoltzmannMaskPolicy::default();
   230	    let price_index = compute_price_index(&econ);
   231	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   232	
   233	    assert_eq!(mask.len(), 1, "exactly one parent should be masked");
   234	    assert!(mask.contains(&TxId("parent_node".into())));
   235	}
   236	
   237	/// Boundary: child price equal to parent price → does NOT mask (gap = 0 < margin).
   238	#[test]
   239	fn child_with_equal_price_does_not_mask() {
   240	    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 1_000_000, 1_000_000);
   241	    let policy = BoltzmannMaskPolicy::default();
   242	    let price_index = compute_price_index(&econ);
   243	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   244	
   245	    assert!(
   246	        !mask.contains(&TxId("parent_node".into())),
   247	        "child price = parent price (gap = 0) MUST NOT mask"
   248	    );
   249	}
   250	
   251	/// Boundary: child gap below margin → does NOT mask.
   252	/// Parent 50/50 (price_yes = 0.5); child 55/45 (price_yes = 0.55). Gap = 0.05.
   253	/// Default margin = 0.10. 0.05 < 0.10 → no mask.
   254	#[test]
   255	fn child_with_gap_below_margin_does_not_mask() {
   256	    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 1_100_000, 900_000);
   257	    let policy = BoltzmannMaskPolicy::default();
   258	    let price_index = compute_price_index(&econ);
   259	    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
   260	

exec
/bin/bash -lc 'git diff --name-only a9fbdf3..38412bf' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/evaluator.rs
genesis_payload.toml
handover/ai-direct/LATEST.md
handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md
handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md
handover/evidence/tb_14_chaintape_smoke_2026-05-03/agent_pubkeys.json
handover/evidence/tb_14_chaintape_smoke_2026-05-03/genesis_report.json
handover/evidence/tb_14_chaintape_smoke_2026-05-03/pinned_pubkeys.json
handover/evidence/tb_14_chaintape_smoke_2026-05-03/replay_report.json
src/bin/audit_dashboard.rs
src/bus.rs
src/kernel.rs
src/lib.rs
src/prediction_market.rs
src/sdk/actor.rs
src/sdk/snapshot.rs
tests/fc_alignment_conformance.rs
tests/tb_13_legacy_cpmm_forward_fence.rs
tests/tb_14_chaintape_smoke.rs
tests/wal_resume.rs

exec
/bin/bash -lc 'git diff -- src/state/price_index.rs src/sdk/actor.rs src/sdk/snapshot.rs src/bus.rs src/kernel.rs src/bin/audit_dashboard.rs experiments/minif2f_v4/src/bin/evaluator.rs tests/tb_14_chaintape_smoke.rs a9fbdf3..38412bf --stat' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "rg -n \"f64|f32|BinaryMarket|prediction_market|buy_yes|buy_no|market_ticker|bounty|resolve_all|AMM|DPMM|oracle|truth|settlement|MarketSnapshot|BoltzmannParams|boltzmann_select_parent\\b|Invested\" src/kernel.rs src/bus.rs src/sdk/snapshot.rs src/sdk/actor.rs src/bin/audit_dashboard.rs experiments/minif2f_v4/src/bin/evaluator.rs src/state/price_index.rs src/lib.rs tests/tb_13_legacy_cpmm_forward_fence.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_13_legacy_cpmm_forward_fence.rs:7://! modules cannot import or reuse the legacy `src/prediction_market.rs`
tests/tb_13_legacy_cpmm_forward_fence.rs:8://! f64 CPMM scaffolding. Architect §4.2 halting triggers:
tests/tb_13_legacy_cpmm_forward_fence.rs:10://!   HALT if new TB-13 code imports legacy prediction_market.rs.
tests/tb_13_legacy_cpmm_forward_fence.rs:11://!   HALT if f64 appears in new CompleteSet / MarketSeed code.
tests/tb_13_legacy_cpmm_forward_fence.rs:12://!   HALT if any AMM / CPMM router function is introduced in TB-13.
tests/tb_13_legacy_cpmm_forward_fence.rs:80:    "prediction_market::",
tests/tb_13_legacy_cpmm_forward_fence.rs:81:    "BinaryMarket",
tests/tb_13_legacy_cpmm_forward_fence.rs:83:    ".buy_yes(",
tests/tb_13_legacy_cpmm_forward_fence.rs:84:    ".buy_no(",
tests/tb_13_legacy_cpmm_forward_fence.rs:85:    "open_bounty_market",
tests/tb_13_legacy_cpmm_forward_fence.rs:86:    "bounty_market",
tests/tb_13_legacy_cpmm_forward_fence.rs:87:    "bounty_lp_seed",
tests/tb_13_legacy_cpmm_forward_fence.rs:88:    "bounty_yes_price",
tests/tb_13_legacy_cpmm_forward_fence.rs:89:    "resolve_bounty",
tests/tb_13_legacy_cpmm_forward_fence.rs:90:    "market_ticker(",
tests/tb_13_legacy_cpmm_forward_fence.rs:91:    "market_ticker_full(",
tests/tb_13_legacy_cpmm_forward_fence.rs:92:    // f64 in money-path context (see SG-13.0.2 dedicated test for the
tests/tb_13_legacy_cpmm_forward_fence.rs:93:    // primary check; this entry catches `f64` in any TB-13-marked span).
tests/tb_13_legacy_cpmm_forward_fence.rs:94:    " f64",
tests/tb_13_legacy_cpmm_forward_fence.rs:95:    "f64,",
tests/tb_13_legacy_cpmm_forward_fence.rs:96:    "f64;",
tests/tb_13_legacy_cpmm_forward_fence.rs:97:    "f64)",
tests/tb_13_legacy_cpmm_forward_fence.rs:98:    // Trading / AMM / orderbook concepts forbidden in TB-13 (per §4.7).
tests/tb_13_legacy_cpmm_forward_fence.rs:103:    "AMM",
tests/tb_13_legacy_cpmm_forward_fence.rs:105:    "DPMM",
tests/tb_13_legacy_cpmm_forward_fence.rs:107:    // Price-as-truth concepts (deferred to TB-14 per §5).
tests/tb_13_legacy_cpmm_forward_fence.rs:175:/// non-marker TB-13 type-use plus f64/AMM tokens outside any marker
tests/tb_13_legacy_cpmm_forward_fence.rs:415:/// Used by Layer 2 forbidden-token scan + `no_f64_in_complete_set_or_market_seed`.
tests/tb_13_legacy_cpmm_forward_fence.rs:418:/// narrower than Layer 1 because Layer 2 tokens (f64 / AMM / orderbook /
tests/tb_13_legacy_cpmm_forward_fence.rs:440:/// crate::prediction_market::BinaryMarket` anywhere in scope is an
tests/tb_13_legacy_cpmm_forward_fence.rs:443:    "use crate::prediction_market::",
tests/tb_13_legacy_cpmm_forward_fence.rs:444:    "use crate::prediction_market;",
tests/tb_13_legacy_cpmm_forward_fence.rs:445:    "crate::prediction_market::BinaryMarket",
tests/tb_13_legacy_cpmm_forward_fence.rs:446:    "crate::prediction_market::MarketError",
tests/tb_13_legacy_cpmm_forward_fence.rs:452:/// `prediction_market.rs`. Two layers of enforcement:
tests/tb_13_legacy_cpmm_forward_fence.rs:460:/// broader `FORBIDDEN_LEGACY_TOKENS` set (API names, trading/AMM
tests/tb_13_legacy_cpmm_forward_fence.rs:483:    // Layer 2: scan for trading/AMM concepts. Restricted to discovered
tests/tb_13_legacy_cpmm_forward_fence.rs:485:    // tokens (f64 / AMM / orderbook / price names) can appear legitimately
tests/tb_13_legacy_cpmm_forward_fence.rs:495:                // The `f64` family entries are checked in SG-13.0.2 — skip
tests/tb_13_legacy_cpmm_forward_fence.rs:497:                if token.starts_with(" f64")
tests/tb_13_legacy_cpmm_forward_fence.rs:498:                    || token.starts_with("f64,")
tests/tb_13_legacy_cpmm_forward_fence.rs:499:                    || token.starts_with("f64;")
tests/tb_13_legacy_cpmm_forward_fence.rs:500:                    || token.starts_with("f64)")
tests/tb_13_legacy_cpmm_forward_fence.rs:521:/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`. Now uses
tests/tb_13_legacy_cpmm_forward_fence.rs:523:/// remediation 2026-05-03 — Layer 2 tokens like f64 can appear in
tests/tb_13_legacy_cpmm_forward_fence.rs:526:/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
tests/tb_13_legacy_cpmm_forward_fence.rs:530:fn no_f64_in_complete_set_or_market_seed() {
tests/tb_13_legacy_cpmm_forward_fence.rs:532:    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
tests/tb_13_legacy_cpmm_forward_fence.rs:539:            for token in &f64_tokens {
tests/tb_13_legacy_cpmm_forward_fence.rs:542:                        "{rel}:{line_no}: TB-13-scope contains f64 (`{token}`) — {line}"
tests/tb_13_legacy_cpmm_forward_fence.rs:550:        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
tests/tb_13_legacy_cpmm_forward_fence.rs:555:/// SG-13.0.3 — `prediction_market_legacy_quarantined` (post-TB-14-Atom-6
tests/tb_13_legacy_cpmm_forward_fence.rs:559:/// fence — the legacy `src/prediction_market.rs` file remained for
tests/tb_13_legacy_cpmm_forward_fence.rs:568:/// into TB-13+ code") is preserved structurally: `src/prediction_market.rs`
tests/tb_13_legacy_cpmm_forward_fence.rs:569:/// is gone, `src/kernel.rs` carries no `markets/bounty_market/bounty_lp_seed`
tests/tb_13_legacy_cpmm_forward_fence.rs:570:/// fields, and `src/lib.rs` does not declare `pub mod prediction_market`.
tests/tb_13_legacy_cpmm_forward_fence.rs:574:fn prediction_market_legacy_quarantined() {
tests/tb_13_legacy_cpmm_forward_fence.rs:575:    // Assertion 1: src/prediction_market.rs is gone.
tests/tb_13_legacy_cpmm_forward_fence.rs:577:        .join("src/prediction_market.rs");
tests/tb_13_legacy_cpmm_forward_fence.rs:580:        "TB-13 SG-13.0.3 (post-TB-14-Atom-6): src/prediction_market.rs \
tests/tb_13_legacy_cpmm_forward_fence.rs:582:         resurrect f64 trading semantics + automatic CPMM liquidity \
tests/tb_13_legacy_cpmm_forward_fence.rs:588:    for forbidden in ["pub mod prediction_market", "mod prediction_market"] {
tests/tb_13_legacy_cpmm_forward_fence.rs:607:    for field in ["markets", "bounty_market", "bounty_lp_seed"] {
tests/tb_13_legacy_cpmm_forward_fence.rs:620:        "fn create_market", "fn buy_yes", "fn buy_no", "fn yes_price",
tests/tb_13_legacy_cpmm_forward_fence.rs:621:        "fn market_ticker", "fn open_bounty_market", "fn bounty_yes_price",
tests/tb_13_legacy_cpmm_forward_fence.rs:622:        "fn resolve_bounty", "fn resolve_all",
tests/tb_13_legacy_cpmm_forward_fence.rs:819:/// type-use-discovered files could ship f64 / AMM tokens unscanned).
tests/tb_13_legacy_cpmm_forward_fence.rs:828:pub fn tb13_thing() -> i32 { 42_f64 as i32 }\n\
tests/tb_13_legacy_cpmm_forward_fence.rs:846:    // lines. The f64 / AMM scan must see the violating line.
tests/tb_13_legacy_cpmm_forward_fence.rs:849:fn forbidden() -> f64 { 0.5_f64 }\n\
tests/tb_13_legacy_cpmm_forward_fence.rs:856:        scanned_text.iter().any(|l| l.contains("f64")),
tests/tb_13_legacy_cpmm_forward_fence.rs:857:        "unmarked-file: non-comment lines must be returned (Layer 2 must see f64)"
tests/tb_13_legacy_cpmm_forward_fence.rs:878:fn stealth(_: CompleteSetMintTx) -> f64 { 0.0_f64 }\n\
tests/tb_13_legacy_cpmm_forward_fence.rs:890:            .any(|l| l.contains("CompleteSetMintTx") && l.contains("f64")),
src/state/price_index.rs:7://! (TB-13 substrate). **Price is signal, not truth** (architect §5.1):
src/state/price_index.rs:87:/// not truth"; charter §7 auto-resolution A: "no second source-of-truth").
src/bin/audit_dashboard.rs:100:    /// TB-7.7 D6: golden path steps (only populated when chain_oracle_verified=true).
src/bin/audit_dashboard.rs:110:    /// The aggregate sum of bounty_micro across all rows is the user's total
src/bin/audit_dashboard.rs:121:    /// settlement anchor).
src/bin/audit_dashboard.rs:133:    /// signal, not truth") and SG-14.6 unit test discipline. NEVER shown
src/bin/audit_dashboard.rs:135:    /// integer-rational pair (charter §5 forbidden list: no f64 / no
src/bin/audit_dashboard.rs:208:    bounty_micro: i64,
src/bin/audit_dashboard.rs:285:    /// TB-7.7 D6: oracle_verified flag from VerificationResult (None = no VR; Some(true) = Lean accepted).
src/bin/audit_dashboard.rs:286:    oracle_verified: Option<bool>,
src/bin/audit_dashboard.rs:297:/// root → ... → the oracle-verified WorkTx, reading payload bytes from CAS.
src/bin/audit_dashboard.rs:305:    oracle_verified: bool,
src/bin/audit_dashboard.rs:399:    // for bounty amount + cross-referencing claims_in_progress for status.
src/bin/audit_dashboard.rs:409:    // TB-7.7 D6: oracle_verified_worktx_ids — set of accepted L4 WorkTx
src/bin/audit_dashboard.rs:413:    let mut oracle_verified_worktx: BTreeMap<
src/bin/audit_dashboard.rs:438:                let mut oracle_verified: Option<bool> = None;
src/bin/audit_dashboard.rs:452:                        // TB-7.7 D6: oracle_verified from VerificationResult.
src/bin/audit_dashboard.rs:455:                                oracle_verified = Some(vr.verified);
src/bin/audit_dashboard.rs:457:                                    oracle_verified_worktx.insert(
src/bin/audit_dashboard.rs:488:                    oracle_verified,
src/bin/audit_dashboard.rs:542:                    oracle_verified: None,
src/bin/audit_dashboard.rs:558:                    oracle_verified: None,
src/bin/audit_dashboard.rs:621:                    oracle_verified: None,
src/bin/audit_dashboard.rs:633:                        bounty_micro: 0,
src/bin/audit_dashboard.rs:650:                    oracle_verified: None,
src/bin/audit_dashboard.rs:655:                // task_id, accumulate the bounty.
src/bin/audit_dashboard.rs:661:                        row.bounty_micro += lock.amount.micro_units();
src/bin/audit_dashboard.rs:674:                    oracle_verified: None,
src/bin/audit_dashboard.rs:706:                    oracle_verified: None,
src/bin/audit_dashboard.rs:714:                    refund_micro: expire.bounty_refunded.micro_units(),
src/bin/audit_dashboard.rs:728:                    oracle_verified: None,
src/bin/audit_dashboard.rs:750:                    oracle_verified: None,
src/bin/audit_dashboard.rs:764:                    oracle_verified: None,
src/bin/audit_dashboard.rs:770:    // TB-7.7 D6: golden path reconstruction. For each oracle-verified
src/bin/audit_dashboard.rs:772:    // Pick the FIRST oracle_verified_worktx as the canonical golden
src/bin/audit_dashboard.rs:775:    if let Some((winner_tx_id, (agent, tactic, payload))) = oracle_verified_worktx.iter().next() {
src/bin/audit_dashboard.rs:804:                    p.oracle_verified.unwrap_or(false),
src/bin/audit_dashboard.rs:820:                oracle_verified: vr,
src/bin/audit_dashboard.rs:867:            oracle_verified: None,
src/bin/audit_dashboard.rs:958:/// source-of-truth"; charter §7 auto-resolution A).
src/bin/audit_dashboard.rs:1078:        "  chain_oracle_verified   : {} {}\n",
src/bin/audit_dashboard.rs:1079:        r.run_facts.chain_oracle_verified,
src/bin/audit_dashboard.rs:1080:        if r.run_facts.chain_oracle_verified { "✓ (Lean accepted ≥1 proof; oracle-level)" } else { "(no oracle-verified WorkTx)" }
src/bin/audit_dashboard.rs:1083:        "  chain_economic_finalized: {} (always false in TB-7; settlement = TB-9 territory)\n",
src/bin/audit_dashboard.rs:1124:        s.push_str("  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject\n");
src/bin/audit_dashboard.rs:1127:            let oracle_marker = match entry.oracle_verified {
src/bin/audit_dashboard.rs:1140:                oracle_marker,
src/bin/audit_dashboard.rs:1188:    s.push_str("§7 Golden path (root → oracle-verified WorkTx)\n");
src/bin/audit_dashboard.rs:1191:        if r.run_facts.chain_oracle_verified {
src/bin/audit_dashboard.rs:1192:            s.push_str("  (chain_oracle_verified=true but golden path empty — likely VR linkage missing)\n");
src/bin/audit_dashboard.rs:1194:            s.push_str("  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)\n");
src/bin/audit_dashboard.rs:1198:            let marker = if step.oracle_verified { "✓" } else { " " };
src/bin/audit_dashboard.rs:1203:                if step.oracle_verified { "[ORACLE]" } else { "        " },
src/bin/audit_dashboard.rs:1347:            "  task_id              | sponsor      | bounty_micro | solver       | claim_status | payout_micro | opened@t\n"
src/bin/audit_dashboard.rs:1357:                ut.bounty_micro,
src/bin/audit_dashboard.rs:1366:        let total_bounty: i64 = r.user_tasks.iter().map(|u| u.bounty_micro).sum();
src/bin/audit_dashboard.rs:1374:            "\n  Aggregate: {} user task(s) | {} Finalized | total bounty = {} micro | total paid = {} micro\n",
src/bin/audit_dashboard.rs:1375:            r.user_tasks.len(), n_finalized, total_bounty, total_paid
src/bin/audit_dashboard.rs:1393:    //   - Bankrupt tasks (TaskBankruptcyTx): future TB-12 Short / NO settlement
src/bin/audit_dashboard.rs:1487:/// truth."). Re-rendering this banner in every dashboard frame is the
src/bin/audit_dashboard.rs:1490:/// **NO DECIMAL** (charter §5 forbidden + G-14.11 ship gate "no f64 in TB-14
src/bin/audit_dashboard.rs:1546:    s.push_str("    Price is signal, not truth. NodeMarketEntry is a derived view —\n");
src/bin/audit_dashboard.rs:1547:    s.push_str("    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.\n");
src/bin/audit_dashboard.rs:1548:    s.push_str("    NO price-based settlement. NO Goodhart leak of private predicates.\n");
src/bin/audit_dashboard.rs:1628:    s.push_str("    total_supply_micro (CR-12.2). NO trading. NO price. NO settlement in TB-12.\n");
src/bin/audit_dashboard.rs:1685:    /// architect §5.1 ("Price is signal, not truth.") at the read-view
src/bin/audit_dashboard.rs:1686:    /// surface; future maintainers adding signal-as-truth language must
src/bin/audit_dashboard.rs:1689:    fn sg_14_6_dashboard_carries_price_is_signal_not_truth_banner() {
src/bin/audit_dashboard.rs:1733:                 (architect §5.6 forbidden: no f64 / no decimal). Got:\n{s}"
experiments/minif2f_v4/src/bin/evaluator.rs:10:use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
experiments/minif2f_v4/src/bin/evaluator.rs:48:/// TRACE_MATRIX FC1-N12: oracle scope — the prompt is the pre-Lean
experiments/minif2f_v4/src/bin/evaluator.rs:95:    /// 0 or 1 — Lean ground truth (= 1 iff runtime_accepted AND post_hoc_verified).
experiments/minif2f_v4/src/bin/evaluator.rs:98:    pput_runtime: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:100:    pput_verified: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:102:    pput_m_verified: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:119:    tactic_diversity: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:120:    /// Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
experiments/minif2f_v4/src/bin/evaluator.rs:138:    far: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:140:    err: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:142:    iac: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:144:    cpr: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:160:    time_secs: f64,                // wall time elapsed (function-entry bracket; legacy)
experiments/minif2f_v4/src/bin/evaluator.rs:161:    pput: f64,                     // 100/time if GP, 0 otherwise (legacy display)
experiments/minif2f_v4/src/bin/evaluator.rs:181:    unique_payload_ratio: Option<f64>,
experiments/minif2f_v4/src/bin/evaluator.rs:184:    // gp_payload = the exact text fed to oracle.verify_omega_detailed at OMEGA accept.
experiments/minif2f_v4/src/bin/evaluator.rs:232:    h_vppu: Option<f64>,
experiments/minif2f_v4/src/bin/evaluator.rs:431:    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
experiments/minif2f_v4/src/bin/evaluator.rs:461:    let oracle = Lean4Oracle::new(
experiments/minif2f_v4/src/bin/evaluator.rs:518:                // payload, post-hoc reflects "no Lean truth observed".
experiments/minif2f_v4/src/bin/evaluator.rs:537:            let verdict = oracle.verify_omega(&response.content);
experiments/minif2f_v4/src/bin/evaluator.rs:540:            // A6 FC1-N12 (Lean oracle scope): per-call event with verdict
experiments/minif2f_v4/src/bin/evaluator.rs:575:                    // Lean verify call (oracle.verify_omega returned Ok(true)),
experiments/minif2f_v4/src/bin/evaluator.rs:711:                    // Single source of truth shared between evaluator and
experiments/minif2f_v4/src/bin/evaluator.rs:860:            // truthy), the preseed sponsor swaps from tb7-7-sponsor → Agent_user_0
experiments/minif2f_v4/src/bin/evaluator.rs:931:            // TURINGOS_USER_TASK_BOUNTY_MICRO first (user's bounty); fallback to
experiments/minif2f_v4/src/bin/evaluator.rs:1104:    // (WALLET_STATE env-var json file) is deleted along with the f64
experiments/minif2f_v4/src/bin/evaluator.rs:1122:    // TB-9 collapse: ensure_agents removed; no f64 ledger to top-up. Agent
experiments/minif2f_v4/src/bin/evaluator.rs:1204:    // so the startup echo + per-tx skill lookup share one source of truth.
experiments/minif2f_v4/src/bin/evaluator.rs:1211:        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
experiments/minif2f_v4/src/bin/evaluator.rs:1236:    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
experiments/minif2f_v4/src/bin/evaluator.rs:1331:            // (cross-multiplication, no f64) for the operator log line.
experiments/minif2f_v4/src/bin/evaluator.rs:1379:                // TB-9 collapse: WalletTool no longer carries owned f64 balances.
experiments/minif2f_v4/src/bin/evaluator.rs:1513:        // price_yes (cross-multiplication argmax; no f64).
experiments/minif2f_v4/src/bin/evaluator.rs:1514:        let market_ticker_str: String = {
experiments/minif2f_v4/src/bin/evaluator.rs:1542:        // The `f64` here is purely the prompt-render contract of
experiments/minif2f_v4/src/bin/evaluator.rs:1543:        // `build_agent_prompt(... balance: f64 ...)` — `prompt.rs` is not a
experiments/minif2f_v4/src/bin/evaluator.rs:1546:        let prompt_balance: f64 = bus.sequencer.as_ref()
experiments/minif2f_v4/src/bin/evaluator.rs:1551:            .map(|micro| micro.micro_units() as f64 / 1_000_000.0)
experiments/minif2f_v4/src/bin/evaluator.rs:1555:            &chain, &skill, &market_ticker_str, &errors, &hits_ref,
experiments/minif2f_v4/src/bin/evaluator.rs:1570:        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
experiments/minif2f_v4/src/bin/evaluator.rs:1571:            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
experiments/minif2f_v4/src/bin/evaluator.rs:1608:                                // (BTreeSet<TxId>) directly — no f64
experiments/minif2f_v4/src/bin/evaluator.rs:1808:                                let oracle = Lean4Oracle::new(
experiments/minif2f_v4/src/bin/evaluator.rs:1815:                                let r_alone = oracle.verify_omega_detailed(payload);
experiments/minif2f_v4/src/bin/evaluator.rs:1842:                                        let r_combined = oracle.verify_omega_detailed(&combined);
experiments/minif2f_v4/src/bin/evaluator.rs:1899:                                        // settlement (RSP-4 / TB-9 territory).
experiments/minif2f_v4/src/bin/evaluator.rs:2107:                                        // Use oracle-blessed path: Lean has already accepted this
experiments/minif2f_v4/src/bin/evaluator.rs:2111:                                        let omega_node_id = match bus.append_oracle_accepted(
experiments/minif2f_v4/src/bin/evaluator.rs:2116:                                                warn!("[art-iv] OMEGA wtool VETO (unexpected after oracle accept): {}", reason);
experiments/minif2f_v4/src/bin/evaluator.rs:2149:                                        // deleted with the f64 mutators. Canonical ledger
experiments/minif2f_v4/src/bin/evaluator.rs:2152:                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
experiments/minif2f_v4/src/bin/evaluator.rs:2185:                                        // lean4_oracle returns "sorry_in_proof" /
experiments/minif2f_v4/src/bin/evaluator.rs:2200:                                        warn!("[tx {}] OMEGA oracle error: {}", tx, e);
experiments/minif2f_v4/src/bin/evaluator.rs:2207:                            // mutated WalletTool's f64 ledger. Per architect directive
experiments/minif2f_v4/src/bin/evaluator.rs:2208:                            // 2026-05-02 line 1574 (no f64 mutation), invest is no
experiments/minif2f_v4/src/bin/evaluator.rs:2263:                            // oracle classifies the accumulated tape+tactic prefix
experiments/minif2f_v4/src/bin/evaluator.rs:2284:                                let oracle = Lean4Oracle::new(
experiments/minif2f_v4/src/bin/evaluator.rs:2291:                                let verdict = oracle.verify_partial(&prefix);
experiments/minif2f_v4/src/bin/evaluator.rs:2532:                                        let _ = bus.append_oracle_accepted(
experiments/minif2f_v4/src/bin/evaluator.rs:2544:                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
experiments/minif2f_v4/src/bin/evaluator.rs:2591:                                        match bus.append_oracle_accepted(
experiments/minif2f_v4/src/bin/evaluator.rs:2655:        Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
experiments/minif2f_v4/src/bin/evaluator.rs:2659:    // f64 mutators. Canonical balance state survives across runs via
experiments/minif2f_v4/src/bin/evaluator.rs:2864:    unique_payload_ratio: Option<f64>,
experiments/minif2f_v4/src/bin/evaluator.rs:2902:    let elapsed = start.elapsed().as_secs_f64();
experiments/minif2f_v4/src/bin/evaluator.rs:3006:/// the exact code that the Lean oracle accepted. An external verifier can run
experiments/minif2f_v4/src/bin/evaluator.rs:3231:            "Lean rejected → progress MUST be 0 (North Star truth)");
src/sdk/actor.rs:6:// Legacy decimal-float `BoltzmannParams` / `is_frontier` / `lineage_score`
src/sdk/actor.rs:7:// / `boltzmann_select_parent` were excised together with
src/sdk/actor.rs:8:// `src/prediction_market.rs`. The integer-rational
src/sdk/snapshot.rs:5:// Legacy decimal-float `MarketSnapshot` + `UniverseSnapshot.markets`
src/sdk/snapshot.rs:6:// HashMap CPMM read-view was excised together with `prediction_market.rs`.
src/sdk/snapshot.rs:9:// + `state::compute_mask_set`. Pricing is signal, not truth.
src/sdk/snapshot.rs:11:// Dead post-TB-9-collapse `balances: HashMap<String, f64>` and
src/sdk/snapshot.rs:12:// `portfolios: HashMap<String, HashMap<NodeId, (f64, f64, f64)>>` were
src/sdk/snapshot.rs:15:// additive cleanup that closes the f64 surface in this file under the
src/sdk/snapshot.rs:16:// G-14.11 "no f64 in TB-14 module surface" ship gate.
src/kernel.rs:9:// All decimal-float CPMM scaffolding (`markets`, `bounty_market`,
src/kernel.rs:10:// `bounty_lp_seed`, `create_market`, `buy_yes`, `buy_no`, `yes_price`,
src/kernel.rs:11:// `market_ticker`, `market_ticker_full`, `open_bounty_market`,
src/kernel.rs:12:// `bounty_yes_price`, `resolve_bounty`, `resolve_all`) was excised
src/kernel.rs:13:// together with `src/prediction_market.rs`. Pricing now lives entirely
src/bus.rs:34:/// TB-14 Atom 6 (2026-05-03): `system_lp_amount: f64` was excised together
src/bus.rs:94:/// dead `Invested { node_id, shares: f64 }` variant excised — was a
src/bus.rs:96:/// arms (`grep -rn "BusResult::Invested\|Invested {"` returned only
src/bus.rs:97:/// its own declaration site). Closes G-14.11 "no f64 in TB-14 module
src/bus.rs:204:    /// TB-14 Atom 6 (2026-05-03): legacy `HAYEK_BOUNTY` env-gated bounty
src/bus.rs:205:    /// market open was excised together with `kernel.open_bounty_market`.
src/bus.rs:224:        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ false)
src/bus.rs:230:    /// work. Once the Lean oracle has accepted a full proof, those same tactics are
src/bus.rs:232:    /// wtool write that Art. IV mandates. Only oracle-accepted payloads should
src/bus.rs:235:    pub fn append_oracle_accepted(&mut self, author: &str, payload: &str,
src/bus.rs:237:        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ true)
src/bus.rs:241:                       parent_id: Option<&str>, oracle_blessed: bool) -> Result<BusResult, String> {
src/bus.rs:242:        // Phase 0: Forbidden pattern check — skipped for oracle-accepted payloads.
src/bus.rs:243:        if !oracle_blessed {
src/bus.rs:253:        // Phase 0b: Payload size limits (V3L-21). Skipped for oracle-accepted since
src/bus.rs:255:        if !oracle_blessed {
src/bus.rs:273:        // bus-level f64 wallet mutators (debit_wallet/credit_wallet/settle_portfolios).
src/bus.rs:274:        // Per architect directive 2026-05-02 line 1574 ("no f64 mutation;
src/bus.rs:279:        // f64 mutator — they live for downstream tool hooks only.
src/bus.rs:327:        // CPMM market open was excised together with `prediction_market.rs`.
src/bus.rs:330:        // `Sequencer::dispatch_transition` — never by bus-level f64 LP grant.
src/bus.rs:355:    /// TB-14 Atom 6 (2026-05-03): legacy `kernel.resolve_all(golden_path)`
src/bus.rs:356:    /// CPMM market resolution was excised together with `prediction_market.rs`.
src/bus.rs:484:    /// replacing the legacy decimal-float `markets: HashMap<_, MarketSnapshot>`
src/bus.rs:485:    /// CPMM read-view excised together with `src/prediction_market.rs`.
src/bus.rs:612:    /// vetoed unknown agents because they had no f64 balance row. After the
src/bus.rs:613:    /// projection collapse (no f64 ledger, `on_pre_append` returns `Pass`
src/bus.rs:630:        // prediction_market.rs. halt_and_settle now only fires RunEnd +
src/bus.rs:631:        // tool.on_halt hooks; settlement state lives in canonical typed-tx

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '115,215p;526,545p;620,680p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   115	
   116	/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
   117	/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
   118	///
   119	/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
   120	///
   121	/// TB-14 Atom 3 (FC2-N28; architect §5.5 + charter §3 Atom 3): `mask_set`
   122	/// is the global per-round set of parent-attempt-node `TxId`s suppressed
   123	/// in the agent read-view because a child node dominates them by
   124	/// `BoltzmannMaskPolicy.price_margin` (FR-14.5 / FR-14.6). **Read-view
   125	/// mask only**, never deletion (CR-14.3 + halt-trigger #3): the underlying
   126	/// `Tape.nodes()` iteration always yields masked parents. Computed by
   127	/// `compute_mask_set` in `src/state/price_index.rs`. `#[serde(default)]`
   128	/// for backward-compat with pre-TB-14 chain snapshots (deserialize as
   129	/// empty set).
   130	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   131	pub struct AgentVisibleProjection {
   132	    pub views: BTreeMap<AgentId, NodeId>,
   133	    #[serde(default)]
   134	    pub mask_set: BTreeSet<TxId>,
   135	}
   136	
   137	// ────────────────────────────────────────────────────────────────────────────
   138	// BudgetSnapshot — global compute / cost / wall-clock budget.
   139	// ────────────────────────────────────────────────────────────────────────────
   140	
   141	/// TRACE_MATRIX § 1.1 — global budget snapshot:
   142	/// cost ceiling (MicroCoin), wall clock remaining (ms), compute cap remaining.
   143	/// Exhaustion → halt_reason ∈ {WallClockCap, ComputeCapViolated, MaxTxExhausted}.
   144	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   145	pub struct BudgetSnapshot {
   146	    pub cost_ceiling_microcoin: MicroCoin,
   147	    pub wall_clock_remaining_ms: u64,
   148	    pub compute_cap_remaining: u64,
   149	}
   150	
   151	impl Default for BudgetSnapshot {
   152	    fn default() -> Self {
   153	        Self {
   154	            cost_ceiling_microcoin: MicroCoin::zero(),
   155	            wall_clock_remaining_ms: 0,
   156	            compute_cap_remaining: 0,
   157	        }
   158	    }
   159	}
   160	
   161	// ────────────────────────────────────────────────────────────────────────────
   162	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
   163	// ────────────────────────────────────────────────────────────────────────────
   164	
   165	/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
   166	/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
   167	/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
   168	/// in the owning atoms (CO P2.1-2.6).
   169	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   170	pub struct EconomicState {
   171	    pub balances_t: BalancesIndex,
   172	    pub escrows_t: EscrowsIndex,
   173	    pub stakes_t: StakesIndex,
   174	    pub claims_t: ClaimsIndex,
   175	    pub reputations_t: ReputationsIndex,
   176	    pub task_markets_t: TaskMarketsIndex,
   177	    pub royalty_graph_t: RoyaltyGraph,
   178	    pub challenge_cases_t: ChallengeCasesIndex,
   179	    // TB-14 Atom 2 (2026-05-03): `price_index_t: PriceIndex` removed.
   180	    // The TB-14 derived view is `compute_price_index(econ)` in
   181	    // `src/state/price_index.rs`; not canonical state per architect §5.1.
   182	    /// TB-11 (architect §6.2 ruling 2026-05-02): runs_t — `RunId` → run-summary
   183	    /// entry written by the TerminalSummaryTx dispatch arm. Anchors
   184	    /// architect's RunExhaustedTx semantics on chain-resident state. Each
   185	    /// failed evaluator run produces exactly one entry (idempotency on
   186	    /// run_id). `#[serde(default)]` for backward-compat with pre-TB-11
   187	    /// chain snapshots.
   188	    #[serde(default)]
   189	    pub runs_t: RunsIndex,
   190	    /// TRACE_MATRIX TB-12 (architect 2026-05-03 ruling §3 + §10): node_positions_t
   191	    /// — flat `BTreeMap<TxId, NodePosition>` index. **Canonical** TB-12 source
   192	    /// of truth for exposure records. **NOT a Coin holding** (CR-12.1 + CR-12.2);
   193	    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
   194	    ///
   195	    /// Architect §3 explicitly REJECTED the nested `node_market_t:
   196	    /// BTreeMap<NodeId, NodeMarketEntry>` shape — that's TB-14 derived view
   197	    /// (price + long_interest + short_interest aggregation), not canonical
   198	    /// state. Avoiding second source-of-truth (architect §3.2 reasoning;
   199	    /// TaskMarket.total_escrow precedent on cache=truth).
   200	    ///
   201	    /// Populated by accept-arm side-effect on accepted WorkTx (FirstLong) +
   202	    /// ChallengeTx (ChallengeShort) per architect §8 Atom 2. VerifyTx writes
   203	    /// nothing here per FR-12.3 + CR-12.8. `#[serde(default)]` for
   204	    /// backward-compat with pre-TB-12 chain snapshots.
   205	    #[serde(default)]
   206	    pub node_positions_t: NodePositionsIndex,
   207	    /// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
   208	    /// §4.3 + §4.4 FR-13.1..7 + CR-13.4): conditional collateral per event.
   209	    /// Locked Coin held against outstanding YES_E + NO_E share inventory.
   210	    ///
   211	    /// **IS** a Coin holding per CR-13.4 ("Locked collateral is Coin
   212	    /// holding"); included in the 6-holding `total_supply_micro` sum
   213	    /// (extends the TB-7R 5-holding sum). Mint/seed credit; redeem debit.
   214	    ///
   215	    /// **Complete-set balanced invariant** (Codex round-3 doc remediation
   526	// TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1): NodePositionsIndex —
   527	// flat exposure record index. ARCHITECT-RULING DISCIPLINE: this is the
   528	// canonical TB-12 state. No `node_market_t / NodeMarketEntry` is added —
   529	// that's TB-14 derived view. Avoids second source-of-truth risk.
   530	// ────────────────────────────────────────────────────────────────────────────
   531	
   532	/// TRACE_MATRIX TB-12 (architect 2026-05-03 §3 + §8 Atom 1): flat
   533	/// `position_id → NodePosition` index. Architect's §3 ruling chose this
   534	/// over nested `node_market_t: BTreeMap<NodeId, NodeMarketEntry>` to keep a
   535	/// single source of truth in TB-12. Populated by accept-arm side-effect
   536	/// (Atom 2) on accepted WorkTx (FirstLong) + ChallengeTx (ChallengeShort).
   537	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   538	pub struct NodePositionsIndex(
   539	    pub BTreeMap<TxId, crate::state::typed_tx::NodePosition>,
   540	);
   541	
   542	// ────────────────────────────────────────────────────────────────────────────
   543	// TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 + §4.4):
   544	// ConditionalCollateralIndex + ConditionalShareBalances — Polymarket / CTF
   545	// conditional-share substrate. **Mint identity: 1 locked Coin = 1 YES_E + 1
   620	/// Full attribution algebra lands CO P2.4.
   621	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   622	pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);
   623	
   624	/// TRACE_MATRIX WP § 2 — single royalty edge (ancestor → reuse weight). Stub; CO P2.4.
   625	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   626	pub struct RoyaltyEdge {
   627	    #[serde(default)]
   628	    pub ancestor: TxId,
   629	    #[serde(default)]
   630	    pub fraction_basis_points: u16,
   631	}
   632	
   633	/// TRACE_MATRIX WP § 2 — tx → challenge case. Full schema lands CO P2.5.
   634	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   635	pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);
   636	
   637	/// TRACE_MATRIX WP § 2 — challenge case shape (stub). Full fields land CO P2.5.
   638	///
   639	/// **TB-4 additive field**: `target_work_tx` is the back-reference to the
   640	/// `WorkTx.tx_id` this challenge accuses. Required by:
   641	/// (a) RSP-3 settlement (routing slash/release on challenge resolve must
   642	///     find the target's stakes_t entry via this backref);
   643	/// (b) Multi-challenger representability (TB-4 charter § 3.5 + directive Q4):
   644	///     two challenge_cases_t rows with distinct ChallengeTx tx_id keys
   645	///     may share the same `target_work_tx` — without the backref the
   646	///     index can't express that.
   647	/// Additive serde-default — pre-TB-4 has no production challenge_cases_t
   648	/// rows (dispatch arm was NotYetImplemented), so the migration is forward-only.
   649	///
   650	/// **TB-5 additive field**: `status: ChallengeStatus` records resolution
   651	/// outcome without removing the entry from challenge_cases_t. Default = Open.
   652	/// Released zeros bond + flips status to Released (audit trail preserved per
   653	/// charter v2 § 7 Q6 — preserves slash-target reference for TB-6).
   654	/// UpheldDeferred preserves bond + flips status (TB-6 slash routing target).
   655	/// Additive serde-default — pre-TB-5 serialized rows deserialize with
   656	/// status = Open.
   657	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   658	pub struct ChallengeCase {
   659	    #[serde(default)]
   660	    pub challenger: AgentId,
   661	    #[serde(default = "MicroCoin::zero")]
   662	    pub bond: MicroCoin,
   663	    #[serde(default)]
   664	    pub opened_at_round: u64,
   665	    #[serde(default)]
   666	    pub target_work_tx: TxId,
   667	    #[serde(default)]
   668	    pub status: ChallengeStatus,    // ← TB-5 NEW
   669	}
   670	
   671	/// TRACE_MATRIX TB-5 charter v2 § 4.4 — challenge resolution status.
   672	///
   673	/// **Single source of truth** per Codex round-2 + round-3 Q4 ruling: defined
   674	/// HERE (not in typed_tx.rs); sequencer.rs imports via
   675	/// `use crate::state::q_state::ChallengeStatus;`. The on-wire resolution
   676	/// outcome enum (`ChallengeResolution { Released | UpheldDeferred }`) lives
   677	/// in typed_tx.rs alongside ChallengeResolveTx — that carries the system-
   678	/// emitted resolution outcome on L4. ChallengeStatus is the Q-side case-state
   679	/// tracker that flips on dispatch.
   680	///

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '575,655p;865,915p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   575	            let has_escrow = market.map_or(false, |m| m.total_escrow.micro_units() > 0);
   576	            if !has_escrow {
   577	                return Err(TransitionError::EscrowMissing);
   578	            }
   579	
   580	            // Step 6: solver solvency gate (charter § 4.3 step 7 NEW). Per
   581	            // WP § 14.1 + § 18 Inv 5, accepted WorkTx commits stake by
   582	            // debiting balance — solver must hold ≥ work.stake.coin.
   583	            let solver_bal = q.economic_state_t.balances_t.0
   584	                .get(&work.agent_id)
   585	                .copied()
   586	                .unwrap_or(crate::economy::money::MicroCoin::zero());
   587	            if solver_bal.micro_units() < work.stake.micro_units() {
   588	                return Err(TransitionError::InsufficientBalance);
   589	            }
   590	
   591	            // Step 7: monetary invariants ordering (existing TB-2; same shape).
   592	            assert_no_post_init_mint(tx, q)
   593	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
   594	            assert_read_is_free(tx.tx_kind(), 0)
   595	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
   596	
   597	            // Step 8: build q_next. **TB-3 NEW (charter § 3.4 lock-on-accept)**:
   598	            // accepted WorkTx atomically debits balance + locks stake into
   599	            // stakes_t. Per WP § 18 Inv 5 the YES stake is event-bound to
   600	            // the WorkTx itself; per Law 2 ("Only Investment Costs Money")
   601	            // investment is consumed at commitment. CTF is conserved
   602	            // (debit balance = credit stakes); no mint, no burn.
   603	            let mut q_next = q.clone();
   604	            let new_bal_micro = solver_bal.micro_units() - work.stake.micro_units();
   605	            q_next.economic_state_t.balances_t.0.insert(
   606	                work.agent_id.clone(),
   607	                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
   608	            );
   609	            q_next.economic_state_t.stakes_t.0.insert(
   610	                work.tx_id.clone(),
   611	                crate::state::q_state::StakeEntry {
   612	                    // StakeMicroCoin(pub MicroCoin) — unwrap the inner
   613	                    // MicroCoin (StakesIndex.amount: MicroCoin per q_state.rs).
   614	                    amount: work.stake.0,
   615	                    staker: work.agent_id.clone(),
   616	                    task_id: work.task_id.clone(),
   617	                },
   618	            );
   619	            // ──────────────────────────────────────────────────────────────
   620	            // TB-12 Atom 2 (architect 2026-05-03 ruling §3 + §8 Atom 2):
   621	            // accepted WorkTx with stake > 0 derives a `FirstLong`
   622	            // NodePosition exposure record. Pure additive index write —
   623	            // **no money mutation**, **no change** to balances_t / stakes_t
   624	            // / total_supply (those are handled above by TB-3 economic
   625	            // logic). NodePosition.amount is **NOT a Coin holding** per
   626	            // CR-12.1 + CR-12.2; the 5-holding CTF sum stays unchanged.
   627	            // FR-12.1 + FR-12.4: kind = FirstLong; node_id = work.tx_id;
   628	            // position_id = source_tx = work.tx_id (one-source-tx-one-position
   629	            // invariant for TB-12 per architect §4 last paragraph).
   630	            // ──────────────────────────────────────────────────────────────
   631	            if work.stake.micro_units() > 0 {
   632	                let position = crate::state::typed_tx::NodePosition {
   633	                    position_id: work.tx_id.clone(),
   634	                    node_id: work.tx_id.clone(),
   635	                    task_id: work.task_id.clone(),
   636	                    owner: work.agent_id.clone(),
   637	                    side: crate::state::typed_tx::PositionSide::Long,
   638	                    kind: crate::state::typed_tx::PositionKind::FirstLong,
   639	                    amount: work.stake.0,
   640	                    source_tx: work.tx_id.clone(),
   641	                    opened_at_round: work.timestamp_logical,
   642	                };
   643	                q_next
   644	                    .economic_state_t
   645	                    .node_positions_t
   646	                    .0
   647	                    .insert(work.tx_id.clone(), position);
   648	            }
   649	            // state_root advance (existing TB-2; WORKTX_ACCEPT_DOMAIN_V1).
   650	            q_next.state_root_t = worktx_accept_state_root(&q.state_root_t, tx);
   651	
   652	            // Step 9: conservation now does REAL work — not a no-op as in
   653	            // TB-2. The debit-to-stakes invariant is the primary CTF check
   654	            // on the runtime spine. Production runtime ALWAYS passes `&[]`
   655	            // (charter § 5 red line 3 / TB-2 #4 inherited).
   865	                    bond: challenge.stake.0,
   866	                    opened_at_round: q.q_t.current_round, // ← § 3.9 anchor
   867	                    target_work_tx: challenge.target_work_tx.clone(),
   868	                    status: crate::state::q_state::ChallengeStatus::Open, // TB-5 ABI default
   869	                },
   870	            );
   871	            // ──────────────────────────────────────────────────────────────
   872	            // TB-12 Atom 2 (architect 2026-05-03 ruling §3 + §8 Atom 2):
   873	            // accepted ChallengeTx with stake > 0 derives a `ChallengeShort`
   874	            // NodePosition exposure record. Pure additive index write —
   875	            // **no money mutation**, **no change** to balances_t /
   876	            // challenge_cases_t / total_supply (those are handled above by
   877	            // TB-4 economic logic). NodePosition.amount is **NOT a Coin
   878	            // holding** per CR-12.1 + CR-12.2; the 5-holding CTF sum stays
   879	            // unchanged. FR-12.2 + FR-12.5: kind = ChallengeShort; node_id
   880	            // = challenge.target_work_tx; position_id = source_tx =
   881	            // challenge.tx_id. task_id derived via stakes_t[target_work_tx]
   882	            // (the target's stake row holds the task_id backref).
   883	            // ──────────────────────────────────────────────────────────────
   884	            if challenge.stake.micro_units() > 0 {
   885	                // Q-derive task_id from the target WorkTx's stake row.
   886	                let task_id_for_position = q
   887	                    .economic_state_t
   888	                    .stakes_t
   889	                    .0
   890	                    .get(&challenge.target_work_tx)
   891	                    .map(|s| s.task_id.clone())
   892	                    .unwrap_or_default();
   893	                let position = crate::state::typed_tx::NodePosition {
   894	                    position_id: challenge.tx_id.clone(),
   895	                    node_id: challenge.target_work_tx.clone(),
   896	                    task_id: task_id_for_position,
   897	                    owner: challenge.challenger_agent.clone(),
   898	                    side: crate::state::typed_tx::PositionSide::Short,
   899	                    kind: crate::state::typed_tx::PositionKind::ChallengeShort,
   900	                    amount: challenge.stake.0,
   901	                    source_tx: challenge.tx_id.clone(),
   902	                    opened_at_round: challenge.timestamp_logical,
   903	                };
   904	                q_next
   905	                    .economic_state_t
   906	                    .node_positions_t
   907	                    .0
   908	                    .insert(challenge.tx_id.clone(), position);
   909	            }
   910	            // Step 7: monetary invariants (debit = credit; challenge_cases.bond
   911	            // is the 5th holding term).
   912	            assert_no_post_init_mint(tx, q)
   913	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
   914	            assert_total_ctf_conserved(
   915	                &q.economic_state_t,

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '780,805p;830,855p;960,1065p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   780	                 If you see a keystore decrypt error, check TURINGOS_AGENT_KEYSTORE_PASSWORD matches \
   781	                 the password used for the previous run.)",
   782	            );
   783	            Arc::new(Mutex::new(reg))
   784	        });
   785	
   786	    // TB-7.7 D2: last submitted tx per agent (for ProposalTelemetry.parent_tx).
   787	    // Map of agent_id → last tx_id submitted via bus.submit_typed_tx (Work or
   788	    // Verify). Root proposals leave parent_tx = None; subsequent same-agent
   789	    // proposals get the previous tx_id as parent. This is what unlocks
   790	    // citation-tree / DAG-edge analysis on chain artifacts.
   791	    let mut last_tx_by_agent: std::collections::HashMap<String, turingosv4::state::q_state::TxId> =
   792	        std::collections::HashMap::new();
   793	
   794	    // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
   795	    // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
   796	    // Default off for backward-compat baseline runs.
   797	    let mut bus = if let Some(ref bundle) = chaintape_bundle {
   798	        info!(
   799	            "[chaintape] bus wired with Sequencer + on-disk ChainTape at {:?}",
   800	            bundle.runtime_repo_path
   801	        );
   802	        TuringBus::with_sequencer(kernel, config, bundle.sequencer.clone())
   803	    } else if let Ok(wal_dir) = std::env::var("WAL_DIR") {
   804	        let problem_stem = std::path::Path::new(problem_file)
   805	            .file_stem().map(|s| s.to_string_lossy().into_owned())
   830	        }
   831	    } else {
   832	        TuringBus::new(kernel, config)
   833	    };
   834	    // TB-6 Atom 3: when chaintape mode is on, seed the on-disk chain with a
   835	    // minimal pair of envelopes — one accepted TaskOpenTx (produces an L4
   836	    // entry) and one rejected zero-stake WorkTx (produces an L4.E entry with
   837	    // synthetic_rejection_for_l4e_gate=true label per architect ruling
   838	    // 2026-05-01 § 3.6 Atom 3). The "real LLM" aspect is the parallel evaluator
   839	    // run on the smoke problem; the synthetic seed satisfies the architect's
   840	    // ≥1 L4 + ≥1 L4.E minimum without requiring per-proposal WorkTx routing
   841	    // (deferred to a future TB).
   842	    //
   843	    // TB-6 Atom 5: each synthetic envelope is also recorded as an
   844	    // AgentProposalRecord in CAS + indexed under tx_id in
   845	    // agent_audit_trail.jsonl. This demonstrates the audit-trail surface
   846	    // end-to-end on the production-binary path. Per-LLM-proposal main-loop
   847	    // routing (run_swarm "append" branch hook) remains a deferred surface
   848	    // — same pattern as Atom 3's deferral.
   849	    if let Some(ref bundle) = chaintape_bundle {
   850	        // TB-7.7 D3: when preseed is enabled, also submit a TaskOpen +
   851	        // EscrowLock for "task-{run_id}" (the SAME task_id that real
   852	        // Agent_i WorkTx submissions use in Atom 2/3 hot path). With
   853	        // pre-seeded sponsor balance, the EscrowLock will succeed,
   854	        // populating task_markets_t["task-{run_id}"].total_escrow > 0.
   855	        // Combined with pre-seeded Agent_i balance, real LLM WorkTx
   960	                    "tb7-7-sponsor",
   961	                    escrow_micro,
   962	                    parent_for_escrow,
   963	                    "tb7-7-d3-escrow",
   964	                )
   965	            };
   966	            if let Err(e) = bus.submit_typed_tx(escrow_lock).await {
   967	                error!("[chaintape/d3] preseed EscrowLock submit failed: {e}");
   968	            } else if user_task_mode {
   969	                info!(
   970	                    "[chaintape/tb10] user-mode EscrowLock {escrow_micro} micro for {real_task_id} sponsor={user_sponsor}"
   971	                );
   972	            } else {
   973	                info!("[chaintape/d3] preseed EscrowLock {escrow_micro} micro for {real_task_id}");
   974	            }
   975	        }
   976	
   977	        let task_id_str = format!("smoke-{}", run_id);
   978	        let task_open = turingosv4::runtime::adapter::make_synthetic_task_open(
   979	            &task_id_str,
   980	            "tb6-smoke-sponsor",
   981	            turingosv4::state::q_state::Hash::ZERO,
   982	            "atom3-seed",
   983	        );
   984	        let task_open_tx_id =
   985	            turingosv4::state::q_state::TxId(format!("taskopen-{}-atom3-seed", task_id_str));
   986	        if let Err(e) = bus.submit_typed_tx(task_open).await {
   987	            error!("[chaintape] synthetic TaskOpen submit failed: {e}");
   988	        } else {
   989	            info!("[chaintape] seeded synthetic TaskOpen for {}", task_id_str);
   990	        }
   991	        let bad_worktx = turingosv4::runtime::adapter::make_synthetic_worktx(
   992	            &task_id_str,
   993	            "tb6-smoke-agent",
   994	            turingosv4::state::q_state::Hash::ZERO,
   995	            0,
   996	            "atom3-l4e-synthetic-rejection",
   997	            true,
   998	        );
   999	        let bad_worktx_tx_id = turingosv4::state::q_state::TxId(format!(
  1000	            "worktx-{}-atom3-l4e-synthetic-rejection",
  1001	            task_id_str
  1002	        ));
  1003	        if let Err(e) = bus.submit_typed_tx(bad_worktx).await {
  1004	            error!("[chaintape] synthetic zero-stake WorkTx submit failed: {e}");
  1005	        } else {
  1006	            info!(
  1007	                "[chaintape] seeded synthetic zero-stake WorkTx \
  1008	                 (synthetic_rejection_for_l4e_gate=true) for {}",
  1009	                task_id_str
  1010	            );
  1011	        }
  1012	        // Mark the synthetic-seed in the evidence dir so verify_chaintape (Atom 4)
  1013	        // can distinguish synthetic-rejection from natural rejection.
  1014	        let label_path = bundle.runtime_repo_path.join("synthetic_rejection_label.json");
  1015	        let _ = std::fs::write(
  1016	            &label_path,
  1017	            format!(
  1018	                r#"{{"synthetic_rejection_for_l4e_gate": true, "run_id": "{}", "atom": "TB-6 Atom 3", "rationale": "≥1 L4.E entry seeded via zero-stake WorkTx; per architect ruling 2026-05-01 § 3.6 Atom 3"}}"#,
  1019	                run_id
  1020	            ),
  1021	        );
  1022	
  1023	        // TB-6 Atom 5: write AgentProposalRecord pairs to CAS + index for both
  1024	        // synthetic envelopes. Each record carries the architect's 9 fields
  1025	        // + logical_t. The index links L4 / L4.E tx_id → CAS record CID.
  1026	        if let Err(e) = turingosv4::runtime::agent_audit_trail::write_synthetic_seed_audit_pair(
  1027	            &bundle.cas_path,
  1028	            &bundle.runtime_repo_path,
  1029	            &run_id,
  1030	            &task_open_tx_id,
  1031	            &bad_worktx_tx_id,
  1032	        ) {
  1033	            error!("[chaintape] Atom 5 audit-trail write failed: {e}");
  1034	        } else {
  1035	            info!(
  1036	                "[chaintape] Atom 5 audit-trail records written to CAS + indexed for {}",
  1037	                task_id_str
  1038	            );
  1039	        }
  1040	
  1041	        // TB-7R Deliverable C (verdict 2026-05-01 §6.1): emit
  1042	        // `<runtime_repo>/genesis_report.json` so post-hoc audits can
  1043	        // verify the run's genesis preconditions (constitution_hash,
  1044	        // runtime_repo, cas_path, system_pubkey, agent_pubkeys path,
  1045	        // initial_balances) plus — when preseed is enabled — the
  1046	        // task_id / task_open_tx / escrow_lock_tx that established the
  1047	        // task and escrow on-chain.
  1048	        let preseed_task_id = if chaintape_preseed_enabled {
  1049	            Some(format!("task-{}", run_id))
  1050	        } else {
  1051	            None
  1052	        };
  1053	        // TB-10 Atom 1+3: tx_id suffix depends on user-mode flag (mirrors the
  1054	        // make_real_*_signed_by suffix passed in lines above).
  1055	        let user_task_mode = std::env::var("TURINGOS_USER_TASK_MODE")
  1056	            .ok()
  1057	            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
  1058	            .unwrap_or(false);
  1059	        let preseed_task_open_tx = preseed_task_id.as_ref().map(|t| {
  1060	            if user_task_mode {
  1061	                format!("taskopen-{}-tb10-user-seed", t)
  1062	            } else {
  1063	                format!("taskopen-{}-tb7-7-d3-seed", t)
  1064	            }
  1065	        });

exec
/bin/bash -lc "sed -n '1,220p' handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md.raw" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019def9a-4808-7231-a549-0b257d3f0530
--------
user
# Codex TB-14 Atom 6 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the TB-14 Atom 6
(production wire-swap + legacy CPMM excision) ship-gate dual external
audit. Independent of Gemini ship audit (parallel, architectural strategic
angle).

**Mandate**: Class 3 (production code path migration; STEP_B restricted
files `src/kernel.rs` + `src/bus.rs` touched). Per memory
`feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Round cap = 2 per
`feedback_elon_mode_policy`.

## Audit target

- Charter: `handover/tracer_bullets/TB-14_charter_2026-05-03.md`
- Atom 6 kickoff: `handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md`
- Architect ruling §5 verbatim: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- Closes: `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`
- ChainTape smoke evidence: `handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md`

```text
TB-14 Atom 5 baseline:  a9fbdf3                 (Atom 5 ship; 6/6 halt-triggers GREEN; 841/0/150)
TB-14 Atom 6 range:     a9fbdf3..38412bf        (this audit)

Atom 6 commits to anchor your reads:
  9cc40e1  TB-14 handover — LATEST.md update + Atom 6 kickoff doc (pre-atom-6 prep)
  44cd480  TB-14 Atom 6 — production wire-swap + legacy CPMM excision
  38412bf  TB-14 Atom 6 follow-up — close internal auditor F1 (dead BusResult::Invested f64)
  c291dde  TB-14 Atom 6 LATEST.md update — local-commits-shipped, external dual audit pending

HEAD (38412bf): cargo test --workspace = 821 passed / 0 failed / 150 ignored
                (delta_vs_a9fbdf3 = -20 net = 25 deleted CPMM tests + 5 new TB-14 tests)
                6/6 architect §5.7 halt-triggers GREEN
                ChainTape smoke (chain-backed, deterministic, non-LLM) PASS

Internal `auditor` subagent verdict (read-only Class 3 self-review on 44cd480):
  CHALLENGE conviction=high, recommendation=FIX-THEN-PROCEED.
  Single finding F1 (CHALLENGE, FIX-NOW): src/bus.rs:95 dead
  `BusResult::Invested { node_id, shares: f64 }` enum variant — pre-TB-9
  invest-path residual; zero call sites, zero match arms; halt-trigger #4
  only fences price_index.rs so this f64 surface in TB-14-touched bus.rs
  was unfenced. ADDRESSED by 38412bf (4-line deletion + bus.rs rehash).
  Other findings F2-F5 ACCEPTED (cosmetic / out-of-scope / process-discipline /
  pending-external).
```

## DELETIONS (closing OBS_TB_12_LEGACY_CPMM_QUARANTINE)

- `src/prediction_market.rs` (entire file — 390 LoC; BinaryMarket CPMM,
  f64 trading semantics, automatic liquidity) — DELETED
- `src/lib.rs` `pub mod prediction_market;` — DELETED
- `src/kernel.rs` market fields (`markets: HashMap<NodeId, BinaryMarket>`,
  `bounty_market: Option<BinaryMarket>`, `bounty_lp_seed: f64`) +
  methods (`create_market` / `buy_yes` / `buy_no` / `yes_price` /
  `market_ticker` / `market_ticker_full` / `open_bounty_market` /
  `bounty_yes_price` / `resolve_bounty` / `resolve_all`) + 5 legacy
  market-related kernel tests + `KernelError::Market` /
  `MarketNotFound` / `MarketExists` variants + `ResolutionResult` —
  DELETED. Kernel restored to V3L-45 pure-topology contract.
- `src/sdk/actor.rs` legacy items (`BoltzmannParams` (f64),
  `is_frontier`, `lineage_score`, legacy `boltzmann_select_parent` (f64)) +
  6 legacy tests — DELETED. `boltzmann_select_parent_v2` (Atom 5;
  integer-rational) is the sole scheduler.
- `src/sdk/snapshot.rs` legacy fields (`MarketSnapshot{f64...}`,
  `UniverseSnapshot.markets` HashMap, `UniverseSnapshot.market_ticker`
  String, dead-since-TB-9 `balances` + `portfolios` + `get_balance` +
  `get_portfolio` impls) — DELETED. All decimal-float surface excised
  under G-14.11.
- `src/bus.rs` `BusConfig.system_lp_amount: f64` + dead
  `BusResult::Invested { shares: f64 }` (F1 follow-up) — DELETED.

## WIRE-SWAPS (production code paths)

- `src/sdk/snapshot.rs`: `UniverseSnapshot` now carries integer-rational
  `price_index: BTreeMap<TxId, NodeMarketEntry>` +
  `mask_set: BTreeSet<TxId>`. Sequencer-optional empty fallback for
  legacy ledger-only mode.
- `src/bus.rs::snapshot()`: rewritten — calls
  `state::compute_price_index(&q.economic_state_t)` +
  `state::compute_mask_set(...)` from live `Sequencer::q_snapshot` when
  the bus is wired with a sequencer (chaintape mode); else empty.
  bus.rs imports TB-14 types here legitimately — halt-trigger #2 fence
  targets `src/state/sequencer.rs` `use` block only; bus.rs is the
  canonical broadcast point.
- `src/bus.rs::init`: removed `HAYEK_BOUNTY` env-gated bounty market open.
- `src/bus.rs::append_internal`: removed per-append `kernel.create_market`
  call.
- `src/bus.rs::halt_and_settle`: no longer calls `kernel.resolve_all`.
- `experiments/minif2f_v4/src/bin/evaluator.rs`: production wire-swap.
  Imports updated; `BoltzmannMaskPolicy::from_env`; `market_ticker_str`
  derived from `snap.price_index` (cross-multiplication argmax sort,
  renders `n/d`, never decimal); `prompt_balance` queried from
  `bus.sequencer.q_snapshot().balances_t` (replaces legacy
  `snap.get_balance`); Boltzmann selector now
  `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set,
  &policy, &mut rng)`.
- `src/bin/audit_dashboard.rs`: ADDITIVE — NEW §14 PriceIndex render
  section. ARCHITECT-MANDATED BANNER: literal "PRICE IS SIGNAL, NOT
  TRUTH" (architect §5.1 verbatim). 4 SG-14.6 unit tests.

## NEW + UPDATED TESTS

- NEW `tests/tb_14_chaintape_smoke.rs`: chain-backed; pattern from
  `tb_13_chaintape_smoke.rs`. Asserts (a) `verify_chaintape` 7/7 GREEN
  post-wire-swap; (b) `replayed_q.economic_state_t == live_q.economic_state_t`
  byte-equal; (c) `compute_price_index(live)` ==
  `compute_price_index(replayed)` byte-equal (FC3-N42 chaintape replay
  determinism for derived view by composition); (d) idempotency across
  5 invocations; (e) empty `node_positions_t` → empty PriceIndex.
- NEW 4 SG-14.6 unit tests in `src/bin/audit_dashboard.rs::tb14_render_tests`.
- NEW `src/kernel.rs::test_trace_golden_path_unknown_node`.
- UPDATED `tests/tb_13_legacy_cpmm_forward_fence.rs::prediction_market_legacy_quarantined`
  rewritten from "label discipline" to "absence discipline" (legacy
  file gone, no fields, no methods, no module declaration).
- UPDATED `tests/fc_alignment_conformance.rs::fc1_n6_input_universe_snapshot_via_bus`
  asserts new `price_index` + `mask_set` fields.
- UPDATED bus.rs internal tests + snapshot.rs internal test.

## Architect-mandated audit questions (CR-14.x conformance + §5.7 halt-triggers)

**Q1 (CR-14.1 / halt-trigger #1)**: Does the price signal influence
predicate gates? `src/state/sequencer.rs` body must contain ZERO
references to TB-14 price/mask types
(`compute_price_index|compute_mask_set|NodeMarketEntry|RationalPrice|BoltzmannMaskPolicy`).
Verify by inspection. Halt-trigger #1 (`tests/tb_14_halt_triggers.rs:27-53`)
is the file-level fence; you do the architectural-soundness check.

**Q2 (CR-14.2 / halt-trigger #2)**: Does the price signal affect L4/L4.E
classification? `src/state/sequencer.rs` `use` block must contain ZERO
TB-14 imports (permanent fence — even after Atom 6's bus.rs snapshot
wire-swap). Verify by inspection.

**Q3 (CR-14.3 / SG-14.3 / halt-trigger #3)**: Are masked parents still in
`tape.nodes()`? `compute_mask_set` MUST only insert into
`BTreeSet<TxId>`; `Tape` MUST never be mutated. Trace
`src/state/price_index.rs:364-429` and `src/bus.rs::snapshot`. Halt-
trigger #3 is functional; you do the production-path soundness check.

**Q4 (CR-14.4 / SG-14.8)**: Low-liquidity children cannot mask parent.
`src/state/price_index.rs:402-407` enforces
`child.liquidity_depth < policy.min_liquidity → continue`. Verify the
threshold rule is right (BoltzmannMaskPolicy::default() has
`min_liquidity = 1 Coin`); challenge if you see edge cases (overflow,
saturation, signed-vs-unsigned).

**Q5 (CR-14.5 / SG-14.7 / halt-trigger #6)**: Open challenges block
masking. `src/state/price_index.rs:377-382, 410-412` indexes
`ChallengeStatus::Open` and skips. Verify the index is correctly built
from `econ.challenge_cases_t` and the skip is unconditional (no
override conditions).

**Q6 (CR-14.6 — Goodhart shield)**: `NodeMarketEntry`
(`src/state/price_index.rs:97-109`) carries 10 fields (node_id,
task_id, event_id, long_interest, short_interest, yes_share_depth,
no_share_depth, price_yes, price_no, liquidity_depth) — NONE expose
predicate content. Dashboard §14 render
(`src/bin/audit_dashboard.rs:1500-1570`) renders only these fields.
Verify no leak path.

**Q7 (G-14.11 / charter §5.6)**: No f64 in TB-14 module surface.
`src/state/price_index.rs` is fenced by halt-trigger #4 runtime fs
read. After F1 fix, verify `src/sdk/snapshot.rs`, `src/bus.rs`,
`src/bin/audit_dashboard.rs` §14 render block, `src/sdk/actor.rs` v2
span are all clean. EXPECTED OUT-OF-SCOPE: `experiments/minif2f_v4/src/bin/evaluator.rs`
`prompt_balance: f64` (this is the prompt.rs render contract;
prompt.rs is NOT a TB-14 module surface — accept this if you agree;
flag if you disagree).

**Q8 (Art.0.2 replay determinism)**:
`tests/tb_14_chaintape_smoke.rs:307-348` claims
`compute_price_index(live) == compute_price_index(replayed)`
byte-equal + idempotent across 5 invocations + empty pre-condition
honest. Verify the assertions actually measure what they claim.

**Q9 (charter §5.6 forbidden)**: No market trading. No price-based
settlement. No parent deletion. No AMM. No DPMM. No price-as-oracle
language. Walk the diff (`git diff a9fbdf3..38412bf`) and confirm
zero introductions of any of these.

## Implementation-paranoid scrutiny (RQ1-RQ8)

**RQ1 — bus.snapshot sequencer-optional empty fallback**: when
`bus.sequencer == None` (legacy WAL-only smoke tests), price_index +
mask_set return empty. Could a downstream consumer (evaluator,
dashboard) misread "empty" as "no positions" when the truth is
"sequencer not wired"? Trace evaluator.rs:1320-1395 zero-tick alarm
and confirm the diagnostic distinguishes (or that the conflation is
benign).

**RQ2 — kernel-purity post-deletion**: `src/kernel.rs` post-Atom-6 has
~140 lines, 5 tests, only `Tape`/`Node`/`NodeId`/`KernelError` in
scope. Verify zero domain strings (lean / tactic / theorem / proof /
mathlib / sorry). V3L-45 contract restored?

**RQ3 — replay determinism via composition**: `compute_price_index` is
a pure function over `EconomicState`. The chaintape smoke verifies
`live.economic_state_t == replayed.economic_state_t` byte-equal (TB-13
already proven for that invariant) and then asserts
`compute_price_index(live) == compute_price_index(replayed)`. Is
this composition argument sound, or are there hidden non-determinism
sources (BTreeMap iteration order, hash randomness, etc.)?

**RQ4 — boltzmann_select_parent_v2 production wire-up**: evaluator.rs
calls `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set,
&policy, &mut boltz_rng).map(|tx| tx.0)`. Is the `.map(|tx| tx.0)`
unwrapping correct (TxId(String) → String)? Does it lose information?
Trace the downstream parent_id consumer.


exec
/bin/bash -lc 'tail -n 180 handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md.raw' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:

HEAD (38412bf): cargo test --workspace = 821 passed / 0 failed / 150 ignored
                (delta_vs_a9fbdf3 = -20 net = 25 deleted CPMM tests + 5 new TB-14 tests)
                6/6 architect §5.7 halt-triggers GREEN
                ChainTape smoke (chain-backed, deterministic, non-LLM) PASS

Internal `auditor` subagent verdict (read-only Class 3 self-review on 44cd480):
  CHALLENGE conviction=high, recommendation=FIX-THEN-PROCEED.
  Single finding F1 (CHALLENGE, FIX-NOW): src/bus.rs:95 dead
  `BusResult::Invested { node_id, shares: f64 }` enum variant — pre-TB-9
  invest-path residual; zero call sites, zero match arms; halt-trigger #4
  only fences price_index.rs so this f64 surface in TB-14-touched bus.rs
  was unfenced. ADDRESSED by 38412bf (4-line deletion + bus.rs rehash).
  Other findings F2-F5 ACCEPTED (cosmetic / out-of-scope / process-discipline /
  pending-external).
```

## DELETIONS (closing OBS_TB_12_LEGACY_CPMM_QUARANTINE)

- `src/prediction_market.rs` (entire file — 390 LoC; BinaryMarket CPMM,
  f64 trading semantics, automatic liquidity) — DELETED
- `src/lib.rs` `pub mod prediction_market;` — DELETED
- `src/kernel.rs` market fields (`markets: HashMap<NodeId, BinaryMarket>`,
  `bounty_market: Option<BinaryMarket>`, `bounty_lp_seed: f64`) +
  methods (`create_market` / `buy_yes` / `buy_no` / `yes_price` /
  `market_ticker` / `market_ticker_full` / `open_bounty_market` /
  `bounty_yes_price` / `resolve_bounty` / `resolve_all`) + 5 legacy
  market-related kernel tests + `KernelError::Market` /
  `MarketNotFound` / `MarketExists` variants + `ResolutionResult` —
  DELETED. Kernel restored to V3L-45 pure-topology contract.
- `src/sdk/actor.rs` legacy items (`BoltzmannParams` (f64),
  `is_frontier`, `lineage_score`, legacy `boltzmann_select_parent` (f64)) +
  6 legacy tests — DELETED. `boltzmann_select_parent_v2` (Atom 5;
  integer-rational) is the sole scheduler.
- `src/sdk/snapshot.rs` legacy fields (`MarketSnapshot{f64...}`,
  `UniverseSnapshot.markets` HashMap, `UniverseSnapshot.market_ticker`
  String, dead-since-TB-9 `balances` + `portfolios` + `get_balance` +
  `get_portfolio` impls) — DELETED. All decimal-float surface excised
  under G-14.11.
- `src/bus.rs` `BusConfig.system_lp_amount: f64` + dead
  `BusResult::Invested { shares: f64 }` (F1 follow-up) — DELETED.

## WIRE-SWAPS (production code paths)

- `src/sdk/snapshot.rs`: `UniverseSnapshot` now carries integer-rational
  `price_index: BTreeMap<TxId, NodeMarketEntry>` +
  `mask_set: BTreeSet<TxId>`. Sequencer-optional empty fallback for
  legacy ledger-only mode.
- `src/bus.rs::snapshot()`: rewritten — calls
  `state::compute_price_index(&q.economic_state_t)` +
  `state::compute_mask_set(...)` from live `Sequencer::q_snapshot` when
  the bus is wired with a sequencer (chaintape mode); else empty.
  bus.rs imports TB-14 types here legitimately — halt-trigger #2 fence
  targets `src/state/sequencer.rs` `use` block only; bus.rs is the
  canonical broadcast point.
- `src/bus.rs::init`: removed `HAYEK_BOUNTY` env-gated bounty market open.
- `src/bus.rs::append_internal`: removed per-append `kernel.create_market`
  call.
- `src/bus.rs::halt_and_settle`: no longer calls `kernel.resolve_all`.
- `experiments/minif2f_v4/src/bin/evaluator.rs`: production wire-swap.
  Imports updated; `BoltzmannMaskPolicy::from_env`; `market_ticker_str`
  derived from `snap.price_index` (cross-multiplication argmax sort,
  renders `n/d`, never decimal); `prompt_balance` queried from
  `bus.sequencer.q_snapshot().balances_t` (replaces legacy
  `snap.get_balance`); Boltzmann selector now
  `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set,
  &policy, &mut rng)`.
- `src/bin/audit_dashboard.rs`: ADDITIVE — NEW §14 PriceIndex render
  section. ARCHITECT-MANDATED BANNER: literal "PRICE IS SIGNAL, NOT
  TRUTH" (architect §5.1 verbatim). 4 SG-14.6 unit tests.

## NEW + UPDATED TESTS

- NEW `tests/tb_14_chaintape_smoke.rs`: chain-backed; pattern from
  `tb_13_chaintape_smoke.rs`. Asserts (a) `verify_chaintape` 7/7 GREEN
  post-wire-swap; (b) `replayed_q.economic_state_t == live_q.economic_state_t`
  byte-equal; (c) `compute_price_index(live)` ==
  `compute_price_index(replayed)` byte-equal (FC3-N42 chaintape replay
  determinism for derived view by composition); (d) idempotency across
  5 invocations; (e) empty `node_positions_t` → empty PriceIndex.
- NEW 4 SG-14.6 unit tests in `src/bin/audit_dashboard.rs::tb14_render_tests`.
- NEW `src/kernel.rs::test_trace_golden_path_unknown_node`.
- UPDATED `tests/tb_13_legacy_cpmm_forward_fence.rs::prediction_market_legacy_quarantined`
  rewritten from "label discipline" to "absence discipline" (legacy
  file gone, no fields, no methods, no module declaration).
- UPDATED `tests/fc_alignment_conformance.rs::fc1_n6_input_universe_snapshot_via_bus`
  asserts new `price_index` + `mask_set` fields.
- UPDATED bus.rs internal tests + snapshot.rs internal test.

## Architect-mandated audit questions (CR-14.x conformance + §5.7 halt-triggers)

**Q1 (CR-14.1 / halt-trigger #1)**: Does the price signal influence
predicate gates? `src/state/sequencer.rs` body must contain ZERO
references to TB-14 price/mask types
(`compute_price_index|compute_mask_set|NodeMarketEntry|RationalPrice|BoltzmannMaskPolicy`).
Verify by inspection. Halt-trigger #1 (`tests/tb_14_halt_triggers.rs:27-53`)
is the file-level fence; you do the architectural-soundness check.

**Q2 (CR-14.2 / halt-trigger #2)**: Does the price signal affect L4/L4.E
classification? `src/state/sequencer.rs` `use` block must contain ZERO
TB-14 imports (permanent fence — even after Atom 6's bus.rs snapshot
wire-swap). Verify by inspection.

**Q3 (CR-14.3 / SG-14.3 / halt-trigger #3)**: Are masked parents still in
`tape.nodes()`? `compute_mask_set` MUST only insert into
`BTreeSet<TxId>`; `Tape` MUST never be mutated. Trace
`src/state/price_index.rs:364-429` and `src/bus.rs::snapshot`. Halt-
trigger #3 is functional; you do the production-path soundness check.

**Q4 (CR-14.4 / SG-14.8)**: Low-liquidity children cannot mask parent.
`src/state/price_index.rs:402-407` enforces
`child.liquidity_depth < policy.min_liquidity → continue`. Verify the
threshold rule is right (BoltzmannMaskPolicy::default() has
`min_liquidity = 1 Coin`); challenge if you see edge cases (overflow,
saturation, signed-vs-unsigned).

**Q5 (CR-14.5 / SG-14.7 / halt-trigger #6)**: Open challenges block
masking. `src/state/price_index.rs:377-382, 410-412` indexes
`ChallengeStatus::Open` and skips. Verify the index is correctly built
from `econ.challenge_cases_t` and the skip is unconditional (no
override conditions).

**Q6 (CR-14.6 — Goodhart shield)**: `NodeMarketEntry`
(`src/state/price_index.rs:97-109`) carries 10 fields (node_id,
task_id, event_id, long_interest, short_interest, yes_share_depth,
no_share_depth, price_yes, price_no, liquidity_depth) — NONE expose
predicate content. Dashboard §14 render
(`src/bin/audit_dashboard.rs:1500-1570`) renders only these fields.
Verify no leak path.

**Q7 (G-14.11 / charter §5.6)**: No f64 in TB-14 module surface.
`src/state/price_index.rs` is fenced by halt-trigger #4 runtime fs
read. After F1 fix, verify `src/sdk/snapshot.rs`, `src/bus.rs`,
`src/bin/audit_dashboard.rs` §14 render block, `src/sdk/actor.rs` v2
span are all clean. EXPECTED OUT-OF-SCOPE: `experiments/minif2f_v4/src/bin/evaluator.rs`
`prompt_balance: f64` (this is the prompt.rs render contract;
prompt.rs is NOT a TB-14 module surface — accept this if you agree;
flag if you disagree).

**Q8 (Art.0.2 replay determinism)**:
`tests/tb_14_chaintape_smoke.rs:307-348` claims
`compute_price_index(live) == compute_price_index(replayed)`
byte-equal + idempotent across 5 invocations + empty pre-condition
honest. Verify the assertions actually measure what they claim.

**Q9 (charter §5.6 forbidden)**: No market trading. No price-based
settlement. No parent deletion. No AMM. No DPMM. No price-as-oracle
language. Walk the diff (`git diff a9fbdf3..38412bf`) and confirm
zero introductions of any of these.

## Implementation-paranoid scrutiny (RQ1-RQ8)

**RQ1 — bus.snapshot sequencer-optional empty fallback**: when
`bus.sequencer == None` (legacy WAL-only smoke tests), price_index +
mask_set return empty. Could a downstream consumer (evaluator,
dashboard) misread "empty" as "no positions" when the truth is
"sequencer not wired"? Trace evaluator.rs:1320-1395 zero-tick alarm
and confirm the diagnostic distinguishes (or that the conflation is
benign).

**RQ2 — kernel-purity post-deletion**: `src/kernel.rs` post-Atom-6 has
~140 lines, 5 tests, only `Tape`/`Node`/`NodeId`/`KernelError` in
scope. Verify zero domain strings (lean / tactic / theorem / proof /
mathlib / sorry). V3L-45 contract restored?

**RQ3 — replay determinism via composition**: `compute_price_index` is
a pure function over `EconomicState`. The chaintape smoke verifies
`live.economic_state_t == replayed.economic_state_t` byte-equal (TB-13
already proven for that invariant) and then asserts
`compute_price_index(live) == compute_price_index(replayed)`. Is
this composition argument sound, or are there hidden non-determinism
sources (BTreeMap iteration order, hash randomness, etc.)?

**RQ4 — boltzmann_select_parent_v2 production wire-up**: evaluator.rs
calls `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set,
&policy, &mut boltz_rng).map(|tx| tx.0)`. Is the `.map(|tx| tx.0)`
unwrapping correct (TxId(String) → String)? Does it lose information?
Trace the downstream parent_id consumer.



exec
/bin/bash -lc "nl -ba tests/tb_14_chaintape_smoke.rs | sed -n '300,352p;380,420p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	    );
   301	
   302	    // ── Atom 6 invariant 2: derived-view replay determinism ────────────
   303	    //
   304	    // FR-14.x / FC3-N42: compute_price_index over byte-equal-replayed
   305	    // EconomicState yields byte-equal BTreeMap<TxId, NodeMarketEntry>.
   306	    // The TB-14 derived view is replay-deterministic by composition.
   307	    let replayed_q = manual_replay_from_disk(&cfg.runtime_repo_path, &cfg.cas_path);
   308	    assert_eq!(
   309	        replayed_q.economic_state_t, live_q.economic_state_t,
   310	        "Atom 6 must NOT regress EconomicState byte-equality across replay"
   311	    );
   312	
   313	    let live_price_index = compute_price_index(&live_q.economic_state_t);
   314	    let replayed_price_index = compute_price_index(&replayed_q.economic_state_t);
   315	    assert_eq!(
   316	        live_price_index, replayed_price_index,
   317	        "TB-14 FC3-N42: compute_price_index must be byte-equal across live vs \
   318	         replay (Art.0.2 derived-view determinism)"
   319	    );
   320	
   321	    // ── Atom 6 invariant 3: compute_price_index idempotent ─────────────
   322	    //
   323	    // Calling compute_price_index N times on the same EconomicState must
   324	    // produce N byte-equal BTreeMaps (Art.0.2 pure-function determinism).
   325	    for _ in 0..5 {
   326	        assert_eq!(
   327	            compute_price_index(&live_q.economic_state_t),
   328	            live_price_index,
   329	            "TB-14 FC3-N42: compute_price_index must be idempotent"
   330	        );
   331	    }
   332	
   333	    // ── Atom 6 invariant 4: empty node_positions_t → empty PriceIndex ──
   334	    //
   335	    // FR-14.3 / halt-trigger #5 extended: this smoke's CompleteSet flow
   336	    // does NOT mutate node_positions_t (TB-12 substrate untouched here),
   337	    // so the resulting PriceIndex is empty by construction. This pins
   338	    // down the invariant at the chaintape integration layer.
   339	    assert!(
   340	        live_q.economic_state_t.node_positions_t.0.is_empty(),
   341	        "TB-14 chaintape smoke pre-condition: node_positions_t empty after \
   342	         CompleteSet-only flow (TB-12 substrate untouched)"
   343	    );
   344	    assert!(
   345	        live_price_index.is_empty(),
   346	        "TB-14 FR-14.3 + halt-trigger #5: empty node_positions_t → empty \
   347	         PriceIndex (BTreeMap)"
   348	    );
   349	
   350	    // ── Persist evidence to canonical handover dir (best-effort) ───────
   351	    let evidence_dir = std::path::Path::new(
   352	        "handover/evidence/tb_14_chaintape_smoke_2026-05-03",
   380	            );
   381	        }
   382	        let _ = std::fs::write(
   383	            evidence_dir.join("README.md"),
   384	            format!(
   385	                "# TB-14 Atom 6 — ChainTape smoke (post-wire-swap regression)\n\
   386	                 \n\
   387	                 **Date**: 2026-05-03\n\
   388	                 **Source**: `tests/tb_14_chaintape_smoke.rs::tb_14_atom_6_post_wire_swap_chaintape_replay_preserves_price_index_determinism`\n\
   389	                 **Trigger**: TB-14 Atom 6 production wire-swap (excise legacy CPMM scaffolding; reroute bus snapshot price-signal surface through `compute_price_index` + `compute_mask_set` integer-rational derived views).\n\
   390	                 \n\
   391	                 ## Headline\n\
   392	                 \n\
   393	                 - L4 entries: {l4} (mint + redeem)\n\
   394	                 - L4.E entries: {l4e}\n\
   395	                 - All 7 ReplayReport indicators GREEN: {all_pass}\n\
   396	                 - Live `state_root_t`: `{live_root}`\n\
   397	                 - Replay `final_state_root_hex`: `{replay_root}`\n\
   398	                 - `live.economic_state_t == replayed.economic_state_t`: byte-equal\n\
   399	                 - `compute_price_index(live)` == `compute_price_index(replayed)`: byte-equal\n\
   400	                 - `compute_price_index` idempotent across 5 invocations: ✓\n\
   401	                 - Empty `node_positions_t` → empty PriceIndex BTreeMap: ✓\n\
   402	                 \n\
   403	                 ## What this evidence proves (Atom 6 specific)\n\
   404	                 \n\
   405	                 1. The Atom 6 production wire-swap (excised `prediction_market.rs`, `kernel.markets`, `BoltzmannParams`, legacy f64 `boltzmann_select_parent`; rewired `bus.snapshot` to derive `price_index` + `mask_set` from `Sequencer::q_snapshot`'s `EconomicState`) does NOT regress chain-replay determinism.\n\
   406	                 2. `verify_chaintape` reconstructs a `QState` from persisted artifacts whose `final_state_root_hex` matches live `state_root_t` (Art.0.2 Tape Canonical preserved across the wire-swap).\n\
   407	                 3. The TB-14 derived view (`compute_price_index(econ)`) is replay-deterministic by composition: pure function over a byte-equal-replayed `EconomicState` yields byte-equal `BTreeMap<TxId, NodeMarketEntry>` (FR-14.x / FC3-N42 chaintape integration evidence).\n\
   408	                 4. `compute_price_index` is idempotent across N calls (Art.0.2 pure-function determinism at the derived-view layer).\n\
   409	                 5. Empty `node_positions_t` → empty PriceIndex (FR-14.3 / halt-trigger #5 extended at the chaintape integration layer).\n\
   410	                 \n\
   411	                 ## What is NOT in scope here\n\
   412	                 \n\
   413	                 - **Non-empty PriceIndex via WorkTx**: this smoke uses CompleteSet flow only (TB-13 substrate). A WorkTx-creates-NodePosition flow (TB-12 substrate that produces non-empty PriceIndex) is covered by the in-memory unit tests at `tests/tb_14_price_index.rs` + halt-triggers + `src/state/price_index.rs` inline tests. Per `feedback_chaintape_externalized_proposal`, the chaintape smoke records what the system externalizes via `submit_typed_tx` end-to-end; the per-position aggregation is pure-function-tested elsewhere.\n\
   414	                 - **`mask_set` via Tape children**: `compute_mask_set` requires a Tape; this smoke does not exercise mask computation (covered by `tests/tb_14_mask_set.rs` + halt-triggers #3 / #6).\n\
   415	                 - **Boltzmann v2 selector**: covered by inline tests in `src/sdk/actor.rs::tests::v2_*`. Production wire-up at `experiments/minif2f_v4/src/bin/evaluator.rs:~1559` is exercised by the `--smoke` / `--half` evaluator runs.\n",
   416	                l4 = report.l4_entries,
   417	                l4e = report.l4e_entries,
   418	                all_pass = report.all_indicators_pass(),
   419	                live_root = live_state_root_hex,
   420	                replay_root = final_state_root_hex,

exec
/bin/bash -lc "rg -n \"fn q_snapshot|pub fn q_snapshot|struct Sequencer|RwLock\" src/state/sequencer.rs | head -20 && nl -ba src/state/sequencer.rs | sed -n '2920,2975p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
20:use std::sync::{Arc, RwLock};
2269:pub struct Sequencer {
2282:    cas: Arc<RwLock<CasStore>>,
2285:    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
2287:    /// `Arc<RwLock<...>>` shape (P0-1 r2: `append_rejected` is `&mut self`).
2290:    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
2322:    q: RwLock<QState>,
2345:        cas: Arc<RwLock<CasStore>>,
2348:        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
2349:        rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
2371:            q: RwLock::new(initial_q),
3114:    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
3155:        Arc<RwLock<RejectionEvidenceWriter>>,
3158:        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
3163:        let writer: Arc<RwLock<dyn LedgerWriter>> =
3164:            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
3165:        let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
3481:        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
3483:        let writer: Arc<RwLock<dyn LedgerWriter>> =
3484:            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
  2920	
  2921	        let diag_bytes = err.to_string().into_bytes();
  2922	        let raw_diagnostic_cid = {
  2923	            let mut cas_w = self
  2924	                .cas
  2925	                .write()
  2926	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  2927	            Some(cas_w.put(
  2928	                &diag_bytes,
  2929	                ObjectType::Generic,
  2930	                &creator,
  2931	                rejection_logical_t,
  2932	                Some("TransitionError.display.v1".to_string()),
  2933	            )?)
  2934	        };
  2935	
  2936	        let agent_id = tx
  2937	            .submitter_id()
  2938	            .unwrap_or_else(|| AgentId(SYSTEM_AGENT_ID_STR.to_string()));
  2939	
  2940	        {
  2941	            let mut writer_w = self
  2942	                .rejection_writer
  2943	                .write()
  2944	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  2945	            writer_w.append_rejected(
  2946	                submit_id,
  2947	                q_snapshot.state_root_t,
  2948	                agent_id,
  2949	                tx.tx_kind(),
  2950	                tx_payload_cid,
  2951	                rejection_class_for(err),
  2952	                raw_diagnostic_cid,
  2953	                public_summary_for(err),
  2954	            );
  2955	        }
  2956	        Ok(())
  2957	    }
  2958	
  2959	    /// TRACE_MATRIX FC3-S3: L4 sequencer per-tx critical section.
  2960	    ///
  2961	    /// Pure transition + CAS put + sign + commit + Q_t mutation. See spec § 3
  2962	    /// stages 1-9. TB-2 Atom 2 changes the input type from `TypedTx` to
  2963	    /// `SubmissionEnvelope` so `submit_id` travels in (charter §1 / P1:6);
  2964	    /// the apply pipeline itself is unchanged in Atom 2.
  2965	    ///
  2966	    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
  2967	    /// **only on commit success** — the original spec § 3 stage-4
  2968	    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
  2969	    /// failure (sign / commit) left `next_logical_t` advanced past a
  2970	    /// logical_t that was never written to the ledger. The next accepted
  2971	    /// tx would then be assigned a logical_t the writer rejects forever
  2972	    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
  2973	    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
  2974	    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
  2975	    /// can be upgraded to a `compare_exchange` reservation pattern.

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '3108,3122p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  3108	        }
  3109	
  3110	        Ok(entry)
  3111	    }
  3112	
  3113	    /// Read-only accessor (testing + CO1.7.5+ wiring).
  3114	    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
  3115	        self.q
  3116	            .read()
  3117	            .map(|g| g.clone())
  3118	            .map_err(|_| ApplyError::QStateLockPoisoned)
  3119	    }
  3120	
  3121	    pub fn next_submit_id_peek(&self) -> u64 {
  3122	        self.next_submit_id.load(Ordering::SeqCst)

exec
/bin/bash -lc "nl -ba src/lib.rs | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	pub mod boot;
     2	pub mod ledger;
     3	pub mod kernel;
     4	pub mod bus;
     5	pub mod sdk;
     6	pub mod drivers;
     7	pub mod wal;
     8	pub mod economy;
     9	pub mod top_white;
    10	pub mod bottom_white;
    11	pub mod state;
    12	/// TRACE_MATRIX FC3-N1: production-path ChainTape runtime — connects evaluator binary to Sequencer + Git2LedgerWriter so LLM-driven runs produce on-disk LedgerEntry chain (TB-6).
    13	pub mod runtime;

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '930,1005p;1458,1480p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   930	        run_facts,
   931	        per_agent,
   932	        proposal_flow,
   933	        branch_lineage,
   934	        golden_path,
   935	        cross_checks,
   936	        claims: claims_in_progress,
   937	        user_tasks: user_tasks_in_progress,
   938	        exhausted_runs: exhausted_runs_in_progress,
   939	        expired_tasks: expired_tasks_in_progress,
   940	        bankrupt_tasks: bankrupt_tasks_in_progress,
   941	        price_index: price_index_from_exposures(&exposures_in_progress),
   942	        exposures: exposures_in_progress,
   943	    })
   944	}
   945	
   946	/// TRACE_MATRIX TB-14 Atom 6 (FC3-N42; architect §5.1 + §5.5 SG-14.6):
   947	/// rebuild a synthetic `EconomicState` from the dashboard's `exposures`
   948	/// vec and call the canonical `state::compute_price_index` over it.
   949	///
   950	/// **Why synthetic**: the dashboard does not run a full `replay_full_transition`
   951	/// to produce a final `QState`; it walks the L4 chain forward to accumulate
   952	/// audit rows. The exposures vec already carries `(node_id, side, amount_micro,
   953	/// owner, task_id, source_tx, opened_at_round)` for every accepted FirstLong
   954	/// (WorkTx) and ChallengeShort (ChallengeTx) — exactly the inputs `compute_price_index`
   955	/// needs. By going through `compute_price_index` rather than re-implementing
   956	/// the long/short aggregation here, the dashboard's price view is canonically
   957	/// identical to the bus snapshot's price view (architect §5.1 "no second
   958	/// source-of-truth"; charter §7 auto-resolution A).
   959	///
   960	/// The `kind` field is irrelevant to `compute_price_index` (which reads only
   961	/// `side` + `amount` + `node_id` + `task_id`); we map by side as a placeholder.
   962	/// `conditional_share_balances_t` is left empty, so the resulting
   963	/// `NodeMarketEntry.yes_share_depth` / `no_share_depth` are zero — TB-14 v0
   964	/// derives price from `node_positions_t` only (FR-14.1 / FR-14.2); share
   965	/// depths are reported but not used in the price computation.
   966	fn price_index_from_exposures(
   967	    exposures: &[ExposureRecordRow],
   968	) -> BTreeMap<TxId, NodeMarketEntry> {
   969	    let mut econ = EconomicState::default();
   970	    for row in exposures {
   971	        let (side, kind) = match row.side.as_str() {
   972	            "Long" => (PositionSide::Long, PositionKind::FirstLong),
   973	            "Short" => (PositionSide::Short, PositionKind::ChallengeShort),
   974	            _ => continue, // unknown side string — drop defensively
   975	        };
   976	        let position = NodePosition {
   977	            position_id: TxId(row.position_id.clone()),
   978	            node_id: TxId(row.node_id.clone()),
   979	            task_id: TaskId(row.task_id.clone()),
   980	            owner: AgentId(row.owner.clone()),
   981	            side,
   982	            kind,
   983	            amount: MicroCoin::from_micro_units(row.amount_micro),
   984	            source_tx: TxId(row.source_tx.clone()),
   985	            opened_at_round: row.opened_at_round,
   986	        };
   987	        econ.node_positions_t
   988	            .0
   989	            .insert(position.position_id.clone(), position);
   990	    }
   991	    compute_price_index(&econ)
   992	}
   993	
   994	fn render_text(r: &DashboardReport) -> String {
   995	    let mut s = String::new();
   996	    s.push_str("=================================================================\n");
   997	    s.push_str(&format!(" TB-8 Audit Dashboard — run_id={} epoch={}\n", r.run_id, r.epoch));
   998	    s.push_str("=================================================================\n\n");
   999	
  1000	    // §1 Run metadata
  1001	    s.push_str("§1 Run metadata\n");
  1002	    s.push_str("---------------\n");
  1003	    s.push_str(&format!(
  1004	        "  head_commit_oid: {}\n",
  1005	        r.chain.head_commit_oid_hex.as_deref().unwrap_or("(empty chain)")
  1458	                "    {:<18} | {:<21} | {:>11} | {}\n",
  1459	                trunc(&bk.task_id, 18),
  1460	                trunc(&bk.bankruptcy_reason, 21),
  1461	                bk.failed_run_count,
  1462	                cap_short,
  1463	            ));
  1464	        }
  1465	    }
  1466	
  1467	    s.push('\n');
  1468	    s.push_str("  Architect mandate (§6.2 ruling 2026-05-02) ✓:\n");
  1469	    s.push_str("    O(1) chain cost / O(N) auditability — failure evidence anchored on L4\n");
  1470	    s.push_str("    via system-emitted system_signature; raw log requires audit-role access\n");
  1471	    s.push_str("    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).\n");
  1472	
  1473	    // §13 TB-12 Node exposure records (architect 2026-05-03 ruling §3 + §10).
  1474	    s.push_str(&render_section_13(&r.exposures));
  1475	
  1476	    // §14 TB-14 PriceIndex (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6).
  1477	    s.push_str(&render_section_14(&r.price_index));
  1478	    s
  1479	}
  1480	

exec
/bin/bash -lc 'test ! -e src/prediction_market.rs; echo prediction_market_absent:$?' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
prediction_market_absent:0

exec
/bin/bash -lc 'rg -n "lean|tactic|theorem|proof|mathlib|sorry" src/kernel.rs' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc 'rg -n "compute_price_index|compute_mask_set|NodeMarketEntry|RationalPrice|BoltzmannMaskPolicy" src/state/sequencer.rs' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
I have enough evidence for a ship-gate verdict. The blocking issue is in the production id namespace between canonical WorkTx price entries and the shadow Tape used by bus.snapshot and bus.append.
  ✓ Load audit authority docs and scope
  ✓ Inspect Atom 6 code paths and tests
  → Classify findings and draft audit
  • Save audit artifact if filesystem permits
codex
I’m going to write the audit artifact now. The write may be rejected by the read-only sandbox; if that happens I’ll keep the same content in the final response summary and call out the save failure.
2026-05-03T20:55:14.099460Z ERROR codex_core::tools::router: error=patch rejected: writing is blocked by read-only sandbox; rejected by user approval settings
The audit artifact write was attempted via apply_patch and rejected by the read-only sandbox. I will report the verdict and the save blocker in the final answer.
  ✓ Load audit authority docs and scope
  ✓ Inspect Atom 6 code paths and tests
  ✓ Classify findings and draft audit
  ✓ Save audit artifact if filesystem permits
codex
Audit completed. I could not save `handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md`: the workspace is read-only and `apply_patch` was rejected by sandbox policy.

Primary result: **VETO**, conviction high, recommendation **REDESIGN before ship**.

Blocking finding: Atom 6 mixes canonical `WorkTx` ids with shadow `kernel.tape` ids. Accepted `NodePosition.node_id` is `work.tx_id` in [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:631), while `bus.snapshot()` computes `mask_set` against `self.kernel.tape` in [bus.rs](/home/zephryj/projects/turingosv4/src/bus.rs:511). The evaluator then unwraps the canonical `TxId` and passes it as a shadow tape parent in [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1612) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1753). But shadow tape nodes are generated as `tx_{count}_by_{author}` in [bus.rs](/home/zephryj/projects/turingosv4/src/bus.rs:295), and dangling parent citations are rejected in [ledger.rs](/home/zephryj/projects/turingosv4/src/ledger.rs:57). Result: production masking cannot see real parent/child edges, and once `price_index` is non-empty the selected parent can break shadow append.

Secondary CHALLENGE: `BoltzmannMaskPolicy::from_env()` accepts nonsensical production values. Negative `BOLTZMANN_MIN_LIQUIDITY_MICRO` flows into `MicroCoin` at [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:306), weakening the low-liquidity guard at [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:402). Zero price margin is also accepted at [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:310), interacting badly with `saturating_sub` in [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:67).

Cleared by inspection: Q1, Q2, Q5, Q6, Q7, Q9. Q8’s assertions measure the empty replay case correctly, but that smoke is insufficient for Class 3 because it explicitly does not exercise non-empty `PriceIndex` or `mask_set`; that is the exact gap hiding the VETO.

## VERDICT: VETO
- RQ4/RQ8/Q3 VETO: production wire-up mixes canonical WorkTx `TxId` PriceIndex entries with shadow `kernel.tape` node ids, making `mask_set` non-functional over real production edges and causing selected parents to become dangling shadow citations.
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
tokens used
186,570
Audit completed. I could not save `handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md`: the workspace is read-only and `apply_patch` was rejected by sandbox policy.

Primary result: **VETO**, conviction high, recommendation **REDESIGN before ship**.

Blocking finding: Atom 6 mixes canonical `WorkTx` ids with shadow `kernel.tape` ids. Accepted `NodePosition.node_id` is `work.tx_id` in [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:631), while `bus.snapshot()` computes `mask_set` against `self.kernel.tape` in [bus.rs](/home/zephryj/projects/turingosv4/src/bus.rs:511). The evaluator then unwraps the canonical `TxId` and passes it as a shadow tape parent in [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1612) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1753). But shadow tape nodes are generated as `tx_{count}_by_{author}` in [bus.rs](/home/zephryj/projects/turingosv4/src/bus.rs:295), and dangling parent citations are rejected in [ledger.rs](/home/zephryj/projects/turingosv4/src/ledger.rs:57). Result: production masking cannot see real parent/child edges, and once `price_index` is non-empty the selected parent can break shadow append.

Secondary CHALLENGE: `BoltzmannMaskPolicy::from_env()` accepts nonsensical production values. Negative `BOLTZMANN_MIN_LIQUIDITY_MICRO` flows into `MicroCoin` at [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:306), weakening the low-liquidity guard at [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:402). Zero price margin is also accepted at [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:310), interacting badly with `saturating_sub` in [price_index.rs](/home/zephryj/projects/turingosv4/src/state/price_index.rs:67).

Cleared by inspection: Q1, Q2, Q5, Q6, Q7, Q9. Q8’s assertions measure the empty replay case correctly, but that smoke is insufficient for Class 3 because it explicitly does not exercise non-empty `PriceIndex` or `mask_set`; that is the exact gap hiding the VETO.

## VERDICT: VETO
- RQ4/RQ8/Q3 VETO: production wire-up mixes canonical WorkTx `TxId` PriceIndex entries with shadow `kernel.tape` node ids, making `mask_set` non-functional over real production edges and causing selected parents to become dangling shadow citations.
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
