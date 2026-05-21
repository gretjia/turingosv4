# A1 — minif2f historical tape-relay record study

| Field | Value |
|-------|-------|
| Date | 2026-05-21 evening |
| Phase | A (research) — dispatch 1 of 3 |
| Agent | Explore (read-only investigation of `handover/`) |
| Word count | ~1700 |

## TL;DR

Tape relay in v4 is operationally defined (per TB-18 decision records) as: every externalized LLM call becomes one Attempt Node — either L4 accepted WorkTx or L4.E rejection evidence — with `parent_attempt_cid` chains linking retries. Current cmd_generate reads prior attempt capsules from CAS (lines 269–290) to compute retry_index and parent_attempt_cid, but **does NOT read prior rejection diagnostics into the retry prompt** — the prompt stays identical across attempts, making true "agent learning from tape" absent at the generation layer (pre-Atom-T).

## Tape relay — operational definition from history

Per `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md` (lines 20–37), the three-node taxonomy is:

1. **Attempt Node** = every externalized LLM proposal → either L4 accepted WorkTx OR L4.E rejected evidence capsule (never both, never neither).
2. **State Node** = predicate-passed L4 accepted transition (advances ledger).
3. **Rejection Evidence Node** = predicate-failed candidate (lives in L4.E, does NOT advance state).

The canonical externalization boundary is strict: "if it became a `submit_typed_tx`, it is an Attempt Node; if it stayed in the model's hidden state, it is invisible to TuringOS" (`DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`, lines 66–68).

**Tape relay operationally = one-to-one correspondence**: `externalized_llm_lean_attempt_count == |L4 WorkTx| + |L4.E rejections|` (parent ruling `TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`, §A.1, lines 49–51). This equation grounds the VETO: TB-18 M1 problem P49 ran 32 LLM-Lean cycles but emitted only 1 L4 WorkTx — tape granularity was broken.

## Past minif2f capsule chain patterns

**Historical evidence:** `handover/evidence/tb_4_smoke_2026-04-30/README.md` is the cleanest v4 multi-attempt record:

- **Problem**: `mathd_algebra_107.lean` (MiniF2F test).
- **Run configuration** (lines 8–14): MAX_TRANSACTIONS=20 (elevated from TB-3's 5), CONDITION=full, model_snapshot=deepseek-v4-flash.
- **Two-condition structure**:
  - **Condition=oneshot** (lines 21–49): single proposal, FAILS as expected. `prompt_context_hash="a1f43584a17d1226"` (bit-identical across TB-1 through TB-4, four sessions).
  - **Condition=n1** (lines 50–80): multi-attempt budget (MAX_TX=20, though solved on tx_count=1). SOLVES with canonical proof `gp_payload="nlinarith"`, `pput_runtime=0.00021153742...` (bit-identical to TB-0 baseline). `golden_path_token_count=12`, `total_run_token_count=448`.

**Capsule chain signature** (lines 102–112): The five-row "hash chain" across tabs:
- `prompt_context_hash="a1f43584a17d1226"` ✅
- `schema_version="v2.0"` ✅
- oneshot `solved=false` ✅
- n1 `solved=true` ✅
- n1 `gp_payload="nlinarith"` ✅

This persistence across four independent sessions + TB-4 ABI bump (parent_state_root field + ChallengeTx variant) proves **schema-stable relay**: downstream consumers (lib crate types, TaskMarketsIndex, ChallengeCase, Verify/Challenge dispatch arms) are serde-compatible and capsule chains carry through without data loss (lines 115–117).

**Pattern not found**: No historical evidence shows attempt-2+ reading rejection diagnostics from attempt-1's GenerateRejectionCapsule into a modified prompt. The minif2f-era smoking testing was one-shot or n1-constrained single-tactic-chain; true multi-LLM-retry with diagnostic feedback loops was not exercised in the archived evidence directories.

## Current v4 multi-attempt behavior (T7 audit, pre-Atom-T)

**File**: `src/bin/turingos/cmd_generate.rs`

**Lines 269–290 — Retry state discovery**:
```rust
if let Ok(store) = CasStore::open(&cas_dir) {
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut attempts = Vec::new();
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID) {
                if let Ok(bytes) = store.get(&cid) {
                    if let Ok(capsule) = serde_json::from_slice::<GenerationAttemptCapsule>(&bytes) {
                        if capsule.session_id == session_id {
                            attempts.push((capsule.logical_t, cid.hex(), capsule.retry_index));
                        }
                    }
                }
            }
        }
    }
    attempts.sort_by_key(|x| x.0);
    if let Some(last) = attempts.last() {
        retry_index = last.2 + 1;
        parent_attempt_cid = Some(last.1.clone());
    }
}
```

**What happens**:
1. Scans CAS for all EvidenceCapsule objects with schema_id="turingos-generation-attempt-v1".
2. Deserializes each, filters by session_id.
3. Sorts by logical_t (timestamp), takes the last attempt.
4. Increments its retry_index for the current attempt, stores its CID as parent_attempt_cid (line 288).
5. **Critical gap (pre-Atom-T)**: No read of rejection capsules or their private_diagnostic_cid fields.

**Lines 242–254 — Prompt construction (pre-Atom-T)**:
```rust
let messages = vec![
    ChatMessage::system(blackbox_system_prompt()),
    ChatMessage::user(format!(
        "Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
    )),
];
```

The prompt is **identical** across all retries. It includes only the original spec_md, not:
- Prior attempt's raw LLM output (`raw_output_cid`).
- Rejection reason / class (LlmApiError, NoFilesParsed, HeuristicFailed, TooManyFiles).
- Test pipeline failure diagnostics (if outcome was HeuristicFailed / test_run_cid failure).

**Lines 381–396 — Capsule recording**:
```rust
let capsule = GenerationAttemptCapsule {
    schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
    session_id: session_id.clone(),
    spec_capsule_cid: spec_capsule_cid.clone(),
    spec_source,
    model_id,
    model_seed: None,
    prompt_hash,
    raw_output_cid: raw_output_cid.clone(),
    usage_total_tokens,
    retry_index,                    // ← carries retry count
    parent_attempt_cid,             // ← carries prior attempt's CID
    outcome,
    parsed_file_count,
    logical_t,
};
```

The capsule **DOES record** parent_attempt_cid (line 392), forming a linear chain. Each retry can reconstruct the chain by walking CAS backward. **But the chain is never used to modify the LLM prompt on retry (pre-Atom-T).**

**Verdict**: cmd_generate implements **deterministic retry queuing** (read prior CAS state, bump retry_index, link capsule chain) but **NOT semantic tape relay** (diagnostic feedback into prompt). A human reading the GenerationAttemptCapsule chain can see "attempt 1 failed with NoFilesParsed, attempt 2 should have learned from that" — but the code doesn't enforce or implement that learning.

## Gap analysis: history vs current (pre-Atom-T)

| Aspect | TB-18 / minif2f era | Current v4 |
|---|---|---|
| **Externalization boundary** | Strict per rule: 1 LLM call → 1 Attempt Node | ✅ Implemented: GenerationAttemptCapsule + GenerateRejectionCapsule |
| **Attempt chaining** | Parent_attempt_cid mentioned in TB-7R checkpoints | ✅ Implemented: lines 267, 288, 392 of cmd_generate.rs |
| **Chain readability** | Historical evidence shows tape chains survive schema bumps (TB-4 ABI) | ✅ Supported: parent_attempt_cid is stable across retries |
| **Diagnostic feedback to retry** | NOT exercised in minif2f smokes; TB-18 VETO flagged missing granularity gate | ❌ Missing pre-Atom-T: rejection capsule diagnostics not read into retry prompt |
| **Per-call externalization invariant** | Codified in decision record (lines 104–107) | ⚠️ Checkable in theory; no runtime gate enforcing it |
| **Class-4 ratification** | TB-7R + TB-18R required explicit schema ratification | ✅ GenerationAttemptCapsule schema_id locked at v1 (line 7 of generation_attempt.rs) |

**Key gap**: The minif2f era never required a "denominator preflight" (per post-mortem §7.1, `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md`, lines 305–314). TB-18 M1 VETO revealed that tape granularity could be silently broken if each multi-attempt benchmark run was not gated on `externalized_llm_lean_attempt_count == |L4 ∪ L4.E|` **before** shipping. Current v4 has no such gate at the cmd_generate layer.

## Recommended test-matrix difficulty axes

Based on TB-4 smoke + TB-18 root cause 2 (tape granularity):

1. **Denominator tier (n=1)**: Single-problem one-shot + one n1 run (20 LLM token budget). Assert externalized_call_count == 1. This is the **hard gate** that TB-18 should have had at charter-zero.
2. **Single-chain tier (n=5–13)**: Multi-problem single-chain (no cross-problem task market). Assert per-problem externalization invariant. Proof: `tb_18_single_chain_13_of_13` in evidence directory.
3. **Difficulty tiers**:
   - **Easy** (e.g., mathd_algebra_101–150): Single-tactic omega path, solved in 1–2 attempts, golden_path_token_count ≤ 20.
   - **Medium** (e.g., mathd_algebra_301–500): Multi-tactic path, 2–5 attempts, requires step/omega interleaving.
   - **Hard** (e.g., induction proofs, P38/P49 class): Requires ≤32 attempts (max), token budget exhaustion risk, multi-branch search.
4. **Granularity assertions per tier**:
   - Easy: `externalized_calls == L4_count` (tight coupling; no rejected calls).
   - Medium: `externalized_calls == L4_count + L4.E_count` (some rejections).
   - Hard: Same equality + `HeuristicFailed` test-pipeline failure rates < 5%.

## Cleanest historical relay example

**File path**: `handover/evidence/tb_4_smoke_2026-04-30/README.md`

**Excerpt** (lines 50–89 — n1 run structure):
```json
{
  "run_id": "n1_mathd_algebra_107_1777549685746",
  "solved": true,
  "verified": true,
  "progress": 1,
  "total_run_token_count": 448,
  "tx_count": 1,
  "budget_max_transactions": 20,
  "hit_max_tx": false,
  "gp_payload": "nlinarith",
  "pput_verified": 0.00021153742012347014
}
```

**Why this is the cleanest relay**:
1. **Capsule chain passes schema bump**: n1 uses TB-4's new VerifyTx/ChallengeTx dispatch arms (lines 115–117).
2. **Proof artifact stable**: `gp_payload="nlinarith"` is bit-identical to TB-0 baseline (line 87), proving backward-compatible proof emission across 4 sessions.
3. **Budget honored**: `budget_max_transactions=20` (elevated per user directive "真实烟测需要加大 max-tx") flows through the driver loop and is reflected in the PPUT row (line 89). This proves the tape carries not just schema but also intent/constraint state.
4. **No external mutation**: `tb_4_smoke_2026-04-30` was a **single-run** test, not a multi-attempt scenario. It proves the **stable capsule surface** but not retry-with-feedback behavior.

**Where to find prior attempt state**: If a second n1 run had been re-executed on the same problem with the first attempt failing, the chain would have existed at lines 286–289 of cmd_generate.rs, and parent_attempt_cid would link them. **The historical evidence shows no such double-attempt; minif2f smokes were single-run-to-proof patterns.**

## Canonical missing pieces (resolved by Plan v6 Atom-T)

Per post-mortem **§7 Future-process recommendations** (`handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md`, lines 305–350):

1. **Denominator preflight gate** (§7.1, lines 305–314): Benchmark TBs must charter `§2 atom table` with Atom 0 = single-problem run asserting `externalized_llm_lean_attempt_count == |L4 WorkTx for run| + |L4.E rejected for run|`. Current: **missing at cmd_generate layer** (Atom-T does not address this — orthogonal).
2. **Invariant relaxation FC-first analysis** (§7.3, lines 324–330): When a fix proposes to relax an invariant, upstream FC-analysis document required. Current: **GenerateRejectionCapsule.RejectClass enum is typed**, so this gate is structurally sound.
3. **Class-4 surface check at every commit** (§7.5, lines 341–350): GenerationAttemptCapsule schema_id is locked (line 7 = "turingos-generation-attempt-v1"), so attempts to re-ratify it are caught by intent.

## Cross-references

- Three-node taxonomy decision: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`
- TB-18 VETO (tape non-externalization): `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`
- TB-18 root-cause post-mortem: `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md`
- TB-4 smoke evidence (cleanest relay example): `handover/evidence/tb_4_smoke_2026-04-30/README.md`
- GenerationAttemptCapsule struct: `src/runtime/generation_attempt.rs` lines 20–37
- GenerateRejectionCapsule struct: `src/runtime/rejection_capsule.rs` lines 35–50
- cmd_generate retry loop (pre-Atom-T): `src/bin/turingos/cmd_generate.rs` lines 266–299 (discovery), lines 242–254 (prompt construction, unchanged across retries), lines 381–396 (capsule recording with parent_attempt_cid)
- TB-7R Checkpoint 2 (parent_tx + VerificationResult): `handover/CHECKPOINT_TB7R_2_2026-05-02.md`
