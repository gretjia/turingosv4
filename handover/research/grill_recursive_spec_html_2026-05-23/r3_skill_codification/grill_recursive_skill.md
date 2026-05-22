# Grill Recursive Skill

Use this skill before starting any TuringOS grill session where recursive slot
convergence is required — when the user's answers are expected to be vague,
contradictory, or under-specified, and the orchestrator needs a rigorous
discipline for extracting clean slot coverage before synthesis.

This is not a personality constraint. It is a structured convergence protocol:
anchor each slot with the user's literal language, loop until the anchor
stabilizes, and declare the slot locked only when the user's last answer
confirms rather than revises the anchor.

## Background

The standard grill meta prompt (`grill_meta_v1.md`) extracts 8 slots in a
linear single-pass interview. For simple tasks this is sufficient. For
ambiguous tasks — where the user's job/anchor/memory answers contradict each
other across turns, or where vague answers produce a spec that fails generation
— the single-pass approach terminates too early.

The recursive mechanism adds a convergence loop *within* each slot: the
interviewer holds the current anchor for a slot and refuses to advance until
the user's follow-up confirms the anchor. If the user revises, the anchor
updates and one more confirmation turn fires. This prevents the synthesis step
from receiving internally contradictory slot content.

The slot lock condition: the current anchor text is "stable" when the user's
last turn *confirms* it (rephrases it without changing substance) rather than
revising or contradicting it.

## Mandatory PRELUDE (before first grill LLM call)

Before invoking the meta LLM for turn 1, the orchestrator MUST set these
session parameters and verify them:

```text
## GRILL RECURSIVE PRELUDE

1. Confirm session mode: this session uses recursive anchor convergence.
   Log: "[grill-recursive] session mode: recursive anchor, max_inner_loops=2"

2. Set anchor state per slot (8 slots, all start as None):
   anchor_state = { job: None, anchor: None, memory: None, first_run: None,
                    robustness: None, scope: None, acceptance: None, mirror: None }

3. Set confirmation state per slot (all start as False):
   confirmed = { job: False, anchor: False, ... (same 8 keys) }

4. Inner loop budget: max 2 additional follow-up turns per slot before
   accepting best-available anchor and moving on. This prevents infinite loops
   if the user is genuinely unable to be specific.
   loop_budget = { job: 2, anchor: 2, ... }

5. Check: grill_meta_v1 prompt is materialized in the workspace.
   Verify: <workspace>/assets/prompts/grill_meta_v1.md exists.
   If missing: run `turingos init --workspace <workspace>` first.
```

## Protocol

### SLOT ADVANCE RULE

The grill interviewer MAY advance from slot S to slot S+1 only when:
- The current anchor for S is non-None, AND
- confirmed[S] is True, OR loop_budget[S] has reached 0.

If confirmed[S] is False and loop_budget[S] > 0:
- Generate a mirror-back question that reflects the current anchor and asks
  the user to confirm or correct.
- Decrement loop_budget[S] by 1.
- If user confirms: set confirmed[S] = True, advance.
- If user revises: update anchor[S] to new content, keep confirmed[S] = False.

If loop_budget[S] == 0 and confirmed[S] is still False:
- Accept the last anchor as the best-available anchor.
- Log: "[grill-recursive] slot <S>: budget exhausted, accepting best-available anchor"
- Mark confirmed[S] = True and advance.

### ANCHOR EXTRACTION

When the user answers a slot question, extract the anchor as:
- The literal phrase(s) from the user's answer that describe the core of the
  slot (not paraphrased, not translated to tech jargon).
- Maximum 40 words.
- If no extractable literal phrase: anchor[S] = "(user did not specify)"

### MIRROR-BACK QUESTION FORMAT

When firing a follow-up confirmation turn for slot S, the question MUST:
- Begin with the Chinese label "听起来你的意思是：" or English "If I heard
  right:".
- Reproduce the extracted anchor verbatim in quotation marks.
- End with "是这样吗？" or "— is that right?".
- Add one concrete follow-up only if the anchor is ambiguous.

Example (slot = memory, anchor = "每个月的支出记录"):
```
听起来你的意思是：「每个月的支出记录」——这个工具要记住的。
是这样吗？还是有其他你觉得更重要的？
```

### SLOT LOCK LOG

After each slot S locks (confirmed[S] = True), log one line to the session
transcript:

```
[grill-recursive] slot <S> locked: anchor="<anchor text>" | turns_used=<n> | confirmed=<True|BudgetExhausted>
```

This log is audit evidence. It MUST be written before advancing to the next
slot.

## Mandatory POSTLUDE (after grill done=true)

After the meta LLM emits `done=true` and before handing off to synthesis:

```text
## GRILL RECURSIVE POSTLUDE

1. Verify all 7 required slots are locked (confirmed or budget-exhausted):
   required = [job, anchor, memory, first_run, robustness, scope, acceptance]
   missing = [s for s in required if anchor_state[s] is None]
   If missing is non-empty:
     HALT: "[grill-recursive] FATAL: slots not locked before synthesis: {missing}"

2. Assemble anchor summary for synthesis context:
   Write one anchor_summary block:
   ---
   GRILL RECURSIVE ANCHOR SUMMARY
   job:        <anchor>
   anchor:     <anchor>
   memory:     <anchor>
   first_run:  <anchor>
   robustness: <anchor>
   scope:      <anchor>
   acceptance: <anchor>
   mirror:     <anchor or "(not covered)">
   ---

3. Pass the anchor_summary to the synthesis prompt alongside the Q/A pairs.
   The synthesis LLM uses the anchor summary as a conflict-resolution oracle:
   if the Q/A transcript is internally contradictory, the locked anchor text wins.
```

## When to use this skill

- Any grill session where the user's prior answers show contradiction or
  vagueness (e.g., "I want something simple" followed by listing 12 features).
- Sessions where the previous generation attempt failed with
  `generate_quality_failed` and the cause was an underspecified spec.
- Sessions flagged for recursive convergence mode by the orchestrator.

## When NOT to use this skill

- Standard single-pass grill sessions for simple, well-defined tasks.
- Sessions where the user is a developer and can be trusted to supply precise
  slot content without follow-up confirmation loops.
- Diagnostic-only or read-only sessions (no synthesis, no generate).

## Related skills

- `runner-preflight` — pre-action gate for runner scripts that mutate evidence/
- `constitution-landing-check` — pre-charter gate for TB charters
- `spec-html-renderer` — visual format skill invoked at generate time
- Reference: `assets/prompts/grill_meta_v1.md` (meta prompt this skill augments)
- Reference: `assets/prompts/grill_synthesis_zh_v2.md` (synthesis prompt that
  consumes the anchor summary this skill produces)
