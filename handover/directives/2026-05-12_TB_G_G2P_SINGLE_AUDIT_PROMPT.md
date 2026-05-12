# TB-G G2P — SINGLE-AUDITOR AUDIT PROMPT (Codex G2 only)

> **Audit kind**: single-auditor (Codex G2). Gemini DeepThink skipped per
> user 2026-05-12 verbatim "Gemini 总是 all pass — 意义不大" + session
> #42 cadence. G2P is a Class-2 production-wire-up atom (charter §1
> Module G2P "Class peak: 2 / §8 packet required: no"); under
> `feedback_dual_audit` "hybrid by risk class" the production-wire-up
> requirement was waived by user direction with mechanism explanation
> (one-auditor signal is the ship gate; Codex's adversarial axis is what
> rigor-finds).
>
> **Audit subject**: 9-task chain-continuous batch at
> `handover/evidence/g_phase_g2p_2026-05-12T01-34-37Z/` produced by
> `scripts/run_g_phase_batch.sh g_phase_g2p_<TS> full` after G2P.1 +
> G2P.2 + G2P.3 atoms shipped.
>
> **Atoms in scope**:
> - **G2P.1** SHIPPED 2026-05-12 — `src/sdk/pending_peer_reviews.rs`
>   per-viewer renderer wired into `build_agent_prompt` under canonical
>   `=== Pending Peer Reviews ===` heading; `evaluator.rs` swarm path
>   calls the renderer per LLM-call boundary.
> - **G2P.2** SHIPPED 2026-05-12 — `src/runtime/peer_verify_coverage.rs`
>   walker + `audit_dashboard --run-report` §F.X integration; explicit
>   MECHANISM BOTTLENECK render contract on `non_solver_verifications == 0`.
> - **G2P.3** SHIPPED 2026-05-12 — Class-1 audit of TB-N1 A4 admission
>   contract; `handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md`
>   filed documenting two forward gaps (Gap-A reputation accumulation,
>   Gap-B bond return at run-resolve — both Class-3+ G3.x forward work).
>
> **Repo HEAD on origin/main**: `eb6dac7` (post-G2P.3 + post-trust-root
> rehash + post-audit-prompt-commit). Run-start manifest pins
> `git_head=9ddc9c1`.
>
> **Empirical result (auto-rendered §F.X)**: `accepted_worktx_total=1`
> (P000 omega-solved), `peer_verifications_total=0`,
> `non_solver_verifications=0`. The §F.X MECHANISM BOTTLENECK block
> auto-emitted as designed by SG-G2P.5.a (silent-zero forbidden);
> this satisfies architect §8.5 "empty market as valid empirical
> result" + the user-approved /goal OR-branch ("≥1 non-solver
> VerifyTx OR explicit §F.X bottleneck"). The audit's primary
> verification surface is therefore: did the silent-zero contract
> render the bottleneck explanation, and are the three Class-1
> source-grep ship gates (G2P.1 shielding + G2P.3 OBS contract) intact?
>
> **Charter ship gates**:
> - SG-G2P.1 / SG-G2P.2 (G2P.1 prompt block — Pending Peer Reviews
>   per-viewer scoped, fixture renders) — closed pure-code.
> - SG-G2P.3 / SG-G2P.4 / SG-G2P.5 (G2P.2 walker + §F.X — per-agent
>   peer_verify_count + coverage % + explicit bottleneck on zero) —
>   closed pure-code.
> - SG-G2P.6 (G2P.3 — TB-N1 A4 gates GREEN OR OBS filed) — BOTH
>   clauses satisfied.
> - **Architect §8.2 ship gate** ("≥1 non-solver VerifyTx on another
>   agent's WorkTx") — this audit's primary empirical question. The
>   /goal accepts architect §8.5 "empty market as valid empirical
>   result" with explicit §F.X bottleneck as the OR branch.
>
> **Constitutional anchors**: charter §1 Module G2P + G-Phase directive
> `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
> §0.6 amendment G-2 + §8.2 + §8.5 + CLAUDE.md §15 shielding + §17
> reporting standard.

---

## §1. Evidence inventory

Evidence dir: `handover/evidence/g_phase_g2p_2026-05-12T01-34-37Z/`.

Expected files:
- `G_PHASE_BATCH_MANIFEST.json` — pre-batch pin; `git_head=9ddc9c1`,
  `problem_count=9`, `active_model=deepseek-chat`, `llm_proxy_url=
  http://localhost:8080`.
- `PROBLEMS.txt` — canonical 9-problem TB-N3 Phase 2 set.
- `runtime_repo/` — ONE shared git repo across the batch; expected L4
  chain length growth pattern mirrors G1.2-7 R2 (1 WorkTx accept +
  TerminalSummary per task). Look for additional Verify entries this
  time (G2P.1 may have activated the verify_peer pathway).
- `cas/` — ONE shared CAS.
- `BatchContinuationManifest.json` — G1.2-4 canonical `g1_2_v1` schema.
- `P000..P008/evaluator.stdout` + `.stderr` — per-task PPUT_RESULT.
- `aggregate_verdict.json` — `audit_tape` over shared chain.
- `PERSISTENCE_BINDING_REPORT.json` — G1.2-5 binding output. Expected
  `is_passing=true`, `n_tasks=9`. `n_witnessed` may match G1.2-7 R2
  baseline (4) — reputation accumulation remains a Class-3+ Gap-A per
  OBS_G2P_VERIFY_PEER_REWARD (not in G2P scope).
- `batch_evaluator.log` — orchestrator log; expect 8
  `ResumePreflight::Ok` + 8 `ChainTapeLease ACQUIRED`.
- `run_log.txt` — canonical post-audit summary.

---

## §2. Audit questions (Q1..Q12)

Each auditor returns:
```
VERDICT: <PROCEED|CHALLENGE|VETO|HALT>
CONVICTION: <high|medium|low>
Q1: <PASS|CHALLENGE|VETO> [reason if not PASS, with file:line refs]
...
Q12: ...
Notes: <free-form observations>
```

**Q1 (G2P.1 prompt block reaches LLM)**: Inspect at least 3
`P*_*/evaluator.stdout` files for evidence the agent observed a
`=== Pending Peer Reviews ===` block in its prompt context. The block
will only render when `q.economic_state_t.stakes_t` has ≥1 entry NOT
owned by the viewer AND not in `agent_verifications_t`. On a 9-task
batch starting with 1 preseed sponsor + LLM-induced solver agents,
the block should appear AT LEAST on tasks where another agent solved a
prior task (e.g., P001+ after P000's WorkTx accept). Confirm the
prompt text reads "Pending Peer Reviews" in the chain context (search
batch_evaluator stdout/stderr or inspect prompt_capsule CAS objects if
any).

**Q2 (G2P.1 selective shielding intact)**: Source-grep
`src/sdk/pending_peer_reviews.rs` to confirm the renderer reads ONLY
`stakes_t` + `agent_verifications_t` from `q.economic_state_t`. Reject
if the renderer reads `attempt_telemetry`, `prompt_capsule`,
`proof_artifact`, raw stderr, or any other private CoT surface. The
SG-G2P.1.c source-grep gate already pins this; confirm the grep set is
adequate.

**Q3 (G2P.2 walker derives from canonical L4 + CAS)**: Run
`compute_peer_verify_coverage_from_paths(runtime_repo, cas)` mentally
or inspect `src/runtime/peer_verify_coverage.rs` — does the walker
iterate `1..=writer.len()` reading `LedgerEntry` payloads via
`canonical_decode<TypedTx>(cas.get(entry.tx_payload_cid))`? Confirm
NO use of attempt_telemetry / verification_result CAS reads (those are
private/audit-only).

**Q4 (§F.X dashboard renders the architect §8.2 signal)**: Run
`audit_dashboard --run-report` against the run's
`runtime_repo` + `cas/` paths. Confirm output contains the
`## §F.X Peer-verify coverage` heading with these labeled rows:
`accepted_worktx_total`, `accepted_worktx_with_verify`, `coverage_pct`,
`peer_verifications_total`, `non_solver_verifications`. If
`non_solver_verifications > 0`, also confirm per-agent breakdown rows
with `(solver)` / `(non_solver)` role tags.

**Q5 (silent-zero forbidden contract)**: If
`non_solver_verifications == 0` in the rendered §F.X, the
`MECHANISM BOTTLENECK` block MUST appear with ≥3 candidate causes
(round-robin scheduler / Pending Peer Reviews prompt-block / bond
budget). Reject if the dashboard shows `non_solver_verifications: 0`
WITHOUT the bottleneck explanation. The constitution-test gate
SG-G2P.5.a pins this in the unit-test layer; confirm the production
runtime path also emits it.

**Q6 (architect §8.2 ship gate empirical outcome)**: This is the
primary question — did the smoke produce `peer_verifications_total ≥ 1`
AND/OR `non_solver_verifications ≥ 1`? Report the exact numbers from
the §F.X block. If 0, confirm the §F.X bottleneck explanation rendered
(architect §8.5 "empty market as valid empirical result" — the goal
accepts this branch).

**Q7 (G2P.3 OBS contract)**: Inspect
`handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md`.
Confirm it documents Gap-A (reputation accumulation absent) + Gap-B
(bond return at run-resolve absent) with forward-closure criteria. The
SG-G2P.6.c "fail-on-fix" scaffold asserts current sequencer.rs DOES
NOT mutate reputations_t in any admission arm; verify the source still
matches that assertion (no Class-3+ closure was attempted in this
atom). Reject if reputations_t mutation was silently added without
architect §8 packet.

**Q8 (TB-N1 A4 admission still GREEN)**: Confirm
`tests/constitution_n1_agent_economy_a4.rs` 7/7 PASS at the audit HEAD
(the G2P.3 binding pins this; quick check that nothing in G2P.1 /
G2P.2 / G2P.3 broke the admission contract).

**Q9 (continuity at scale)**: Mirror Q1 from G1.2-7 R2 — verify the
9-task BatchContinuationManifest has continuity invariant
`tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex` for k ∈
{0..7}, and the live `runtime_repo` HEAD matches
`tasks.last().end_head_t_hex`.

**Q10 (one genesis at scale)**: Mirror Q2 from G1.2-7 R2 — confirm
EXACTLY ONE `genesis_report.json` at `runtime_repo/`; no per-task
genesis files; no `P*_*/runtime_repo` directories.

**Q11 (persistence binding regression)**: `PERSISTENCE_BINDING_REPORT.json`
should still show `is_passing=true`, `n_tasks=9`. The architect §3.5
clean-negative is permitted on the 5 Empty fields (reputation +
autopsy remain Empty per OBS_G2P_VERIFY_PEER_REWARD Gap-A; the other
4 should match G1.2-7 R2 baseline shape). Reject if any field flipped
from Witnessed to Reset (kill_criteria_tested #1 violation).

**Q12 (kill-criteria across batch)**: Mirror Q9 from G1.2-7 R2 — audit
against charter §0 `kill_criteria_tested` 1-5: no per-problem genesis
reset / Predicate doesn't read price-market-trace / no bankrupt-cap
bypass / no hidden model switch / conservation invariants hold. Plus
G2P-specific: `agent_verifications_t` if non-empty should be set-only
(no duplicate (verifier, target) pairs at the wire level — TB-N1 A4
Step-3.5 enforces this; verify by counting unique pairs).

---

## §3. Halt conditions (block ship)

- Q2 FAIL — G2P.1 shielding broken (private surface read in renderer).
- Q5 FAIL — silent-zero on `non_solver_verifications == 0` (no
  bottleneck block).
- Q7 FAIL — reputations_t mutation silently added without architect
  §8 packet (Class-3+ hidden in Class-2 commit per
  `feedback_class4_cannot_hide_in_class3`).
- Q8 FAIL — TB-N1 A4 regression (admission contract broken).
- Q9 FAIL — HEAD_t discontinuity.
- Q10 FAIL — fresh `genesis_report.json` at task_index > 0.
- Q11 FAIL — Witnessed → Reset on any field.
- Q12 FAIL — any kill_criteria_tested clause violation.

Q1 / Q6 are EMPIRICAL questions; their answers (verify_count = N) are
data points, not pass/fail.

---

## §4. Output format

Codex G2 emits in this order:

```
VERDICT: <PROCEED|CHALLENGE|VETO|HALT>
CONVICTION: <high|medium|low>
Q1: <PASS|CHALLENGE|VETO> [reason if not PASS, with file:line refs]
Q2: ...
...
Q12: ...
Notes: <free-form observations; flag any provenance / audit-trail gaps
       even if not blocking; report Q6 empirical numbers verbatim>
```

---

## §5. Cross-references

- G2P.1 ship commit: `6e374f9` ("TB-G G2P.1 SHIPPED")
- G2P.2 ship commit: `ebc2e29` ("TB-G G2P.2 SHIPPED")
- G2P.3 ship commit: `93a3068` ("TB-G G2P.3 SHIPPED")
- Trust-root rehashes: `58d4ded` (G2P.1) + `9ddc9c1` (G2P.2)
- Predecessor G1.2-7 R2 dual-audit prompt:
  `handover/directives/2026-05-11_TB_G_G1_2_7_DUAL_AUDIT_PROMPT.md`
- Predecessor G1.2-7 R2 Codex verdict:
  `handover/audits/CODEX_G2_TB_G_G1_2_7_R2_VERDICT.md`
- OBS Gap-A + Gap-B: `handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md`
- Charter §1 Module G2P:
  `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
- G-Phase directive §0.6 + §8.2 + §8.5:
  `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
