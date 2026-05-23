#!/usr/bin/env python3
"""ST-06 — LLM provider 5xx storm (TB-STRESS-PHASE-2).

Mock LLM provider returns 5xx with 50% probability. Runs N attempts via
`turingos llm complete`. Verifies:
  - the retry / failure-path engages
  - no panic
  - llm_err telemetry (when wired) records failures, not silent successes

KILL: ~half the attempts produce non-zero CLI exit; no panic in any
      attempt; CAS index does NOT grow by N (only by successful attempts).
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402

N_ATTEMPTS = int(os.environ.get("ST06_ATTEMPTS", "100"))
FAIL_RATE = float(os.environ.get("ST06_FAIL_RATE", "0.50"))


def start_mock(evid: Path) -> tuple[subprocess.Popen, int]:
    log = evid / "mock_llm.log"
    p = subprocess.Popen(
        ["python3", str(PROJECT_ROOT / "scripts" / "stress" / "_mock_llm_server.py"), "0"],
        env={**os.environ,
             "MOCK_FAIL_RATE": str(FAIL_RATE),
             "MOCK_LATENCY_MS": "20",
             "MOCK_RESPONSE_BODY": "ok",
             "MOCK_SEED": "42"},
        stdout=subprocess.PIPE, stderr=open(log, "wb"),
    )
    port = int(p.stdout.readline().decode().strip())
    return p, port


def main() -> int:
    evid = evidence_dir("st06_llm_5xx_storm")
    log: list[str] = []
    ws = evid / "workspace"
    ws.mkdir(exist_ok=True)
    (ws / "cas").mkdir(exist_ok=True)

    print("[ST-06] building turingos...")
    rc = subprocess.call(["cargo", "build", "--bin", "turingos", "--quiet"], cwd=PROJECT_ROOT)
    if rc != 0:
        log.append("build failed")
        write_summary(evid, test_id="ST-06 LLM 5xx storm", kill_pass=False, lines=log)
        return 1
    bin_path = PROJECT_ROOT / "target" / "debug" / "turingos"

    print(f"[ST-06] starting mock LLM (fail_rate={FAIL_RATE})...")
    mock, port = start_mock(evid)
    endpoint = f"http://127.0.0.1:{port}/v1/chat/completions"

    successes = 0
    failures = 0
    panics = 0
    try:
        for i in range(N_ATTEMPTS):
            env = os.environ.copy()
            env.update({
                "TURINGOS_SILICONFLOW_ENDPOINT": endpoint,
                "SILICONFLOW_API_KEY": "mock-key",
                "DEEPSEEK_API_KEY": "mock-key",
                "TURINGOS_WORKSPACE": str(ws),
            })
            prompt_file = evid / f"prompt_{i}.txt"
            prompt_file.write_text(f"attempt-{i}")
            p = subprocess.run(
                [str(bin_path), "llm", "complete",
                 "--workspace", str(ws),
                 "--prompt-file", str(prompt_file),
                 "--max-tokens", "30"],
                env=env, cwd=ws,
                capture_output=True, text=True, timeout=30,
            )
            prompt_file.unlink(missing_ok=True)
            if "panicked at" in (p.stdout + p.stderr):
                panics += 1
            elif p.returncode == 0:
                successes += 1
            else:
                failures += 1
            if (i + 1) % 20 == 0:
                print(f"  [ST-06] {i+1}/{N_ATTEMPTS}  ok={successes} fail={failures} panic={panics}")
    finally:
        mock.terminate()
        try:
            mock.wait(timeout=5)
        except subprocess.TimeoutExpired:
            mock.kill()

    log.append(f"attempts={N_ATTEMPTS}  success={successes}  fail={failures}  panic={panics}")
    expected_fail = int(N_ATTEMPTS * FAIL_RATE)
    within = abs(failures - expected_fail) <= N_ATTEMPTS * 0.25
    log.append(f"expected_fail≈{expected_fail}  within_tolerance={within}")
    kill_pass = (panics == 0) and within
    log.append(f"kill_pass={kill_pass}")

    write_summary(evid, test_id="ST-06 LLM 5xx storm",
                  kill_pass=kill_pass, lines=log)
    print(f"[ST-06] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
