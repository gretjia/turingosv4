# Codex impl audit — TB-7 full-diff

**Date**: 2026-05-01
**Diff range**: 05c5be7..9e74195
**Audit class**: Class 2 (production wire-up; per feedback_risk_class_audit)
**Mode**: Codex impl only (Gemini arch with degraded fallback per TB-5/TB-6 supplement precedent)
**Branch at audit**: main @ 9e74195
**Scope**: TB-7 Atoms 0/0.5/1/1.5/1.7/2/3/4/5/6/7

## §0 Headline verdict

**SOME_CHALLENGE**.

The TB-7 happy path is materially wired: evaluator append / OMEGA branches build ProposalTelemetry, sign WorkTx / VerifyTx, call `bus.submit_typed_tx`, and retain legacy tape writes only behind `// shadow_only:` annotations. Gate 7 scanner tests pass 3/3.

I am not comfortable giving `ALL_PASS` for a Class 2 production wire-up audit because:

- A1 is fail-open after bootstrap: once ChainTape mode is booted, CAS / signing / submission failures are logged and the run continues into the shadow-only legacy tape path.
- Gate 2 / `ChainDerivedRunFacts.proposal_count` has a semantic gap: docs say accepted + rejected WorkTx, but implementation counts only accepted L4 WorkTx and ignores L4.E WorkTx records.
- A7 self-audit has a headline contradiction: §0 says 5/7 closed and 2 partial, while §3 correctly says 3/7 fully closed and 4/7 partial with TB-6 audit-pending open.

## §1 Per-dimension verdicts

### A1 — TB-7 charter §4.0 authoritative path

**Verdict**: **CHALLENGE** for production fail-closed semantics; **PASS** for happy-path routing and `shadow_only` annotations.

**Evidence file:line**:

- Append branch constructs ProposalTelemetry and WorkTx, then calls `bus.submit_typed_tx(real_worktx).await`: `experiments/minif2f_v4/src/bin/evaluator.rs:1284`, `experiments/minif2f_v4/src/bin/evaluator.rs:1303`, `experiments/minif2f_v4/src/bin/evaluator.rs:1330`, `experiments/minif2f_v4/src/bin/evaluator.rs:1348`.
- Append branch retained legacy write is annotated `// shadow_only:` immediately before `bus.append`: `experiments/minif2f_v4/src/bin/evaluator.rs:1366`, `experiments/minif2f_v4/src/bin/evaluator.rs:1374`.
- Full-proof OMEGA branch constructs WorkTx + VerifyTx and calls `bus.submit_typed_tx` for both: `experiments/minif2f_v4/src/bin/evaluator.rs:1514`, `experiments/minif2f_v4/src/bin/evaluator.rs:1552`, `experiments/minif2f_v4/src/bin/evaluator.rs:1573`, `experiments/minif2f_v4/src/bin/evaluator.rs:1585`, `experiments/minif2f_v4/src/bin/evaluator.rs:1590`.
- Full-proof OMEGA retained legacy write is annotated `// shadow_only:` before `bus.append_oracle_accepted`: `experiments/minif2f_v4/src/bin/evaluator.rs:1600`, `experiments/minif2f_v4/src/bin/evaluator.rs:1608`.
- Per-tactic OMEGA branch constructs WorkTx + VerifyTx and calls `bus.submit_typed_tx` for both: `experiments/minif2f_v4/src/bin/evaluator.rs:1866`, `experiments/minif2f_v4/src/bin/evaluator.rs:1904`, `experiments/minif2f_v4/src/bin/evaluator.rs:1920`, `experiments/minif2f_v4/src/bin/evaluator.rs:1932`, `experiments/minif2f_v4/src/bin/evaluator.rs:1937`.
- Per-tactic OMEGA retained legacy write is annotated `// shadow_only:`: `experiments/minif2f_v4/src/bin/evaluator.rs:1947`, `experiments/minif2f_v4/src/bin/evaluator.rs:1949`.
- PartialOk retained write is explicitly `shadow_only` and points to append-branch authoritative routing: `experiments/minif2f_v4/src/bin/evaluator.rs:1999`, `experiments/minif2f_v4/src/bin/evaluator.rs:2001`, `experiments/minif2f_v4/src/bin/evaluator.rs:2008`.
- Production concern: errors in the authoritative path warn and continue rather than failing the proposal/run: append submit error `experiments/minif2f_v4/src/bin/evaluator.rs:1348`, CAS/signing warnings `experiments/minif2f_v4/src/bin/evaluator.rs:1352`, `experiments/minif2f_v4/src/bin/evaluator.rs:1358`, `experiments/minif2f_v4/src/bin/evaluator.rs:1361`; full-proof OMEGA submit warnings `experiments/minif2f_v4/src/bin/evaluator.rs:1585`, `experiments/minif2f_v4/src/bin/evaluator.rs:1590`; per-tactic OMEGA submit warnings `experiments/minif2f_v4/src/bin/evaluator.rs:1932`, `experiments/minif2f_v4/src/bin/evaluator.rs:1937`.
- `bus.submit_typed_tx` itself returns `QueueClosed` if no Sequencer is wired, so a failed/absent ChainTape path is not equivalent to authoritative mutation: `src/bus.rs:127`, `src/bus.rs:135`, `src/bus.rs:139`.

**Fix items**:

- In ChainTape mode, make CAS open/write, q_snapshot, signing, and `submit_typed_tx` failures fail closed or produce explicit L4.E evidence. Do not continue to the legacy tape-only path after an authoritative-path failure.
- Consider making the evaluator require ChainTape mode for TB-7 production runs, or make the legacy mode label impossible to confuse with Frame B closure.

### A2 — Gate 7 conformance

**Verdict**: **PASS**.

**Evidence file:line**:

- Scanner scope is the production evaluator path and flags `bus.append(` / `bus.append_oracle_accepted(` without an immediately preceding contiguous `// shadow_only:` comment block: `tests/tb_7_legacy_append_regression.rs:16`, `tests/tb_7_legacy_append_regression.rs:35`, `tests/tb_7_legacy_append_regression.rs:47`, `tests/tb_7_legacy_append_regression.rs:55`, `tests/tb_7_legacy_append_regression.rs:60`, `tests/tb_7_legacy_append_regression.rs:76`, `tests/tb_7_legacy_append_regression.rs:86`.
- Live evaluator regression test fails on unannotated sites and prints a fix message: `tests/tb_7_legacy_append_regression.rs:93`, `tests/tb_7_legacy_append_regression.rs:100`, `tests/tb_7_legacy_append_regression.rs:107`.
- Positive control rejects an unannotated synthetic call: `tests/tb_7_legacy_append_regression.rs:116`, `tests/tb_7_legacy_append_regression.rs:124`, `tests/tb_7_legacy_append_regression.rs:128`.
- Positive control exempts a `shadow_only`-annotated synthetic call: `tests/tb_7_legacy_append_regression.rs:135`, `tests/tb_7_legacy_append_regression.rs:142`, `tests/tb_7_legacy_append_regression.rs:147`.
- Command run: `cargo test --test tb_7_legacy_append_regression -- --nocapture`.

**Observed test result**:

```text
running 3 tests
test gate_7_scanner_positive_control_flags_unannotated_call ... ok
test gate_7_scanner_exempts_shadow_only_annotated_call ... ok
test gate_7_no_unannotated_legacy_append_in_evaluator ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**Fix items**: None for A2.

### A3 — Frame B end-to-end smoke evidence

**Verdict**: **PASS**, with evidence-scope caveat.

**Evidence file:line**:

- `replay_report.json` has `l4_entries = 1`, `l4e_entries = 6`, and all seven boolean indicators true: `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:2`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:3`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:4`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:5`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:6`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:7`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:8`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:9`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:10`.
- ReplayReport aggregator checks exactly the seven booleans: `src/runtime/verify.rs:190`, `src/runtime/verify.rs:191`, `src/runtime/verify.rs:195`, `src/runtime/verify.rs:196`, `src/runtime/verify.rs:197`.
- Smoke README labels the evidence synthetic-LLM, lists L4/L4.E counts, and says all 7 indicators are GREEN: `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md:4`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md:5`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md:10`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md:12`.
- Test header documents the manual real-LLM smoke procedure and explains why CI evidence is synthetic: `tests/tb_7_atom6_chain_backed_smoke.rs:11`, `tests/tb_7_atom6_chain_backed_smoke.rs:20`, `tests/tb_7_atom6_chain_backed_smoke.rs:25`.

**Fix items**:

- Non-blocking: serialize `all_indicators_pass` into `replay_report.json` if downstream humans expect the JSON file itself to contain that summary bit. Current JSON contains the seven booleans but no summary field.

### A4 — TB-6 carry-forward closure

**Verdict**: **PASS**, with one stale-comment cleanup.

**Evidence file:line**:

- `AgentProposalRecord` contains the architect fields and has no `logical_t` member; the local comment states `logical_t` was removed: `src/runtime/agent_audit_trail.rs:83`, `src/runtime/agent_audit_trail.rs:86`, `src/runtime/agent_audit_trail.rs:90`, `src/runtime/agent_audit_trail.rs:98`, `src/runtime/agent_audit_trail.rs:102`, `src/runtime/agent_audit_trail.rs:106`, `src/runtime/agent_audit_trail.rs:109`, `src/runtime/agent_audit_trail.rs:111`, `src/runtime/agent_audit_trail.rs:115`, `src/runtime/agent_audit_trail.rs:116`.
- `audit_hash` domain prefix is v2 and excludes record-level `logical_t`: `src/runtime/agent_audit_trail.rs:132`, `src/runtime/agent_audit_trail.rs:138`, `src/runtime/agent_audit_trail.rs:140`.
- `chain_link` binds row-level `logical_t`: `src/runtime/agent_audit_trail.rs:186`, `src/runtime/agent_audit_trail.rs:192`, `src/runtime/agent_audit_trail.rs:197`.
- `BootstrapError::RejectionWriter` exists and is returned when JSONL L4.E writer bootstrap fails: `src/runtime/mod.rs:183`, `src/runtime/mod.rs:196`, `src/runtime/mod.rs:202`, `src/runtime/mod.rs:390`, `src/runtime/mod.rs:394`.
- Evaluator exits non-zero on ChainTape bootstrap failure: `experiments/minif2f_v4/src/bin/evaluator.rs:674`, `experiments/minif2f_v4/src/bin/evaluator.rs:683`, `experiments/minif2f_v4/src/bin/evaluator.rs:691`.
- I91e structural witness asserts the 10-key serde shape corresponding to 9 logical fields + rejection discriminator, and rejects `logical_t`: `tests/tb_6_agent_audit_trail.rs:198`, `tests/tb_6_agent_audit_trail.rs:204`, `tests/tb_6_agent_audit_trail.rs:218`, `tests/tb_6_agent_audit_trail.rs:227`, `tests/tb_6_agent_audit_trail.rs:231`.

**Fix items**:

- Non-blocking doc cleanup: `src/runtime/agent_audit_trail.rs:29` still says "`9-field record (10 with logical_t for chronological ordering)`"; update it to match Atom 1.7.

### A5 — ChainDerivedRunFacts §4.4 field set

**Verdict**: **PASS** for the requested field-set check; see cross-cutting finding C1 for proposal-count semantics.

**Evidence file:line**:

- Module doc lists exactly the 11 architect structural fields and excludes time-sensitive fields: `src/runtime/chain_derived_run_facts.rs:15`, `src/runtime/chain_derived_run_facts.rs:16`, `src/runtime/chain_derived_run_facts.rs:20`, `src/runtime/chain_derived_run_facts.rs:26`, `src/runtime/chain_derived_run_facts.rs:30`, `src/runtime/chain_derived_run_facts.rs:32`, `src/runtime/chain_derived_run_facts.rs:35`, `src/runtime/chain_derived_run_facts.rs:37`.
- Struct has the 11 fields: `src/runtime/chain_derived_run_facts.rs:64`, `src/runtime/chain_derived_run_facts.rs:65`, `src/runtime/chain_derived_run_facts.rs:69`, `src/runtime/chain_derived_run_facts.rs:72`, `src/runtime/chain_derived_run_facts.rs:75`.
- Constructor populates those fields and keeps `gp_proof_file` `None`: `src/runtime/chain_derived_run_facts.rs:235`, `src/runtime/chain_derived_run_facts.rs:238`, `src/runtime/chain_derived_run_facts.rs:243`, `src/runtime/chain_derived_run_facts.rs:248`.

**Fix items**:

- Field-set fix: none.
- Semantic fix: see §3 action item 2.

### A6 — ProposalTelemetry CAS schema

**Verdict**: **PASS**.

**Evidence file:line**:

- Binding schema is the 8 ruling D5 fields: `src/runtime/proposal_telemetry.rs:102`, `src/runtime/proposal_telemetry.rs:103`, `src/runtime/proposal_telemetry.rs:105`, `src/runtime/proposal_telemetry.rs:108`, `src/runtime/proposal_telemetry.rs:111`, `src/runtime/proposal_telemetry.rs:113`, `src/runtime/proposal_telemetry.rs:115`, `src/runtime/proposal_telemetry.rs:117`.
- Struct has exactly those 8 fields: `src/runtime/proposal_telemetry.rs:120`, `src/runtime/proposal_telemetry.rs:121`, `src/runtime/proposal_telemetry.rs:123`, `src/runtime/proposal_telemetry.rs:125`, `src/runtime/proposal_telemetry.rs:126`, `src/runtime/proposal_telemetry.rs:128`.
- CAS writer stores canonical-encoded telemetry with schema id, and reader fetches / decodes from CAS: `src/runtime/proposal_telemetry.rs:231`, `src/runtime/proposal_telemetry.rs:237`, `src/runtime/proposal_telemetry.rs:239`, `src/runtime/proposal_telemetry.rs:244`, `src/runtime/proposal_telemetry.rs:252`, `src/runtime/proposal_telemetry.rs:256`.
- Structural witness asserts exactly 8 fields and rejects forbidden deliberation/raw-prompt fields: `src/runtime/proposal_telemetry.rs:339`, `src/runtime/proposal_telemetry.rs:347`, `src/runtime/proposal_telemetry.rs:358`, `src/runtime/proposal_telemetry.rs:359`, `src/runtime/proposal_telemetry.rs:364`, `src/runtime/proposal_telemetry.rs:366`.

**Fix items**: None for A6.

### A7 — Self-audit honesty

**Verdict**: **CHALLENGE** due contradictory top-line accounting; detailed §3 is honest.

**Evidence file:line**:

- Contradiction: §0 says "5 of 7 ... CLOSED" and "2 are partial / manual carry-forward": `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:18`.
- Detailed §3 correctly says 3/7 fully closed and 4/7 partially closed: `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:169`, `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:170`.
- §3 preserves the §13.4 anti-pile-up rule and keeps TB-6 audit-pending open: `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:173`, `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:177`.
- Manual real-LLM smoke carry-forward is explicitly documented: `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:14`, `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:128`, `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:253`.

**Fix items**:

- Correct §0 lines 18-19 to match §3: "3/7 fully closed; 4/7 partial / manual carry-forward; TB-6 audit-pending remains open."

## §2 Cross-cutting findings

1. **Diff enumeration mismatch**: The prompt says 9 commits, but `git log --format=%H 05c5be7..9e74195` returned 11 hashes: `9e74195`, `2559c84`, `4cfe7cb`, `d03814f`, `3572141`, `2bc879c`, `0414b30`, `eed4837`, `c3ad31e`, `48c02e2`, `cc7b3dd`. This audit uses the actual command output.

2. **Worktree dirty at audit start**: source/test files audited for A1/A2/A4/A5/A6/A7 had no uncommitted diff, but `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json` and `agent_pubkeys.json` were modified in the worktree. The replay booleans at HEAD and in the worktree are the same; only detail hashes / pubkeys changed. HEAD replay booleans are visible at `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:2` through `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:10`.

3. **Gate 2 / proposal_count semantics are not closed**: `ChainDerivedRunFacts` documentation says `proposal_count` counts accepted + rejected WorkTx, but implementation increments only while walking accepted L4 entries. L4.E records contain `tx_kind` and `tx_payload_cid`, so rejected WorkTx can be counted from chain evidence, but current `compute_run_facts_from_chain` does not do it.

   Evidence: field doc says accepted + rejected at `src/runtime/chain_derived_run_facts.rs:20`; implementation increments only in L4 WorkTx match at `src/runtime/chain_derived_run_facts.rs:162` and `src/runtime/chain_derived_run_facts.rs:173`; L4.E records expose `tx_kind` and `tx_payload_cid` at `src/bottom_white/ledger/rejection_evidence.rs:181`, `src/bottom_white/ledger/rejection_evidence.rs:189`, `src/bottom_white/ledger/rejection_evidence.rs:191`; records are readable at `src/bottom_white/ledger/rejection_evidence.rs:547`.

4. **Smoke is structural, not real-LLM**: this is honestly documented and acceptable for structural Frame B smoke, but it should not be rephrased as completed live DeepSeek + Lean evidence. Evidence: `tests/tb_7_atom6_chain_backed_smoke.rs:11` and `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md:26`.

## §3 Action items

1. **Fail closed after ChainTape bootstrap on authoritative-route failures**.
   - File:line: `experiments/minif2f_v4/src/bin/evaluator.rs:1348`, `experiments/minif2f_v4/src/bin/evaluator.rs:1358`, `experiments/minif2f_v4/src/bin/evaluator.rs:1585`, `experiments/minif2f_v4/src/bin/evaluator.rs:1932`.
   - Suggested fix: change CAS/q_snapshot/signing/submit failures from warn-and-continue to explicit run failure or L4.E-backed rejection evidence. The shadow-only tape call must not be the only state mutation after an authoritative-path failure.
   - Blocking: **Yes** for a strict Class 2 "every real LLM proposal routes through `submit_typed_tx`" production claim.

2. **Repair `ChainDerivedRunFacts.proposal_count` to include rejected WorkTx in L4.E**.
   - File:line: `src/runtime/chain_derived_run_facts.rs:20`, `src/runtime/chain_derived_run_facts.rs:173`, `src/bottom_white/ledger/rejection_evidence.rs:189`, `src/bottom_white/ledger/rejection_evidence.rs:191`, `src/bottom_white/ledger/rejection_evidence.rs:547`.
   - Suggested fix: iterate `l4e_writer.records()`, count records whose `tx_kind` is Work as proposals, and, where `tx_payload_cid` decodes to a WorkTx with nonzero `proposal_cid`, include ProposalTelemetry in token/tactic/tool aggregation or explicitly document why rejected proposals are excluded.
   - Blocking: **Yes** for Gate 2 / exact proposal-count conformance.

3. **Fix self-audit top-line 5/7 vs 3/7 contradiction**.
   - File:line: `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:18`, `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:169`, `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:177`.
   - Suggested fix: update §0 to match §3: 3/7 full, 4/7 partial, TB-6 audit-pending remains open.
   - Blocking: **No** for code, **yes** for relying on the self-audit as an honest ship artifact.

4. **Clean stale `AgentProposalRecord` public-surface comment**.
   - File:line: `src/runtime/agent_audit_trail.rs:29`.
   - Suggested fix: remove "(10 with logical_t for chronological ordering)" and say chronology lives in `AgentAuditTrailIndexRow.logical_t`.
   - Blocking: **No**.

5. **Record exact diff-count / dirty-worktree caveats in the ship packet**.
   - File:line: evidence booleans in `handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json:2`; self-audit range currently open-ended at `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md:4`.
   - Suggested fix: list the 11 actual commit hashes from `git log --format=%H 05c5be7..9e74195`; avoid saying 9 commits.
   - Blocking: **No**.

## §4 Closure recommendation

TB-7 is sufficient for the **structural Frame B happy-path/smoke claim**: real-signature WorkTx / VerifyTx construction exists, evaluator hot paths call `bus.submit_typed_tx`, legacy calls are annotated `shadow_only`, ReplayReport booleans are green on the smoke evidence, and Gate 7 passes 3/3.

TB-7 is **not yet sufficient for the stronger Class 2 production claim** that every real LLM proposal is guaranteed to traverse the authoritative path under operational failure, nor for exact Gate 2 proposal-count conformance. The next TB can proceed only if action items 1 and 2 are carried as blocking follow-up before any beta / production launch claim. Action item 3 should be fixed in the ship artifacts so TB-6 audit-pending status remains unambiguous.
