//! TRACE_MATRIX FC2-N16: Phase 7 W0 smoke test — verifies the web router
//! module compiles and the router type is constructable without binding a port.
//!
//! Gated on `#[cfg(feature = "web")]` so non-web builds never see this.
//! Run with: `cargo test --test cli_web_smoke --features web`
#![cfg(feature = "web")]

// Mirror the same path-based module declaration used in `turingos_web.rs`
// so the test exercises the exact same module tree.
#[path = "../src/web/mod.rs"]
mod web;

#[test]
fn web_router_builds_without_binding() {
    // TRACE_MATRIX FC2-N16: prove the router is constructable (crate compiles
    // with web feature, router type is usable) without actually binding a port.
    let router = web::router::build_router();
    // axum::Router implements Debug; just confirming the type exists.
    let _dbg = format!("{router:?}");
}

#[test]
fn web_router_is_send() {
    // TRACE_MATRIX FC2-N16: axum routers must be Send for tokio multi-thread.
    fn assert_send<T: Send>(_: T) {}
    assert_send(web::router::build_router());
}
