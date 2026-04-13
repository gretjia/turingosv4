# V-007: reqwest+rustls macOS HTTPS Deadlock

## Timeline
1. evaluator uses reqwest+rustls to call DashScope API (HTTPS)
2. On macOS, requests hang indefinitely — no timeout, no error
3. Same code works on Linux (omega-vm, linux1)

## Failed Approaches (6 attempts, all failed)
1. `http1_only()` — still hangs
2. `no_proxy()` — VPN was NOT the issue (user confirmed: "不需要关闭vpn吧，我又没开tun模式")
3. `native-tls` feature — different TLS backend, still hangs
4. `curl` subprocess via `std::process::Command` — 64KB pipe buffer deadlock on macOS when output exceeds buffer
5. `tokio::process::Command` with `wait_with_output()` — drains pipes but tokio reactor starvation when blocking calls occupy all worker threads
6. `spawn_blocking` + `try_wait` polling — works intermittently, unreliable

## Root Cause
macOS has a 64KB pipe buffer limit. When subprocess stdout exceeds this, the child blocks on write, parent blocks on wait → mutual deadlock. Linux has 1MB+ buffers so the same code works there.

Additionally, reqwest+rustls may have TLS negotiation issues with Chinese cloud endpoints (DashScope/Aliyun) that don't manifest on Linux.

## Resolution
Python OpenAI SDK local HTTP proxy (llm_proxy.py on 127.0.0.1:8088). Evaluator speaks plain HTTP to localhost, proxy handles HTTPS to cloud. Industrial standard pattern (Ollama, LocalAI, vLLM all use this).

Commits: 4916aee, 1e76974
