/// TRACE_MATRIX FC2-N16: Phase 7 Web MVP — module root for the HTTP/WebSocket
/// server surface. Declared from `src/bin/turingos_web.rs` via
/// `#[path = "../web/mod.rs"]` because `src/lib.rs` is a hard-constraint
/// DO-NOT-TOUCH surface (Phase 7 §7). All items are `pub(crate)` or lower;
/// no public API leaks from this module tree.
pub(crate) mod artifact;
/// TRACE_MATRIX FC2-N16: Phase 7 web — UI IR fixtures submodule.
pub(crate) mod fixtures;
/// TRACE_MATRIX FC2-N16: Phase 7 web — POST /api/generate handler submodule (Phase 6.3 codegen wire).
pub(crate) mod generate;
/// TRACE_MATRIX FC2-N16: Phase 7 web — UI IR types + serde shapes.
pub(crate) mod ir;
/// TRACE_MATRIX FC2-N16: Phase 7 web — server-side IR-to-HTML renderer.
pub(crate) mod render;
/// TRACE_MATRIX FC2-N16: Phase 7 web — axum route table builder.
pub(crate) mod router;
/// TRACE_MATRIX FC2-N16: Phase 7 web — POST /api/spec/submit handler (8-question grill wrapper).
pub(crate) mod spec;
/// TRACE_MATRIX FC2-N16: Phase 7 web — in-memory task store (AppState).
pub(crate) mod store;
/// TRACE_MATRIX FC2-N16: Phase 7 web — heuristic artifact verifier (auto-retry trigger).
pub(crate) mod verify;
/// TRACE_MATRIX FC2-N16: Phase 7 web — onboarding welcome flow submodule.
pub(crate) mod welcome;
/// TRACE_MATRIX FC2-N16: Phase 7 web — POST /api/task/open handler submodule.
pub(crate) mod write;
/// TRACE_MATRIX FC2-N16: Phase 7 web — WebSocket handler + WsBroadcastMsg tagged union.
pub(crate) mod ws;

// W7: re-export driven-mode session types so integration tests can construct them.
pub use ws::{GrillSession, SlotState, WsBroadcastMsg};
