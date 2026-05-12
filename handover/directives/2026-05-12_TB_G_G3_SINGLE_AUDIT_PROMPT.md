# TB-G G3 — SINGLE-AUDITOR AUDIT PROMPT (Codex G2 only)

> **Audit kind**: single-auditor (Codex G2). Gemini DeepThink skipped per
> user 2026-05-12 verbatim "Gemini 总是 all pass — 意义不大" + sessions
> #42/#43/#44 cadence. G3.1 + G3.4 are Class-2 production-wire-up; G3.3
> is Class-3 prompt-block + signature bump (param-bump is the only
> Class-up vector; mirror G2P.1 framing). All three atoms shipped under
> parent §8 G-Phase autonomous-forward authorization (G1.1 packet §6
> 「好，确认可以 ship」 2026-05-11 + G-Phase directive); user-adjudicated
> G3.3 Class-3 envelope at session #45 boot. No per-atom §8 packet
> required.
>
> **Audit subject**: 9-task chain-continuous batch at
> `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/` produced by
> `scripts/run_g_phase_batch.sh g_phase_g3_2026-05-12T11-02-27Z full`
> after G3.1 + G3.4 + G3.3 atoms shipped. Smoke wall=3088s; chain_length
> grew 1→6→7→8→9→10→11→12→13→14 (monotone, SG-G1.7 one-continuous-
> ChainTape; baseline-matched G1.2-7 R2 / G2P R1 / G2 R1).
> `aggregate_verdict=PROCEED 40/0/0/11`;
> `PERSISTENCE_BINDING_REPORT.json is_passing=true n_witnessed=4`.
>
> **Atoms in scope**:
> - **G3.1** SHIPPED 2026-05-12 `97e6527` — NEW `src/runtime/agent_pnl.rs`:
>   `compute_agent_pnl(q, agent_id, initial_balance_micro) ->
>   AgentMarketStateView` (architect-verbatim 7-field shape:
>   `agent_id` / `balance` / `open_positions` / `realized_pnl` /
>   `unrealized_pnl` / `solvency_status` / `reputation_score`). Pure
>   derivation over canonical `EconomicState`; integer-only math
>   (CLAUDE.md §13 no-f64); per-viewer isolation (Art. III.4). 5-variant
>   `OpenPosition` enum + 3-tier `SolvencyStatus`. PnL semantics:
>   realized = balance - initial; unrealized = signed MTM on
>   conditional-share holdings against active CpmmPool reserves (cost
>   basis = 1 μC / share-pair per mint cash flow). Balanced N+N mint
>   yields 0 unrealized regardless of pool skew; asymmetric position
>   yields signed PnL. Sibling helper
>   `initial_balance_micro_from_default_preseed(agent_id)` reads the
>   canonical preseed factory.
> - **G3.4** SHIPPED 2026-05-12 `2e7839f` — §G PnL trajectory dashboard
>   section. Added to `src/runtime/agent_pnl.rs`: `PnlTrajectoryRow` +
>   `PnlTrajectorySection` + `compute_from_q(q)` walker (iterates
>   `default_pput_preseed_pairs` 13-entry registry) + `render_section_g()`
>   renderer + `compute_pnl_trajectory_from_paths(repo, cas_path)` path
>   wrapper (loads `pinned_pubkeys.json` + `initial_q_state.json`,
>   replays via canonical `replay_full_transition` FC2 Boot primitive
>   for the one-continuous-ChainTape SG-G1.7 dual-bind). Silent-zero-
>   forbidden contract: when every row is flat (no PnL movement, no
>   positions, no reputation), render `MECHANISM BOTTLENECK` with ≥3
>   candidate causes (no router buys / no accepted WorkTx / no
>   reputation mutation pending G3.2). `audit_dashboard --run-report`
>   `render_tb_n3_run_report` injects the `## §G PnL trajectory` block
>   between §F.X (peer-verify coverage) and the price-is-signal banner.
>   Pre-existing banner renamed `## §G` → `## §H` to free the §G label;
>   SG-14.6 architect-mandated banner contract still enforced via
>   `render_section_14` (separate path; unchanged).
> - **G3.3** SHIPPED 2026-05-12 `903d164` — Class-3 `=== Your Position ===`
>   per-viewer prompt block. NEW `src/sdk/your_position.rs`:
>   `render_your_position(q, viewer)` mirrors G2P.1
>   `pending_peer_reviews.rs` + N1 A2 `econ_position.rs` patterns.
>   `DRUCKER_FRAMING_LINE` constant carries the architect-verbatim
>   framing: `"Drucker: 'What gets measured gets managed' — your
>   position drives your next decision."` `build_agent_prompt` signature
>   gains 10th `your_position: &str` parameter; suppression contract on
>   empty string mirrors econ_position + pending_peer_reviews. Evaluator
>   wires the renderer at the build_agent_prompt call site (~line 2204)
>   from sequencer `q_snapshot()`. Per-viewer isolation enforced by
>   `compute_agent_pnl(q, viewer, ...)`'s viewer-keyed filter.
>
> **Repo HEAD on origin/main**: `903d164` (G3.3 ship commit; G_PHASE_BATCH_MANIFEST.json
> + run_log.txt both pin `git_head=903d16407106ded31e7acfb1d5ecaf36cce3353b`).
> Trust-root rehashes: `src/runtime/mod.rs` (G3.1) +
> `src/bin/audit_dashboard.rs` (G3.4) + `experiments/minif2f_v4/src/bin/evaluator.rs`
> (G3.3); each bundled into its own atom commit per G2 collapsed-
> rehash pattern.
>
> **Empirical result (auto-rendered §G PnL trajectory)**:
> ```
> ## §G PnL trajectory
>   (per-agent realized/unrealized PnL over the batch; integer-rational μC; cost basis 1 μC/share-pair)
>   - tb7-7-sponsor: balance=9900000 μC (initial 10000000); realized=-100000; unrealized=0; positions=0; rep=0; solvent
>   - Agent_user_0: balance=10000000 μC (initial 10000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_0: balance=999000 μC (initial 1000000); realized=-1000; unrealized=0; positions=2; rep=0; solvent
>   - Agent_1: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_2: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_3: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_4: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_5: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_6: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_7: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_8: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - Agent_9: balance=1000000 μC (initial 1000000); realized=0; unrealized=0; positions=0; rep=0; solvent
>   - MarketMakerBudget: balance=4900000 μC (initial 5000000); realized=-100000; unrealized=0; positions=1; rep=0; solvent
> ```
> **3 of 13 rows are NON-FLAT** (tb7-7-sponsor escrow -100k μC; Agent_0
> solver stake + open claim -1000 μC, positions=2; MarketMakerBudget
> collateral -100k μC, positions=1). Silent-zero MECHANISM BOTTLENECK
> contract correctly ABSENT (only fires when ALL rows flat). §H banner
> renamed from §G to §H (price-is-signal-not-truth). Architect §G3
> SG-G3.5 "PnL is visible in dashboard as materialized view" empirically
> SATISFIED — better outcome than G2 R1 / G2P R1 where the dashboard
> rendered all-zero shape; G3 batch carries the canonical baseline
> economic activity (escrow + stake + collateral seed) as non-flat rows.
>
> **Charter ship gates**:
> - SG-G3.1 / SG-G3.2 / SG-G3.3.a-e / SG-G3.9.a-d (G3.1 7-field shape +
>   genesis 0 PnL + 5-scenario coverage + source-grep) — closed pure-
>   code at `tests/constitution_g3_pnl.rs` 12/12 GREEN.
> - SG-G3.8.a-e + SG-G1.7-bind (G3.4 §G render + dual-bind) — closed
>   pure-code at
>   `tests/constitution_g3_pnl_trajectory_evidence_binding.rs` 6/6 GREEN.
> - SG-G3.6 / SG-G3.7 / SG-G3.13 / SG-G3.13.a-e (G3.3 prompt block +
>   Drucker verbatim + per-viewer isolation + signature bump) — closed
>   pure-code at `tests/constitution_g3_your_position_prompt.rs` 8/8
>   GREEN.
> - **Architect §G3 SG-G3.5 ship gate** ("PnL is visible in dashboard
>   as materialized view") — empirically: dashboard §G PnL trajectory
>   block rendered; per-agent rows visible; silent-zero MECHANISM
>   BOTTLENECK contract satisfies architect §8.5 OR-branch.
>
> **Constitutional anchors**: charter §1 Module G3 + G-Phase directive
> `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
> §G3 verbatim 7-field shape + Drucker framing + CLAUDE.md §13 economy
> laws + §15 shielding + §17 reporting standard.

---

## §1. Evidence inventory

Evidence dir: `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/`.

Expected files:
- `G_PHASE_BATCH_MANIFEST.json` — pre-batch pin; `git_head=<HEAD>`,
  `problem_count=9`, `active_model=deepseek-chat`, `llm_proxy_url=
  http://localhost:8080`.
- `PROBLEMS.txt` — canonical 9-problem TB-N3 Phase 2 set.
- `runtime_repo/` — ONE shared git repo across the batch. Expected L4
  chain length 14 (G1.2-7 R2 baseline shape).
- `cas/` — ONE shared CAS, ~875 objects (G2 R1 / G2P R1 baseline).
- `BatchContinuationManifest.json` — G1.2-4 canonical `g1_2_v1` schema.
- `P000..P008/evaluator.stdout` + `.stderr` — per-task PPUT_RESULT.
- `aggregate_verdict.json` — `audit_tape` over shared chain.
- `PERSISTENCE_BINDING_REPORT.json` — G1.2-5 binding output;
  `is_passing=true`, `n_witnessed=4`, `n_tasks=9`.
- `batch_evaluator.log` — orchestrator log; expect 8
  `ResumePreflight::Ok` + 8 `ChainTapeLease ACQUIRED`.
- `run_log.txt` — canonical post-audit summary.

---

## §2. Audit questions (Q1..Q12)

Each auditor returns:
```
VERDICT: <PROCEED|CHALLENGE|VETO|HALT>
CONVICTION: <high|medium|low>
Q1: <PASS|CHALLENGE|VETO> [reason if not PASS, with file:line refs]
...
Q12: ...
Notes: <free-form observations>
```

**Q1 (G3.1 7-field shape source-grep)**: Source-grep
`src/runtime/agent_pnl.rs` for the 7 architect-verbatim field names of
`AgentMarketStateView`: `agent_id`, `balance`, `open_positions`,
`realized_pnl`, `unrealized_pnl`, `solvency_status`, `reputation_score`.
Each must appear ≥ 2× (struct def + downstream accessor / render).
Reject if any field is missing or renamed. SG-G3.9 gate pins this;
spot-verify the gate test
`tests/constitution_g3_pnl.rs::sg_g3_9_source_grep_seven_field_shape`.

**Q2 (G3.1 integer-only math)**: Source-grep
`src/runtime/agent_pnl.rs` for forbidden tokens
`": f64"`, `": f32"`, `"as f64"`, `"as f32"`, `" f64::"`, `" f32::"`.
ZERO occurrences required (CLAUDE.md §13 no-f64-in-money-path).
SG-G3.9.c gate pins this; spot-verify.

**Q3 (G3.1 5-scenario PnL semantics)**: Run
`cargo test --test constitution_g3_pnl --no-fail-fast` and confirm
12 tests PASS. Spot-check:
- SG-G3.1 genesis at preseed baseline → `realized_pnl == 0`,
  `unrealized_pnl == 0`, `solvency == Solvent`.
- SG-G3.2 post-BuyWithCoinRouter fixture (150k YES + 50k NO holding,
  pool 50:150 reserves) → `realized_pnl == -100_000`,
  `unrealized_pnl == +25_000` (signed MTM).
- SG-G3.3.b balanced 100k+100k mint with no pool →
  `unrealized_pnl == 0` (no MTM signal).
- SG-G3.3.c balanced 100k+100k mint with skewed pool 20:80 →
  `unrealized_pnl == 0` (constant-product YES+NO prices sum to 1).
- SG-G3.3.e Resolved-status pool → `unrealized_pnl == 0` (architect
  §G3 future-oracle path; deferred).

Reject if any of the 12 binding tests fail OR if the asymmetric SG-G3.2
math doesn't match the documented `(125_000 - 100_000) = +25_000`.

**Q4 (G3.4 §G render contract)**: Run
```
target/release/audit_dashboard --run-report \
  --repo handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/runtime_repo \
  --cas  handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/cas
```
Confirm output contains the new `## §G PnL trajectory` block with 13
rows (one per canonical preseed agent: `tb7-7-sponsor`, `Agent_user_0`,
`Agent_0..9`, `MarketMakerBudget`). Each row carries
`balance=<N> μC (initial <I>); realized=<R>; unrealized=<U>;
positions=<P>; rep=<X>; <solvent|near_insolvent|bankrupt>`.

Confirm the pre-existing price-is-signal banner is now under
`## §H PRICE IS SIGNAL, NOT TRUTH` (renamed from §G to free the label
for PnL trajectory). SG-14.6 architect-mandated banner contract still
enforced via `render_section_14` (separate section; unchanged).

Reject if §G is missing, if the 13 preseed rows are non-exhaustive, or
if §H banner is missing.

**Q5 (G3.4 silent-zero MECHANISM BOTTLENECK contract — both branches)**:
On this batch 3 of 13 rows are NON-FLAT (tb7-7-sponsor + Agent_0 +
MarketMakerBudget; canonical baseline economic activity); therefore the
`MECHANISM BOTTLENECK` block MUST be ABSENT in the rendered §G. Confirm
the rendered §G does NOT contain the substring `MECHANISM BOTTLENECK`.

Also verify the inverse branch by running the binding test:
`cargo test --test constitution_g3_pnl_trajectory_evidence_binding
sg_g3_8_b_empty_fixture_triggers_mechanism_bottleneck`. It must PASS,
which pins the silent-zero contract: when ALL rows are flat the
bottleneck block MUST be present with ≥3 enumerated causes
(numbered `1./2./3.`):
- Cause #1 names "BuyWithCoinRouter" + G5.1 forward fix.
- Cause #2 names "accepted WorkTx" + `TURINGOS_CHAINTAPE_PRESEED=1`.
- Cause #3 names "reputations_t" + G3.2 Class-4 forward bind.

Reject if either branch breaks: empirical batch shows bottleneck-when-
not-all-flat, OR binding test fails to trigger bottleneck on all-flat
fixture.

**Q6 (G3.4 dual-bind to G1 SG-G1.7)**: Confirm
`compute_pnl_trajectory_from_paths` delegates to the canonical
`replay_full_transition` FC2 Boot primitive (NOT a stitch of
independent QStates). Source-grep
`src/runtime/agent_pnl.rs` for `replay_full_transition` and verify the
call sequence: load `pinned_pubkeys.json` + `initial_q_state.json` →
open `Git2LedgerWriter` + `CasStore` → collect entries `[1..=chain_len]`
→ `replay_full_transition(initial_q, entries, cas_view, pinned,
predicate_registry, tool_registry)` → walk final QState.

Reject if the trajectory walker re-implements replay or uses a derived-
view-only path that bypasses `replay_full_transition`.

**Q7 (G3.3 Drucker verbatim framing)**: Source-grep
`src/sdk/your_position.rs` for the exact framing constant:
`"Drucker: 'What gets measured gets managed' \u{2014} your position
drives your next decision."` Verify the constant is exported as
`DRUCKER_FRAMING_LINE` and that `render_your_position` calls it as the
FIRST line of the rendered block. SG-G3.13 gate pins this; spot-verify
`tests/constitution_g3_your_position_prompt.rs::sg_g3_13_drucker_verbatim_framing`.

Reject if any of the three subphrases is missing or paraphrased:
`"Drucker:"`, `"What gets measured gets managed"`, `"your position
drives your next decision"`.

**Q8 (G3.3 per-viewer isolation source-grep)**: Source-grep
`src/sdk/your_position.rs` for `compute_agent_pnl(q, viewer` (the
per-viewer filter call). Confirm the renderer does NOT iterate
`balances_t.0.values()` or any aggregating helper without a viewer
filter. Spot-verify SG-G3.13.a binding test: a Bob-stake fixture
returns Alice's render block that does NOT contain Bob's tx_id, stake
amount, or agent_id. SG-G3.6 + SG-G3.13.a gates pin this.

Reject if the renderer aggregates across agents OR if the per-viewer
source-grep fails.

**Q9 (G3.3 evaluator wire + signature bump)**: Inspect
`experiments/minif2f_v4/src/bin/evaluator.rs` around line 2204 for:
- `your_position::render_your_position(&q, &AgentId(agent_id.clone()))`
  invocation from `bus.sequencer.as_ref().and_then(|seq|
  seq.q_snapshot().ok()).map(...)` chain.
- `build_agent_prompt(...)` call passes `&your_position` as the 10th
  argument.

Inspect `src/sdk/prompt.rs::build_agent_prompt` for:
- 10th parameter signature: `your_position: &str`.
- `=== Your Position ===` heading emission with empty-string-
  suppression contract.

Reject if any wire is missing or if the signature is incomplete.

**Q10 (trust-root rehash discipline)**: Confirm
`genesis_payload.toml [trust_root]` was rehashed for the 3 edited
trust-rooted files:
- `src/runtime/mod.rs` (G3.1 ship `97e6527`): `1d128067` → `f0caecfc`
- `src/bin/audit_dashboard.rs` (G3.4 ship `2e7839f`):
  `aad73808` → `27bffa9f`
- `experiments/minif2f_v4/src/bin/evaluator.rs` (G3.3 ship `903d164`):
  `4a369b4f` → `27537f26`

Run `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
and confirm PASS. Reject if any rehash is missing or the trust-root
test fails at HEAD `903d164`.

**Q11 (continuity at scale)**: Mirror G2 R1 Q9 — verify the
9-task BatchContinuationManifest has continuity invariant
`tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex` for k ∈
{0..7}. The live `runtime_repo` HEAD should match
`tasks.last().end_head_t_hex`. Pre-visible from `batch_evaluator.log`:
chain_length grows monotone across the 8 resume boundaries.

Confirm EXACTLY ONE `genesis_report.json` at `runtime_repo/`; no
per-task genesis files; no `P*_*/runtime_repo` directories.

**Q12 (persistence binding regression + kill-criteria)**: 
`PERSISTENCE_BINDING_REPORT.json` shows `is_passing=true`,
`n_witnessed=4`, `n_tasks=9`. Matches G1.2-7 R2 / G2P R1 / G2 R1
baseline shape. Reject if any field flipped from Witnessed to Reset.

Audit against charter §0 `kill_criteria_tested` 1-5:
- no per-problem genesis reset
- Predicate doesn't read price-market-trace
- no bankrupt-cap bypass (G3.1 solvency classifier is read-only;
  G3.2 admission deferred per Class-4 §8 packet)
- no hidden model switch
- conservation invariants hold (CLAUDE.md §13:
  `assert_no_post_init_mint`, `assert_total_ctf_conserved`,
  `assert_complete_set_balanced` GREEN end-of-batch)

Plus G3-specific:
- The §G PnL trajectory walker is read-only over ChainTape + CAS;
  never mutates state (G3.1 "Pure derivation" contract).
- The `=== Your Position ===` prompt block is per-viewer; no
  cross-agent PnL aggregation (Art. III.4 shielding).
- The Drucker framing string is verbatim from architect §G3.

---

## §3. Halt conditions (block ship)

- Q1 FAIL — any of the 7 architect-verbatim fields missing.
- Q2 FAIL — any f64/f32 token in agent_pnl.rs.
- Q3 FAIL — any of the 12 G3.1 binding tests fails.
- Q4 FAIL — §G block missing or 13-row preseed iteration non-exhaustive.
- Q5 FAIL — silent-zero MECHANISM BOTTLENECK contract broken (missing
  when all flat OR present when non-flat).
- Q6 FAIL — trajectory walker doesn't use canonical `replay_full_transition`.
- Q7 FAIL — Drucker verbatim framing missing or paraphrased.
- Q8 FAIL — per-viewer isolation broken (aggregation across agents).
- Q9 FAIL — evaluator wire missing or signature bump incomplete.
- Q10 FAIL — trust-root rehash missing or verify test fails.
- Q11 FAIL — HEAD_t discontinuity OR fresh genesis_report.json at
  task_index > 0.
- Q12 FAIL — Witnessed → Reset on any persistence field OR any
  kill_criteria_tested violation.

---

## §4. Output format

Codex G2 emits in this order:

```
VERDICT: <PROCEED|CHALLENGE|VETO|HALT>
CONVICTION: <high|medium|low>
Q1: <PASS|CHALLENGE|VETO> [reason if not PASS, with file:line refs]
Q2: ...
...
Q12: ...
Notes: <free-form observations; flag any provenance / audit-trail gaps
       even if not blocking; report Q4 §G counts verbatim>
```

---

## §5. Cross-references

- G3.1 ship commit: `97e6527` ("TB-G G3.1 SHIPPED")
- G3.4 ship commit: `2e7839f` ("TB-G G3.4 SHIPPED")
- G3.3 ship commit: `903d164` ("TB-G G3.3 SHIPPED")
- Trust-root rehashes: bundled into the 3 atom commits per G2
  collapsed-rehash pattern (NOT separate commits).
- Predecessor G2 single-audit prompt:
  `handover/directives/2026-05-12_TB_G_G2_SINGLE_AUDIT_PROMPT.md`
- Predecessor G2 Codex verdict:
  `handover/audits/CODEX_G2_TB_G_G2_VERDICT.md` (PROCEED 12/12)
- Charter §1 Module G3:
  `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
- G-Phase directive §G3 verbatim 7-field shape + Drucker framing:
  `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
