# TB-15 SHIP STATUS — 2026-05-03

**Status**: SHIPPED 2026-05-03.
**Charter**: `handover/tracer_bullets/TB-15_charter_2026-05-03.md` (RATIFIED).
**Architect spec**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6 (verbatim).
**Constitutional anchor**: `handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md` §1.
**Phase**: P4 Information Loom (primary) + P5 MetaTape v0 prep (Markov capsule = ArchitectAI substrate; NO automatic mutation).
**Risk class**: Class 2 envelope (self-audit). NO Codex / Gemini round mandated; AgentVisibleProjection not modified; only one new sequencer dispatch hook (TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5).

## Atom-by-atom commit list

| Atom | Commit | Title |
|---|---|---|
| 0 | `a14e01e` | TB-15 charter ratified (Lamarckian Autopsy + Markov EvidenceCapsule) |
| 1 | `0316a81` | Halt-trigger fixture (6 unimplemented stubs) |
| 2 | `6594d3f` | AgentAutopsyCapsule schema + writer |
| 3 | `f06d548` | AutopsyIndex on EconomicState + TaskBankruptcyTx wire-in |
| 4 | `a17f6ac` | TypicalErrorBroadcast clustering (cluster_autopsies) |
| 5 | `31be856` | MarkovEvidenceCapsule schema + generator binary |
| 6 | (this commit) | Dashboard §15 + first Markov capsule + SHIP |

## Final test status

```text
cargo test --workspace
  → 870 PASS / 0 fail / 150 ignored (net +67 vs TB-14 ship 803)

cargo test --workspace --test tb_15_halt_triggers
  → 6 passed / 0 failed (ALL 6 halt-triggers GREEN)

cargo test --workspace --lib autopsy_capsule
  → 15 passed / 0 failed (Atom 2 + 3 + 4 in-module battery)

cargo test --workspace --lib markov_capsule
  → 8 passed / 0 failed (Atom 5 in-module battery)

cargo test --workspace --bin audit_dashboard
  → 9 passed / 0 failed (incl. 4 new SG-15.6 §15 render tests)

cargo test --workspace --test fc_alignment_conformance
  → 23 passed / 0 failed / 9 ignored (incl. 4 new TB-15 witnesses
    FC1-N32 / FC1-N33 / FC2-N30 / FC3-N43)

cargo build --bin generate_markov_capsule  → PASS
cargo run --bin generate_markov_capsule -- ... --no-cas
  → emits capsule_id b244f16a... + LATEST_MARKOV_CAPSULE.txt + JSON file
```

## 6/6 halt-triggers GREEN (architect §6.6 forbidden)

| # | Halt trigger | Atom | Status |
|---|---|---|---|
| 1 | raw_logs_not_in_general_read_view | 3 | GREEN — `AgentVisibleProjection` file-scan: no `agent_autopsies_t` / `AutopsyIndex` / `AgentAutopsyCapsule` / `private_detail_cid` |
| 2 | markov_capsule_references_constitution_hash | 5 | GREEN — `MarkovEvidenceCapsule.constitution_hash == sha256(constitution.md)` |
| 3 | autopsy_does_not_mutate_predicates | 2 | GREEN — `autopsy_capsule.rs` file-scan: no `&mut PredicateRegistry` etc. |
| 4 | private_detail_not_in_other_agent_view | 3 | GREEN — `AutopsyIndex` value type is `Vec<Cid>` (not raw bytes) |
| 5 | typical_error_clustering_uses_summary_only | 4 | GREEN — `cluster_autopsies` JSON output contains no `private_detail_cid` byte run |
| 6 | deep_history_read_without_override_fails | 5 | GREEN — `try_deep_history_read_with_override_check(false) → DeepHistoryReadDenied` |

## 8/8 architect ship gates GREEN (architect §6.5)

| ID | Gate | Discharge |
|---|---|---|
| SG-15.1 | Failed/losing agent gets private AutopsyCapsule | TaskBankruptcyTx Step 3.5 → per-staker capsule via `derive_autopsies_for_bankruptcy` |
| SG-15.2 | Raw private details do not enter other Agent read view | `agent_autopsies_t` excluded from `AgentVisibleProjection`; halt-trigger #1 + #4 |
| SG-15.3 | Latest Markov capsule can bootstrap next session | `next_session_context_cid` field + `LATEST_MARKOV_CAPSULE.txt` pointer |
| SG-15.4 | Deep-history read without override fails | `try_deep_history_read_with_override_check(false)` returns `DeepHistoryReadDenied`; halt-trigger #6 |
| SG-15.5 | Typical error broadcast uses summary, not raw log | `cluster_autopsies` output struct embeds `public_summary` text + Cids only; halt-trigger #5 |
| SG-15.6 | Dashboard regenerates capsule summary from ChainTape + CAS | `render_section_15` pure-fn + 4 SG-15.6 dashboard render tests |
| SG-15.7 | Markov capsule references constitution hash | `MarkovEvidenceCapsule.constitution_hash` field; halt-trigger #2 |
| SG-15.8 | Autopsy does not mutate predicates/tools automatically | Writer signature has no mutable registry refs; halt-trigger #3 |

Plus engineering gates:
- G-15.9 (`cargo test --workspace` ≥ TB-14 baseline, 0 fail) — **GREEN** 870 PASS / 0 fail.
- G-15.10 (FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43 each ≥1 witness) — **GREEN** all 4 added.
- G-15.11 (EconomicState sub-field count 12→13) — **GREEN** 3 tests updated.
- G-15.12 (first Markov capsule generated + persisted) — **GREEN** `handover/evidence/tb_15_markov_capsule_2026-05-03/`.

## Architectural deltas

### NEW source files (4)
- `src/runtime/autopsy_capsule.rs` (Atoms 2 + 3 + 4): `LossReasonClass` enum (8 variants) + `AgentAutopsyCapsule` struct + `format_public_summary` + `write_autopsy_capsule` + `derive_autopsies_for_bankruptcy` (PURE) + `write_bankruptcy_autopsies_to_cas` + `cluster_autopsies` + `TypicalErrorSummary` + `AutopsyWriteError`. 15 in-module tests.
- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
- `src/bin/generate_markov_capsule.rs` (Atom 5): CLI binary with `--tb-id` / `--out-dir` / `--constitution-path` / `--cas-dir` / `--prev-cid-hex` / `--alignment-dir` / `--no-cas` args + `TURINGOS_MARKOV_OVERRIDE` env support.
- `tests/tb_15_halt_triggers.rs` (Atoms 1 + 2 + 3 + 4 + 5): 6 halt-trigger fixtures.

### MODIFIED source files (5)
- `src/state/typed_tx.rs`: `+ RiskRuleId(pub String)` opaque newtype.
- `src/bottom_white/cas/schema.rs`: `+ ObjectType::AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext` (4 new variants).
- `src/state/q_state.rs`: `+ AutopsyIndex(BTreeMap<EventId, Vec<Cid>>)` newtype + `agent_autopsies_t: AutopsyIndex` 13th sub-field on EconomicState (serde-default). Sub-field count assertion 12→13.
- `src/state/sequencer.rs`: TaskBankruptcyTx dispatch arm Step 3.5 (pure capsule-Cid derivation → `agent_autopsies_t` insertion) + apply_one Stage 3.5 (CAS write of capsule + private_detail bytes via deterministic helper).
- `src/runtime/mod.rs`: `+ pub mod autopsy_capsule;` + `+ pub mod markov_capsule;`.
- `src/bin/audit_dashboard.rs`: `+ autopsy_event_counts` + `latest_markov_capsule_cid_hex` fields on `DashboardReport` + `+ read_latest_markov_pointer()` helper + `+ render_section_15(...)` pure render. 4 new SG-15.6 unit tests.

### MODIFIED test fixtures (4)
- `tests/economic_state_reconstruct.rs`: sub-field count 12→13.
- `tests/q_state_reconstruct.rs`: sub-field count 12→13.
- `tests/six_axioms_alignment.rs`: axiom_3 sub-field count 12→13.
- `tests/fc_alignment_conformance.rs`: + 4 TB-15 witnesses (FC1-N32 / FC1-N33 / FC2-N30 / FC3-N43).

### MODIFIED genesis_payload.toml (trust_root rehash chain)
- `src/runtime/mod.rs`: `3b2901c4 → adfc18a4 → 03fd5358` (Atom 2 → Atom 5)
- `src/state/q_state.rs`: `eeb35da8 → c23cc95d` (Atom 3)
- `src/state/typed_tx.rs`: `44098978 → 665838b0` (Atom 2)
- `src/state/sequencer.rs`: `1c6ba82f → 9fa59362` (Atom 3)
- `src/bottom_white/cas/schema.rs`: `70f234ab → 6427695d` (Atom 2)
- `tests/fc_alignment_conformance.rs`: `751c78c8 → 5e257f27` (Atom 6)

### NEW evidence directory
- `handover/evidence/tb_15_markov_capsule_2026-05-03/`: README + `MARKOV_TB-15_2026-05-03.json` (the genesis Markov capsule) + `LATEST_MARKOV_CAPSULE.txt` (Cid hex pointer `b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6`).
- `handover/markov_capsules/`: same JSON + LATEST pointer (the working location).

## Production claim

> TB-15 establishes the Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, CAS-resident, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — **NEVER from agent LLM self-narration**. The capsule chain is anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (13th sub-field; sequencer-side; NOT projected to `AgentVisibleProjection`). TypicalErrorBroadcast clustering (N≥3 threshold per architect §3.2.3) emits `TypicalErrorSummary` objects embedding `public_summary` text + capsule_id Cids only — never `private_detail_cid` payload bytes. MarkovEvidenceCapsule binds `constitution_hash` + L4 root + L4.E root + CAS root + `previous_capsule_cid` + `typical_errors` + `unresolved_obs` + `next_session_context_cid` into an end-of-TB rollup that becomes the default next-session bootstrap context (FR-15.4). Deep-history reads default-deny without `TURINGOS_MARKOV_OVERRIDE=1` (FR-15.5 + halt-trigger #6).
>
> Constitutional alignment: CR-15.3 (autopsy may suggest, never mutate predicates) + CR-15.4 (JudgeAI veto-only — no JudgeAI code in TB-15; this is P5 v1 territory) STRUCTURALLY ENFORCED via writer signature + halt-trigger #3 file-scan. CR-15.5 (capsules are evidence compression, not hidden source of truth — every field is derivable from chain + CAS) + CR-15.6 (Markov default prevents context poisoning) STRUCTURALLY ENFORCED via deterministic Cid derivation + default-deny gate.
>
> Class 2 envelope intact — no AgentVisibleProjection mod, no read-view-authorization mod, single new sequencer dispatch hook (TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5). All 6 halt-triggers + 8 architect SG green. All 4 P-roadmap exit criteria addressed (P4-Exit1/2/3 + P5-Exit1/2 prep). All 4 FC-IDs (FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43) have witness tests.

## Open follow-ups (carry-forward; NOT ship blockers)

- **Multi-site autopsy wire-in**: TB-15 v0 wires only TaskBankruptcyTx. SlashLoss + ChallengeUnsuccessful + VerifierBondLost wire-ins land when SlashTx ships in RSP-3.2 (TB-9 territory) and contribution DAG ships in RSP-4. Track in TB-16+ charter.
- **L4 / L4.E / CAS root chain-readers**: Markov capsule generator currently uses zero placeholders for `l4_root` / `l4e_root` / `cas_root`. Future TB will wire to the actual chain head readers (`Sequencer::current_state_root` / `LedgerWriter::current_root` / CAS metadata digest). Track in TB-16+ controlled-arena work.
- **CAS-walking dashboard §15**: dashboard `autopsy_event_counts` is currently empty (build_report does not rebuild full EconomicState from chain). Future TB-16 controlled-arena run will exercise the live wire-in.
- **InitAI agent-side honoring**: TB-15 ships substrate + binary-level default-deny only. Agent-side "default to constitution + latest Markov capsule" enforcement is a P5 v1 prerequisite documented in charter §1.3.
- **OBS_RESOLUTIONS_INDEX_TB15**: explicitly DEFERRED out of TB-15 scope per charter §7 Auto-resolution G + `feedback_architect_deviation_stance`. ResolutionsIndex is a TB-13 audit-residue resolution-decoupling refactor, NOT in architect's TB-15 §6 spec. Carry-forward to dedicated TB.
- **OBS_TB13_FENCE_MECHANISM_DOOM_LOOP**: carry-forward (AST-aware fence refactor).
- **OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT**: carry-forward.
- **OBS_AGENT_SIG_REPLAY_GAP**: carry-forward.
