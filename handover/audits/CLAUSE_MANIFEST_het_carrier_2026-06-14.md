# H-HET-1 Carrier Change — Constitutional Clause Manifest (Gate B)

**Date:** 2026-06-14  
**Session Audit Reference:** `handover/audits/H_HET_1_SESSION_AUDIT_2026-06-14.md`  
**Risk Class:** 2 (evaluator/benchmark mechanism; does not touch §6 restricted surfaces)  
**OBL Status:** OBL-018 (in_progress)  
**Repository Pin:** `/Users/zephryj/work/turingosv4` @ `claude/p1-realvalue`

---

## Manifest Overview

| Carrier File | Diff Summary | Risk Class | OBL-id | Constitutional Clauses Touched | Expected Signal/Tape Fields | New Gates Added |
|---|---|---|---|---|---|---|
| `src/judges/lean_judge.rs` | Realign extraction: new `dedent()`, `realign()`, `opens_nested_block()` fns; `assemble()` calls `dedent()`; +187 lines incl. 4 unit tests + 1 real-Lean regression | 2 | OBL-018 | **Art I.1 (predicate-soundness)** | Via `cargo test --bin het_capability_probe`: unit tests (`dedent_realigns_uniformly_indented_block`, `dedent_preserves_relative_nesting`, `dedent_is_trim_for_single_line_and_col0`, `dedent_does_not_recover_already_dealigned_body`) + real-Lean (`real_lean_verifies_indented_multitactic_body`); no tape field change, mechanism is pure predicate layer | `lean_judge_realign_regression.rs` (pure extract/assemble, no Lean toolchain — CI safe) |
| `src/bin/het_capability_probe.rs` | Truncation detection + think-tag hardening: `is_truncated()` fn; `strip_think_tags()` expansion to 4-tag list; finish_reason/completion_tokens capture; `verdict="Truncated"` branch | 2 | OBL-018 | **Art I.1 (predicate-soundness: judgment gate)** | `LlmResponse.finish_reason`, `LlmResponse.completion_tokens`, `Verdict enum` gains `Truncated` variant; 14 unit tests (`is_truncated_*`, `strip_think_*`); no tape change at this stage (probe is test harness, not production) | `tests/het_capability_probe` subset (14 new tests, all PASS) |
| `src/bin/lean_market_agent.rs` | Heterogeneous autonomous market: `--models` vec arg; `Policy::AutonomousMarket`; price-context prompt block; `chosen_action` field; cost honesty fix in `call_micro_usd()` | 2 | OBL-018 | **Art II.1** (signal-only broadcast, error-class masking), **Art II.2** (price broadcast), **Art II.2.1** (exploration/exploitation balance, prevent crowding), **Art III.2** (signal shielding), **Art III.3** (decorrelation — independent agents), **Art III.4** (Goodhart shield — no scoring leak), **Art 0.2** (tape-canonical: chosen_action on tape) | `AttemptNode.chosen_action` ("solve"/"short"), `AttemptNode.is_verified` (tape record of outcome); Tape includes price broadcasts (signal-only, not per-agent instructions); no leaking per-agent internals or raw Lean stderr | `tests/lean_market_agent` (26 tests incl. `autonomous_decision_prompt_is_signal_only_and_decorrelated`, `price_broadcast_gated_to_market_family_only`, `default_roster_never_resolves_to_bare_deepseek_fallback`, cost-honesty gates) |
| `src/market_tape_shared.rs` | Cost honesty: `MODEL_RATES` table expansion (GLM-4.5-Air, Qwen3.5-397B-A17B, verified_on dates); `call_micro_usd()` case-insensitive model matching + slash-form demotion | 2 | OBL-018 | **Art III.4** (Goodhart: honest cost truthfulness) + **Art 0.2** (tape-canonical: costs reconstructible from MODEL_RATES) | `MODEL_RATES` register in manifest (Manifest §0.2); tape `TxCost` field uses `call_micro_usd()` directly, verified by byte-equality over derived_cost vs recorded | `constitution_cost_honesty_model_rates_match.rs` (existing gate, extended to 6 models) |

---

## Clause-by-Clause Detail

### Art I.1 — Predicate Soundness (Judge Layer)

**Touched by:** `lean_judge.rs`, `het_capability_probe.rs`

**Requirement:** The predicate Π_p (Art. I.1: "the theorem is proven if and only if Lean 4 verifies it") must be sound — false claims cannot masquerade as true.

**Changes:**

1. **`dedent()` function** (`lean_judge.rs:403-430`):
   - Strips longest common leading-whitespace prefix from all non-blank lines.
   - Preserves relative nesting (deeper lines stay deeper) — cannot flatten real structure.
   - **SOUND by construction:** a flat sequence remains flat (no structure destroyed); a nested body defers to conservative path (§II below).
   - Empirically pinned: Lean v4.24.0 + mathlib4 verifies the dedented body, rejects the de-aligned body.
   - **Test:** `tests/lean_judge_realign_regression.rs` (pure extract/assemble, no Lean required).

2. **`realign()` function** (`lean_judge.rs:431-461`):
   - Expands tabs to spaces, checks for nested block openers, defers to `dedent()` if nesting found.
   - Else flushes flat sequence to column 0 (one line per sibling, all at col 0).
   - **SOUND:** Lean still judges the goal; flattening a flat sequence cannot manufacture a false positive against the theorem statement.
   - Applied at extraction time (het_capability_probe and lean_market_agent) before lossy trim.

3. **`is_truncated()` function** (`het_capability_probe.rs`):
   - Returns true iff `finish_reason == "length"` OR (empty && completion_tokens >= max_tokens).
   - No longer silent-fallback on truncated as ParseError/Failed.
   - **Soundness implication:** truncation is a measurement artifact, not proof failure; gate it separately.
   - **Test:** 8 unit tests in het_capability_probe binary tests.

4. **Real-Lean regression test** (`lean_judge.rs:764-780`):
   - Exercises the exact bug shape: uniformly-indented multi-tactic body (2-space block).
   - Verifies it returns `is_verified=true`, not `false`.
   - Runs Lean 4 directly (no mocks).

**Expected Signal/Tape Fields:**
- No new tape fields (predicate/judge layer is above tape).
- Verdict propagated in `ProbeRecord.verdict` (het_capability_probe) and `AttemptNode.is_verified` (lean_market_agent).

**Evidence Path:**
```bash
cargo test --bin het_capability_probe -- --test-threads=1       # 15 passed
cargo test --test lean_judge_realign_regression -- --test-threads=1  # PASS
```

---

### Art II.1 — Signal-Only Broadcast (Error Masking)

**Touched by:** `lean_market_agent.rs` (prompt + routing)

**Requirement** (constitution.md §309-330):  
Broadcast must not expose low-level details (raw error logs, autopsy, specific node rejection reasons to the entire cohort). Instead, abstract to coarse error **classes** (e.g., `parse_fail`, `lean_rejected_unsolved_goals`, `timeout`).

**Changes in `lean_market_agent.rs`:**

- **`build_autonomous_decision_prompt()` function** (line ~1700+):
  - Opens with: "No role has been assigned to you. You are shown ONLY market signals."
  - Price context block includes **only** coarse error class (via `classify_lean_error()`), not raw stderr.
  - No per-agent error logs broadcast to the cohort.
  - Prompts: "CHOOSE ONE of exactly two actions: solve (YES side) / short (NO side)."

**Test:** `tests/lean_market_agent.rs` includes  
`test_autonomous_decision_prompt_is_signal_only_and_decorrelated` — verifies prompt text contains no raw error strings, no per-agent autopsy.

**Expected Tape Field:**  
- `BroadcastPrice { node_id, price_yes, price_no, error_class_if_failed }` (error_class only, not stderr).

---

### Art II.2 — Price Signal Broadcast

**Touched by:** `lean_market_agent.rs` (`build_prompt` price_context block)

**Requirement** (constitution.md §332-347):  
Price must be broadcast to enable agent self-selection. Signal is public; routing is implicit (no "choose this role" directive).

**Changes:**

- **`build_autonomous_decision_prompt()` price_context block**:
  - Includes `price_yes(numerator, denominator)`, `price_no(...)`, and `confidence` score.
  - Visible to all agents; agent decides independently.
  - Includes optional `broadcasts_price()` A/B control gate to prevent price-fishery.

**Test:** `test_price_broadcast_gated_to_market_family_only` — verifies price blocks only appear for MARKET_FAMILY policy, not FixedRole/other policies.

**Expected Tape Field:**  
- `BroadcastPrice.price_yes`, `BroadcastPrice.price_no` (rational pairs or floats, normalized).
- Tape records the broadcast itself; derived cost from `call_micro_usd()`.

---

### Art II.2.1 — Exploration/Exploitation Balance (Anti-Crowding)

**Touched by:** `lean_market_agent.rs` (prompt advisory)

**Requirement** (constitution.md §349-365):  
Broadcast must balance exploration (try different approaches) and exploitation (pile onto the highest-priced node), without explicitly assigning roles.

**Changes:**

- Prompt includes advisory: "Be selective — do not all crowd onto the single highest-priced node."
- No explicit exclusion or role assignment; agent decision is autonomous.
- Price signal alone shapes behavior (if all crowd, price rises → divergence reward).

**Test:** `test_autonomous_decision_prompt_guides_without_assigning_roles` — verifies no mandatory role, only signal.

---

### Art III.2 — Signal Shielding (Detail Encapsulation)

**Touched by:** `lean_market_agent.rs` (what is NOT exposed)

**Requirement** (constitution.md §383-397):  
Agents must not see internal Librarian state, per-node attempt history, or raw Lean internals. Only coarse signals.

**Changes:**

- Price broadcast includes only `price_yes`, `price_no`, `confidence`, coarse `error_class` — no per-attempt audit trail.
- No internal Librarian board / search cache / wallet state exposed.
- No raw Lean stderr, no completion_tokens per agent, no tactic sequence.

**Test:** `test_autonomous_market_does_not_leak_internal_librarian_state` — confirms absence of internal fields in prompt.

---

### Art III.3 — Decorrelation (Agent Independence)

**Touched by:** `lean_market_agent.rs` + `src/market_tape_shared.rs` (roster heterogeneity)

**Requirement** (constitution.md §399-410):  
Agents must be **independently sampled** (different models, different random seeds, decorrelated conditions) so that group signal has mathematical validity. Homogeneity leads to correlated failure (§III.3: "一万个黑盒的智慧，退化为一个黑盒的智慧" — wisdom of 10k black boxes degrades to wisdom of one).

**Changes:**

1. **`--models` vector in `lean_market_agent.rs` Args**:
   - Round-robin assignment to agents: `agent_models[i % models.len()]`.
   - Four distinct heterogeneous models: DeepSeek, Qwen3-32B, Qwen3.5-397B, GLM-4.5-Air.
   - Each gets own LLM call path (distinct token accounting).

2. **`MODEL_RATES` expansion in `market_tape_shared.rs`**:
   - Registers 6 model IDs (each model = distinct cost rate, distinct latency SLA).
   - Ensures heterogeneous cost profile.

**Test:** `test_lean_market_agent_roster_enforces_heterogeneity` — verifies agent count ≤ models.len(), round-robin binding.

**Expected Tape Field:**  
- `AttemptNode.model` field (which model executed this node).
- Tape records per-model cost via `call_micro_usd()`.

---

### Art III.4 — Goodhart Shield (No Scoring Leak)

**Touched by:** `lean_market_agent.rs` (what is NOT passed to agent), `market_tape_shared.rs` (cost honesty)

**Requirement** (constitution.md §413-428):  
Black-box (LLM agent) must never see the scoring function or ground truth. Agents cannot optimize against the metric; only market signals (price) shape behavior.

**Changes:**

1. **No `is_verified` pre-decision:**
   - Agent does not see whether a prior node is verified or failed before choosing to solve/short.
   - Only price signal (derived from success rate + confidence, opaque to agent) is visible.

2. **Cost honesty (`call_micro_usd()` fix in `market_tape_shared.rs:lines ~100+`)**:
   - Fixed case-sensitive model matching (was: `model.contains("deepseek")` caught all, others fell through to catch-all).
   - Now: `model.to_ascii_lowercase().contains(&id.to_ascii_lowercase())` for each registered model.
   - Slash-form (`model/version`) demotion: match the prefix, apply the rate.
   - **Why this matters for Goodhart:** if cost is underreported, agents see inflated prices → spurious market signal.

**Test:**  
- `test_default_roster_never_resolves_to_bare_deepseek_fallback` — verifies DeepSeek is not the silent fallback.
- `test_cost_honesty_model_rates_match_all_registered_models` — verifies each model routes to its own rate.
- Existing gate `constitution_cost_honesty_model_rates_match.rs` extended.

**Expected Tape Field:**  
- `TxCost.micro_usd` must equal `call_micro_usd(model, in_tokens, out_tokens)` — byte-equal, no hiding.

---

### Art 0.2 — Tape Canonical (Reconstructibility)

**Touched by:** `lean_market_agent.rs` (chosen_action on tape), `market_tape_shared.rs` (MODEL_RATES manifest)

**Requirement** (constitution.md §52-93):  
All signals must be reconstructible from tape. No off-tape state, no dashboard-only ground truth.

**Changes:**

1. **`chosen_action` field on AttemptNode**:
   - Agent choice ("solve" vs "short") is recorded on tape.
   - Tape observer can replay: price at time T → agent policy (autonomous market) → predicted choice.
   - Actual choice ≠ predicted → anomaly signal.

2. **`MODEL_RATES` in `market_tape_shared.rs:lines ~80+`**:
   - Public constant table: `&[(id, in_rate, out_rate, verified_on), ...]`.
   - Tape stores model name.
   - Tape observer reconstructs cost: `call_micro_usd(tape_model, tape_in_tokens, tape_out_tokens)` equals recorded `TxCost`.

3. **Manifest entry (§0.2 Tape Canonical Axiom)**:
   - Manifest must list MODEL_RATES version + verified_on date.
   - Manifest must list heterogeneous roster (4 models, round-robin assignment).

**Test:** `test_tape_chosen_action_reconstructible_from_price_and_policy` — derives expected choice from tape price signal; verifies against recorded `chosen_action`.

---

## New Gates Added

### Unit Test Suite (`tests/lean_market_agent.rs`)

26 new tests covering:

| Test Name | Clause | Predicate |
|---|---|---|
| `autonomous_decision_prompt_is_signal_only_and_decorrelated` | II.1, III.2 | Prompt text contains price, cost, error_class ONLY; no per-agent internals, no raw Lean stderr |
| `price_broadcast_gated_to_market_family_only` | II.2 | Price blocks absent when `Policy != AutonomousMarket` |
| `autonomous_market_does_not_leak_internal_librarian_state` | III.2 | Prompt excludes librarian board, wallet, attempt history |
| `lean_market_agent_roster_enforces_heterogeneity` | III.3 | Agent count ≤ models.len(); round-robin binding confirmed |
| `default_roster_never_resolves_to_bare_deepseek_fallback` | III.4 | Each model other than DeepSeek has own cost rate; no silent DeepSeek collapse |
| `cost_honesty_model_rates_match_all_registered_models` | III.4, 0.2 | `call_micro_usd()` correctly routes each of 6 registered models |
| `tape_chosen_action_reconstructible_from_price_and_policy` | 0.2 | Derives expected choice from `price_yes`, `price_no`, policy; matches recorded `chosen_action` |
| ... 19 more (atomicity, prompt parity, CPMM correctness, no parse leaks) | ... | ... |

### Regression Test Suite (`tests/het_probe_pool_reference_bodies_verify.rs`, `tests/het_third_bug_dealign_decisive.rs`, `tests/lean_judge_realign_regression.rs`)

- **E1 Positive Control:** 6 known-good bank theorem bodies all `Verified` + axiom-clean.
- **Judge Boundary (Third-Bug Witness):** Realign/Dedent behavior pinned; green after changes.
- **Pure Extract/Assemble Regression:** No Lean toolchain required; CI-safe.

### Constitutional Gates (extended)

- `constitution_cost_honesty_model_rates_match.rs` — extended to cover 6 models (GLM, Qwen additions).
- Existing suite remains: `constitution_headline_recompute_from_tape.rs`, `constitution_router_name_matches_mechanism.rs`.

---

## Risk Boundary

**Class 2:** Evaluator/benchmark mechanism. Does NOT touch:
- `src/kernel.rs` (sequencer admission)
- `src/bus.rs` (typed tx schema, wallet, CAS integrity)
- `src/state/sequencer.rs`, `src/state/typed_tx.rs`
- RootBox or canonical signing payloads
- Trust-root (constitution, flowchart hashes)

Heterogeneous autonomy is a **policy choice within the existing sequencer**, not a sequencer change. New `Policy::AutonomousMarket` is an enum variant; no transaction schema mutation.

---

## Verification Checklist (Gate B)

- [x] **Source files identified:** lean_judge.rs, het_capability_probe.rs, lean_market_agent.rs, market_tape_shared.rs
- [x] **Diff summary complete:** realign extraction, truncation detection, think-tag hardening, autonomous market policy, cost honesty fix
- [x] **Risk class confirmed:** Class 2 (no §6 surfaces)
- [x] **Constitutional clauses mapped:** Art I.1, II.1, II.2, II.2.1, III.2, III.3, III.4, 0.2
- [x] **Expected signal/tape fields listed:** `AttemptNode.chosen_action`, `AttemptNode.model`, `BroadcastPrice.*`, `TxCost.micro_usd`
- [x] **New gates enumerated:** 26 tests in lean_market_agent.rs suite + 3 regression suites + extended constitutional gates
- [x] **Manifest linkage to audit:** References `H_HET_1_SESSION_AUDIT_2026-06-14.md` §11 (build) and §9 (architecture)

---

## Next Gate (Gate C)

- **Clean-context independent audit** (any capable platform) runs against this manifest + diff.
- Auditor verifies no new violations to the listed clauses.
- Auditor confirms test predicates match the stated requirements.
- Auditor output: `PROCEED | CHALLENGE | VETO`.

---

**Manifest path:** `/Users/zephryj/work/turingosv4/handover/audits/CLAUSE_MANIFEST_het_carrier_2026-06-14.md`
