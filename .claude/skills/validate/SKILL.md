---
name: validate
description: Multi-layer validation — cargo check, cargo test, and kernel purity audit
user_invocable: true
---

# /validate — Multi-Layer Validation

## Layer A: Compilation
```bash
cargo check
```

## Layer B: Tests
```bash
cargo test --workspace --no-fail-fast
```

## Layer C: Kernel Audit
Invoke `auditor` agent for full integrity check.

## Layer D: External Audit (if code changes)
Request one clean-context audit by a fresh agent on any capable platform
(Claude / Codex / Antigravity / …) per AGENTS.md §9. The auditor must run in a
clean context and must not hold the implementation transcript.

Report: PASS/FAIL per layer. Fail-closed — any layer failure = overall FAIL.
