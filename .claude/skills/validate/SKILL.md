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
cargo test
```

## Layer C: Kernel Audit
Invoke `auditor` agent for full integrity check.

## Layer D: External Audit (if code changes)
Delegate to Codex or Gemini per Rule 23.

Report: PASS/FAIL per layer. Fail-closed — any layer failure = overall FAIL.
