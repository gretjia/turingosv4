# Phase 8 R1-α + R2 + R3 Re-Audit Brief — Final Addressment

**Date**: 2026-04-22
**Target commit**: `4a72507` on `experiment/phase-8a-snapshot-fix`
**Focus**: verify the Codex-driven final fixes clear the prior **VETO** (R1) + residual **CHALLENGE** (R2, R3).
**Out of scope**: R4-R8 (both auditors already PASS; any regression flag must be explicit).

## Prior audit outcome (re-audit round 2)

Codex (`EXT_CODEX_PHASE_8_R1R8_2026-04-22.md`):
- R1 = VETO (nonce capability still forgeable by `&mut Bus`)
- R2 = CHALLENGE (WAL-open fallback silently drops `wal: None`)
- R3 = CHALLENGE (nested block comments unsupported)
- R4-R8 = PASS

Gemini (`EXT_GEMINI_PHASE_8_R1R8_2026-04-22.md`):
- All PASS

Conservative verdict: **VETO** → R1-α/R2/R3 must clear.

## What changed in commit `4a72507`

### R1-α (Codex VETO fix — real crypto capability)

- Added `ed25519-dalek = "2"` dep in both `turingosv4` + `minif2f_v4`.
- `OracleReceipt` — fields private; added `issuer_pub: [u8;32]` + `signature: [u8;64]`; `sign_new` signs canonical message `(payload_hash || context_hash || kind_byte || verdict_encoding)` with a `SigningKey`.
- `OracleReceipt::verify_and_match(payload, expected_ctx, &VerifyingKey)` runs: consistency (pub matches claimed issuer), Ed25519 verify, payload hash, context hash, verdict non-reject.
- `Lean4Oracle` — private `SigningKey` field (fresh per `new()`); exposes only `public_key() -> [u8;32]`, `issue_complete_receipt(payload, parent)`, `issue_partial_receipt(payload, parent)`.
- `TuringBus` — `trusted_oracle_pubs: HashSet<[u8;32]>` + `oracles_frozen: bool`. `register_oracle(pub_key) -> Result<(), String>` returns Err after `init()` or non-empty WAL replay.
- `bus.append_oracle_accepted(author, payload, parent, receipt)` — checks (a) `receipt.issuer_pub()` ∈ trusted set (b) Ed25519 signature verifies (c) payload/context/verdict bindings.
- Evaluator refactored: **single** persistent `Lean4Oracle` per run_swarm / run_oneshot; registered BEFORE `init()`; cloned for tool-mount; per-tx oracle creation removed.

### R2 (WAL durability)

Removed silent in-memory fallback in run_oneshot. If `TuringBus::with_wal_path` fails, emit `MEASUREMENT_ERROR oneshot WAL: <e>` and return a non-result PputResult (condition=`measurement_error`, solved=false, tx=1). Batch runner retries; no bus without WAL will ever claim OMEGA.

### R3 (nested block comments)

`strip_strings_and_comments` now tracks block-comment depth (u32 counter). Openings `/-` increment, closings `-/` decrement; strip continues until depth==0. Doc comments `/--` / `/-!` enter the same branch and strip correctly. New tests: `handles_nested_block_comments`, `handles_doc_comments`.

## 必答 3 项

For each R-item, return **PASS** (cleared) / **CHALLENGE** (improvement left) / **VETO** (still broken) + evidence (`file:line`).

### R1-α — Is Ed25519 capability truly unforgeable?

Key questions:
1. Can a caller with `&mut TuringBus` post-init mint a valid receipt? Try to construct an attack path that doesn't violate any invariant.
2. Is `oracles_frozen` set at the right times? (after init(), after non-empty WAL replay) Missed any path?
3. Does `OracleReceipt::sign_new` need private visibility, or is it safe for any caller (no SigningKey → no forgery)? Trade-off: tests + oracle both construct receipts via `sign_new`; making it private would require adding trait extension.
4. Is the canonical signed-message encoding (`payload_hash || context_hash || kind_byte || verdict_encoding`) injective? Could two different `(payload, parent, verdict, kind)` tuples produce the same signing input?
5. `Lean4Oracle: Clone` copies the SigningKey. Is that a capability leak (two bus calls can share the same trust anchor)? Intended per R1-α design (one oracle per run, mounted and used for receipts); but worth flagging.

### R2 — Is WAL-open failure handled correctly?

Key questions:
1. Is `measurement_error` the right way to signal this? Would `ErrorHalt` via halt_with_reason be more consistent?
2. What about the **swarm** path (`run_swarm` line ~348-365)? That code still has the old WAL-fallback-to-new pattern. R2 only fixed oneshot; swarm still degrades silently on WAL error. Is that a missed fix or intentional?

### R3 — Nested comments fully handled?

Key questions:
1. Does the depth counter correctly handle all edge cases:
   - `/- inside /- a -/ b -/ c` — correct?
   - Unterminated `/- ...` — correct?
   - `/- /- /-` (3 opens, 0 closes) — does loop terminate?
   - `/-` inside a string literal (should NOT count as comment start)?
2. Are doc comments `/--` treated correctly? They start with `/` + `-`, which enters the block branch; inside, a `-/` closes. Fine?

### Optional: R4-R8 sanity check

If you see any regression from R1-α/R2/R3 changes on R4-R8 areas (e.g., halt_and_settle still working, reputation still wired, serde still deserializes), note it. Otherwise skip.

## Files to read (absolute paths)

Amendment diff: `/home/zephryj/projects/turingosv4/handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff`

Worktree current state:
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/sdk/oracle_receipt.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/sdk/predicate.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/bus.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/src/lean4_oracle.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/src/bin/evaluator.rs`
- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/tests/oracle_receipt_bus.rs`

Prior audits:
- `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_PHASE_8_R1R8_2026-04-22.md`
- `/home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_8_R1R8_2026-04-22.md`

## Output

**Per-R verdict**: PASS / CHALLENGE / VETO + 1-line evidence.
**Overall**: PASS → proceed Phase 2 A/B. CHALLENGE → list residuals. VETO → stop.

Save to:
- Codex: `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md`
- Gemini: `/home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md`

**C-066**: `ls -la` verify. Codex sandbox blocks → paste full text in transcript.
