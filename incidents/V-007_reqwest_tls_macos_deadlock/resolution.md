# V-007: Resolution

## Fix
- Created `src/drivers/llm_proxy.py` — local HTTP proxy using Python OpenAI SDK
- Rewrote `src/drivers/llm_http.rs` — pure reqwest HTTP, no TLS/subprocess logic
- Added `proxy` provider + `DEEPSEEK_URL` override to evaluator

## Verification
- Mac: 3 agents × qwen3-8b via DashScope → 59 successful appends in 2 minutes
- Zero hangs, zero timeouts, zero 502s (after ThreadingMixIn fix)

## Enforcement
- llm_proxy.py IS the enforcement — all cloud API traffic now goes through it
- No rule needed — the old code path (direct HTTPS) has been physically removed
