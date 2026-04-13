# V-008: Proxy Single-Thread 502s

## Timeline
1. llm_proxy.py deployed with `BaseHTTPRequestHandler` (Python stdlib)
2. Single curl test: works perfectly
3. Evaluator spawns 15 agents, all hit proxy simultaneously
4. 13 of 15 agents get HTTP 502 Bad Gateway
5. Proxy log shows only 2 requests processed (sequential)

## Root Cause
Python's `http.server.HTTPServer` uses `BaseHTTPRequestHandler` which is single-threaded by default. When 15 requests arrive simultaneously, the server can only process one at a time. Pending connections are rejected or time out, returning 502 to the client.

## Resolution
Changed `HTTPServer` to `ThreadedHTTPServer(ThreadingMixIn, HTTPServer)` with `daemon_threads = True`. Each request now gets its own thread, supporting 15+ concurrent agent calls.

Commit: 1e76974
