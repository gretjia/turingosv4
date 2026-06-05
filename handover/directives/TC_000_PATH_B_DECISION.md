# TC-000 Path B Decision

Date: 2026-06-05
Status: active task packet
Class: 0
Parent ADR: `handover/architecture/ADR_2026-06-05_PATH_B_GITTAPE_AS_OS_SUBSTRATE.md`

## Goal

Bind all future tape-related atoms to Path B: GitTape/ChainTape is the sole
`Q_t` source of truth.

## Build

This packet is documentation-only. It authorizes no runtime edits by itself.
Future atoms must implement the Path B gates through small PRs from
`origin/main`.

## Transactions

No ChainTape transaction is emitted by this document. It is a workspace
directive, not Tier 2 fact.

## Predicates

Future Path B atoms must prove:

```text
accepted ref movement error is not swallowed
rejected branch is append-only and reconstructable
reopen append resumes at tn-N+1
every event has logical_t, run_id, parent, author, kind, payload_hash
no derived projection is stored as source of truth
git fsck --full passes on generated test repo
tampering any event or ref causes verify_integrity failure
```

## Exit Criteria

This packet is satisfied when:

```text
ADR exists and says Path B.
ADR says Vec<Node> legacy is compatibility-only.
ADR says GitTape/ChainTape is sole Q_t source of truth.
ADR says market/wallet/price/search/librarian/dashboard/report are derived.
Production PR template blocks direct ancestry from #280/#283.
```

## Forbidden

```text
No runtime code.
No tests.
No evidence rewrite.
No direct merge of #280 or #283.
No MarketTape or market_tape_shared as a root source of truth.
```
