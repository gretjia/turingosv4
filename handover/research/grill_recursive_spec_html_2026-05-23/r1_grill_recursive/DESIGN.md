# Recursive Anchor Design — Grill Interview System

**Document**: R1 Research Output — Recursive Anchor Mechanism  
**Date**: 2026-05-23  
**Status**: Production-ready, for direct integration into grill_meta_v1_recursive.md and grill_synthesis_zh_recursive.md

---

## 1. Problem Analysis: Why the Current Grill Drifts

### 1.1 The Scope-Drift Failure Mode

The user's observation — "到最后我们聊的内容和我最想做的东西已经远超于我最早想做的东西了" — points to a structural property of JTBD + SPIN interview methodology applied without a convergence constraint.

SPIN's four-phase structure (Situation → Problem → Implication → Need-Payoff) is designed to move the respondent from surface symptoms to root cause and then to an articulated felt need. This movement is psychologically powerful precisely because it makes people feel understood at a deeper level. The danger: the "Need-Payoff" phase often lands on the deepest possible resolution of the underlying problem, not the smallest buildable thing the user came in for. SPIN was designed for enterprise sales where "bigger picture" is desirable; in spec generation for a tool-builder, it creates scope inflation.

JTBD (Moesta's struggling moment frame) similarly focuses on the job-to-be-done at its most fundamental level — not "I want a flashcard app" but "I want to not fail my exam." This is analytically correct but, without an anchor mechanism, the spec ends up describing an ideal solution to the root job rather than the concrete thing the user arrived wanting to build.

Voss mirroring amplifies the effect: when the interviewer reflects the user's language back with subtle upward framing ("听起来你真正想要的是..."), the user confirms the inflated version because it feels emotionally accurate.

### 1.2 Which Slots Drive Drift

Examining the 8 canonical slots:

| Slot | Drift Risk | Mechanism |
|---|---|---|
| `job` | High | JTBD root-cause pull; "struggling moment" surfaces the deepest underlying job |
| `anchor` | Low | Anchors to existing products, tends to constrain |
| `memory` | Medium | Can inflate from "save high score" to "full user account system" if probed |
| `first_run` | Low | Concrete, visual; hard to inflate |
| `robustness` | Low | Negative space; harder to over-expand |
| `scope` | Medium | "Who uses it" can expand from "just me" to "and my colleagues and..." |
| `acceptance` | Medium | "What success looks like" can drift from "I can do X in 30 seconds" to "it changes how teams work" |
| `mirror` | High | Final Voss playback can consolidate the inflated frame as the agreed reality |

The highest-risk pair is `job` (which pulls deepest) and `mirror` (which locks in whatever has drifted). The `scope` and `acceptance` slots are secondary amplifiers.

### 1.3 The Absence of a Re-grounding Mechanism

Neither v1 nor v2 of the meta-prompt contains any instruction for the interviewer to check whether the current exploration trajectory is consistent with the user's original request. The `mirror` slot is defined as a final playback — it confirms the accumulated spec, but by that point the spec has already drifted. There is no mid-session checkpoint that asks: "Are we still building what you came here for?"

---

## 2. Framework Analysis: What the Literature Offers

### 2.1 SPIN Selling — Built-in Return to Payoff

SPIN's "Need-Payoff" phase is actually the correct model: after surfacing the implication of the problem, the interviewer asks "how would it help you if you could solve X?" This forces the conversation back to a concrete payoff — but the payoff is tied to the deepest problem, not the original request. The fix is to run Need-Payoff against the original stated want, not the discovered root cause: "Given everything we've discussed, does solving [original request] still feel like the right next step?"

### 2.2 JTBD — The Struggling Moment as Anchor

Moesta's methodology emphasizes that the "struggling moment" is the actual unit of analysis — not the product category, not the feature set. The struggling moment is usually anchored in a specific recent episode: "two weeks ago, when X happened, I thought 'I wish I had a tool for this.'" This is narrow and concrete. The drift occurs when interviewers move from this specific moment to the abstract job-category it belongs to. The recursive anchor mechanism uses the struggling moment itself — the user's first-turn answer — as an immutable reference that is never overwritten.

### 2.3 Design Thinking — "What They Asked For" vs "What They Need"

Classic Design Thinking methodology (particularly the Stanford d.school "How Might We" framing) explicitly separates the user's stated request from the underlying need. The technique produces two outputs: a "Point of View" (who the user is, their need, and the insight that explains why) alongside an "Artifact Brief" (what to actually build next). This two-track output is the structural model for our two-layer synthesis: the immediate buildable thing alongside the deeper insight.

### 2.4 Voss Mirroring — Confirmation Before Depth

Voss's technique of mirroring before advancing is already partially implemented in the grill methodology, but it is used to confirm understanding before going deeper — not to confirm that depth is warranted. The recursive anchor adapts this: at the anchor checkpoint (turn ~3), the interviewer mirrors the original stated goal and explicitly asks whether going deeper is desired, or whether the user would prefer to stay close to what they originally came for.

---

## 3. The Recursive Anchor Mechanism — Design

The mechanism has four interlocking components. Each component operates at a different point in the interview loop and together they form a complete convergence constraint system.

### 3.1 Component A: The `original_anchor` Slot (8th required slot, v2)

**What it is**: A new slot extracted from the user's very first turn answer, before any SPIN/JTBD elaboration occurs. The slot captures the user's initial stated want in near-verbatim form — not the underlying job, not the reframed goal, but the literal first thing they said they wanted.

**Why it works**: The first turn is the least contaminated data point. The user has not yet been guided toward depth by SPIN implication questions. Whatever they say in turn 1 is their closest articulation of "what I came here to build today." By extracting this into a named slot and treating it as immutable, the system preserves a ground truth that cannot be overwritten by subsequent elaboration.

**Implementation**: `original_anchor` is added to the canonical slot list. It is extracted from the user's first turn answer, carries no follow-up question (it is derived, not asked), and is forwarded to the synthesis prompt unchanged. The slot is required for termination (the kernel predicate must include it).

**Immutability rule**: Once `original_anchor` enters `covered_slots`, it must never be modified by subsequent turns. The slot's content is the first-turn answer text, not the interviewer's interpretation of it.

### 3.2 Component B: Turn-N Anchor Checkpoint (around turn 3)

**What it is**: A mandatory re-grounding question inserted approximately at turn 3, after enough slots have been covered to give the user a sense of what they're building, but before the SPIN implication questions pull the conversation toward abstract job-level depth.

**Trigger condition**: After `covered_slots` contains at least `{job, anchor}` and the turn count reaches 3 (or when `covered_slots` contains 3+ slots, whichever comes first).

**The question form**: The interviewer restates the `original_anchor` in one sentence and asks: "这仍然是你最想做的核心吗，还是我们聊着聊着发现了更重要的方向？" (Is this still the core of what you most want to build, or have we discovered a more important direction as we talked?)

**Why this wording**: The question is non-leading — it legitimizes both answers. If the user says "yes, that's still what I want," the interviewer has an explicit ratification to stay close to the original scope. If the user says "actually, what we discovered is more important," the interviewer has an explicit mandate to explore depth — and the synthesis will document both, with the deeper direction labeled as future rather than immediate.

**Effect on subsequent turns**: The user's answer to the anchor checkpoint question is stored in the `original_anchor` slot metadata. If the user confirms the original, the remaining turns should prioritize completing coverage of the original request's slots (`memory`, `first_run`, `robustness`, `scope`, `acceptance`) without expanding the job further. If the user endorses the deeper direction, the synthesizer is informed to frame that direction as the primary output with an explicit "这超出了本次构建范围" qualifier.

### 3.3 Component C: Scope Gate in Meta-Prompt

**What it is**: A depth-monitoring rule embedded in the meta-prompt's CONSTRAINTS section. The interviewer tracks how many "layers" deep the current question is from the original job statement. A "layer" is defined operationally: a question is layer N+1 if it asks about the cause, consequence, or context of the answer to the previous question, rather than asking about a new slot.

**The 2-layer rule**: If the current exploration chain is more than 2 questions deep from the original job statement (i.e., the current question is driven by "why does that matter?" rather than "what else do you need?"), the interviewer must surface the depth explicitly: "这个发现很深。我想确保我们还在做你最开始想做的东西——[original_anchor verbatim]。我们是应该继续探索这个方向，还是回到最初的目标？"

**Implementation in prompt**: Added as rule 6 in the CONSTRAINTS section of grill_meta_v1_recursive.md. The depth tracking is self-monitored by the LLM (not a hard counter), consistent with how confidence is self-assessed in the current system.

**Why 2 layers**: One layer of implication is valuable — it surfaces the "why" behind the first-run experience and makes the spec more resilient. Two layers starts entering philosophical territory. Three or more layers almost always produces scope that exceeds the original ask by an order of magnitude.

### 3.4 Component D: Two-Layer Synthesis Output

**What it is**: The synthesis prompt is extended with two new output sections that separate the immediate buildable scope from the discovered deeper insight.

**Section `## 立刻能做的 (Build Now)`**: Grounds the spec in the `original_anchor`. Every feature described in this section must be traceable to either the `original_anchor` or the slots that concretize it (`first_run`, `memory`, `robustness`). This is what gets built today. It is complete, scoped, and actionable without further elaboration.

**Section `## 更深的洞察 (Deeper Insight)`**: Acknowledges the latent need surfaced by the JTBD/SPIN exploration, without treating it as an immediate build requirement. Includes the note: "这是更大的可能，留待将来。本次构建不需要解决这个。" (This is a larger possibility, to be kept for the future. This build does not need to address it.)

**Why two layers instead of one**: Single-layer synthesis forces a choice: either ignore the deep insight (and the user feels unheard) or include it in the spec (and the spec becomes unreachable). Two layers means both the user's immediate goal and their deeper discovery are honored — the immediate goal gets built, the deeper discovery gets preserved as a named future possibility. This is the same structure as a good product backlog: the sprint commitment is different from the roadmap.

**Placement in synthesis**: The two new sections appear after `## 一句话给 AI 编程员` (the immediate AI coder prompt) and before the optional `## 我听到的矛盾` section. This ensures they do not interrupt the existing spec structure while still being present in every synthesis output.

---

## 4. Parser Compatibility and JSON Contract

The changes to the meta-prompt are additive to the JSON output contract. The new slot `original_anchor` appears in `covered_slots` and `open_slots` alongside the existing 7 required slots. The kernel termination predicate gains one additional required slot. The `playback` field at `done=true` continues to carry the 7-row fridge note (job/anchor/memory/first_run/robustness/scope/acceptance) — the `original_anchor` is not part of the playback rows because it is derived, not asked.

The `spec-grill.ts` parser reads `covered_slots`, `open_slots`, `confidence`, `done`, `question`, `playback` — none of these field names change. The addition of `original_anchor` to the slot list only affects the values inside `covered_slots` and `open_slots`, which are arrays of strings parsed without a fixed schema. No parser changes are required.

The synthesis prompt receives the same Q/A transcript format — the new `original_anchor` slot's content is available in the transcript as the user's turn 1 answer, which is already present. No new data pipeline is required; the synthesis prompt is instructed to extract it from A1.

---

## 5. Expected Behavioral Change

Before the recursive anchor mechanism, a typical grill session on a "make me a habit tracker" request might proceed:

- T1: user says "I want a simple habit tracker"  
- T3: SPIN implication question surfaces "you want to change your behavior at a deeper level"  
- T6: job slot becomes "help me become the person I want to be"  
- Synthesis: spec describes a behavior-change platform with social accountability, streak visualization, and identity journaling

After the recursive anchor mechanism:

- T1: user says "I want a simple habit tracker" → `original_anchor` = "I want a simple habit tracker"  
- T3: anchor checkpoint — "你最早说你想做一个简单的 habit tracker，这仍然是你最想做的核心吗？"  
- User: "对，就是这个，别搞太复杂"  
- Remaining turns: focus on `memory`, `first_run`, `robustness`, `acceptance` for the habit tracker  
- Synthesis: `## 立刻能做的` = a minimal habit tracker, complete and buildable; `## 更深的洞察` = "你提到了想成为你理想中的自己——这是更大的可能，留待将来"

---

## 6. Limitations and Failure Modes

**LLM self-monitoring**: The depth-tracking (2-layer rule) relies on the LLM's own assessment of how deep the conversation has gone, same as confidence self-assessment. If the LLM miscounts layers, the gate may fire late or not at all. This is acceptable because the turn-N anchor checkpoint is a hard trigger (turn-count-based), providing a redundant convergence signal.

**User may genuinely want the deeper thing**: If the user's anchor checkpoint answer endorses the deeper direction, the mechanism respects that. The two-layer synthesis will document the deeper direction as `## 立刻能做的` and the original anchor as a note in `## 更深的洞察`. The interviewer must handle this branch gracefully (see meta-prompt rule 6b).

**First-turn vagueness**: If the user's first turn answer is too vague to anchor ("I want an app"), the `original_anchor` slot will contain vague content. In this case the anchor checkpoint question is still valuable but the user's answer to it ("that's still what I want") may not provide enough constraint. The synthesis prompt handles this by checking whether `original_anchor` has sufficient specificity; if not, it adds the content to `## 还没问到` rather than treating it as a valid anchor.

---

## 7. Summary of Changes by File

| File | Change type | What changes |
|---|---|---|
| `grill_meta_v1_recursive.md` | Additive | New slot `original_anchor`, scope-gate rule (rule 6), anchor-checkpoint trigger (rule 7), updated slot count in termination predicate |
| `grill_synthesis_zh_recursive.md` | Additive | Two new output sections (`## 立刻能做的`, `## 更深的洞察`), `original_anchor` extraction instruction from A1, placement after `## 一句话给 AI 编程员` |

No parser changes. No schema changes. No new API surface. No new state machine states. The mechanism is entirely prompt-level.
