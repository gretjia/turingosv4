//! TRACE_MATRIX FC2 boot + FC3 readonly guard: TC boot trust-root manifest checks.

use std::path::PathBuf;
use std::process::ExitCode;

use turingosv4::runtime::boot_trust_root_manifest::{
    read_tc_boot_manifest, verify_tc_boot_constitution_hash, verify_tc_boot_manifest,
    verify_tc_boot_predicates,
};

pub(crate) const SHORT_HELP: &str = "Verify TC boot trust-root manifest gates";

fn print_help() {
    println!("turingos boot — TC boot trust-root manifest verifier");
    println!();
    println!("USAGE:");
    println!("    turingos boot --verify-manifest <PATH> [--repo-root <PATH>]");
    println!("    turingos boot --verify-constitution-hash <PATH> [--repo-root <PATH>]");
    println!("    turingos boot --verify-predicates <PATH>");
}

pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return ExitCode::SUCCESS;
    }

    let mut mode: Option<&str> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--verify-manifest" | "--verify-constitution-hash" | "--verify-predicates" => {
                if mode.is_some() {
                    eprintln!("turingos boot: choose exactly one verify mode");
                    return ExitCode::from(2);
                }
                let flag = args[i].as_str();
                let Some(value) = args.get(i + 1) else {
                    eprintln!("turingos boot: {flag} requires a manifest path");
                    return ExitCode::from(2);
                };
                mode = Some(flag);
                manifest_path = Some(PathBuf::from(value));
                i += 2;
            }
            "--repo-root" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("turingos boot: --repo-root requires a path");
                    return ExitCode::from(2);
                };
                repo_root = PathBuf::from(value);
                i += 2;
            }
            other => {
                eprintln!("turingos boot: unknown argument: {other}");
                return ExitCode::from(2);
            }
        }
    }

    let Some(mode) = mode else {
        print_help();
        return ExitCode::from(2);
    };
    let manifest_path = manifest_path.expect("mode parser sets manifest path");
    let manifest = match read_tc_boot_manifest(&manifest_path) {
        Ok(manifest) => manifest,
        Err(err) => {
            eprintln!("TC-BOOT-MANIFEST-VETO: {err}");
            return ExitCode::from(1);
        }
    };

    let result = match mode {
        "--verify-manifest" => verify_tc_boot_manifest(&repo_root, &manifest),
        "--verify-constitution-hash" => verify_tc_boot_constitution_hash(&repo_root, &manifest),
        "--verify-predicates" => verify_tc_boot_predicates(&manifest),
        _ => unreachable!("mode parser only accepts known modes"),
    };

    match result {
        Ok(()) => {
            println!("TC-BOOT-MANIFEST-PASS {mode}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("TC-BOOT-MANIFEST-VETO: {err}");
            ExitCode::from(1)
        }
    }
}
