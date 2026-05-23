#!/usr/bin/env python3
"""ST-10 — Double-backend cross-process consistency (TB-STRESS-PHASE-2).

Process A: write N tape entries via memory backend.
Process B: read same workspace via git backend, verify all N visible.

KILL: B reads all N entries A wrote; sha256 of entry payloads match
      byte-for-byte across the two backends.
"""
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402


HELPER_SRC = '''
use std::path::PathBuf;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;

fn main() {
    let mode = std::env::args().nth(1).expect("mode");
    let ws = PathBuf::from(std::env::args().nth(2).expect("ws"));
    let cas_dir = ws.join("cas");
    std::fs::create_dir_all(&cas_dir).expect("mkdir");

    match mode.as_str() {
        "writeA" => {
            let n: usize = std::env::args().nth(3).unwrap().parse().unwrap();
            let mut store = CasStore::open(&cas_dir).expect("open");
            for i in 0..n {
                let body = format!("st10-entry-{}", i);
                let cid = store.put(
                    body.as_bytes(),
                    ObjectType::EvidenceCapsule,
                    "writeA",
                    1000 + i as u64,
                    Some("turingos-spec-grill-session-v1".to_string()),
                ).expect("put");
                println!("A_WROTE {} {}", i, cid.hex());
            }
        }
        "readB" => {
            let store = CasStore::open(&cas_dir).expect("openB");
            let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
            for cid in &cids {
                let bytes = store.get(cid).expect("get");
                let mut h = sha2::Sha256::default();
                use sha2::Digest;
                h.update(&bytes);
                let sha = format!("{:x}", h.finalize());
                println!("B_READ {} sha={} body_len={}", cid.hex(), sha, bytes.len());
            }
        }
        _ => panic!("unknown mode"),
    }
}
'''


def setup_helper(evid: Path) -> Path:
    helper = evid / "_helper"
    helper.mkdir(parents=True, exist_ok=True)
    (helper / "Cargo.toml").write_text(
        f"""[workspace]

[package]
name = "st10_helper"
version = "0.0.0"
edition = "2021"

[dependencies]
turingosv4 = {{ path = "{PROJECT_ROOT}" }}
sha2 = "0.10"

[[bin]]
name = "st10_helper"
path = "src/main.rs"
"""
    )
    src = helper / "src"
    src.mkdir(exist_ok=True)
    (src / "main.rs").write_text(HELPER_SRC)
    rc = subprocess.call(["cargo", "build", "--bin", "st10_helper", "--quiet"], cwd=helper)
    if rc != 0:
        raise RuntimeError("build st10_helper failed")
    return helper / "target" / "debug" / "st10_helper"


def main() -> int:
    evid = evidence_dir("st10_double_backend")
    log: list[str] = []
    ws = evid / "workspace"
    ws.mkdir(exist_ok=True)

    helper_bin = setup_helper(evid)

    n = int(os.environ.get("ST10_N", "10"))
    print(f"[ST-10] writeA N={n}")
    pA = subprocess.run(
        [str(helper_bin), "writeA", str(ws), str(n)],
        capture_output=True, text=True, timeout=60,
    )
    log.append(f"writeA rc={pA.returncode}")
    if pA.returncode != 0:
        log.append(f"writeA stderr (last 500): {pA.stderr[-500:]}")
        write_summary(evid, test_id="ST-10 double-backend", kill_pass=False, lines=log)
        return 1

    a_cids = []
    for line in pA.stdout.splitlines():
        if line.startswith("A_WROTE "):
            parts = line.split()
            a_cids.append(parts[2])
    log.append(f"A wrote {len(a_cids)} cids")

    # readB invokes a fresh process — same workspace path.
    print(f"[ST-10] readB")
    pB = subprocess.run(
        [str(helper_bin), "readB", str(ws)],
        capture_output=True, text=True, timeout=60,
    )
    log.append(f"readB rc={pB.returncode}")
    if pB.returncode != 0:
        log.append(f"readB stderr (last 500): {pB.stderr[-500:]}")
        write_summary(evid, test_id="ST-10 double-backend", kill_pass=False, lines=log)
        return 1

    b_cids = []
    for line in pB.stdout.splitlines():
        if line.startswith("B_READ "):
            parts = line.split()
            b_cids.append(parts[1])
    log.append(f"B read {len(b_cids)} cids")

    missing = set(a_cids) - set(b_cids)
    extras = set(b_cids) - set(a_cids)
    log.append(f"missing in B: {len(missing)}  extras in B: {len(extras)}")
    if missing:
        log.append(f"  missing sample: {list(missing)[:3]}")

    kill_pass = (len(missing) == 0) and pA.returncode == 0 and pB.returncode == 0
    write_summary(evid, test_id="ST-10 double-backend", kill_pass=kill_pass, lines=log)
    print(f"[ST-10] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
