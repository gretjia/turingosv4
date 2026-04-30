# Codex TB-2 Phase-1c R2 Narrowed Audit
**Date**: 2026-04-30
**Target**: experiment branch HEAD `abf3581` (remediation r1)
**Base**: `138d5ac` (the prior Phase-1c r1 audit subject)
**Mandate**: verify remediation r1 fixes the 3 r1 CHALLENGE items without introducing new issues.
**Note**: Codex sandbox blocked filesystem writes to `handover/audits/`; this report transcribed manually from the agent's returned summary.

---

## Section A: Overall Verdict

**Verdict: CHALLENGE. Conviction: 4/5.**

R2-Q1 PASS, R2-Q2 PASS, R2-Q3 CHALLENGE (procedural — frame misread; see merged verdict for resolution), R2-Q4 UNVERIFIED (sandbox cargo blocked).

## Section B: Per-Q Disposition

**R2-Q1 — PASS.** The visibility shuffle is correct and shrinks the public TB-2 helper surface. `WORKTX_ACCEPT_DOMAIN_V1` and `worktx_canonical_hash` are demoted to `pub(crate)`; `worktx_accept_state_root` is promoted to `pub`. I9 in `tests/tb_2_runtime_boundary.rs` calls `worktx_accept_state_root(&q0.state_root_t, &tx)` directly. The single new public item is semantically a cleaner surface (the accept-side state-root contract) than the two raw building blocks. Public surface count: WAS 2 (domain const + canonical hash); IS 1 (composite helper).

**R2-Q2 — PASS.** The literal TB-3 deletion marker is present at `src/state/sequencer.rs:205` adjacent to the bridge line `let lookup_tx_id = TxId(work.task_id.0.clone())` at `:206`. Exact text matches preflight v3 §8 line 9.

**R2-Q3 — CHALLENGE.** The diff `138d5ac..abf3581` for `src/bottom_white/ledger/transition_ledger.rs` shows TWO changes inside the test arm: the matches!() variant (NotYetImplemented → EscrowMissing) AND the inline `h(1)` comment ("dispatch reject" → "dispatch stub"). Codex r1 Q7 demanded "trim to expected assertion-only change." The remediation diff reintroduces the comment-line change, even if it's reverting it.

**R2-Q4 — UNVERIFIED.** All cargo invocations exited 101 due to read-only `target/debug/.cargo-lock` in the Codex sandbox. Pass counts could not be confirmed inside this audit.

## Section C: New Issues

None. The visibility / marker / sandbox-cargo issues are all handled.

## Section D: Recommendation

Resolve R2-Q3 — either revert the inline comment to match the prior wording exactly, or amend the audit comparison frame to use main base `f9ace5e..abf3581` (which would render this finding moot). Then re-run `cargo test --workspace` outside the sandbox and confirm 13/13 + 9/9 + boot PASS before merge.
