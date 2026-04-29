#!/usr/bin/env bash
# Codex round-1 audit on CO1.8 v1 (L5 Materializer + Agent Read View; greenfield).
# Implementer-paranoid angle: are the 5 open questions exhaustive? Are the 8
# sub-atoms a valid decomposition? Hidden P0 defects in the apply() contract?
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/co1_8_codex_round1.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.8 v1 L5 Materializer (Round 1; greenfield)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (parallel).

**Mandate**: round-1 dual external audit on CO1.8 v1 — a GREENFIELD spec for the L5 Materialized State + Agent Read View atom. Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Background**: CO1.7-extra (final L4 atom; STEP_B closed at `4a978f0`) just shipped. CO1.8 is the determinate next atom per LATEST.md "Wave 6 #2" framing + SPRINT_DEPENDENCY_GRAPH line 109 (Materialized State, 8 atoms). This is a greenfield draft from primary sources; no prior rounds.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md` (~300 lines)
2. **Frozen prior specs (consumed surfaces)**:
   - CO1.1.4-pre1 v1.2.2 — 7-variant TypedTx ABI (`src/state/typed_tx.rs`)
   - CO1.7 v1.2 + CO1.7-impl bundle — `LedgerEntry` + `Sequencer::apply_one` 9-stage
   - CO1.7-extra v1.2.2 — `q.head_t` post-commit binding + `LedgerWriter::head_commit_oid_hex`
   - STATE_TRANSITION_SPEC v1.4 — 7 invocation sites of `materializer::apply` (lines 399/466/560/624/700/758/852)
   - WP § 5.L5 (lines 392-413) — primary source for L5 module structure
3. **Frozen Q_t types**: `src/state/q_state.rs` (Hash sha256-32 + state_root_t field + QState)
4. **L3 CAS surface (snapshot storage)**: `src/bottom_white/cas/store.rs`

## Round 1 audit questions (7)

**Q1. Greenfield scope soundness**: spec § 0.1 declares CO1.8 "the unique unblocked next L-layer atom" post-CO1.7-extra. Verify:
- Is this true? Sprint graph line 109 shows `[CO1.8] blockedBy: CO1.7` — CO1.7 family is closed, but does CO1.8 also (transitively) need CO1.7.5 transition bodies to exist to be testable?
- Spec § 0.4 #1 explicitly defers "live wiring with Sequencer stage 4-7" to a future atom (gated on CO1.7.5). Is the v1 atom (pure-function materializer + tests against fixtures) genuinely standalone, or does the test plan (§ 4) hide a transition-body dependency?
- Is "ship the materializer as a standalone library" a valid Anti-Oreo three-layer decomposition, or does it create a dead-code period (materializer compiled but unused) that violates the "no half-finished implementation" CLAUDE.md "Doing tasks" guidance?

**Q2. The 5 open questions** (spec § 5):
- **Q1 state_root semantics** (sha256-of-snapshot vs literal git-tree object id): is the author's lean architecturally sound? q_state.rs:27 says "generic 32-byte hash (sha256)". STATE spec line 78 says "git tree root in Path B". These are reconcilable IFF state_root = sha256(serialize(snapshot)) AND the snapshot bytes are stored as a git blob whose tree-root is computed separately. Is this the right reading? Are there other readings?
- **Q2 PriorRootNotFound failure mode**: v1 ships failure semantics; lazy reconstruction deferred to CO1.8-extra. Will this break Sequencer apply_one stage 4-7 invocations after a process restart (BTreeMap cache empty post-restart, all subsequent applies fail)? Should v1 ship eager-fill-from-L4-replay instead?
- **Q3 single backing vs separate BTreeMaps**: author lean is single backing with namespaced keys. Does this introduce a write-amp problem (every index update rewrites a single map)? Path B git-tree migration consideration: would namespaced keys map to git-tree paths cleanly, or do separate BTreeMaps map better to separate git refs?
- **Q4 bincode v2 serialization**: re-uses CO1.7-impl precedent. But CO1.7-impl serializes individual `LedgerEntry`s; CO1.8 serializes the FULL state snapshot. Is bincode v2 deterministic across compilation flags / target archs / future versions? Should CO1.8 specify a frozen format-version field?
- **Q5 agent_view stub**: v1 ships no-op filter. Does this leak evaluator internals (Inv 10 Goodhart shield breach) in the interim? Should v1 instead ship a deny-by-default stub that returns minimal hardcoded fields until CO1.5 ships?

**Q3. Sub-atom decomposition (spec § 0.3 8-atom table)**:
- Sprint graph says "Materialized State (8 atoms)" but does NOT enumerate. Author's mapping uses WP § 5.L5's 7 named modules + apply() = 8. Is this the right decomposition?
- Alternative decomposition: WP § 5.L5 lines 397-402 enumerate 6 modules (current_state_db, task_index, agent_reputation_index, error_taxonomy_index, price_signal_index, permission_view) + `read_tool` (line 408) + `apply()` = 8. The author folded "permission_view" into CO1.8.7 `agent_view::project_for_agent`. Is this correct, or is permission_view a SEPARATE module from agent_view (e.g., the access-control matrix vs the projected view)?
- LoC estimates per sub-atom (§ 0.3 column 3): are these grounded? CO1.8.2 apply.rs at ~120 LoC dispatching on TxKind + 7 variants seems low (~17 LoC per variant including doc + test stub).

**Q4. The apply() interface contract** (spec § 2):
- I-DET (determinism): is `apply(r, tx) == apply(r, tx)` actually achievable with bincode v2 serialization of complex types like `BTreeMap<String, Value>`? Map iteration order is deterministic for BTreeMap but not for HashMap. Does the spec mandate BTreeMap throughout state representation?
- I-PURE (no I/O): the BTreeMap-backed state_db is process-state. If multiple apply() calls share the same state_db (CO1.8.3), is apply() reading shared mutable state? If yes, "pure function" is a misnomer — it's a function over (cache, prior_root, tx) with implicit cache parameter.
- Atomicity (§ 2.3): "no atomicity concerns at the materializer layer". But if the state_db is shared mutable (point above), parallel Sequencer instances would race. Spec assumes single-Sequencer; should this be made explicit as a v1 invariant?

**Q5. STATE spec invocation surface match**:
- All 7 STATE spec invocation sites use signature `materializer::apply(&q.state_root_t, tx)`. CO1.8 § 2.1 signature is `apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, MaterializerError>`. The 7 STATE spec sites discard the result (no `?` or unwrap shown). Is the STATE spec stale on this point? Should CO1.8 introduce a panic-or-error helper?
- STATE spec line 852 has `materializer::apply(&q.state_root_t, &summary)` — `summary` is a `RejectedAttemptSummary`, NOT a `TypedTx`. Spec § 2.1 signature only accepts `&TypedTx`. Mismatch: either CO1.8 needs an overloaded `apply_summary` variant, or STATE spec line 852 is wrong.

**Q6. Test plan adequacy** (spec § 4, 5 tests):
- `apply_determinism` (4.1): adequate. But: does it cover bincode-version-skew, or only single-process determinism?
- `apply_genesis_to_first_state` (4.2): the assertion `assert_ne!(h, Hash::ZERO)` is necessary but not sufficient; a bug that hashes "v1.0\n" → some_constant for ANY tx would also pass. Should the test check `h ==` a specific golden hex?
- `agent_reputation_increments` (4.3): assumes `materializer::indices::agent_reputation::reputation_for(&new_root, "A0")` exists. But § 0.3 puts CO1.8.5 at ~70 LoC; this implies the public accessor signature. Is the accessor signature spec'd anywhere?
- `agent_view_filters_internals` (4.4): asserts `!view.contains_evaluator_internal_field("oracle_seed")`. But `oracle_seed` is not a field defined in v1 (no PredicateRegistry visibility tags). Is the test compilable in v1, or does it assume CO1.5 surfaces?
- `state_root_reproducibility` (4.5): genuinely substrate-independent.
- Missing tests: NO test exercises `MaterializerError::PriorRootNotFound` path (Q2 above). NO test exercises sub-index cross-references (e.g., does TaskMarket task creation affect agent reputation index?).

**Q7. Strategic risks not yet flagged**:
- Per memory `project_thesis`: "Frozen 5-step compile loop: Proposal → Ground-Truth Feedback → Logging → Capability Compilation → ↑H-VPPUT". Does CO1.8 advance this loop, or is it pure infrastructure that doesn't directly affect H-VPPUT measurability?
- CO1.8.7 `agent_view` is the KEY surface for the "minimal sufficient context" property (WP § 9.2). Stubbing it as no-op might be acceptable for v1 compile, but is there a HARD GATE somewhere downstream (PPUT-CCL Phase D? Phase C unfreeze?) that requires the real visibility filter to be in place?
- The 8 sub-atoms ship as a single compile unit but spec § 3 says "MAY get its own STEP_B-non-restricted commit during impl phase if size warrants". Does this disclaimer leave room for a half-finished interim state where some sub-indices are wired but others aren't?

## Verdict format

Section A: Verdict (PASS/CHALLENGE/VETO) with conviction (LOW/MED/HIGH).
Section B: P0 blockers (must-fix before round-2).
Section C: Open questions raised (architectural).
Section D: Suggested patches (specific spec line/section edits).
Section E: Forward-sustainability notes.

Be concrete. Cite spec § + line where possible.

BRIEF_EOF

# Append spec + key reference files
printf '\n\n---\n\n# XREF: spec — handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: WP § 5.L5 (whitepaper primary source)\n\n```\n' >> "$TMP_PROMPT"
sed -n '380,440p' "${ROOT}/handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: STATE_TRANSITION_SPEC v1.4 (7 materializer::apply invocation sites)\n\n```\n' >> "$TMP_PROMPT"
grep -n -B2 -A2 'materializer::apply' "${ROOT}/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: src/state/q_state.rs (Hash + state_root_t field)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/q_state.rs" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: src/state/typed_tx.rs (TypedTx 7-variant ABI)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: src/bottom_white/cas/store.rs (snapshot storage surface)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/cas/store.rs" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

printf '\n\n---\n\nNow give your INDEPENDENT round-1 audit. Cite spec § + line where possible.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.8 r1] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.8 Round-1 Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: spec v1 (greenfield)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.8 r1] API returned in ${elapsed}s" >&2
echo "[codex co1.8 r1] saved: $OUT" >&2
