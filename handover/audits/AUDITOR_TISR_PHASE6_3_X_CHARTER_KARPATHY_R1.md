# AUDITOR_TISR_PHASE6_3_X_CHARTER_KARPATHY_R1

Auditor: clean-context Opus xhigh, Karpathy Software 3.0 lens
Date: 2026-05-18
Target: handover/directives/2026-05-18_TISR_PHASE6_3_X_GRILL_LLM_DRIVEN_SECTION8_PACKET.md
Predecessor: none

## Verdict: PROCEED

(Charter is faithful Software 3.0 for v1; two CHALLENGE-level concerns documented below for architect ratification, but none rise to a redesign-blocking VETO.)

## Software 3.0 fidelity rating: B

(Directionally correct with caveats. The grill's per-turn agency, dual-gated termination, replay-without-recall, and prompt-as-program design are A-grade Karpathy moves. The B-grade comes from three specific compromises: (i) the deferred Blackbox triage collapses Karpathy's "scheduler/worker" pattern to "worker-only" in v1; (ii) the orchestration is a Rust loop that shells out to a stateless CLI per turn rather than handing the LLM the loop directly, leaving an FC1-N4-shaped seam where TuringOS still drives turn cadence even though the LLM drives turn content; (iii) the meta-prompt is filesystem-resident but its discovery is hardcoded in cmd_spec.rs rather than registry-discoverable, weakening the future LLM-authored-grill story.)

## Strongest Software 3.0 claims this charter does support

- The meta-prompt at `assets/prompts/grill_meta_v1.md` (W1 brief, charter:594-649) is the actual program. Interview logic — methodology, rubric, mirror-back style, slot-priority — lives in English/Chinese, not Rust. The Rust kernel only enforces a JSON wire contract (W2/W3 briefs, charter:653-870). This is the Karpathy "instructed, not coded" claim landing.
- Per-turn LLM agency over content. The LLM autonomously decides which slot to probe next, how to phrase the question, when to mirror back, when to declare done (charter:32-39 diff table; W6 brief charter:1108-1136). The 8 hardcoded `canonical_questions(Lang::Zh)` are replaced with LLM-generated text per turn. This is Karpathy's "LLM is the process, not a function call" landing on this surface.
- Replay-without-recall (I-8, charter:129; §6 step 5, charter:320-324; R-1 charter:1467). Replay reads parsed turn payload from CAS, never re-invokes SiliconFlow. This is the only way to make a non-deterministic Software-3.0 surface audit-replayable, and matches Anthropic's "context is a compiled view over richer state" principle.
- Dual-gated termination (I-6, charter:127; W3 brief charter:810-816; R-2 charter:1468). LLM self-declares `done=true` but kernel-side `termination_predicate` requires `{job, anchor, memory, first_run, robustness, scope, acceptance}` ⊆ covered_slots AND confidence ≥ 0.8 AND turn ≥ 4. This is Karpathy's "LLMs hallucinate / lack self-awareness" anti-pattern's correct mitigation: agent self-decides + kernel backstops.
- FC1-aligned tape anchoring (charter:108-117). Every externalized LLM cycle produces (PromptCapsule, AttemptTelemetry, EvidenceCapsule) triple with `prompt_context_hash` identity (I-1, I-2). This honors CLAUDE.md §6 externalized-attempt rule and makes the Software-3.0 surface auditable under the same invariant as Lean cycles.
- Bounded context engineering (charter:1115-1116, R-7 charter:1473). Per-turn prompt = system meta-prompt + coverage-state summary + last 3 Q/A pairs. This is Researcher B §4.1 option C (structured history). Honors Karpathy's "RAM is finite" and Anthropic's "context is a compiled view" by not concatenating verbatim history.
- Hashable, versioned program (W1 charter:629-633; FC config `system_prompt_template_hash` per CLAUDE.md §4.3). The meta-prompt's sha256 is pinned in every PromptCapsule, so the "program version" is a first-class CAS object. This is the Software-3.0 analog of a binary's SHA.

## Weakest Software 3.0 claims this charter does NOT support

- The Rust kernel still drives the turn loop. cmd_spec.rs (W6 brief charter:1108-1136) runs `for turn in 1..15` and shells out to `turingos llm complete` per iteration. The LLM has agency over turn *content* but no agency over turn *cadence*, *retry budget*, *coverage-state injection*, or *termination override*. Karpathy's stronger framing — "LLM is the runtime, not a library inside your runtime" — is not landed; this is closer to "LLM is the per-turn worker invoked by the Rust scheduler." Verdict: this is intentional in v1 and is the right factoring for tape anchoring, but the claim should be stated as "Software 2.5 with Software-3.0 turn-content authority" not "Software 3.0 native."
- The "scheduler/worker" multi-LLM pattern from Researcher A §2.4 is collapsed to "worker-only" by deferring Blackbox triage to atom-1.5 (charter:16, charter:93, R-3 charter:1469). Karpathy's keynote explicitly invokes the OS scheduler-vs-CPU distinction for multi-LLM choreography; deferring the cheap scheduler means the expensive Meta model bears prompt-injection triage on every user turn. The 4096-char cap is structural defense but is not the Karpathy answer.
- The grill is implemented as cmd_spec.rs --mode driven with hardcoded references to `assets/prompts/grill_meta_v1.md` (W6 charter:1114, W1 charter:599). A second grill (cooking, Lean Canvas) cannot be added by dropping a TOML and a markdown file; it requires a new Rust subcommand or a refactor. Karpathy's endgame (Researcher C §8.5: "the next grill is authored by an LLM in <30 seconds, dropped into agents/ and run") is not enabled by v1.
- The envelope (charter:614-626; W2 charter:683-693) has no `tool_calls` field. A future LLM grill that wants to invoke MCP tools mid-conversation (Researcher C §8.2, Anthropic Agent SDK 2025) would need a charter amendment. P1 schema-parse predicate (charter:58) treats any field outside the fixed set as a parse failure, which is the correct v1 strictness but is a forward-compat lock.
- The W6 retry path (charter:1120-1123) appends a system message and re-shells; the conversation transcript shown to the LLM does not include the parsed `rationale_brief` of prior turns. This is correct for shielding (Researcher B §4.3, R-7 charter:1473) but means the LLM cannot self-debug its prior failure — Karpathy's "LLM can debug its own runtime" claim is half-landed (the human auditor can debug; the LLM cannot).

## Critical concerns (block §8 or require redesign)

None. All concerns are addressable via ratified scope decisions or post-v1 evolution; none indicate the charter is Software-1.5 cosplay.

## Challenges (require architect ratification or scope decision)

- [C1] **Naming-vs-reality gap on the "Software 3.0 native" claim.** Charter title (charter:1) and §1 prose ("Software 3.0 native LLM-driven grill", charter:24) overclaim. By the strict Karpathy rubric this is Software-2.5 (LLM has turn-content agency; Rust still drives the loop and the retry policy and the termination backstop). Architect should choose one of:
  - (a) Retitle to "Software 2.5 LLM-driven grill" and reserve "Software 3.0 native" for a Phase 6.3.z that gives the LLM the loop (e.g. agent-driven retry, agent-driven termination cadence, agent-emitted tool calls). This is honest but lowers the marketing claim.
  - (b) Keep the title; document explicitly in §1 that "Software 3.0 native" here refers specifically to per-turn content authority + prompt-as-program + replay-without-recall, NOT to LLM-as-runtime-scheduler. Add a row to the §1 diff table acknowledging "loop driver: Rust (unchanged)". This is the lower-friction option and the one I'd recommend.
  - Either way, do NOT proceed with the §8 sign-off describing this as "faithful Software 3.0" without the qualifier; that overstatement will create audit drag later when the second grill ships and reveals that cmd_spec.rs --mode driven is not a reusable substrate.
- [C2] **Blackbox triage deferral collapses the multi-LLM Karpathy story.** Researcher A §5.3 (charter cites at R-3, charter:1469) identifies prompt injection as the failure mode this triage classifier solves. Deferring it to optional atom-1.5 leaves Meta + 4096-char cap as the only defense. Architect should confirm one of:
  - (a) Accept the v1 risk (4096 cap + structural defense is enough for non-production demo).
  - (b) Promote atom-1.5 to atom-2 (in-scope for this packet), upgrading the charter from "Software 2.5 worker-only" to "Software 2.5 scheduler+worker" which is materially closer to Karpathy's claim.
  - (c) Land v1 as-is but commit to atom-1.5 in the same release window before any non-zephryj user touches the grill. Most pragmatic.
- [C3] **Hardcoded prompt-asset path is a quiet forward-compat lock.** W6 brief (charter:1114) hardcodes `assets/prompts/grill_meta_v1.md` in cmd_spec.rs. A second grill (cooking, Lean Canvas, code review) cannot reuse cmd_spec.rs --mode driven — it would need a new --grill-kind flag or a registry lookup. This is exactly the "premature framework" trap Karpathy warns against, BUT it is also exactly the "don't generalize from one instance" discipline Researcher C §9.2 endorses. Architect should ratify one of:
  - (a) Accept the lock; document in §11 that adding cooking/Lean-Canvas grills requires a Phase 6.3.y refactor (Researcher C's CCS, ratified after second real instance exists). This matches the charter's stated decision (charter:17, charter:90).
  - (b) Soften the lock by making the meta-prompt path a CLI flag `--meta-prompt <path>` with default = `assets/prompts/grill_meta_v1.md`. Zero LOC cost, keeps the door open for ad-hoc experiments without refactor.
  - I recommend (b) as a near-free amendment; it adds zero abstraction layers but preserves the "LLM-authored prompt drop-in" path for power users.

## Non-blocking observations

- [O1] The `rationale_brief` field (charter:622; envelope §3.2 of W1) is justified per R-7 (charter:1473) as audit-only. Karpathy lens: this is correct private-CoT discipline (CLAUDE.md Art. III). The charter should add an explicit Class-1 test (Researcher B §8.6 names it `rationale_brief_never_appears_in_subsequent_prompt_capsule_visible_context`) to mechanize this; charter §5 (Exit Gates) lists constitution gates but does not call out this specific shielding test. Recommendation: add to §5 or §9.W3 acceptance tests.
- [O2] Hard turn ceiling = 15 (charter:64, §7 charter:427). Karpathy lens: at the strict end, LLMs should self-terminate. But cost discipline (CLAUDE.md §13 "writes/append/challenge/verify/settle require stake/escrow/bond") is non-negotiable on TuringOS. The dual-gated design (P5 turn-bounded + LLM `done=true`) is the right shape; the ceiling is a backstop, not the primary terminator. PASS.
- [O3] Per-turn retry budget = 1 (charter:430). Karpathy lens: LLMs occasionally need 2-3 retries with refined prompts (especially on JSON schema failures with weak models). Budget=1 may produce more `predicate_double_fail` session aborts than necessary in production. Suggest watching session-abort rate during §6 witness; if abort rate > 5% with retry=1, atom-1.7 should bump to retry=2.
- [O4] Termination predicate requires turn ≥ 4 (W3 charter:813-816). Karpathy lens: a "10× smarter LLM" (Researcher C §8.1) might cover all 7 required slots in 2-3 turns; turn ≥ 4 floor is arbitrary. PASS for v1 (cost discipline + dialog-naturalness), but worth a comment that future LLMs may push this lower.
- [O5] Charter §1 row 5 of diff table claims "final synthesis ... UNCHANGED: same wrap_spec_md over reconstructed answers list" (charter:36). Researcher A §7.3 explicitly endorses this ("The Software-3.0 transformation is in the interview, not the synthesis"). This is correct scope discipline; do not let scope creep expand this packet to re-LLM-ify the synthesis path.
- [O6] The 8-slot vocabulary `{job, anchor, memory, first_run, robustness, scope, acceptance, mirror}` is invented within this charter, not anchored in a public methodology. Researcher A §2.1 lists JTBD / IDEO / Mom Test / Voss as the methodology basis, but the slot names are TuringOS-specific. This is acceptable v1 — the LLM-readable rubric in `grill_meta_v1.md` references the public methods — but the slot ids themselves are not Karpathy-blessed; they are an interview-design choice. CONCERN level: low; documenting the methodology anchor inside `grill_meta_v1.md` (W1) is the right place, which the W1 brief already mandates (charter:600-603).
- [O7] WebSocket broadcast `SpecTurnAdvanced` / `SpecGrillComplete` (charter:1235-1237). MCP `elicitation` (server-initiated info request) is the wire-level Karpathy/MCP equivalent. The charter is wire-incompatible with MCP elicitation by accident (different message shape, different framing). This is fine — TuringOS is a substrate, not an MCP server — but worth noting if Phase 8+ wants to expose the grill as an MCP endpoint, a translation layer will be needed.

## Per-checklist verdicts

### I. Three load-bearing claims

I.1 (Substrate claim): PASS — Charter §1 (charter:24-39) treats TuringOS as the substrate (kernel/storage: CAS, PromptCapsule, AttemptTelemetry, EvidenceCapsule) and the Meta LLM as the executing CPU. The FC mapping at §2 (charter:108-117) explicitly maps grill turns onto FC1-N4/N5/N7/N9 (Q_t → rtool → externalized output → predicate). Researcher A §1 (DESIGN.md:11-37) is the rubric; charter inherits the framing.

I.2 (Program claim): PASS — The meta-prompt at `assets/prompts/grill_meta_v1.md` (W1 charter:594-649) is the program. cmd_spec.rs (W6 charter:1101-1137) does NOT encode interview logic — it only enforces the JSON wire contract and the predicate suite. The "what's the next question" decision lives in the meta-prompt's rubric ("Build on the LATEST answer. Mirror back."), not in Rust. Verify by inspection: charter §9.W6 explicitly says "Do NOT modify `canonical_questions` (line 293) or `system_prompt` (line 346) — those remain for legacy `--mode static`" (charter:1103-1104). The new driven mode has no canonical_questions equivalent in Rust.

I.3 (Build mode claim): PASS with caveat — The architect/developer specifies role/rubric/termination (in meta-prompt + termination predicate); the LLM produces per-turn behavior (next-question, slot coverage assessment, done declaration). However, the Rust kernel makes per-turn *meta* decisions (retry-or-halt, coverage-state assembly, termination backstop). So "build mode" is hybrid: instructed-not-coded for content, coded for control flow. This is correct factoring for tape anchoring (Researcher B §1.3); flag as the C1 challenge above.

### II. Anti-pattern detection

II.1 ("AI in a web app" smell): CONCERN — W6 brief (charter:1108-1136) describes a Rust `for turn in 1..15` loop that shells out to `turingos llm complete` per iteration. Strict Karpathy reading: this is the Rust loop driving the LLM. Softer reading: the LLM has full agency over what to ask, when to declare done, and how to phrase mirrors — content authority is Karpathy-faithful even if control flow is Rust-resident. The charter §1 row 5 (charter:34) claim "LLM-generated each turn via `turingos llm complete`" is faithful for content but slightly oversells: a more honest restatement is "LLM-authored each turn; Rust orchestrates the per-turn dispatch." Not a VETO; document as C1.

II.2 (Premature framework): PASS — Charter explicitly rejects Researcher C's CCS / agent.toml abstraction (charter:17, charter:90). Researcher C §9 ("premature generalisation" risk, DESIGN.md:728-787) endorses this exact discipline: "two real instances, then ratify." Karpathy's "don't over-abstract" principle and Anthropic SDK's "minimal loop, rely on the model" both support this decision. The right call for v1.

II.3 (Scheduler/worker inversion): CONCERN — By deferring Blackbox triage (charter:16, charter:93, R-3 charter:1469), v1 has Meta doing both scheduler (input triage) and worker (next-question generation) work. Researcher A §2.4 (DESIGN.md:120-128) argues for the explicit split. This is a defensible v1 scope decision but it weakens the Software-3.0 marketing claim. Document as C2.

II.4 (Context stuffing): PASS — Structured-history view (last 3 Q/A pairs + coverage-state summary) per W6 brief (charter:1115-1117). This is Researcher B §4.1 option C and is Karpathy/Anthropic-aligned. The charter avoids the "global context stuffing with historical logs" anti-pattern (CLAUDE.md §15). Note: 3-turn window may be too narrow for very long sessions (15 turns × 3-turn window = LLM never sees turns 1-12 directly in turn 15); if the LLM relies on the kernel-injected coverage-state summary to remember earlier turns, this is correct context engineering. If session coherence degrades, atom-1.7 can revisit the window size.

II.5 (Non-determinism handling): PASS — I-8 (charter:129) + §6 step 5 (charter:320-324) + R-1 (charter:1467) enforce replay-without-recall. Replay reads `candidate_payload_cid` from CAS, never re-invokes SiliconFlow. This is Researcher A §3.3 + Researcher B §6/§8.2. Aligns with Anthropic's "context is a compiled view over richer state" and Karpathy's "non-determinism is a feature, design around it." Strongest Software-3.0 claim in the charter.

### III. LLM-as-CPU contract design

III.1 (Output envelope): PASS with O1 observation — Envelope shape (charter:614-626; W2 charter:683-693):
```
{turn, question_text, covered_slots, open_slots, confidence, done, rationale_brief, playback?}
```
- `turn`: justified (replay anchor, P5 bound).
- `question_text`: justified (the actual LLM product per turn).
- `covered_slots` / `open_slots`: justified (kernel needs explicit state for termination predicate; Researcher A §2.2 + B §3.2).
- `confidence`: justified (gates termination at ≥ 0.8).
- `done`: justified (dual-gated terminator).
- `rationale_brief`: justified as audit signal IF shielded from next-turn context (O1 — add explicit test).
- `playback`: justified (Voss mirror methodology, Researcher A §2.1).
- Missing fields a future LLM would need: `tool_calls` (Researcher C §8.2) — see Weak Claim 4 above. Acceptable v1 deferral.
- Stable: yes (versioned via `system_prompt_template_hash`).
- Minimal: yes for v1 — every field is justified.
- Constrained: yes (machine-checkable via P1-P6).
Field count = 8 is on the upper edge of "minimal" but each is load-bearing.

III.2 (Slot vocabulary): PASS with O6 observation — Vocabulary anchored in stated methodologies (JTBD/Voss/IDEO/Mom Test per Researcher A §2.1) but slot ids themselves are TuringOS-coined. The 8-slot table is constrainingly tight (it could exclude valid interview branches like "team size" or "data sensitivity"), but for v1 a small canonical vocabulary is the right call — it lets P3 (slots_in_canonical_vocab) be a hard predicate. Karpathy lens: "LLM is the interpreter; manifests are the system calls" fits this 8-slot model adequately. If a second grill (cooking, Lean Canvas) requires different slots, that's the trigger to revisit; the constraint is honest and reversible.

III.3 (Termination as predicate): PASS — Dual-gated (charter:62-63, W3 charter:810-816). LLM emits `done=true`; kernel `termination_predicate` checks required slot subset ⊆ covered AND confidence ≥ 0.8 AND turn ≥ 4. Both layers active. Karpathy-aligned: agent self-decides AND kernel backstops.

### IV. Future-AGI readiness

IV.1 (LLM-authored grills): CONCERN — Prompt assets are in a discoverable location (`assets/prompts/`, charter:178-180). BUT W6 brief (charter:1114) hardcodes the path in cmd_spec.rs. A future LLM that writes a new meta-prompt + drops it at `assets/prompts/grill_coaching_v1.md` cannot have it picked up without a code change. This is C3. Recommendation: add `--meta-prompt <path>` flag with default; zero-cost forward-compat hook.

IV.2 (Tool use mid-conversation): CONCERN — TurnPayload envelope (W2 charter:683-693) has no `tool_calls` field. P1 (schema_parse_ok, charter:58) treats any extra field as a parse failure on strict deserialize. A future LLM grill that emits tool calls would VETO under P1. Acceptable v1 lock — the charter (charter:96) explicitly excludes "Multi-LLM concurrent calls per turn" — but worth flagging in §11 as a Phase 6.3.y trigger.

IV.3 (Cross-session memory): PASS — Charter §10 R-6 (charter:1472) says session is in-memory per-process (`AppState.sessions: Arc<Mutex<HashMap<...>>>`). Researcher C §8.3 says cross-session memory needs a `[memory]` block in CCS. v1 default of "no cross-session memory" is correct for the grill (each spec session is independent of prior specs). Karpathy lens: "context window = RAM" framing supports per-session memory; persistent memory is a future additive when Markov capsule replay is green. No regression risk.

IV.4 (Multi-modal): PASS — Charter §7 (charter:425) hard-constrains "No multimodal grill input. Text only." Researcher C §8.4 acknowledges this is a v1-acceptable scope discipline (input side easy when needed; output side needs per-kind emitter). The grill's domain (interviewing a non-developer for a software spec) is text-native. Multi-modal is a Phase 7+ Webcam/voice-grill story, not a Phase 6.3.x regression risk. PASS.

IV.5 (LLM-as-debugger): PARTIAL PASS — Capsule chain (PromptCapsule + AttemptTelemetry + EvidenceCapsule per turn, chained via parent_turn_cid + turn_cids[]) exposes full state for offline replay. A future ArchitectAI inspecting a failed session can reconstruct the conversation view + every predicate verdict + every retry attempt. This satisfies the "LLM can debug its own runtime" claim AT THE AUDIT LAYER. But the in-session LLM (during a live grill) does NOT see prior `rationale_brief` (shielded per R-7). So the LLM cannot self-debug mid-conversation, only post-hoc. Acceptable v1.

### V. Industry alignment

V.1 (MCP elicitation wire compatibility): O7 observation — `POST /api/spec/turn` (charter:75) is shape-incompatible with MCP `elicitation` (server-initiated info request via JSON-RPC). The endpoint is client-driven (browser POSTs user_answer; backend returns next question). MCP elicitation is server-initiated (server emits `elicitation/create` request to client). Different framing. Not a defect — TuringOS is a substrate, not an MCP server — but documenting the wire-incompat means a future MCP exposure of grills will need an adapter layer.

V.2 (Open Agent Spec compatibility): PARTIAL — At the wire envelope level, TurnPayload has `inputs` (user_answer + lang) and `outputs` (question_text + slots + done) which maps cleanly to AgentNode's `inputs/outputs`. `{{slot}}` placeholder syntax is not yet in the meta-prompt (W1 brief shows ROLE/METHODOLOGY/CONSTRAINTS structure, not placeholders), but the slot vocabulary is explicit. A future Open Agent Spec compliance layer would be additive, not redesign.

V.3 (Anthropic Agent SDK alignment): PASS — Charter's per-turn flow is: assemble messages → call LLM → parse + validate → on fail retry-once → on success update state → next turn. This is a minimal loop. 6 predicates is more than zero, but each is independently justified (one schema, one well-formedness, one vocab, one monotonicity, one bound, one language) and they are all pure functions over the envelope. The retry budget of 1 (charter:430) is on the low end but defensible (cost discipline). Aligns with "tool-use-first, minimal loop, rely on the model" — though "tool-use-first" is partial since no tools are exposed to the LLM in v1.

### VI. Specific charter design choices

VI.1 (Turn ceiling = 15): PASS — Cost discipline backstop; LLM self-terminates via `done=true` typically. 15 is a hard ceiling not an expected length (soft target 6-12, W1 charter implicitly via meta-prompt §2.1 "Stop after 6–12 turns"). Karpathy-aligned: agent self-pace + kernel backstop.

VI.2 (Retry budget = 1): O3 observation — Defensible but on the aggressive end. Production data from §6 witness should drive whether to bump to 2.

VI.3 (`rationale_brief`): PASS with O1 caveat — CAS-side, never in next prompt is the correct Karpathy + Anthropic shielding pattern (private CoT discipline). The charter R-7 (charter:1473) names this exactly. Add the explicit test (O1).

VI.4 (Synthesis path unchanged): PASS — Researcher A §7.3 endorses this. The Software-3.0 transformation is in the interview, not the synthesis. Avoiding scope creep here protects the Phase 6.3 spec capsule wire (which took weeks to land). Correct scope discipline.

VI.5 (Multi-LLM choreography deferred): CONCERN — C2 above. v1 = Meta-only. Researcher A §2.4 argues Meta + Blackbox is the OS-scheduler analogy. Deferring weakens the "Software 3.0 native" claim by collapsing to "one model does everything." Architect must ratify the deferral.

### VII. Gut check answer

Verdict: (b) Software 2.5 — the LLM has agency over turn content but TuringOS still drives the loop.

Justification:
- The grill is NOT (c) "Software 1.5 with a chat skin" — that label fits Phase 6.3 today (8 hardcoded questions, LLM only synthesizes), but does NOT fit Phase 6.3.x. Phase 6.3.x gives the LLM real authority: which slot to probe, how to phrase the question, when to mirror back, when to declare done. The 8 hardcoded strings are replaced by LLM-generated text. This is a meaningful upgrade beyond (c).
- The grill is NOT (a) "faithful Software 3.0 — the LLM is the kernel for this surface" — that would require the LLM to drive the loop (deciding "should I retry?" itself), to invoke tools (no tool surface in v1), to do its own input triage (Blackbox triage deferred), and to be reachable as a Software-3.0 substrate where new grills are added by drop-in (Researcher C §8.5 endgame, blocked by C3). The Rust kernel in cmd_spec.rs (W6) still owns the loop, retry policy, and per-turn dispatch.
- The grill IS (b) "Software 2.5 — turn content driven by LLM; turn cadence and control flow driven by TuringOS." This is the honest characterization. Charter §1 (charter:32-39) and §9.W6 (charter:1108-1136) support this reading. The "Software 3.0 native" framing in the charter title is aspirational; the implementation is one half-step short.

VII.2 What would need to change to reach (a):
- The LLM, not the Rust kernel, decides "should I retry this turn?" — implies an envelope field `next_action: continue|retry_self|terminate` and a kernel that respects LLM authority modulo bounded loops.
- Blackbox triage in-scope (C2) — gives the multi-LLM scheduler/worker split Karpathy invokes.
- Tool calls in the envelope (IV.2) — gives the LLM agency to invoke MCP-style tools mid-conversation when a future grill (research-grade interview) wants to fetch a doc, query a calendar, etc.
- Prompt-asset registry (C3) — letting LLM-authored grills drop in without code changes (Researcher C §8.5).
- Termination authority: kernel checks the LLM's `done=true` for safety but doesn't override on missing slots (instead it asks the LLM to revise its plan) — Karpathy's "rely on the model" maxim at termination level.
None of these are necessary for v1 ship; all are reachable in Phase 6.3.y/z without redesigning the Phase 6.3.x substrate.

VII.3 (Researcher C "two real instances, then ratify"): PASS — Charter's deferral of CCS (charter:17, charter:90) buys exactly the optionality Researcher C §9.2 recommends. The Phase 6.3.x substrate (cmd_spec.rs --mode driven + assets/prompts/* + 6 predicates + 8-slot vocab) is one instance. A second grill (Researcher C §3.1 cooking, §3.2 Lean Canvas) ships without CCS first, exposing what actually needs to generalize. At that point CCS or a slimmer alternative gets ratified. The C3 challenge (soft `--meta-prompt` flag) is a near-free amendment that improves the optionality without overcommitting.

## Recommended pre-signature actions

1. **C1 — Title/scope honesty.** Architect decides: retitle to "Software 2.5" OR keep title and add a one-line clarifier in §1 stating that "Software 3.0 native" here refers to per-turn LLM content authority + prompt-as-program + replay-without-recall, NOT to LLM-as-runtime-scheduler. Recommendation: option (b), one-line clarifier.
2. **C2 — Blackbox triage decision.** Architect ratifies one of: (a) defer-and-accept-risk; (b) promote to atom-2 in this packet; (c) defer but commit to atom-1.5 same release window. Recommendation: (c).
3. **C3 — Optional `--meta-prompt <path>` flag.** Add to W4/W6 CLI surface with default = `assets/prompts/grill_meta_v1.md`. Zero-cost forward-compat hook. Recommendation: ratify.
4. **O1 — Shielding test for `rationale_brief`.** Add to §5 exit gates (or §9.W3 acceptance tests): a Class-1 test asserting `rationale_brief` from turn N never appears in the visible context of turn N+1's PromptCapsule. Researcher B §8.6 already names this test. Recommendation: ratify.
5. (Optional) **§1 diff table addition.** Add a row "Loop driver" with both columns saying "Rust (unchanged)" — makes the Software-2.5 nature explicit and prevents future audit drag.

## What this charter must NOT regress in future Phase 6.3.y when CCS is added

1. **Replay-without-recall (I-8).** When CCS abstraction lands, the per-turn capsule chain MUST still be reconstructable from CAS without re-invoking the LLM. Adding a manifest layer cannot break this invariant.
2. **PromptCapsule 7-field schema (Class-4 architect-pinned).** CCS must thread its metadata through `read_set` / `agent_view_manifest_cid` etc.; it must NOT propose adding manifest fields to PromptCapsule itself.
3. **Dual-gated termination.** The "LLM self-declare + kernel slot-coverage predicate" pattern is the canonical Software-3.0 terminator. CCS's `completion = "predicate:<id>"` must compile down to a kernel predicate; it cannot collapse to "LLM says done = session done."
4. **Hard turn ceiling enforced by kernel.** CCS may parameterize `max_turns` but the kernel must still enforce it as a backstop; manifest cannot disable it.
5. **`assets/prompts/grill_meta_v1.md` content + hash.** The Phase 6.3.x meta-prompt becomes the v1 baseline for the spec-grill preset. CCS must preserve byte-identity for `spec-grill-v1` preset → identical spec.md output → byte-compat test I-7 stays green.
6. **Class-4 surfaces still untouched.** `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs::ObjectType` MUST remain unmutated when CCS lands. CCS is a Class-2/3 abstraction over existing primitives; if CCS forces a Class-4 mutation, that is the signal to redesign CCS, not to relax the freeze.
7. **8-slot vocabulary preserved for spec-grill-v1.** Other grills (cooking, Lean Canvas) get their own slot vocabularies; the spec-grill slot vocab is frozen because changing it invalidates Phase 6.3.x EvidenceCapsules.
8. **Replay-without-recall AND byte-identical legacy `--mode static`.** Both invariants from this packet (I-7, I-8) must remain green after CCS ratification. If CCS lands and either of these regresses, that is a VETO event for the CCS charter.
