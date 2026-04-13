# V-009: Root Cause Analysis

## WHY Chain (3 branches)

### Branch 1: JSON Prefix
1. WHY did LLM add "append: " prefix? → Qwen3-8B interpreted the format instruction loosely
2. WHY didn't parser handle it? → Parser assumed clean JSON between `<action>` and `</action>`
3. WHY that assumption? → Parser was designed for Qwen3.5-9B on llama.cpp which outputs cleaner format
4. WHY different format? → Different serving infrastructure (llama.cpp vs DashScope API) + different tokenization

### Branch 2: LaTeX Escapes
1. WHY did LaTeX break JSON? → `\cdot`, `\cos` are invalid JSON escapes (`\c` is not a valid escape char)
2. WHY LaTeX in JSON? → LLM writes math formulas naturally, doesn't know JSON escape rules
3. WHY wasn't this anticipated? → Prior models on llama.cpp used simpler math notation
4. WHY is this fundamental? → ANY LLM doing math WILL output LaTeX. JSON + LaTeX is inherently incompatible without escaping.

### Branch 3: Bare Tags
1. WHY bare tags? → 8B model sometimes "forgets" the JSON format mid-generation
2. WHY no fallback? → Parser had two modes: JSON action OR legacy [Tactic:] format. No middle ground.
3. WHY binary? → Parser designed for specific model behavior, not for graceful degradation

## Structural Root Cause
**The parser was a BRITTLE CONTRACT — it assumed a specific output format and failed silently when the format deviated even slightly.** This is the exact opposite of the Bitter Lesson: hardcoding expected LLM behavior instead of building robust, tolerant parsing.

The format contract between LLM and evaluator was never tested across:
- Different model sizes (8B vs 9B vs 32B)
- Different serving backends (llama.cpp vs cloud API)
- Different content types (simple text vs LaTeX math)

## Meta-Lesson
LLM output is a PROBABILISTIC signal, not a deterministic API contract. Parsers for LLM output must be MAXIMALLY TOLERANT — they should extract intent from messy, approximate output rather than demanding exact format compliance.
