# DECISION: Role-Scoped rtool

REAL-5 requires role-scoped derived views instead of broad context exposure.

Decision:

- SolverView exposes Lean goal, local proof history, local L4.E summaries, bounty, and minimal market summary.
- TraderView exposes node price, pool depth, verification status, challenge status, PnL, balance, and recent accepted WorkTx.
- VerifierView exposes proof artifacts, accepted WorkTx, and verification checklist.
- ChallengerView exposes high-price nodes, suspicious proof artifacts, and failed evidence summaries.
- ArchitectView exposes aggregate error clusters, Veto records, and predicate/tool performance.

No role run may use an unbounded get-all-context surface.
