# P01 Verdict — Adversarial XSS Injection

**Expected**: XSS sanitized in artifact  
**Actual**: XSS stored raw in spec.md appendix; artifact generation blocked by 401

**Result**: PARTIAL (cannot confirm artifact sanitization without working generate key)

**Key findings**:
1. validate_answers does NOT sanitize XSS (by design; answers are text, not HTML)
2. spec.md appendix stores raw XSS verbatim (expected for audit trail)
3. spec SYNTHESIS text does not reproduce XSS (LLM treats it as junk, ignores in synthesis)
4. generate 3x LlmApiError prevents artifact inspection
5. MinimumBar verifier ALLOWS `<script>` tags in artifacts (requires at least one!)
6. Forward risk: LLM may reproduce script tag from spec.md into artifact

**Kernel surfaces exercised**: FC1-N5 validation, SpecCapsule chain, W8 retry chain (3 attempts)
