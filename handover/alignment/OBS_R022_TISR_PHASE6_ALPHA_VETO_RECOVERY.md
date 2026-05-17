# OBS_R022 — TISR Phase 6.0/6.1 Alpha VETO Recovery TRACE_MATRIX Removal Justification

**Date**: 2026-05-17
**Triggered by**: Codex VERDICT VETO on commit `f74588e0` (TISR Phase 6.0/6.1 alpha first slice)
**Authority**: §8 packet `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
**Affected commit (incoming)**: TISR Phase 6.0/6.1 alpha rework (single-file binary)
**Scope**: REMOVAL of TRACE_MATRIX backlinks on pub items previously introduced in `f74588e0`

---

## 1. Removed pub items + TRACE_MATRIX FC2-N16 backlinks

Due to deletion of the `src/cli/` directory in the rework, the following pub items (originally introduced in `f74588e0`) are removed along with their TRACE_MATRIX backlinks:

| File | Pub item |
|---|---|
| `src/cli/mod.rs` | `pub fn run`, `pub mod args`, `pub mod commands`, `pub mod error`, `pub mod templates` (5 items) |
| `src/cli/args.rs` | `pub struct Cli`, `pub enum Commands`, `pub struct InitArgs`, `pub enum Template` (4 items) |
| `src/cli/commands/init.rs` | `pub fn run` (1 item) |
| `src/cli/commands/mod.rs` | `pub mod init` (1 item) |
| `src/cli/error.rs` | `pub enum TuringosCliError`, `pub fn exit_code`, `pub type CliResult` (3 items) |
| `src/cli/templates.rs` | `pub const GENESIS_PROOF / GENESIS_POLYMARKET / GENESIS_MULTI_AGENT` (3 items) |
| `src/lib.rs` | `pub mod cli` (1 item) |

**Total**: 18 pub items deleted along with their host directory `src/cli/`.

---

## 2. Justification

### 2.1 Codex VETO root cause

Codex clean-context audit on `f74588e0` detected that `Cargo.toml`, `Cargo.lock`, and `src/lib.rs` are Trust-Root pinned per `genesis_payload.toml [trust_root]` section. The `f74588e0` slice modified all three files (adding `clap = "4.5"` dep + `pub mod cli;` in lib.rs), which broke:

```bash
$ cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
test boot::tests::verify_trust_root_passes_on_intact_repo ... FAILED
```

This violated:
- **§8 PACKET §3.E** (forbidden tools list): "Any required Trust Root pinned-file rehash is **not authorized** here and requires a separate explicit packet."
- **Tools Budget §8 supplement §4**: "Trust Root tools / pinned-file updates remain excluded."

### 2.2 Recovery: Option A per user-as-architect

User chose Option A on 2026-05-17: rework the slice to avoid Trust Root pinned file modification. This required:

- **No new dependency** (drop `clap`)
- **No `pub mod cli;` in src/lib.rs**
- **All CLI code in `src/bin/turingos.rs` single-file** with manual `std::env::args` parsing — matches the established pattern in `lean_market`, `turingos_dev`, `audit_dashboard`, and all 11 other bins in `src/bin/`.

### 2.3 Consequence on TRACE_MATRIX coverage

The 18 pub items above are removed *because the host directory `src/cli/` is no longer needed*, not because they were defective. The functionality they implemented is preserved within `src/bin/turingos.rs` as private items (private `fn`, private `const`, private `enum`).

The single-file binary's module-level doc-comment carries the FC2-N16 binding for the whole CLI module:

```rust
//! TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — turingos init wraps boot/
//! genesis preparation flow (per §8 PACKET §2). Pure filesystem; 0 sequencer
//! call; 0 typed_tx; 0 CAS write; 0 ChainTape advance.
```

Since the single-file binary has **0 new pub items**, R-022 does not require per-item backlinks for the rework commit. The R-022 I-REMOVAL invariant is satisfied by this OBS document referenced via the `[R-022-skip: ...]` token in the rework commit message.

---

## 3. Constitutional References

- **§8 packet (RATIFIED)**: `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md` §3.E (Trust Root exclusion)
- **§8 sign-off**: `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_SIGN_OFF.md`
- **Tools Budget §8 supplement (RATIFIED)**: `handover/directives/2026-05-17_TISR_PHASE6_TOOLS_BUDGET_SECTION8_SUPPLEMENT.md` §4 (excluded tools list)
- **Codex VETO log**: `/tmp/tisr_phase6_alpha_codex_audit_out.log` (commit `f74588e0`, 2026-05-17)
- **Recovery commit**: TISR Phase 6.0/6.1 alpha rework (post-VETO, this commit)

---

## 4. Forward Coverage

After rework, the alpha slice is contained in:

- `src/bin/turingos.rs` (~475 LOC, single file, 0 pub items, FC2-N16 module-level doc-comment)
- `tests/cli_init_smoke.rs` (5 integration tests, all passing)
- `handover/evidence/dev_self_hosting/dev_1779011072273_1536110/` (turingos_dev evidence with full record-diff covering all rework changes)

Phase 6.1+ extensions (turingos batch / agent / task / etc.) may reintroduce a `src/cli/` module structure if Phase 6.1 §8 packet authorizes Trust Root rehash for the necessary Cargo.toml + lib.rs evolution; until then, additional subcommands extend `src/bin/turingos.rs` inline.

---

**End of OBS_R022 TISR Phase 6.0/6.1 Alpha VETO Recovery rationale.**
