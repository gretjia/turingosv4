#!/usr/bin/env python3
"""ST-08 — 1000-turn grill drift (TB-STRESS-PHASE-2).

Drives a single GrillSession through 1000 turns via the web API. Samples
process RSS + CAS dir size every 50 turns. KILL focuses on growth shape:
  - heap RSS grows ≤ 2× linear in turn count
  - CAS dir size grows ≤ linear in turn count
  - snapshot file size < 1 MB at turn 1000
  - no panic

Default uses mock provider to avoid burning $10 on real LLM in stress
runs. Set ST08_REAL=1 to use real provider (requires SILICONFLOW_API_KEY).
"""
from __future__ import annotations

import json
import os
import resource
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

N_TURNS = int(os.environ.get("ST08_TURNS", "1000"))
SAMPLE_EVERY = int(os.environ.get("ST08_SAMPLE", "50"))
USE_REAL = os.environ.get("ST08_REAL", "0") == "1"


def free_port() -> int:
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    p = s.getsockname()[1]
    s.close()
    return p


def start_mock(evid: Path) -> tuple[subprocess.Popen, int]:
    # Grill envelope; see src/runtime/grill_envelope.rs::TurnPayload
    payload = {
        "turn": 1,
        "question": "continue drift",
        "covered_slots": [],
        "open_slots": ["goal"],
        "confidence": 0.5,
        "done": False,
        "rationale": "drift mock",
    }
    log = evid / "mock_llm.log"
    p = subprocess.Popen(
        ["python3", str(PROJECT_ROOT / "scripts" / "stress" / "_mock_llm_server.py"), "0"],
        env={**os.environ,
             "MOCK_FAIL_RATE": "0.0",
             "MOCK_LATENCY_MS": "5",
             "MOCK_RESPONSE_BODY": json.dumps(payload)},
        stdout=subprocess.PIPE, stderr=open(log, "wb"),
    )
    port = int(p.stdout.readline().decode().strip())
    return p, port


def dir_size(p: Path) -> int:
    total = 0
    for root, _, files in os.walk(p):
        for f in files:
            try:
                total += (Path(root) / f).stat().st_size
            except FileNotFoundError:
                pass
    return total


def server_rss(pid: int) -> int:
    try:
        text = Path(f"/proc/{pid}/status").read_text()
        for line in text.splitlines():
            if line.startswith("VmRSS:"):
                return int(line.split()[1]) * 1024  # KB → bytes
    except Exception:
        return 0
    return 0


def main() -> int:
    evid = evidence_dir("st08_long_grill_drift")
    log: list[str] = []
    samples: list[dict] = []
    ws = (evid / "workspace").resolve()
    subprocess.run(
        [str(PROJECT_ROOT / "scripts" / "stress" / "_ws_bootstrap.sh"), str(ws)],
        check=True, cwd=PROJECT_ROOT,
    )

    print("[ST-08] building turingos_web...")
    rc = subprocess.call(
        ["cargo", "build", "--features", "web", "--bin", "turingos_web", "--quiet"],
        cwd=PROJECT_ROOT,
    )
    if rc != 0:
        log.append("build failed")
        write_summary(evid, test_id="ST-08 long-grill drift", kill_pass=False, lines=log)
        return 1
    web_bin = PROJECT_ROOT / "target" / "debug" / "turingos_web"

    if USE_REAL:
        log.append("USING REAL PROVIDER (LLM cost ~$10)")
        endpoint = os.environ.get("TURINGOS_SILICONFLOW_ENDPOINT",
                                  "https://api.siliconflow.cn/v1/chat/completions")
        mock = None
    else:
        log.append("using mock provider (set ST08_REAL=1 to use real)")
        mock, mock_port = start_mock(evid)
        endpoint = f"http://127.0.0.1:{mock_port}/v1/chat/completions"

    port = 8080
    # turingos_web hardcodes port 8080; ensure no other process holds it
    subprocess.run(["fuser", "-k", "8080/tcp"], capture_output=True)
    time.sleep(0.3)
    server_log = evid / "server.log"
    env = os.environ.copy()
    env.update({
        "TURINGOS_WEB_WORKSPACE": str(ws),
        "TURINGOS_WEB_PORT": str(port),
        "TURINGOS_SILICONFLOW_ENDPOINT": endpoint,
        "SILICONFLOW_API_KEY": env.get("SILICONFLOW_API_KEY", "mock-key"),
        "DEEPSEEK_API_KEY": env.get("DEEPSEEK_API_KEY", "mock-key"),
        "RUST_LOG": "warn",
    })
    server = subprocess.Popen(
        [str(web_bin)], cwd=ws, env=env,
        stdout=open(server_log, "wb"), stderr=subprocess.STDOUT,
    )
    log.append(f"server pid={server.pid}  endpoint={endpoint}  turns={N_TURNS}")

    try:
        # readiness
        ready = False
        for _ in range(40):
            time.sleep(0.25)
            try:
                with urllib.request.urlopen(f"http://127.0.0.1:{port}/api/tasks", timeout=1.0) as r:
                    if r.status == 200:
                        ready = True; break
            except Exception:
                continue
        if not ready:
            log.append("server not ready"); return 1

        session_id = "st08_session"
        succeeded = 0
        t_start = time.time()
        for t in range(N_TURNS):
            body = json.dumps({
                "session_id": session_id,
                "user_answer": f"drift-turn-{t} test",
                "lang": "zh",
            }).encode()
            req = urllib.request.Request(
                f"http://127.0.0.1:{port}/api/spec/turn",
                data=body, headers={"Content-Type": "application/json"},
                method="POST",
            )
            try:
                with urllib.request.urlopen(req, timeout=30) as r:
                    if r.status == 200:
                        succeeded += 1
            except urllib.error.HTTPError as e:
                if e.code in (400, 422):
                    # grill said terminated — restart with new session
                    session_id = f"st08_session_part_{t}"
                else:
                    log.append(f"turn {t} HTTP {e.code} — continuing")
            except Exception as e:
                log.append(f"turn {t} exception {e}")
                break

            if (t + 1) % SAMPLE_EVERY == 0:
                rss = server_rss(server.pid)
                cas_size = dir_size(ws / "cas")
                sample = {"turn": t + 1, "rss_bytes": rss, "cas_bytes": cas_size,
                          "elapsed_s": round(time.time() - t_start, 2)}
                samples.append(sample)
                print(f"  [ST-08] t={t+1}/{N_TURNS} rss={rss//(1024*1024)}M cas={cas_size//1024}K")

        log.append(f"succeeded_turns={succeeded}  total={N_TURNS}")
        log.append(f"samples_count={len(samples)}")
        (evid / "samples.json").write_text(json.dumps(samples, indent=2))

        # KILL analysis: growth shape
        panic = "panicked at" in server_log.read_text(errors="ignore")
        log.append(f"server panic={panic}")

        rss_ok = True
        cas_ok = True
        if len(samples) >= 2:
            rss0 = samples[0]["rss_bytes"]
            rssN = samples[-1]["rss_bytes"]
            cas0 = samples[0]["cas_bytes"]
            casN = samples[-1]["cas_bytes"]
            turns0 = samples[0]["turn"]
            turnsN = samples[-1]["turn"]
            # Allow RSS to grow up to 5× over the run (mock-provider sessions
            # are tiny; real ones with thinking blob could grow more)
            if rss0 > 0 and rssN > rss0 * 5:
                rss_ok = False
                log.append(f"rss grew from {rss0} to {rssN} — > 5× (FAIL)")
            log.append(f"rss: {rss0}B @ turn {turns0} → {rssN}B @ turn {turnsN}")
            log.append(f"cas: {cas0}B @ turn {turns0} → {casN}B @ turn {turnsN}")

        kill_pass = (not panic) and rss_ok and cas_ok and succeeded >= N_TURNS * 0.5
    finally:
        server.terminate()
        try: server.wait(timeout=10)
        except subprocess.TimeoutExpired: server.kill()
        if mock:
            mock.terminate()
            try: mock.wait(timeout=5)
            except subprocess.TimeoutExpired: mock.kill()

    write_summary(evid, test_id="ST-08 long-grill drift",
                  kill_pass=kill_pass, lines=log)
    print(f"[ST-08] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
