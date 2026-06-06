//! TRACE_MATRIX FC2 + FC3-N34: `turingos boot` readonly manifest verification.
//!
//! A13 exposes a CLI entry for the existing Trust Root verifier. It does not
//! create a new boot authority and does not mutate the repository.

use std::path::PathBuf;
use std::process::ExitCode;

/// TRACE_MATRIX FC2 + FC3-N34: `boot` short-help
pub(crate) const SHORT_HELP: &str = "Verify the boot Trust Root manifest";

/// TRACE_MATRIX FC2 + FC3-N34: `boot` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos boot — verify boot manifest

USAGE:
    turingos boot --verify-manifest [--repo <PATH>]

DESCRIPTION:
    Runs the existing boot Trust Root verifier against genesis_payload.toml.
    Read-only. No ChainTape append, no sequencer call, no provider/network call.

OPTIONS:
    --verify-manifest        Verify genesis_payload.toml Trust Root pins.
    --repo <PATH>            Repository root containing genesis_payload.toml.
                             Defaults to the current working directory.
    -h, --help               Print this help.
"#;

/// TRACE_MATRIX FC2 + FC3-N34: `boot` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    let mut verify_manifest = false;
    let mut repo: Option<PathBuf> = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--verify-manifest" => verify_manifest = true,
            "--repo" => {
                let Some(value) = iter.next() else {
                    eprintln!("error: --repo requires <PATH>");
                    return ExitCode::from(2);
                };
                repo = Some(PathBuf::from(value));
            }
            other => {
                eprintln!("error: unknown boot option `{other}`");
                eprintln!("Run `turingos boot --help` for usage.");
                return ExitCode::from(2);
            }
        }
    }

    if !verify_manifest {
        eprintln!("error: boot requires --verify-manifest");
        eprintln!("Run `turingos boot --help` for usage.");
        return ExitCode::from(2);
    }

    let repo_root = repo.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
    });
    match turingosv4::boot::verify_trust_root(&repo_root) {
        Ok(()) => {
            println!("BOOT-MANIFEST-VERIFIED repo={}", repo_root.display());
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}
