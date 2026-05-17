# TuringOS v4

TuringOS v4 is a tape-first constitutional operating substrate for LLM/AGI
agents. The authoritative state of a run is ChainTape plus CAS evidence; reports,
dashboards, and handover notes are materialized views.

## Current Main Status

- `main` includes the audited CAS Git constitutional repair merged by PR #3.
- CAS repair merge commit:
  `802b18053d063bd5503a6b0eb2e7b1f46ceda93b`.
- CAS now has a Git commit-chain layer while preserving `Cid = sha256(content)`.
  New CAS writes advance `refs/chaintape/cas` as a commit chain and keep the
  sidecar index as rebuildable cache.
- CAS `open()` / reload paths take the same CAS chain lock used by `put()`, so
  readers do not misclassify an in-flight commit-chain + sidecar refresh as hard
  corruption.
- MiniF2F is a development benchmark package, not a fixed TuringOS kernel or OS
  gate. It is excluded from the root workspace and is only run explicitly via
  `--manifest-path experiments/minif2f_v4/Cargo.toml`.

## Authoritative Orientation

Read these first for a cold start:

1. `AGENTS.md`
2. `CLAUDE.md`
3. `HARNESS_MANUAL.md`
4. `constitution.md`
5. `handover/ai-direct/LATEST.md`
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
7. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`

Truth order is defined in `AGENTS.md`: constitution and flowchart contracts
outrank ChainTape/CAS, gates, handover, dashboards, and README text.

## Core Checks

Preferred ship-level checks:

```bash
git diff --check
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

For MiniF2F development work, use the experiment manifest explicitly:

```bash
cargo test --manifest-path experiments/minif2f_v4/Cargo.toml -- --test-threads=1
```

Do not treat MiniF2F as a default root workspace or core constitution gate.
