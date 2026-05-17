/// TRACE_MATRIX FC2-N16: Phase 7 Web MVP — minimal axum 0.7 router.
/// Foundation only; no state, no WebSocket wiring yet (that lands in W2).
/// The single route GET `/` returns an HTML placeholder page confirming
/// the scaffold is alive. All items are `pub(crate)`.
use axum::{response::Html, routing::get, Router};

/// TRACE_MATRIX FC2-N16: Build the axum router for the TuringOS Web MVP.
/// W0 scaffold: one GET `/` route. WebSocket + read-endpoints land in W1/W2.
pub(crate) fn build_router() -> Router {
    Router::new().route("/", get(root_handler))
}

async fn root_handler() -> Html<&'static str> {
    Html(
        "<!doctype html>\
<html lang=\"en\">\
<head><meta charset=\"utf-8\"><title>TuringOS</title></head>\
<body><h1>TuringOS</h1><p>Phase 7</p>\
<p>TuringOS Phase 7 \u{2014} Web MVP placeholder</p>\
</body></html>",
    )
}
