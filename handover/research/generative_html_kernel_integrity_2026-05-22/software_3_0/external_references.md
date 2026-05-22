# External References — Software 3.0 Conformance Audit

**Produced**: 2026-05-22

---

## Primary External Reference: html-anything

**URL**: https://github.com/nexu-io/html-anything
**Fetch date**: 2026-05-22

### Product purpose and target user

html-anything positions HTML as "the final form for humans" in the agentic era. Its
stated thesis is that agents should write HTML directly — not Markdown — because human
readers consume HTML. Target users are content creators, designers, and engineers who
already have a local LLM agent CLI installed (Claude Code, Cursor, Codex, Gemini, etc.).
It explicitly does not ship its own agent: "We don't ship an agent. Yours is good enough."

### Core architectural moves

**Prompt-to-HTML pipeline**: A browser-based editor accepts Markdown, CSV, JSON, SQL, or
plain text. The user presses a keyboard shortcut, triggering a POST to `/api/convert`.
The server spawns the user's local coding-agent CLI via Node.js `child_process.spawn`.
The agent's stdout is streamed as Server-Sent Events back to the browser. The client
appends text deltas to an iframe `srcdoc` in real-time.

**Intermediate Representation (IR)**: This is html-anything's most architecturally
distinctive move. It ships 75 "skill" templates as its IR layer — pre-built HTML+CSS
modules with hard-coded constraints: CJK-first font stacks, 8px baseline grids, contrast
ratios at minimum 4.5, and a mandate to use real user data rather than placeholder text.
The skill system functions as a locked constraint layer that prevents the LLM from
"freestyling" — the model fills in real content within a predetermined structure. This is
a form of constrained generation without requiring a formal grammar constraint library.

**Persistence**: Single-file `.html` output with inlined CSS (via the `juice` library for
email/WeChat/Zhihu compatibility). No server-side session state. History persistence is
listed as "planned" but not shipped as of the fetch date.

**Capability boundary**: Generates HTML documents only. Explicitly does not generate
video (hands off to Remotion separately for animation). Does not manage API keys or LLM
credentials. Does not preserve history across sessions (marked as planned).

### Software 3.0 / Karpathy positioning

html-anything does not use "Software 3.0" terminology explicitly. Its positioning is
implicit: it frames agents as the primary author of documents in a new era, replacing
human hand-editing. The phrase "in the agentic era, you don't hand-edit docs anymore"
captures its alignment with Karpathy's "Build for agents!" principle — but the project
names no theoretical framework. It is practitioner-first, theory-light.

### Streaming vs batch

Streaming-first. SSE streaming is the primary UX: users "watch the AI draw." Interruption
is supported. Batch mode is not the primary pattern.

### Single-file vs bundle

Single-file HTML exclusively. CSS is inlined; assets embedded. The single-file constraint
is a product decision aligned with target distribution channels (WeChat articles, Zhihu
posts, email).

### Sandbox boundary

The generated HTML is isolated in an `<iframe sandbox="allow-scripts allow-same-origin">`.
Third-party scripts (Tailwind CDN, Google Fonts) execute within the iframe. The host page
is protected because cookies and localStorage are scoped to the iframe origin.

### Anti-claims

1. Does not ship an agent. Reuses the user's existing CLI.
2. No API key management.
3. No auto-layout regeneration — skill constraints are locked.
4. History persistence is unshipped (planned).
5. No video generation.

### Comparative assessment vs TuringOS

html-anything makes weaker architectural claims than TuringOS but ships more capability
in the specific HTML-generation domain today. Its skill-based IR (75 locked templates)
is a concrete, deployed solution to the IR-grounded reversibility problem (TuringOS C10
FAIL). Its single-file output constraint is a deliberate capability boundary (TuringOS C6
WARN). Its streaming-first architecture is a practitioner-level implementation of Karpathy's
"fast verify/generate loops." It has no tape, no CAS, no predicate-gated admission, no
evidence chain — making it impossible to audit or replay a session. It is Software 3.0
in the user-experience sense (agent-produced, NL-first, streaming) but not in the
constitutional/verifiable sense. TuringOS outperforms it on C4, C5, C7 by a wide margin;
html-anything outperforms TuringOS on C10 (IR), C9 (implicit autonomy slider via
interrupt), and C3 (NL surface is unconstrained — the agent sees the raw prompt, not 8
pre-defined slots).

---

## Secondary External References

### v0.dev (Vercel)

**URLs consulted**:
- https://blog.tooljet.com/lovable-vs-bolt-vs-v0/
- https://uibakery.io/blog/vercel-v0-alternatives

**Prompt mutability**: Iterative prompt refinement via chat. Post-generation mutation is
prompt-driven, not code-level. Generated React components are the artifact; UI changes
require new prompts (no IR mutation layer).

**Persistence model**: Git-based version control for generated projects. No cross-session
agent memory. State lives in the generated code repository.

**Capability boundary**: Frontend React component generation only. No backend capabilities,
no authentication, no integrations. The narrowest capability boundary of the three
comparators — and therefore the clearest conformance with C6.

**Agent-writable memory**: None documented.

**IR**: React components are the de facto IR. Vercel's component library serves as a
constraint layer analogous to html-anything's skills, but more implicit.

**Software 3.0 assessment**: v0 is Software 3.0 at the NL surface layer (prompts
in, components out) but has no tape, no capsule chain, and no predicate gates. It excels
at capability boundary clarity (C6) and NL surface (C3) but fails C4, C5, C7 entirely.

---

### bolt.new (StackBlitz)

**URLs consulted**:
- https://amankhan1.substack.com/p/how-ai-prototyping-tools-actually
- https://github.com/stackblitz/bolt.new
- https://newsletter.posthog.com/p/from-0-to-40m-arr-inside-the-tech

**Prompt mutability**: Iterative prompt-based editing. The agent maintains conversation
history within a session. Post-generation changes are expressed as follow-up prompts;
the agent decides which files to modify.

**Persistence model**: WebAssembly container (WebContainer) isolates the runtime.
No persistent storage between sessions by default — localStorage only within the browser.
External databases (Supabase) can be wired in explicitly.

**Capability boundary**: Full-stack (React frontend + Node.js API + Prisma/Supabase DB).
The capability boundary is broader than v0 but less explicit — bolt generates more but
also fails more frequently in complex cases.

**Agent-writable memory**: No cross-session memory. Saved prompt templates exist as
a library but are human-managed, not agent-writable.

**IR**: Uses structured XML-like tags embedded in natural language output (`<boltAction
type="file" ...>`) as an implicit IR — parsed post-generation rather than a formal schema.
This is closer to TuringOS's evidence capsule discipline than v0 is, but without
content-addressing or predicate gates.

**Software 3.0 assessment**: bolt.new is the closest commercial comparator to TuringOS's
design intent. Its WebContainer provides a real sandbox boundary (unlike TuringOS's
`NetworkPolicyClaim::NotEnforced`). Its implicit boltAction IR is a form of structured
output without formal enforcement. It lacks tape, capsule chain, predicate gates, and
cross-session memory. Conformance: strong on C3 (NL surface), C6 (implicit capability
boundary), and C9 (user can interrupt/redirect at any turn); weak on C4, C5, C7, C8, C10.

---

## Karpathy Software 3.0 Canonical Citation

**Primary URL**: https://www.ycombinator.com/library/MW-andrej-karpathy-software-is-changing-again
(YC AI Startup School 2025-06-17, "Software Is Changing (Again)")
**Transcription sources**: http://ikyle.me/blog/2025/andrej-karpathy-software-is-changing-again
**Commentary**: https://www.mindstudio.ai/blog/software-3-0-explained-karpathy-context-window-ram-model-weights-cpu
**Latent Space deep-read**: https://www.latent.space/p/s3

### Canonical Software 3.0 claims (paraphrase + direct quotes ≤15 words)

1. **Prompts as programs**: "LLMs are a new kind of computer, and you program them in
   English." The prompt is the program; NL is the programming language.

2. **LLM as OS**: "model weights = CPU, context window = RAM" — tool calls, files, APIs
   are the system calls. LLMs abstract complexity like an OS.

3. **Non-determinism**: LLMs are "stochastic simulations" of people — probabilistic, not
   deterministic like RAM. Systems must compensate.

4. **Capability boundary / jagged intelligence**: LLMs "peak in capability in verifiable
   domains like math and code" and fail unpredictably elsewhere.

5. **Agent-native design**: "Build for agents!" — markdown-readable interfaces, machine-
   consumable documentation. Agents are "primary consumer/manipulator of digital information."

6. **Partial autonomy**: "AI on tight leash" — autonomy sliders, fast human-AI loops,
   not full autonomy. "Decade of agents," not year.

7. **Long-horizon memory gap**: "anterograde amnesia" — LLMs lack persistent cross-session
   memory. This is an identified gap, not a solved problem.

8. **Demo-product gap**: "Demo is `works.any()`, product is `works.all()`" — closing
   this gap requires relentless testing infrastructure.

### Two corroborating sources

**arXiv 2501.11613v3** — "Conversation Routines: Task-Oriented Dialog Systems with
Embedded Prompt Logic" (cited in researcher_a/DESIGN.md): validates the structured
output / JSON envelope / rubric decomposition pattern as the 2025-2026 engineering
convergence for LLM-driven conversational loops.

**Anthropic Engineering 2025-09** — "Effective Context Engineering for AI Agents"
(https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents):
corroborates "context window as RAM" with engineering practice — just-in-time loading,
structured note-taking, sub-agent architectures for context isolation. Confirms that
context management is "a first-class engineering concern" in Software 3.0 systems.
