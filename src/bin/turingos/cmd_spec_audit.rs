//! TRACE_MATRIX FC1 + FC2: turingos `spec audit` subcommand.
//!
//! Audits the spec sub-graph of a build session without any LLM or network calls.
//! Verifies that the latest `turingos-spec-capsule-v1` body bytes hash matches
//! the on-disk `spec.md` sha256 (if spec.md exists).
//!
//! FC-trace: FC1 (replay loop), FC2 (boot reconstruction)
//! Risk class: Class 2

use std::path::PathBuf;
use std::process::ExitCode;

/// TRACE_MATRIX FC2: `spec audit` short-help
pub(crate) const SHORT_HELP: &str =
    "Audit spec sub-graph from CAS; verify spec.md hash matches latest capsule";

/// TRACE_MATRIX FC2: `spec audit` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos spec audit — offline spec sub-graph audit

USAGE:
    turingos spec audit --workspace <PATH> --session <ID>

OPTIONS:
    --workspace <PATH>   Workspace directory (required).
    --session <ID>       Session ID to audit (required).
    -h, --help           Print this help.

DESCRIPTION:
    Reconstructs the spec sub-graph from CAS for the given session.
    Verifies that the latest `turingos-spec-capsule-v1` body bytes SHA-256
    matches the SHA-256 of on-disk `<workspace>/sessions/<session-id>/spec.md`
    (if spec.md is present). If spec.md has been deleted or was never written,
    only the CAS chain is verified.

    Exits 0 if all checks pass, non-zero on any mismatch or dangling reference.
    No LLM calls. No network. Pure CAS reconstruction.

EXAMPLES:
    turingos spec audit --workspace /data/my_ws --session abc123
"#;

/// TRACE_MATRIX FC2: `spec audit` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    let mut workspace: Option<PathBuf> = None;
    let mut session_id: Option<String> = None;
    let mut iter = args.iter();

    while let Some(a) = iter.next() {
        match a.as_str() {
            "--workspace" => {
                workspace = Some(PathBuf::from(
                    iter.next().cloned().unwrap_or_default(),
                ));
            }
            "--session" => {
                session_id = Some(
                    iter.next().cloned().unwrap_or_default(),
                );
            }
            _ => {}
        }
    }

    let workspace = match workspace {
        Some(p) => p,
        None => {
            eprintln!("turingos spec audit: --workspace is required");
            return ExitCode::from(2);
        }
    };
    let session_id = match session_id {
        Some(s) => s,
        None => {
            eprintln!("turingos spec audit: --session is required");
            return ExitCode::from(2);
        }
    };

    run_spec_audit(&workspace, &session_id)
}

fn run_spec_audit(workspace: &std::path::Path, session_id: &str) -> ExitCode {
    use turingosv4::runtime::replay::reconstruct_session;
    use turingosv4::runtime::spec_capsule::{latest_spec_capsule_cid, read_spec_capsule};

    // 1. Reconstruct session from CAS.
    let result = match reconstruct_session(workspace, session_id) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("spec audit: CAS reconstruction failed: {e}");
            return ExitCode::from(2);
        }
    };

    // 2. Report dangling references.
    if !result.dangling_cid_errors.is_empty() {
        eprintln!("spec audit: FAIL — dangling CID references:");
        for err in &result.dangling_cid_errors {
            eprintln!("  - {err}");
        }
        return ExitCode::from(1);
    }

    // 3. Retrieve latest spec capsule.
    let spec_capsule_cid = match latest_spec_capsule_cid(workspace) {
        Ok(Some(cid)) => cid,
        Ok(None) => {
            println!("spec audit: no spec capsule found in CAS for session {session_id}");
            println!("spec audit: PASS (no spec capsule to verify)");
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("spec audit: error reading latest spec capsule CID: {e}");
            return ExitCode::from(2);
        }
    };

    // 4. Read spec capsule body.
    let capsule_bytes = match read_spec_capsule(workspace, &spec_capsule_cid) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("spec audit: failed to read spec capsule {spec_capsule_cid}: {e}");
            return ExitCode::from(2);
        }
    };

    // 5. Compare with on-disk spec.md (if present).
    let session_dir = workspace.join("sessions").join(session_id);
    let spec_md_path = session_dir.join("spec.md");

    if spec_md_path.exists() {
        let spec_md_bytes = match std::fs::read(&spec_md_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("spec audit: failed to read on-disk spec.md: {e}");
                return ExitCode::from(2);
            }
        };

        use sha2::{Digest, Sha256};
        let capsule_hash = format!("{:x}", Sha256::digest(&capsule_bytes));
        let disk_hash = format!("{:x}", Sha256::digest(&spec_md_bytes));

        if capsule_hash == disk_hash {
            println!("spec audit: OK — spec.md sha256 matches CAS capsule {spec_capsule_cid}");
            println!("spec audit: PASS");
            ExitCode::SUCCESS
        } else {
            eprintln!("spec audit: FAIL — spec.md sha256 MISMATCH");
            eprintln!("  CAS capsule sha256: {capsule_hash}");
            eprintln!("  on-disk spec.md sha256: {disk_hash}");
            ExitCode::from(1)
        }
    } else {
        // spec.md deleted or not present — only verify CAS chain is intact.
        println!("spec audit: spec.md not found at {:?}", spec_md_path);
        println!("spec audit: OK — CAS chain verified (no on-disk spec.md to compare)");
        println!("spec audit: PASS");
        ExitCode::SUCCESS
    }
}
