#!/usr/bin/env bash
# Codex round-4 audit on CO1.7-extra v1.2 (post 4 mechanical fixes B1-B4).
# Tight verification: did v1.2 close round-3 B1-B4 cleanly without introducing regressions?
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_extra_codex_round4.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.7-extra v1.2 (Round 4; post round-3 mechanical fixes)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-4 (parallel).

**Mandate**: round 4 dual external audit on CO1.7-extra v1.2 — applied 4 mechanical patches (B1-B4) per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md`. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Round-3 disposition**: Codex r3 CHALLENGE/High (3 blockers + 1 non-blocking); Gemini r3 PASS/High ("model of post-audit closure"). Conservative-merged CHALLENGE; v1.2 fixes applied.

**Round-4 expectation**: PASS unless v1.2 introduced new defects or missed a closure. Patches are mechanical (1-line code/text fixes); no architectural surface change since v1.1.

## What changed since round-3 (v1.1 → v1.2)

- **B1**: § 1.1 stage-9 snippet `&**writer_w` → `&*writer_w` (compile error fix; `RwLockWriteGuard<dyn LedgerWriter>` cannot double-deref)
- **B2**: § 1.1 helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (integration tests need `pub`); FC-trace doc-comment added
- **B3**: removed 2 stale Kernel references — preface line 14 single-sentence summary + § 6 pre-implementation gate file list
- **B4** (non-blocking): § 2.1 `#[serde(skip)]` made conditional with explicit comment; § 7 LoC vs patch log synced (200-280 → 210-300)

## Round 4 audit questions (tight)

**Q1. B1 closure**: spec § 1.1 stage-9 snippet uses `&*writer_w` (single deref). Verify:
- Compiles given `writer_w: RwLockWriteGuard<dyn LedgerWriter>` (per src/state/sequencer.rs:201, :363-368)
- Produces `&dyn LedgerWriter` matching `advance_head_t` signature `&dyn LedgerWriter`
- No residual `&**writer_w` anywhere in spec

**Q2. B2 closure**: spec § 1.1 declares `pub fn advance_head_t` (was `pub(crate)`). Verify:
- Integration test at § 3.3 (`turingosv4::state::sequencer::advance_head_t`) can now access it
- Doc-comment includes FC-trace `/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4)` per CLAUDE.md "Alignment Standard"
- The `pub` keyword change is the ONLY visibility change (no over-exposure)

**Q3. B3 closure**: stale Kernel references removed. Verify:
- Preface line 14 single-sentence summary now says "single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED)" — NO "combined" or "TuringBus + Kernel"
- § 6 pre-implementation gate file list does NOT include `src/kernel.rs`; explicitly states "Kernel UNTOUCHED"
- Any other residual Kernel references in v1.2 that should be removed?

**Q4. B4 closure**: non-blocking inconsistencies fixed. Verify:
- § 2.1 `#[serde(skip)]` now conditional with explicit comment about TuringBus serde derive state
- § 7 LoC says `~210-300`; patch log says `~210-300` (synced). Or is there still divergence?

**Q5. New defects in v1.2**: any new issues introduced by these 4 patches?
- Internal contradictions between sections?
- New compile blockers in code snippets?
- Stale references to round-3 / B-numbered items that should be cleaned up?

## Output format

# Codex CO1.7-extra Round-4 Audit
## Q1 B1 closure (stage-9 deref)
## Q2 B2 closure (advance_head_t pub)
## Q3 B3 closure (stale Kernel refs)
## Q4 B4 closure (serde-skip conditional + LoC sync)
## Q5 New defects in v1.2
## **VERDICT**: PASS / CHALLENGE / VETO
## Top issues (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.7-extra v1.2 (target of audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: round-3 merged verdict (drove the v1.2 patches)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped src/state/sequencer.rs (B1 + B2 target)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/sequencer.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bus.rs (B4 serde context — TuringBus has no serde derives)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bus.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-4 audit. Round-4 expects PASS unless v1.2 introduced regressions or missed a closure.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7-extra r4] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7-extra Round-4 Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: spec v1.2 (post round-3 mechanical fixes)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7-extra r4] API returned in ${elapsed}s" >&2
echo "[codex co1.7-extra r4] saved: $OUT" >&2
