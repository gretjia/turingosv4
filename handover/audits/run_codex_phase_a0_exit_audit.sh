#!/usr/bin/env bash
# Codex Phase A0 exit audit — harness modernization gate.
# Independent of Gemini.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md"
TMP_PROMPT="$(mktemp /tmp/codex_a0_exit.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Phase A0 Exit Audit — Harness Modernization

**Role**: skeptical adversarial reviewer. Independent of Gemini. Decision rule: PASS → Phase A1-A8 engineering atoms can begin; CHALLENGE → fix in same cycle; VETO → A0 redesign.

**Mandate**: Phase A0 is the harness modernization that PRECEDES Phase A engineering work. Per the auto-research execution plan: "harness 不修复, Phase A 之后的 atoms 也会有 silent drift". A0 must resolve the harness gaps before atoms A1-A8 (engineering: swarm_N=1, AGENT_MODELS, decomposed metrics, budgets, FC tracing, SiliconFlow smoke, Phase A→B exit).

**A0 deliverables (4 commits)**:
- `2e7f75a` A0a: 4 new rules R-014/R-015/R-018/R-019 + judge.sh constitution-special-case + R-016 fc_trace_in_commit hook
- `d8950ee` A0b: tests/fc_alignment_conformance.rs witness battery (17 PASS + 9 ignored stubs)
- `2a65339` A0c: 5 new cases C-071..C-075 (constitution amendment process / Veto-AI scope / ArchitectAI commit authority / FC-first / DO-178C tool qualification)
- `e94e1b9` A0d: TRACE_MATRIX_v2 + Trust Root manifest 20 → 24 entries (rules/MANIFEST.sha256 + rules/engine.py + .claude/hooks/judge.sh + tests/fc_alignment_conformance.rs added)

**Constitutional context** (Art. V amendment 2026-04-25, commit c061450 — sudo-authorized):
- Art. V.1.1: sudo applies *only* to constitution.md
- Art. V.1.2: ArchitectAI has commit authority on non-constitution files (this is the authority under which A0a-d landed without sudo)
- Art. V.1.3: JudgeAI → Veto-AI; output domain {PASS, VETO} only

**Verification done** (by ArchitectAI):
- 4 smoke tests on judge.sh confirm: R-018 BLOCKS constitution.md edit / R-016 WARNS on git commit without FC-trace / silent pass on FC-traced commit / non-constitution .md still skipped
- cargo test --workspace: 204 passed (was 187, +17 fc_alignment_conformance) + 29 ignored (was 20, +9 ignored stubs)
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 24-entry manifest

## Specific questions

### Q1 — Rule completeness for B7+ surface
4 new rules cover: Trust Root manifest drift (R-014 warn), TRACE_MATRIX pub-symbol freshness (R-015 warn), constitution.md sudo gate (R-018 BLOCK), model-snapshot canonical labeling (R-019 warn). Plus R-016 fc_trace_in_commit (in judge.sh, not engine.py rule).

Vulnerabilities:
- (Q1.a) Are there OTHER B7+ surfaces still uncovered by rules? E.g., synthetic_short_circuit field ground-truth-feedback discipline (Q7.b round-2 lesson), AGENT_MODELS env var sanity, BUDGET_MODE env values, etc.?
- (Q1.b) R-014 fires on EVERY *.rs edit (file_glob "*.rs"). Is the constant warn noise acceptable, or should it scope-narrow (e.g., file_glob covering only the 12 .rs files in current Trust Root, plus a per-rule meta about which manifest entries the file may map to)?
- (Q1.c) R-018 BLOCK uses file_glob "constitution.md" + check.pattern ".*". It depends on judge.sh special-case BEFORE the *.md skip-list. Is the path-routing logic in judge.sh (lines 50-67) airtight, or could a creative path string (e.g., relative path with `./`, symlink) bypass?
- (Q1.d) R-016 fc_trace_in_commit greps the COMMAND for `FC-trace:`. If user uses `git commit -F /tmp/msg` instead of `-m "..."`, the grep misses the file content. Is this a known limitation worth fixing now or post-A0?

### Q2 — Conformance battery coverage and integrity
17 witnesses + 9 ignored stubs. Each witness imports an FC-anchored symbol and references it.

Vulnerabilities:
- (Q2.a) Are the 17 witnesses TRULY witnesses (i.e., would fail to compile if the symbol is removed/renamed) or are some only "type_name::<T>()" calls that wouldn't catch behavioral regressions?
- (Q2.b) The 9 ignored stubs each include `panic!()` body — they fire if `--ignored` flag is passed. Is this the right pattern, or should they be `#[ignore]` with `unimplemented!()` or empty body?
- (Q2.c) Cross-crate symbols (Lean4Oracle in minif2f_v4) need a SECOND fc_alignment_conformance.rs in `experiments/minif2f_v4/tests/`. A0b ignored this. Is that an acceptable deferral or should A0e cover it?
- (Q2.d) Binary-only symbols (run_swarm/run_oneshot in evaluator.rs) are ignored with rationale. Should A1-A8 atoms refactor evaluator to expose these as lib functions for test access?

### Q3 — Case-law sediment quality
5 cases C-071..C-075 cover the major architectural decisions of the 2026-04-25 session. Each follows the existing schema (incident / facts / ruling / precedent / cross-ref).

Vulnerabilities:
- (Q3.a) Are the cases TRUE precedents (binding for future similar incidents) or just session-summary docs? CLAUDE.md "Common Law" framing: "宪法高度压缩, 具体裁决查 cases/C-xxx.yaml". Do C-071..C-075 stand as standalone rulings without needing this session's chat history?
- (Q3.b) C-073 (ArchitectAI commit authority) declares scope "ALL files EXCEPT constitution.md" — including PREREG amendments, Trust Root manifest, governance instrumentation. Is this correctly aligned with Art. V.1.2 amendment text, or does it overreach?
- (Q3.c) C-074 (FC-first) overlaps significantly with memory feedback_fc_first_problem_handling.md. Is the duplication acceptable (memory = AI's runtime guide; case = constitutional precedent for future reference)?
- (Q3.d) Cross-references between cases (C-071 ↔ C-073, etc.) form a DAG. Any circular reasoning hazard?

### Q4 — Trust Root manifest expansion 20 → 24
4 new entries: rules/MANIFEST.sha256 + rules/engine.py + .claude/hooks/judge.sh + tests/fc_alignment_conformance.rs.

Vulnerabilities:
- (Q4.a) Is the inclusion of rules/engine.py sufficient, or should rules/SCHEMA.yaml also be in Trust Root (defines rule format)?
- (Q4.b) `.claude/hooks/judge.sh` is included; what about the other hooks (`build-check.sh` PostToolUse + `session-end.sh` Stop hook)? They run on every Edit/Write/Stop respectively. If they're tampered, is the gate weaker?
- (Q4.c) `tests/fc_alignment_conformance.rs` is included. Should ALL tests be in Trust Root (they're the witness battery for behavioral correctness)? Or only the FC-conformance one?
- (Q4.d) Trust Root self-reference: TRACE_MATRIX_v?.md docs themselves are NOT in Trust Root. v2 § 5 records this as future-deferred. Is this acceptable for now or a real gap?

### Q5 — Constitutional alignment (meta-check)
Per case C-073, A0a-d landed under ArchitectAI commit authority (no sudo). Per case C-072, Veto-AI proxy is the dual-audit pipeline (this audit).

Vulnerabilities:
- (Q5.a) Did A0a-d touch constitution.md? Trace each commit's file changes — verify no constitution.md modification.
- (Q5.b) Did each commit message contain `FC-trace:`? Per R-016, missing FC-trace = WARN. Verify all 4 A0 commits have FC-trace.
- (Q5.c) Phase A0 introduced harness that ENFORCES governance going forward. Is there a regression test that asserts A0 itself can't be silently rolled back? (E.g., if attacker reverts judge.sh change, do tests fail?)

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase A engineering atoms (A1-A8) / FIX-THEN-PROCEED / REDESIGN A0

PASS if: A0 closes the harness-gap claim AND no new P0 defect.

---

# Appended source files

BRIEF_EOF

# Source files
printf '\n## rules/active/R-014_trust_root_manifest_drift.yaml (NEW)\n\n```yaml\n' >> "$TMP_PROMPT"
cat "${ROOT}/rules/active/R-014_trust_root_manifest_drift.yaml" >> "$TMP_PROMPT"

printf '\n```\n\n## rules/active/R-015_trace_matrix_pub_symbol.yaml (NEW)\n\n```yaml\n' >> "$TMP_PROMPT"
cat "${ROOT}/rules/active/R-015_trace_matrix_pub_symbol.yaml" >> "$TMP_PROMPT"

printf '\n```\n\n## rules/active/R-018_constitution_amendment_sudo.yaml (NEW)\n\n```yaml\n' >> "$TMP_PROMPT"
cat "${ROOT}/rules/active/R-018_constitution_amendment_sudo.yaml" >> "$TMP_PROMPT"

printf '\n```\n\n## rules/active/R-019_model_snapshot_canonical.yaml (NEW)\n\n```yaml\n' >> "$TMP_PROMPT"
cat "${ROOT}/rules/active/R-019_model_snapshot_canonical.yaml" >> "$TMP_PROMPT"

printf '\n```\n\n## .claude/hooks/judge.sh (constitution.md special-case + R-016)\n\n```bash\n' >> "$TMP_PROMPT"
cat "${ROOT}/.claude/hooks/judge.sh" >> "$TMP_PROMPT"

printf '\n```\n\n## tests/fc_alignment_conformance.rs (NEW)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/tests/fc_alignment_conformance.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## handover/alignment/TRACE_MATRIX_v2_2026-04-25.md (NEW)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/alignment/TRACE_MATRIX_v2_2026-04-25.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n## genesis_payload.toml (24 entries, expanded for A0)\n\n```toml\n' >> "$TMP_PROMPT"
cat "${ROOT}/genesis_payload.toml" >> "$TMP_PROMPT"

printf '\n```\n\n## cases/C-071..C-075 (NEW — 5 case yaml files)\n\n' >> "$TMP_PROMPT"
for case_id in 071 072 073 074 075; do
    printf '\n### cases/C-%s_*.yaml\n\n```yaml\n' "$case_id" >> "$TMP_PROMPT"
    cat "${ROOT}"/cases/C-${case_id}_*.yaml >> "$TMP_PROMPT"
    printf '\n```\n' >> "$TMP_PROMPT"
done

printf '\n---\n\nNow give your INDEPENDENT Phase A0 exit audit. Cite §/file:line for every finding.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex a0 exit] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex Phase A0 Exit Audit (Harness Modernization)\n'
  printf '**Date**: 2026-04-25\n'
  printf '**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9\n'
  printf '**Test baseline**: 204 PASS + 29 ignored\n'
  printf '**Trust Root**: 24-entry manifest verifies clean\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex a0 exit] done in ${elapsed}s, saved: $OUT" >&2
