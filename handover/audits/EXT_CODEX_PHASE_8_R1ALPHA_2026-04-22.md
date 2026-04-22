# Codex Phase 8 R1-α + R2 + R3 Re-Audit
**Date**: 2026-04-22
**Subagent**: codex:codex-rescue
**Target commit**: `4a72507`
**File save**: Codex CLI sandbox blocked write (`read-only filesystem`); Claude persisted transcript manually per C-066.

---

## Overall Verdict: **CHALLENGE**

R1 VETO **cleared**. R3 cleared. R2 has subtle but real signaling gap.

| R | Verdict | One-line |
|---|---|---|
| R1-α | **PASS** | Ed25519 + oracles_frozen + private fields — attack path from `&mut Bus` blocked at all checked points |
| R2 | CHALLENGE | silent fallback gone, but `measurement_error` goes into `gp_path` not `condition`; batch scripts treat as ordinary failure, not retry-worthy |
| R3 | **PASS** | depth counter + string/comment/Unicode all handled |

---

## R1-α — PASS (VETO cleared)

Codex walked through 9 attack vectors, none succeed:

1. **`register_oracle` post-init** → `Err(oracles_frozen)`. `init()` sets flag before tool hooks; `with_wal_path` sets it after non-empty replay.
2. **Forged receipt with attacker SigningKey** → rejected at `trusted_oracle_pubs` check (first gate in `append_oracle_accepted`).
3. **Creating fresh `Lean4Oracle`** — doesn't help: new key not in trusted set.
4. **Race on `oracles_frozen`** — no: both `register_oracle` + `init` take `&mut self`, single-threaded writes.
5. **Receipt field tampering** — private fields, signature verification catches same-module mutation.
6. **`sign_new` being `pub`** — OK; requires `&SigningKey`, no path from `&mut Bus` to a trusted oracle's key.
7. **Signed message encoding injective** — fixed 32-byte hashes + tag bytes + LE f64 + length-prefixed Reject. No collision path found.
8. **`SigningKey` via `Clone`** — only usable by caller who already holds the oracle.
9. **Step-mode context replay** — receipt's `context_hash` includes parent_id, bus recomputes and checks.

**Attribution to Codex's original attack**: the `attacker_with_mut_bus_cannot_forge_post_init` test replays the exact scenario Codex described → rejected.

Residual non-security note: `with_wal_path` resumes with empty trusted set → WAL-resumed runs currently fail to re-register their oracle (registration frozen by replay). Availability regression on crash-resume, not forgeability. Paper 1 scope can accept; Phase 10 should address if crash-resume matters for soak tests.

## R2 — CHALLENGE (measurement_error signaling gap)

What's right:
- Silent `wal: None` fallback **removed** from both `run_oneshot` (evaluator.rs:243-267) AND `run_swarm` (evaluator.rs:342-374).
- `eprintln!("MEASUREMENT_ERROR ...")` emitted on WAL failure.

What's wrong (Codex catch):
- `make_pput(..., Some("measurement_error".to_string()), ...)` puts the string into **`gp_path`** field, NOT `condition`.
- Batch scripts (`run_batch.sh:162-180` etc.) only trigger retry when **no `PPUT_RESULT`** line is emitted. My branch still calls `make_pput()` which emits `PPUT_RESULT:...` → batch scripts record ordinary failure, not retry.

**Fix plan**: either (a) don't emit `PPUT_RESULT` at all on WAL failure (just stderr + early return), or (b) add `measurement_error: bool` field to PputResult and have make_pput skip emission.

## R3 — PASS

`strip_strings_and_comments` with depth counter correctly handles:
- 2-deep + 3-deep nesting (`handles_nested_block_comments` test)
- Doc comments `/--` / `/-!`
- String literal that contains `/-` → NOT a comment opener (string branch checked first)
- Unterminated `/- ...` → loop break on EOF
- Unicode-safe word boundary

## Note on `swarm` WAL fix

Codex audited commit `4a72507` which at the time had `run_oneshot` fixed but NOT `run_swarm`. Since this audit, swarm was also fixed (commit amendment in the worktree, not yet committed at Codex's verdict time). The swarm-side silent fallback is now removed.

---

## Meta (C-066)

Codex sandbox blocked write to the output path. Codex also couldn't locate `EXT_CODEX_PHASE_8_R1R8_2026-04-22.md` (found `EXT_CODEX_PHASE_8_BATCH_2026-04-22.md` instead). Both issues recorded but don't affect verdict credibility.
