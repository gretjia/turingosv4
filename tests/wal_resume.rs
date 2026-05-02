// Phase 1 integration test: simulate crash + WAL resume.
//
// The premise of WAL is that Q_t survives a process exit between transactions.
// This test simulates that by:
//   1. Creating a Bus with a WAL path
//   2. Initialising agents (writes RunStart event)
//   3. Appending several nodes (writes Node + Append event each)
//   4. Dropping the Bus (simulates crash — no halt, no graceful shutdown)
//   5. Re-opening Bus::with_wal_path on the same file
//   6. Asserting the new bus has all nodes + the RunStart event in its tape/ledger
//
// If this passes, "tape persistence across process restart" is proven for the
// no-halt case. The resumed bus can continue appending; we don't yet test
// settlement-then-resume because halt_and_settle is terminal in current code.

use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::tools::wallet::WalletTool;

fn make_config() -> BusConfig {
    BusConfig {
        max_payload_chars: 1200,
        max_payload_lines: 18,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![],
    }
}

fn tmp_wal_path(name: &str) -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("turingos_wal_resume_{}_{}.jsonl",
                   name, std::process::id()));
    let _ = std::fs::remove_file(&p);
    p
}

#[test]
fn wal_persists_appends_across_bus_drop() {
    let path = tmp_wal_path("persists");

    // Phase A: write nodes through the bus, then drop.
    {
        let mut bus = TuringBus::with_wal_path(Kernel::new(), make_config(), &path)
            .expect("first open");
        bus.mount_tool(Box::new(WalletTool::new()));
        bus.init(&["A0".into(), "A1".into()]);

        for i in 0..5 {
            let payload = format!("step {}", i);
            let r = bus.append("A0", &payload, None).unwrap();
            assert!(matches!(r, BusResult::Appended { .. }));
        }

        assert_eq!(bus.kernel.tape.time_arrow().len(), 5);
        // Drop — simulates ungraceful exit.
    }

    // Phase B: reopen against same WAL path; replay should rebuild state.
    {
        let bus2 = TuringBus::with_wal_path(Kernel::new(), make_config(), &path)
            .expect("second open / replay");
        let arrow = bus2.kernel.tape.time_arrow();
        assert_eq!(arrow.len(), 5, "all 5 nodes must replay");
        for (i, nid) in arrow.iter().enumerate() {
            let node = bus2.kernel.tape.get(nid).expect("node present");
            assert_eq!(node.payload, format!("step {}", i));
            assert_eq!(node.author, "A0");
        }
        // Ledger should have replayed RunStart + 5 Append events.
        assert!(bus2.ledger.len() >= 6,
                "ledger replayed events should be ≥ 6, got {}", bus2.ledger.len());
        // Hash chain on replayed ledger must verify.
        bus2.ledger.verify().expect("recomputed ledger hash chain verifies");
    }

    let _ = std::fs::remove_file(&path);
}

#[test]
fn wal_empty_file_yields_fresh_bus() {
    let path = tmp_wal_path("empty");
    let bus = TuringBus::with_wal_path(Kernel::new(), make_config(), &path)
        .expect("open empty");
    assert_eq!(bus.kernel.tape.time_arrow().len(), 0);
    assert_eq!(bus.ledger.len(), 0);
    let _ = std::fs::remove_file(&path);
}
