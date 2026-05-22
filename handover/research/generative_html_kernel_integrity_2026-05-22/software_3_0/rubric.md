# Software 3.0 Conformance Rubric — TuringOS Generative HTML

**Produced**: 2026-05-22
**Basis**: Karpathy YC AI Startup School 2025-06 keynote + latent.space transcript +
MindStudio commentary + html-anything reference implementation + prior researcher analyses
(grill_software_3_0_2026-05-18/researcher_a/DESIGN.md + researcher_b/DESIGN.md +
interaction_substrate/30_frontier/track_h_software_3.md)

**Scope**: generative HTML pipeline only (SpecCapsule -> GenerationAttemptCapsule ->
ArtifactBundleManifest -> PreviewRunCapsule -> TestRunCapsule -> GenerateRejectionCapsule).
The spec-grill loop is covered where it feeds the generative pipeline.

---

## How to read this rubric

Each criterion has:
- **Name** — short label used in verdict_table.md
- **Definition** — one-line technical statement
- **Why it matters** — the Software 3.0 architectural reason
- **Kill condition** — what makes a system NON-conforming on this criterion

---

## C1 — Prompt-as-Program

**Definition**: The primary behavioural specification of the system is a natural-language
prompt, not a compiled artifact. The prompt is versioned, content-addressed, and drives
observable output change when mutated.

**Why it matters**: Karpathy's core Software 3.0 claim is that "prompts are programs" —
the unit of programming is the prompt, not the function. A system where behaviour is
entirely determined by hardcoded Rust/Python is Software 1.0 regardless of whether it
calls an LLM.

**Kill condition**: Behaviour is fully determined by hardcoded logic; the prompt is a
decoration or template with no meaningful branching power. OR: the prompt is not versioned
or content-addressed (cannot replay with same behaviour).

---

## C2 — LLM-as-Runtime

**Definition**: The LLM executes the program (the prompt) rather than being called as a
function by deterministic code. The LLM chooses next steps, controls branching, decides
termination. The surrounding code is the "OS kernel" that pages context, enforces
predicates, and anchors evidence — not the decision-maker.

**Why it matters**: Karpathy's OS analogy: model weights = CPU, context window = RAM,
tool calls = system calls. If the surrounding code makes all decisions and the LLM is
only a text formatter, the system is Software 1.5 at best.

**Kill condition**: Every decision point (what to ask next, when to stop, what to
generate) is made by deterministic code; the LLM only produces text that the code then
decides how to use.

---

## C3 — Natural Language (NL) as Primary Surface

**Definition**: The system's primary input from the human is natural language, and the
system's primary output is human-consumable. The NL interface is the authoritative
interface — not a convenience wrapper over a structured form.

**Why it matters**: Software 3.0 builds on "programming in English." A system that
reduces NL input to 8 fixed structured slots before feeding it to the LLM is imposing
a Software 1.0 schema on a Software 3.0 interface.

**Kill condition**: NL input is pre-parsed into a fixed schema before the LLM sees it,
meaning the LLM cannot use uncaptured context from the user's natural expression.

---

## C4 — Non-Determinism Tolerance with Anchored Evidence

**Definition**: The system explicitly acknowledges LLM non-determinism and compensates
by anchoring the actual (non-deterministic) LLM output as canonical evidence at the
moment it occurs. Replay does not re-run the LLM; it reads anchored output.

**Why it matters**: Karpathy: LLMs are "stochastic simulations" — probabilistic, not
deterministic like RAM. A Software 3.0 system cannot treat LLM output as deterministically
reproducible; it must anchor it.

**Kill condition**: No per-call anchoring of LLM output. Replay re-invokes the LLM and
accepts different output. OR: the system silently discards LLM output after use, losing
the evidence.

---

## C5 — Predicate-Gated Output Admission (Agent Trust Boundary)

**Definition**: LLM output is not trusted directly. A deterministic predicate (schema
check, constraint check, type check) gates admission before LLM output influences
subsequent system state. Rejection is typed, capsule-anchored, and retryable.

**Why it matters**: Karpathy: LLMs "hallucinate, are inconsistent." Software 3.0 systems
need verify-generate loops. A system that passes raw LLM output directly into production
paths without a predicate layer is unconstitutional regardless of how good the model is.

**Kill condition**: LLM output is used without any deterministic predicate gate. OR:
rejection is untyped, logged-only, or silently swallowed with no evidence trail.

---

## C6 — Capability Boundary Explicit and Enforced

**Definition**: The system explicitly states what it can and cannot produce, and the
claimed boundary is enforced mechanically (not just described in documentation). The
"generated artifact" is constrained to a defined capability surface.

**Why it matters**: Karpathy notes LLMs have "jagged intelligence" — they exceed at
some tasks and fail catastrophically at others. A Software 3.0 system designs around
this by constraining its claimed capability surface to what it can reliably verify.

**Kill condition**: The capability boundary is stated only in documentation but not
enforced by any predicate or test. OR: the system makes no explicit boundary claim.

---

## C7 — Tape-First Evidence with Replay

**Definition**: All LLM interactions (prompt context, model used, output received,
predicate verdict) are content-addressed in a CAS store with deterministic reconstruction
from genesis + ChainTape + CAS. A third party can replay any session without re-invoking
the LLM.

**Why it matters**: This is the TuringOS-specific amplification of Karpathy's partial
autonomy + verify loop. "Tight generate/verify cycles" require that every cycle is
auditable. Vendor-managed trace is not sufficient — the user must be able to independently
replay.

**Kill condition**: Evidence is log-only (not content-addressed). Replay requires re-running
the LLM. A third party cannot reconstruct a session from public evidence alone.

---

## C8 — Agent-Writable Long-Horizon Memory

**Definition**: Agents can persist and subsequently query observations across sessions.
Memory is not limited to the current context window. The system provides an explicit
mechanism for cross-session knowledge accumulation that the agent (not a human operator)
can write to.

**Why it matters**: Karpathy identifies "anterograde amnesia" as the primary LLM
limitation — agents start fresh every session. Software 3.0 systems must address this
architecturally, not just by making context windows larger.

**Kill condition**: No cross-session memory that the agent can write. PromptCapsule is
an audit record, not queryable memory. Each session starts with only what a human
operator pre-seeds.

---

## C9 — Partial Autonomy with Autonomy Slider

**Definition**: The system supports a spectrum of human control — from fully supervised
(human approves every LLM decision) to partially autonomous (LLM decides within predicate
boundaries). The autonomy level is configurable, not hardcoded.

**Why it matters**: Karpathy argues against both extremes: full autonomy (dangerous,
premature) and pure assistant mode (Software 1.5). The design pattern is "AI on tight
leash" with the leash length configurable.

**Kill condition**: The system is hardwired to one autonomy level. The user cannot
increase autonomy (allow LLM to run more turns, skip confirmation) or decrease it
(require confirmation before each LLM turn).

---

## C10 — IR-Grounded Reversibility

**Definition**: The generated artifact has an intermediate representation (IR) that is
richer than the final artifact. Mutations can be expressed at the IR level and regenerated
without re-prompting. The IR enables structured diff, selective regeneration, and
rollback.

**Why it matters**: html-anything uses a skills-based IR (75 locked templates). v0.dev
uses React component IR. Without an IR, every mutation requires a full re-prompt, making
the system brittle under iterative use (the primary user pattern in Software 3.0).

**Kill condition**: The generated artifact (e.g. index.html) is the only representation.
No IR exists. Mutation requires full re-generation.

---

## C11 — Layered Evaluation (Deterministic + Heuristic)

**Definition**: The system evaluates generated artifacts with at least two layers:
(a) a deterministic / mechanical check (schema parse, structure test), and
(b) a heuristic or domain-specific quality check. A single-layer gate is insufficient.

**Why it matters**: Karpathy notes the "demo-product gap" — `works.any()` vs `works.all()`.
Closing this gap requires layered eval. TuringOS track H notes the absence of an LLM-as-judge
layer between deterministic Lean predicates and human review.

**Kill condition**: Only one evaluation layer exists. OR: the only evaluation is the
LLM's self-report. OR: no evaluation exists at all and the generated artifact is
served unverified.
