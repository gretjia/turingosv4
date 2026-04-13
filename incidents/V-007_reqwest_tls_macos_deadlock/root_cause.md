# V-007: Root Cause Analysis

## WHY Chain
1. WHY did evaluator hang? → reqwest+rustls HTTP call never returns
2. WHY doesn't reqwest return? → TLS handshake or data transfer stalls on macOS + Chinese HTTPS
3. WHY only macOS? → macOS 64KB pipe buffer causes deadlock when subprocess workaround attempted
4. WHY did 6 approaches fail? → Each approach worked on Linux but hit macOS-specific limits
5. WHY wasn't this caught earlier? → All development/testing on Linux (omega-vm), macOS deployment was new

## Structural Cause
The evaluator had tight coupling between LLM call mechanism and HTTP/TLS stack. There was no abstraction boundary between "call LLM" and "manage network transport". This made it impossible to swap transport without rewriting the driver.

## Prevention
The HTTP proxy architecture creates a clean boundary: evaluator ↔ localhost HTTP ↔ proxy ↔ cloud HTTPS. This decouples the Rust code from all TLS/network stack issues permanently.
