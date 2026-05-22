# Kernel Integrity Coverage Matrix — Design

**Date**: 2026-05-22  
**Branch**: `claude/generative-html-kernel-probe-20260522`  
**Agent**: Kernel Agent B (adversarial/structural matrix)  
**Scope**: Complement Agent A's persona-driven matrix (natural variation)

## Agent B Problem Set (8 required + 1 optional)

| ID | Slug | Kernel Surface | Expected Outcome | Actual Outcome | Status |
|----|------|---------------|-----------------|----------------|--------|
| P01 | adversarial_injection | FC1-shielding (artifact sanitization) | XSS escaped in artifact | Spec contains raw XSS; generate 401 blocked | PARTIAL |
| P02 | length_limit | FC1-N5 (trust boundary validation) | 4096 pass, 4097 reject | Correct on both paths | PASS |
| P03 | mode_drift | FC3 meta-architecture (synthesis filter) | Refuse/coerce PDF request | Absorbed into spec as feature | FAIL |
| P04 | impossible_network | FC1 generate/verify shielding | L4.E reject (network-dependent) | Absorbed; verifier has no network check | PARTIAL |
| P05 | contradictory_spec | FC3 meta-architecture (contradiction handling) | Contradiction surfaced | Correctly surfaced in "矛盾" section | PASS |
| P06 | multilingual_mixed | FC1-N5 (LLM language handling) | Normalize to Simplified Chinese | Correct normalization + Cantonese preserved | PASS |
| P07 | reentry_respec | FC1-N5 (session state idempotency) | Second submit detected/rejected | Overwrites silently (spec/submit path) | FAIL |
| P08 | empty_whitespace | FC1-shielding (trust boundary validation) | All-whitespace rejected | Empty rejects OK; whitespace passes validation | PARTIAL |
| P09 | trivial_baseline | FC1 full chain (baseline) | Spec + generate capsule chain | Spec OK; generate 401 blocked | PARTIAL |

## Agent A Persona Sessions (reference — 6-8 natural personas)

Agent A ran persona sessions in the same workspace. As of probe time:
- `p01_fifth_grader`: COMPLETED spec (deepseek-v4-pro SpecCapsule in CAS); generate not run
- `p02_retired_teacher`, `p03_boba_shop`, `p04_viet_student`, `p05_pm`, `p06_anxious`, `p07_adversarial`, `p08_engineer`: session dirs created, no spec.md (answers.json only; spec shellout failed)

**Total expected matrix**: 9 Agent B + 8 Agent A persona = 17 problems  
**Actually completed spec synthesis**: P01/P03/P04/P05/P06/P09 (6 Agent B) + p01_fifth_grader (1 Agent A) = 7 problems

## Infrastructure Blockers (NOT covered by either agent)

1. **C10 Promotion Guard**: spec/turn path requires PromptPromotionReceipt in CAS before any triage LLM call. Workspace CAS is empty → spec/turn triage blocked. Only spec/submit path works.
2. **deepseek-v4-pro generate 401**: Generate shellout fails with HTTP 401 for all sessions. No artifact generation possible. W8 retry chain runs all 3 attempts (correct) but all fail.
3. **spec/submit workspace-toml bug**: spec/submit passes session_dir as --workspace but turingos.toml lives in outer workspace. step 4b in generate handler copies toml but has a spawn_blocking bug where the copy silently fails. Workaround: manual toml copy before spec/submit.
4. **assets not in workspace**: spec/turn reads `workspace/assets/prompts/grill_meta_v1.md` but assets live at repo root. Required adding symlink to workspace.
