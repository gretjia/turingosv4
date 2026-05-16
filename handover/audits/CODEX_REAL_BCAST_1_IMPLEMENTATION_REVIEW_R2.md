# CODEX REAL-BCAST-1 Implementation Review R2

Date: 2026-05-16
Repository: `/home/zephryj/projects/turingosv4-real12-action-probes`
Branch reviewed: `codex/real12-economic-action-probes`
Risk class reviewed: Class 4, due Trust Root rehash of pinned files.

## Findings

No blocking production defects found.

1. Non-blocking: selector source coverage is MVP-level, not full charter-scale broadcast coverage.

   `src/runtime/librarian_broadcast.rs:350-355` selects `EVDecisionTrace`, `EconomicJudgment`, `LeanResult`, and `AttemptTelemetry`. It explicitly skips market-review sidecars at `src/runtime/librarian_broadcast.rs:287-297`, and transitional `MarketDecisionTrace` JSON in the shared `AttemptTelemetry` slot is skipped by `src/runtime/attempt_telemetry.rs:888-912`. I did not find production evidence that `EvidenceCapsule`, `MarkovEvidenceCapsule`, L4.E public rejection summaries, `MarketDecisionTrace`, or `NoTradeReasonTrace` are ingested into the digest yet. This is acceptable for the narrow REAL-BCAST-1 MVP claim because the report and evidence claim CAS-backed digest/crop/prompt binding, not complete all-evidence broadcast. Do not broaden the claim until these source classes have explicit selector gates.

2. Resolved by post-audit delta: Lean progress taxonomy now preserves `Verified` at digest construction.

   The R2 base audit found that `decode_librarian_candidate` distinguished `LeanVerdictKind::Verified`, but `build_librarian_digest` wrote every `PartialProgressSummary` as `PartialAccepted`. The delta now maps `event.class_label == "lean:Verified"` to `ProgressKind::Verified` at `src/runtime/librarian_broadcast.rs:426-438`. This resolves the progress-kind finding without touching TypedTx, sequencer admission, signing, or CAS `ObjectType`.

3. Partially resolved by post-audit delta: Trust Root hashes remain sufficient, and the evaluator comment now names REAL-BCAST-1.

   The actual hashes match `genesis_payload.toml` for the pinned REAL-BCAST touched files: `experiments/minif2f_v4/src/bin/evaluator.rs` at `genesis_payload.toml:164`, `src/runtime/mod.rs` at `genesis_payload.toml:219`, `src/runtime/librarian_broadcast.rs` at `genesis_payload.toml:220`, and `src/bin/audit_dashboard.rs` at `genesis_payload.toml:244`. The evaluator line now explicitly says `REAL-BCAST-1 Librarian Broadcast Loop` at `genesis_payload.toml:164`, and the updated `src/runtime/librarian_broadcast.rs` hash is pinned at `genesis_payload.toml:220`. The `src/runtime/mod.rs` and `src/bin/audit_dashboard.rs` comments still carry predecessor-stage wording, but their hashes match and the Class-4 ratification document covers the intent at `handover/directives/2026-05-16_REAL_BCAST_1_CLASS4_RATIFICATION.md:17-31`; remaining comment cleanup is archival polish, not a ship blocker.

## Audit Question Answers

1. Hidden Class-4 surface: not found.

   No diff was present in the restricted runtime surfaces I checked: `src/state/typed_tx.rs`, `src/state/sequencer.rs`, `src/bottom_white/cas/schema.rs`, `src/kernel.rs`, `src/bus.rs`, or `src/sdk/tools/wallet.rs`. The new module export declares the intended boundary at `src/runtime/mod.rs:219-222`: materialized view only, no new `TypedTx`, sequencer admission, signing payload, or CAS `ObjectType`. Digest and role crop writes use `ObjectType::Generic` at `src/runtime/librarian_broadcast.rs:582-599` and `src/runtime/librarian_broadcast.rs:737-754`.

2. Trust Root rehash: explicit enough to proceed.

   The ratification scope explicitly authorizes Trust Root rehash for `evaluator.rs`, `audit_dashboard.rs`, `runtime/mod.rs`, and inclusion of `runtime/librarian_broadcast.rs` at `handover/directives/2026-05-16_REAL_BCAST_1_CLASS4_RATIFICATION.md:20-31`, and explicitly excludes TypedTx, sequencer admission, signing payload, CAS ObjectType schema, raw broadcast, full async, forced trade, price-as-truth, and E2/E3/E4 overclaim at `handover/directives/2026-05-16_REAL_BCAST_1_CLASS4_RATIFICATION.md:34-47`. Command evidence `command_0001` reports Trust Root exit 0.

3. LibrarianDigest remains a CAS-backed materialized view.

   The digest root is derived from explicit CAS index CIDs, with the code comment stating it is not a global pointer at `src/runtime/librarian_broadcast.rs:170-177`. Digest discovery is by CAS metadata schema scan at `src/runtime/librarian_broadcast.rs:613-620`, not a filesystem latest pointer. The dashboard section labels itself a ChainTape/CAS materialized view and not truth at `src/bin/audit_dashboard.rs:2895-2896`.

4. Shielding/no-raw boundary holds for the reviewed surfaces.

   The core guard rejects raw Lean stderr, raw prompt/completion, private CoT, raw diagnostics, and untriaged historical text at `src/runtime/librarian_broadcast.rs:179-196`. Selection and digest construction re-run that guard at `src/runtime/librarian_broadcast.rs:359-365` and `src/runtime/librarian_broadcast.rs:374-386`; role crop write and prompt binding validate rendered notices/visible context at `src/runtime/librarian_broadcast.rs:737-754` and `src/runtime/librarian_broadcast.rs:775-800`. The dashboard checks digest shielding at `src/bin/audit_dashboard.rs:2883-2894`. I found redaction-label strings in `PromptCapsuleV2.hidden_fields_redacted` at `experiments/minif2f_v4/src/bin/evaluator.rs:180-184`; these are labels, not raw prompt/completion/CoT payloads.

5. PromptCapsule read-set and visible-context binding is real enough for MVP replay.

   `real5_write_prompt_capsule_v2_for_view` appends the Librarian digest and role crop CIDs to the PromptCapsule read set at `experiments/minif2f_v4/src/bin/evaluator.rs:163-171`, validates the binding at `experiments/minif2f_v4/src/bin/evaluator.rs:193-201`, and the turn path computes the prompt hash after the Librarian notice is inserted at `experiments/minif2f_v4/src/bin/evaluator.rs:4433-4448`. The per-turn call passes `bundle.digest_cid` and `bundle.role_crop_cid` into the read set at `experiments/minif2f_v4/src/bin/evaluator.rs:4462-4477`. The validator requires both CIDs, the visible context CID, the prompt context hash, and the `=== Librarian Notices ===` section at `src/runtime/librarian_broadcast.rs:775-800`.

6. Half-async deterministic replay contract is sufficient for the shipped scope.

   `MarketReviewWindow`, `MarketReviewResponse`, and `MarketReviewSummary` carry optional frozen digest/epoch fields with backward-compatible serde defaults at `src/runtime/market_review.rs:40-88`. Response ordering is deterministic by `(agent_id, response_id)` at `src/runtime/market_review.rs:90-101`. The broadcast contract validates frozen digest and epoch consistency across window, responses, summary, and `BroadcastEpoch` at `src/runtime/market_review.rs:141-174`. This supports the barriered/sequential MVP. It does not ship full async, and the report does not claim full async.

7. A/B and hard10 claims are appropriately narrow.

   The final stress report says the substrate works under smoke and hard10 pressure while `E2` is not achieved at `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md:21-27`. The A/B interpretation explicitly avoids a causal performance claim at `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md:175-180`. The hard10 counters show `buy_with_coin_router=0`, `agent_economic_action_tx_count=0`, all reviewed EV reasons `NegativeEV`, and `librarian_digest_cas_count=261` / role crop count `261` at `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md:198-222`. The recommendation section explicitly says not to claim spontaneous market emergence, causal performance improvement, E2, E3, model ranking, or price-as-truth at `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md:306-326`.

## Evidence Reviewed

- Diff: `handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/diff.patch`.
- Trust Root gate: `handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0001_*`, exit 0.
- Targeted REAL-BCAST tests: `handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0002_*`, exit 0.
- A/B smoke A off: `handover/evidence/real_bcast_1_ab_A_20260516T100140Z`, `librarian_digest_cas_count=0`, `role_crop=0`, shielding PASS, E2 not achieved.
- A/B smoke B on: `handover/evidence/real_bcast_1_ab_B_20260516T100140Z`, `librarian_digest_cas_count=14`, `role_crop=14`, shielding PASS, E2 not achieved.
- hard10 B on: `handover/evidence/real_bcast_1_hard10_B_20260516T100140Z`, `problem_count=10`, `tx_count=213`, `EVDecisionTrace=106`, `MarketReviewSummary=106`, `LibrarianDigest=261`, `role_crop=261`, `buy_with_coin_router=0`, `agent_economic_action_tx_count=0`.
- Constitution gates: `handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0006_*`, 461 passed / 0 failed / 1 ignored.
- Workspace tests: `handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0007_*`, exit 0.
- Final stress report: `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md`.
- Fresh audit-local sanity check: `git diff --check`, exit 0.
- Post-audit targeted delta tests: `handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0011_*`, 12 passed / 0 failed.
- Post-audit Trust Root gate: `handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0012_*`, 1 passed / 0 failed.
- Post-audit audit-local sanity check: `git diff --check`, exit 0.

## Post-Audit Delta Review

Date: 2026-05-16

Delta reviewed:

- `genesis_payload.toml:164` now names `REAL-BCAST-1 Librarian Broadcast Loop` for the evaluator Trust Root comment.
- `genesis_payload.toml:220` pins the updated `src/runtime/librarian_broadcast.rs` hash `a7b812aab798a593c6d7e6b9fed3370eba8a6074a0f4e6ccfae043399ae04c4c`, matching `sha256sum`.
- `src/runtime/librarian_broadcast.rs:426-438` now sets `ProgressKind::Verified` for `class_label == "lean:Verified"` and keeps `ProgressKind::PartialAccepted` otherwise.

Delta findings:

- No new blocking defect found.
- No new hidden Class-4 surface found. The reviewed delta stays in Trust Root metadata and the existing Librarian materialized-view module.
- The previous progress-kind taxonomy finding is resolved.
- The evaluator Trust Root comment finding is resolved. Remaining predecessor-stage wording on other pinned-file comments is still non-blocking archival polish.

Delta verdict:

PROCEED

## Verdict

PROCEED
