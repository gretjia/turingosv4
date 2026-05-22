# Spec HTML Renderer Skill

Use this skill when invoking `turingos generate` and the expected output is an
`index.html` artifact that should use the TuringOS spec display visual format
— Fraunces headings, JetBrains Mono code blocks, IBM Plex Sans body text,
oxidized-teal accent color — rather than a generic or framework-default style.

This skill does not change the spec content or the grill session. It only
changes the visual format instruction injected into the Blackbox LLM system
prompt at generation time.

## Background

The Blackbox LLM (`blackbox_system_prompt()` in
`src/bin/turingos/cmd_generate.rs`) currently uses a generic rule set: prefer
a single self-contained `index.html`, no external dependencies, honor the
spec's out-of-scope section. It says nothing about typography, color, or visual
structure.

The TuringOS spec display template (designed by R2) defines a consistent visual
language for all spec-derived artifacts: Fraunces for headings (loaded from
Google Fonts or bundled as a data URI), JetBrains Mono for code and monospace
elements, IBM Plex Sans for body copy, and an oxidized-teal (`#5f9ea0` or
equivalent) accent for interactive elements and highlights.

This skill tells the orchestrator exactly how to inject that visual format
instruction into the generate pipeline. It does not require the Blackbox LLM
to copy-paste the template — it instructs the LLM to apply the design
conventions as inline CSS, keeping the output self-contained.

## Mandatory PRELUDE (before calling turingos generate)

Before shelling out to `turingos generate`, the orchestrator MUST:

```text
## SPEC HTML RENDERER PRELUDE

1. Confirm output type: is the spec expected to produce an index.html?
   - Check: spec.md does NOT contain "Out of Scope" entries that exclude HTML.
   - Check: spec.md "一句话给 AI 编程员" section suggests a UI/web app.
   If neither condition holds, skip this skill (Python script output, etc.).

2. Confirm the design token set to inject:
   - Font stack:  Fraunces (headings) / IBM Plex Sans (body) / JetBrains Mono (code)
   - Accent color: oxidized-teal — use CSS variable --accent: #4e8b7a (or #5f9ea0)
   - Background:   near-white #f8f6f1 or #fafaf8
   - Text:         near-black #1a1a1a
   - No Inter, Roboto, Arial, or purple-gradient. These are forbidden.
   - No external CSS frameworks (Tailwind CDN, Bootstrap CDN) unless spec demands them.

3. Prepare the visual format instruction block (copy verbatim into prompt):
   ---
   VISUAL FORMAT RULE (mandatory for index.html output):
   Apply these design tokens inline — do not use any CSS framework CDN:
   - Headings: font-family 'Fraunces', serif (load from Google Fonts OR embed
     as a @font-face data URI if CDN is undesirable for the spec).
   - Body: font-family 'IBM Plex Sans', sans-serif.
   - Code/mono: font-family 'JetBrains Mono', monospace.
   - Accent color: CSS variable --accent: #4e8b7a; use for links, buttons,
     borders, and highlights.
   - Background: #f8f6f1. Text: #1a1a1a.
   - Do NOT use Inter, Roboto, Arial, or any purple gradient.
   - Page layout: max-width 720px centered, comfortable padding (1.5rem body),
     clear section hierarchy with H1 (Fraunces 36px+) > H2 (Fraunces 24px) > body.
   ---
```

## Protocol

### INJECTION POINT

The visual format instruction block from the PRELUDE is injected into the
Blackbox LLM prompt. There are two valid injection paths:

**Path A — System prompt injection (preferred, requires code change):**

In `src/bin/turingos/cmd_generate.rs`, function `blackbox_system_prompt()`
(line 889 to approximately 927), append the visual format instruction block as
a new Rule 7 after Rule 6. This is the authoritative path — the instruction
arrives in the system turn and cannot be overridden by spec content.

**Path B — User message injection (zero-code, immediate):**

The orchestrator appends the visual format instruction block to the user
message, after the spec content. In `run_inner` in `cmd_generate.rs`, the
user_msg is assembled at lines 305–316. The instruction goes at the end:

```
{prior_feedback}Below is the spec. Generate the working code per the rules.

spec source: {source}

{spec_md}

---
{visual_format_instruction_block}
```

Path B can be done today without a binary rebuild, by modifying the user_msg
assembly in cmd_generate.rs. Path A is the durable solution.

### QUALITY CHECK

After `turingos generate` completes and produces `artifacts/index.html`, the
orchestrator SHOULD verify the visual tokens were applied:

```bash
# Token presence check (non-blocking — log warning, do not retry on miss)
grep -q "Fraunces\|IBM Plex Sans\|JetBrains Mono" artifacts/index.html \
  && echo "[spec-html-renderer] PASS: font tokens present" \
  || echo "[spec-html-renderer] WARN: font tokens missing — LLM may have ignored instruction"

grep -q "#4e8b7a\|#5f9ea0\|--accent" artifacts/index.html \
  && echo "[spec-html-renderer] PASS: accent color present" \
  || echo "[spec-html-renderer] WARN: accent color missing"

grep -q "Inter\|Roboto\|purple-gradient" artifacts/index.html \
  && echo "[spec-html-renderer] WARN: forbidden font/color detected" \
  || echo "[spec-html-renderer] PASS: no forbidden tokens"
```

This check is advisory. A missing token is a soft failure — do not block the
artifact from delivery. Log the warning for model improvement tracking.

## Mandatory FINAL REPORT FORMAT

After generate completes with this skill active:

```text
SPEC_HTML_RENDERER: ACTIVE
INJECTION_PATH: <A_system_prompt | B_user_message>
FONT_TOKENS_PRESENT: <PASS | WARN>
ACCENT_COLOR_PRESENT: <PASS | WARN>
FORBIDDEN_TOKENS: <CLEAN | WARN: {list}>
```

## When to use this skill

- Generating a UI artifact (index.html) from any spec produced by the TuringOS
  grill session, when the output should match the TuringOS visual language.
- When the orchestrator has confirmed the spec targets a web/HTML output type.
- When re-generating an artifact and prior outputs lacked visual consistency.

## When NOT to use this skill

- When the spec's output type is Python, data analysis, or non-HTML.
- When the spec's "Out of Scope" section explicitly excludes styling or visual
  formatting (e.g., "no CSS, raw HTML only").
- When the user has specified a different design system in the spec content.

## Related skills

- `grill-recursive` — slot convergence skill invoked before this one
- `runner-preflight` — pre-action gate for runner scripts that mutate evidence/
- Reference: `src/bin/turingos/cmd_generate.rs` lines 889–927 (blackbox_system_prompt)
- Reference: `src/bin/turingos/cmd_generate.rs` lines 305–316 (user_msg assembly)
- Reference: R2 design (`handover/research/grill_recursive_spec_html_2026-05-23/r2_spec_html/`)
  for the HTML template file produced by R2
