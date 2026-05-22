# Integration Map — Grill Recursive + Spec HTML Renderer Skills

Date: 2026-05-23  
Agent: R3 (Skill Codification)  
Risk class of all changes below: Class 1 (additive only; no kernel, no sequencer, no CAS schema)

This document specifies exactly which files need to change, which lines, and
what the change is. A coding agent implementing these changes needs no
additional research.

---

## PART 1: Install the two skill files

**Action**: copy the two draft files from the research directory into `skills/`.

| Source file (research draft) | Destination file (production) |
|---|---|
| `handover/research/grill_recursive_spec_html_2026-05-23/r3_skill_codification/grill_recursive_skill.md` | `skills/grill-recursive.md` |
| `handover/research/grill_recursive_spec_html_2026-05-23/r3_skill_codification/spec_html_renderer_skill.md` | `skills/spec-html-renderer.md` |

No other changes are required to make the skill *files* visible. Claude Code
discovers skills from the `skills/` directory automatically via the
`available-skills` system reminder mechanism.

---

## PART 2: CLAUDE.md — add two pre-action gates to §5

**File**: `/Users/zephryj/work/turingosv4/CLAUDE.md`  
**Section**: `## 5. Pre-action skill gates`

**Current state** of §5 (verbatim from file):
```
## 5. Pre-action skill gates

Before drafting TB charter / dispatching G1 audit: `/constitution-landing-check`

Before drafting a charter that will touch `src/` or `scripts/`: check in-flight
PRs for path overlap (see `AGENTS.md §4.1`). One-liner, no new mechanism.

Before runner script that mutates handover/evidence/: `/runner-preflight`

Before writing new feedback_*.md: ask "what mechanism enforces this?"

After TB SHIPPED FINAL or audit rounds > 3: `/harness-reflect`
```

**Required addition**: insert two new lines at the end of §5, before the blank
line that separates §5 from §6:

```
Before starting a grill session where answer vagueness or contradiction is
expected: `/grill-recursive`

Before calling `turingos generate` when the artifact is an HTML UI app:
`/spec-html-renderer`
```

**Exact insertion point**: after the line "After TB SHIPPED FINAL or audit
rounds > 3: `/harness-reflect`", before the `## 6. Audit boundary` section
header.

**Why**: §5 is the canonical list of pre-action gates for Claude Code. Adding
the two new gates here makes them machine-discoverable and ensures any session
reading CLAUDE.md will know when to fire them.

---

## PART 3: AGENTS.md — no change required

`AGENTS.md §4.1` and the shared harness contract do not enumerate individual
skills. Skills are a Claude-Code-specific mechanism (CLAUDE.md + `skills/`
directory). No change to `AGENTS.md` is needed.

---

## PART 4: spec-html-renderer — code integration in cmd_generate.rs

**File**: `src/bin/turingos/cmd_generate.rs`

There are two valid integration paths. **Path B** (user message injection)
requires only one localized change and no binary-visible constant changes.
**Path A** (system prompt injection) is more durable. Both are described.

### Path B (minimum-change, no system prompt modification)

**Location**: function `run_inner`, approximately lines 305–316.

**Current code shape** (user_msg assembly):
```rust
let user_msg = if let Some(ref fb) = prior_feedback {
    eprintln!("[generate] tape-relay: feeding prior rejection diagnostics into LLM prompt (attempt #{})", retry_index);
    format!(
        "{fb}Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
    )
} else {
    format!(
        "Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
    )
};
```

**Required change**: add a `visual_format_hint` constant and append it to
both branches of the `if let` block. The hint is a plain string appended after
`{spec_md}`. The exact text to append is:

```
\n\n---\nVISUAL FORMAT RULE (mandatory for index.html output):\n\
Apply these design tokens inline — do not use any CSS framework CDN:\n\
- Headings: font-family 'Fraunces', serif (load from Google Fonts or embed as @font-face data URI).\n\
- Body: font-family 'IBM Plex Sans', sans-serif.\n\
- Code/mono: font-family 'JetBrains Mono', monospace.\n\
- Accent color: CSS variable --accent: #4e8b7a; use for links, buttons, borders, highlights.\n\
- Background: #f8f6f1. Text: #1a1a1a.\n\
- Do NOT use Inter, Roboto, Arial, or any purple gradient.\n\
- Layout: max-width 720px centered, 1.5rem body padding, Fraunces H1 36px+, H2 24px.\n\
- If the spec is not a UI/HTML app, ignore this rule entirely.\
```

The simplest implementation: define a `const VISUAL_FORMAT_HINT: &str = "..."` 
near the top of `run_inner` (or at module level), then change both `format!`
calls to end with `"{spec_md}{VISUAL_FORMAT_HINT}"` instead of `"{spec_md}"`.

**Exact line to locate**: search for the string `"Below is the spec. Generate the working code"` — it appears exactly twice in this function, once in each branch of the if-let. Both occurrences must be updated.

### Path A (system prompt injection, more durable)

**Location**: function `blackbox_system_prompt()`, approximately lines 889–927.

**Current code shape**: the function returns a `&'static str` literal with
rules 1–6 and an example block.

**Required change**: append a Rule 7 at the end of the string, before the
closing `"#`. The rule text:

```
7. VISUAL FORMAT for HTML outputs: when your output is `index.html`, apply
   these design tokens as inline CSS (no CDN frameworks):
   - Fraunces serif for headings, IBM Plex Sans for body, JetBrains Mono for code.
   - Accent color --accent: #4e8b7a (oxidized teal).
   - Background #f8f6f1, text #1a1a1a. Max-width 720px centered.
   - Do NOT use Inter, Roboto, Arial, or purple gradients.
   - If the spec does not target a UI app, skip this rule.
```

This is a change to a `&'static str` constant in a private function. It
requires a binary rebuild to take effect. It is a Class 1 additive change with
no test implications (the existing unit tests do not inspect the system prompt
content).

**Recommendation**: implement Path A. Path B is a workaround. The system prompt
is the authoritative instruction channel for the Blackbox LLM; injecting visual
format rules there ensures they are present on every generate call, including
those triggered without the skill active (since the rule text says "if the spec
does not target a UI app, skip this rule").

---

## PART 5: No changes to grill_meta_v1.md for grill-recursive

The grill-recursive skill is entirely orchestrator-enforced via the skill file.
It does NOT require modifications to `assets/prompts/grill_meta_v1.md`.

Rationale: the recursive anchor loop is implemented by the orchestrator (Claude
Code following the skill instructions), not by the grill LLM itself. The grill
LLM continues to use `grill_meta_v1.md` unchanged. The orchestrator applies the
PRELUDE/PROTOCOL/POSTLUDE from the skill file around the normal grill turns.

If a future atom decides to bake recursive convergence into the meta-prompt
itself (so any LLM calling the meta prompt gets it natively), that would require
a new `grill_meta_v2.md` with the recursive protocol embedded. That is out of
scope for this skill installation.

---

## PART 6: embedded_prompts.rs — no change

`src/runtime/embedded_prompts.rs` embeds `grill_meta_v1.md`,
`grill_triage_blackbox_v1.md`, and `grill_synthesis_zh.md` via `include_bytes!`.
Since neither skill requires changes to these prompt files, `embedded_prompts.rs`
needs no modification.

The skill files themselves (`skills/grill-recursive.md`,
`skills/spec-html-renderer.md`) are read-at-invocation by Claude Code from the
filesystem, not compiled into the binary. No embedding required.

---

## Change summary table

| File | Change type | Scope |
|---|---|---|
| `skills/grill-recursive.md` | New file | Create from draft |
| `skills/spec-html-renderer.md` | New file | Create from draft |
| `CLAUDE.md` | Additive insert | §5, 2 new gate lines |
| `src/bin/turingos/cmd_generate.rs` | Additive modify | `blackbox_system_prompt()` — append Rule 7 (Path A, recommended) |

**No changes to**: AGENTS.md, constitution.md, any `src/kernel.rs`, `src/bus.rs`,
any CAS schema file, any sequencer file, any typed-tx file, any test that
constitution gates depend on.

**Risk class**: Class 1. `cargo check` and `cargo test --workspace` must still
pass after the changes. The new Rule 7 in the system prompt does not affect any
test that inspects the system prompt content (none currently do).

---

## Verification recipe

After implementing the changes above, run:

```bash
cargo check
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
```

All three must exit 0. No new tests are required for the skill files or CLAUDE.md
edits (Class 0 doc change for those two). The cmd_generate.rs change is Class 1;
predicate self-test (cargo test) is sufficient per AGENTS.md §14 cadence table.

To verify the visual format tokens appear in generated output, run one manual
generate pass on any HTML-type spec and grep the artifacts/index.html output
per the QUALITY CHECK in the skill file.
