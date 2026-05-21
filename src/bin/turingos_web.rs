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
//!
//! W4: Constructs `AppState` with a broadcast channel (capacity = 64) and
//! passes it into the router via `.with_state()`. Logs the resolved workspace
//! directory at startup (`TURINGOS_WEB_WORKSPACE` env var, default = cwd).
#![cfg(feature = "web")]

// Declare the `web` module from its sibling directory without touching
// `src/lib.rs` (which is a hard-constraint DO-NOT-TOUCH surface per
// Phase 7 §7 and the W0 task brief).
#[path = "../web/mod.rs"]
mod web;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // OBS: initialize log backend so `log::info!` events from src/web/* reach stderr.
    // Default filter: env_logger respects RUST_LOG env var; falls back to "info".
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Resolve and log workspace directory.
    let workspace = if let Ok(v) = std::env::var("TURINGOS_WEB_WORKSPACE") {
        if !v.is_empty() {
            v
        } else {
            std::env::current_dir()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|_| ".".to_string())
        }
    } else {
        std::env::current_dir()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| ".".to_string())
    };
    println!("TuringOS Phase 7 Web MVP — workspace: {workspace}");
    println!(
        "  (set TURINGOS_WEB_WORKSPACE to override; \
         TURINGOS_BACKEND_OVERRIDE to replace the turingos binary)"
    );

    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("hardcoded addr is valid");
    // build_with_state initialises AppState (broadcast channel capacity=64).
    let router = web::router::build_with_state(64);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind 127.0.0.1:8080");
    println!("TuringOS Phase 7 Web MVP listening on http://{addr}");
    axum::serve(listener, router)
        .await
        .expect("axum serve error");
}
