# TISR Phase 6.3.x — Charter R2 Post-Audit Amendments

**Revision**: R2 (delta amendments on top of R1)
**Date**: 2026-05-18
**Base document**: [`2026-05-18_TISR_PHASE6_3_X_GRILL_LLM_DRIVEN_SECTION8_PACKET.md`](2026-05-18_TISR_PHASE6_3_X_GRILL_LLM_DRIVEN_SECTION8_PACKET.md) (1583 lines)
**Audit basis**:
- [`AUDITOR_TISR_PHASE6_3_X_CHARTER_CONSTITUTION_R1.md`](../audits/AUDITOR_TISR_PHASE6_3_X_CHARTER_CONSTITUTION_R1.md) — verdict CHALLENGE (3 must-fix)
- [`AUDITOR_TISR_PHASE6_3_X_CHARTER_KARPATHY_R1.md`](../audits/AUDITOR_TISR_PHASE6_3_X_CHARTER_KARPATHY_R1.md) — verdict PROCEED, rating B (3 advisory)

**Architect mid-flight ratifications (chat 2026-05-18 post-R1-audit)**:
- **K-C2 Blackbox triage**: option **(b)** — promote to atom **W4.5** in this packet. LOC budget +200. Software 3.0 fidelity from B → B+.
- **W10 audit model**: **Claude (Sonnet/Opus xhigh)** — NOT Codex. Both R1 audits and W10 audit use Claude.
- **attempt_count tally**: option **(b)** — independent `grill_attempt_count` counter, parallel to (not summed into) `evaluator_reported_completed_llm_calls`.

This R2 amends R1. R1 sections not listed here are unchanged. When R1 and R2 conflict, R2 wins.

---

## A1. AttemptOutcome re-purpose fix [Constitution C1 — MUST-FIX]

**Problem**: R1 W4 brief (charter:939-942) instructs sub-agent to set `AttemptOutcome::LeanPass` for predicate-accepted grill turns and `AttemptOutcome::ParseFail` for parse failures. `src/runtime/attempt_telemetry.rs:148-186` pins all 7 outcome variants as Lean-evaluator-specific with byte-stable discriminants locked at `attempt_telemetry.rs:1050-1056`. Re-purpose would silently break `evaluator_reported_completed_llm_calls` semantics (CLAUDE.md §6 + OBS_TB18R_INV1_NONLLM_TX) and conflate grill turns with Lean cycles in any future audit grep.

**Resolution**: Grill turns do NOT write `AttemptTelemetry`. They write a grill-specific telemetry record **embedded in the EvidenceCapsule body** (`GrillTurnCapsuleBody`), satisfying CLAUDE.md §3.1 hard invariant via the **third bucket** `explicitly_anchored_capsule_attempt_count` (Researcher B §1.3 + §1.4). The `AttemptTelemetry` struct stays reserved for Lean evaluator cycles; no enum extension required; no Class-4 mutation.

**Constitution reading** (architect ratifies this as §8 interpretation):
> CLAUDE.md §6 "AttemptTelemetry must exist in CAS" applies to Lean-evaluator-driven externalized attempts. Grill turns are application-layer externalized attempts (LLM output used in future prompt context per §6 explicit clause) but route via the CAS-only EvidenceCapsule path. FC1 anchoring is satisfied by the third bucket per CLAUDE.md §3.1 + Researcher B §1.4. This reading is forward-consistent with TB-8+ reclassification anticipated in `attempt_telemetry.rs:179-186`.

### Charter R1 §1 W4 specific edits

R1 W4 lines 929-944 (PromptCapsule + AttemptTelemetry capsule write spec) — REPLACE with:

```
- If --capsule-dir is set, write the following CAS objects per turn:
  1. PromptCapsule via existing `prompt_capsule::write_prompt_capsule_to_cas`:
     - prompt_context_hash = sha256(canonical-encoded message array)
     - read_set = empty Vec for Phase 6.3.x v1 (future atoms may populate
       per Researcher B §4.2: last_3_turn_cids + canonical_slot_table)
     - policy_version = "grill_meta_v1" (strict-json mode)
                       OR "complete_v1" (non-strict mode)
     - hidden_fields_redacted = TRUE      [A2 fix]
     - visible_context_cid = CAS-store raw message-array bytes; cid hex
     - system_prompt_template_hash = sha256(system message content) if any
     - agent_view_manifest_cid = same as visible_context_cid for v1
  2. NO AttemptTelemetry write for grill turns.            [A1 fix]
     FC1 anchoring is via the parent EvidenceCapsule (turingos-spec-grill-
     turn-v1) written in W5, which carries an embedded GrillAttemptRecord
     field (typed verdict; see A6 for schema). This satisfies CLAUDE.md
     §3.1 hard invariant via the third bucket
     `explicitly_anchored_capsule_attempt_count`.
  3. Output stdout JSON shape (R2):
     {
       "ok": true,
       "content": "<verbatim LLM content string>",
       "parsed_envelope": <TurnPayload | null>,
       "usage": {...},
       "finish_reason": "<string>",
       "model": "<string>",
       "prompt_capsule_cid": "<hex>",
       "elapsed_ms": <int>
     }
     // attempt_telemetry_cid removed; not written for grill turns.
```

### Charter R1 §1 W5 specific edits

W5 `GrillTurnCapsuleBody` schema (R1 lines 1001-1011) — REPLACE with:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GrillTurnCapsuleBody {
    pub session_id: String,
    pub turn_index: u32,
    pub prompt_capsule_cid: String,
    pub user_answer_cid: Option<String>,    // None on turn 1
    pub parent_turn_cid: Option<String>,    // None for turn 1
    pub grill_attempt_record: GrillAttemptRecord,    // [A1]
    pub predicate_verdicts: GrillPredicateVerdicts,
    pub turn_payload_snapshot: serde_json::Value,
    pub logical_t: u64,
}

/// Grill-specific telemetry record. Embedded in capsule body; NOT a
/// stand-alone CAS object. Replaces AttemptTelemetry usage for grill turns
/// per Constitution audit R1 §C1 + R2 §A1.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GrillAttemptRecord {
    pub schema_version: u32,                 // = 1
    pub session_id: String,
    pub turn_index: u32,
    pub model_id: String,                     // e.g. "deepseek-ai/DeepSeek-V3.2"
    pub prompt_context_hash: String,          // hex of sha256, matches PromptCapsule
    pub candidate_payload_cid: String,        // parsed turn payload bytes in CAS
    pub outcome: GrillAttemptOutcome,         // [A1] typed grill verdict
    pub token_counts: GrillTokenCounts,
    pub elapsed_ms: u64,
    pub retry_index: u32,                     // 0 for first try, 1 for retry
}

/// Grill-specific outcome class. Discriminants are byte-stable. Tail-additive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum GrillAttemptOutcome {
    /// All 6 turn predicates passed; turn accepted into session state.
    PredicatesPassed = 0,
    /// P1 schema-parse failed; envelope malformed.
    SchemaParseFailed = 1,
    /// P2 kind discriminator failed (done=false + null question, or done=true + null playback).
    KindMismatch = 2,
    /// P3 slot vocabulary mismatch (invented slot id).
    UnknownSlot = 3,
    /// P4 monotonicity violation (covered_slots shrank).
    NonMonotonic = 4,
    /// P5 turn index out of bounds (turn > 15).
    TurnOutOfRange = 5,
    /// P6 language predicate failed (Han-script ratio / ASCII ratio off).
    LanguageMismatch = 6,
    /// LLM API itself errored (network / 5xx / timeout).
    LlmApiError = 7,
    /// Two consecutive predicate failures on same turn; session aborted.
    DoubleRetryFailed = 8,
    /// Termination predicate refused done=true (required slots missing or
    /// confidence too low or turn < 4). Turn loops back to continue interview.
    TerminationGated = 9,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GrillTokenCounts {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
```

### Test additions for A1

Add to `tests/grill_turn_capsule_write_read.rs`:
- `grill_attempt_record_embedded_not_standalone` — verify capsule body has GrillAttemptRecord; verify no AttemptTelemetry written to CAS during driven turn
- `grill_attempt_outcome_discriminant_lock` — assert byte-stable discriminants 0..9
- `grill_attempt_record_independent_of_attempt_outcome_enum` — verify no `use` of `crate::runtime::attempt_telemetry::AttemptOutcome` in new module

---

## A2. PromptCapsule hidden_fields_redacted fix [Constitution C2 — MUST-FIX]

**Problem**: R1 W4 brief charter:932 says `hidden_fields_redacted = false`. The constructor at `src/runtime/prompt_capsule.rs:200-209` documents "Constructor refuses `hidden_fields_redacted == false` per architect §4.3 — this is the Class-3 invariant." A non-thinking Sonnet sub-agent would either panic at runtime or attempt to bypass the constructor (which is itself a Class-4 surface edit).

**Resolution**: Set `hidden_fields_redacted = true` with an explicit redaction-policy note.

### Charter R1 §1 W4 specific edit

(included in A1's replacement block above; specifically line `hidden_fields_redacted = TRUE`)

**Redaction policy** to record in W1 prompt asset doc:

> Phase 6.3.x grill PromptCapsules carry `hidden_fields_redacted = true`. The redaction policy is `"none-applies-grill-v1"`: no fields are actually shielded because the grill prompt envelope contains no secret-bearing fields (no API keys, no user PII beyond what the user voluntarily typed into the answer text, no Lean stderr, no internal tool output). Setting `hidden_fields_redacted = true` honors the `prompt_capsule.rs:200-209` Class-3 constructor invariant; the policy name `"none-applies-grill-v1"` is the audit trail for "we considered shielding and found nothing to shield for this surface."

Add this paragraph verbatim to `assets/prompts/grill_meta_v1.md` as a footer section `### Redaction policy declaration` (W1 brief addendum).

---

## A3. Termination predicate FC mapping [Constitution C3 — MUST-FIX]

**Problem**: R1 §2 maps P1-P6 to FC1-N9 but doesn't map the termination predicate, which operates on session-aggregate state (covered_slots ⊇ REQUIRED_SLOTS ∧ confidence ≥ 0.8 ∧ turn ≥ 4) and gates session-level termination. Art. I.1 / CLAUDE.md §14 needs explicit framing or this looks like a 2nd unmapped oracle.

**Resolution**: Map termination predicate to **FC1-N9 (session-aggregate variant)** with explicit note. It is a Class-3 predicate, not a Class-4 oracle.

### Charter R1 §2 specific edit

R1 §2 "Touched / extended FC nodes" list — INSERT new bullet before `New invariants for Phase 6.3.x`:

```
- FC1-N9 (session-aggregate variant): termination predicate gates LLM-emitted
  `done=true` against required-slot subset {job, anchor, memory, first_run,
  robustness, scope, acceptance} ⊆ covered_slots AND confidence ≥ 0.8 AND
  turn ≥ 4. This is a Class-3 predicate operating over session-aggregate
  state, distinct from P1-P6 which run per-turn. Predicate-fail at the
  termination boundary does NOT route to L4.E; instead it loops the
  interview back ("inject 'You declared done but slot X is missing. Ask
  one more question.' to next LLM call"). This is Researcher A §2.3
  dual-gated termination + Researcher B §5.2 disagreement contract.
  FC1-N9 framing is preserved (predicate fail → backstop action); the
  "session-aggregate" qualifier marks the scope as session-level not
  per-turn.
```

### Charter R1 §2 invariants table specific edit

Add new row to invariants table (after I-6):

| Inv ID | Statement | Test witness |
|---|---|---|
| **I-9** | Grill turns do NOT write `AttemptTelemetry`; FC1 anchoring is via EvidenceCapsule (turingos-spec-grill-turn-v1) with embedded `GrillAttemptRecord`, satisfying CLAUDE.md §3.1 hard invariant via the third bucket `explicitly_anchored_capsule_attempt_count` per Researcher B §1.3 + §1.4. | `tests/grill_turn_capsule_write_read.rs::grill_attempt_record_embedded_not_standalone` |
| **I-10** | Termination predicate enforces 7-required-slot subset AND confidence ≥ 0.8 AND turn ≥ 4 (R1 I-6 expanded with confidence + turn floor). FC mapping = FC1-N9 (session-aggregate variant). Predicate fail loops interview, not L4.E. | `tests/grill_predicates_termination.rs::termination_pass_when_seven_required_and_confidence_high` + `tests/grill_predicates_termination.rs::termination_fail_when_confidence_below_0_8` + `tests/grill_predicates_termination.rs::termination_fail_when_turn_below_4` |

---

## A4. Software 3.0 fidelity honesty [Karpathy K-C1 — ADVISORY]

**Problem**: R1 title and §1 prose call this "Software 3.0 native LLM-driven grill." Karpathy auditor gut-check verdict = (b) Software 2.5 (LLM has turn-content authority; Rust still owns the loop + retry + termination backstop). "Software 3.0 native" overclaims.

**Resolution**: Keep title (zero-friction) but add one-line clarifier in §1 + new row in diff table. This is option (b) from Karpathy auditor's K-C1.

### Charter R1 §1 specific edits

Insert immediately after R1 line 17 (after the scope-decision bullet list):

```
**On the "Software 3.0 native" claim** (per Karpathy audit R1 §VII gut check):
"Software 3.0 native" in this packet refers to **three specific properties** —
(1) per-turn LLM content authority (the LLM, not Rust, decides what to ask
next and how to phrase it); (2) prompt-as-program (interview logic lives in
`assets/prompts/grill_meta_v1.md`, hashed via system_prompt_template_hash,
not in Rust); (3) replay-without-recall (parsed turn payload cached in CAS;
LLM is never re-invoked at replay). It does NOT refer to LLM-as-runtime-
scheduler — the Rust kernel still drives the turn loop, the retry budget,
and the termination backstop. Karpathy's stronger "LLM is the kernel"
framing is a Phase 6.3.z target, not this packet's promise. This packet
is honestly **Software 2.5 with all three above Software-3.0 properties
landed**; v1 fidelity B+ once W4.5 Blackbox triage lands per A5.
```

### Charter R1 §1 diff table specific edit

Insert new row at the END of the table (after row "Frontend"):

| Surface | Phase 6.3 (today) | Phase 6.3.x (this packet) |
|---|---|---|
| **Loop driver** | Rust (`for i in 0..8` over hardcoded array) | **Rust (unchanged in v1)** — Software-2.5 boundary; Software-3.0-z would move loop ownership to LLM |
| **Retry policy owner** | n/a (no LLM in loop) | **Rust** — Software-2.5 boundary |
| **Termination authority** | `i == 8` (Rust) | **Dual-gated**: LLM emits `done=true`, Rust kernel backstop via termination predicate |

---

## A5. Promote Blackbox triage to atom W4.5 [Karpathy K-C2 — Architect ratified (b)]

**Architect decision**: option **(b)** — Blackbox triage is in-scope for this packet as new atom **W4.5**. LOC budget bumps from ~900/1500 cap to **~1100/1700 cap**.

### Charter R1 §1 specific edit

In the "Phase 6.3.x ratifies six artifact additions" list, INSERT new artifact #2 (shifting current 2-6 to 3-7):

```
2. **Blackbox triage classifier on user input** (extending `src/bin/turingos/cmd_llm.rs`)
   - Per-answer user input first passes through Qwen3-Coder-30B (Blackbox model,
     existing in `cmd_llm.rs::read_blackbox_model`) for content classification
   - Output: one of `{relevant, off_topic, abusive, gibberish}` (50-token cap;
     ~5x cheaper than burning Meta on triage per Researcher A §2.4)
   - If `relevant`: user answer feeds into Meta-LLM next-turn prompt as-is
   - If `off_topic`: kernel injects a gentle nudge ("能换一种说法吗？刚才听
     不太懂") + re-renders same turn's question; counts toward turn budget
   - If `abusive` or `gibberish`: kernel does NOT pass raw answer to Meta;
     re-prompts user with "您似乎在测试我，可以继续吗？" + pause flag in
     GrillSession; two consecutive non-relevant → session abort with
     termination_reason = "user_input_unparseable"
   - All triage calls anchored via the same GrillAttemptRecord pattern as
     Meta turns (model_id = blackbox_model_name; outcome reflects triage
     verdict in a new GrillAttemptOutcome::TriageNonRelevant = 10 variant
     tail-appended)
   - This is Researcher A §2.4 multi-LLM scheduler/worker pattern landing
     in v1; Karpathy K-C2 fidelity upgrade from B → B+
```

### Charter R1 §9 specific edit — insert W4.5 atom

Between R1 W4 (cmd_llm complete) and W5 (spec_capsule writers), INSERT:

```
---

### W4.5 — cmd_llm.rs `triage` action + Blackbox integration

**Agent role**: Sonnet 4.6
**Depends on**: W4 (cmd_llm complete)
**Risk class**: Class 2
**LOC**: ~150 source + ~120 tests

**Brief**:

> Extend `src/bin/turingos/cmd_llm.rs` with a second sub-action `triage`.
> Like `complete`, it's a thin wrapper around `siliconflow_client::chat_complete`
> but uses the Blackbox model (Qwen3-Coder-30B) for cheap classification of
> user input.
>
> CLI surface:
>
> ```
> turingos llm triage
>     --workspace <PATH>             # required
>     --user-answer <STRING>         # or read from stdin if --user-answer=-
>     --lang <zh|en>                 # for classifier prompt phrasing
>     --capsule-dir <PATH>           # optional; writes PromptCapsule
>                                    # + GrillAttemptRecord-embedded turn
>                                    # capsule per A1 pattern
>     --turn-id <STRING>             # for capsule filenames
> ```
>
> Internal Blackbox prompt (hashed; pinned in
> `assets/prompts/grill_triage_blackbox_v1.md`):
>
> ```
> You are a fast classifier for spec-grill user input.
> Given one user answer (≤ 4096 chars), classify into ONE of:
>   - relevant: the answer addresses the question or is on-topic interview content
>   - off_topic: the answer is coherent but doesn't address the question
>   - abusive: the answer contains hostile / harmful / disallowed content
>   - gibberish: the answer is unparseable / random characters / empty
> Output exactly: {"class": "relevant" | "off_topic" | "abusive" | "gibberish",
>                  "confidence": <float 0..1>}
> No prose. No explanation.
> ```
>
> Stdout JSON shape:
> ```
> {
>   "ok": true,
>   "class": "relevant" | "off_topic" | "abusive" | "gibberish",
>   "confidence": <float>,
>   "model": "Qwen/Qwen3-Coder-30B-A3B-Instruct",
>   "usage": {...},
>   "prompt_capsule_cid": "<hex>"
> }
> ```
>
> Wire into `run()` dispatcher after `complete` branch. Update help.
>
> Allowed edits:
>   - src/bin/turingos/cmd_llm.rs (extend)
>   - assets/prompts/grill_triage_blackbox_v1.md (NEW; ~30 lines)
>   - tests/cmd_llm_triage_stub.rs (NEW)
>
> Required tests (stub-based):
>   - relevant_answer_classified_as_relevant
>   - hostile_text_classified_as_abusive
>   - random_chars_classified_as_gibberish
>   - off_topic_answer_classified_as_off_topic
>   - classifier_output_malformed_returns_ok_false
>   - blackbox_model_id_recorded_correctly_in_capsule
>
> Done criteria:
>   - cargo build --bin turingos passes
>   - cargo test --test cmd_llm_triage_stub --no-fail-fast all passing
>   - Help string includes "triage" action
>   - No edits outside allowed list
```

### Charter R1 §9 W6 specific edit — integrate triage into driven loop

Insert into R1 W6 step 2.h (user answer collection step), REPLACE step 2.h with:

```
h. Display question to user (CLI: stdin); read user answer.
   **Triage step (R2 A5)**: shell out
   `turingos llm triage --workspace <ws> --user-answer <answer>
    --lang <zh|en> --capsule-dir <session>/capsules/
    --turn-id turn-<N>-triage`
   Parse stdout JSON.
   - If class == "relevant": proceed to step 2.i with answer as-is
   - If class == "off_topic": inject nudge as new turn (does NOT advance
     coverage); count toward turn budget; loop back to step 2.b without
     adding answer to last_3_turns
   - If class == "abusive" or "gibberish":
       * Do NOT pass raw answer to Meta on next turn
       * Increment session.non_relevant_count
       * If non_relevant_count == 2: abort session with
         termination_reason = "user_input_unparseable"
       * Otherwise re-render same question + display "您似乎在测试我..."
         prompt; loop back to step 2.b
   - CAS-store the user answer bytes via existing pattern; record
     answer_cid in the next GrillTurnCapsuleBody
```

### Charter R1 §9 W7 specific edit — web endpoint integration

R1 W7 handler logic step 7 — REPLACE with:

```
7. Spawn-blocking sequence:
   a. Triage user answer (if present and turn_count >= 1):
      shell `turingos llm triage --workspace <ws> --user-answer <answer>
       --lang <zh|en> --capsule-dir <session>/capsules
       --turn-id turn-<N>-triage`
      - If non-relevant (off_topic / abusive / gibberish): handle per W6
        2.h triage branch; broadcast `SpecTurnTriageReject` WS event with
        triage_class; do NOT call Meta this turn
   b. If triage relevant OR turn_count == 0:
      shell `turingos llm complete ... --strict-json ...` with assembled
      message array (same as R1 step 7)
   c. Parse stdout JSON; run grill_predicates; on fail retry once
   d. Write GrillTurnCapsuleBody (embedding GrillAttemptRecord per A1)
   e. Broadcast `SpecTurnAdvanced` to WS subscribers
   f. Update GrillSession state
```

### Add WS broadcast variant

R1 W7 `WsBroadcastMsg` extension — ADD:

```rust
pub enum WsBroadcastMsg {
    // ... existing variants from R1 ...
    SpecTurnTriageReject {
        session_id: String,
        turn_index: u32,
        triage_class: String,    // "off_topic" | "abusive" | "gibberish"
        non_relevant_count: u32,
    },
}
```

### LOC budget update

R1 §7 hard constraint — REPLACE:
- "New code LOC target: target ~900 LOC. Hard cap 1500 LOC."
- WITH: "New code LOC target: ~1100 LOC (Blackbox triage included). Hard cap 1700 LOC."

---

## A6. attempt_count tally semantics [Constitution B3 + Architect ratified (b)]

**Architect decision**: option **(b)** — grill turns produce an **independent `grill_attempt_count` counter** that is reported alongside but **NOT summed into** `evaluator_reported_completed_llm_calls` (CLAUDE.md §6 LHS scope per OBS_TB18R_INV1_NONLLM_TX).

### Charter R1 §2 specific edit

Insert new bullet in §2 "New invariants for Phase 6.3.x" between I-8 and the (new R2) I-9:

```
| **I-11** | Grill driven sessions emit a parallel `grill_attempt_count` tally,
distinct from `evaluator_reported_completed_llm_calls`. The grill count is
the sum of: `meta_turns_accepted + meta_turns_rejected + triage_calls +
synthesis_calls`. It is reported in §6 step 4 `cas_walk_output.txt`
under a separate header `=== Grill Attempt Count ===` and is NOT mixed
into `tool_dist.{step,parse_fail,llm_err}` (CLAUDE.md §6 LHS scope is
preserved). | `tests/constitution_grill_driven_anchors.rs::grill_count_separate_from_evaluator_count` |
```

### Charter R1 §6 step 4 specific edit

R1 §6 step 4 "CAS walk verification" — ADD new sub-step:

```
- Tally `grill_attempt_count` per A6:
    grep "=== Grill Attempt Count ===" cas_walk_output.txt
    verify: meta_turns_accepted + meta_turns_rejected + triage_calls +
            synthesis_calls = total reported
    verify: this tally is REPORTED but NOT ADDED to
            evaluator_reported_completed_llm_calls
```

### Charter R1 §9 W5 specific edit — session capsule schema

Add to `GrillSessionCapsuleBody` (extending R1 schema):

```rust
pub struct GrillSessionCapsuleBody {
    // ... existing R1 fields ...
    pub grill_attempt_tally: GrillAttemptTally,         // [A6]
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GrillAttemptTally {
    pub meta_turns_accepted: u32,
    pub meta_turns_rejected: u32,
    pub triage_calls_relevant: u32,
    pub triage_calls_non_relevant: u32,
    pub synthesis_calls: u32,  // = 1 on success, 0 on session abort
}
```

---

## A7. `--meta-prompt <path>` flag [Karpathy K-C3 — ADVISORY, ratified]

**Decision**: ratify. Zero-LOC forward-compat hook for LLM-authored grill drop-in.

### Charter R1 §9 W4 specific edit — CLI surface

R1 W4 CLI args — ADD after `--lang`:

```
    --meta-prompt <PATH>           # default: assets/prompts/grill_meta_v1.md
                                    # path resolved relative to workspace if
                                    # not absolute; W6 also accepts this flag
```

### Charter R1 §9 W6 specific edit — CLI surface

R1 W6 `--mode driven` CLI extension — ADD argument:

```
    --meta-prompt <PATH>           # default: assets/prompts/grill_meta_v1.md
                                   # passed through to each `turingos llm
                                   # complete` shell-out invocation
```

W6 implementation note: load meta-prompt content from this path; sha256 the
content for `system_prompt_template_hash`; pass content as the first system
message in the assembled prompt.

---

## A8. PredicateVerdict typed enum [Constitution F1 — HARDENING]

**Resolution**: promote `PredicateVerdict::Fail{reason: String}` to typed enum, aligned with existing `LeanErrorClass` discipline.

### Charter R1 §9 W3 specific edit

R1 W3 public surface — REPLACE `PredicateVerdict`:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PredicateVerdict {
    Pass,
    Fail(PredicateFailureClass),
}

/// Typed predicate failure class. Discriminants are byte-stable. Tail-additive.
/// Mirrors the existing LeanErrorClass / RejectionClass enum-typed verdict
/// discipline per CLAUDE.md §14.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum PredicateFailureClass {
    SchemaParseError = 0,         // P1
    KindMismatch = 1,              // P2
    UnknownSlot = 2,               // P3
    NonMonotonic = 3,              // P4
    TurnOutOfRange = 4,            // P5
    LanguageMismatch = 5,          // P6 (lang doesn't match --lang)
    QuestionTooShort = 6,          // P6 (< 8 chars)
    QuestionMissing = 7,           // P2 sub-case (done=false + null question)
    PlaybackMissing = 8,           // P2 sub-case (done=true + null playback)
    ConfidenceOutOfRange = 9,      // optional; for envelope validation
}

impl PredicateVerdict {
    pub fn is_pass(&self) -> bool { matches!(self, PredicateVerdict::Pass) }
    pub fn failure_class(&self) -> Option<PredicateFailureClass> {
        match self {
            PredicateVerdict::Pass => None,
            PredicateVerdict::Fail(c) => Some(*c),
        }
    }
}
```

### Test additions for A8

Add to `tests/grill_predicates_p1_p6.rs`:
- `predicate_failure_class_discriminant_lock` — assert byte-stable 0..9

---

## A9. Forbidden-file regex for §9 dispatch protocol [Constitution M3 — HARDENING]

**Decision**: ratify R1 audit's regex verbatim.

### Charter R1 §9 dispatch protocol specific edit

R1 §9 dispatch protocol step 3 — REPLACE with:

```
3. On agent return:
   a. Run §5 gates locally
   b. Run forbidden-file regex check:
      git diff --name-only <base>..HEAD | rg \
        '^(src/(state/(sequencer|typed_tx)\.rs|kernel\.rs|bus\.rs|sdk/tools/wallet\.rs|bottom_white/cas/schema\.rs|runtime/(prompt_capsule|attempt_telemetry)\.rs)|genesis_payload\.toml|Cargo\.(toml|lock))$' \
        && echo "FORBIDDEN FILE TOUCHED — ABORT ATOM" && exit 1
      Match → orchestrator aborts atom, dispatches a fresh agent with the
      gate-fail message + the original spec
   c. On gate fail: dispatch fresh agent with gate-fail context
   d. On gate pass: mark atom complete; dispatch next atom
```

---

## A10. W10 / §6a audit model = Claude (architect ratified)

**Architect decision**: Use Claude (Sonnet 4.6 for §6a verifier; Opus xhigh for §9.W10 audit). NOT Codex.

### Charter R1 §6a specific edit

R1 §6a "Agent specification" table — UPDATE row `Dispatch`:

```
| Dispatch | `Agent` tool, `subagent_type: general-purpose`, `model: opus`, no worktree isolation. Architect ratification (R2 A10): Claude over Codex for all Phase 6.3.x audits, per Phase 7 W8.2 precedent and chat 2026-05-18. |
```

### Charter R1 §9.W10 specific edit

R1 W10 agent dispatch block — UPDATE:

```
Agent({
  description: "Phase 6.3.x clean-context audit",
  subagent_type: "general-purpose",
  model: "opus",                    // Claude Opus xhigh per R2 A10
  prompt: <verbatim W10 brief>,
})
```

### Charter R1 §13 Q10 specific edit

R1 §13 Appendix A Q10 "Audit model" — REPLACE with:

```
10. **Q: Audit model: Opus xhigh, or other?**
    [x] **Opus xhigh (Claude)** — ratified R2 A10 per architect 2026-05-18
    [ ] Codex
    [ ] Other: ____________
```

---

## A11. Rationale_brief shielding test [Karpathy O1 + Constitution G2 — HARDENING]

**Resolution**: Add Class-1 test in W3 brief.

### Charter R1 §9 W3 specific edit

R1 W3 "Required test cases (minimum)" list — ADD:

```
- `rationale_brief_never_appears_in_subsequent_prompt_capsule_visible_context`
  — verify that after running run_turn_predicates over a payload that includes
  rationale_brief, no subsequent prompt assembly (W6 step 2.a) includes that
  rationale_brief string in visible_context_cid bytes
```

### Charter R1 §5 specific edit

R1 §5 "New constitution gates added by this packet" — ADD:

```
- `tests/constitution_grill_driven_anchors.rs::rationale_brief_shielded_across_turns`
  (Researcher B §8.6 mechanization; R2 A11)
```

---

## A12. W2/W3 import-path pre-flight [Constitution O2 + M1 — HARDENING]

**Problem**: R1 W3 brief says `use crate::bin::turingos::grill_envelope::{...}` but cargo treats `src/bin/turingos/` as a binary crate, not `crate::bin::turingos::*`. A non-thinking sub-agent might attempt to "fix" by hoisting types to `src/runtime/` (outside Allowed Paths) or by editing the bin crate root.

**Resolution**: Orchestrator inspects the actual module layout BEFORE dispatching W2 and W3; rewrites the import-path line in the brief if needed.

### Charter R1 §9 dispatch protocol specific edit

R1 §9 dispatch protocol step 1 — REPLACE with:

```
1. Create / verify branch `codex/tisr-phase6-3-x-grill-driven` at base
   `86e82406` (in-place dispatch; no worktree isolation per architect's
   memory `feedback_agent_worktree_drift`).
   **Pre-flight (R2 A12)**: before dispatching W2:
     - Inspect `src/bin/turingos.rs` vs `src/bin/turingos/mod.rs`
     - Determine actual module root path for new `grill_envelope.rs`
     - If `src/bin/turingos/` is bin-crate root with `mod.rs`:
       * grill_envelope.rs goes under that mod tree; W3 import path:
         `use crate::grill_envelope::{...}` (binary's local crate root)
       * NOT `use crate::bin::turingos::grill_envelope::{...}`
     - If `src/bin/turingos.rs` is a single-file bin (no `mod.rs`):
       * grill_envelope needs to be hoisted to library crate:
         `src/lib/grill_envelope.rs` or `src/runtime/grill_envelope.rs`
       * In that case Allowed Paths §4 must add the new location
       * **Stop and re-ratify with architect** before proceeding
     - Rewrite the W3 brief's import path line to match actual layout
       before dispatching W3
   2. Dispatch Agent tool with the (possibly rewritten) W<N> brief...
```

---

## A13. W3 brief test-file enumeration tightening [Constitution M2 — HARDENING]

**Problem**: R1 W3 brief charter:861 says "No edits outside: src/runtime/grill_predicates.rs (new), src/runtime/mod.rs (add pub mod line), test files" — "test files" is plural and unspecific.

### Charter R1 §9 W3 specific edit

R1 W3 "Done criteria" → "No edits outside" list — REPLACE with:

```
- No edits outside the following EXHAUSTIVE list:
    - src/runtime/grill_predicates.rs (NEW)
    - src/runtime/mod.rs (add `pub mod grill_predicates;` line only)
    - tests/grill_predicates_p1_p6.rs (NEW)
    - tests/grill_predicates_termination.rs (NEW)
  Any other file edit → orchestrator forbidden-file check aborts atom (A9).
```

Apply the same exhaustive-list pattern to W2, W4, W4.5, W5, W6, W7, W8 briefs (R1 already has them; verify each is exhaustive).

---

## A14. Session resume from CAS on web restart [Constitution C2 — HARDENING]

**Problem**: R1 §1 W7 says AppState.sessions is process-local in-memory; CAS holds canonical state. But if the server crashes and restarts, AppState.sessions is empty, while CAS session capsule already says terminated. A returning frontend client would see undefined behavior.

**Resolution**: Architect ratifies that **in-flight session resumption is NOT supported in v1**. The server-restart edge case is documented; frontend treats "session not found" as fatal and falls back to legacy static mode.

### Charter R1 §7 specific edit

R1 §7 "Hard" constraints — ADD bullet:

```
- **In-flight session resumption is NOT supported in v1.** If the web server
  restarts mid-session, AppState.sessions is lost; the returning frontend
  client receives 404 on POST /api/spec/turn for the unknown session_id
  and falls back to legacy `?mode=static` per the existing 5xx fallback
  path (W8 brief). CAS-resident turn capsules remain queryable via
  `turingos spec audit --session <id>` for offline review. Phase 6.3.y may
  add AppState reconstruction from CAS on boot; not in this packet.
```

### Charter R1 §11 specific edit

R1 §11 "Backout / Rollback" — ADD point:

```
6. **Server-restart edge case**: if web server restarts during an active
   session, that session is lost in memory; client sees 404 → frontend
   falls back to static mode (W8). No CAS corruption; turn capsules remain.
   No remediation required; ratified as v1 limitation per A14.
```

---

## A15. LOC budget consolidated update

R1 §7 Soft constraint — REPLACE with:

```
- **New code LOC target: ~1100 LOC** (R1 base ~900 + W4.5 Blackbox triage +200).
  Hard cap **1700 LOC**.
  Atom-1 cannot exceed 1700 without architect §8 amendment.
```

---

## §8 Sign-off Section (R2)

Architect must mark each of the following before atom dispatch:

```
[ ] R1 §8 sign-off block — read and re-confirmed
[ ] A1  AttemptOutcome fix (Constitution C1) — confirmed
[ ] A2  hidden_fields_redacted = true (Constitution C2) — confirmed
[ ] A3  Termination predicate FC mapping + I-9, I-10 invariants — confirmed
[ ] A4  Software 3.0 fidelity honesty clarifier (Karpathy K-C1) — confirmed
[ ] A5  Promote Blackbox triage to W4.5 in packet — confirmed (chat 2026-05-18)
[ ] A6  Independent grill_attempt_count counter — confirmed (chat 2026-05-18)
[ ] A7  --meta-prompt <path> flag — confirmed
[ ] A8  PredicateFailureClass typed enum — confirmed
[ ] A9  Forbidden-file regex in dispatch protocol — confirmed
[ ] A10 W10 audit model = Claude (NOT Codex) — confirmed (chat 2026-05-18)
[ ] A11 rationale_brief shielding test — confirmed
[ ] A12 W2/W3 import-path pre-flight — confirmed
[ ] A13 W3 test-file enumeration tightening — confirmed
[ ] A14 No in-flight session resume in v1 — confirmed
[ ] A15 LOC budget update: ~1100 / 1700 cap — confirmed
[ ] Dispatch authorization: orchestrator may proceed to W0
```

**Architect signature** (free-text + date): _____________________________

---

## A16. Updated atom dispatch order

R1 §9 dependency graph — REPLACE with:

```
W0 (worktree + plan)                        ← in-place; pre-flight inspect (A12)
  │
  ├─→ W1 (prompt assets) ─────┐               ← Haiku docs (incl. triage prompt)
  │                            │
  ├─→ W2 (grill_envelope.rs)──┤               ← Sonnet pure parser
  │                            │
  └─→ W3 (grill_predicates) ──┤               ← Sonnet pure functions; typed enum
                              │
                              ▼
                W4 (cmd_llm complete) ──┐    ← depends on W2
                                        │
                W4.5 (cmd_llm triage) ──┤    ← depends on W4 [A5]
                                        │
                W5 (capsule writers) ───┤    ← depends on W2
                                        │
                                        ▼
              W6 (cmd_spec --mode driven)    ← depends on W4 + W4.5 + W5 + W3
                                        │
                                        ▼
              W7 (web /api/spec/turn)        ← depends on W6
                                        │
                                        ▼
              W8 (frontend driven mode)      ← depends on W7
                                        │
                                        ▼
              W9 (real-LLM E2E witness)      ← Opus Chrome MCP [A10]
                                        │
                                        ▼
              W10 (clean-context audit)      ← Opus xhigh Claude [A10]
```

Notes:
- W1, W2, W3 parallel-dispatchable after W0
- W4.5 depends on W4 (extends same file); cannot parallel with W4 in practice
- W5 can parallel with W4 (different files) but both depend on W2
- W6 fans in W4 + W4.5 + W5 + W3

---

## End of R2 amendments

**R1 base sections unchanged**: §0 header (except revision line), §2 partial (only the noted additions), §3, §4, §5 (except A11 addition), §6 (except A6 addition), §6a (except A10), §7 (except A14 + A15), §9 dispatch protocol (except A9 + A12), §10 risk register, §11 (except A14), §12 glossary, §13 (except Q10 per A10), §14.

**R1 sections functionally REPLACED by R2**: §1 (clarifier + W4.5 artifact + Loop driver row), §9 W3 (typed enum), §9 W4 (no AttemptTelemetry; --meta-prompt; hidden_fields_redacted=true), §9 W5 (GrillAttemptRecord + GrillAttemptTally), §9 W6 (triage integration + --meta-prompt), §9 W7 (triage integration + WS variant), §9 dispatch protocol (A9 + A12).

**Forward to**: architect §8 signature, then W0 dispatch.
