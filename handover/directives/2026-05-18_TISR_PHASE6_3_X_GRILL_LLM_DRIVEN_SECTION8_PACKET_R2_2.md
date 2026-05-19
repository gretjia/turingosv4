# TISR Phase 6.3.x — Charter R2.2 §8 Amendment: LOC Cap Revision

**Revision**: R2.2 (post-W8 §8 amendment; pre-W9 evidence)
**Date**: 2026-05-18
**Base**: R2 §A15 LOC budget
**Architect authorization**: chat 2026-05-18 (explicit ratification: "同意你的建议，立即执行")
**Trigger**: W0-W8 completed; actual new-code LOC exceeds R2 §A15 hard cap

## §A17. LOC budget revision

### Actual LOC (post-W8, pre-W9)

Branch `codex/tisr-phase6-3-x-grill-driven` at HEAD `4e80a8f6`. Compared with `origin/main`:

| Atom | File(s) | New source LOC |
|---|---|---|
| W2 | `src/runtime/grill_envelope.rs` | 133 |
| W3 | `src/runtime/grill_predicates.rs` | 273 |
| W4 | `src/bin/turingos/cmd_llm.rs` (complete branch) | 607 |
| W4.5 | `src/bin/turingos/cmd_llm.rs` (triage branch) | 419 |
| W5 | `src/bin/turingos/spec_capsule.rs` (grill writers) | 379 |
| W6 | `src/bin/turingos/cmd_spec.rs` (--mode driven) | 1159 |
| W7 | `src/web/{spec.rs, ws.rs, router.rs, mod.rs}` | 932 |
| W8 | `frontend/src/{spec-grill.ts, ir.ts, types/spec.ts}` | 532 |
| | **Source total** | **~4434** |
| | Test code (Rust + TS) | ~1807 |
| | Prompt assets (markdown) | 163 |
| | **Grand total touched** | **~6404** |

R2 §A15 hard cap: **1700 LOC source**. Actual: **2.6x over** (source) / **3.7x over** (incl. tests).

### Architect ratification

This R2.2 ratifies the overrun per §8 amendment authority. The new caps:

- **Source code hard cap: 5000 LOC** (was 1700; comfortably covers 4434 actual + small margin)
- **Test code soft cap: 2500 LOC** (was implicit ~800; covers 1807 actual)
- **Total Phase 6.3.x touched LOC: 7500 hard cap** (covers 6404 + future W9/W10 evidence + 10% growth margin)

W10 audit checklist item C9 ("Total LOC added ≤ 1500") is **superseded** by this amendment: the new C9 check is "Total source LOC added ≤ 5000".

### Justification (why the overrun is real not negligence)

Real complexity outweighed R1/R2 estimates:

1. **W4 complete CLI (607 vs ~150 estimated)**: full async/tokio integration with siliconflow_client + 9-flag arg parser + PromptCapsule write per R2 §A1/§A2 + structured stdout JSON + 4 typed error categories with exit-code mapping. Estimate was a thin wrapper; reality is a self-contained Software-3.0-callable per Researcher A §4.
2. **W4.5 triage CLI (419 vs ~150 estimated)**: same shape as W4 but with Blackbox-specific prompt extraction + Qwen3 `<think>` stripping + class enum validation. Mirroring W4 doubled overhead.
3. **W6 cmd_spec driven loop (1159 vs ~250 estimated)**: integrates W4 + W4.5 + W5 + W3 + existing Phase 6.3 synthesis. Includes CoverageState struct + GrillAttemptTally + per-turn message assembly + shell-out helpers (sha256_hex / generate_session_id / cas_store_user_answer / build_turn_prompt_json / parse_*_from_llm_output) + retry-once logic + triage routing per R2 §A5 + termination handling. Each helper is small but they add up.
4. **W7 web /api/spec/turn (932 vs ~220 estimated)**: web-layer mirror of W6's flow with axum handler + 3 WS broadcast variants + AppState.sessions thread-safety + spawn_blocking wrappers for shell-outs + 4 typed error categories + extensive request/response validation.

The estimates were honest at charter-write time; the gap is the difference between "imagined architecture" and "what compiles + passes tests on macOS Rust 2024 with serde + tokio + axum 0.7". Phase 7 Web MVP showed similar pattern (W5 was ~700 LOC vs ~300 estimated).

### What this amendment does NOT change

- Class-4 surfaces still untouched (§3 forbidden list)
- No new Cargo dependencies
- No new ObjectType variants
- No Trust Root rehash
- Atom dependency graph (R2 §A16) unchanged
- W9 + W10 gates (real LLM + clean-context audit) still required

### Forward-going

R2.2 covers Phase 6.3.x ship. Phase 6.3.y (if cooking/Lean Canvas/code-review grills materialize) should re-estimate LOC honestly with Phase 6.3.x as the reference point. A future R3 may push toward 3000-LOC cap if CCS abstraction (Researcher C) lands.

## §A18. W10 audit split (R2.2 addition)

Per user direction 2026-05-18 (option 2(b)): W10 audit runs in **two rounds**:

- **W10-R1 (pre-W9, static-diff audit)**: Opus xhigh reviews the diff + charter against 5 checks that don't need real-LLM evidence (C1 / C7 / C8 / C9-revised / C10 + architectural read).
- **W10-R2 (post-W9, evidence audit)**: Opus xhigh reviews W9's `agent_verdict.json` + `cas_walk_output.txt` + `replay_diff.txt` + `legacy_byte_compat_hash.txt` against the 5 evidence-dependent checks (C2 / C3 / C4 / C5 / C6).

Rationale: W9 costs ~75 min wall clock + real SiliconFlow API spend. Running W10-R1 first (~30 min cheap) flags any catastrophic issue in the diff before committing to W9. If W10-R1 returns VETO or major CHALLENGE, we fix-and-redispatch before W9; if PROCEED, W9 runs with confidence.

This is consistent with AGENTS.md §9 "one clean-context Codex audit" (interpreted as one logical audit, executed in two evidence-aligned rounds).

## End of R2.2
