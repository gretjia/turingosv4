# Thesis v2 audit — "feedback from ground truth" alignment

**Date**: 2026-04-25 (post user thesis update — added "feedback from ground truth" as physical anchor)
**Purpose**: re-audit current Phase B7-extra implementation against the new claim 7 ("white-box predicates settle state transitions based on STRICT FEEDBACK FROM GROUND TRUTH") and claim 8 ("failure logs MUST be ground-truth-validated before quarantine").
**Status**: 2 alignment gaps identified, both Phase D scope (consumer-side), neither blocks Phase B → C.

---

## Causal chain audit

```
Proposal (LLM)
  → Feedback from Ground Truth (Lean / FS / external compiler)
  → Logging (ground-truth-validated, isolated from active context)
  → Capability Compilation (Phase D)
  → ↑ H-VPPUT
```

For each link, where does the current Phase B7-extra system stand?

### Link 1 — Proposal (black-box LLM)
- ✅ Aligned. `client.generate` → `parse_agent_output` → `AgentOutput { tool: append/complete/invest/search }`. The LLM never settles state; it only proposes.

### Link 2 — Feedback from Ground Truth
- ✅ **Aligned for capability claims** (`complete` action triggering `oracle.verify_omega_detailed` at evaluator.rs:777,783,1016). Lean compiler returns objective verdict.
- ✅ Aligned for Trust Root (`boot::verify_trust_root` — FS hash reality).
- ⚠️ **Distinction**: non-capability transitions (`append` for tape comments, `invest` for market stakes, `search` for retrieval) are settled by **policy** (`forbidden_patterns`, `Sorry`, `PayloadSize`, market math) — NOT ground-truth. Per thesis spirit, this is acceptable: only capability claims need ground-truth. Tape comments are organisational; market stakes are economic; neither claims capability.
- 📅 Phase E sealed eval on heldout-54 = ground-truth on capability **generalization** (Gate H requirement).

### Link 3 — Logging (ground-truth-validated, isolated from active context)

Two surfaces:

#### 3a. WAL / `Ledger` (durable, structural)
- `EventType` enum **defines** OmegaInvoke / OmegaAccepted / OmegaRejected / OmegaError (`src/ledger.rs:155-158`)
- ❌ **GAP (Finding C)**: these Omega* events are **never emitted in production code**. Only RunStart, Append, Invest, MarketCreate, MarketResolve, RunEnd reach the ledger. The structured oracle verdict trail does not exist in the WAL.
- Phase D ArchitectAI reading WAL alone would see NO Lean verdicts; it would have to infer success from Append events and absence of Append events from "no proposal" — losing the distinction between "tried + rejected" and "never tried".

#### 3b. `recent_rejections` (in-memory queue, fed to next-tx agent prompt)
- On oracle reject (evaluator.rs:876): `bus.record_rejection(agent_id, class.label())`
- The CLASS LABEL is an abstracted classification from `classify_lean_error(err_detail)` — deterministic transform of ground-truth output, NOT raw oracle stderr.
- ✅ **Aligned for thesis spirit (claim 8)**: agents see abstracted labels (e.g., "TacticFailed"), not raw ground-truth content. The strong physical isolation = "raw oracle stderr is in stdout, not in agent prompt". Step-B v3 (C-022 shield) was specifically introduced to enforce this.
- ⚠️ **GAP (Finding D)**: `recent_rejections` mixes class labels from ground-truth rejects (Lean verdict) with class labels from policy rejects ("Forbidden pattern: decide", "Payload too long: ..."). Phase D consumer reading these labels has no field telling it which is which. Ground-truth rejects and policy rejects should be DIFFERENT signals to ArchitectAI.

#### 3c. `PputResult` jsonl emit (durable, per-run summary)
- ✅ **Aligned**. `verified: bool` field comes from `oracle.verify_omega_detailed` (B4 post_hoc_verifier separates `runtime_accepted` from `post_hoc_verified`). Each emitted row IS ground-truth-validated at run granularity.
- For PPUT-CCL Phase D, the calibration jsonl IS the ground-truth-validated log Phase D consumes for capability compilation.

### Link 4 — Capability Compilation (Phase D, not yet built)
- 📅 Will read PputResult jsonl + WAL + stdout/stderr
- The thesis-v2 critical requirement: ArchitectAI proposals MUST be tagged with which leg of the ground-truth feedback they're learning from. Phase D design must account for Findings C and D.

### Link 5 — ↑ H-VPPUT (held-out verified PPUT)
- 📅 Phase E sealed eval on heldout-54
- ✅ Trust Root + B7 Boot freeze + B7-extra p_0 calibration build the measurement scaffolding for this. North Star is achievable iff the chain above is honored.

---

## Findings (C, D — extending B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md)

### Finding C — WAL Omega* events never emitted

**What**: `EventType::{OmegaInvoke, OmegaAccepted, OmegaRejected, OmegaError}` are declared in `src/ledger.rs:155-158` but no production code path emits them. The WAL contains only structural events; ground-truth verdicts are visible in stdout/jsonl, not in the durable ledger.

**Why it matters per thesis-v2**: Claim 8 says ground-truth-validated failure logs must be quarantined for capability compilation. The WAL is the canonical durable log surface; if it lacks ground-truth events, Phase D ArchitectAI must reach into other surfaces (jsonl + stdout) to reconstruct ground-truth feedback. The reconstruction is doable but fragile.

**Severity**: Phase D scope. Phase B's calibration jsonl is correctly ground-truth-validated at run granularity; intra-run ground-truth events are missing from WAL but will surface in stderr (raw) + jsonl (`verified` + tool_dist counters).

**Action**: Phase D ArchitectAI design must specify which surface(s) it reads. Recommend: jsonl as primary ground-truth source, WAL as structural cross-reference, stderr as fallback for fine-grained diagnostics. OR (more invasive): emit OmegaInvoke/Accepted/Rejected/Error to ledger from `oracle.verify_omega_detailed` call sites (touches evaluator.rs but not bus.rs/kernel.rs — outside STEP_B_PROTOCOL restricted-file zone).

### Finding D — `recent_rejections` mixes policy + ground-truth class labels

**What**: `bus.record_rejection(author, reason)` is called from BOTH:
- Policy paths: forbidden_pattern (`bus.rs:186`), payload too long (`bus.rs:198`), too many lines (`bus.rs:205`), tool veto (`bus.rs:215`)
- Ground-truth path: oracle reject after classification (`evaluator.rs:876`)

The `recent_rejections(author, max)` consumer (line 594, fed into agent prompt) sees a flat list of class labels, no provenance tag.

**Why it matters per thesis-v2**: Claim 7 says white-box predicates settle state with ground-truth feedback. For agent next-tx learning (FC1's basic cycle), labels of any kind may be acceptable — agents adapt to any signal that helps them produce non-rejectable output. But for Phase D ArchitectAI compiling capability, the signal source matters: ArchitectAI should learn from ground-truth (Lean said this proof tactic doesn't work) differently than from policy (we banned `decide` for brute-force-prevention).

**Severity**: Phase D scope. Current implementation is acceptable for Phase B production runs (control); does not corrupt p_0 calibration (compute_p0.py only joins on SOLVED/UNSOLVED, ignores recent_rejections content).

**Action**: when Phase D ArchitectAI consumer is built, tag bus rejections at the call site with `provenance: GroundTruth | Policy` — extend BusResult::Vetoed or add a separate `record_rejection_classified(author, reason, provenance)` API. Backwards-compatible if the existing `record_rejection` defaults provenance to Policy (since 4 of 5 call sites are policy).

---

## Why neither finding blocks Phase B → C

Phase B delivers MEASUREMENT SCAFFOLDING. The PPUT-CCL arc Gate B → C transition validates:
- A1-A5 ✓
- B1-B7 ✓ (Trust Root + Boot freeze landed)
- B7-extra (in flight): p_0 calibration via 576 runs

The calibration jsonl rows are **per-run ground-truth-validated** (`verified` field comes from Lean4Oracle). Findings C+D are about **intra-run** ground-truth event granularity (per-tx, per-proposal) — needed by Phase D ArchitectAI but not by Phase B's measurement gate.

Thesis-v2 alignment status:
- Phase B: ✅ aligned (per-run ground-truth-validated jsonl is the measurement surface)
- Phase C: should plan for Finding C/D before Phase D consumer is built
- Phase D: MUST close Findings C+D as a precondition

---

## Cross-references

- `~/.claude/.../memory/project_thesis.md` — thesis v2 with 5-step compile loop + 11 atomic claims
- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` — Findings A+B (abstraction depth)
- `src/ledger.rs:147-160` — EventType enum (Omega* declared, never emitted in production)
- `src/bus.rs:186,198,205,215,444` — record_rejection call sites (4 policy + 1 from evaluator at oracle reject)
- `experiments/minif2f_v4/src/bin/evaluator.rs:876` — bus.record_rejection at oracle reject
- `experiments/minif2f_v4/src/lean4_oracle.rs:112,177` — Lean ground-truth verdict surfaces
- PREREG § 1.7 ArtifactState lifecycle — Phase D consumer specification anchor
