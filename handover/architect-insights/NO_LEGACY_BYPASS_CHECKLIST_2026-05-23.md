# NO_LEGACY_BYPASS Checklist — TB-SOFTWARE-3-0 Atom S5.2

**Date**: 2026-05-23
**Risk class**: 0 (docs)
**Charter**: `handover/tracer_bullets/TB-SOFTWARE-3-0_charter_2026-05-23.md`
**§8 directive**: `handover/directives/2026-05-23_SOFTWARE_3_0_CONSOLIDATION_DIRECTIVE_AND_§8.md`
**Companion script**: `scripts/audit_legacy_bypass.sh`

## Purpose

This checklist is a pre-merge reporting baseline. It tells reviewers and
audit witnesses where to look for "legacy bypass" smells re-entering the
codebase. It is **not** a constitution gate, **not** wired into
`scripts/run_constitution_gates.sh`, and **not** blocking on CI.

Per Karpathy K14 (no permanent escape hatches; rollback via `git revert`),
the long-term policy is to remove bypass scaffolding rather than hide it
behind feature flags. This checklist makes that scaffolding visible.

## Companion: `scripts/audit_legacy_bypass.sh`

The script audits the 6 patterns below and exits non-zero if any pattern
hits. Run it manually before drafting a PR that touches Class 2/3 surfaces
or before declaring a TB shipped:

```bash
bash scripts/audit_legacy_bypass.sh
# or for CI piping:
bash scripts/audit_legacy_bypass.sh --quiet
```

Non-zero exit is informational, not a hard fail.

## The 6 patterns

### 1. Synthesized id fallback (`t_hash_*` / `simple_hash`)
**Where**: `src/web/*`
**Why bad**: when a CLI exit-0 stdout is unparseable, fabricating a `t_hash_<digest>` id from the stdout bytes is dashboard-only proof (FC2 anti-pattern). S1 removed it; the failure path now returns `502 BAD_GATEWAY` with `kind="task_id_parse_failed"` and writes no `TaskEntry`.
**Test**: `cargo test --features web --test cli_web_write_smoke`

### 2. Ceremonial `// removed` stubs
**Where**: anywhere in `src/`
**Why bad**: tombstone comments accumulate. Per Karpathy "no half-finished implementations" — if code is removed, the diff carries the story; the comment does not.
**Fix**: delete the comment entirely.

### 3. `panic!()` in src/
**Where**: any non-test file in `src/`
**Why bad**: panics in production paths take the whole process down. Production code should `Result`-propagate.
**Exception**: panics inside `#[cfg(test)]` modules are fine.

### 4. `.unwrap()` in `src/web/*`
**Where**: web handlers
**Why bad**: web handlers serve adversarial user input. An `.unwrap()` on a `Result` from JSON parsing or DB lookup hands an attacker an oracle for crashing the server. Use `?` or explicit 4xx/5xx mapping.
**Exception**: `Mutex::lock().unwrap_or_else(|e| e.into_inner())` for poisoned lock recovery is acceptable.

### 5. Legacy-feature flags (`compat_*` / `legacy_*` in Cargo.toml)
**Where**: `Cargo.toml`
**Why bad**: backwards-compat feature flags become permanent escape hatches. Either the old code is needed (then it lives unflagged) or it isn't (then it's deleted). Per Karpathy K14.
**Fix**: delete the flag and either inline or remove the gated code.

### 6. (Reserved for future patterns)
This slot is intentionally open. When a new bypass pattern appears
(e.g. dashboard-only state, shadow-ledger source-of-truth, memory-only
preseed), document it here and add a corresponding grep in
`scripts/audit_legacy_bypass.sh`.

## How to use this in a review

1. Reviewer runs `bash scripts/audit_legacy_bypass.sh`.
2. If exit code is 0: log "NO-LEGACY-BYPASS-FOUND" in the audit, move on.
3. If exit code is non-zero:
   - Read the per-pattern counts.
   - For each non-zero pattern, decide: is this a regression introduced by
     the PR under review, or pre-existing baseline noise?
   - Regression → CHALLENGE.
   - Pre-existing → note in the audit, file a follow-up task, don't block ship.

## Why this is a checklist, not a gate

Per `feedback_audit_after_evidence` (audit AFTER evidence, not before): a
hard grep gate before the PR has real tape evidence would block legitimate
work over surface smells. The right time to apply this checklist is
post-implementation, alongside the clean-context Codex audit.

Per `feedback_audit_obs_bias`: when only "CHALLENGE-only ≠ unresolved
violation" residuals remain, prefer shipping with OBS-deferred follow-ups
over blocking on subjective taste.

## Cross-references

- TB charter: `handover/tracer_bullets/TB-SOFTWARE-3-0_charter_2026-05-23.md`
- §8 directive: `handover/directives/2026-05-23_SOFTWARE_3_0_CONSOLIDATION_DIRECTIVE_AND_§8.md`
- Karpathy K14 (no escape hatches): `skills/KARPATHY_SIMPLE_CODE.md`
- Audit-after-evidence policy: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_audit_after_evidence.md`

TRACE_MATRIX: FC3-replay (the audit reads code state, not chain state — so
it is by definition a derived-view spot-check, not a truth source). Class 0
docs; no constitution gate touched.
