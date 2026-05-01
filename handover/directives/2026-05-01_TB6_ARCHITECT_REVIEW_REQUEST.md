# Architect Review Request — TB-6 sequencing + post-TB-5 stage gate

**Date**: 2026-05-01
**Branch**: `main` @ `c472823` (TB-5 SHIPPED + book-keeping)
**Requester**: AI session post-TB-5 ship
**Author signature**: this prompt was authored by the same AI that shipped TB-5; reviewer should treat as self-reported state, not external audit.

---

## §0 What the architect needs to decide

5 binding decision items (D1–D5). Each has options + a recommended default + a kill-condition. The architect's role is to RULE on each (PASS / CHALLENGE / VETO with alternative) so TB-6 can charter immediately on response.

---

## §1 Current state (verified 2026-05-01)

### §1.1 TB-5 ship

- Merge `1bdc55a` (--no-ff) + book-keeping `c472823` on `main`.
- 8 atoms post charter v2 sign-off (`4c3414e..2fb4ed9`).
- Production claim adds: Anti-Oreo agent-vs-system ingress separation **structurally enforced** at runtime spine (was documented norm); `emit_system_tx` constructs+signs internally; apply_one stage 1.5 verifies via `PinnedSystemPubkeys`; ChallengeResolve dispatch arm Released + UpheldDeferred paths green; CTF conserved across mixed sequences; 9-sub-field EconomicState + 5-holding invariant preserved.
- `cargo test --workspace` → **617 passed / 0 failed** (TB-4 baseline 571 + 46 net TB-5 additions).

### §1.2 Self-audit findings (2026-05-01)

Two issues, one cosmetic + one substantive.

#### §1.2a (cosmetic) Test-count under-report 464 → 617

`README` + `RECURSIVE_AUDIT` + `TB_LOG.tsv` + `NOTEPAD` + merge commit `1bdc55a` body all say **464/464**. That was the bare `cargo test` count (root crate only), not `cargo test --workspace`. Actual workspace count is 617/617 (46 suites; 0 failed). TB-3 + TB-4 baselines were `--workspace`, so the comparison was apples-to-oranges.

Severity: medium. Substantive claim ("suite green at TB-5 ship; ~44 new tests") intact; off by 2 (actual 46). Patch commit drafted; affects 5 living docs (the merge commit body cannot be amended).

#### §1.2b (substantive) Chaintape gap

The TB-5 "smoke tape" evidence at `handover/evidence/tb_5_smoke_2026-04-30/` is **not on a chain**. It is:
- `oneshot_run.log` — 2-line stdout dump (1 PPUT_RESULT JSON line)
- `n1_run.log` — same shape
- `proof_n1.lean` — Lean source (CAS-verified bit-identical to runtime emit; re-verifies under pinned toolchain v4.24.0)
- `README.md` — narrative

None of these traverse `Sequencer::apply_one` → `LedgerWriter::commit`. The evaluator binary (`experiments/minif2f_v4/src/bin/evaluator.rs`) does not import `turingosv4::state::sequencer` at all. `bus.rs:73`'s `sequencer: Option<Arc<Sequencer>>` field is `None` in production (`TuringBus::new_legacy()` — `main.rs`).

**The chaintape machinery exists** (`transition_ledger::LedgerEntry` with `parent_ledger_root` + `system_signature` + `tx_payload_cid`; `Git2LedgerWriter` + `InMemoryLedgerWriter`; `apply_one` produces signed entries; replay tests I29 + I80 reconstruct economic state). **But it only runs inside `cargo test`.** No production binary drives it; no on-disk chaintape from any LLM-driven run exists yet.

The "smoke tape" naming is a v3 PaperTape-era metaphor. If someone tampered with two characters in `n1_run.log`, no invariant in the codebase would catch it — there is no signature, no chain, no replay path against that evidence. The TB-5 README §"What this smoke does NOT prove" actually concedes this ("evaluator's PputResult emit path is pre-runtime; routing it through Sequencer::emit_system_tx is P2 Agent Runtime territory, not TB-5 scope") — but the language elsewhere (charter, audit, NOTEPAD) treats the .log files as if they were a stronger guarantee than they are.

Severity: high. This is an **honest-naming gap**, not a code bug. The runtime kernel is genuinely Anti-Oreo; we just can't prove it from the smoke evidence alone, because the smoke evidence didn't traverse the kernel. Cargo test --workspace exercises the kernel comprehensively (617 tests including replay invariants); LLM-driven smoke runs do not.

### §1.3 Where TB-1..TB-5 actually got the kernel

| TB | Production claim | Kernel-exercised by tests | Kernel-exercised by smoke runs |
|---|---|---|---|
| TB-1 | RSP-0 primitives + invariant scaffolding | ✅ Tier-A 9/9 | ❌ Day-4 h_vppu pre-dates `prompt_context_hash` field; no chain |
| TB-2 | Runtime spine green: accepted WorkTx → canonical L4 | ✅ I1-I13 | ❌ oneshot only; no chain |
| TB-3 | RSP-1 formal tx surface | ✅ I20-I30 + bridge invariant | ❌ oneshot only; no chain |
| TB-4 | RSP-2 admission spine | ✅ I31-I44 + replay/property | ⚠️ medium batch evidence (4/5 SOLVED at MAX_TX=30); also pre-runtime |
| TB-5 | RSP-3.0/3.1 system-emitted resolution gate | ✅ I60-I89 + anti-drift CI | ❌ oneshot + n1; pre-runtime |

**Pattern**: every TB ships a kernel improvement that is fully tested in `cargo test --workspace` but not exercised by any LLM-driven binary. The cumulative debt is now 5 TBs of kernel functionality with no production wire-up.

---

## §2 Decision items

### D1 — TB-6 sequencing: RSP-3.2 vs P2 Agent Runtime

**Current ROADMAP plan**: TB-6 = RSP-3.2 slash execution (`SlashTx` system-emitted; balances/stakes/challenge_cases mutations conditional on `ChallengeCase.status == UpheldDeferred`; builds on TB-5's UpheldDeferred anchor + emit_system_tx + apply_one stage 1.5).

**Alternative**: TB-6 = P2 Agent Runtime atom — wire `experiments/minif2f_v4/src/bin/evaluator.rs` to drive a real `Sequencer` + persistent `Git2LedgerWriter`; one LLM-driven run produces a walkable on-disk chain; smoke gate from TB-7 onward becomes "show me the chaintape" instead of stdout dump.

| Axis | RSP-3.2 (slash) | P2 Agent Runtime |
|---|---|---|
| Builds on | TB-5 UpheldDeferred anchor + emit_system_tx | TB-2..TB-5 entire kernel (now) |
| Discharges | P3:9 (slash) | The 5-TB chaintape debt + § 1.3 smoke gap |
| Closes the §1.2b honest-naming gap | ❌ no | ✅ directly |
| Hits 5-step compile loop steps | Logging + Capability Compilation | Ground-Truth Feedback + Logging |
| Iteration-cap (24h) compatibility | ⚠️ pure-kernel atom; no new evaluator pass/fail signal — needs explicit exception again | ✅ yields LLM-driven evaluator output that traverses the chain (capability signal native) |
| Risk if deferred | TB-6 stays clean; can defer to TB-7 | Each additional TB widens the 5-TB-already-debt; chain wire-up complexity grows |

**Recommended default**: **P2 Agent Runtime atom first** (TB-6); slash deferred to TB-7. Reason: 5 TBs of kernel work without wire-up is a credibility scaling problem, not an engineering one. One small wire-up atom converts "trust the cargo test suite" into "trust the on-disk chaintape" — which is the central structural property TuringOS claims.

If architect prefers staying on RSP-N micro-version sequence (RSP-3.2 next), that's defensible but should come with explicit ruling on §1.2b: how is the "smoke tape" rename + chain wire-up funded over time? An explicit "P2 atom = TB-7" target is fine.

**Architect ruling needed**: ☐ TB-6 = P2 Agent Runtime / ☐ TB-6 = RSP-3.2 / ☐ TB-6 = something else (specify)

---

### D2 — Smoke gate evolution

**Current**: TB-N ship gate includes `handover/evidence/tb_N_smoke_YYYY-MM-DD/` — oneshot + (since TB-4) n1 — with `prompt_context_hash` invariance + Lean re-verify as the only structural checks.

**Question**: should the smoke gate require chaintape traversal from TB-X onward?

Options:
- **(a)** Soft: smoke evidence is non-blocking per directive § 5.4 (current); rename "smoke tape" → "smoke evidence" in all docs to remove the chain-implying terminology
- **(b)** Hard from TB-6: chain wire-up is a TB-6 deliverable; from TB-7 onward, smoke must produce ≥1 LedgerEntry walkable on-disk + ship gate verifies chain integrity
- **(c)** Hard from TB-7: defer chain wire-up to its own TB (per D1 alt); rename retroactively to "smoke evidence"
- **(d)** Drop smoke gate entirely; rely on `cargo test --workspace` as the sole structural check

**Recommended default**: **(b)** if D1 = P2 Agent Runtime; **(c)** if D1 = RSP-3.2.

**Architect ruling needed**: ☐ (a) / ☐ (b) / ☐ (c) / ☐ (d)

---

### D3 — Audit-mode standard going forward

**TB-3 / TB-4 precedent**: Option B = self-audit + 真题烟测 replaces dual external audit (per user 2026-04-30 authorization).

**TB-5 actual**: Option A reinstated by directive § 4 Q4 (system-emitted economic mutators ≠ Option B precedent); Gemini strategic-tier `MODEL_CAPACITY_EXHAUSTED` across rounds 1–3; Codex-only mode by supplement; round-4 fell back to **grep-based self-verification** when Codex agent infrastructure failed mid-audit.

**Question**: what is the standing policy for TB-6+?

Options:
- **(a)** Default Option B (self-audit + smoke); flip to Option A only on architect directive (TB-5 model)
- **(b)** Default Option A Codex-only with grep self-verification fallback as TB-5 supplement legitimized; revisit when Gemini capacity recovers
- **(c)** Default Option A dual external; if Gemini capacity exhausted, halt the TB until capacity returns (maximum strict)
- **(d)** Hybrid: Option B for kernel-only atoms; Option A Codex-only for system-emitted economic mutators (TB-5 carve-out generalized)

**Recommended default**: **(d)** — match audit weight to constitutional risk class. Kernel additive atoms (RSP-1, RSP-2 admission paths) self-audit; system-emitted state writers (RSP-3 resolution, RSP-3.2 slash) Codex-only with explicit charter-time supplement when Gemini exhausted.

**Architect ruling needed**: ☐ (a) / ☐ (b) / ☐ (c) / ☐ (d) / ☐ alternative

---

### D4 — Test-count reporting standard

**Question**: lock down `cargo test --workspace` as the canonical command for ship-gate test counts in all ship docs (charter § 5.4 / TB_LOG capability_metric / RECURSIVE_AUDIT)? This avoids future TB-5-style under-counts.

**Recommended default**: yes; bake into `feedback_phased_checkpoint` memory ("ship-gate test counts MUST be `cargo test --workspace`").

**Architect ruling needed**: ☐ yes / ☐ no / ☐ different command (specify)

---

### D5 — Chaintape gap honest-naming

**Question**: rename "smoke tape" → "smoke evidence" across all docs that refer to .log evidence files? "Tape" is reserved for chaintape (LedgerEntry chain), once it's walkable.

**Recommended default**: yes; one-shot find-replace patch commit + retire "smoke tape" terminology in NOTEPAD + LATEST + all charter templates.

**Architect ruling needed**: ☐ yes / ☐ no / ☐ partial (specify)

---

## §3 What architect should NOT need to decide

These are AI-side execution details, surfaced for transparency only:
- 464→617 patch commit on main (mechanical correction; will be done unless vetoed)
- Self-audit doc `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` (already drafted; will be committed unless vetoed)
- Stage audit doc `handover/audits/STAGE_AUDIT_TB_1_TO_TB_5_2026-05-01.md` (drafted; commit unless vetoed)

---

## §4 Reading order for architect

1. This file (decision items live here)
2. `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` (substantive evidence for §1.2)
3. `handover/audits/STAGE_AUDIT_TB_1_TO_TB_5_2026-05-01.md` (cumulative TB-1..TB-5 picture)
4. `handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md` (ship-time self-audit; superseded on count by §1.2a)
5. `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § 3 P3 + § 6 (RSP-N micro-version sequence)
6. `handover/tracer_bullets/TB_LOG.tsv` (TB-5 row; 8-atom commit chain)

## §5 Response shape requested

A short doc at `handover/directives/2026-05-XX_TB6_DIRECTIVE.md` ruling on D1–D5 + any binding charter constraints for TB-6 (analogous to `2026-04-30_TB5_VETO_redesign_directive.md` shape).
