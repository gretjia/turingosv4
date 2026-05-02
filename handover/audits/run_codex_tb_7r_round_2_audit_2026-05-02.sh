#!/usr/bin/env bash
# Codex TB-7R Ship Audit ROUND 2 — verifies remediation closes round-1 VETO.
# Independent re-check, NOT a re-do of round 1.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_7R_SHIP_AUDIT_R2_2026-05-02.md"
TMP_PROMPT="$(mktemp /tmp/tb7r_codex_r2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-7R Ship Audit — ROUND 2 (post-remediation re-check)

**Role**: same skeptical adversarial implementer-reviewer as round 1.

**Mandate**: round-2 ship audit per `feedback_elon_mode_policy` after determinate-best surgical remediation of round-1 VETO. Confirm whether the four explicit round-1 findings are closed. NOT a re-do of round 1 — focus on the deltas.

You returned **VETO / HIGH conviction** in round 1 (`handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md`). The remediation since round 1 is uncommitted (working tree). HEAD is still `4470036`.

## Round-1 findings + remediation summary

| Round-1 finding | Severity | Remediation |
|---|---|---|
| #1 Evidence packaging: `runtime_repo/.git/` + `cas/.git/objects/` not committed; only sidecar JSON. CasStore::get fails on committed-only state. README claims `.git` present. | VETO basis | Each of 10 evidence dirs now ships `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` (compressed `.git/` stores; ~12KB each, total 892KB across 10 runs). README §0 + §4 + §5.1 updated to reflect new packaging + add reproduction recipe. Round-trip verified: extract tar.gz pair into a fresh dir + `verify_chaintape` produces structurally identical replay_report.json to the committed one. |
| #2 No `replay_report.json` per run | constitutional debt #3 in round 1 | Each evidence dir now ships `replay_report.json` (literal `verify_chaintape` output: all 9 boolean indicators true; `l4_entries` + `l4e_entries` counts; `final_state_root_hex` + `final_ledger_root_hex` + `head_commit_oid_hex`). |
| #3 PartialOk → Complete proof-prefix dependency: accepted L4 WorkTx `proof_artifact_cid` resolves to `tactic` only, but verify_partial uses `tape_chain + tactic`. Replay can't reconstruct full proof. | CHALLENGE (independent of VETO) | OBS-1 (`OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`) §2.1 added with sharper framing per your Q2 quote. §4 hardening recommendation extended with explicit "self-contained proof artifact" requirement (§4.3). Deferred to TB-8+ per architect verdict A1=B′ (per-tactic decomposition is TB-8+ scope). NOT touched in code. |
| #4 OBS-2 premise stale: `acc.record_tool_stdout` only increments token cost via `cost_aggregator.rs:57`; raw Lean text doesn't flow to prompt. | (your Q10 PASS w/ correction) | OBS-2 (`OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`) §0 added: explicit "premise empirically unfounded per Codex round-1 Q10" correction. Closed-as-empirically-unfounded. |

## What changed since round 1 (working tree, uncommitted)

```text
M  handover/evidence/tb_7r_smoke_2026-05-02/README.md      — §0 packaging update + §4 artifact list rewrite + §5.1 reproduce recipe
M  handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md  — §2.1 + §4.3 added
M  handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md  — §0 closure-as-stale-premise added
A  handover/evidence/tb_7r_smoke_2026-05-02/<10 runs>/runtime_repo.dotgit.tar.gz
A  handover/evidence/tb_7r_smoke_2026-05-02/<10 runs>/cas.dotgit.tar.gz
A  handover/evidence/tb_7r_smoke_2026-05-02/<10 runs>/replay_report.json
```

No code changes. `cargo test --workspace` still 712/0/150.

## Round-2 audit questions (4; targeted re-check of round-1 VETO basis)

**RQ1. Packaging completeness**: pick 2 random evidence dirs (e.g. `single_n1_mathd_algebra_171` and `full_5_problems_n1/run_4_aime_1997_p9` — pick 1 SOLVED + 1 UNSOLVED). For each:
- Confirm `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` exist and are non-empty.
- Extract them into a fresh tempdir (along with the sidecar JSONs as in §5.1).
- Run `target/debug/verify_chaintape --repo $WORK/runtime_repo --cas $WORK/cas --out $WORK/r.json`.
- Confirm the resulting `r.json` has all 9 boolean indicators true.
- Compare to the committed `replay_report.json` (modulo `run_id` + `epoch` which are runtime-tagged).

If both runs round-trip cleanly → RQ1 closed. If any indicator flips false or any tar.gz is corrupted → RQ1 fails.

**RQ2. Round-1 finding #1 closure** (clause 4 + ship cond #5): given RQ1 outcome, can a third-party auditor with only this repo + a built `target/debug/verify_chaintape` independently reproduce the chain_oracle_verified claim for any of the 10 runs? This is the strict reading of acceptance clause 4. If yes → finding #1 closed. If no → identify the residual gap.

**RQ3. CID resolution from committed state** (was VETO sub-finding on Q1): pick one accepted L4 Work entry from any solved run. From the extracted tempdir, demonstrate that the WorkTx's `proposal_cid` (read from CAS sidecar `.turingos_cas_index.jsonl`) resolves through `CasStore::get(cid)` to a non-empty payload. Then walk `proposal_telemetry.verification_result_cid` → `VerificationResult.proof_artifact_cid` chain, confirming each CID resolves. (One run is sufficient — you found 1 in-scope L4 Work in the L4 purity audit; that pattern repeats.)

**RQ4. OBS framing tightness** (round-1 finding #3 + #4):
- Read OBS-1 §2.1 + §4.3. Does the new §2.1.a ("PartialOk → Complete proof-prefix dependency") accurately capture what you found in round-1 Q2? Does §4.3 ("self-contained proof artifact") propose a sufficient post-TB-7R fix?
- Read OBS-2 §0. Is the closure-as-empirically-unfounded language correct given your round-1 Q10 finding? Or does it overstate / understate the conclusion?

## Verdict format (round 2)

Section A: Round-2 verdict (PASS / CHALLENGE-but-clearable / VETO).
- PASS = round-1 VETO is closed; ship may proceed.
- CHALLENGE-but-clearable = remaining items are non-VETO and have clear written remediations; ship may proceed with explicit OBS carry-forward.
- VETO = round-1 VETO not closed (or new VETO surfaced by remediation).

Section B: Per-RQ1-RQ4 disposition (one paragraph each + verdict tag + cite file:line / commit / shell-result).

Section C: Round-1 → round-2 finding closure table (4 rows: #1, #2, #3, #4).

Section D: Recommendation. Specifically:
- If PASS → state explicitly that round-1 VETO is closed.
- If CHALLENGE-but-clearable → list which OBS need final-form text changes before ship.
- If VETO → state what additional remediation is required.

The remediation is uncommitted in working tree. You can run `git status`, `git diff`, and any reproduction commands. Sandbox is `danger-full-access`.

BRIEF_EOF

# Append diff of remediation since round-1 audit.
{
  printf '\n\n---\n\n# XREF: working tree status (post-remediation)\n\n```\n'
  git -C "$ROOT" status --short
  printf '\n```\n'
} >> "$TMP_PROMPT"

# Append README diff.
{
  printf '\n\n---\n\n# XREF: README diff (handover/evidence/tb_7r_smoke_2026-05-02/README.md)\n\n```diff\n'
  git -C "$ROOT" diff -- handover/evidence/tb_7r_smoke_2026-05-02/README.md
  printf '\n```\n'
} >> "$TMP_PROMPT"

# Append OBS-1 diff.
{
  printf '\n\n---\n\n# XREF: OBS-1 diff (OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md)\n\n```diff\n'
  git -C "$ROOT" diff -- handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md
  printf '\n```\n'
} >> "$TMP_PROMPT"

# Append OBS-2 diff.
{
  printf '\n\n---\n\n# XREF: OBS-2 diff (OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md)\n\n```diff\n'
  git -C "$ROOT" diff -- handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md
  printf '\n```\n'
} >> "$TMP_PROMPT"

# Append round-1 audit (so round-2 can cross-reference verdicts).
{
  printf '\n\n---\n\n# XREF: round-1 audit (your prior verdict)\n\n'
  cat "${ROOT}/handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md"
} >> "$TMP_PROMPT"

# List committed-tree state of evidence dir.
{
  printf '\n\n---\n\n# XREF: ls -la of one evidence dir (single_n1_mathd_algebra_171)\n\n```\n'
  ls -la "${ROOT}/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/" 2>&1
  printf '\n```\n'
  printf '\n\n# XREF: replay_report.json from single_n1\n\n```json\n'
  cat "${ROOT}/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json" 2>&1
  printf '\n```\n'
} >> "$TMP_PROMPT"

printf '\n\n---\n\nNow give your INDEPENDENT round-2 audit. Be direct. Cite file:line / shell-result. Verdict: PASS / CHALLENGE-but-clearable / VETO.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex tb-7r r2] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex TB-7R Ship Audit — Round 2 (post-remediation re-check)\n'
  printf '**Date**: 2026-05-02\n'
  printf '**Range**: `9e74195..4470036` + working-tree remediation\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Workspace test count**: 712 / 0 / 150 (unchanged from round 1)\n'
  printf '**Audit class**: Class 3 (auth-crypto-money) — round-2 per `feedback_elon_mode_policy` round-cap\n'
  printf '**Auditor**: Codex (implementation-paranoid; same as round 1)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex tb-7r r2] API returned in ${elapsed}s" >&2
echo "[codex tb-7r r2] saved: $OUT" >&2
