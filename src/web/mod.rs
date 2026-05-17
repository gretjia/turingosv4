/// TRACE_MATRIX FC2-N16: Phase 7 Web MVP — module root for the HTTP/WebSocket
/// server surface. Declared from `src/bin/turingos_web.rs` via
/// `#[path = "../web/mod.rs"]` because `src/lib.rs` is a hard-constraint
/// DO-NOT-TOUCH surface (Phase 7 §7). All items are `pub(crate)` or lower;
/// no public API leaks from this module tree.
pub(crate) mod artifact;
pub(crate) mod fixtures;
pub(crate) mod generate;
pub(crate) mod ir;
pub(crate) mod render;
pub(crate) mod router;
pub(crate) mod spec;
pub(crate) mod store;
pub(crate) mod write;
pub(crate) mod ws;
