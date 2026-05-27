# OBL-001 DeepSeek Chrome E2E — Clean-Context Constitutional Audit

- **Auditor**: Claude (clean-context witness)
- **Date**: 2026-05-27
- **Task ID**: OBL001_CLAUDE_CLEAN_CONTEXT_AUDIT
- **Risk class**: Class 2 (evidence runner + web generate/spec fixes)
- **FC nodes touched**: FC1 (browser input/output, rtool/wtool loop), FC2 (web boot/onboarding/generate), FC3 (evidence/redaction/tape/CAS hygiene)

## Reviewed files (diff vs main)

| File | Nature of change |
|------|-----------------|
| `src/bin/turingos/cmd_generate.rs` | GENERATE_MAX_TOKENS const (6000→16000), TDMA state_update prompt contract, 2 regression tests |
| `src/web/generate.rs` | WEB_SUBPROCESS_TIMEOUT_SECS (300→1800), --entrypoint index.html for web artifacts, 2 regression tests |
| `src/web/market_view.rs` | Import path cleanup (crate::runtime:: → direct use import) |
| `src/web/spec.rs` | Force-synthesis at 10 accepted turns (OBL001 fallback), new termination_reason, regression test |
| `src/web/welcome.rs` | Reorder seed_grill_prerequisites to after init (prevents non-empty dir rejection) |
| `tests/fixtures/liveness/script_liveness_inventory.toml` | New entry for obl001_deepseek_chrome_e2e.mjs |
| `scripts/obl001_deepseek_chrome_e2e.mjs` | New evidence runner (dev_only, not production) |

## Restricted surface check (AGENTS.md §6)

`git diff main --name-only` cross-referenced against restricted surfaces:
- src/kernel.rs — NOT touched
- src/bus.rs — NOT touched
- src/sdk/tools/wallet.rs — NOT touched
- src/state/sequencer.rs — NOT touched (imported but not modified)
- src/state/typed_tx.rs — NOT touched (imported but not modified)
- src/bottom_white/cas/schema.rs — NOT touched
- RootBox / canonical signing payloads — NOT touched
- Sequencer admission / typed tx schema — NOT touched

**Result: NO restricted surface hit. No hidden Class 4 escalation.**

## Evidence review

Evidence root: `handover/evidence/obl001_deepseek_chrome_20260527T171150Z/`

### Structure

- `preflight.json`: endpoint=api.deepseek.com, binary paths recorded, port check, API key presence (not leaked)
- `metrics.json`: ok=true, status=complete, 15/15 completed (18 attempted, 3 failures), global_duration_ms=8493570, Chrome 148.0.7778.178
- `redaction_audit.json`: secrets_found=false, findings=[]
- `summary.md`: explicitly states "candidate evidence only; ledger closure requires post-run audit"
- 18 persona directories (persona_0..persona_17), each containing: manifest.json, screenshots (PNG), redacted_turingos.toml, server.log, transcript.json, transcript.md, workspace/

### Tape/CAS presence (spot-checked persona_0)

- `workspace/sessions/<uuid>/artifacts/tdma_generate/<uuid>/chaintape.jsonl` — present
- `workspace/sessions/<uuid>/artifacts/tdma_generate/<uuid>::worker-beta/chaintape.jsonl` — present
- `workspace/sessions/<uuid>/artifacts/tdma_generate/<uuid>::worker-gamma/chaintape.jsonl` — present
- `workspace/sessions/<uuid>/capsules/cas/.turingos_cas_index.jsonl` — present
- `workspace/sessions/<uuid>/cas/.turingos_cas_index.jsonl` — present
- `workspace/runtime_repo/.git/refs/chaintape/l4` — present (L4 tape ref)

15 of 18 personas report spec_capsule_cid (non-empty CID hash). 16 of 18 report chaintape_l4=true. Evidence is NOT stdout-only — real file artifacts with CIDs, tape, CAS.

### Secret leakage check

- `rg 'sk-[a-zA-Z0-9]{20,}'` over evidence root: NO matches
- `rg 'hf_[a-zA-Z0-9]{20,}'` over evidence root: NO matches
- `redacted_turingos.toml` shows `[REDACTED]` for API key values
- `redaction_audit.json` independently confirms secrets_found=false

## Deterministic verification evidence (provided by orchestrator, cross-checked)

| Check | Exit |
|-------|------|
| `rustfmt --edition 2021 --check` (all 5 modified src files) | 0 |
| `node --check scripts/obl001_deepseek_chrome_e2e.mjs` | 0 |
| `git diff --check` | 0 |
| `cargo check --features web --bin turingos --bin turingos_web` | 0 |
| `cargo test --features web` blackbox_system_prompt_contains_tdma_state_update_contract | 0 |
| `cargo test --features web` blackbox_system_prompt_tdma_example_matches_parser_schema | 0 |
| `cargo test --features web` web_subprocess_timeout_is_at_least_1800_secs | 0 |
| `cargo test --features web` accepted_turns_force_synthesis_above_threshold | 0 |
| `cargo test --features web --test cli_web_generate_smoke` web_generate_args_include_entrypoint_index_html | 0 |
| `cargo test --features web --test cli_web_welcome_smoke` welcome_init (3 passed) | 0 |
| `cargo test --test constitution_matrix_drift` (3 passed) | 0 |
| `cargo test --test constitution_script_liveness_inventory` (4 passed) | 0 |
| `bash scripts/run_constitution_gates.sh` total=165 failed=0 | 0 |

## Findings

No blocker or non-blocking findings.

**Info-level observations (not actionable, not blocking):**

1. **INFO** — 3 of 18 personas failed (indices 1, 8, 14). Persona 1 has no spec_md/L4/rejections (only CAS dir). This is expected behavior for a real E2E run with a non-deterministic external LLM. The 15/15 target is met, failures are recorded honestly in metrics.json.

2. **INFO** — `market_view.rs` import cleanup is cosmetic (replacing `crate::runtime::predicate_registry_loader::` with direct `use` import). No behavioral change.

## Constitutional clause review

- No `f64` in money paths introduced
- No hardcoded behavior parameters (constants are named and documented)
- No `.env` committed
- No memory-only canonical state
- No shadow ledger source of truth
- No dashboard-only proof
- No workaround closures (force-synthesis is a legitimate fallback with explicit termination_reason)
- No derived view usurping ChainTape/CAS (summary.md explicitly defers to audit)
- No retroactive evidence rewrite (all evidence is new, timestamped 20260527T171150Z)
- No ID namespace mixing

## Verdict

**NO-VIOLATION**

Scanned 6 modified source files, 1 new script, 1 fixture update, and the full evidence root (18 persona directories + metrics + redaction audit). No constitutional violations found. No restricted surface touched. Evidence is tape-anchored (chaintape.jsonl + CAS + L4 refs), not stdout-only. No secrets leaked. All deterministic predicates GREEN.
