//! Phase Z — WriteTool tools_other contract tests.
//!
//! Constitutional basis: Art. IV mermaid explicit form
//!   `wtool(output | tape_t, HEAD_t, tools_other)`.
//!
//! These tests pin down the `write_with_tools` trait method added in the
//! `phase-z-wtool-tools` branch. The contract is:
//!   1. If every named tool in `tools_other` is mounted on the bus,
//!      the call delegates to `write` and succeeds.
//!   2. If any named tool is missing, the call returns `Err` without
//!      touching tape.
//!   3. The pre-existing `write` method remains the unblessed free-topology
//!      path (Law 1) and must not regress.
//!
//! Bus internals (append_internal fan-out over `bus.tools`) are not
//! modified — `write_with_tools` is contract clarity, not behavior change.
//! See the docstring on `WriteTool::write_with_tools` for details.
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::tools::search::SearchTool;
use turingosv4::sdk::write_tool::{DefaultWriteTool, WriteTool};

fn make_bus() -> TuringBus {
    TuringBus::new(Kernel::new(), BusConfig::default())
}

#[test]
fn write_with_tools_succeeds_with_mounted_tool() {
    let mut bus = make_bus();
    // Mount SearchTool — manifest() == "search".
    bus.mount_tool(Box::new(SearchTool::new(vec![], 8)));

    let wt = DefaultWriteTool;
    let res = wt.write_with_tools(
        &mut bus,
        "A0",
        "hello tape (explicit tools_other)",
        None,
        None,
        &["search"],
    );

    assert!(
        res.is_ok(),
        "write_with_tools should succeed when requested tool is mounted: {:?}",
        res.err()
    );
    match res.unwrap() {
        BusResult::Appended { node_id } => {
            assert!(!node_id.is_empty(), "appended node should have non-empty id");
        }
        other => panic!("expected Appended, got {:?}", other),
    }
}

#[test]
fn write_with_tools_fails_when_tool_missing() {
    // Bus with NO tools mounted.
    let mut bus = make_bus();

    let wt = DefaultWriteTool;
    let res = wt.write_with_tools(
        &mut bus,
        "A0",
        "payload that should never land",
        None,
        None,
        &["wallet"], // WalletTool not mounted → must reject.
    );

    assert!(
        res.is_err(),
        "write_with_tools should Err when tool is missing, got Ok({:?})",
        res.ok()
    );
    let msg = res.unwrap_err();
    assert!(
        msg.contains("wallet"),
        "error message should name the missing tool, got: {}",
        msg
    );
    assert!(
        msg.contains("not mounted"),
        "error message should state 'not mounted', got: {}",
        msg
    );
}

#[test]
fn default_write_tool_backward_compat() {
    // The original `write` method must still work with no tools_other plumbing.
    let mut bus = make_bus();
    let wt = DefaultWriteTool;
    let res = wt.write(&mut bus, "A0", "legacy unblessed path", None, None);
    assert!(
        res.is_ok(),
        "legacy write() path regressed: {:?}",
        res.err()
    );
    match res.unwrap() {
        BusResult::Appended { node_id } => assert!(!node_id.is_empty()),
        other => panic!("expected Appended, got {:?}", other),
    }
}
