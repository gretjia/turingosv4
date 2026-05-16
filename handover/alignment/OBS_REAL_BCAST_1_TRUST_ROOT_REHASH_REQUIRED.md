# OBS_REAL_BCAST_1_TRUST_ROOT_REHASH_REQUIRED

Date: 2026-05-16

## Status

REAL-BCAST-1 implementation reached the evaluator prompt-injection surface:

```text
experiments/minif2f_v4/src/bin/evaluator.rs
```

That file is Trust-Root pinned. The first REAL-BCAST A/B smoke attempt
therefore failed closed at boot:

```text
Evidence dir:
handover/evidence/real_bcast_1_ab_A_20260516T000000Z/

Failure:
TRUST_ROOT_TAMPERED: experiments/minif2f_v4/src/bin/evaluator.rs hash mismatch
expected 40f645af21d0a908e3b49d9f8aa9d1810304a39462baa6fba6106572bae0ad01
actual   38e281876a95d51d993ca2c0a7aeac02cbabc818742574b45887680da0837d12
```

This is a correct fail-closed result, not a batch conclusion.

## Interpretation

REAL-BCAST-1 was planned as Class 3 by default, but prompt injection into the
real MiniF2F evaluator changes a Trust-Root-pinned executable surface. The code
can be tested and audited, but conclusion-bearing real A/B evidence cannot be
produced from this worktree until the Trust Root package is explicitly
ratified and rehashed.

## Boundary

Do not:

```text
- skip Trust Root verification;
- treat real_bcast_1_ab_A_20260516T000000Z as conclusion-bearing evidence;
- claim REAL-BCAST A/B smoke PROCEED;
- ship evaluator prompt-injection changes without Trust Root ratification.
```

Allowed while pending ratification:

```text
- targeted unit/integration tests;
- cargo check for affected binaries;
- clean-context audit of the diff;
- drafting the Class-4 ratification packet / Trust Root rehash request.
```

## Required Next Step

If the architect/user wants REAL-BCAST prompt injection to proceed through the
real runner, approve a Class-4 Trust Root rehash package for the pinned
evaluator change. After rehash:

```text
1. verify_trust_root must pass;
2. rerun REAL-BCAST A/B smoke;
3. audit_tape must PROCEED for both arms;
4. the contaminated pre-rehash A arm remains remediation-only.
```
