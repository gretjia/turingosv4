# TuringOS v4 - Shared Agent Instructions

This file is the short shared execution adapter for Codex, Claude Code, and
future fast agents. Keep it practical and stable. Do not copy the full
constitution, flowcharts, or historical handover here; route to them.

`CLAUDE.md` imports this file and may add Claude-specific operating detail.
If duplicated guidance conflicts, obey this file for the shared harness
contract, then the deeper/more specific adapter for tool-local mechanics.

## 1. Identity and Truth Order

TuringOS v4 is a tape-first constitutional operating substrate for LLM/AGI
agents. If meaningful activity is not on tape, it is not a TuringOS run.

Supreme truth order:

1. `constitution.md`
2. The canonical constitution flowcharts and their hashes
3. ChainTape + CAS evidence
4. Executable gates and replay/audit verifiers
5. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
6. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
7. `handover/ai-direct/LATEST.md`
8. Current TB charter / directive / ratification
9. Dashboards, reports, README files, stdout logs

Dashboards, reports, evaluator counters, README files, smoke summaries, and
audit text are materialized views. If a report contradicts ChainTape/CAS, trust
ChainTape/CAS. If ChainTape/CAS contradicts constitution gates, stop.

## 2. Cold Start

For a new non-trivial task, read in this order:

1. `CLAUDE.md`
2. `HARNESS_MANUAL.md`
3. `constitution.md`
4. `handover/ai-direct/LATEST.md`
5. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
6. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
7. Current directives/charters relevant to the task
8. Source files and tests for the touched surface

Optional Claude memory lives outside the repo at
`~/.claude/projects/-home-zephryj-projects-turingosv4/memory/MEMORY.md`.
Use it only as supporting context; do not make repo behavior depend on that path
existing on another machine.

Dynamic state belongs in `handover/ai-direct/LATEST.md` and
`handover/tracer_bullets/TB_LOG.tsv`, not in this file. Do not encode current
HEADs, gate counts, temporary freezes, or active round counts in `AGENTS.md`.

Default user-facing language is Chinese. Technical terms may remain in English.

## 3. Constitution Flowcharts

The current constitution has the three-flowchart version:

- FC1: Runtime loop, `constitution.md` around line 455.
  `Q_t -> rtool -> input -> Agent delta -> output -> predicates -> wtool -> Q_{t+1}`.
- FC2: Boot/full architecture, `constitution.md` around line 571.
  Human spec / InitAI / predicates / Q0 / map-reduce tick / halt.
- FC3: Meta-architecture, `constitution.md` around line 826.
  Constitution + logs archive -> ArchitectAI/Veto-AI -> tools/logs -> feedback -> re-init.

`handover/alignment/TRACE_FLOWCHART_MATRIX.md` carries the canonical hashes.
FC1 is split into 1a/1b hashes because the runtime loop spans two fragments.
Treat those hashes as architectural contracts; changing the flowcharts is a
Class 4 constitution event.

Before designing or implementing any non-trivial change, state which FC nodes
or invariants the change touches. If the FC mapping is unclear, inspect
`TRACE_FLOWCHART_MATRIX.md` before editing.

## 4. Operating Loop

Prime operating mode: Constitutional Harness Engineering.

Required loop:

1. Write or identify the constitution/test gate first.
2. Run the minimal real evidence path when the change is evidence-bearing.
3. Implement only enough to make the harness and evidence correct.
4. Re-run the relevant checks.
5. For high-risk or ship-path work, request clean-context Codex audit after
   implementation evidence exists.
6. Ship only after gates, evidence, and review agree.

Forbidden loop:

```text
charter -> atom -> self-audit -> external audit -> more docs -> delayed test
```

No tape, no test. Stdout, human-readable dashboards, private counters, LLM
self-reports, final proof text, unanchored JSON, memory-only preseed, or global
latest pointers are not sufficient evidence.

## 5. Risk Classes

Use the project risk model before editing:

- Class 0: docs, plans, charters, handover updates.
- Class 1: additive isolated helpers, parsers, formatters, non-authoritative
  views.
- Class 2: production wire-up, evaluator adapters, dashboards, replay
  verifiers, benchmark harness code.
- Class 3: auth, money, CAS integrity, capabilities, market/economic state,
  production evidence, `audit_tape`.
- Class 4: constitution, flowcharts, sequencer admission, typed transaction
  schema, canonical signing payload, RootBox/kernel-level authority.

Class 3/4 work requires harness -> real evidence -> audit. Do not audit a
failing harness as a pass.

Class 4 requires explicit per-atom section-8 architect/user ratification before
implementation or ship. One-word messages such as `fix`, `go`, `ok`,
`continue`, or `can` do not constitute Class 4 sign-off.

## 6. Restricted Surfaces

Stop and classify before editing any of these:

- `src/kernel.rs`
- `src/bus.rs`
- `src/sdk/tools/wallet.rs`
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/bottom_white/cas/schema.rs`
- RootBox or canonical signing payload surfaces
- Any sequencer admission rule
- Any typed tx wire schema or discriminant
- Any trust-root or constitution/flowchart authority surface

If a change touches sequencer admission, typed tx schema, or canonical signing
payloads, treat it as Class 4 candidate until proven otherwise. Class 4 cannot
hide inside a Class 3 umbrella.

## 7. Commands

Preferred checks:

```bash
cargo check
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
make constitution
cargo fmt --all
cargo clippy --workspace --tests --no-deps
```

Use `cargo test --workspace --no-fail-fast` for ship-level workspace reports,
not bare `cargo test`. Use targeted tests during development, then broaden
according to risk.

`bash scripts/run_constitution_gates.sh` and `make constitution` are the
constitutional gate paths. Gate tests must be able to fail; a test that cannot
fail is documentation, not a gate.

## 8. Dirty Tree and Evidence

You may start in a dirty worktree. Never revert user changes, generated
evidence, or unrelated drift unless the user explicitly asks. If unrelated
files are modified, ignore them. If existing changes affect your task, work
with them and explain the interaction.

Before any runner that writes to `handover/evidence/` or evaluates real
problems, invoke `/runner-preflight` when available or follow its checklist:

1. clean/understood tree
2. fresh binaries vs current source/HEAD
3. evidence immutability
4. risk class
5. FC trace
6. charter/directive completeness
7. audit-round state

Do not edit Trust-Root-pinned source files during active batch runs. If a fix
is needed mid-batch, abort the batch or accept the wasted run before editing.

Never retroactively rewrite old ChainTape/L4/L4.E/CAS evidence, fabricate a
genesis report, or migrate historical evidence to satisfy new rules. New rules
apply going forward. If an old document is stale, add a new OBS/annotation
document rather than mutating historical evidence.

## 9. Audit Default

Default audit path for this repository is now one clean-context Codex audit.
Do not require Gemini unless a future user message or directive explicitly asks
for Gemini.

Implementation agents must not self-certify high-risk work. For Class 3/4 or
ship-path changes, after implementation evidence exists, invoke a fresh
clean-context Codex reviewer/session. Provide only:

- task brief and risk class
- touched FC nodes/invariants
- current diff or commit
- relevant source/docs
- evidence paths
- exact verification command output
- required verdict format: `PROCEED | CHALLENGE | VETO`

Do not provide the implementation transcript. The reviewer must lead with
findings, cite files/lines, distinguish production defects from test-scaffold
gaps, and end with a clear verdict.

Conservative interpretation:

- `VETO` blocks ship.
- `CHALLENGE` requires fix or explicit forward deferral with rationale.
- `PROCEED` is necessary but not a substitute for passing gates/evidence.

## 10. Self-Hosting Dev Entry

Use `turingos_dev` for non-trivial harness/code work once available. It is a
development-evidence sidecar, not a second canonical tape:

Detailed operating playbook: `HARNESS_MANUAL.md`.

```bash
turingos_dev open --title <title> --module <module> --risk <0-4> --fc <nodes> --allowed <paths>
turingos_dev record-diff --run <run_id>
turingos_dev record-command --run <run_id> -- <command...>
turingos_dev record-audit --run <run_id> --reviewer clean-context-codex --verdict PROCEED|CHALLENGE|VETO --file <audit.md>
turingos_dev validate --run <run_id>
turingos_dev close --run <run_id>
```

No global latest pointer: use `--run`, `--run-dir`, or explicit
`TURINGOS_DEV_RUN`. `record-diff` and `close` must fail closed on restricted
surface hits, broken event hash chains, failing commands, or missing required
audit.

## 11. Done Definition

A task is done only when:

- relevant gates/tests for the risk class pass
- the diff has been reviewed against touched FC nodes
- evidence is linked or intentionally not required for the risk class
- Class 3/4 or ship-path work has clean-context Codex review
- `handover/ai-direct/LATEST.md` changes only when dynamic state truly changes

## 12. Engineering Rules

Use `rg`/`rg --files` for search. Prefer existing patterns, types, parsers, and
helpers over inventing new abstractions. Keep edits scoped to the task.

For structured data, use structured parsers/APIs where available. Avoid ad hoc
string parsing for schemas, manifests, chain records, or evidence payloads.

Money/economy paths must use integer math only. No `f64`/`f32` in money or
market conservation paths.

Agent read views must be scoped, reconstructable, and shielded. Do not expose
raw Lean stderr, raw autopsy logs, private diagnostics, benchmark leaks, or
untriaged historical logs in ordinary agent prompts.

Canonical IDs and shadow IDs must not be mixed. Dashboard and report code must
derive from ChainTape/CAS, not become a source of truth.

No workaround closures: do not turn a failing gate into a skip, null pointer,
empty evidence path, or dashboard-only proof. Align with the constitution and
FC1/FC2/FC3, or stop.

## 11. Done Definition

A task is done only when:

- The touched FC nodes and risk class are stated.
- Relevant unit/integration/constitution gates pass.
- Evidence-bearing changes have a minimal real run or an explicit reason why no
  real run is required.
- The diff is reviewed for regressions, hidden Class 4 surfaces, evidence
  rewrite, ID namespace drift, and money/tape/shielding violations.
- Clean-context Codex audit is completed for high-risk or ship-path work.
- Dynamic handover files are updated only if current project state actually
  changed.

For docs-only changes like this file, no Rust tests are required unless the doc
change also modifies scripts, source code, or executable workflow.

## 12. Codex Guidance Maintenance

This file should stay concise and load-bearing. If Codex repeats the same
mistake twice, update `AGENTS.md` or create a small referenced playbook/skill.
If this file grows too large, split task-specific material into separate docs
and reference them here.

Useful research baseline:

- Codex docs: keep `AGENTS.md` practical, include commands/constraints/done
  criteria, and test/review work before accepting it.
- Evals best practice: use task-specific evals, trace/tool-level checks,
  continuous evaluation, and clear graders rather than final-output vibes.
- Recent AGENTS/harness research is mixed on instruction volume; therefore
  prefer minimal, accurate, high-signal instructions over long repeated policy.
