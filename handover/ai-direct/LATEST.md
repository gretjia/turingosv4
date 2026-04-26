# TuringOS v4 — Handover State

**Updated**: 2026-04-26 (Phase C scaffolding 100% complete; ready to launch C2 100-row batch)
**HEAD commit**: `4f981cd` (C5: mode_flag_binary_purity test) — C2 runner add still uncommitted at writing
**Origin**: `origin/main` synced through `4f981cd` (60+ commits pushed this UTC date across two phases)

## Session Summary

This session continued from Phase A→B exit (commits 60292dc..136b7f5) into Phase C scaffolding (1d04f6a..4f981cd + uncommitted C2 runner). **Phase C 7/9 atoms shipped + C2 runner ready**:
- C-pre1: hard-10 deterministic freeze (sealed sha256 `6667e6bdd2aa381c…`)
- C1a-e: 5 ablation modes wired (Full/SoftLaw/Homogeneous/Panopticon/Amnesia) via 4 pure helpers (apply_mode_to_accept / skill_index_for_agent / is_panopticon / is_amnesia)
- C5: mode_flag_binary_purity inline test (binary-identity discipline)
- C2 runner: `run_c2_phase_c_ablation.sh` — `--smoke` validated 1/5 modes end-to-end (Homogeneous, 4 min wall-clock); 4/5 modes timeout at 5 min cell limit (heterogeneous-skill thinking-on path is slower)

**Phase A→B exit (prior portion of session)**: 13-round dual-audit cycle, 14 substantive findings caught + closed; latest R13 verdicts CHALLENGE/PASS — audit gate at asymptote. Harness amplifier C-076 + R-020 sedimented.

> **新 session 入口**: read this file + `handover/ai-direct/HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md` (this session's Phase C handover with C2 launch decision tree) + `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 6 (Phase C protocol) + § 9 (statistical plan) + `handover/preregistration/scripts/run_c2_phase_c_ablation.sh` (the C2 batch runner). 这 4 个文件足以无 context 接手。Phase A handover (`HANDOVER_PHASE_A_EXIT_2026-04-26.md`) + A8 audit history + EXIT_PACKET remain authoritative for prior context.

## Current State

### Active research arc
**PPUT-driven Capability Compilation Loop (CCL)** — 30-day arc 2026-04-26 → 2026-05-26.
- North Star: Held-out Verified PPUT (H-VPPUT) on heldout-54
- Success criterion: WBCG_PPUT > 0 (≥1 Certified user-space artifact)
- Caps: 30 wall-clock days + USD 500 API budget (硬停)
- Backbone: `deepseek-v4-flash` thinking-off (Phase B+C); 异构 LLM at Phase D (v4-flash thinking-on + Gemini 2.5 Pro + SiliconFlow catalog via A7 plumbing)

### Phase A — COMPLETE (atoms A0–A7) + A8 audit gate cleared
Phase A engineering atoms shipped in prior mid-stream session (commits 6be6eb4 .. 90953d6):
- **A0a–e ✅** harness modernization (rules + cases + TRACE_MATRIX_v2)
- **A1 ✅** PREREG amendment p_0 calibration deferral
- **A2 ✅** swarm_N=1 mode + parse_swarm_condition_n
- **A3 ✅** AGENT_MODELS env var + Phase B+C single-model gate
- **A4 ✅** decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
- **A5 ✅** BUDGET_REGIME + MAX_TRANSACTIONS env vars
- **A6 ✅** fc_trace.rs + 7-variant FcId enum + 9 wired anchor sites
- **A7 ✅** SiliconFlow heterogeneous-LLM plumbing (proxy + 3-key smoke)

A8 audit gate (this session, commits 60292dc .. 50b5afc):
- **A8 prep + 13 dual-audit rounds + 15 in-cycle fix bundles (A8e..A8e15)**
- Real-bug yield: 14 substantive findings caught + closed
- Documentary lessons sedimented: case C-076 + rule R-020 (commit-claim diff parity)
- Trust Root hardened: recursive child-manifest verification (A8e13 Q1); src/boot.rs ALSO in TR
- Cost: ~$80 / $500 cap = 16% spend

### Phase B — DONE (B1-B7 from prior session; B7-extra deferred per amendment)
Per `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md`:
- **B1–B7 ✅** all green; tests + Trust Root + smoke + conformance battery passing
- **B7-extra ⏸ DEFERRED** per `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (5 conditions must complete first; operationally pushed to post-Phase D)

### Phase C — STARTING POINT for next session
Per `AUTO_RESEARCH_NOTEPAD.md` § Active roadmap:
> **Phase C — Ablation smoke tests** (days 11-17)
> - 5 modes: Full / Panopticon / Amnesia / Soft Law / Homogeneous
> - hard-10 adaptation × N=20 paired
> - Verify H1–H4: violations show on PPUT axis

Next session reads `PREREG_PPUT_CCL_2026-04-26.md` § 2 + § 5 + § 6 (Phase C protocol + H1-H4 hypotheses + statistical plan), then implements + smokes the 5 mode toggles.

## Verified state at HEAD

| Metric | Value |
|---|---|
| `cargo test --workspace` | **267 PASS / 29 ignored / 0 failed** |
| `python3 scripts/test_llm_proxy.py` | **16/16 PASS** (also wrapped in cargo test) |
| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys live)** |
| Trust Root manifest | **38 entries**, recursive child-manifest enforcement live |
| `boot::tests::verify_trust_root_passes_on_intact_repo` | **PASS** |
| Cases (C-001..C-076) | 76 (C-076 added in A8e12) |
| Active rules (R-001..R-020 with gaps) | 15 (R-020 added in A8e12) |
| FC-trace anchor sites (evaluator.rs) | 9 (run_swarm × 8 + run_oneshot × 1) |
| `make_pput` arity | 24 positional args (Phase B+ refactor candidate) |
| Git commits ahead of `origin/main` | 0 (synced 2026-04-26) |

## What this session did NOT do (per user honest-framing question)

- **Not DO-178C**: 13 rounds were adversarial dual external review (Codex + Gemini, skeptical-reviewer mandate). Case C-075 invokes DO-178C tool-qualification *as analogy*; the cycle did not produce DO-178C planning artifacts (PSAC/SDP/SVP), DAL declarations, structural coverage analysis, or formal TQL-1..TQL-5 tool qualification. Research-grade rigor, not certified-avionics rigor.
- **Not just "no constitution.md edits"**: zero edits is necessary but not sufficient. Constitutional alignment per substantive fix verified against FC1/FC2/FC3 invariants and Article rules — see `HANDOVER_PHASE_A_EXIT_2026-04-26.md` § 6 for per-fix retrospective.

## Reference (canonical sources of truth)

### A8 audit gate (this session)
| 文件 | 用途 |
|---|---|
| `handover/ai-direct/HANDOVER_PHASE_A_EXIT_2026-04-26.md` | **This session's handover** — full Phase A→B exit retrospective |
| `handover/audits/A8_EXIT_PACKET_2026-04-26.md` | Current-state Phase A exit packet (post-A8e15) |
| `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` | Append-only 13-round chronology + per-round verdicts/fixes |
| `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26[_R2..R13].md` | 13 rounds × 2 auditors = 26 audit transcripts |
| `handover/audits/run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py` | Audit runners (in Trust Root per A8e11; require A8_AUDIT_ROUND env per A8e10) |
| `cases/C-076_commit_claim_diff_parity.yaml` | A8e12 false-closure prevention precedent |
| `rules/active/R-020_commit_claim_diff_parity.yaml` | A8e12 pre-commit WARN rule |

### Phase A engineering atom code (mid-stream session)
| 文件 | 用途 |
|---|---|
| `experiments/minif2f_v4/src/agent_models.rs` (A3) | Per-agent model assignment + Phase B+C single-model gate |
| `experiments/minif2f_v4/src/budget_regime.rs` (A5) | BUDGET_REGIME enum + MAX_TRANSACTIONS resolver |
| `experiments/minif2f_v4/src/fc_trace.rs` (A6) | Structured JSON event emitter + FcId enum |
| `experiments/minif2f_v4/src/run_id.rs` (A8e F1) | Single per-run identifier minted once, threaded everywhere |
| `experiments/minif2f_v4/src/jsonl_schema.rs` (A4) | v2 schema with hit_max_tx + tactic_diversity + verifier_wait_ms + budget_regime + budget_max_transactions fields |
| `src/boot.rs` (A8e13 Q1) | Trust Root verifier; recursive child-manifest enforcement |
| `src/drivers/llm_proxy.py` (A7) | Multi-key round-robin OpenAI-compatible proxy (in TR per A8e11) |
| `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py` (A7) | 3-key fail-closed smoke (in TR per A7) |
| `scripts/test_llm_proxy.py` (A8e F2) | 16-test routing + round-robin conformance (in TR per A8e2) |

### PPUT-CCL arc (frozen contracts)
| 文件 | 用途 |
|---|---|
| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | Round-4 frozen pre-registration; 总章法 |
| `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md` | p_0 calibration deferral; § 2 + § 8 wording corrected via A8e F6 + G2 + M4 + N1 |
| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | 三 split frozen output + sealed hash |
| `handover/preregistration/scripts/split_pput_ccl.py` | 可重现 split 生成 |
| `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` | Phase B detailed implementation (B1-B7 DONE; B7-extra deferred) |
| `handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md` | Architect v1 measure-theoretic FULL PASS |
| `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md` | Architect v2 ontological FULL PASS |
| `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md` | PREREG round-4 PASS/PASS verdict |

### Constitutional alignment + handover meta
| 文件 | 用途 |
|---|---|
| `handover/alignment/TRACE_MATRIX_v2_2026-04-25.md` | FC↔code alignment; § 1 has A0a..A8e14 trigger entries |
| `handover/alignment/FC_ELEMENTS_2026-04-22.md` | Canonical FC node IDs |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` | Active research state (memory `project_auto_research_notepad` points here) |
| `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` | Pending user decisions (D1-D4 all RESOLVED 2026-04-26) |

### Memory entry points (auto-loaded per session)
- `MEMORY.md` indexes `project_pput_ccl_arc.md` → points here (`LATEST.md`)
- `feedback_phased_checkpoint.md`, `feedback_dual_audit*.md`, `feedback_step_b_protocol.md` are critical for Phase B+ execution discipline
- `reference_siliconflow.md` (NEW this session) — SiliconFlow as Phase D heterogeneous lane + context-loss anti-pattern lesson

## Repo state
- HEAD: `50b5afc` (A8e15)
- origin/main: `50b5afc` (synced; 54 commits pushed this session)
- Working tree: `rules/enforcement.log` modified (session-runtime artifact, do not stage)
- Tags pushed (prior): `paper1-v2.1.1`, `archive/art-ii1-v3-abandoned-20260416`
- Branches: `main` (active), 23 archive refs preserved

## Compute spent (cumulative across all sessions)
- Phase A PREREG dual-audit (4 rounds, mid-stream session): ~$15-20
- Phase B B2-B4 mid-term audit (mid-stream session): ~$3-5
- Phase A → B exit dual-audit (this session, 13 rounds): ~$80
- **Cumulative arc spend**: ~$100 / $500 cap = 20%
- Remaining: ~$400 for Phase C ablation (5 modes × 10 problems × 2 seeds = 100 jsonl rows + audit) + Phase D shadow CCL + Phase E sealed eval + B7-extra calibration if/when § 3 conditions complete

## Next-session boot sequence

1. Read 4-file list at top of this doc (HANDOVER_PHASE_C_SCAFFOLD + this LATEST + PREREG § 6/9 + run_c2 runner)
2. Re-verify state: `cargo test --workspace` (expect **298 PASS**), `bash scripts/smoke_siliconflow.sh` (expect 3/3 PASS)
3. Read `HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md` § 3 (C2 launch decision tree) + § 4 (open questions)
4. Smoke-then-launch C2: re-run `bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh --smoke` (~5-25 min); if Homogeneous succeeds end-to-end, decide between **Path A serial overnight** (~25-50 hours, ~$13-25), **Path B parallel-runner upgrade** (~5-10 hours after engineering), or **Path C reduced scope** (lower stat power)
5. After C2 batch: C3 (H1-H4 McNemar paired sign tests on 100 rows) + C4 (CHECKPOINT_PHASE_C dual external audit)
