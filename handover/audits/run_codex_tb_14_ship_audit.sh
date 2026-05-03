#!/usr/bin/env bash
# Codex TB-14 ship audit — Class 3 (production wire-swap; STEP_B restricted
# files src/kernel.rs + src/bus.rs touched). Implementation-paranoid angle.
# Independent of Gemini ship audit (parallel, architectural strategic angle).
# Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
ROUND="${TB14_AUDIT_ROUND:-R1}"
OUT="${ROOT}/handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_${ROUND}.md"
TMP_PROMPT="$(mktemp /tmp/tb14_codex_ship.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -e "$OUT" ]; then
  echo "[codex tb-14] error: $OUT already exists; refusing to overwrite" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
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

BRIEF_EOF

echo "  Codex audit prompt prepared at: $TMP_PROMPT" >&2
echo "  Output target: $OUT" >&2
echo "  Round: $ROUND" >&2
echo "  Invoking codex exec..." >&2

cat "$TMP_PROMPT" | codex exec --skip-git-repo-check --sandbox read-only --color never - > "$OUT.raw" 2>&1
EXIT=$?

if [ $EXIT -ne 0 ]; then
  echo "  codex exec returned exit code $EXIT" >&2
  echo "  partial output saved to $OUT.raw" >&2
fi

mv "$OUT.raw" "$OUT"
echo "  Audit saved: $OUT" >&2
exit $EXIT
