//! TB-16.x.fix — 5 守恒 tests for OBS_R022 closure (Art. 0.2 parallel-ledger).
//!
//! Architect ruling 2026-05-04: `LATEST_MARKOV_CAPSULE.txt` is a parallel
//! ledger and must be removed from canonical inputs. These 5 tests
//! together prevent the violation from regressing:
//!
//! 1. `markov_pointer_no_global_parallel_ledger`
//!    — the global pointer file does not exist on disk.
//! 2. `audit_tape_genesis_without_markov_pointer`
//!    — `audit_tape` invoked without `--markov-pointer` produces a
//!      verdict with Layer G assertions Skipped (genesis chain).
//! 3. `audit_tape_blocks_unresolvable_present_markov_pointer`
//!    — `audit_tape` invoked WITH `--markov-pointer` pointing to a
//!      garbage-cid file BLOCKs (exit 1) — TB-16.x.1 fail-closed
//!      semantic preserved post-de-canonicalization.
//! 4. `generate_markov_capsule_does_not_write_global_latest`
//!    — running the generator with `--out-dir <tmp> --no-cas` does not
//!      write `LATEST_MARKOV_CAPSULE.txt` anywhere (neither tmp nor
//!      the global path).
//! 5. `markov_capsule_historical_artifact_not_reference_input`
//!    — no source file under `src/` reads
//!      `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`.
//!
//! TRACE_MATRIX FC1-N34 (audit_tape binary) + FC2-N31 (verdict.json
//! schema). FC1/FC2/FC3 unchanged (Markov capsule is a derived view per
//! architect Q2.a).

use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn target_bin(name: &str) -> Option<PathBuf> {
    let manifest = manifest_dir();
    let dbg = manifest.join("target").join("debug").join(name);
    if dbg.exists() {
        return Some(dbg);
    }
    let rel = manifest.join("target").join("release").join(name);
    if rel.exists() {
        return Some(rel);
    }
    None
}

fn fixture_smoke_dir() -> Option<PathBuf> {
    let p = manifest_dir()
        .join("handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171");
    if p.is_dir() && p.join("runtime_repo").is_dir() && p.join("cas").is_dir() {
        Some(p)
    } else {
        None
    }
}

// ─────────────────────────────────────────────────────────────────────
// Test 1 — global pointer file does not exist
// ─────────────────────────────────────────────────────────────────────

#[test]
fn markov_pointer_no_global_parallel_ledger() {
    let global = manifest_dir().join("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt");
    assert!(
        !global.exists(),
        "Art. 0.2 violation: global Markov pointer must not exist as canonical input. \
         Found at {:?}. Per architect ruling 2026-05-04 (Option α), this file is \
         de-canonicalized and must be deleted. See \
         handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md.",
        global
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 2 — fresh chain without --markov-pointer → Layer G Skipped
// ─────────────────────────────────────────────────────────────────────

#[test]
fn audit_tape_genesis_without_markov_pointer() {
    let Some(bin) = target_bin("audit_tape") else {
        eprintln!("skipping: audit_tape binary not built");
        return;
    };
    let Some(smoke) = fixture_smoke_dir() else {
        eprintln!("skipping: TB-13 fixture not present");
        return;
    };
    let manifest = manifest_dir();
    let out_path = std::env::temp_dir()
        .join(format!("tb_16_x_fix_genesis_verdict_{}.json", std::process::id()));
    let _ = std::fs::remove_file(&out_path);
    let status = Command::new(&bin)
        .arg("--runtime-repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas-dir")
        .arg(smoke.join("cas"))
        .arg("--agent-pubkeys")
        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
        .arg("--pinned-pubkeys")
        .arg(smoke.join("runtime_repo/pinned_pubkeys.json"))
        .arg("--genesis")
        .arg(manifest.join("genesis_payload.toml"))
        .arg("--constitution")
        .arg(manifest.join("constitution.md"))
        .arg("--alignment-dir")
        .arg(manifest.join("handover/alignment"))
        .arg("--out")
        .arg(&out_path)
        // NOTE: NO --markov-pointer flag — this is the genesis API per
        // architect Q2.b (`previous_capsule_cid: None` is constitutional
        // genesis on fresh isolated chain).
        .status()
        .expect("audit_tape spawn");
    assert!(
        out_path.exists(),
        "audit_tape did not write verdict.json without --markov-pointer (status={:?}); \
         architect Q2.c: absent flag MUST be the genesis API",
        status
    );
    let verdict_text = std::fs::read_to_string(&out_path).expect("read verdict");
    let v: serde_json::Value = serde_json::from_str(&verdict_text).expect("parse verdict");
    assert_eq!(v["schema_version"], "v1/audit_tape_verdict");
    let assertions = v["assertions"].as_array().expect("assertions array");
    let layer_g_skipped = assertions
        .iter()
        .filter(|a| a["layer"].as_str() == Some("G"))
        .filter(|a| a["result"].as_str() == Some("Skipped"))
        .count();
    assert!(
        layer_g_skipped >= 4,
        "expected ≥4 Layer G Skipped on genesis chain; got {layer_g_skipped}. \
         Per architect Q2.b, fresh isolated chain → markov_capsule=None → \
         Layer G assertions Skipped (constitutional, not bypass)."
    );
    let _ = std::fs::remove_file(&out_path);
}

// ─────────────────────────────────────────────────────────────────────
// Test 3 — present but unresolvable pointer → BLOCK
// ─────────────────────────────────────────────────────────────────────

#[test]
fn audit_tape_blocks_unresolvable_present_markov_pointer() {
    let Some(bin) = target_bin("audit_tape") else {
        eprintln!("skipping: audit_tape binary not built");
        return;
    };
    let Some(smoke) = fixture_smoke_dir() else {
        eprintln!("skipping: TB-13 fixture not present");
        return;
    };
    let manifest = manifest_dir();
    let pointer_path = std::env::temp_dir()
        .join(format!("tb_16_x_fix_garbage_pointer_{}.txt", std::process::id()));
    // Garbage cid hex — well-formed (64 hex chars) but NOT in any CAS.
    std::fs::write(
        &pointer_path,
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
    )
    .expect("write garbage pointer");
    let out_path = std::env::temp_dir()
        .join(format!("tb_16_x_fix_block_verdict_{}.json", std::process::id()));
    let _ = std::fs::remove_file(&out_path);
    let status = Command::new(&bin)
        .arg("--runtime-repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas-dir")
        .arg(smoke.join("cas"))
        .arg("--agent-pubkeys")
        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
        .arg("--pinned-pubkeys")
        .arg(smoke.join("runtime_repo/pinned_pubkeys.json"))
        .arg("--genesis")
        .arg(manifest.join("genesis_payload.toml"))
        .arg("--constitution")
        .arg(manifest.join("constitution.md"))
        .arg("--markov-pointer")
        .arg(&pointer_path)
        .arg("--alignment-dir")
        .arg(manifest.join("handover/alignment"))
        .arg("--out")
        .arg(&out_path)
        .status()
        .expect("audit_tape spawn");
    // TB-16.x.1 fail-closed semantic: present-but-unresolvable pointer
    // must produce a non-zero exit (BLOCK or load-error). NOT silent
    // Skipped (architect Q2.c last paragraph).
    assert!(
        !status.success(),
        "audit_tape should BLOCK when --markov-pointer points to unresolvable cid; \
         got status={:?}. Per architect Q2.c: 'pointer present but CAS cannot \
         resolve -> silent None' is forbidden — must be fail-closed.",
        status
    );
    let _ = std::fs::remove_file(&pointer_path);
    let _ = std::fs::remove_file(&out_path);
}

// ─────────────────────────────────────────────────────────────────────
// Test 3b — supplied-but-FS-absent pointer → BLOCK
// (Closes Codex CHALLENGE 6 — distinct from Test 3 which uses a
// well-formed-but-CAS-unresolvable hex.)
// ─────────────────────────────────────────────────────────────────────

#[test]
fn audit_tape_blocks_supplied_but_fs_absent_markov_pointer() {
    let Some(bin) = target_bin("audit_tape") else {
        eprintln!("skipping: audit_tape binary not built");
        return;
    };
    let Some(smoke) = fixture_smoke_dir() else {
        eprintln!("skipping: TB-13 fixture not present");
        return;
    };
    let manifest = manifest_dir();
    let absent_path = std::env::temp_dir()
        .join(format!("tb_16_x_fix_absent_pointer_{}.txt", std::process::id()));
    // ensure it does NOT exist
    let _ = std::fs::remove_file(&absent_path);
    assert!(!absent_path.exists());
    let out_path = std::env::temp_dir()
        .join(format!("tb_16_x_fix_absent_verdict_{}.json", std::process::id()));
    let _ = std::fs::remove_file(&out_path);
    let status = Command::new(&bin)
        .arg("--runtime-repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas-dir")
        .arg(smoke.join("cas"))
        .arg("--agent-pubkeys")
        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
        .arg("--pinned-pubkeys")
        .arg(smoke.join("runtime_repo/pinned_pubkeys.json"))
        .arg("--genesis")
        .arg(manifest.join("genesis_payload.toml"))
        .arg("--constitution")
        .arg(manifest.join("constitution.md"))
        .arg("--markov-pointer")
        .arg(&absent_path)
        .arg("--alignment-dir")
        .arg(manifest.join("handover/alignment"))
        .arg("--out")
        .arg(&out_path)
        .status()
        .expect("audit_tape spawn");
    assert!(
        !status.success(),
        "audit_tape should BLOCK when --markov-pointer is supplied but FS-absent; \
         got status={:?}. Per architect Q2.c last paragraph: 'pointer present but \
         cannot resolve' must NOT silently None.",
        status
    );
    let _ = std::fs::remove_file(&out_path);
}

// ─────────────────────────────────────────────────────────────────────
// Test 3c — --markov-pointer + --prior-chain-runtime-repo are mutex
// (Closes Codex CHALLENGE 6 — verifies CLI mutex per architect §B.3.1)
// ─────────────────────────────────────────────────────────────────────

#[test]
fn audit_tape_rejects_both_markov_pointer_and_prior_chain() {
    let Some(bin) = target_bin("audit_tape") else {
        eprintln!("skipping: audit_tape binary not built");
        return;
    };
    let Some(smoke) = fixture_smoke_dir() else {
        eprintln!("skipping: TB-13 fixture not present");
        return;
    };
    let manifest = manifest_dir();
    let out_path = std::env::temp_dir()
        .join(format!("tb_16_x_fix_mutex_verdict_{}.json", std::process::id()));
    let _ = std::fs::remove_file(&out_path);
    let out = Command::new(&bin)
        .arg("--runtime-repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas-dir")
        .arg(smoke.join("cas"))
        .arg("--agent-pubkeys")
        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
        .arg("--pinned-pubkeys")
        .arg(smoke.join("runtime_repo/pinned_pubkeys.json"))
        .arg("--genesis")
        .arg(manifest.join("genesis_payload.toml"))
        .arg("--constitution")
        .arg(manifest.join("constitution.md"))
        .arg("--markov-pointer")
        .arg("/tmp/anything")
        .arg("--prior-chain-runtime-repo")
        .arg("/tmp/anything-else")
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("audit_tape spawn");
    assert!(
        !out.status.success(),
        "audit_tape must reject both flags simultaneously; got status={:?}",
        out.status
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("mutually exclusive"),
        "audit_tape mutex error message must say 'mutually exclusive'; got stderr:\n{stderr}"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 3d — --prior-chain-runtime-repo resolver semantic
// (Closes Codex CHALLENGE 6 — confirms α resolver behavior:
// markov_tip.cid present → uses it; absent → genesis)
// ─────────────────────────────────────────────────────────────────────

#[test]
fn audit_tape_prior_chain_resolver_genesis_when_tip_absent() {
    let Some(bin) = target_bin("audit_tape") else {
        eprintln!("skipping: audit_tape binary not built");
        return;
    };
    let Some(smoke) = fixture_smoke_dir() else {
        eprintln!("skipping: TB-13 fixture not present");
        return;
    };
    let manifest = manifest_dir();
    // Create an empty tmp dir as the "prior chain runtime_repo" with NO
    // markov_tip.cid — resolver must treat this as genesis-equivalent
    // (None inheritance), so the run should produce Layer G Skipped just
    // like Test 2 (no flag at all).
    let prior = std::env::temp_dir()
        .join(format!("tb_16_x_fix_prior_chain_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&prior);
    std::fs::create_dir_all(&prior).expect("create prior dir");
    let out_path = std::env::temp_dir()
        .join(format!("tb_16_x_fix_prior_verdict_{}.json", std::process::id()));
    let _ = std::fs::remove_file(&out_path);
    let status = Command::new(&bin)
        .arg("--runtime-repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas-dir")
        .arg(smoke.join("cas"))
        .arg("--agent-pubkeys")
        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
        .arg("--pinned-pubkeys")
        .arg(smoke.join("runtime_repo/pinned_pubkeys.json"))
        .arg("--genesis")
        .arg(manifest.join("genesis_payload.toml"))
        .arg("--constitution")
        .arg(manifest.join("constitution.md"))
        .arg("--prior-chain-runtime-repo")
        .arg(&prior)
        .arg("--alignment-dir")
        .arg(manifest.join("handover/alignment"))
        .arg("--out")
        .arg(&out_path)
        .status()
        .expect("audit_tape spawn");
    assert!(
        out_path.exists(),
        "audit_tape did not write verdict.json with --prior-chain-runtime-repo (no \
         markov_tip.cid present); status={:?}",
        status
    );
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out_path).expect("read verdict"))
            .expect("parse verdict");
    let layer_g_skipped = v["assertions"]
        .as_array()
        .expect("assertions")
        .iter()
        .filter(|a| a["layer"].as_str() == Some("G"))
        .filter(|a| a["result"].as_str() == Some("Skipped"))
        .count();
    assert!(
        layer_g_skipped >= 4,
        "expected ≥4 Layer G Skipped on prior-chain-runtime-repo with no \
         markov_tip.cid (genesis-equivalent inheritance); got {layer_g_skipped}"
    );
    let _ = std::fs::remove_dir_all(&prior);
    let _ = std::fs::remove_file(&out_path);
}

// ─────────────────────────────────────────────────────────────────────
// Test 4 — generate_markov_capsule does not write global LATEST
// ─────────────────────────────────────────────────────────────────────

#[test]
fn generate_markov_capsule_does_not_write_global_latest() {
    let Some(bin) = target_bin("generate_markov_capsule") else {
        eprintln!("skipping: generate_markov_capsule binary not built");
        return;
    };
    let manifest = manifest_dir();
    let global_pointer = manifest.join("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt");
    let global_existed_before = global_pointer.exists();

    let out_dir = std::env::temp_dir()
        .join(format!("tb_16_x_fix_gen_outdir_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);
    std::fs::create_dir_all(&out_dir).expect("create out_dir");

    let status = Command::new(&bin)
        .arg("--tb-id")
        .arg("16xfix-test")
        .arg("--out-dir")
        .arg(&out_dir)
        .arg("--constitution-path")
        .arg(manifest.join("constitution.md"))
        .arg("--no-cas")
        .status()
        .expect("generate_markov_capsule spawn");
    assert!(
        status.success(),
        "generate_markov_capsule failed in --no-cas mode (status={:?})",
        status
    );

    // (a) per-run out_dir must NOT contain LATEST_MARKOV_CAPSULE.txt.
    let per_run_latest = out_dir.join("LATEST_MARKOV_CAPSULE.txt");
    assert!(
        !per_run_latest.exists(),
        "generate_markov_capsule wrote {per_run_latest:?} — architect ruling \
         §B.3.1 #2: 'generate_markov_capsule 不再写 LATEST_MARKOV_CAPSULE.txt'."
    );

    // (b) per-run JSON should still be present (historical artifact).
    let any_json = std::fs::read_dir(&out_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .any(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("MARKOV_TB-")
        });
    assert!(
        any_json,
        "expected MARKOV_TB-*.json historical artifact in {out_dir:?}"
    );

    // (c) global pointer at handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt
    //     must still not exist (if it did not exist before; if architect-pre-α
    //     state, the test will fail on test 1 first which is the right alarm).
    if !global_existed_before {
        assert!(
            !global_pointer.exists(),
            "generate_markov_capsule created global pointer {global_pointer:?} — \
             violates Art. 0.2"
        );
    }

    let _ = std::fs::remove_dir_all(&out_dir);
}

// ─────────────────────────────────────────────────────────────────────
// Test 5 — no source under src/ references the literal global path
// ─────────────────────────────────────────────────────────────────────

#[test]
fn markov_capsule_historical_artifact_not_reference_input() {
    let src = manifest_dir().join("src");
    let mut offenders: Vec<String> = Vec::new();
    walk_for_literal(&src, "handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt", &mut offenders);
    // Doc-comment / module-comment mentions are also offenders if they
    // imply runtime use; allow comments only if they explicitly disclaim.
    // Actionable = the path appears as live Rust code, NOT inside a
    // comment. Doc-comments are permitted to reference the path as
    // historical context (the deprecation/removal disclaimer itself
    // names the path so future readers know what was removed).
    let actionable: Vec<String> = offenders
        .into_iter()
        .filter(|line_with_loc| {
            // Strip "<path>:<lineno>: " prefix to get the source text.
            let src_text = line_with_loc.splitn(3, ':').nth(2).unwrap_or(line_with_loc);
            !is_comment_line(src_text)
        })
        .collect();
    assert!(
        actionable.is_empty(),
        "src/ files reference handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt as a \
         (non-disclaimed) input — Art. 0.2 violation per architect ruling §B.7.1 \
         (runtime fact must come from Q_t/ChainTape/CAS, not global pointer). \
         Offenders:\n{}",
        actionable.join("\n")
    );
}

/// Returns true when `line` is a Rust comment / doc-comment / block-
/// comment continuation (so it cannot be runtime code referencing the
/// path). The check is conservative: any leading whitespace + comment
/// marker, or a leading `* ` block-comment continuation, qualifies.
fn is_comment_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("///")
        || trimmed.starts_with("//!")
        || trimmed.starts_with("//")
        || trimmed.starts_with("* ")
        || trimmed.starts_with("*/")
        || trimmed.starts_with("/*")
}

fn walk_for_literal(dir: &Path, needle: &str, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk_for_literal(&p, needle, out);
            continue;
        }
        let ext_ok = p
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s == "rs")
            .unwrap_or(false);
        if !ext_ok {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        for (i, line) in text.lines().enumerate() {
            if line.contains(needle) {
                out.push(format!("{}:{}: {}", p.display(), i + 1, line.trim()));
            }
        }
    }
}
