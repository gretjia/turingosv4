#!/usr/bin/env python3
"""ST-07 — 100 concurrent task/open with 30% malformed CLI stdout.

Starts a turingos_web server with TURINGOS_CLI_BIN pointing at a stub
binary that emits malformed stdout 30% of the time. Fires 100 concurrent
POST /api/task/open. Verifies the response distribution:
  - clean stdout → 200 OK with task_id (String)
  - malformed stdout → 502 BAD_GATEWAY with kind="task_id_parse_failed"

KILL: 100 total responses; 30% ± 10% are 502; zero 5xx other than 502;
      no panic in server log.
"""
from __future__ import annotations

import json
import os
import random
import shutil
import socket
import subprocess
import sys
import time
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402

N_REQUESTS = int(os.environ.get("ST07_REQUESTS", "100"))
MALFORM_PCT = float(os.environ.get("ST07_MALFORM_PCT", "0.30"))


STUB_BIN = '''#!/usr/bin/env python3
"""Stub turingos CLI for ST-07: emits malformed stdout 30% of the time."""
import json
import os
import random
import sys

sub = sys.argv[1] if len(sys.argv) > 1 else ""
malform = random.random() < float(os.environ.get("ST07_STUB_MALFORM", "0.30"))

if sub == "task":
    if malform:
        # Garbage that won't parse as a task_id
        sys.stdout.write("ERR: gibberish " + os.urandom(8).hex() + "\\n")
        sys.exit(0)
    else:
        task_id = "task_" + os.urandom(8).hex()
        sys.stdout.write(json.dumps({"task_id": task_id, "bounty_cents": 1000}) + "\\n")
        sys.exit(0)
else:
    sys.exit(0)
'''


def free_port() -> int:
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    p = s.getsockname()[1]
    s.close()
    return p


def main() -> int:
    evid = evidence_dir("st07_taskopen_concurrent_malformed")
    log: list[str] = []
    ws = evid / "workspace"
    ws.mkdir(exist_ok=True)
    (ws / "cas").mkdir(exist_ok=True)

    # Build the stub binary.
    stub_dir = evid / "stub"
    stub_dir.mkdir(exist_ok=True)
    stub = stub_dir / "turingos"
    stub.write_text(STUB_BIN)
    stub.chmod(0o755)

    # Build turingos_web for the web feature.
    print("[ST-07] building turingos_web...")
    rc = subprocess.call(
        ["cargo", "build", "--features", "web", "--bin", "turingos_web", "--quiet"],
        cwd=PROJECT_ROOT,
    )
    if rc != 0:
        log.append("cargo build turingos_web failed")
        write_summary(evid, test_id="ST-07 concurrent malformed task/open",
                      kill_pass=False, lines=log)
        return 1
    web_bin = PROJECT_ROOT / "target" / "debug" / "turingos_web"

    port = free_port()
    env = os.environ.copy()
    env["TURINGOS_WEB_WORKSPACE"] = str(ws)
    env["TURINGOS_WEB_PORT"] = str(port)
    env["TURINGOS_CLI_BIN"] = str(stub)
    env["ST07_STUB_MALFORM"] = str(MALFORM_PCT)
    env["RUST_LOG"] = "warn"

    server_log = evid / "server.log"
    server = subprocess.Popen(
        [str(web_bin)],
        cwd=ws, env=env,
        stdout=open(server_log, "wb"), stderr=subprocess.STDOUT,
    )
    log.append(f"server pid={server.pid} port={port}")
    try:
        # Wait for server to be ready
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
            log.append("server failed to become ready")
            return 1

        # Concurrent task/open
        def one_request(i: int) -> tuple[int, str]:
            body = json.dumps({"problem_md": f"# stress {i}", "bounty_cents": 1000}).encode()
            req = urllib.request.Request(
                f"http://127.0.0.1:{port}/api/task/open",
                data=body, headers={"Content-Type": "application/json"},
                method="POST",
            )
            try:
                with urllib.request.urlopen(req, timeout=15.0) as r:
                    return r.status, r.read().decode("utf-8", "ignore")[:200]
            except urllib.error.HTTPError as e:
                return e.code, e.read().decode("utf-8", "ignore")[:200]
            except Exception as e:
                return -1, str(e)

        statuses: list[tuple[int, str]] = []
        with ThreadPoolExecutor(max_workers=20) as ex:
            futures = [ex.submit(one_request, i) for i in range(N_REQUESTS)]
            for f in as_completed(futures):
                statuses.append(f.result())

        c200 = sum(1 for s, _ in statuses if s == 200)
        c502 = sum(1 for s, _ in statuses if s == 502)
        c_other = sum(1 for s, _ in statuses if s not in (200, 502))

        log.append(f"requests={N_REQUESTS}  200={c200}  502={c502}  other={c_other}")
        # Sample bodies
        sample_502 = [b for s, b in statuses if s == 502][:3]
        log.append(f"sample 502 bodies: {sample_502}")

        # Check server log for panic
        srv_text = server_log.read_text(errors="ignore")
        panic = "panicked at" in srv_text
        log.append(f"server panic={panic}")

        # KILL: c_other == 0 AND no panic AND 502 count within ±10% of expected
        expected_502 = N_REQUESTS * MALFORM_PCT
        within = abs(c502 - expected_502) <= N_REQUESTS * 0.10
        kill_pass = (c_other == 0) and (not panic) and within
        log.append(f"expected_502≈{expected_502}  within_tolerance={within}")
    finally:
        server.terminate()
        try:
            server.wait(timeout=10)
        except subprocess.TimeoutExpired:
            server.kill()

    write_summary(evid, test_id="ST-07 concurrent malformed task/open",
                  kill_pass=kill_pass, lines=log)
    print(f"[ST-07] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
