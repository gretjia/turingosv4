# TuringOS PR Contract

## OS Layer

L0/L1/L2/L3/L4/L5/L6/L7/L8/L9:

## Risk Class

Class 0/1/2/3/4:

## Exact File List

```text
<all files changed>
```

## Forbidden Surface Check

- `constitution.md` touched? yes/no
- `build.rs` touched? yes/no
- `genesis_payload.toml` touched? yes/no
- sequencer/schema/signing touched? yes/no
- `src/lib.rs` or `src/runtime/mod.rs` touched? yes/no
- any §6 restricted surface touched? yes/no
- if yes, cite risk class and §8 ratification:

## Source Of Truth Claim

- Does this PR create or modify source of truth? yes/no
- If yes, why is it ChainTape/GitTape/CAS authority?
- If no, what `derive_from_tape` or replay test proves projection-only status?

## Claim Boundary

Supported:

Unsupported:

## Snapshot Ancestry Check

```bash
set -euo pipefail
git fetch origin pull/280/head:refs/audit-snapshots/pr-280
git fetch origin pull/283/head:refs/audit-snapshots/pr-283
test "$(git rev-parse refs/audit-snapshots/pr-280)" = e1605911c883aea4ce842b7fee7d41bd0448f947
test "$(git rev-parse refs/audit-snapshots/pr-283)" = 4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
for oid in \
  e1605911c883aea4ce842b7fee7d41bd0448f947 \
  4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
do
  if git merge-base --is-ancestor "$oid" HEAD; then
    echo "forbidden audit snapshot ancestor: $oid"
    exit 1
  fi
done
```

Expected: exit 0.

## Tests

Targeted:

```bash
<exact commands and expected outputs>
```

Constitution gates:

```bash
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
```

Workspace if applicable:

```bash
cargo test --workspace --no-fail-fast
```

Diff checks:

```bash
git diff --check
git diff --cached --check
```

Claim scan:

```bash
claim_hits=$(grep -RInE 'PROVEN|DEFINITIVE|causal|world-first|TASK-PASS' \
  handover src tests AGENTS.md CLAUDE.md .github \
  | grep -vE 'no-proven-checklist|CLAIM_INTEGRITY|pull_request_template|AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT|P1_REALVALUE_SCOPE_CORRECTION|A14_WORKLOAD_ADAPTER_BOUNDARY_PREFLIGHT' || true)
test -z "$claim_hits" || { printf '%s\n' "$claim_hits"; exit 1; }
```

Secret scan:

```bash
grep -RInE 'api[_-]?key|Authorization|Bearer|SECRET|TOKEN' \
  . --exclude-dir target --exclude-dir .git || true
```

Raw evidence scan:

```bash
find handover/evidence -type f \
  \( -name 'run_*.log' -o -name 'CRACK_*.txt' -o -path '*repo_*' -o -path '*cas_*' \) -print
```

## Clean-Context Audit

- Required for Class 2+? yes/no
- Verdict:
- Evidence path:

## Global Hard Blockers

Any hit blocks merge:

```text
B1. PR is based on #280/#283 by merge instead of clean split.
B2. PR mixes runtime code and bulk evidence JSON/logs.
B3. PR adds raw repo_*/cas_* logs to main.
B4. PR creates wallet/market/price as non-derived source of truth.
B5. PR lets price override predicate.
B6. PR claims TASK-PASS without verifier-backed TASK-PASS.
B7. PR touches build.rs/genesis_payload.toml outside declared trust-root atom.
B8. PR adds benchmark adapter into kernel authority path.
B9. PR contains PROVEN/DEFINITIVE/causal/world-first headline without G1-G6.
B10. PR leaks hidden tests/raw stderr/private diagnostics into AgentView.
B11. PR adds a new latest/global pointer as canonical input.
B12. PR adds Manager/Factory/Engine/Platform/Framework as fake future ceremony.
B13. PR introduces f32/f64 in money/economy conservation paths.
B14. PR audits before runnable evidence exists for Class 2+ ship work.
B15. PR rewrites historical ChainTape/L4/L4.E/CAS evidence.
```
