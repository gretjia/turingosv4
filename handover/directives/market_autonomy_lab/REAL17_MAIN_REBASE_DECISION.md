# REAL-17 Main Rebase Decision

Date: 2026-05-17

## Decision

Proceed from the updated `origin/main` CAS Git repair baseline.

New worktree:

```text
/home/zephryj/projects/turingosv4-real17-main-cas
branch: codex/real17-emergence-hardening-20260517
base: origin/main @ 88fa1f6694ca6b423d9e7df54e4872c1a47f6b1f
```

Do not continue REAL-17 claim-bearing work directly from the older
`codex/market-autonomy-lab-20260516` worktree.

Do not merge the older worktree directly into `main` first.

## Evidence

`origin/main` README states that main now includes the audited CAS Git
constitutional repair merge:

```text
CAS repair merge commit: 802b18053d063bd5503a6b0eb2e7b1f46ceda93b
CAS writes now advance refs/chaintape/cas as a commit chain.
The sidecar index is a rebuildable cache.
CAS open/reload uses the same CAS chain lock as put().
MiniF2F is excluded from root workspace gates and must be run explicitly.
```

The market-autonomy branch merge-base with updated main is:

```text
2dd4820082e6ebd853063807c05acd05d2e2440e
```

The older market branch is ahead with market-emergence candidate evidence, but
it predates the CAS Git commit-chain repair. Therefore its historical evidence
remains useful as candidate context, not as the forward CAS baseline for new
claim-bearing REAL-17 evidence.

Any imported REAL-14/REAL-15/REAL-16/market-emergence packet from the older
worktree is historical context unless REAL-17 regenerates matching evidence on
this main-based repaired-CAS branch. Missing old evidence directories are not a
VETO for integration, but they cannot support forward claims.

## Merge Strategy

1. Freeze the old market worktree evidence as historical context.
2. Integrate market-autonomy code/docs into the new main-based REAL-17 branch
   only through a controlled merge or selective cherry-pick.
3. Preserve all CAS Git repair files and main README semantics.
4. Resolve `genesis_payload.toml` by retaining the CAS repair Trust Root
   entries and rehashing only touched allowed pinned files.
5. Re-run Trust Root verification and constitution gates after integration.
6. Re-run REAL-17 evidence on the new CAS Git commit-chain baseline.

## Clean-Context Audit

The REAL-17 main-CAS integration audit is recorded at:

```text
handover/directives/market_autonomy_lab/REAL17_MAIN_CAS_INTEGRATION_AUDIT.md
```

Verdict:

```text
PROCEED
```

This verdict only authorizes continuing REAL-17 constitutional research on the
updated main/CAS repair baseline. It does not authorize E2/E3/E4 achieved,
market emergence proven, market mechanism shipped, or ship evidence claims.

## Claim Boundary

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

## Rationale

Continuing on the old worktree would make new evidence depend on a pre-repair
CAS sidecar model. Merging the old worktree into `main` first would mix a
large evidence-bearing branch into an updated Trust Root/CAS baseline before
REAL-17 hardening has rerun gates.

The safer constitutional path is a fresh main-based REAL-17 worktree, followed
by controlled integration, verification, and new evidence generation.
