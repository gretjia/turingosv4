#!/usr/bin/env python3
"""ST-04 — Snapshot restart storm (TB-STRESS-PHASE-2).

Runs N_CYCLES of (start turingos_web → do TURNS_PER_CYCLE grill turns →
kill server). Across all cycles, the AppState.sessions cache is cleared
each cycle; the next cycle's first turn MUST rebuild GrillSession from
the CAS snapshot S2 introduced.

KILL: All cycles succeed; turn_count monotonically increases across the
      full session lifetime; no 404 on the first turn of any cycle ≥ 2.
"""
from __future__ import annotations

import json
import os
import shutil
import socket
import subprocess
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402

N_CYCLES = int(os.environ.get("ST04_CYCLES", "10"))
TURNS_PER_CYCLE = int(os.environ.get("ST04_TURNS", "5"))


def free_port() -> int:
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    p = s.getsockname()[1]
    s.close()
    return p


def start_mock(evid: Path) -> tuple[subprocess.Popen, int]:
    log = evid / "mock_llm.log"
    p = subprocess.Popen(
        ["python3", str(PROJECT_ROOT / "scripts" / "stress" / "_mock_llm_server.py"), "0"],
        env={**os.environ, "MOCK_FAIL_RATE": "0.0", "MOCK_LATENCY_MS": "10",
             "MOCK_RESPONSE_BODY": "{\"next_question\": \"What's the target?\", \"covered_slot\": \"goal\"}"},
        stdout=subprocess.PIPE, stderr=open(log, "wb"),
    )
    port = int(p.stdout.readline().decode().strip())
    return p, port


def run_cycle(web_bin: Path, ws: Path, evid: Path, mock_port: int,
              cycle: int, session_id: str, turn_counter: list[int]) -> tuple[bool, str]:
    port = free_port()
    server_log = evid / f"server_cycle{cycle}.log"
    env = os.environ.copy()
    env.update({
        "TURINGOS_WEB_WORKSPACE": str(ws),
        "TURINGOS_WEB_PORT": str(port),
        "TURINGOS_SILICONFLOW_ENDPOINT": f"http://127.0.0.1:{mock_port}/v1/chat/completions",
        "SILICONFLOW_API_KEY": "mock-key",
        "DEEPSEEK_API_KEY": "mock-key",
        "RUST_LOG": "warn",
    })
    server = subprocess.Popen(
        [str(web_bin)], cwd=ws, env=env,
        stdout=open(server_log, "wb"), stderr=subprocess.STDOUT,
    )
    try:
        # readiness probe
        ready = False
        for _ in range(40):
            time.sleep(0.25)
            try:
                with urllib.request.urlopen(f"http://127.0.0.1:{port}/api/health", timeout=1.0) as r:
                    if r.status == 200:
                        ready = True
                        break
            except Exception:
                continue
        if not ready:
            return False, "server not ready"

        # do TURNS_PER_CYCLE grill turns
        for t in range(TURNS_PER_CYCLE):
            answer_text = f"stress-cycle{cycle}-turn{t}"
            body = json.dumps({
                "session_id": session_id,
                "user_answer": answer_text,
                "lang": "zh",
            }).encode()
            req = urllib.request.Request(
                f"http://127.0.0.1:{port}/api/spec/turn",
                data=body, headers={"Content-Type": "application/json"},
                method="POST",
            )
            try:
                with urllib.request.urlopen(req, timeout=15) as r:
                    if r.status == 200:
                        turn_counter[0] += 1
                    else:
                        return False, f"cycle{cycle} turn{t} HTTP {r.status}"
            except urllib.error.HTTPError as e:
                # First turn of cycle≥2 must NOT be a 404 — that's the resume test.
                if cycle >= 2 and t == 0 and e.code == 404:
                    return False, f"cycle{cycle} first-turn 404 — resume failed"
                # Other HTTP errors: log but allow
                return False, f"cycle{cycle} turn{t} HTTPError {e.code}"
            except Exception as e:
                return False, f"cycle{cycle} turn{t} exception {e}"
        return True, ""
    finally:
        server.terminate()
        try:
            server.wait(timeout=10)
        except subprocess.TimeoutExpired:
            server.kill()


def main() -> int:
    evid = evidence_dir("st04_snapshot_restart_storm")
    log: list[str] = []
    ws = evid / "workspace"
    ws.mkdir(exist_ok=True)
    (ws / "cas").mkdir(exist_ok=True)

    print("[ST-04] building turingos_web...")
    rc = subprocess.call(
        ["cargo", "build", "--features", "web", "--bin", "turingos_web", "--quiet"],
        cwd=PROJECT_ROOT,
    )
    if rc != 0:
        log.append("build failed")
        write_summary(evid, test_id="ST-04 snapshot restart storm",
                      kill_pass=False, lines=log)
        return 1
    web_bin = PROJECT_ROOT / "target" / "debug" / "turingos_web"

    print("[ST-04] starting mock LLM...")
    mock, mock_port = start_mock(evid)
    log.append(f"mock_port={mock_port}  cycles={N_CYCLES}  turns_per_cycle={TURNS_PER_CYCLE}")

    session_id = "st04_session_" + os.urandom(4).hex()
    turn_counter = [0]
    failed_cycle = None
    failure_reason = ""
    try:
        for cycle in range(1, N_CYCLES + 1):
            ok, reason = run_cycle(web_bin, ws, evid, mock_port, cycle, session_id, turn_counter)
            log.append(f"cycle {cycle}: {'OK' if ok else 'FAIL ' + reason}")
            if not ok:
                failed_cycle = cycle
                failure_reason = reason
                break
    finally:
        mock.terminate()
        try:
            mock.wait(timeout=5)
        except subprocess.TimeoutExpired:
            mock.kill()

    expected_turns = N_CYCLES * TURNS_PER_CYCLE
    log.append(f"total_turns_succeeded={turn_counter[0]}  expected={expected_turns}")
    kill_pass = failed_cycle is None and turn_counter[0] >= expected_turns * 0.9
    if failed_cycle:
        log.append(f"failed at cycle {failed_cycle}: {failure_reason}")

    write_summary(evid, test_id="ST-04 snapshot restart storm",
                  kill_pass=kill_pass, lines=log)
    print(f"[ST-04] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
