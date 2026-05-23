# TB-SOFTWARE-3-0-CONSOLIDATION — TB Charter

**Status**: ACTIVE 2026-05-23
**Source**: User strategic blueprint 2026-05-23 (v2 post-correction)
**§8**: `handover/directives/2026-05-23_SOFTWARE_3_0_CONSOLIDATION_DIRECTIVE_AND_§8.md`
**Plan**: `/home/zephryj/.claude/plans/harness-orchestrator-multi-agent-agent-typed-diffie.md`
**Predecessor**: TB-PHASE-E-REAL-VALIDATION (validation complete; F1 fix merged PR #118)

---

## Phase ID tags (per `feedback_tb_phase_tag_required`)

```
phase_id:                        P7.z V4 Product-CAK Hardening (single-maintainer substrate)
roadmap_exit_criteria_addressed: P7.z:web-product-path-fragility — close 4 production
                                 fragilities observed in PHASE_E_REAL_VALIDATION:
                                 • stdout-as-truth in task open
                                 • no cross-CLI session resume from CAS
                                 • BuildSessionView swallows decode errors
                                 • provider-coupled LLM client filename
kill_criteria_tested:
  KILL-stdout-1:        `! grep -E "t_hash_|simple_hash" src/web/write.rs`
  KILL-resume-1:        AppState.sessions cleared after one accepted turn → next turn
                        resumes via CAS snapshot
  KILL-buildview-1:     3 tests distinguish empty-ok vs corrupt-error vs bad-capsule-error
  KILL-provider-rename-1: `! grep -E "siliconflow_client" src/bin/turingos/cmd_*.rs`
                          AND `! grep -E "siliconflow_client" src/bin/turingos.rs`
  KILL-scope-freeze:    Class 4 surfaces unchanged on every PR
```

---

## Core illusion

```text
user answers → slot evidence snapshot → SpecCapsule/CAS → artifact → derived view/replay
```

ChainTape + CAS remain truth source. Dashboards, Web sessions, BuildSessionView remain derived views.

---

## Non-goals (explicit)

This package does **NOT**:
- Introduce JetStream / Kafka / Postgres / MCP Gateway / Model Gateway microservices
- Rename siliconflow_client to anything that implies abstraction (only generic name rename)
- Add `ChatProvider` enum or trait-based provider dispatch
- Add `ModelCallReceipt` runtime module
- Touch `typed_tx` / `sequencer` / `bus` / `cas/schema` (Class 4 surfaces)
- Edit `scripts/run_constitution_gates.sh`
- Touch `constitution.md` or `genesis_payload.toml`

All deferred items have explicit deferral notes in `LLM_BOUNDARY_INVENTORY_2026-05-23.md`
or `NO_LEGACY_BYPASS_CHECKLIST_2026-05-23.md`. Hidden refactors are forbidden.

---

## Atoms

See §3 of the §8 directive for the 12-atom table.

---

## Cross-cutting rules

### Karpathy invariants
- Plain structs over trait-based abstractions
- Direct functions over Manager/Factory/Engine
- Small surgical changes; no broad refactors mixed with bug fixes
- Tests before implementation per atom
- DEFER abstraction until 2nd concrete impl ([feedback](../../.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_defer_abstraction_until_second_impl.md))

### Constitutional invariants
- ChainTape + CAS remain truth (Art. 0.2)
- BuildSessionView / GrillSession remain derived views
- No mocking misrepresented as real
- Evidence under `handover/evidence/` only

### Execution hygiene
- Fresh worktree from origin/main per atom branch
- Targeted stash only ([feedback](../../.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_git_hygiene_no_bulk_ops.md))
- NEVER `git stash -u`, NEVER `git add -A`
- Empty states are NORMAL (Ok variants), not errors ([feedback](../../.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_conservative_error_semantics.md))
- HTTP failures use explicit status codes (502/500); no 200-with-warning

---

## Ship status

| Atom | Status | PR | Notes |
|------|--------|----|----|
| S0.1 §8 directive + charter | IN PROGRESS | — | this file + the §8 file |
| S0.2 worktree bootstrap | PASS | — | baseline green; cargo check + cli_web_write_smoke compile cleanly |
| S1.1 remove stdout-as-truth | PENDING | — | |
| S1.2 tests | PENDING | — | |
| S2.1 GrillSessionSnapshot CAS write | PENDING | — | |
| S2.2 cache-miss rebuild | PENDING | — | |
| S2.3 integration test | PENDING | — | |
| S3.1 BuildSessionViewError | PENDING | — | |
| S3.2 callers updated | PENDING | — | |
| S3.3 3 tests | PENDING | — | |
| S4.1 client rename | PENDING | — | |
| S4.2 LLM_BOUNDARY_INVENTORY doc | PENDING | — | |
| S5.1 audit_legacy_bypass.sh | PENDING | — | |
| S5.2 NO_LEGACY_BYPASS_CHECKLIST | PENDING | — | |
| S6.1 ship report | PENDING | — | |
| S6.2 cumulative audits | PENDING | — | |

Updates: this file is the ship-progress source of truth for this TB.
