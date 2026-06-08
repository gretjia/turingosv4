# §8 Decision Packet — Capability Attenuation / Wallet-Gating (the WHO-may-invoke leg, complement of S4 arg-taint)

**Status**: **AWAITING ARCHITECT RATIFICATION.** No implementation happens until
the architect supplies the exact token in §8. This document is **Class-0
documentation only** — it describes a Class-4 candidate (privileged-sink
authority gate touching the §6 restricted `src/sdk/tools/wallet.rs` surface and
capability authority), requests per-atom §8 ratification, and **authorizes
nothing by itself**.

**Date**: 2026-06-08
**Branch**: `claude/capability-attenuation-wallet-gate` (base `origin/main`).
**Risk class**: **Class 4 candidate — capability authority + §6 restricted
surface.** Per `AGENTS.md §6` the wallet tool (`src/sdk/tools/wallet.rs`) is an
explicitly enumerated restricted surface, and "any trust-root or
constitution/flowchart authority surface" plus "any sequencer admission rule"
are Class-4 candidates until proven otherwise. Introducing a *capability /
authority* gate that decides WHO may drive a privileged sink is an authority
surface: it can hide a second admission authority if done wrong. Per
`AGENTS.md §5`, explicit per-atom §8 ratification is required before any
implementation or ship. Short replies (`go`, `ok`, `continue`, `can`, `完成`)
do **not** constitute Class-4 sign-off (`feedback_no_batch_class4_signoff`).

**Honest pin finding up front (determines whether this is STEP_B):** the live
wallet tool (`src/sdk/tools/wallet.rs`), the kernel admission seam
(`src/memory_kernel.rs`), the shared admission contract
(`src/predicate_admission.rs`), and the read-tool (`src/rtool.rs`) are **ALL
genesis pin-count 0 (UNPINNED)** — verified by
`grep -c '"src/sdk/tools/wallet.rs"' genesis_payload.toml` → `0`,
`'"src/rtool.rs"'` → `0`, `'"src/memory_kernel.rs"'` → `0`,
`'"src/predicate_admission.rs"'` → `0`. Only `src/bus.rs` (count 1) and
`src/bottom_white/tools/registry.rs` (count 1, the `Capability` enum +
`ToolMetadata` home) are pinned. **The core capability-attenuation mechanism can
therefore be built with ZERO pinned-file edits**, exactly mirroring the shipped
S4 arg-taint precedent — see §3. This is a Class-4 *authority* surface by
content (it gates who may reach a privileged sink), NOT by trust-root pin. No
STEP_B parallel-branch / pin rehash / signed `v4-ratify` tag is required **unless**
a chosen design must change the pinned `Capability` enum or `ToolMetadata` shape
in `registry.rs`, or add a `Capability`/permission check inside the pinned
`bus.rs` dispatch — those would each be a further Class-4 pin surface (§3, §6).

**Recommended posture:** ratify the **unpinned-only** form (build the gate in a
new `src/predicate_admission/cap_attenuation.rs` `#[path]` submodule, wired
through the already-unpinned `memory_kernel` admission seam). Defer/reject any
variant that requires editing the pinned `registry.rs` `Capability` enum or the
pinned `bus.rs` dispatch to a SEPARATE packet with its own pin rehash.

**Proposed §8 token** (the architect replies with this exact phrase to ratify):

```text
APPROVE-CAPABILITY-ATTENUATION-WALLET-GATE
```

```text
Reject / defer option:
  REJECT-CAPABILITY-GATE-FOR-NOW   # privileged sinks stay gated ONLY by S4 arg-taint (WHAT flows in); the WHO-may-invoke authority gate stays unbuilt
```

**Authority chain** (constitution articles + exact src `file:line`):
- Constitution: **Art. III 信号的选择性屏蔽** (`constitution.md:360` header;
  Art. III.4 Goodhart shielding `constitution.md:413-423` — the verification /
  authority logic must live in a region the black-box cannot reach or steer),
  **Art. I.1 谓词 Π_p** (the binary admission predicate boundary, referenced by
  `constitution.md:46` "谓词 Π_p (Art. I.1) + Veto-AI (Art. V.1.3) + 本宪法"),
  **Art. V.1.3 Veto-AI** (`constitution.md:740`; output domain `{PASS, VETO}`,
  `constitution.md:765` — objective constitutionality only, no subjective
  judgment), **Art. V.2 reversibility + determinism**
  (`constitution.md:798-799` — 任何状态变更必须具有可逆性；核心谓词逻辑必须是
  确定性的，禁止引入概率模型). Parallel-ledger discipline that makes wallet a
  derived view, not an authority of its own: `constitution.md:60-61, 72`
  (`WalletTool` is a tape-derived projection, not a source of truth).
- The S4 arg-taint privileged-sink definition this leg is the **complement** of
  (gates WHAT flows in; this leg gates WHO may invoke):
  `src/predicate_admission/arg_taint.rs:155-208` (`PrivilegedSink` enum +
  `SinkReason`), `:227-234` (`PRIVILEGED_WRITE_NAMESPACES` table incl.
  `"wallet/"`, `"capability/"`), `:240-260` (`classify_tool_sink` — wallet /
  oracle / sandboxed-exec capability ⇒ `SinkReason::PrivilegedCapability`),
  `:335-376` (`arg_taint_v1`), `:381-383` (`has_tainted_privileged_flow`).
- The shared single-admission oracle both legs route through:
  `src/predicate_admission.rs:149` (`decide_admission`),
  `:217` (`decide_admission_with_taint` — the unpinned hard-gate wrapper),
  `:28-29` (`#[path] = "predicate_admission/arg_taint.rs"` submodule precedent).
- The live wtool admission seam (UNPINNED) where a capability check would hook:
  `src/memory_kernel.rs:266` (`step_forward_with_taint`),
  `:291-292` (the `decide_admission_with_taint(...)` call site),
  `:294-349` (Pass commits a tape-recorded receipt + advances head; Fail routes
  to the existing non-advancing rejection path).
- The existing (pinned) capability vocabulary to attenuate against, NOT a new
  authority: `src/bottom_white/tools/registry.rs:23-34` (`Capability` enum —
  `EconomicWallet`, `LeanOracle`, `SandboxedExec`, …), `:36-42`
  (`PermissionPolicy::{Open, SystemOnly}`), `:70-86` (`ToolMetadata` incl.
  `capability` + `permission_policy`).
- The wallet tool itself (UNPINNED, §6 restricted): `src/sdk/tools/wallet.rs:32`
  (`struct WalletTool` — read-only projection, zero owned ledger state),
  `:45-51` (`balance(&AgentId, &EconomicState)` read), `:64-66`
  (`on_pre_append` returns `ToolSignal::Pass` UNCONDITIONALLY — today the tool
  itself imposes **no** authority check; admission gates own all veto logic).
- Constitution / harness binding: `AGENTS.md §5–§6, §9, §12, §14`;
  `CLAUDE.md §3–§4`; `feedback_trust_root_pin_trap`;
  `feedback_class4_cannot_hide_in_class3`; `feedback_admission_fail_closed_default`;
  `feedback_no_workarounds_strict_constitution`;
  `feedback_single_site_gate_illusion`.

---

## §1. Decision statement (what capability attenuation means)

**Add a least-privilege *authority* gate to the single admission oracle:** before
a wtool call is allowed to drive a **privileged sink** (a wallet-capability /
economic-write tool, or a privileged write namespace such as `wallet/`,
`capability/`, `trust_root/`), the admission contract requires that the invoking
principal holds a **properly attenuated capability grant** authorizing exactly
that sink. **Fail-closed: no valid capability grant for the sink ⇒ REFUSE the
advance** (no `Q_{t+1}`), with a tape-recorded rejection receipt — the same
non-advancing rejection path S4 arg-taint already uses.

This is the **complement** of the shipped S4 arg-taint hard-gate. The two gates
are orthogonal and stack at the SAME single admission seam:

| Gate | Question it answers | Lever |
|---|---|---|
| **S4 arg-taint** (LIVE) | **WHAT flows in** — does a tainted (non-`Trusted`) argument value reach a privileged sink? (confused-deputy / provenance) | `arg_taint_v1` findings (`arg_taint.rs:335`) |
| **This leg — capability attenuation** | **WHO may invoke** — does the invoking principal hold an attenuated capability grant for this sink at all? (authority / least-privilege) | new `cap_attenuation` finding |

Today the wallet tool imposes no authority check of its own
(`wallet.rs:64-66` `on_pre_append` → `Pass` unconditionally; the comment states
"admission gates own all veto logic at the typed_tx layer"). A wallet-capability
sink is reachable by any principal that S4 arg-taint does not flag — i.e. an
agent can drive a wallet/economic sink with perfectly `Trusted`-labelled
arguments and there is **no gate on the authority to invoke it at all**. This
leg closes that authority gap.

FC trace: this lives at the **FC1 wtool → predicate boundary** (FC1a-predicates
/ FC1b-Q_{t+1}, `arg_taint.rs:34-37`), the same boundary S4 occupies — it adds a
second admission predicate, it does NOT touch the runtime loop topology or any
flowchart.

---

## §2. Precondition: what is ALREADY live (this leg builds on it, does not re-do it)

This packet is requestable only because the privileged-sink definition, the
single admission oracle, the unpinned kernel seam, and the typed capability
vocabulary already exist. Do not re-authorize these; they are the load-bearing
foundation.

| Precondition | Status | Evidence |
|---|---|---|
| **Privileged-sink definition** — `PrivilegedSink` + `SinkReason` enumerate exactly what counts as privileged (system-only permission, economic/oracle/sandboxed-exec capability, external side effect, non-idempotent write, or a privileged write namespace incl. `wallet/`, `capability/`). The capability leg REUSES this; it does NOT redefine "privileged". | LIVE | `src/predicate_admission/arg_taint.rs:155-208` (`PrivilegedSink`/`SinkReason`); `:227-234` (`PRIVILEGED_WRITE_NAMESPACES`); `:240-274` (`classify_tool_sink` / `classify_write_key_sink`). |
| **Single admission oracle (S4 hard-gate wrapper)** — `decide_admission_with_taint` runs the taint check FIRST, then delegates to the unchanged `decide_admission`. The capability finding hooks into the SAME wrapper pattern (run cap check first/alongside, refuse on a non-empty finding set) with zero pinned edits. | LIVE | `src/predicate_admission.rs:149` (`decide_admission`); `:217-229` (`decide_admission_with_taint`); `:28-29` (`#[path]` submodule precedent). |
| **Live unpinned wtool admission seam** — the kernel already threads findings into the oracle and routes a non-empty finding set to the non-advancing rejection path with a tape receipt. The capability finding rides the SAME seam. | LIVE | `src/memory_kernel.rs:266` (`step_forward_with_taint`); `:291-292` (oracle call); `:294-349` (Pass-commits-receipt / Fail-routes-to-rejection). |
| **Typed capability vocabulary** — `Capability` enum (incl. `EconomicWallet`) + `PermissionPolicy::{Open, SystemOnly}` + `ToolMetadata` already classify each tool. The leg attenuates AGAINST this vocabulary; it does not invent a parallel authority type. | LIVE (pinned) | `src/bottom_white/tools/registry.rs:23-34` (`Capability`); `:36-42` (`PermissionPolicy`); `:70-86` (`ToolMetadata`). |
| **Wallet tool is a derived read-only projection** — already collapsed to zero owned ledger state (TB-9); it is NOT an independent authority, consistent with the parallel-ledger discipline (`constitution.md:60-61`). | LIVE (unpinned) | `src/sdk/tools/wallet.rs:28-52`. |

**Gap this leg fills:** there is a `PermissionPolicy::SystemOnly` *flag* on
`ToolMetadata` (`registry.rs:38-42`) and S4 treats `SystemOnly` as a privileged
sink reason (`arg_taint.rs:241-243`), but there is **no runtime gate that checks
the invoking principal actually holds authority for a privileged sink** — i.e.
no attenuation, no per-principal capability grant, no fail-closed authority
refusal. `SystemOnly` today is a *classification of the tool*, not a *check on
the caller*. This leg adds the caller-side authority check.

---

## §3. Allowed engineering actions (only under the §8 token)

The following are the **only** moves authorized once the architect supplies the
token. **Prefer the UNPINNED-only form** (A-ALLOW-1..4); anything touching a
pinned surface (A-FLAG-1) is OUT of this token and requires a separate packet.

- **A-ALLOW-1 — capability-attenuation module (NEW unpinned `#[path]`
  submodule, e.g. `src/predicate_admission/cap_attenuation.rs`).** Nested under
  the UNPINNED `src/predicate_admission.rs` (pin-count 0), EXACTLY mirroring the
  shipped `predicate_admission/arg_taint.rs` precedent (`arg_taint.rs:1-13`
  documents the least-pinned discipline). It defines a `CapabilityGrant` value
  (the attenuated authority a principal holds: principal id + the set of
  `Capability` / privileged-namespace targets it is authorized for + a
  tape-anchored provenance), a `CapAttenuationFinding` (raised when a privileged
  sink is reached WITHOUT a matching grant), and a deterministic
  `cap_attenuation_v1(call, grants) -> Vec<CapAttenuationFinding>` analogous to
  `arg_taint_v1`. It REUSES `classify_tool_sink` / `classify_write_key_sink` /
  `PrivilegedSink` from `arg_taint.rs` for the sink definition — it does NOT
  redefine "privileged".
- **A-ALLOW-2 — extend the unpinned admission wrapper.** Add a sibling to
  `decide_admission_with_taint` (e.g. `decide_admission_with_authority`, or fold
  the capability findings into the existing wrapper) in the UNPINNED
  `src/predicate_admission.rs`. A non-empty `CapAttenuationFinding` set REFUSES
  the advance via the EXISTING `AdmissionVerdict::Fail` path with a distinct
  `failed_predicate` prefix (e.g. `cap_attenuation_v1[...]`), reusing the
  `AcceptancePredicateFalse` carrier exactly as arg-taint does
  (`predicate_admission.rs:127-134`). The pinned exhaustive match stays valid
  with ZERO pinned edits.
- **A-ALLOW-3 — wire through the EXISTING unpinned kernel seam.** Thread the
  capability findings into `src/memory_kernel.rs` alongside the existing
  `taint_findings` argument (`memory_kernel.rs:266-292`), routing a non-empty
  set to the SAME non-advancing rejection path with a `cap_attenuation_v1[...]`
  rejection receipt + a distinct reject_class (mirroring
  `memory_kernel.rs:342-349`). `memory_kernel.rs` is UNPINNED (pin-count 0), so
  this is a zero-pin-rehash edit.
- **A-ALLOW-4 — triple-coupled non-vacuous gate.** Add
  `tests/constitution_capability_attenuation_wallet_gate.rs` proving (a) a
  privileged wallet sink invoked WITHOUT a matching grant is REFUSED
  (head does not advance), (b) the SAME sink WITH a valid attenuated grant
  PASSES (positive control), (c) a grant for capability X does NOT authorize
  sink Y (attenuation is real, not a blanket allow), and (d) the gate has a
  caught mutant (mutation-proof per `feedback_single_site_gate_illusion`).
  Triple-couple per `feedback_constitution_gate_triple_coupling` (manifest +
  `CONSTITUTION_EXECUTION_MATRIX.md` row + `ls tests/constitution_*.rs` glob).
- **A-FLAG-1 — (OUT OF SCOPE under this token; flagged for honesty) pinned-surface
  variants.** If a design instead chose to (i) add a new `Capability` variant or
  change `ToolMetadata` in the PINNED `src/bottom_white/tools/registry.rs`
  (pin-count 1), or (ii) insert the capability check inside the PINNED
  `src/bus.rs` dispatch (pin-count 1), that is a **trust-root pin surface**:
  `feedback_trust_root_pin_trap` requires the SHA-256 rehash to land in the same
  commit as the byte change, AND `feedback_squash_merge_orphans_ratification_tag`
  requires a fresh signed `v4-ratify` tag over the merge commit. Those are
  **Class-4 STEP_B parallel-branch** moves I CANNOT self-authorize — they need
  their OWN §8 packet with the pin rehash spelled out. This packet deliberately
  routes around them (A-ALLOW-1..3 touch zero pinned files), so the
  recommendation is to keep the pinned `Capability`/`bus.rs` surfaces untouched.

**Sourcing constraint (binding):** the capability grant set MUST derive from a
**tape-anchored grant** (a CAS-anchored / chain-derived authority record), NOT a
hardcoded allowlist constant in source (`CLAUDE.md §4` — no hardcoded behavior
parameter). The privileged-sink definition derives from the existing
`arg_taint.rs` table; the capability vocabulary from the existing `Capability`
enum. Any economic amount stays integer-only (no `f64`/`f32`).

---

## §4. Hard guards (binding even under the token)

If any guard cannot be met, the leg STOPS — do not weaken a guard into a skip
(`feedback_no_workarounds_strict_constitution`).

- **G-GUARD-1 — fail-closed default.** No matching capability grant for a
  reached privileged sink ⇒ `CapAttenuationFinding` ⇒ REFUSE the advance. An
  unknown / unparseable / ambiguous grant fails CLOSED (treated as "no
  authority"), exactly as `ArgTaint::from_tag` fails closed to
  `UntrustedExternal` (`arg_taint.rs:89-97`) and per
  `feedback_admission_fail_closed_default`. Error in the check ⇒ REFUSE.
- **G-GUARD-2 — least-privilege attenuation, not a blanket allow.** A grant
  authorizes a SPECIFIC sink set (specific `Capability` targets / specific
  privileged namespaces), never "all privileged sinks". A grant for capability X
  must NOT authorize sink Y (gate test A-ALLOW-4c). Attenuation = the grant is
  strictly narrower than full authority.
- **G-GUARD-3 — capability derives from a tape-anchored grant, not a hardcoded
  allowlist.** The authority a principal holds is reconstructable from
  tape/CAS — an auditor must be able to replay "principal P held grant G at
  logical_t" from the chain, not read it out of a source constant. No
  filesystem-side or global-pointer authority (`feedback_markov_inheritance_tape_derived`,
  `AGENTS.md §12`).
- **G-GUARD-4 — single authority, no second admission oracle.** The capability
  check routes through the ONE shared `decide_admission` contract
  (`predicate_admission.rs:149`) via its unpinned wrapper, exactly as S4 does.
  It MUST NOT become a parallel admission authority, a dashboard-only gate, or a
  second source of truth (`feedback_single_site_gate_illusion`,
  `feedback_class4_cannot_hide_in_class3`). The wallet tool stays a derived
  read-only projection (`wallet.rs:28-52`); the gate lives at admission, not as a
  new authority inside the tool.
- **G-GUARD-5 — deterministic + replay-stable, objective only.**
  `cap_attenuation_v1` is a pure function: same (call, grants) ⇒ same findings,
  emitted in a stable (sink × principal) order so the receipt is reconstructable
  byte-for-byte (Art. V.2 determinism, `constitution.md:799`). No probabilistic
  model, no subjective "trust score" — the verdict is a binary authority
  predicate (Art. I.1 Π_p boundary), never a ranking.
- **G-GUARD-6 — reversibility preserved.** Because a refusal routes to the
  EXISTING non-advancing rejection path (head stays frozen,
  `memory_kernel.rs:329-349`), the gate never produces an irreversible state
  change; an admitted advance remains rollback-able to `Q_{t-1}`
  (Art. V.2, `constitution.md:798`). The gate adds a refusal, not a new
  state-mutation.
- **G-GUARD-7 — zero pinned-file diff for the ratified form.** The implemented
  diff under this token MUST show `git diff main --name-only` touching NO
  genesis-pinned file (`grep -c '"<path>"' genesis_payload.toml == 0` for every
  edited src file). If implementation discovers it CANNOT avoid the pinned
  `registry.rs`/`bus.rs`, STOP and escalate to a fresh §8 (A-FLAG-1) — do not
  silently edit a pinned file under this token.

---

## §5. Forbidden (even under the token)

- **No pinned-surface edit under this token.** No new `Capability` variant /
  `ToolMetadata` change in the pinned `registry.rs`, no capability check inside
  the pinned `bus.rs` dispatch. Those are a separate Class-4 pin surface with
  their own §8 + pin rehash + signed `v4-ratify` tag (§3 A-FLAG-1,
  `feedback_trust_root_pin_trap`).
- **No `constitution.md` write.** Human sudo only (Art. V.1.1,
  `constitution.md:704`).
- **No hardcoded capability allowlist** as the authority source — the grant must
  be tape-anchored (G-GUARD-3; `CLAUDE.md §4`).
- **No second admission authority and no blanket grant** — single oracle,
  least-privilege only (G-GUARD-2, G-GUARD-4).
- **No subjective judgment in the gate** — authority is a binary
  constitutionality-class predicate; code-style / performance / coverage opinions
  are out-of-scope and belong to the independent clean-context auditor
  (Art. V.1.3, `constitution.md:765`; `AGENTS.md §9, §14`).
- **No typed-tx SCHEMA change, no sequencer-admission-arm change.** This leg
  rides the EXISTING kernel admission seam + the EXISTING `AdmissionVerdict::Fail`
  carrier. A new `TxKind` discriminant, typed-tx wire change, or sequencer
  admission arm is a SEPARATE Class-4 surface — STOP and request a fresh §8.
- **No `f64`/`f32`** anywhere on a money/economic path (`AGENTS.md §12`).
- **No audit before runnable evidence** — gate GREEN on a real build first
  (`feedback_audit_after_evidence`).

---

## §6. Risk classification & FC trace

**Risk class: Class 4 candidate — capability/authority surface + §6 restricted
wallet surface.** The change introduces an *authority* gate (who may reach a
privileged economic/wallet sink). Even though the ratified form edits ZERO
genesis-pinned files (the mechanism lives in unpinned modules per §3), an
authority gate is precisely the kind of surface that can smuggle a second
admission authority or a Class-4 change inside a Class-3 umbrella
(`feedback_class4_cannot_hide_in_class3`); and `src/sdk/tools/wallet.rs` is an
explicitly enumerated §6 restricted surface. Per `AGENTS.md §5–§6` it is treated
as Class-4 until proven otherwise, hence this per-atom §8 packet. **It is NOT a
trust-root STEP_B atom in its ratified form** (no pinned byte change ⇒ no pin
rehash ⇒ no signed `v4-ratify` tag), and that is the deliberate design — see the
honest pin finding in the header and G-GUARD-7. Only the A-FLAG-1 pinned variant
would be STEP_B, and that is explicitly OUT of this token.

**FC trace (FC1 wtool → predicate boundary, same seam as S4):**
- **FC1a-predicates** — capability attenuation is a second admission predicate at
  the wtool argument/authority seam, parallel to `arg_taint_v1`
  (`arg_taint.rs:34-37`).
- **FC1b-Q_{t+1}** — a `CapAttenuationFinding` blocks the head advance (no
  `Q_{t+1}`) via the existing non-advancing rejection path
  (`memory_kernel.rs:329-349`).
- **Art. III shielding** (`constitution.md:360, 413-423`) — the authority /
  verification logic lives in the admission region the black-box agent cannot
  steer; the agent only feels a fail-closed refusal, never the grant table as an
  optimization shortcut.
- **Art. I.1 Π_p** (`constitution.md:46`) — the gate is a binary admission
  predicate, not a score.
- **Art. V.1.3 Veto-AI domain** (`constitution.md:740, 765`) — objective
  constitutionality only; `{PASS, VETO}`-style binary, no subjective dimension.
- **Art. V.2** (`constitution.md:798-799`) — deterministic + reversible: refusal
  freezes the head, admitted advance stays rollback-able.

**STEP_B protocol** (`feedback_step_b_protocol`): NOT triggered by the ratified
unpinned form (A-ALLOW-1..4 touch zero pinned files). Triggered ONLY by the
A-FLAG-1 pinned variant (`registry.rs` `Capability` enum or `bus.rs` dispatch),
which is OUT of this token and needs its own §8 + pin rehash + signed
`v4-ratify` tag.

---

## §7. What this packet does NOT authorize

- It does NOT authorize any `src/` edit. The capability gate stays BLOCKED until
  the token in §8.
- It does NOT authorize editing any genesis-pinned file — NOT the
  `Capability` enum / `ToolMetadata` in `registry.rs`, NOT the `bus.rs`
  dispatch. Those are a separate Class-4 pin surface (§3 A-FLAG-1, §5).
- It does NOT re-authorize the S4 arg-taint gate, the single-admission oracle,
  or the kernel seam — those are already live (§2).
- It does NOT permit any `constitution.md` mutation (human sudo only,
  Art. V.1.1).
- It does NOT permit a typed-tx schema change, a new `TxKind` discriminant, or a
  sequencer-admission-arm change — that is a separate Class-4 surface (§5).
- It does NOT itself create or sign any `v4-ratify` tag (the ratified form
  changes no pinned bytes).
- It does NOT declare the work done. Done requires the triple-coupled gate GREEN
  on a real build with no constitution-gate-suite regression
  (`cargo test --workspace --no-fail-fast` exit 0,
  `bash scripts/run_constitution_gates.sh` exit 0,
  `cargo test --test constitution_matrix_drift` exit 0), zero pinned-file diff,
  and a clean-context audit (`AGENTS.md §9`).

---

## §8. Architect ratification (to be filled at user verbatim)

**Status: AWAITING ARCHITECT RATIFICATION.** No `src/` work begins until the
architect supplies the exact token below. Per `feedback_no_batch_class4_signoff`,
a short reply (`go` / `ok` / `continue` / `can` / `完成`) is NOT Class-4 sign-off.

```text
Ratify:
  APPROVE-CAPABILITY-ATTENUATION-WALLET-GATE

Reject / defer:
  REJECT-CAPABILITY-GATE-FOR-NOW   # privileged sinks stay gated ONLY by S4 arg-taint (WHAT flows in); the WHO-may-invoke authority gate stays unbuilt
```

**Scope confirmation requested from the architect:** ratify the **unpinned-only**
form (A-ALLOW-1..4; zero pinned-file diff; no STEP_B). If the architect instead
wants the pinned `Capability`-enum / `bus.rs`-dispatch variant (A-FLAG-1), that
needs a SEPARATE §8 packet carrying the trust-root pin rehash + a signed
`v4-ratify` tag — please say so explicitly and it will be drafted as its own
Class-4 STEP_B packet.

**Architect §8 sign-off (FILLED IN AT USER VERBATIM):**

- Verbatim quote: _<to be filled — exact user words>_
- Token consumed: `APPROVE-CAPABILITY-ATTENUATION-WALLET-GATE`
- Scope confirmed: _<unpinned-only A-ALLOW-1..4 / OR explicitly-authorized pinned A-FLAG-1 with separate pin-rehash packet>_
- Date: _<to be filled>_
- Branch at ratification: `claude/capability-attenuation-wallet-gate`
- Parent commit: _<origin/main HEAD at ratification>_

---

`FC-trace: FC1a-predicates + FC1b-Q_{t+1} — capability-attenuation authority gate at the wtool → predicate boundary, the WHO-may-invoke complement of the live S4 arg-taint WHAT-flows-in gate (arg_taint.rs:155-260, 335). New unpinned cap_attenuation submodule under predicate_admission.rs (pin-count 0) reuses PrivilegedSink/classify_tool_sink, refuses a privileged-sink invocation lacking a tape-anchored attenuated CapabilityGrant via the existing AdmissionVerdict::Fail carrier, wired through the unpinned memory_kernel step_forward seam (memory_kernel.rs:266-349); fail-closed, least-privilege, deterministic, reversible (Art. III shielding constitution.md:360/413-423; Art. I.1 Π_p; Art. V.1.3 {PASS,VETO}; Art. V.2 reversibility+determinism constitution.md:798-799). Class-4 candidate (capability authority + §6 restricted wallet.rs) but ZERO genesis-pinned diff in the ratified form ⇒ NOT trust-root STEP_B, NO pin rehash, NO signed v4-ratify tag; the pinned Capability-enum/bus.rs variant (A-FLAG-1) is OUT of this token and needs its own §8. Per-atom §8 required; no implementation until token supplied.`

**End of Capability-Attenuation-Wallet-Gate §8 decision packet (AWAITING ARCHITECT RATIFICATION; documentation only).**
