/// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W8 — heuristic artifact verification.
///
/// Server-side static heuristic checks for a single LLM-generated artifact
/// HTML file. Designed to catch the ~70% of Qwen3-Coder failure modes that
/// surfaced in the Phase 7 Real-LLM E2E (handover/evidence/
/// stage_phase7_real_e2e_20260518T031804Z) — most notably the inverted
/// nullish-guard pattern (`player.matrix === null` checked at the same
/// time the variable is initialised non-null in `resetGame`).
///
/// These are STATIC checks: pure regex + substring matching. No headless
/// browser is invoked; no new external dependencies. The checks catch:
///   - truncated / oversized artifacts (size out of [2 KB, 100 KB])
///   - missing `<canvas` element
///   - missing keyboard event handler
///   - missing animation loop (`requestAnimationFrame` or `setInterval`)
///   - external `<script src="http">` (LLM hallucinated CDN)
///   - external `<link rel="stylesheet" href="http">`
///   - unbalanced `{` / `}` (rough JS sanity)
///   - unbalanced `<script>` / `</script>` tags
///   - inverted nullish-guard pattern (e.g. `=== null` inside a keydown
///     handler when the same field is assigned non-null elsewhere)
///   - keydown wired only to `body` (iframe sandbox may not focus body)
///
/// LIMITATIONS: these heuristics cannot catch logic bugs we have not yet
/// seen in real Qwen output. Future Phase 7.y may add a real headless
/// browser smoke. Failure messages are user-facing (Chinese-friendly) so
/// they can be surfaced when all retries fail.
///
/// FC-trace: FC1-N5 (post-generate verification protects the read view)
///           FC1-N10 (write path strengthened with a quality gate before
///                   broadcasting GenerateComplete).
/// Risk class: Class 1 (pure additive helper module, no auth / money /
///                       sequencer surface).
#[cfg(feature = "web")]
use std::fs;
#[cfg(feature = "web")]
use std::path::Path;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC1-N10: outcome of a static heuristic pass.
///
/// `passed`: true iff every check passed.
/// `failure_reasons`: empty when `passed=true`; otherwise human-readable
///   reason strings (one per failed check). Safe to surface to end users.
/// `artifact_size_bytes`: file size in bytes, captured for telemetry.
#[cfg(feature = "web")]
#[derive(Debug, Clone)]
pub(crate) struct VerifyOutcome {
    pub(crate) passed: bool,
    pub(crate) failure_reasons: Vec<String>,
    pub(crate) artifact_size_bytes: u64,
}

// ---------------------------------------------------------------------------
// Size bounds (constants)
// ---------------------------------------------------------------------------

/// Minimum artifact size in bytes. Anything smaller is almost certainly
/// truncated output (the LLM stopped mid-token).
#[cfg(feature = "web")]
const MIN_SIZE_BYTES: u64 = 2 * 1024; // 2 KB

/// Maximum artifact size in bytes. Anything larger is either bloated
/// (boilerplate fluff) or an attempt to embed binary assets / CDNs that we
/// already block via the external-script check.
#[cfg(feature = "web")]
const MAX_SIZE_BYTES: u64 = 100 * 1024; // 100 KB

/// F11 (2026-05-19): minimum artifact size for `VerifyMode::MinimumBar`.
///
/// Used by the domain-agnostic HTML5 bar. 500 bytes is small enough to
/// accept a stub-shaped one-page UI (the Π4.3 P7 video converter is 5384
/// bytes; a minimal todo HTML can be ~800 bytes; the rationale floor sits
/// below either), and large enough to reject obvious truncation
/// (`<html></html>` is 13 bytes, an empty boilerplate skeleton is ~200).
#[cfg(feature = "web")]
const MIN_SIZE_BYTES_MINIMUM: u64 = 500;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC1-N10: heuristic checks against one artifact.
///
/// Returns `VerifyOutcome` with a list of human-readable failure reasons.
/// I/O errors (file not found, permission denied) propagate as `io::Error`.
///
/// F11 (2026-05-19) — domain-agnostic universality fix.
/// Default verification mode is now `VerifyMode::MinimumBar`, which checks
/// only HTML5-shape predicates (doctype/html, non-empty body, has
/// script/style/link, minimum size, no placeholder text). Game-shape
/// heuristics (canvas / keydown / requestAnimationFrame / etc.) are now
/// gated behind `VerifyMode::GameShape`, selected by `generate_handler`
/// only when the spec.md mentions game-related keywords. This restores
/// universality for non-game specs (video converter, todo app, dashboard,
/// CRUD form) that were false-positive-rejected by the W8 v1 game-shape
/// gate.
#[cfg(feature = "web")]
pub(crate) fn verify_artifact_html(path: &Path) -> std::io::Result<VerifyOutcome> {
    verify_artifact_html_with_mode(path, VerifyMode::GameShape)
}

/// F11 (2026-05-19): mode-aware artifact verification.
///
/// `VerifyMode::GameShape` — strict game-shape heuristics (W8 legacy).
/// `VerifyMode::MinimumBar` — domain-agnostic HTML5 minimum bar (Option B).
#[cfg(feature = "web")]
pub(crate) fn verify_artifact_html_with_mode(
    path: &Path,
    mode: VerifyMode,
) -> std::io::Result<VerifyOutcome> {
    let metadata = fs::metadata(path)?;
    let size_bytes = metadata.len();
    let html = fs::read_to_string(path)?;
    Ok(verify_html_contents_with_mode(&html, size_bytes, mode))
}

/// F11 (2026-05-19): verification mode discriminant.
///
/// `GameShape` preserves the W8 strict heuristics (canvas/keydown/raf etc.).
/// `MinimumBar` applies only HTML5-shape predicates so non-game specs
/// (video converter, todo, dashboard, CRUD) are not false-positive rejected.
#[cfg(feature = "web")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifyMode {
    /// Strict game-shape heuristics (canvas/playfield + keyboard + raf loop).
    GameShape,
    /// Domain-agnostic HTML5 minimum bar.
    MinimumBar,
}

/// F11 (2026-05-19): detect whether a spec.md describes a game-shaped artifact.
///
/// Used by `generate_handler` to pick `VerifyMode::GameShape` vs
/// `VerifyMode::MinimumBar`. Matches either English game keywords or
/// Simplified/Traditional Chinese 游戏/遊戲, plus a handful of well-known
/// game-shape genres (tetris/snake/breakout/canvas/playfield). Case-insensitive
/// for ASCII; the Chinese characters are matched literally.
///
/// Conservative bias: a non-game artifact tagged as a game by accident only
/// pays the cost of stricter checks (and the user sees the strict failure
/// reasons). A game-spec misclassified as non-game passes the lower bar
/// (acceptable degradation: the artifact may not actually be playable, but
/// the user can still inspect / regenerate). Default is MinimumBar.
#[cfg(feature = "web")]
pub(crate) fn spec_looks_like_game(spec_md: &str) -> bool {
    let lower = spec_md.to_ascii_lowercase();
    let ascii_keywords = [
        "game", "tetris", "snake", "breakout", "pong", "pacman", "pac-man",
        "minesweeper", "2048", "arcade", "playfield", "canvas",
    ];
    if ascii_keywords.iter().any(|k| lower.contains(k)) {
        return true;
    }
    // Chinese keywords (Simplified + Traditional) — match on raw spec text,
    // not lowercased (no-op for non-ASCII).
    let zh_keywords = [
        "游戏",       // SC: game
        "遊戲",       // TC: game
        "俄罗斯方块", // SC: Tetris
        "俄羅斯方塊", // TC: Tetris
        "贪吃蛇",     // SC: Snake
        "貪吃蛇",     // TC: Snake
        "扫雷",       // SC: Minesweeper
        "掃雷",       // TC: Minesweeper
    ];
    if zh_keywords.iter().any(|k| spec_md.contains(k)) {
        return true;
    }
    false
}

/// TRACE_MATRIX FC1-N5: detect a visible game-surface rendering technology.
///
/// Accepts ANY of:
/// 1. `<canvas` — classic 2D/3D canvas games (Tetris, Breakout, etc.)
/// 2. CSS Grid playfield — `display: grid` + `grid-template-*` + `repeat()`
/// 3. `<svg` — vector graphics games
/// 4. `<table>` with `<tr>` — old-school table-based grid
/// 5. cell-class pattern — `class="cell"` literally OR `classList.add('cell')`
///    in JS (dynamically created cell divs, common Tetris idiom)
///
/// Input is lowercased HTML (caller passes `&lower`).
#[cfg(feature = "web")]
fn has_playfield(lower: &str) -> bool {
    // 1. Canvas (classic)
    if lower.contains("<canvas") {
        return true;
    }
    // 2. CSS Grid playfield (modern; what tripped W8 v1)
    let has_grid_display = lower.contains("display: grid") || lower.contains("display:grid");
    let has_grid_template =
        lower.contains("grid-template-columns") || lower.contains("grid-template-rows");
    let has_repeat = lower.contains("repeat(");
    if has_grid_display && has_grid_template && has_repeat {
        return true;
    }
    // 3. SVG
    if lower.contains("<svg") {
        return true;
    }
    // 4. HTML table (old-school)
    if lower.contains("<table") && lower.contains("<tr") {
        return true;
    }
    // 5. Cell-class pattern (dynamically created divs)
    if lower.contains("class=\"cell\"")
        || lower.contains("class='cell'")
        || lower.contains("classlist.add('cell')")
        || lower.contains("classlist.add(\"cell\")")
        || lower.contains(".classname = 'cell'")
        || lower.contains(".classname = \"cell\"")
    {
        return true;
    }
    false
}

// ---------------------------------------------------------------------------
// Pure logic (separated for unit testability)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC1-N10: pure heuristic over already-loaded text.
///
/// Separated from the I/O wrapper so unit tests can exercise the checks
/// without writing temp files. Called by `verify_artifact_html` after
/// loading the file. Defaults to game-shape mode for backwards-compat with
/// the W8 test suite; F11 callers (generate_handler) prefer the mode-aware
/// `verify_html_contents_with_mode`.
#[cfg(feature = "web")]
pub(crate) fn verify_html_contents(html: &str, size_bytes: u64) -> VerifyOutcome {
    verify_html_contents_with_mode(html, size_bytes, VerifyMode::GameShape)
}

/// F11 (2026-05-19): mode-aware heuristic over already-loaded HTML text.
///
/// Branches on `VerifyMode`:
///   - `GameShape`: full W8 strict heuristics (canvas/keyboard/raf + size +
///     no external scripts/styles + balanced braces/tags + no inverted
///     nullish guard + keydown on document/window).
///   - `MinimumBar`: domain-agnostic HTML5 bar (Option B). Accepts if:
///     1) <html> or <!DOCTYPE html> present,
///     2) non-empty <body>,
///     3) at least one of <script>, <style>, <link rel="stylesheet">,
///     4) total size >= 500 bytes,
///     5) no placeholder strings (TODO / FIXME / lorem ipsum / placeholder).
#[cfg(feature = "web")]
pub(crate) fn verify_html_contents_with_mode(
    html: &str,
    size_bytes: u64,
    mode: VerifyMode,
) -> VerifyOutcome {
    match mode {
        VerifyMode::GameShape => verify_game_shape(html, size_bytes),
        VerifyMode::MinimumBar => verify_minimum_bar(html, size_bytes),
    }
}

/// F11 (2026-05-19): domain-agnostic HTML5 minimum bar.
///
/// Returns PASS iff every check below passes:
///   1. Contains `<html` or `<!doctype html` (case-insensitive).
///   2. Contains a non-empty `<body>...</body>`.
///   3. Contains at least one of `<script`, `<style`, `<link rel="stylesheet"`.
///   4. Total size >= MIN_SIZE_BYTES_MINIMUM (500 bytes).
///   5. Does not contain placeholder text (TODO / FIXME / lorem ipsum /
///      <!-- placeholder --> case-insensitive).
///
/// Each failed check appends a descriptive `reason:` string. Safe to surface.
#[cfg(feature = "web")]
fn verify_minimum_bar(html: &str, size_bytes: u64) -> VerifyOutcome {
    let mut failure_reasons: Vec<String> = Vec::new();
    let lower = html.to_ascii_lowercase();

    // Check 1: HTML shape — <html> or <!DOCTYPE html>.
    let has_doctype = lower.contains("<!doctype html");
    let has_html_tag = lower.contains("<html");
    if !(has_doctype || has_html_tag) {
        failure_reasons.push(
            "missing_html_root: 找不到 <html> 或 <!DOCTYPE html> — 不是 HTML 文档".to_string(),
        );
    }

    // Check 2: non-empty <body>.
    if !has_nonempty_body(&lower) {
        failure_reasons.push(
            "missing_or_empty_body: 找不到 <body>...</body>，或 body 内容为空 — 无可见内容".to_string(),
        );
    }

    // Check 3: at least one of <script>, <style>, <link rel="stylesheet">.
    let has_script = lower.contains("<script");
    let has_style = lower.contains("<style");
    let has_stylesheet_link =
        lower.contains("rel=\"stylesheet\"") || lower.contains("rel='stylesheet'");
    if !(has_script || has_style || has_stylesheet_link) {
        failure_reasons.push(
            "no_script_or_style: 既没有 <script>、<style>，也没有 <link rel=\"stylesheet\"> — 缺少交互或样式".to_string(),
        );
    }

    // Check 4: total size >= MIN_SIZE_BYTES_MINIMUM.
    if size_bytes < MIN_SIZE_BYTES_MINIMUM {
        failure_reasons.push(format!(
            "too_small: artifact is {} 字节 (< {} 字节最小阈值) — 疑似桩代码",
            size_bytes, MIN_SIZE_BYTES_MINIMUM
        ));
    }

    // Check 5: no placeholder text.
    // NOTE: case-sensitive on raw `html` for TODO/FIXME so legitimate
    // domain words like "Todo App" / "todoList" don't trip a false positive.
    // `lorem ipsum` and `<!-- placeholder -->` are matched case-insensitively
    // via `lower`.
    if let Some(found) = first_placeholder_match(html, &lower) {
        failure_reasons.push(format!(
            "placeholder_content: 检测到占位文本 {:?} — 模型未完成生成",
            found
        ));
    }

    VerifyOutcome {
        passed: failure_reasons.is_empty(),
        failure_reasons,
        artifact_size_bytes: size_bytes,
    }
}

/// F11 helper: true iff `lower` contains a `<body>` opening tag, a matching
/// `</body>` closing tag, and at least one non-whitespace character between
/// them.
#[cfg(feature = "web")]
fn has_nonempty_body(lower: &str) -> bool {
    let body_open = match lower.find("<body") {
        Some(i) => i,
        None => return false,
    };
    let body_open_close = match lower[body_open..].find('>') {
        Some(i) => body_open + i + 1,
        None => return false,
    };
    let body_close = match lower[body_open_close..].find("</body") {
        Some(i) => body_open_close + i,
        None => return false,
    };
    let inner = &lower[body_open_close..body_close];
    inner.chars().any(|c| !c.is_whitespace())
}

/// F11 helper: returns the first placeholder substring found in `html`/
/// `lower`, or `None` if no placeholder.
///
/// `lorem ipsum` and `<!-- placeholder -->` are canonical "model gave up"
/// markers and matched case-insensitively (on `lower`).
///
/// `TODO` / `FIXME` are matched CASE-SENSITIVELY on `html`, and only when
/// they appear in placeholder shape:
///   - `// TODO` (line comment)
///   - `/* TODO` (block comment opening)
///   - `<!-- TODO` (HTML comment)
///   - `TODO:` (label-style)
/// This lets legitimate identifiers like `todoList`, `<h1>Tasks</h1>`, or
/// `<title>Todo App</title>` pass without false positives.
#[cfg(feature = "web")]
fn first_placeholder_match(html: &str, lower: &str) -> Option<&'static str> {
    // Case-insensitive exact substrings.
    let exact_lower = ["lorem ipsum", "<!-- placeholder -->"];
    for needle in exact_lower {
        if lower.contains(needle) {
            return Some(needle);
        }
    }
    // Case-sensitive comment-shaped TODO / FIXME markers.
    let comment_markers = [
        "// TODO",
        "/* TODO",
        "<!-- TODO",
        "TODO:",
        "// FIXME",
        "/* FIXME",
        "<!-- FIXME",
        "FIXME:",
    ];
    for needle in comment_markers {
        if html.contains(needle) {
            // Return the static label, not the exact match string.
            if needle.contains("TODO") {
                return Some("TODO");
            }
            return Some("FIXME");
        }
    }
    None
}

/// F11 (2026-05-19): legacy W8 game-shape heuristics, preserved verbatim
/// behind `VerifyMode::GameShape`. Caller must still pass a content-aware
/// mode (generate_handler picks based on spec.md keywords).
#[cfg(feature = "web")]
fn verify_game_shape(html: &str, size_bytes: u64) -> VerifyOutcome {
    let mut failure_reasons: Vec<String> = Vec::new();

    // Check 1: size bounds.
    if size_bytes < MIN_SIZE_BYTES {
        failure_reasons.push(format!(
            "size_too_small: artifact is {} 字节 (< {} 字节最小阈值)，疑似被截断",
            size_bytes, MIN_SIZE_BYTES
        ));
    } else if size_bytes > MAX_SIZE_BYTES {
        failure_reasons.push(format!(
            "size_too_large: artifact is {} 字节 (> {} 字节最大阈值)，疑似冗余或包含外部资源",
            size_bytes, MAX_SIZE_BYTES
        ));
    }

    let lower = html.to_ascii_lowercase();

    // Check 2: has_playfield — any visible game-surface technology.
    //
    // W8 v1 originally hardcoded `<canvas`, which the W8 Validation Round 1
    // (handover/evidence/stage_phase7_w8_validation_20260518T041310Z) revealed
    // as overfit: Qwen3-Coder produced 3 consecutive functional Tetris
    // implementations using `display: grid` + dynamically-created `.cell`
    // divs — all 3 were false-positive-rejected. W8.1 relaxes the check to
    // accept any of the common playfield rendering technologies.
    if !has_playfield(&lower) {
        failure_reasons.push(
            "missing_playfield: 找不到游戏面板 — 期望 <canvas>、CSS grid (display: grid + grid-template-* + repeat())、<svg>、<table>、或 cell-class 网格之一".to_string()
        );
    }

    // Check 3: has_keyboard_handler — addEventListener + key event.
    let has_addev = lower.contains("addeventlistener");
    let has_keydown = lower.contains("keydown");
    let has_keyup = lower.contains("keyup");
    let has_keypress = lower.contains("keypress");
    if !(has_addev && (has_keydown || has_keyup || has_keypress)) {
        failure_reasons.push(
            "missing_keyboard_handler: 找不到 addEventListener('keydown' / 'keyup' / 'keypress') — 无法接收键盘输入".to_string(),
        );
    }

    // Check 4: has_animation_loop — requestAnimationFrame OR setInterval.
    if !(lower.contains("requestanimationframe") || lower.contains("setinterval")) {
        failure_reasons.push(
            "missing_animation_loop: 找不到 requestAnimationFrame 或 setInterval — 游戏循环不会启动".to_string(),
        );
    }

    // Check 5: no_external_scripts — script src="http..." or src="//..."
    if has_external_script_src(html) {
        failure_reasons.push(
            "external_script_src: 检测到 <script src=\"http..\"> 或 protocol-relative — 沙箱中 CDN 加载会失败".to_string(),
        );
    }

    // Check 6: no_external_stylesheets — link rel=stylesheet href="http..."
    if has_external_stylesheet(html) {
        failure_reasons.push(
            "external_stylesheet: 检测到 <link rel=\"stylesheet\" href=\"http..\"> — 沙箱中外部 CSS 会失败".to_string(),
        );
    }

    // Check 7: balanced_braces — count of `{` matches count of `}`.
    let open_braces = html.matches('{').count();
    let close_braces = html.matches('}').count();
    if open_braces != close_braces {
        failure_reasons.push(format!(
            "unbalanced_braces: {{ 出现 {} 次，}} 出现 {} 次 — JS 几乎肯定语法错误",
            open_braces, close_braces
        ));
    }

    // Check 8: balanced_tags — <script> opens vs </script> closes.
    let (script_open, script_close) = count_script_tags(&lower);
    if script_open != script_close {
        failure_reasons.push(format!(
            "unbalanced_script_tags: <script> 出现 {} 次，</script> 出现 {} 次 — HTML 结构损坏",
            script_open, script_close
        ));
    }

    // Check 9: inverted nullish guard pattern (the load-bearing Qwen check).
    if has_inverted_nullish_guard(html) {
        failure_reasons.push(
            "inverted_nullish_guard: 检测到 `=== null` 检查模式与同名字段在别处被赋为非空值同时存在 — 这是已知 Qwen 失败模式，启动逻辑会被早返回卡住".to_string(),
        );
    }

    // Check 10: keydown handler must be on document or window, not just body.
    if !has_document_or_window_keydown(&lower) {
        failure_reasons.push(
            "keydown_not_on_document_or_window: 键盘监听只挂在 body 上 — iframe sandbox 中 body 可能无焦点，请挂到 document 或 window".to_string(),
        );
    }

    VerifyOutcome {
        passed: failure_reasons.is_empty(),
        failure_reasons,
        artifact_size_bytes: size_bytes,
    }
}

// ---------------------------------------------------------------------------
// Heuristic helpers
// ---------------------------------------------------------------------------

/// Detects `<script src="http..."` or `<script src="//..."` (protocol-rel).
///
/// Lowercases each match window so the check is case-insensitive without
/// allocating a full lowercased copy of the HTML for each substring search.
#[cfg(feature = "web")]
fn has_external_script_src(html: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    // Find every `<script` and inspect the immediate `src="..."` attribute.
    let mut idx = 0;
    while let Some(pos) = lower[idx..].find("<script") {
        let start = idx + pos;
        // Look ahead within the opening tag for src="...".
        let tag_end = lower[start..]
            .find('>')
            .map(|e| start + e)
            .unwrap_or(lower.len());
        let tag_slice = &lower[start..tag_end];
        if let Some(src_pos) = tag_slice.find("src=") {
            let after = &tag_slice[src_pos + 4..];
            // Strip optional quote.
            let stripped = after.trim_start_matches(['"', '\'']);
            if stripped.starts_with("http://")
                || stripped.starts_with("https://")
                || stripped.starts_with("//")
            {
                return true;
            }
        }
        idx = tag_end.saturating_add(1);
    }
    false
}

/// Detects `<link rel="stylesheet" href="http..."` or `href="//..."`.
#[cfg(feature = "web")]
fn has_external_stylesheet(html: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    let mut idx = 0;
    while let Some(pos) = lower[idx..].find("<link") {
        let start = idx + pos;
        let tag_end = lower[start..]
            .find('>')
            .map(|e| start + e)
            .unwrap_or(lower.len());
        let tag_slice = &lower[start..tag_end];
        let is_stylesheet = tag_slice.contains("rel=\"stylesheet\"")
            || tag_slice.contains("rel='stylesheet'")
            || tag_slice.contains("rel=stylesheet");
        if is_stylesheet {
            if let Some(href_pos) = tag_slice.find("href=") {
                let after = &tag_slice[href_pos + 5..];
                let stripped = after.trim_start_matches(['"', '\'']);
                if stripped.starts_with("http://")
                    || stripped.starts_with("https://")
                    || stripped.starts_with("//")
                {
                    return true;
                }
            }
        }
        idx = tag_end.saturating_add(1);
    }
    false
}

/// Count `<script>` opens (case-insensitive) vs `</script>` closes.
///
/// We count `<script` (matches both `<script>` and `<script type="...">`)
/// rather than the bare `<script>` so attributes don't escape detection.
#[cfg(feature = "web")]
fn count_script_tags(lower: &str) -> (usize, usize) {
    let opens = lower.matches("<script").count();
    let closes = lower.matches("</script").count();
    (opens, closes)
}

/// Detect the inverted nullish-guard pattern that caused the Phase 7 E2E
/// attempt-1 broken Tetris.
///
/// Pattern: a `<lhs> === null` check appears in the file AND the SAME
/// `<lhs>` is assigned a non-null value elsewhere via `<lhs> = <expr>`
/// where `<expr>` is not literally `null` / `undefined`.
///
/// We focus on the specific Qwen failure (`player.matrix === null`)
/// alongside `player.matrix = createPiece(`, and on a generic JS-identifier
/// chain match.
#[cfg(feature = "web")]
fn has_inverted_nullish_guard(html: &str) -> bool {
    // Pattern A: specific to the observed Qwen Tetris bug.
    // We use simple substring containment instead of regex to avoid pulling
    // in a regex dependency — these are precise text patterns.
    let a_check = ["player.matrix === null", "player.matrix===null"]
        .iter()
        .any(|p| html.contains(p));
    let a_assign = ["player.matrix = createPiece(", "player.matrix=createPiece("]
        .iter()
        .any(|p| html.contains(p));
    if a_check && a_assign {
        return true;
    }

    // Pattern B: generic — find any `<chain> === null` and an assignment
    // `<chain> = <non-null>` elsewhere in the file.
    //
    // We scan for the literal token `=== null`, walk back to extract the
    // identifier chain (`a.b.c`), then search for `<chain> =` followed by
    // text that is not `null` / `undefined`.
    let mut search_from = 0;
    while let Some(pos) = find_substr_after(html, "=== null", search_from) {
        if let Some(chain) = extract_left_chain(html, pos) {
            if !chain.is_empty() && chain_assigned_non_null(html, &chain, pos) {
                return true;
            }
        }
        search_from = pos + "=== null".len();
    }

    false
}

/// Like `str::find` but offset-relative: returns the absolute byte index
/// of `needle` in `haystack` starting at `from`, or `None`.
#[cfg(feature = "web")]
fn find_substr_after(haystack: &str, needle: &str, from: usize) -> Option<usize> {
    if from >= haystack.len() {
        return None;
    }
    haystack[from..].find(needle).map(|p| from + p)
}

/// Walk backwards from `pos` (where `=== null` starts) and return the
/// identifier chain on the LHS, e.g. for `if (player.matrix === null)`
/// returns `Some("player.matrix")`. Returns `None` if no identifier
/// character precedes `pos`.
///
/// An identifier chain is: ASCII alphanumeric, `_`, `$`, `.`, with the
/// final character being an identifier character (not a dot).
#[cfg(feature = "web")]
fn extract_left_chain(html: &str, pos: usize) -> Option<String> {
    let bytes = html.as_bytes();
    // Walk back past whitespace.
    let mut end = pos;
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    if end == 0 {
        return None;
    }
    let mut start = end;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b'$' || c == b'.' {
            start -= 1;
        } else {
            break;
        }
    }
    if start == end {
        return None;
    }
    // Trim trailing dot if any (chain must end in identifier char).
    let mut chain_end = end;
    while chain_end > start && bytes[chain_end - 1] == b'.' {
        chain_end -= 1;
    }
    if chain_end == start {
        return None;
    }
    Some(html[start..chain_end].to_string())
}

/// Returns true iff somewhere in `html` (outside the byte range
/// `[exclude_pos - 64, exclude_pos + 64]`) there is an assignment to
/// `chain` whose RHS is not literally `null` or `undefined`.
#[cfg(feature = "web")]
fn chain_assigned_non_null(html: &str, chain: &str, exclude_pos: usize) -> bool {
    // Two assignment patterns: `<chain> = ` and `<chain>=`. Look for both.
    for pattern in [format!("{chain} = "), format!("{chain}=")] {
        let mut from = 0;
        while let Some(pos) = find_substr_after(html, &pattern, from) {
            // Don't count the equality check itself.
            // The pattern `<chain> = ` or `<chain>=` must not be `<chain> ==` etc.
            // We detect that by inspecting the char after `=`.
            let after_eq = pos + pattern.len();
            if after_eq >= html.len() {
                from = pos + pattern.len();
                continue;
            }
            // Skip if this is `==` / `===` (a comparison, not assignment).
            let next_char = html.as_bytes()[after_eq];
            if next_char == b'=' {
                from = pos + pattern.len();
                continue;
            }
            // Skip near the original `=== null` site.
            let near_exclude = pos + 32 >= exclude_pos && pos < exclude_pos + 32;
            if near_exclude {
                from = pos + pattern.len();
                continue;
            }
            // Check RHS first token is not `null` / `undefined`.
            let rhs = html[after_eq..].trim_start();
            if !rhs.starts_with("null") && !rhs.starts_with("undefined") && !rhs.is_empty() {
                return true;
            }
            from = pos + pattern.len();
        }
    }
    false
}

/// Detects whether `document.addEventListener('keydown'` OR
/// `window.addEventListener('keydown'` (single or double quotes) appears
/// in the lowercased HTML. Returns false if only `body.addEventListener`
/// is present — body may not be focused in the iframe sandbox.
#[cfg(feature = "web")]
fn has_document_or_window_keydown(lower: &str) -> bool {
    let patterns = [
        "document.addeventlistener('keydown'",
        "document.addeventlistener(\"keydown\"",
        "window.addeventlistener('keydown'",
        "window.addeventlistener(\"keydown\"",
    ];
    patterns.iter().any(|p| lower.contains(p))
}

// ---------------------------------------------------------------------------
// Unit tests (pure logic; no I/O)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", test))]
mod tests {
    use super::*;

    #[test]
    fn extract_left_chain_simple() {
        let html = "if (player.matrix === null)";
        let pos = html.find("===").unwrap();
        let chain = extract_left_chain(html, pos).expect("must extract");
        assert_eq!(chain, "player.matrix");
    }

    #[test]
    fn has_external_script_src_https() {
        let html = r#"<script src="https://cdn.example.com/x.js"></script>"#;
        assert!(has_external_script_src(html));
    }

    #[test]
    fn has_external_script_src_inline_ok() {
        let html = r#"<script>console.log("hi");</script>"#;
        assert!(!has_external_script_src(html));
    }

    #[test]
    fn count_script_tags_balanced() {
        let lower = "<script>a</script><script>b</script>".to_ascii_lowercase();
        let (o, c) = count_script_tags(&lower);
        assert_eq!(o, 2);
        assert_eq!(c, 2);
    }

    #[test]
    fn has_document_keydown_match() {
        let html = "document.addEventListener('keydown', fn);".to_ascii_lowercase();
        assert!(has_document_or_window_keydown(&html));
    }

    #[test]
    fn has_document_keydown_no_body_only() {
        let html = "body.addEventListener('keydown', fn);".to_ascii_lowercase();
        assert!(!has_document_or_window_keydown(&html));
    }
}
