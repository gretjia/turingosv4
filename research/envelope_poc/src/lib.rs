//! Research PoC for AgentOutputEnvelope.
//!
//! Validates 4 of the 7 assertions made in
//! `handover/research/AGENT_OUTPUT_ENVELOPE_RESEARCH_2026-05-26.md`:
//!   A1 — structure-gate vs predicate-gate decoupling
//!   A3 — every EnvelopeValidationSubclass surjects to {ParseFailed, PolicyViolation}
//!   A4 — FC1 invariant LHS unchanged when envelope sub-classes route through parse_fail bucket
//!   A5 — envelope rejection diagnostics carry more information than current ad-hoc parser
//!
//! NOT validated here (out of PoC scope, requires real run):
//!   A2 — zero touch on src/state/sequencer.rs / typed_tx.rs (validated by grep, see VALIDATION_PLAN)
//!   A6 — privacy invariant CR-18R.4 v2 (validated by EnvelopeRejectionPayload byte inspection)
//!   A7 — per-benchmark schema fixtures (deferred to Phase A real impl)
//!
//! Standalone subcrate; does NOT depend on turingosv4. Surrogate types mirror
//! the main crate's enums by value/variant order. If main-crate enums drift,
//! the surrogate mapping table here MUST be regenerated.

pub mod envelope;
