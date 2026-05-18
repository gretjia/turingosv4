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

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC1-N10: heuristic checks against one artifact.
///
/// Returns `VerifyOutcome` with a list of human-readable failure reasons.
/// I/O errors (file not found, permission denied) propagate as `io::Error`.
#[cfg(feature = "web")]
pub(crate) fn verify_artifact_html(path: &Path) -> std::io::Result<VerifyOutcome> {
    let metadata = fs::metadata(path)?;
    let size_bytes = metadata.len();
    let html = fs::read_to_string(path)?;
    Ok(verify_html_contents(&html, size_bytes))
}

// ---------------------------------------------------------------------------
// Pure logic (separated for unit testability)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC1-N10: pure heuristic over already-loaded text.
///
/// Separated from the I/O wrapper so unit tests can exercise the checks
/// without writing temp files. Called by `verify_artifact_html` after
/// loading the file.
#[cfg(feature = "web")]
pub(crate) fn verify_html_contents(html: &str, size_bytes: u64) -> VerifyOutcome {
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

    // Check 2: has_canvas — substring match on `<canvas`.
    if !lower.contains("<canvas") {
        failure_reasons
            .push("missing_canvas: 找不到 <canvas> 元素 — 游戏类应用必须有画布".to_string());
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
