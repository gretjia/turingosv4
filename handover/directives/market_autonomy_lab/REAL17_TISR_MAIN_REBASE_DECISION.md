# REAL-17 TISR-Main Rebase Decision

Date: 2026-05-17

## Decision

Continue REAL-17 from a fresh worktree based on the updated `origin/main`
TISR/CAS baseline, not from the older REAL-17 worktree as the forward evidence
baseline.

Forward worktree:

```text
path: /home/zephryj/projects/turingosv4-real17-tisr-main
branch: codex/real17-emergence-hardening-tisr-main-20260517
base: origin/main @ 53cc4442253f49753d76d8126de51a1c9ddbc1b7
```

The previous REAL-17 worktree remains historical evidence context:

```text
path: /home/zephryj/projects/turingosv4-real17-main-cas
branch: codex/real17-emergence-hardening-20260517
head: 803ecc17f885f8a84cdd517208e3e15aea232b51
```

Do not merge the old worktree evidence wholesale into the new branch before
rerunning claim-bearing evidence on the latest main baseline.

## Main README Findings

The updated `README.md` says current `main` includes:

```text
TISR Phase 6.0-6.3 alpha CLI stack
turingos CLI as the primary user entry point
SiliconFlow-backed two-LLM wire
spec/generate paths with CAS-anchored EvidenceCapsule records
CAS Git constitutional repair with refs/chaintape/cas commit-chain writes
MiniF2F excluded from the root workspace and run explicitly by manifest
```

The top of `handover/ai-direct/LATEST.md` records the same Phase 6.3 state and
explicitly says the Phase 6.0-6.3 alpha ship does not block ongoing
G-Phase/market-autonomy work on main or active feature branches.

## Integration Performed

The forward branch cherry-picked the REAL-17 code-path commits onto latest
`origin/main`:

```text
1d532ad2 REAL-17 integrate market hardening on CAS main
60b88ba0 REAL-17 add direct prompt provenance sidecar
fec9300b REAL-17 expose strict direct provenance verifier flag
35ee961f REAL-17 let BCAST skip prompt provenance sidecar
626a255d REAL-17 stabilize NO EventResolve poll budget
```

The older P3 evidence freeze commit was not cherry-picked. Its evidence remains
valid only for its recorded old head and is not used as forward claim-bearing
evidence on the TISR/CAS main baseline.

## Why Not Merge The Old Worktree First

The old worktree includes useful REAL-17/P3 historical evidence, but that
evidence was generated before the latest TISR Phase 6.3 main update. Merging it
directly would risk mixing:

```text
old-head runtime/CAS evidence
new-head TISR/CAS CLI semantics
historical candidate labels
future claim-bearing labels
```

The constitutional path is to keep historical evidence immutable, integrate only
the required code mechanisms, and rerun future evidence on the latest baseline.

## Verification On The New Baseline

Commands run on the forward branch:

```text
git diff --check
exit 0

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
exit 0

cargo test --test constitution_real17_evaluator_prompt_provenance_wire \
  --test constitution_real17_market_decision_provenance_link \
  --test constitution_real14_e2_candidate_verifier \
  --test constitution_librarian_market_no_trade \
  --test constitution_real6_task_outcome_market \
  -- --test-threads=1
exit 0

cargo check
exit 0

cargo build --bin turingos
exit 0

cargo build --bin real14_e2_candidate_verifier
exit 0
```

`cargo fmt --all -- --check` currently reports formatting diffs in the newly
landed TISR CLI files from `origin/main`. This was treated as inherited
mainline formatting drift for this decision because the touched REAL-17
mechanism gates, Trust Root verification, diff whitespace check, and builds pass.

## Claim Boundary

Allowed forward status:

```text
REAL-17 may continue on the updated TISR/CAS main baseline.
Future claim-bearing market-autonomy evidence must be regenerated here.
```

Historical labels remain candidate-only:

```text
E2 replicated candidate
two-sided market candidate
E3 candidate pending audit
E4a candidate pending audit
market emergence candidate -- final audit PROCEED, hardening pending
```

Forbidden active claims remain forbidden:

```text
E2 achieved
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
```

## Next Step

Run the next REAL-17 claim-bearing evidence cycle from
`/home/zephryj/projects/turingosv4-real17-tisr-main`, using the latest
TISR/CAS main baseline. The first regenerated run should verify direct
PromptCapsule provenance with the strict exact-join verifier before any stronger
market-emergence label is considered.
