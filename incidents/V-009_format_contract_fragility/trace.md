# V-009: Format Contract Fragility — Silent Parse Failure

## Timeline
1. Proxy V-007 fix delivered LLM responses correctly (content verified in proxy log)
2. Evaluator showed "30s idle timeout" — agents appeared to do nothing
3. Root cause: `parse_agent_output()` returned `None` for every response
4. Three distinct format violations found, each requiring its own fix:

### Layer 1: JSON prefix in action tags
- Expected: `<action>{"tool":"append",...}</action>`
- Actual: `<action>append: {"tool":"append",...}</action>`
- The model prepends the tool name before JSON
- Fix: `find('{')` to skip prefix text (commit db12049)

### Layer 2: LaTeX backslash escapes
- Expected: valid JSON string escapes (`\n`, `\t`, etc.)
- Actual: `\cdot`, `\cos`, `\to`, `\infty` — LaTeX math notation
- These are INVALID JSON escape sequences → serde_json rejects entire string
- Fix: `fix_json_escapes()` doubles lone backslashes (commit 1f18458)

### Layer 3: Bare action tags
- Expected: `<action>{"tool":"append","tactic":"..."}</action>`
- Actual: `<action>append</action>` — bare tool name, no JSON at all
- Math reasoning written as free text before the tag
- Fix: use preceding text as tactic when tag contains no JSON (commit 0ffd94c)

## Critical Failure Mode
ALL THREE layers failed SILENTLY. No error logged. `parse_agent_output` simply returned `None`, the `if let Some(action)` branch was skipped, and the agent loop continued with heartbeat update only. From the evaluator's perspective, agents were "alive but idle" — the most deceptive failure mode possible.

## Impact
- First deployment: 15 agents ran for 90+ seconds doing nothing
- Every API call cost real money (DashScope billing) with zero useful work
- Without debug logging, the root cause was invisible
