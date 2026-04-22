# Gemini Phase 8 R1-α + R2 + R3 Re-Audit
**Date**: 2026-04-22
**Elapsed**: 46.1s
**Prompt size**: 124,793 chars
**Target commit**: 4a72507 (R1-α + R2 + R3 addressment)

---

# Phase 8 R1-α + R2 + R3 Re-Audit Brief — Final Addressment

**Auditor**: Codex
**Date**: 2026-04-22
**Outcome**: CHALLENGE

---

### R1-α — Is Ed25519 capability truly unforgeable?

**Verdict**: **PASS**
**Evidence**: `bus.rs:121` (`oracles_frozen` check) + `tests/oracle_receipt_bus.rs:140` (`attacker_with_mut_bus_cannot_forge_post_init`).

**Analysis**:
The replacement of the nonce with an Ed25519 signature is a complete and robust fix for the VETO. The core attack vector identified in the prior audit (an attacker with `&mut Bus` registering their own capability post-init) is now closed by the `oracles_frozen` flag.

1.  **Forgery by `&mut Bus`**: This is prevented. `bus.init()` and `bus.with_wal_path()` (on non-empty replay) both set `oracles_frozen = true`. Any subsequent call to `register_oracle` returns an `Err`, as confirmed by the new test `post_init_register_oracle_fails`. The end-to-end test `attacker_with_mut_bus_cannot_forge_post_init` correctly simulates the full attack and asserts its failure.
2.  **`oracles_frozen` Timing**: The flag is set at the correct lifecycle points (`bus.rs:215` in `init`, `bus.rs:203` in `with_wal_path`), effectively locking the set of trusted public keys before any transactions can occur.
3.  **`sign_new` Visibility**: The function is `pub`, but this is safe. Its signature requires a `&SigningKey`, and since oracles keep their signing keys private, an attacker cannot forge a receipt for a trusted oracle. The capability is correctly bound to the key, not the function call.
4.  **Canonical Message**: The `signable_message` encoding (`oracle_receipt.rs:168`) is injective. It uses fixed-size hashes followed by a properly implemented tagged union for the `Verdict`, preventing any ambiguity or collision between different receipt parameters.
5.  **`SigningKey` Clone**: Cloning the `SigningKey` inside `Lean4Oracle` is safe and intentional for the specified design, where a single oracle instance serves multiple roles (bus tool, receipt issuer) within a single trusted process. It is not a capability leak to an untrusted context.

The implementation is sound, follows cryptographic best practices, and is backed by strong, specific tests that replicate the original VETO condition.

### R2 — Is WAL-open failure handled correctly?

**Verdict**: **CHALLENGE**
**Evidence**: `experiments/minif2f_v4/src/bin/evaluator.rs:355` (in original file, not diff) still contains the silent fallback for `run_swarm`.

**Analysis**:
The fix for `run_oneshot` is correct. It now properly emits a `MEASUREMENT_ERROR` and returns a non-successful result (`evaluator.rs:253`) instead of silently degrading to an in-memory bus. This upholds the durability guarantee for oneshot runs.

However, the exact same issue was overlooked and **not fixed** in the `run_swarm` code path. The `match TuringBus::with_wal_path(...)` block for `run_swarm` still contains the `warn!(...); falling back to in-memory bus` logic. This is a significant residual issue, as it means swarm runs can silently lose their durability guarantees on a WAL-open error (e.g., disk full, permissions issue), potentially leading to data loss or non-reproducible experiments. The fix is incomplete.

### R3 — Nested comments fully handled?

**Verdict**: **PASS**
**Evidence**: `experiments/minif2f_v4/src/lean4_oracle.rs:441` (depth counter logic) + `tests:handles_nested_block_comments`.

**Analysis**:
The fix is correct and robust. The `strip_strings_and_comments` function now implements a depth counter for block comments (`/- ... -/`), which is the standard algorithm for handling nested structures.

1.  **Edge Cases**: The logic correctly handles multiple levels of nesting, unterminated comments (by breaking on EOF), and interaction with string literals (which are checked for and stripped first).
2.  **Doc Comments**: Lean doc comments (`/--` and `/-!`) are correctly treated as a variant of block comments, entering the same stripping logic and being terminated by `- /`.
3.  **Tests**: The new tests `handles_nested_block_comments` and `handles_doc_comments` specifically validate the new logic against the cases that would have failed previously.

The implementation is sound and fully addresses the prior challenge.

---

### Overall Verdict: **CHALLENGE**

The critical R1 VETO has been resolved with a high-quality, cryptographically sound fix. The R3 challenge on comment parsing is also fully addressed.

However, the fix for the R2 WAL durability challenge was incomplete, leaving a silent failure mode in the `run_swarm` path. While not a security vulnerability like R1, it is a correctness and data integrity issue that must be resolved.

**Recommendation**:
- List the R2 residual (`run_swarm` WAL fallback) as a high-priority task.
- Proceed to Phase 2 A/B testing, as the core VETO is fixed and the residual issue does not compromise the security model, though it may affect experiment reliability.