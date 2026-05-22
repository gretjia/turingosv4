# Charter Recommendations — Software 3.0 Gap Closure

**Produced**: 2026-05-22
**Basis**: gap_list.md G1-G6, verdict_table.md, LATEST.md "Recommended Next Work"

---

## Context: LATEST.md Recommended Next Work

The current handover (handover/ai-direct/LATEST.md, session #57 close 2026-05-21)
lists three options:

> 1. OS-level sandbox phase 1
> 2. P7.z truthfulness follow-up
> 3. Tiny replayable-decision smoke test

The instructions specify: "If choosing sandbox phase 1, make the mechanism explicit
first: process-only, bwrap/unshare/seccomp, or VM/Wasmtime. Do not smuggle this into
a generic 'predicate layer' task." And: "If choosing replayable decision, do not call
it the predicate layer yet. Keep it to deterministic boolean decision record/replay with
no schema catalog, oracle, cooldown, or predicate taxonomy."

The three charter candidates below are evaluated against these three options.

---

## Charter A — Generative HTML IR (closes G1: C10 FAIL)

**Working title**: "Generative HTML Intermediate Representation — Phase 1"

**What it does**: Define a `GenerativeHtmlIr` JSON schema representing the high-level
structure of a generated HTML artifact (sections, components, state, data bindings).
Modify the generate path to emit IR JSON first, then render IR to HTML. Write IR CID
into `GenerationAttemptCapsule` as a new tail-additive field. Add an `ir_to_html` Rust
renderer and a corresponding test gate.

**Gap closed**: G1 (C10 FAIL — no IR). Also partially closes G3 (C1 WARN — generate
prompt becomes a template over the IR schema, making it versioned).

**Risk class**: Class 2 (new schema string + additive field in `GenerationAttemptCapsule`
+ new module `src/runtime/generative_html_ir.rs`). No typed_tx change. No Class 4 surface.

**Relation to LATEST.md options**:
- **Orthogonal** to OS sandbox phase 1 (different surface).
- **Orthogonal** to P7.z truthfulness follow-up (which was about truthfulness of existing
  capsule fields, not about adding new IR).
- **Complementary** to replayable-decision smoke test: if a replayable-decision test
  runs against IR mutations, it is much richer than testing against raw HTML regeneration.

**Recommendation**: This is the highest-priority Software 3.0 charter. C10 is a FAIL —
the only FAIL that is purely a missing feature (C8 is also FAIL but is deferred by track H).
The IR gives TuringOS a structural advantage over all three commercial comparators (v0,
bolt.new, html-anything) which use implicit or locked-template IRs. With an explicit
CAS-resident IR, TuringOS can offer something none of the comparators have: a formally
auditable, content-addressed intermediate representation for every generated artifact.

---

## Charter B — Autonomy Slider + Web Driven-Mode Default (closes G3, G5: C1/C2/C9 WARN)

**Working title**: "Web Driven-Mode Promotion — Phase 7 Web +1"

**What it does**:
1. Expose the `--mode driven` grill in the web frontend as a user-selectable option
   (toggle: "固定8题 / AI自由提问").
2. Extract the generate-side system prompt to `assets/prompts/generate_system_v1.md`,
   hash it, write `system_prompt_template_hash` into `GenerationAttemptCapsule` (tail-
   additive field; no schema version bump).
3. Expose `max_generate_attempts` as a user-configurable web parameter (default 3,
   range 1-5) in the generate request body.

**Gap closed**: G3 (C1/C2 WARN — generate prompt unversioned, web defaults to static),
G5 (C9 WARN — no autonomy slider).

**Risk class**: Class 1-2. Frontend: Class 1 (UI toggle). Generate prompt extraction:
Class 1 (asset file + hash field, tail-additive). max_generate_attempts: Class 2
(web API surface change).

**Relation to LATEST.md options**:
- **Orthogonal** to OS sandbox phase 1.
- **Complementary** to replayable-decision smoke test: if decisions include "driven vs
  static mode choice," the slider provides the variation surface for the smoke test.
- **Supersedes** P7.z truthfulness follow-up on the generate-prompt-hash dimension —
  adding `system_prompt_template_hash` to the generation capsule is exactly the kind of
  truthfulness tightening P7.z was doing for other fields.

**Recommendation**: This is a high-value, low-risk increment. The driven grill is already
shipped in the CLI and the web handler; promoting it to web default is 80% done. The
generate prompt hash is a 50-LoC additive change. Together they move C1 from WARN to PASS
and C2 from WARN to PASS on the generate side, without touching any Class 3-4 surface.
This is the "smallest next increment" that closes the Software 1.5 critique of the current
web flow.

---

## Charter C — Spec-Faithful Evaluation + Sandbox Static Analysis (closes G4, G6: C6/C11 WARN)

**Working title**: "Layered Eval Phase 1 — Spec Faithful + Sandbox Heuristic"

**What it does**:
1. Add `TestScenario::SpecFaithful` — an optional Blackbox LLM judge call that scores
   whether the generated HTML matches the spec.md requirements. Result stored in
   `TestRunCapsule` but not used as a generation gate (non-deterministic; evidence only).
2. Add a static-analysis predicate in `src/web/verify.rs` that flags external network
   calls (fetch to non-localhost, XMLHttpRequest to external hosts, external `<script
   src>`). This is a heuristic gate that raises the sandbox boundary floor.

**Gap closed**: G6 (C11 WARN — no LLM-as-judge layer), G4 (C6 WARN — sandbox boundary
not mechanically enforced). Partially closes G4 (no full OS hermetic sandbox, but the
static analysis raises the floor).

**Risk class**: Class 2.
- SpecFaithful: additive `TestScenario` variant + one Blackbox LLM call per session.
  Follows hidden-oracle discipline (scenario set CID not propagated to generation prompt).
- Sandbox static analysis: additive check in `src/web/verify.rs`, parallel to existing
  heuristic checks. No new capsule types.

**Relation to LATEST.md options**:
- **Complementary** to OS sandbox phase 1: the static analysis predicate is the
  application-layer complement to OS-level sandboxing. It does not substitute for bwrap/
  seccomp but it does raise the floor before that work ships.
- **Orthogonal** to replayable-decision smoke test.
- **Supersedes** some P7.z truthfulness follow-up work on the capability boundary dimension.

**Recommendation**: Lower priority than A and B. The SpecFaithful judge is useful but
non-deterministic by nature — it cannot be a gate. The sandbox static analysis is
valuable but limited (heuristic-only). This charter makes sense as a follow-on after
A or B ship.

---

## Comparison and Priority Order

| Charter | Gaps closed | Risk class | Relation to LATEST.md | Recommended order |
|---------|-------------|------------|----------------------|-------------------|
| A — Generative HTML IR | G1 (C10 FAIL) + partial G3 | Class 2-3 | Orthogonal to all 3 options | 1st (highest-impact FAIL closure) |
| B — Autonomy Slider + Web Driven | G3 (C1/C2 WARN) + G5 (C9 WARN) | Class 1-2 | Supersedes P7.z on prompt truthfulness | 2nd (lowest risk, high Software 3.0 signal) |
| C — Layered Eval + Sandbox Heuristic | G4 (C6 WARN) + G6 (C11 WARN) | Class 2 | Complementary to sandbox phase 1 | 3rd (after A or B) |

**Top recommendation**: Charter A (Generative HTML IR) for the following reason:
C10 (IR-grounded reversibility) is a FAIL, not a WARN. It is the single deepest
structural gap that all commercial comparators (html-anything, v0, bolt.new) have
addressed at least partially. Closing it gives TuringOS a unique capability: a formally
auditable, content-addressed IR that is neither purely locked templates (html-anything)
nor an implicit structured tag format (bolt.new) — it is an explicit CAS-resident schema
linking spec → IR → artifact, with predicate gates at each step. This directly supports
TuringOS's positioning as a "protocol-level accountability substrate" and differentiates
it from every commercial comparator on the dimension they are weakest: auditability.

Charter B is the "smallest next atom" if the team wants to avoid Class 3 work. It takes
the driven grill (already shipped in CLI) to the web layer where real users are. The
generate-prompt hash closes a P7.z-style truthfulness gap identified in this audit. Both
sub-tasks are Class 1-2.

The LATEST.md option "replayable-decision smoke test" is **orthogonal** to all three
charters. It can run in parallel with any of them without path overlap. If that smoke
test is chosen over A or B, it does not close any of the six Software 3.0 gaps identified
here — it is constitutional harness work (Class 0-1), not a Software 3.0 feature increment.
