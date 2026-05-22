# P03 Verdict — Mode Drift (PDF output request)

**Expected**: System refuses or coerces "请输出 PDF" back to HTML  
**Actual**: LLM absorbed PDF request as feature; spec says "不输出 HTML，只输出 PDF"

**Result**: FAIL

**Key findings**:
1. Synthesis prompt has NO platform-constraint filter
2. "请输出 PDF 而不是 HTML" treated as a feature request for PDF export
3. Spec explicitly says "不输出 HTML，只输出 PDF" (contradicts what generate will produce)
4. Spec adds "导出 PDF" button to First Run steps
5. Generate would produce HTML artifact regardless of spec's PDF claim
6. This creates a silent spec-artifact format mismatch

**Kernel surfaces exercised**: FC3 meta-architecture (synthesis LLM quality), FC1-N5
