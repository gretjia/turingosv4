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

Truth order (3 tiers, flat — receipt-driven; see
`handover/architect-insights/K-2-2_TRUTH_TIER_GREP_RECEIPTS.md` for src/ grep
evidence):

**Tier 1: Axioms** (immutable, checked at compile/start time)
- `constitution.md`
- The 3 canonical flowchart hashes (stored in tests and docs)

**Tier 2: Facts** (live state machine)
- ChainTape (L4 + L4.E transitions)
- CAS (evidence objects, indexed by content hash)
- Replay/audit verifier (deterministic reconstruction from ChainTape + CAS)

**Tier 3: Workspace pointers** (mutable, derived)
- Current TB charter / directive / ratification
- `handover/ai-direct/LATEST.md` (explicit derived view; ChainTape wins if conflict)

**Derived views** (all below tier 3 — no src/ runtime reader, per K-2.2 receipts):
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
- `handover/tracer_bullets/TB_LOG.tsv` (append-only log)
- Dashboards, reports, README files, stdout logs

If a derived view contradicts ChainTape/CAS, trust ChainTape/CAS.
If ChainTape/CAS contradicts constitution gates, stop.

## 2. Cold Start

For a new non-trivial task, read in this order:

1. `CLAUDE.md`
2. `HARNESS_MANUAL.md`
3. `constitution.md`
4. `handover/ai-direct/LATEST.md`
5. Key Coding Principles: [KARPATHY_ARCHITECT.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_ARCHITECT.md) & [KARPATHY_SIMPLE_CODE.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_SIMPLE_CODE.md)
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
7. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
8. Current directives/charters relevant to the task
9. Source files and tests for the touched surface

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

## 13. Key Coding Principles (Karpathy Skills)

All agents must strictly adhere to the following coding and architectural guidelines:

- **Karpathy Architect Skill**: Apply first-principles architecture, data-flow-first design, monolithic/flat default architecture, and micro-implementation. See [KARPATHY_ARCHITECT.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_ARCHITECT.md).
- **Karpathy Simple Code Skill**: Focus on direct computation, small state machines, transparent data flow, and minimal abstractions. Avoid unnecessary dependencies and boilerplate lifecycle complexity. See [KARPATHY_SIMPLE_CODE.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_SIMPLE_CODE.md).

## 14. Class-by-Class Cadence & Audit Checklist (K-4.1, v3 plan §5)

### Cadence

Match ceremony to risk class — Karpathy: surgical changes. Old loop (charter →
atom → self-audit → external audit → more docs → delayed test) is forbidden;
required loop is constitution gate → real run → debug → fix → rerun → audit →
ship.

| Class | Charter | Directive | Matrix update | §8 | Independent audit (witness only) | Memory |
|-------|---------|-----------|---------------|-----|-----------------------------------|--------|
| 0 docs | no | no | no | no | no | only recurring rule |
| 1 additive | no | no | no | no | predicate self-test only | only recurring rule |
| 2 wire-up | brief | optional | yes | no | clean-context Codex audit (witness, output `{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}`) | surprise only |
| 3 auth/money/CAS | TB charter | yes | yes | required | full dual independent witness (Codex + Gemini) | yes |
| 4 constitution/sequencer | TB charter | yes | yes | per-atom §8 | dual independent witness PRE-§8 | yes |

**Audit witness is not judge.** Ship gate = predicates GREEN (hard judge —
`cargo test --workspace`, `bash scripts/run_constitution_gates.sh`,
`cargo test --test constitution_matrix_drift`) ∧ audit witness output ≠
unresolved violation. Subjective code-style / performance / coverage /
architecture-preference opinions are out-of-scope per constitutional Veto-AI
boundary (output domain `{PASS, VETO}`).

### Predicate verification checklist (pre-merge, machine-deterministic)

Auditor copy-pastes this batch; no human judgment in any line:

- [ ] PR title states risk class
- [ ] `git diff main --name-only` 未触及 §6 restricted-surface 列表，OR 该 PR 引用 per-atom §8 directive 文件
- [ ] `cargo test --workspace --no-fail-fast` exit 0
- [ ] `bash scripts/run_constitution_gates.sh` exit 0
- [ ] `cargo test --test constitution_matrix_drift` exit 0
- [ ] PR body 含 acceptance-criteria 命令块与期望输出
- [ ] 该 atom 自身的 predicate verification recipe 输出 `PREDICATES-GREEN`
- [ ] 无新 `Manager` / `Factory` / `Engine` / `Platform` / `Framework` type（structural grep）
- [ ] 无新 trait + 单一非-idiomatic impl（structural grep）
- [ ] 无新 board-as-truth 文件存在 evidence chain 外
- [ ] 无新 global latest pointer 作为 canonical input

### Independent audit witness verdict domain

Clean-context Codex / Gemini audit 可合法输出（仅这四个）:
- `NO-VIOLATION`（扫了 N 条款，无违宪发现）
- `VIOLATION-FOUND <constitutional-clause> <file>:<line>`
- `RECONSTRUCTION-FAILURE <which-tape-or-cas-path-cannot-be-reconstructed>`
- `SECOND-SOURCE-DRIFT <which-derived-view-is-usurping-ground-truth>`

出现以下即越界，可由 user 或 Claude 主体拒收该次 audit 报告：
- "I think the code style ..."
- "Performance could be improved ..."
- "Test coverage feels low ..."
- "Architecture would be better if ..."
- 任何其他主观品味 / 性能 / 覆盖率 / 架构偏好类语句

## 15. Codex Guidance Maintenance

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
