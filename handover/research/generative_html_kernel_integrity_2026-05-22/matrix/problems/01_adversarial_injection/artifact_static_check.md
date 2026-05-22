# P01 Artifact Static Check — p01_xss_v2

No artifact produced (generate failed 3x with HTTP 401 LlmApiError).

## XSS in spec.md

XSS payload `<script>alert(document.cookie)</script>` found in spec.md at:
- `**A1**: <script>alert(document.cookie)</script>` (Raw Q/A appendix)

The spec.md appendix stores user answers VERBATIM without HTML escaping.
This is EXPECTED for spec.md (internal document, not browser-rendered).
The spec SYNTHESIS text (sections above appendix) does NOT contain the script tag —
the LLM synthesized a "简单笔记软件" spec without referencing the XSS payload content.

## Forward finding

If a future generate succeeds:
1. The LLM (deepseek-v4-pro for generate) receives spec.md as prompt input.
2. The spec.md appendix contains the raw `<script>` tag.
3. Risk: LLM may reproduce the script tag in generated HTML.
4. The MinimumBar verifier DOES NOT strip `<script>` tags (it requires at least one!).
5. Therefore: XSS in generated artifact is a REAL risk when generate path works.
6. Mitigation needed: artifact post-processing to remove `<script>alert(...)` patterns.
