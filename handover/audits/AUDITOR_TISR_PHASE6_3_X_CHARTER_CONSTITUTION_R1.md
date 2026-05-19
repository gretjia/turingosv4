# AUDITOR_TISR_PHASE6_3_X_CHARTER_CONSTITUTION_R1

Auditor: clean-context Opus xhigh, charter pre-flight audit
Date: 2026-05-18
Target: handover/directives/2026-05-18_TISR_PHASE6_3_X_GRILL_LLM_DRIVEN_SECTION8_PACKET.md
Predecessor: none (first audit of this charter)

## Verdict: CHALLENGE

The charter is constitutionally clean at the Class-4 surface boundary and FC1
anchoring level, but contains three CHALLENGE-class defects that must be fixed
or explicitly ratified before §8 signature, plus several non-blocking observations.
None of the defects are VETO-grade (no inherent constitution violation), but each
relies on a non-thinking sub-agent honoring an under-specified contract that the
constitution treats strictly.

## Critical violations (block §8 signature)

(none — there is no clean VETO. See Challenges below.)

## Challenges (require fix or explicit forward-defer with architect rationale)

- **[C1] AttemptOutcome misuse (LeanPass / ParseFail re-purposed for grill
  predicate verdict)** — charter:939-942 instructs W4 to set
  `AttemptOutcome::LeanPass` for a predicate-accepted turn and
  `AttemptOutcome::ParseFail` for a parse failure. The W4 brief calls this a
  "re-purpose per Researcher B §7 note." `src/runtime/attempt_telemetry.rs:148-156`
  fixes the discriminant semantics: LeanPass = 0 is Lean-verifier-accepted,
  ParseFail = 2 is Lean-prefix-parse-fail. The discriminants are
  byte-stable per `attempt_telemetry.rs:1050-1056` (a Class-4-adjacent
  invariant locked by tests). Re-purposing the semantic meaning of a typed
  enum without extending it would silently violate CLAUDE.md §14
  "Predicate / Oracle Rules: Partial verdicts must be typed. Do not allow
  ambiguous states." It also creates a downstream replay-divergence risk:
  any audit that interprets `outcome=LeanPass` will now see grill turns
  conflated with real Lean-verified attempts, breaking
  `evaluator_reported_completed_llm_calls = l4_work_attempt_count +
  l4e_work_attempt_count + capsule_anchored_attempt_count` semantics that
  CLAUDE.md §6 + the OBS_TB18R_INV1_NONLLM_TX 2026-05-07 clarification
  pinned down. Either (a) extend `AttemptOutcome` with a typed variant
  (e.g. `GrillTurnAccepted` / `GrillTurnParseFail`) — but that is a
  **Class-4 typed-tx-schema change** explicitly forbidden by charter §3 and
  AGENTS.md §6; or (b) demote grill-turn telemetry to a new schema_id that
  does NOT use `AttemptOutcome` at all (Researcher B's intent), but the
  charter text instructs the opposite.

- **[C2] PromptCapsule construction in W4 uses `hidden_fields_redacted = false`
  which the constructor refuses** — charter:932 says
  "`hidden_fields_redacted` = false." `src/runtime/prompt_capsule.rs:200-209`
  documents: "Constructor refuses `hidden_fields_redacted == false` per
  architect §4.3 — this is the Class-3 invariant." A non-thinking Sonnet
  sub-agent given this brief verbatim will produce code that either (a)
  panics at runtime on every grill turn, or (b) bypasses the constructor —
  which would itself be a CLAUDE.md §4.3 + Art. III prompt-persistence
  violation. The charter must specify `hidden_fields_redacted = true` AND
  state which redaction policy applies (the architect-pinned 7 fields say
  redaction must be declared per turn). Cite charter §1 W4 line 932 vs
  CLAUDE.md §4.3 G-016/G-019/G-021/G-028 default + prompt_capsule.rs:200-209
  constructor invariant.

- **[C3] Termination predicate is a 2nd oracle that needs Class-4 framing
  ratification** — charter §1 (lines 62-64) defines a kernel-side
  termination predicate **separate from P1-P6** that gates whether
  LLM-emitted `done=true` actually terminates. Art. I.1 + CLAUDE.md §14
  says predicates are Boolean and route to L4/L4.E. The charter's
  6 turn predicates (P1-P6) are run per turn, and predicate-fail routes
  to retry/halt — that is FC1-N11/N12 compliant. But the termination
  predicate operates over the **session aggregate** (covered_slots ⊇
  REQUIRED_SLOTS ∧ confidence ≥ 0.8 ∧ turn ≥ 4) and gates session-level
  termination, not per-turn admission. This is novel framing not
  obviously covered by Art. I.1; it more closely resembles an
  ArchitectAI-style "is this run complete" oracle (FC3-N32/N33). The
  cleanest reading is that this is an additional Class-3 predicate, not
  a constitution violation — but its framing is under-specified relative
  to AGENTS.md §3 ("state which FC nodes or invariants the change
  touches"). Charter §2 lists FC1-N9 for P1-P6 but does NOT list an FC
  node for the termination predicate. Architect must either (a) ratify
  the termination predicate as a session-aggregate Class-3 predicate
  with explicit FC1-N9 or new node mapping, or (b) reframe as a
  non-predicate guard.

## Non-blocking concerns / observations

- **[O1] AttemptOutcome::LeanPass discriminant lock**:
  `attempt_telemetry.rs:1050-1056` has hard `assert_eq!` tests that lock
  the byte-stable discriminants. The W4 reuse risks future drift if
  Researcher B's §7 note is ever rolled back — adds invisible coupling
  between grill code and the Lean evaluator's typed-tx schema. Audit
  trace would become harder to reason about.

- **[O2] W2 brief specifies an import path that may not exist as written**:
  charter:773 says `use crate::bin::turingos::grill_envelope::{...}` —
  but `cargo` treats `src/bin/turingos/` items as a binary crate, not
  `crate::bin::turingos::*`. A Sonnet sub-agent following the W3 brief
  may need to either (a) hoist the types into the library crate, or
  (b) duplicate them. The brief itself says "adjust import path based
  on actual module layout — inspect first" but the orchestrator should
  pre-flight this rather than relying on the sub-agent's inspection.

- **[O3] Verbatim message-array bytes stored in CAS via `visible_context_cid`
  is allowed but should be policy-named**: charter:934 stores raw message
  array bytes under `visible_context_cid`. Per CLAUDE.md §4.3 verbatim
  prompt is NOT in canonical tape (L4); CAS via PromptCapsule indirection
  is the allowed pattern. The W4 brief follows the allowed pattern, but
  the policy_version string `"grill_meta_v1"` / `"complete_v1"` should be
  explicit — confirm with architect that this naming is part of §1
  scope and not a free choice for the sub-agent.

- **[O4] Charter §6a verifier model says Opus**, but the W9 brief in §9
  is consistent with that. **§9.W10 also says Opus xhigh**, which
  contradicts AGENTS.md §9 default ("one clean-context Codex audit").
  Phase 7 W8.2 precedent (`AUDITOR_TISR_PHASE7_W8_2_ROUND4_PROCEED.md`
  line 1) is a Claude auditor, so the precedent is mixed. Charter §13 Q10
  surfaces this for architect decision — acceptable as long as architect
  affirmatively chooses Opus.

- **[O5] Sub-atom forbidden-file regex (R-11 mitigation) is not in the
  charter**: charter §10 R-11 says "Orchestrator runs `git diff --name-only`
  after each atom and aborts if forbidden files appear" but no regex is
  given. Specify the exact pattern (see below in Recommended actions).

- **[O6] Session prune at 30 min wall clock (charter §7) is correct
  per Art. 0.2 because canonical state lives in CAS**, but the charter
  should make explicit that pruning a session from `AppState.sessions`
  does NOT prune any CAS object (the CAS objects are the canonical
  replay source). Currently §7 says only "Idle sessions pruned from
  AppState.sessions" which is correct but understated.

- **[O7] LLM economy cost (DeepSeek-V3.2 up to 15 calls/session, plus
  synthesis)** is borne by the operator; user pays no Coin. This is
  consistent with CLAUDE.md §13 Laws (Information is Free / Only
  Investment Costs Money — the user gets free reads/grill conversation;
  no Coin moves). Acceptable.

- **[O8] Cargo.toml unchanged (charter §4 explicit)**: this is correct
  because reqwest/serde_json/axum/tokio are already in workspace deps.
  Verified at `/Users/zephryj/work/turingosv4/Cargo.toml` (no new deps
  needed for the W2-W8 work as described).

## Per-checklist verdicts

### A. Class-4 surface scrupulousness

- **A1**: PASS — charter §3 (lines 144-153) lists `src/state/sequencer.rs`,
  `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs::ObjectType`,
  `src/runtime/prompt_capsule.rs::PromptCapsule` struct,
  `genesis_payload.toml`, canonical signing payload, `src/kernel.rs`,
  `src/bus.rs`, `src/sdk/tools/wallet.rs`,
  `attempt_telemetry.rs::AttemptKind`/`AttemptOutcome` enums. This is a
  proper superset of CLAUDE.md §12 STEP_B + AGENTS.md §6 Restricted
  Surfaces.

- **A2**: PASS — charter §4 (lines 162-223) explicitly enumerates the
  allowed surface and explicitly states Cargo.toml, Cargo.lock,
  genesis_payload.toml NOT touched (lines 219-223). The list excludes
  every Class-4 file from §3.

- **A3**: CONCERN — three sub-atom briefs contain language that could
  let a non-thinking Sonnet/Haiku drift into Class-4 territory:
  (i) **W4 (charter:939-942)** instructs re-use of `AttemptOutcome` enum
  values via re-purpose — see [C1] above. Sub-agent will not realize
  this is a typed-schema concern.
  (ii) **W4 (charter:932)** says `hidden_fields_redacted = false`, which
  the constructor refuses — see [C2]. Sub-agent may bypass constructor
  and accidentally edit `prompt_capsule.rs` (Class-4).
  (iii) **W3 (charter:773)** import path is ambiguous between bin and
  library crate. Sub-agent could "fix" by moving `grill_envelope.rs` to
  `src/runtime/` (not Class-4 but outside Allowed Paths §4) or by
  modifying `prompt_capsule.rs` (Class-4).

- **A4**: PASS — verified via grep:
  `src/bottom_white/cas/schema.rs:44-126` defines `ObjectType` with
  existing variants `EvidenceCapsule` / `PromptCapsule` / `AttemptTelemetry`
  / etc. The W5 brief (charter:1037 / 1062) explicitly says "The CAS
  write call uses `ObjectType::EvidenceCapsule` with the schema_id
  constant" — reusing existing variant. No new ObjectType variant added.
  Charter §3 line 148 forbids new variants and W5 honors this.

### B. FC1 invariants

- **B1**: PASS — every LLM call in driven mode produces all three:
  PromptCapsule + AttemptTelemetry + EvidenceCapsule (turn capsule) per
  charter §1 I-1 invariant (line 122) and W4 brief (charter:937-944).
  CLAUDE.md §3.1 + §6 satisfied for the externalized-attempt rule.

- **B2**: PASS — invariant I-1 (charter:122) correctly states FC1
  anchoring at the per-turn level. FC1-N12 (L4 accepted via
  `explicitly_anchored_capsule_attempt_count` bucket, charter:115)
  matches CLAUDE.md §3.1 "high-volume evidence → CAS EvidenceCapsule
  + L4 anchor."

- **B3**: CONCERN — the charter routes all grill turns to
  `capsule_anchored_attempt_count`. CLAUDE.md §6 reaffirmed in
  OBS_TB18R_INV1_NONLLM_TX 2026-05-07: the LHS scope is
  `tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err`. Grill
  turns are NOT in `tool_dist` — they are not Lean evaluator cycles.
  Per the explicit clause in CLAUDE.md §3.1 ("explicitly counted in an
  anchored EvidenceCapsule"), CAS-only routing is allowed. **But** the
  charter does not state how `attempt_count_equality_report` (CLAUDE.md
  §17) will tally these. The grill flow must produce a separate
  attempt-count tally that does NOT mix with the Lean evaluator's
  externalized-attempt count, else CLAUDE.md §6 equality will appear
  to break on benchmark runs. Charter §6 step 4 lists `cas_walk_output.txt`
  but does not name an `attempt_count_equality_report` field for grill.
  Architect should pre-spell the tally semantics.

- **B4**: PASS — W6 atom-1.d (charter:1120-1123) specifies the
  predicate-fail path retries once then halts with
  `termination_reason = "predicate_double_fail"`. The session capsule
  records this. This is L4.E-equivalent evidence per CLAUDE.md §5.2.
  However, charter does not state whether the failed attempt also writes
  a turn capsule with `predicate_verdicts.failure_reasons` populated, or
  whether the failure path silently overwrites — W6.d "halt session
  with typed error" is the explicit path; this is acceptable for v1.

### C. FC2 invariants

- **C1**: CONCERN — charter I-8 (line 129) claims "replay walks per-turn
  capsules without invoking SiliconFlow API." This is correct in
  structure: replay reads parsed payload from CAS (PromptCapsule.visible
  _context_cid + GrillTurnCapsuleBody.turn_payload_snapshot per W5 line
  1009). LLM nondeterminism is not invoked at replay because the
  parsed envelope is stored, not the LLM seed. However, the charter
  does NOT pin the LLM model version into the session capsule. If the
  upstream model changes (DeepSeek-V3.2 → V3.3), and a debugger later
  tries to "re-invoke" by changing replay semantics, the audit chain
  drifts. This is forward-debt, not a current violation. Acceptable.

- **C2**: PASS — `AppState.sessions: HashMap<SessionId, GrillSession>`
  is process-local memory but is NOT canonical state. The W7 brief
  (charter:1208-1228) places coverage_state, last_3_turns, terminated,
  parent_turn_cid all in memory, but the same data is also written to
  GrillTurnCapsuleBody per turn (W5 line 1001-1011). Canonical state
  is the CAS chain of turn capsules; AppState.sessions is a derived
  view, restartable from CAS. Charter §11 line 1487 reinforces this
  ("new schemas coexist with existing capsules but are never read by
  legacy code paths"). CLAUDE.md §16 "memory-only canonical state" is
  not violated. Note that CLAUDE.md §12 strict reading would catch a
  case where AppState.sessions becomes the only source — the charter
  must ensure no path reads AppState.sessions without falling through
  to CAS. W7 step 4 ("If session.terminated → return 400") is the
  edge case: if the server crashes and restarts, AppState.sessions is
  empty, but the CAS session capsule already says terminated. Charter
  must either rebuild AppState from CAS at boot or accept that
  in-flight session resumption is lost (acceptable for v1).

- **C3**: PASS — 30-min idle prune in §7 prunes AppState.sessions only;
  CAS objects are immutable per Art. 0.2 ("CAS objects must be reachable
  through ChainTape references"). Pruning a memory-only view loses no
  canonical state because the per-turn capsules + session capsule
  remain CAS-resident. See also [O6] above.

### D. FC3 invariants

- **D1**: PASS — charter does NOT introduce in-runtime ArchitectAI or
  Veto-AI. CONSTITUTION_GAP_ANALYSIS_2026-05-07.md:113 lists Art. V.1.2
  in-runtime ArchitectAI as "architectural choice, not debt." Charter
  §1 line 100 explicitly says "ArchitectAI / Veto-AI in-runtime hooks
  (Phase 11+ deferred per CONSTITUTION_GAP_ANALYSIS_2026-05-07.md:169
  'likely permanent' — architectural choice, not debt)". Consistent
  with EXECUTION_MATRIX §F (line 81 "🟢 GREEN").

- **D2**: PASS — the LLM grill is a content interviewer producing
  spec.md content; it does NOT propose architecture changes, modify
  predicate definitions, or assume Veto-AI authority. The meta-prompt
  per W1 (charter:599-602) is a domain-specific interview prompt for
  Mrs. Chen's product spec generation. No FC3 role overlap.

- **D3**: CONCERN — charter §1 W6 step 2.a says "For turns 2..N: prior
  Q/A pairs as alternating user/assistant messages (only last 3 pairs
  included)" — this honors CLAUDE.md §15 + Art. III.2 (progressive
  disclosure). PromptCapsule.read_set per turn limited to last 3 turn
  cids per charter §2 FC1-N5 (line 111). **However**: W6 step 2.a is
  vague on whether the *full Q/A text content* or *only structural CIDs*
  go into the prompt. The brief uses both phrasings ("Q/A pairs as
  alternating user/assistant messages" suggests text content;
  "PromptCapsule.read_set includes only ... last_3_turn_cids" suggests
  CID indirection). Sub-agent must include the actual content in the
  LLM API call (LLM cannot deref CIDs), but charter must specify that
  the PromptCapsule.read_set is recorded as CIDs while the prompt body
  itself contains text. This is a documentation-level CONCERN, not a
  violation, but a non-thinking sub-agent may mis-build the read_set
  field.

### E. Prompt persistence (CLAUDE.md §4.3)

- **E1**: PASS (with C2 caveat) — W4 (charter:929-936) writes
  PromptCapsule with 7 fields: prompt_context_hash, read_set,
  policy_version, hidden_fields_redacted, visible_context_cid,
  system_prompt_template_hash, agent_view_manifest_cid. These exactly
  match the architect-pinned 7 fields in CLAUDE.md §4.3 + prompt_capsule.rs.
  No struct extension. **BUT** see [C2] above — charter sets
  `hidden_fields_redacted = false` which the constructor refuses.

- **E2**: PASS — charter §1 W4 line 934 says
  "`visible_context_cid` = CAS-store the raw message array bytes; cid hex".
  This is CAS storage with capsule indirection (allowed per CLAUDE.md
  §4.3). Verbatim does NOT go into L4 / canonical tape — it goes into
  CAS via PromptCapsule's `visible_context_cid` field, which the
  constitution permits.

- **E3**: PASS — W4 brief (charter:929-935) constructs PromptCapsule
  via existing helpers; does not extend or mutate the struct.
  Charter §3 line 149 explicitly forbids struct mutation.

### F. Predicate / oracle discipline

- **F1**: CONCERN — `PredicateVerdict { Pass, Fail{reason: String} }` per
  charter W3 line 779-780. Reason is a free `String`, not a typed reason
  class. CLAUDE.md §14 says "Partial verdicts must be typed. Do not
  allow ambiguous states." A free `String` "reason" is closer to "ambiguous
  state" than typed. Compare to `attempt_telemetry.rs::LeanErrorClass`
  enum which TB-18R landed for symmetry. The charter should specify a
  `enum PredicateFailureReason { SchemaParseError, UnknownSlot, ... }`
  with discriminant lock to match the existing typed-error discipline.
  Mitigation: charter §5 line 271 says "tests/grill_predicates_*.rs"
  must cover each predicate × pass/fail; W3 brief lines 840-857 enumerate
  fail-class names. Could be promoted from `String` to enum.

- **F2**: CONCERN — see [C3]. Termination predicate has different
  semantics from P1-P6 and is not mapped to an FC node in charter §2.

- **F3**: PASS — CLAUDE.md §4.2 / G-012 PCP soundness. Invalid LLM
  output cannot enter "accepted state" because P1 (schema parse) +
  P2 (kind ok) gate before any envelope value is acted upon.
  Termination predicate gates session-level termination. Charter §10 R-2
  binds the termination flow. Invalid envelopes route to retry-once-then-halt.
  Acceptable.

### G. Shielding (Art. III, CLAUDE.md §15)

- **G1**: PASS — Art. III.2 progressive-disclosure check satisfied via
  charter §2 FC1-N5 (line 111) restricting per-turn PromptCapsule.read_set
  to `{session_id, last_3_turn_cids, canonical_slot_table_cid,
  system_prompt_template_hash}`. Last 3 turn cids → progressive disclosure.

- **G2**: PASS — charter §10 R-7 line 1473 explicitly says
  "Structured-history view (last 3 Q/A only) excludes rationale_brief
  from next prompt assembly." Art. III.3 lateral-correlation check
  satisfied. W6 step 2.a + W3 + the structured history view in W6
  honor this.

- **G3**: CONCERN — Blackbox triage classifier (Researcher A §2.4 / §5.3)
  is deferred. Charter §10 R-3 says "Per-answer 4096-char cap (existing).
  Blackbox triage deferred to atom-1.5 (v1 ships without)." CLAUDE.md
  §15 says "do not feed raw failure logs / abusive answers into ordinary
  Agent read view." User answers go directly into the LLM Meta prompt
  (via Q/A pair history). 4096-char cap is a length gate, not a content
  shield. Adversarial / abusive content fits in 4096 chars easily and
  will reach the next LLM call. **However**: the Meta-LLM is the only
  consumer of these answers (not "ordinary Agent" — there is no other
  Agent in the loop). The charter's interpretation is that the Meta-LLM
  is itself a shielded consumer (it's the producer of the next turn,
  not a system-level Agent). This is defensible but should be
  ratified by architect — Researcher A §5.3 flags it as Medium-severity
  risk specifically because the user → Meta-LLM channel is the
  injection vector.

- **G4**: PASS — Art. III.3 says "刻意屏蔽个体之间的横向相关性." The
  `session_id` namespace + `is_safe_session_id` validation (charter:1241)
  plus per-process AppState.sessions means cross-session leakage requires
  someone reading the wrong session_id. Adequate for v1.

### H. Economy laws (CLAUDE.md §13)

- **H1**: PASS — Meta-LLM up to 15 calls/session + 1 synthesis call =
  16 calls max. User pays no Coin. Operator absorbs LLM cost. Per
  CLAUDE.md §13 Law 1 ("Information is Free — reads/search/thinking
  do not spend core Coin"). The grill conversation is a "read/think"
  flow, not a "WorkTx submission." No on-chain economic move.
  Structurally OK.

- **H2**: PASS — charter §10 R-5 mitigation is P5 (turn ceiling = 15)
  + P4 (monotonic coverage). These are Class-1 predicates per charter §3
  classifying P1-P6 as Class 1. No economic conservation touched.

### I. Tape / ID canonicality (CLAUDE.md §16)

- **I1**: PASS — `session_id` is opaque, validated by `is_safe_session_id`
  (existing Phase 7 helper at `src/web/spec.rs:424`). It is NOT a
  canonical WorkTx.tx_id derivative. Verified via:
  `src/web/spec.rs:424` defines `is_safe_session_id(s: &str) -> bool`
  per existing pattern. Charter §1 line 1241 reuses this. No canonical
  ID collision.

- **I2**: PASS — schema_ids `turingos-spec-grill-turn-v1` and
  `turingos-spec-grill-session-v1` do NOT collide with existing
  `turingos-spec-capsule-v1` (line 34 of `src/bin/turingos/spec_capsule.rs`).
  grep verified at `src/bin/turingos/spec_capsule.rs:34` +
  `/Users/zephryj/work/turingosv4/tests/cli_phase63_cas_wire.rs:170`.

### J. AGENTS.md §6 restricted surfaces

- **J1**: PASS — charter §4 Allowed Paths (lines 162-223) lists only:
  `src/bin/turingos/cmd_llm.rs`, `cmd_spec.rs`, `spec_capsule.rs`,
  `grill_envelope.rs (NEW)`, `src/runtime/grill_predicates.rs (NEW)`,
  `src/runtime/mod.rs`, `src/web/spec.rs`, `src/web/ws.rs`,
  `src/web/router.rs`, `src/web/mod.rs`. None of `kernel.rs / bus.rs /
  wallet.rs / sequencer.rs / typed_tx.rs / cas/schema.rs / RootBox /
  signing payload` appears. Note: `src/web/ws.rs` is touched
  (AppState extension); this is borderline because AppState is the
  axum dependency-injection surface, but it is NOT a STEP_B Class-4
  surface per CLAUDE.md §12.

- **J2**: PASS — charter §4 line 219-220 says
  "Cargo.toml — UNCHANGED (no new dependencies; everything within
  existing axum/tokio/serde/reqwest stack)." Verified at
  `/Users/zephryj/work/turingosv4/Cargo.toml` (W2-W8 use existing deps).

- **J3**: PASS — charter §4 line 221 says
  "genesis_payload.toml — UNCHANGED (no Trust Root rehash)." Consistent
  with §3 line 150. Per CLAUDE.md §0.2 / AGENTS.md §8 the Trust Root is
  preserved.

### K. AGENTS.md §9 audit default

- **K1**: CONCERN — charter §9 W10 brief specifies `model: opus` for
  audit. AGENTS.md §9 default is "one clean-context Codex audit." The
  Phase 7 precedent (`AUDITOR_TISR_PHASE7_W8_2_ROUND4_PROCEED.md` line 1)
  used Claude auditor. Charter §13 Q10 surfaces this for explicit
  architect decision — acceptable if architect affirmatively chooses
  Opus over Codex. Otherwise the AGENTS.md §9 default should apply.

- **K2**: PASS — charter §9 W10 brief (lines 1392-1402) restricts the
  reviewer to: charter file, git diff, agent_verdict.json, cas_walk_output.txt,
  replay_diff.txt, legacy_byte_compat_hash.txt. Explicitly says "Do NOT
  read the implementation chat transcript" (line 1393). Matches
  AGENTS.md §9 "Do not provide the implementation transcript."

### L. CLAUDE.md §19 no manipulation by sequencing

- **L1**: PASS — relevant load-bearing blockers:
  - HEAD_t C1: 🟢 GREEN (CONSTITUTION_GAP_ANALYSIS line 55)
  - PCP soundness: 🟢 GREEN (synthetic corpus; line 65)
  - PromptCapsule landing: 🟢 GREEN (line 89, Constitution Landing First
    2026-05-07)
  - system tx authorization: 🟢 GREEN
  - tape canonical ID: 🟢 GREEN
  - economic conservation: 🟢 GREEN
  Phase 6.3.x is application-layer wire-up over Phase 7 Web MVP (which
  shipped 2026-05-18). No constitution gate is being "stepped around."

- **L2**: PASS — charter §1 line 17 explicitly REJECTS Researcher C's
  CCS / agent.toml abstraction. The rejection is anchored to the
  "two real instances, then ratify" rule. This is correct sequencing
  per `feedback_class4_cannot_hide_in_class3`-style discipline: don't
  build a generalization before second real case exists. Forward
  defer is correctly flagged in charter §1 line 90-91 and §10 R-8
  line 1474.

### M. Sub-atom non-thinking model risk

- **M1**: CONCERN — three briefs have ambiguity that could mislead a
  non-thinking Sonnet/Haiku:
  (i) W3 import path (charter:773) — "`crate::bin::turingos::*`" is
  non-standard; sub-agent may hoist types to `src/runtime/` (outside
  Allowed Paths) or duplicate them.
  (ii) W4 (charter:932) — `hidden_fields_redacted = false` is the
  refused-by-constructor value (see [C2]).
  (iii) W4 (charter:939-942) — re-purposing AttemptOutcome::LeanPass
  for grill turns is semantically wrong (see [C1]).

- **M2**: CONCERN — Allowed-edits lists in W4-W8 are explicit ("Allowed
  edits: - `src/bin/turingos/cmd_llm.rs` (extend) - `tests/...` (new)
  No other files."). This is good practice. **But** W3 brief
  (charter:861) says "No edits outside: `src/runtime/grill_predicates.rs`
  (new), `src/runtime/mod.rs` (add pub mod line), test files" — "test
  files" is plural without specific name list. A sub-agent could create
  arbitrary test files. Charter §4 lists specific test paths (lines
  194-203) so the orchestrator must cross-check.

- **M3**: CONCERN — charter §10 R-11 mitigation says "Orchestrator runs
  `git diff --name-only` after each atom and aborts if forbidden files
  appear" but does not specify the regex. Per [O5] above, the
  orchestrator should use a regex such as:

  ```
  ^(src/(state/(sequencer|typed_tx)\.rs|kernel\.rs|bus\.rs|sdk/tools/wallet\.rs|bottom_white/cas/schema\.rs|runtime/(prompt_capsule|attempt_telemetry)\.rs)|genesis_payload\.toml|Cargo\.(toml|lock))$
  ```

  Match in `git diff --name-only` → abort atom. Charter should include
  this regex verbatim so the orchestrator is mechanical.

## Recommended pre-signature actions

1. **Fix [C1] AttemptOutcome re-purpose** — Either (a) the grill turn
   capsule does NOT carry AttemptTelemetry at all (use a fresh
   schema_id and a parallel telemetry record that does NOT use
   `AttemptOutcome` enum); or (b) reframe Researcher B §7 as Class-4
   typed-tx extension and dispatch via a fresh §8 amendment before
   W4. Option (a) is preferred — it preserves the Class-4 forbidden
   boundary while letting grill turns have their own typed verdict
   alphabet (PredicateAccepted / SchemaParseFail / TerminationGated).

2. **Fix [C2] PromptCapsule hidden_fields_redacted** — Change W4 brief
   line 932 from `hidden_fields_redacted = false` to
   `hidden_fields_redacted = true` AND add a sentence specifying the
   redaction policy applied (e.g., "no fields redacted because no
   secret-bearing fields exist in the grill prompt; redaction policy =
   `none-applies` per architect-pinned default"). This honors
   `prompt_capsule.rs:200-209` constructor invariant.

3. **Resolve [C3] termination predicate framing** — Add a §2 FC mapping
   row for the termination predicate (suggest FC1-N9 + note "session-aggregate
   variant"). Or reframe as a non-predicate "termination gate" that does
   NOT use the `Predicate` semantic surface.

4. **Add concrete forbidden-file regex** to §9 dispatch protocol step 3
   so R-11 mitigation is mechanical (see M3 above).

5. **Clarify [O3] policy_version naming** — confirm with architect that
   `policy_version = "grill_meta_v1"` is the canonical name; lock it as
   part of §1 scope.

6. **Clarify [O5] / B3 attempt count tally semantics** — specify that
   grill-turn attempts produce a separate `grill_attempt_count` counter
   that is reported alongside but NOT summed into
   `evaluator_reported_completed_llm_calls` (CLAUDE.md §6 LHS).

7. **Promote [F1] PredicateVerdict::Fail reason from String to enum** —
   align with existing typed-error discipline (`LeanErrorClass` /
   `RejectionClass`). Define
   `PredicateFailureClass { SchemaParseError, UnknownSlot,
   NonMonotonic, TurnOutOfRange, LanguageMismatch, ShortQuestion,
   MissingPlayback, ConfidenceOutOfRange }` and gate on it.

8. **Architect explicit decision on K1** — affirmatively choose Opus
   xhigh for W10 (per charter §13 Q10) so the AGENTS.md §9 default
   is overridden by architect ratification, not by silent drift.

9. **Spell out [G3] user-answer shielding rationale** — add a line
   stating "the Meta-LLM is the sole consumer of user answers; no
   other Agent reads them; 4096-char length cap is the v1 shield;
   Blackbox triage atom-1.5 is the next-line defense." This makes
   the Art. III + CLAUDE.md §15 reasoning explicit.

10. **Pre-flight W2 import-path question** — orchestrator should
    inspect `src/bin/turingos.rs` vs `src/bin/turingos/mod.rs` ahead
    of W2 dispatch and rewrite the W3 brief's import-path snippet
    accordingly. Avoids sub-agent guessing.

After items 1-3 are addressed (the three CHALLENGE-class findings), this
charter is constitutionally clean enough for §8 signature. Items 4-10
are recommended hardening to reduce sub-atom risk per AGENTS.md §9
discipline.
