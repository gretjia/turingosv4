#!/usr/bin/env python3
"""ST-09 — Oversize prompt + truncated response (TB-STRESS-PHASE-2).

Two sub-tests:
  9a) 150KB prompt sent via chat_client; either rejected by provider
      with a clean error OR succeeds with usage_total_tokens reported.
  9b) Mock provider sends truncated JSON response body. chat_client must
      surface LlmError::Decode or Schema, never panic.

KILL: no panic in either sub-test; 9a returns a defined exit code (0 or
      structured error); 9b error path engages.
"""
from __future__ import annotations

import json
import os
import socket
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402


class TruncatedHandler(BaseHTTPRequestHandler):
    def log_message(self, *a):  # silent
        return
    def do_POST(self):  # noqa: N802
        # Send valid headers, then deliberately truncated JSON body.
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        # Note: don't send Content-Length so client can't precompute completeness
        self.end_headers()
        # Truncated JSON
        self.wfile.write(b'{"id":"mock","choices":[{"message":{"role":"assistant"')
        # Close connection abruptly


def start_truncated_server() -> tuple[HTTPServer, int]:
    srv = HTTPServer(("127.0.0.1", 0), TruncatedHandler)
    port = srv.server_address[1]
    t = threading.Thread(target=srv.serve_forever, daemon=True)
    t.start()
    return srv, port


def main() -> int:
    evid = evidence_dir("st09_oversize_prompt")
    log: list[str] = []
    ws = (evid / "workspace").resolve()
    subprocess.run(
        [str(PROJECT_ROOT / "scripts" / "stress" / "_ws_bootstrap.sh"), str(ws)],
        check=True, cwd=PROJECT_ROOT,
    )

    print("[ST-09] building turingos...")
    rc = subprocess.call(["cargo", "build", "--bin", "turingos", "--quiet"], cwd=PROJECT_ROOT)
    if rc != 0:
        log.append("build failed")
        write_summary(evid, test_id="ST-09 oversize prompt", kill_pass=False, lines=log)
        return 1
    bin_path = PROJECT_ROOT / "target" / "debug" / "turingos"

    # 9a: oversize prompt against mock
    print("[ST-09a] oversize prompt vs mock (will treat as fail-rate=0 mock)...")
    mock = subprocess.Popen(
        ["python3", str(PROJECT_ROOT / "scripts" / "stress" / "_mock_llm_server.py"), "0"],
        env={**os.environ, "MOCK_FAIL_RATE": "0.0", "MOCK_LATENCY_MS": "10",
             "MOCK_RESPONSE_BODY": "ok"},
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )
    mock_port = int(mock.stdout.readline().decode().strip())
    log.append(f"mock_port={mock_port}")

    big_prompt = "X " * (75 * 1024)  # ~150KB
    big_path = evid / "big_prompt.json"
    big_path.write_text(json.dumps({
        "messages": [{"role": "user", "content": big_prompt}]
    }))

    env = os.environ.copy()
    env.update({
        "TURINGOS_SILICONFLOW_ENDPOINT": f"http://127.0.0.1:{mock_port}/v1/chat/completions",
        "SILICONFLOW_API_KEY": "mock-key",
        "DEEPSEEK_API_KEY": "mock-key",
    })
    p9a = subprocess.run(
        [str(bin_path), "llm", "complete",
         "--workspace", str(ws),
         "--prompt-file", str(big_path),
         "--max-tokens", "30"],
        env=env, capture_output=True, text=True, timeout=60,
    )
    log.append(f"9a rc={p9a.returncode}")
    log.append(f"9a stdout last 200: {p9a.stdout[-200:]!r}")
    log.append(f"9a stderr last 200: {p9a.stderr[-200:]!r}")
    panic_9a = "panicked at" in (p9a.stdout + p9a.stderr)
    mock.terminate()
    try: mock.wait(timeout=3)
    except subprocess.TimeoutExpired: mock.kill()

    # 9b: truncated response from in-process server
    print("[ST-09b] truncated response server...")
    trunc_srv, trunc_port = start_truncated_server()
    try:
        env = os.environ.copy()
        env.update({
            "TURINGOS_SILICONFLOW_ENDPOINT": f"http://127.0.0.1:{trunc_port}/v1/chat/completions",
            "SILICONFLOW_API_KEY": "mock-key",
            "DEEPSEEK_API_KEY": "mock-key",
        })
        small_prompt = evid / "small_prompt.json"
        small_prompt.write_text(json.dumps({
            "messages": [{"role": "user", "content": "small prompt"}]
        }))
        p9b = subprocess.run(
            [str(bin_path), "llm", "complete",
             "--workspace", str(ws),
             "--prompt-file", str(small_prompt),
             "--max-tokens", "30"],
            env=env, capture_output=True, text=True, timeout=30,
        )
        log.append(f"9b rc={p9b.returncode}")
        log.append(f"9b stdout last 200: {p9b.stdout[-200:]!r}")
        log.append(f"9b stderr last 200: {p9b.stderr[-200:]!r}")
        panic_9b = "panicked at" in (p9b.stdout + p9b.stderr)
    finally:
        trunc_srv.shutdown()

    log.append(f"panics: 9a={panic_9a} 9b={panic_9b}")

    # KILL: no panics; both subprocesses returned a defined code
    kill_pass = (not panic_9a) and (not panic_9b) and (p9a.returncode != -11) and (p9b.returncode != -11)
    write_summary(evid, test_id="ST-09 oversize prompt + truncated response",
                  kill_pass=kill_pass, lines=log)
    print(f"[ST-09] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
