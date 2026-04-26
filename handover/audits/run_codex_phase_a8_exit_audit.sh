#!/usr/bin/env bash
# Codex Phase A → B exit audit — covers A0–A7 deliverables.
# Independent of Gemini.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
# A8_AUDIT_ROUND is REQUIRED (A8e10 fix O1, Codex R9#1). Earlier the
# script defaulted to R2, which silently overwrote the round-2
# transcript on unattended re-runs. Now: explicit env var or fail
# fast. Round-1 (no suffix) lives at the un-suffixed transcript;
# round 2+ uses _R2/_R3/... per chronology.
if [ -z "${A8_AUDIT_ROUND:-}" ]; then
    echo "[run_codex_a8_exit] error: A8_AUDIT_ROUND env var is required" >&2
    echo "    usage: A8_AUDIT_ROUND=R<n> bash $0" >&2
    echo "    sets the suffix for handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_<round>.md" >&2
    exit 2
fi
ROUND="$A8_AUDIT_ROUND"
OUT="${ROOT}/handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_${ROUND}.md"
if [ -f "$OUT" ]; then
    echo "[run_codex_a8_exit] error: $OUT already exists; refusing to overwrite" >&2
    echo "    (prior audit transcripts are append-only governance artifacts;" >&2
    echo "    delete the file explicitly if you really intend to re-run round $ROUND)" >&2
    exit 2
fi
TMP_PROMPT="$(mktemp /tmp/codex_a8_exit.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

PACKET="${ROOT}/handover/audits/A8_EXIT_PACKET_2026-04-26.md"
HISTORY="${ROOT}/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md"

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Phase A → B Exit Audit (PPUT-CCL arc)

**Role**: skeptical adversarial reviewer. Independent of Gemini. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: Phase A is pre-flight (days 1–3 of the 30-day arc). 8 atoms (A0a–e + A1–A7) must be auditable as a unit before Phase B (kernel instrumentation + PPUT accounting) is authorized to start. PREREG_PPUT_CCL_2026-04-26.md (round-4 PASS/PASS, frozen) + PREREG_AMENDMENT_p0_defer_2026-04-25.md (Trust Root entry 25) are the contracts you're auditing against.

The packet below is self-contained. Read it as a standalone document — your conclusions go to ArchitectAI, who will iterate on CHALLENGE items in the same audit cycle. The Phase A0 exit audit (CHALLENGE/CHALLENGE → 7 fixes) is the precedent for how rigorous to be.

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase B / FIX-THEN-PROCEED / REDESIGN

Cite §/file:line for every finding. Be specific about which atom and which line.

---

BRIEF_EOF

# Append the packet itself + the audit history (append-only chronology
# companion document — reviewers needing round-N closure context find it
# there; the packet is current-state only post-A8e7 structural rewrite).
cat "$PACKET" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Audit history (append-only chronology)\n\n' >> "$TMP_PROMPT"
cat "$HISTORY" >> "$TMP_PROMPT"

# Append source files referenced by the packet so the auditor can verify
# without round-tripping to the repo. Order: most-load-bearing first.
printf '\n\n---\n\n# Appended source files\n\n' >> "$TMP_PROMPT"

for f in \
    handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md \
    experiments/minif2f_v4/src/agent_models.rs \
    experiments/minif2f_v4/src/budget_regime.rs \
    experiments/minif2f_v4/src/fc_trace.rs \
    experiments/minif2f_v4/src/run_id.rs \
    experiments/minif2f_v4/src/jsonl_schema.rs \
    experiments/minif2f_v4/src/bin/evaluator.rs \
    src/drivers/llm_proxy.py \
    scripts/smoke_siliconflow.sh \
    scripts/_smoke_siliconflow.py \
    scripts/test_llm_proxy.py \
    experiments/minif2f_v4/tests/fc_trace_smoke.rs \
    experiments/minif2f_v4/tests/trust_root_immutability.rs \
    experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs \
    experiments/minif2f_v4/examples/fc_trace_emit_one.rs \
    handover/alignment/TRACE_MATRIX_v2_2026-04-25.md \
    genesis_payload.toml ; do
    case "$f" in
        *.rs) lang="rust" ;;
        *.py) lang="python" ;;
        *.sh) lang="bash" ;;
        *.toml) lang="toml" ;;
        *.yaml|*.yml) lang="yaml" ;;
        *) lang="" ;;
    esac
    printf '\n## %s\n\n```%s\n' "$f" "$lang" >> "$TMP_PROMPT"
    cat "${ROOT}/${f}" >> "$TMP_PROMPT"
    printf '\n```\n' >> "$TMP_PROMPT"
done

printf '\n---\n\nGive your INDEPENDENT Phase A → B exit audit. Cite §/file:line for every finding.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex a8 exit] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex Phase A → B Exit Audit (PPUT-CCL arc)\n'
  printf '**Round**: %s\n' "$ROUND"
  printf '**Date**: 2026-04-26\n'
  printf '**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` for full chronology including atom commit chain + per-round verdicts/fixes.\n'
  printf '**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); 16/16 PASS (Python proxy tests)\n'
  printf '**Trust Root**: 35-entry manifest verifies clean\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex a8 exit] done in ${elapsed}s, saved: $OUT" >&2
