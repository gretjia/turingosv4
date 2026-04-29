# OBS: STEP_B restricted-file list path drift (2026-04-29)

**Discovered during**: CO1.7.5 spec smoke verification (S5 check).
**Severity**: hygiene — no behavior impact; audit-trail / reader-clarity only.

## Finding

CLAUDE.md "Code Standard" line 14 + STEP_B_PROTOCOL.md line 3 both listed the STEP_B-restricted set as `src/{kernel,bus,wallet}.rs`, but `src/wallet.rs` does not exist at HEAD `2f5093a`. Wallet was relocated to `src/sdk/tools/wallet.rs` at some prior commit (not bisected; not on critical path).

## Verification that wallet is still institutional (deserves STEP_B)

`src/sdk/tools/wallet.rs` lines 1-30 self-document as:
- "Tier 2: WalletTool — balance + YES/NO/LP portfolios"
- "Constitutional basis: Law 2 (only investment costs money, CTF conservation)"
- "Law 2 invariants: GENESIS (on_init) is the ONLY legal coin injection"

This file IS the institutional wallet that the original CLAUDE.md "Code Standard" line was designed to protect. The drift is purely a path-name change, not a constitutional-role change.

## Fix applied (this commit)

- `CLAUDE.md` line 14: `src/{kernel,bus,wallet}.rs` → `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs`
- `handover/ai-direct/STEP_B_PROTOCOL.md` line 3: `kernel.rs, bus.rs, wallet.rs` → `src/kernel.rs, src/bus.rs, src/sdk/tools/wallet.rs`

Both edits are path-corrections only; STEP_B-restricted set is semantically unchanged (same 3 files, just one with corrected path).

## Why fix now (not post-PASS/PASS)

CO1.7.5 spec § 7 smoke S5 surfaced this. Sending a spec to dual external audit while the referenced restricted-file list contradicts repo state is a guaranteed CHALLENGE — a wasted round. Per memory `feedback_smoke_before_batch`, smoke discrepancies block audit launch. Fixing the docs at smoke time is the cheapest path; deferring would leak the inconsistency into the audit prompt.

## Out of scope

- Bisecting which commit moved `src/wallet.rs` → `src/sdk/tools/wallet.rs`. Not on critical path; the relocation predates this session. `git log --follow --diff-filter=D src/wallet.rs` would surface it if needed later.
- Constitutional flowchart re-mapping. The wallet's FC role is unchanged (still bottom-white tool serving Law 2); only its filesystem path moved.
- Memory updates. `feedback_step_b_protocol` memory is path-agnostic; references "restricted files" semantically, no edit needed.

**FC-trace**: this OBS is constitutional-hygiene observation, not a behavior change. It does not require a flowchart node mapping per se, but the wallet itself remains FC3 (readonly subgraph) institutional state.
