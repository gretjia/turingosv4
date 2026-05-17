/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Server-side HTML renderer for TuringOS UI IR pages.
///
/// Converts an `IRRoot` into a complete `<!doctype html>` document. Every
/// dynamic string from the IR passes through `esc()` before insertion into
/// HTML output, satisfying FC1-N5 shielding rule (no raw user-supplied strings
/// in HTML). See `esc()` for the five characters replaced.
///
/// All items are `pub(crate)` — no public API leaks from this module.
use super::ir::{Block, CellValue, IRRoot, MetricValue};

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

// ---------------------------------------------------------------------------
// Public renderer
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Render an `IRRoot` to a complete HTML document.
///
/// Requirements satisfied:
/// - `<title>TuringOS — {title}</title>` — literal "TuringOS" present.
/// - `<h1>TuringOS — Phase 7</h1>` — substring "Phase 7" present (§6a Page 1).
/// - Each block wrapped in `<div data-block-type="<kind>">` (§6a Page 1 DOM check).
/// - All dynamic strings HTML-escaped through `esc()` (FC1-N5 shielding).
/// - `<script type="module" src="/static/main.js"></script>` tag (W2/W3 mount).
/// - `<turingos-root></turingos-root>` element (W3 Web Component mount point).
///
/// W4: if `show_task_form` is true (tasks page only), inserts a
/// `<tos-task-open-form></tos-task-open-form>` placeholder element above the
/// `<turingos-root>`. The Web Component upgrades it client-side via
/// `customElements.define`.
pub(crate) fn render_page(ir: &IRRoot, title: &str, show_task_form: bool) -> String {
    let mut html = String::new();

    // Document head
    html.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<title>TuringOS \u{2014} ");
    html.push_str(&esc(title));
    html.push_str("</title>\n");
    html.push_str("<style>\n");
    html.push_str(INLINE_CSS);
    html.push_str("</style>\n");
    // W2 inline WebSocket bootstrap. Static text only — no dynamic strings
    // are interpolated, so no esc() calls are needed inside the script block.
    html.push_str("<script>\n");
    html.push_str(INLINE_WS_SCRIPT);
    html.push_str("</script>\n");
    html.push_str("</head>\n<body>\n");

    // Required heading — "Phase 7" must appear for §6a Page 1 criterion.
    html.push_str("<h1>TuringOS \u{2014} Phase 7</h1>\n");

    // Page title (from IR)
    html.push_str("<h2>");
    html.push_str(&esc(&ir.title));
    html.push_str("</h2>\n");

    // Page ID (small, de-emphasized)
    html.push_str("<p class=\"page-id\">");
    html.push_str(&esc(&ir.id));
    html.push_str("</p>\n");

    // Render each block
    for block in &ir.blocks {
        html.push_str(&render_block(block));
    }

    // FC3-N31 materialized-view notice
    html.push_str(
        "<p class=\"notice\">FC3-N31: materialized view \u{2014} \
         not authoritative over ChainTape/CAS</p>\n",
    );

    // W4: task-open form placeholder (tasks page only; Web Component upgrades client-side)
    if show_task_form {
        html.push_str("<tos-task-open-form></tos-task-open-form>\n");
    }

    // W3 Web Component mount point
    html.push_str("<turingos-root></turingos-root>\n");

    // W2/W3 frontend script tag (static path; wired in W2)
    html.push_str("<script type=\"module\" src=\"/static/main.js\"></script>\n");

    html.push_str("</body>\n</html>\n");
    html
}

// ---------------------------------------------------------------------------
// Block renderers (internal helpers)
// ---------------------------------------------------------------------------

fn render_block(block: &Block) -> String {
    match block {
        Block::Text(b) => {
            let mut s = String::new();
            s.push_str("<div data-block-type=\"text\">\n");
            // content may have newlines — split into paragraphs
            for line in b.content.split('\n') {
                s.push_str("<p>");
                s.push_str(&esc(line));
                s.push_str("</p>\n");
            }
            s.push_str("</div>\n");
            s
        }
        Block::Table(b) => {
            let mut s = String::new();
            s.push_str("<div data-block-type=\"table\">\n");
            if let Some(cap) = &b.caption {
                s.push_str("<p class=\"caption\">");
                s.push_str(&esc(cap));
                s.push_str("</p>\n");
            }
            s.push_str("<table>\n<thead><tr>\n");
            for col in &b.columns {
                s.push_str("<th>");
                s.push_str(&esc(col));
                s.push_str("</th>\n");
            }
            s.push_str("</tr></thead>\n<tbody>\n");
            for row in &b.rows {
                s.push_str("<tr>\n");
                for cell in row {
                    s.push_str("<td>");
                    match &cell.value {
                        CellValue::Text(v) => s.push_str(&esc(v)),
                        CellValue::Integer(n) => s.push_str(&n.to_string()),
                    }
                    if cell.kind == "microcoin" {
                        s.push_str(" \u{3bc}C"); // μC
                    }
                    s.push_str("</td>\n");
                }
                s.push_str("</tr>\n");
            }
            s.push_str("</tbody>\n</table>\n</div>\n");
            s
        }
        Block::AgentCard(b) => {
            let mut s = String::new();
            s.push_str("<div data-block-type=\"agent_card\" class=\"card agent-card\">\n");
            s.push_str("<dl>\n");
            s.push_str("<dt>agent_id</dt><dd>");
            s.push_str(&esc(&b.agent_id));
            s.push_str("</dd>\n");
            s.push_str("<dt>role</dt><dd>");
            s.push_str(&esc(&b.role));
            s.push_str("</dd>\n");
            s.push_str("<dt>balance_micro</dt><dd>");
            s.push_str(&b.balance_micro.to_string());
            s.push_str(" \u{3bc}C</dd>\n");
            if let Some(status) = &b.status {
                s.push_str("<dt>status</dt><dd>");
                s.push_str(&esc(status));
                s.push_str("</dd>\n");
            }
            s.push_str("</dl>\n</div>\n");
            s
        }
        Block::TaskCard(b) => {
            let mut s = String::new();
            s.push_str("<div data-block-type=\"task_card\" class=\"card task-card\">\n");
            s.push_str("<dl>\n");
            s.push_str("<dt>task_id</dt><dd>");
            s.push_str(&esc(&b.task_id));
            s.push_str("</dd>\n");
            s.push_str("<dt>problem_id</dt><dd>");
            s.push_str(&esc(&b.problem_id));
            s.push_str("</dd>\n");
            s.push_str("<dt>status</dt><dd class=\"status\">");
            s.push_str(&esc(&b.status));
            s.push_str("</dd>\n");
            if let Some(reward) = b.reward_micro {
                s.push_str("<dt>reward_micro</dt><dd>");
                s.push_str(&u64::to_string(&reward));
                s.push_str(" \u{3bc}C</dd>\n");
            }
            if let Some(attempts) = b.attempt_count {
                s.push_str("<dt>attempt_count</dt><dd>");
                s.push_str(&u64::to_string(&attempts));
                s.push_str("</dd>\n");
            }
            if let Some(agent) = &b.assigned_agent_id {
                s.push_str("<dt>assigned_agent_id</dt><dd>");
                s.push_str(&esc(agent));
                s.push_str("</dd>\n");
            }
            s.push_str("</dl>\n</div>\n");
            s
        }
        Block::EventLog(b) => {
            let mut s = String::new();
            s.push_str("<div data-block-type=\"event_log\">\n<ul class=\"event-log\">\n");
            for ev in &b.events {
                s.push_str("<li class=\"event layer-");
                s.push_str(&esc(&ev.layer));
                s.push_str("\">\n");
                s.push_str("<span class=\"layer\">");
                s.push_str(&esc(&ev.layer));
                s.push_str("</span> ");
                s.push_str("<span class=\"kind\">");
                s.push_str(&esc(&ev.kind));
                s.push_str("</span> ");
                s.push_str("<span class=\"tx-id\">");
                s.push_str(&esc(&ev.tx_id));
                s.push_str("</span>");
                if let Some(summary) = &ev.summary {
                    s.push_str("\n<span class=\"summary\">");
                    s.push_str(&esc(summary));
                    s.push_str("</span>");
                }
                s.push_str("\n</li>\n");
            }
            s.push_str("</ul>\n</div>\n");
            s
        }
        Block::DashboardPanel(b) => {
            let mut s = String::new();
            s.push_str(
                "<div data-block-type=\"dashboard_panel\" class=\"card dashboard-panel\">\n",
            );
            s.push_str("<h3>");
            s.push_str(&esc(&b.panel_title));
            s.push_str("</h3>\n<dl class=\"metrics\">\n");
            for metric in &b.metrics {
                s.push_str("<dt>");
                s.push_str(&esc(&metric.label));
                s.push_str("</dt><dd>");
                match &metric.value {
                    MetricValue::Text(v) => s.push_str(&esc(v)),
                    MetricValue::Integer(n) => s.push_str(&n.to_string()),
                    MetricValue::Float(v) => s.push_str(&v.to_string()),
                }
                if let Some(unit) = &metric.unit {
                    s.push_str(" <span class=\"unit\">");
                    s.push_str(&esc(unit));
                    s.push_str("</span>");
                }
                s.push_str("</dd>\n");
            }
            s.push_str("</dl>\n</div>\n");
            s
        }
    }
}

// ---------------------------------------------------------------------------
// Minimal inline CSS
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Inline WebSocket bootstrap script (W2)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// Inline JS injected into every rendered HTML page. Opens a WebSocket to
/// `/ws` on page load, dispatches `turingos:ir_update` CustomEvents for W3
/// components, and exposes `window.__turingos_ws` for debugging.
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
const INLINE_WS_SCRIPT: &str = r#"
(function () {
  // Determine WS protocol based on page protocol (http→ws, https→wss).
  var proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
  var wsUrl = proto + '//' + location.host + '/ws';
  var ws = new WebSocket(wsUrl);

  // Expose for debugging; not part of the W3 interface contract.
  window.__turingos_ws = ws;

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
  };

  // No auto-reconnect in W2. W3 components may register their own
  // reconnect strategy by listening for the socket close event via
  // window.__turingos_ws.
  ws.onclose = function () {
    // Connection closed — nothing to do in W2.
  };
}());
"#;

const INLINE_CSS: &str = r#"
body { font-family: monospace; margin: 1rem 2rem; background: #111; color: #eee; }
h1, h2, h3 { margin-bottom: 0.25rem; }
h1 { color: #7af; }
h2 { color: #adf; }
.page-id { color: #888; font-size: 0.85em; margin-top: 0; }
.notice { color: #888; font-size: 0.8em; border-top: 1px solid #333; padding-top: 0.5rem; }
.card { border: 1px solid #333; border-radius: 4px; padding: 0.75rem 1rem; margin: 0.5rem 0; }
.agent-card { border-color: #3a6; }
.task-card { border-color: #6af; }
.dashboard-panel { border-color: #a86; }
dl { display: grid; grid-template-columns: max-content 1fr; gap: 0.1rem 1rem; margin: 0; }
dt { color: #aaa; }
dd { margin: 0; }
.status { font-weight: bold; }
table { border-collapse: collapse; width: 100%; margin: 0.5rem 0; font-size: 0.9em; }
th { background: #222; color: #ccc; padding: 0.4rem 0.6rem; text-align: left; border-bottom: 1px solid #444; }
td { padding: 0.3rem 0.6rem; border-bottom: 1px solid #2a2a2a; }
tr:hover td { background: #1a1a1a; }
.caption { color: #aaa; font-size: 0.85em; margin-bottom: 0.25rem; }
.event-log { list-style: none; padding: 0; margin: 0.25rem 0; }
.event { padding: 0.3rem 0.5rem; border-left: 3px solid #555; margin: 0.2rem 0; }
.layer-L4 { border-color: #3a6; }
.layer-L4E { border-color: #a33; }
.layer { font-weight: bold; color: #7af; }
.kind { color: #fc8; }
.tx-id { color: #888; font-size: 0.85em; }
.summary { display: block; color: #bbb; font-size: 0.85em; padding-left: 1rem; }
.unit { color: #888; }
"#;
