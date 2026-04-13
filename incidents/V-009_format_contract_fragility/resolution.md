# V-009: Resolution

## Fixes (3 layers in protocol.rs)
1. **Prefix tolerance**: `find('{')` skips non-JSON prefix within `<action>` tags
2. **LaTeX escape fix**: `fix_json_escapes()` doubles backslashes not followed by valid JSON escape chars
3. **Bare tag fallback**: when `<action>` contains no JSON, preceding text becomes tactic

## Enforcement
- New rule R-013 (format_contract_test) — WARN on protocol.rs changes
- All three fixes are in `src/sdk/protocol.rs`

## Verification
- 3 agents × qwen3-8b × DashScope: 59 appends in 2 min, zero parse failures
- Previously: 100% parse failure rate

## Principle Established
**LLM output parsers must follow the Postel's Law: "Be conservative in what you send, be liberal in what you accept."** For LLM output specifically:
1. Never assume exact format — always have fallbacks
2. Never fail silently — log when falling through to tolerance mode
3. Test across model sizes and serving backends before deployment
4. LaTeX math + JSON = always need escape handling
