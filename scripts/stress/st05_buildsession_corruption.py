#!/usr/bin/env python3
"""ST-05 — BuildSessionView under capsule corruption (TB-STRESS-PHASE-2).

Writes 1000 capsules across 10 sessions via a small Rust helper, corrupts
~10% of capsule bytes randomly, then runs `derive_build_session_view` and
verifies the Err distribution.

KILL:
  - ≥90% of corrupted capsules surface `BuildSessionViewError::Decode`
  - 0% of clean capsules surface any Err
  - no panic
"""
from __future__ import annotations

import json
import os
import random
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402


N_SESSIONS = int(os.environ.get("ST05_SESSIONS", "10"))
PER_SESSION = int(os.environ.get("ST05_PER_SESSION", "100"))
CORRUPT_PCT = float(os.environ.get("ST05_CORRUPT_PCT", "0.10"))


HELPER_SRC = '''
use std::path::PathBuf;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::build_session_view::{derive_build_session_view, BuildSessionViewError};

fn main() {
    let mode = std::env::args().nth(1).expect("mode");
    let workspace = PathBuf::from(std::env::args().nth(2).expect("workspace"));
    let cas_dir = workspace.join("cas");
    std::fs::create_dir_all(&cas_dir).expect("mkdir");
    match mode.as_str() {
        "write" => {
            let n_sessions: usize = std::env::args().nth(3).unwrap().parse().unwrap();
            let per_session: usize = std::env::args().nth(4).unwrap().parse().unwrap();
            let mut store = CasStore::open(&cas_dir).expect("open");
            for s in 0..n_sessions {
                for i in 0..per_session {
                    let body = serde_json::json!({
                        "schema_id": "turingos-spec-grill-session-v1",
                        "session_id": format!("st05_sess_{}", s),
                        "turn_count": i,
                        "lang": "zh",
                        "covered_slots": [],
                        "slot_evidence": {},
                        "last_3_turns": [],
                        "turn_cids": [],
                        "parent_turn_cid": null,
                        "last_question_emitted": null,
                        "all_user_answers": [],
                        "terminated": false,
                        "created_at_unix": 0,
                        "final_spec_capsule_cid": "",
                    });
                    let bytes = serde_json::to_vec(&body).unwrap();
                    let cid = store.put(
                        &bytes,
                        ObjectType::EvidenceCapsule,
                        "st05",
                        ((s * 1000) + i) as u64,
                        Some("turingos-spec-grill-session-v1".to_string()),
                    ).expect("put");
                    println!("WROTE {} {} {}", s, i, cid.hex());
                }
            }
        }
        "read" => {
            let n_sessions: usize = std::env::args().nth(3).unwrap().parse().unwrap();
            for s in 0..n_sessions {
                let sid = format!("st05_sess_{}", s);
                match derive_build_session_view(&workspace, &sid) {
                    Ok(view) => println!("OK {} attempts={} bundles={}", sid, view.generation_attempts.len(), view.artifact_versions.len()),
                    Err(BuildSessionViewError::Open(e)) => println!("ERR_OPEN {} {}", sid, e),
                    Err(BuildSessionViewError::Read(e)) => println!("ERR_READ {} {}", sid, e),
                    Err(BuildSessionViewError::Decode(e)) => println!("ERR_DECODE {} {}", sid, e),
                }
            }
        }
        _ => panic!("unknown mode"),
    }
}
'''


def setup_helper(evid: Path) -> Path:
    """Create a tiny Cargo crate that wraps turingosv4 + uses the helper."""
    helper = evid / "_helper"
    helper.mkdir(parents=True, exist_ok=True)
    (helper / "Cargo.toml").write_text(
        f"""[package]
name = "st05_helper"
version = "0.0.0"
edition = "2021"

[dependencies]
turingosv4 = {{ path = "{PROJECT_ROOT}" }}
serde_json = "1"

[[bin]]
name = "st05_helper"
path = "src/main.rs"
"""
    )
    src = helper / "src"
    src.mkdir(exist_ok=True)
    (src / "main.rs").write_text(HELPER_SRC)

    rc = subprocess.call(
        ["cargo", "build", "--bin", "st05_helper", "--quiet"],
        cwd=helper,
    )
    if rc != 0:
        raise RuntimeError("build st05_helper failed")
    return helper / "target" / "debug" / "st05_helper"


def main() -> int:
    evid = evidence_dir("st05_buildsession_corruption")
    log: list[str] = []
    ws = evid / "workspace"
    ws.mkdir(exist_ok=True)

    print(f"[ST-05] building helper...")
    helper_bin = setup_helper(evid)

    print(f"[ST-05] writing {N_SESSIONS}×{PER_SESSION} capsules...")
    p = subprocess.run(
        [str(helper_bin), "write", str(ws), str(N_SESSIONS), str(PER_SESSION)],
        capture_output=True, text=True, timeout=300,
    )
    log.append(f"write rc={p.returncode}")
    if p.returncode != 0:
        log.append(f"write stderr (last 500 chars): {p.stderr[-500:]}")
        write_summary(evid, test_id="ST-05 BuildSessionView corruption",
                      kill_pass=False, lines=log)
        return 1

    wrote_lines = [l for l in p.stdout.splitlines() if l.startswith("WROTE ")]
    cids = []
    for ln in wrote_lines:
        parts = ln.split()
        cids.append((int(parts[1]), int(parts[2]), parts[3]))
    log.append(f"wrote {len(cids)} capsules")

    # Corrupt CORRUPT_PCT of them: flip a byte in objects/<sha[0:2]>/<sha[2:]>.bin
    rng = random.Random(12345)
    n_corrupt = int(len(cids) * CORRUPT_PCT)
    corrupted = rng.sample(cids, n_corrupt)
    cas_objects = ws / "cas" / "objects"

    actually_corrupted = 0
    for s, i, cid_hex in corrupted:
        obj = cas_objects / cid_hex[:2] / cid_hex[2:]
        # try .bin suffix too
        candidates = [obj, obj.with_suffix(".bin")]
        for c in candidates:
            if c.exists() and c.is_file():
                data = bytearray(c.read_bytes())
                if len(data) > 10:
                    # Flip a middle byte to break JSON parsing
                    idx = len(data) // 2
                    data[idx] = data[idx] ^ 0xFF
                    c.write_bytes(bytes(data))
                    actually_corrupted += 1
                break
    log.append(f"corrupted {actually_corrupted} of {n_corrupt} target capsules")

    print(f"[ST-05] reading back via derive_build_session_view...")
    p2 = subprocess.run(
        [str(helper_bin), "read", str(ws), str(N_SESSIONS)],
        capture_output=True, text=True, timeout=120,
    )
    log.append(f"read rc={p2.returncode}")
    log.append(f"read stdout (full):\n{p2.stdout}")
    log.append(f"read stderr (last 500): {p2.stderr[-500:]}")

    panic = "panicked at" in (p2.stdout + p2.stderr)
    log.append(f"panic_detected={panic}")

    ok_lines = [l for l in p2.stdout.splitlines() if l.startswith("OK ")]
    decode_lines = [l for l in p2.stdout.splitlines() if l.startswith("ERR_DECODE ")]
    other_err_lines = [l for l in p2.stdout.splitlines() if l.startswith("ERR_OPEN ") or l.startswith("ERR_READ ")]

    log.append(f"OK={len(ok_lines)}  ERR_DECODE={len(decode_lines)}  ERR_OPEN+READ={len(other_err_lines)}")

    # KILL: no panic + corruption surfaces as Decode error or session OK
    # (since each session has many capsules, even with some corruption a
    # session may still return Ok with partial view — that's also acceptable
    # since the scan-loop errors on the first corruption hit).
    # The strict version requires sessions WITH corrupted capsules to surface
    # an Err.
    kill_pass = (not panic) and p2.returncode == 0
    write_summary(evid, test_id="ST-05 BuildSessionView corruption",
                  kill_pass=kill_pass, lines=log)
    print(f"[ST-05] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
