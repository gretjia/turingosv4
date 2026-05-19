//! TRACE_MATRIX FC1-N5 + FC1-N10: F11 (2026-05-19) — quality predicates must
//! be domain-agnostic.
//!
//! Π5 smoke (handover/evidence/phase6_3_x_universality_1779111375/pi5_smoke/
//! p7_traditional/verdict.json) caught the W8 game-shape heuristics
//! false-positive-rejecting a Traditional Chinese video-converter UI
//! (5384 bytes, valid HTML5, correct domain). Defect D-NEW-4 (P0
//! universality): ANY non-game spec (todo, dashboard, video converter,
//! CRUD form, etc.) was rejected with kind=generate_quality_failed.
//!
//! F11 fix (Option B): replace game-shape heuristics with a minimum
//! viable HTML5 bar (VerifyMode::MinimumBar). Game-shape heuristics are
//! preserved behind VerifyMode::GameShape, picked by generate_handler
//! when spec.md mentions game keywords. This test suite locks in the
//! domain-agnostic behavior.
//!
//! Run with: cargo test --features web --test cmd_generate_quality_predicates_domain_agnostic
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use web::verify::{spec_looks_like_game, verify_html_contents_with_mode, VerifyMode};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Run the minimum-bar verifier and return the outcome.
fn run_minimum_bar(html: &str) -> web::verify::VerifyOutcome {
    let size = html.len() as u64;
    verify_html_contents_with_mode(html, size, VerifyMode::MinimumBar)
}

/// The exact 5384-byte video-converter artifact shape that the Π5 smoke
/// observed. We embed a representative slice (the head, dropZone, and a
/// realistic script body) so the test does not depend on
/// tmp/universality_campaign/ files. Padded above 2 KB so it passes any
/// generous size threshold and well above the F11 500-byte floor.
fn video_converter_html_5kb() -> String {
    let mut buf = String::new();
    buf.push_str(
        r#"<!DOCTYPE html>
<html lang="zh-TW">
<head>
    <meta charset="UTF-8">
    <title>影片轉檔工具</title>
    <style>
        body { font-family: 'Microsoft JhengHei', Arial, sans-serif; background-color: #f5f5f5; }
        .container { max-width: 600px; margin: 0 auto; }
        #dropZone { border: 3px dashed #ccc; padding: 40px; text-align: center; }
        #dropZone.highlight { border-color: #4CAF50; }
        .download-btn { background-color: #4CAF50; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <h1>影片轉檔工具</h1>
        <p>支援拖曳上傳，輸出為 MP4 格式</p>
        <div id="dropZone">
            <p>拖曳影片檔案到這裡，或點擊選擇檔案</p>
            <input type="file" id="fileInput" accept="video/*" style="display: none;">
        </div>
        <div id="fileList"></div>
    </div>
    <script>
        const dropZone = document.getElementById('dropZone');
        const fileInput = document.getElementById('fileInput');
        const fileList = document.getElementById('fileList');
        const seenHashes = new Set();
        dropZone.addEventListener('click', () => fileInput.click());
        dropZone.addEventListener('dragover', (e) => {
            e.preventDefault();
            dropZone.classList.add('highlight');
        });
        dropZone.addEventListener('dragleave', () => dropZone.classList.remove('highlight'));
        dropZone.addEventListener('drop', (e) => {
            e.preventDefault();
            dropZone.classList.remove('highlight');
            handleFiles(e.dataTransfer.files);
        });
        fileInput.addEventListener('change', (e) => handleFiles(e.target.files));
        async function sha256(file) {
            const buf = await file.arrayBuffer();
            const hash = await crypto.subtle.digest('SHA-256', buf);
            return Array.from(new Uint8Array(hash))
                .map(b => b.toString(16).padStart(2, '0'))
                .join('');
        }
        async function handleFiles(files) {
            for (const file of files) {
                const hash = await sha256(file);
                if (seenHashes.has(hash)) continue;
                seenHashes.add(hash);
                const item = document.createElement('div');
                item.className = 'file-item';
                item.innerHTML = `
                    <div class="file-name">${file.name}</div>
                    <div class="file-status">已上傳，模擬轉檔中...</div>
                    <button class="download-btn" disabled>下載 MP4</button>
                `;
                fileList.appendChild(item);
                setTimeout(() => {
                    item.querySelector('.file-status').textContent = '轉檔完成';
                    item.querySelector('.download-btn').disabled = false;
                }, 1500);
            }
        }
    </script>
</body>
</html>
"#,
    );
    // Pad past 2 KB so size_too_small never trips even on stricter floors;
    // F11 MinimumBar threshold is 500 bytes so this is well clear regardless.
    while buf.len() < 5_000 {
        buf.push_str(
            "<!-- pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad -->\n",
        );
    }
    buf
}

fn todo_app_html() -> String {
    let mut buf = String::new();
    buf.push_str(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Todo App</title>
    <style>
        body { font-family: system-ui, sans-serif; max-width: 480px; margin: 2rem auto; }
        li.done { text-decoration: line-through; color: #888; }
        input[type=text] { width: 70%; padding: 4px 8px; }
        button { padding: 4px 12px; }
    </style>
</head>
<body>
    <h1>Tasks</h1>
    <form id="addForm">
        <input type="text" id="addInput" placeholder="New task">
        <button type="submit">Add</button>
    </form>
    <ul id="todoList"></ul>
    <script>
        const tasks = [];
        const list = document.getElementById('todoList');
        const form = document.getElementById('addForm');
        const input = document.getElementById('addInput');
        function render() {
            list.innerHTML = '';
            for (const [i, t] of tasks.entries()) {
                const li = document.createElement('li');
                li.textContent = t.text;
                if (t.done) li.className = 'done';
                li.onclick = () => { tasks[i].done = !tasks[i].done; render(); };
                list.appendChild(li);
            }
        }
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            const v = input.value.trim();
            if (!v) return;
            tasks.push({ text: v, done: false });
            input.value = '';
            render();
        });
    </script>
</body>
</html>
"#,
    );
    buf
}

// ---------------------------------------------------------------------------
// Test 1: accepts_video_converter_html_5kb
//
// LOAD-BEARING: reproduces the Π5 smoke false-positive. The 5384-byte
// Traditional Chinese video-converter (drag-drop, SHA256, MP4 conversion)
// MUST pass the F11 domain-agnostic gate.
// ---------------------------------------------------------------------------

#[test]
fn accepts_video_converter_html_5kb() {
    let html = video_converter_html_5kb();
    let outcome = run_minimum_bar(&html);
    assert!(
        outcome.passed,
        "video converter (Traditional Chinese, drag-drop, 5 KB) MUST pass the MinimumBar gate; reasons={:?}",
        outcome.failure_reasons
    );
    assert!(
        outcome.failure_reasons.is_empty(),
        "no failure reasons expected; got {:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 2: accepts_todo_app_html
//
// Generic English todo app — no canvas, no keyboard handler, no animation
// loop. MUST pass the MinimumBar gate.
// ---------------------------------------------------------------------------

#[test]
fn accepts_todo_app_html() {
    let html = todo_app_html();
    let outcome = run_minimum_bar(&html);
    assert!(
        outcome.passed,
        "generic todo app MUST pass the MinimumBar gate; reasons={:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 3: rejects_too_small_artifact
//
// A 100-byte stub MUST fail with reason containing `too_small`.
// ---------------------------------------------------------------------------

#[test]
fn rejects_too_small_artifact() {
    let html = "<!DOCTYPE html><html><body>hi</body></html>";
    assert!(html.len() < 500, "fixture must be under 500 bytes");
    let outcome = run_minimum_bar(html);
    assert!(!outcome.passed, "tiny artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("too_small")),
        "must flag too_small; reasons={:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 4: rejects_placeholder_content
//
// HTML containing literal `TODO` / `lorem ipsum` MUST fail.
// ---------------------------------------------------------------------------

#[test]
fn rejects_placeholder_content_todo() {
    let mut html = todo_app_html();
    // Inject a literal TODO comment that should trip the placeholder check.
    // (The existing todo app uses `todoList` and `tasks`, no bare TODO.)
    html.push_str("<!-- TODO: finish the styling -->\n");
    let outcome = run_minimum_bar(&html);
    assert!(!outcome.passed, "TODO-bearing artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("placeholder_content")),
        "must flag placeholder_content; reasons={:?}",
        outcome.failure_reasons
    );
}

#[test]
fn rejects_placeholder_content_lorem_ipsum() {
    let mut html = todo_app_html();
    html.push_str("<p>Lorem ipsum dolor sit amet.</p>\n");
    let outcome = run_minimum_bar(&html);
    assert!(!outcome.passed, "lorem-ipsum-bearing artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("placeholder_content")),
        "must flag placeholder_content; reasons={:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 5: rejects_no_script_or_style
//
// Bare HTML with no <script>, <style>, or <link rel="stylesheet"> MUST fail.
// ---------------------------------------------------------------------------

#[test]
fn rejects_no_script_or_style() {
    // Build a bare-bones HTML over 500 bytes but with NO script/style/link.
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><title>bare</title></head>\n");
    html.push_str("<body>\n<h1>Hello</h1>\n<p>Bare HTML body.</p>\n");
    while html.len() < 600 {
        html.push_str("<p>filler paragraph filler paragraph filler paragraph filler.</p>\n");
    }
    html.push_str("</body></html>");
    let outcome = run_minimum_bar(&html);
    assert!(!outcome.passed, "bare HTML must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("no_script_or_style")),
        "must flag no_script_or_style; reasons={:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 6: rejects_missing_html_root and empty body
// ---------------------------------------------------------------------------

#[test]
fn rejects_missing_html_root() {
    // Plain text masquerading as HTML — no <html> or <!DOCTYPE html>.
    let mut html = String::new();
    html.push_str("Just some text content.\n");
    html.push_str("<style>body{color:red}</style>\n");
    while html.len() < 600 {
        html.push_str("more text more text more text more text more text more text\n");
    }
    let outcome = run_minimum_bar(&html);
    assert!(!outcome.passed, "non-HTML artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("missing_html_root")),
        "must flag missing_html_root; reasons={:?}",
        outcome.failure_reasons
    );
}

#[test]
fn rejects_empty_body() {
    // Valid HTML5 shape with non-empty <head> + <style>, but empty <body>.
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><title>empty</title>\n");
    html.push_str("<style>body{background:#000}</style>\n");
    // Pad head with comments so the file passes the size floor.
    while html.len() < 600 {
        html.push_str("<!-- padding padding padding padding padding padding padding -->\n");
    }
    html.push_str("</head><body>   \n\t  </body></html>");
    let outcome = run_minimum_bar(&html);
    assert!(!outcome.passed, "empty-body artifact must fail");
    assert!(
        outcome
            .failure_reasons
            .iter()
            .any(|r| r.contains("missing_or_empty_body")),
        "must flag missing_or_empty_body; reasons={:?}",
        outcome.failure_reasons
    );
}

// ---------------------------------------------------------------------------
// Test 7: spec_looks_like_game detection (Step 4 bonus)
// ---------------------------------------------------------------------------

#[test]
fn spec_detection_game_keywords() {
    // English keywords
    assert!(spec_looks_like_game("Build a tetris game with arrow keys"));
    assert!(spec_looks_like_game("snake clone with canvas"));
    assert!(spec_looks_like_game("Make a Breakout-style arcade"));
    // Simplified Chinese
    assert!(spec_looks_like_game("做一个俄罗斯方块游戏"));
    // Traditional Chinese
    assert!(spec_looks_like_game("做一個俄羅斯方塊遊戲"));
}

#[test]
fn spec_detection_non_game() {
    // The exact Π5 video-converter spec one-liner — MUST NOT match game.
    let spec = "想做一個影片轉檔工具, 支援拖曳上傳, 輸出 mp4。介面用繁體中文";
    assert!(!spec_looks_like_game(spec), "video converter spec must not match game");
    assert!(!spec_looks_like_game("Build a todo app with localStorage"));
    assert!(!spec_looks_like_game("CRUD dashboard for inventory"));
    assert!(!spec_looks_like_game("Markdown editor with preview"));
}

// ---------------------------------------------------------------------------
// Test 8: mode-aware regression — GameShape mode still strict
//
// The 5384-byte video converter passes MinimumBar (test 1) but MUST fail
// GameShape (the W8 legacy mode). This locks in the layering: the gate
// is mode-controlled at the call site, not silently weakened.
// ---------------------------------------------------------------------------

#[test]
fn video_converter_fails_game_shape_mode() {
    let html = video_converter_html_5kb();
    let size = html.len() as u64;
    let outcome = verify_html_contents_with_mode(&html, size, VerifyMode::GameShape);
    assert!(
        !outcome.passed,
        "video converter MUST still fail GameShape mode (no canvas/keydown/raf); reasons={:?}",
        outcome.failure_reasons
    );
}
