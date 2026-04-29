# CO1.7-extra Dual External Audit — Round-2 Merged Verdict

**Date**: 2026-04-29
**Target**: `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` at HEAD `617f01e`
**Audits**: Codex r2 (`CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`) + Gemini r2 (`GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`)

## Verdict matrix

| Audit | Verdict | Conviction |
|---|---|---|
| Codex | **CHALLENGE** | High |
| Gemini | **CHALLENGE** | High |

**Conservative-merged verdict**: **CHALLENGE / High**. No VETO. Both audits explicitly note "no foundational design flaw"; v1 issues are "readily correctable" (Gemini) / "implementable after fixes" (Codex).

## Round-1 must-fix closure status (per round-2 review)

| R1 MF | Audit-2 status |
|---|---|
| M1 substrate gap | ✅ closed by scope split (both audits PASS Q1) |
| M2 D1 purity | ✅ closed by scope split (Codex Q2 PASS) |
| M3a TuringBus rename | ✅ closed (Codex Q3 PASS) |
| M3b Kernel serde-skip | ✅ partial — serde-skip sufficient; **but** Sequencer Debug derive insufficient (Codex Q3 — Ed25519Keypair has no Debug derive at `src/bottom_white/ledger/system_keypair.rs:282-284`; blanket derive fails) |
| M3c Sequencer placement | ❌ **rejected by Codex (Q3 + Q7)**; "papers over real ownership/dependency coupling"; Kernel `src/kernel.rs:5-6` has hard warning re: domain-specific terms; less-invasive alternative exists (TuringBus) |
| M4 § 0.4 active reconciliation | ⚠️ partial — principle assertion accepted; commitment language directionally correct; **but** § 0.4 contains **factual contradiction** about supersession migration (head_t enacted HERE, not migrated) |
| M5 Q1 head_commit_oid_hex synthesis | ❌ **rejected by both audits**; default-None is convention not compiler-enforced; both r2 verdicts converge on **third option** (no default) |

## Round-2 new must-fix items

### MF1 — § 0.4 factual contradiction (both audits agree)

§ 0.4 states "two STATE supersessions migrate intact to future CO1.7.5", but D2 (`q_w.head_t = state::q_state::NodeId(commit_oid_hex)` at spec lines 72-73) **enacts** the head_t supersession HERE in CO1.7-extra. Only the SignalKind supersession migrates to future CO1.7.5.

**Fix**: amend § 0.4 to clearly state "head_t supersession enacted in CO1.7-extra D2; SignalKind supersession migrates to future CO1.7.5".

### MF2 — D2 code path test gap (both audits agree)

Neither `cas_payload_round_trip` nor `git2_writer_returns_some_after_commit` exercises the new D2 code in `Sequencer::apply_one` stage 9 (the `q_w.head_t = NodeId(...)` assignment). If implementation omits these lines, both proposed tests still pass.

**Fix**: add a test that asserts `q.head_t` is correctly updated post-commit. Implementation challenge: dispatch_transition currently returns `Err(NotYetImplemented)` for all variants, blocking apply_one from reaching stage 9. **Resolution adopted (per ArchitectAI; in v1.1)**: extract D2 logic into helper `advance_head_t(q: &mut QState, writer: &dyn LedgerWriter)` callable from apply_one stage 9 AND directly testable via mock writer. The extraction adds zero behavior change but makes D2 unit-testable without injection of dispatch_transition. Test file: `tests/co1_7_extra/head_t_advancement.rs` (or flat-named `tests/co1_7_extra_head_t_advancement.rs` per Cargo discovery; resolved in MF5 below).

### MF3 — `head_commit_oid_hex` trait method becomes required (both audits converge to 3rd option)

Round-1: Gemini voted `unimplemented!()`; Codex voted `default { None }` + override + test. v1 synthesized to default-None + mandatory-override-by-convention + defensive test. Round-2 both audits **reject** this synthesis as fragile (mandate is convention, not compiler-enforced).

Both r2 audits converge on **third option**: remove the default impl entirely. Rust compiler then enforces every `LedgerWriter` impl declares `head_commit_oid_hex` (Gemini Q3 cleanest; Codex Q5 also).

**Fix**: amend § 1.2 trait definition:
```rust
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn len(&self) -> u64;
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
    fn head_commit_oid_hex(&self) -> Option<String>;  // REQUIRED — no default
}
```

Both `Git2LedgerWriter` and `InMemoryLedgerWriter` (and any future impl) must explicitly declare. Compiler-enforced. Both audits' safety arguments (silent stagnation prevention + post-commit no-panic) satisfied.

### MF4 — Sequencer placement: TuringBus, not Kernel (Codex Q-3 + Q-7)

Codex Q-7: STEP_B Phase 0 asks for less-invasive alternative; TuringBus already owns runtime orchestration and avoids Kernel serde/debug/topology debt. Codex Q-3 (M3c): saying Kernel "holds the driver, not the data" papers over Sequencer's heavy ownership of CAS + keypair + registries + writer + QState. Kernel `src/kernel.rs:5-6` explicit warning against domain-specific terms.

Gemini Q5 r2 framing: Kernel placement is "pragmatic but architecturally compromising"; argues for separate runtime layer in stricter cases. The TuringBus alternative addresses Gemini's concern directly.

**Fix**: rewrite § 2.1 + § 2.2 + § 2.3:
- TuringBus gets `Option<Arc<Sequencer>>` field + `with_sequencer` constructor + `submit_typed_tx` forwarder
- Kernel UNTOUCHED (preserves "pure topology" doctrine)
- STEP_B becomes single-file ceremony on `src/bus.rs` only
- Combined-ceremony justification (functional coupling) no longer needed; replaced with simpler "single restricted-file change" rationale

**Architectural side-benefit**: cleaner layering. TuringBus owns Kernel + Sequencer as peers. Kernel stays at pure-topology layer. Sequencer at runtime-orchestration layer. Single-file STEP_B ceremony is simpler than combined.

## Round-2 smaller findings (to be patched in v1.1)

| ID | Source | Fix |
|---|---|---|
| **MF5** test harness | Codex Q-8 | `tests/co1_7_extra/*.rs` not auto-discovered by Cargo without `tests/co1_7_extra/main.rs` harness OR flat-named `tests/co1_7_extra_*.rs`. Spec § 3 chooses **flat-named** (simplest; aligns with existing convention in `tests/`). |
| **MF6** Sequencer Debug impl | Codex Q-3 (M3b refinement) | Manual `impl Debug for Sequencer` using `f.debug_struct("Sequencer").finish_non_exhaustive()`. Cannot use `#[derive(Debug)]` because Sequencer holds `Arc<Ed25519Keypair>` and `Ed25519Keypair` intentionally has no Debug derive (system_keypair.rs:282-284). Spec § 2.1 patched. |
| **MF7** canonical_test_entry private | Codex Q-5 | Helper at `transition_ledger.rs:813` is private to module tests. v1.1 either makes a small `pub(crate) fn canonical_test_entry()` helper, OR the new test inlines `LedgerEntry { ... }` construction. Spec § 3.2 chooses inline construction (test doesn't need cross-module reuse). |
| **MF8** stale Sequencer comments | Codex Q-8 | sequencer.rs:180-184 + :359-361 say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation must update these comments to reflect "head_t closed by CO1.7-extra D2". Spec § 1.1 adds to atom landing checklist. |
| **MF9** atomicity wording | Codex Q-6 | "post-commit non-failing best-effort head binding (Some path; Git2)" + "explicit no-op preservation (None path; InMemory)". Drop "atomic head close for all writers" framing. § 1.1 patched. |
| **MF10** LoC estimate | Codex Q-8 | 150-230 → 200-280 LoC (manual Debug impl + test harness + helper extraction + D2 head_t test add overhead). § 7 patched. |

## Where the audits agree (for the record)

- ✅ Scope split is constitutionally sound (Gemini Q1 + Codex Q1)
- ✅ Round-1 substrate gap MF closed (Gemini Q1 + Codex Q1)
- ✅ Round-1 D1 purity MF closed (Gemini implicit + Codex Q2)
- ✅ Round-1 § 0.4 process commitment principle (downstream supersedes upstream) is within ArchitectAI authority (Gemini Q2 + Codex Q4)
- ✅ STEP_B functional-coupling argument is stronger than minimum-sufficient-version invocation (Gemini Q4 PASS strong; Codex Q7 acknowledges-but-still-CHALLENGE because less-invasive alternative exists)
- ✅ Forward sustainability preserved (Gemini Q7 + Codex Q1)

## Where the audits disagree (none significantly)

Both round-2 verdicts converged on the same MF set with high agreement. Disagreements are at the level of severity weighting and small wording details, not fundamental directions. Codex performed deeper source-level verification (Sequencer field types, Ed25519Keypair Debug, Cargo test discovery); Gemini provided cleaner architectural framing (third-option trait design, scope-split soundness).

## Conservative-merged decision (no further audit input needed)

ArchitectAI applies all 10 patches (MF1-MF10) directly to v1 spec → v1.1. Per "无损压缩即智能", the patches are systematic application of audit findings; no architectural decision points remain ambiguous.

Round-3 audit budget after v1.1: ~$5-10 (1 round expected to PASS/PASS — small focused atom; r2 issues are correctable; no new architectural surface introduced).

## Audit cost summary

- Codex r2: 158,872 tokens
- Gemini r2: prompt=130,878 / candidates=2,968 / total=137,133 tokens
- Estimated round cost: ~$6-12
- Cumulative project audit spend: ~$189-300 / $890 mid-budget (~21-34%)

## Status going forward

1. **CO1.7-extra v1.1**: spec patched in place this session; awaiting round-3 dual audit
2. **CO1.7.5 (transition bodies)**: future atom (unchanged from r1 verdict)
3. **LATEST.md**: should be patched to reflect Wave 6 #1 ~30-40% diagnosis (per r1 verdict, reconfirmed in r2 Q1)
4. **PROJECT_DECISION_MAP**: should track CO1.7-extra as new bridge atom (was added to task #5 but should be sedimented in the canonical decision-map doc)
