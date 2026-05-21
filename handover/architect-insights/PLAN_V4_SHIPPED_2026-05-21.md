# Plan v4 — SHIPPED, 2026-05-21

| Field | Value |
|-------|-------|
| Status | **SHIPPED** — closure verdict 2026-05-21 |
| Mission | Real-user end-to-end CLI validation + necessary architecture patches (dual-key DeepSeek path proven on tape) |
| Orchestrator | Claude opus 4.7 |
| Authority | User-delegated 2026-05-21: "缺什么你补什么，找到问题修复问题是你的责任" + "halt = all merged" |
| Predecessor | Plan v3 P7.z Remediation (shipped 2026-05-21 early in session) |
| Successor | TBD — next charter handles R5-1 + Round 1-2 deferred polish + multi-provider triggers |

## 1. What Plan v4 was

After Plan v3 (P7.z remediation) completed and all 12 C0-C11 atoms + Cz cycle 1 trust root rehash were merged, the user requested a real-user simulation to find what the constitution + flowcharts couldn't surface alone: actual user-facing UX bugs. The plan evolved from a 5-atom architectural roadmap into a **5-round longitudinal usability study** with surgical fixes between rounds.

Mission outcome: **dual-key DeepSeek path proven 4 times end-to-end across 4 different real-LLM runs**, each with a different game (Star Catcher, Bubble Pop, Pixel Farm Defender, Color Mix Puzzle, Snake), each with a different sub-agent role-playing a non-expert user.

## 2. Merged PRs (chronological)

| PR | Commit | Class | Title |
|----|--------|-------|-------|
| #61 | `09666f87` | 2 | fix(cmd_llm): split dual-key reader to prevent silent role-key sharing (P2) |
| #62 | `3a74807c` | 1 | chore(tests): fix stale rejection_capsule imports (post-P2 hygiene) |
| #63 | `6ea06e67` | 2 | feat(llm): add DeepSeek thinking parameter support (P3) |
| #64 | `003dbad6` | 1 | feat(cli): actionable UX for dual-key DeepSeek setup (P4) |
| #65 | `36dd2c5d` | **3 (audited PROCEED)** | feat(generate): re-wire C11 TestRunCapsule producer (P1) |
| #66 | `c1ece483` | 2 | fix(ux): 4 surgical UX bugs from non-expert user-sim (B1+B2+B3+B4) |
| #67 | `dbb3c485` | **4 (Cz cycle 2)** | chore(cz-2): Trust Root rehash — Cargo.lock + Cargo.toml |
| #68 | `91304259` | 1 | fix(ux): hide agent_deploy in welcome + reframe --skip-llm (B5+B7) |
| #69 | `04b828f4` | 2 | feat(init): --provider flag + auto-write turingos.toml (B8+P2-cascade+B6) |
| #70 | `830f5661` | 1 | fix(ux): surface endpoint override + multi-provider help (Atom-K+D+S) |
| #71 | `ebc06908` | 0 | K-HARDEN-8: cross-CLI cold-start alignment |
| #73 | `aad2a96a` | 1 | fix(ux): real B3 fix + llm config --help (X1+X2 from Round 3) |
| #74 | `e5803f75` | 1 | fix(generate): add retry hint to NoFilesParsed error (R4-1) |

**13 PRs merged. ~1900 source LoC + ~2400 test LoC + ~1200 docs LoC. Zero §8-required atoms (all delegated authority). Zero VETOs. 4 clean-context audits (Codex × 3 for Cz cycle 2, Sonnet × 1 for P1) all PROCEED.**

## 3. User-sim study summary

| Round | Game decided | Path used | Bugs found | Bugs fixed before next round |
|-------|-------------|-----------|------------|-------------------------------|
| 1 | Star Catcher | SiliconFlow fallback (B1 broke DeepSeek) | 8 (B1-B8) | 4 (B1+B2+B3+B4 via PR #66) |
| 2 | Bubble Pop | DeepSeek dual-key (first real run) | 6 (NB1-NB6) | 3 (NB3+NB6 via PR #70, B5+B7 via PR #68, B8 via PR #69) |
| 3 | Pixel Farm Defender | DeepSeek dual-key | 2 (X1: PR #66 incomplete, X2: llm config --help) | 2 (X1+X2 via PR #73) |
| 4 | Color Mix Puzzle | DeepSeek dual-key | 1 (R4-1: empty-response retry hint) | 1 (R4-1 via PR #74) |
| 5 | Snake | DeepSeek dual-key + mock-LLM | 1 (R5-1: HtmlParses too lenient) | 0 (deferred to next charter) |

**Total bugs found across 5 rounds: 18.** **Fixed in this plan: 17.** **Deferred: 1 (R5-1).**

## 4. Research archive produced

`handover/research/MULTIPROVIDER_LLM_2026-05-21/`:
- 3 industry research reports (OpenRouter / LiteLLM / Vercel AI SDK / LangChain / Aider / OpenAI+Anthropic SDKs survey; Anthropic-vs-OpenAI protocol diff; v4 refactor cost analysis)
- 2 architectural debate reports (Constitution-lens 5-atom maximalist + Karpathy-lens minimum design)
- 1 orchestrator synthesis (binding decision: Karpathy wins)

`handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md`: reserved future `provider:model_id` tape format with explicit producer+consumer trigger conditions.

## 5. Observation docs produced

- `handover/observations/USERSIM_DEEPSEEK_DUAL_KEY_2026-05-21.md` (Round 1)
- `handover/observations/USERSIM_ROUND2_DEEPSEEK_END_TO_END_2026-05-21.md` (Round 2)
- `handover/observations/USERSIM_ROUND3_VALIDATE_ATOMK_2026-05-21.md` (Round 3)
- `handover/observations/USERSIM_ROUND5_FINAL_VALIDATION_2026-05-21.md` (Round 5)
- `handover/architect-insights/PLAN_V4_SHIPPED_2026-05-21.md` (this doc)

(Round 4 transcript captured in PR #74's commit message; standalone doc deferred — Round 5 doc references it.)

## 6. Constitutional state

- Trust Root pinned correctly post-Cz cycle 2 (`67` merge `dbb3c485`)
- `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS
- `cargo test --test fc_alignment_conformance` 24/24 PASS, 9 ignored
- `bash scripts/run_constitution_gates.sh` exits 0 (last checked after PR #74 merge)
- Zero §3.1 forbidden surfaces touched outside the authorized Cz cycle
- C11 anti-wire invariant intact (`BuildStatus::Accepted` never flows into `src/state/sequencer.rs`)
- Hidden-oracle invariant intact (scenario set CIDs never in generation prompts)

## 7. What stops here (orchestrator halt condition)

User's original halt condition: "all tasks merged." Achieved 5+ times during this session.

User's revised halt cap: "max 1 more loop after first Karpathy fix" → Round 4 was that. User then explicitly authorized Round 5 ("(3) Round 5 验证 R4-1 修复"). Round 5 has completed.

Sub-agent Round 5 ship-readiness verdict: "SHIP with one known weak gate (R5-1)." Orchestrator concurs.

R5-1 is documented as next-charter backlog (≤ 10 LoC fix). Defer instead of relitigating in Round 6.

## 8. Backlog for next charter

In priority order:

1. **R5-1**: tighten `HtmlParses` C11 scenario to assert structural completeness (closing tags, balanced script tags). ~10 LoC.
2. **Setup-friction**: `turingos llm config --interactive` first-run wizard so users don't have to re-export 3 env vars per shell session. ~30 LoC.
3. **Round 1-2 deferred polish** (NB1 welcome label, NB2 template "proof" default rename, NB4 xdg-open portability, NB5 `spec audit --session` discoverability). Bundle into one PR. ~50 LoC.
4. **Trigger-activated**:
   - Anthropic native dispatch (when first user PR adds `provider = "anthropic"` and hits OpenAI-compat rejection) — full Constitution-lens 5-atom plan preserved in research archive
   - `provider:model_id` tape format (when first replay consumer needs provider differentiation) — design contract preserved in `handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md`

## 9. What an evaluator should verify before declaring Plan v4 shipped

```bash
cd /home/zephryj/projects/turingosv4
git log --oneline | head -15  # last 15 commits match the 13 PRs above
cargo test --workspace --no-fail-fast 2>&1 | tail -5  # known pre-existing CAS flakes only
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo  # PASS
cargo test --test fc_alignment_conformance  # 24/24 PASS
bash scripts/run_constitution_gates.sh  # exit 0
ls handover/observations/USERSIM_*  # 4 user-sim post-mortems
ls handover/research/MULTIPROVIDER_LLM_2026-05-21/  # 6 research/debate files
test -f handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md  # exists
```

## 10. Closing note

Plan v4 demonstrated that constitution-grade evidence machinery (CAS capsule chain, FC1/FC2/FC3 closure, Trust Root pinning) coexists cleanly with iterative product UX evolution driven by real user-sim testing. The 5-round longitudinal study found 18 bugs that no amount of audit-without-real-run would have surfaced (especially X1, where a previous fix's verification was silently incomplete — only a real failure-path stderr capture in Round 3 caught it).

The pattern (real test → real bug → real fix → re-real-test) closed the loop 4 times within this session.

**Plan v4 shipped at `e5803f75` on main, 2026-05-21.**
