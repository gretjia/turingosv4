# Codex impl audit — TB-6 full-diff (Atoms 0-7)

**Date**: 2026-05-01
**Diff range**: 7970d2d..17c5e73
**Audit class**: Class 2 (production wire-up; per feedback_risk_class_audit)
**Mode**: Codex impl only (Gemini arch deferred or already covered by ship-time degraded label)
**Branch at audit**: main @ 05c5be7
**Scope**: full TB-6 implementation (excludes TB-7 Atom 0 ratification commit 05c5be7)

## §0 Headline verdict

**SOME_CHALLENGE** - production ChainTape path exists and replays from disk, but TB-6 still relies on synthetic seed entries, leaves real LLM proposal writes on legacy bus paths, and has incomplete audit-chain / tamper-test coverage.

Per-dimension counts: PASS 1 (A5). CHALLENGE 6 (A1, A2, A3, A4, A6, A7). VETO 0.

## §1 Per-dimension verdicts

### A1 — Production binary drives Sequencer::apply_one

**Verdict**: CHALLENGE

**Evidence**:
- `experiments/minif2f_v4/src/bin/evaluator.rs:673` builds an optional `ChaintapeBundle` from env config, but bootstrap errors are logged and converted to `None` at `experiments/minif2f_v4/src/bin/evaluator.rs:675`-`experiments/minif2f_v4/src/bin/evaluator.rs:680`, causing legacy fallback instead of fail-closed startup.
- `experiments/minif2f_v4/src/bin/evaluator.rs:690`-`experiments/minif2f_v4/src/bin/evaluator.rs:696` wires `TuringBus::with_sequencer` when ChainTape bootstrap succeeds.
- `src/runtime/mod.rs:432`-`src/runtime/mod.rs:454` runs the ChainTape driver and calls `sequencer.apply_one(envelope)` on normal receive and shutdown drain.
- `experiments/minif2f_v4/src/bin/evaluator.rs:743`-`experiments/minif2f_v4/src/bin/evaluator.rs:770` submits only the synthetic TaskOpen and zero-stake WorkTx through `bus.submit_typed_tx`.
- The real proposal hot path still uses legacy `bus.append(...)` at `experiments/minif2f_v4/src/bin/evaluator.rs:1250`; OMEGA/accepted proof paths still use `bus.append_oracle_accepted(...)` at `experiments/minif2f_v4/src/bin/evaluator.rs:1393`, `experiments/minif2f_v4/src/bin/evaluator.rs:1650`, and `experiments/minif2f_v4/src/bin/evaluator.rs:1702`.
- `src/bus.rs:218`-`src/bus.rs:237` routes both `append` and `append_oracle_accepted` into `append_internal`, which writes directly to `kernel.append` at `src/bus.rs:328` and legacy ledger append at `src/bus.rs:376`-`src/bus.rs:377`, with no Sequencer call.
- The TB-6 bootstrap test explicitly says evaluator construction is only construction-level and that `run_swarm` does not call `submit_typed_tx`: `tests/tb_6_runtime_chaintape_bootstrap.rs:129`-`tests/tb_6_runtime_chaintape_bootstrap.rs:133`.

**Fix items**:
- Fail closed when `TURINGOS_CHAINTAPE_PATH` is set but ChainTape bootstrap fails.
- Convert real proposal, OMEGA, accepted, and rejection branches from legacy `append`/`append_oracle_accepted` to typed ChainTape submissions.
- Keep synthetic seeds only as test fixtures, not as the production evidence substitute.

### A2 — Replay verifier soundness

**Verdict**: CHALLENGE

**Evidence**:
- `src/runtime/verify.rs:207`-`src/runtime/verify.rs:283` reopens `pinned_pubkeys.json`, L4, CAS, and L4.E, then calls `replay_full_transition`.
- `src/bottom_white/ledger/transition_ledger.rs:433`-`src/bottom_white/ledger/transition_ledger.rs:493` checks parent roots, signature, CAS lookup, payload decode, tx-kind match, pure dispatch, state root, and ledger root.
- `src/runtime/verify.rs:263`-`src/runtime/verify.rs:269` treats a missing `rejections.jsonl` as an empty L4.E writer. That is acceptable for empty-chain tests but weak for evidence directories expected to contain L4.E.
- `src/bin/verify_chaintape.rs:40`-`src/bin/verify_chaintape.rs:62` makes the CLI a thin wrapper with exit 0 only when all indicators pass.
- I90 covers happy path and empty-chain replay at `tests/tb_6_verify_chaintape.rs:35`-`tests/tb_6_verify_chaintape.rs:110`; I90c covers only pinned-pubkey tampering at `tests/tb_6_verify_chaintape.rs:112`-`tests/tb_6_verify_chaintape.rs:165`.
- Lower-level replay tests cover some mutation classes, e.g. bad signature at `src/bottom_white/ledger/transition_ledger.rs:1304`-`src/bottom_white/ledger/transition_ledger.rs:1330`, tx-kind mismatch at `src/bottom_white/ledger/transition_ledger.rs:1375`-`src/bottom_white/ledger/transition_ledger.rs:1432`, and payload decode failure at `src/bottom_white/ledger/transition_ledger.rs:1435`-`src/bottom_white/ledger/transition_ledger.rs:1488`, but these are not end-to-end verifier tests over on-disk evidence.

**Fix items**:
- Add I90-style end-to-end tests for CAS payload/index tamper, Git L4 entry tamper, derivative root tamper, and L4.E deletion/corruption.
- Add a strict mode for evidence replay that requires expected files and expected minimum counts instead of treating absent L4.E as empty.

### A3 — Agent audit trail schema and chain integrity

**Verdict**: CHALLENGE

**Evidence**:
- The architect-required fields are documented at `src/runtime/agent_audit_trail.rs:3`-`src/runtime/agent_audit_trail.rs:6`.
- `AgentProposalRecord` defines those fields plus an extra `logical_t` at `src/runtime/agent_audit_trail.rs:83`-`src/runtime/agent_audit_trail.rs:119`; the comments also state "9-field record (10 with logical_t)" at `src/runtime/agent_audit_trail.rs:28`-`src/runtime/agent_audit_trail.rs:30`.
- The record hash binds `logical_t` at `src/runtime/agent_audit_trail.rs:123`-`src/runtime/agent_audit_trail.rs:176`, but `logical_t` has not been ratified as one of the architect fields.
- Audit records are CAS-backed via `write_to_cas` / `read_from_cas` at `src/runtime/agent_audit_trail.rs:225`-`src/runtime/agent_audit_trail.rs:250`.
- The JSONL index row stores `tx_id`, `proposal_record_cid`, `logical_t`, `prev_hash`, and `hash` at `src/runtime/agent_audit_trail.rs:255`-`src/runtime/agent_audit_trail.rs:265`.
- `AgentAuditTrailIndex::verify_chain` only checks the row spine, not the row hash against the CAS record; the comment acknowledges this at `src/runtime/agent_audit_trail.rs:378`-`src/runtime/agent_audit_trail.rs:387`.
- The synthetic seed writer uses placeholder zero CIDs and synthetic records at `src/runtime/agent_audit_trail.rs:446`-`src/runtime/agent_audit_trail.rs:506`; it is explicitly a future-TB bridge for real LLM proposals at `src/runtime/agent_audit_trail.rs:409`-`src/runtime/agent_audit_trail.rs:416`.
- I91d only grep-checks forbidden field names, not schema conformance or real-chain correlation: `tests/tb_6_agent_audit_trail.rs:160`-`tests/tb_6_agent_audit_trail.rs:181`.

**Fix items**:
- Either remove `logical_t` from `AgentProposalRecord` or ratify it as a tenth field.
- On index reopen, fetch each `proposal_record_cid` from CAS and recompute the row hash from the stored record.
- Add mutation tests for every bound field, including `tx_id`, `proposal_record_cid`, `logical_t`, `prev_hash`, and CAS record content.

### A4 — RunSummary from L4 + L4.E + CAS

**Verdict**: CHALLENGE

**Evidence**:
- `RunSummary` claims L4, L4.E, and CAS sources at `src/runtime/run_summary.rs:8`-`src/runtime/run_summary.rs:16`.
- Accepted tx IDs are extracted by reading Git L4 entries, fetching `tx_payload_cid` from CAS, decoding `TypedTx`, and extracting the tx ID at `src/runtime/run_summary.rs:129`-`src/runtime/run_summary.rs:148`.
- Rejected tx IDs are extracted from L4.E records, but CAS lookup/decode failures are skipped with no error at `src/runtime/run_summary.rs:159`-`src/runtime/run_summary.rs:169`.
- The summary never opens `agent_audit_trail.jsonl`, so it does not prove `tx_id -> CID -> AgentProposalRecord` correlation: `src/runtime/run_summary.rs:122`-`src/runtime/run_summary.rs:183`.
- The production evaluator writes `RunSummary` only on the canonical exit path after `bundle.shutdown()`; the nearby comment states early returns drop the bundle without explicit shutdown at `experiments/minif2f_v4/src/bin/evaluator.rs:1802`-`experiments/minif2f_v4/src/bin/evaluator.rs:1809`.
- I92 verifies summary shape and basic accepted/rejected sets at `tests/tb_6_run_summary.rs:36`-`tests/tb_6_run_summary.rs:110`, but it does not assert audit-trail correlation.

**Fix items**:
- Make rejected-side CAS lookup/decode failure a hard summary error.
- Cross-check each accepted/rejected `tx_id` against `agent_audit_trail.jsonl` and the referenced CAS `AgentProposalRecord`.
- Ensure summary emission runs for early-return success paths or is backfilled automatically from final on-disk ChainTape.

### A5 — Rejection evidence disjoint from accepted transition ledger

**Verdict**: PASS

**Evidence**:
- The L4.E module states the disjoint-ledger invariant and says rejections never mutate `state_root_t` / `ledger_root_t` at `src/bottom_white/ledger/rejection_evidence.rs:11`-`src/bottom_white/ledger/rejection_evidence.rs:18`.
- `RejectedSubmissionRecord` is keyed by `submit_id`, lacks `logical_t`, and stores only parent state plus rejection evidence at `src/bottom_white/ledger/rejection_evidence.rs:171`-`src/bottom_white/ledger/rejection_evidence.rs:215`.
- `RejectionEvidenceWriter` documents the split from accepted logical time at `src/bottom_white/ledger/rejection_evidence.rs:328`-`src/bottom_white/ledger/rejection_evidence.rs:334`.
- `Sequencer::apply_one` records rejections and returns before logical time, state root, or ledger root advances at `src/state/sequencer.rs:1513`-`src/state/sequencer.rs:1527`.
- Accepted L4 `LedgerEntry` carries `logical_t`, parent/resulting roots, payload CID, and system signature at `src/bottom_white/ledger/transition_ledger.rs:81`-`src/bottom_white/ledger/transition_ledger.rs:104`.

**Fix items**:
- None for TB-6 closure. Keep future L4.E signature work disjoint from state/economic mutation.

### A6 — Production ingress uses typed submissions and stage 1.5 verification

**Verdict**: CHALLENGE

**Evidence**:
- `TuringBus::submit_typed_tx` delegates to `seq.submit(tx)` when a sequencer exists at `src/bus.rs:135`-`src/bus.rs:142`; `Sequencer::submit` delegates to `submit_agent_tx` at `src/state/sequencer.rs:1326`-`src/state/sequencer.rs:1343`.
- `submit_agent_tx` rejects system-emitted variants before queueing at `src/state/sequencer.rs:1198`-`src/state/sequencer.rs:1227`.
- `emit_system_tx` constructs and signs system transactions internally at `src/state/sequencer.rs:1238`-`src/state/sequencer.rs:1262`.
- `apply_one` stage 1.5 re-verifies system signatures and routes failures to L4.E at `src/state/sequencer.rs:1490`-`src/state/sequencer.rs:1510`; U27 confirms self-signed emit path passes stage 1.5 at `src/state/sequencer.rs:2983`-`src/state/sequencer.rs:3014`.
- The evaluator only uses typed submission for synthetic seed entries at `experiments/minif2f_v4/src/bin/evaluator.rs:753` and `experiments/minif2f_v4/src/bin/evaluator.rs:770`.
- New hot-path legacy calls remain: `bus.append` at `experiments/minif2f_v4/src/bin/evaluator.rs:1250`; `bus.append_oracle_accepted` at `experiments/minif2f_v4/src/bin/evaluator.rs:1393`, `experiments/minif2f_v4/src/bin/evaluator.rs:1650`, and `experiments/minif2f_v4/src/bin/evaluator.rs:1702`.

**Fix items**:
- Route evaluator proposal acceptance/rejection through `submit_agent_tx` via `bus.submit_typed_tx`.
- Route system-only OMEGA/terminal actions through `emit_system_tx` or add the required typed system command variants.
- Remove or explicitly quarantine legacy `append` / `append_oracle_accepted` from ChainTape mode.

### A7 — End-to-end tamper resistance

**Verdict**: CHALLENGE

**Evidence**:
- CAS retrieval verifies content hash against CID at `src/bottom_white/cas/store.rs:198`-`src/bottom_white/cas/store.rs:221`, and corrupt sidecar parse is unit-tested at `src/bottom_white/cas/store.rs:454`-`src/bottom_white/cas/store.rs:476`.
- Replay maps bad signature, CAS missing, payload decode, and ledger-root mismatch into separate replay failures at `src/runtime/verify.rs:353`-`src/runtime/verify.rs:374`.
- I90c mutates `pinned_pubkeys.json` and asserts `system_signatures_verified=false` at `tests/tb_6_verify_chaintape.rs:112`-`tests/tb_6_verify_chaintape.rs:165`.
- There is no I90 end-to-end test that mutates an on-disk CAS payload/index row and asserts `cas_payloads_retrievable=false` or replay failure.
- There is no I90 end-to-end test that mutates an on-disk Git L4 entry and asserts both replay failure and the relevant signature/ledger-root indicator behavior.
- There is no I90 end-to-end test that mutates derivative roots in the shipped evidence repo; lower-level chain-only tests exist, but not verifier-over-disk tests.

**Fix items**:
- Add end-to-end tamper tests for CAS payload/index, Git ledger entry, derivative roots, L4.E row, and pinned pubkeys.
- For each tamper class, assert both CLI exit status and exact `ReplayReport` booleans.

## §2 Cross-cutting findings

- **Synthetic seed dependence**: the production binary produces L4 and L4.E only because it injects synthetic TaskOpen and WorkTx seed entries (`experiments/minif2f_v4/src/bin/evaluator.rs:728`-`experiments/minif2f_v4/src/bin/evaluator.rs:743`). The real LLM proof path remains parallel evidence, not ChainTape state.
- **Fail-open behavior**: ChainTape bootstrap falls back to legacy mode (`experiments/minif2f_v4/src/bin/evaluator.rs:675`-`experiments/minif2f_v4/src/bin/evaluator.rs:680`), and L4.E open failure inside bootstrap falls back to in-memory at `src/runtime/mod.rs:365`-`src/runtime/mod.rs:379`.
- **Audit correlations are present as parts but not enforced as one chain**: L4 stores `tx_payload_cid`; L4.E stores rejected payload CIDs; agent audit index stores `tx_id -> proposal_record_cid`; RunSummary stores tx ID sets. No verifier currently proves all of those agree for one run.
- **Evidence generated before real routing**: the smoke README states actual proof artifacts remain legacy and the chaintape entries are synthetic at `handover/evidence/tb_6_chaintape_smoke_2026-05-01/README.md:19`-`handover/evidence/tb_6_chaintape_smoke_2026-05-01/README.md:31`, and the synthetic label is explicit at `handover/evidence/tb_6_chaintape_smoke_2026-05-01/synthetic_rejection_label.json:1`.

## §3 Action items

1. **Fail closed on ChainTape bootstrap / L4.E JSONL open failures.**
   - File:line: `experiments/minif2f_v4/src/bin/evaluator.rs:675`-`experiments/minif2f_v4/src/bin/evaluator.rs:680`; `src/runtime/mod.rs:365`-`src/runtime/mod.rs:379`.
   - Suggested fix: if ChainTape env is set, propagate bootstrap and JSONL-open failures out of evaluator startup instead of constructing a legacy/in-memory fallback.
   - Blocking? yes.

2. **Route real evaluator proposal, OMEGA, and rejection paths through typed ChainTape submissions.**
   - File:line: `experiments/minif2f_v4/src/bin/evaluator.rs:1250`; `experiments/minif2f_v4/src/bin/evaluator.rs:1393`; `experiments/minif2f_v4/src/bin/evaluator.rs:1650`; `experiments/minif2f_v4/src/bin/evaluator.rs:1702`.
   - Suggested fix: build `TypedTx`/system commands from the live evaluator decisions and submit through `bus.submit_typed_tx` / `Sequencer::emit_system_tx`; make legacy append unreachable in ChainTape mode.
   - Blocking? yes.

3. **Fix `AgentProposalRecord` schema or ratify the extra `logical_t` field.**
   - File:line: `src/runtime/agent_audit_trail.rs:83`-`src/runtime/agent_audit_trail.rs:119`; `src/runtime/agent_audit_trail.rs:123`-`src/runtime/agent_audit_trail.rs:176`.
   - Suggested fix: either remove `logical_t` from the record and keep chronology in the index row, or update the spec/tests/docs to assert a ratified tenth field.
   - Blocking? yes.

4. **Recompute audit-index row hashes from CAS records and test all field mutations.**
   - File:line: `src/runtime/agent_audit_trail.rs:378`-`src/runtime/agent_audit_trail.rs:387`; `tests/tb_6_agent_audit_trail.rs:160`-`tests/tb_6_agent_audit_trail.rs:181`.
   - Suggested fix: on `AgentAuditTrailIndex::open`, read each `proposal_record_cid`, decode `AgentProposalRecord`, recompute `record.audit_hash(prev_hash)`, and compare against row `hash`; add mutation tests for every row and record field.
   - Blocking? yes.

5. **Add strict RunSummary tx_id-to-CID-to-audit-record correlation.**
   - File:line: `src/runtime/run_summary.rs:122`-`src/runtime/run_summary.rs:183`; `src/runtime/run_summary.rs:159`-`src/runtime/run_summary.rs:169`.
   - Suggested fix: make summary construction load `agent_audit_trail.jsonl`, require every accepted/rejected tx ID to resolve to a CAS `AgentProposalRecord`, and fail on missing or undecodable rejected payloads.
   - Blocking? yes.

6. **Add I90 end-to-end tamper tests for CAS payloads, Git ledger entries, derivative roots, and pinned pubkeys.**
   - File:line: `tests/tb_6_verify_chaintape.rs:35`-`tests/tb_6_verify_chaintape.rs:165`; `src/runtime/verify.rs:353`-`src/runtime/verify.rs:374`.
   - Suggested fix: extend `tests/tb_6_verify_chaintape.rs` with disk-level mutation cases and assert exact report booleans plus CLI exit code.
   - Blocking? yes.

7. **Regenerate TB-6 smoke evidence once real proposal routing and audit-trail files exist.**
   - File:line: `handover/evidence/tb_6_chaintape_smoke_2026-05-01/README.md:19`-`handover/evidence/tb_6_chaintape_smoke_2026-05-01/README.md:31`; `handover/evidence/tb_6_chaintape_smoke_2026-05-01/synthetic_rejection_label.json:1`; `handover/evidence/tb_6_chaintape_smoke_2026-05-01/run_summary.json:1`-`handover/evidence/tb_6_chaintape_smoke_2026-05-01/run_summary.json:84`.
   - Suggested fix: rerun the production evaluator with ChainTape mode after typed real-proposal routing lands; require non-synthetic L4/L4.E entries, audit trail index, CAS records, replay report, and run summary to agree.
   - Blocking? yes.

## §4 Closure recommendation

TB-6 audit-pending status should **not** close. The implementation has a real on-disk ChainTape path and a replay verifier, but the shipped production evidence is still synthetic-seed-driven and does not bind the live LLM proposal/OMEGA/rejection workflow into typed ChainTape records.

TB-7 Atom 1 is **not cleared** as a closure gate for TB-6 production wire-up. It may proceed only under the existing degraded / audit-pending label until the seven action items above are closed and fresh evidence is regenerated.
