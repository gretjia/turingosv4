# §8 Decision Packet — FC3 Runtime Veto-AI Gate + Trust-Root Recompute + Re-init (the IRREVERSIBLE leg)

**Status**: **AWAITING ARCHITECT RATIFICATION.** No implementation happens until
the architect supplies the exact token in §7. This document is **Class-0
documentation only** — it describes a Class-4 self-evolution-closure change,
requests per-atom §8 ratification, and authorizes nothing by itself.

**Date**: 2026-06-07
**Branch**: `claude/fc3-observable-canary` (base `origin/main`).
**Risk class**: **Class 4 — HIGHEST blast radius in this repository.** This is
the system rewriting its own Trust Root and re-initializing the process to
activate code it generated. It touches RootBox / boot trust-root authority
(`src/boot.rs`, `genesis_payload.toml`) and the constitution-amendment boundary
(Art. V.1.1). Per `AGENTS.md §5–§6`, it requires explicit **per-atom §8**
ratification before any implementation or ship. Short replies (`go`, `ok`,
`continue`, `can`, `完成`) do **not** constitute Class-4 sign-off
(`feedback_no_batch_class4_signoff`).

**Recommendation (operating posture):** ratify and execute this leg **WITH
ACTIVE ARCHITECT SUPERVISION — not unattended.** Given the blast radius (a PASS
candidate triggers a trust-root rewrite + process re-init), the architect should
be present and watching the tape for the first activations, with the manual
abort path ready, rather than authorizing an autonomous overnight loop. The
autonomous-execution authorization (2026-05-07) explicitly carves Class-4 STEP_B
+ per-atom §8 OUT of auto-execute; this leg is the canonical example of why.

**Proposed §8 token** (the architect replies with this exact phrase to ratify):

```text
APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT
```

```text
Reject / defer option:
  REJECT-FC3-IRREVERSIBLE-LEG-FOR-NOW   # keep the loop OPEN at canary; observable+canary half stays the terminal state; G5 stays standing-pending
```

**Authority chain**:
- Constitution: `constitution.md` Art. V.1.1 (line 704 — 宪法是唯一基准真相;
  human sudo 仅作用于 `constitution.md` 本身), Art. V.1.2 (line 719 + 736 —
  ArchitectAI 对 Trust Root manifest 的 commit 权限，经 Veto-AI PASS 后落盘，
  **不**需要人类 sudo，但 `constitution.md` 永远在范围外), Art. V.1.3
  (line 740 — Veto-AI 输出域 `{PASS, VETO}`，白名单严格排除主观评判),
  **Art. V.2 (line 798 — 任何状态变更必须具有可逆性，总是能够回滚到 Q_{t-1})**,
  Art. V.3 (line 804 — 宪法修订只由人类 sudo 触发).
- The standing-pending closure gate this leg promotes:
  `tests/pending/constitution_fc3_meta_loop_closure.rs` (G5) — RED by design;
  its header states promotion REQUIRES §8 Class-4 ratification of exactly this
  irreversible-commit path.
- The observable+canary half (the ONLY FC3 runtime leg authorized so far):
  token `APPROVE-FC3-OBSERVABLE-CANARY-HALF`; modules `src/runtime/fc3_proposer.rs`
  + `src/runtime/fc3_canary.rs`; gate
  `tests/constitution_fc3_proposer_canary_observable.rs` (GREEN);
  matrix row `Art. V.1.2 FC3 OBSERVABLE + CANARY half` in
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`.
- The M07 single-admission predicate floor (precondition):
  `handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md`;
  `src/predicate_admission.rs` (`decide_admission`, G3 zero-root quarantine);
  `src/memory_kernel.rs:204` + `src/state/q_state.rs` (`os_qualified_t`).
- The FC3 governance typed-tx substrate (already live, base `origin/main`):
  `src/state/typed_tx.rs` — `VetoVerdict{Pass,Veto}` (line 1878),
  `VetoDecisionTx` / `VetoDecisionCapsule` (line 1999 / 2019),
  `ArchitectCommitTx` / `ArchitectCommitCapsule` (line 2033 / 2051),
  `ReinitRequestTx` (line 2084), `ReinitBootTx` (line 2105), with sequencer
  transition + admission arms in `src/state/sequencer.rs` (e.g.
  `ArchitectCommitBlockedByVeto`, `ReinitBootReplayedRootMismatch`, lines
  649–666) and `src/bin/fc3_governance_reinit_current_kernel.rs`.
- Constitution binding: `AGENTS.md §5–§6, §9, §14`; `CLAUDE.md §3–§4`;
  `feedback_trust_root_pin_trap`; `feedback_class4_cannot_hide_in_class3`;
  `feedback_no_workarounds_strict_constitution`.

---

## §1. Decision statement (what closing FC3 means)

**Close the FC3 self-evolution loop:** wire a runtime **Veto-AI `{PASS,VETO}`
clause-walker** that gates an **`ArchitectCommit`**, which on PASS performs a
**boot Trust-Root manifest rewrite** and triggers a **process re-init** that
brings the PASS candidate LIVE.

Today the loop is OPEN by design. The observable+canary half produces a
tape-anchored proposal (`fc3_proposer`) and a tape-anchored canary metric
(`fc3_canary`), but the terminal disposition of even a PASSING candidate is the
dead-end `CANARY_ONLY_TERMINAL_STATUS = "sandbox:canary_only"`
(`src/runtime/fc3_canary.rs:52`). `closes_fc3_loop(terminal) == false` is
asserted on tape. Nothing the runtime emits today changes the Trust Root or
activates code.

This leg removes that dead-end: it adds the path from a Veto-AI PASS to a
tape-visible re-init that recomputes the boot Trust Root and advances the
constitution-bound state so the candidate is actually running. This is FC3
edge `… -> ArchitectAI proposal -> Veto-AI verdict -> tools/logs -> re-init`
finally closing (`constitution.md` ~line 826; TRACE_MATRIX FC3-N3x/N4x).

It is the part the observable+canary token (`APPROVE-FC3-OBSERVABLE-CANARY-HALF`)
**explicitly did NOT authorize** — both `fc3_proposer.rs` and `fc3_canary.rs`
name this leg as a HARD NON-GOAL with NO token.

---

## §2. Precondition: what is ALREADY live (this leg builds on it, does not re-do it)

This packet is requestable only because the safe two-thirds of the loop, the
admission floor, and the ground-truth oracle are already standing and
tape-anchored. Do not re-authorize these; they are the load-bearing foundation
this irreversible leg sits on.

| Precondition | Status | Evidence |
|---|---|---|
| **Observable half** — proposer emits a REAL `ArchitectProposalCapsule` (non-`Noop` kind + `target_path` + `proposed_artifact_cid`) via the EXISTING `SystemEmitCommand::ArchitectProposal` path; no new typed-tx variant. | LIVE | `src/runtime/fc3_proposer.rs`; gate `tests/constitution_fc3_proposer_canary_observable.rs` GREEN. |
| **Canary / safety-evaluation half** — candidate scored against the EXISTING executable `Predicate` (`top_white::predicates::registry`); integer-only `MetricEstimate` written to CAS; terminal status stays `sandbox:canary_only`, loop does NOT close. | LIVE | `src/runtime/fc3_canary.rs`; same gate; `closes_fc3_loop(terminal)==false` on tape. |
| **M07 predicate floor** — admission refuses an OS-qualified run backed by a zero predicate-registry root (G3 zero-root quarantine); `os_qualified_t` is a first-class Q field. | LIVE | `src/predicate_admission.rs` (`decide_admission`); `src/memory_kernel.rs:204`; `src/state/q_state.rs` `os_qualified_t`; `handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md`. |
| **External ground-truth oracle** — the Lean checker the canary scores against is the real external oracle, not a self-graded shell. | LIVE | `LeanArtifactPredicate` / `lean4_oracle` in `src/top_white/predicates/registry.rs:944`. |

Because these are live, the residual risk of THIS leg is concentrated entirely
in the single new authority it grants: turning a canary PASS into a real
trust-root rewrite + re-init. Everything before that point is already observed
and tape-anchored.

---

## §3. Allowed engineering actions (only under the §8 token)

The following are the **only** moves authorized once the architect supplies the
token. Each touches Class-4 trust-root / re-init authority and is BLOCKED until
then. Prefer NEW unpinned runtime modules (`src/runtime/fc3_*.rs`) over editing
PINNED surfaces.

- **A-ALLOW-1 — runtime Veto-AI clause-walker (NEW module, e.g.
  `src/runtime/fc3_veto.rs`).** A deterministic walker that, given a synthesized
  proposal capsule, walks a fixed enumerated set of constitutionality checks and
  emits exactly one of `VetoVerdict::Pass` / `VetoVerdict::Veto` (the existing
  `{PASS,VETO}` domain, `src/state/typed_tx.rs:1878`). It feeds the EXISTING
  `VetoDecisionTx` / `VetoDecisionCapsule` path; it MUST be whitelisted to
  constitutionality only (Art. V.1.3: 白名单严格排除 主观质量 / 性能 / 覆盖率 /
  任何与违宪与否无关的判定). Output domain is exactly two values — no score, no
  ranking, no "confidence".
- **A-ALLOW-2 — drive `ArchitectCommit` only on PASS, via the EXISTING tx
  path.** On `VetoVerdict::Pass`, the runtime drives the existing
  `ArchitectCommitTx` / `ArchitectCommitCapsule` through `emit_system_tx`; the
  sequencer's existing `ArchitectCommitBlockedByVeto` arm
  (`src/state/sequencer.rs`) must reject any commit not backed by a recorded
  PASS. The runtime never hand-builds or hand-signs the typed tx — `emit_system_tx`
  does (preserving the Anti-Oreo barrier), exactly as `fc3_proposer` does today.
- **A-ALLOW-3 — Trust-Root manifest rewrite scoped to Art. V.1.2 (NEVER
  `constitution.md`).** A PASS-gated commit may recompute the pinned-hash
  manifest for the candidate's touched payload files (the ArchitectAI upgrade
  range per Art. V.1.2 line 736: kernel / oracle / predicate-registry / scoring
  pipeline / cases, i.e. entries of `genesis_payload.toml`). The recompute MUST
  reuse the single existing verifier authority (`src/boot.rs::verify_trust_root`,
  the KEEP-SRC-BOOT-ratified sole verifier) for the post-rewrite integrity check;
  it must NOT introduce a second hash authority.
- **A-ALLOW-4 — re-init via the EXISTING `ReinitRequestTx` / `ReinitBootTx`
  path.** Activation of a PASS candidate happens through the existing re-init txs
  (`src/state/typed_tx.rs:2084 / 2105`) and their sequencer arms
  (`ReinitRequestTriggerNotErrorHalt`, `ReinitBootReplayedRootMismatch`, …). The
  re-init is the tape-visible event that flips `closes_fc3_loop(terminal)` from
  `false` to `true`. Every activation is one ChainTape system tx + CAS capsule;
  no out-of-band activation.
- **A-ALLOW-5 — promote the standing-pending closure gate (NEW top-level gate).**
  `tests/pending/constitution_fc3_meta_loop_closure.rs` (G5) is git-moved to a
  top-level `tests/constitution_*.rs` gate and triple-coupled (manifest +
  `CONSTITUTION_EXECUTION_MATRIX.md` row + `ls tests/constitution_*.rs` glob,
  per `feedback_constitution_gate_triple_coupling`) only once both its
  observations flip GREEN on a real build: (A) the live role-path proposal
  carries a real spec, and (B) a Veto-AI PASS reaches a loop-closing re-init
  terminal, not the `sandbox:canary_only` dead-end.

**Sourcing constraint (binding):** no new hardcoded behavior parameter
(`CLAUDE.md §4`). The Veto-AI clause set derives from `constitution.md` /
`constitution_source_hash()`; the manifest hashes derive from
`genesis_payload.toml`; metrics stay integer-only (no `f64` in the metric or any
money/conservation path).

---

## §4. Hard guards (binding even under the token)

These are the constitutional guard-rails that make the irreversible leg
admissible. If any cannot be met, the leg STOPS — do not weaken a guard into a
skip (`feedback_no_workarounds_strict_constitution`).

- **G-GUARD-1 — Veto-AI is deterministic + whitelisted to constitutionality.**
  Output domain is exactly `{PASS, VETO}` (Art. V.1.3, line 765). No subjective
  quality / performance / coverage / architecture-preference judgment may enter
  the walker; those belong to the independent non-Veto-AI clean-context auditor
  (`AGENTS.md §9, §14`), never to the runtime gate. The walker must be
  replay-stable (same proposal → same verdict), no probabilistic model (Art. V.2
  line 799: 核心谓词逻辑必须是确定性的，禁止引入概率模型).
- **G-GUARD-2 — `constitution.md` is OUT of the rewrite range (Art. V.1.1).**
  The Trust-Root rewrite may touch ArchitectAI-range payload entries
  (`genesis_payload.toml` payloads per V.1.2) but MUST refuse any candidate that
  proposes mutating `constitution.md` itself — that requires human sudo and is
  rejected by Veto-AI (the existing `VetoVerdict::Veto` path), never auto-committed.
- **G-GUARD-3 — every activation is tape-recorded AND reversible (Art. V.2,
  line 798).** Each PASS → commit → trust-root-rewrite → re-init step is one
  ChainTape system tx + CAS capsule (no out-of-band activation), and the
  pre-activation Q must be reconstructable so the system can roll back to
  `Q_{t-1}` (Art. V.2: 任何状态变更必须具有可逆性). An activation that cannot be
  rolled back to the prior Q is FORBIDDEN. The re-init records the prior
  trust-root / state so rollback is a tape operation, not a manual rebuild.
- **G-GUARD-4 — the trust-root rewrite needs its own signed `v4-ratify` tag.**
  The actual re-pin of `genesis_payload.toml` (changing pinned bytes) is the
  trust-root-pin trap (`feedback_trust_root_pin_trap`): the rehash must land in
  the same commit as the payload change, and the resulting trust-root change is
  anchored by its own **signed `v4-ratify` tag** — separate from this §8 token.
  The §8 token authorizes BUILDING the closure mechanism; each concrete
  candidate that actually rewrites the live boot manifest is gated by its own
  signed ratify tag at activation time. (This packet's atom edits NO pinned file;
  if a future variant must re-pin at build time, that is a further Class-4 surface.)
- **G-GUARD-5 — fail-closed everywhere.** Veto-AI error / ambiguous verdict →
  `VETO` (admission fail-closed default). A trust-root post-rewrite integrity
  check failure → abort the activation, do NOT bring the candidate live. No
  bypass env var, no `catch_unwind`, no `SKIP/ALLOW_TRUST_ROOT` surface (the
  KEEP-SRC-BOOT ratification forbids any bypass surface).
- **G-GUARD-6 — active human supervision for first activations.** Per the
  recommendation above: the first real PASS→re-init activations run with the
  architect watching the tape and the abort path ready, not as an unattended
  autonomous loop. This is an operating-posture guard, codified here so it is not
  lost.

---

## §5. Forbidden (even under the token)

- **No `constitution.md` write, ever.** Art. V.1.1: human sudo only. A candidate
  proposing a constitution edit is VETO'd, never committed.
- **No irreversible activation.** Any activation that cannot roll back to
  `Q_{t-1}` violates Art. V.2 and is forbidden — reversibility is a hard
  precondition of the commit, not a nice-to-have.
- **No second Veto-AI authority and no second trust-root verifier.**
  `src/boot.rs::verify_trust_root` stays the sole verifier; the runtime Veto-AI
  is the sole constitutionality gate. No inline re-implementation that can drift.
- **No subjective judgment in the runtime gate.** Code style / performance /
  coverage / architecture preference are out-of-scope for Veto-AI (Art. V.1.3)
  and belong to the independent clean-context auditor.
- **No typed-tx SCHEMA change under this token.** This leg emits via the EXISTING
  `VetoDecisionTx` / `ArchitectCommitTx` / `ReinitRequestTx` / `ReinitBootTx`
  paths. A new `SystemEmitCommand` variant or typed-tx wire/discriminant change is
  a SEPARATE Class-4 schema surface beyond this token — STOP and request a fresh §8.
- **No batching with the observable/canary or M07 legs.** This token is for the
  irreversible leg ONLY (`feedback_no_batch_class4_signoff`).
- **No audit before runnable evidence.** Promotion of G5 requires both
  observations GREEN on a real build + no constitution-gate-suite regression,
  before any clean-context audit (`AGENTS.md §9`, `feedback_audit_after_evidence`).

---

## §6. Risk classification & FC trace

**Risk class: Class 4, HIGHEST blast radius.** The change grants the runtime the
authority to rewrite its own boot Trust Root and re-initialize to activate
self-generated code. This is the constitution-amendment-adjacent boundary
(Art. V.1.1/V.1.2) and the RootBox/boot trust-root surface (`src/boot.rs`,
`genesis_payload.toml`). Class-4 cannot hide inside a Class-3 umbrella
(`feedback_class4_cannot_hide_in_class3`). Per `AGENTS.md §5`, explicit per-atom
§8 ratification is required before any implementation or ship.

**FC trace (FC3 meta-loop closure):**
- **FC3-N3x/N4x** — closes the edge
  `logs+constitution -> archived feedback -> ArchitectAI proposal ->
  Veto-AI verdict -> tools/logs -> re-init` (`constitution.md` ~line 826).
- **FC3-N32/N43** — runtime Veto-AI `{PASS,VETO}` verdict (`VetoDecisionTx` /
  `VetoDecisionCapsule`).
- **FC3-N44/N45** — re-init (`ReinitRequestTx` / `ReinitBootTx`), the
  loop-closing tape-visible event.
- **FC2 boot** — the trust-root recompute re-enters the FC2 boot guard; the
  re-init must pass `verify_trust_root` post-rewrite (ties to the
  all-canonical-writers-verify-trust-root atom).

**STEP_B protocol** (`feedback_step_b_protocol`): build the closure mechanism in
NEW unpinned runtime modules (`src/runtime/fc3_veto.rs` + the commit/re-init
driver) with the gate suite GREEN before commit. PINNED surfaces
(`src/state/typed_tx.rs`, `src/state/sequencer.rs`, `src/state/q_state.rs`) are
AVOIDED — the leg drives them via the existing emit paths. Any re-pin of
`genesis_payload.toml` at activation is the rehash-in-same-commit rule
(`feedback_trust_root_pin_trap`) plus its own signed `v4-ratify` tag (G-GUARD-4).

---

## §7. What this packet does NOT authorize

- It does NOT authorize any `src/` edit. The closure mechanism stays BLOCKED
  until the token in §8.
- It does NOT re-authorize the observable/canary half or the M07 floor — those
  are already ratified and live (§2).
- It does NOT permit any `constitution.md` mutation (human sudo only, Art. V.1.1).
- It does NOT permit a typed-tx schema change or a new `SystemEmitCommand`
  variant — that is a separate Class-4 surface (§5).
- It does NOT itself re-pin the Trust Root; each concrete activation that rewrites
  the live boot manifest carries its own signed `v4-ratify` tag (G-GUARD-4).
- It does NOT close G5. G5 closes only when both its observations are GREEN on a
  real build with no constitution-gate-suite regression
  (`cargo test --workspace --no-fail-fast` exit 0,
  `bash scripts/run_constitution_gates.sh` exit 0,
  `cargo test --test constitution_matrix_drift` exit 0), under the token, with
  the gate promoted + triple-coupled.

---

## §8. Architect ratification (to be filled at user verbatim)

**Status: AWAITING ARCHITECT RATIFICATION.** No `src/` work begins until the
architect supplies the exact token below. Per `feedback_no_batch_class4_signoff`,
a short reply (`go` / `ok` / `continue` / `can` / `完成`) is NOT Class-4 sign-off
for the HIGHEST-blast-radius leg in the repository.

```text
Ratify:
  APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT

Reject / defer:
  REJECT-FC3-IRREVERSIBLE-LEG-FOR-NOW   # keep the loop OPEN at canary; G5 stays standing-pending
```

**Recommended posture if ratifying:** execute WITH active supervision (architect
present, watching the tape, abort path ready) for the first PASS→re-init
activations — not as an unattended autonomous loop. The blast radius (system
rewrites its own trust root + re-inits to activate self-generated code) warrants
a human in the loop the first times it fires.

**Architect §8 sign-off (FILLED IN AT USER VERBATIM):**

- Verbatim quote: _<to be filled — exact user words>_
- Token consumed: `APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT`
- Supervision posture confirmed: _<attended / explicitly-authorized-unattended>_
- Date: _<to be filled>_
- Branch at ratification: `claude/fc3-observable-canary`
- Parent commit: _<origin/main HEAD at ratification>_

---

`FC-trace: FC3-N3x/N4x meta-loop closure — runtime Veto-AI {PASS,VETO} clause-walker (FC3-N32/N43, VetoDecisionTx) gating an ArchitectCommit (ArchitectCommitBlockedByVeto) that recomputes the boot Trust Root (Art. V.1.2 ArchitectAI manifest range; constitution.md OUT per V.1.1) and triggers a tape-visible re-init (FC3-N44/N45, ReinitRequestTx/ReinitBootTx) that flips closes_fc3_loop false->true; every activation tape-recorded + reversible to Q_{t-1} per Art. V.2; trust-root re-pin carries its own signed v4-ratify tag. Class-4 HIGHEST blast radius; per-atom §8 required; no implementation until token supplied.`

**End of FC3 Runtime-Veto-and-Trust-Root-Reinit §8 decision packet (AWAITING ARCHITECT RATIFICATION; documentation only).**
