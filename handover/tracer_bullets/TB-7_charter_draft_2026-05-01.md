# TB-7 Charter — DRAFT — Per-LLM-Proposal WorkTx Routing (Frame B chaintape closure)

**Status**: **DRAFT — NOT YET AUTHORIZED**. Awaiting architect ratification per `handover/directives/2026-05-01_TB7_ARCHITECT_REVIEW_REQUEST.md`.
**Date**: 2026-05-01
**Author**: Claude (post-TB-6 ship analysis)
**Predecessor**: TB-6 SHIPPED 2026-05-01 (`17c5e73`); architect Path A goal achieved at narrow Frame A (D2 8-condition gate).
**phase_id**: **P2 (primary; closes Frame B from architect § 5.2 distance estimate)**

---

## §0 Why this TB exists (one paragraph)

TB-6 closed Frame A — `production binary triggers Sequencer::apply_one + on-disk LedgerEntry chain + replay verifier`. But the **real LLM activity** (proposal generation, Lean verification, OMEGA accept) still routes through the legacy `bus.append` / `bus.append_oracle_accepted` path, NOT through `bus.submit_typed_tx → Sequencer::apply_one`. The `handover/evidence/tb_6_chaintape_smoke_2026-05-01/` chain contains exactly 2 entries (1 synthetic TaskOpen + 1 synthetic zero-stake WorkTx) — none of the 20 LLM proposals on `mathd_algebra_107` traverse the chain. PputResult is computed by the evaluator's in-memory accumulator and is NOT chain-derivable. TB-7 closes Frame B: every meaningful LLM action becomes an on-chain `WorkTx`/`VerifyTx` so PputResult is verifiable from L4 + L4.E + CAS alone, not from evaluator stdout.

---

## §1 One-line goal

Every LLM proposal in `evaluator::run_swarm` (append branch + complete/OMEGA branch) routes through `bus.submit_typed_tx` and lands in L4 (accepted) or L4.E (rejected) on the production chaintape, with chain-derivable PputResult that matches the in-memory accumulator within tolerance ε.

---

## §2 roadmap_exit_criteria_addressed

- **P1:5,6,7,8,9** — re-discharged on REAL LLM activity (TB-6 was synthetic seed only).
- **P2:1** — Agent identity is on-chain via per-agent Ed25519 keypair (runtime-generated; pinned at first-tx-submission timestamp). NEW exit criterion candidate.
- **P2:6** — REAL Agent outputs enter CAS (proposal payload bytes); ledger records CID. Re-discharged from synthetic seed to real per-proposal.
- **P3 carry-forward** — WorkTx.stake / escrow / ChallengeResolve invariants still replay under real LLM activity.

---

## §3 kill_criteria_tested

- **P1:1** — Agent cannot bypass `bus.submit_typed_tx` to mutate state — re-tested via real LLM evaluator runs (every proposal MUST go through submit_typed_tx; verified via post-run chain length ≥ N where N = proposal count).
- **P1:2** — Rejected WorkTx does not advance state_root — re-tested on real per-proposal rejections (predicate fail / stake insufficient / parent root stale).
- **P1:3** — Replay reconstructs state — re-tested on a chain with ~20 real LLM tx entries.
- **P1:4** — Rejected raw log does not pollute agent read view — re-tested on real per-proposal rejections.
- **P3:1,2,3** — re-tested through real per-proposal admission.

**Kill criteria NOT tested by TB-7**:
- P3:9 (slash) — RSP-3.2 = TB-9.
- Settlement / FinalizeRewardTx → real reward payout — RSP-4 = TB-11.
- NodeMarket position semantics — RSP-M = TB-7+ separate sequencing.

---

## §4 WP-canonical decision blocks (binding once ratified)

### 4.1 No new TypedTx variant

Per WP § 14.1: existing `WorkTx` + `VerifyTx` cover proposal + verification. TB-7 wires evaluator → these existing variants; does NOT introduce new variants. Charter § 6 #6 rule from TB-6 inherited.

### 4.2 Agent keypair = per-agent runtime-generated Ed25519 + AgentPublicKeyRegistry

Each `agent_id` (e.g., `n1`, `swarm_a`, `swarm_b`) gets a runtime-generated Ed25519 keypair at evaluator startup. The public key is recorded in a new on-disk manifest `<runtime_repo>/agent_pubkeys.json` (analogous to TB-6's `pinned_pubkeys.json` but agent-side). Private key lives in process memory only and is dropped at evaluator exit. **Structurally analogous to TB-5's PinnedSystemPubkeys but for agents** — same proven shape.

WorkTx.signature is now a real Ed25519 signature over the canonical WorkTx digest. Replay verifier (extended Atom 4 path) re-verifies every WorkTx.signature against the agent_pubkeys manifest.

### 4.3 OMEGA-accept path scope (NARROWED)

The full OMEGA → settlement loop is `WorkTx → VerifyTx → ChallengeWindow → FinalizeRewardTx`. TB-7 covers ONLY the front half:
- Accepted OMEGA proposal → `WorkTx` with non-zero stake + signature
- Lean verification result → `VerifyTx` with bond + verdict
- ChallengeWindow stays OPEN at TB-7 ship (no slash; no settlement)
- `FinalizeRewardTx` → DEFERRED to TB-11 RSP-4 (SettlementEngine + ContributionDAG)
- `SlashTx` → DEFERRED to TB-9 RSP-3.2

Rationale: settlement requires reward distribution math (creator royalty + verifier fee + reuse-DAG flow) which is RSP-4 territory. Architect ruling 2026-05-01 § 4.5 sequences this as TB-11. TB-7 stops at "the chain shows who proposed what + who verified what + which were accepted/rejected" without claiming the economics are settled.

### 4.4 Chain-derived PPUT must match in-memory PputResult within tolerance

Atom 5 introduces `runtime::chain_derived_pput::compute_pput_from_chain(runtime_repo, cas) -> PputResult`. The chain-derived value MUST match the in-memory `PputResult` from `run_swarm` within tolerance ε for time-insensitive fields (`solved`, `verified`, `tx_count`, `golden_path_token_count`, `gp_payload`, `tactic_diversity`). Time-sensitive fields (`total_wall_time_ms`, `verifier_wait_ms`) are excluded — chain replay is byte-deterministic but wall time is not.

If divergence: in-memory accumulator is treated as **wrong** (the chain is canonical post-TB-7). Tests force any drift to surface as an Atom 5 test failure.

### 4.5 Audit mode = hybrid by risk class (production wire-up class — Codex impl + Gemini arch)

Per architect ruling D3 inherited. TB-7 touches `experiments/minif2f_v4/src/bin/evaluator.rs` main loop (~2-3 hot per-proposal sites) + adds agent keypair management. This IS production wire-up class. Codex impl audit on ship; Gemini arch with `degraded` fallback per `feedback_dual_audit`.

If TB-6 follow-up Codex impl audit on full TB-6 diff has not closed by TB-7 ship, bundle the two together at Atom 7.

### 4.6 STEP_B-protected if `src/state/sequencer.rs` or `src/bus.rs` touched

Atom 1 (agent keypair) is purely additive in a NEW module `src/runtime/agent_keypairs.rs`. NOT STEP_B-protected.

Atom 2 (evaluator wiring) edits `experiments/minif2f_v4/src/bin/evaluator.rs` only. NOT STEP_B-protected (sub-crate).

If any atom needs to thread state through `Sequencer::SubmissionEnvelope` or `bus.submit_typed_tx`, that triggers STEP_B Phase-0 preflight per CLAUDE.md.

### 4.7 cargo test --workspace canonical at every atom

Per architect ruling D4 inherited from TB-6.

### 4.8 24h iteration cap with production wire-up exception

Per `feedback_iteration_cap_24h`: TB-7 produces evaluator pass/fail signal at Atom 6 (chain-backed real-LLM smoke). Atoms 1-5 are spec-class but on shortest path. Discharge gate: Atom 6 must run within 72h of Atom 0 ship; if not, escalate.

---

## §5 Build surface

### 5.1 Files touched (anticipated)

| File | Touch | Audit-class | STEP_B? |
|---|---|---|---|
| `src/runtime/agent_keypairs.rs` (NEW) | Atom 1 — per-agent Ed25519 keypair manager + on-disk pubkey manifest | additive non-STEP_B | no |
| `src/runtime/adapter.rs` | Atom 2 — extend `make_synthetic_worktx` family with real-signature variant `make_real_worktx_signed_by(...)` | additive | no |
| `src/runtime/chain_derived_pput.rs` (NEW) | Atom 5 — read L4 + L4.E + CAS, compute PputResult equivalents | additive | no |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | Atoms 2-3 — wire `bus.append` + `bus.append_oracle_accepted` sites to also emit WorkTx via `bus.submit_typed_tx` | sub-crate; production wire-up class | no |
| `tests/tb_7_per_proposal_routing.rs` (NEW) | Atom 4-6 — battery covering ≥3 LLM-proposal-equivalents on the chain | additive | no |

NO change to `src/state/sequencer.rs` / `src/bus.rs` / `src/main.rs` / `src/sdk/tools/wallet.rs` / `src/kernel.rs` / `src/state/typed_tx.rs` anticipated. STEP_B not triggered.

### 5.2 Tests added in TB-7 (minimum)

- **Atom 1**: 4-5 in-module unit tests for agent_keypairs (generate / persist manifest / reload / sign / verify).
- **Atom 2**: 2-3 integration tests proving real-signature WorkTx admits cleanly via `bus.submit_typed_tx`.
- **Atom 3**: 2 integration tests for OMEGA path → WorkTx + VerifyTx pair on chain.
- **Atom 4**: extension to `verify_chaintape` to verify agent-signature chain (analogous to system-signature path).
- **Atom 5**: chain-derived PPUT round-trip — runs a synthetic 5-proposal sequence + asserts derived PPUT == in-memory accumulator within ε.
- **Atom 6**: chain-backed real-LLM smoke run on `mathd_algebra_107`; produces ≥N L4 entries where N = LLM proposal count.

Target: `cargo test --workspace` workspace_count green; +15-20 new tests over TB-6 ship 660.

### 5.3 Smoke evidence shape

`handover/evidence/tb_7_chaintape_smoke_2026-05-XX/` mirroring TB-6 shape plus:
- `agent_pubkeys.json` (NEW) — per-agent Ed25519 manifest
- `chain_derived_pput.json` (NEW) — Atom 5 output; cross-compared to `pput_result.jsonl`
- L4 entry count ≥ N (where N = LLM proposal count, expected ~20 for mathd_algebra_107 single-shot)

### 5.4 README MUST answer (extending TB-6's 8 questions)

Plus:
- Q9: Was every LLM proposal recorded in L4 / L4.E? (count comparison)
- Q10: Does chain-derived PPUT match in-memory PputResult? (within ε)
- Q11: Are all WorkTx signatures verifiable against `agent_pubkeys.json`?

---

## §6 Forbidden (inherits TB-6 § 6 + TB-7-specific additions)

Inherits all 20 TB-6 forbidden items. TB-7 additions:

21. **No FinalizeRewardTx wiring** — RSP-4 / TB-11 territory. TB-7 stops at proposal + verify; does NOT close settlement.
22. **No SlashTx wiring** — RSP-3.2 / TB-9 territory.
23. **No NodeMarket position semantics** — RSP-M / TB-7+ separate sequencing per architect ruling § 4.5.
24. **No new TypedTx variant** (re-emphasized; existing WorkTx + VerifyTx suffice).
25. **No Q schema mutation** — agent_pubkeys live in CAS / sidecar manifest, not in QState (mirrors TB-6 charter § 6 #10).
26. **No agent chain-of-thought broadcast** — re-emphasized; AgentProposalRecord (TB-6 Atom 5) shape preserved without expansion.
27. **No bypassing TB-6 chaintape gate** — every TB-7 atom STILL must produce chain-backed evidence + verify_chaintape PASS.
28. **No regression on Atom 5 + Atom 6 hooks** — TB-6 audit-trail + RunSummary auto-write must continue to work post-TB-7 changes.
29. **No claim that TB-7 closes Frame C** — TB-7 = Frame B ONLY. Frame C (full economic loop) is TB-9 + TB-11 + TB-7+RSP-M0.
30. **No Codex-only ship without explicit `degraded` Gemini label** — TB-6 ship audit precedent.

---

## §7 Atom plan (DRAFT; subject to architect refinement)

```text
Atom 0 — Charter ratification + audit prompt + memory updates if any (THIS DRAFT)
Atom 1 — Agent keypair management (src/runtime/agent_keypairs.rs + agent_pubkeys.json manifest)
Atom 2 — Evaluator append-branch routing (per-LLM-proposal WorkTx via bus.submit_typed_tx)
Atom 3 — Evaluator OMEGA-branch routing (WorkTx + VerifyTx pair; ChallengeWindow OPEN; no settlement)
Atom 4 — verify_chaintape extension (agent-signature verification path)
Atom 5 — chain_derived_pput.rs — compute PputResult from L4 + L4.E + CAS alone
Atom 6 — chain-backed real-LLM smoke run on mathd_algebra_107 (≥N L4 entries; chain-derived PPUT matches in-memory)
Atom 7 — Audit + ship (Codex impl + Gemini arch with degraded fallback; recursive self-audit)
```

### 7.1 Sequencing

- Atom 1 must land before Atom 2 (Atom 2 needs the keypair manager).
- Atom 2 + Atom 3 in any order after Atom 1.
- Atom 4 + Atom 5 after Atoms 2+3 (need real chain to verify).
- Atom 6 after Atoms 1-5 (smoke uses everything).
- Atom 7 after all.

### 7.2 24h iteration cap

Atom 6 is the discharge gate. Must run within 72h of Atom 0 ship per production wire-up exception.

---

## §8 Three declarative success proofs

**Proof 1 — Every LLM proposal lands on chain**:
> Real LLM evaluator run on `mathd_algebra_107` (single-shot, MAX_TX=20) produces a `runtime_repo/` containing N L4 + L4.E entries where N matches the in-memory `proposal_count` from PputResult. Each WorkTx carries a non-zero Ed25519 signature verifiable against `agent_pubkeys.json`.

**Proof 2 — Chain-derived PPUT matches in-memory**:
> `runtime::chain_derived_pput::compute_pput_from_chain(runtime_repo, cas)` returns a PputResult that matches the in-memory accumulator's PputResult on `solved`, `verified`, `tx_count`, `golden_path_token_count`, `gp_payload`, `tactic_diversity` (time-insensitive fields). Drift fails the Atom 5 test.

**Proof 3 — Tampering with any chain entry breaks both replay AND chain-derived PPUT**:
> `verify_chaintape` extended to verify agent_signatures detects tampered WorkTx; `compute_pput_from_chain` bails out (or returns wrong PputResult, detected by Atom 5 test) on any chain-byte tamper. The pair makes chain-derived PPUT cryptographically attested rather than evaluator-stdout-attested.

---

## §9 Audit gate

**Charter stage (Atom 0)**: NO external audit. User + architect review of charter only.

**Atom 2 STEP_B Phase-0** (if `bus.rs` / `sequencer.rs` touch surfaces during preflight): narrow Codex audit. Currently anticipated NOT to need this; preflight at Atom 1 STEP_B-equivalent doc will confirm.

**Atom 7 ship audit**: Codex impl audit + Gemini arch audit. Per architect ruling D3 hybrid-by-risk: production wire-up = full audit required. If Gemini exhausted → `degraded` label.

**Bundling**: if TB-6 follow-up Codex impl audit on full TB-6 diff is still pending at Atom 7, bundle the two audits.

---

## §10 Day-1 deliverables (Atom 0; this DRAFT commit)

1. **This charter draft** at `handover/tracer_bullets/TB-7_charter_draft_2026-05-01.md` (NEW; this file).
2. **Architect review request** at `handover/directives/2026-05-01_TB7_ARCHITECT_REVIEW_REQUEST.md` (NEW). 5 binding decision items D1-D5.
3. **NOTEPAD draft entry** — referenced as DRAFT until architect ratifies.
4. **TB_LOG TB-7 row** — NOT created until architect ratifies (preserves the "no fake activity" property).

No production code is touched on Atom 0.

---

## §11 Cross-references

- **Architect ruling that authorized TB-6**: `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md`
- **TB-6 charter (predecessor shape)**: `handover/tracer_bullets/TB-6_charter_2026-05-01.md`
- **TB-6 recursive self-audit**: `handover/audits/RECURSIVE_AUDIT_TB_6_2026-05-01.md`
- **TB-6 ship merge series**: `7970d2d..17c5e73`
- **TB-7 architect review request**: `handover/directives/2026-05-01_TB7_ARCHITECT_REVIEW_REQUEST.md`
- **9-phase roadmap**: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § 11.5 amendment
- **Frame B distance estimate**: post-TB-6 user dialogue (this session); architect § 5.2 self-estimate
- **Memory rules** (inherited):
  - `feedback_chaintape_wire_up_priority` — Frame B is the natural successor to Frame A
  - `feedback_dual_audit` — production wire-up class with degraded fallback
  - `feedback_workspace_test_canonical` — canonical ship-gate
  - `feedback_iteration_cap_24h` — 72h-to-feedback-loop production exception
  - `feedback_step_b_protocol` — only triggered if restricted file touched
  - `feedback_smoke_evidence_naming` — chain-backed only may be called "smoke tape"
  - `feedback_tb_phase_tag_required` — phase_id + roadmap_exit_criteria_addressed + kill_criteria_tested mandatory at charter time

---

## §12 Open questions for architect (resolve at ratification time)

- **Q1**: Is OMEGA-accept narrowing (no FinalizeRewardTx in TB-7) the right scope, or should TB-7 also wire FinalizeRewardTx as a stub (zero reward + audit-only)?
- **Q2**: Agent keypair lifecycle — runtime-generated per evaluator-run (current proposal) or persistent across runs (would require key storage)?
- **Q3**: Tolerance ε for chain-derived PPUT — bit-exact on time-insensitive fields, or numerical tolerance for any float fields?
- **Q4**: Should TB-7 bundle the deferred Codex impl audit on full TB-6 diff at Atom 7, or run that as a separate pre-TB-7 follow-up?
- **Q5**: NodeMarket / RSP-M0/M1 — is TB-7 exclusively Frame B, or should we ALSO activate RSP-M0 decision record now (architect ruling § 4.3 mentioned it as reserved-future post-TB-6)?
- **Q6**: 30 atoms is too many; do we need to merge any?

These resolve at architect ratification of this draft.
