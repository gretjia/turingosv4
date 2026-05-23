# LLM Generate Prompt — Spec HTML Section
**Embed this block verbatim in the spec generation system prompt.**

---

## SPEC HTML GENERATION INSTRUCTIONS

You are generating a TuringOS software spec as a self-contained HTML file. The output must be a complete, valid HTML document using the spec template. Follow these rules precisely.

---

### STEP 1: Start with the base template

Use the file `spec_template.html` as your starting scaffold. Do NOT change the `<head>`, CSS, or structural HTML. Only replace the `{{VARIABLE}}` placeholders and the demo content blocks marked with `<!-- ─── DEMO ... (LLM replaces) ─── -->` comments.

---

### STEP 2: Fill these simple text variables

Replace each `{{TOKEN}}` with the appropriate string. Do not add HTML tags inside these unless specified.

| Token | Expected content |
|-------|-----------------|
| `{{PROJECT_TITLE}}` | Project name, in the user's language (e.g., "SmartChef 智能菜谱助手") |
| `{{ONE_LINE_GOAL}}` | One sentence: what the product does for whom. Max 20 words. No quotes. |
| `{{COMPLEXITY}}` | One of: `低 Low` / `中 Medium` / `高 High` / `极高 Very High` |
| `{{DOMAIN}}` | Primary domain, e.g., `消费类 App` / `工具 / 效率` / `企业 SaaS` / `AI 原生` |
| `{{TIMELINE_ESTIMATE}}` | Rough effort estimate, e.g., `3–5 天` / `2 周` / `1 个月` |
| `{{GENERATED_DATE}}` | Today's date in YYYY-MM-DD format |
| `{{BUILD_NOW_TITLE}}` | 4–8 word title for the "build now" panel |
| `{{BUILD_NOW_SUMMARY}}` | 3–5 sentence paragraph explaining what to build immediately. Chinese preferred. |
| `{{INSIGHT_TITLE}}` | 4–8 word title for the "deeper insight" panel |
| `{{DEEPER_INSIGHT_SUMMARY}}` | 3–5 sentence paragraph revealing the underlying user need. Chinese preferred. |
| `{{AI_CODER_PROMPT}}` | See Step 6 below |

---

### STEP 3: Generate `{{FEATURES_HTML}}`

Replace the entire `{{FEATURES_HTML}}` placeholder AND the demo cards block (from `<!-- ─── DEMO CARDS` to the last `</div>` before `</div><!-- /feature-grid -->`) with feature cards.

**Each feature card follows this exact HTML pattern:**

```html
<div class="feature-card" role="listitem">
  <div class="feature-card-top">
    <span class="feature-icon">EMOJI</span>
    <span class="priority-badge PRIORITY_CLASS">PRIORITY</span>
  </div>
  <div class="feature-name">FEATURE NAME</div>
  <div class="feature-desc">SHORT DESCRIPTION (1–2 sentences, ≤40 words)</div>
  <!-- OPTIONAL — only include if this feature depends on another -->
  <div class="feature-dep">→ 依赖：DEPENDENCY NAME</div>
</div>
```

**Rules for feature cards:**
- `EMOJI`: Choose a relevant emoji that visually represents the feature. Use a single emoji character.
- `PRIORITY_CLASS`: Use exactly `p0`, `p1`, or `p2` (lowercase).
- `PRIORITY`: Use exactly `P0`, `P1`, or `P2` (uppercase).
- Priority semantics: `P0` = must-have (app broken without it), `P1` = should-have (major UX value), `P2` = nice-to-have.
- Generate between **4 and 8** feature cards. More than 8 overwhelms; fewer than 4 undersells the spec.
- Order cards: all P0 cards first, then P1, then P2.
- The `feature-dep` div is optional. Only include it if the feature genuinely requires another feature to function.
- Keep descriptions concrete and user-facing. Avoid technical jargon ("REST API", "microservice") in this section.

---

### STEP 4: Generate `{{FIRST_RUN_STEPS_HTML}}`

Replace `{{FIRST_RUN_STEPS_HTML}}` AND the demo steps block with timeline steps.

**Each step follows this exact HTML pattern:**

```html
<div class="timeline-step">
  <div class="step-dot">STEP_NUMBER</div>
  <div class="step-card">
    <div class="step-title">STEP TITLE (≤8 words)</div>
    <div class="step-detail">WHAT THE USER SEES AND DOES (2–4 sentences)</div>
    <!-- OPTIONAL — include only for key UX moments -->
    <span class="step-ux-note">💡 UX: UX INSIGHT (≤15 words)</span>
  </div>
</div>
```

**Rules for first-run steps:**
- Generate between **4 and 7** steps. This is a linear walkthrough of a new user's first 5 minutes.
- `STEP_NUMBER`: Integer starting at 1, incrementing by 1.
- Each step describes what the user *sees* first, then what they *do*.
- The `step-ux-note` span is optional. Use it for the 2–3 most important UX decisions (e.g., zero-login entry, primary action placement).
- Steps must be sequential — no branching. If there is branching in the UX, pick the happy path.
- Escape HTML special characters: use `&amp;` for `&`, `&lt;` for `<`, `&gt;` for `>`.

---

### STEP 5: Generate `{{CRITERIA_ROWS_HTML}}`

Replace `{{CRITERIA_ROWS_HTML}}` AND the demo rows with acceptance criteria table rows.

**Each row follows this exact HTML pattern:**

```html
<tr>
  <td class="given-col">PRECONDITION (short phrase, ≤10 words)</td>
  <td class="when-col">USER ACTION (start with verb)</td>
  <td><span class="check-icon">✓</span> EXPECTED OUTCOME (specific, measurable)</td>
</tr>
```

**Rules for acceptance criteria:**
- Generate between **5 and 8** rows — one row per key user-observable behavior.
- Cover: the happy path for each P0 feature, at least one error/edge case, and one non-functional requirement (speed, offline, data correctness).
- `PRECONDITION`: State where/when the user is (e.g., "用户在购物清单页面" / "网络断开时").
- `USER ACTION`: Start with an action verb (点击、输入、滑动、打开、刷新…).
- `EXPECTED OUTCOME`: Be specific. Include numbers where possible (e.g., "在 2 秒内", "至少 3 个结果", "不超过 1 次请求"). Never write "system works correctly".
- Do NOT write acceptance criteria for out-of-scope features.

---

### STEP 6: Generate `{{AI_CODER_PROMPT}}`

This is the most important output. It is the complete natural-language prompt that a developer will paste into Claude, GPT, or Cursor to generate the actual application code.

**Format requirements:**
- Plain text, no Markdown formatting, no bullet points.
- Write in Chinese (simplified) unless the user's project is entirely English.
- Structure: `[产品名称 + 核心价值] [P0功能列表] [P1功能列表] [技术栈偏好（如果已知）] [关键验收约束] [明确不做的事]`
- Length: 80–150 words. Long enough to be complete; short enough to fit in a single prompt.
- Be specific about data persistence, tech stack, and constraints from the robustness rules.
- End with a statement of what NOT to implement (the out-of-scope items), prefixed with "不实现：".

**Template for the AI coder prompt:**
```
构建一个名为 [PROJECT_NAME] 的 [PRODUCT_TYPE]。
核心功能：[P0_FEATURE_1]（P0）、[P0_FEATURE_2]（P0）、[P1_FEATURE_1]（P1）。
技术要求：[TECH_STACK_OR_DEFAULT]。
验收约束：[KEY_ROBUSTNESS_RULES_AS_PROSE]。
不实现：[OOS_1]、[OOS_2]、[OOS_3]。
```

---

### STEP 7: Generate `{{ROBUSTNESS_RULES_HTML}}`

Replace `{{ROBUSTNESS_RULES_HTML}}` AND the demo rules with robustness rule items.

**Each rule follows this pattern:**

```html
<div class="rule-item">
  <span class="rule-icon">⚠️</span>
  <div class="rule-text">
    <strong>RULE TITLE (2–4 words)</strong>
    RULE EXPLANATION (2–4 sentences describing what must never break and why)
  </div>
</div>
```

**Rules for robustness section:**
- Generate **3 to 5** rules.
- Each rule is a hard constraint — something that, if violated, constitutes a bug not a "feature gap".
- Focus on data integrity, consistency, error states, and user trust (e.g., "data loss never", "empty states always handled", "offline graceful").
- Write from the user's perspective: "User must never see..." / "System must always...".

---

### STEP 8: Generate `{{OOS_ITEMS_HTML}}`

Replace `{{OOS_ITEMS_HTML}}` AND the demo out-of-scope items.

**Each item follows this pattern:**

```html
<div class="oos-item">
  <span class="oos-x">✗</span>
  <span class="oos-text">OUT OF SCOPE FEATURE DESCRIPTION</span>
</div>
```

**Rules:**
- Generate **3 to 6** items.
- Each item describes a feature that a user *might* expect but that is explicitly excluded from this version.
- Write each as a noun phrase (not a sentence). Include a brief parenthetical reason where helpful.
- Items must be genuinely related to the product domain — not generic exclusions like "user authentication" unless that was actually discussed.

---

### VALIDATION CHECKLIST

Before returning the HTML, verify:

- [ ] All `{{TOKEN}}` placeholders are replaced (none remain)
- [ ] All `<!-- ─── DEMO ... (LLM replaces) ─── -->` demo blocks are removed and replaced with real content
- [ ] Feature cards: 4–8 cards, ordered P0 → P1 → P2
- [ ] Timeline steps: 4–7 steps, numbered sequentially starting at 1
- [ ] Acceptance criteria: 5–8 rows, each with measurable outcomes
- [ ] Robustness rules: 3–5 items
- [ ] Out of scope: 3–6 items
- [ ] AI coder prompt: 80–150 words, ends with "不实现："
- [ ] No Markdown in the HTML output — only valid HTML
- [ ] The `<title>` tag contains the project title
- [ ] The HTML document is complete: starts with `<!DOCTYPE html>` and ends with `</html>`
- [ ] No `<script>` tags added beyond the sidebar enhancement already in the template
- [ ] No external CDN links added beyond `https://cdn.tailwindcss.com`

---

### CRITICAL: Do NOT modify

- The `<head>` element and all CSS in `<style>` — do not change any class names, CSS properties, or media queries
- The sidebar `<nav>` links — they are hardcoded to match section IDs
- The section `id` attributes (`id="header"`, `id="summary"`, etc.) — the sidebar and print styles depend on them
- The Tailwind CDN script tag
- The sidebar IntersectionObserver script at the bottom
- The `class="no-print"` attribute on the sidebar and footer

---

### OUTPUT FORMAT

Return ONLY the complete HTML document. No preamble, no explanation, no code fences. The response must start with `<!DOCTYPE html>` and end with `</html>`.
