# A13 Agentic OS E2E CLI Clean-Context Audit - 2026-06-06

Risk: Class 2

Scope:
- A13 network-off Agentic OS v0 E2E CLI.
- `turingos boot --verify-manifest`
- `turingos os run --task fixtures/os/hello_agentic_task.json --policy single_tree --market on --network off`
- `turingos os replay --run-dir <run-dir>`
- `turingos os audit --run-dir <run-dir>`

Touched FC nodes:
- FC1 runtime loop: deterministic network-off intent, terminal, predicate receipts, and scoped agent view.
- FC2 boot/replay: trust-root verification wrapper and replay from ChainTape/GitTape plus CAS.
- FC3 audit: machine-readable predicate summary emitted by `turingos os audit`.

Witness:
- Clean-context Codex auditor.
- Read-only, ephemeral sandbox.
- No implementation transcript provided.
- Compared tracked and untracked changes against `origin/main`.

Witness summary:
- No Section 6 restricted surfaces modified.
- No provider/network execution path introduced.
- Money/economy path remains integer microcredits.
- Replay/audit reconstruct from `git_tape_repo/tape/events.jsonl` plus the CAS fixture object, then verify derived artifact bytes and hashes instead of trusting `run_manifest.json`, `replay_report.json`, `economy_projection.json`, or `agent_view_audit.json`.
- `production_module_liveness.toml` remains `OBL005_REAUDIT_IN_PROGRESS`; A13 does not claim final OBL-005 closure.

Witness note:
- The witness did not rerun cargo or constitution gates from the read-only sandbox.
- The witness did run `git diff --check origin/main` and `rustfmt --edition 2021 --check` on the new Rust files successfully.

Final verdict:

```text
NO-VIOLATION
```
