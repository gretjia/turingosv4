# REAL-17 Atom 1 Direct PromptCapsule Provenance Audit

Status: PROCEED as in-envelope research hardening.

Claim boundary:

- This does not claim E2/E3/E4 achieved.
- This does not claim market emergence proven.
- This does not authorize ship or main merge by itself.
- This only approves continuing REAL-17 evidence generation with direct
  PromptCapsule provenance hardening on the repaired CAS baseline.

## Scope

Worktree:

```text
/home/zephryj/projects/turingosv4-real17-main-cas
```

Branch:

```text
codex/real17-emergence-hardening-20260517
```

Base:

```text
origin/main 88fa1f6694ca6b423d9e7df54e4872c1a47f6b1f
```

Risk class:

```text
Class 3 / Trust-root-pinned research-envelope hardening.
No Class 4 typed-tx, sequencer, canonical signing, CAS ObjectType schema,
kernel, bus, wallet, constitution, or flowchart change.
```

Touched invariants:

```text
FC1 runtime evidence write/read path.
ChainTape/CAS evidence truth order.
Prompt provenance shielding.
No raw prompt/completion/CoT/log broadcast.
Exact-join verifier remains ChainTape/CAS-derived.
Dashboard remains a materialized view.
```

## Patch Summary

Atom 1 uses an additive sidecar instead of mutating the
`MarketDecisionTrace` schema:

```text
MarketDecisionProvenanceLink
schema_id = real17.market_decision_provenance_link.v1
CAS ObjectType = Generic
```

The sidecar links:

```text
submitted MarketDecisionTrace CID
submitted router tx id
agent id
same-turn PromptCapsule CID
optional EVDecisionTrace CID
optional MarketOpportunityTrace CID
```

The exact-join verifier now prefers direct sidecar PromptCapsule provenance
and preserves the historical indirect EVDecisionTrace fallback. Future strict
REAL-17 gates can require direct PromptCapsule provenance with
`require_direct_prompt_capsule_provenance=true`.

## Verification

The orchestrator ran:

```text
cargo fmt --all -- --check
git diff --check
cargo test --test constitution_real17_market_decision_provenance_link -- --test-threads=1
cargo test --test constitution_real17_evaluator_prompt_provenance_wire -- --test-threads=1
cargo test --test constitution_real14_e2_candidate_verifier -- --test-threads=1
cargo check --bins
CARGO_TARGET_DIR=target cargo check --manifest-path experiments/minif2f_v4/Cargo.toml --bin evaluator
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Observed results:

```text
fmt/check/diff check: pass
REAL-17 sidecar tests: 2 passed / 0 failed
REAL-17 evaluator source wiring gate: 1 passed / 0 failed
REAL-14 verifier tests: 10 passed / 0 failed
evaluator cargo check: pass
Trust Root verify after allowed envelope rehash: pass
constitution gates: 461 passed / 0 failed / 1 ignored
workspace tests: completed with no failures in final output
restricted-surface filename scan: no forbidden surface hits
```

The allowed Trust Root rehash covered only touched pinned files:

```text
experiments/minif2f_v4/src/bin/evaluator.rs
src/runtime/mod.rs
src/bin/audit_dashboard.rs
```

## Clean-Context Audit

Auditor verdict:

```text
PROCEED
```

Findings summary:

- No blocking production defects found.
- The sidecar validates schema, submitted trace linkage, router tx id,
  agent id, and PromptCapsuleV2 decode before CAS write.
- The sidecar uses `ObjectType::Generic` with schema id and does not change
  CAS ObjectType schema.
- Evaluator wiring writes the sidecar only after a submitted router tx and
  MarketDecisionTrace CID exist.
- The verifier remains ChainTape/CAS-derived, adds direct/indirect/missing
  counts, and can VETO missing direct provenance in strict mode.
- The dashboard renders verifier-derived counts only.

Non-blocking gap:

```text
The evaluator wiring gate is source-string based. A follow-up true-problem run
must prove that live submitted market decisions produce direct PromptCapsule
sidecars in repaired-CAS evidence.
```

## Next Action

Proceed to REAL-17 forward evidence generation on the repaired CAS baseline:

```text
1. Run a true hard10 or hard20 direct-provenance validation.
2. Enable strict direct PromptCapsule provenance in the verifier for new
   claim-bearing REAL-17 runs.
3. If live exact-join actions appear, require direct provenance count to equal
   exact_join_count before stronger hardening labels.
4. If no actions appear, write clean-negative and continue the market-emergence
   loop under the constitution.
```
