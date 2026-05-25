# Flowchart Liveness Inventory - 2026-05-25

Authority:
- FC1: `constitution.md:455-509`
- FC2: `constitution.md:571-660`
- FC3: `constitution.md:826-870`

This is an audit artifact, not a replacement constitution. `OBL-005` closes
only when the listed production paths are implemented, replay-verified, and
kept green by constitution gates. Status values used here are only:
`LIVE`, `PARTIAL`, `MISSING`, `STRUCTURAL_ONLY`.

## FC1 Runtime Loop

| Anchor | Status | Current production evidence | Missing path |
|---|---:|---|---|
| `Q_t` carrier and typed state root | LIVE | `QState`, `Sequencer::q_snapshot`, L4 `LedgerEntry.resulting_state_root` | none for typed sequencer path |
| `rtool -> input` | LIVE | `tests/constitution_flowchart_livenow.rs::fc1_rtool_input_snapshot_is_chain_cas_derived` proves `TuringBus::snapshot` / `UniverseSnapshot` reads typed sequencer state plus L4 WorkTx and CAS `ProposalTelemetry.parent_tx`; legacy shadow `Tape` is empty in the proof | none for the tested typed read-view path |
| `delta / Agent output` | PARTIAL | `AgentAction`, `parse_agent_output`, typed tx adapters | real LLM path is product workload coverage, not flowchart proof by itself |
| predicates | LIVE | executable `PredicateRegistry`, `BoolWithProof`, sequencer re-verification | per-kind proof expansion remains future work where a predicate needs external proof checking |
| `wtool -> Q_{t+1}` accepted branch | LIVE | `Sequencer::apply_one` writes accepted typed tx to L4; real `WorkTx.read_set` binds `cas.proposal_telemetry:<cid>` and no longer uses `k.read` / `k.write` fixture placeholders | none for tested typed path |
| failed predicate to rejection evidence | LIVE | `RejectionEvidenceWriter`, L4.E `RejectedSubmissionRecord` | none for tested typed path |

## FC2 Boot / Replay / Tick / Halt

| Anchor | Status | Current production evidence | Missing path |
|---|---:|---|---|
| boot to initial ChainTape state | LIVE | `build_chaintape_sequencer`, activation entry, pinned pubkeys, `initial_q_state.json` | none for fresh boot |
| replay verifier | LIVE | `verify_chaintape`, `replay_full_transition_with_predicate_binding` | none for tested replay path |
| resume boot from existing tape | LIVE | `resume_existing_chain`, `bootstrap_resume_state`, `Sequencer::new_at_logical_t` | none for tested resume path |
| map-reduce tick (`clock -> mr`, map/reduce edges) | LIVE | fresh boot commits `MapReduceTickTx` to L4 after predicate activation; replay re-verifies prefix roots and clock advance | none for boot-visible scheduled tick |
| terminal / halt summary | LIVE | `TerminalSummaryTx`, `RunOutcome`, `SystemEmitCommand::TerminalSummary`, `runs_t` | none for tested terminal-summary path |

## FC3 Meta Architecture

| Anchor | Status | Current production evidence | Missing path |
|---|---:|---|---|
| constitution and logs read-only boundary | PARTIAL | Trust-root verification, raw-log shielding tests, CAS-backed capsules | typed runtime boundary for all tool/log interactions still needs stronger live proof |
| Veto-AI role | LIVE | `VetoDecisionTx` / `TxKind::VetoDecision = 25`, `VetoDecisionCapsule` schema `fc3.veto_decision.v1`, deterministic runtime verdict `{PASS,VETO}`, constitution mutation veto and commit-block test | none for typed runtime Veto-AI verdict path |
| ArchitectAI role | LIVE | `ArchitectProposalTx` / `TxKind::ArchitectProposal = 24` + `ArchitectCommitTx` / `TxKind::ArchitectCommit = 26`, proposal/commit CAS capsules, PASS-only commit path, replay verification | none for typed runtime proposal/veto/commit path |
| tools-to-log typed boundary | PARTIAL | typed tx admission and L4/L4.E writers | need direct liveness probe proving tools cannot mutate outside tape/log path |
| logs feedback to ArchitectAI | LIVE | `LogFeedbackArchiveTx` / `TxKind::LogFeedbackArchive = 21` is system-emitted to L4, binds L4/L4.E/CAS/constitution roots, stores `ArchitectFeedbackCapsule` in CAS under schema `fc3.architect_feedback.v1`, rejects agent ingress, replays, and feeds runtime `ArchitectProposalTx` in `tests/constitution_fc3_closure.rs` | none for the typed tape-visible feedback-to-proposal edge |
| error to re-init semantics | LIVE | `ReinitRequestTx` / `ReinitBootTx` are system-emitted to L4, bind an ErrorHalt `TerminalSummaryTx` trigger, store `ReinitReasonCapsule` in CAS under schema `fc3.reinit_reason.v1`, recompute replayed boot state root, and never rewrite old evidence | none for the typed tape-visible re-init edge |

## Extra Functionality Classification

| Surface | Classification | Reason |
|---|---|---|
| Predicate registry binding | required substrate | needed to make FC1 predicates executable ground truth instead of runner-stamped claims |
| RejectionEvidenceWriter / L4.E | required substrate | records failed admissions without advancing accepted state |
| ProposalTelemetry-bound WorkTx read/write keys | required substrate | keeps FC1 agent output reconstructable from CAS-backed proposal evidence instead of synthetic `k.read` / `k.write` placeholders |
| Trust-root verification | support invariant | enforces FC3 read-only constitution/log boundary; not itself a separate flowchart node |
| Markov capsule and deep-history default-deny | support invariant | protects context inheritance; it must not be reused as the FC3 feedback edge |
| Price index, mask set, Boltzmann parent selection | product workload | useful market workload machinery, not canonical FC node coverage |
| Autopsy capsules and typical-error clustering | product workload | useful learning/evidence substrate, not canonical FC node coverage |
| legacy `TuringBus::append` forbidden-pattern gate | legacy/zombie candidate | retained for legacy mode only; typed sequencer path is the current authority |

## LiveNow Test Commands

Expected current green path:

```bash
cargo test --test constitution_fc3_closure
cargo test --test constitution_flowchart_source_alignment
cargo test --test constitution_flowchart_livenow
cargo test --test constitution_matrix_drift
cargo test --test fc_alignment_conformance
```

The FC3 closure probe currently contains 12 tests:
- FC3 typed tx discriminants are tail-only
- FC3 feedback is system-only, L4/CAS-bound, shielded, and replay-verified
- FC3 runtime ArchitectAI/Veto-AI proposal, PASS, commit, veto, and commit
  retarget rejection are typed, CAS-bound, and replay-visible
- FC3 re-init links ErrorHalt to request and boot acknowledgement without
  evidence rewrite

The LiveNow probe currently contains 7 tests:
- FC1 typed WorkTx routes to L4 or L4.E
- FC1 real WorkTx provenance is CAS-bound, not a synthetic fixture placeholder
- FC1 rtool/input snapshot is ChainTape/CAS-derived, with legacy shadow `Tape`
  empty in the proof
- FC2 boot, replay, and resume are live
- FC2 map-reduce tick is boot-visible and replay-verified
- FC2 forged map-reduce tick is rejected at agent ingress
- FC2 terminal summary anchors `RunOutcome`
