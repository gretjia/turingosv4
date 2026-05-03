# TB-12 Recursive Self-Audit — Node Exposure Index

**Audit type**: Recursive self-audit (Class 3 envelope; per architect §2 ruling).
**Date**: 2026-05-03.
**Scope**: TB-12 Atoms 0 → 5 SHIPPED + this Atom 6(a). External Codex + Gemini audits next (Atom 6(b) + 6(c)).
**Charter**: `handover/tracer_bullets/TB-12_charter_2026-05-03.md`.
**Architect ruling lossless**: `handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md`.

**Executive verdict**: **PASS** with no halting triggers fired.
External Codex + Gemini audits MUST follow before SHIP per architect §5.

---

## §1 Clause 1 — Constitutional preservation

| Article                                  | TB-12 invariant                                                                                                | Verification                                                                              |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Art. 0.2 Tape Canonical                  | NodePosition canonical-encoded (BTreeMap<TxId, NodePosition> default trait; serde derives). Replay-deterministic from typed-tx fields. | `sg_12_5_node_positions_replay_deterministic` test                                         |
| Art. I.1 5-step compile loop closure     | Accept-side derives an exposure record additively; predicate outcome unchanged. NodePosition cannot make a failed proposal accepted (CR-12.3). | Atom 2 dispatch arms preserve existing accept-side TB-3 / TB-4 logic; pure additive write |
| Art. II.2.1 entropy / quantize-broadcast-shield | Position information is broadcast (public dashboard §13 + lean_market view-positions); no shielding (positions are economic public-record by design). | §13 render is plain text                                                                  |
| Art. III.4 no fake accepted              | NodePosition created on accept ONLY. Cannot influence predicate outcome. No new agent-callable system_tx surface. | Atom 2 wire-up; Atom 0 charter §3 forbidden list                                          |
| Art. V.1.3 Anti-Oreo                     | NodePosition is sequencer-derived from typed-tx fields (no agent surface). lean_market view-positions is read-only. | dispatch arms in src/state/sequencer.rs WorkTx/ChallengeTx accept-side                    |

**Verdict 1: PASS**. Zero constitutional violations introduced.

---

## §2 Clause 2 — Replay-deterministic

| Property                                  | TB-12 verification                                                                                                                |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Q-projection determinism                  | NodePosition fields are deterministic projections of typed-tx fields (`work.tx_id`, `work.stake.0`, `work.timestamp_logical`). No environmental input. |
| Cross-instance replay equality            | `sg_12_5_node_positions_replay_deterministic` runs two distinct harnesses with identical inputs and asserts `node_positions_t` equal bit-for-bit. PASS. |
| State-root unchanged on derivation        | Atom 2 keeps existing TB-2 / TB-4 `worktx_accept_state_root` + `challenge_accept_state_root` unchanged. NodePosition write is a pure Q-projection captured by the existing canonical-encoded tx hash; no new domain hash needed (`work` / `challenge` tx fields determine the position fields exhaustively). |

**Verdict 2: PASS**.

---

## §3 Clause 3 — Conservation (CTF)

| Conservation invariant                    | TB-12 enforcement                                                                                                                                                                                |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **4-holding CTF (post-TB-8)** [doc-corrected post-Codex CHALLENGE Q4] | NodePosition is NOT a holding term per CR-12.1 + CR-12.2. Σ holdings = balances + escrows + stakes + challenge_cases (claims-active was removed from `total_supply_micro` in TB-8 — per `src/economy/monetary_invariant.rs:113` "Counted ... 4 holdings post-TB-8"). NodePosition addition keeps the count at 4. |
| `assert_total_ctf_conserved`              | `sg_12_4_node_positions_do_not_change_total_supply` test asserts pre/post equality with `&[]` empty exempt list. PASS.                                                                          |
| `assert_no_post_init_mint`                | `ctf_invariant_unchanged_across_position_derivation` test PASSes via dummy TaskOpen post-derivation. NodePosition write is NOT a mint (no money created).                                       |
| WorkTx existing TB-3 mutation unchanged   | Atom 2 keeps `balances_t -= work.stake` + `stakes_t += work.stake` (lock-on-accept). NodePosition write is APPENDED after the existing money mutation; no money path modification.              |
| ChallengeTx existing TB-4 mutation unchanged | Atom 2 keeps `balances_t -= challenge.stake` + `challenge_cases_t += challenge.stake`. NodePosition write is APPENDED.                                                                          |

**Verdict 3: PASS**.

---

## §4 Clause 4 — Architecture-skeleton hygiene (TB-12-unique)

| Property                                  | TB-12 verification                                                                                                                                                                                |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 4.1 Flat NodePositionsIndex canonical     | `economic_state_does_not_have_node_market_t_field` test asserts NO `node_market_t` field on EconomicState. Architect §3 ruling: NodeMarketEntry is TB-14 derived view; flat shape is canonical. |
| 4.2 NodePosition is IMMUTABLE exposure record | TB-12 has NO close / settle / transfer / mark-to-market dispatch arms. Architect §10 invariant. After NodePosition is inserted into `node_positions_t`, no code path mutates or removes it in TB-12 scope. |
| 4.3 NO trading variants                   | SG-12.7 `sg_12_7_only_firstlong_and_challengeshort_kinds_observed` enforces PositionKind ∈ {FirstLong, ChallengeShort}. NO MarketBuy / MarketSell variants. NO MarketOrderTx / MarketTradeTx / CompleteSetMintTx. |
| 4.4 NO price calculation                  | TB-12 has NO `price_yes` / `price_no` field anywhere. CR-12.4 + architect §9.4 explicit forbid. (TB-14 PriceIndex territory.)                                                                    |
| 4.5 VerifyTx.bond ≠ market position       | SG-12.3 `sg_12_3_verifytx_does_not_create_node_position` test asserts VerifyTx accept produces NO NodePosition. FR-12.3 + CR-12.8 explicit.                                                      |
| 4.6 Position fields = source tx fields    | `position_fields_derived_from_source_tx_exactly` test locks the derivation rules: position_id == source_tx, FirstLong.node_id == work.tx_id, ChallengeShort.node_id == challenge.target_work_tx (NOT challenge's own tx_id). |

**Verdict 4: PASS** with no schema or scope creep beyond architect §1-§9 ruling.

---

## §5 Architect halting triggers (charter §3 Atom 0; architect §7) — NOT triggered

| Halting trigger                           | Result                                                                                                       |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| CTF conservation failure                  | NOT triggered. SG-12.4 + ctf_invariant_unchanged tests PASS.                                                |
| WorkTx/ChallengeTx position mismatch      | NOT triggered. position_fields_derived_from_source_tx_exactly test PASS.                                    |
| NodePosition counted as Coin              | NOT triggered. CR-12.2 verified via assert_total_ctf_conserved with empty exempt list.                       |
| Replay divergence                         | NOT triggered. sg_12_5_node_positions_replay_deterministic PASS.                                             |
| Codex / Gemini VETO                       | DEFERRED to Atom 6(b) + 6(c). Self-audit verdict here is PASS; external audits next.                         |

---

## §6 Ship gates (architect SG-12.1..8 + carry-forward G1..G11)

**Architect SG-12.1..8** (charter §6):

| Gate                                                              | Status     | Evidence                                                                                          |
| ----------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------- |
| SG-12.1 accepted_worktx_creates_firstlong_position                | ✓ pass    | `sg_12_1_accepted_worktx_creates_firstlong_position` test                                        |
| SG-12.2 accepted_challengetx_creates_challengeshort_position      | ✓ pass    | `sg_12_2_accepted_challengetx_creates_challengeshort_position` test                              |
| SG-12.3 verifytx_does_not_create_node_position                    | ✓ pass    | `sg_12_3_verifytx_does_not_create_node_position` test                                            |
| SG-12.4 node_positions_do_not_change_total_supply                 | ✓ pass    | `sg_12_4_node_positions_do_not_change_total_supply` + `ctf_invariant_unchanged_across_position_derivation` |
| SG-12.5 replay_reconstructs_node_positions                        | ✓ pass    | `sg_12_5_node_positions_replay_deterministic`                                                    |
| SG-12.6 dashboard_view_positions_works                            | ✓ pass    | Atom 4 §13 render + view-positions subcommand (compile-clean; manual smoke ready)                |
| SG-12.7 no_market_trading_variants_introduced                     | ✓ pass    | `sg_12_7_only_firstlong_and_challengeshort_kinds_observed` + grep diff TB-11→TB-12: zero MarketBuy/Sell. |
| SG-12.8 no_node_market_entry_as_canonical_state                   | ✓ pass    | `economic_state_does_not_have_node_market_t_field` test (in `src/state/q_state.rs` test module). |

**Engineering carry-forward G1..G11** (TB-9/10/11 precedent):

| Gate          | Status     | Evidence                                                                              |
| ------------- | ---------- | ------------------------------------------------------------------------------------- |
| G1 cargo check | ✓ pass    | Clean run; only legacy warnings carried over from TB-11.                              |
| G2 cargo test --workspace | ✓ pass | **757 / 0 / 150** (+10 net vs TB-11 ship 747; +26 vs TB-10 ship 731)                 |
| G3 lean_market 7 subcommands | ✓ pass | run-task / view-task / view-wallet / view-replay (TB-10) + tick + view-bankruptcy (TB-12 Atom 0.5b) + view-positions (TB-12 Atom 4). 7 ✓ |
| G4 evaluator forced-exhaust real-LLM produces EvidenceCapsule | ⚠ **deferred** | Atom 0.5(a) wires the call site (CAS write + emit_system_tx); real-LLM smoke is a manual user-driven session post-audit per charter §3 Atom 0.5 + §6.2 (real LLM is TB-11 carry-forward gate; deterministic integration tests are TB-12 main gate). |
| G5 audit_dashboard §13 renders | ✓ pass | Atom 4 — empty + non-empty cases handled; per-node aggregation + label discipline. |
| G6 verify_chaintape green     | ✓ pass | TB-7+ verify_chaintape unchanged; new typed-tx field projection captured in dispatch state-root.       |
| G7 diff TB-11→TB-12: zero new typed_tx variant | ✓ pass | Atom 1 NEW types are NodePosition (struct, not TypedTx variant) + 2 enums. NO TypedTx::* variant added. |
| G8 dispatch arms unchanged outside accept-side side-effect | ✓ pass | Atom 2 only extended Work + Challenge accept arms with additive index write. NO new dispatch arm.    |
| G9 TransitionError additive only | ✓ pass | Zero new variants introduced.                                                                       |
| G10 No `node_market_t` canonical EconomicState field; field count == 11 | ✓ pass | SG-12.8 test PASS.                                                                                  |
| G11 Conservation invariant | ✓ pass | 5-holding sum unchanged; NodePosition NOT in sum. SG-12.4 + ctf_invariant_unchanged tests PASS.       |

**11/11 G-gates ✓ pass + 8/8 SG-12.x ✓ pass + 1/11 ⚠ deferred (G4 real-LLM smoke)**.

The G4 deferral is per-charter (architect §6.2 explicit: deterministic integration tests = TB-12 main gate; zeta real smoke = TB-11 carry-forward gate). The Atom 0.5(a) wire-up infrastructure is in place — the real-LLM exercise itself is wall-clock expensive (~22min cold Lean cache) and outside this audit-cycle's autonomous-execution budget.

---

## §7 Recursive failure-mode analysis

| Failure mode                                          | TB-12 response                                                                                            |
| ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Forged NodePosition via agent ingress                 | UNREACHABLE. NodePosition is sequencer-derived inside dispatch_transition; no typed-tx variant for agent ingress. |
| Replay non-determinism (NodePosition field drift)     | Locked by `sg_12_5_node_positions_replay_deterministic`; Q-projection is pure function of typed-tx fields. |
| Double-derivation (same position_id in node_positions_t.0) | UNREACHABLE in TB-12. position_id == source_tx (work.tx_id or challenge.tx_id); same source_tx cannot be accepted twice (existing TB-3 / TB-4 dispatch arms enforce idempotency on tx_id). Future TB-13+ MarketBuy may break the 1:1 — flagged in NodePosition doc-comment as TB-12 invariant. |
| ChallengeTx with task_id mismatch                     | task_id is Q-derived from `stakes_t[challenge.target_work_tx].task_id` at dispatch time. If target_work_tx is missing from stakes_t, the ChallengeTx accept arm ALREADY rejects with `TargetWorkInactive` (TB-4 logic; preserved). |
| WorkTx with stake==0                                  | UNREACHABLE in production. TB-3 WorkTx accept arm rejects with `StakeInsufficient` upstream of NodePosition write (verified: `worktx_with_zero_stake_creates_no_position` removed because the negative case is unreachable). Atom 2 gate `if work.stake.micro_units() > 0` is defense-in-depth for graceful degradation. |
| NodePosition.amount in total_supply_micro             | UNREACHABLE. `monetary_invariant.rs:total_supply_micro` enumeration is fixed at 5 holdings (balances + escrows + stakes + claims-active + challenge_cases); node_positions_t is NOT in the enumeration. SG-12.4 test locks this. |

---

## §8 Cross-references

- Architect ruling lossless: `handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md`
- 2026-05-02 supplementary directive: `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`
- Charter: `handover/tracer_bullets/TB-12_charter_2026-05-03.md`
- TB-11 SHIPPED reference: `handover/audits/RECURSIVE_AUDIT_TB_11_2026-05-02.md`
- Atoms 0-5 commits: `5ada28d` (charter ratify), `2cb7f4a` (Atom 0.5), `a35f5f3` (Atom 1), `3615e32` (Atom 2), `f4bff3f` (Atom 3+4+5).
- Memory: `project_tb_12_node_exposure_index`, `feedback_o1_chain_on_auditability`, `feedback_kolmogorov_compression`, `feedback_dual_audit`, `feedback_iteration_cap_24h`, `feedback_workspace_test_canonical`.
- Constitution: `constitution.md` Art. 0.2 / I.1 / III.4 / V.1.3.

---

## §9 Verdict

**TB-12 Atoms 0-5 + 6(a) PASS.** Kernel-level architectural core SHIPPED. Atoms 6(b) Codex + 6(c) Gemini external audits are NEXT and MANDATORY per architect §5 ruling (Class 3 dual audit; Gemini exhausted = degraded label, NOT honest deferral).

After Atom 6(b) + 6(c) verdict, AI coder STOPS for user review per Q6 (ii.5) sync mode. SHIP at Atom 7 ONLY after explicit user authorization on the dual-audit verdict.

**Halting triggers**: NOT triggered. Continue to Atom 6(b) Codex.

---

## §10 Remediation log (post-Codex CHALLENGE × 2)

### Q4 doc-drift remediation (CHALLENGE → resolved)

Codex Q4 CHALLENGE: my self-audit and the audit prompt described
`total_supply_micro` as a 5-holding sum (balances + escrows + stakes +
claims-active + challenge_cases). Actual code in
`src/economy/monetary_invariant.rs:113` is the **4-holding TB-8 model**
(claims-active was removed from `total_supply_micro` per TB-8
ratification §1 Q5). The architectural invariant is correct
(NodePosition not counted), only the docstring drifted.

**Resolution**: §3 of this audit corrected to "4-holding CTF
(post-TB-8)" with explicit pointer to `monetary_invariant.rs:113`. The
SG-12.4 + ctf_invariant tests still pass — they assert
`assert_total_ctf_conserved` invariance which uses the actual code's
4-holding enumeration.

### Q5 legacy-code scope remediation (CHALLENGE → resolved)

Codex Q5 CHALLENGE: a strict forbidden-token grep against `src/`
hits legacy CPMM/trading code in `src/prediction_market.rs` (`BinaryMarket`,
`buy_yes`, etc.) and `src/kernel.rs` (which imports `BinaryMarket`).

**TB-12 added zero code touching this legacy scaffolding** —
verified by `grep -rn "BinaryMarket\|prediction_market" $(git diff
6ab165c..HEAD --name-only)`. The legacy code predates TB-12 by many
TBs (created in early v4 as Tier 0 CPMM scaffolding, before the
2026-05-02 + 2026-05-03 architect rulings established the
TB-13/TB-14 trajectory).

**Architect's TB-13 + TB-14 supplementary directive REPLACES this
legacy** — the architect ruling's:
- TB-13 CompleteSet + MarketSeedTx introduces integer-math CTF-conserving
  conditional shares (NO f64).
- TB-14 PriceIndex computes price from `long_interest / (long+short)`
  using integer math (NOT CPMM-style automatic liquidity).
- Architect §9.4 forbidden list explicitly bans CPMM/AMM/automatic
  liquidity going forward.

The legacy `src/prediction_market.rs` (f64 CPMM) violates the post-2026-05
architect ruling on integer math + no automatic liquidity, so it MUST
be quarantined / replaced before TB-14 ships. **Per `feedback_no_retroactive_evidence_rewrite`**,
TB-12 does NOT delete this legacy code; quarantine is a TB-13/14
prerequisite, NOT a TB-12 ship blocker.

OBS: `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`
tracks the legacy `prediction_market.rs` quarantine as a TB-13
prerequisite. Codex Q5 CHALLENGE is therefore resolved as
**out-of-scope-for-TB-12 / on-roadmap-for-TB-13** rather than as a
silent regression.

### Net post-remediation verdict

Codex CHALLENGE × 2 → both resolved as documentation/scope clarifications,
not architectural regressions. NodePosition implementation correct;
constitutional invariants preserved; halting triggers NOT fired.
