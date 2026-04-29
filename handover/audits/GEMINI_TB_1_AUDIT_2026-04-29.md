# Gemini TB-1 Round-1 Dual External Audit
**Date**: 2026-04-29
**Target**: TB-1 Days 1-5 ship readiness (commits 063b003..HEAD)
**HEAD**: 6c04c26a658b150e94b1d3148046531c65ef98ef
**Prompt size**: 197491 chars
**API latency**: 53.0s
**Mandate**: strategic / architectural / constitutional (Q1-Q8). Independent of Codex r1 (parallel).

---

### Section A: Verdict

**Verdict: PASS**
**Conviction: 5/5**

TB-1 Days 1-5 represents an exceptionally disciplined execution of the 9-phase roadmap. The central claim holds: the architectural primitives (L4/L4.E split) and invariant assertions (RSP-0) have been established as pure, decoupled components. By avoiding premature `dispatch_transition` wiring, TB-1 successfully lays the foundation for TB-2 without introducing scope creep, phase-ordering violations, or state-machine contamination.

---

### Section B: Per-Question Disposition

**Q1. Phase-ordering integrity (P1 → P3 → P5/P6)**
**Cleanly orthogonal.** The decision to wire the P6 `h_vppu` metric post-hoc in `evaluator.rs` `main()` (rather than deep inside the state machine or `make_pput`) is an architectural win. It keeps the core consensus and transition logic completely unaware of P6 analytical telemetry. This perfectly honors the phase-ordering principle: P6 is acting strictly as an out-of-order "anchor evidence" overlay, creating zero technical debt for future P5 MetaTape work.

**Q2. L4 vs L4.E primitive separation**
**Architecturally sound and cryptographically isolated.** 
a. Two ledgers is the correct primitive count. A single ledger with a "status" field would pollute `replay_full_transition` and risk Goodharting the read views.
b. TB-2's `escrow_lock_tx` will fit cleanly into L4 as a standard accepted transition; no third "pending" ledger is needed because escrow is correctly modeled as a sub-index of `EconomicState`.
c. The disjointness mechanism is robust: `ledger.rs` uses `b"turingosv4.l4_accepted.v1"` and `rejection_evidence.rs` uses `b"turingosv4.l4e_rejection_evidence.v1"`. This domain separation guarantees that a hash from one ledger can never collide with or be replayed on the other.

**Q3. Monetary invariant scope (RSP-0)**
**Perfectly scoped.** 
a. RSP-0 correctly ships the *rules* (pure functions) and *data structures* (in-memory vault) without forcing premature I/O wiring. 
b. The Tier-A test 7 using an N=5 deterministic sequence is sufficient for a tracer bullet; it proves the closed-loop conservation property across the fundamental sub-indices (balances, escrow). 
c. The lack of `escrow_vault` in Tier-A tests is architecturally correct for RSP-0: the vault is unit-tested now, and will be integration-tested in TB-2 when `escrow_lock_tx` actually exercises it.

**Q4. h_vppu Goodhart-shield**
**Safe and spec-compliant.** 
a. Returning `None` when mean=0 is the correct anti-Goodhart behavior; it preserves "no signal" semantics rather than emitting skewing values (like 0.0 or inf) into the JSONL. 
b. The rolling window (N=3) means the baseline shifts, which correctly implements the charter's `mean(history N=1..3)` spec for measuring *recent* regression. 
c. As noted in Q1, the divergence to wire post-hoc in `main()` rather than inside `make_pput` is a justified architectural improvement that keeps I/O at the outermost edge.

**Q5. Tier-A vs Tier-B downgrade**
**Highly defensible.** The CF-5 downgrade correctly isolates deterministic infrastructure verification (P1/P3) from stochastic live-LLM capability tests (P6). The framing is exactly right: TB-1 guarantees P1/P3 correctness, while P6 capability is anchored by out-of-band evidence. Using `#[ignore]` for T10 is the correct Rust idiom to document the contract in the harness without breaking deterministic CI runs.

**Q6. TRACE_MATRIX coverage**
**Excellent hygiene.** Every new module and public symbol carries precise `TRACE_MATRIX` backlinks to the Constitution, Roadmap, or PREREG. The use of the `orphan` tag for `h_vppu_history.rs` with explicit justification to PREREG § 5 is the correct application of the alignment standard.

**Q7. Trust Root manifest hygiene**
**Flawless.** All 5 new files (`monetary_invariant.rs`, `escrow_vault.rs`, `ledger.rs`, `rejection_evidence.rs`, `h_vppu_history.rs`) are present. Furthermore, the implementation correctly caught and rehashed `src/economy/mod.rs` and `src/bottom_white/ledger/mod.rs` to account for the new `pub mod` declarations. This demonstrates deep attention to the R-014/R-018 Trust Root protocol.

**Q8. STEP_B-protected file violation**
**No violations.** `src/kernel.rs` and `src/bus.rs` hashes remain unchanged in the manifest, and `src/sdk/tools/wallet.rs` was not touched. The implementation successfully built the new primitives around the protected core.

---

### Section C: P0 List (Must-Fix)

*(None. The implementation is ship-ready.)*

---

### Section D: P1 List (Should-Fix / Observations)

1. **P6 Rolling Baseline Semantics (Observation):** 
   While `h_vppu_history` correctly implements the N=3 rolling window per the charter, note that for long-term Epistemic Lab tracking (Phase P6 v2+), a shifting baseline makes absolute capability comparisons difficult over time. *Action for future:* When P6 gets its own dedicated TB, consider adding a parallel `static_baseline` field that never drops the first successful run. No changes needed for TB-1.
2. **Forward-Compat of `exempt_tx_kinds`:** 
   The design of `assert_total_ctf_conserved` taking an `exempt_tx_kinds` slice is excellent forward-thinking. It ensures that when P3 RSP-4 (System Rewards) or P8 (Autonomous Economy) introduces legitimate mints/burns, the core invariant function won't need a rewrite.

---

### Section E: TB-2 Readiness Assessment

**TB-2 (RSP-1) is fully unblocked and ready to begin.**

TB-1 leaves the codebase in an ideal state:
*   The **L4 / L4.E ledgers** are ready to receive entries.
*   The **monetary invariants** are pure functions ready to be called inside the `Sequencer`'s `dispatch_transition` match arms.
*   The **escrow vault** is ready to be wrapped by the `SettlementEngine`.

Because TB-1 strictly avoided premature wiring, TB-2 can focus entirely on implementing the `escrow_lock_tx`, `work_tx`, and `yes_stake_tx` state transitions without having to untangle or refactor any existing P1/P3 spaghetti. Proceed to TB-2.