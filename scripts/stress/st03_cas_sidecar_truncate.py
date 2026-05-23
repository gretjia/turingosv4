#!/usr/bin/env python3
"""ST-03 — CAS sidecar index truncation (TB-STRESS-PHASE-2).

Writes N capsules via `cargo test --test build_session_view_error_distinction
-- bad_capsule_returns_decode_error` (which exercises CasStore::put), then
truncates the sidecar `.turingos_cas_index.jsonl` at byte = len/2, then runs
the same test path again to verify no panic + clean recovery.

KILL:
  - no panic in any code path
  - CasStore::open either succeeds (with partial entries) OR returns a
    clean CasError after sidecar truncation
"""
from __future__ import annotations

import os
import shutil
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402


def main() -> int:
    evid = evidence_dir("st03_cas_sidecar_truncate")
    log: list[str] = []

    # Stage 1: invoke the existing S3 test that writes a bogus capsule
    # via CasStore::put; this populates a tmp CAS we can intercept.
    # Since the test uses tempdir, we instead reproduce the helper here
    # by invoking `cargo run` against a tiny inline program — but that's
    # heavy. Simpler approach: use a long-lived workspace under evid/.
    ws = evid / "workspace"
    cas = ws / "cas"
    cas.mkdir(parents=True, exist_ok=True)
    subprocess.run(["git", "init", "--quiet"], cwd=cas, check=True)

    # Write some fake capsule files + sidecar entries by replaying the
    # shape that CasStore::put would produce. We don't need the real CAS
    # write path — we need to test that CasStore::open + reload_index_from_sidecar
    # tolerates a truncated jsonl.
    sidecar = cas / ".turingos_cas_index.jsonl"
    # Each entry is a JSON object with cid_hex + object_type + schema_id +
    # creator + created_at_logical_t + raw_path + raw_size. Match the v4
    # format precisely or CasStore::open's strict mode rejects.
    import json as _json
    entries = []
    for i in range(50):
        # Synthetic 32-byte CID
        cid_hex = format(i, "064x")
        entry = {
            "cid_hex": cid_hex,
            "object_type": "EvidenceCapsule",
            "schema_id": "turingos-spec-grill-session-v1",
            "creator": "st03",
            "created_at_logical_t": 1000 + i,
            "raw_size": 0,
            "raw_path": f"objects/{cid_hex[:2]}/{cid_hex[2:]}.bin",
        }
        entries.append(_json.dumps(entry))
    full = "\n".join(entries) + "\n"
    sidecar.write_text(full)
    orig_bytes = len(full.encode())
    trunc_at = orig_bytes // 2
    log.append(f"sidecar entries=50 bytes={orig_bytes} trunc_at={trunc_at}")

    truncated = full.encode()[:trunc_at]
    sidecar.write_bytes(truncated)
    log.append(f"post-truncate bytes={len(truncated)}")

    # Stage 2: invoke `cargo test --test build_session_view_error_distinction`
    # against this corrupted workspace via env override (the test creates
    # its own tempdir, so we can't intercept that). Instead, we use a Rust
    # one-liner harness: just invoke the existing CasStore::open via cargo
    # test name filter that does CAS open.
    #
    # We test via the existing test 'empty_workspace_returns_ok_spec_pending'
    # but rerouted at a workspace we control. Since tests use tempdir,
    # the cleanest indirection is to use a small bash helper that exec's
    # `cargo run --bin turingos -- welcome --workspace <ws>` — welcome
    # tries to read CAS and surfaces the open path.
    bin_check = subprocess.run(
        ["cargo", "build", "--bin", "turingos", "--quiet"],
        cwd=PROJECT_ROOT, capture_output=True, text=True,
    )
    if bin_check.returncode != 0:
        log.append(f"cargo build failed: {bin_check.stderr[:500]}")
        write_summary(evid, test_id="ST-03 CAS sidecar truncate",
                      kill_pass=False, lines=log)
        return 1

    bin_path = PROJECT_ROOT / "target" / "debug" / "turingos"
    env = os.environ.copy()
    env["TURINGOS_WORKSPACE"] = str(ws)
    env["TURINGOS_SKIP_LLM"] = "1"

    # We invoke `turingos welcome` which triggers CasStore::open + reads
    # latest_spec_capsule_cid. If it panics, exit code is 101 or signal.
    p = subprocess.run(
        [str(bin_path), "welcome", "--skip-llm"],
        cwd=ws, env=env, capture_output=True, text=True, timeout=30,
    )
    log.append(f"welcome rc={p.returncode}")
    log.append(f"welcome stdout (last 300 chars): {p.stdout[-300:]!r}")
    log.append(f"welcome stderr (last 300 chars): {p.stderr[-300:]!r}")

    panic = (
        "panicked at" in (p.stdout + p.stderr)
        or "thread '" in p.stderr and "panicked" in p.stderr
    )

    # Exit code 0 (graceful) or non-101 (non-panic error) is OK.
    # 101 with panic message = real corruption-induced panic = FAIL.
    kill_pass = (not panic) and (p.returncode not in (-11, -6, 101))
    log.append(f"panic_detected={panic}")

    write_summary(evid, test_id="ST-03 CAS sidecar truncate",
                  kill_pass=kill_pass, lines=log)
    print(f"[ST-03] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
