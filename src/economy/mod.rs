//! TuringOS v4 economy layer (Anti-Oreo Top White cross-cut).
//!
//! Contains RSP-1 modules per `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md`
//! § 19. v4 ship target = 9 modules; v1 lands `money` (CO1.0a P1 prerequisite per Plan v3.2-fix3).
//!
//! /// TRACE_MATRIX WP-econ-§19 + Const-Laws (基本法 1: Coin 守恒): RSP-1 module root

/// TRACE_MATRIX 基本法 1 + Inv 3 + ROADMAP P3:5/P3:6/P3:8: task-keyed escrow vault (RSP-0).
pub mod escrow_vault;
/// TRACE_MATRIX WP § 5.L4 + Art IV + ROADMAP P1:5/P1:6/P1:7/P1:8: L4 accepted-only ledger wrapper (RSP-0).
pub mod ledger;
/// TRACE_MATRIX 基本法 1 + Inv 4 + ROADMAP P3:1/P3:2: monetary invariant guards (RSP-0).
pub mod monetary_invariant;
pub mod money;
