# External Audit Prompt — TuringOS v4 TB-4 Ship Review + TB-5 Charter Soundness

**Date prepared**: 2026-04-30
**Repo HEAD**: `1b60237` (`origin/main`)
**Audit subject**: backward-looking soundness of TB-4 (P3 RSP-2 Verifier Bond + Challenger NO Stake) + forward-looking alignment of TB-5 charter v1 (P3 RSP-3.1 Challenge Resolve / Bond Release)
**Session**: post-TB-4 ship 2026-04-30 + medium-difficulty real-question batch validation
**Intended audiences**: Codex (implementer-paranoid lens) + Gemini (strategic / WP-canonical alignment lens). Each receives the full prompt + reads the listed files; merged verdict per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).

---

## 1. Audit scope (two parts)

### Part A — TB-4 ship soundness (backward)

You are auditing whether the TB-4 ship at HEAD `1b60237` (merge `edb8089`; ship_commits `cfc81de..a17d477`) is constitutional, line-grounded, and free of WP-canonical drift. Specifically:

- (A1) Does TB-4's implementation match its charter v2 § 3 ten decision blocks line-for-line in src + tests?
- (A2) Are TB-4's 20 forbidden lines (charter § 5) actually enforced (either by code construction OR by acceptance tests) without exception?
- (A3) Are the three TB-4 ship proofs (charter § 8) demonstrably green via the 30 new TB-4 tests?
- (A4) Does the TB-4 architect directive (`handover/directives/2026-04-30_TB4_directive.md`) — 7 Q-decisions + 5 anti-drift clauses — appear correctly applied in code + charter?
- (A5) Is the WP-canonical reconciliation rule (`feedback_wp_vs_roadmap_reconciliation`) preserved? Specifically: zero `NoStakeTx` / `VerifierBondTx` / `ChallengeStakeTx` / `VerifierStakeTx` literals in src/, AND the I44 anti-drift CI scanner is genuine (not a stub).
- (A6) Are there any **silent regressions** vs TB-3 (the predecessor)? Specifically: bridge-resurrection invariant still GREEN; 5-holding CTF preserved; 9-sub-field EconomicState invariant preserved.

### Part B — TB-5 charter v1 alignment (forward)

You are auditing whether the TB-5 charter v1 plan (`handover/tracer_bullets/TB-5_charter_2026-04-30.md`) is sound, narrowly scoped, and free of inflation drift. Specifically:

- (B1) Is `ChallengeResolveTx` correctly classified as a **first-class allowed-named** TypedTx variant (per WP § 19 + ROADMAP § 3 P3 list line 178), NOT a phantom-variant forbidden by `feedback_wp_vs_roadmap_reconciliation`?
- (B2) Is the system-emitted shape (mirror `FinalizeRewardTx` / `TaskExpireTx` / `TerminalSummaryTx`) correct — `system_signature` + `SystemEpoch`, `HasSubmitter::submitter_id() = None`?
- (B3) Are the two entry-shape additives (`StakeEntry +accepted_at_round`, `TaskMarketEntry +challenge_window_length`) genuinely additive (serde-default) and 9-sub-field-invariant-safe?
- (B4) Is the Released vs UpheldDeferred split correctly atomic (Released = active CTF round-trip; UpheldDeferred = typed marker with zero Q_t mutation)?
- (B5) Are TB-5's 28 forbidden lines (charter § 5; TB-4's 20 inherited + 8 new) drawing the right RSP-3.1 / RSP-3.2 / RSP-4 boundary? Identify any boundary creep (e.g., is TB-5 secretly half-implementing slash, or pre-empting RSP-4 settlement)?
- (B6) The seven OPEN scope-review questions (charter § 10 Q1-Q7) — for each one, do you concur with the charter v1 recommendation, or do you propose an alternative? Be specific.
- (B7) Does the audit-mode recommendation (Option B = self-audit + 真实烟测, mirror TB-4 precedent) appropriately balance cost vs. drift-risk for TB-5's institutional-change size?
- (B8) Identify any **unstated dependencies**: e.g., does TB-5 actually need anything from TB-3 / TB-4 that's not currently in src? Is there a hidden gating constraint we missed (e.g., does the dispatch arm need `q.q_t.current_round` advancement we forbid)?

---

## 2. Project context (read this section in full before touching files)

**Project**: TuringOS v4 — Silicon-Native Microkernel for LLM Formal Verification Swarm. Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4. Monorepo at `github.com/gretjia/turingosv4`.

**Top-level constitutional sources**:
- Constitution: `constitution.md` (project root). Articles I-V + Laws + Boot.
- 9-phase roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` (post-2026-04-29 architect directive). P0 → P1 → P2/P3/P4 → P5 → P6 → P7 → P8 → P9. P3 (RSP Economy Core) is the most critical phase per the directive.
- WP architecture: `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` — 反奥利奥架构 (Anti-Oreo) + ChainTape + RSP economy.
- WP economic supplement: `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md` — 12 economic invariants (§ 18) + 9 RSP-1 modules (§ 19) + 21 final reward formula.

**RSP-N micro-version sequence** (P3 internal, ROADMAP § 6):
```
RSP-0  on_init + balances + monetary_invariant            (TB-1 ✓ shipped)
RSP-1  task escrow + work_tx + yes_stake (formal surface) (TB-3 ✓ shipped 2026-04-30 e99b158)
RSP-2  verifier + challenge_tx + no_stake                 (TB-4 ✓ shipped 2026-04-30 edb8089)
RSP-3  challenge window + slash + provisional reward      (TB-5 charter v1 = RSP-3.1 first slice)
RSP-4  Contribution DAG + settlement_tx
RSP-5  deferred impact bonus + reuse royalty
RSP-6  price index + risk market
RSP-7  public settlement adapter (= P7 entry)
```

**Selection rule** (ROADMAP § 11): "lowest-numbered phase with a RED kill criterion wins." P3:9 (失败 Solver slash) is the only remaining RED kill criterion in P3 post-TB-4; RSP-3 is its phase; TB-5 is the first RSP-3 slice.

**WP-canonical reconciliation rule** (codified TB-3, re-applied TB-4):

> The ROADMAP § 3 transaction list mixes "first-class TypedTx variants" with "semantic roles of existing fields." On data-structure questions, WP implementation shape wins:
>
> - **Allowed-named variants** (named verbatim AND have no inline-field equivalent): `task_open_tx`, `escrow_lock_tx`, `verify_tx`, `challenge_tx`, `challenge_resolve_tx`, `provisional_accept_tx`, `settlement_tx`, `slash_tx`, `reputation_update_tx`. Each gets a first-class `TypedTx` variant.
> - **Forbidden-phantom variants** (semantic roles of existing inline fields): `yes_stake_tx` ↔ `WorkTx.stake` (TB-3); `no_stake_tx` ↔ `ChallengeTx.stake` (TB-4); `verifier_bond` ↔ `VerifyTx.bond` (TB-4). NO new TypedTx variant for these — they ride existing inline fields.
> - CI-enforced by `tests/tb_4_rsp2_admission_surface.rs::no_no_stake_tx_or_verifier_bond_tx_variant_in_src` (I44).

**Memory rules in scope** (auditor doesn't read user memory directly; summarized inline):
- `feedback_wp_vs_roadmap_reconciliation`: above rule.
- `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS conservative-merge.
- `feedback_iteration_cap_24h`: every PR must produce evaluator pass/fail signal in 24h; spec/audit not shortest-path = default-reject.
- `feedback_no_fake_menus`: when project plan determines next atom, state and execute; no 3-5 option menus.
- `feedback_phased_checkpoint`: paired N=20 A/B + checkpoint doc + 7 red-line check between phases.
- `feedback_elon_mode_policy`: round-cap=2 + auto-execute on determinate-best surgical patch.
- `feedback_tb_phase_tag_required`: every TB charter MUST declare phase_id + roadmap_exit_criteria_addressed + kill_criteria_tested.

---

## 3. Files to read (organized by purpose)

All paths are repo-root-relative. Read top-down within each subsection; the order is "trust-anchor first, then derived."

### 3.1 Constitution & roadmap (read FIRST; ground truth for every other doc)

```
constitution.md                                                          # Articles + Laws + Boot
CLAUDE.md                                                                # project conventions + standards
handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md                # 9-phase canonical roadmap (post-audit § 11)
handover/directives/2026-04-29_9_phase_roadmap.md                        # verbatim architect directive (the directive that produced ROADMAP)
handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md   # WP architecture
handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md       # WP economic § 18 Invariants + § 19 RSP-1 modules
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md      # L4 / L4.E split decision (TB-3/TB-4 inherited)
cases/C-031_institution_over_tuning.yaml                                 # institution > tuning principle
cases/V3_LESSONS.md                                                      # v3 case-law map
```

### 3.2 TB-3 / TB-4 charters + directives (binding context for TB-4 ship + TB-5 plan)

```
handover/tracer_bullets/TB_LOG.tsv                                       # 5-row history: TB-0..TB-4
handover/tracer_bullets/TB-3_charter_2026-04-30.md                       # RSP-1 formal-tx-surface charter (TB-3 ✓ shipped)
handover/tracer_bullets/TB-4_charter_2026-04-30.md                       # RSP-2 charter v2 (TB-4 ✓ shipped); 10 decision blocks + 20 forbidden + 3 ship proofs
handover/directives/2026-04-30_TB4_directive.md                          # TB-4 architect directive: 7 Q-decisions + 5 anti-drift clauses + 9-atom plan
handover/ai-direct/TB-4_RSP2_ADMISSION_SURFACE_2026-04-30.md             # TB-4 STEP_B Phase-0 preflight (line-grounded snippets vs main HEAD pre-TB-4)
handover/ai-direct/TB-3_RSP1_FORMAL_TX_SURFACE_2026-04-30.md             # TB-3 STEP_B Phase-0 preflight (template TB-4 mirrored)
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md                              # research notepad — TB-3 + TB-4 SHIPPED log sections; TB-5 charter v1 section
```

### 3.3 TB-4 source-code touch surface (the actual implementation)

```
src/state/typed_tx.rs                                                    # schema bumps: VerifyTx + ChallengeTx +parent_state_root field#2; signing payload 6→7 fields; 4 new TransitionError (BondInsufficient, TargetWorkInactive, EmptyCounterexample) + 2 reserved; golden digest rotations
src/state/q_state.rs                                                     # ChallengeCase +target_work_tx (additive serde-default)
src/state/sequencer.rs                                                   # 2 new state-root domain consts (VERIFY_ACCEPT_DOMAIN_V1, CHALLENGE_ACCEPT_DOMAIN_V1) + 2 helpers + Verify dispatch arm + Challenge dispatch arm + 10 in-crate unit tests U12-U21; rejection_class_for + public_summary_for table extension
genesis_payload.toml                                                     # Trust Root manifest rehash (R-014; non-sudo per R-018) — sequencer/typed_tx/q_state SHA updated 4x across atoms
tests/tb_4_rsp2_admission_surface.rs                                     # 12 integration tests I31-I44 + 1 anti-drift CI scanner + 1 positive-control + 4 helper fns
tests/tb_3_bridge_deletion_invariant.rs                                  # TB-3 inherited CI invariant (must stay GREEN at TB-4 HEAD)
tests/tb_3_rsp1_formal_surface.rs                                        # TB-3 inherited; reference for replay/property test patterns
tests/tb_2_runtime_boundary.rs                                           # TB-2 inherited; reference for Sequencer harness shape
tests/economic_state_reconstruct.rs                                      # ChallengeCase init updated (test fixture only; one-line)
```

### 3.4 TB-4 audit + smoke evidence (what the ship was gated on)

```
handover/audits/RECURSIVE_AUDIT_TB_4_2026-04-30.md                       # TB-4 self-audit: 7/7 directive Q-decisions + 5/5 anti-drift + 10/10 charter §3 + 20/20 §5 + 3/3 ship proofs all line-grounded
handover/audits/RECURSIVE_AUDIT_TB_3_2026-04-30.md                       # TB-3 self-audit precedent (TB-4's audit mirrors this shape)
handover/evidence/tb_4_smoke_2026-04-30/README.md                        # ship-gate smoke (oneshot bit-identical + n1 SOLVED)
handover/evidence/tb_4_smoke_2026-04-30/oneshot_run.log                  # raw oneshot evaluator stdout
handover/evidence/tb_4_smoke_2026-04-30/n1_run.log                       # raw n1 evaluator stdout (SOLVED + VERIFIED)
handover/evidence/tb_4_smoke_2026-04-30/proof_n1.lean                    # CAS-stable proof artifact (lean --stdin re-verifiable)
```

### 3.5 TB-4 medium-difficulty real-question batch (post-ship capability validation)

```
handover/evidence/tb_4_medium_batch_2026-04-30/README.md                 # 5-problem batch report: configuration + per-problem table + aggregate + verdict
handover/evidence/tb_4_medium_batch_2026-04-30/batch_results.jsonl       # 5 PPUT_RESULT v2.0 rows (one per problem)
handover/evidence/tb_4_medium_batch_2026-04-30/mathd_algebra_107.log     # SOLVED tx=1 nlinarith pput_m=33.6
handover/evidence/tb_4_medium_batch_2026-04-30/mathd_algebra_125.log     # SOLVED tx=1 nlinarith pput_m=209.4
handover/evidence/tb_4_medium_batch_2026-04-30/mathd_algebra_141.log     # SOLVED tx=1 nlinarith pput_m=236.9
handover/evidence/tb_4_medium_batch_2026-04-30/mathd_algebra_148.log     # SOLVED tx=23 (composite tactic; 22 failed branches) pput_m=0.42
handover/evidence/tb_4_medium_batch_2026-04-30/amc12a_2003_p5.log        # MAX_TX EXHAUSTED tx=30 hit_max_tx=true (expected hard-problem failure)
handover/evidence/tb_4_medium_batch_2026-04-30/proof_*.lean              # 4 CAS-stable proof artifacts (re-verifiable)
handover/evidence/tb_4_medium_batch_2026-04-30/aggregate.sh              # log → jsonl aggregator (re-runnable)
```

### 3.6 TB-5 charter v1 (the next-phase plan you are auditing)

```
handover/tracer_bullets/TB-5_charter_2026-04-30.md                       # DRAFT v1: ChallengeResolveTx scheme + 10 decision blocks + 28 forbidden + 3 ship proofs + 7 OPEN scope-review questions
```

### 3.7 Reproducibility anchors (test-time signals to verify)

Run this in repo root:

```bash
cargo test --workspace 2>&1 | grep "^test result:" | awk '{p+=$4; f+=$6} END {print "PASS="p, "FAIL="f}'
# Expected: PASS=571 FAIL=0
```

Run this to re-verify 4 of the medium-batch proofs (requires Lean 4 + Mathlib environment):

```bash
cd /path/to/mathlib4
LEAN_PATH=$(lake env printenv LEAN_PATH) lean --stdin < /path/to/turingosv4/handover/evidence/tb_4_medium_batch_2026-04-30/proof_mathd_algebra_125.lean
# Expected: zero diagnostics; proof typechecks
```

Run this to confirm Trust Root manifest matches code:

```bash
sha256sum src/state/sequencer.rs src/state/typed_tx.rs src/state/q_state.rs
# Compare to genesis_payload.toml lines 227-229; must match
```

---

## 4. Real smoke test details (Section 3.4 + 3.5 deep dive)

### 4.1 TB-4 ship-gate smoke (`handover/evidence/tb_4_smoke_2026-04-30/`)

**Configuration**:
```
binary    = ./target/debug/evaluator (built post-Atom-7; commit bbe2d16)
mode      = full
proxy     = LLM_PROXY_URL=http://localhost:8080 (deepseek-v4-flash via LLM proxy)
```

**Run 1 — `CONDITION=oneshot`** (pipeline-liveness):
- Problem: `mathd_algebra_107`
- `MAX_TRANSACTIONS=20` (env-set; clamped to 1 by oneshot driver per single-proposal semantics)
- Result: `solved=false` (oneshot prompt-template regression at HEAD; documented in TB-1 / TB-2 / TB-3 / TB-4 evidence READMEs)
- **Strict structural invariant assertion**: `prompt_context_hash="a1f43584a17d1226"` is **bit-identical** to:
  - TB-1 Day-1 spike (2026-04-29; commit `f0b659f`-class)
  - TB-2 ship smoke (2026-04-30; commit `cf32735`)
  - TB-3 ship smoke (2026-04-30; commit `2eee4ee`)
  - **TB-4 ship smoke** (this run; commit `bbe2d16`)
- 4 sessions, bit-identical hash → strongest possible pipeline-isolation signal: every TB-4 institutional change is on the runtime spine + state schema; the agent-facing prompt build pipeline is **structurally untouched**.

**Run 2 — `CONDITION=n1`** (capability replication; elevated MAX_TX):
- Problem: `mathd_algebra_107`
- `MAX_TRANSACTIONS=20`
- Result: **SOLVED + VERIFIED**
  - `solved=true`, `verified=true`, `progress=1`
  - `pput_runtime=0.000211537...` (bit-identical to TB-0 baseline at `mathd_algebra_107` first v4-native solve)
  - `gp_payload="nlinarith"` (canonical OMEGA proof)
  - `golden_path_token_count=12`, `total_run_token_count=448`
  - `budget_max_transactions=20` (env honored)
  - `hit_max_tx=false` (solved on first tx; budget was safety ceiling not binding constraint)
  - `total_wall_time_ms=10552` (~10s)
- Proof artifact at `proof_n1.lean` is CAS-stable: `lean --stdin` typechecks zero diagnostics

### 4.2 TB-4 medium-difficulty batch (`handover/evidence/tb_4_medium_batch_2026-04-30/`)

**Configuration**:
```
binary           = ./target/debug/evaluator (post-TB-4 ship; main HEAD 6c42cf7)
mode             = full
CONDITION        = n1 (single-agent multi-tx; lets MAX_TX budget actually exercise multi-step search)
MAX_TRANSACTIONS = 30 (4× TB-3 ship-gate; 1.5× TB-4 ship-gate)
per-prob timeout = 600 s (coreutils timeout)
proxy            = LLM_PROXY_URL=http://localhost:8080
```

**Problem set** (5 problems from pre-registered adaptation split, mixed difficulty):
```
1. mathd_algebra_107      EASY      canonical baseline (TB-0 first v4 solve)
2. mathd_algebra_125      EASY-MED  linear nat; small case x=6
3. mathd_algebra_141      MED       (a+b)² - 2ab = a² + b² identity
4. mathd_algebra_148      MED       cubic at point; needs eval substitution f(2)=9
5. amc12a_2003_p5         MED-HARD  multi-digit decomposition; AMC-style
```

**Per-problem results**:
| Problem | solved | tx_count | failed_branch | hit_max_tx | pput_m | tactic_diversity | wall_s | gp_payload |
|---|---|---|---|---|---|---|---|---|
| mathd_algebra_107 | ✅ true | 1 | 0 | false | 33.61 | 1.00 | 66.4 | `nlinarith` |
| mathd_algebra_125 | ✅ true | 1 | 0 | false | 209.38 | 1.00 | 10.3 | `nlinarith` |
| mathd_algebra_141 | ✅ true | 1 | 0 | false | 236.90 | 1.00 | 9.6 | `nlinarith` |
| mathd_algebra_148 | ✅ true | **23** | **22** | false | 0.42 | 0.13 | 211.7 | `rw [h₀ 2] at h₁; nlinarith` |
| amc12a_2003_p5 | ❌ false | **30** | 30 | **true** | 0.00 | 0.10 | 400.3 | (none) |

**Aggregate** (per Art. I.2 + C-052/C-053/C-057 main-metric requirements):
```
ΣPPUT_m_verified      = 480.31
Mean PPUT_m_verified  = 120.08 (over n=4 SOLVED only)
Solve rate            = 4/5 = 80%
Wilson 95% CI         = [0.38, 0.96]   (wide; n=5 small-sample)
Total wall time       = 698.4s (~11.6 min)

halt_reason_distribution (Art. IV):
  OmegaAccepted=4  MaxTxExhausted=1  WallClockCap=0  ComputeCapViolated=0  ErrorHalt=0

Multi-agent statistics (Art. II.2.1):
  CONDITION=n1 ⇒ N/A (parent_selection_entropy + pairwise_payload_diversity require n≥2)
```

**Key signals to audit**:

1. **mathd_algebra_148**: 23-tx multi-step search converged on a composite tactic (`rw [h₀ 2] at h₁\nnlinarith`) after 22 failed branches. `tactic_diversity=0.13` evidences focused-but-iterative search (model converged, didn't randomly hop). This is the strongest validation that the elevated MAX_TX budget actually flows through `dispatch_transition` + `apply_one` + the reactor loop without being short-circuited.

2. **amc12a_2003_p5**: hit `MAX_TX=30` cleanly with `solved=false`, `hit_max_tx=true`, `tx_count=failed_branch_count=30=MAX_TRANSACTIONS`. No false-positive solve, no system crash, no L4.E corruption. **Expected hard-problem failure mode preserved** (this is a property — the system fails safely on hard inputs, doesn't corrupt state).

3. **Capability replication**: 4 distinct problems produce CAS-stable proofs re-verifiable via `lean --stdin`. The TB-4 ABI changes (parent_state_root schema bumps + ChallengeCase additive + 4 new TransitionError variants + 2 new state-root domains + Verify/Challenge dispatch arms) are serde-compatible across diverse problems.

**What the batch does NOT prove** (auditors must NOT credit these):
- TB-4 RSP-2 admission spine is **NOT reachable** from the evaluator's PPUT emit path. The evaluator solve path is currently pre-runtime; TB-4's Verify/Challenge dispatch arms are exercised only by the in-crate + integration test battery (30 new TB-4 tests under `cargo test --workspace`).
- This is by design per TB-4 charter § 5 #1 + ROADMAP § 11 P2 dependency graph: P2 Agent Runtime is the phase that wires evaluator → Sequencer::submit. TB-4 ships the runtime; P2 ships the wiring.

---

## 5. Audit questions (Part A — TB-4 ship)

Answer each with: **PASS** / **CHALLENGE [reason + remediation]** / **VETO [structural blocker + required redesign]**.

**A1.** Verify TB-4 charter v2 § 3 ten decision blocks are all line-grounded:
- 3.1 WP-canonical (no NoStakeTx/VerifierBondTx) → tests/tb_4_rsp2_admission_surface.rs I44 + src/state/typed_tx.rs TypedTx enum (9 variants, NOT 10+)
- 3.2 9-sub-field invariant + 5-holding CTF preserved → src/state/q_state.rs EconomicState struct + src/economy/monetary_invariant.rs unchanged
- 3.3 ChallengeCase additive `target_work_tx` → src/state/q_state.rs:333-365
- 3.4 Verify admission steps 1-7 → src/state/sequencer.rs Verify arm
- 3.5 Challenge admission steps 1-9 → src/state/sequencer.rs Challenge arm
- 3.6 No status field on WorkTx → src/state/typed_tx.rs:222-236 unchanged
- 3.7 Slashing 100% out of scope → no DELETE/REMOVE on stakes_t / challenge_cases_t in dispatch arms
- 3.8 Three-class error taxonomy → src/state/typed_tx.rs TransitionError + sequencer.rs rejection_class_for
- 3.9 Window-only-anchor (no closure) → no `current_round - opened_at_round` arithmetic anywhere in src
- 3.10 VerifyTx is signal+stake NOT judge → Verify arm reads `verify.verdict` zero times

**A2.** Verify TB-4 charter v2 § 5 twenty forbidden lines: pick five at random and check the implementation site OR the boundary-test that enforces each. Especially #5 (no new TypedTx variants → I44 CI scanner), #10 (no L4.E mutation of economic_state → tb_4 I40), #11 (no challenge-window CLOSURE logic → grep src/ for `current_round.*opened_at_round` arithmetic).

**A3.** Verify the three TB-4 ship proofs (charter § 8) via the 30 new TB-4 tests:
- Proof 1: I31, I33, I35, I37 cover verifier bond admission spine
- Proof 2: I32, I34, I36, I38, I39, I40 cover challenger NO admission + multi-challenger + L4.E-no-mutation
- Proof 3: I41, I42, I43, I44 cover replay + property + window-anchor + anti-drift CI

**A4.** Verify the TB-4 architect directive (`handover/directives/2026-04-30_TB4_directive.md`) clauses are all addressed:
- Q1 DEFER (no idempotency dedup) → I42 step 8-9 (same agent multiple admissions)
- Q2 ACCEPT Option A (parent_state_root schema bump) → typed_tx.rs:240-250 + 269-280
- Q3 NEW TargetWorkInactive (3-class taxonomy) → typed_tx.rs TransitionError + T5
- Q4 Multi-challenger explicit test → I39
- Q5 Audit mode → § 9 narrowed Option A (ship audit replaced by self-audit + smoke per TB-3 precedent)
- Q6 ReputationsIndex untouched → grep src/ for `reputations_t` mutation in dispatch arms (zero hits)
- Q7 EmptyCounterexample variant → typed_tx.rs TransitionError + sequencer.rs Challenge arm step 5 + tb_4 U20

**A5.** Verify the WP-canonical reconciliation rule:
- Run: `grep -rn "NoStakeTx\|VerifierBondTx\|ChallengeStakeTx\|VerifierStakeTx" src/` (expected: zero hits)
- Verify I44 scanner is genuine: read `tests/tb_4_rsp2_admission_surface.rs::no_no_stake_tx_or_verifier_bond_tx_variant_in_src` + `no_drift_scanner_positive_control_finds_known_match` (positive control ensures the scanner actually traverses files)

**A6.** Identify silent regressions:
- Run `cargo test --test tb_3_bridge_deletion_invariant` → expected GREEN (TB-3 invariant must hold at TB-4 HEAD)
- Verify 9-sub-field invariant via existing test `economic_state_has_nine_sub_fields` (in q_state.rs::tests)
- Verify 5-holding CTF: read `monetary_invariant.rs::total_supply_micro` (must sum 5 holdings, not 6 or 4)

---

## 6. Audit questions (Part B — TB-5 charter v1)

Answer each with: **PASS** / **CHALLENGE [reason + suggested charter v2 amendment]** / **VETO [structural blocker; charter must be redesigned]**.

**B1.** Is `ChallengeResolveTx` correctly classified as **first-class allowed-named** (NOT phantom)?

Check: WP § 19 RSP-1 ChallengeCourt module names "挑战期 + 反例 + 冻结 + 回滚 + slash"; ROADMAP § 3 P3 transactions list (line 178) names `challenge_resolve_tx` verbatim parallel to `task_open_tx` / `escrow_lock_tx` / `verify_tx` / `challenge_tx` / `provisional_accept_tx` / `settlement_tx` / `slash_tx` / `reputation_update_tx`. No existing inline-field equivalent (ChallengeResolveTx semantics — resolve a specific ChallengeCase — has no "ride existing field" alternative).

**B2.** Is the system-emitted shape correct?

Check charter § 3.2 + § 3.4 against:
- `src/state/typed_tx.rs:312-323` (FinalizeRewardTx — system-emitted reference shape)
- `src/state/typed_tx.rs:328-337` (TaskExpireTx — second reference)
- `src/state/typed_tx.rs:794-804` (HasSubmitter for FinalizeRewardTx — returns `None`)

**B3.** Are the entry-shape additives genuinely additive?
- `StakeEntry +accepted_at_round: u64` with `#[serde(default)]` — does NOT add to EconomicState 9 sub-fields
- `TaskMarketEntry +challenge_window_length: u64` with `#[serde(default = "task_market_default_window_len")]` (default 10) — does NOT add to EconomicState 9 sub-fields
- Pre-TB-5 has zero accepted ChallengeResolveTx rows → forward-compat hygiene only

**B4.** Released vs UpheldDeferred split atomicity:
- Released: balances += bond; remove ChallengeCase (CTF round-trip closes)
- UpheldDeferred: ZERO Q_t mutation (just emits a typed marker on canonical L4 row)
- Charter § 3.6: Released does NOT release Solver/Verifier stakes, NOT decrement total_escrow, NOT touch claims/reputations/royalty/price (RSP-3.x / RSP-4 territory)

Is this split clean? Identify any boundary creep.

**B5.** TB-5 forbidden list (28 lines) — RSP-3.1 / RSP-3.2 / RSP-4 boundary check. Specifically:
- #16 (no slash exec) is the RSP-3.2 boundary
- #21 (no window-closure deadline check) is the RSP-3.2 boundary (window-closure math TB-6 will install)
- #22 (no auto round-tick) is the P2 boundary
- #23, #24 (no verifier-bond-release / no solver-stake-release on resolve) is the RSP-3.x / RSP-4 boundary
- #25 (no predicate-eval-of-counterexample at resolve time) is the RSP-3.x runtime-upgrade boundary

Identify any boundary creep — does TB-5 secretly half-implement slash, settlement, reputation, attribution, or window-closure?

**B6.** Seven OPEN scope-review questions (charter § 10 Q1-Q7):
- **Q1** `challenge_window_length` source — global default 10? or TaskOpenTx schema bump?
- **Q2** `accepted_at_round` set on WorkTx YES stake? Verifier bond? BOTH?
- **Q3** `SystemSignatureForbiddenAtAgentSubmit` — keep as RESERVED or DROP?
- **Q4** ChallengeResolution enum vs bool?
- **Q5** Audit mode A vs B?
- **Q6** UpheldDeferred also remove ChallengeCase (or keep for TB-6 slash target)?
- **Q7** Add `ProvisionalAcceptTx` in TB-5 to make Solver provisional state explicit?

For each: do you concur with charter v1 recommendation, or propose alternative? If alternative, justify with reference to WP § / ROADMAP § / existing memory rule.

**B7.** Audit-mode recommendation (Option B = self-audit + 真实烟测):
- TB-5 institutional change: 1 new TypedTx variant + 2 entry-shape additives + 1 dispatch arm
- TB-4 institutional change: 0 new variants + 2 schema bumps + 1 entry-shape additive + 2 dispatch arms
- TB-5 is SMALLER than TB-4 by aggregate ABI surface
- TB-4 used Option B (self-audit + 真实烟测) successfully (per RECURSIVE_AUDIT_TB_4_2026-04-30.md)
- Concur with Option B for TB-5? Or recommend Option A (narrow dual external audit) for institutional-discipline rebalancing?

**B8.** Unstated dependencies:
- Does TB-5's ChallengeResolveTx dispatch arm need `q.q_t.current_round` to advance? (Check § 3.7 — TB-5 deliberately does NOT advance current_round; tests set it manually)
- Does TB-5 need anything from TB-3/TB-4 not currently in src? (Check ChallengeCase shape after TB-4 — does it have everything ChallengeResolveTx needs?)
- Is there any Lean / Mathlib dependency change implied? (Check evaluator binary path; should be unchanged)
- Is there any P5/P6/P7/P8 implicit dependency? (Charter forbidden #28 explicitly excludes; verify by grep)

---

## 7. Verdict format

Each auditor returns:

```
## Part A (TB-4 ship)
A1: PASS | CHALLENGE [...] | VETO [...]
A2: ...
A3: ...
A4: ...
A5: ...
A6: ...

## Part B (TB-5 charter v1)
B1: ...
B2: ...
B3: ...
B4: ...
B5: ...
B6:
  Q1: PASS | CHALLENGE [...]
  Q2: ...
  ...
  Q7: ...
B7: ...
B8: ...

## Overall verdict
PART A: PASS | CHALLENGE | VETO
PART B: PASS | CHALLENGE | VETO

## Top-3 must-fix items (ordered by severity)
1. [...]
2. [...]
3. [...]

## Optional improvements (charter v2 candidates; not blockers)
- [...]
```

Conservative-merge rule (per `feedback_dual_audit_conflict`): if Codex says VETO and Gemini says PASS on the same item, the merged verdict is VETO. Round-cap = 2 narrow re-runs allowed for unanimous-CHALLENGE → revised-charter cycles per `feedback_elon_mode_policy`.

---

## 8. What to deliver back

For each part:
- **Part A merged verdict**: PASS / CHALLENGE / VETO + top-3 must-fix items if not PASS
- **Part B merged verdict**: PASS / CHALLENGE / VETO + per-Q-decision recommendation for each of Q1-Q7 + audit-mode recommendation (A vs B)
- **Charter v2 amendment list**: concrete one-line edits to apply to `handover/tracer_bullets/TB-5_charter_2026-04-30.md` if PART B is CHALLENGE

If PART A is PASS and PART B is PASS-or-narrow-CHALLENGE: TB-5 enters Phase-0 preflight + atom-by-atom ship pattern (mirror TB-4).

If PART A is CHALLENGE: identify the regression source (commit hash from `cfc81de..a17d477`) and propose remediation patch.

If PART A is VETO: TB-4 ship is recalled; main HEAD reverts to TB-3 ship `da4c67a` until remediation is verified.

If PART B is VETO: TB-5 charter is redesigned from scratch; user issues a new directive supplementing `2026-04-30_TB4_directive.md` to address the structural blocker.

---

## 9. Reproducibility note (per C-012 / C-016 / C-039)

The 4 SOLVED proofs in the medium batch (`handover/evidence/tb_4_medium_batch_2026-04-30/proof_*.lean`) are CAS-stable Lean 4 source files. Independent re-verification is the **measurement-correctness anchor** (C-012) — proofs stand or fall on Lean 4's typechecker, not on TuringOS-internal claims. If you can run Lean + Mathlib, please re-verify ≥ 1 proof and report back.

`cargo test --workspace` at HEAD `1b60237` is the **acceptance-correctness anchor** — 571 / 571 PASS / 0 FAIL across 43 test suites, 30 of which are new TB-4 tests. If you can build + test, please report your local pass count.

`prompt_context_hash="a1f43584a17d1226"` bit-identical across **4 sessions** is the **runtime-spine-isolation anchor** — the agent-facing prompt build pipeline is structurally untouched by every TB-N institutional change (TB-1 → TB-2 → TB-3 → TB-4). Verifiable by running the oneshot smoke independently.

---

## 10. Project conventions reminder (auditor norms)

- Be conservative on slash / settlement / reputation / attribution surfaces. Per Constitution Art V.1.3 + WP § 12.4: only the system-keypair-controlled top-white-box can mutate stake holdings; agent-driven slash is a backdoor.
- Per WP-canonical reconciliation: prefer existing inline fields over new TypedTx variants when WP names BOTH a tx and an inline field that could express the same role.
- Per `feedback_no_fake_menus`: when project plan determines next atom, state and execute; do NOT surface 3-5 option menus with a recommended pick.
- Per `feedback_phased_checkpoint`: paired N=20 A/B + checkpoint doc + 7 red-line check between phases. Auto-pause at each gate.
- Per `feedback_iteration_cap_24h`: every PR must produce evaluator pass/fail signal in 24h. Spec/audit not shortest-path to runnable feedback loop = default-reject.

---

## 11. Cross-references

- TB-4 ship merge commit: `edb8089` (in `git log --oneline -20` output)
- TB-4 atom commit range: `cfc81de..a17d477` (9 atoms; readable via `git log --oneline cfc81de..a17d477`)
- TB-4 book-keeping: `6c42cf7`
- TB-4 medium-batch evidence commit: `16121c1`
- TB-5 charter v1 commit: `1b60237` (current HEAD; `origin/main` at audit prep)
- Constitutional anchors: `constitution.md` § Articles I-V + Boot Art IV + Laws 1-2

Audit verdict files (when ready) belong at `handover/audits/CODEX_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md` and `handover/audits/GEMINI_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md`. Merged verdict at `handover/audits/DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md` per the TB-2 r1 / TB-2 r2 / TB-2 Phase-1c naming pattern.
