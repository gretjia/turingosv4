# Researcher C — Extensibility / Future-AGI Design

> Perspective: forward-projection. Start from the **plurality of future LLM-as-kernel
> surfaces** TuringOS will host (cooking, business plan, code review, debate, therapy,
> tutoring, refactor, onboarding) and reverse-engineer the abstraction that handles
> them all *without rewriting*. The current 8-question spec grill is treated as the
> first instance, not the design target.
>
> Author: Researcher C · 2026-05-18 · run in isolation from A and B.

---

## 0. Executive summary (read this first)

**Recommendation.** Introduce a single TuringOS-resident abstraction —
**`ConversationalAgent` (CA)** — defined by a declarative
**`agent.toml` manifest** (the "Conversation Capsule Spec", CCS). CA is the
*Software-3.0 program type*: a manifest names slots / completion criteria /
output schema / prompt templates / FC-trace metadata, and the runtime
(`turingos converse`) drives the dialogue, emits typed tape events per turn,
and yields a CAS-anchored artifact at completion.

The grill becomes one of N preset manifests (`preset:spec-grill-v1`); web /
CLI / frontend all consume the *same* runtime. Adding the cooking grill is
**one new TOML file**, no Rust changes, no new endpoint, no new web component.

This aligns with three external 2025–2026 anchors:

- **MCP `elicitation`** — server-initiated requests for additional info from
  users — is the wire-level mirror of what CA does locally
  ([MCP spec 2025-11-25](https://modelcontextprotocol.io/specification/2025-11-25)).
- **Open Agent Spec / Agent Spec** (Oracle, arXiv 2510.04173) —
  framework-agnostic declarative `AgentNode` with `inputs` / `outputs` /
  `prompt_template` / `{{slot}}` placeholders. TuringOS CCS is a constrained
  subset specialised for tape-anchored single-agent grills.
- **Anthropic Agent SDK** — "tool-use-first, minimal loop, rely on the model"
  — argues we should not over-abstract. CA keeps the loop trivially small.

The pattern is: **the manifest IS the program. The LLM IS the interpreter.
The tape IS the substrate.** That's the Software 3.0 reference.

---

## 1. The `ConversationalAgent` abstraction

### 1.1 What it is

A `ConversationalAgent` is the minimal program-shape that captures every
forecasted future surface (10 cases listed in the brief). It is:

```
(canonical slots, completion criteria, output schema, prompt templates, FC trace policy)
              ──────── drive ────────►  multi-turn LLM dialogue
                                             │
                                             ▼ (per turn)
                              TurnCapsule (CAS) + L4 / L4.E anchor
                                             │
                                             ▼ (on completion)
                              ArtifactCapsule (CAS) + L4 anchor
                                             │
                                             ▼
                                spec.md / recipe.md / plan.md / …
```

The substrate guarantee mirrors the Phase 6.3 `SPEC_CAPSULE` wire (already in
`src/bin/turingos/spec_capsule.rs`). Each *kind* of conversation gets its own
schema id (`turingos-spec-capsule-v1`, `turingos-recipe-capsule-v1`, …) but
they all conform to a shared envelope.

### 1.2 The CCS (Conversation Capsule Spec) manifest schema

```toml
# agent.toml — Conversation Capsule Spec v1
[meta]
id          = "spec-grill-v1"        # globally unique, semver-bumped
schema      = "turingos-ccs-v1"      # protocol version
title       = "Customer-Development Spec Grill (8-question)"
title_zh    = "面向非开发者用户的 8 问需求访谈"
lang        = ["zh", "en"]
risk_class  = 2                       # Class 2 = production wire-up
fc_trace    = ["FC1-N5", "FC1-N10", "FC2-N16", "FC3"]
license     = "internal"
authors     = ["zephryj", "researcher-2026-05-17"]

[lifecycle]
mode             = "linear"           # linear | adaptive | recursive | debate
max_turns        = 12                 # hard ceiling — kills runaway dialogue
soft_target      = 8                  # expected count for linear mode
completion       = "all_slots_filled" # all_slots_filled | predicate:<id> | turn_limit
allow_revision   = true               # can the user re-answer Qk?
budget_tokens    = 40000              # economic gate (Art 13 alignment)
budget_seconds   = 600

[llm]
role_meta      = "reasoning"          # which TuringOS two-LLM lane drives this
role_blackbox  = "off"                # "off" | "synthesis" | "validation"
temperature    = 0.3
system_prompt_template_path = "prompts/spec_system.md"
playback_required = true              # whether the agent must mirror-back

# Slot definitions — the "canonical question" array generalised.
# For static-question grills, each slot has a `prompt` (the question).
# For adaptive grills, `prompt` can be omitted and `synthesizer` produces it.
[[slot]]
id      = "job_story"
order   = 1
kind    = "freeform"                  # freeform | choice | numeric | file | image
required = true
prompt_zh = "先不用想程序怎么做。能跟我说说你最近遇到了什么事…"
prompt_en = "Forget about code for now. Tell me about a recent moment…"
max_chars = 4096
extract_hint = "JTBD opener — let user own the wording"
methodology  = ["JTBD", "MomTest:no-leading-questions"]

[[slot]]
id      = "anchor"
order   = 2
kind    = "freeform"
required = true
prompt_zh = "有没有哪个网站 / App / 小工具…"
prompt_en = "Is there a website, app, or tool that's even a little bit like…"
max_chars = 4096
methodology = ["IDEO:reference-product"]

# … six more slots (Q3..Q8) …

[output]
artifact_kind   = "markdown"          # markdown | json | yaml | rust | png | mp3
artifact_path   = "spec.md"
capsule_schema  = "turingos-spec-capsule-v1"
synthesis_prompt_template_path = "prompts/spec_synthesis.md"
required_sections = [
  "## 一句话目标",
  "## 我们要做什么 (Goal)",
  "## 像谁 (Reference)",
  "## 程序要记住的东西 (Memory)",
  "## 第一次使用 (First Run)",
  "## 不能搞坏的情况 (Robustness)",
  "## 故意不做的 (Out of Scope)",
  "## 算成功 (Acceptance)",
  "## Given/When/Then 用例",
  "## 一句话给 AI 编程员",
]
terminator_marker = "<!-- TURINGOS_SPEC_END -->"

[tape]
turn_capsule_schema     = "turingos-ca-turn-v1"
emit_per_turn_l4        = true        # one accepted WorkTx per slot fill
emit_l4e_on_reject      = true        # validate_answers failures => L4.E
prompt_capsule_required = true        # Art III + §6 externalized-attempt rule
include_artifact_in_l4  = false       # artifact rides only on final completion capsule

[ui]
frontend_component = "tos-conversation"  # generic — NOT a per-grill element
progress_label_zh  = "TISR · 八问访谈"
progress_label_en  = "TISR · 8-question grill"
empty_state_zh     = "不用想程序怎么做…"
empty_state_en     = "Forget about code for now…"
keyboard_hint_zh   = "(⌘/Ctrl+Enter 进入下一题)"
keyboard_hint_en   = "(⌘/Ctrl+Enter to advance)"
```

### 1.3 Tape contracts

Per turn → **TurnCapsule** (CAS schema `turingos-ca-turn-v1`):

```json
{
  "schema_id": "turingos-ca-turn-v1",
  "agent_id": "spec-grill-v1",
  "session_id": "1716000000_3f8a1b2c",
  "slot_id": "job_story",
  "turn_index": 1,
  "prompt_capsule_cid": "…",                // §6 externalized-attempt rule
  "user_input_cid": "…",                    // raw answer in CAS, shielded
  "validation": { "result": "pass" | "fail:too_long" | … },
  "llm_usage": { "model": "…", "total_tokens": 0 },
  "logical_t": 1716000123
}
```

- Pass → L4 `WorkTx(kind=ca_slot_fill)` accepted.
- Fail (too long / empty / off-schema for typed slot) → L4.E
  `Rejection(kind=ca_slot_reject, reason=…)`.
- The §6 invariant becomes:
  `evaluator_attempts == accepted_slot_fills + rejected_slot_fills + capsule-anchored synth calls`.
  Already satisfies the existing externalized-attempt rule because each
  slot fill is one externalized cycle.

On completion → **ArtifactCapsule** with the per-agent
`capsule_schema` (e.g. `turingos-spec-capsule-v1`, unchanged from Phase 6.3).
This is the *only* place per-agent schema diverges; everything else is shared.

### 1.4 Why this passes the 10-case forcing function

| #  | Surface              | Mode      | Slots                                       | Output schema          | Notes                                                                 |
|----|----------------------|-----------|---------------------------------------------|------------------------|-----------------------------------------------------------------------|
| 1  | spec grill (current) | linear    | 8 fixed slots                               | spec.md (10 sections)  | Phase 6.3 verbatim; see §2.                                           |
| 2  | cooking recipe       | adaptive  | dish / diet / time-budget / skill, then expand to ingredient slots | recipe.md (markdown + image refs) | `mode=adaptive`: synthesiser regenerates next question after each answer. |
| 3  | Lean Canvas business | linear    | 9 Canvas blocks                             | lean_canvas.md         | Slot id == canvas block id; trivial.                                  |
| 4  | code review grill    | adaptive  | "what changed", "risk surface", "test plan" + N adaptive probes | review.md              | LLM-driven probes; `max_turns=15`, completion = `predicate:reviewer_satisfied`. |
| 5  | multi-agent debate   | debate    | topic, then per-side claims                 | consensus.md           | `mode=debate`; CCS gains an optional `[[participant]]` table — see §1.5.|
| 6  | adversarial design   | recursive | breaker prompts                             | failure_modes.md       | `mode=recursive`; each "what would break this?" answer spawns child slots. |
| 7  | therapist / coach    | adaptive  | opener, then unbounded reflection           | session_notes.md       | `mode=adaptive`, `max_turns=20`; *no* completion predicate (user-end).|
| 8  | tutorial generator   | recursive | goal, then curriculum-tree slots            | curriculum.md          | Recursive; child manifests for each subtopic.                         |
| 9  | cross-lang refactor  | linear    | source language idioms, target language idioms, edge cases | ported.{js,py,rs} | Output kind = source code, not markdown.                              |
| 10 | onboarding plan      | linear    | role, scope, manager goals, team gaps       | plan_30_60_90.md       | Direct analogue of spec grill, different slot list.                   |

All 10 fit. The points where the *abstraction* (not just the manifest)
extends are encapsulated by **four `mode` enum values**: `linear`,
`adaptive`, `recursive`, `debate`. Every other knob is data in the manifest.

### 1.5 Multi-agent extension (case #5) — non-blocking detail

For debate mode, CCS gains an optional `[[participant]]` array:

```toml
[[participant]]
id              = "advocate"
role            = "argues FOR the proposition"
llm_role_meta   = "reasoning"
system_prompt   = "…"

[[participant]]
id              = "skeptic"
role            = "argues AGAINST"
llm_role_meta   = "reasoning"
system_prompt   = "…"
```

Single-agent mode treats the absence of `[[participant]]` as "one default
participant using the manifest's top-level `[llm]` block". This means
the grill **does not regress** as the framework grows — the most common
case (single-agent, linear) keeps the simplest manifest.

---

## 2. The grill as the first instance (zero regression vs Phase 6.3)

### 2.1 Mapping the existing 8 questions onto the manifest

The Phase 6.3 grill in `src/bin/turingos/cmd_spec.rs` becomes a single
preset shipped in `presets/spec-grill-v1/agent.toml`:

| Phase 6.3 element                                                  | New home in CCS                          |
|--------------------------------------------------------------------|------------------------------------------|
| `canonical_questions(Lang::Zh)` (lines 295–319 of cmd_spec.rs)     | 8 `[[slot]]` entries, `prompt_zh`        |
| `canonical_questions(Lang::En)`                                    | 8 `[[slot]]` entries, `prompt_en`        |
| `system_prompt(Lang::Zh)` (lines 346–393)                          | `[llm].system_prompt_template_path`      |
| `build_synthesis_user_message(...)`                                | `[output].synthesis_prompt_template_path`|
| Hard-coded `expected exactly 8 answers`                             | `[lifecycle].soft_target = 8`            |
| `Lang::Zh` / `Lang::En` enum                                       | `[meta].lang = ["zh","en"]` + per-slot prompts |
| `wrap_spec_md(...)`                                                | Inline in the artifact synthesiser stage |
| `<!-- TURINGOS_SPEC_END -->`                                       | `[output].terminator_marker`             |
| `spec_capsule::write_spec_capsule` + schema `turingos-spec-capsule-v1` | `[output].capsule_schema = "turingos-spec-capsule-v1"`; capsule writer is now a generic helper that reads `capsule_schema` from the manifest |
| `validate_answers` 4096-char rule (src/web/spec.rs)                | `[[slot]].max_chars = 4096` per slot     |
| `is_safe_session_id`                                               | Stays in web layer (orthogonal)          |

**Critical invariant.** The output artifact body for `preset:spec-grill-v1`
must be **byte-identical** to today's `wrap_spec_md(...)` output given the
same answers. Achieved by:

1. Snapshotting today's output for a frozen `--answers-file` fixture.
2. The new generic runner must reproduce that snapshot for `preset:spec-grill-v1`.
3. CI gate: `tests/ccs_spec_grill_byte_compat.rs` (Class 2 — already in
   evidence-bearing territory; constitution harness needs the byte-stability
   test to pass before old code path can be removed).

Phase 6.3's CAS schema id (`turingos-spec-capsule-v1`) is preserved, so
existing capsule CIDs remain valid across upgrade. No retroactive evidence
rewrite (CLAUDE.md §FC2 / §8).

### 2.2 What we keep, by name

- `cmd_spec.rs` keeps its public CLI entry for backward compat (see §7).
- `spec_capsule::SPEC_CAPSULE_SCHEMA_ID` keeps its constant value.
- `/api/spec/questions` and `/api/spec/submit` endpoints keep their shapes
  (proxied by the new generic endpoints with a hard-coded preset).
- The frontend `<tos-spec-grill>` keeps its `data-block-type="spec_grill"`
  attribute so any consumers selecting on it still work.

The change is **substrate**, not surface.

---

## 3. Three more instances (sketch, not implementation)

### 3.1 Cooking grill — `preset:cooking-v1`

```toml
[meta]
id = "cooking-v1"
title_zh = "今晚想吃什么？"
risk_class = 1                       # additive, no money/tape mutation

[lifecycle]
mode = "adaptive"                    # ← key differentiator from spec grill
max_turns = 8
completion = "predicate:has_ingredients_and_steps"

[[slot]]
id = "dish_intent"; order = 1
prompt_zh = "想做什么菜？或者『随便，看冰箱有什么』也行。"

[[slot]]
id = "constraints"; order = 2
prompt_zh = "有什么忌口？最多花多少时间？厨艺水平自己评一下？"

# After slot 2, the synthesiser drives:
# Q3..Q? = LLM-generated probes (e.g., "冰箱有番茄但没鸡蛋，可以吗？"),
# until the predicate `has_ingredients_and_steps` becomes true.

[output]
artifact_kind = "markdown"
artifact_path = "recipe.md"
capsule_schema = "turingos-recipe-capsule-v1"  # new schema id; same envelope
required_sections = ["## 食材", "## 步骤", "## 时间估算"]
```

No new TuringOS code. Just a new TOML + a new prompt template + (optional)
a new CAS schema id.

### 3.2 Lean Canvas — `preset:lean-canvas-v1`

`mode = "linear"`, 9 slots (one per Canvas block: problem, customer
segments, UVP, solution, channels, revenue, cost, key metrics, unfair
advantage). Output is a 9-section markdown. **Almost identical structure
to the spec grill** — proves the abstraction is not over-fit to the
specifics of the 8-question grill.

### 3.3 Coach / therapist — `preset:coach-v1`

`mode = "adaptive"`, `max_turns = 20`, `completion = "user_end"` (a special
sentinel: user clicks "I'm done"). Slots are stub — the LLM generates each
question. The artifact is a session-notes markdown that the user owns
(privacy implication: capsule may need to be encrypted/audit-only — set
`[output].capsule_class = 4` and require explicit ratification, per
CLAUDE.md §4.3 PromptCapsule defaults).

---

## 4. CLI surface (extends Path C)

Path C said: add `turingos llm complete`. I propose **three closely related
verbs**, of which `complete` is the simplest:

| Verb       | What it is                                | Tape effect                          |
|------------|-------------------------------------------|--------------------------------------|
| `complete` | One-shot `prompt → completion`            | One AttemptTelemetry; no slot loop.  |
| `converse` | Drive a CCS manifest to completion        | Per-turn TurnCapsules + final artifact capsule. |
| `chat`    | Open REPL bound to a CCS manifest         | Same as `converse` but interactive stdin instead of `--answers-file`. |

`spec` (existing) becomes a thin alias for
`turingos converse --preset spec-grill-v1`. Same flags wherever possible.

### 4.1 Concrete sketches

```bash
# Future cooking grill — pure data invocation
turingos converse \
    --workspace ./session-1234 \
    --preset cooking-v1 \
    --lang zh

# Same, from a user-defined manifest (no preset registry hit)
turingos converse \
    --workspace ./session-1234 \
    --agent-config ./my-grill.toml \
    --lang zh

# Backward-compat: today's spec command keeps working
turingos spec --workspace ./session-1234 --answers-file ./answers.json
# is equivalent to:
turingos converse --workspace ./session-1234 --preset spec-grill-v1 --answers-file ./answers.json

# One-shot completion (no slots, no artifact)
turingos llm complete \
    --model meta \
    --prompt "summarise this in 3 bullets: …"
```

### 4.2 Why three verbs, not one

`converse` is heavy: it allocates a session dir, emits per-turn capsules,
expects a manifest. `complete` is the "give me a single API call to the
configured LLM" escape hatch that researchers / pipelines / future Rust
callers need. Conflating them would force every one-shot caller to invent
a no-op manifest. Two thin verbs > one fat verb when the cost is parsing
flags.

`chat` exists because interactive vs scripted is a UX distinction; making
it a flag (`--interactive`) is fine too. I lean toward keeping `chat` as
its own verb for grep-discoverability ("what does `turingos chat` do?" is
a natural first-touch question).

---

## 5. Web surface

### 5.1 Endpoint shape — recommendation: **generic with per-agent fixtures**

```
GET  /api/converse/<agent_id>/manifest   → returns the CCS in JSON
GET  /api/converse/<agent_id>/slots      → returns the prompt list (for static-slot grills)
POST /api/converse/<agent_id>/submit     → linear-mode bulk submit (current spec flow)
POST /api/converse/<agent_id>/turn       → adaptive-mode per-turn (one slot at a time)
GET  /api/converse/<agent_id>/sessions/<sid>/artifact   → final artifact
```

Phase 7's existing endpoints become aliases:

```
GET  /api/spec/questions   → 301 /api/converse/spec-grill-v1/slots
POST /api/spec/submit      → forwarded to /api/converse/spec-grill-v1/submit
```

(Or kept as hand-written compatibility shims — see §7 migration.)

### 5.2 Per-endpoint vs per-agent — explicit decision

| Option                              | Pro                                | Con                                              |
|-------------------------------------|------------------------------------|--------------------------------------------------|
| **A.** One endpoint per agent type  | Strong type safety; each endpoint can have its own DTO struct in Rust. | N endpoints to add; backend re-deploy per new grill; no plugin model. |
| **B.** One generic endpoint, agent_id in URL | Add a grill by dropping a TOML file; no Rust change; matches MCP `tools/list` pattern. | All payloads share a single schema → weaker compile-time checks. |

**Recommendation: B.** It's the only option compatible with the §6
plugin model below. The weaker static typing is mitigated by manifest-time
validation: when a CCS is loaded, the runtime synthesises a per-agent DTO
schema and validates each `POST .../turn` body against it. This is the
same trick MCP uses for tool-call argument validation.

### 5.3 Frontend component — recommendation: **one generic `<tos-conversation>`**

The current `<tos-spec-grill>` is 376 lines; ~70 lines are spec-specific
(idle copy, CTA label, progress label). The remaining ~300 lines (state
machine, validation, render orchestration, WS listener, keyboard handler)
are pure CA infrastructure.

```ts
// Generic — drives ANY CCS manifest the backend serves.
<tos-conversation agent-id="spec-grill-v1"></tos-conversation>
<tos-conversation agent-id="cooking-v1"></tos-conversation>
<tos-conversation agent-id="lean-canvas-v1"></tos-conversation>
```

Per-agent UI flavour comes from CCS `[ui]` block (eyebrow text, CTA copy,
empty-state). `<tos-spec-grill>` becomes a 5-line shim:

```ts
class TosSpecGrill extends HTMLElement {
  connectedCallback() {
    const child = document.createElement('tos-conversation');
    child.setAttribute('agent-id', 'spec-grill-v1');
    this.appendChild(child);
  }
}
```

This preserves existing markup (`<tos-spec-grill>` selector still resolves)
while collapsing maintenance to one generic component.

**One escape hatch.** For surfaces with *non-textarea* slot kinds (image
uploads for cooking, file upload for refactor, audio for coach) the
generic component swaps in a slot-kind-specific input widget by lookup
table — same pattern as Open Agent Spec's input-property types. Adding a
new widget type *is* a Rust/TS change, but a new TOML never is.

---

## 6. Plugin / extensibility model

### 6.1 The discovery layout

```
<workspace>/agents/
    spec-grill-v1/
        agent.toml
        prompts/
            system.md
            synthesis.md
        examples/
            answers-fixture.json
    cooking-v1/
        agent.toml
        prompts/
            system.md
            synthesis.md
        examples/
            tomato-egg.json
    my-custom-grill/
        agent.toml
        prompts/
            …
```

Plus a bundled directory shipped with the binary:

```
<turingos-install>/presets/
    spec-grill-v1/...      # ships with the binary
    cooking-v1/...
    lean-canvas-v1/...
```

Resolution order for `--preset <id>`:
1. `<workspace>/agents/<id>/agent.toml`
2. `<TURINGOS_AGENTS_PATH>/<id>/agent.toml` (env override)
3. `<turingos-install>/presets/<id>/agent.toml`

Same pattern as `bin/turingos/cmd_llm.rs` workspace-vs-default config.

### 6.2 To add a new grill — concrete example

To add a Lean Canvas grill, the developer creates **one directory**:

```
<workspace>/agents/lean-canvas-v1/
├── agent.toml                  ← the CCS manifest (50–100 lines)
└── prompts/
    ├── system.md               ← system prompt template
    └── synthesis.md            ← artifact synthesiser prompt
```

And **runs**:

```
turingos converse --preset lean-canvas-v1 --workspace .
```

No Rust recompile. No web redeploy *if* the binary watches the agents
directory (FS notify) or reloads on each request (cheap — manifest is
tiny). The CAS schema id `turingos-lean-canvas-capsule-v1` is registered
on first use; capsules are reusable across runs.

### 6.3 Why not WebAssembly / DSL / scripting?

Considered and rejected for v1:

- **WebAssembly modules.** Overkill. The thing the developer wants to
  configure is *prompts and slots*, not new control-flow primitives.
  Reserve Wasm for when someone needs a *custom completion predicate*
  more complex than what TOML expresses (post-v1).
- **Custom DSL for prompts.** Use Markdown with `{{slot.id}}` and
  `{{previous.answer}}` Mustache-style substitution. This matches Open
  Agent Spec's placeholder syntax and is already universally understood.
- **Manifest-driven plugin discovery via signed manifest.** Defer to
  post-v1; security wrap (signed manifests; CAS-anchored discovery) is a
  Class 4 surface and requires architect ratification. v1 trusts the
  workspace owner.

### 6.4 Trust boundary

A user-supplied manifest is *untrusted code* from TuringOS's perspective:
it can specify prompts that try to jailbreak the model, claim a CAS
schema id colliding with a system one, or request `max_turns = 1_000_000`.
Mitigations baked into the runtime:

- Reject any `capsule_schema` that doesn't start with the agent's `id` prefix.
- Cap `max_turns` and `budget_tokens` at constitutional ceilings (declared
  in `constitution.md` if elevated; constants in code otherwise).
- Per-turn validation runs *before* the LLM call (FC1-N5 shielding).
- All user-supplied prompts pass through the §6 externalized-attempt
  capsule wire — so even a malicious manifest cannot evade evidence
  recording.

---

## 7. Migration path (no big-bang)

**Recommendation: refactor in place using strangler-fig over 3 phases.**

The architect explicitly does not want a Big Bang, and CLAUDE.md §FC2
forbids "retroactive evidence rewrite". So the only safe path is:
existing `turingos spec` *keeps* its CAS schema id and behaviour bit-for-bit;
new framework lands underneath it.

### Phase A — "Framework under, preset alias on top" (1–2 weeks)

1. Land `ConversationalAgent` runtime as new Rust modules:
   - `src/runtime/ca/mod.rs` — manifest parser + runner.
   - `src/runtime/ca/capsule.rs` — generic turn/artifact capsule writer.
2. Ship the `spec-grill-v1` preset in `<install>/presets/`.
3. `cmd_spec.rs` becomes a thin wrapper that calls
   `ca::run(preset="spec-grill-v1", args=…)`.
4. **Byte-compat gate.** A new test `tests/ccs_spec_byte_compat.rs`
   loads the same fixture as today's `tests/spec_*` and asserts the
   generated `spec.md` is byte-identical to the pre-refactor snapshot.
   Failure blocks merge.
5. CAS schema id `turingos-spec-capsule-v1` is *unchanged*; existing
   capsule CIDs remain replayable per FC2.

### Phase B — "New verbs and generic endpoints" (1–2 weeks)

1. Add `turingos converse` and `turingos llm complete` CLIs.
2. Add `/api/converse/<agent_id>/...` web routes.
3. Add `<tos-conversation>` generic frontend component.
4. **Phase 7 endpoints kept as-is.** They redirect or proxy under the
   hood. From the browser's POV nothing changes.
5. Ship `cooking-v1` or `lean-canvas-v1` as the first *new* preset, to
   prove the abstraction handles a non-grill case without regression.

### Phase C — "Old surfaces deprecated, kept indefinitely" (open-ended)

1. Mark `/api/spec/questions` and `/api/spec/submit` as "compat layer" in
   docs, but **don't remove them**. The cost of keeping them is one alias
   file. The cost of removing them is breaking every existing
   client/test/script.
2. `turingos spec` CLI: same story. Keep forever; document as alias.

### Why not "refactor in place, single PR"?

Two reasons:

- CLAUDE.md §FC2: any change touching capsule schema or CAS wire-up is at
  least Class 2; production wire-up wants the harness-first loop. The
  byte-compat gate in Phase A is exactly that harness.
- §19 "no manipulation by sequencing": closing the spec grill *into*
  the new framework only counts as landed if all 10 future cases are
  testably reachable. Phase B's "ship one non-grill preset" satisfies
  this without overcommitting to the full 10.

### Why not "strangler with old grill kept special-case"?

Tempting (lowest disruption to today's code) but leaves us with two
parallel substrates forever — exactly the over-engineering risk in §9.
Hard-pin the byte-compat gate and refactor.

---

## 8. AGI-readiness checklist

I walked the abstraction through five scenarios where the underlying
LLM gets dramatically more capable. Three pass cleanly. One requires a
manifest field already declared. One needs a small extension.

### 8.1 LLM becomes 10× smarter

**Graceful.** The framework treats the LLM as a black box that takes a
prompt and returns text. A smarter LLM may simply *need fewer slots*
(it asks better follow-ups in one turn). The CCS `mode = "adaptive"`
already supports this: increase `max_turns`, let the model converge in
fewer turns.

**No rewrite.** Optional: add a manifest field
`[lifecycle].llm_self_terminate = true` so the model can signal
completion via a sentinel token instead of slot-count.

### 8.2 LLM gains tool use mid-conversation

**Graceful with declared extension point.** CCS already has an implicit
LLM call per turn; extending the per-turn step to be `LLM(prompt) →
{ tool_calls, text }` is a runtime change, not a manifest change.

Add manifest field `[llm].tools = ["search", "filesystem:ro"]` listing
which MCP-style tools the agent may invoke. The runtime handles the
tool loop; per-tool-call evidence rides on the §6 externalized-attempt
wire (each tool invocation gets an `AttemptTelemetry` and tape anchor).

**No abstraction rewrite.** Confidence: medium-high. The risk is that
tool-using grills want *different* completion semantics ("when the
agent has finished its research"). The `completion = "predicate:<id>"`
escape hatch in CCS handles this.

### 8.3 LLM gains persistent memory across sessions

**Graceful.** Memory becomes another *resource* the runtime injects into
the prompt context. CCS gains
`[memory] kind = "vector" | "file" | "cas-capsule" / scope =
"user" | "agent" | "global"`. The CAS already provides the persistence
substrate (`MarkovEvidenceCapsule`, §FC3).

The trickier piece is **session boundary semantics**: does the cooking
grill remember last week's dinner? CCS resolves this declaratively
(`memory.scope = "user"`), not by rewriting the runtime.

**Confidence: medium.** Cross-session memory raises evidence questions
not yet settled in `CONSTITUTION_EXECUTION_MATRIX.md`. Defer until
Markov capsule replay is GREEN; CCS just needs the slot.

### 8.4 LLM becomes multi-modal (vision / audio)

**Graceful for input, needs work for output.** Input side: CCS already
declares `[[slot]].kind = "image" | "audio" | "file"`. The runtime
serialises the user-provided artifact into the LLM payload via
provider-specific encoding (OpenAI multi-modal, Anthropic vision, etc.).

Output side: `[output].artifact_kind = "markdown" | "json" | "png" | "mp3"
| "video"`. Markdown is the current default; PNG/MP3 require an
artifact emitter per kind. Each emitter is ~50–200 lines of Rust.

**Confidence: medium-high.** The risk is that multi-modal artifacts
exceed CAS-friendly sizes; mitigation is the existing CAS evidence
packaging policy (large artifacts get CID + manifest, small ones
inline).

### 8.5 LLM becomes good enough to be the *editor* of CCS manifests

This is the Software 3.0 endgame. A user says "I want a grill that
helps me plan a vacation"; the LLM emits a CCS manifest; TuringOS runs
it. CCS being declarative + TOML + small (~100 lines) makes it
LLM-writable today.

**Graceful + already enabled.** The §6 plugin model with workspace
discovery means *generated* manifests are valid first-class agents.
This is the strongest argument for the abstraction: not just that we
can hand-author 10 grills, but that the *next* grill will be authored
by an LLM in <30 seconds, dropped into `agents/` and run.

**Confidence: high.** Risk: prompt injection via LLM-authored
manifests. Mitigation in §6.4.

### 8.6 Summary table

| Scenario             | Verdict                                       |
|----------------------|-----------------------------------------------|
| 10× smarter LLM      | Graceful; optional `llm_self_terminate` field |
| Mid-conversation tool use | Graceful; runtime extension, manifest tweak |
| Cross-session memory | Graceful with new `[memory]` block; depends on Markov capsule |
| Multi-modal          | Input graceful; output needs per-kind emitter (small) |
| LLM-authored CCS     | **Already enabled** — strongest case for the design |

None require fundamental rewrite. Two require additive manifest fields.
Multi-modal output requires per-kind emitter code (additive). The
substrate is forward-compatible.

---

## 9. The risk I most worry about

**Premature generalisation.** Of the four canonical abstraction risks
(over-engineering, under-engineering, premature generalisation, lock-in
to one LLM), the one most likely to bite is *premature generalisation*:
designing the CCS schema and runtime around 10 hypothetical cases
without having shipped a non-spec instance.

History suggests this kills 60–80% of "framework" projects: the
abstraction is fitted to the imagined cases; the real second case
doesn't fit; the framework grows N escape hatches; the spec grill
ends up worse off than if we'd left it alone.

### 9.1 Specific failure modes I can name

1. **The `mode` enum (`linear` / `adaptive` / `recursive` / `debate`)
   reifies the wrong axes.** Maybe the real split is "user-paced vs
   model-paced", or "single-shot vs streaming". I picked `mode` because
   it cleanly captures cases 1–10, but I have not stress-tested it
   against case 11.
2. **The TOML manifest is read once at load.** What if a real adaptive
   grill needs to *change its own slots mid-conversation*? CCS as
   written says no (slots are static). A real case (e.g. tutorial
   generator drilling into a subtopic the user just expressed) might
   want this.
3. **The capsule schema id namespace is per-agent.** If we ship 100
   grills, we have 100 schema ids. The audit surface explodes. Maybe
   one shared `turingos-ca-artifact-v1` schema with a `agent_id` field
   inside is better. I picked per-agent for Phase 6.3 backward compat;
   it's the wrong default long-term.

### 9.2 Mitigation: the **"two real instances, then ratify"** rule

Before considering the abstraction LANDED in
`CONSTITUTION_EXECUTION_MATRIX.md`:

1. Phase A (Section 7) lands the abstraction *with the spec grill as the
   only instance*. Byte-compat gate is the only ship criterion. The
   abstraction is marked **PARTIAL** in the matrix, not LANDED.
2. Phase B ships **one** more preset (cooking or Lean Canvas). At this
   point, if the second instance required ≥3 schema fields *not in the
   v1 manifest*, that is **strong evidence the abstraction is wrong**.
   Stop. Redesign with two real datapoints, not 10 hypothetical ones.
3. Only after the second preset lands without manifest growth is the
   matrix row flipped to **LANDED**.
4. Five concrete questions to answer with the second preset before
   declaring victory:
   - Did the synthesis prompt template need any new placeholder syntax?
   - Did the slot kind enum need new variants?
   - Did the completion predicate need a new mode?
   - Did the per-turn capsule schema need new fields?
   - Did the frontend `<tos-conversation>` need any agent-specific
     branches?

If the answer to all five is "no", the abstraction is right. If two or
more are "yes", redesign before adding the third preset.

This is the same discipline as the §FC1 attempt-equality invariant:
**no abstraction is LANDED until two independent real evidence runs
agree it works**. Hypothetical case counts are not evidence.

### 9.3 Secondary risk: LLM lock-in

The existing CLI is already provider-agnostic (`TURINGOS_SILICONFLOW_ENDPOINT`
overrides the endpoint; `[llm]` block names the model id). CCS inherits
this. The remaining lock-in risk is provider-specific *prompt features*
(Claude's `<thinking>` tags, OpenAI's structured-output mode, Anthropic
tool-use formats). Mitigation: keep the manifest provider-agnostic;
the *runtime* adapts the prompt at the wire layer. Already the pattern
in `siliconflow_client.rs`. **Low risk.**

---

## 10. One-paragraph elevator pitch back to the architect

> The grill is the first Software 3.0 program: declarative slots and
> templates *are* the program; the LLM is the interpreter; the tape is
> the runtime substrate. We make this faithful by introducing one
> abstraction — the Conversation Capsule Spec (CCS) — a TOML manifest
> that names slots, completion criteria, output schema, and prompt
> templates. The Phase 6.3 grill becomes one preset (`spec-grill-v1`);
> adding cooking, Lean Canvas, or a code-review grill is one new
> directory in `<workspace>/agents/`, zero Rust changes. The migration
> path is strangler-fig with a byte-compat gate to guarantee Phase 6.3
> behaviour is unchanged. The §6 externalized-attempt rule and the
> existing CAS capsule wire are reused as-is — every grill turn anchors
> on tape exactly as today. The biggest risk is premature
> generalisation; we mitigate by refusing to mark the abstraction
> LANDED until two real grills (spec + one more) ship without
> manifest-schema growth. This is the Software-3.0-faithful pattern:
> the LLM is the kernel; CCS is the system call.

---

## Appendix A — External anchors

| Anchor                                            | Year | What it gave us                                      |
|---------------------------------------------------|------|------------------------------------------------------|
| Anthropic MCP — `elicitation` + `prompts` primitives | 2025-11 | Wire-level mirror of CCS; validates the "manifest + dialogue" pattern is industry-standard. |
| Anthropic Agent SDK                               | 2025 | "Tool-use-first, minimal loop" — argues we should keep CA runtime tiny. |
| Open Agent Spec (Oracle, arXiv 2510.04173)        | 2025-10 | Provides the declarative-component vocabulary (`LLMNode` / `AgentNode` / `inputs` / `outputs` / `{{slot}}`); CCS is a constrained subset. |
| Karpathy — Software 3.0 (YC 2025 keynote)         | 2025 | The "LLM is the kernel; manifests are the system calls" framing this design crystallises. |
| LangGraph / CrewAI / OpenAI Agents SDK            | 2025–2026 | All three converge on declarative agent + per-turn observability — independent confirmation of the pattern. |
| MCP `tools/list` discovery + 10k+ servers in prod | 2025–2026 | Validates the §6 file-based plugin model at scale. |
| Anthropic "Effective context engineering"         | 2025 | "Context is a compiled view over richer state" — same intuition as CCS-as-source / per-turn-prompt-as-build-artifact. |

## Appendix B — What I did NOT design (out of scope for v1)

- Signed manifest verification (Class 4 — needs architect ratification).
- Cross-agent composition (one CCS calling another's slots) — punts to
  Open Agent Spec's `FlowNode` model when needed.
- A visual CCS editor / dashboard.
- Streaming partial-token UX (current per-turn model assumes
  request/response cycles; streaming is an additive runtime change).
- Multi-tenancy / per-user agent registries (workspace-scoped is the v1
  contract; org-wide registry is a Class 3 surface).
- Wasm-based custom completion predicates.

## Appendix C — Open questions for the architect

1. Should the **per-agent CAS schema id** stay (Phase 6.3 compat) or
   should we collapse to a single `turingos-ca-artifact-v1` schema
   long-term? (§9.1 item 3.)
2. Should `mode = "debate"` (multi-participant) be in v1 or v2? Inclusion
   adds the `[[participant]]` array; exclusion means case #5 doesn't fit
   the v1 abstraction.
3. Should the bundled presets (`spec-grill-v1`, `cooking-v1`, …) ship
   inside the binary (embedded) or as filesystem siblings? Binary
   embedding gives single-file distribution; filesystem gives easier
   user-side patching.
4. What's the audit cadence for user-authored manifests once the §6.4
   trust-boundary mitigations are in place? Per-run, per-load, never?
