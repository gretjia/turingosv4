# R3 Skill Codification — Design Rationale

Date: 2026-05-23  
Agent: R3 (Skill Codification)  
Research chain: R1 (grill recursive design) → R2 (spec HTML renderer) → R3 (skill files + integration map)

---

## 1. Skill File Format Analysis

Three existing skill files were read in full: `KARPATHY_ARCHITECT.md`, `KARPATHY_SIMPLE_CODE.md`, and `SUBAGENT_HARNESS.md`. The format is not formally specified anywhere, but the three files reveal a clear convergent pattern:

### Observed common structure

All three files share:

1. **Title line** (`# <Skill Name>`): a single H1, no subtitle.
2. **One-sentence use declaration**: the very first paragraph after the title, starting with "Use this skill when..." or "Use this skill only when...". This is the trigger description Claude Code uses to match against the `available-skills` system reminder.
3. **Philosophy / Background section** (optional but present in 2 of 3): a framing paragraph explaining *why* the skill exists, not just *what* it does.
4. **Core content sections** (H2): the actual rules, protocols, or templates. These vary by skill type:
   - Conceptual skills (KARPATHY_*): H3 numbered principles with explanations and code examples.
   - Operational skills (SUBAGENT_HARNESS): named protocol blocks — PRELUDE, MIDFLIGHT, POSTLUDE — with literal shell/pseudocode blocks.
5. **"When to use" / "When NOT to use" sections**: explicit binary triggers. Present in SUBAGENT_HARNESS; implicit in KARPATHY_* (the opening paragraph does the same job in short form).
6. **Checklist section** (KARPATHY_*) or **Report format** (SUBAGENT_HARNESS): a terminal section with a structured self-audit or output schema.
7. **Related skills** section (SUBAGENT_HARNESS only): links to related skill names and docs.

### Key design insight

The skill files are *instruction documents for the LLM at invocation time*, not configuration files for a framework. Claude Code calls `/skill-name` when the user invokes it as a slash command, or when the system reminder `available-skills` list triggers an automatic match. The skill file is then read and injected as additional context. This means:

- Every section is prose read by the LLM — there is no parser.
- Protocol blocks (bash, pseudocode, fenced text) are instructions to copy and execute literally.
- The "When to use" section controls the automatic trigger, but the LLM still decides whether to invoke the skill tool.

### SUBAGENT_HARNESS is the best template for operational skills

The two new skills (grill-recursive and spec-html-renderer) are both *operational protocols* — they define specific behaviors and steps, not architectural philosophies. SUBAGENT_HARNESS is the correct structural model: named protocol sections, explicit When/When-NOT triggers, a report format, and related-skills links.

---

## 2. Generate.rs Integration Map

### What generate.rs actually does

`src/web/generate.rs` is the *web API handler* for `POST /api/generate`. It does NOT construct the LLM prompt itself. It shells out to `turingos generate --workspace <session-dir>`, which is the binary in `src/bin/turingos/cmd_generate.rs`. The web layer:

1. Validates session_id format (line 141).
2. Verifies session directory and spec.md exist (lines 155–187).
3. Copies turingos.toml from global workspace to session dir (lines 190–197).
4. Constructs CLI args (`generate --workspace <dir> [--from-capsule] [--max-files N]`) (lines 260–271).
5. Spawns `turingos generate` as a subprocess (lines 293–351).
6. After exit, walks `artifacts/` and runs `verify_artifact_html_with_mode` (lines 404–465).
7. Broadcasts GenerateComplete via WebSocket.

### Where the LLM prompt is actually constructed

`src/bin/turingos/cmd_generate.rs` is the real prompt construction site. The critical path:

- **Line 302** (approx): `read_prior_rejection_feedback` — reads prior rejection capsules from CAS, formats them as prepended feedback text for the LLM.
- **Lines 306–316**: `user_msg` is assembled: either `{feedback}\nBelow is the spec...\n\n{spec_md}` or just `Below is the spec...\n\n{spec_md}`.
- **Lines 313–316**: `messages` vector is constructed: `[ChatMessage::system(blackbox_system_prompt()), ChatMessage::user(user_msg)]`.
- **Lines 889–927**: `blackbox_system_prompt()` is a `&'static str` returned by a private function. It is hardcoded inline — not read from a file, not injected from a prompt asset.

### The system prompt text (lines 890–927)

The hardcoded system prompt is:
- "You are TuringOS Blackbox AI, a fast code-generation assistant."
- Defines the `### File: <path>` + fenced-code-block output format.
- Rules 1–6: single-file preference, no external deps, must run as-emitted, no prose, no extra files, honor Out of Scope.

### Integration point for spec-html-renderer

There is **no existing "output format hint" injection point** in the system prompt. The only dynamic content in the LLM call is:

1. The system prompt (static, hardcoded in `blackbox_system_prompt()`).
2. The user message (dynamic: optional prior feedback + spec_md content).

To inject spec HTML rendering instructions, the integration point is the `blackbox_system_prompt()` function at lines 889–927 of `cmd_generate.rs`. Specifically:

- The system prompt currently ends with Rule 6 and an example block.
- A new Rule 7 (or an append block) would instruct the LLM to apply the spec HTML visual format when generating `index.html` outputs.
- Alternatively, the instruction could be appended to the `user_msg` after the spec content, so it arrives in the user turn rather than the system prompt.

The user-turn injection is safer (no binary rebuild required if done via spec.md content), but the system-prompt injection is more authoritative (LLM cannot override it from user context).

### Integration point for grill-recursive

Grill-recursive is a prompt-layer behavior, not a generate-layer behavior. Its integration points are:

1. `assets/prompts/grill_meta_v1.md` — the meta-prompt that drives the grill LLM during the interview session. The recursive anchor mechanism is a behavioral constraint on the LLM; it belongs here or in a v2 of this prompt.
2. `assets/prompts/grill_synthesis_zh.md` / `grill_synthesis_zh_v2.md` — the synthesis prompt that runs after the grill session ends. Recursive convergence behavior during the interview does not directly affect synthesis, but the anchor output of the recursion (the spec slot convergence) is what synthesis transforms into spec.md.
3. `src/runtime/spec_synthesis.rs` — the in-process LLM-less synthesis path (A6 atom, Phase 6.3.y). If recursive grill produces higher-confidence slot coverage, this synthesis path benefits because the slot answers it receives are less vague.

The grill-recursive skill has no Rust code integration point — it is purely a prompt-layer discipline enforced by the LLM following the skill instructions. The skill file tells the orchestrator what constraints to apply when invoking the meta LLM during a grill session.

---

## 3. Design Decisions

### Decision 1: grill-recursive is a pre-session gate skill, not a mid-session hook

The skill is invoked *before* the grill session is started (i.e., before the first call to the grill meta LLM). It does not fire on each turn. This follows the pattern of `constitution-landing-check` (pre-charter gate) and `runner-preflight` (pre-runner gate) from CLAUDE.md §5.

Rationale: the recursive anchor mechanism defines the *strategy* for the session — how the LLM should handle vague answers, how many follow-up loops per slot are permitted, and when to declare a slot "locked". This is a session initialization discipline, not a per-turn decoration.

### Decision 2: spec-html-renderer is invoked at generate time, not at spec-synthesis time

The spec HTML renderer changes the *visual format of the generated artifact*, not the content of spec.md itself. Generate time is the correct invocation point. The skill tells the Blackbox LLM what visual template to use for `index.html` outputs.

Rationale: the skill is irrelevant to spec creation (grill session), synthesis (slot-to-spec transform), or triage (input validation). It only affects the `turingos generate` call that produces the artifact.

### Decision 3: spec-html-renderer skill does not hardcode the HTML template

The skill references the HTML template by path and by its design conventions (typography, color tokens), but does not embed the full template inline. This keeps the skill file stable even as the template evolves. The specific template file path (`assets/html_templates/spec_display_v1.html` or similar) should be discovered/confirmed by R2's output.

Since R2's r2_spec_html directory was empty at the time of this research, we cannot confirm the exact template file path. The skill therefore references the design conventions abstractly and directs the implementing agent to confirm the template path from R2's design.

### Decision 4: follow SUBAGENT_HARNESS structural pattern exactly

Both new skills follow the SUBAGENT_HARNESS section layout: background/rationale block, mandatory protocol sections (PRELUDE, PROTOCOL, POSTLUDE or equivalent), When to use, When NOT to use, Related skills. The KARPATHY_* pattern (principles + checklist) is reserved for architectural/coding philosophy skills, not operational invocation protocols.

### Decision 5: no changes to restricted surfaces

The grill-recursive skill is entirely prompt-layer. The spec-html-renderer skill requires a one-line change to `blackbox_system_prompt()` in `cmd_generate.rs` — this is a Class 1 additive change (no CAS, no kernel, no sequencer). The integration map records the exact change needed but this research agent does not implement it.

---

## 4. Key Findings Summary

- `generate.rs` (web handler) does not construct LLM prompts; it shells out to the `turingos` binary.
- The LLM prompt for artifact generation is in `src/bin/turingos/cmd_generate.rs`, function `blackbox_system_prompt()` at line 889.
- The user message is assembled at lines 305–316; spec_md is appended there, after any prior-rejection feedback.
- The grill meta prompt is in `assets/prompts/grill_meta_v1.md` (embedded as `GRILL_META_V1_BYTES`).
- The synthesis prompt (`grill_synthesis_zh.md`) is informational only in the current LLM-less A6 path.
- Skills are invoked as slash commands (`/skill-name`) and are matched from the `available-skills` system reminder list.
- The §5 pre-action gates in CLAUDE.md define the canonical invocation trigger format.
