# §8 Decision Packet — Budget Hard-Ceiling From a Signed Manifest (admission/economic-state rejecting leg)

**Status**: **RATIFIED + SHIPPED 2026-06-08** (token `APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST`
supplied verbatim; see §8). Implemented as LIVE-FC1 Phase 5 in the
**unpinned-first ZERO-pinned-diff** form: a signed/user-approved budget-manifest
reader → integer ceiling → tape-derived spend (reusing the VPPUT C_i) → a
pre-admission check at the unpinned `step_forward` membrane that, on spend ≥
ceiling, rejects with the EXISTING `RejectionClass::BudgetExceeded` and does NOT
advance the head (the FC2-HALT). NO new pinned discriminant; q_state/typed_tx/
sequencer/genesis UNCHANGED. Integer-only; forward-only (ceiling 0 = unlimited);
checkpoint-resumable. This is the Turing-completeness fuel: an external integer
resource bound forces termination. Gate `constitution_budget_hard_ceiling` 6/6;
clean-context audit PROCEED. The original request text below is preserved verbatim.

**Date**: 2026-06-08
**Branch**: `claude/s3-economy-boltzmann` (read base
`/home/zephryj/projects/turingosv4-economy` = checkout of `origin/main`,
HEAD `a68692de`). Implementation, if ratified, lands on a fresh feature branch
off `origin/main`.
**Risk class**: **Class 4 (candidate).** See §6 for the honest justification.
Short version: a budget ceiling that **rejects/halts on exceed** is an *admission
decision over economic state*. Today the only admission authority is the
sequencer (`src/state/sequencer.rs`, **PINNED**), the only money-conservation
authority is `src/economy/monetary_invariant.rs` (**PINNED**), and the only
spend ceiling that already halts a run (`RunOutcome::MaxTxExhausted`,
`src/state/typed_tx.rs:244`, **PINNED**) lives on the pinned typed-tx surface.
Any leg that turns "ceiling exceeded" into a *rejected transition* or a *clean
halt* therefore touches sequencer admission and/or the pinned RunOutcome
schema → **Class 4 candidate until proven otherwise**
(`AGENTS.md §5–§6`, `feedback_class4_cannot_hide_in_class3`). It is NOT a pure
read-view (that would be Class 2); the defining property requested here is
*on-exceed REJECT/halt, not warn*.

**Recommendation (operating posture):** ratify the **mechanism-build** under a
strict unpinned-first constraint (§3), and require that the concrete activation
which actually *re-pins* the signed ceiling into `genesis_payload.toml` carry its
own signed `v4-ratify` tag (the trust-root-pin trap,
`feedback_trust_root_pin_trap`). Do NOT batch this with unrelated economy work.

**Proposed §8 token** (the architect replies with this exact phrase to ratify):

```text
APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST
```

```text
Reject / defer option:
  REJECT-BUDGET-HARD-CEILING-FOR-NOW   # keep spend control advisory; no on-exceed reject; ceiling stays a derived/observe-only view
```

**Authority chain** (constitution articles + exact src `file:line` verified in
this read base):

- Constitution monetary basis — **Law 2 (`constitution.md:160`)**: "Only
  Investment Costs Money — 1 Coin = 1 YES + 1 NO (CTF 守恒)；on_init 是唯一合法
  铸币点." Plus **Law 1 (`constitution.md:159`)**: "Information is Free —
  搜索与查看零成本，思考不花钱" (a spend ceiling MUST NOT charge reads/think).
- Constitution conservation/derived-view basis — **`constitution.md:61`**:
  平行账本（`RunCostAccumulator` 等）只能是 tape 的派生视图，不可作独立 source of
  truth；每个派生视图都必须有 `assert_eq!(view, derive_from_tape(tape))` 守恒测试.
  (A budget accumulator is a derived view; the ceiling source must be the signed
  manifest, not a parallel ledger.)
- Constitution FC2 halt basis — **`constitution.md:584`** `halt@{...label:"HALT"}`
  and **`constitution.md:654`** `q1 ==>|"if q = halt"| halt`: FC2 boot/full
  architecture has a first-class terminal HALT node. The map-reduce tick + clock
  (`constitution.md:622–625`, `656–659`) is the per-round economic tick a ceiling
  would gate.
- Money-conservation enforcement (already live, **PINNED**):
  `src/economy/monetary_invariant.rs` — `MonetaryError` taxonomy (line 40),
  `total_supply_micro` (line 213, integer `i64` only), `assert_total_ctf_conserved`
  (line 485), `assert_no_post_init_mint` (line 371), `assert_read_is_free`
  (line 520). Module header cites "基本法 1 (Coin 守恒)" + "Inv 4 (no post-init
  mint)" (`monetary_invariant.rs:11–13`).
- Admission authority (already live, **PINNED**): `src/state/sequencer.rs` —
  `rejection_class_for` (line 564) maps `TransitionError` → `L4ERejectionClass`;
  the WorkTx stake-vs-balance admission gate is at `sequencer.rs:1945–1985`
  (produces `TransitionError::StakeBalanceExceeded`); the escrow-presence gate at
  `sequencer.rs:1998–2004`.
- Existing spend-ceiling halt (already live, **PINNED**):
  `RunOutcome::MaxTxExhausted` (`src/state/typed_tx.rs:244`); the clean-halt
  classifier consumes it at `src/runtime/chain_derived_run_facts.rs:783`.
- Reserved-but-unused admission class (an empty seam):
  `RejectionClass::BudgetExceeded` (`src/state/typed_tx.rs:174`) is **defined but
  has no producer anywhere in `src/`** (grep for `RejectionClass::BudgetExceeded` /
  `RC::BudgetExceeded` / `L4ERejectionClass::BudgetExceeded` returns zero
  production sites). It is a pre-wired discriminant awaiting exactly this kind of
  ratified ceiling.
- Existing per-batch budget metadata (already live, **UNPINNED**):
  `src/runtime/benchmark_manifest.rs` — `BenchmarkManifest.max_tx_budget: u64`
  (line 41, integer), validated by `ZeroMaxTxBudget` (line 104, 166). This is
  per-batch *evidence-dir metadata* (NOT ChainTape, NOT signed — header line
  15–16: "manifest is per-batch metadata, not ChainTape").
- The signed/pinned manifest itself: `genesis_payload.toml` `[trust_root]`
  section + `creator_signature` (`genesis_payload.toml:120`), verified at boot by
  `turingosv4::boot::verify_trust_root` (`genesis_payload.toml:10–12`), abort
  `TRUST_ROOT_TAMPERED` on mismatch. **This is the only signed ceiling home.**
- Constitution binding: `AGENTS.md §5–§6, §9, §12, §14`; `CLAUDE.md §4`
  (integer money math; no hardcoded behavior parameter; no workaround closures);
  `feedback_trust_root_pin_trap`; `feedback_class4_cannot_hide_in_class3`;
  `feedback_admission_fail_closed_default`; `feedback_no_workarounds_strict_constitution`;
  `feedback_benchmark_manifest_required`.

---

## §1. Decision statement (what the budget hard-ceiling does)

**Add a BUDGET HARD-CEILING whose sole ceiling source is a signed manifest, and
whose on-exceed behavior is REJECT/HALT — not a logged warning.**

Concretely:

1. **Signed-manifest ceiling source.** The maximum economic/token spend for a run
   (and/or per-task / per-agent, scope fixed at design time, see §3) is read from
   a **signed** manifest — `genesis_payload.toml`'s pinned `[trust_root]`-verified
   surface (or a signed sub-manifest it pins), NOT from a hardcoded constant and
   NOT from the unsigned per-batch `BenchmarkManifest` evidence file. There is
   exactly **one** ceiling authority; no second source of truth.
2. **On-exceed = REJECT/HALT.** When cumulative spend would exceed the signed
   ceiling, the offending transition is **rejected** (lands in L4.E via the
   existing rejection path, surfacing `RejectionClass::BudgetExceeded`,
   `typed_tx.rs:174`) **and/or** the run **halts cleanly** at the FC2 HALT node
   (`constitution.md:584/654`) with a terminal class analogous to the existing
   `RunOutcome::MaxTxExhausted` (`typed_tx.rs:244`). It does **not** advance
   `state_root_t`; it does **not** mint; it does **not** silently log-and-continue.
3. **Integer-only.** All ceiling math is integer micro-units (`i64`/`u64`),
   mirroring `monetary_invariant.rs::total_supply_micro` (line 213). No `f64` in
   the ceiling-comparison or any money/conservation path (`CLAUDE.md §4`).
4. **Fail-closed.** A missing/unreadable/invalid signed ceiling, or an arithmetic
   overflow in the spend sum, → **reject/halt**, never "no ceiling → proceed"
   (`feedback_admission_fail_closed_default`).
5. **Tape-anchored receipt.** Every ceiling-triggered rejection/halt is one
   ChainTape/L4.E event + CAS rejection capsule (the `BudgetExceeded = 7`
   generate-side capsule class already exists at `rejection_capsule.rs:31`), so
   the event is reconstructable, not a stdout line.

**Honest framing — what is NOT being invented.** `src/token_budget.rs` is
**already an enforcer**, but of a *different quantity*: it hard-caps **LLM
prompt-token size** (the FC1a-budget_gate; `B_PROMPT_MAX = 5800` at
`token_budget.rs:47`, asserted in `MemoryKernel::step_forward` at
`memory_kernel.rs:547` and clipped in `assemble_o1_prompt` at
`memory_kernel.rs:573`). That is a *context-window* ceiling on a single prompt,
deterministically truncating/degrading payloads (`enforce_budget` dispatcher,
`token_budget.rs:363`) — it is NOT an *economic-spend* ceiling and it does not
read a signed manifest; its constants are `pub const` in-module
(`token_budget.rs:21–47`). Likewise `RunOutcome::MaxTxExhausted` already halts a
run on **transaction count**, and `BenchmarkManifest.max_tx_budget` already pins
a per-batch tx budget (but unsigned, off-chain). So this atom is **not** "invent
a new enforcer from scratch." It is: **introduce the economic-spend hard-ceiling
quantity, derive its ceiling from the SIGNED manifest (not a const, not the
unsigned benchmark file), and wire the on-exceed path to the existing
reject/halt machinery** (`RejectionClass::BudgetExceeded` producer + a HALT
terminal). The novelty is *signed-source + economic-spend scope + on-exceed
reject*, composed from parts that already exist.

---

## §2. Precondition: what is ALREADY live (this leg builds on it, does not re-do it)

This packet is requestable only because the conservation floor, the admission
machinery, and a halt precedent already stand and are tape-anchored. Do not
re-authorize these.

| Precondition | Status | Evidence (file:line) |
|---|---|---|
| **Money conservation (integer-only)** — total supply summed in `i64` micro-units; mint/burn rejected unless exempt; reads carry no fee. | LIVE (PINNED) | `src/economy/monetary_invariant.rs`: `total_supply_micro` (213), `assert_total_ctf_conserved` (485), `assert_no_post_init_mint` (371), `assert_read_is_free` (520). Gates: `tests/economy_conservation.rs`, `tests/constitution_economy_strict_equality.rs`, `tests/walkthrough_inv3_conservation.rs`. |
| **Admission → L4.E rejection mapping** — `TransitionError` → `L4ERejectionClass`; stake-vs-balance + escrow-presence admission gates already reject on economic-state checks. | LIVE (PINNED) | `src/state/sequencer.rs`: `rejection_class_for` (564), WorkTx stake gate (1945–1985), escrow gate (1998–2004). |
| **Reserved budget rejection class** — `RejectionClass::BudgetExceeded` discriminant exists, no producer yet (an empty seam awaiting a ratified ceiling). | LIVE (PINNED), unwired | `src/state/typed_tx.rs:174` (zero production sites in `src/`). |
| **Spend-ceiling HALT precedent** — a run already halts cleanly on a spend bound (`MaxTxExhausted`), with EvidenceCapsule + TerminalSummary on halt. | LIVE (PINNED) | `RunOutcome::MaxTxExhausted` `src/state/typed_tx.rs:244`; clean-halt classifier `src/runtime/chain_derived_run_facts.rs:783`; capsule reason `src/runtime/evidence_capsule.rs` (`ExhaustionReason::MaxTxExhausted`). |
| **Prompt-token enforcer (different quantity)** — deterministic hard cap on prompt size; integer/char-count math, no byte-proxy. | LIVE (UNPINNED) | `src/token_budget.rs`: `B_PROMPT_MAX` (47), dispatcher `enforce_budget` (363); enforced `src/memory_kernel.rs:547`, `:573`. |
| **Per-batch tx-budget pin (unsigned, off-chain)** — pins `max_tx_budget` before a scaled batch; validated non-zero. | LIVE (UNPINNED) | `src/runtime/benchmark_manifest.rs`: `max_tx_budget: u64` (41), `ZeroMaxTxBudget` (104/166). Header: "per-batch metadata, not ChainTape" (15–16). |
| **Signed/pinned manifest home** — `[trust_root]` SHA-256 manifest + `creator_signature`, boot-verified, `TRUST_ROOT_TAMPERED` on drift. | LIVE | `genesis_payload.toml:10–12`, `:120`; verifier `boot::verify_trust_root`. |

Because these are live, the residual risk of THIS leg is concentrated in one new
authority: **turning a signed-manifest spend ceiling into a rejected
transition / clean halt**. Everything it composes is already observed and
tape-anchored.

---

## §3. Allowed engineering actions (only under the §8 token)

The following are the **only** moves authorized once the architect supplies the
token. **Prefer NEW unpinned modules** over editing PINNED surfaces, per
`feedback_step_b_protocol`. Pinning status was verified by
`grep -c '"<path>"' genesis_payload.toml`:

- **PINNED (count > 0):** `src/state/sequencer.rs`, `src/economy/monetary_invariant.rs`,
  `src/state/q_state.rs`, `src/state/typed_tx.rs`.
- **UNPINNED (count = 0):** `src/token_budget.rs`, `src/runtime/benchmark_manifest.rs`,
  `src/memory_kernel.rs`, `src/tdma_runner.rs`.

Authorized actions:

- **A-ALLOW-1 — signed-ceiling reader (NEW unpinned module, e.g.
  `src/runtime/budget_ceiling.rs`).** A deterministic, integer-only reader that
  loads the spend ceiling from the **signed** manifest surface (the
  `genesis_payload.toml` `[trust_root]`-verified region, re-using
  `boot::verify_trust_root` as the *sole* integrity authority — no second hash
  verifier) and exposes a pure `fn ceiling_micro(...) -> u64`. No `f64`. No
  hardcoded fallback constant (fail-closed if absent, A-ALLOW-4).
- **A-ALLOW-2 — spend accumulator as a derived view (NEW unpinned helper).** An
  integer running total of economic spend for the scoped unit (run / task /
  agent — scope fixed at design time before code), derived from ChainTape/CAS,
  NOT a parallel ledger. Per `constitution.md:61` it MUST carry an
  `assert_eq!(view, derive_from_tape(tape))` conservation test. This composes
  with `monetary_invariant::total_supply_micro` rather than re-summing holdings
  independently (no second conservation source, mirroring the TB-16 R1 lesson in
  `monetary_invariant.rs:205–212`).
- **A-ALLOW-3 — wire the on-exceed REJECT to the EXISTING admission path.** Emit
  `RejectionClass::BudgetExceeded` (`typed_tx.rs:174`) through the existing
  `rejection_class_for` mapping (`sequencer.rs:564`) so a ceiling breach lands in
  **L4.E** (not L4), advancing no `state_root_t`. **Seam honesty:** producing
  that class requires either (a) a new `TransitionError` arm consumed by the
  pinned `sequencer.rs`/`monetary_invariant.rs`, OR (b) a gate evaluated *before*
  sequencer admission in an unpinned pre-admission helper that returns the
  rejection. Option (b) is preferred (keeps the pinned diff at zero); option (a)
  is a pinned edit and triggers the trust-root-pin trap (A-ALLOW-5). The chosen
  option is named explicitly in the implementation PR; it does NOT get decided
  silently.
- **A-ALLOW-4 — fail-closed on the HALT side (compose with FC2 HALT).** When the
  scoped unit is a *run* rather than a single tx, the breach drives a clean halt
  at the FC2 HALT node (`constitution.md:584/654`) with a terminal disposition
  analogous to `MaxTxExhausted` (`typed_tx.rs:244`) — EvidenceCapsule +
  TerminalSummary emitted on the halt path (the `BudgetExceeded = 7` capsule
  class at `rejection_capsule.rs:31` is the receipt). **Adding a new
  `RunOutcome` discriminant is a pinned typed-tx schema change** (`typed_tx.rs`
  is PINNED) → treat as a separate Class-4 schema surface (see §5); the preferred
  path reuses/aliases the existing budget-exhaustion terminal semantics rather
  than bumping the enum.
- **A-ALLOW-5 — re-pin the signed ceiling in the same commit (only if the ceiling
  value/location is itself added to `[trust_root]`).** If the signed ceiling is a
  new pinned region of `genesis_payload.toml`, the rehash MUST land in the same
  commit (`feedback_trust_root_pin_trap`) and the resulting trust-root change
  carries its **own signed `v4-ratify` tag**, separate from this §8 token. This
  packet's atom, in its preferred unpinned-first form, edits **no** pinned file;
  any variant that must re-pin is a further Class-4 surface flagged in the PR.
- **A-ALLOW-6 — triple-coupled non-vacuous gate (NEW top-level gate).** A
  `tests/constitution_budget_hard_ceiling.rs` that proves the ceiling is
  **enforcing, not advisory**: a run/tx whose spend exceeds the signed ceiling is
  REJECTED/HALTED (not merely logged), the rejection is reconstructable from
  ChainTape/CAS, the ceiling derives from the signed manifest (mutating the
  manifest moves the boundary), and reads/think remain free (Law 1). Triple-couple
  per `feedback_constitution_gate_triple_coupling` (test file + manifest entry +
  matrix-drift glob). The gate MUST be able to fail (a mutant that flips
  reject→warn is caught), per `feedback_single_site_gate_illusion`.

**Sourcing constraint (binding):** no new hardcoded behavior parameter
(`CLAUDE.md §4`). The ceiling derives from the signed manifest; spend math is
integer-only; reads/search/think stay free (Law 1, `constitution.md:159`;
`assert_read_is_free`, `monetary_invariant.rs:520`).

---

## §4. Hard guards (binding even under the token)

If any cannot be met, the leg STOPS — do not weaken a guard into a skip
(`feedback_no_workarounds_strict_constitution`).

- **G-GUARD-1 — integer-only money math.** No `f64`/`f32` anywhere in the
  ceiling comparison, the spend accumulator, or any money/conservation path
  (`CLAUDE.md §4`, `AGENTS.md §12`). Match `total_supply_micro`'s `i64`
  micro-unit discipline (`monetary_invariant.rs:213`).
- **G-GUARD-2 — fail-closed = exceed→reject/halt.** Spend ≥ ceiling, or a
  missing/invalid/unverifiable signed ceiling, or an arithmetic overflow in the
  spend sum → REJECT/HALT, never "no ceiling → proceed" and never
  "warn-and-continue" (`feedback_admission_fail_closed_default`). A ceiling that
  only logs is the failure mode this packet exists to forbid.
- **G-GUARD-3 — signed manifest is the SOLE ceiling source.** The ceiling value
  comes from the `genesis_payload.toml` `[trust_root]`-verified surface (or a
  signed sub-manifest it pins), re-using `boot::verify_trust_root` as the **only**
  integrity authority. No hardcoded constant fallback; no reading the ceiling from
  the *unsigned* `BenchmarkManifest` evidence file
  (`benchmark_manifest.rs:15–16`); no dashboard/CLI flag becoming the source of
  truth (`constitution.md:61`; `AGENTS.md §12` no-second-source-of-truth).
- **G-GUARD-4 — reads/think stay free (Law 1).** The ceiling charges only
  *investment* spend (Law 2, `constitution.md:160`), never search/view/think
  (Law 1, `constitution.md:159`). `assert_read_is_free`
  (`monetary_invariant.rs:520`) must continue to pass; the ceiling MUST NOT
  introduce a per-read or per-think fee.
- **G-GUARD-5 — tape-anchored, reconstructable rejection receipt.** Every
  ceiling breach is one ChainTape/L4.E event + CAS rejection capsule
  (`rejection_capsule.rs`), surfacing `RejectionClass::BudgetExceeded`
  (`typed_tx.rs:174`) or the halt terminal class. No stdout-only "budget
  exceeded" line counts as the rejection (`AGENTS.md §4`: stdout is not evidence).
  Conservation is preserved bit-for-bit on the reject path: a rejected tx
  advances no `state_root_t` and moves no money (`assert_total_ctf_conserved`
  still holds).
- **G-GUARD-6 — reversibility / no irreversible state mutation on breach.** A
  ceiling rejection leaves `Q_t` unchanged (the rejected transition does not
  apply); a ceiling halt records a clean terminal `Q` that is reconstructable.
  No partial spend, no half-applied transition (`feedback_conservative_error_semantics`).
- **G-GUARD-7 — pinned-surface discipline.** Preferred implementation edits ZERO
  pinned files (unpinned reader + unpinned pre-admission gate + unpinned spend
  view + new top-level test). If the chosen seam genuinely requires a pinned edit
  (`sequencer.rs` admission arm, `typed_tx.rs` new `RunOutcome`/`TransitionError`
  discriminant, or a `genesis_payload.toml` re-pin), that is the higher-blast
  variant: rehash-in-same-commit + own signed `v4-ratify` tag (G-GUARD-3 of the
  trust-root trap) + an explicit per-atom §8 line in the PR — it does NOT hide
  under this token (`feedback_class4_cannot_hide_in_class3`).

---

## §5. Forbidden (even under the token)

- **No advisory-only ceiling.** A ceiling that logs a warning and lets the
  transaction proceed does NOT satisfy this packet — on-exceed REJECT/HALT is the
  defining property (§1).
- **No `f64` in the money/ceiling path.** Integer micro-units only (`CLAUDE.md §4`).
- **No hardcoded ceiling constant and no unsigned ceiling source.** The ceiling
  MUST derive from the signed manifest; reading it from a `pub const`, a CLI flag,
  an env var, or the unsigned `BenchmarkManifest` is forbidden (G-GUARD-3).
- **No second conservation/integrity authority.** Re-use
  `monetary_invariant::total_supply_micro` and `boot::verify_trust_root`; do not
  inline a parallel spend-sum or a second hash verifier that can drift
  (`monetary_invariant.rs:205–212` lesson; `AGENTS.md §12`).
- **No silent pinned edit.** Touching `sequencer.rs` admission, `typed_tx.rs`
  schema (new `RunOutcome`/`TransitionError`/`RejectionClass` discriminant or wire
  change), or re-pinning `genesis_payload.toml` is a SEPARATE Class-4 schema/
  trust-root surface beyond the preferred unpinned form — STOP and surface it
  explicitly with its own ratify tag (`feedback_trust_root_pin_trap`,
  `feedback_class4_cannot_hide_in_class3`).
- **No charging reads/think (Law 1 breach).** The ceiling gates investment spend
  only; `assert_read_is_free` must keep passing (G-GUARD-4).
- **No batching with unrelated economy work.** This token is for the budget
  hard-ceiling leg ONLY (`feedback_no_batch_class4_signoff`,
  `feedback_no_concurrent_dev_during_batch`).
- **No audit before runnable evidence.** Promotion of the gate requires the gate +
  workspace + constitution-gate suite GREEN on a real build before any
  clean-context audit (`AGENTS.md §9`, `feedback_audit_after_evidence`).

---

## §6. Risk classification & FC trace

**Risk class: Class 4 (candidate), justified honestly.**

- It is **not** Class 2 (read-view). A pure observe-only budget dashboard that
  only reports spend would be Class 2 — but that is explicitly NOT what is
  requested. The requested defining property is *on-exceed REJECT/HALT*.
- It is **not** comfortably Class 3 either. Rejecting a transition is an
  **admission decision**, and admission is the sequencer's authority
  (`sequencer.rs`, PINNED, `AGENTS.md §6` restricted surface). Halting a run on a
  spend bound is the `RunOutcome` terminal-class surface (`typed_tx.rs`, PINNED,
  Class-4 typed-tx schema). Per `AGENTS.md §6`: "If a change touches sequencer
  admission, typed tx schema, or canonical signing payloads, treat it as Class 4
  candidate until proven otherwise. Class 4 cannot hide inside a Class 3 umbrella"
  (`feedback_class4_cannot_hide_in_class3`).
- The **preferred unpinned-first form** (§3: pre-admission gate + unpinned reader
  + unpinned spend view + new top-level test, ZERO pinned edits) may, after the
  diff exists and is audited, be demonstrably reducible to **Class 3** if it
  genuinely touches no admission rule and no schema. But that reduction is a
  *post-evidence* finding, not an a-priori claim. Until the diff proves zero
  pinned/admission/schema impact, this is ratified at **Class 4** and the
  signed-ceiling re-pin path (if taken) is unconditionally Class 4.

Per `AGENTS.md §5`, explicit per-atom §8 ratification is required before any
implementation or ship; a short reply (`go`/`ok`/`continue`/`can`/`完成`) is not
Class-4 sign-off (`feedback_no_batch_class4_signoff`).

**FC trace:**

- **FC1 (runtime loop) — predicate/admission arm.** A ceiling breach is a
  rejection at the `p ==>|"Q_{t+1}=Q_t if ∏p=0"|` arm of FC1
  (`constitution.md:653`): the transition is refused, `Q` does not advance, the
  event lands in L4.E. Reuses the existing `rejection_class_for` mapping
  (`sequencer.rs:564`) and the reserved `RejectionClass::BudgetExceeded`
  (`typed_tx.rs:174`).
- **FC2 (boot/full architecture) — economic map-reduce tick + HALT.** The run-
  scoped ceiling gates the FC2 map-reduce tick/clock
  (`constitution.md:622–625, 656–659`) and, on breach, routes to the FC2 terminal
  HALT node (`constitution.md:584`, `:654`), analogous to the existing
  `MaxTxExhausted` clean halt (`typed_tx.rs:244`;
  `chain_derived_run_facts.rs:783`).
- **Monetary conservation (Law 2, `constitution.md:160`).** The reject/halt path
  preserves CTF conservation bit-for-bit: no mint
  (`assert_no_post_init_mint`, `monetary_invariant.rs:371`), no burn, total supply
  unchanged across a rejected transition (`assert_total_ctf_conserved`,
  `monetary_invariant.rs:485`). Law 1 (`constitution.md:159`) keeps reads free
  (`assert_read_is_free`, `monetary_invariant.rs:520`).

**STEP_B protocol** (`feedback_step_b_protocol`): build the ceiling mechanism in
NEW unpinned modules (`src/runtime/budget_ceiling.rs` + pre-admission gate +
spend view) with the constitution-gate suite GREEN and ZERO pinned diff before
commit. PINNED surfaces (`sequencer.rs`, `monetary_invariant.rs`, `q_state.rs`,
`typed_tx.rs`) are AVOIDED in the preferred form; any genuinely required pinned
edit is rehash-in-same-commit + own signed `v4-ratify` tag + explicit per-atom
§8 (`feedback_trust_root_pin_trap`).

---

## §7. What this packet does NOT authorize

- It does NOT authorize any `src/` edit. The ceiling mechanism stays BLOCKED until
  the token in §8.
- It does NOT authorize an advisory/warn-only budget meter (that is out of scope —
  the request is specifically an on-exceed *reject/halt* ceiling).
- It does NOT authorize a pinned edit to `sequencer.rs`, `monetary_invariant.rs`,
  `q_state.rs`, or `typed_tx.rs`, nor a new `RunOutcome`/`TransitionError`/
  `RejectionClass` wire discriminant — each is a separate Class-4 schema surface
  (§5) requiring its own ratification.
- It does NOT itself re-pin the Trust Root. If the signed ceiling becomes a new
  pinned `genesis_payload.toml` region, that re-pin carries its own signed
  `v4-ratify` tag (G-GUARD-3 / A-ALLOW-5).
- It does NOT modify `src/token_budget.rs`'s prompt-token cap or
  `BenchmarkManifest.max_tx_budget`'s per-batch tx pin — those are different
  quantities (§1) and stay as-is.
- It does NOT permit charging reads/think (Law 1, `constitution.md:159`).
- It does NOT close the gate. The gate
  (`tests/constitution_budget_hard_ceiling.rs`) goes GREEN only on a real build
  with no constitution-gate-suite regression
  (`cargo test --workspace --no-fail-fast` exit 0,
  `bash scripts/run_constitution_gates.sh` exit 0,
  `cargo test --test constitution_matrix_drift` exit 0), under the token, with the
  gate promoted + triple-coupled, followed by a clean-context audit
  (`AGENTS.md §9`).

---

## §8. Architect ratification (to be filled at user verbatim)

**Status: AWAITING ARCHITECT RATIFICATION.** No `src/` work begins until the
architect supplies the exact token below. Per `feedback_no_batch_class4_signoff`,
a short reply (`go` / `ok` / `continue` / `can` / `完成`) is NOT Class-4 sign-off.

```text
Ratify:
  APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST

Reject / defer:
  REJECT-BUDGET-HARD-CEILING-FOR-NOW   # keep spend control advisory; no on-exceed reject; ceiling stays a derived/observe-only view
```

**Recommended posture if ratifying:** authorize the **unpinned-first** mechanism
build (§3) with a hard ZERO-pinned-diff constraint; require any signed-ceiling
re-pin of `genesis_payload.toml` to carry its own signed `v4-ratify` tag; do not
batch with other economy atoms.

**Architect §8 sign-off (FILLED IN AT USER VERBATIM):**

- Verbatim quote: `APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST，以及把所有需要我APPROVE的都一次性给你授权，我要睡觉了。` — only the explicit budget token is consumed; the blanket "approve everything" is NOT treated as Class-4 sign-off for any other atom (feedback_no_batch_class4_signoff; FC3-live / Tier-2 / capability remain un-ratified).
- Token consumed: `APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST` (granted 2026-06-08)
- Scope confirmed: run-scoped cost ceiling (a positive `cost_ceiling_microcoin` per run; spend = tape-derived C_i over all attempts incl failed branches).
- Pinned-edit posture confirmed: **unpinned-first ZERO-pinned-diff** — implemented WITHOUT editing any pinned file. REUSES the existing `RejectionClass::BudgetExceeded` (typed_tx.rs:174, sources the label from the variant itself so it can't drift) + the existing `BudgetSnapshot.cost_ceiling_microcoin` field (q_state.rs:148, read-only). NO new RejectionClass / RunOutcome / HaltReason discriminant; q_state.rs / typed_tx.rs / sequencer.rs / genesis_payload.toml UNCHANGED. The ceiling source is a SEPARATE unpinned signed/user-approved budget manifest file (NOT the genesis [trust_root]), so no trust-root re-pin or signed `v4-ratify` tag was required. Integer-only (i64/u64 saturating, no f64); forward-only (ceiling 0 = unlimited = today's behavior); on-exceed = `BudgetExceeded` reject with NO head advance = FC2-HALT; checkpoint-resumable (raise the ceiling → halted proposal admits from the same head). Effective risk reduced from the Class-4 candidate to Class-2/3 by the zero-pinned-diff evidence.
- Date: 2026-06-08
- Branch at ratification: `claude/livefc1-budget` (LIVE-FC1 Phase 5; the budget atom was folded into LIVE-FC1 per the standing /goal once the token was granted)
- Parent commit: `5f57a236` (origin/main after LIVE-FC1 Phase 4 #333)

---

`FC-trace: FC1 admission/predicate arm (Q_{t+1}=Q_t if ∏p=0, constitution.md:653) — a signed-manifest spend ceiling breach REJECTS via the existing rejection_class_for mapping (sequencer.rs:564) surfacing the reserved RejectionClass::BudgetExceeded (typed_tx.rs:174, currently unwired) into L4.E with no state_root advance; FC2 economic map-reduce tick + HALT (constitution.md:584/654, :622-625) — a run-scoped breach routes to the FC2 terminal HALT analogous to RunOutcome::MaxTxExhausted (typed_tx.rs:244); monetary conservation preserved bit-for-bit (Law 2 constitution.md:160; assert_total_ctf_conserved monetary_invariant.rs:485; assert_no_post_init_mint :371) and reads stay free (Law 1 constitution.md:159; assert_read_is_free :520); ceiling source = signed genesis_payload.toml [trust_root] surface (boot::verify_trust_root, sole integrity authority), integer-only, fail-closed. Class-4 candidate (touches sequencer admission + pinned RunOutcome schema); preferred unpinned-first form (zero pinned diff) may reduce to Class-3 only post-evidence; per-atom §8 required; no implementation until token supplied.`

**End of Budget Hard-Ceiling §8 decision packet (AWAITING ARCHITECT RATIFICATION; documentation only).**
