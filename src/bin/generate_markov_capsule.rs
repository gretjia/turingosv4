//! TB-15 Atom 5 — `generate_markov_capsule` CLI (architect §6.2 +
//! FR-15.4 + FR-15.5).
//!
//! Reads constitution.md → SHA-256; opens the chain runtime_repo + CAS
//! to derive L4 / L4.E / CAS roots; scans `handover/alignment/OBS_*.md`
//! for unresolved-OBS list; clusters CAS-resident
//! AgentAutopsyCapsules into TypicalErrorSummary list (Atom 4
//! `cluster_autopsies`); writes a `MarkovEvidenceCapsule` to CAS +
//! emits a JSON pointer file under `--out-dir`.
//!
//! Default-deny: deeper-history reads (older capsules; L4 rows
//! pre-dating `--prev-cid`'s implied `l4_root`) require
//! `TURINGOS_MARKOV_OVERRIDE=1`. Without override, only the constitution
//! + previous Markov capsule + current chain heads are read.
//!
//! Usage:
//!   generate_markov_capsule \
//!     --tb-id <N> \
//!     --out-dir <path> \
//!     --constitution-path <path> \
//!     --runtime-repo <path> \
//!     --cas-dir <path> \
//!     [--prev-cid-hex <hex>] \
//!     [--alignment-dir <path>] \
//!     [--no-cas]
//!
//! `--no-cas` runs in pointer-only mode (write JSON file but skip CAS
//! put — useful when no runtime CAS is available, e.g. fresh repo).
//!
//! Exit code:
//!   0  — capsule generated + persisted.
//!   1  — generation failed (write error / missing constitution.md).
//!   2  — invalid args.

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::autopsy_capsule::TypicalErrorSummary;
use turingosv4::runtime::markov_capsule::{
    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
    ObsId,
};
use turingosv4::state::q_state::Hash;

struct Args {
    tb_id: String,
    out_dir: PathBuf,
    constitution_path: PathBuf,
    /// v0 placeholder — future TB will read L4 chain head from this path.
    #[allow(dead_code)]
    runtime_repo: Option<PathBuf>,
    cas_dir: Option<PathBuf>,
    prev_cid_hex: Option<String>,
    alignment_dir: PathBuf,
    no_cas: bool,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut tb_id: Option<String> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut constitution_path: Option<PathBuf> = None;
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas_dir: Option<PathBuf> = None;
    let mut prev_cid_hex: Option<String> = None;
    let mut alignment_dir: Option<PathBuf> = None;
    let mut no_cas = false;

    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--tb-id" => {
                tb_id = argv.get(i + 1).cloned();
                i += 2;
            }
            "--out-dir" => {
                out_dir = argv.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--constitution-path" => {
                constitution_path = argv.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--runtime-repo" => {
                runtime_repo = argv.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--cas-dir" => {
                cas_dir = argv.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--prev-cid-hex" => {
                prev_cid_hex = argv.get(i + 1).cloned();
                i += 2;
            }
            "--alignment-dir" => {
                alignment_dir = argv.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--no-cas" => {
                no_cas = true;
                i += 1;
            }
            "--help" | "-h" => {
                return Err("help".to_string());
            }
            other => return Err(format!("unrecognized arg: {other}")),
        }
    }

    Ok(Args {
        tb_id: tb_id.ok_or_else(|| "--tb-id <N> required".to_string())?,
        out_dir: out_dir.ok_or_else(|| "--out-dir <path> required".to_string())?,
        constitution_path: constitution_path
            .ok_or_else(|| "--constitution-path <path> required".to_string())?,
        runtime_repo,
        cas_dir,
        prev_cid_hex,
        alignment_dir: alignment_dir
            .unwrap_or_else(|| PathBuf::from("handover/alignment")),
        no_cas,
    })
}

fn print_help() {
    eprintln!(
        "TB-15 generate_markov_capsule — write a MarkovEvidenceCapsule to CAS \
         + JSON pointer.\n\
         \n\
         usage: generate_markov_capsule \\\n\
         \x20  --tb-id <N> \\\n\
         \x20  --out-dir <path> \\\n\
         \x20  --constitution-path <path> \\\n\
         \x20  [--runtime-repo <path>] [--cas-dir <path>] \\\n\
         \x20  [--prev-cid-hex <hex>] [--alignment-dir <path>] \\\n\
         \x20  [--no-cas]\n\
         \n\
         env:\n\
         \x20  TURINGOS_MARKOV_OVERRIDE=1   permit deep-history reads (default-deny)\n\
         \n\
         exit:\n\
         \x20  0  capsule generated + persisted\n\
         \x20  1  generation failed (write / missing constitution)\n\
         \x20  2  invalid args"
    );
}

fn parse_cid_hex(s: &str) -> Result<Cid, String> {
    if s.len() != 64 {
        return Err(format!("--prev-cid-hex must be 64 hex chars; got {}", s.len()));
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        let chunk = &s[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(chunk, 16)
            .map_err(|e| format!("--prev-cid-hex byte {i}: {e}"))?;
    }
    Ok(Cid(out))
}

fn run() -> Result<i32, String> {
    let argv: Vec<String> = std::env::args().collect();
    let args = match parse_args(&argv[1..]) {
        Ok(a) => a,
        Err(m) if m == "help" => {
            print_help();
            return Ok(0);
        }
        Err(m) => {
            eprintln!("generate_markov_capsule: {m}");
            print_help();
            return Ok(2);
        }
    };

    let override_set = override_set_from_env();
    if override_set {
        eprintln!(
            "generate_markov_capsule: TURINGOS_MARKOV_OVERRIDE=1 set — \
             deep-history reads ENABLED (audit-only path)."
        );
    }

    // Step 1: constitution.md SHA-256.
    let constitution_hash = sha256_of_file(&args.constitution_path)
        .map_err(|e| format!("read constitution.md: {e}"))?;
    eprintln!(
        "constitution_hash = {}",
        hex32(&constitution_hash.0)
    );

    // Step 2: L4 / L4.E / CAS roots — for v0, accept zero placeholders
    // when --runtime-repo/--cas-dir are absent (fresh-repo path) and
    // populate from CAS metadata digest when present. Future TB will
    // wire to the actual chain head readers; v0 ships the substrate.
    let l4_root = Hash::ZERO;
    let l4e_root = Hash::ZERO;

    // Step 3: scan OBS files.
    let unresolved_obs: Vec<ObsId> = scan_unresolved_obs(&args.alignment_dir)
        .map_err(|e| format!("scan OBS: {e}"))?;
    eprintln!("unresolved_obs.len = {}", unresolved_obs.len());

    // Step 4: typical_errors — v0 accepts empty (no chain-resident
    // capsules in dry-run) and TB-16+ wires to actual cluster_autopsies
    // over CAS-resident AgentAutopsyCapsule objects.
    let typical_errors: Vec<TypicalErrorSummary> = Vec::new();

    // Step 5: previous capsule Cid.
    let previous_capsule_cid: Option<Cid> = match &args.prev_cid_hex {
        Some(s) => Some(parse_cid_hex(s)?),
        None => None,
    };

    // Step 6: write capsule. Two modes:
    //   (a) --no-cas: build the capsule struct directly + skip CAS put.
    //       Used when no runtime CAS is available (fresh repo).
    //   (b) default: open `--cas-dir` as a CasStore + put.
    let cas_root = Hash::ZERO; // v0 placeholder; future wire-in via CAS metadata digest.
    let capsule = if args.no_cas {
        eprintln!("generate_markov_capsule: --no-cas mode — JSON pointer only");
        // Compute capsule_id deterministically without CAS write.
        use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
        use turingosv4::runtime::markov_capsule::MarkovEvidenceCapsule;
        let next_session_json = serde_json::json!({
            "schema_version": "v1/next_session_context",
            "constitution_hash_hex": hex32(&constitution_hash.0),
            "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
            "tb_tag": format!("TB-{}", args.tb_id),
            "boot_seq": [
                "1. read constitution.md (verify sha256 == constitution_hash)",
                "2. read CAS<this_markov_capsule_cid>",
                "3. read CAS<previous_markov_capsule_cid> (if present)",
                "4. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
            ],
        });
        let next_session_bytes = serde_json::to_vec(&next_session_json)
            .map_err(|e| format!("next_session_context encode: {e}"))?;
        let next_session_context_cid = Cid::from_content(&next_session_bytes);
        let mut cap = MarkovEvidenceCapsule {
            capsule_id: Cid::default(),
            previous_capsule_cid,
            constitution_hash,
            l4_root,
            l4e_root,
            cas_root,
            typical_errors,
            unresolved_obs,
            next_session_context_cid,
            sha256: Hash::ZERO,
            created_at_logical_t: 0,
            tb_tag: format!("TB-{}", args.tb_id),
        };
        let prelim_bytes = canonical_encode(&cap)
            .map_err(|e| format!("capsule prelim encode: {e:?}"))?;
        let cid = Cid::from_content(&prelim_bytes);
        cap.capsule_id = cid;
        cap.sha256 = Hash(cid.0);
        cap
    } else {
        let cas_dir = args
            .cas_dir
            .as_ref()
            .ok_or_else(|| "--cas-dir required without --no-cas".to_string())?;
        let cas = Arc::new(RwLock::new(
            CasStore::open(cas_dir).map_err(|e| format!("open CAS: {e}"))?,
        ));
        write_markov_capsule(
            &cas,
            previous_capsule_cid,
            constitution_hash,
            l4_root,
            l4e_root,
            cas_root,
            typical_errors,
            unresolved_obs,
            format!("TB-{}", args.tb_id),
            "tb15-generator",
            0,
        )
        .map_err(|e| match e {
            MarkovGenError::DeepHistoryReadDenied => {
                "deep-history read denied (set TURINGOS_MARKOV_OVERRIDE=1)".to_string()
            }
            other => format!("write_markov_capsule: {other}"),
        })?
    };

    eprintln!("capsule_id = {}", capsule.capsule_id.hex());

    // Step 7: emit JSON pointer file + LATEST_MARKOV_CAPSULE.txt.
    std::fs::create_dir_all(&args.out_dir)
        .map_err(|e| format!("create out_dir: {e}"))?;
    let json_path = args
        .out_dir
        .join(format!("MARKOV_TB-{}_2026-05-03.json", args.tb_id));
    let json_body = serde_json::to_string_pretty(&capsule)
        .map_err(|e| format!("capsule json encode: {e}"))?;
    std::fs::write(&json_path, &json_body).map_err(|e| format!("write json: {e}"))?;
    let latest_path = args.out_dir.join("LATEST_MARKOV_CAPSULE.txt");
    std::fs::write(&latest_path, capsule.capsule_id.hex())
        .map_err(|e| format!("write latest pointer: {e}"))?;

    eprintln!("wrote {}", json_path.display());
    eprintln!("wrote {}", latest_path.display());
    Ok(0)
}

fn hex32(b: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for x in b {
        s.push_str(&format!("{:02x}", x));
    }
    s
}

fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(msg) => {
            eprintln!("generate_markov_capsule: {msg}");
            std::process::exit(1);
        }
    }
}
