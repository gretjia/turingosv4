# Phase 8 R1-R8 Re-Audit Brief — VETO/CHALLENGE Addressment Verification

**Date**: 2026-04-22
**Scope**: single amendment commit `2502dc9` on `experiment/phase-8a-snapshot-fix`.
**Focus**: verify each of **R1-R8** truly addresses the corresponding VETO/CHALLENGE from the prior dual audit.
**Not in scope**: re-reviewing the preceding 5 commits (already audited). Call out only if R1-R8 introduces a NEW issue on previously-reviewed code.

---

## Prior audit outcomes (2026-04-22 earlier today)

**Gemini batch audit**: CHALLENGE (3 items)
**Codex batch audit**: **VETO** (2 items) + CHALLENGE (3 items)
Conservative verdict: **VETO** → R1-R8 must clear 2 VETOs + addres CHALLENGEs.

Prior reports:
- `handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md`
- `handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md`

## R1-R8 mapping

| R | Prior finding | Severity | Claude claim |
|---|---|---|---|
| **R1** | OracleReceipt forgeable + step-mode context replay | Codex VETO | Private fields + `oracle_nonce` + `context_hash`; bus `register_oracle` gate |
| **R2** | oneshot ephemeral bus `wal: None` + never halts | Codex VETO | `with_wal_path` per-problem + `halt_and_settle` emits durable `Halt{OmegaAccepted}` |
| **R3** | `has_bare_tactic` string/comment FP + Unicode unsafe | Gemini+Codex CHALLENGE | `strip_strings_and_comments()` + char-boundary scan; 8 new tests |
| **R4** | WAL replay doesn't restore `q_state` | Gemini CHALLENGE | `with_wal_path` scans last Halt event → set q_state |
| **R5** | `reputation` in snapshot but not rendered in prompt | Codex additional risk | `build_agent_prompt` gains `own_reputation` param; evaluator passes it |
| **R6** | `halt_with_reason` live/durable divergence semantics unclear | Codex CHALLENGE | Expanded doc comment explicitly splitting live vs durable |
| **R7** | `Predicate` trait unused (M-1 preservation) | Gemini CHALLENGE | `#[allow(dead_code)]` + justification |
| **R8** | `#[serde(default)]` logic correct but untested | Codex CHALLENGE | 2 new tests deserialize pre-8.F JSON, assert empty-map default |

## 必答 8 项（VETO-first）

For each R, return **PASS** (issue cleared) / **CHALLENGE** (improvement left) / **VETO** (still broken). Evidence = `file:line` or grep.

### R1 — VETO cleared?

Expected fix evidence:
- `src/sdk/oracle_receipt.rs`: struct fields are private; constructors `new_lean4_complete/new_lean4_partial` take `oracle_nonce: u64`; validates takes `expected_context: &[u8;32]`
- `src/bus.rs`: `registered_oracle_nonces: HashSet<u64>` + `register_oracle()` + append_oracle_accepted rejects unregistered nonce
- `experiments/minif2f_v4/src/lean4_oracle.rs`: `nonce: u64` field + `nonce()` accessor
- `experiments/minif2f_v4/src/bin/evaluator.rs`: all 4 receipt construction sites pass `oracle.nonce()`; all 3 `register_oracle` call sites
- `tests/oracle_receipt_bus.rs`: `blessed_write_rejects_unregistered_oracle_nonce` + `blessed_write_rejects_cross_context_replay`

**Verify**: is the receipt truly not forgeable from code that lacks a `Lean4Oracle` reference? Is the step-mode replay attack blocked by context_hash?

### R2 — VETO cleared?

Expected fix evidence:
- `experiments/minif2f_v4/src/bin/evaluator.rs` run_oneshot: constructs `TuringBus::with_wal_path(..., wal/oneshot/<problem>_<ts>.wal.jsonl)`, calls `register_oracle`, `init`, `append_oracle_accepted`, then `halt_and_settle(&[node_id])`
- Fallback to in-memory only on WAL open error (logged warn)
- halt_and_settle emits Halt event via halt_with_reason (Phase 8.E)

**Verify**: after oneshot returns, does `wal/oneshot/<problem>_*.wal.jsonl` exist on disk containing at minimum an Append + Halt event? (Can grep the file path pattern + `fs::exists` reasoning; don't actually run.)

### R3 — CHALLENGE cleared?

Expected fix:
- `experiments/minif2f_v4/src/lean4_oracle.rs`: new `strip_strings_and_comments()` function; `has_bare_tactic_invocation` uses `.chars().next_back()` / `.chars().next()` not byte indexing
- 8 new tests: `ignores_string_literal_occurrence`, `ignores_escape_sequences_in_string`, `ignores_line_comment_occurrence`, `ignores_block_comment_occurrence`, `catches_bare_tactic_even_with_comments_elsewhere`, `handles_unicode_identifier_neighbors`, `handles_unicode_in_comment_and_string`, `strip_leaves_real_tactic_visible`

**Verify**:
- String literal `"decide"` not flagged?
- `αβγ.decide` (Unicode namespace) not flagged?
- Bare `by decide` still flagged if also surrounded by comments?
- Block comment `/- ... decide ... -/` ignored?

Edge cases to probe: nested comments (Lean actually supports), `decide` as part of doc string content, multi-line string.

### R4 — CHALLENGE cleared?

Expected fix:
- `src/bus.rs::with_wal_path`: after replaying events, scan `bus.ledger.events().iter().rev()` for first Halt; set `bus.q_state = QState::Halted { reason }`
- Two new tests: `wal_replay_restores_q_state_halted`, `wal_replay_preserves_running_when_no_halt`

**Verify**: does restoration happen before `bus.wal = Some(...)` so halt state is set before subsequent ops? Does it correctly preserve Running when no Halt event in replay?

### R5 — CHALLENGE cleared?

Expected fix:
- `src/sdk/prompt.rs::build_agent_prompt`: new `own_reputation: u32` param between `balance` and `tools_description`; rendered as `Reputation: {} citations\n`
- `experiments/minif2f_v4/src/bin/evaluator.rs`: build_agent_prompt call passes `snap.get_reputation(agent_id)`
- All 5 prompt unit tests updated

**Verify**: reputation line actually in prompt output for a non-zero-rep agent? Prior absence was the issue — now it's structurally present, but is the LLM-facing phrasing clear ("Reputation: N citations")?

### R6 — CHALLENGE cleared?

Expected fix: doc comment on `halt_with_reason` now explicitly documents:
- Durable (ledger/WAL) records *first* reason only
- Live `q_state` updates to *latest* reason
- `halt_reason_distribution` metric uses durable, not live

**Verify**: is the semantic choice clear enough that future auditors won't re-flag it?

### R7 — CHALLENGE cleared?

Expected fix: `src/sdk/predicate.rs` `Predicate` trait has `#[allow(dead_code)]` with justification comment pointing at GENERALIZATION_ROADMAP.

**Verify**: warning gone? Justification clear?

### R8 — CHALLENGE cleared?

Expected fix: `tests/reputation.rs` adds:
- `tape_deserializes_without_reputation_field`: old-format Tape JSON → empty reputation map
- `snapshot_deserializes_without_reputation_field`: old-format UniverseSnapshot JSON → empty reputation map

**Verify**: tests use realistic pre-8.F JSON snapshots?

---

## Files to read (absolute paths)

Amendment diff: `/home/zephryj/projects/turingosv4/handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff`

Worktree (current state after R1-R8):
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/sdk/oracle_receipt.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/sdk/predicate.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/sdk/prompt.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/bus.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/src/lean4_oracle.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/src/bin/evaluator.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/tests/oracle_receipt_bus.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/tests/q_halt_state.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/tests/reputation.rs`

Prior audit reports:
- `/home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md`
- `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md`

---

## Output

**Verdict per R** (R1 through R8): PASS / CHALLENGE / VETO + 1-line evidence.
**Overall**: PASS → progress to Phase 2 A/B. CHALLENGE → list residuals. VETO → stop.

Save to:
- Codex: `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_PHASE_8_R1R8_2026-04-22.md`
- Gemini: `/home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_8_R1R8_2026-04-22.md`

**C-066**: verify file actually saved with `ls -la`. If Codex sandbox blocks again, return full text in transcript.
