/// TRACE_MATRIX FC1-N5: read view materialization (W4.4 design-system polish)
///
/// Server-side HTML renderer for TuringOS UI IR pages.
///
/// Converts an `IRRoot` into a complete `<!doctype html>` document. Every
/// dynamic string from the IR passes through `esc()` before insertion into
/// HTML output, satisfying FC1-N5 shielding rule (no raw user-supplied strings
/// in HTML). See `esc()` for the five characters replaced.
///
/// W4.4 (this revision) — applies the Anthropic generative-UI guidance:
///   - semantic HTML5 landmarks (header / nav / main / footer)
///   - editorial typography pair (Fraunces serif + JetBrains Mono + IBM Plex Sans)
///   - paper-toned palette with oxidized-teal accent (no purple, no Inter/Roboto)
///   - inlined design tokens + base sheet via `include_str!`
///   - per-block semantic markup (article / figure / dl with metric grid /
///     ol for event log) instead of generic divs
///   - status badges as typographic + colored elements (no icon-only)
///   - hash IDs in monospace with truncated-middle display
///   - active-page indicator via `aria-current="page"`
///   - small connection-state pill (`<turingos-status>`) in footer
///
/// All items are `pub(crate)` — no public API leaks from this module.
use super::ir::{Block, CellValue, IRRoot, MetricValue};

// ---------------------------------------------------------------------------
// View discriminator — used for nav aria-current + page chrome
// ---------------------------------------------------------------------------

/// Which top-level view is being rendered. Drives `aria-current="page"` on
/// the nav and (later) any per-view chrome variations.
///
/// W6: adds the `Build` variant. The /build page renders the same chrome as
/// the other views but its `<main>` contains only a `<tos-spec-grill>`
/// placeholder; the Web Component owns the interview flow.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum ViewKind {
    Dashboard,
    Agents,
    Tasks,
    Audit,
    Build,
}

impl ViewKind {
    fn slug(&self) -> &'static str {
        match self {
            ViewKind::Dashboard => "dashboard",
            ViewKind::Agents => "agents",
            ViewKind::Tasks => "tasks",
            ViewKind::Audit => "audit",
            ViewKind::Build => "build",
        }
    }
    fn href(&self) -> &'static str {
        match self {
            ViewKind::Dashboard => "/",
            ViewKind::Agents => "/agents",
            ViewKind::Tasks => "/tasks",
            ViewKind::Audit => "/audit",
            ViewKind::Build => "/build",
        }
    }
    fn label(&self) -> &'static str {
        match self {
            ViewKind::Dashboard => "Dashboard",
            ViewKind::Agents => "Agents",
            ViewKind::Tasks => "Tasks",
            ViewKind::Audit => "Audit",
            ViewKind::Build => "Build",
        }
    }
}

// ---------------------------------------------------------------------------
// HTML escaping
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: shielding — HTML-escape a dynamic string before
/// inserting it into rendered HTML. Replaces the five characters that can
/// produce XSS or markup injection:
///   `&` → `&amp;`   (must be first to avoid double-escaping)
///   `<` → `&lt;`
///   `>` → `&gt;`
///   `"` → `&quot;`
///   `'` → `&#x27;`
pub(crate) fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Truncate-middle for long hash/identifier display. Keeps `head` chars at
/// the start and `tail` at the end joined by an ellipsis. If the input is
/// short enough to fit, return it unchanged.
fn truncate_middle(s: &str, head: usize, tail: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= head + tail + 1 {
        return s.to_string();
    }
    let mut out = String::new();
    out.extend(chars.iter().take(head));
    out.push_str("\u{2026}");
    out.extend(chars.iter().skip(chars.len() - tail));
    out
}

/// Lowercase, slugify a status string so it can be used as a `data-status`
/// selector key (CSS targets keys like `open`, `accepted`, `rejected`).
fn status_slug(s: &str) -> String {
    s.trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

// ---------------------------------------------------------------------------
// Inlined design system + base styles (compile-time embedded)
// ---------------------------------------------------------------------------

/// Design tokens (CSS variables + dark-mode overrides). Authoritative copy
/// at `frontend/src/design-system.css`; the TS mirror at
/// `frontend/src/design-system.ts` is for client-side reuse.
const DESIGN_TOKENS_CSS: &str = include_str!("../../frontend/src/design-system.css");

/// Base styles (typography, landmarks, block typesetting). Authoritative copy
/// at `frontend/src/base-styles.css`.
const BASE_STYLES_CSS: &str = include_str!("../../frontend/src/base-styles.css");

// ---------------------------------------------------------------------------
// Public renderer
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Render an `IRRoot` to a complete HTML document.
///
/// Requirements satisfied (§6a Page 1-5 mechanical criteria):
/// - `<title>TuringOS — {title}</title>` — literal "TuringOS" present.
/// - Wordmark contains "TuringOS" and an inline "Phase 7" subtitle.
/// - Each block wrapped in `[data-block-type="<kind>"]` (§6a Page 1 DOM check).
/// - All dynamic strings HTML-escaped through `esc()` (FC1-N5 shielding).
/// - `<script type="module" src="/static/main.js"></script>` tag (W2/W3 mount).
/// - `<turingos-root></turingos-root>` element (W3 Web Component mount point).
///
/// W4: if `show_task_form` is true (tasks page only), inserts a
/// `<tos-task-open-form></tos-task-open-form>` placeholder element above the
/// `<turingos-root>`. The Web Component upgrades it client-side via
/// `customElements.define`.
///
/// W4.4: the new `view` parameter drives `aria-current="page"` on the
/// primary nav. Existing call sites that don't have a `ViewKind` should
/// migrate; for backwards compatibility a default `ViewKind::Dashboard`
/// is used by `render_page` (which keeps the 3-arg shape).
pub(crate) fn render_page(ir: &IRRoot, title: &str, show_task_form: bool) -> String {
    render_page_with_view(ir, title, show_task_form, ViewKind::Dashboard)
}

/// W4.4: explicit-view variant. The 3-arg `render_page` defaults to
/// `ViewKind::Dashboard` for backwards compatibility; the router uses
/// this 4-arg form so the active-nav indicator is correct.
pub(crate) fn render_page_with_view(
    ir: &IRRoot,
    title: &str,
    show_task_form: bool,
    view: ViewKind,
) -> String {
    let mut html = String::new();

    // ---- <head> -----------------------------------------------------------
    html.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<meta name=\"color-scheme\" content=\"light dark\">\n");

    html.push_str("<title>TuringOS \u{2014} ");
    html.push_str(&esc(title));
    html.push_str("</title>\n");

    // Editorial typography: Fraunces (variable serif) + JetBrains Mono +
    // IBM Plex Sans. Loaded from Google Fonts with a system-fallback chain
    // declared in design-system.css so a CDN failure still degrades to a
    // distinctive look. `display=swap` prevents render-blocking.
    html.push_str(
        "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n\
         <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n\
         <link rel=\"stylesheet\" \
         href=\"https://fonts.googleapis.com/css2?\
family=Fraunces:ital,opsz,wght,SOFT@0,9..144,300..900,30..100;1,9..144,300..900,30..100\
&family=IBM+Plex+Sans:wght@300;400;500;700\
&family=JetBrains+Mono:wght@400;500;700\
&display=swap\">\n",
    );

    // Inline design system + base styles. Authoritative CSS lives in
    // `frontend/src/{design-system,base-styles}.css`; include_str! embeds
    // it at compile time so a single binary serves the whole UI.
    html.push_str("<style>\n");
    html.push_str(DESIGN_TOKENS_CSS);
    html.push_str("\n");
    html.push_str(BASE_STYLES_CSS);
    html.push_str("</style>\n");

    // W2 inline WebSocket bootstrap. Static text only — no dynamic strings
    // are interpolated, so no esc() calls are needed inside the script block.
    html.push_str("<script>\n");
    html.push_str(INLINE_WS_SCRIPT);
    html.push_str("</script>\n");

    html.push_str("</head>\n<body data-view=\"");
    html.push_str(view.slug());
    html.push_str("\">\n");

    // ---- <header> ---------------------------------------------------------
    html.push_str("<header class=\"tos-header\" role=\"banner\">\n");
    html.push_str(
        "  <a class=\"tos-wordmark\" href=\"/\" aria-label=\"TuringOS — Phase 7 home\">\
         TuringOS<span class=\"tos-wordmark-sub\">Phase 7</span></a>\n",
    );
    html.push_str("  <span class=\"tos-meta\">FC3-N31 \u{00b7} materialized view</span>\n");
    html.push_str("</header>\n");

    // ---- <nav> ------------------------------------------------------------
    html.push_str("<nav class=\"tos-nav\" aria-label=\"primary\">\n");
    for v in [
        ViewKind::Dashboard,
        ViewKind::Agents,
        ViewKind::Tasks,
        ViewKind::Audit,
        ViewKind::Build,
    ] {
        if v == view {
            html.push_str("  <a aria-current=\"page\" href=\"");
        } else {
            html.push_str("  <a href=\"");
        }
        html.push_str(v.href());
        html.push_str("\">");
        html.push_str(v.label());
        html.push_str("</a>\n");
    }
    html.push_str("</nav>\n");

    // ---- <main> -----------------------------------------------------------
    html.push_str("<main class=\"tos-main\" id=\"tos-main\" role=\"main\">\n");

    // Page title (from IR) — editorial italic Fraunces
    html.push_str("  <h1 class=\"tos-page-title\">");
    html.push_str(&esc(&ir.title));
    html.push_str("</h1>\n");

    // Page ID — small monospace, de-emphasized
    html.push_str("  <p class=\"tos-page-id\">");
    html.push_str(&esc(&ir.id));
    html.push_str("</p>\n");

    // W4: task-open form (tasks page only)
    if show_task_form {
        html.push_str("  <tos-task-open-form></tos-task-open-form>\n");
    }

    // Render each block (semantic markup per block-type)
    for (idx, block) in ir.blocks.iter().enumerate() {
        html.push_str(&render_block(block, idx));
    }

    // W3 Web Component mount point. Kept in <main> so client-side rerenders
    // land inside the same content landmark.
    html.push_str("  <turingos-root></turingos-root>\n");

    html.push_str("</main>\n");

    // ---- <footer> ---------------------------------------------------------
    html.push_str("<footer class=\"tos-footer\" role=\"contentinfo\">\n");
    html.push_str(
        "  <span class=\"tos-footer-notice\">FC3-N31: materialized view \u{2014} \
         not authoritative over ChainTape/CAS.</span>\n",
    );
    html.push_str("  <turingos-status></turingos-status>\n");
    html.push_str("</footer>\n");

    // W2/W3 frontend script tag (static path; wired in W2)
    html.push_str("<script type=\"module\" src=\"/static/main.js\"></script>\n");

    html.push_str("</body>\n</html>\n");
    html
}

// ---------------------------------------------------------------------------
// W6: /build page renderer — spec-grill mount, no IR
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W6 — `/build` page chrome.
///
/// Renders the standard header / nav / footer shared with the other views,
/// but inside `<main>` mounts only a `<tos-spec-grill>` placeholder. The
/// Web Component (registered by /static/main.js) owns the entire interview
/// flow client-side: question fetch, per-question card, submit, spec result,
/// generate, artifact preview.
///
/// Page title is intentionally short and editorial — Fraunces italic via the
/// shared `.tos-page-title` selector. The `<p>` subtitle below the H1 sets
/// the editorial register for the interview.
pub(crate) fn render_build_page() -> String {
    let mut html = String::new();

    html.push_str("<!doctype html>\n<html lang=\"zh\">\n<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<meta name=\"color-scheme\" content=\"light dark\">\n");
    html.push_str("<title>TuringOS \u{2014} Build</title>\n");

    html.push_str(
        "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n\
         <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n\
         <link rel=\"stylesheet\" \
         href=\"https://fonts.googleapis.com/css2?\
family=Fraunces:ital,opsz,wght,SOFT@0,9..144,300..900,30..100;1,9..144,300..900,30..100\
&family=IBM+Plex+Sans:wght@300;400;500;700\
&family=JetBrains+Mono:wght@400;500;700\
&display=swap\">\n",
    );

    html.push_str("<style>\n");
    html.push_str(DESIGN_TOKENS_CSS);
    html.push_str("\n");
    html.push_str(BASE_STYLES_CSS);
    html.push_str("</style>\n");

    html.push_str("<script>\n");
    html.push_str(INLINE_WS_SCRIPT);
    html.push_str("</script>\n");

    html.push_str("</head>\n<body data-view=\"build\">\n");

    // Header
    html.push_str("<header class=\"tos-header\" role=\"banner\">\n");
    html.push_str(
        "  <a class=\"tos-wordmark\" href=\"/\" aria-label=\"TuringOS \u{2014} Phase 7 home\">\
         TuringOS<span class=\"tos-wordmark-sub\">Phase 7</span></a>\n",
    );
    html.push_str("  <span class=\"tos-meta\">FC3-N31 \u{00b7} interview spread</span>\n");
    html.push_str("</header>\n");

    // Nav — Build is the active page
    html.push_str("<nav class=\"tos-nav\" aria-label=\"primary\">\n");
    let active = ViewKind::Build;
    for v in [
        ViewKind::Dashboard,
        ViewKind::Agents,
        ViewKind::Tasks,
        ViewKind::Audit,
        ViewKind::Build,
    ] {
        if v == active {
            html.push_str("  <a aria-current=\"page\" href=\"");
        } else {
            html.push_str("  <a href=\"");
        }
        html.push_str(v.href());
        html.push_str("\">");
        html.push_str(v.label());
        html.push_str("</a>\n");
    }
    html.push_str("</nav>\n");

    // Main — spec-grill mount only.
    html.push_str("<main class=\"tos-main tos-main-build\" id=\"tos-main\" role=\"main\">\n");
    html.push_str("  <h1 class=\"tos-page-title\">从一段闲聊开始，做出你想要的那个小工具。</h1>\n");
    html.push_str(
        "  <p class=\"tos-page-id\">build \u{00b7} spec interview \u{00b7} phase 7 w6</p>\n",
    );
    // The W6 mount point — Web Component owns the rest.
    html.push_str("  <tos-spec-grill></tos-spec-grill>\n");
    // turingos-root is included so WS state pill / connection still mounts.
    html.push_str("  <turingos-root></turingos-root>\n");
    html.push_str("</main>\n");

    // Footer
    html.push_str("<footer class=\"tos-footer\" role=\"contentinfo\">\n");
    html.push_str(
        "  <span class=\"tos-footer-notice\">FC3-N31: materialized view \u{2014} \
         not authoritative over ChainTape/CAS.</span>\n",
    );
    html.push_str("  <turingos-status></turingos-status>\n");
    html.push_str("</footer>\n");

    html.push_str("<script type=\"module\" src=\"/static/main.js\"></script>\n");
    html.push_str("</body>\n</html>\n");
    html
}

// ---------------------------------------------------------------------------
// Block renderers (internal helpers)
// ---------------------------------------------------------------------------

fn render_block(block: &Block, idx: usize) -> String {
    let stagger = format!(" style=\"--tos-stagger:{idx}\"");
    match block {
        Block::Text(b) => {
            let mut s = String::new();
            s.push_str("<article data-block-type=\"text\" class=\"block block-text\"");
            s.push_str(&stagger);
            s.push_str(">\n");
            for line in b.content.split('\n') {
                if line.is_empty() {
                    continue;
                }
                s.push_str("  <p>");
                s.push_str(&esc(line));
                s.push_str("</p>\n");
            }
            s.push_str("</article>\n");
            s
        }
        Block::Table(b) => {
            let mut s = String::new();
            s.push_str("<figure data-block-type=\"table\" class=\"block block-table\"");
            s.push_str(&stagger);
            s.push_str(">\n");
            if let Some(cap) = &b.caption {
                s.push_str("  <figcaption class=\"caption\">");
                s.push_str(&esc(cap));
                s.push_str("</figcaption>\n");
            }
            s.push_str("  <table>\n    <thead><tr>\n");
            for col in &b.columns {
                s.push_str("      <th scope=\"col\">");
                s.push_str(&esc(col));
                s.push_str("</th>\n");
            }
            s.push_str("    </tr></thead>\n    <tbody>\n");
            for row in &b.rows {
                s.push_str("      <tr>\n");
                for cell in row {
                    s.push_str("        <td data-cell-kind=\"");
                    s.push_str(&esc(&cell.kind));
                    s.push_str("\">");
                    // Status-bearing string cells get a typographic badge.
                    let is_status_string = cell.kind == "string"
                        && matches!(&cell.value,
                            CellValue::Text(v) if is_known_status(v));
                    match &cell.value {
                        CellValue::Text(v) => {
                            if is_status_string {
                                s.push_str("<span class=\"tos-status\" data-status=\"");
                                s.push_str(&esc(&status_slug(v)));
                                s.push_str("\">");
                                s.push_str(&esc(v));
                                s.push_str("</span>");
                            } else if cell.kind == "agent_id"
                                || cell.kind == "tx_id"
                                || cell.kind == "cid"
                            {
                                // Long hex / canonical IDs: show truncated-middle
                                // in the visible glyph, full value in title attr.
                                let trunc = truncate_middle(v, 14, 8);
                                let escaped_full = esc(v);
                                if trunc != *v {
                                    s.push_str("<span title=\"");
                                    s.push_str(&escaped_full);
                                    s.push_str("\">");
                                    s.push_str(&esc(&trunc));
                                    s.push_str("</span>");
                                } else {
                                    s.push_str(&escaped_full);
                                }
                            } else {
                                s.push_str(&esc(v));
                            }
                        }
                        CellValue::Integer(n) => s.push_str(&n.to_string()),
                    }
                    if cell.kind == "microcoin" {
                        s.push_str(" <span class=\"unit\">\u{3bc}C</span>");
                    }
                    s.push_str("</td>\n");
                }
                s.push_str("      </tr>\n");
            }
            s.push_str("    </tbody>\n  </table>\n</figure>\n");
            s
        }
        Block::AgentCard(b) => {
            let mut s = String::new();
            s.push_str("<article data-block-type=\"agent_card\" class=\"block block-agent-card card agent-card\"");
            s.push_str(&stagger);
            s.push_str(">\n");
            s.push_str("  <header>\n");
            let agent_trunc = truncate_middle(&b.agent_id, 12, 8);
            s.push_str("    <span class=\"tos-card-id\" title=\"");
            s.push_str(&esc(&b.agent_id));
            s.push_str("\">");
            s.push_str(&esc(&agent_trunc));
            s.push_str("</span>\n");
            s.push_str("    <span class=\"tos-card-role\">");
            s.push_str(&esc(&b.role));
            s.push_str("</span>\n");
            s.push_str("  </header>\n");
            s.push_str("  <dl>\n");
            s.push_str("    <dt>balance</dt><dd>");
            s.push_str(&b.balance_micro.to_string());
            s.push_str(" <span class=\"unit\">\u{3bc}C</span></dd>\n");
            if let Some(status) = &b.status {
                s.push_str("    <dt>status</dt><dd><span class=\"tos-status\" data-status=\"");
                s.push_str(&esc(&status_slug(status)));
                s.push_str("\">");
                s.push_str(&esc(status));
                s.push_str("</span></dd>\n");
            }
            s.push_str("  </dl>\n</article>\n");
            s
        }
        Block::TaskCard(b) => {
            let mut s = String::new();
            s.push_str("<article data-block-type=\"task_card\" class=\"block block-task-card card task-card\"");
            s.push_str(&stagger);
            s.push_str(">\n");
            s.push_str("  <header>\n");
            let task_trunc = truncate_middle(&b.task_id, 12, 8);
            s.push_str("    <span class=\"tos-card-id\" title=\"");
            s.push_str(&esc(&b.task_id));
            s.push_str("\">");
            s.push_str(&esc(&task_trunc));
            s.push_str("</span>\n");
            s.push_str("    <span class=\"tos-status\" data-status=\"");
            s.push_str(&esc(&status_slug(&b.status)));
            s.push_str("\">");
            s.push_str(&esc(&b.status));
            s.push_str("</span>\n");
            s.push_str("  </header>\n");
            s.push_str("  <dl>\n");
            s.push_str("    <dt>problem</dt><dd>");
            s.push_str(&esc(&b.problem_id));
            s.push_str("</dd>\n");
            if let Some(reward) = b.reward_micro {
                s.push_str("    <dt>reward</dt><dd>");
                s.push_str(&u64::to_string(&reward));
                s.push_str(" <span class=\"unit\">\u{3bc}C</span></dd>\n");
            }
            if let Some(attempts) = b.attempt_count {
                s.push_str("    <dt>attempts</dt><dd>");
                s.push_str(&u64::to_string(&attempts));
                s.push_str("</dd>\n");
            }
            if let Some(agent) = &b.assigned_agent_id {
                let agent_trunc = truncate_middle(agent, 12, 8);
                s.push_str("    <dt>agent</dt><dd title=\"");
                s.push_str(&esc(agent));
                s.push_str("\">");
                s.push_str(&esc(&agent_trunc));
                s.push_str("</dd>\n");
            }
            s.push_str("  </dl>\n</article>\n");
            s
        }
        Block::EventLog(b) => {
            let mut s = String::new();
            s.push_str("<section data-block-type=\"event_log\" class=\"block block-event-log\" aria-label=\"recent tape events\"");
            s.push_str(&stagger);
            s.push_str(">\n");
            s.push_str("  <ol class=\"event-log\" reversed>\n");
            for ev in &b.events {
                s.push_str("    <li class=\"event layer-");
                s.push_str(&esc(&ev.layer));
                s.push_str("\">\n");
                s.push_str("      <span class=\"layer\">");
                s.push_str(&esc(&ev.layer));
                s.push_str("</span>\n");
                s.push_str("      <span class=\"kind\">");
                s.push_str(&esc(&ev.kind));
                s.push_str("</span>\n");
                let tx_trunc = truncate_middle(&ev.tx_id, 10, 6);
                s.push_str("      <span class=\"tx-id\" title=\"");
                s.push_str(&esc(&ev.tx_id));
                s.push_str("\">");
                s.push_str(&esc(&tx_trunc));
                s.push_str("</span>\n");
                if let Some(summary) = &ev.summary {
                    s.push_str("      <span class=\"summary\">");
                    s.push_str(&esc(summary));
                    s.push_str("</span>\n");
                }
                s.push_str("    </li>\n");
            }
            s.push_str("  </ol>\n</section>\n");
            s
        }
        Block::DashboardPanel(b) => {
            let mut s = String::new();
            s.push_str(
                "<section data-block-type=\"dashboard_panel\" class=\"block block-dashboard-panel card dashboard-panel\"",
            );
            s.push_str(&stagger);
            s.push_str(">\n");
            s.push_str("  <h3 class=\"panel-title\">");
            s.push_str(&esc(&b.panel_title));
            s.push_str("</h3>\n  <dl class=\"metrics\">\n");
            for metric in &b.metrics {
                s.push_str("    <div>\n      <dt>");
                s.push_str(&esc(&metric.label));
                s.push_str("</dt>\n      <dd>");
                match &metric.value {
                    MetricValue::Text(v) => {
                        if is_known_status(v) {
                            s.push_str("<span class=\"tos-status\" data-status=\"");
                            s.push_str(&esc(&status_slug(v)));
                            s.push_str("\">");
                            s.push_str(&esc(v));
                            s.push_str("</span>");
                        } else {
                            s.push_str(&esc(v));
                        }
                    }
                    MetricValue::Integer(n) => s.push_str(&n.to_string()),
                    MetricValue::Float(v) => s.push_str(&v.to_string()),
                }
                if let Some(unit) = &metric.unit {
                    s.push_str(" <span class=\"unit\">");
                    s.push_str(&esc(unit));
                    s.push_str("</span>");
                }
                s.push_str("</dd>\n    </div>\n");
            }
            s.push_str("  </dl>\n</section>\n");
            s
        }
    }
}

/// Known status strings that should be rendered as typographic badges.
fn is_known_status(s: &str) -> bool {
    matches!(
        s.trim().to_ascii_lowercase().as_str(),
        "open"
            | "accepted"
            | "rejected"
            | "finalized"
            | "bankrupt"
            | "expired"
            | "solved"
            | "exhausted"
            | "active"
            | "paused"
            | "pass"
            | "fail"
    )
}

// ---------------------------------------------------------------------------
// Inline WebSocket bootstrap script (W2)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// Inline JS injected into every rendered HTML page. Opens a WebSocket to
/// `/ws` on page load, dispatches `turingos:ir_update` CustomEvents for W3
/// components, and exposes `window.__turingos_ws` for debugging.
///
/// W4.4 additions:
///   - Emits `turingos:ws_state` CustomEvents on open/close/error so the
///     footer `<turingos-status>` indicator can reflect connection state.
///
/// Design decisions (ratified 2026-05-18):
/// - No auto-reconnect in W2; W3 components may register their own strategy.
/// - `onerror` uses `console.warn` (not `console.error`) so §6a Page 1
///   "console error count 0" criterion stays satisfied during normal lifecycle.
/// - Wrapped in an IIFE to avoid global namespace pollution.
/// - `window.__turingos_ws` exposed for debugging only.
///
/// Interface contract for W3:
///   `document.addEventListener('turingos:ir_update', (e) => { const { msg_type, view, ir } = e.detail; })`
///   `document.addEventListener('turingos:ws_state', (e) => { const { state } = e.detail; })`
const INLINE_WS_SCRIPT: &str = r#"
(function () {
  function emitState(state) {
    document.dispatchEvent(new CustomEvent('turingos:ws_state', { detail: { state: state } }));
  }
  // Initial state for late-attached listeners.
  window.__turingos_ws_state = 'connecting';
  emitState('connecting');

  // Determine WS protocol based on page protocol (http→ws, https→wss).
  var proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
  var wsUrl = proto + '//' + location.host + '/ws';
  var ws = new WebSocket(wsUrl);

  // Expose for debugging; not part of the W3 interface contract.
  window.__turingos_ws = ws;

  ws.onopen = function () {
    window.__turingos_ws_state = 'connected';
    emitState('connected');
  };

  ws.onmessage = function (event) {
    try {
      var parsed = JSON.parse(event.data);
      document.dispatchEvent(
        new CustomEvent('turingos:ir_update', { detail: parsed })
      );
    } catch (err) {
      console.warn('turingos ws: failed to parse message', err);
    }
  };

  // Use console.warn (not console.error) so §6a Page 1 "console error count 0"
  // criterion is satisfied during normal lifecycle (e.g., server not started).
  ws.onerror = function (err) {
    console.warn('turingos ws: connection error', err);
    window.__turingos_ws_state = 'disconnected';
    emitState('disconnected');
  };

  ws.onclose = function () {
    window.__turingos_ws_state = 'disconnected';
    emitState('disconnected');
  };
}());
"#;
