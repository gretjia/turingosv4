# OBS_R023 — `evaluator.rs:2956` writes hardcoded `RunOutcome::MaxTxExhausted` EvidenceCapsule on every chaintape_bundle drain (Art. IV terminal-state semantic-purity)

**Date discovered**: 2026-05-05 (TB-16.x.2.2.fix Patch A bug-hunt session)
**Discovered by**: Claude (Opus 4.7) under user instruction "根据宪法和架构师意见自主决策" while fixing TB-16.x.2.2 FORCE_CHALLENGE_RESOLVE hook placement
**Severity**: latent (currently masked, not exploited; out-of-scope for TB-16.x.2.2 ship-gate)
**Sudo required**: no — surgical refactor; ratification path = next TB owning EvidenceCapsule semantic-purity
**Filed by**: Claude session 2026-05-05 (TB-16.x.2.2.fix)

---

## §1 The fact

`experiments/minif2f_v4/src/bin/evaluator.rs` lines 2940-3137 wrap an `if let Some(bundle) = chaintape_bundle { ... }` block in which an inner unconditional `{}` scope (line 2956) writes an `EvidenceCapsule` to CAS with **hardcoded** `RunOutcome::MaxTxExhausted` (line 3033) and emits a `TerminalSummary` system-tx. There is no outer condition guarding this block on the actual run terminal state.

The block is currently reached only on the MaxTxExhausted exit path of `eval_one_problem` (the function-level early-returns at line 2333 / 2798 for OMEGA-Confirm bypass it). So the hardcoded `MaxTxExhausted` capsule outcome happens to coincide with the actual run outcome **for the only path that reaches this block today**.

## §2 Why it is constitutionally suspect

Art. IV terminal-state distinction is one of the report-standard pillars (`CLAUDE.md` Report Standard §):

> Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}

The EvidenceCapsule `outcome` field carries semantic load — downstream consumers (TB-15 Lamarckian Autopsy, TB-16 Markov inheritance) read it as ground truth. A capsule labeled `MaxTxExhausted` written for a future code path that legitimately reaches it under, say, `WallClockCap` would silently misrepresent run terminal state — Art. I.2 reproducibility violation propagating through the chain.

This is currently **masked** because:
- the only reachable invocation site IS the MaxTxExhausted exit
- so `outcome=MaxTxExhausted` is accidentally correct
- but the **structural separation** "always write capsule" vs "always label MaxTxExhausted" is the bug — the moment a future TB adds another reaching path (e.g. WallClockCap exit feeding into the same cleanup), the capsule will silently mislabel.

## §3 Why this OBS, not a fix-now

`feedback_audit_obs_bias` decision table:

| dimension | value |
|---|---|
| id | OBS_R023 (filed under R022 OBS family per existing convention) |
| fix cost | non-trivial (~30-60 min): plumb the actual `RunOutcome` from caller into the capsule construction; touches `eval_one_problem` signature + every caller site |
| severity | latent — masked today, hidden trap for a future TB |
| contradicts prior user instruction? | no — instruction was "fix TB-16.x.2.2 ship gate honestly per constitution and architect intent"; this block is orthogonal to ChallengeResolve hook placement |
| OBS-defer rationale | multi-hour future-arch class — the right home is a TB owning EvidenceCapsule purity (likely TB-15.x or RSP-3.2 if Lamarckian Autopsy expands), not a band-aid in TB-16.x.2.2.fix |

## §4 Reproduction

`grep -n "RunOutcome::MaxTxExhausted" experiments/minif2f_v4/src/bin/evaluator.rs` shows the hardcoded outcome at line 3033, inside the unconditional `{}` block at lines 2956-3137.

The outer `if let Some(bundle) = chaintape_bundle {` (line 2940) is itself the function tail that runs after both the OMEGA-early-return paths (line 2333 / 2798) have NOT been taken. So semantically it IS the MaxTxExhausted path today. But the proper fix is: write `outcome` from the caller-supplied terminal state, not literal `MaxTxExhausted`.

## §5 Proposed remediation (deferred)

In a future TB:

1. Plumb the actual `RunOutcome` discriminant into `eval_one_problem` return / capsule construction. Candidate: extract from the same `apply_mode_to_accept(mode, false, false)` branch decision at line 2917 (or earlier in the function where the budget-exhausted vs wall-clock vs error-halt distinction is made).
2. Replace `RunOutcome::MaxTxExhausted` literal at line 3033 with the propagated value.
3. Add a regression test that constructs a synthetic non-MaxTx exit and asserts capsule.outcome matches.

## §6 Status

- **Filed**: 2026-05-05
- **Owner**: deferred to TB-15.x or first TB that adds a non-MaxTxExhausted path through the same cleanup section
- **Blocking**: no (out-of-scope for TB-16.x.2.2.fix; masked at HEAD)
- **Cross-ref**: TB-16.x.2.2 commit `5e32cbf` + TB-16.x.2.2.fix (this session); discovery context = FORCE_CHALLENGE_RESOLVE hook placement bug fix
