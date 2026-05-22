# Software 3.0 Conformance Verdict Table — TuringOS Generative HTML

**Audit date**: 2026-05-22
**Branch**: `claude/generative-html-kernel-probe-20260522`
**Pipeline scope**: SpecCapsule -> GenerationAttemptCapsule -> ArtifactBundleManifest
-> PreviewRunCapsule -> TestRunCapsule -> GenerateRejectionCapsule

**Verdict key**:
- PASS — concrete file:line evidence confirms conformance
- WARN — partial conformance; gap exists and is named
- FAIL — non-conforming; kill condition met
- UNKN — evidence insufficient to verdict

---

| # | Criterion | Verdict | Evidence (file:line or URL) | One-line rationale |
|---|-----------|---------|----------------------------|--------------------|
| C1 | Prompt-as-Program | WARN | `src/runtime/grill_envelope.rs:17-39` (CANONICAL_SLOTS), `src/bin/turingos/cmd_spec.rs:73-80` (--mode static/driven), `assets/prompts/grill_meta_v1.md` (referenced but not read in audit) | The driven grill (--mode driven) correctly uses a hashed meta-prompt as the program; BUT the static mode (default for web spec.rs submissions) uses 8 hardcoded Rust strings (`SPEC_QUESTIONS_ZH` in `src/web/spec.rs:73-80`) as fixed questions, not as a prompt-driven program. The generate side uses a hardcoded system prompt in cmd_generate.rs — that prompt is not content-addressed or versioned in any capsule. |
| C2 | LLM-as-Runtime | WARN | `src/bin/turingos/cmd_spec.rs:19-25` (driven mode), `src/web/spec.rs:609-613` (W7 turn handler), `src/bin/turingos/cmd_generate.rs:1-57` | Driven grill: LLM controls turn selection and termination (predicate-gated). PASS for grill in driven mode. Generate side: LLM receives spec.md and produces files; but all retry decisions, verification, and artifact selection are made by Rust code in `src/web/generate.rs:226-444`. The LLM is a text emitter; Rust is the decision-maker on the generate path. |
| C3 | NL as Primary Surface | WARN | `src/web/spec.rs:73-80` (8 static questions), `src/runtime/grill_envelope.rs:17-39`, `frontend/src/components/spec-grill.ts` | Static mode (shipped default for web): NL input is funnelled through 8 pre-defined slot questions, meaning user's natural expression must fit the question frames. Driven mode (--mode driven): NL-first. The web frontend uses static mode per `src/web/spec.rs:73` constant. Driven mode exists in CLI but web integration uses static path as primary. |
| C4 | Non-Determinism Tolerance | PASS | `src/runtime/generation_attempt.rs:27-37` (`raw_output_cid`, `prompt_hash`), `src/runtime/artifact_bundle.rs:33-44` (`generation_attempt_cid`), `src/runtime/rejection_capsule.rs:37-50` (`private_diagnostic_cid`) | Every generation attempt writes a `GenerationAttemptCapsule` with `raw_output_cid` (CAS-resident LLM output bytes) and `prompt_hash` (sha256 of canonical request). Rejection capsule links to the attempt. Replay reads anchored output, not re-invoking LLM. Evidence chain: C2 -> C3 -> C5 capsule chain is complete. |
| C5 | Predicate-Gated Admission | PASS | `src/runtime/grill_predicates.rs:1-60` (P1-P6 predicates), `src/runtime/rejection_capsule.rs:20-50` (RejectClass enum), `src/web/generate.rs:387-443` (heuristic gate + GenerateAttemptFailed broadcast), `src/runtime/test_run.rs:73-197` (3 scenario types) | Three predicate layers exist: (a) grill predicates P1-P6 gate LLM turn output; (b) heuristic verify (`verify_artifact_html_with_mode`) gates generated HTML; (c) TestRunCapsule (C11) runs 3 deterministic scenarios. Rejection is typed via `RejectClass` enum and capsule-anchored via `write_generate_rejection_capsule_observed`. |
| C6 | Capability Boundary Explicit | WARN | `src/web/generate.rs:57-66` (MAX_GENERATE_ATTEMPTS=3), `src/sdk/sanitized_runner.rs:12-13` (NetworkPolicyClaim::NotEnforced), `handover/ai-direct/LATEST.md:134-148` ("Active Non-Claims") | The LATEST.md non-claims section explicitly states no OS-level hermetic sandbox, no DenyAll network. `NetworkPolicyClaim::NotEnforced` is encoded in the runner. BUT: the capability boundary for the generated artifact itself (what HTML features are allowed/forbidden, what JS APIs are safe) is NOT mechanically enforced. The iframe sandbox attribute is documented but not verified to be correct in the generated output by any predicate (TestRunCapsule's `SandboxPolicyPreserved` scenario only checks that the attribute string appears — not that it is correctly applied). |
| C7 | Tape-First Evidence + Replay | PASS | `src/runtime/spec_capsule.rs:44` (SPEC_CAPSULE_SCHEMA_ID), `src/runtime/generation_attempt.rs:7` (GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID), `src/runtime/artifact_bundle.rs:7` (ARTIFACT_BUNDLE_SCHEMA_ID), `src/runtime/preview_run.rs:15` (PREVIEW_RUN_CAPSULE_SCHEMA_ID), `src/runtime/test_run.rs:19` (TEST_RUN_CAPSULE_SCHEMA_ID), `src/runtime/rejection_capsule.rs:18` (GENERATE_REJECTION_CAPSULE_SCHEMA_ID) | All six capsule types in the pipeline are CAS-resident via `ObjectType::EvidenceCapsule + schema_id`. The chain is: SpecCapsule -> GenerationAttemptCapsule.spec_capsule_cid -> ArtifactBundleManifest.generation_attempt_cid -> PreviewRunCapsule.artifact_bundle_cid -> TestRunCapsule.artifact_bundle_cid. GenerateRejectionCapsule.generation_attempt_cid links failures. The chain is traversable; `src/bin/turingos/cmd_replay.rs` implements replay without LLM re-invocation. |
| C8 | Agent-Writable Long-Horizon Memory | FAIL | `src/runtime/prompt_capsule.rs:1-59` (PromptCapsule is audit-only), `handover/research/interaction_substrate/30_frontier/track_h_software_3.md:127-129` | No cross-session agent-writable memory exists. PromptCapsule records what the agent saw (audit trail) but agents cannot query or append to it. Track H explicitly identifies this: "TuringOS 缺什么: 没有 archival_memory_* 等价" and "没有 self-editing memory tool." Each session starts cold. This is the deepest Software 3.0 gap in the system. |
| C9 | Partial Autonomy Slider | WARN | `src/web/generate.rs:66` (MAX_GENERATE_ATTEMPTS=3 constant), `src/bin/turingos/cmd_spec.rs:73-80` (--mode static/driven flag), `handover/ai-direct/LATEST.md:152-160` (next-work options include "replayable decision smoke test") | Some autonomy axis exists: --mode static vs --mode driven (CLI only); MAX_GENERATE_ATTEMPTS is configurable via constant but not user-exposed. No runtime autonomy slider. The user cannot configure "how much the LLM decides" at runtime. The driven mode (more autonomous grill) is CLI-only; web defaults to static. No explicit partial-autonomy design surface exposed to end users. |
| C10 | IR-Grounded Reversibility | FAIL | `src/web/ir.rs:1-227` (IRRoot/Block types are a dashboard IR, NOT a generative-HTML IR), `src/bin/turingos/cmd_generate.rs:1-57` | The `src/web/ir.rs` module defines an IR for the TuringOS dashboard (AgentCard, TaskCard, EventLog blocks — Lean market data) — NOT for generated HTML artifacts. Generated HTML has no intermediate representation. The output of `turingos generate` is a raw index.html; mutations require full regeneration. html-anything uses 75 locked skill templates as its IR; TuringOS has no equivalent. |
| C11 | Layered Evaluation | WARN | `src/web/verify.rs` (heuristic HTML checks), `src/runtime/test_run.rs:73-197` (3 scenarios: EntrypointExists, HtmlParses, SandboxPolicyPreserved), `src/web/generate.rs:387-443` (VerifyMode::MinimumBar / GameShape) | Two layers exist: (a) heuristic structural checks in `src/web/verify.rs` (DOCTYPE, game-shape keywords, keyboard listener location, etc.); (b) 3 deterministic TestRunCapsule scenarios. Missing: LLM-as-judge layer for subjective quality (layout coherence, spec faithfulness, UX). Track H names this as a gap: "没有 LLM-as-judge 路径." Single-shot quality for non-standard spec types (productivity tools, forms, charts) is unverified. |

---

## Summary counts

| Verdict | Count | Criteria |
|---------|-------|----------|
| PASS | 3 | C4, C5, C7 |
| WARN | 5 | C1, C2, C3, C6, C9, C11 (6 actual) |
| FAIL | 2 | C8, C10 |
| UNKN | 0 | — |

Wait — recounting: PASS=3 (C4, C5, C7), WARN=6 (C1, C2, C3, C6, C9, C11), FAIL=2 (C8, C10).

**Software 3.0 conformance**: Partial. The evidence/tape substrate (C4, C5, C7) is
strong and exceeds commercial comparators. The generative pipeline's LLM-runtime role
(C2), NL surface (C3), IR reversibility (C10), and cross-session memory (C8) are the
primary gaps.
