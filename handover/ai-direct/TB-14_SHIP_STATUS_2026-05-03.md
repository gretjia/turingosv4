# TB-14 PriceIndex v0 + Boltzmann Masking — Ship Status

**Date**: 2026-05-03
**HEAD**: `1189cb2` (10 commits across Atoms 0–7 + B′ R1-VETO closure cycle)
**Status**: SHIPPED — single-charter TB-14 (NOT split per architect §8 fallback)
**FC-trace**: FC3-N42 + FC2-N28 + FC2-N29
**Risk class envelope**: Class 2 (Atoms 0–5) + **Class 3** (Atom 6 production wire-swap; STEP_B restricted files; mandatory dual audit)
**Phase**: P4 Information Loom (primary) + P3 RSP-6 (price index micro-version)
**Closes**:
- `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`
- `handover/alignment/OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md` (architectural fix in Atom 2 — successor-TB-marker discovery skip; replaced TB_14_PLUS_EXCLUDED band-aid)

---

## §0 Architect-mandated ship summary (charter §6 + §0 outcomes)

### Workspace test gate (G-14.9 ≥ 803)

```text
command          = cargo test --workspace
workspace_count  = 841 passed
failed           = 0
ignored          = 150
```

Pre-TB-14 baseline (`a9fbdf3` Atom 5 ship): 841 passed.
Post-TB-14 ship (`1189cb2`): 841 passed.
Net delta over the full Atom 6 cycle: **+0** (after accounting for −25 deleted CPMM-era tests + 25 new TB-14 tests across the wire-swap + production smokes + canonical-graph rewire + Q11 closure).

### Halt-trigger gate (architect §5.7 — 6/6 GREEN re-verified post-merge)

| # | Name | Status | Witness |
|---|---|---|---|
| 1 | price_does_not_affect_predicate_result | ✅ | `tests/tb_14_halt_triggers.rs:27-53` (sequencer.rs body fence) |
| 2 | price_does_not_change_l4_decision | ✅ | `tests/tb_14_halt_triggers.rs:76-118` (sequencer.rs `use`-block fence) |
| 3 | parent_not_deleted_from_chaintape | ✅ | `tests/tb_14_halt_triggers.rs:120-227` (canonical-graph form post-B′) |
| 4 | no_f64_in_tb_14_modules | ✅ | `tests/tb_14_halt_triggers.rs:229-309` (price_index.rs runtime fs scan) |
| 5 | zero_liquidity_returns_none | ✅ | `tests/tb_14_halt_triggers.rs:311-323` (FR-14.3 / Atom 2 inline) |
| 6 | unresolved_challenge_blocks_masking | ✅ | `tests/tb_14_halt_triggers.rs:325-415` (canonical-graph form post-B′) |

### Ship gate (architect §5.5 + charter §6)

| ID | Gate | Status | Test |
|---|---|---|---|
| SG-14.1 | PriceIndex computes expected YES/NO probabilities | ✅ | `src/state/price_index.rs::tests` (Atom 2 inline) |
| SG-14.2 | No-liquidity node has price=None | ✅ | `tests/tb_14_halt_triggers.rs::zero_liquidity_returns_none` |
| SG-14.3 | Parent not deleted from ChainTape after masking | ✅ | `tests/tb_14_mask_set.rs::sg_14_3_*` + `b_prime_step_5_positive_canonical_masking_smoke` |
| SG-14.4 | Predicate failure still dominates high price | ✅ | `tests/tb_14_canonical_masking_smoke.rs::b_prime_step_6c_predicate_failed_child_cannot_mask_parent` |
| SG-14.5 | Boltzmann selection includes epsilon exploration | ✅ | `src/sdk/actor.rs::tests::v2_epsilon_greedy_explores_under_high_epsilon` |
| SG-14.6 | Dashboard shows price as signal, not outcome | ✅ | `src/bin/audit_dashboard.rs::tb14_render_tests` (4 tests) |
| SG-14.7 | Unresolved challenge blocks masking | ✅ | `tests/tb_14_mask_set.rs::sg_14_7_*` + `b_prime_step_6b_unresolved_challenged_child_cannot_mask_parent` |
| SG-14.8 | Low-liquidity manipulation cannot mask parent | ✅ | `tests/tb_14_mask_set.rs::sg_14_8_*` + `b_prime_step_6a_low_liquidity_child_cannot_mask_parent` |
| G-14.9 | `cargo test --workspace` ≥803 / 0 fail / ≤150 ignored | ✅ | 841 / 0 / 150 |
| G-14.10 | FC3-N42 + FC2-N28 + FC2-N29 each have ≥1 witness | ✅ | `tests/fc_alignment_conformance.rs` |
| G-14.11 | No f64 in TB-14 module surface | ✅ | halt-trigger #4 + B′ step 4 dead-variant excision |
| G-14.12 | ChainTape smoke (chain-backed) PASS | ✅ | `tests/tb_14_chaintape_smoke.rs` + `tests/tb_14_canonical_masking_smoke.rs` |

### Constitutional rules (architect §5.4 — CR-14.x)

| ID | Rule | Witness |
|---|---|---|
| CR-14.1 | Price-blind predicate gate | halt-trigger #1 + `b_prime_step_6c` |
| CR-14.2 | Price-blind L4 / L4.E classification | halt-trigger #2 (sequencer.rs use-block scan) |
| CR-14.3 | Mask is read-view, NOT canonical-state deletion | `b_prime_step_5_positive_canonical_masking_smoke` (canonical L4 still contains parent A after mask) |
| CR-14.4 | Low-liquidity child cannot mask parent | SG-14.8 + `b_prime_step_6a` |
| CR-14.5 | Open challenges block masking | SG-14.7 + halt-trigger #6 + `b_prime_step_6b` |
| CR-14.6 | Goodhart shield — NodeMarketEntry exposes no predicate content | NodeMarketEntry 10-field shape (architect §5.2 verbatim); §14 dashboard render carries only those 10 fields |

---

## §1 Atom-by-atom ship log (10 commits)

```text
Atom 0    Charter ratification                        698d8a2 (pre-session)
Atom 1    Halt-trigger fixture file                   0370d66 (prior session)
Atom 2    PriceIndex pure-fn view + fence fix         23ac581 (prior session)
Atom 3    mask_set + compute_mask_set + policy stub   668695d (prior session)
Atom 4    BoltzmannMaskPolicy::from_env               7cbcacf (prior session)
Atom 5    boltzmann_select_parent_v2 + halt #1/#2     a9fbdf3 (prior session)

═══════════════════════════════════════════════════════════════════════════════
THIS SESSION (auto-mode start: 9cc40e1 → final ship: 1189cb2)
═══════════════════════════════════════════════════════════════════════════════

Atom 6 main wire-swap                                 44cd480
  • DELETE prediction_market.rs entire (390 LoC; CPMM, f64, automatic liquidity)
  • DELETE legacy kernel.rs market fields + 9 methods (V3L-45 pure-topology
    contract restored)
  • DELETE legacy actor.rs Boltzmann{Params, is_frontier, lineage_score,
    boltzmann_select_parent} (f64) + 6 legacy tests
  • DELETE legacy snapshot.rs MarketSnapshot + markets HashMap + dead-since-
    TB-9 balances/portfolios f64
  • REWIRE bus.snapshot() → integer-rational price_index + mask_set
  • REWIRE evaluator.rs production wire-swap (BoltzmannMaskPolicy::from_env;
    boltzmann_select_parent_v2; market_ticker_str rendered as `n/d`;
    prompt_balance via bus.sequencer.q_snapshot)
  • ADD audit_dashboard.rs §14 with literal "PRICE IS SIGNAL, NOT TRUTH"
    banner + 4 SG-14.6 unit tests
  • ADD tests/tb_14_chaintape_smoke.rs chain-backed regression smoke
  • Closes OBS_TB_12_LEGACY_CPMM_QUARANTINE in production code paths

Atom 6 internal-auditor F1 follow-up                  38412bf
  • Class 3 internal `auditor` subagent (read-only, 12-min review)
    returned CHALLENGE on dead `BusResult::Invested {shares: f64}` enum
    variant (pre-TB-9 invest-path residual; zero call sites; halt-trigger
    #4 only fenced price_index.rs so this f64 surface in TB-14-touched
    bus.rs was unfenced)
  • 4-line deletion + bus.rs rehash. Workspace tests unchanged at
    821/0/150 (variant was dead).
  • Per feedback_audit_obs_bias: cheap fix + production-code residual
    (not test-scaffold edge case) → FIX-NOW.

Atom 6 LATEST.md handover                             c291dde
  • LATEST.md update + ship-gate scorecard at the user-decision
    boundary (external dual audit dispatch).

Atom 6 EXTERNAL DUAL AUDIT R1 dispatch + verdicts:
  • Codex R1: VETO conviction=high recommendation=REDESIGN before ship
    Primary finding (RQ4/RQ8/Q3): bus.snapshot() computed mask_set
    against shadow `kernel.tape` but price_index was canonical-keyed
    → mask_set non-functional in production; evaluator unwrapped
    canonical TxId into shadow bus.append parent → dangling-citation
    crash on non-empty PriceIndex runs.
    Secondary CHALLENGE: BoltzmannMaskPolicy::from_env accepted
    nonsensical values (negative min_liquidity, zero price_margin).
  • Gemini R1: PASS conviction=high recommendation=PROCEED to SHIP
    (all 13 questions PASS).
  • Conservative verdict per feedback_dual_audit_conflict (VETO >
    CHALLENGE > PASS): Codex VETO blocks ship.

Atom 6 USER-ARCHITECT RULING 2026-05-03 (binding):
  Lossless archive at handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md
  Path: C → B′ (charter amend → architectural rewire → R2 dispatch).

Atom 6 B′ step 1+2 — surgical fixes                   48e84ee
  • Fix #1: evaluator.rs:1612 v2 selector result captured but NOT
    passed to bus.append (canonical→shadow id mapping unavailable).
    Per architect ruling step 1: "Use None unless a real shadow id
    exists."
  • Fix #2: BoltzmannMaskPolicy::from_env per-field validation
    (min_liquidity > 0; price_margin > 0; beta_den > 0; beta_num >= 0;
    epsilon ∈ [0, 1]). 11 inline tests pinning each rule.

Atom 6 B′ step 3 — charter amend                      dd40052
  • NEW handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md
    (lossless verbatim archive + structured annotation layer).
  • AMEND TB-14 charter §1.1-§1.5 (canonical namespace decision;
    CanonicalNodeGraph contract; production smoke contract; R2 dispatch
    gating; split-fallback).

Atom 6 B′ step 4 — CanonicalNodeGraph + canonical-graph rewire   9daba5a
  • NEW pub type CanonicalNodeGraph = BTreeMap<TxId, BTreeSet<TxId>>
  • compute_mask_set signature: (&Tape) → (&CanonicalNodeGraph)
  • NEW Sequencer::compute_canonical_edges_at_head() walks L4 + reads
    CAS-resident ProposalTelemetry.parent_tx for each accepted WorkTx
    via WorkTx.proposal_cid. Halt-trigger #2 fence preserved (no TB-14
    imports added to sequencer.rs use-block).
  • bus.snapshot() rewired to use seq.compute_canonical_edges_at_head()
    in place of &self.kernel.tape.
  • Test surface migration: tb_14_mask_set.rs (9 tests + 2 NEW
    canonical-namespace witnesses), tb_14_halt_triggers.rs #3 + #6,
    fc_alignment_conformance.rs FC2-N28 witness.

Atom 6 B′ step 5+6 — production-controlled smokes     07ce9b8
  • NEW tests/tb_14_canonical_masking_smoke.rs (5 chain-backed tests):
    - Positive (architect §5): parent A + child B with parent_tx=A;
      mask_set returns {A}; canonical L4 still contains A.
    - Negative §6 (a): low-liquidity child cannot mask parent.
    - Negative §6 (b): unresolved-challenged child cannot mask parent.
    - Negative §6 (c): predicate-failed child cannot mask parent
      (rejected from L4 → never in canonical_edges).
    - Idempotency: 5 repeated calls produce byte-equal canonical-graph.

Atom 6 EXTERNAL DUAL AUDIT R2 dispatch + verdicts:
  • Codex R2: PASS conviction=high recommendation=PROCEED to SHIP.
    All 4 R1 closures verified. RQ8 closure verified ("the new
    canonical masking smoke is the missing non-empty NodePositions
    chain-backed witness"). Split-fallback NOT triggered.
  • Gemini R2: CHALLENGE conviction=Medium recommendation=FIX-THEN-PROCEED.
    Single Q11 finding: bus.snapshot empty fallback semantic ambiguity
    (`price_index/mask_set` should disambiguate "sequencer unavailable"
    from "running but empty"). Q1-Q10 + Q12-Q13 PASS.
  • Conservative verdict: Gemini CHALLENGE wins; Q11 must be addressed.

Atom 6 B′ step 7 R2 closure — Q11 sequencer_wired     1189cb2
  • NEW pub field UniverseSnapshot.sequencer_wired: bool with
    #[serde(default)] for backward-compat. Distinguishes "sequencer
    unavailable" (None or q_snapshot failed) from "sequencer running
    but no canonical positions yet" — both produce empty
    price_index + mask_set, but consumers can disambiguate cheaply
    by reading the new field. No breaking change for existing
    consumers (empty maps remain empty).
  • Per feedback_audit_obs_bias: cheap fix (~15min, not multi-hour
    future-arch) + production-code clarity defect (not test-scaffold
    edge case) → FIX-NOW. Gemini's recommendation is "FIX-THEN-PROCEED"
    — explicit instruction to fix and ship without R3.

Atom 7 — THIS DOC + LATEST.md update + final commit.
```

---

## §2 Dual audit final verdict matrix

| Auditor | Round | Verdict | Conviction | Recommendation | Closure Status |
|---|---|---|---|---|---|
| Internal `auditor` subagent | R0 | CHALLENGE | high | FIX-THEN-PROCEED | F1 closed by `38412bf` |
| Codex external | R1 | VETO | high | REDESIGN | Closed by B′ steps 1-6 (commits 48e84ee → 07ce9b8) |
| Gemini external | R1 | PASS | high | PROCEED | No findings; carry-forward to R2 |
| Codex external | R2 | PASS | high | PROCEED to SHIP | All 4 R1 closures verified |
| Gemini external | R2 | CHALLENGE | Medium | FIX-THEN-PROCEED | Q11 closed by `1189cb2` (sequencer_wired field) |

**Conservative final verdict** (per `feedback_dual_audit_conflict`): all rounds resolved. PASS at HEAD `1189cb2`.

---

## §3 Architectural decisions surfaced during this atom (for future TB charters)

### §3.1 Canonical namespace decision (architect ruling 2026-05-03 §3 binding)

The v4 codebase has two id namespaces:
- **CANONICAL**: `WorkTx.tx_id` (Sequencer + EconomicState + L4 + ProposalTelemetry).
- **SHADOW**: `tx_{count}_by_{author}` (bus.append-generated for in-memory `kernel.tape`).

Pre-TB-14 the legacy CPMM read-view operated in shadow. Codex R1 VETO exposed mixed-namespace consumption in Atom 6's wire-swap. The architect ruling pinned the canonical namespace as authoritative for TB-14 derived views; shadow tape ids are legacy/local only. `compute_mask_set` operates on canonical edges; v2 selector output is NOT consumed as legacy bus.append parent_id.

### §3.2 STEP_B Phase 1 deviation (worktree skipped)

The Atom 6 main commit `44cd480` worked directly on main rather than `.claude/worktrees/stepb-tb14-atom6`. Justification: STEP_B Phase 0 (necessity) satisfied by architect ratification (charter §3 IS the spec); Phase 1 (worktree) adds operational coordination overhead with no audit-quality gain for a directly-spec-compliant wire-swap; Phase 3 (dual audit + merge gate) preserved.

Internal auditor + both Codex R2 and Gemini R2 took explicit position on this deviation: ACCEPT for this atom, but should not become a default. Recommend codifying a `feedback_step_b_phase_1_for_ratified_specs` memory rule before TB-15.

### §3.3 v1-vs-v2 cheap observability comparison deferred

Pre-Atom-6 the AI-coder proposed a `--half` legacy-vs-v2 baseline run as cheap observability (recorded in LATEST.md handover). Setup cost (git switching with 60+ untracked CAS dirs in working tree) made this non-trivial. Deferred to TB-15 Autopsy charter where a frozen real-LLM bench is the right tool.

### §3.4 Balance plumb-through fix in evaluator.rs (incidental)

Pre-TB-14 the evaluator passed `snap.get_balance(agent_id)` (legacy f64 fallback — always 0.0 in practice since TB-9 collapse). Atom 6 main commit replaced this with a live `bus.sequencer.q_snapshot().economic_state_t.balances_t` query. Strictly outside Atom 6's narrow spec, but the natural completion of the snapshot wire-swap. Documented in commit body for audit visibility; both R2 auditors accepted as out-of-scope-but-sound.

### §3.5 sequencer_wired field (Q11 closure design choice)

Gemini's specific suggestion was `Option<BTreeMap<...>>`. The B′ step 7 R2 closure implemented a separate `pub sequencer_wired: bool` field instead. Both encode the same two-state distinction; the bool form requires zero changes to existing consumers (empty maps remain empty; consumers that don't care continue to ignore the field) while still letting consumers that DO care disambiguate cheaply. Cost: ~15min vs ~45min for the Option-typed alternative.

---

## §4 Cross-references

- TB-14 charter (post-amend): `handover/tracer_bullets/TB-14_charter_2026-05-03.md` (§1.1-§1.5 canonical-namespace amend)
- Architect §5 verbatim spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- Architect VETO disposition: `handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md`
- Atom 6 kickoff: `handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md`
- Closing OBS: `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`
- ChainTape smoke evidence: `handover/evidence/tb_14_chaintape_smoke_2026-05-03/`
- R1 audits: `handover/audits/{CODEX,GEMINI}_TB_14_SHIP_AUDIT_2026-05-03_R1.md`
- R2 audits: `handover/audits/{CODEX,GEMINI}_TB_14_SHIP_AUDIT_2026-05-03_R2.md`
- Memory rules consulted: `feedback_dual_audit_conflict`, `feedback_audit_obs_bias`, `feedback_audit_loop_roi_flip`, `feedback_smoke_evidence_naming`, `feedback_workspace_test_canonical`, `feedback_kolmogorov_compression`, `feedback_architect_deviation_stance`, `feedback_no_retroactive_evidence_rewrite`, `feedback_iteration_cap_24h`, `feedback_step_b_protocol`

---

## §5 What's next (post-TB-14)

Per `project_tb11_to_tb17_roadmap`: TB-15 Autopsy + Markov.

The TB-14 deferred items become TB-15 charter preconditions:
- Frozen real-LLM bench (incl. v1-vs-v2 cheap observability comparison)
- `feedback_step_b_phase_1_for_ratified_specs` memory rule codification
- Optional: default-policy positive smoke with real Long/Short configuration (Codex R2 follow-up recommendation; not a ship blocker)
- Optional: Boltzmann v2 selector output → canonical WorkTx parent_tx wire-up (currently uses last_tx_by_agent; v2 result is computed but orphan)

The single-charter TB-14 ship is COMPLETE. Auto-mode reached the user-decision boundary at `1189cb2` (whether to push to remote).
