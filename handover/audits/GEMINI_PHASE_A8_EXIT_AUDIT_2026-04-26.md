# Gemini Phase A → B Exit Audit (PPUT-CCL arc)
**Date**: 2026-04-26
**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9 / 62c4e14 / 6be6eb4 / 180a300 / 7f4bc0c / a5c78e4 / 30f2a14 / 89994c7 / 90953d6
**Test baseline**: 261 PASS + 29 ignored + 0 failed
**Trust Root**: 30-entry manifest verifies clean
**Elapsed**: 46.5s
**Prompt size**: 246,339 chars

---

As an independent, skeptical adversarial reviewer, I have completed the audit of the Phase A → B Exit Audit Packet (A8) for the PPUT-CCL arc. My findings are detailed below, following the conservative merge rule (VETO > CHALLENGE > PASS).

---

### Audit Findings

My audit is structured around the specific questions posed in § 6, with additional findings noted where applicable.

#### Q1 — Atomicity and FC-trace discipline

1.  **(CHALLENGE)** **Finding**: A significant, untracked code change exists outside the declared atoms. The `hybrid_v1` condition, a prior valid execution mode, has been disabled via a hard `eprintln!` and `std::process::exit(1)`. While the rationale provided in the code comment is sound (it was a deprecated mode with an accounting flaw), its removal constitutes a behavioral change that was not captured in an atomic, FC-traced commit. This violates the FC-first discipline mandated by case C-074 and memory `feedback_fc_first_problem_handling`.
    -   **§/file:line**: `experiments/minif2f_v4/src/bin/evaluator.rs:293-306`

#### Q2 — PREREG amendment soundness (A1)

No findings. The amendment's logic is sound.
-   (Q2.a) The substitution of `p_0 = 0.10` is conservative and does not introduce Type-I inflation, as it holds artifacts to the strictest possible standard defined in the original PREREG.
-   (Q2.b) The re-calibration conditions in `PREREG_AMENDMENT...md` § 3 correctly couple calibration to Phase D readiness, but the hardcoded `p_0 = 0.10` substitution correctly unblocks forward progress for Phases B, C, and E.
-   (Q2.c) The amendment's SHA-256 is correctly included in the Trust Root manifest, preventing silent edits.
    -   **§/file:line**: `genesis_payload.toml:122`

#### Q3 — Budget regime soundness (A5)

No findings. The budget regime implementation is sound and correctly preserves the baseline.
-   (Q3.a) The `base × N` scaling for the total loop bound, combined with round-robin agent selection, correctly implements the "fixed proposal budget per agent" intent on average.
-   (Q3.b) The "fail loud" `UnimplementedRegime` error is the correct, safe default. A silent fallback would invalidate experiments and waste budget.
-   (Q3.c) The default behavior (env unset) correctly and verifiably preserves the `total_proposal` × 200 transaction baseline bit-for-bit.
    -   **§/file:line**: `experiments/minif2f_v4/src/budget_regime.rs:170-173`, `experiments/minif2f_v4/src/budget_regime.rs:180-183`

#### Q4 — FC tracing coverage (A6)

1.  **(CHALLENGE)** **Finding**: The hand-rolled JSON encoder in `fc_trace.rs` is brittle and unnecessary. The justification "Zero new crate dependencies" is invalid, as `serde_json` is already a core dependency of the workspace. The current implementation uses `write_kv_unchecked` and manual string concatenation, which is less robust and harder to maintain than using the existing, battle-tested `serde_json::to_string`. This represents a minor but clear violation of engineering best practices.
    -   **§/file:line**: `experiments/minif2f_v4/src/fc_trace.rs:25-32`, `experiments/minif2f_v4/src/fc_trace.rs:240-245`

2.  **(CHALLENGE)** **Finding**: A data hazard exists due to timestamp divergence between two run identifiers. `run_corr_id` is created at the start of `run_swarm`, while `run_id` is created at the end in `make_pput`. They will differ by milliseconds, complicating joins for Phase D consumers. The join semantics are not documented, and this ambiguity should be resolved by using a single, consistently generated ID for both FC events and the final JSONL record.
    -   **§/file:line**: `experiments/minif2f_v4/src/bin/evaluator.rs:400-408` (creation of `run_corr_id`), `experiments/minif2f_v4/src/bin/evaluator.rs:1360-1365` (creation of `run_id`).

#### Q5 — SiliconFlow plumbing (A7)

1.  **(CHALLENGE)** **Finding**: The A7 acceptance test suite is incomplete. The smoke test (`_smoke_siliconflow.py`) intentionally bypasses the proxy to get a per-key verdict. However, there is no corresponding automated test that routes calls *through* the proxy. This leaves the proxy's core logic—`detect_provider` routing and the multi-key round-robin distribution—without a recurring, automated conformance test. A bug in this logic would be a silent regression.
    -   **§/file:line**: `scripts/smoke_siliconflow.sh`, `scripts/_smoke_siliconflow.py` (The finding is what is *absent*).

#### Q6 — Trust Root manifest expansion

1.  **(VETO)** **Finding**: The primary mitigation for a known catastrophic failure mode (V3L-27) is not under automated, recurring conformance testing. The A7 atom introduced multi-key round-robin logic in `llm_proxy.py` to prevent rate-limit collapse. The packet claims "Proxy round-robin verified [2,2,2] across 6 calls," but this was a one-time manual check, not a codified test. The Trust Root protects the *source code* of `llm_proxy.py` from tampering, but it does not protect against latent bugs or future regressions in its behavior. Without an automated test that asserts the round-robin distribution, the correctness of this critical mitigation is unverified from commit to commit. This is a direct violation of the spirit of C-075 (DO-178C tool qualification) and the rigor established in the A0 audit.
    -   **§/file:line**: `src/drivers/llm_proxy.py:99-104` (the `get_client_round_robin` logic), `§ 3. A7` (the claim of manual verification).

#### Q7 — Phase A → B exit decision

No findings on the checklist items themselves, but the VETO from Q6 blocks the exit decision. The infrastructure for Phase B appears to be in place, but the unverified risk in the underlying A7 plumbing is too high to authorize kernel instrumentation.

---

### Final Verdict

-   **VERDICT**: VETO
-   **Conviction**: High
-   **Recommendation**: REDESIGN

The VETO is issued due to the critical finding in Q6: the multi-key round-robin logic in `llm_proxy.py`, which is the sole mitigation for the documented V3L-27 rate-limit collapse, is not covered by any automated conformance test. Its correctness relies on a one-time manual check, which is insufficient for a constitutional gate. Authorizing Phase B, which will increase reliance on this proxy for larger-scale experiments, would be irresponsible without this assurance.

The "REDESIGN" recommendation applies specifically to the acceptance criteria and test suite for atom A7. A new atom (e.g., A7b) must be introduced to implement a recurring conformance test that:
1.  Spins up the `llm_proxy.py` server in a test harness.
2.  Issues a series of requests (e.g., 6 requests with 3 keys configured).
3.  Queries the `/stats` endpoint.
4.  Asserts that the `per_key_requests` distribution matches the expected round-robin pattern (e.g., `[2, 2, 2]`).

This new test must be added to the CI battery.

The CHALLENGE items from Q1, Q4, and Q5 should be addressed in a subsequent `FIX-THEN-PROCEED` cycle after the VETO condition is resolved. Proceeding to Phase B is not authorized until the A7 testing gap is closed and this audit passes.