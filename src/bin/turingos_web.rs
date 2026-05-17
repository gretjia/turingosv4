//! TRACE_MATRIX FC2-N16: TuringOS Phase 7 Web MVP binary.
//!
//! Binds to `127.0.0.1:8080` HARD (no flag, no env-var override — per
//! Phase 7 §8 architect-ratified decision #4: localhost:8080 HARD constraint).
//! Non-loopback binding is Phase 8+ scope.
//!
//! Build and run:
//!   cargo run --bin turingos_web --features web
//!
//! If built WITHOUT `--features web` the binary stubs out with a friendly
//! error and exits 2, so `cargo build --bin turingos_web` never silently
//! produces a no-op binary.
#![cfg(feature = "web")]

// Declare the `web` module from its sibling directory without touching
// `src/lib.rs` (which is a hard-constraint DO-NOT-TOUCH surface per
// Phase 7 §7 and the W0 task brief).
#[path = "../web/mod.rs"]
mod web;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("hardcoded addr is valid");
    let router = web::router::build_router();
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind 127.0.0.1:8080");
    println!("TuringOS Phase 7 Web MVP listening on http://{addr}");
    axum::serve(listener, router)
        .await
        .expect("axum serve error");
}
