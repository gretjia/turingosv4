# §8 Decision Packet — All Canonical-Write Binary Entries Verify Trust Root

**Status**: **GRANTED by user 2026-06-07** (token `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT`
+ self-sign delegation; see §7). This document is **Class-0** (a decision record);
it describes a Class-4 trust-root-authority change. Implementation lands in a
SEPARATE Class-4 PR — NOT in this conformance-remediation PR (which ships only the
pending RED gate + this record).

**Date**: 2026-06-07
**Source finding**: `handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md`
§2 #3 (boot-trust-root, MAJOR) + §3 Gate 3.
**Branch**: `claude/conformance-remediation` (base `origin/main`; M07 G1+G2+G3
already landed).
**Risk class**: **Class 4** — touches a §6 trust-root / constitution-flowchart
authority surface (the "every binary launch verifies the Trust Root"
constitutional boundary) AND every canonical-write binary entry. Requires
**per-atom §8** architect ratification before any implementation or ship
(`AGENTS.md §5`, §6). Short replies (`go`, `ok`, `continue`, `can`, `完成`) do
**not** constitute Class-4 sign-off (`feedback_no_batch_class4_signoff`).

**Proposed §8 token** (the architect replies with this exact phrase to ratify):

```text
APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT
```

```text
Reject / defer option:
  REJECT-ALL-WRITERS-TRUST-ROOT-FOR-NOW   # keep the trust-root wiring frozen at 2 sites; continue Class-0 docs only
```

**Authority chain**:
- Conformance sweep (this finding's source, with adversarial per-site verify):
  `handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md` §2 #3 + §3 Gate 3.
- Verification-strategy redesign (why single-site gates produced the gap):
  `handover/design/CONSTITUTION_CONFORMANCE_HARNESS_2026-06-07.md`.
- Trust-root authority precedent (KEEP-SRC-BOOT, the ratified shape that keeps
  `src/boot.rs::verify_trust_root` the sole verifier):
  `handover/directives/2026-06-05_A03_BOOT_TRUST_ROOT_MANIFEST_PREFLIGHT_AND_SECTION8_REQUEST.md`
  + the existing 2-site gate `tests/constitution_tc_boot_trust_root_manifest.rs`.
- Pending gate (red today, demonstrating the gap):
  `tests/pending/constitution_all_canonical_writers_verify_trust_root.rs`,
  runnable via `scripts/run_pending_agentic_os_kill_conditions.sh`
  (token `ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_STANDING_PENDING`).
- Constitution binding: `AGENTS.md §5–§6, §9, §14`; `CLAUDE.md §3–§4`;
  `constitution.md` FC2 boot / FC3-N29/N34 readonly Trust Root guard;
  `feedback_trust_root_pin_trap`.

---

## §1. Decision statement

**Every binary entry that performs a canonical write must verify the boot Trust
Root before doing any work.**

A canonical write = advancing canonical state in any of these forms (the four
markers the sweep enumerated, §3 Gate 3):

- `put_json(...)` — writing a CAS evidence object,
- `GitTapeLedger::open/init_bare` — opening/initialising the durable ChainTape
  writer,
- `build_chaintape_sequencer_with_initial_q(...)` — building the live Sequencer
  that admits signed `WorkTx` advancing `state_root`,
- `SystemEmitCommand::*` / `emit_system_tx(...)` — emitting a system tx
  (`MapReduceTick` / governance) on the live tape.

No such entry may open a tape, write a CAS object, advance `state_root`, or emit
a system tx unless `boot::verify_trust_root(&repo_root)` has returned `Ok` for
that process. Tampering `constitution.md` or the pinned-hash manifest must
**halt** the writer before it touches canonical state — running `turingos boot`
first is an operator CONVENTION, not enforcement, and conventions are not gates.

---

## §2. Evidence the gap is real (pre-§8, already landed red)

`rg verify_trust_root src/` lands on exactly three production sites:

| Site | Binary | Verifies? |
|------|--------|-----------|
| `src/boot.rs:97` | (the verifier itself) | — defines `pub fn verify_trust_root` |
| `src/main.rs:14` | `turingosv4` (verify-only, no subcommands) | YES — panics `TRUST_ROOT_TAMPERED` on mismatch |
| `src/bin/turingos/cmd_boot.rs:66` | `turingos boot` subcommand | YES |

The existing gate `tests/constitution_tc_boot_trust_root_manifest.rs:129-140`
asserts ONLY those two call sites. That is the **M07 single-site illusion**: the
constitution's intent ("every binary launch verifies the Trust Root") is
enforced at two hand-picked sites while ~18 OTHER binary entries advance
canonical state with NO check.

The pending gate
`tests/pending/constitution_all_canonical_writers_verify_trust_root.rs`
ENUMERATES the canonical-write class from the live `src/bin/**` tree and asserts
each member's owning binary verifies the Trust Root. It is **RED today**:
**21 of 21 discovered canonical-write entries do not verify the Trust Root.** The
unguarded set (machine-enumerated by the gate, not hand-curated):

```text
src/bin/boot_cli_current_kernel_fresh.rs          (SystemEmitCommand::MapReduceTick)
src/bin/cybench_security_sandbox_current_kernel.rs (put_json + build_chaintape_sequencer)
src/bin/fc3_governance_reinit_current_kernel.rs   (SystemEmitCommand — highest-trust MapReduceTick / governance)
src/bin/full_system_augment_current_kernel.rs     (build_chaintape_sequencer + SystemEmitCommand)
src/bin/g0_market_activation_current_kernel.rs    (build_chaintape_sequencer + SystemEmitCommand)
src/bin/g1_market_live_agent.rs                   (build_chaintape_sequencer + SystemEmitCommand)
src/bin/gaia_general_assistant_current_kernel.rs  (put_json + build_chaintape_sequencer)
src/bin/gpqa_science_reasoning_current_kernel.rs  (put_json + build_chaintape_sequencer)
src/bin/lean_market_agent.rs                      (build_chaintape_sequencer + SystemEmitCommand)
src/bin/market_external_agent_current_kernel.rs   (put_json + build_chaintape_sequencer)
src/bin/math_competition_reasoning_current_kernel.rs (put_json + build_chaintape_sequencer)
src/bin/mind2web_browser_action_current_kernel.rs (put_json + build_chaintape_sequencer)
src/bin/osworld_computer_use_current_kernel.rs    (put_json + build_chaintape_sequencer)
src/bin/reputation_constitutional.rs              (build_chaintape_sequencer + SystemEmitCommand)
src/bin/swebench_live_coding_repair_current_kernel.rs (put_json + build_chaintape_sequencer)
src/bin/tdma_proof_current_kernel.rs              (put_json + GitTapeLedger + build_chaintape_sequencer)
src/bin/toolbench_api_tool_use_current_kernel.rs  (put_json + build_chaintape_sequencer)
src/bin/turingos/cmd_generate.rs                  (GitTapeLedger + build_chaintape_sequencer)  [turingos generate]
src/bin/turingos/cmd_tape_migrate.rs              (GitTapeLedger)                              [turingos tape-migrate]
src/bin/turingos/cmd_tdma.rs                      (GitTapeLedger)                              [turingos tdma run]
src/bin/webarena_web_agent_current_kernel.rs      (put_json + build_chaintape_sequencer)
```

The `turingos` dispatcher (`src/bin/turingos.rs`) itself has no
`verify_trust_root`, so its canonical-write subcommands (`tdma run`, `generate`,
`tape-migrate`) run unguarded — that is why the three `cmd_*.rs` submodules
appear above.

Mutation-proof witness: injecting `verify_trust_root(` into one writer drops the
gate's count 21→20, proving the gate tracks the live set and is not satisfiable
by `assert!(true)`.

---

## §3. Allowed engineering actions (only under the §8 token)

The following are the **only** engineering moves authorized once the architect
supplies the token. Each touches the Class-4 trust-root authority surface and is
BLOCKED until then.

- **A-ALLOW-1 — verify at the shared factory (preferred).** Insert one
  `turingosv4::boot::verify_trust_root(&repo_root)?` inside
  `build_chaintape_sequencer_with_initial_q` (`src/runtime/mod.rs:724`) so every
  runner that builds the live Sequencer is guarded by construction. This covers
  the majority of writers (all `*_current_kernel.rs` + `cmd_generate`) with a
  single insertion and no per-binary edit.
- **A-ALLOW-2 — verify at each remaining `main` top.** The writers that do NOT
  go through the factory — `fc3_governance_reinit_current_kernel.rs` and
  `boot_cli_current_kernel_fresh.rs` (emit `SystemEmitCommand` directly), and the
  `turingos` dispatcher path for `tdma run` / `tape-migrate` (open `GitTapeLedger`
  directly) — must call `verify_trust_root(&repo_root)` at the top of `main`
  (or the dispatcher entry, before routing to a canonical-write subcommand),
  before any tape/CAS work.
- **A-ALLOW-3 — reuse the `src/main.rs:14` abort semantics.** On verification
  failure the writer aborts the process (panic `TRUST_ROOT_TAMPERED` / non-zero
  `ExitCode`), exactly as `src/main.rs:14` does today. No new bypass env var, no
  `catch_unwind`, no `ALLOW/BYPASS/SKIP_TRUST_ROOT` surface (the KEEP-SRC-BOOT
  ratification forbids any bypass surface; see
  `tests/constitution_tc_boot_trust_root_manifest.rs:142-153`).
- **A-ALLOW-4 — EXTEND, do not replace, the existing gate.** The existing 2-site
  gate `tests/constitution_tc_boot_trust_root_manifest.rs` must be EXTENDED from
  its two hand-picked sites (`main.rs` + `cmd_boot.rs`) to the all-sites
  enumeration that the pending gate
  `tests/pending/constitution_all_canonical_writers_verify_trust_root.rs`
  already encodes. On promotion the pending gate is moved to a top-level
  `tests/constitution_*.rs` gate and triple-coupled (manifest +
  `CONSTITUTION_EXECUTION_MATRIX.md` row + `ls tests/constitution_*.rs` glob),
  per `feedback_constitution_gate_triple_coupling`.

**Sourcing constraint (binding):** `verify_trust_root` already reads the pinned
hashes from `genesis_payload.toml`; no new hardcoded behavior parameter may be
introduced (`CLAUDE.md §4`).

---

## §4. Forbidden (even under the token)

- **No second trust-root authority.** `src/boot.rs::verify_trust_root` stays the
  SOLE verifier (the KEEP-SRC-BOOT ratification). Writers must CALL it, never
  re-implement a hash check inline — a second verifier can drift, which is the
  same single-site/duplication failure mode this packet closes.
- **No bypass surface.** No env var, `catch_unwind`, or
  `ALLOW/BYPASS/SKIP_TRUST_ROOT` flag may be added to any writer. Verification is
  unconditional on the production path.
- **No `genesis_payload.toml` / `build.rs` / `Cargo.toml` edit.** This atom does
  not re-pin the Trust Root. `Cargo.toml` is itself pinned on this worktree —
  editing it trips `verify_trust_root` (`TRUST_ROOT_TAMPERED`,
  `feedback_trust_root_pin_trap`). The fix only INSERTS calls to the existing
  verifier from unpinned call sites; it does not touch any pinned file. (If a
  future variant must re-pin, that is a separate Class-4 atom with its own §8.)
- **No partial coverage claim.** "We added verify to the factory" is not done
  until the all-sites gate (A-ALLOW-4) is GREEN, i.e. the direct-`SystemEmit`
  and `turingos`-dispatcher writers are covered too. The completeness gate, not
  prose, is the done signal.
- **No audit before runnable evidence.** Promotion requires the all-sites gate
  GREEN on a real build plus no regression in the constitution gate suite, before
  any clean-context audit (`AGENTS.md §9`, `feedback_audit_after_evidence`).

---

## §5. Risk classification & FC trace

**Risk class: Class 4.** The change touches a §6 trust-root authority surface:
it changes WHICH binary launches are gated by the Trust Root (from 2 sites to
every canonical-write entry), which is the "every binary launch verifies"
constitutional boundary, not a Class-2 runner wiring detail. Class-4 cannot hide
inside a Class-3 umbrella (`feedback_class4_cannot_hide_in_class3`). Per
`AGENTS.md §5`, it requires explicit per-atom §8 ratification before
implementation or ship.

**FC trace:**
- **FC2** boot / CLI entry: the boot Trust Root guard must run before any
  canonical-write entry does work (the gap is that ~18 entries skip the FC2 boot
  guard).
- **FC3-N29 / FC3-N34**: `boot` ties to the re-init loop and the readonly Trust
  Root guard; `src/main.rs:14` implements the immediate-abort variant of FC3-E14
  (Trust Root mismatch at boot panics; the surrounding harness is the "re-init"
  layer). This atom extends that abort-on-tamper semantics to every canonical
  writer.

**STEP_B protocol** (`feedback_step_b_protocol`): the changes are insertions of
calls to the existing verifier from unpinned call sites
(`src/runtime/mod.rs`, the direct-`SystemEmit` mains, the `turingos` dispatcher).
They must be developed with the gate suite GREEN before commit. No pinned file is
edited, so no `genesis_payload.toml` rehash is required for this atom; if any
edit ever lands in a pinned file, the rehash-in-same-commit rule
(`feedback_trust_root_pin_trap`) applies and is a separate Class-4 surface.

---

## §6. What this packet does NOT authorize

- It does NOT authorize any `src/` edit. The fix stays BLOCKED until the token.
- It does NOT close finding #3. #3 is closed only when the all-sites gate is
  GREEN on a real build with no constitution-gate-suite regression
  (`cargo test --workspace --no-fail-fast` exit 0,
  `bash scripts/run_constitution_gates.sh` exit 0,
  `cargo test --test constitution_matrix_drift` exit 0), under the token, with
  the pending gate promoted + triple-coupled.
- It does NOT move trust-root authority. `src/boot.rs::verify_trust_root` remains
  the sole verifier (KEEP-SRC-BOOT preserved).
- It does NOT re-pin the Trust Root or touch `genesis_payload.toml` / `Cargo.toml`.

---

## §7. Architect ratification (to be filled at user verbatim)

**Status: GRANTED by user 2026-06-07.** The architect supplied the exact token
`APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT` + standing self-sign delegation.
This packet is now the ratification RECORD; the implementation lands in a SEPARATE
Class-4 PR (wire `verify_trust_root` into all canonical-write binary entries +
promote the pending all-canonical-writers gate to an all-sites GREEN gate). The
pending gate ships RED in THIS conformance-remediation PR; it turns GREEN in the
implementation PR.

```text
Ratify:
  APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT

Reject / defer:
  REJECT-ALL-WRITERS-TRUST-ROOT-FOR-NOW   # keep wiring at 2 sites; Class-0 docs only
```

**Architect §8 sign-off (FILLED IN AT USER VERBATIM):**

- Verbatim quote: user 2026-06-07 — `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT` (under the directive "使用 workflow 严格按宪法进行修复 … 任何导致宪法无法落地的行为").
- Token consumed: `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT`
- Implementation: SEPARATE Class-4 PR — wire `verify_trust_root` into all ~18 canonical-write binary entries + shared `build_chaintape_sequencer_with_initial_q`, abort on tamper; promote `constitution_all_canonical_writers_verify_trust_root` to an all-sites GREEN gate; signed `v4-ratify` tag if any genesis pin is touched.
- Date: `<pending>`
- Branch at ratification: `claude/conformance-remediation`
- Parent commit: `<origin/main HEAD at ratification>`
- Sign-off doc (created at user verbatim §8): `handover/section8/APPROVE_ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_§8_SIGN_OFF_2026-06-XX.md`

---

`FC-trace: FC2 boot Trust Root guard extended from 2 hand-picked sites to every canonical-write binary entry (CAS put_json / GitTapeLedger / live Sequencer / SystemEmitCommand) + FC3-N29/N34 readonly Trust Root guard / FC3-E14 abort-on-tamper semantics reused. Class-4 trust-root-authority change; per-atom §8 required; no implementation until token supplied.`

**End of All-Canonical-Writers-Verify-Trust-Root §8 decision packet (PENDING ARCHITECT RATIFICATION; documentation only).**
