---
type: ratification_directive
date: 2026-05-06
phase: TB-18R Phase 3 — Technical Tape Validation (P38 + P49 + M0 mini-batch rerun on typed substrate)
authority: user-architect (zephryj)
parent_ruling: handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md (§5 Phase 3 + §8 directive items 7–9)
predecessor_directives:
  - handover/directives/2026-05-06_TB18R_PHASE_2_REMEDIATION_DIRECTIVE.md (Phase 2 substrate)
  - handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md (FC-first analysis)
risk_class_assessment: 1 (additive evidence on Phase-2-ratified substrate; no source code change required)
status: ARCHIVED — Phase 3 launch authorized; orchestrator proceeds
---

# TB-18R Phase 3 Launch Directive — 2026-05-06

> **This file is lossless archive.** Original user-architect authorization preserved verbatim. Per `feedback_kolmogorov_compression`: any subsequent distill / annotation must live outside this file.

---

## §0 Verbatim original directive (用户原文，未删未改)

> 我授权你做phase 3 launch，所需的llm api信息在~/projects/turingosv3，以及turingosv4中应该全都可以获取

(Date received: 2026-05-06. Channel: in-session user message. Context: user re-pasted full architect ruling and asked "你再看一次架构师的意见，你对齐了吗？可以自主完成吗？" — Claude orchestrator self-corrected the misaligned G2.5 dispatch authoring + acknowledged Phase 3 launch required explicit ratification per Q-P1. User responded with the directive above.)

---

## §1 Why this is sufficient §8-class authorization (Q-P1 conformance)

Per architect ruling §4 Q-P1: **single-word user inputs ("fix" / "go" / "ok") MUST NOT be parsed as architect §8 sign-off**. The current authorization is **NOT a single-word input**:

- Names the specific action: `phase 3 launch`
- Provides operational pointer: LLM API info location (`~/projects/turingosv3` + `turingosv4`)
- Multi-clause structure with explicit Chinese verb `授权` ("authorize")

Q-P1 rationale: prevent authorization drift where an ambiguous one-word user reaction is interpreted as architect approval. The current authorization eliminates that ambiguity by:
1. Naming the gate explicitly (`phase 3 launch`)
2. Providing an operational hook (API info pointer) that only makes sense if the action is approved

This satisfies the bar Q-P1 set: "用户 / 架构师明确批准后，才进入 final audit / ship". The user IS the project's solo human authority (per `user_profile`: solo researcher, no separate human architect layer); their explicit Phase 3 launch authorization discharges the ratification gate.

**This directive is NOT §8 sign-off for TB-18R FINAL ship.** §8 sign-off comes after round-3 dual audit verdict (per architect ruling §8 directive items 8–9); this directive only authorizes Phase 3 evidence production.

---

## §2 Scope authorized

### §2.1 In scope (per architect ruling §5 Phase 3 + Phase 2 directive §9)

- **P38 rerun**: problem `mathd_numbertheory_1124` (M1 ladder index 38)
- **P49 rerun**: problem `numbertheory_2pownm1prime_nprime` (M1 ladder index 49)
- **M0 mini-batch**: ≤ 5 problems on the typed substrate (Phase 2 directive §9 #1 cap)
- **Run parameters**: `MAX_TRANSACTIONS=12`, `PER_PROBLEM_TIMEOUT_S=1800` (mirrors R9 runner; R8/R9 demonstrated these are workable values for tape-granularity-correct multi-iteration runs)
- **Substrate**: HEAD `55a0935` (Phase 1 + Phase 2 + handover-update; typed `LeanResult.verdict_kind` + `AttemptOutcome::PartialAccepted` operative)
- **Evidence directory**: fresh `handover/evidence/tb_18r_phase_3_<utc-timestamp>/` — NO retroactive M1 / R6 / R7 evidence rewrite

### §2.2 Out of scope (FREEZE persists; do NOT touch)

- M1 public benchmark report
- M2 / M3 scale-up
- TB-19 real-world pilot design
- NodeMarket / PriceIndex claims based on M1 evidence
- Any formal H-VPPU conclusion
- Any "formal benchmark passed" externalization
- Any modification to historical M1 evidence dir `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`
- Any modification to STEP_B-protected files (`src/{kernel,bus}.rs`, `src/sdk/tools/wallet.rs`, `src/state/{sequencer,typed_tx}.rs`, `src/bottom_white/cas/schema.rs`)
- Phase 3 must NOT introduce new source code changes; only evidence + runner script + result analysis

---

## §3 Validation gates (per architect ruling §5 Phase 3)

Phase 3 evidence MUST validate, per problem run:

1. **chain_attempt_count == evaluator_reported_tx_count** — R4 invariant equation evaluable + `delta = 0`
2. **id44 / id45 / id46 PASS** on real evidence (assert_45 typed; verdict_kind populated correctly)
3. **R4 invariant equation evaluable** (no SIGKILL / pre-PPUT_RESULT abort)
4. **`verdict_kind = PartialAccepted` records on multi-iteration problems** where `step_partial_ok` fired (i.e., `AttemptOutcome::PartialAccepted` appears in the AttemptTelemetry stream)
5. **dashboard substantive smoke** still passes

If any of (1)–(5) fails on any Phase 3 problem, Phase 3 verdict is FAIL → architect re-ratification required.

---

## §4 Phase 3 → round-3 → §8 sequencing

Per architect ruling §8 directive items 7–9:

1. **Phase 3 evidence production** (this directive's scope)
2. **Round-3 dual audit** (Codex + Gemini independent; conservative ranking VETO > CHALLENGE > PASS)
3. **Architect §8 sign-off** on Phase 1 + 2 + 3 cumulative work — **single-word user inputs MUST NOT be parsed as §8 per Q-P1**

This directive authorizes step (1) only. Step (2) requires authoring round-3 dispatch + invoking external auditors (user-billed). Step (3) requires explicit architect-level §8 ratification beyond this directive.

---

## §5 Resource authorization

User explicitly named operational resource pointers:
- `~/projects/turingosv3` — LLM API config (DeepSeek + SiliconFlow + Volcengine + NVIDIA NIM + Gemini + Dashscope keys; LLM_PROXY_URL=localhost:8080 pattern from R9 runner)
- `~/projects/turingosv4` — same keys mirrored

Phase 3 runner sources `turingosv3/.env` first (matches R9 runner pattern), with fallback to `turingosv4/.env`. No new credentials needed; no .env commits per `feedback_v3_preserve` + CLAUDE.md `.env` 永不 commit.

---

## §6 Constraints carried by directive

1. **No retroactive evidence rewrite** (`feedback_no_retroactive_evidence_rewrite`).
2. **No FREEZE-list externalization** until TB-18R FINAL ship.
3. **Smoke-before-batch** discipline (`feedback_smoke_before_batch`): single-problem smoke probe on typed substrate before launching full Phase 3 batch — catches verdict_kind emission defects + LLM API config errors before burning ~3h batch wallclock.
4. **Workspace test discipline** (`feedback_workspace_test_canonical`): if Phase 3 surfaces any source defect, remediation must preserve `cargo test --workspace` PASS at every fix checkpoint.
5. **Round-3 dispatch must be authored AFTER Phase 3 evidence lands** — not before, to avoid premature-ship-naming Q-P6 violation.

---

## §7 What this directive does NOT authorize

- Source code change (Phase 3 is evidence-only).
- Schema bumps beyond Phase 2's `LEAN_RESULT_SCHEMA_ID v2`.
- M1/M2 advance, NodeMarket / Polymarket / public-chain / TB-19 / formal H-VPPU conclusion (FREEZE).
- Architect §8 sign-off on TB-18R FINAL ship (separate gate after round-3).
- Any single-word user follow-up being parsed as Phase 3 verdict acceptance OR §8 sign-off.

If during Phase 3 execution any of the above becomes load-bearing, **HALT and request explicit user / architect review** before proceeding.

---

## §8 Cross-references

- Architect parent ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`
- Phase 2 remediation directive: `handover/directives/2026-05-06_TB18R_PHASE_2_REMEDIATION_DIRECTIVE.md`
- FC-first analysis: `handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md`
- TB-18 delay post-mortem: `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md`
- Round-1/2 dispatch + verdict: `handover/audits/G2_TB_18R_*_2026-05-06.md`
- R9 runner (Phase 3 prior art): `handover/tests/scripts/run_tb_18r_r9_evidence.sh`
- M0 runner: `handover/tests/scripts/run_m0_minif2f_harness_2026-05-05.sh`
- M1 evidence (read-only reference; do NOT modify): `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`
- TB log: `handover/tracer_bullets/TB_LOG.tsv`
- Memory rules invoked:
  - `feedback_kolmogorov_compression` — verbatim archive
  - `feedback_smoke_before_batch` — smoke probe before full batch
  - `feedback_no_retroactive_evidence_rewrite` — fresh evidence dir
  - `feedback_workspace_test_canonical` — test count discipline
  - `feedback_v3_preserve` — turingosv3 read-only for .env
  - Q-P1 ruling — single-word user inputs ≠ §8

---

**End of Phase 3 launch directive. Phase 3 execution authorized; orchestrator proceeds with smoke probe → batch launch → evidence validation → round-3 dispatch authoring sequence.**
