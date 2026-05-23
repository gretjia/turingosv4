#!/usr/bin/env python3
"""Mock LLM provider for TB-STRESS-PHASE-2.

Implements the SiliconFlow / OpenAI chat-completion API surface enough for
chat_client.rs to consume. Configurable via env:
  MOCK_FAIL_RATE     — 0.0..1.0 probability of returning 5xx
  MOCK_LATENCY_MS    — base latency added to every response
  MOCK_RESPONSE_BODY — canned response body (default: a small valid spec)
  MOCK_MAX_TOKENS    — usage.total_tokens to report
"""
from __future__ import annotations

import json
import os
import random
import sys
import time
from http.server import BaseHTTPRequestHandler, HTTPServer

FAIL_RATE = float(os.environ.get("MOCK_FAIL_RATE", "0.0"))
LATENCY_MS = int(os.environ.get("MOCK_LATENCY_MS", "30"))
RESPONSE_BODY = os.environ.get("MOCK_RESPONSE_BODY",
    "OK done. \\n\\n```spec\\nfeature: stress-mock\\n```")
MAX_TOKENS = int(os.environ.get("MOCK_MAX_TOKENS", "150"))
SEED = int(os.environ.get("MOCK_SEED", "0"))

rng = random.Random(SEED)


class MockHandler(BaseHTTPRequestHandler):
    def _set_json(self, code: int) -> None:
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.end_headers()

    def log_message(self, fmt: str, *args) -> None:  # noqa: D401
        # Silent — server output would drown the runner log
        return

    def do_POST(self) -> None:  # noqa: N802
        time.sleep(LATENCY_MS / 1000.0)
        if rng.random() < FAIL_RATE:
            self._set_json(503)
            self.wfile.write(json.dumps({"error": "mock 5xx for stress"}).encode())
            return
        length = int(self.headers.get("Content-Length", "0"))
        _ = self.rfile.read(length)  # discard request body
        body = {
            "id": "mock-" + format(rng.getrandbits(64), "016x"),
            "object": "chat.completion",
            "created": int(time.time()),
            "model": "mock-stress-v1",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": RESPONSE_BODY},
                "finish_reason": "stop",
            }],
            "usage": {
                "prompt_tokens": MAX_TOKENS // 2,
                "completion_tokens": MAX_TOKENS // 2,
                "total_tokens": MAX_TOKENS,
            },
        }
        self._set_json(200)
        self.wfile.write(json.dumps(body).encode())


def main() -> int:
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    addr = ("127.0.0.1", port)
    srv = HTTPServer(addr, MockHandler)
    actual_port = srv.server_address[1]
    # Print actual port to stdout (caller reads first line)
    print(f"{actual_port}", flush=True)
    try:
        srv.serve_forever()
    except KeyboardInterrupt:
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main())
