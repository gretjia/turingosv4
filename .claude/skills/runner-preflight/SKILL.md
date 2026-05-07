---
name: runner-preflight
description: Pre-action gate before any runner script that mutates handover/evidence/ or runs evaluation. Validates clean tree, fresh binary, evidence immutability, Class classification, FC-trace plan. Catches stale-binary smoke + retroactive evidence rewrite.
user_invocable: true
---

# /runner-preflight — Pre-Action Gate

## When to fire (mandatory)
- Before invoking ANY of:
  - `bash handover/tests/scripts/run_*.sh`
  - `bash scripts/run_*.sh` *(except `run_constitution_gates.sh` which is read-only CI)*
  - any python runner that writes to `handover/evidence/`
  - any TB ship-path action (smoke probe, batch, dispatch evidence run)
- After `git stash pop` and before resuming any runner: re-run stages 1-3.

## Stages

### 1. Working tree clean
```bash
git status --porcelain
```
- Output containing `M handover/evidence/` or `M handover/tracer_bullets/` → **STOP**. Past evidence is write-once; modification = drift (per `feedback_no_retroactive_evidence_rewrite`).
- `??` (untracked) lines under `handover/evidence/<NEW_TIMESTAMP>/` are normal — only flag if they contain pre-existing TB run dirs.
- If unsure: `git diff --name-only HEAD -- handover/` and check if any listed file existed before this session.

### 2. Binary freshness
```bash
# binary mtime
stat -c '%Y %n' target/release/<bin>
# newest src/ file mtime
find src -name '*.rs' -printf '%T@ %p\n' | sort -n | tail -1
# binary's commit
strings target/release/<bin> | grep -E '^[a-f0-9]{40}$' | head -1  # if commit hash embedded
git rev-parse HEAD
```
- Binary mtime MUST be ≥ newest `src/**/*.rs` mtime.
- If stale → `cargo build --release --bin <bin>` first; do NOT proceed.
- Common bins to check: `evaluator`, `kernel`, `harness`, any binary the runner script invokes.

### 3. Evidence dir immutability
```bash
git diff --stat HEAD -- handover/evidence/
git diff --stat HEAD -- handover/audits/  handover/directives/
```
- Any modification to existing dirs → STOP. Investigate which script wrote there.
- If runner is about to write to a dir that already exists in git → reject; demand new timestamp dir.

### 4. Class classification (when changing src/)
- Read `feedback_risk_class_audit` and `feedback_class4_cannot_hide_in_class3`.
- Class 4 file list (current): `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, `src/state/sequencer.rs` (admission), `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`.
- Class 4 → STEP_B parallel-branch required, NOT direct main edit (per `feedback_step_b_protocol`).
- Class 4 hidden in Class 3 commit → block.

### 5. FC-trace requirement (when fixing bugs / changing src/)
- Per `feedback_fc_first_problem_handling`: any src/ commit must trace to FC1/FC2/FC3 node.
- Commit msg trailer required: `FC-trace: FC?-INV?` or `FC-trace: orphan(<reason>)`.

### 6. Charter check (when starting new TB)
- Per `feedback_tb_phase_tag_required`: charter MUST have frontmatter `phase_id` + `roadmap_exit_criteria_addressed` + `kill_criteria_tested`.
- Missing → reject charter.

### 7. Audit round count (when dispatching new audit)
- Per `feedback_elon_mode_policy`: round-cap = 2.
- Round 3+ in same TB → require explicit user authorization + invoke `/harness-reflect` first to identify missing gate.

## Output format

```
=== RUNNER PREFLIGHT (<runner_name>) ===
1. Tree clean:       [PASS|FAIL: <files>]
2. Binary freshness: [PASS|FAIL: <bin> stale by <delta>; rebuild required]
3. Evidence imm:     [PASS|FAIL: <files modified outside session>]
4. Class:            [Class N] [STEP_B required: yes/no]
5. FC-trace:         [pending — fill before commit | not applicable]
6. Charter:          [N/A | PASS | missing: <fields>]
7. Audit rounds:     [round N — within cap | exceeds cap, retro required]
VERDICT: [GO | STOP]
Remediation (if STOP): <one-line action>
```

## On FAIL
- Print explicit remediation. Do NOT proceed to runner.
- If user overrides: require verbal acknowledgment **with reason** ("override preflight stage N: <reason>"). Log the override in commit msg trailer `PREFLIGHT_OVERRIDE: <reason>`.

## Linked rules
- `feedback_pre_runner_checklist` — when to invoke this skill (memory)
- `feedback_norm_needs_mechanism` — why this skill exists (meta-rule)
- `feedback_no_retroactive_evidence_rewrite` — stage 1 + 3 root norm
- `feedback_smoke_before_batch` — stage 2 + parent of this skill
- `feedback_class4_cannot_hide_in_class3` — stage 4
- `feedback_fc_first_problem_handling` — stage 5
- `feedback_tb_phase_tag_required` — stage 6
- `feedback_elon_mode_policy` — stage 7
