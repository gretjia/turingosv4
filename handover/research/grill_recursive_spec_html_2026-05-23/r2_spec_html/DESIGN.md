# Spec HTML Template — Design Rationale
**R2 Research Artifact · 2026-05-23**

---

## 1. Research Summary

### 1.1 Visual Spec Design Best Practices
Standard SRS literature (Perforce, Asana, Document360) converges on three principles for human-readable specs:
1. **Diagrams over paragraphs** for sequential flows and dependency maps
2. **Consistent structure and terminology** — every section follows the same visual grammar
3. **Scannability first** — users skim specs; the most critical information (title, goal, P0 features, blockers) must be readable in under 10 seconds

The key failure mode of traditional specs (even Markdown): walls of text that hide the hierarchy. Notion's design guide confirms the remedy: vary block types, apply liberal whitespace, use callout blocks for high-signal information, and use dividers to segment sections.

### 1.2 Pure CSS Flow Diagrams
CSS-only flow diagrams rely on three techniques (freefrontend.com, Lee Jordan's CSS-only flowcharts):
- **Pseudo-elements (`::before`, `::after`)** to draw connecting lines between nodes using `border-top`, `border-left`, and absolute positioning
- **Flexbox** for horizontal centering and distribution of nodes at each level
- **Nested list structure** (`ul > li > ul`) as the semantic backbone — hierarchies map naturally to nesting

Limitation: bidirectional or cross-links (graph topology) cannot be done in pure CSS. The spec's "First-run flow" is strictly linear (step 1 → step 2 → step N), so this limitation does not apply. We use a vertical timeline pattern instead, which is simpler and more readable for linear flows.

### 1.3 Tailwind Card Grids
Flowbite, Windframe, and Tailwind UI documentation confirm the card grid pattern:
- `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4` for responsive multi-column cards
- Each card: consistent padding (`p-5`), visible border, subtle shadow, clear title hierarchy
- Priority/status badges: small colored `span` in top-right corner of card header
- Users scan cards top-left → bottom-right; put the most critical info (name, priority, one-line desc) first

### 1.4 v0.dev / Bolt.new Presentation Patterns
Both tools generate specs as part of their "app plan" pre-generation step. Key patterns observed:
- **Two-panel summary**: what the user asked for vs. what the system understood (exactly the "立刻能做的 vs 更深的洞察" distinction)
- **Numbered build steps**: linear sequence with step numbers prominently displayed
- **Feature cards with priority tags**: P0/P1/P2 colored badges, short description, emoji icon for fast scanning
- **Highlighted prompt box**: the final "codegen prompt" is visually distinguished as a terminal/code block

---

## 2. Design System Decisions

### 2.1 Typography
**Decision: System font stack, no external CDN**

The HTML must render correctly offline and in environments where CDN calls may be blocked. System fonts are used:
```
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
font-family: ui-monospace, 'Cascadia Code', 'Source Code Pro', Menlo, monospace; /* code blocks */
```

These cover macOS (SF Pro), Windows (Segoe UI), Linux (system-ui). No Google Fonts, no @import.

**Type scale** (Tailwind defaults, no custom config needed):
- H1: `text-3xl font-bold` — project title only
- H2: `text-xl font-semibold` — section headers
- H3: `text-base font-semibold` — card titles / subsections
- Body: `text-sm leading-relaxed` — description text
- Caption: `text-xs text-gray-500` — metadata labels

### 2.2 Color Palette
**TuringOS brand: oxidized-teal + dark neutrals**

| Role | Light mode | Dark mode |
|------|-----------|-----------|
| Primary accent | `#1a8a8a` (oxidized-teal) | `#2ab8b8` |
| Background | `#f9fafb` (gray-50) | `#111827` (gray-900) |
| Card surface | `#ffffff` | `#1f2937` (gray-800) |
| Border | `#e5e7eb` (gray-200) | `#374151` (gray-700) |
| Text primary | `#111827` (gray-900) | `#f9fafb` (gray-50) |
| Text secondary | `#6b7280` (gray-500) | `#9ca3af` (gray-400) |
| P0 badge | `#dc2626` red | same |
| P1 badge | `#f59e0b` amber | same |
| P2 badge | `#3b82f6` blue | same |
| Warning | `#f59e0b` amber-500 | same |
| Out-of-scope | `#9ca3af` gray-400, strikethrough | same |

The accent color is implemented as a CSS custom property `--teal` so it can be overridden by any downstream theming.

### 2.3 Layout
**Decision: Single-column reading view, max-width 860px, left-anchored on wide screens**

Research confirms (Notion VIP, document360) that documentation is read linearly. Dashboard multi-column layouts cause users to lose their place. The 860px max-width matches readable prose width (≈70ch at 16px base). On mobile, it collapses to full-width.

Grid is used **only inside** sections (feature cards, acceptance criteria table) — never as a top-level page layout.

Sticky left sidebar for section navigation is included but degrades gracefully without JavaScript (the sidebar is visible but not auto-highlighted).

### 2.4 Icons
**Decision: Unicode/emoji only**

No icon library CDN. Emoji are used as:
- Feature card icons (chosen by the LLM per feature)
- Section header decorations (▸ for steps, ✓ for criteria, ⚠ for warnings, ✗ for out-of-scope)
- Priority badges use text labels (P0/P1/P2), not icons

This approach is zero-dependency and works in all browsers.

### 2.5 Diagrams
**Decision: CSS vertical timeline for first-run flow; CSS flexbox arrows for dependency hints**

The first-run flow (what user sees on first open) maps perfectly to a vertical timeline:
- Each step is a card connected by a vertical line (`border-left: 2px solid`)
- The step number is a circle `width: 2rem; border-radius: 50%` using the accent color
- No JavaScript required; the line is the `::before` pseudo-element of the step container

Feature dependency relationships are shown via a simple horizontal "depends on →" tag inside each feature card rather than a full graph diagram (which would require JavaScript).

---

## 3. Information Architecture

### 3.1 Section Order Rationale
```
[1] Header        — identity and context (who, what, scope)
[2] Summary       — two-panel insight contrast (orient the reader)
[3] Core features — the meat; what gets built
[4] First-run     — sequential UX walkthrough; makes it tangible
[5] Acceptance    — testable contracts; answers "how do we know it's done?"
[6] Robustness    — non-negotiable constraints; protect against regressions
[7] Out of scope  — prevents scope creep; answers "what NOT to build"
[8] AI coder      — the action; the output of the entire spec process
```

This order follows the **inverted pyramid** pattern from journalism: most important information first, context and detail later. A product owner reading section [1] and [2] gets 80% of the value in 30 seconds.

### 3.2 "Build Now vs Deeper Insight" Visual Pattern
The two-panel summary card is the most novel section. It visualizes the distinction between:
- **立刻能做的**: The user's explicit request, translated into buildable requirements
- **更深的洞察**: The system's inferred understanding — what the user probably needs but didn't say

This is rendered as a two-column card with a vertical divider and distinct background tints: warm (build now) vs cool teal (deeper insight). This visual contrast reinforces the conceptual distinction without words.

### 3.3 Acceptance Criteria Scannability
Acceptance criteria are rendered as a `<table>` with `Given / When / Then` columns, not as a bulleted list. Tables are scannable in a way lists are not — the eye can jump to the "When" column to find a specific trigger. Each row alternates background for readability.

### 3.4 Feature Relationships
Feature cards include an optional "Depends on:" tag with a `→` arrow. This is purely textual but structured so the LLM can fill it consistently. A full graph diagram was considered but rejected because:
1. It requires JavaScript or SVG coordinate math
2. Specs rarely have enough features to need a graph (typically 5-12 cards)
3. The dependency tag is readable and sufficient for 95% of cases

---

## 4. Template Variable Design

### 4.1 Handlebars-style `{{VARIABLE}}` Placeholders
All content sections use `{{VARIABLE_NAME}}` syntax. The LLM filling the template finds-and-replaces these tokens. Multi-item sections (features, steps, criteria) use JSON arrays that the LLM serializes into HTML inline.

### 4.2 Why JSON Arrays Instead of Individual Tokens
For repeating sections (feature cards, first-run steps, acceptance criteria), we provide a single `{{FEATURES_JSON}}` token rather than `{{FEATURE_1_NAME}}`, `{{FEATURE_2_NAME}}` etc. This is because:
1. The number of items varies per spec (5 to 15 features)
2. The LLM can generate the HTML for the entire card grid as a block
3. Individual tokens would require the LLM to count and number items — error-prone

The `GENERATE_PROMPT_SECTION.md` explains the expected HTML structure for each section.

### 4.3 Self-contained Rendering
Tailwind CSS is loaded via CDN script (`https://cdn.tailwindcss.com`). This is acceptable because:
- The spec is for local preview; network access is available
- No custom Tailwind config is needed (we use default utilities only)
- CSS custom properties provide all brand-specific values without PurgeCSS

For fully offline use, the generated HTML could include an inline `<style>` block that duplicates the critical utility classes. This is noted as a future optimization.

---

## 5. Accessibility & Cross-Platform

- **Dark mode**: `prefers-color-scheme: dark` media query toggles CSS custom properties at `:root` level. All colors switch; no class toggling needed.
- **Print**: `@media print` suppresses the sidebar, expands all collapsed sections, removes shadows. The spec prints cleanly on A4/Letter.
- **Mobile**: `max-width: 100%` on all cards; feature grid collapses to 1 column; sidebar hidden on narrow viewports; font sizes use `rem` throughout.
- **Screen readers**: Section headers use semantic `<h2>` / `<h3>` elements; the feature card grid uses `<ul>` with `role="list"`; the acceptance criteria use a proper `<table>` with `<thead>`.

---

## 6. Design Decisions Not Made (and Why)

| Considered | Rejected | Reason |
|-----------|---------|--------|
| Google Fonts (Fraunces + JetBrains) | System fonts | Self-contained requirement; Fraunces fails offline |
| D3.js force graph for features | CSS dependency tags | Over-engineered for 5-12 node graphs |
| Sidebar collapsible nav with JS | Static visible nav | Requires JavaScript; no JS = no nav |
| Animated step counters | Static numbered circles | Animation adds no comprehension value |
| Dark background default | Light default, dark optional | Spec is a document; light is more printable |
| Multiple accent colors | Single teal accent | Brand consistency; complexity provides no value |
