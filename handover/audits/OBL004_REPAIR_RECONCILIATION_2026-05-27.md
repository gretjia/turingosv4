# OBL-004 Repair Reconciliation — 2026-05-27

Active obligations: OBL-001 open, OBL-004 in_progress, OBL-005 in_progress.

## Objective

Reconcile the stale OBL-004 evidence ledger against current mainline. The
2026-05-24 ledger text expected PR-B/PR-C/PR-D/PR-E branches and a
`CONSTITUTION_REPAIR_R1R2R3_SYNTHESIS_2026-05-24.md` report. Current GitHub
and git history show those branch names were not merged as named. This report
does not retroactively fabricate that 2026-05-24 synthesis; it records the
current reconstruction from merged PR receipts, source gates, and targeted
verification.

## Current Receipts

| Repair surface | Current receipt | Reconciliation |
|---|---|---|
| PR-A orphan/delete/docstring wave | PR #139 `constitution-repair/wave1-pr-a-orphan-delete`, merged 2026-05-24T11:22:55Z | Satisfied by named PR-A receipt |
| W3-2 PredicateRegistry Class 4 binding | PR #140 `codex/w3-predicate-registry-bind`, merged 2026-05-24T14:03:00Z | Satisfied by Class 4 receipt and active predicate registry gates |
| PR-B shielding/judge surface | No branch named `constitution-repair/wave1-pr-b-shielding-judge` merged | Satisfied by current shielding/judge gates and evidence-binding tests: `constitution_shielding_gate`, `constitution_shielding_evidence_binding`, TDMA/judge prompt leak tests, and sanitized low-pollution rejection paths |
| PR-C librarian disjointness | No branch named `constitution-repair/wave1-pr-c-librarian-disjointness` merged | Satisfied by active `runtime::librarian_broadcast` gates plus PR #184 removing dead SDK `librarian` / `search` tools |
| PR-D bus cleanup / Node retire / WAL cleanup | No branch named `constitution-repair/wave2-pr-d-bus-cleanup-node-retire` merged | Satisfied by current `TuringBus::snapshot` ChainTape/CAS-derived input gate and PR #192 Class 4 WAL/SDK closure. Remaining `Node`/`NodeKind` symbols are active TDMA/GitTape ledger vocabulary, not the retired TS prototype node surface |
| PR-E `build_agent_prompt` deletion wording | No branch named `constitution-repair/wave1-pr-e-build-agent-prompt-retire` merged | Superseded: `src/sdk/prompt.rs::build_agent_prompt` is an active retained surface, not a zombie. It is bound by G3/G5/REAL-12 gates, production liveness group `agent_prompt_model_boundary`, and broad/realworld contracts. Deleting it would remove currently tested prompt/PnL/action-menu behavior |
| R3-B3 sanitized error surface | No exact `SanitizedErrorTag` type was added | Satisfied by PR #78 sanitized runner hygiene plus current CAS/CID private diagnostic routing and shielding gates. The constitution requires sanitized, shielded error routing; it does not require the old planned type name |

## Verification

GitHub receipt query:

- `gh pr list --state all --json number,state,mergedAt,headRefName,title,url --limit 200`
- `gh pr list --state all --search "wave1-pr-b OR wave1-pr-c OR wave2-pr-d OR wave1-pr-e" ...` returned no matching PR rows.
- `gh pr view 78`, `gh pr view 184`, and `gh pr view 192` verified the current merged surfaces.

Targeted gate package, 2026-05-27:

```bash
cargo test \
  --test constitution_shielding_gate \
  --test constitution_shielding_evidence_binding \
  --test constitution_librarian_digest \
  --test constitution_librarian_source_scope \
  --test constitution_librarian_no_raw_leakage \
  --test constitution_librarian_prompt_injection \
  --test constitution_librarian_selector \
  --test constitution_librarian_market_no_trade \
  --test constitution_librarian_real_evidence_binding \
  --test constitution_predicate_registry_immutability \
  --test constitution_predicate_registry_binding \
  --test constitution_predicate_binding_activation \
  --test constitution_predicate_registry_replay \
  --test constitution_flowchart_livenow \
  --test constitution_g3_your_position_prompt \
  --test constitution_g5_action_menu \
  --test constitution_real12_task_market_action \
  --test constitution_production_module_liveness \
  -- --nocapture
```

Result: 83 passed, 0 failed.

## Residual Risk

This reconciliation closes OBL-004 as a stale-ledger blocker. It does not close
OBL-005 final no-zombie status and does not close OBL-001. OBL-005 still needs
a separate final closure witness before any candidate-only true-suite manifest
is flipped to closure.

## Verdict

`OBL004-RECONCILED-NO-UNRESOLVED-VIOLATION`
