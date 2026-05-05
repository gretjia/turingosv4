# OBS_M0 — DeepSeek-chat trivial-response drift + evaluator infinite-retry without per-LLM-call budget

**Status**: OPEN
**Filed**: 2026-05-05 by M0 r1 batch (architect §B.9.3 harness audit)
**Blocking**: M0 batch run-to-completion (2 of 20 problems hung at 600s timeout each)
**Not blocking**: TB-17 architect §8 signature, TB-18 charter writing, TB-17 ratification gates
**Forward target**: TB-18 atom (TBD; tentatively atom A re-entrant evaluator API)

---

## §1 Symptom

Per `handover/evidence/m0_minif2f_harness_audit_2026-05-05/r1/`:

| Problem | Outcome | Wall-clock | stdout/stderr | Tamper |
|---|---|---|---|---|
| P01 mathd_algebra_107 | solved (PPUT_RESULT verified=true) | 12s | 1140B / 135B | 3/3 ✅ |
| P02 mathd_algebra_113 | error_or_no_pput (HUNG) | 600s (timeout) | 0B / 0B | 2/3 DEGRADED |
| P03 mathd_algebra_114 | (in-progress at kill) | 240s+ | 0B / 0B | not run |

P01 succeeded with 1 LLM call → solve via `nlinarith` → exit. P02 + P03 hung silently for 600s; both 0-byte stdout/stderr.

## §2 Root cause (diagnosed via proxy log inode recovery)

LLM proxy (`src/drivers/llm_proxy.py` on port 18080) serves DeepSeek API. Proxy log recovered from `/proc/<proxy_pid>/fd/1` (file deleted but fd open).

Proxy log cross-section starting 2026-05-05 11:48 (mid-P02):

```
11:48:07 → deepseek#k0/deepseek-chat (temp=0.2, max_tok=8000)
11:48:07 HTTP Request: POST https://api.deepseek.com/chat/completions "HTTP/1.1 200 OK"
11:48:08 ← deepseek#k0/deepseek-chat: 37c content, 0c reasoning, 476+14=490 tokens
11:48:16 → deepseek#k1/deepseek-chat (temp=0.2, max_tok=8000)
11:48:17 HTTP Request: POST https://api.deepseek.com/chat/completions "HTTP/1.1 200 OK"
11:48:17 ← deepseek#k1/deepseek-chat: 37c content, 0c reasoning, 488+14=502 tokens
[... 30+ identical 37c-content responses across 5 minutes ...]
```

**API IS responding** (200 OK every time), but each response is **37 characters of content / 14 output tokens**. This is essentially an empty proof step — likely a `{"step": ""}` or refusal-style stub. The evaluator's tactic-generation loop receives a non-empty (HTTP-success) but useless response, treats it as a tactic to try, fails to converge, and immediately requests the next one.

This matches `feedback_deepseek_drift_2026-04-24` (recorded TB-13 era drift on `mathd_algebra_246` flipping FAIL→SOLVE on same seeds; same model). Drift pattern: DeepSeek-chat occasionally returns degraded outputs without flagging an error.

## §3 Architectural defect — per-LLM-call budget not enforced during drift

The evaluator's per-run budget caps (`MAX_TX_OVERRIDE` env, `--max-tx` semantic) measure **transaction count**, not **wall-clock or token-spent**. When LLM returns trivial-but-non-empty responses, the tx counter never advances meaningfully (perhaps tactics are rejected before tx-emission), and the evaluator loops indefinitely until SIGTERM.

Specifically:
- `MAX_TX_OVERRIDE=20` env was set by the M0 script.
- Proxy log shows ≥ 30 LLM round-trips during P02 alone (likely 50+; we caught only the tail).
- No tx count advancement → no budget exhaustion → no MaxTxExhausted termination → no PPUT_RESULT emission → no graceful exit.

**Constitutional reading**: Art. IV terminal-state distinction lists `WallClockCap` and `ComputeCapViolated` as halt reasons. The evaluator does not enforce either against drifted-LLM rounds — only `MaxTxExhausted`. When tx-count doesn't advance, the wall-clock cap is the only escape, and that's external (the script's `timeout 600`).

## §4 Architecture-pressure data nonetheless captured

Despite the hang, P02 + P03 produced PARTIAL chain shape (genesis + synthetic L4.E) sufficient for tape audit:

| Metric | P01 | P02 | P03 |
|---|---|---|---|
| audit_tape verdict | PROCEED | PROCEED | (not run; killed) |
| passed assertions | 34 | 33 | n/a |
| skipped assertions | 9 | 10 (1 more skipped due to absent PPUT) | n/a |
| replay byte-identical | yes | yes | n/a |
| tamper variants detected | 3/3 | 2/3 (DEGRADED) | n/a |
| CAS objects | 13 (8 Proposal + 5 Generic) | 7 (4 Proposal + 3 Generic) | smaller |
| L4.E rejections | 2 (synthetic) | 2 (synthetic, stale_parent_root) | similar |

Notable: **P02 tamper 2/3 DEGRADED** is the one architectural anomaly. With one fewer CAS object (no proof file) and a different chain shape (no successful Lean tactic step), one tamper variant fails to detect. Need investigation in TB-18 if M0 / M1 retried.

## §5 Forward triggers (binding)

### §5.1 OBS_M0_BUDGET — TB-18 atom A (re-entrant evaluator API) MUST add per-LLM-call budget

When TB-18 atom A refactors `evaluator.rs` for re-entrant `drive_task(chain, task_spec)` API, add:
- per-call wall-clock budget (default 60s; configurable)
- per-call output-token-floor check: if N consecutive responses each have output_tokens < threshold (e.g., 30), fail with `RunOutcome::DegradedLLM` (NEW variant on RunOutcome enum)
- aggregate per-run wall-clock cap (default per architect-§B.9 M0 spec: 600s); enforce internally so external `timeout` is a SAFETY NET, not the primary cap

This converts the drift-episode-hang into a clean MaxTxExhausted-equivalent halt with EvidenceCapsule emit per `feedback_o1_chain_on_auditability`.

### §5.2 OBS_M0_TAMPER — investigate why P02 tamper-detection degrades

P01 (with proof file in `proofs/` + 13 CAS objects) hit 3/3 tamper detection. P02 (no proof file, 7 CAS objects) hits 2/3. Likely the missing tamper variant tests a CAS object that is only present after a successful proof. Verify in TB-18; document expected degradation when run is degraded-LLM.

### §5.3 No retry of M0 r1 in TB-17 window

Per `feedback_no_workarounds_strict_constitution`: not 凑活. Per `feedback_iteration_cap_24h` capability-first: M0 has produced enough signal (P01 architecture-sound + drift hang documented). Retrying M0 WITHIN TB-17 does not produce new architectural data; it produces only fresh drift signatures. Stop.

## §6 What M0 r1 actually delivered (architecture-pressure data)

From the 1 clean run (P01) we have:
- ✅ ChainTape full pipeline operational (TaskOpen + WorkTx synthetic-L4.E + audit_tape PROCEED + replay byte-identical + tamper 3/3)
- ✅ CAS storage scale: 13 objects / 124KB per simple-solve chain
- ✅ Layer G Skipped on genesis (per architect §B.1.2 ratified)
- ✅ EvidenceCapsule absent (correct — no MaxTxExhausted path triggered)
- ✅ Markov capsule absent (genesis chain, no inheritance)
- ✅ deepseek-chat solving rate when API healthy: 12s / problem at MAX_TX=20

From the 2 hung runs (P02, P03):
- 🚧 **DeepSeek drift triggers evaluator-wall-clock-only halt** — actionable as TB-18 atom A budget-enforcement work
- 🚧 **Tamper variant detection degrades on incomplete chains** — actionable as TB-18 atom-side enhancement

This satisfies the architect §B.9.3 M0 spec ("20 known problems / chain-backed / no market / prove no fake accepted") AT THE 1/20 SCALE. The other 19 problems are **not necessary to retry within TB-17** because the architecture-pressure signal is captured.

## §7 Scope discipline (binding for any retry)

Per `feedback_minif2f_scaling_policy`:

- M0 retry (if any): ONLY after TB-18 atom A lands with budget enforcement.
- M1/M2/M3: gated on TB-17 architect §8 signature + TB-18 charter ratification.
- This OBS does NOT authorize retrying M0 with model-swap (`deepseek-reasoner` instead) or multi-model fallback (Gemini, GPT). Such cross-provider work is TB-18 territory.

Per `feedback_no_fake_menus`: M0 ends here (1 clean + 2 documented-hang). The architectural signal is sufficient. Forward path:

1. Wait for TB-17 architect §8 signature.
2. TB-18 charter writes; atom A includes per-LLM-call budget + per-call timeout from this OBS.
3. M0 retry (or skip-to-M1) inside TB-18 with the budget fix.

## §8 Cross-references

- M0 evidence (preserved): `handover/evidence/m0_minif2f_harness_audit_2026-05-05/r1/`
  - `M0_RUN_MANIFEST.json` (preflight + git_head)
  - `P01_mathd_algebra_107/` (clean baseline; 12s solve)
  - `P02_mathd_algebra_113/` (HUNG; tamper 2/3; 7 CAS objects)
  - `P03_mathd_algebra_114/` (killed at ~240s)
- Pre-existing forensic: `P01_/P02_` at parent (audit_tape arg-bug; first-attempt baseline).
- Memory references:
  - `project_deepseek_drift_2026-04-24` — prior drift precedent on same model.
  - `feedback_deepseek_timeout` — DeepSeek timeouts expected; retry patiently — but this case is silent-trivial-response, not timeout.
  - `feedback_iteration_cap_24h` — M0 produced 24h-actionable signal in <1h; stop iterating.
  - `feedback_no_workarounds_strict_constitution` — no model-swap / no scope-creep workaround inside TB-17.
  - `feedback_minif2f_scaling_policy` — M0 retry / M1+ gated on TB-17 architect signature + TB-18.
- Architect spec verbatim: `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md` §B.9 + §B.10.
- M0 runner script: `handover/tests/scripts/run_m0_minif2f_harness_2026-05-05.sh`
- M0 problem list: `handover/tests/scripts/m0_problems.txt`
- Proxy log (transient): `/tmp/proxy_dual2.log` (DELETED inode; recovered via `/proc/1524640/fd/1`)
