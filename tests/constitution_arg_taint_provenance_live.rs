//! LIVE CONSTITUTION GATE — arg-taint PROVENANCE forward-wiring: the PRODUCTION
//! `MemoryKernel::step_forward` entry-point now DERIVES real arg-taint findings
//! from the proposal on the tape, instead of passing the empty `&[]` placeholder.
//!
//! STATUS: LIVE / GREEN. Added under the §8 token
//! `APPROVE-ARG-TAINT-ADMISSION-SUBARTICLE` (forward-wiring of the same
//! sub-article) — LIVE-FC1 Phase 4 production-driving of the proven-but-unwired
//! `arg_taint_v1` analysis.
//!
//! ── WHAT THE EXISTING GATE PROVED vs WHAT THIS ONE PROVES ─────────────────────
//! `tests/constitution_arg_taint_admission.rs` proves the hard-gate when findings
//! are HAND-BUILT and threaded through the explicit `step_forward_with_taint`.
//! But the PRODUCTION call path (`step_forward` → `step_forward_with_claims` →
//! `step_forward_with_workspace`) used to pass `step_forward_with_taint(..., &[])`
//! — the EMPTY findings gap. No real proposal could ever produce a finding.
//!
//! This gate proves the gap is CLOSED: the production `step_forward` derives the
//! `WtoolCall` from the worker's on-tape raw_output via
//! `arg_taint_provenance::derive_wtool_call_from_proposal`, runs `arg_taint_v1`,
//! and routes the result into the admission hard-gate — with NO change to the
//! kernel call signature any existing caller uses.
//!
//! ── NON-VACUITY (three legs, the production entry-point) ──────────────────────
//!   (1) NO FALSE REJECT (the binding constraint): an ordinary proposal — NO
//!       `wtool_call` declaration in its header — ADMITS through the production
//!       `step_forward` exactly as before (verified head advances). This is the
//!       common case; the forward-wiring must not regress it.
//!
//!   (2) NEW FINDING ON GENUINE PROVENANCE: a proposal whose header declares a
//!       `wtool_call` routing an EXTERNAL-provenance arg (interop_capsule ingress
//!       precedent) into a privileged (economic-wallet) sink is REJECTED by the
//!       production `step_forward` (verified head frozen, no `Q_{t+1}`).
//!
//!   (3) PROVENANCE IS THE SOLE DISCRIMINATOR: the SAME header shape but with a
//!       TRUSTED-provenance arg ADMITS. Same sink, same call shape — only the
//!       declared provenance flips the outcome. This is the non-vacuity proof.
//!
//! ── DETERMINISTIC / REPLAY-STABLE (Art.V.2 + Art.0.2) ────────────────────────
//! `derive_wtool_call_from_proposal` is a pure function of the raw_output bytes
//! (already on the AgentProposal tape node) — no RNG, no wall-clock. Leg (4)
//! drives the SAME proposal twice and asserts the SAME verdict.
//!
//! ── ZERO-PINNED-FILE DISCIPLINE ──────────────────────────────────────────────
//! The provenance module is nested as a `#[path]` submodule of the UNPINNED
//! `src/predicate_admission.rs`; the kernel seam is the UNPINNED
//! `src/memory_kernel.rs`. No genesis-pinned file is touched.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_arg_taint_provenance_live`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh` and
//! built by `cargo test --workspace`.

use turingosv4::charter_core::compile_charter_core;
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger};
use turingosv4::memory_kernel::{EnvironmentResult, MemoryKernel, Task};
use turingosv4::tokenizer::Tokenizer;

// ── fixtures ─────────────────────────────────────────────────────────────────

/// Build a kernel at verified head `H0` with an empty (zero-root) registry — the
/// legacy happy path: an admitted Proceed advances the head.
fn kernel_at_h0() -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, "run-arg-taint-provenance-live", charter)
}

/// An ordinary worker Proceed — NO `wtool_call` declaration. The COMMON CASE.
fn ordinary_proceed() -> EnvironmentResult {
    EnvironmentResult {
        raw_output: r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED"}
---BODY---
the proof is done"#
            .to_string(),
        raw_stderr: String::new(),
        success: true,
    }
}

/// A worker Proceed whose header declares a `wtool_call` routing an
/// EXTERNAL-provenance arg into a privileged (economic-wallet) sink + a `wallet/`
/// write namespace — the confused-deputy hazard.
fn external_arg_into_wallet_proceed() -> EnvironmentResult {
    EnvironmentResult {
        raw_output: r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED","wtool_call":{"args":[{"name":"recipient","value":"attacker-addr-from-the-web","source":"external"}],"tools":[{"tool_id":"wallet.transfer","capability":"economic_wallet","permission_policy":"open","side_effect_class":"none","determinism_class":"idempotent_write"}],"write_keys":["wallet/agent-7/balance"]}}
---BODY---
pay out"#
            .to_string(),
        raw_stderr: String::new(),
        success: true,
    }
}

/// The SAME `wtool_call` shape (same wallet sink, same write key) but with a
/// TRUSTED-provenance arg. The clean positive control.
fn trusted_arg_into_wallet_proceed() -> EnvironmentResult {
    EnvironmentResult {
        raw_output: r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED","wtool_call":{"args":[{"name":"recipient","value":"operator-treasury-address","source":"trusted"}],"tools":[{"tool_id":"wallet.transfer","capability":"economic_wallet","permission_policy":"open","side_effect_class":"none","determinism_class":"idempotent_write"}],"write_keys":["wallet/agent-7/balance"]}}
---BODY---
pay out"#
            .to_string(),
        raw_stderr: String::new(),
        success: true,
    }
}

/// Drive the PRODUCTION `step_forward` (NOT `step_forward_with_taint`) and report
/// whether the verified head advanced (admitted).
fn production_step_forward_admits(env: EnvironmentResult) -> bool {
    let mut k = kernel_at_h0();
    let task = Task {
        id: "t1".into(),
        prompt: "transfer funds".into(),
    };
    let before = k.tape.get_verified_head();
    let _step = k.step_forward(&task, env);
    let after = k.tape.get_verified_head();
    after != before
}

// ── LEG (1): NO FALSE REJECT — the common case still admits ───────────────────

/// The forward-wiring must NOT regress the common case: an ordinary Proceed (no
/// `wtool_call`) admits through the PRODUCTION `step_forward` exactly as before.
#[test]
fn production_step_forward_admits_ordinary_proposal() {
    assert!(
        production_step_forward_admits(ordinary_proceed()),
        "NO-FALSE-REJECT REGRESSION: the production step_forward did NOT advance \
         the verified head for an ORDINARY proposal (no wtool_call declaration). \
         The provenance derivation must yield an empty WtoolCall → no findings → \
         decide_admission_with_taint delegates to the passing decide_admission. \
         An ordinary agent proposal to a non-privileged sink MUST still admit."
    );
}

// ── LEG (2): NEW FINDING — genuine external provenance into a privileged sink ──

/// A proposal declaring an EXTERNAL-provenance arg flowing into a privileged
/// (economic-wallet) sink is REJECTED by the PRODUCTION step_forward — the head
/// stays frozen at H0 (no `Q_{t+1}`). This is the finding that was IMPOSSIBLE
/// before forward-wiring (the `&[]` gap).
#[test]
fn production_step_forward_rejects_external_arg_into_privileged_sink() {
    let mut k = kernel_at_h0();
    let task = Task {
        id: "t1".into(),
        prompt: "transfer funds".into(),
    };
    let _step = k.step_forward(&task, external_arg_into_wallet_proceed());
    assert_eq!(
        k.tape.get_verified_head(),
        "H0",
        "ARG-TAINT FORWARD-WIRING REGRESSION: the production step_forward ADMITTED \
         (advanced the head) a Proceed whose declared wtool argument is \
         external-provenance (UntrustedExternal) and flows into a privileged \
         (economic-wallet) sink. The production path must DERIVE the WtoolCall from \
         the on-tape raw_output, run arg_taint_v1, and route the finding through \
         decide_admission_with_taint → Fail → handle_rejection (no advance)."
    );
}

// ── LEG (3): provenance is the sole discriminator ─────────────────────────────

/// The SAME header shape with a TRUSTED-provenance arg ADMITS. Same sink, same
/// call shape — only the declared provenance differs. Proves the gate is
/// non-vacuous (it does not reject every wtool_call Proceed).
#[test]
fn production_step_forward_admits_trusted_arg_into_same_sink() {
    assert!(
        production_step_forward_admits(trusted_arg_into_wallet_proceed()),
        "NON-VACUITY: a Trusted-provenance arg into the SAME wallet sink must ADMIT \
         through the production step_forward (no tainted→privileged flow → no \
         findings). A failure here means the forward-wiring over-rejects clean \
         privileged calls."
    );
}

/// META-GUARD — the external-provenance and trusted-provenance proposals reach
/// OPPOSITE verdicts though they differ ONLY in the declared arg provenance. The
/// single non-vacuity assertion: provenance alone flips the production admission.
#[test]
fn declared_provenance_is_the_sole_discriminator() {
    let external_admitted = production_step_forward_admits(external_arg_into_wallet_proceed());
    let trusted_admitted = production_step_forward_admits(trusted_arg_into_wallet_proceed());
    assert_ne!(
        external_admitted, trusted_admitted,
        "NON-VACUITY VIOLATION: the production step_forward reached the SAME verdict \
         for an external-provenance call ({external_admitted}) and a \
         trusted-provenance call ({trusted_admitted}) that differ ONLY in the \
         declared arg provenance. The forward-wired derivation must REJECT the \
         external-provenance flow and ADMIT the trusted one."
    );
    assert!(!external_admitted && trusted_admitted);
}

// ── LEG (4): deterministic / replay-stable ────────────────────────────────────

/// The provenance derivation is a pure function of the on-tape raw_output: the
/// SAME proposal driven twice yields the SAME admission verdict (no RNG, no
/// wall-clock). Art.V.2 + Art.0.2 replay-stability.
#[test]
fn production_provenance_verdict_is_replay_stable() {
    let a = production_step_forward_admits(external_arg_into_wallet_proceed());
    let b = production_step_forward_admits(external_arg_into_wallet_proceed());
    assert_eq!(a, b, "replay-stability: identical proposal → identical verdict");
    assert!(!a, "the external-provenance proposal must reject on every replay");

    let c = production_step_forward_admits(ordinary_proceed());
    let d = production_step_forward_admits(ordinary_proceed());
    assert_eq!(c, d, "replay-stability: identical ordinary proposal → identical verdict");
    assert!(c, "the ordinary proposal must admit on every replay");
}
