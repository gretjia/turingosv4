//! TRACE_MATRIX FC1a-predicate_pi: TDMA-Bounded JudgeAI predicate module.
//!
//! Constitution explicitly permits JudgeAI as a verdict authority for FC1
//! predicates. This module holds the math-step judge used in the Atom 7.5
//! real-evidence run on the user-supplied problem
//! "证明所有自然数之和 = -1/12 via m·exp(-m/N)·cos(m/N)".
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

/// TRACE_MATRIX FC1a-predicate_pi: math-step JudgeAI submodule.
pub mod math_step_judge;
/// TRACE_MATRIX FC1a-predicate_pi: deterministic scripted-verdict judge (Atom 9 stress fixture).
pub mod injected_judge;
/// TRACE_MATRIX FC1a-predicate_pi: Nesbitt's-inequality multi-category step judge (Atom 10).
pub mod nesbitt_step_judge;
/// TRACE_MATRIX FC1a-predicate_pi: Putnam 2024 A1 strict step judge (Atom 13 extreme stress).
pub mod putnam_2024_a1_judge;
