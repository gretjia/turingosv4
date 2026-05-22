//! Phase 5.7: GET /api/spec/view/:session_id
//!
//! Server-renders the per-session spec.md as an R2-aesthetic HTML view using
//! the TuringOS visual language (Fraunces + JetBrains Mono + IBM Plex Sans +
//! oxidized teal `#4e8b7a`, dark mode aware, print-friendly).
//!
//! Section mapping:
//!   `## 一句话目标`           → Hero header (project tagline)
//!   `## 我们要做什么 (Goal)`    → Goal narrative card
//!   `## 像谁 (Reference)`     → Reference card
//!   `## 立刻能做的 (Build Now)` → Left summary panel (green border, primary)
//!   `## 更深的洞察 (Deeper Insight)` → Right summary panel (teal-tinted)
//!   `## 程序要记住的东西 (Memory)` → Memory bullet card
//!   `## 第一次使用 (First Run)`   → Timeline-style steps
//!   `## 不能搞坏的情况 (Robustness)` → Robustness warning card
//!   `## 故意不做的 (Out of Scope)` → Strikethrough out-of-scope card
//!   `## 算成功 (Acceptance)`   → Acceptance criteria card
//!   `## 一句话给 AI 编程员`     → Highlighted AI-coder prompt call-out
//!
//! Anything beyond `<!-- TURINGOS_SPEC_END -->` (raw Q/A audit appendix) is
//! collapsed into an optional `<details>` block at the bottom.
//!
//! Risk: Class 1 (additive read-only handler; no kernel/CAS/sequencer touch).

use std::collections::BTreeMap;
use std::path::PathBuf;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

use super::ws::AppState;

/// TRACE_MATRIX FC2-N16: Phase 7 web — GET /api/spec/view/:session_id handler.
/// Returns `text/html`. Reads `<workspace>/sessions/<session_id>/spec.md` and
/// server-renders R2-aesthetic HTML (Fraunces + JetBrains Mono + IBM Plex Sans +
/// oxidized teal `#4e8b7a`, dark-mode aware, print-friendly). 404 if spec.md
/// missing for the session. Read-only, no kernel/CAS/sequencer touch.
pub async fn spec_view_handler(
    Path(session_id): Path<String>,
    State(_state): State<AppState>,
) -> Response {
    if !is_safe_session_id(&session_id) {
        return (
            StatusCode::BAD_REQUEST,
            Html(error_page("invalid session_id")),
        )
            .into_response();
    }
    let workspace = super::welcome::resolve_workspace_path();
    let spec_path: PathBuf = PathBuf::from(&workspace)
        .join("sessions")
        .join(&session_id)
        .join("spec.md");
    let spec_md = match std::fs::read_to_string(&spec_path) {
        Ok(s) => s,
        Err(e) => {
            let msg = format!(
                "spec.md not found for session {session_id}: {e}\nexpected at {}",
                spec_path.display()
            );
            return (StatusCode::NOT_FOUND, Html(error_page(&msg))).into_response();
        }
    };
    let html = render_spec_html(&spec_md);
    (StatusCode::OK, Html(html)).into_response()
}

fn is_safe_session_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Split spec.md into `(section_title, section_body)` ordered pairs.
///
/// Stops at the `<!-- TURINGOS_SPEC_END -->` marker; everything past that
/// (raw Q/A audit appendix) goes into `appendix` as a single blob.
struct ParsedSpec {
    sections: Vec<(String, String)>,
    appendix: String,
}

fn parse_spec_md(md: &str) -> ParsedSpec {
    let body_end = md.find("<!-- TURINGOS_SPEC_END -->").unwrap_or(md.len());
    let body = &md[..body_end];
    let appendix = if body_end < md.len() {
        let rest = &md[body_end + "<!-- TURINGOS_SPEC_END -->".len()..];
        rest.trim().to_string()
    } else {
        String::new()
    };

    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_body: Vec<String> = Vec::new();

    for line in body.lines() {
        if let Some(stripped) = line.strip_prefix("## ") {
            if let Some(t) = current_title.take() {
                sections.push((t, current_body.join("\n").trim().to_string()));
                current_body.clear();
            }
            current_title = Some(stripped.trim().to_string());
        } else if line.starts_with("# ") {
            // Top-level header (e.g., "# TuringOS Spec (Phase 6.3)") — skip.
            continue;
        } else if line.starts_with("> ") {
            // Block-quote subtitle — skip.
            continue;
        } else {
            current_body.push(line.to_string());
        }
    }
    if let Some(t) = current_title.take() {
        sections.push((t, current_body.join("\n").trim().to_string()));
    }
    ParsedSpec { sections, appendix }
}

/// Look up a section by Chinese label (with optional English suffix tolerance).
fn pick<'a>(map: &'a BTreeMap<String, String>, key: &str) -> Option<&'a String> {
    map.get(key)
}

/// Best-effort: clean spec content for a section. Strips the placeholder
/// "（用户未在本轮访谈中提供该信息）" if it's the only content.
fn clean(s: &str) -> String {
    let t = s.trim();
    if t == "（用户未在本轮访谈中提供该信息）" || t == "(user did not provide this information in the interview)" {
        String::new()
    } else {
        t.to_string()
    }
}

/// Convert a section body (which may include bullets like `- foo`) into
/// rendered HTML — bullets become `<ul><li>...</li></ul>`, multi-line text
/// becomes paragraphs.
fn render_section_body(raw: &str) -> String {
    let txt = clean(raw);
    if txt.is_empty() {
        return r#"<p class="muted">（用户未在本轮访谈中提供）</p>"#.to_string();
    }
    let lines: Vec<&str> = txt.lines().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            out.push_str("<ul>");
            while i < lines.len() {
                let t = lines[i].trim_start();
                if t.starts_with("- ") || t.starts_with("* ") {
                    out.push_str("<li>");
                    out.push_str(&html_escape(t[2..].trim()));
                    out.push_str("</li>");
                    i += 1;
                } else {
                    break;
                }
            }
            out.push_str("</ul>");
        } else if let Some(rest) = trimmed.strip_prefix(|c: char| c.is_ascii_digit()) {
            // numbered list "1. foo"
            if rest.starts_with(". ") {
                out.push_str("<ol>");
                while i < lines.len() {
                    let t = lines[i].trim_start();
                    let mut chars = t.chars();
                    let first = chars.next();
                    let second = chars.next();
                    let third = chars.next();
                    if first.is_some_and(|c| c.is_ascii_digit())
                        && (second == Some('.') || (second.is_some_and(|c| c.is_ascii_digit()) && third == Some('.')))
                    {
                        if let Some(idx) = t.find(". ") {
                            out.push_str("<li>");
                            out.push_str(&html_escape(t[idx + 2..].trim()));
                            out.push_str("</li>");
                            i += 1;
                            continue;
                        }
                    }
                    break;
                }
                out.push_str("</ol>");
            } else {
                out.push_str("<p>");
                out.push_str(&html_escape(line));
                out.push_str("</p>");
                i += 1;
            }
        } else if line.trim().is_empty() {
            i += 1;
        } else {
            // Paragraph: collect until blank or list/heading.
            let mut buf = vec![line.to_string()];
            i += 1;
            while i < lines.len() {
                let nxt = lines[i];
                let nxt_t = nxt.trim_start();
                if nxt.trim().is_empty()
                    || nxt_t.starts_with("- ")
                    || nxt_t.starts_with("* ")
                {
                    break;
                }
                buf.push(nxt.to_string());
                i += 1;
            }
            out.push_str("<p>");
            out.push_str(&html_escape(&buf.join(" ")));
            out.push_str("</p>");
        }
    }
    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Main renderer.
fn render_spec_html(spec_md: &str) -> String {
    let parsed = parse_spec_md(spec_md);
    let mut by_title: BTreeMap<String, String> = BTreeMap::new();
    for (title, body) in parsed.sections {
        by_title.insert(title, body);
    }

    let project_title = pick(&by_title, "一句话目标")
        .map(|s| {
            // Use first 60 chars as a hero title; full text becomes goal.
            let cleaned = clean(s);
            cleaned
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(80)
                .collect::<String>()
        })
        .unwrap_or_else(|| "TuringOS Spec".to_string());

    let goal_body = pick(&by_title, "我们要做什么 (Goal)")
        .or_else(|| pick(&by_title, "一句话目标"))
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let reference_body = pick(&by_title, "像谁 (Reference)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let memory_body = pick(&by_title, "程序要记住的东西 (Memory)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let first_run_body = pick(&by_title, "第一次使用 (First Run)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let robustness_body = pick(&by_title, "不能搞坏的情况 (Robustness)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let oos_body = pick(&by_title, "故意不做的 (Out of Scope)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let acceptance_body = pick(&by_title, "算成功 (Acceptance)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let ai_coder = pick(&by_title, "一句话给 AI 编程员")
        .map(|s| html_escape(&clean(s)))
        .unwrap_or_default();

    let build_now = pick(&by_title, "立刻能做的 (Build Now)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();
    let deeper_insight = pick(&by_title, "更深的洞察 (Deeper Insight)")
        .map(|s| render_section_body(s))
        .unwrap_or_default();
    let user_addition = pick(&by_title, "用户补充")
        .map(|s| render_section_body(s))
        .unwrap_or_default();

    let appendix_block = if parsed.appendix.is_empty() {
        String::new()
    } else {
        format!(
            r#"
  <details class="appendix">
    <summary>访谈原文（Q/A 审计记录）</summary>
    <pre class="qa-raw">{}</pre>
  </details>"#,
            html_escape(&parsed.appendix)
        )
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<title>{title_esc} — TuringOS Spec</title>
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link href="https://fonts.googleapis.com/css2?family=Fraunces:opsz,wght@9..144,400;9..144,600;9..144,700&family=IBM+Plex+Sans:wght@400;500;600&family=JetBrains+Mono:wght@400;600&display=swap" rel="stylesheet" />
<style>
:root {{
  --accent: #4e8b7a;
  --accent-soft: #e8f0ee;
  --accent-dark: #2f5d50;
  --bg: #f8f6f1;
  --surface: #ffffff;
  --border: #d8d4c8;
  --text: #1a1a1a;
  --muted: #6b6b6b;
  --build-now-tint: #f3faf6;
  --build-now-border: #4e8b7a;
  --insight-tint: #fbf6ec;
  --insight-border: #d8b96c;
  --warn-tint: #fbeeec;
  --warn-border: #c97765;
  --oos-tint: #f1efe9;
  --oos-text: #97907f;
  --code-bg: #f5f1e8;
  --shadow: 0 1px 2px rgba(0,0,0,0.04), 0 4px 16px rgba(0,0,0,0.04);
}}
@media (prefers-color-scheme: dark) {{
  :root {{
    --bg: #1a1a1a;
    --surface: #232323;
    --border: #3a3a3a;
    --text: #f0eee8;
    --muted: #a8a59c;
    --accent: #66a896;
    --accent-soft: #1f2e2a;
    --accent-dark: #aed0c4;
    --build-now-tint: #1e2a25;
    --build-now-border: #66a896;
    --insight-tint: #2a2418;
    --insight-border: #c6a96b;
    --warn-tint: #2a1d1a;
    --warn-border: #b97565;
    --oos-tint: #2a2825;
    --oos-text: #6f6a5f;
    --code-bg: #2a2820;
    --shadow: 0 1px 2px rgba(0,0,0,0.4), 0 4px 16px rgba(0,0,0,0.3);
  }}
}}
@media print {{
  body {{ background: white !important; }}
  .ai-coder-cta {{ break-inside: avoid; }}
  .section-card {{ break-inside: avoid; }}
}}
* {{ box-sizing: border-box; margin: 0; padding: 0; }}
html, body {{ background: var(--bg); color: var(--text); }}
body {{
  font-family: 'IBM Plex Sans', system-ui, -apple-system, sans-serif;
  font-size: 16px;
  line-height: 1.65;
  padding: 2.5rem 1.5rem 4rem;
}}
.shell {{ max-width: 820px; margin: 0 auto; }}
.hero {{
  border-bottom: 1px solid var(--border);
  padding-bottom: 1.5rem;
  margin-bottom: 2rem;
}}
.hero h1 {{
  font-family: 'Fraunces', Georgia, serif;
  font-weight: 600;
  font-size: 2.4rem;
  line-height: 1.2;
  color: var(--text);
  margin-bottom: 0.6rem;
  letter-spacing: -0.01em;
}}
.hero .subtitle {{
  color: var(--muted);
  font-size: 0.95rem;
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  text-transform: uppercase;
  letter-spacing: 0.08em;
}}
.two-panel {{
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.2rem;
  margin-bottom: 2rem;
}}
@media (max-width: 640px) {{
  .two-panel {{ grid-template-columns: 1fr; }}
}}
.panel {{
  border-radius: 12px;
  padding: 1.4rem 1.5rem;
  background: var(--surface);
  box-shadow: var(--shadow);
}}
.panel-build-now {{
  border-left: 4px solid var(--build-now-border);
  background: var(--build-now-tint);
}}
.panel-insight {{
  border-left: 4px solid var(--insight-border);
  background: var(--insight-tint);
}}
.panel-label {{
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 0.4rem;
  color: var(--accent-dark);
}}
.panel-insight .panel-label {{ color: #8a6d2c; }}
@media (prefers-color-scheme: dark) {{
  .panel-insight .panel-label {{ color: var(--insight-border); }}
}}
.panel h2 {{
  font-family: 'Fraunces', Georgia, serif;
  font-weight: 600;
  font-size: 1.3rem;
  margin-bottom: 0.8rem;
  color: var(--text);
}}
.section-card {{
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 1.4rem 1.5rem;
  margin-bottom: 1.2rem;
  box-shadow: var(--shadow);
}}
.section-card h2 {{
  font-family: 'Fraunces', Georgia, serif;
  font-weight: 600;
  font-size: 1.4rem;
  margin-bottom: 0.8rem;
  color: var(--text);
  display: flex;
  align-items: baseline;
  gap: 0.6rem;
}}
.section-card h2 .emoji {{ font-size: 1.1rem; }}
.section-card h2 .en {{
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--muted);
  text-transform: uppercase;
  letter-spacing: 0.1em;
}}
.section-robustness {{
  border-color: var(--warn-border);
  background: var(--warn-tint);
}}
.section-oos {{
  background: var(--oos-tint);
}}
.section-oos li, .section-oos p {{
  color: var(--oos-text);
  text-decoration: line-through;
  text-decoration-thickness: 1px;
}}
p {{ margin-bottom: 0.6rem; color: var(--text); }}
p.muted {{ color: var(--muted); font-style: italic; }}
ul, ol {{ margin-left: 1.2rem; margin-bottom: 0.6rem; }}
li {{ margin-bottom: 0.3rem; }}
.ai-coder-cta {{
  background: var(--accent);
  color: white;
  border-radius: 12px;
  padding: 1.5rem 1.8rem;
  margin: 2rem 0 1.2rem;
  box-shadow: var(--shadow);
}}
.ai-coder-cta .label {{
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  opacity: 0.8;
  margin-bottom: 0.4rem;
}}
.ai-coder-cta .prompt {{
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-size: 0.95rem;
  line-height: 1.6;
  white-space: pre-wrap;
}}
.appendix {{
  margin-top: 3rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border);
}}
.appendix summary {{
  cursor: pointer;
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.8rem;
  color: var(--muted);
  padding: 0.4rem 0;
  user-select: none;
}}
.appendix summary:hover {{ color: var(--accent); }}
.qa-raw {{
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.82rem;
  line-height: 1.65;
  background: var(--code-bg);
  padding: 1rem 1.2rem;
  border-radius: 8px;
  white-space: pre-wrap;
  overflow-x: auto;
  margin-top: 1rem;
}}
footer {{
  margin-top: 2.5rem;
  text-align: center;
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.7rem;
  color: var(--muted);
  letter-spacing: 0.05em;
}}
</style>
</head>
<body>
<div class="shell">
  <header class="hero">
    <p class="subtitle">TuringOS Spec · Software 3.0 grill</p>
    <h1>{title_esc}</h1>
  </header>

  <section class="two-panel">
    <div class="panel panel-build-now">
      <p class="panel-label">立刻能做的 · Build Now</p>
      {build_now}
    </div>
    <div class="panel panel-insight">
      <p class="panel-label">更深的洞察 · Deeper Insight</p>
      {deeper_insight}
    </div>
  </section>

  <article class="section-card">
    <h2><span class="emoji">🎯</span> 目标 <span class="en">Goal</span></h2>
    {goal_body}
  </article>

  {reference_card}

  {memory_card}

  {first_run_card}

  <article class="section-card section-robustness">
    <h2><span class="emoji">🛡️</span> 不能搞坏 <span class="en">Robustness</span></h2>
    {robustness_body}
  </article>

  <article class="section-card">
    <h2><span class="emoji">✅</span> 算成功 <span class="en">Acceptance</span></h2>
    {acceptance_body}
  </article>

  <article class="section-card section-oos">
    <h2><span class="emoji">🚫</span> 故意不做 <span class="en">Out of Scope</span></h2>
    {oos_body}
  </article>

  {user_addition_card}

  <div class="ai-coder-cta">
    <p class="label">⚡ 一句话给 AI 编程员 · AI-coder prompt</p>
    <p class="prompt">{ai_coder}</p>
  </div>

  {appendix_block}

  <footer>TuringOS Phase 7 · spec view · oxidized-teal aesthetic</footer>
</div>
</body>
</html>"##,
        title_esc = html_escape(&project_title),
        build_now = if build_now.is_empty() {
            r#"<p class="muted">（待生成）</p>"#.to_string()
        } else {
            build_now
        },
        deeper_insight = if deeper_insight.is_empty() {
            r#"<p class="muted">（待生成）</p>"#.to_string()
        } else {
            deeper_insight
        },
        goal_body = if goal_body.is_empty() {
            r#"<p class="muted">（待生成）</p>"#.to_string()
        } else {
            goal_body
        },
        reference_card = if reference_body.is_empty() {
            String::new()
        } else {
            format!(
                r#"<article class="section-card">
    <h2><span class="emoji">🔗</span> 像谁 <span class="en">Reference</span></h2>
    {reference_body}
  </article>"#
            )
        },
        memory_card = if memory_body.is_empty() {
            String::new()
        } else {
            format!(
                r#"<article class="section-card">
    <h2><span class="emoji">🧠</span> 要记住的 <span class="en">Memory</span></h2>
    {memory_body}
  </article>"#
            )
        },
        first_run_card = if first_run_body.is_empty() {
            String::new()
        } else {
            format!(
                r#"<article class="section-card">
    <h2><span class="emoji">🚪</span> 第一次使用 <span class="en">First Run</span></h2>
    {first_run_body}
  </article>"#
            )
        },
        robustness_body = if robustness_body.is_empty() {
            r#"<p class="muted">（待生成）</p>"#.to_string()
        } else {
            robustness_body
        },
        acceptance_body = if acceptance_body.is_empty() {
            r#"<p class="muted">（待生成）</p>"#.to_string()
        } else {
            acceptance_body
        },
        oos_body = if oos_body.is_empty() {
            r#"<p class="muted">（无）</p>"#.to_string()
        } else {
            oos_body
        },
        user_addition_card = if user_addition.is_empty() {
            String::new()
        } else {
            format!(
                r#"<article class="section-card">
    <h2><span class="emoji">💬</span> 用户补充 <span class="en">User Additions</span></h2>
    {user_addition}
  </article>"#
            )
        },
        ai_coder = if ai_coder.is_empty() {
            "（待生成）".to_string()
        } else {
            ai_coder
        },
        appendix_block = appendix_block,
    )
}

fn error_page(msg: &str) -> String {
    format!(
        r##"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Spec View Error</title>
<style>body{{font-family:system-ui;padding:2rem;color:#1a1a1a;background:#f8f6f1;}}
pre{{background:#f5f1e8;padding:1rem;border-radius:6px;white-space:pre-wrap;}}</style>
</head><body><h1>Spec View Error</h1><pre>{}</pre></body></html>"##,
        html_escape(msg)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_extracts_sections_and_appendix() {
        let md = "# Title\n\n> subtitle\n\n## 一句话目标\n\n做一个看板\n\n## 我们要做什么 (Goal)\n\n详细一些\n\n<!-- TURINGOS_SPEC_END -->\n\n## Appendix\n\n**Q1**: 你好\n";
        let parsed = parse_spec_md(md);
        assert_eq!(parsed.sections.len(), 2);
        assert_eq!(parsed.sections[0].0, "一句话目标");
        assert_eq!(parsed.sections[0].1, "做一个看板");
        assert!(parsed.appendix.contains("Q1"));
    }

    #[test]
    fn render_html_contains_design_tokens_and_content() {
        let md = "## 一句话目标\n\n奶茶店看板\n\n## 立刻能做的 (Build Now)\n\n做一个看板\n\n## 更深的洞察 (Deeper Insight)\n\n用户想了解员工\n\n<!-- TURINGOS_SPEC_END -->\n";
        let html = render_spec_html(md);
        // Design tokens present
        assert!(html.contains("--accent: #4e8b7a"));
        assert!(html.contains("Fraunces"));
        assert!(html.contains("IBM Plex Sans"));
        assert!(html.contains("JetBrains Mono"));
        // Content rendered
        assert!(html.contains("奶茶店看板"));
        assert!(html.contains("做一个看板"));
        assert!(html.contains("用户想了解员工"));
        // Two-panel split present
        assert!(html.contains("立刻能做的 · Build Now"));
        assert!(html.contains("更深的洞察 · Deeper Insight"));
    }

    #[test]
    fn empty_sections_show_placeholder_not_panic() {
        let md = "## 一句话目标\n\n仅这一节\n\n<!-- TURINGOS_SPEC_END -->\n";
        let html = render_spec_html(md);
        assert!(html.contains("（待生成）"));
        assert!(html.contains("仅这一节"));
    }

    #[test]
    fn html_escape_strips_dangerous_chars() {
        assert_eq!(
            html_escape("<script>alert(1)</script>"),
            "&lt;script&gt;alert(1)&lt;/script&gt;"
        );
    }

    #[test]
    fn safe_session_id_rejects_traversal() {
        assert!(!is_safe_session_id("../etc/passwd"));
        assert!(!is_safe_session_id(""));
        assert!(is_safe_session_id("c8d82de8-ca4e-45fa-882d-41133290be31"));
    }
}
