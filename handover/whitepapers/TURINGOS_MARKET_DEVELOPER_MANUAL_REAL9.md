# TuringOS Market Developer Manual

This REAL-9 manual summarizes the current lawful-market development boundary.

Required framing:

```text
v4 does not copy v3.
v4 rebuilds v3's economic pressure under constitution.
price = signal, not truth.
market = role-specific institution, not prompt decoration.
```

Operating rules:

```text
no forced trades
no price-as-truth
no ghost liquidity
no f64 economy
no off-tape WAL as truth
no private CoT recording
no raw-log broadcast
```

Market work must be ChainTape/CAS-backed. Dashboards and reports are materialized views,
not sources of truth. Trader, Verifier, Challenger, Solver,
MarketMaker, ArchitectAI, VetoAI, and Observer behavior must pass through
role-scoped views and typed output gates before any L4/L4.E effect is claimed.

REAL-8 benchmark arms:

```text
A: market disabled
B: market visible, no TaskOutcomeMarket
C: TaskOutcomeMarket enabled
D: TaskOutcomeMarket + scripted AttemptPrediction fixture
```

Benchmark comparisons must use the same problem set, same model assignment, and
same budgets across arms. The allowed conclusion format is descriptive:
chain-backed observations, audit outcomes, and negative results. Do not claim
causality or spontaneous emergence unless a later explicitly ratified benchmark
protocol supports that claim.
