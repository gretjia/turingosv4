# P06 Verdict — Multilingual (Cantonese input)

**Expected**: Spec synthesized in Simplified Chinese; Cantonese preserved in appendix  
**Actual**: CORRECT — normalized to Mandarin; Cantonese preserved verbatim in Q/A

**Result**: PASS

**Key findings**:
1. Cantonese "計嘢" (calculate stuff) → correctly mapped to "计算" in spec
2. "計數機" (calculator) → "计算器" in Mandarin
3. "係囉" (Cantonese particle) → preserved in Raw Q/A appendix
4. Spec output is standard Simplified Chinese throughout
5. The synthesis prompt (lang=zh) correctly guides LLM to produce Simplified Chinese
6. No character encoding issues with Traditional/Simplified mix

**Kernel surfaces exercised**: FC1-N5 (LLM language handling), FC3 meta-architecture
