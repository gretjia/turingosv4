# REAL-17 Main-CAS Integration Audit

Date: 2026-05-17

## Scope

This audit covers the integration baseline only:

```text
worktree: /home/zephryj/projects/turingosv4-real17-main-cas
branch: codex/real17-emergence-hardening-20260517
base: origin/main @ 88fa1f6694ca6b423d9e7df54e4872c1a47f6b1f
```

It does not approve `E2 achieved`, `E3 achieved`, `E4 achieved`,
`market emergence proven`, `market mechanism shipped`, or ship authorization.

## Audit Question

Can this branch proceed as the REAL-17 integration baseline on updated main/CAS
repair, with old REAL-14/REAL-15/REAL-16/market-emergence packets treated as
historical context only and forward claim-bearing evidence required to be
regenerated on this branch?

## Clean-Context Verdict

```text
PROCEED
```

The first clean-context review returned `CHALLENGE` because several imported
historical packet/audit/report markdown files lacked an explicit REAL-17
main-CAS historical-context banner.

The fix added a uniform `REAL-17 main-CAS integration note` to all matching
REAL14F/REAL14G/REAL14H/REAL15/REAL16/MARKET_EMERGENCE markdown documents under
`handover/directives/market_autonomy_lab`.

The reviewer then verified the fix and returned:

```text
PROCEED
```

## Verification Evidence

Fresh commands run on this branch:

```text
cargo fmt --all -- --check
exit 0

git diff --check --cached
exit 0

git diff --check
exit 0

cargo test --workspace --no-fail-fast -- --test-threads=1
exit 0

bash scripts/run_constitution_gates.sh
exit 0
Totals: 461 passed, 0 failed, 1 ignored

CARGO_TARGET_DIR=target cargo test --manifest-path experiments/minif2f_v4/Cargo.toml --no-fail-fast -- --test-threads=1
exit 0

rg --files -g '*.json' handover/directives/market_autonomy_lab | xargs -r jq empty
exit 0
```

The ordinary MiniF2F `--manifest-path` run without `CARGO_TARGET_DIR=target`
hit only `tb_16_comprehensive_arena_smoke` binary-path failures because updated
main excludes MiniF2F from the root workspace while that smoke test expects the
binary in root `target/debug`. Re-running with root `CARGO_TARGET_DIR=target`
passed.

Historical-context coverage check:

```text
for f in $(rg --files handover/directives/market_autonomy_lab | rg '/(REAL14F|REAL14G|REAL14H|REAL15|REAL16|MARKET_EMERGENCE).*\.md$'); do
  if ! rg -q "REAL-17 main-CAS integration note" "$f"; then
    echo "$f"
  fi
done

exit 0, no output
```

Restricted-surface check:

```text
git diff --cached --name-only | rg '^(constitution\.md|handover/alignment/TRACE_FLOWCHART_MATRIX\.md|src/state/typed_tx\.rs|src/state/sequencer\.rs|src/bottom_white/cas/schema\.rs|src/kernel\.rs|src/bus\.rs|src/sdk/tools/wallet\.rs)$'

exit 1, no output
```

## Claim Boundary

Allowed forward status:

```text
REAL-17 may proceed on the updated main/CAS repair baseline.
```

Not allowed from this audit:

```text
E2 achieved
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
ship evidence
```

Forward REAL-17 claim-bearing evidence must be regenerated on the updated CAS
Git commit-chain baseline before it can support any stronger market-emergence
taxonomy label.
