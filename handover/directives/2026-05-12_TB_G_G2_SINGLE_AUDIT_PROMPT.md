# TB-G G2 — SINGLE-AUDITOR AUDIT PROMPT (Codex G2 only)

> **Audit kind**: single-auditor (Codex G2). Gemini DeepThink skipped per
> user 2026-05-12 verbatim "Gemini 总是 all pass — 意义不大" + sessions
> #42/#43 cadence. G2 is a Class-2 production-wire-up + dashboard-render
> atom (charter §1 Module G2 row 3 atoms, each Class 2; no §8 packet
> required); under `feedback_dual_audit` "hybrid by risk class" the
> production-wire-up requirement was waived by user direction with
> mechanism explanation (one-auditor signal is the ship gate; Codex's
> adversarial axis is what rigor-finds).
>
> **Audit subject**: 9-task chain-continuous batch at
> `handover/evidence/g_phase_g2_2026-05-12T07-48-28Z/` produced by
> `scripts/run_g_phase_batch.sh g_phase_g2_<TS> full` after G2.1 +
> G2.2 + G2.3 atoms shipped.
>
> **Atoms in scope**:
> - **G2.1** SHIPPED 2026-05-12 `f22140a` — `src/runtime/market_decision_trace.rs`
>   `NoTradeReason` tail-append 2 variants (11 → 13 total):
>   `NoPerceivedEdge` + `PromptBudgetExceeded`. `AmountExceedsBalance`
>   carries the architect §8.2 `InsufficientBalance` doc-alias.
>   `src/runtime/adapter.rs::InvestRouteError` gains 2 caller-constructible
>   classifier variants mapping 1:1. `experiments/minif2f_v4/src/bin/evaluator.rs`
>   end-of-turn classifier wires both new variants — fires
>   `MarketDecisionTrace::no_trade(NoPerceivedEdge | PromptBudgetExceeded, …)`
>   on every non-invest market-bearing agent turn.
> - **G2.2** SHIPPED 2026-05-12 `9b05563` —
>   `src/runtime/market_decision_trace_summary.rs` (NEW) library helper
>   lifted from `src/bin/audit_dashboard.rs::render_tb_n3_run_report`
>   inline §F walker. Adds `submitted_vs_traced_ratio` row (integer-
>   rational percent, `n/a` on empty batches) + `## §F.A NoTradeReason
>   exhaustive breakdown` (13-row stable iteration over
>   `NoTradeReason::ALL`, zeros included for forward grep stability).
> - **G2.3** SHIPPED 2026-05-12 `297042c` —
>   `tests/constitution_g2_failed_invest_l4e.rs` (NEW) — 4 binding tests
>   against a real Sequencer + InMemoryLedgerWriter harness:
>   SG-G2.5.a balance-shortfall router rejection lands in L4.E with coarse
>   `RejectionClass::PolicyViolation` + `public_summary == "policy_violation"`;
>   SG-G2.5.b pool-not-Active → same coarse class; SG-G2.5.c adapter
>   pre-classifier `AmountExceedsBalance` round-trips through
>   `MarketDecisionTraceSummary::compute_from_path`; SG-G2.5.d full
>   architect §8.6 "Failed invest 也算有意义 tape activity" chain (L4.E
>   rejection AND CAS MarketDecisionTrace trace).
>
> **Repo HEAD on origin/main**: `297042c` (G2.3 ship commit; trust-root
> already rehashed at G2.1 + G2.2 commits). Run-start manifest pins
> `git_head=297042c`.
>
> **Empirical result (auto-rendered §F + §F.A)**:
> ```
> ## §F MarketDecisionTrace summary
>   total_traces: 0
>   submitted_vs_traced_ratio: 0/0 = n/a (no traces)
>
> ## §F.A NoTradeReason exhaustive breakdown
>   (architect §G2 13-variant taxonomy; stable insertion order; zeros included for forward grep stability)
>   no_prompt_tool = 0
>   no_parsed_invest = 0
>   malformed_node = 0
>   zero_amount = 0
>   amount_exceeds_balance = 0
>   no_pool = 0
>   router_rejected = 0
>   agent_declined = 0
>   too_fast_solve = 0
>   slippage_out_zero = 0
>   unknown = 0
>   no_perceived_edge = 0
>   prompt_budget_exceeded = 0
> ```
> Zero-trace outcome is consistent with G2P R1 (same 9-task batch shape).
> The new wire is DEFENSIVELY correct (SG-G2.6.a source-grep + SG-G2.6
> type-level invariant both GREEN) but had no opportunity to fire in
> this batch because:
> 1. Only 1 WorkTx was accepted across 9 tasks (1 OMEGA-solve on P000).
> 2. TB-N3 pools emit on the OMEGA-accept path; the OMEGA exit returns
>    immediately so no further LLM call in that task sees the pool.
> 3. Cross-task pool filter (`amendment 5 same-task isolation`) strips
>    the prior task's pool from subsequent tasks.
> 4. Net result: `tb_n3_market_block_present == false` for every LLM turn
>    across the batch → end-of-turn classifier never fires.
>
> **Audit primary question** mirrors G2P R1: did the §F + §F.A render
> contract render correctly (G2.2 ship gate)? G2.1's wire correctness is
> SG-G2.6.a source-grep + SG-G2.6 type-level; G2.3's L4.E binding is
> covered by 4 gate tests against a real Sequencer harness. The empirical
> `total_traces=0` is the architect-§8.5 "empty market as valid empirical
> result" branch, identical to G2P R1's `peer_verifications_total=0`
> outcome.
>
> **Charter ship gates**:
> - SG-G2.1 / SG-G2.2 / SG-G2.6 (G2.1 13-variant taxonomy exhaustive +
>   source-grep + trace-or-tx invariant) — closed pure-code at
>   `tests/constitution_g2_no_trade_reason_taxonomy.rs` 8/8 GREEN.
> - SG-G2.4.a-e (G2.2 §F + §F.A render contract) — closed pure-code at
>   `tests/constitution_g2_dashboard_no_trade_rows.rs` 5/5 GREEN.
> - SG-G2.5.a-d (G2.3 failed-invest L4.E binding) — closed pure-code at
>   `tests/constitution_g2_failed_invest_l4e.rs` 4/4 GREEN.
> - **Architect §G2 SG-G2.3 ship gate** ("NoTradeReason appears in
>   dashboard and CAS") — empirically: dashboard rendering CONFIRMED via
>   §F.A 13-row stable block; CAS side `total_traces=0` (zero-trace
>   empirical baseline; architect §8.5 OR-branch).
>
> **Constitutional anchors**: charter §1 Module G2 + G-Phase directive
> `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
> §G2 verbatim 9-variant taxonomy + CLAUDE.md §6 (externalized attempt
> rule) + §15 shielding + §17 reporting standard.

---

## §1. Evidence inventory

Evidence dir: `handover/evidence/g_phase_g2_2026-05-12T07-48-28Z/`.

Expected files:
- `G_PHASE_BATCH_MANIFEST.json` — pre-batch pin; `git_head=297042c`,
  `problem_count=9`, `active_model=deepseek-chat`, `llm_proxy_url=
  http://localhost:8080`.
- `PROBLEMS.txt` — canonical 9-problem TB-N3 Phase 2 set.
- `runtime_repo/` — ONE shared git repo across the batch. Expected L4
  chain length 14 (G1.2-7 R2 baseline shape: 1 WorkTx + TerminalSummary
  per task + bootstrap entries).
- `cas/` — ONE shared CAS, ~875 objects (G2P R1 baseline).
- `BatchContinuationManifest.json` — G1.2-4 canonical `g1_2_v1` schema.
- `P000..P008/evaluator.stdout` + `.stderr` — per-task PPUT_RESULT.
- `aggregate_verdict.json` — `audit_tape` over shared chain
  (PROCEED 40/0/0/11; same shape as G2P R1).
- `PERSISTENCE_BINDING_REPORT.json` — G1.2-5 binding output;
  `is_passing=true`, `n_witnessed=4`, `n_tasks=9`.
- `batch_evaluator.log` — orchestrator log; expect 8
  `ResumePreflight::Ok` + 8 `ChainTapeLease ACQUIRED` (already
  pre-visible in current `/tmp/g_phase_g2_smoke.log`).
- `run_log.txt` — canonical post-audit summary
  (verdict=PROCEED, persistence_passing=true).

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

**Q1 (G2.1 taxonomy exhaustive)**: Source-grep
`src/runtime/market_decision_trace.rs` for the 13 variants:
`NoPromptTool, NoParsedInvest, MalformedNode, ZeroAmount,
AmountExceedsBalance, NoPool, RouterRejected, AgentDeclined,
TooFastSolve, SlippageOutZero, Unknown, NoPerceivedEdge,
PromptBudgetExceeded`. Each must appear in (1) the enum body, (2) the
`label()` match arm, and (3) `NoTradeReason::ALL`. Reject if any variant
is missing or if `label()` is non-exhaustive (the Rust compiler enforces
this but verify the gate test `tests/constitution_g2_no_trade_reason_taxonomy.rs`
SG-G2.1 source-grep passes for each variant ≥2×).

**Q2 (G2.1 architect §8.2 doc-alias)**: Confirm `AmountExceedsBalance`
variant carries the architect §8.2 `InsufficientBalance` doc-alias note
in the rustdoc. Run `grep -n "InsufficientBalance"
src/runtime/market_decision_trace.rs` and verify ≥1 occurrence in the
rustdoc block. The SG-G2.4 gate pins this; spot-verify.

**Q3 (G2.1 evaluator end-of-turn classifier wire)**: Inspect
`experiments/minif2f_v4/src/bin/evaluator.rs` for:
- `tb_n3_market_block_present` boolean (~line 2106 block; set true when
  the TB-N3 render returned non-empty).
- `tb_n3_market_block_budget_elided` boolean (set true when `K==0`
  forced top-K elision while same-task pools existed).
- `invest_action_emitted_this_turn` boolean (set true at the head of
  `"invest" =>` arm).
- End-of-turn classifier between the parse-match close and the LLM-error
  arm: fires `MarketDecisionTrace::no_trade(NoTradeReason::NoPerceivedEdge
  | NoTradeReason::PromptBudgetExceeded, …)` when
  `!invest_action_emitted_this_turn && (tb_n3_market_block_present ||
  tb_n3_market_block_budget_elided)`. Writes to CAS via
  `write_market_decision_trace_to_cas` + bumps `tool_dist["invest_no_trade_<label>"]`.

Reject if any of the 4 source-grep witnesses is missing.

**Q4 (G2.2 §F render contract)**: Run
```
target/release/audit_dashboard --run-report \
  --repo handover/evidence/g_phase_g2_2026-05-12T07-48-28Z/runtime_repo \
  --cas handover/evidence/g_phase_g2_2026-05-12T07-48-28Z/cas
```
Confirm output contains both:
- `## §F MarketDecisionTrace summary` block with `total_traces: 0` +
  `submitted_vs_traced_ratio: 0/0 = n/a (no traces)`.
- `## §F.A NoTradeReason exhaustive breakdown` block with all 13 labels
  in stable insertion order, each `= 0`.

Reject if §F.A is missing OR the 13-row block is non-exhaustive OR the
ratio row is missing OR uses f64 (look for `.` in the ratio number).

**Q5 (G2.2 binary uses library helper)**: Source-grep
`src/bin/audit_dashboard.rs` for
`MarketDecisionTraceSummary::compute_from_cas` and `.render_section_f()`.
Confirm the inline §F walker (`let mut total_traces: u64 = 0;`,
`for entry in cas.list_all_cids()` deserializing
`MarketDecisionTrace`) was REMOVED from the binary. The library helper
`src/runtime/market_decision_trace_summary.rs` is now the canonical
source. SG-G2.4.e pins this; spot-verify.

**Q6 (G2.3 L4.E binding test contract)**: Run
`cargo test --test constitution_g2_failed_invest_l4e --no-fail-fast`
and confirm 4 tests PASS (SG-G2.5.a/b/c/d). Spot-check:
- SG-G2.5.a asserts the rejection record's
  `rejection_class == RejectionClass::PolicyViolation` (NOT
  RouterInsufficientCoinBalance — the coarse-class fold is the
  architect's shielding policy per
  `src/state/sequencer.rs::rejection_class_for` wildcard arm).
- SG-G2.5.a also asserts `public_summary == Some("policy_violation")`.
- SG-G2.5.c demonstrates the adapter-side classifier writes a
  `MarketDecisionTrace::no_trade(AmountExceedsBalance, …)` trace to CAS
  that round-trips through `MarketDecisionTraceSummary::compute_from_path`
  with count = 1 in the `amount_exceeds_balance` row.

Reject if any of the 4 binding tests fails OR if the assertion shape is
weaker than the test claims (e.g., if the test passes without actually
exercising the Sequencer admission path).

**Q7 (G2 architect §G2 SG-G2.3 ship gate empirical outcome)**: This is
the primary empirical question. Report the rendered §F.A counts verbatim.
If `total_traces == 0`, confirm the architect §8.5 "empty market as
valid empirical result" OR-branch applies (the new wire is defensively
correct but had no opportunity to fire because no agent invoked the
invest tool and no in-task pool was visible at any LLM turn). Report
this as DATA, not pass/fail (mirrors G2P R1's Q6).

**Q8 (G2.1 trust-root rehash discipline)**: Confirm
`genesis_payload.toml [trust_root]` was rehashed for the 4 edited files:
- `src/runtime/adapter.rs` (G2.1 ship `f22140a`): `a84afccf` → `bdd4be50`
- `experiments/minif2f_v4/src/bin/evaluator.rs` (G2.1): `b5c5ec97` → `4a369b4f`
- `src/runtime/mod.rs` (G2.2 ship `9b05563`): `b653e247` → `1d128067`
- `src/bin/audit_dashboard.rs` (G2.2): `2dba81a2` → `aad73808`

Run `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
and confirm PASS. Reject if any rehash is missing or the trust-root
test fails at HEAD `297042c`.

**Q9 (continuity at scale)**: Mirror G2P R1 Q9 — verify the
9-task BatchContinuationManifest has continuity invariant
`tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex` for k ∈
{0..7}. The live `runtime_repo` HEAD should match
`tasks.last().end_head_t_hex`. Pre-visible from `batch_evaluator.log`:
chain_length grows 6 → 7 → 8 → 9 → 10 → 11 → 12 → 13 across the 8
resume boundaries.

**Q10 (one genesis at scale)**: Mirror G2P R1 Q10 — confirm EXACTLY ONE
`genesis_report.json` at `runtime_repo/`; no per-task genesis files;
no `P*_*/runtime_repo` directories.

**Q11 (persistence binding regression)**: `PERSISTENCE_BINDING_REPORT.json`
shows `is_passing=true`, `n_witnessed=4`, `n_tasks=9`. Matches G1.2-7 R2
and G2P R1 baseline shape. Reject if any field flipped from Witnessed
to Reset (kill_criteria_tested #1 violation).

**Q12 (kill-criteria across batch)**: Mirror G2P R1 Q12 — audit against
charter §0 `kill_criteria_tested` 1-5: no per-problem genesis reset /
Predicate doesn't read price-market-trace / no bankrupt-cap bypass / no
hidden model switch / conservation invariants hold. Plus G2-specific:
- `NoTradeReason::ALL` is stable-order-frozen (any future commit that
  re-orders the existing 11 variants would invalidate §F.A column
  positions — assert the SG-G2.2 gate pins this).
- The end-of-turn classifier writes ONLY to CAS (not L4 directly) when
  the trace is no-trade; the on-chain anchor remains the
  `BuyWithCoinRouterTx` (architect §8.6 invariant).

---

## §3. Halt conditions (block ship)

- Q1 FAIL — any of the 13 variants missing from enum / label / ALL.
- Q3 FAIL — evaluator end-of-turn classifier wire missing or incorrect.
- Q4 FAIL — §F.A 13-row block missing or non-exhaustive.
- Q5 FAIL — binary still uses inline §F walker (architecture regression).
- Q6 FAIL — any of the 4 L4.E binding tests fails.
- Q8 FAIL — trust-root rehash missing or verify test fails.
- Q9 FAIL — HEAD_t discontinuity.
- Q10 FAIL — fresh `genesis_report.json` at task_index > 0.
- Q11 FAIL — Witnessed → Reset on any persistence field.
- Q12 FAIL — any kill_criteria_tested clause violation.

Q2 / Q7 are EMPIRICAL / soft-verification questions; the empirical
`total_traces=0` is the architect-§8.5 OR-branch (DATA, not pass/fail)
per the precedent G2P R1 set.

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
       even if not blocking; report Q4 + Q7 §F.A counts verbatim>
```

---

## §5. Cross-references

- G2.1 ship commit: `f22140a` ("TB-G G2.1 SHIPPED")
- G2.2 ship commit: `9b05563` ("TB-G G2.2 SHIPPED")
- G2.3 ship commit: `297042c` ("TB-G G2.3 SHIPPED")
- Trust-root rehashes: bundled into the 3 atom commits (NOT separate
  commits this round — the G2.1 commit message documents the post-edit
  sha256 → manifest update). Predecessor G2P pattern split trust-root
  rehashes into separate commits (`58d4ded`, `9ddc9c1`); G2 collapses
  them into atom commits for tighter atomicity.
- Predecessor G2P single-audit prompt:
  `handover/directives/2026-05-12_TB_G_G2P_SINGLE_AUDIT_PROMPT.md`
- Predecessor G2P Codex verdict:
  `handover/audits/CODEX_G2_TB_G_G2P_VERDICT.md`
- Charter §1 Module G2:
  `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
- G-Phase directive §G2 verbatim 9-variant taxonomy:
  `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
