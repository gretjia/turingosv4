//! TB-G G5 — 7-action menu and parser gates.

use turingosv4::sdk::prompt::build_agent_prompt;
use turingosv4::sdk::protocol::parse_agent_output;

#[test]
fn sg_g5_7_prompt_renders_seven_action_menu() {
    let prompt = build_agent_prompt("", "", "", &[], &[], "", "", "", "", "");
    for tool in [
        "\"tool\":\"step\"",
        "\"tool\":\"verify_peer\"",
        "\"tool\":\"search\"",
        "\"tool\":\"invest\"",
        "\"tool\":\"post\"",
        "\"tool\":\"challenge_node\"",
        "\"tool\":\"bid_task\"",
        "\"tool\":\"abstain\"",
    ] {
        assert!(
            prompt.contains(tool),
            "prompt missing G5 tool schema: {tool}"
        );
    }
}

#[test]
fn sg_g5_8_parser_accepts_new_g5_actions_without_schema_repair() {
    for raw in [
        r#"<action>{"tool":"challenge_node","node":"worktx-Agent_0-1","payload":"counterexample"}</action>"#,
        r#"<action>{"tool":"bid_task","amount":1000,"direction":"long"}</action>"#,
        r#"<action>{"tool":"abstain","payload":"no perceived edge"}</action>"#,
    ] {
        let action = parse_agent_output(raw).expect("G5 action should parse");
        assert!(
            matches!(
                action.tool.as_str(),
                "challenge_node" | "bid_task" | "abstain"
            ),
            "unexpected tool parsed: {}",
            action.tool
        );
    }
}

#[test]
fn sg_g5_9_abstain_schema_does_not_require_payload_or_amount() {
    let action = parse_agent_output(r#"<action>{"tool":"abstain"}</action>"#)
        .expect("minimal abstain parses");
    assert_eq!(action.tool, "abstain");
    assert!(action.payload.is_none());
    assert!(action.amount.is_none());
}
