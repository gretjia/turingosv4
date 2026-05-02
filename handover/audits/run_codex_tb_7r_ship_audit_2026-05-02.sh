#!/usr/bin/env bash
# Codex TB-7R ship audit (audit-point-2 per charter §3 Deliverable G).
# Class 3 (auth-crypto-money) — Codex implementation-paranoid angle.
# Independent of Gemini ship audit (parallel, architectural angle).
# Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md"
TMP_PROMPT="$(mktemp /tmp/tb7r_codex_ship.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-7R Ship Audit (audit-point-2; implementation-paranoid)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini ship audit (parallel, architectural angle).

**Mandate**: TB-7R ship-gate dual external audit per charter §3 Deliverable G audit-point-2. Class 3 (auth-crypto-money). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Round cap = 2 per `feedback_elon_mode_policy`.

You did the audit-point-1 micro-audit on 2026-05-02 (`handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md`). That audit returned CHALLENGE on Claim 7 (TRACE_MATRIX backlink shape). The remediation log §"Claim 7 fix detail" + commit `b517ae5` registered the affected items as orphans in `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md` per architect verdict §C9. Aggregate concern #2 (raw-CAS purity transcript reproducibility) carries forward to THIS audit. Aggregate concern #3 (atom-6 README annotation hook revert) carries forward as non-blocking cleanup.

This ship audit covers the FULL TB-7R range — the additional commits since the micro-audit are `b517ae5..4470036` (audit-fix + smoke evidence + parent_tx ParentTxState + 6 conformance tests).

## Audit target

```text
Predecessor (TB-7 ship):  9e74195
TB-7R range:              9e74195..4470036  (5 commits)
HEAD:                     4470036  TB-7R parent_tx ParentTxState + 6 conformance tests + verdict 2026-05-02
                          013f2ce  TB-7R F (smoke evidence) — 10 runs across single/half/full
                          b517ae5  TB-7R audit-fix — Codex CHALLENGE Claim 7 → orphan TRACE_MATRIX
                          392a516  TB-7R C+D+CP2 — genesis_report.json emission + on-chain TaskOpen/EscrowLock
                          696d10f  TB-7R A+B+E — verdict ingestion + L4 purity audit + ChainTape-mode fail-closed
```

Workspace canonical test count at HEAD: `cargo test --workspace` → **712 / 0 / 150** (+26 net TB-7R tests vs TB-7 ship 686/0/150 baseline). Mandated reporting shape per `feedback_workspace_test_canonical`.

## The four-clause acceptance criterion (charter §1)

```text
1. For every externalized LLM proposal:
     L4 accepted WorkTx OR L4.E rejected evidence — never both, never neither.
2. For every L4 accepted WorkTx:
     predicate evidence (Lean VerificationResult) exists and resolves from CAS.
3. For every failed proposal:
     in L4.E only; raw diagnostic shielded but auditable.
4. For every dashboard report:
     deletable and regeneratable from ChainTape + CAS alone.
```

## The 7 ship conditions (architect verdict 2026-05-02 §4)

```text
1. All seven dashboard indicators remain green
2. All real externalized proposals are represented in L4 or L4.E
3. Solved runs have chain_oracle_verified=true and a rendered golden path
4. Unsolved runs have no fake accepted nodes
5. Proposal telemetry and proposal payload CIDs resolve
6. Forced parent_tx conformance test passes (6/6 in tests/tb_7r_parent_tx_conformance.rs)
7. README explicitly states that natural parent_tx_edges=0 occurred because
   complete-tool runs solved in one proposal
```

## Two architect-acknowledged OBS items (folded in for awareness)

These are KNOWN at the time of this audit. Treat them as published claims; flag VETO only if you find evidence the OBS bounding is wrong (i.e. the issue is actually a TB-7R ship blocker, not a follow-up).

**OBS-1 (architect-acknowledged):** `handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`
- Architect verdict 2026-05-02 §6 explicitly frames coverage-denominator hardening as **post-TB-7R**, not a TB-7R blocker.
- Concern: `step` tool's `PartialOk` and `Reject` branches consume LLM responses without routing through `submit_typed_tx`. Under the strict three-node taxonomy, that's internally consistent (only `submit_typed_tx` is "externalized"), but it leaves an implicit "LLM-output → externalized-proposal" step uncovered.
- Question for you: is the OBS's bounding correct? Specifically — do the `PartialOk` and `Reject` branches actually leave any path that fakes an L4 accepted entry, OR do they only leave LLM activity unchained without crossing into L4? If the latter, OBS bounding stands and this is a follow-up. If the former, that's a VETO.

**OBS-2 (architect-silent):** `handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`
- Claude self-flagged 2026-05-02; architect verdict did NOT explicitly address. Per OBS §3, "this is NOT a TB-7R ship blocker per the verdict's listed ship conditions." Flagged for future architect ruling.
- Concern: `acc.record_tool_stdout(reason)` at `experiments/minif2f_v4/src/bin/evaluator.rs:2430-2438` flows raw Lean error reasons into next-prompt's errors_history. In multi-agent runs, if `acc` is shared, agent-1's raw Lean error could leak into agent-2's prompt, potentially violating Art. III.4 selective-broadcasting / shielding.
- Question for you: in TB-7R smoke evidence (n5 in particular — `full_n5_mathd_algebra_171/`), can you find concrete cross-agent prompt pollution? Or is `acc` per-agent in practice? If per-agent → OBS stands as future-ruling-required-but-not-blocking. If actually polluted → flag the severity (CHALLENGE if minor / VETO if it makes multi-agent runs fundamentally unsafe).

## Round-1 audit questions (10; implementation-paranoid)

**Q1. L4 purity (raw-CAS reproducibility carry-over from audit-point-1 #2)**: walk the TB-7R smoke evidence at `handover/evidence/tb_7r_smoke_2026-05-02/`. The smoke captures 10 runs (1 single + 3 half + 1 full-A n5 + 5 full-B n1). For each L4 Work entry across those 10 runs, can you confirm the four purity criteria (ProposalTelemetry CID resolves; verification_result_cid resolves; verified==true; proof_artifact_cid resolves)? You said in audit-point-1 the gap was "committed evidence does not include the raw CAS store" — does the new smoke include cas/ directories committed to the repo? Cite paths + CID counts.

**Q2. Coverage denominator (OBS-1 bounding check)**: Read `experiments/minif2f_v4/src/bin/evaluator.rs` `step` tool dispatch. Trace each of `Complete`, `PartialOk`, and `Reject` paths from LLM-response-receipt to its terminal write. For each path:
- Does it write to L4? L4.E? Neither?
- Could a `PartialOk` LLM action ever produce an "accepted" claim downstream (e.g. via OMEGA-pertactic summary)? If yes, where, and is that path predicate-checked?
- Is there ANY code path where a Lean-failed (`Reject`) proposal could land in L4 accepted under a different label? Example anti-pattern: a wrapper/aggregator that summarizes multiple `step` rejects into a single "successful proof attempt" WorkTx.

If you find a path that lets a `Reject` reach L4 — VETO. If you find LLM activity that's just unchained (PartialOk → kernel.tape only; Reject → in-memory counter), the OBS bounding stands.

**Q3. Fail-closed gate (verdict §C ChainTape-mode unsupported paths)**: re-verify your audit-point-1 Claim 2 finding holds at HEAD. Specifically check `experiments/minif2f_v4/src/chaintape_mode_gate.rs` truth table; the call site in `evaluator.rs`; and that no NEW production-binary entry-point bypasses the gate. List any newly-added `bus.append(`, `bus.append_oracle_accepted(`, or other shadow-write site introduced in `b517ae5..4470036` and verify they are either annotated `shadow_only` or covered by the gate.

**Q4. Genesis report emission integrity (Deliverable C)**: at `392a516`, `genesis_report.json` lands. Verify:
- The 9 fields all populate when preseed enabled.
- The constitution_hash matches `genesis_payload.toml` at HEAD.
- The on-chain `task_open_tx` + `escrow_lock_tx` strings reference real L4 entries (not synthetic placeholders).
- TB-7R smoke single_n1 dir's `genesis_report.json` is internally consistent with its `runtime_repo`'s L4 chain.

**Q5. On-chain TaskOpen + EscrowLock (Deliverable D)**: at `392a516`, the bootstrap path emits TaskOpenTx + EscrowLockTx via `bus.submit_typed_tx`. Verify:
- No remaining `task_markets_t.insert(...)` / `escrows_t.insert(...)` direct memory writes on the production preseed path (search whole `src/` + `experiments/`).
- TB-7R smoke dirs' L4 chain reconstructs `task_markets_t` + `escrows_t` from L4 alone (replay-side check; cite the replay_report's `economic_state_reconstructed` + state field counts).

**Q6. parent_tx ParentTxState (commit `4470036` per verdict 2026-05-02 §3)**: verify all six conformance tests at `tests/tb_7r_parent_tx_conformance.rs` are real (not stubbed). Specifically:
- Test 2 (`second_attempt_same_branch_has_parent_tx`) — fixture uses `≥2 attempts on same branch` and asserts `attempt_2.parent_tx == Some(attempt_1.tx_id)`. Is the assertion the actual chain field, or a derived materialized-view field?
- Test 3 (`missing_parent_on_nonroot_attempt_is_violation`) — fixture deliberately sets `attempt_2.parent_tx = None` on a non-root attempt; assertion is that the dashboard / replay flags this as `MissingParentTxViolation`. Is the violation detection done at chain layer or only at dashboard layer? If only dashboard, that's a CHALLENGE — the chain itself must reject (or at least observably-flag) malformed multi-attempt branches.
- The four `parent_tx_state` enum values (`SingletonGoldenPathValid` / `NoMultiAttemptObserved` / `MultiAttemptDagValid` / `MissingParentTxViolation`) are exhaustively distinguished — no panic-on-unhandled fallthrough.

**Q7. Smoke-evidence integrity (Deliverable F + ship condition #1, #2, #3, #4, #5, #7)**: walk the smoke README at `handover/evidence/tb_7r_smoke_2026-05-02/README.md`. For each of the 10 runs:
- All 7 dashboard indicators GREEN (per ship cond #1)?
- chain_oracle_verified matches solved-status (true on solved, false on unsolved per ship cond #3 + #4)?
- replay_report.json exists and matches (per ship cond #5)?
- No fake accepted nodes on the 2 unsolved runs (per ship cond #4)?
- The README §2 "natural parent_tx_edges=0" wording satisfies ship cond #7 — is it explicit and architect-aligned?

**Q8. Forbidden-list compliance (charter §4 hard guardrails)**: re-verify your audit-point-1 Claim 5 finding holds across the FULL TB-7R range `9e74195..4470036`. Specifically check no commit introduces:
- NodeMarket / position semantics
- new TypedTx variants
- per-tactic decomposition logic
- predicate-registry mutation
- constitution.md / RootBox / sudo touches
- retroactive ledger rewrite (e.g. modifying old runtime_repo/refs/transitions/main commits)
- fabricated historical genesis_report.json (i.e. no genesis_report.json appearing inside `handover/evidence/tb_7_*_2026-05-01/` directories that didn't have one before)

**Q9. Workspace test count + canonical reporting**: at HEAD `4470036`, the claim is `cargo test --workspace = 712/0/150`. Re-verify:
- The +26 delta vs TB-7 ship is accounted for by net-new TB-7R tests + 6 parent_tx conformance + ParentTxState unit tests, NOT by uncounted ignored tests promoted into the count.
- `cargo test --workspace` count (canonical) is what's reported; bare `cargo test` count is NOT used (per `feedback_workspace_test_canonical`).
- No tests are `#[ignore]`'d that should be running (check the TB-7R commit range for new `#[ignore]` annotations).

**Q10. Prompt pollution OBS-2 (multi-agent shielding)**: in TB-7R full-A smoke (n5 × `mathd_algebra_171`), `agent_audit_trail.jsonl` records what each agent saw + submitted. Can you concretely demonstrate either:
- (a) one agent's raw Lean error reason appears in another agent's prompt context (= Art. III.4 violation; flag severity), OR
- (b) `acc.errors_history` is per-agent in practice / no cross-agent leakage (= OBS bounding stands; ship-eligible)?
Cite specific lines from the smoke evidence.

## Verdict format

Section A: Overall ship verdict (PASS / CHALLENGE / VETO) with conviction (LOW / MED / HIGH).
Section B: Per-Q1-Q10 disposition (one paragraph each + verdict tag + cite file:line).
Section C: NEW constitutional debt introduced by TB-7R (each entry: file:line + what + why + remediation).
Section D: Open observation review — does OBS-1 bounding stand? Does OBS-2 bounding stand? If either is wrong, severity classification.
Section E: Recommendation — ship-clear / ship-with-OBS-tightening / revise / VETO + rationale.

Be direct. Cite file:line. The materials below include: charter, authorization verdict, parent_tx verdict, two OBS files, smoke evidence README, audit-point-1 micro-audit (your prior verdict), L4 purity audit, the diff `git diff 9e74195..4470036`, and the new/heavily-changed source files post-state.

BRIEF_EOF

# Append all reference materials.
append_xref() {
  local label="$1"
  local path="$2"
  local fence="${3:-markdown}"
  local full="${ROOT}/${path}"
  if [[ ! -f "$full" ]]; then
    printf '\n\n---\n\n# XREF: %s — `%s` [missing]\n' "$label" "$path" >> "$TMP_PROMPT"
    return
  fi
  printf '\n\n---\n\n# XREF: %s — `%s`\n\n```%s\n' "$label" "$path" "$fence" >> "$TMP_PROMPT"
  cat "$full" >> "$TMP_PROMPT"
  printf '\n```\n' >> "$TMP_PROMPT"
}

# Charter + verdicts.
append_xref "TB-7R charter (the ship gate)" "handover/tracer_bullets/TB-7R_charter_2026-05-01.md"
append_xref "TB-7R authorization verdict 2026-05-01" "handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md"
append_xref "TB-7R parent_tx verdict 2026-05-02 (BINDING)" "handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md"

# Two OBS items folded in for visibility.
append_xref "OBS-1: post-TB-7R coverage denominator (architect-acknowledged follow-up)" "handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md"
append_xref "OBS-2: Art. III.4 prompt pollution (architect-silent; flag for future ruling)" "handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md"

# Prior audit-point-1 micro-audit (your own verdict).
append_xref "Codex TB-7R micro-audit (audit-point-1; YOUR own prior verdict)" "handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md"

# L4 purity audit.
append_xref "L4 purity audit (Deliverable A; zero violations)" "handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md"

# Smoke evidence top-level README.
append_xref "TB-7R smoke evidence README (Deliverable F)" "handover/evidence/tb_7r_smoke_2026-05-02/README.md"

# TRACE_MATRIX orphan registry (Claim 7 remediation).
append_xref "TRACE_MATRIX orphan registry (Claim 7 remediation)" "handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md"

# Three-node taxonomy (background).
append_xref "Three-node taxonomy decision record" "handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md"

# L4 / L4.E ledger separation (background).
append_xref "L4 / L4.E ledger separation decision record" "handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md"

# Heavily-changed source files (post-state).
append_xref "src/runtime/genesis_report.rs (NEW; Deliverable C)" "src/runtime/genesis_report.rs" "rust"
append_xref "src/runtime/proposal_telemetry.rs (extended)" "src/runtime/proposal_telemetry.rs" "rust"
append_xref "src/runtime/verification_result.rs (NEW; Deliverable A predicate evidence)" "src/runtime/verification_result.rs" "rust"
append_xref "src/runtime/chain_derived_run_facts.rs (extended; chain_oracle_verified)" "src/runtime/chain_derived_run_facts.rs" "rust"
append_xref "src/runtime/agent_audit_trail.rs (small change)" "src/runtime/agent_audit_trail.rs" "rust"
append_xref "src/runtime/mod.rs (delta)" "src/runtime/mod.rs" "rust"
append_xref "src/bottom_white/cas/store.rs (delta — CAS surface)" "src/bottom_white/cas/store.rs" "rust"
append_xref "src/bin/audit_dashboard.rs (NEW; Deliverable F dashboard)" "src/bin/audit_dashboard.rs" "rust"
append_xref "experiments/minif2f_v4/src/chaintape_mode_gate.rs (NEW; Deliverable B fail-closed)" "experiments/minif2f_v4/src/chaintape_mode_gate.rs" "rust"
append_xref "tests/tb_7r_parent_tx_conformance.rs (NEW; verdict 2026-05-02 §3 6-pack)" "tests/tb_7r_parent_tx_conformance.rs" "rust"
append_xref "tests/tb_6_verify_chaintape.rs (extended)" "tests/tb_6_verify_chaintape.rs" "rust"

# Diff (the delta auditors must verify).
{
  printf '\n\n---\n\n# XREF: full TB-7R diff `git diff 9e74195..4470036` (188 files; 11.9k +; 270 −)\n\n'
  printf '## (Note: large; truncated to first 250k chars to fit budget; auditor should request full diff if specific section needed)\n\n'
  printf '```diff\n'
  git -C "$ROOT" diff 9e74195..4470036 | head -c 250000
  printf '\n```\n'
} >> "$TMP_PROMPT"

# Commit log on TB-7R range.
{
  printf '\n\n---\n\n# XREF: TB-7R commit log (5 commits)\n\n'
  printf '```\n'
  git -C "$ROOT" log --pretty=format:'%h %s%n%n%b%n----' 9e74195..4470036
  printf '\n```\n'
} >> "$TMP_PROMPT"

# Closing instruction.
printf '\n\n---\n\nNow give your INDEPENDENT round-1 ship audit. Cite file:line. Be direct.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex tb-7r ship] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex TB-7R Ship Audit (audit-point-2; round 1)\n'
  printf '**Date**: 2026-05-02\n'
  printf '**Range**: `9e74195..4470036` (5 commits)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Workspace test count**: 712 / 0 / 150 (cargo test --workspace canonical)\n'
  printf '**Audit class**: Class 3 (auth-crypto-money) — full dual; Codex-impl + Gemini-arch.\n'
  printf '**Auditor**: Codex (implementation-paranoid)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex tb-7r ship] API returned in ${elapsed}s" >&2
echo "[codex tb-7r ship] saved: $OUT" >&2
