#!/usr/bin/env bash
# Codex round-1 audit on CO1.13 v1 (TRACE_MATRIX impl + R-022 hook; factory atom).
# Implementer-paranoid angle: does R-022 enforcement actually work? Are the 3
# sub-atoms a coherent decomposition? Hidden P0 defects in the YAML-script
# delegation pattern? Is the 5-line context window correct?
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/co1_13_codex_round1.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.13 v1 TRACE_MATRIX Impl + R-022 Hook (Round 1; greenfield + Elon-mode)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (parallel).

**Mandate**: round-1 dual external audit on CO1.13 v1 — a 3-sub-atom factory bundle that lands TRACE_MATRIX_v3 doc completion + R-022 commit-time hook + reverse-map § F population. Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. **Per NEW Elon-mode policy (this v1 is the FIRST application)**: round cap = 2; ship-with-OBS if r2 still CHALLENGE.

**Background**:
- CO1.7-extra (final L4 atom; STEP_B closed) shipped 2026-04-29 (commit `4a978f0`)
- CO1.8 v1 spec drafted (commit `6cc5cc9`) but its round-1 audit was DEFERRED in favor of CO1.13 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms)
- CO1.13 is the canonical "TRACE_MATRIX impl + R-022 hook" atom per SPRINT_DEPENDENCY_GRAPH line 129 (3 atoms)
- TRACE_MATRIX_v3 is the existing 324-line doc; CO1.13.1 closes its empty/stub fields
- 15 active YAML rules already exist; CO1.13.2 lands R-022 as the 16th (within 30-rule cap)
- ~75% of src/ pub symbols lack TRACE_MATRIX backlinks (87/354 = 24.6% baseline coverage)

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md` (~283 lines)
2. **Frozen primary references**:
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` — 324 lines; the doc CO1.13.1 closes
   - `docs/rules.md` — rule engine architecture
   - `rules/SCHEMA.yaml` — YAML rule schema
   - `rules/active/R-015_trace_matrix_pub_symbol.yaml` — existing pre-edit warn variant; R-022 is its commit-time block sibling
   - `rules/engine.py` (145 LoC) — rule evaluator (CO1.13.2 extends with `external_script` check type)
   - `.claude/hooks/judge.sh` — existing pre-edit hook (R-022 lands separate `.git/hooks/pre-commit` shim)

## Round-1 audit questions (7)

**Q1. R-022 enforcement boundary correctness**:
- Spec § 2.1 says: "for each NEW pub line, reads 5 lines preceding... greps for /// TRACE_MATRIX ... if not found, greps TRACE_MATRIX_v3.md § 3... else BLOCK"
- Edge case: pub symbol inside a `#[cfg(test)]` mod tests block — should R-022 apply, or are test pub items exempt?
- Edge case: pub symbol re-exported from a child module via `pub use` — already-traced child symbol shouldn't double-require trace at re-export site
- Edge case: macro-generated pub items — backlink can't be added (no source location)
- Spec § 2.2 escape hatch `// R-022-skip: <reason>` — is the reason captured in stats / logged for quarterly audit, or just permitted silently?

**Q2. The 5 open questions** (spec § 5):
- **Q1 forward-only vs edit-also**: spec § 0.5 #2 says "does NOT enforce backlinks on legacy pub symbols". But what about pub symbols that EXISTED but are MODIFIED (e.g., signature change adding a parameter)? Author lean: forward-only (NEW additions). Is this the right call, or does signature change deserve re-tracing?
- **Q2 5-line window**: is this empirically validated? What % of current backlinks are within 5 lines vs further away? If the convention isn't 95%+ within 5 lines, the heuristic is wrong.
- **Q3 escape hatch rigor**: `// R-022-skip: <reason>` permissive in v1. Should it require `cases/Cxxx` reference now, or accept abuse risk for v1?
- **Q4 reverse-map source of truth**: doc-comments authoritative (spec lean) — but what if a doc-comment is removed in a later edit? Does R-022 catch the *removal* of an existing backlink, or only missing on creation?
- **Q5 R-015 retention**: defense in depth (spec lean). Or does R-015 (warn) become noise if R-022 (block) is the real gate?

**Q3. The 3-sub-atom decomposition**:
- CO1.13.1 doc completion: ~150 LoC docs delta. Is this an audit gate or just a "document hygiene" task? If hygiene, why is it in the audited spec scope?
- CO1.13.2 R-022 hook: ~165 LoC across YAML + Python + shell hook. Does this belong in `rules/active/` (with the other rules) or in a dedicated `hooks/` dir?
- CO1.13.3 reverse-map: ~100 LoC bash. Why bash and not Python (since check_trace_matrix.py is already Python)? Code hygiene risk: 2 languages for similar logic.

**Q4. The `external_script` YAML check.type extension**:
- Spec § 1.2 says: "delegates to script via new `check.type: external_script` extension". This is a new extension to `rules/SCHEMA.yaml` not currently supported.
- Is this extension architecturally clean? Alternative: keep `check.type: grep` and have engine.py detect `R-022` rule_id specifically + dispatch to script. (Avoids schema change.)
- Does the extension need to update SCHEMA.yaml documentation as part of CO1.13.2? Spec doesn't mention schema.yaml update.
- Backward compat: what happens to existing rules if `external_script` type is added — do they still load cleanly?

**Q5. Test plan adequacy** (spec § 3, 5 tests):
- 3.1 `r_022_blocks_missing_backlink.rs`: stages a fake pub symbol; runs check_trace_matrix.py; asserts exit 2. **Is this a Rust test or a shell test?** Rust tests can't easily invoke a Python script + git hooks. Should be `tests/r_022_*.sh` integration test instead.
- 3.4 `trace_matrix_reverse_map_idempotent.rs`: idempotency check. But the script is bash (CO1.13.3). Mismatch: Rust testing a bash script. Better to make it `tests/trace_matrix_reverse_map_idempotent.sh`.
- 3.5 `trace_matrix_v3_doc_coverage.rs`: tests that every Constitution Article + WP § has a populated Class column. **What does this test actually do — read the markdown file and parse rows?** Brittle to format changes.
- Missing tests: no test for the escape hatch path (§ 2.2). No test for the legacy-symbol-modified case (Q1 above).

**Q6. Forward sustainability**:
- Spec § 0.4 #1-#3 lists scaffold scripts as "non-constitutional devtools, no audit". But these scripts (scaffold_co_spec.sh / scaffold_audit_launcher.sh / rehash_trust_root.sh) directly affect future spec-drafting + audit-launching cycles. They're load-bearing for the Elon-mode hypothesis ("cycle time 14d → 2d"). Should they be in the audited scope to ensure quality?
- If they're not audited and turn out to be buggy, every subsequent atom suffers. Risk vs cost trade-off.

**Q7. Strategic risks not yet flagged**:
- Per memory `project_thesis`: "Frozen 5-step compile loop". Does CO1.13 advance the loop, or pure infrastructure? If infrastructure, what's its 11-atomic-claim audit score?
- Per Elon-mode: this is the FIRST atom under the 2-round audit cap. If r1 is CHALLENGE/CHALLENGE and r2 is still CHALLENGE, does ship-with-OBS actually capture the unresolved issues, or does it sweep them under the rug?
- Cycle time: spec says "2-day target". If the 2-day target is missed (e.g., takes 5 days due to round-2 patches), does the Elon-mode policy auto-relax or does the user need to reauthorize?

## Verdict format

Section A: Verdict (PASS/CHALLENGE/VETO) with conviction (LOW/MED/HIGH).
Section B: P0 blockers (must-fix before round-2).
Section C: Open questions raised (architectural).
Section D: Suggested patches (specific spec line/section edits).
Section E: Forward-sustainability notes.

Be concrete. Cite spec § + line where possible.

BRIEF_EOF

# Append spec + key reference files
printf '\n\n---\n\n# XREF: spec — handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: TRACE_MATRIX_v3 (existing doc; CO1.13.1 closes stub fields)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/alignment/TRACE_MATRIX_v3_2026-04-27.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: docs/rules.md (rule engine architecture)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/docs/rules.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: rules/SCHEMA.yaml\n\n```yaml\n' >> "$TMP_PROMPT"
cat "${ROOT}/rules/SCHEMA.yaml" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: rules/active/R-015_trace_matrix_pub_symbol.yaml (existing pre-edit warn)\n\n```yaml\n' >> "$TMP_PROMPT"
cat "${ROOT}/rules/active/R-015_trace_matrix_pub_symbol.yaml" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: rules/engine.py (rule evaluator; ~145 LoC)\n\n```python\n' >> "$TMP_PROMPT"
cat "${ROOT}/rules/engine.py" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\nNow give your INDEPENDENT round-1 audit. Cite spec § + line where possible.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.13 r1] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.13 Round-1 Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: spec v1 (greenfield; first Elon-mode 2-round-cap atom)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.13 r1] API returned in ${elapsed}s" >&2
echo "[codex co1.13 r1] saved: $OUT" >&2
