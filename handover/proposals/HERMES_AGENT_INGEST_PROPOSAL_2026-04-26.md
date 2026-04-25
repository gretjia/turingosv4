# Hermes-Agent + TuringOS PPUT-CCL — Integration Proposal

**Date**: 2026-04-26
**Status**: Draft — awaits user approval (logged as D4 in `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md`)
**Author**: Claude Opus 4.7 (with user 2026-04-26 directive: "能否用现在的 turingos 架构去做升级自己的项目, 学习 nousresearch/hermes-agent")
**Cross-refs**: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` (frozen at A5), `handover/ai-direct/LATEST.md`

---

## 1. What hermes-agent is (so we are arguing about the same thing)

**Hermes-Agent** = Nous Research's production-capable self-improving agent framework. Key facts:

| Dimension | Hermes-Agent | TuringOS PPUT-CCL |
|---|---|---|
| Stack | Python 87% / TS 9% | Rust 2021 + Python helpers |
| Stage | Production (running on $5 VPS, GPU clusters, serverless) | 30-day research arc, Phase A complete, Phase B starting |
| Self-improvement | Built-in learning loop creates skills from experience; agent-curated memory; periodic nudges | ArchitectAI distills L_t → user-space artifacts; AuditorAI gates; 4-state ArtifactState (Accepted/Quarantined/Certified/Reverted) |
| Skill format | agentskills.io standard (`name` / `description` / `instructions` / etc.) | TuringOS native artifact schema (`artifact_id` / `source_log_hashes` / `state` / `meta_predicate_results` / `pput_gain_*`) — see PREREG § 6 D3 |
| Memory layer | FTS5 session search + LLM summarization | Tape Q^world (versioned) + L_t archive (read-only, ArchitectAI sole reader) |
| Backend | Multi-platform (Telegram/Discord/Slack/...), 6 terminal backends, cron, subagent spawning | Lean 4 oracle, single-task minif2f benchmark, Boltzmann routing swarm |
| Domain | General agent (chat-shaped tasks) | Formal proof verification (constitution-bound) |

**Critical observation**: hermes-agent is **what TuringOS PPUT-CCL aspires to be at scale**. Two systems with overlapping ambition, very different designs, very different domains.

---

## 2. Five interpretations of "use TuringOS to learn from hermes-agent"

| # | Interpretation | Coherence | 30-day Risk |
|---|---|---|---|
| A | Feed hermes-agent source code as `L_t` for ArchitectAI to distill | **Incoherent** — `L_t` per PREREG § 1.5 / Patch 5 = "rejected proposals + golden-path traces from THIS system's runs", not external project source. Violates Trust Root sealing of L_t scope. | High (PREREG amendment + restart audit) |
| B | Manual skill transfer: PI reads hermes-agent docs, copies useful skills into TuringOS user_space | Outside PPUT-CCL arc; this is regular software engineering. Doesn't test the CCL hypothesis. | Zero |
| C | Rebuild TuringOS Phase D ArchitectAI/AuditorAI on hermes-agent foundations | Massive scope creep — different language, different skills system, different backend. Invalidates 4-round-audited PREREG. | Catastrophic (arc restart) |
| D | Use hermes-agent itself AS the Phase D ArchitectAI/AuditorAI runtime (substitute for v4-flash thinking-on + Gemini) | Architecturally interesting but: (i) violates frozen PREREG § 12.2 heterogeneous-LLM choice, (ii) brings unrelated production complexity (multi-platform messaging etc.), (iii) gets hermes-agent's results, not TuringOS' — short-circuits the experiment | High (PREREG amendment + audit) |
| E | Use hermes-agent as a **post-arc external-validity benchmark** for the same heldout-54 | Doesn't change PREREG; runs as Phase F (day 31+); answers "does TuringOS' constitutional CCL beat / match a production-grade general CCL?" | Zero (post-arc) |
| F | **Adopt agentskills.io artifact frontmatter** so TuringOS-generated artifacts are portable to hermes-agent and other agentskills consumers | Small textual amendment to PREREG; aligns artifact schema; tests via the same conformance battery | Low (1 round formal addendum + dual re-audit, ~$5) |

**Recommendation**: pursue **E + F combined**, both gated on user approval.

Why E + F:
- E gives the comparative-benchmark answer that Paper 2-style reviewers will demand ("yes, but does it beat existing tools?")
- F is a hygiene win — TuringOS-generated artifacts become portable beyond TuringOS, which is the only way they meaningfully test "white-box capability" (closed-format = vendor lock-in = not real white-box)
- Neither risks the 30-day arc PASS criterion

---

## 3. Concrete plan — Option F (artifact schema alignment, in-arc)

### F.1 — minimal PREREG amendment

Add to PREREG § 6 D3 (artifact JSONL row) — **artifact frontmatter MUST be compatible with agentskills.io schema**:

```yaml
# Required (TuringOS native + agentskills.io compatible)
name: "<artifact_id>"
description: "<one-line summary>"
when_to_use: "<scope predicate per PREREG § 3.5 docs_include_scope_and_expiration>"
expires: "<ISO date | 'permanent' with justification>"

# TuringOS-specific (agentskills consumers will ignore unknown keys gracefully)
artifact_id: "<unique>"
state: Accepted | Quarantined | Certified | Reverted
source_log_hashes: [<sha256>, ...]
generated_by: "ArchitectAI"
audited_by: "AuditorAI"
meta_predicate_results: { ... }
estimated_pput_gain_prior: <float>
actual_pput_gain_meta_val: <float | null>
used_count: <int>
rollbackable: true
```

This is a **change to the artifact emit format only**. No change to:
- PPUT formal definitions (§ 1)
- 3-split protocol (§ 2)
- conformance batteries (§ 3 / § 3.5 / § 3.5.1)
- hypotheses + multiplicity (§ 5 / § 9)
- Phase A-E execution plan (§ 6)
- FINAL PASS gates (§ 7)

### F.2 — formal addendum process (per PREREG § 14)

1. Write `PREREG_PPUT_CCL_2026-04-26_ADDENDUM_F1_2026-04-XX.md` (one-page; format spec + rationale)
2. Submit to Codex + Gemini round-5 (single-round, narrow scope)
3. Expected verdict: PASS/PASS (small textual change, no statistical impact)
4. On PASS/PASS: implement in B5 ArchitectAI emit code (Phase D delivery)

**Estimated impact**: +1 day Phase B (artifact emitter format), ~$3-5 audit cost. Inside budget.

### F.3 — payoff

Generated artifacts in `user_space/Δ_*/` are loadable as agentskills by:
- Hermes-Agent
- Anthropic Claude Code (skills are first-class)
- Anthropic API + Skills feature
- Any agentskills.io consumer

**This is the strongest "white-box capability compilation" demonstration we can make**: not just "we generated a rule", but "we generated a portable skill that another agent system can use".

---

## 4. Concrete plan — Option E (post-arc benchmark, day 31+)

### E.1 — Hermes-Agent for Lean math configuration

Hermes-agent is general-purpose; it doesn't ship Lean tooling. Setup requires:

1. **Configure backend**: install hermes-agent on a machine with Lean 4 + Mathlib pre-built (we have this at `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/`)
2. **Wire Lean oracle as a hermes tool**: write `hermes_tools/lean_verify.py` exposing `verify_omega_detailed` to hermes-agent's tool registry
3. **Configure model**: use `deepseek-v4-flash thinking-off` to match TuringOS backbone (apples-to-apples)
4. **Enable hermes' built-in skill-learning loop**: agent will create skills from successful proof attempts naturally

**Engineering cost**: 1-2 weeks (1 person, full-time). NOT in 30-day arc; explicitly post-arc.

### E.2 — Benchmark protocol (Phase F — separate arc)

```
Phase F start condition: PPUT-CCL Phase E complete (PASS or NEGATIVE).
Phase F is a SEPARATE pre-registration (PREREG_HERMES_BENCHMARK_2026-05-XX.md).
  - same heldout-54 (already burned by TuringOS Phase E? if yes → use a fresh draw from MiniF2F/Valid)
  - same backbone (deepseek-v4-flash thinking-off)
  - same wall-clock budget per problem
  - measure: H-VPPUT_hermes, FAR, RR, CPR, IAC
  - compare: H-VPPUT_TuringOS_post_CCL vs H-VPPUT_hermes
  - hypothesis (one-sided): TuringOS_post_CCL ≥ hermes (constitution-bound CCL ≥ general-purpose self-improvement on Lean math)
```

**Outcomes**:
- TuringOS WINS: validates constitution-bound approach as superior in formal-verification domain
- Hermes WINS: TuringOS' constraint overhead doesn't pay off vs production-grade general agent — important null result, motivates redesign
- TIE: both approaches work; the constitution gives reproducibility / safety benefits, hermes gives ergonomic / production benefits — joint paper

This is **Paper 2** material. NOT 30-day arc gating.

---

## 5. What we are explicitly NOT doing

- ❌ Replace Phase D ArchitectAI/AuditorAI with hermes-agent (Option D) — short-circuits the CCL hypothesis
- ❌ Rewrite TuringOS Rust kernel in Python on hermes-agent (Option C) — invalidates 4-round audit
- ❌ Treat hermes-agent source as L_t (Option A) — violates L_t scope
- ❌ Adopt hermes' multi-platform messaging / cron / subagent infrastructure — out of scope for math benchmark

These would all be coherent SEPARATE projects. Don't conflate with the 30-day PPUT-CCL arc.

---

## 6. User decisions required

**D4-a — Approve Option F (artifact schema alignment, in-arc, +1 day)?**
- Default if no response: skip; TuringOS-native schema only; revisit post-arc.

**D4-b — Approve Option E (Phase F post-arc benchmark)?**
- Default if no response: defer; review post-Phase-E results before committing.

**D4-c — Anything else from hermes-agent worth pulling that I missed?**
- E.g., FTS5 memory search for L_t (very useful for ArchitectAI's distillation step in Phase D — could be Option F.5)
- Subagent spawning patterns (TuringOS swarm has this differently)

If user replies with simply "F + E approved, set up F now": proceed with F PREREG addendum draft + dual round-5 audit during Phase B B1-B2 (no schedule slip).

---

## 7. One-line recommendation

> **Pursue F now (low-risk, high-portability win) + log E for Phase F post-arc decision; do NOT entangle hermes-agent inside the 30-day PPUT-CCL arc.**
