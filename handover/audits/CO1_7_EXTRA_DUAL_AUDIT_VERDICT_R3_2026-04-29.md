# CO1.7-extra Dual External Audit — Round-3 Merged Verdict

**Date**: 2026-04-29
**Target**: `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.1 at HEAD `a3952cf`
**Audits**: Codex r3 (`CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md`) + Gemini r3 (`GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md`)

## Verdict matrix

| Audit | Verdict | Conviction |
|---|---|---|
| Codex | **CHALLENGE** | High (3 concrete patch blockers; "no foundational design flaw") |
| Gemini | **PASS** | High ("v1.1 spec is a model of post-audit closure") |

**Conservative-merged verdict** (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **CHALLENGE / High**. Codex's deeper source-level review caught 2 compile-blocking errors + 1 internal-contradiction blocker that Gemini's strategic angle did not surface.

## Round-2 must-fix closure (per round-3 review)

| R2 MF | Codex r3 status | Gemini r3 status |
|---|---|---|
| MF1 § 0.4 disposition table | ✅ PASS (factually correct) | ✅ PASS (Q5 sound) |
| MF2 advance_head_t helper | ❌ CHALLENGE (visibility + compile error) | ✅ PASS (Q4 testable + clean) |
| MF3 required trait method | ✅ PASS (compiler enforces; safety claims hold) | ✅ PASS (Q3 constitutionally sound) |
| MF4 Sequencer placement (TuringBus) | ⚠️ partial PASS — main rewrite correct; **stale Kernel references at spec line 14 + 395** | ✅ PASS (Q2 strict improvement) |
| MF5 flat test names | ✅ PASS | (covered in Q4) |
| MF6 manual Debug | ✅ PASS | (architectural verdict OK) |
| MF7 `entry_at` private | ✅ PASS | (architectural verdict OK) |
| MF8 stale comment cites | ✅ PASS (lines 180-184 + 359-361 match) | (architectural verdict OK) |
| MF9 atomicity wording | ✅ PASS | (architectural verdict OK) |
| MF10 LoC estimate | ⚠️ PARTIAL — § 7 says `210-300`, patch log says `200-280`; inconsistent | (out of scope for strategic review) |

## Round-3 new must-fix items (Codex-found)

### B1 — `&**writer_w` compile error (Codex Q2)

Spec § 1.1 stage-9 snippet:
```rust
advance_head_t(&mut *q_w, &**writer_w);
```

`writer_w` is `RwLockWriteGuard<dyn LedgerWriter>` per `src/state/sequencer.rs:201` + `:363-368`. `dyn LedgerWriter` cannot be double-dereferenced. The correct expression is `&*writer_w` (single deref turning the `RwLockWriteGuard<dyn LedgerWriter>` into `&dyn LedgerWriter`).

**Fix**: § 1.1 — `&**writer_w` → `&*writer_w`.

### B2 — `advance_head_t` visibility contradicts integration-test plan (Codex Q2)

Spec § 1.1 declares `pub(crate) fn advance_head_t(...)` but § 3.3 places the test in `tests/co1_7_extra_sequencer_head_t_advancement.rs` calling `turingosv4::state::sequencer::advance_head_t`. Integration tests live in a separate compilation unit; `pub(crate)` is not accessible from there. The MF5 flat-test choice forces this to be `pub`.

**Fix**: § 1.1 — `pub(crate) fn advance_head_t` → `pub fn advance_head_t`. Add doc-comment `/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4)` per FC-trace requirement.

### B3 — Stale MF4 references at spec line 14 + 395 (Codex Q4)

Spec body main § 2 correctly rewritten to TuringBus-only architecture, but two pre-existing v1 references survived the v1.1 patch round:
- **Line 14** (single-sentence summary, top of doc): still says "perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel". MF4 made this single-file (TuringBus only); "combined" + "Kernel" are stale.
- **Line 395** (pre-implementation gate paragraph in § 6): still mentions `src/kernel.rs` field as part of the gate. MF4 removed that touch surface.

**Fix**: § preface line 14 + § 6 line 395 — remove "combined" + "TuringBus + Kernel" from line 14; remove `src/kernel.rs` reference from line 395.

### B4 — Non-blocking inconsistencies (Codex Q5 + Q6)

- **B4a `#[serde(skip)]` conditional**: § 2.1 shows `#[serde(skip)]` on the new `TuringBus.sequencer` field unconditionally, but `src/bus.rs:53` `pub struct TuringBus` has no `Serialize/Deserialize` derive currently — the literal `#[serde(skip)]` would not be invalid syntactically (serde-skip without serde just no-ops) but is misleading documentation. v1.1 spec already has a comment "applied if TuringBus has Serialize/Deserialize" (line ~167); Codex flags that this conditional should be more explicit at the code-comment level.
- **B4b LoC inconsistency**: § 7 says `~210-300 LoC`; patch log says `~200-280`. Mechanical sync.

## Where audits agreed (round-3)

- ✅ All round-2 MF1, MF3, MF5, MF6, MF7, MF8, MF9 fully closed by v1.1 patches
- ✅ Architectural changes (Sequencer → TuringBus) are sound (Gemini "strict improvement"; Codex "main rewrite correct")
- ✅ Required trait method (MF3) is the correct pattern for constitutional anchor
- ✅ Helper extraction (MF2) is the right design — Codex's only complaint is `pub(crate)` vs `pub` (one-keyword fix)
- ✅ No foundational design flaw

## Where audits disagreed (round-3)

- **MF2 helper visibility**: Gemini didn't notice that `pub(crate)` blocks integration-test access (MF5 flat-test placement); Codex caught it. Conservative-wins → fix.
- **MF4 stale references**: Gemini didn't find the line-14 and line-395 leftovers; Codex caught them. Mechanical fix.
- **B1 compile error**: pure source-level finding only Codex's deep-grep would catch.

The disagreements are NOT architectural — Gemini's PASS is architecturally sound; Codex's CHALLENGE is mechanical-correctness. Both verdicts are coherent at their respective angles.

## Conservative-merged decision (no further audit input needed)

ArchitectAI applies all 4 patches (B1-B4) directly to v1.1 spec → v1.2. Per "无损压缩即智能", these are mechanical fixes; no architectural decision points remain ambiguous. Round-4 audit budget after v1.2: ~$3-7 (1 round expected to PASS/PASS — only mechanical fixes need verification).

## Audit cost summary

- Codex r3: 140,559 tokens (smaller than r2's 158k)
- Gemini r3: prompt + candidates ~140k tokens
- Estimated round cost: ~$5-10
- Cumulative project audit spend: ~$194-310 / $890 mid-budget (~22-35%)

## Status going forward

1. **CO1.7-extra v1.2**: spec patched in place this session (4 patches: B1 compile fix, B2 visibility, B3 stale refs ×2, B4 small inconsistencies); awaiting round-4 dual audit
2. **CO1.7.5 (transition bodies)**: future atom (unchanged from r1+r2 verdicts)
3. **LATEST.md correction**: still pending (per r1 + r2 + r3 verdicts; ~30-40% Wave 6 #1 progress diagnosis confirmed three rounds running)
