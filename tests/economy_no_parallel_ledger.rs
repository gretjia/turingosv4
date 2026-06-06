use std::path::{Path, PathBuf};

use turingosv4::economy::price_broadcast::price_broadcast_from_projection;
use turingosv4::economy::projections::EconomyProjection;

const HEAD: &str = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

#[test]
fn price_broadcast_is_prefix_bound_projection() {
    let projection = EconomyProjection::empty_for_tape_head(HEAD.to_string(), 7);
    let broadcast = price_broadcast_from_projection(&projection);

    assert_eq!(broadcast.derived_from_tape_head, HEAD);
    assert_eq!(broadcast.last_applied_logical_t, 7);
    assert_eq!(broadcast.price_index, projection.price_index);
}

#[test]
fn a09_sources_do_not_create_parallel_market_tape() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    collect_rs_files(&workspace.join("src/economy"), &mut files);

    let forbidden = [
        concat!("market_tape_", "shared"),
        concat!("Market", "Tape"),
        "pub mod market_tape",
        "pub struct ParallelLedger",
        "pub struct MarketLedger",
    ];
    for file in files {
        let file_name = file.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if file_name == "ledger.rs" || file_name == "escrow_vault.rs" {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("read source");
        for pattern in forbidden {
            assert!(
                !text.contains(pattern),
                "A09 must not create a parallel market ledger: {} contains `{}`",
                file.display(),
                pattern
            );
        }
    }
}
