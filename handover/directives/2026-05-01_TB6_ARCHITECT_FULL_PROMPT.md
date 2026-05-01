# Architect Full Prompt — TB-6 sequencing + chaintape gap escalation

**Date**: 2026-05-01
**Branch / HEAD**: `main` @ `8c6d95f` (origin/main synced)
**Author**: AI session post-TB-5 ship — single-AI self-audit; no external auditor invoked.
**Supersedes**: `2026-05-01_TB6_ARCHITECT_REVIEW_REQUEST.md` (kept on disk as precursor; this doc is the canonical architect-facing prompt).
**Response shape requested**: `handover/directives/2026-05-XX_TB6_DIRECTIVE.md` analogous to `2026-04-30_TB5_VETO_redesign_directive.md` — binding rulings on D1–D7 + any charter constraints for TB-6.

---

## §0 TL;DR (the headline finding)

**Smoke evidence is not on a chain. No production binary drives the Sequencer.**

TuringOS v4 has shipped 5 tracer-bullets (TB-1..TB-5) over 2026-04-29 to 2026-04-30, each adding kernel functionality to `src/state/sequencer.rs` + `src/state/typed_tx.rs` + `src/bottom_white/ledger/transition_ledger.rs`. The cumulative claim is "TuringOS runtime kernel structurally enforces L4 / L4.E split + Anti-Oreo agent-vs-system separation + RSP-1/2/3.0/3.1 economy core."

Today (2026-05-01) audit found:

- **All 5 TBs are tested only inside `cargo test --workspace`** (617/617 green). The cargo test harness uses `InMemoryLedgerWriter` and produces a real cryptographic chain — but only inside test process memory, dropped when test ends.
- **No production binary drives `Sequencer::apply_one`.** `bus.rs:73`'s `sequencer: Option<Arc<Sequencer>>` is `None` in `main.rs` (`TuringBus::new_legacy()`). The evaluator binary at `experiments/minif2f_v4/src/bin/evaluator.rs` does **not import** `turingosv4::state::sequencer` at all (zero hits on grep).
- **No on-disk chaintape has ever been produced** from any LLM-driven run in TuringOS v4 history.
- **The "smoke tape" in `handover/evidence/tb_N_smoke_*/` directories is paper trail** — `*_run.log` (stdout dumps) + `proof_n1.lean` (source) + `README.md` (narrative). None traverse `Sequencer::apply_one` → `LedgerWriter::commit`. No `system_signature`, no `parent_ledger_root` chain, no replay path against this evidence. The naming is a v3 PaperTape-era metaphor.

**This is the testing-platform gap the user surfaced 2026-05-01.** The kernel is real; the kernel is tested; but the kernel has never run in production. Architect ruling required on TB-6 sequencing (D1) + 4 follow-on standards (D2–D5) + 2 codification items (D6–D7).

---

## §1 What the architect should audit (file paths, exhaustive)

### §1.1 Recent audit reports (read in this order)

| Path | Purpose |
|---|---|
| `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` | The smoke-tape audit. § 1 8 PASS / § 2 cosmetic count / § 3 chaintape gap / § 4 verdict / § 5 caveats. **Read first.** |
| `handover/audits/STAGE_AUDIT_TB_1_TO_TB_5_2026-05-01.md` | Cumulative stage audit. Per-TB summary table + what's structurally green + what's gap + 5 open debts + 8 production claims rolling forward. |
| `handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md` | TB-5 ship-time self-audit (test-count corrected 2026-05-01 in-place). 6/6 directive Q + 10/10 charter v2 § 4 decision blocks + 4/4 anti-drift renames + 3/3 ship gate proofs all GREEN. |
| `handover/audits/RECURSIVE_AUDIT_TB_4_2026-04-30.md` | TB-4 ship-time self-audit (precedent for self-audit + 真实烟测 mode replacing dual external audit). |
| `handover/audits/RECURSIVE_AUDIT_TB_3_2026-04-30.md` | TB-3 ship-time self-audit (first instance of Option B audit mode under user 2026-04-30 authorization). |
| `handover/audits/DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md` | TB-5 charter v1 VETO verdict (Codex Part B VETO on agent-forgery hole; Gemini degraded MODEL_CAPACITY_EXHAUSTED). Drove the v2 redesign. |
| `handover/audits/CODEX_TB_5_PHASE0_R4_AUDIT_2026-04-30.md` | TB-5 Atom 1 round-4 self-verification fallback (Codex agent infrastructure failure mid-audit; user-authorized grep-based mechanical text-presence checks). |

### §1.2 Charter + directive history

| Path | Purpose |
|---|---|
| `handover/tracer_bullets/TB-5_charter_2026-04-30.md` | TB-5 v2 charter (post-VETO redesign). |
| `handover/tracer_bullets/TB-4_charter_2026-04-30.md` | TB-4 charter (RSP-2 admission). |
| `handover/tracer_bullets/TB-3_charter_2026-04-30.md` | TB-3 charter (RSP-1 formal tx surface). |
| `handover/tracer_bullets/TB-2_charter_2026-04-30.md` | TB-2 charter (runtime spine closure). |
| `handover/directives/2026-04-30_TB5_VETO_redesign_directive.md` | TB-5 v2 directive (post-VETO; 6 Q + 11 structural rulings). |
| `handover/directives/2026-04-30_TB5_audit_mode_supplement.md` | TB-5 Codex-only mode authorization (Gemini exhausted). |
| `handover/directives/2026-04-30_TB4_directive.md` | TB-4 directive (Q1-Q7 + 5 anti-drift clauses). |
| `handover/directives/2026-04-29_9_phase_roadmap.md` | Active 9-phase roadmap directive. |
| `handover/directives/2026-05-01_TB6_ARCHITECT_REVIEW_REQUEST.md` | Earlier shorter review request (precursor to this doc). |

### §1.3 Source code (kernel surface)

| Path | What it contains | TB ownership |
|---|---|---|
| `src/state/sequencer.rs` (~3050 LOC) | `Sequencer` struct, `submit_agent_tx` + `emit_system_tx` + `apply_one` (stages 1.5/2/3-9), `dispatch_transition` (10-variant exhaustive match), `record_rejection` helper, `system_message_for_verification` exhaustive helper, 5 state-root domains | TB-2/3/4/5 |
| `src/state/typed_tx.rs` (~1600 LOC) | 10 `TypedTx` variants, `TransitionError` enum (32 variants + 6 added in TB-3/4/5), 8 SigningPayload structs with `canonical_digest()`, `HasSubmitter` trait, golden-digest tests T1-T5 | TB-1/3/4/5 |
| `src/state/q_state.rs` | `QState` + `EconomicState` (9 sub-fields) + `ChallengeCase {+status}` + `ChallengeStatus` enum (TB-5 single-source-of-truth) + `StakeEntry {+task_id}` + `TaskMarketEntry` (with derived `total_escrow` cache) | TB-1/3/4/5 |
| `src/bottom_white/ledger/transition_ledger.rs` | `LedgerEntry` (with `parent_ledger_root` chain + `system_signature`) + `LedgerEntrySigningPayload` + `InMemoryLedgerWriter` + `Git2LedgerWriter` (line 642; on-disk persistence; **never instantiated in production**) + `transition_ledger_emitter::sign_ledger_entry` | TB-1/2 |
| `src/bottom_white/ledger/system_keypair.rs` | `Ed25519Keypair` + `PinnedSystemPubkeys` + `CanonicalMessage` (8 system-message variants incl. `ChallengeResolveSigning` from TB-5) + `terminal_summary_emitter::sign_*` helpers + `verify_system_signature` | TB-1/3/4/5 |
| `src/bottom_white/ledger/rejection_evidence.rs` | `RejectionEvidenceWriter` + `RejectedSubmissionRecord` (with `raw_diagnostic_cid` serde-shielded per TB-1 P0-3) + `PublicRejectionView` projection | TB-1 |
| `src/economy/monetary_invariant.rs` | `assert_no_post_init_mint` (exhaustive over 10 TypedTx variants) + `assert_total_ctf_conserved` (5 holdings) + `assert_read_is_free` + `assert_task_market_total_escrow_matches_locks` (cache=truth) + K1-K5 unit tests | TB-1/3/5 |
| `src/bus.rs` | `TuringBus` struct with `pub sequencer: Option<Arc<Sequencer>>` field (line 73) — **always `None` in production** via `new_legacy()` | TB-1 + bus skeleton |
| `src/main.rs` | `TuringBus::new_legacy()` constructor — **does not wire Sequencer**. This is the production-wire-up gap site. | (none — gap) |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | LLM-driven evaluator binary (the only thing producing PPUT_RESULT smoke evidence). **Does not import `turingosv4::state::sequencer`.** This is the production-wire-up gap site. | (none — gap) |

### §1.4 Test files (the actual structural audit surface)

| Path | Tests | What it proves |
|---|---|---|
| `src/state/sequencer.rs::tests` (in-crate) | ~30 unit tests U1-U34 + stage_1_5_* | `dispatch_transition` per-arm correctness; ingress barrier; stage 1.5 forged-sig rejection × 4 system variants; ChallengeResolve dispatch arm Released + UpheldDeferred + AlreadyResolved + ChallengeNotFound + StaleParent paths |
| `src/state/typed_tx.rs::tests` | T1-T5 | Canonical digest determinism; signing payload field counts; golden digests for ChallengeResolveSigningPayload; TransitionError Display non-empty for InvalidSystemSignatureLive |
| `tests/tb_2_runtime_boundary.rs` | I1-I13 | Real `Sequencer::submit` traverses dispatch + writes L4 / L4.E correctly; replay from L4 reconstructs state |
| `tests/tb_3_rsp1_formal_surface.rs` | I20-I30 | TaskOpen + EscrowLock first-class; bridge deletion; WorkTx admission structural; lock-on-accept; CTF conservation; replay invariant |
| `tests/tb_3_bridge_deletion_invariant.rs` | 2 | Bridge pattern must not resurrect (scanner) |
| `tests/tb_4_rsp2_admission_surface.rs` | I31-I44 | VerifyTx + ChallengeTx admission; opened_at_round anchor; target_work_tx backref; multi-challenger; FORBIDDEN_VARIANTS scanner (NoStakeTx/VerifierBondTx/ChallengeStakeTx/VerifierStakeTx) |
| `tests/tb_5_system_ingress_barrier.rs` | I60-I69 + T5 | Anti-Oreo agent ingress rejects 4 system variants pre-queue; emit_system_tx accepts + emit_id namespace; legacy submit alias inheritance |
| `tests/tb_5_challenge_resolve_surface.rs` | I70-I81 + I88/I89 | ChallengeResolve dispatch through full pipeline (emit_system_tx → try_apply_one → L4 advance); Released + UpheldDeferred + AlreadyResolved + ChallengeNotFound + boundary tests (no solver/verifier stake mutation) + replay + property |
| `tests/tb_5_anti_drift.rs` | 3 | FORBIDDEN_VARIANTS extended (SlashTx/SettlementTx/ProvisionalAcceptTx/ReputationUpdateTx); charter hygiene; P6-touch git-diff guard |
| `tests/economic_invariant_INV3.rs` | (existing TB-3 era) | 5-holding CTF invariant + total_supply_micro |
| `tests/economic_state_reconstruct.rs` | (existing) | EconomicState round-trips through serde |
| `tests/fc_alignment_conformance.rs` | governance | Each ✅ FC row in `handover/alignment/TRACE_MATRIX_v0_*.md` has ≥1 witness test |

### §1.5 Smoke evidence (what's in the "tape" directories)

| Path | Files | Caveat |
|---|---|---|
| `handover/evidence/tb_5_smoke_2026-04-30/` | README.md + oneshot_run.log + n1_run.log + proof_n1.lean | TB-5 ship; **paper trail not chain** per § 0; oneshot bit-identical hash + n1 SOLVED + Lean re-verify |
| `handover/evidence/tb_4_smoke_2026-04-30/` | same shape | TB-4 ship; same paper-trail caveat |
| `handover/evidence/tb_4_medium_batch_2026-04-30/` | 5-problem mixed-difficulty batch | 4/5 SOLVED at MAX_TX=30; same paper-trail caveat |
| `handover/evidence/tb_3_smoke_2026-04-30/` | README.md + oneshot_run.log | TB-3 ship; oneshot only |
| `handover/evidence/tb_2_phase1_smoke_2026-04-30/` | same shape | TB-2 ship |
| `handover/evidence/tb_1_day4_h_vppu/` | run1.jsonl + run2.jsonl + h_vppu_history.json + README.md | TB-1 Day-4; pre-`prompt_context_hash` field; capability anchor only |
| `handover/evidence/first_v4_solve_2026-04-29/` | First v4-native solve evidence | TB-0 / TB-1 day-1 spike; capability anchor |

### §1.6 Book-keeping + alignment

| Path | Purpose |
|---|---|
| `handover/tracer_bullets/TB_LOG.tsv` | All TBs' status / phase_id / kill criteria / capability_metric / ship_commits |
| `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` | 9-phase roadmap; § 3 P3 RSP economy core current state (post-TB-5); § 6 RSP-N micro-version sequence; § 11 phase dependency graph |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` | Active research log; TB-5 SHIPPED log + 2026-05-01 self-audit log |
| `handover/ai-direct/LATEST.md` | Session-by-session handover state (top entry = most recent) |
| `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` | WP § 14.1 / § 18 / § 19 (the canonical reference for inline-stake/bond shape + ChallengeCourt + WP-canonical reconciliation rule) |
| `genesis_payload.toml` | Trust Root manifest (40+ files SHA-tracked + verified at boot) |
| `constitution.md` | The constitutional anchor; Anti-Oreo per Art V.1.3; sudo authority per Art VIII |
| `cases/V3_LESSONS.md` | 50 v3 lessons → current judicial precedents |
| `CLAUDE.md` | Project-level AI instructions |

### §1.7 Memory (persistent AI context — affects how next session reads this)

| Path | Relevance |
|---|---|
| `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/MEMORY.md` | Index of 30+ feedback / project / reference memories |
| `feedback_step_b_protocol.md` | STEP_B-protected files (sequencer.rs / typed_tx.rs / etc.) require parallel-branch A/B, not direct edit |
| `feedback_dual_audit.md` | Codex + Gemini dual external audit standard (TB-5 broke this due to Gemini exhaustion) |
| `feedback_wp_vs_roadmap_reconciliation.md` | WP § 14.1 / § 18 inline-stake shape wins over ROADMAP `yes_stake_tx`-style names |
| `feedback_phased_checkpoint.md` | Multi-phase plans need paired N=20 A/B + checkpoint doc + 7 red-line check |
| `feedback_iteration_cap_24h.md` | 24h iteration cap: every PR must produce evaluator pass/fail signal within 24h; spec/audit-only PR = default-reject (relevant to D1) |
| `feedback_smoke_before_batch.md` | Any config change requires smoke probe before full batch |
| `feedback_no_fake_menus.md` | When project plan determines next atom, state and execute; don't surface 3-5 option menus |

---

## §2 Prior work summary (TB-1 to TB-5)

### §2.1 Per-TB digest

| TB | Date | Production claim | Tests added | Smoke | Chain on disk? |
|---|---|---|---|---|---|
| TB-0 | 2026-04-29 | First v4-native solve `pput=0→0.000215` on `mathd_algebra_107` via nlinarith | 0 | first_v4_solve_2026-04-29/ | ❌ |
| TB-1 | 2026-04-30 | RSP-0 primitives + L4.E rejection-evidence + Trust Root + monetary invariants | +9 Tier-A | tb_1_day4_h_vppu/ (capability anchor) | ❌ |
| TB-2 | 2026-04-30 | Runtime spine: accepted WorkTx → canonical L4 (state_root_t/ledger_root_t/logical_t advance); rejected → L4.E with submit_id; replay from L4 alone reaches same state | +16 (3 in-crate + 13 integration I1-I13) | tb_2_phase1_smoke/ (oneshot) | ❌ |
| TB-3 | 2026-04-30 | RSP-1 formal tx surface: TaskOpen + EscrowLock first-class; bridge DELETED (CI-enforced); WorkTx admission structural; lock-on-accept | +29 (5 typed + 8 in-crate + 11 integration + 2 bridge invariant + 3 monetary) | tb_3_smoke/ (oneshot) | ❌ |
| TB-4 | 2026-04-30 | RSP-2 admission: VerifyTx (verifier bond → stakes_t) + ChallengeTx (challenger NO stake → challenge_cases_t with opened_at_round + target_work_tx); 3-class error taxonomy | +30 (5 typed + 10 in-crate + 12 integration + 3 control/replay) | tb_4_smoke/ (oneshot + n1 SOLVED) + tb_4_medium_batch/ (4/5 SOLVED at MAX_TX=30) | ❌ |
| TB-5 | 2026-04-30 | RSP-3.0/3.1: Anti-Oreo agent-vs-system ingress separation **structurally enforced**; emit_system_tx constructs+signs internally; apply_one stage 1.5 PinnedSystemPubkeys verify; ChallengeResolve dispatch arm Released + UpheldDeferred | +46 (5 typed + 13 in-crate + 10 + 13 integration + 3 anti-drift + 2 misc) | tb_5_smoke/ (oneshot + n1 SOLVED) | ❌ |

**Cumulative**: 130 net new rust tests; 617 workspace count at TB-5 ship; 0 failed.

### §2.2 8 production claims rolling forward

All demonstrated by `cargo test --workspace`; **none** demonstrated by an LLM-driven production run:

1. Runtime kernel honors L4 / L4.E split (TB-2)
2. RSP-1 formal tx surface is on canonical L4 (TB-3)
3. WorkTx.stake commits real money on accept; rejected WorkTx leaves economic state untouched (TB-3)
4. task_market.total_escrow is derived cache, not money holding (TB-3)
5. VerifyTx.bond + ChallengeTx.stake stay inline; no VerifierBondTx / NoStakeTx variants (TB-4 + CI-enforced)
6. ChallengeResolve is system-emitted only; agent forging structurally impossible (TB-5)
7. emit_system_tx constructs+signs internally; apply_one stage 1.5 verifies via PinnedSystemPubkeys (TB-5)
8. 5-holding CTF invariant + 9-sub-field EconomicState preserved across all admission + resolution paths (TB-3..TB-5 cumulative)

### §2.3 What's RED (gap)

- **Production binary chaintape wire-up** — § 0 finding; 5-TB cumulative debt
- RSP-3.2 slash execution (TB-6 candidate)
- RSP-4 settlement_engine (TB-7+)
- RSP-5 contribution_dag, RSP-6 reputation/price index, RSP-7 production market dynamics
- P2 Agent Runtime end-to-end role separation (depends on production wire-up)
- P4 Information Loom (depends on real L4.E rows + reputation events)
- P5/P6/P7/P8 (out of scope by design)

---

## §3 Testing methods + results

### §3.1 `cargo test --workspace` (the actual structural audit surface)

```
cargo test --workspace
→ 617 passed / 0 failed / 1 ignored  (46 suites)
```

**What it proves**:
- Every kernel state transition: `dispatch_transition` exhaustive over 10 TypedTx variants
- L4/L4.E split: K1 invariant (rejection does not consume logical_t) + Inv 7 (no state_root_t / ledger_root_t advance on reject)
- Anti-Oreo barrier: 4 system variants rejected pre-queue at `submit_agent_tx`; defense-in-depth stage 1.5 forged-sig rejection × 4 variants → InvalidSystemSignatureLive + 1 L4.E PolicyViolation row
- ChallengeResolve dispatch: Released + UpheldDeferred + AlreadyResolved + ChallengeNotFound + StaleParent paths
- Replay determinism: I29 + I80 reconstruct economic state from L4 alone
- CTF conservation across mixed sequences (5-holding sum unchanged after Released bond refund + UpheldDeferred marker)
- Anti-drift CI: 4 forbidden variant names absent from src/ (TB-3 bridge + TB-4 4 phantoms + TB-5 4 phantoms = 9 cumulative scanner entries)
- Trust Root manifest: 40+ load-bearing files SHA-verified at boot

**What it does NOT prove**:
- That any LLM-driven binary ever invokes `Sequencer::apply_one`
- That any chaintape was ever persisted to disk via `Git2LedgerWriter`
- That Anti-Oreo barriers fire under real network/agent conditions

### §3.2 Smoke evidence (capability + compat signal; NOT chain audit)

For TB-N: `handover/evidence/tb_N_smoke_*/` produced by `./target/debug/evaluator <problem>.lean` (in `experiments/minif2f_v4/` workspace member).

**Instrumented signals** (from `PPUT_RESULT:{...}` JSON line):
- `prompt_context_hash` — hashed deterministic representation of agent-facing prompt
- `solved` / `verified` / `progress` — Lean ground-truth booleans
- `gp_payload` / `gp_path` / `gp_proof_file` — golden path proof artifact
- `pput_runtime` / `pput_verified` — cost-time-progress metric
- `budget_max_transactions` — env-honored budget
- `tx_count` / `failed_branch_count` / `rollback_count`

**TB-1..TB-5 cross-session invariance**: oneshot `prompt_context_hash="a1f43584a17d1226"` is **bit-identical across 5 sessions** (TB-1 day-1 spike + TB-2 + TB-3 + TB-4 + TB-5 ship smokes). This is the strongest compat signal — proves the agent-facing prompt-build pipeline is structurally untouched by every TB-N kernel edit.

**TB-4 + TB-5 n1**: SOLVED+VERIFIED on `mathd_algebra_107` with `gp_payload="nlinarith"`; proof artifacts re-verify under pinned Lean toolchain v4.24.0.

**Critical caveat (the chaintape gap §0)**: the evaluator emits `PPUT_RESULT` to stdout. Stdout is captured by shell redirect into `*_run.log`. The .log files are bounded by conventional file-system trust, not cryptographic chain trust. **The runs themselves never traverse `Sequencer::apply_one`**. The README in each smoke directory concedes this at the bottom: "What this smoke does NOT prove — that the new dispatch arm is reachable from the evaluator."

### §3.3 Trust Root + governance

- `genesis_payload.toml` lists 40+ load-bearing files; rehashed at every STEP_B-protected file change.
- `cargo test boot::tests::verify_trust_root_passes_on_intact_repo` → ok at HEAD.
- Pre-commit hook enforces `R-022 TRACE_MATRIX backlinks` on `pub fn` items + `R-014 Trust Root rehash`.
- 9 anti-drift scanners across TB-3/4/5 catch resurrection attempts (bridge pattern; 8 forbidden TypedTx variant names).

---

## §4 AI's proposed next-step plan

Two atomic options for TB-6, depending on architect ruling on D1.

### §4.1 Path A (recommended) — TB-6 = P2 Agent Runtime atom

**Goal**: close the §0 gap. Wire `experiments/minif2f_v4/src/bin/evaluator.rs` to drive a real `Sequencer` + persistent `Git2LedgerWriter`. After TB-6 ships, **at least one LLM-driven run produces an on-disk chaintape** that is walkable + replayable post-hoc.

**Charter sketch**:

| Atom | Goal |
|---|---|
| 1 | STEP_B Phase-0 preflight + dual external audit launch (Option A audit mode reinstated for production-wire-up; this is constitutional risk class equivalent to TB-5) |
| 2 | Decide chaintape persistence path: `data/chaintape/` git2 backend? Or fresh repo per run? Anchor in genesis_payload.toml? |
| 3 | Wire evaluator main(): construct `TuringBus::new_with_sequencer()` (currently exists per `bus.rs:120`; never called); pass `Arc<Sequencer>` configured with `Git2LedgerWriter` |
| 4 | Define WHICH PputResult emits become L4 entries (proposal accepts? verifier confirms? challenger emits? all three?) — depends on architect ruling |
| 5 | First chaintape: re-run `mathd_algebra_107` × n1 × deepseek-v4-flash; produce `data/chaintape/<run_id>/` with `LedgerEntry` rows |
| 6 | Replay invariant: `tools/replay-chaintape <path>` reconstructs economic state from L4 alone (extends I29/I80 to disk-backed chain) |
| 7 | Smoke gate evolution: from TB-7 onward, ship-gate smoke MUST produce ≥1 walkable chaintape + ship doc verifies chain integrity |
| 8 | Self-audit + 真实链审 (NEW; replaces 真实烟测 — chain-walking the on-disk evidence) |

**Risks**:
- Chaintape persistence design is genuinely new — needs architect ruling on which evaluator events become L4 entries
- May surface new bugs in the kernel that were latent under in-memory testing
- Iteration-cap (24h) friendly: this atom DOES produce evaluator pass/fail signal that traverses the chain (capability signal native)
- Closes 5-TB cumulative debt in one TB

**Dependencies satisfied**: TB-2..TB-5 all already shipped; the kernel surface is ready.

### §4.2 Path B — TB-6 = RSP-3.2 slash execution (current ROADMAP plan)

**Goal**: extend RSP-N micro-version sequence. `SlashTx` system-emitted; balances/stakes/challenge_cases mutations conditional on `ChallengeCase.status == UpheldDeferred`. Builds on TB-5's UpheldDeferred anchor + emit_system_tx + apply_one stage 1.5.

**Charter sketch**:

| Atom | Goal |
|---|---|
| 1 | STEP_B Phase-0 preflight + audit launch |
| 2 | Substrate: `SlashTx` typed_tx ABI (system-emitted); add to TypedTx enum (11th variant); add to anti-drift FORBIDDEN_VARIANTS removal (charter must lift SlashTx specifically); cascade monetary_invariant.rs |
| 3 | `SlashCommand` + `emit_system_tx` extension; charter-time decision: who triggers slash + when (deadline arithmetic? threshold? challenger-petition?) |
| 4 | Dispatch arm: SlashTx accepted → debit stakes_t[target_work_tx].amount + credit challenger refund + zero ChallengeCase.bond + flip status from UpheldDeferred → Slashed (NEW status); rejected paths (NotUpheldDeferred / SlashAlreadyExecuted / etc.) |
| 5 | Boundary tests: slash must NOT mutate verifier bond (TB-6 Forbidden); idempotency; multi-challenger slash routing |
| 6 | Replay + property + anti-drift CI extension |
| 7 | Self-audit + 真实烟测 (same paper-trail evidence as TB-3/4/5) |

**Risks**:
- Iteration-cap (24h): pure-kernel atom; no new evaluator pass/fail signal — needs explicit cap exception (consistent with TB-5 precedent)
- 5-TB chaintape debt now becomes 6-TB debt; gap widens
- Doesn't close §0 gap; smoke evidence remains paper-trail

**Dependencies satisfied**: TB-5 UpheldDeferred status + emit_system_tx + apply_one stage 1.5.

### §4.3 Hybrid path (if architect prefers)

TB-6 = small P2 wire-up bringing one event onto the chain (no policy decision on which events) + RSP-3.2 deferred to TB-7. Smaller scope than §4.1; bigger than § 4.2 in chaintape impact; smaller in slash discharge.

---

## §5 Decisions I cannot make solo

### D1 — TB-6 sequencing: Path A vs Path B vs Hybrid

**Current ROADMAP § 6 RSP-N sequence** says RSP-3.2 is next (slash). User's recent question 2026-05-01 ("现在 turingos 具有真正的 chaintape 了吗？") strongly suggests the chaintape gap has higher priority in user's view.

**Cannot decide solo because**:
- ROADMAP § 3 P3 ordering vs P2 ordering is a constitutional-level choice (cf. ROADMAP § 11 dependency graph: "P2 Agent Runtime depends on P3 RSP-1, not P1 alone")
- 5-TB cumulative debt vs adding 6th — judgment call about credibility scaling
- iteration-cap-24h memory vs round-cap-2 memory tension if pure-kernel atom continues
- Scope of WP-canonical reconciliation: SlashTx is allowed-named per WP § 19 but currently in tb_5_anti_drift FORBIDDEN_VARIANTS; lifting that needs charter-time architect ruling (similar to how TB-3 lifted bridge invariant for TB-3+ but kept it CI-enforced)

**AI recommendation**: Path A (P2 Agent Runtime atom). 5-TB kernel-only debt is a credibility scaling problem; one wire-up atom converts "trust the cargo test suite" to "trust the on-disk chain". User question signal supports this read.

**Architect ruling needed**: ☐ Path A (P2) / ☐ Path B (RSP-3.2) / ☐ Hybrid (specify) / ☐ alternative

### D2 — Smoke gate evolution (chaintape requirement timeline)

When does ship-gate smoke MUST produce a walkable chaintape (vs paper trail)?

Options:
- **(a)** Soft retroactive: rename "smoke tape" → "smoke evidence" in templates + retroactive (no chain requirement; just honest naming)
- **(b)** Hard from TB-6 if D1=Path A: chaintape required as TB-6 deliverable; from TB-7 onward, ship gate verifies chain integrity
- **(c)** Hard from TB-7: chain wire-up is its own TB (consistent with D1=Path B); rename retroactively; chain requirement starts TB-7+
- **(d)** Drop smoke gate entirely; rely on `cargo test --workspace`

**AI recommendation**: (b) if D1=A; (c) if D1=B.

**Architect ruling needed**: ☐ (a) / ☐ (b) / ☐ (c) / ☐ (d)

### D3 — Audit-mode standard for TB-6+

TB-3/4 used Option B (self-audit + 真实烟测) per user 2026-04-30 authorization. TB-5 reinstated Option A (dual external) per directive § 4 Q4 because system-emitted economic mutators ≠ Option B precedent class. TB-5 actual: Codex-only + grep self-verification fallback (Gemini exhausted).

Options:
- **(a)** Default Option B (self-audit + smoke); flip to Option A only on architect directive
- **(b)** Default Option A Codex-only with grep self-verification fallback as TB-5 supplement legitimized
- **(c)** Default Option A dual external; halt the TB until Gemini capacity returns
- **(d)** Hybrid by constitutional risk class: kernel-only additive atoms = Option B; system-emitted economic mutators / production-wire-up = Option A Codex-only with charter-time supplement

**AI recommendation**: (d). Match audit weight to risk class.

**Architect ruling needed**: ☐ (a) / ☐ (b) / ☐ (c) / ☐ (d) / ☐ alternative

### D4 — Test-count reporting standard

Lock down `cargo test --workspace` as canonical ship-gate test command? Avoids future TB-5-style 464/617 under-counts.

**AI recommendation**: yes; codify in memory `feedback_phased_checkpoint`.

**Architect ruling needed**: ☐ yes / ☐ no / ☐ different command (specify)

### D5 — "Smoke tape" → "smoke evidence" rename

Rename across charter templates + LATEST/NOTEPAD/audit doc references? Preserve "tape"/"chaintape" exclusively for the LedgerEntry chain (when it exists on-disk from a production run).

**AI recommendation**: yes.

**Architect ruling needed**: ☐ yes / ☐ no / ☐ partial

### D6 — Memory updates required

Per the audit findings, AI memories (in `~/.claude/projects/.../memory/`) should be updated. Architect approval requested for:

- **NEW** `feedback_workspace_test_canonical.md` — `cargo test --workspace` mandatory for ship-gate counts (D4)
- **NEW** `feedback_smoke_evidence_naming.md` — "smoke tape" reserved for chain-backed evidence; default term is "smoke evidence" (D5)
- **NEW** `feedback_chaintape_wire_up_priority.md` — production-binary chaintape wire-up is a ship-gate honesty requirement; cumulative debt monitored (D1+D2)
- **UPDATE** `feedback_dual_audit.md` — add Codex-only-with-grep-fallback as legitimate degraded path when Gemini strategic-tier exhausted (D3)
- **UPDATE** `feedback_iteration_cap_24h.md` — clarify which TBs can claim cap exception (kernel-only atoms with explicit charter justification) and which cannot (production-wire-up atoms must produce capability signal native)

**Architect ruling needed**: ☐ approve all 5 / ☐ approve subset (specify) / ☐ reject

### D7 — Constitution amendment (if any)

The chaintape gap surfacing is **not a constitutional violation** — Anti-Oreo Art V.1.3 + WP § 12.4 are about agent-vs-system separation, not about whether kernel runs in production. The kernel correctly enforces Anti-Oreo when invoked. The gap is in HOW OFTEN it's invoked.

But: should constitution add a new article codifying "production-binary structural audit"? Or is this a roadmap-level concern only?

**AI recommendation**: roadmap-level. No constitution amendment. ROADMAP § 3 P2 already covers this; just needs sequencing acceleration via D1.

**Architect ruling needed**: ☐ roadmap-level only / ☐ constitution amendment (specify) / ☐ ROADMAP edit (specify)

---

## §6 Output requested

A binding directive at `handover/directives/2026-05-XX_TB6_DIRECTIVE.md` with rulings on **D1–D7**, analogous to the TB-5 directive shape. AI will:

1. Codify rulings into TB-6 charter v1 (filename: `handover/tracer_bullets/TB-6_charter_<date>.md`)
2. Update ROADMAP § 3 P3 + § 6 RSP-N sequence to reflect ruling
3. Update memories per D6
4. Begin TB-6 atom 1 (STEP_B Phase-0 preflight) on a fresh experiment branch

If architect needs more evidence before ruling, list specific files / signals + AI will produce.

---

## §7 Audit caveats (transparency)

- This prompt was authored by the same AI that shipped TB-5; treat as self-reported state.
- Disk near-full at audit time (417MB free); all checks performed against existing local state without target/ rebuild from scratch.
- Lean toolchain pinned to v4.24.0 (matches `minif2f_data_lean4/lean-toolchain` exactly) for re-verification.
- No external auditor invoked per user instruction "由你负责审计，不需要外审"; this prompt formally requests architect review as escalation path.
