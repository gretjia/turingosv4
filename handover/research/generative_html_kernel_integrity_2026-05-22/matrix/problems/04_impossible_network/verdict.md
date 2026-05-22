# P04 Verdict — Impossible Network Request (real-time stock prices)

**Expected**: L4.E rejection at verify stage (network-dependent artifact fails sandbox check)  
**Actual**: Spec absorbed network requirement; verifier has no network detection; generate 401 blocks

**Result**: PARTIAL (verifier gap confirmed; L4.E would not trigger for network-dependent HTML)

**Key findings**:
1. Synthesis LLM accepted "实时查股票价格" as legitimate spec requirement
2. Spec calls for `fetch()` to external stock API
3. MinimumBar verifier does NOT detect fetch()/XHR in artifact HTML
4. Network-dependent artifact would PASS static verification
5. At runtime: fetch() would fail due to CORS/sandbox restrictions (silently)
6. No L4.E rejection for this class of failure — user would see broken stock page

**Kernel surfaces exercised**: FC1 generate/verify (verifier gap confirmed)
