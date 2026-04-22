# Paper 1 Draft Outline — TuringOS: Constitutional Microkernel for LLM Swarm

**Status**: draft (2026-04-22), pre-Phase-9 data
**Target**: arXiv preprint, adversarial-audit defensible
**Core thesis (2026-04-22 revision post Codex CHALLENGE)**: **First implementation of a Turing-machine-topology constitutional substrate for LLM swarms, with cryptographic capability tokens and runtime-verified three-process governance. We empirically test N Hayek-inspired market mechanisms on this substrate and report observed effects on PPUT (Progress Per Unit Time).**

Key revisions from earlier draft:
- "drives emergent faster proof discovery" → "we empirically test ... and report observed effects"
- Scope limited to *tested mechanisms*, *tested seeds*, *this benchmark*
- No unjustified generalization claims
- Results sections report null or negative findings honestly

---

## Abstract (draft)

Large language model (LLM) swarms performing formal reasoning face a
constitutional-layer gap: mechanisms that are agent-visible at the prompt
level but unenforced at the code level permit drift, Goodhart attacks,
and non-reproducible results. We present **TuringOS**, a Rust microkernel
that implements the Turing machine topology $Q_t \to \delta \to \Pi p
\to w_{\text{tool}} \to Q_{t+1}$ with cryptographically-enforced capability
tokens, a three-process governance layer (InitAI / ArchitectAI / JudgeAI),
and PPUT-first (Progress Per Unit Time) as the sole optimization metric.
We evaluate on MiniF2F-Lean4 across 244 problems × 3 seeds × {step,
complete} and demonstrate (i) `Mean PPUT (solved-only)` Wilson 95% CI
lower bound ≥ X; (ii) depth-N golden-path DAGs (up to N=23) produced via
per-tactic $\delta$-step architecture; (iii) $\ge 1$ cryptographically
vetoed patch from an adversarial proposal, proving non-null governance;
(iv) Ed25519-signed receipts making blessed writes unforgeable even by
code with `&mut Bus`. We release a Docker reproducibility bundle that
reconstructs headline numbers within 5%.

---

## § 1. Introduction

### 1.1 Problem
- LLM agents can drift from constitutional intent under adversarial pressure or via prompt-level constraint erosion
- Existing swarm frameworks (AutoGPT, CrewAI, BabyAGI) enforce at prompt, not code — "soft constraints" → C-022 context poisoning class
- Formal verification benchmarks (MiniF2F, PutnamBench) measure terminal solve rate, not architecture's throughput-per-unit-time

### 1.2 Contribution
1. **Constitutional microkernel**: Rust implementation where Art. I-V of a natural-language constitution compile into specific Rust invariants (enumerated in § 3)
2. **Cryptographic capability tokens**: Ed25519-signed `OracleReceipt` makes blessed tape writes unforgeable
3. **Per-tactic $\delta$-step**: three-way predicate verdict (Complete / PartialOk / Reject) allows real Turing-topology execution
4. **Three-process governance**: InitAI + ArchitectAI + dual JudgeAI with git-CI pre-merge gate; red-team veto traces as evidence
5. **PPUT-first evaluation**: we argue (following C-052) that solve-rate alone is Art. I.1 (boolean predicate) while PPUT is the required Art. I.2 (statistical signal)

### 1.3 Scope limitations (Paper 1)
- Closed-world in-process; external agents deferred to Paper 3 (omegav4 / PCP predicates)
- Single-problem-class benchmark (Lean4 formal proofs); generalization to open-ended proof discovery → Paper 2 (zeta_sum_proof)
- Governance runtime is single-machine; distributed consensus left to future work

---

## § 2. Related work

- **Koolen/Olshausen constitutional architecture**: top-level governance as immutable axioms (our § 3.1 mirror)
- **Satoshi whitepaper**: append-only ledger + capability via signatures (our § 3.2 OracleReceipt)
- **Hayek**: market as informational compression (our Phase 3A prediction market prices)
- **Turing 1936**: machine = $\langle q, \text{tape}, \delta \rangle$ (our § 3.3 Q_t structure)
- **Karpathy** "LLM IS the search algorithm": scaffold design over model choice (our C-034 mechanism-over-prompt)
- **Existing LLM swarm systems**: AutoGPT / CrewAI / BabyAGI (prompt-level constraints only — we compare explicitly)
- **Existing MiniF2F papers**: DeepSeek-Prover / Mathlib-trained variants — we measure PPUT not solve rate

---

## § 3. Architecture

### 3.1 Constitution as Ground Truth (Art. V.1.1)
- Markdown file + GPG-signed + chmod 444 + dm-verity
- Self-referential: constitution encodes its own modification permissions
- Judicial precedent library (`cases/C-*.yaml`) for case-law-style interpretation

### 3.2 Turing Machine Topology (Art. IV)
$Q_t = \langle q_t, \text{HEAD}_t, \text{tape}_t \rangle$ → `rtool` → $\delta$ (LLM) → ∏p → `wtool` → $Q_{t+1}$

Key: each arrow is a concrete Rust function or trait method (table in § 3.2.1).
`q_t` is now a first-class `QState` enum (`Running` / `Halted{reason}`).

### 3.3 Predicate layer (Art. I.1 / I.1.1)
- Art. I.1 boolean predicate: `Lean4Oracle::verify_omega_detailed`
- Art. I.1.1 PCP: implemented via `PartialVerdict` three-way enum
- Paper 2/3 extension: `Predicate` trait with `Verdict::PartialOk { confidence }`

### 3.4 Capability tokens (C-067)
- `OracleReceipt` struct with private fields + Ed25519 signature
- Signed message: `payload_hash || context_hash || kind_byte || verdict_encoding`
- Bus registers `trusted_oracle_pubs` pre-init, freezes on `init()` → attacker with `&mut Bus` post-init cannot forge

### 3.5 Statistical signals (Art. I.2 three)
- **PPUT** (effort rate): `100% / time_to_omega`
- **Reputation**: per-author citation counter in tape
- **Consensus**: via prediction market prices (C-036 telemetry)

### 3.6 Broadcast layer (Art. II)
- Typical error classes with frequency threshold (C-055): broadcast only when ≥ 3 agents hit same class
- Market prices (Hayek bounty) as signal

### 3.7 Three-process governance (Art. V)
- Architecture from `ART_V_MIN_DESIGN_2026-04-22.md`

---

## § 4. Evaluation

### 4.1 Benchmark
- MiniF2F-Lean4: 244 formal math problems
- Conditions: dual-mode (step + complete available) vs step-only
- 3 pre-registered seeds (from Phase 9)
- Model: deepseek-chat (default; demonstrates weak-model sufficiency)

### 4.2 Headline metrics (Phase 9 output)

| Condition | Mean PPUT (solved) | 95% CI | depth≥10 solves | ΣPPUT |
|---|---|---|---|---|
| dual-mode | TBD | TBD | TBD | TBD |
| step-only | TBD | TBD | TBD | TBD |

### 4.3 Art. V veto evidence (Phase 10c)
- Red-team proposal: remove `native_decide` from forbidden_patterns
- Expected: JudgeAI-α + JudgeAI-β both VETO citing C-011
- Archived trace: `handover/audits/VETO_TRACE_C011_REDTEAM_XXXX.md`

### 4.4 Capability forgery resistance
- Test: `attacker_with_mut_bus_cannot_forge_post_init`
- Claim: an attacker with `&mut TuringBus` post-init **cannot** produce a receipt that `append_oracle_accepted` honours

### 4.5 Reproducibility
- Docker image + `reproduce.sh` script
- Target: replay seed 74677 N=20 dual-mode, ΣPPUT within ±5% of reported
- `audit_proof.py`: external Lean re-verification of all golden-path proofs

---

## § 5. Discussion

### 5.1 What the PPUT metric reveals
- solve-rate alone doesn't distinguish "fast 50% solve" from "slow 100% solve"
- PPUT captures both dimensions in a single scalar per Art. I.2

### 5.2 Why cryptographic capability tokens matter
- Call-site discipline is insufficient under adversarial evaluation
- Ed25519 makes the capability boundary first-class; impossible to bypass without breaking the crypto

### 5.3 What governance buys you
- Runtime-verified constitutional compliance is novel for LLM agent systems
- Self-modifying swarm remains bounded by the original constitution

### 5.4 Honest limitations
- PCP predicates (Art. I.1.1) not exercised in Paper 1 scope
- Art. V instances share LLM providers → not statistically independent in the limit
- Paper 1's "closed-world" assumption doesn't hold for external agents

### 5.5 Limitations that point to future papers
- Paper 2: v3 zeta_sum_proof — Lean oracle remains, but open-ended problem space
- Paper 3: omegav4 — PCP statistical predicates, external audit

---

## § 6. Conclusion

Constitutional governance can be more than documentation: it can be code,
with cryptographic teeth. TuringOS demonstrates the smallest-scale viable
instance of this thesis.

---

## § 7. Reproducibility statement

Following CR protocol:
- Docker image hash pinned
- 6 pre-registered seeds (4 baseline + 2 independent)
- Full source: github link (TBD)
- All proofs externally re-verifiable via `audit_proof.py`
- Dataset: MiniF2F-Lean4 (public)

---

## § 8. Ethics / impact

- No PII; formal proofs only
- Open-sourced (Apache-2.0)
- No capability usable for harmful generation — system is a proof verifier,
  not a code generator
- Environmental impact: N=244 × 3 seeds × 2 conditions = 1464 runs, est.
  <100 kWh total

---

## Appendix A — Constitution text (verbatim, for adversarial audit)

[embed constitution.md]

## Appendix B — Judicial cases index

[embed cases/SCHEMA.md + bullet of 40+ cases with 1-line ruling]

## Appendix C — Code→Constitution mapping table

| Article | Constitutional requirement | Rust implementation | Key test |
|---|---|---|---|
| I.1 | Boolean predicate | `verify_omega_detailed` | `test_bare_decide_forbidden` |
| I.1.1 | PCP predicate | `PartialVerdict::PartialOk` | Phase 3 future |
| I.2 (信誉) | Reputation counter | `Tape.reputation_by_author` | `reputation_accumulates_across_citers` |
| I.2 (效用) | PPUT | `PputResult.pput` | `evaluator.rs:36-42` |
| II.1 | Typical error threshold | `min_class_count_to_broadcast` | `threshold_blocks_single_instance_classes` |
| II.2 | Price signals visible | `UniverseSnapshot.market_ticker` + `balances` | `snapshot_balances_nonempty_after_genesis` |
| III.1 | Gardener | **deferred Phase 11+** | — |
| III.2 | Progressive disclosure | `SearchTool` + prompt | — |
| III.3 | Correlation shielding | TEMP_LADDER + pairwise diversity | `pairwise_diversity_mean` metric |
| III.4 | Goodhart defense | Metric code compile-hidden | C-051 (open-source退化) |
| IV (q-halt) | q state machine | `QState` + `EventType::Halt` | `q_state_starts_running` |
| IV (capability) | Unforgeable wtool | `OracleReceipt` + Ed25519 | `attacker_with_mut_bus_cannot_forge_post_init` |
| V.1.1 | Constitution readonly | chmod 444 + GPG | CI `constitution_immutable.yml` |
| V.1.2 | ArchitectAI | Codex CLI weekly cron | `routines/architect_ai.yaml` |
| V.1.3 | JudgeAI | Dual Gemini+DeepSeek | `VETO_TRACE_C011_REDTEAM_XXXX.md` |

## Appendix D — Threat model

- **Paper 1 closed-world**: all code in-process is trusted; Ed25519 + oracles_frozen prevents forgery from code that lacks SigningKey
- **Not defended (Paper 1)**: external adversarial agents (Paper 3); process-level compromise; git-key theft
- **Defended in Paper 1**: constitutional drift via code or prompt-level attacks; C-022 context poisoning; F-2026-04-20-05 brute-force class

## Appendix E — Related statistics

- Phase 8 commit trail: 8 commits `a4d744c` → `70a7a54`
- Test count: 184+ tests green
- Judicial cases立档: C-044/045/046/048/049/050/052/053/055/061/066/067

---

## Notes for future drafts

- § 4.2 table remains TBD until Phase 9 completes (6 seeds × N=50)
- § 4.3 veto trace remains TBD until Phase 10c red-team exercise
- § 4.4 should include timing: how long does Ed25519 verify add to per-tx cost? (microbench needed)
- Consider adding "Why Rust?" sidebar — memory safety crucial for constitutional-level code
- Consider ablation: what if we remove Ed25519? (measure prior version's forge rate)
