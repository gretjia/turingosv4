//! `turingos init` — initialize a new TuringOS workspace directory.
//!
//! TISR Phase 6.0/6.1 alpha first slice. Per §8 packet
//! `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`:
//! pure filesystem operation; Class 1.
//!
//! 0 sequencer call; 0 typed_tx; 0 CAS write; 0 ChainTape advance.
//! Creates: project_dir/{runtime_repo,cas}/, genesis_payload.toml,
//! agent_pubkeys.json (empty placeholder).

use std::fs;
use std::path::PathBuf;

use crate::cli::args::{InitArgs, Template};
use crate::cli::error::{CliResult, TuringosCliError};
use crate::cli::templates::{GENESIS_MULTI_AGENT, GENESIS_POLYMARKET, GENESIS_PROOF};

/// Empty `agent_pubkeys.json` placeholder content.
const AGENT_PUBKEYS_PLACEHOLDER: &str = "{}\n";

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — `turingos init` subcommand handler.
/// Run `turingos init`.
/// Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md` §2 (FC2-N16 init flow).
pub fn run(args: InitArgs) -> CliResult<()> {
    let project_dir = PathBuf::from(&args.project);

    if project_dir.exists() && !args.force {
        return Err(TuringosCliError::ProjectExists(args.project.clone()));
    }

    fs::create_dir_all(&project_dir)?;
    fs::create_dir_all(project_dir.join("runtime_repo"))?;
    fs::create_dir_all(project_dir.join("cas"))?;

    let genesis_template = match args.template {
        Template::Proof => GENESIS_PROOF,
        Template::Polymarket => GENESIS_POLYMARKET,
        Template::MultiAgent => GENESIS_MULTI_AGENT,
    };

    let genesis_path = project_dir.join("genesis_payload.toml");
    let agent_pubkeys_path = project_dir.join("agent_pubkeys.json");

    // Write scaffold files. `--force` allows overwrite of these specific files;
    // it never deletes other user content the user may have placed in the dir.
    fs::write(&genesis_path, genesis_template)?;
    fs::write(&agent_pubkeys_path, AGENT_PUBKEYS_PLACEHOLDER)?;

    let template_name = match args.template {
        Template::Proof => "proof",
        Template::Polymarket => "polymarket",
        Template::MultiAgent => "multi-agent",
    };

    println!(
        "Initialized TuringOS workspace at {} (template: {})",
        project_dir.display(),
        template_name,
    );
    println!();
    println!("Scaffold files created:");
    println!("  {}", project_dir.join("runtime_repo").display());
    println!("  {}", project_dir.join("cas").display());
    println!("  {}", genesis_path.display());
    println!("  {}", agent_pubkeys_path.display());
    println!();
    println!("Next steps (Phase 6.1+ subcommands; not yet implemented):");
    println!("  cd {}", project_dir.display());
    println!("  # Edit genesis_payload.toml and agent_pubkeys.json,");
    println!("  # then: turingos agent deploy ... / turingos task open ...");
    println!();
    println!("Phase 6.0 baseline: use experiments/minif2f_v4/src/bin/lean_market.rs");
    println!("for the established TB-10 workflow until later subcommands ship.");

    Ok(())
}
