# Constitution Conformance Harness — Verification-Strategy Redesign

- **Date:** 2026-06-07
- **Risk class:** Class 0 (design / verification-strategy record). It changes how
  we WRITE constitution gates; it does not by itself edit any `src/` surface.
- **Source evidence:** `handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md`
  (the adversarial sweep that found 5 M07-class bypasses, 3 of them MAJOR).
- **Companion §8 (Class-4 finding still pending):**
  `handover/section8/APPROVE_ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_2026-06-07.md`
  (finding #3, boot-trust-root).
- **FC trace:** meta — this governs the GATE LAYER (Tier-1 "the 3 canonical
  flowchart hashes + the constitution gates"), i.e. the soundness of the
  verification harness itself, per `AGENTS.md §7` "a test that cannot fail is
  documentation, not a gate" and Art. I.1.1 soundness applied to the gates.

---

## §0. TL;DR

M07 was not an isolated bug. The 2026-06-07 sweep proved it is a **class of
systemic failure**: a completeness invariant ("property P holds at EVERY site of
class S") is enforced — both in the wiring AND in the gate — at exactly ONE
"obvious" site, so the gate stays GREEN while a parallel/new site of class S
silently violates P. Three independent MAJOR true-parallel bypasses
(`llm_err` not on tape, market failure branch not CAS-anchored, ~18
canonical-write entries not verifying the Trust Root) all share this shape.

The fix to the HARNESS (not just the findings) is to move every completeness
invariant from a **single-site assertion gate** to an **ENUMERATE-ALL-SITES
completeness gate**, and to require each gate to ship with a recorded mutant it
catches. This doc codifies four mechanisms: (a) the completeness-gate pattern,
(b) the mutation-proof requirement, (c) recurring adversarial clean-context
conformance sweeps, (d) the pinned-but-dead lesson.

---

## §1. The M07 illusion, generalized (the evidence)

The original M07 finding: the kernel admitted state predicate-blind while the
sequencer enforced predicate admission — two authorities, one of them
unguarded. The gate at the time asserted the sequencer leg only, so it was
GREEN. The sweep then asked the obvious next question — *is M07 unique, or a
pattern?* — and ran an exhaustive per-site scan of 14 constitution invariants.
Result (sweep §1–§2):

| # | Invariant | Site | Why the existing gate missed it | Severity |
|---|-----------|------|---------------------------------|----------|
| 1 | append-only-rubber | `tdma_runner.rs` `llm_call` `Err` arm | every OTHER failure class commits a `verified=false` node; only `llm_err` short-circuits before the kernel | MAJOR |
| 2 | evidence-cas-anchor | `market_external_agent_*.rs` failure branch | sibling `swebench` always anchors a capsule; market early-returns `Err` BEFORE its first `put_json` | MAJOR |
| 3 | boot-trust-root | ~18 canonical-write binary entries | gate `constitution_tc_boot_trust_root_manifest.rs` asserts ONLY `main.rs` + `cmd_boot.rs` (2 of ~20 entries) | MAJOR |
| 4 | raw-diagnostic-shield | `swebench_test_judge.rs` stderr-tail fallback | sibling judges all emit bounded structured strings; only swebench leaks raw subprocess stderr | MODERATE |
| 5 | goodhart-shield | `assert_no_metric_leak` had zero production callers | the guard was trust-root pinned yet wired to nothing | LATENT |

The common structure across #1, #2, #3: **the invariant is a "for-all-sites"
property, but both the wiring and the gate covered one site.** The gate's GREEN
told us nothing about the OTHER sites — and a parallel site (a new domain
binary, a new failure arm, a new runner) is exactly where the violation hides.

This is the M07 illusion: a single-site witness ("X is enforced at the
sequencer" / "main.rs calls verify") cannot distinguish "P holds everywhere"
from "P holds at the one place I looked".

---

## §2. Mechanism (a) — the completeness-gate pattern

**Rule.** For any invariant of the shape "property P must hold at every site of
class S", the constitution gate MUST:

1. **Enumerate S from the live source tree** — grep / directory-walk the
   canonical source files to DISCOVER the full set of class-S sites at test time.
   The set is DERIVED, never a frozen hand-curated list. (A frozen list is itself
   a single-site illusion: it goes stale the moment a new site is added.)
2. **Assert P at EACH discovered site** — iterate the discovered set and assert P
   on every member, failing with the specific offending sites named.
3. **Be non-vacuous by construction** — because the set comes from the tree, a
   future parallel site that forgets P is auto-discovered and turns the gate RED.
   The gate cannot be satisfied by `assert!(true)`.
4. **Guard against vacuity** — assert the discovered set is non-empty (if the
   grep markers ever drift to match nothing, fail LOUD rather than pass empty).

**Reference template.** `tests/constitution_single_admission_contract.rs` is the
canonical example (greps the canonical source files for the verdict-trusting
loop and asserts it has exactly one home + both authorities call it). The
2026-06-07 remediation produced four more in this family:

- `tests/constitution_llm_err_lands_on_tape.rs` (#1) — enumerate every
  `llm_call` failure arm; assert each commits before `break`.
- `tests/constitution_external_attempt_anchored_on_failure.rs` (#2) — enumerate
  the post-LLM-call → first-`put_json` span in every runner; assert no
  `?`/`return Err` parse-guard in the span.
- `tests/constitution_judge_reason_no_raw_subprocess_stderr.rs` (#4) — enumerate
  every judge fail path; assert no raw `tail_chars(stderr, N)` in the reason.
- `tests/constitution_metric_leak_guard_wired.rs` (#5) — enumerate every final
  prompt-assembly site; assert each calls `assert_no_metric_leak`.
- (pending, Class-4) `tests/pending/constitution_all_canonical_writers_verify_trust_root.rs`
  (#3) — walk `src/bin/**`, discover the canonical-write class, assert each
  member's owning binary verifies the Trust Root.

**Anti-pattern (forbidden).** "Check P at the sequencer" / "check main.rs calls
verify" / "check the swebench judge is clean" — any gate that names ONE site for
a for-all-sites invariant. If the invariant is for-all-sites, the gate must
enumerate-all-sites.

---

## §3. Mechanism (b) — mutation-proof requirement (Art. I.1.1 applied to gates)

`AGENTS.md §7`: "a test that cannot fail is documentation, not a gate." Art. I.1.1
soundness, applied to the gates themselves, sharpens this: **every constitution
gate must come with a recorded mutant it catches.** A gate with no catchable
mutant is documentation.

**Rule.** When promoting (or modifying) a `constitution_*.rs` gate, the author
records — in the gate's header doc-comment or its accompanying §8/handover note —
a concrete MUTANT (a one-line edit to the production source) that the gate turns
RED on, with the observed before/after. The mutant must be a realistic
regression (a deleted call, a re-duplicated contract, a forgotten arm), not a
contrived syntax break.

**Worked example (this batch).** The pending #3 gate was mutation-checked:
injecting `verify_trust_root(` into ONE writer dropped the gate's unguarded count
21→20 (recorded in the §8 packet §2 and the gate header). That proves the gate
tracks the live set and is not satisfiable vacuously. Each Phase-1 gate
(#1/#2/#4/#5) likewise red-on-defect / green-after-fix, which is its mutant pair
(the pre-fix source IS the recorded mutant the gate catches).

**Why this matters.** A completeness gate that enumerates the wrong marker, or
that reads the wrong file, can be accidentally vacuous (empty set → green). The
recorded-mutant requirement forces the author to demonstrate the gate's RED state
on a real defect, which is the only proof the enumeration is wired to reality.

---

## §4. Mechanism (c) — recurring adversarial clean-context conformance sweeps

A single sweep on 2026-06-07 found 5 confirmed bypasses across 14 invariants.
This is too high a hit rate to treat as a one-off. The harness must SCHEDULE the
sweep as a recurring mechanism, not rely on it being remembered.

**Rule.** A constitution conformance sweep is run on a recurring cadence
(recommended: at each ship-path milestone / before any "OS-qualified" or
equivalent strong claim, and at minimum once per major TB closure), under the
`AGENTS.md §9` clean-context audit doctrine:

- **Method.** For each constitution invariant, treat it as a for-all-sites
  completeness property: `rg`-enumerate ALL sites of the class (all writers /
  all binary entries / all judges / all failure arms), check P at each, and
  adversarially VERIFY each candidate to source `file:line` before reporting
  (false positives destroy trust — only adversarially-confirmed bypasses are
  listed). The 2026-06-07 sweep is the reference method (sweep §"方法" /
  Executive Summary).
- **Clean context.** Run by a fresh agent on any capable platform with NO
  implementation transcript (platform-agnostic clean-context audit, 2026-05-29
  ratification). The sweep is a WITNESS, not a judge: its output space is the
  §9/§14 verdict domain (`NO-VIOLATION` / `VIOLATION-FOUND` /
  `RECONSTRUCTION-FAILURE` / `SECOND-SOURCE-DRIFT`).
- **Output.** A dated audit doc (`handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_<date>.md`)
  with a CONFIRMED table (per-site `file:line` + why-unguarded + whether a
  sibling enforces P, i.e. is it a true parallel bypass) and a gate proposal per
  finding. Each MAJOR finding spawns either a live completeness gate (Class ≤ 3)
  or a pending gate + §8 packet (Class 4).

**Why recurring.** Completeness gates catch REGRESSIONS of known invariants. They
do not discover NEW invariants or NEW classes whose enumeration nobody wrote yet.
The recurring adversarial sweep is the discovery mechanism that feeds new
completeness gates into the suite. Without it, the harness only defends the sites
someone already thought to enumerate.

---

## §5. Mechanism (d) — the pinned-but-dead lesson

Finding #5 (goodhart-shield): `src/sdk/prompt_guard.rs::assert_no_metric_leak`
was **trust-root pinned** (`genesis_payload.toml:152`) yet had ZERO production
callers — its only references were its own `#[cfg(test)]` module. The
Art. III.4 runtime enforcement site was a vacuum. **Pinning a file's HASH does
not prove the file is RUN.**

**Lesson.** A trust-root pin guarantees a file's BYTES are unchanged. It says
nothing about whether the file is WIRED into a production call graph. A
completeness gate over a pinned guard must therefore ALSO verify that a
production caller exists — not merely that the guard's hash matches.

**Codified rule.** For any pinned runtime guard G (a named enforcement function
whose job is to enforce a constitutional clause at runtime), the completeness
gate must assert BOTH:

1. **Wiring** — enumerate every site of the class G defends (e.g. every final
   prompt-assembly boundary) and assert each calls G; AND
2. **Liveness** — assert G has at least one PRODUCTION (non-test, non-self)
   caller, so it is not pinned-but-dead.

`tests/constitution_metric_leak_guard_wired.rs` is the reference (it enumerates
the prompt-assembly sites AND asserts `production_callers >= 1`). Compare
`assert_no_metric_leak` (now wired from unpinned callers at
`src/memory_kernel.rs:577` etc.) — the wiring was added at UNPINNED call sites
so the pinned guard file itself was never edited (no rehash needed). This is the
template: defend a pinned-but-dead guard by wiring its callers, not by touching
the pinned file.

**Corollary for #3.** The boot-trust-root finding is the dual of pinned-but-dead:
`verify_trust_root` IS live (called at `main.rs` + `cmd_boot.rs`) but only at 2
of ~20 canonical-write entries — "live at one site" is no more sufficient than
"pinned but dead". The same completeness lens applies: a pin/definition existing
≠ it being called everywhere the invariant requires.

---

## §6. Adoption checklist (binding on future gate authors)

When you write or modify a constitution gate:

- [ ] Is the invariant for-all-sites? If yes, the gate ENUMERATES the site set
      from the live source tree (no frozen hand-list) and asserts P at each
      (§2). Single-site assertion for a for-all-sites invariant is forbidden.
- [ ] Does the gate assert its discovered set is non-empty (vacuity guard, §2.4)?
- [ ] Have you recorded a concrete mutant the gate catches, with observed
      before/after (§3)?
- [ ] For a pinned runtime guard: does the gate assert BOTH wiring (every site
      calls it) AND liveness (a production caller exists)? (§5)
- [ ] Triple-coupling on promotion: `tests/constitution_*.rs` (flat) + manifest
      entry + `CONSTITUTION_EXECUTION_MATRIX.md` row, atomically
      (`feedback_constitution_gate_triple_coupling`).
- [ ] Class-4 invariants (trust-root authority, sequencer admission, typed-tx
      schema) ship as a PENDING gate + §8 packet first; promote only under the
      architect token (`AGENTS.md §5`). See finding #3.

---

`FC-trace: meta / gate-layer soundness. This redesign governs how constitution gates are written (Tier-1 gate layer), applying Art. I.1.1 soundness to the gates themselves: a for-all-sites invariant requires an enumerate-all-sites completeness gate + a recorded catchable mutant; a pinned guard requires a wiring+liveness gate. Source: handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md.`

**End of Constitution Conformance Harness verification-strategy redesign.**
