# TRACE_MATRIX v1 — Constitutional Flowchart ↔ Rust Code (2026-04-25)

**Predecessor**: `TRACE_MATRIX_v0_2026-04-22.md`
**Trigger**: Phase B B7 (Trust Root + Boot freeze) shipped runtime code that (a) implements the Phase 11+ deferred FC3-N34 row and (b) introduces new files to the readonly base. Per CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素", v1 documents the new mappings before downstream work piles on top.

> **2026-04-25 amendment** (post-constitution V.3 修订日志, mid-session): the constitution renamed **JudgeAI → Veto-AI** (Art. V.1.3 + FC3 mermaid `judgeAI` → `vetoAI`). All TRACE_MATRIX v0 references to `JudgeAI` / `judgeAI` (rows FC3-N32, FC3-N42, FC3-N43, FC3-E4/E5/E15, edge `FC3-Veto`) should be read forward-compatibly as Veto-AI / vetoAI. v0 + `FC_ELEMENTS_2026-04-22.md` are immutable audit-trail baselines and are NOT backfilled. Constitutional clarifications also added at V.1.1 (sudo scope = constitution.md only) + V.1.2 (ArchitectAI commit authority on non-constitution files); these reframe how Trust Root is *enforced* (Veto-AI proposal gate + Boot manifest runtime gate) without changing what's *in* the manifest.

**Scope**: delta only. v0 rows that did not change are still authoritative — read v0 first.

**Legend** (unchanged from v0):
- ✅ well-aligned · ⚠️ partial · 🔨 missing-actionable · 📅 deferred Phase 11+ · 📄 docs-only

---

## § 1. Status flips (rows that changed since v0)

| FC Element ID | v0 Status | v1 Status | Justification |
|---|---|---|---|
| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |

No rows regressed. No previously ✅ rows changed.

---

## § 2. New code symbols added in B7 (FC anchors)

| Symbol | File:Line | FC Anchor | DocComment | Status |
|---|---|---|---|---|
| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
| `boot::parse_trust_root_section` | `src/boot.rs:91` | FC3-N34 (helper) | Y (line 86-90) | ✅ |
| `boot::TrustRootError` | `src/boot.rs:24` | FC3-N34 (failure variants) | Y (line 19-23) | ✅ |
| `fn main` (Trust Root verify call site) | `src/main.rs:11` | FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`) | Y (line 3-10) | ✅ |
| `rollback_sim::should_simulate_rollback` | `experiments/minif2f_v4/src/rollback_sim.rs:48` | FC1-E18 (∏p=0 → Q_t) repeated · FC2-N22 HALT (existing `MaxTxExhausted` variant) — **outcome-equivalent only on (problem, seed, solved)** | Y (file header + fn doc) | ⚠️ partial (audit-fix 2026-04-25) |
| `rollback_sim::rollback_simulation_enabled` | `experiments/minif2f_v4/src/rollback_sim.rs:39` | same FC1-E18 + FC2-N22 anchor (env-var read for the predicate); narrow equivalence per above | Y | ⚠️ partial |
| `rollback_sim::ROLLBACK_TX_THRESHOLD` | `experiments/minif2f_v4/src/rollback_sim.rs:34` | PREREG § 5.5 frozen constant (calibration anchor — not a runtime parameter) | Y | ✅ |
| `rollback_sim::ROLLBACK_ENV_VAR` | `experiments/minif2f_v4/src/rollback_sim.rs:38` | env-var name (mirrors PREREG § 5.5 `--simulate-rollback-at-tx-50`) | Y | ✅ |
| `evaluator.rs` short-circuit at line 503-518 | `experiments/minif2f_v4/src/bin/evaluator.rs:503` | FC1-E18 + FC2-N22 (call-site of the synthetic predicate); **path-equivalent NOT verified — bus's evaluate_predicates is not exercised in calibration treatment** | Y (block comment) | ⚠️ partial |

Internal helpers (`has_section`, `strip_comment`, `unquote`, `hex_lower`) are private — no FC backlink required (per CLAUDE.md scoping to `pub` symbols).

---

## § 3. New `readonly` extensions (FC3-S3 subgraph membership change)

The constitutional FC3-S3 `readonly` subgraph contains FC3-N3 (`constitution as ground truth`) and FC3-N4 (`logs archive as ground truth`). PREREG § 1.8 (round-4 dual-audit PASS/PASS) extended this base for the PPUT-CCL experiment. Each addition is a research-protocol orphan with explicit constitutional justification (case-law / measurement-fidelity / pre-registration commitment).

| Path (manifest entry) | Justification |
|---|---|
| `src/kernel.rs` | FC3-N10 (`tape Q`) source — kernel topology immutability is a Law-1 invariant (Art. I.1) |
| `src/wal.rs` | FC3-N11 (`log`) implementation — append-only WAL is the constitutional logs-archive surface |
| `src/bus.rs` | FC1-N11/N13/N14 (`∏p`, `wtool`, `Q_{t+1}`) implementation — execution semantics that PPUT measures rest on |
| `src/drivers/llm_http.rs` | FC1-N7 (`δ / AI`) — cost source-of-truth (prompt_tokens / completion_tokens). Tampering with this defeats every C_i count |
| `src/sdk/prompt_guard.rs` | B6 PPUT-context-leak runtime gate — measurement-isolation invariant (no metric reaches agent prompt) |
| `experiments/minif2f_v4/src/lean4_oracle.rs` | FC1-N12 (∏p ground-truth oracle) — Lean ground-truth cannot drift mid-experiment |
| `experiments/minif2f_v4/src/cost_aggregator.rs` | B2 PPUT cost C_i computation — accounting invariant |
| `experiments/minif2f_v4/src/wall_clock.rs` | B3 PPUT time T_i computation — accounting invariant |
| `experiments/minif2f_v4/src/post_hoc_verifier.rs` | B4 verified-vs-runtime PPUT separation — accounting invariant |
| `experiments/minif2f_v4/src/jsonl_schema.rs` | B1 emit schema — auditable artifact format |
| `experiments/minif2f_v4/src/rollback_sim.rs` | B7-extra calibration toggle; PREREG § 5.5 commits a frozen `ROLLBACK_TX_THRESHOLD = 50` and a binary `SIMULATE_ROLLBACK_AT_TX_50` env var — tampering with either defeats the p_0 measurement |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | the wiring; tampering with it defeats every layer above |
| `constitution.md` | FC3-N3 (constitution as ground truth) — direct |
| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | sealed heldout split — pre-registration commitment per § 2.3 |
| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | the spec being committed to — pre-registration anchor |

`genesis_payload.toml` itself is **not** self-hashed (chicken-and-egg). The semantic anchor is the `[pput_accounting_0]` section content, not its hash. Section 6 below records this limitation.

**Total manifest size**: 16 files as of 2026-04-25 (15 from B7 + `rollback_sim.rs` from B7-extra). Manifest size will grow by 1 more file when B7-extra calibration lands (the `experiments/minif2f_v4/logs/p0_calibration_*.jsonl` data file becomes part of Trust Root per PREREG § 5.5 freeze step).

---

## § 4. `src/boot.rs` is **not** in the Trust Root manifest

Conscious choice — recorded here so the next reviewer does not file it as an oversight:

- Trust Root's threat model = passive tamper between runs (file-system edits without recompile).
- A motivated attacker who can edit `src/boot.rs` and recompile can replace `verify_trust_root` with `Ok(())` and bypass the entire check. SHA-256 of the source file cannot defend against this — the running binary is already the attacker's binary.
- Adding `src/boot.rs` to its own manifest gives a slightly stronger passive-tamper guarantee (catches edits to boot.rs without recompile, e.g. on a deployed system where the binary and source are out of sync) at the cost of one more file to maintain.
- Phase B7 chooses the smaller surface. Phase C+ may revisit if signed-binary attestation lands.

---

## § 5. Boot panic ↔ FC mapping

`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:

- FC2-N22 HALT requires the kernel/bus to be initialized (HaltReason variants are emitted by `TuringBus::halt_with_reason`).
- TRUST_ROOT_TAMPERED fires *before* any kernel/bus construction. The process aborts with no `Tape`, no `Bus`, no agent — there is no constitutional execution to halt.
- Closer match: FC3-E14 (`init → error → re-init → boot`). Boot-panic is the immediate-abort variant; the surrounding harness (batch runner, supervisord, shell wrapper) is the "re-init" actor.

See `OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` for the rationale to keep this as panic rather than promoting it into HaltReason. No constitution change requested.

---

## § 6. Updated stats (v1)

Compared to v0:
- ✅ count: **15 → 16** (+1: FC3-N34 promoted from 📅)
- 📅 deferred: **7 → 6** (-1)
- New orphan rows: **15 readonly extension paths** (above § 3) — each with constitutional justification, none requiring constitution change

Targets at end of Phase B (Stage 2/3 completion + B7):
- ✅ count: 38 + 1 = 39
- 📅/📄: 10 - 1 + 0 = 9
- 🔨/⚠️: 0 (per v0 § 4 actionable plan)

v1 does not address remaining v0 ⚠️ rows; those are Stage 2/3 work that has not yet landed (out of B7 scope).

---

## § 7. Outstanding work flagged for next alignment cycle

1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
2. ~~**TRACE_MATRIX of B7-extra (p_0 calibration toggle)**~~ — landed. Final implementation differs slightly from the original sketch in this section: the constitutional `bus.register_predicate(...)` API does not currently exist on `main` (it lives on the unmerged `phase-z-wtool-tools` branch — TRACE_MATRIX_v0 row FC1-N11 references it aspirationally). Rather than scope-creep B7-extra into reviving Phase Z, the synthetic predicate is implemented at the evaluator layer in `rollback_sim.rs` with an explicit short-circuit at the threshold tx. The constitutional anchor (FC1-E18 ∏p=0 → Q_t repeated, then FC2-N22 HALT via existing `MaxTxExhausted`) is unchanged; only the abstraction depth differs. Listed under § 2 above as ✅ entries.
3. **`src/boot.rs` self-hash decision** (§ 4 above) is open — Phase C+ revisit point.
