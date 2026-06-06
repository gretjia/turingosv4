//! Workload adapter boundary.
//!
//! A workload adapter is user workload evidence, not OS authority. These
//! modules are intentionally compiled through tests for A14 without wiring the
//! kernel or sequencer.

/// TRACE_MATRIX FC3: adapter result claim boundary is report-side evidence, not kernel authority.
pub mod benchmark_boundary;
/// TRACE_MATRIX FC3: Lean workload adapter namespace stays outside kernel authority.
pub mod lean;
/// TRACE_MATRIX FC3: market research preregistration stays outside kernel authority.
pub mod market_research;
/// TRACE_MATRIX FC3: SWE-bench workload adapter namespace stays outside kernel authority.
pub mod swebench;
