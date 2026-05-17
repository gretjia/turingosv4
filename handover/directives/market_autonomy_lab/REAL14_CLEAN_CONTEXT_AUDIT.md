# REAL-14 Clean-Context Codex Audit

auditor: GPT-5.5 high clean-context Codex

audit question:

```text
Can R16 be labeled exactly `E2 candidate pending audit` under REAL-14?
```

forbidden upgrade:

```text
Do not decide or claim `E2 achieved`.
```

## Findings

No production defects were found.

- The verifier derives L4 `BuyWithCoinRouterTx` ids from the ledger plus CAS
  payload decode, not dashboard text.
- Submitted `MarketDecisionTrace` ids are independently scanned from CAS.
- R16 evidence supports the narrow label: 8 exact joins, zero duplicate
  L4/submitted tx ids, zero scripted fixture txs, and
  `policy_counts_for_e2=false`.
- BCAST shielding is independently scanned across digests, role crops, and
  visible contexts. R16 reports 278/278/278 and PASS.
- R17/R18 are clean-negative and are not mislabeled as candidates.

## Test-Scaffold / Reporting Gaps

Non-blocking hardening gap:

```text
Tests cover exact join, zero-join labeling, missing provenance, duplicate L4 tx
ids, unknown schemas, and dashboard separation, but do not yet include an
explicit mismatch-fixture test for a joined tx_id with disagreeing
buyer/event/direction/amount.
```

The auditor treated this as non-blocking because current R16 evidence shows
the matched fields aligned, and audit tape economic assertions pass.

## Residual Risks

- PromptCapsule linkage is indirect via EVDecisionTrace and is correctly
  exposed as residual risk, not hidden as direct linkage.
- R16 is single-run and YES-side only.
- R17/R18 did not replicate the candidate.
- This remains research-envelope evidence only.

## Verification Rerun By Auditor

```text
R16 verifier --expect-count 8: PASS
R17 verifier --expect-count 0: PASS
R18 verifier --expect-count 0: PASS
manifest sha256 checks: PASS
cargo test --test constitution_real14_e2_candidate_verifier -- --test-threads=1: PASS, 6/6
```

## Verdict

```text
PROCEED
```

R16 may be labeled:

```text
E2 candidate pending audit
```

This is not:

```text
E2 achieved
```
