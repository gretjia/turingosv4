# Constitutional Operationalization MEGA PLAN v3.1 — Atomized

> **Plan v3.1** — atomization of `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md`. Supersedes Plan v3 outline (`CO_MEGA_PLAN_2026-04-26.md`) and Plan v1 / TFR v1 (now legacy).
>
> **Authoritative spec**: white paper architecture chapter + economic chapter (both 2026-04-26). Constitution Art. 0–0.4 amendments.
>
> **Status**: ArchitectAI v3.1 atomization. Awaiting user D1-D6 + dual external audit.

---

## § 1 What v3.1 Adds Over v3

v3 was the **structural framework** (which phases, which atoms count, which files). v3.1 adds:

1. **Per-atom file path** + **direct white paper §** + **direct constitution Art** reference.
2. **Atom dependency graph** (which atom blocks which).
3. **Economic chapter integration** — all 12 invariants and 9 RSP modules as named atoms.
4. **STEP_B atom marking** — every atom touching `src/{bus,kernel,wal,ledger}.rs` or any new restricted file gets explicit STEP_B parallel-branch protocol invocation.
5. **Conformance test mapping** — every atom names its destination test file.

---

## § 2 CO Phase 0 — Foundation (1 week, 7 atoms)

Goal: get plan v3.1, blueprint, constitutional amendment, and TR migration **frozen and dual-audited** before any code touches.

| Atom | Scope | File(s) | STEP_B? | Test | Audit |
|---|---|---|---|---|---|
| **CO0.1** | Save FINAL_BLUEPRINT (this commit) | `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` | No | n/a (doc) | Internal review |
| **CO0.2** | Save Plan v3.1 (this commit) | `handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md` | No | n/a (doc) | Internal review |
| **CO0.3** | Constitution Art. 0.5 (white paper integration) per D2 | `constitution.md` (cp workflow per R-018 V.3) | No (constitution lives outside src/) | `tests/constitution_root_amendment.rs` | Dual audit on Art. 0.5 text |
| **CO0.4** | PREREG amendment v2 per D1 | `handover/preregistration/PREREG_AMENDMENT_v2_2026-04-26.md` | No | n/a (doc) | Internal review |
| **CO0.5** | TFR v1 deprecate (rename + LEGACY banner) | `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md` (add deprecate banner only — preserve content per D3) | No | n/a (doc) | n/a |
| **CO0.6** | Trust Root migration — TR manifest **43 → 49 entries (+6)**: 2 white papers (architecture + economic) + blueprint + plan v3.1 + protocol + amendment v1. Constitution Art 0.5 DRAFT and PREREG v2 DRAFT are NOT Trust-Rooted until user-enacted. | `genesis_payload.toml` | No | `tests/trust_root_v4_manifest.rs` | n/a |
| **CO0.7** | External dual audit on blueprint + plan v3.1 — PASS/PASS gate | (audit reports written to `handover/audits/`) | No | n/a | **DUAL AUDIT GATE** |

**CO P0 exit criteria**: PASS/PASS from Codex + Gemini on blueprint + plan v3.1; user formally answers D1-D6; constitution Art. 0.5 SHA frozen.

---

## § 3 CO Phase 1 — GitTape + Anti-Oreo + Predicate/Tool Registries (8-10 weeks, 50-65 atoms)

Goal: all of white paper architecture chapter (§ 1-12) operationalized. Phase 1 of Economic § 20 (Local Ledger Economy) substrate ready.

### CO P1.0 — Constitution Root formalization (L0; 2 atoms)

| Atom | Scope | File | STEP_B? |
|---|---|---|---|
| CO1.0.1 | `chain_tape::ChainTape::genesis()` reads constitution_hash + signature + sudo_policy from `genesis_payload.toml` | `src/bottom_white/tape/chain_tape.rs` (NEW) | Yes (new restricted file) |
| CO1.0.2 | Conformance test for L0 root | `tests/chain_tape_L0_constitution_root.rs` | No |

WP ref: § 5.L0; Inv 12.

### CO P1.1 — Anti-Oreo 3-layer module split (4-6 atoms)

| Atom | Scope | Files | STEP_B? |
|---|---|---|---|
| CO1.1.1 | Create `src/top_white/`, `src/middle_black/`, `src/bottom_white/`, `src/economy/`, `src/state/`, `src/transition/` skeletons with `mod.rs` | (above) | No (skeleton only) |
| CO1.1.2 | Move `src/wal.rs` → `src/bottom_white/ledger/wal.rs`; `src/ledger.rs` → `src/bottom_white/ledger/ledger.rs` (preserve content; only re-export from src/lib.rs) | (above) | **Yes (wal.rs + ledger.rs restricted)** |
| CO1.1.3 | Move `src/sdk/sandbox.rs` → `src/bottom_white/sandbox/exec.rs` | (above) | No |
| CO1.1.4 | Split `src/bus.rs` per § 9 of blueprint (5-way split) | `src/bus.rs` retired; new files in top_white/bottom_white/economy/middle_black | **Yes (bus.rs restricted, full 5-way split needs A/B parallel branch)** |
| CO1.1.5 | Split `src/kernel.rs` per § 9 of blueprint (3-way split) | `src/kernel.rs` retired; new in `state/`, `transition/`, `economy/` | **Yes (kernel.rs restricted, 3-way split needs A/B parallel branch)** |
| CO1.1.6 | Layer-leak conformance test | `tests/anti_oreo_layer_audit.rs` | No |

WP ref: § 3; Constitution Art. I.1.

### CO P1.2 — Q_t struct expansion to 9 components (3-4 atoms)

| Atom | Scope | Files | STEP_B? |
|---|---|---|---|
| CO1.2.1 | Define `QState` 9-field struct | `src/state/q_state.rs` (NEW) | Yes (new restricted file) |
| CO1.2.2 | Define `EconomicState` 9-sub-field struct | `src/state/q_state.rs` | Yes |
| CO1.2.3 | `replay_from_genesis` reconstruction routine | `src/state/q_state.rs` | Yes |
| CO1.2.4 | Conformance test: replay genesis + assert byte-identical Q_t | `tests/q_state_reconstruct.rs` + `tests/economic_state_reconstruct.rs` | No |

WP ref: Architecture § 4 + Economic § 2.

### CO P1.3 — gix substrate integration (Path B) (3 atoms)

> **Priority directive** (per Gemini CO P0.7 audit run 2 must-fix #3): CO1.3.1 is the **FIRST atom executed in CO Phase 1** (sequential, before P1.0-P1.2 atoms can finalize). Time-box: 5 working days. If spike fails (gix can't satisfy multi-parent or concurrent init requirements), **immediate pivot** to git2-rs fallback with a fresh CO1.3.1' spike atom; Plan v3.1 must be amended to v3.2 reflecting fallback before CO P1.0 begins.

| Atom | Scope | Files | STEP_B? |
|---|---|---|---|
| **CO1.3.1 (FIRST in P1)** | gix capability spike — multi-parent commit + concurrent runtime_repo init; **5-day time-box; failure → git2-rs pivot** | `src/bottom_white/tape/git_substrate.rs` (NEW) — spike only | Yes (new restricted) |
| CO1.3.2 | Per-cell `experiments/<problem>/<run_id>/runtime_repo/` initialization at evaluator.rs::on_cell_start | `src/bottom_white/tape/git_substrate.rs` + `experiments/.../bin/evaluator.rs` | Yes (evaluator.rs is restricted-adjacent) |
| CO1.3.3 | Conformance test: cell init creates valid git repo + first commit = constitution root | `tests/git_substrate_runtime_repo.rs` | No |

WP ref: Architecture § 13.2; Constitution Art. 0.4.

**Risk gate**: if gix spike fails (multi-parent or concurrent init blocked), fallback to git2-rs. Decision branch documented in atom CO1.3.1 PR.

### CO P1.4 — CAS layer (L3) (3-4 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.4.1 | `cas::store::put` / `get` — git blob backed | `src/bottom_white/cas/store.rs` (NEW) |
| CO1.4.2 | CAS object schema (cid + hash + type + creator + visibility) | `src/bottom_white/cas/schema.rs` (NEW) |
| CO1.4.3 | Existing `proposal_cid` references in WAL transitions point to CAS | `src/bottom_white/ledger/transition.rs` (modify) |
| CO1.4.4 | Conformance test for L3 | `tests/chain_tape_L3_cas.rs` |

WP ref: § 5.L3.

### CO P1.5 — Predicate Registry (L1) + Visibility Policy (6-8 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.5.1 | `PredicateRegistry` struct + load/store API | `src/top_white/predicates/registry.rs` (NEW) |
| CO1.5.2 | `Visibility::{Public, Private, CommitReveal}` enum + airgap guard | `src/top_white/predicates/visibility.rs` (NEW) |
| CO1.5.3 | Move `lean4_oracle.rs` → `top_white/predicates/acceptance/lean4_oracle.rs` (Visibility::Public) | (move) |
| CO1.5.4 | Format/safety predicates from old bus.rs → `top_white/predicates/acceptance/{format_check, safety_class}.rs` | (move + refactor) |
| CO1.5.5 | Settlement predicates: monetary_invariant + attribution_check + reputation_update | `src/top_white/predicates/settlement/*.rs` (NEW) |
| CO1.5.6 | `PredicateRunner` orchestrator | `src/top_white/predicates/runner.rs` (NEW) |
| CO1.5.7 | Goodhart shield airgap test (private predicate must not leak via error msg / log / retry count) | `tests/goodhart_shield.rs` |
| CO1.5.8 | Conformance test for L1 + Inv 6 + Inv 10 | `tests/chain_tape_L1_predicate_registry.rs` + `tests/economic_invariant_INV6_predicate_gated.rs` + `tests/economic_invariant_INV10_signal_vs_evaluator.rs` |

WP ref: § 5.L1 + § 9.4; Inv 6, 10.

### CO P1.6 — Tool Registry (L2) + Capability Classification (4-5 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.6.1 | `ToolRegistry` struct + capability/permission/determinism classification | `src/bottom_white/tools/registry.rs` (NEW) |
| CO1.6.2 | Refactor `TuringTool` trait → register-on-startup pattern | `src/bottom_white/tools/registry.rs` + existing `src/sdk/tool.rs` migration |
| CO1.6.3 | RTool / WTool split | `src/bottom_white/tools/{rtool, wtool}/*` |
| CO1.6.4 | Permission policy enforcement | `src/bottom_white/tools/permission.rs` (NEW) |
| CO1.6.5 | Conformance test for L2 | `tests/chain_tape_L2_tool_registry.rs` |

WP ref: § 5.L2.

### CO P1.7 — Transition Ledger (L4) — Full 11-Field Schema (5-7 atoms)

| Atom | Scope | Files | STEP_B? |
|---|---|---|---|
| CO1.7.1 | `TransitionTx` **12-field struct** (tx_id, **task_id**, parent_state_root, agent_id, read_set, write_set, proposal_cid, predicate_results, stake, signature, timestamp, status). **Codex CO P0.7 audit fix**: white paper Architecture § 5.L4 (lines 357-369) requires `task_id` linking each tx to a TaskMarket entry. Earlier "11-field" wording omitted it; now corrected. | `src/bottom_white/ledger/transition.rs` (NEW) | Yes |
| CO1.7.2 | Append API + WAL fsync semantics | `src/bottom_white/ledger/transition.rs` | Yes |
| CO1.7.3 | Replay from L4 reconstructs L5 + L6 | `src/bottom_white/materializer/state_db.rs` | No (new file) |
| CO1.7.4 | Migrate existing 5 EventType variants → TransitionTx subtypes (V-05 detail closure) | `src/bottom_white/ledger/transition.rs` | Yes |
| CO1.7.5 | step_transition pseudo-code from blueprint § 4 → actual fn | `src/transition/mod.rs` (NEW) | Yes (new restricted) |
| CO1.7.6 | Conformance test for L4 | `tests/chain_tape_L4_transition_ledger.rs` |
| CO1.7.7 | Inv 11 test | `tests/economic_invariant_INV11_chain_record_only.rs` |

WP ref: § 5.L4; Inv 11.

### CO P1.8 — Materialized State + Agent Read View (L5) (6-8 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.8.1 | `state_db` materialization (apply L4 → snapshot) | `src/bottom_white/materializer/state_db.rs` |
| CO1.8.2 | Task index | `src/bottom_white/materializer/indices.rs::task_index` |
| CO1.8.3 | Reputation index | `src/bottom_white/materializer/indices.rs::reputation_index` |
| CO1.8.4 | Error taxonomy index | `src/bottom_white/materializer/indices.rs::error_index` |
| CO1.8.5 | Price signal index | `src/bottom_white/materializer/indices.rs::price_index_view` |
| CO1.8.6 | Permission view (per-agent visibility filter) | `src/bottom_white/materializer/agent_view.rs` |
| CO1.8.7 | `prompt_builder` reads ONLY from agent_view (no leakage of private predicates) | `experiments/.../agents/*.rs` (multiple) |
| CO1.8.8 | Conformance test for L5 + Inv 10 | `tests/chain_tape_L5_materialized_state.rs` |

WP ref: § 5.L5 + § 9.2; Inv 10.

### CO P1.9 — Signal Indices (L6) (4-5 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.9.1 | Boolean signal index (pass/fail from acceptance predicates) | `src/bottom_white/signal_index/boolean_index.rs` |
| CO1.9.2 | Statistical signal index (price, reputation, scarcity) | `src/bottom_white/signal_index/stat_index.rs` |
| CO1.9.3 | Reuse count tracking | `src/bottom_white/signal_index/reuse_count.rs` |
| CO1.9.4 | `top_white::signals::price_broadcast::emit` writes to L6 | `src/top_white/signals/price_broadcast.rs` (NEW) |
| CO1.9.5 | Conformance test for L6 | `tests/chain_tape_L6_signal_indices.rs` |

WP ref: § 5.L6 + § 7 + § 8.

### CO P1.10 — Boolean vs Statistical Signal Explicit (3-4 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.10.1 | `Signal::Boolean` enum (pass/fail) | `src/top_white/signals/boolean.rs` (NEW) |
| CO1.10.2 | `Signal::Statistical` enum (price, reputation, scarcity, explore/exploit) | `src/top_white/signals/statistical.rs` (NEW) |
| CO1.10.3 | Refactor `bus.rs::reputation_distribution` → `top_white::signals::statistical` | (move) |
| CO1.10.4 | Conformance test: signal dichotomy + Constitution Art. II.2.1 statistical signals presence | `tests/signal_dichotomy.rs` + `tests/statistical_signals_complete.rs` |

WP ref: § 7.

### CO P1.11 — Safety vs Creation Domain Fail-Policy (2-3 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.11.1 | `SafetyClass::{Safety, Creation}` per predicate | `src/top_white/predicates/registry.rs` (extend) |
| CO1.11.2 | Fail-closed for Safety, fail-open-with-signal for Creation | `src/top_white/predicates/runner.rs` (extend) |
| CO1.11.3 | Conformance test | `tests/safety_creation_dichotomy.rs` |

WP ref: § 7.2.

### CO P1.12 — V-01..V-24 + E-01..E-04 Closure (covered by P1.5-P1.9 atoms)

Audit per `TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md` + `ECONOMIC_MECHANISM_AUDIT_2026-04-26.md`. Conformance tests:

```
tests/tape_canonical_V01..V24.rs       (24 tests)
tests/economic_audit_E01..E04.rs       (4 tests)
```

These are completed as side-effects of P1.5-P1.9 atom commits. Per atom: a check-off line in commit message.

### CO P1.13 — TRACE_MATRIX_v3 (3 atoms)

| Atom | Scope | Files |
|---|---|---|
| CO1.13.1 | TRACE_MATRIX_v3 markdown skeleton with all rows from blueprint § 6 | `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md` (NEW) |
| CO1.13.2 | Pre-commit hook R-022 — every commit touching `src/{top_white,middle_black,bottom_white,economy,state,transition}/*.rs` MUST update matrix | `.git/hooks/pre-commit` + `scripts/check_trace_matrix_updated.sh` |
| CO1.13.3 | Bidirectional conformance test (rust syn + markdown parser) | `tests/trace_matrix_v3_bidirectional.rs` |

DO-178C standard.

### CO P1.14 — Phase 1 Exit Ceremony (1 atom)

| Atom | Scope |
|---|---|
| CO1.14.1 | Dual external audit on Phase 1 + heldout-49 sealing re-validation + heldout-54 reservation honored. PASS/PASS gate to CO P2. |

---

## § 4 CO Phase 2 — RSP Economy (8-10 weeks, 50-70 atoms)

Goal: all of Economic chapter §§ 1-21 operationalized. White paper Phase 2 (Internal Task Market) live.

### CO P2.0 — Inv 4 (No Post-Mint) Pre-condition (1 atom)

| Atom | Scope | File |
|---|---|---|
| CO2.0.1 | `Coin` mint API guarded — only `genesis_init` may mint base Coin | `src/economy/escrow_vault.rs` (will be created in P2.2) — placeholder + `tests/economic_invariant_INV4_no_post_mint.rs` |

### CO P2.1 — TaskMarket (4 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.1.1 | `TaskMarket::publish_task` (locks bounty) | `src/economy/task_market.rs` (NEW) |
| CO2.1.2 | Per-task bounty contract (replaces global bounty_market) | `src/economy/task_market.rs` |
| CO2.1.3 | Price broadcast from `top_white::signals::price_broadcast::emit` consumed by TaskMarket | (cross-link) |
| CO2.1.4 | Smoke test | `tests/task_market_smoke.rs` |

WP ref: Economic § 19, § 21 Escrow term.

### CO P2.2 — EscrowVault (3 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.2.1 | `EscrowVault` 4-pool structure (bounty + stake + deferred + royalty) | `src/economy/escrow_vault.rs` (NEW) |
| CO2.2.2 | Lock/unlock/transfer atomicity | `src/economy/escrow_vault.rs` |
| CO2.2.3 | Inv 3 conformance test (escrow-only payouts) | `tests/economic_invariant_INV3_escrow_only.rs` |

WP ref: Economic § 19; Inv 3.

### CO P2.3 — ContributionLedger (5-6 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.3.1 | `WorkTx` schema (extends TransitionTx with stake direction = YES_E) | `src/economy/contribution_ledger.rs` (NEW) |
| CO2.3.2 | `VerifyTx` schema (verifier reputation/bond stake) | `src/economy/contribution_ledger.rs` |
| CO2.3.3 | `ChallengeTx` schema (NO_E stake) | `src/economy/contribution_ledger.rs` |
| CO2.3.4 | `ReuseTx` schema (downstream tool reuse → royalty trigger) | `src/economy/contribution_ledger.rs` |
| CO2.3.5 | Append + signature verification | `src/economy/contribution_ledger.rs` |
| CO2.3.6 | Inv 1 + Inv 2 conformance test | `tests/economic_invariant_INV1_no_thinking_reward.rs` + `tests/economic_invariant_INV2_no_direct_collect.rs` |

WP ref: Economic § 6 + § 13; Inv 1, 2.

### CO P2.4 — AttributionEngine — Contribution DAG (6-8 atoms; +1 spike atom per Gemini CO P0.7 audit Q3)

**Pre-implementation gate**: per Gemini CO P0.7 audit run 1 Q3 CHALLENGE — Inv 8 determinism is currently aspirational without a specified algorithm. CO2.4.0 spike (below) is BLOCKING for CO2.4.1+. No implementation begins until algorithm specification PASSes dual audit.

| Atom | Scope | File |
|---|---|---|
| **CO2.4.0** (NEW) | **DESIGN SPIKE**: specify provably deterministic Contribution DAG **CONSTRUCTION** algorithm (not just weight function — Codex CO P0.7 audit Q3 flagged that "same DAG → same weights" tests the easy half; the hard half is "same inputs → same DAG"). Inputs = L4 read_set/write_set tuples (deterministic) + Tx parent state roots + chronological tx ordering by (timestamp, tx_id) tiebreak. Outputs = canonical DAG (same inputs → byte-identical adjacency list AND byte-identical weights). Must address: (a) concurrent write_tx ordering, (b) multi-parent merge ambiguity, (c) citation transitivity, (d) edge type discrimination (builds-on vs cites vs reuses) from L4 fields ONLY (no agent self-declaration), (e) cycle detection (DAG must be acyclic). **Deliverable**: 1-page algorithm spec + 3-tx adversarial worked example with byte-identical expected output for two independent runs. Dual audit gate (Codex + Gemini PASS/PASS) before CO2.4.1 starts. | `handover/architect-insights/INV8_DAG_DETERMINISM_SPEC_2026-04-26.md` (drafted in CO P2.4 entry) |

| Atom | Scope | File |
|---|---|---|
| CO2.4.1 | `ContributionDAG` graph structure (nodes = work_tx, edges = builds-on/cites/reuses) | `src/economy/attribution_engine.rs` (NEW) |
| CO2.4.2 | Deterministic weight computation (path-based + reputation-weighted) | `src/economy/attribution_engine.rs` |
| CO2.4.3 | Attribution policy hash locked at task creation (anti-post-hoc-tweak) | `src/economy/attribution_engine.rs` |
| CO2.4.4 | DAG citation = git multi-parent merge commit (per Path B) | cross-link to `git_substrate.rs` |
| CO2.4.5 | Reuse royalty edge insertion | `src/economy/attribution_engine.rs` |
| CO2.4.6 | Determinism conformance test (same DAG → same weights, byte-identical) | `tests/attribution_engine_determinism.rs` |
| CO2.4.7 | Inv 8 conformance test | `tests/economic_invariant_INV8_dag_attribution.rs` |

WP ref: Economic § 6 + § 19; Inv 8.

**Design risk** (per blueprint § 11): subjectivity in DAG construction is the biggest threat to Inv 8. CO2.4.2-CO2.4.3 must enforce deterministic build from L4 read_set/write_set ONLY, not from agent self-declaration.

### CO P2.5 — ChallengeCourt (4-5 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.5.1 | `ChallengeWindow` open/close on provisional reward | `src/economy/challenge_court.rs` (NEW) |
| CO2.5.2 | Challenger NO_E stake collection | `src/economy/challenge_court.rs` |
| CO2.5.3 | Counterexample acceptance predicate | `src/top_white/predicates/settlement/counterexample.rs` (NEW) |
| CO2.5.4 | Slash + rollback on successful challenge | `src/economy/challenge_court.rs` |
| CO2.5.5 | Inv 7 conformance test (provisional → final) | `tests/economic_invariant_INV7_provisional_then_final.rs` |

WP ref: Economic § 7.2 + § 14.3; Inv 7.

### CO P2.6 — SettlementEngine — 3-Layer Rewards (4-5 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.6.1 | Immediate bounty payout (post-acceptance, post-challenge-survival) | `src/economy/settlement_engine.rs` (NEW) |
| CO2.6.2 | Deferred impact bonus (downstream-utility metric over N-tx window) | `src/economy/settlement_engine.rs` |
| CO2.6.3 | Reuse royalty stream (from royalty_graph_t edges) | `src/economy/settlement_engine.rs` |
| CO2.6.4 | `finalize_reward` impl of Economic § 21 final formula | `src/economy/settlement_engine.rs::finalize_reward` |
| CO2.6.5 | Conformance test for final formula | `tests/final_reward_formula.rs` + `tests/economic_invariant_INV2_no_direct_collect.rs` |

WP ref: Economic § 5 + § 21; Inv 2.

### CO P2.7 — Verifier + Challenger Roles (5-6 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.7.1 | `Verifier` agent — verify_tx submission | `experiments/.../agents/verifier.rs` (NEW) |
| CO2.7.2 | `Challenger` agent — challenge_tx submission | `experiments/.../agents/challenger.rs` (NEW) |
| CO2.7.3 | Solver agent code refactored from current monolith | `experiments/.../agents/solver.rs` (NEW from refactor) |
| CO2.7.4 | Builder agent (creates reusable wtool) | `experiments/.../agents/builder.rs` (NEW) |
| CO2.7.5 | Role self-selection logic via Librarian board (memory: feedback_emergent_roles) | `src/middle_black/role_self_select.rs` (NEW) |
| CO2.7.6 | Inv 5 conformance test (YES/NO event-bound rights) | `tests/economic_invariant_INV5_yes_no_event_bound.rs` |

WP ref: Economic § 7.

### CO P2.8 — CTF Stake Symmetry (3 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.8.1 | YES_E / NO_E stake instantiation per task event | `src/economy/contribution_ledger.rs` (extend) |
| CO2.8.2 | Stake symmetry math invariant (Solver-YES = Challenger-NO at task close) | `src/economy/settlement_engine.rs` (extend) |
| CO2.8.3 | Conformance test | `tests/ctf_stake_symmetry.rs` |

WP ref: Economic § 8 (CTF basic law).

### CO P2.9 — ReputationIndex (Proper, Not Solve-Count Alias) (2-3 atoms)

| Atom | Scope | File |
|---|---|---|
| CO2.9.1 | `Reputation` struct (success / failure / reuse / verifier_quality components, non-transferable) | `src/economy/reputation_index.rs` (NEW) |
| CO2.9.2 | Update API on every settlement_tx | `src/economy/reputation_index.rs` |
| CO2.9.3 | Inv 9 conformance test (immutable, can't be transferred or substitute predicates) | `tests/economic_invariant_INV9_reputation_immutable.rs` |

WP ref: Economic § 6; Inv 9.

### CO P2.10 — E-01..E-04 Closures (4 atoms)

| Atom | Scope | Detail |
|---|---|---|
| CO2.10.1 | E-01 production-default-on (TAPE_ECONOMY_V2 retired or default=1) | `experiments/.../scripts/*` + env handling |
| CO2.10.2 | E-02 jsonl summary completeness | `experiments/.../bin/evaluator.rs` summary block |
| CO2.10.3 | E-03 naming hygiene (TAPE_ECONOMY_V2 → final canonical name) | grep + rename |
| CO2.10.4 | E-04 founder grant Law-2 reconciliation | `src/economy/escrow_vault.rs` + `tests/economic_audit_E04_founder_grant_law2.rs` |

WP ref: ECONOMIC_MECHANISM_AUDIT_2026-04-26.md.

### CO P2.11 — RSP MVP-1 Deployment as LedgerTape (2 atoms)

| Atom | Scope |
|---|---|
| CO2.11.1 | Phase 1 → Phase 2 promote: ledger.jsonl + SQLite predicates → in-process LedgerTape with full RSP |
| CO2.11.2 | Conformance test: full task lifecycle (publish → 3 solver attempts → 1 verifier → 1 challenger → settle → 3-layer rewards) |

WP ref: Economic § 17 Phase 2.

### CO P2.12 — Phase 2 Exit Ceremony (1 atom)

Dual external audit + 12 invariant conformance + 24 V + 4 E pass + heldout sealing intact.

---

## § 5 v4 Out-of-Scope (Phase 3+)

Per blueprint § 7 and Economic § 20, **Phase 3-5 are v4.x or v5**:

- Permissioned ChainTape (Hyperledger Fabric) → v4.x
- State Channels → v4.x
- Optimistic Rollup → v4.x
- ZK / Validity Proof predicates → v4.x
- Oracles → v4.x
- Public AGI Market with cross-domain reputation → v5

**MetaTape (CO Phase 3)** = ArchitectAI/JudgeAI runtime — ArchitectAI rec is **defer to v4.1** (D4 = B).

---

## § 6 Atom Total + Wall Clock

> **Cost amendment** (per Gemini CO P0.7 audit run 2 Q7 CHALLENGE + CO_P0_AMENDMENT_v1 § 2): the original $250-500 figure assumed Codex/Gemini as auditors only. Tri-model co-execution per `TRI_MODEL_ORCHESTRATION_PROTOCOL` § 5 raises the authoritative budget to **$435-950** (+$22-66 for Hard rule 2 mandatory Claude auditor reviews on Codex-implemented atoms). The $250-500 figure below is **deprecated**; refer to the right column.

| Phase | Atoms | Weeks | External audit cost (deprecated) | External audit cost (authoritative, tri-model) |
|---|---|---|---|---|
| CO P0 | 7 | 1 | $50-100 | $50-100 |
| CO P1 | ~62 (this doc) | 8-10 | $100-200 | $200-400 |
| CO P2 | ~64 (incl. CO2.4.0 spike) | 8-10 | $100-200 | $200-400 |
| Hard rule 2 mandatory Claude reviewer ×22 STEP_B atoms | — | — | $0 | $22-66 |
| **v4 total** | **~133** | **17-21** | ~~$250-500~~ | **$435-950 (mid $700)** |

**MVP option** (D5=B): drops CO P2.5 (ChallengeCourt full), CO P2.7 partial (no Challenger role), CO P2.8 (CTF symmetry), CO P2.4 partial (no royalty edges). Estimated saving: ~20 atoms + 4-5 weeks. Risk: Inv 5, 7, 8 conformance partial → forfeits "full RSP" claim.

ArchitectAI rec: **D5 = A (full RSP)**. The 12 invariants are interdependent; partial coverage means some invariants violated by construction.

---

## § 7 STEP_B Atom Inventory

Atoms touching restricted files (`src/{bus,kernel,wal,ledger}.rs` or new restricted: `src/state/q_state.rs`, `src/transition/*.rs`, `src/bottom_white/tape/*.rs`, `src/bottom_white/ledger/*.rs`, `src/economy/escrow_vault.rs`, `src/economy/settlement_engine.rs`):

```
CO0.6 (genesis manifest)
CO1.0.1 (chain_tape.rs)
CO1.1.2 (wal/ledger move)
CO1.1.4 (bus.rs split)              ← largest STEP_B; A/B 5-way parallel branches
CO1.1.5 (kernel.rs split)           ← largest STEP_B; A/B 3-way parallel branches
CO1.2.1, CO1.2.2, CO1.2.3 (q_state.rs)
CO1.3.1, CO1.3.2 (git_substrate.rs)
CO1.7.1, CO1.7.2, CO1.7.4 (transition.rs)
CO1.7.5 (transition/mod.rs)
CO2.2.* (escrow_vault.rs)
CO2.6.* (settlement_engine.rs)
```

Estimated **~22 STEP_B atoms × $5-10 audit each** = $110-220 inside the $250-500 total budget.

---

## § 8 Decision Mapping (D1-D6 → Plan Branch)

| Decision | Plan branch implication |
|---|---|
| **D1 (PREREG)** = A (PAUSE) | freeze heldout-54 reservation; PPUT-CCL Phase C deferred to post-CO P2 |
| **D1** = B (NEGATIVE) | declare arc failed; new PREREG written post-CO P2; heldout-54 NOT reserved |
| **D1** = C (MVP-pivot) | run abbreviated PPUT-CCL Phase C **after CO P1 only** (no full RSP); compromise |
| **D2** = A (full text in const) | constitution.md ~2000 lines longer; Art. 0.5 = full white paper text |
| **D2** = B (pointer + 6 axioms) | constitution Art. 0.5 ≤ 50 lines; pointer to whitepapers/ |
| **D3** = A (deprecate TFR v1) | CO0.5 atom: rename + add LEGACY banner |
| **D3** = B (delete) | NOT recommended |
| **D4** = A (CO P3 in v4) | +4-6 weeks + 40+ atoms; v4 total ~22 weeks |
| **D4** = B (defer P3 to v4.1) | v4 = CO P0+P1+P2 only, ~17-21 weeks |
| **D5** = A (full RSP) | all of CO P2 atoms above |
| **D5** = B (RSP MVP) | drops P2.5, partial P2.7, P2.8, partial P2.4 (~20 atoms saved) |
| **D5** = C (RSP min) | drops P2.5, P2.7, P2.8, P2.4 entirely (~30 atoms saved; only Inv 1, 2, 3, 4, 6, 11, 12 covered — 5 invariants violated) |
| **D6** = A (full audit) | $250-500 |
| **D6** = B (reduced) | $150-250; STEP_B atom audits internal only |

**ArchitectAI default recommendation** (used unless user overrides): **D1=C, D2=B, D3=A, D4=B, D5=A, D6=A**.

---

## § 9 Risk Register (Updated from v3 § 11)

Same 10 risks as v3 § 11. Two updates:

- **Risk #3 (AttributionEngine subjectivity)** — mitigation strengthened: CO P2.4 atoms enforce deterministic build from L4 read_set/write_set; subjectivity test in CO P2.4.6.
- **Risk #11 (NEW)** — Anti-Oreo split via 5-way / 3-way parallel branches (CO1.1.4 + CO1.1.5) is the highest-risk individual atom pair. Mitigation: dedicate 1 full week per atom; do not rush.

---

## § 10 What v3.1 Doesn't Yet Solve

- **Sprint dependency graph** (atom → atom dependency arrows): drafted at CO P0 entry, refined at each CO P1 sub-phase entry.
- **Per-atom test coverage threshold**: stated as "≥1 conformance test per atom" but no line-coverage / mutation-coverage threshold.
- **Cross-cell pollution prevention** in Path B git substrate: needs CO P1.3 spike output before atom CO1.3.2 design finalizes.
- **Deferred bonus computation window** (CO P2.6.2): needs design choice (rolling N-tx vs absolute time vs solve-count window). Defer to CO P2.6 entry; user may override at that gate.

---

## § 11 Self-Audit on This Plan

**What plan v3.1 commits to**:
- Every white paper § (architecture + economic) → at least one named atom
- Every constitution Article (0, 0.1, 0.2, 0.3, 0.4, I.1, I.2, II.2.1, IV) → at least one conformance test
- Every economic invariant Inv 1-12 → exactly one conformance test
- 132 atoms ± 10, 17-21 weeks ± 2

**What plan v3.1 deliberately does not commit to**:
- Individual atom PR diffs (drafted at sprint launch)
- Exact conformance test code (drafted in test atom)
- Constitution Art. 0.5 final wording (depends on D2)
- PREREG_AMENDMENT_v2 final wording (depends on D1)

**What plan v3.1 is honest about**:
- This is the FIRST plan in v4 history that maps every white paper § to a code symbol. v1-v3 did not.
- Anti-Oreo file split (CO1.1.4 + CO1.1.5) is the architectural gravity center; if it slips, every later atom slides.
- gix Path B is a calculated bet — git2-rs fallback exists but adds complexity (CO1.3.1 spike must validate).

— ArchitectAI, 2026-04-26
