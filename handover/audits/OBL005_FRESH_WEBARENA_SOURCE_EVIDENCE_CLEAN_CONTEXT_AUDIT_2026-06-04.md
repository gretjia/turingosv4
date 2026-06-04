# OBL-005 Fresh WebArena Source Evidence Clean-Context Audit

Date: 2026-06-04

Reviewer: Claude Sonnet headless clean-context witness

Risk class: Class 2 evidence / fixture reconciliation

Scope:
- `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`
- `OBLIGATIONS.md`
- `handover/ai-direct/LATEST.md`
- `handover/evidence/true_suite/obl005_fresh_webarena_20260604T200738Z/`

Required verdict domain:
- `NO-VIOLATION`
- `VIOLATION-FOUND <constitutional-clause> <file>:<line>`
- `RECONSTRUCTION-FAILURE <which-tape-or-cas-path-cannot-be-reconstructed>`
- `SECOND-SOURCE-DRIFT <which-derived-view-is-usurping-ground-truth>`

## Witness Result

```json
{
  "verdict": "NO-VIOLATION",
  "findings": {
    "1_closure_overclaim": "CLEAR. fixture final_closure_claimed=false. webarena_web_agent_manifest.json has final_closure_possible=false and closure_scope=domain_adapter_smoke_only. full_system_participation.json final_closure_possible=true applies only to the source receipt (FULL_SYSTEM_LIT + replay green + source tree recorded). OBLIGATIONS.md Status=in_progress. LATEST.md repeats 'does not claim final closure' for this atom. No text anywhere flips OBL-005 to satisfied.",
    "2_raw_prompts_secrets": "CLEAR. webarena_web_agent_manifest.json notes explicitly state raw prompt and raw provider response are not written to evidence. Evidence files carry only prompt_sha256 and provider_response_sha256 (opaque hashes), no plaintext. turingos.toml sk-... strings are documentation-comment placeholders, not live keys. No raw_prompt/raw_response/messages fields found in any evidence JSON.",
    "3_fixture_blocker_movement": "CLEAR. git diff shows exactly: evidence_run pointer updated from broad_agi_webarena_20260527T053225Z to obl005_fresh_webarena_20260604T200738Z; source_receipt_final_closure_false and source_tree_fingerprint_missing removed; domain_receipt_final_closure_false, benchmark_capability_not_solved, and fresh_final_closure_witness_missing retained. Movement is strictly limited to the two source/source-tree blockers. No other entry in the fixture was touched.",
    "4_class4_surfaces": "CLEAR. git diff main --name-only shows exactly three files changed: OBLIGATIONS.md, handover/ai-direct/LATEST.md, tests/fixtures/liveness/true_suite_evidence_reconciliation.toml. New evidence directory is data-only. None of the AGENTS.md restricted surfaces are touched.",
    "5_worktx_escrow_market": "CLEAR. full_system_participation.json tx_kind_counts show work=1, finalize_reward=0, challenge=0. LATEST.md explicitly states the constitution has no WorkTx uniqueness text, admission allows multiple same-task WorkTxs, and single-solver sweep is in claim/finalize (TB-8). This atom records a single-WorkTx-node run with market-invest activity (BuyWithCoinRouter YES-side only). No multi-node priced-DAG reward settlement claimed. Consistent with audit scope constraint.",
    "6_evidence_facts_cross_check": "CLEAR. source_family=WebArena, public_source=github.com/web-arena-x/webarena config_files/test.raw.json, model_requested=deepseek-chat, model_returned=deepseek-chat, work_tx_landed=true, answer_correct=false, benchmark_verdict=browser_task_answer_mismatch, closure_scope=domain_adapter_smoke_only, final_closure_possible=false. Full-system: source commit e1ad26dc9260b219e8c328ac2543c766469418f2, replay all_indicators_pass=true, full_system_verdict=FULL_SYSTEM_LIT, verdict final_closure_possible=true in source receipt scope. 4 packaged evidence stores present. All match specification.",
    "residual_note_dirty_tree": "full_system_participation.json records status=dirty_allowed_recorded and changed_paths_count=285. This reflects the current-branch dirty working tree at run time and is honestly recorded, not concealed. It is the established pattern for in-flight branch evidence. Not a violation."
  },
  "checked": [
    "handover/evidence/true_suite/obl005_fresh_webarena_20260604T200738Z/webarena/webarena_web_agent_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_webarena_20260604T200738Z/webarena/full_system_participation.json",
    "handover/evidence/true_suite/obl005_fresh_webarena_20260604T200738Z/webarena/webarena_web_agent_run_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_webarena_20260604T200738Z/webarena/full_system_augmentation_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_webarena_20260604T200738Z/webarena/turingos.toml",
    "handover/evidence/true_suite/obl005_fresh_webarena_20260604T200738Z/evidence_package_manifest.json",
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml",
    "OBLIGATIONS.md",
    "handover/ai-direct/LATEST.md",
    "git diff main --name-only",
    "grep scan for raw_prompt/raw_response/messages/hf_/sk- over evidence JSON files"
  ],
  "residual_risk": "LOW. fresh_final_closure_witness_missing and domain_receipt_final_closure_false remain as explicit blockers in the fixture, consistent with the constraint that this atom binds a source receipt only. The dirty-tree recording is honest but means the source commit e1ad26dc is not the exact HEAD SHA of the run environment; this is the known current-branch evidence pattern and does not affect gate correctness. No unresolved constitutional exposure identified."
}
```

## Verification Inputs Reported To Witness

```text
scripts/run_true_suite_broad_agi_batch.sh --execute-installed \
  --run-id obl005_fresh_webarena_20260604T200738Z \
  --runners webarena_web_agent_fresh
# exit 0

git diff --check
# exit 0

secret scan over evidence + docs + fixture for hf_/sk-
# exit 0; no hits

raw provider scan over evidence for raw_prompt/raw_response/messages/prompt
# exit 0; no hits

cargo test -p turingosv4 \
  --test constitution_true_suite_evidence_reconciliation \
  --test constitution_obl005_final_closure_witness \
  --test constitution_realworld_liveness_coverage \
  --test constitution_matrix_drift -- --nocapture
# exit 0

cargo test -p turingosv4 \
  --test constitution_true_suite_webarena_runner -- --nocapture
# exit 0

bash scripts/run_constitution_gates.sh
# exit 0; [k-1-5] total=165 failed=0

cargo test --workspace --no-fail-fast
# exit 0
```
