//! TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W8 smoke tests — verifies the
//! heuristic artifact verifier `src/web/verify.rs` catches the documented
//! Qwen3-Coder failure modes and accepts known-good artifacts.
//!
//! Critical real-world cases (LOAD-BEARING):
//!   - `verify_rejects_inverted_nullish_guard` exercises the actual broken
//!     artifact from `handover/evidence/stage_phase7_real_e2e_20260518T031804Z/
//!     audit/attempt_1/index.html` — the Tetris where `player.matrix === null`
//!     guarded the Space-press start logic even though the variable is
//!     assigned `createPiece()` in `resetGame()`. Heuristic MUST flag.
//!   - `verify_accepts_good_artifact` exercises the actual working artifact
//!     from `handover/evidence/.../audit/attempt_2/index.html` — heuristic
//!     MUST accept it cleanly.
//!
//! Run with: `cargo test --test cli_web_verify_smoke --features web`
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Path helper — resolve the Phase 7 real-E2E artifact paths.
// ---------------------------------------------------------------------------

fn phase7_artifact(attempt: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("handover");
    p.push("evidence");
    p.push("stage_phase7_real_e2e_20260518T031804Z");
    p.push("audit");
    p.push(attempt);
    p.push("index.html");
    p
}

// ---------------------------------------------------------------------------
// Helpers — write a temp HTML file and run verify on it.
// ---------------------------------------------------------------------------

fn write_temp_html(name: &str, body: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let path = dir.path().join(name);
    std::fs::write(&path, body).expect("write temp html");
    (dir, path)
}

// A baseline known-good HTML used as the starting point for the negative
// tests — each negative test mutates one aspect to make exactly that
// failure dominate.
//
// Keep it >= MIN_SIZE (2 KB), <= MAX_SIZE (100 KB), contains:
//   - <canvas>
//   - document.addEventListener('keydown', ...)
//   - requestAnimationFrame loop
//   - balanced braces + <script>/</script>
//   - no external script src / stylesheet
fn baseline_good_html() -> String {
    // Build a ~3 KB game-ish skeleton. Padded with comments to clear MIN_SIZE.
    let mut buf = String::new();
    buf.push_str("<!DOCTYPE html><html><head><title>g</title>");
    buf.push_str("<style>body{background:#000;color:#fff}canvas{border:1px solid #fff}</style>");
    buf.push_str("</head><body><canvas id=\"c\" width=\"200\" height=\"400\"></canvas><script>\n");
    buf.push_str("let ctx = document.getElementById('c').getContext('2d');\n");
    buf.push_str("let state = { x: 0, y: 0 };\n");
    buf.push_str("function tick() { ctx.fillStyle='#fff'; ctx.fillRect(state.x, state.y, 10, 10); requestAnimationFrame(tick); }\n");
    buf.push_str("document.addEventListener('keydown', function(e) {\n");
    buf.push_str("  if (e.code === 'ArrowLeft') { state.x = state.x - 1; }\n");
    buf.push_str("  if (e.code === 'ArrowRight') { state.x = state.x + 1; }\n");
    buf.push_str("});\n");
    // Padding comments to clear MIN_SIZE (2 KB).
    for _ in 0..40 {
        buf.push_str(
            "// long padding comment line to clear the minimum-size heuristic threshold ok\n",
        );
    }
    buf.push_str("tick();\n");
    buf.push_str("</script></body></html>\n");
    buf
}

// ---------------------------------------------------------------------------
// Test 1: verify_rejects_truncated_artifact (file < 2 KB)
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_truncated_artifact() {
    let (dir, path) = write_temp_html("tiny.html", "<html></html>");
    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(!outcome.passed, "tiny artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("size_too_small")),
        "must flag size_too_small; reasons={:?}",
        outcome.failure_reasons
    );
    drop(dir);
}

// ---------------------------------------------------------------------------
// Test 2: verify_rejects_oversized_artifact (file > 100 KB)
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_oversized_artifact() {
    // 200 KB of HTML.
    let mut body = String::new();
    body.push_str("<!DOCTYPE html><html><body><canvas id=\"c\"></canvas><script>");
    body.push_str(
        "document.addEventListener('keydown', function(e){}); requestAnimationFrame(function(){});",
    );
    while body.len() < 200_000 {
        body.push_str("// padding to push past MAX_SIZE_BYTES\n");
    }
    body.push_str("</script></body></html>");
    let (dir, path) = write_temp_html("huge.html", &body);
    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(!outcome.passed, "huge artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("size_too_large")),
        "must flag size_too_large; reasons={:?}",
        outcome.failure_reasons
    );
    drop(dir);
}

// ---------------------------------------------------------------------------
// Test 3: verify_rejects_missing_playfield (W8.1: renamed from missing_canvas)
//
// W8 v1 hardcoded `<canvas` substring; W8 Validation Round 1 caught 3 false
// positives where Qwen produced functional Tetris with `display: grid` +
// `.cell` divs. W8.1 broadens to has_playfield(canvas|grid|svg|table|cell).
// This test now strips ALL playfield indicators to confirm rejection.
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_missing_playfield() {
    let mut html = baseline_good_html();
    // Strip the canvas tag. (baseline_good_html() uses <canvas> exclusively;
    // no CSS-grid/SVG/table/cell-class is present, so removing canvas is
    // sufficient to remove ALL playfield indicators.)
    html = html.replace(
        "<canvas id=\"c\" width=\"200\" height=\"400\"></canvas>",
        "",
    );
    let (dir, path) = write_temp_html("noplayfield.html", &html);
    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(!outcome.passed, "no-playfield artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("missing_playfield")),
        "must flag missing_playfield; reasons={:?}",
        outcome.failure_reasons
    );
    drop(dir);
}

// ---------------------------------------------------------------------------
// Test 4: verify_rejects_missing_keyboard_handler
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_missing_keyboard_handler() {
    let mut html = baseline_good_html();
    // Remove the addEventListener block.
    let begin = html.find("document.addEventListener").unwrap();
    let end_marker = "});";
    let end_rel = html[begin..].find(end_marker).unwrap() + end_marker.len();
    html.replace_range(begin..begin + end_rel, "");
    let (dir, path) = write_temp_html("nokb.html", &html);
    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(!outcome.passed, "no-keyboard artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("missing_keyboard_handler")
                || r.contains("keydown_not_on_document_or_window")),
        "must flag missing keyboard handler; reasons={:?}",
        outcome.failure_reasons
    );
    drop(dir);
}

// ---------------------------------------------------------------------------
// Test 5: verify_rejects_missing_animation_loop
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_missing_animation_loop() {
    let mut html = baseline_good_html();
    html = html.replace("requestAnimationFrame(tick)", "/* removed */");
    // Also ensure setInterval isn't present.
    assert!(
        !html.contains("setInterval"),
        "baseline must not use setInterval"
    );
    let (dir, path) = write_temp_html("noloop.html", &html);
    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(!outcome.passed, "no-animation-loop artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("missing_animation_loop")),
        "must flag missing_animation_loop; reasons={:?}",
        outcome.failure_reasons
    );
    drop(dir);
}

// ---------------------------------------------------------------------------
// Test 6: verify_rejects_external_script_src
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_external_script_src() {
    let mut html = baseline_good_html();
    // Inject an external script src before the inline <script>.
    html = html.replace(
        "<canvas",
        "<script src=\"https://cdn.example.com/jquery.js\"></script><canvas",
    );
    let (dir, path) = write_temp_html("ext.html", &html);
    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(!outcome.passed, "external script src must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("external_script_src")),
        "must flag external_script_src; reasons={:?}",
        outcome.failure_reasons
    );
    drop(dir);
}

// ---------------------------------------------------------------------------
// Test 7: verify_rejects_unbalanced_braces
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_unbalanced_braces() {
    // Construct HTML with 50 `{` and 100 `}`.
    let mut body = String::new();
    body.push_str("<!DOCTYPE html><html><body><canvas id=\"c\"></canvas><script>\n");
    body.push_str("document.addEventListener('keydown', function(e){});\n");
    body.push_str("requestAnimationFrame(function(){});\n");
    // Fifty opens, hundred closes.
    for _ in 0..50 {
        body.push('{');
    }
    for _ in 0..100 {
        body.push('}');
    }
    body.push('\n');
    // Pad past MIN_SIZE.
    for _ in 0..40 {
        body.push_str(
            "// padding line padding line padding line padding line padding line padding line\n",
        );
    }
    body.push_str("</script></body></html>");
    let (dir, path) = write_temp_html("unbal.html", &body);
    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(!outcome.passed, "unbalanced braces must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("unbalanced_braces")),
        "must flag unbalanced_braces; reasons={:?}",
        outcome.failure_reasons
    );
    drop(dir);
}

// ---------------------------------------------------------------------------
// Test 8 — LOAD-BEARING: verify_rejects_inverted_nullish_guard
// Uses the EXACT broken Phase 7 E2E attempt_1 index.html.
// ---------------------------------------------------------------------------

#[test]
fn verify_rejects_inverted_nullish_guard() {
    let path = phase7_artifact("attempt_1");
    assert!(
        path.exists(),
        "phase 7 attempt_1 artifact must exist at {:?}",
        path
    );

    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(
        !outcome.passed,
        "the REAL broken Phase 7 attempt_1 Tetris MUST be flagged; reasons={:?}",
        outcome.failure_reasons
    );
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("inverted_nullish_guard")),
        "must flag inverted_nullish_guard for the real broken artifact; reasons={:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 9 — LOAD-BEARING: verify_accepts_good_artifact
// Uses the EXACT working Phase 7 E2E attempt_2 index.html.
// ---------------------------------------------------------------------------

#[test]
fn verify_accepts_good_artifact() {
    let path = phase7_artifact("attempt_2");
    assert!(
        path.exists(),
        "phase 7 attempt_2 artifact must exist at {:?}",
        path
    );

    let outcome = web::verify::verify_artifact_html(&path).expect("verify ok");
    assert!(
        outcome.passed,
        "the REAL working Phase 7 attempt_2 Tetris MUST pass; reasons={:?}",
        outcome.failure_reasons
    );
    assert!(
        outcome.failure_reasons.is_empty(),
        "no failure reasons expected for the good artifact; got {:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 10 — LOAD-BEARING: W8.1 regression — DOM-grid Tetris accepted
//
// W8 v1 hardcoded `<canvas` substring; W8 Validation Round 1 caught Qwen
// producing functional Tetris using `display: grid` + dynamically-created
// `.cell` divs (3 consecutive false positives, all rejected). W8.1 broadens
// `has_playfield()` to accept CSS-grid signature. This test uses the EXACT
// 11814-byte W8-validation artifact (sha256 a857599b…d666) as a fixture and
// asserts it now PASSES.
// ---------------------------------------------------------------------------

#[test]
fn verify_accepts_dom_grid_tetris_w8_1_regression() {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("handover");
    p.push("evidence");
    p.push("stage_phase7_w8_validation_20260518T041310Z");
    p.push("dom_grid_tetris_fixture.html");
    assert!(
        p.exists(),
        "W8 validation DOM-grid Tetris fixture must exist at {:?}",
        p
    );

    let outcome = web::verify::verify_artifact_html(&p).expect("verify ok");
    assert!(
        outcome.passed,
        "the REAL DOM-grid Tetris from W8 Validation Round 1 MUST pass after W8.1; reasons={:?}",
        outcome.failure_reasons
    );
    assert!(
        outcome.failure_reasons.is_empty(),
        "no failure reasons expected for the DOM-grid Tetris; got {:?}",
        outcome.failure_reasons
    );
}
