# Agentic-OS Roadmap S1–S6 — Final Status (autonomous boundary reached)

**Date**: 2026-06-08
**Author**: Claude (orchestrator), under standing `/goal` 2026-06-07: *"连续使用
workflow完成全部任务，由你负责根据任务定制workflow的组织方式，只要符合宪法的，
就给你全部授权去做。中途你需要自己 open PR, merge PR. 继续推进，直到主任务完全
完成。"*
**Scope**: the agentic-os gap-closure roadmap derived from
`handover/research/agentic_os_gap_2026-06-06/AGENTIC_OS_REPORT.md` and the PR #314
follow-up plan (OBL-016).
**Status**: **every autonomously-shippable atom is SHIPPED + merged.** The residual
work is genuinely blocked on (a) four Class-4 per-atom §8 user tokens and (b) one
real heterogeneous-LLM API run. Neither is self-authorizable or fakeable.

---

## 1. What shipped (10 PRs, #315–#324, all self-opened + self-merged)

Every atom below is constitution-clean (Class 0–2), carries a **non-vacuous,
mutation-proven gate**, makes **ZERO trust-root / pinned-file change** (via the
unpinned-`#[path]`-submodule pattern, except M07 which carried its own signed
ratification tag), and passed a **clean-context audit (PROCEED / NO-VIOLATION)**.

| Atom | PR | What it closes | Gate |
|---|---|---|---|
| **M07** single-admission predicate gate | [#315](https://github.com/gretjia/turingosv4/pull/315) + [#316](https://github.com/gretjia/turingosv4/pull/316) | FC1 kernel no longer bypasses ∏p; both legs route through one shared `decide_admission`; zero-root quarantine | `constitution_kernel_predicate_gate`, `constitution_single_admission_contract`, `constitution_single_admission_behavioral`, `constitution_kernel_predicate_receipt_replay` |
| **Conformance sweep** (5 more bypasses) | [#317](https://github.com/gretjia/turingosv4/pull/317) | `llm_err` lands on tape; external attempt CAS-anchored on failure; judge reason no raw stderr; metric-leak guard wired | `constitution_llm_err_lands_on_tape`, `constitution_external_attempt_anchored_on_failure`, `constitution_judge_reason_no_raw_subprocess_stderr`, `constitution_metric_leak_guard_wired` |
| **All-canonical-writers verify trust-root** | [#318](https://github.com/gretjia/turingosv4/pull/318) | 21 `src/bin/` entries call `verify_trust_root`; pinned ≠ wired closed | `constitution_all_canonical_writers_verify_trust_root` (all-sites) |
| **S6** verification redesign | [#319](https://github.com/gretjia/turingosv4/pull/319) | OS-qualifying conjunction made explicit + falsifiable; pending gates compile-checked in CI (no bit-rot) | `constitution_agentic_os_minimum_qualification` (16-gate/7-dim), `constitution_pending_gates_compile` |
| **S1** FC3 observable + canary half | [#320](https://github.com/gretjia/turingosv4/pull/320) | System reads its own failure logs → synthesizes a real proposal on tape → canary-scores it; **no irreversible activation** | `constitution_fc3_proposer_canary_observable` (5) |
| **S4** arg-taint admission hard-gate | [#321](https://github.com/gretjia/turingosv4/pull/321) | Input/control-side shielding: untrusted args into a privileged sink → `Fail` before self-reported predicates | `constitution_arg_taint_admission` (6 + 15 unit) |
| **S2** Tier-1 cross-session memory | [#322](https://github.com/gretjia/turingosv4/pull/322) | System-authored, CAS+L4-chained `SkillCapsule`; agent read-only by construction | `skill_capsule_tier1_memory` (4) |
| **S5** interop surface | [#323](https://github.com/gretjia/turingosv4/pull/323) | External-agent interactions auditable on tape; inbound A2A hard-stamped `UntrustedExternal` (ingress shield) | `constitution_interop_surface_capsule` (4) |
| **S3** economy boltzmann observe-only trace | [#324](https://github.com/gretjia/turingosv4/pull/324) | First tape-anchored reconstructable record of the integer-rational scheduler selection over live econ | `constitution_boltzmann_selection_observe_only` (7) |

**Workspace at #324**: `cargo test --workspace --no-fail-fast` = **2696 passed /
0 failed**; `bash scripts/run_constitution_gates.sh` = **total=184 failed=0**;
`cargo test --test constitution_matrix_drift` = **3/3**.

---

## 2. The verification-strategy redesign (the user's critique, vindicated)

The user's pivotal challenge — *"constitution gate, 还有那么多 harness 都没发现谓词
没有 wire in the kernel … 现在的各种测试到底有没有用？"* — was correct. The root
cause was the **single-site gate illusion**: a large green suite asserted each
invariant at ONE site, not at ALL sites of its class. M07 (kernel skips
predicates) slipped through because the predicate-gate invariant was only checked
at the sequencer site.

The redesign (now load-bearing, see `feedback_single_site_gate_illusion` +
`handover/design/CONSTITUTION_CONFORMANCE_HARNESS_2026-06-07.md`):

1. **Enumerate-all-sites completeness gates** — grep every writer / entry / judge
   / failure-arm of a class and assert the invariant at each (e.g.
   `constitution_all_canonical_writers_verify_trust_root`,
   `constitution_agentic_os_minimum_qualification`).
2. **Mutation-proof** — every gate must have a caught mutant; a gate that cannot
   fail is documentation, not a gate. (S3's gate was proven by a live
   `selected_parent := wrong node` source mutation → 4 tests RED → reverted →
   7/7 green.)
3. **Recurring adversarial conformance sweep** — which immediately found **5 more
   bypasses** (PR #317), directly vindicating the critique.
4. **"Pinned ≠ wired"** — a constant being pinned in the manifest does not mean it
   is enforced at runtime; the sweep checks the wire, not the pin.

---

## 3. Residual work — blocked on the user (NOT self-authorizable)

Per `AGENTS.md §5` + `feedback_no_batch_class4_signoff`, the following Class-4
atoms each need an **explicit per-atom §8 token**. `go` / `continue` / `完成` do
**not** constitute Class-4 sign-off. Each has a fully source-faithful §8 decision
packet ready for ratification.

| # | Class-4 atom | §8 token to ratify | Packet |
|---|---|---|---|
| 1 | **FC3 irreversible leg** — runtime Veto-AI `{PASS,VETO}` → `ArchitectCommit` → boot trust-root rewrite → re-init (system activates self-generated code). HIGHEST blast radius. | `APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT` | `handover/section8/APPROVE_FC3_RUNTIME_VETO_AND_TRUSTROOT_REINIT_2026-06-07.md` |
| 2 | **Tier-2 agent-writable memory** — lets agents write (not just read) tape-anchored memory; needs a shielded write-authority boundary. | `APPROVE-AGENT-WRITABLE-TAPE-MEMORY` | `handover/section8/APPROVE_AGENT_WRITABLE_TAPE_MEMORY_2026-06-07.md` |
| 3 | **Budget hard-ceiling from a signed manifest** — on-exceed REJECT/halt = an admission decision over economic state (today the only admission authority is the pinned sequencer; the on-exceed reject reuses the empty pre-wired `RejectionClass::BudgetExceeded` seam). | `APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST` | `handover/section8/APPROVE_BUDGET_HARD_CEILING_FROM_MANIFEST_2026-06-08.md` |
| 4 | **Capability attenuation / wallet-gating** — gates WHO may drive a privileged sink (complements S4's WHAT-flows-in). The wallet tool's `on_pre_append` returns `Pass` unconditionally today; `Capability` exists as a discovery meta-layer but there is no runtime authority gate. Core mechanism is unpinned-doable (mirrors S4); a `registry.rs`/`bus.rs` touch would be a further pinned surface. | `APPROVE-CAPABILITY-ATTENUATION-WALLET-GATE` | `handover/section8/APPROVE_CAPABILITY_ATTENUATION_WALLET_GATE_2026-06-08.md` |

**Plus one non-§8 obligation that needs a real run, not a workflow:** the
**multi-LLM-on-tape real evidence** the user has required since session #34 — a
heterogeneous-LLM market run whose calls land on the canonical tape. This needs a
live API run (keys + spend); it is flagged here, **not faked**. A workflow cannot
synthesize it.

---

## 4. Honest scope limits carried forward (disclosed, not hidden)

- **S1 FC3**: only the observable + canary half is live; the terminal disposition
  of even a PASS candidate is the `sandbox:canary_only` dead-end. The loop does
  not close until residual #1 above is ratified.
- **S4 arg-taint**: the hard-gate is reachable + enforcing + gate-exercised, but
  the live TDMA path passes empty findings — deriving real provenance labels from
  proposal data is a forward wiring item.
- **S3 economy**: the observe-only trace is real and runs on canonical-tape econ,
  but the admission-side / live-tick selection (where the pick would actually
  steer accepted state) touches the pinned `bus.rs` / `audit_dashboard.rs` — a
  separate higher-class surface, deferred.
- **S2 memory**: Tier-1 (system-authored, agent-read-only) only; Tier-2 is
  residual #2.

---

## 5. Bottom line

The honest answer to *"是不是只剩工程?"* at the autonomous boundary: **the
engineering that can be done without amending the constitution or its 3
flowcharts, and without a Class-4 authority grant, is done and on tape.** What
remains is exactly the set of changes that the constitution *intends* a human to
ratify (self-evolution activation, agent write-authority, economic admission
ceilings, capability authority) plus one real-world evidence run. That residual is
not an engineering gap — it is the constitution's human-in-the-loop boundary
working as designed.

`FC-trace: FC1-N* (M07 ∏p gating, S4 arg-taint, S3 scheduler read-view) + FC2-N29 (S3 economic derived view, S2 memory, S5 interop CAS anchoring) + FC3-N3x/N4x (S1 observable proposer/canary; irreversible leg gated on §8). All shipped atoms WITHIN_FC; the 4 residual legs are the FC3/FC2 authority surfaces the constitution reserves for human §8 ratification.`
