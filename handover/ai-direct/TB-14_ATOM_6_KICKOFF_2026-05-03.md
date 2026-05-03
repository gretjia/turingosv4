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
