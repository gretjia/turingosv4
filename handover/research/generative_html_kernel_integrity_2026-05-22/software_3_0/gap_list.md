# Software 3.0 Gap List — TuringOS Generative HTML

**Produced**: 2026-05-22
**Source**: verdict_table.md WARN and FAIL rows

Gaps are ordered by severity (FAIL first, then WARN by Software 3.0 architectural
importance). Each gap includes: what's missing, file path / surface, why it matters,
and recommended fix scope (Class 1/2/3/4 estimate).

---

## G1 — No IR for Generated HTML (C10 FAIL)

**What's missing**: An intermediate representation layer between the spec.md input and
the raw HTML output. Currently, `turingos generate` calls the Blackbox LLM with a
hardcoded system prompt and receives a raw code-fenced response. The parser in
`cmd_generate.rs` extracts files from the code fences and writes them directly to disk.
There is no structured IR that can be mutated, diffed, or regenerated selectively.

**File path / surface**:
- `src/bin/turingos/cmd_generate.rs` (the generate command; no IR step)
- `src/web/ir.rs` (IRRoot/Block — this is a dashboard IR, not a generative-HTML IR)
- No equivalent to html-anything's 75-skill template library

**Why it matters for Software 3.0**: Without an IR, every user request for a change
requires a full re-generation of the entire artifact. This is the "full re-prompt" failure
mode Karpathy's partial-autonomy slider is designed to avoid. html-anything's skill
templates function as an IR that allows targeted mutations ("change the font" = swap one
skill parameter) without touching the full generation. TuringOS's 3-attempt retry loop
(W8) is a brute-force substitute for IR-backed targeted repair. With an IR:
- Mutations are structured edits, not re-generations
- Diffing before/after a spec change is possible
- The LLM can be directed to fix only the failing section

**Recommended fix scope**: Class 2-3.
- Class 2: Define a JSON-serializable `GenerativeHtmlIr` struct representing the HTML
  document's high-level structure (sections, components, data bindings). Write an
  `ir_to_html` renderer and an `html_to_ir` extractor. This is ~500-800 LoC additive.
- Class 3: Integrate the IR into the generate path: LLM writes IR JSON, Rust validates
  and renders to HTML, TestRunCapsule references IR CID alongside artifact CID.
  This touches `cmd_generate.rs` and `src/web/generate.rs` (Class 2-3 surfaces).
  No typed_tx schema change; no Class 4 surface.

---

## G2 — No Agent-Writable Cross-Session Memory (C8 FAIL)

**What's missing**: Agents (the LLM executing in the grill or generate pipeline) cannot
persist observations across sessions. Every session starts cold. The PromptCapsule is
an audit record of what the agent saw, but it is write-once at session time and not
queryable by future agents. There is no equivalent of Letta's `archival_memory_*` or
Anthropic's `NOTES.md` pattern.

**File path / surface**:
- `src/runtime/prompt_capsule.rs` (PromptCapsule: read-only audit record, not queryable
  by future agents)
- No `agent_memory.rs` or equivalent module exists
- `handover/research/interaction_substrate/30_frontier/track_h_software_3.md:127-129`
  confirms: "没有 archival_memory_* 等价"

**Why it matters for Software 3.0**: Karpathy names "anterograde amnesia" as the primary
LLM limitation. A Software 3.0 substrate that cannot address this architecturally leaves
every hosted agent re-learning the same context from scratch. For generative HTML
specifically: a user who has done 5 prior sessions (Tetris, a budget tool, a recipe card,
a quiz, a calendar) should benefit from the agent having seen those patterns. Currently
it cannot. For the spec-grill: the LLM cannot observe "this user tends to skip the memory
slot — probe harder there" across sessions.

**Recommended fix scope**: Class 2-3.
- Class 2 (minimal): A session-searchable CAS index — allow `turingos agent-memory search
  --query <text>` to find prior EvidenceCapsule summaries by embedding similarity or
  keyword. The capsules already exist; the missing piece is a query surface.
- Class 3 (full): An agent-writable `AgentMemory` CAS object type per session/agent,
  with read_set integration in PromptCapsule so the agent's visible context can include
  retrieved memories. This is the Letta-equivalent pattern and requires careful shielding
  policy design (Art. III.3 — do not leak cross-session private facts into other agents'
  contexts).

Note: This is explicitly deferred in track_h: "Phase 4 不动, 但要在 archive 里记一条
'TuringOS 是能力受限的 audit-first substrate'."

---

## G3 — Prompt-as-Program Only in Driven Mode; Generate Prompt is Unversioned (C1, C2 WARN)

**What's missing**: Two sub-gaps:
(a) The web spec handler defaults to static mode (8 hardcoded Rust strings in
    `src/web/spec.rs:73-80`), bypassing the driven-mode LLM-as-runtime design.
(b) The generate-side system prompt (the prompt that tells the LLM how to produce HTML
    from spec.md) is a hardcoded string in `cmd_generate.rs` that is NOT content-
    addressed, NOT written to CAS, and NOT versioned. If this prompt changes, there is
    no capsule evidence that the change happened.

**File path / surface**:
- `src/web/spec.rs:73-80` (SPEC_QUESTIONS_ZH hardcoded)
- `src/bin/turingos/cmd_generate.rs:1-57` (hardcoded generation system prompt)
- `src/runtime/generation_attempt.rs:27-37` (prompt_hash captures request bytes, but
  the system prompt template itself is not separately CAS-resident)

**Why it matters for Software 3.0**: C1 requires that the prompt be the program —
versioned, content-addressed, and the driver of observable behaviour change. A hardcoded
Rust string does not qualify. If someone ships a prompt improvement, no capsule records
"this session used generation_system_prompt_v2 not v1." The `prompt_hash` in
`GenerationAttemptCapsule` captures the full request hash, which indirectly includes
the system prompt — but there is no explicit `system_prompt_template_hash` field (unlike
the grill, which has this in `GrillAttemptRecord`).

**Recommended fix scope**: Class 2.
- (a) Web default: expose `--mode driven` as the web spec flow (or make it user-selectable
  in the frontend). This is a frontend + web handler change; no new capsule types. ~100 LoC.
- (b) Generate prompt versioning: extract the generation system prompt to
  `assets/prompts/generate_system_v1.md`, hash it, write hash into a new field in
  `GenerationAttemptCapsule`. This follows the exact pattern in `GrillAttemptRecord.
  prompt_context_hash`. Additive field, no schema version bump required (tail-additive).
  ~50 LoC in `cmd_generate.rs`.

---

## G4 — No OS-Level Sandbox for Generated Artifact Execution (C6 WARN)

**What's missing**: The generated HTML artifact runs in the user's browser inside an
iframe with `sandbox="allow-scripts allow-same-origin"`. This is browser-level isolation.
TuringOS does not enforce OS-level hermetic sandboxing (no bwrap, no seccomp, no Wasmtime,
no network DenyAll). The `NetworkPolicyClaim::NotEnforced` sentinel in
`src/sdk/sanitized_runner.rs:13` correctly names this — but the capability boundary for
what the generated HTML can do (network calls, localStorage abuse, etc.) is documented
but not mechanically enforced at generation time.

**File path / surface**:
- `src/sdk/sanitized_runner.rs:12-13` (NetworkPolicyClaim::NotEnforced)
- `src/runtime/preview_run.rs:21-25` (SandboxPolicy enum — AllowScripts or
  AllowScriptsAllowSameOrigin — no DenyAll option)
- `src/runtime/test_run.rs:175-196` (SandboxPolicyPreserved scenario: checks only that
  the policy attribute string appears; does NOT verify it is applied correctly)
- `handover/ai-direct/LATEST.md:134-148` (Active Non-Claims)

**Why it matters for Software 3.0**: Capability boundary must be enforced, not just
described (C6). An LLM producing HTML can accidentally (or via prompt injection) generate
code that calls external APIs, exfiltrates localStorage, or loads remote scripts that
bypass the iframe sandbox. The current SandboxPolicyPreserved test verifies the attribute
exists — it does not verify that no external network calls are present in the generated
code.

**Recommended fix scope**: Class 2-3.
- Class 2: Add a static analysis predicate in `src/web/verify.rs` that flags generated
  HTML containing `fetch()` calls to non-same-origin URLs, `XMLHttpRequest` to external
  hosts, or `<script src="...">` loading non-CDN URLs. This is heuristic but raises the
  floor significantly.
- Class 3: A `SandboxPolicy::DenyAll` option that strips all external resource references
  from the generated HTML before serving. Requires care to avoid breaking legitimate CDN
  usage (Tailwind, Google Fonts). The LATEST.md notes "if choosing sandbox phase 1, make
  the mechanism explicit first: process-only, bwrap/unshare/seccomp, or VM/Wasmtime."

---

## G5 — No Autonomy Slider Exposed to End Users (C9 WARN)

**What's missing**: The driven-mode grill (LLM-controlled turn count, slot coverage)
exists as a CLI flag but is not exposed in the web frontend. Users cannot configure
how much autonomy the LLM has. `MAX_GENERATE_ATTEMPTS=3` is a compile constant, not
a runtime user setting.

**File path / surface**:
- `src/web/generate.rs:66` (MAX_GENERATE_ATTEMPTS=3, not user-configurable)
- `src/bin/turingos/cmd_spec.rs:73-80` (--mode static/driven is CLI-only)
- `frontend/src/components/spec-grill.ts` (no autonomy controls exposed)

**Why it matters for Software 3.0**: Karpathy's "partial autonomy slider" is a first-
class design requirement. TuringOS has the driven grill as a higher-autonomy mode, but
it is not surfaced to web users. The web user always runs the static 8-question form —
the same Software 1.0 flow the prior researcher identified as the core defect.

**Recommended fix scope**: Class 1-2.
- Add a toggle in the web frontend: "快速模式 (8题固定) / 智能模式 (AI 自由提问)."
  The backend handler already implements both paths; the gap is solely in the web
  frontend and the `POST /api/spec/turn` endpoint being underused vs `POST /api/spec/submit`.
  ~50-100 LoC frontend + minor web handler routing change.

---

## G6 — Missing LLM-as-Judge Evaluation Layer (C11 WARN)

**What's missing**: The evaluation pipeline has two layers (heuristic structural checks
+ 3 deterministic scenarios). There is no LLM-as-judge layer for subjective quality:
spec faithfulness ("does the generated HTML match what the user actually described?"),
layout coherence, UX quality. For non-game artifacts (productivity tools, data tables,
forms), the heuristic and structural checks pass trivially even for low-quality output.

**File path / surface**:
- `src/web/verify.rs` (heuristic checks — game-shape only)
- `src/runtime/test_run.rs:116-197` (3 scenarios: EntrypointExists, HtmlParses,
  SandboxPolicyPreserved — all structural, none semantic)
- Track H explicitly: "没有 LLM-as-judge 路径"

**Why it matters for Software 3.0**: The demo-product gap (Karpathy: "works.any() vs
works.all()") is closed by testing infrastructure. Structural tests alone cannot close
the subjective quality gap. html-anything uses skill constraints to prevent drift;
TuringOS currently has neither skill constraints nor an LLM judge for the generate path.

**Recommended fix scope**: Class 2.
- Add an optional `TuringOS.spec_faithful` check to TestRunCapsule: spawn a fast Blackbox
  model call with the spec.md + the generated HTML + a structured rubric question ("does
  this HTML implement the spec requirement X?"). The result goes into a new
  `TestScenario::SpecFaithful` variant — not used as a gate (non-deterministic), but
  as an additional evidence capsule. The hidden-oracle discipline (scenario set CID not
  propagated to generation prompt) already applies. ~200 LoC in test_run.rs + test_scenario.rs.
