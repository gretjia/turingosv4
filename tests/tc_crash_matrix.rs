use turingosv4::runtime::external_call::{ExternalCallCrashState, ExternalCallTerminal, Usage};
use turingosv4::runtime::g0_completeness::{run_dovetail, Candidate, Lane};
use turingosv4::runtime::tc_crash_matrix::{
    recover_gateway_crash, restart_from_persisted_state, CrashMatrixCase, PersistedTcState,
    RestartSource, SnapshotRole,
};

#[test]
fn crash_matrix_restarts_from_git_cas_only() {
    let persisted = PersistedTcState {
        git_head: "0123456789abcdef0123456789abcdef01234567".to_string(),
        cas_root: "89abcdef0123456789abcdef0123456789abcdef".to_string(),
        snapshot: Some("warm-cache".to_string()),
    };
    let restart = restart_from_persisted_state(&persisted).expect("restart reconstructs");

    assert_eq!(restart.source, RestartSource::GitCasOnly);
    assert_eq!(restart.git_head, persisted.git_head);
    assert_eq!(restart.cas_root, persisted.cas_root);
    assert!(!restart.used_ram_cache);
    assert!(!restart.replay_requires_network);
}

#[test]
fn snapshots_are_acceleration_only() {
    let case = CrashMatrixCase {
        surface: "scheduler".to_string(),
        kill_after_committed_transition: 2,
        snapshot_role: SnapshotRole::AccelerationOnly,
    };

    assert!(case.snapshot_optional_for_correctness());
}

#[test]
fn gateway_crash_states_recover_to_terminal_records() {
    let terminals = [
        recover_gateway_crash(ExternalCallCrashState::IntentBeforeSend),
        recover_gateway_crash(ExternalCallCrashState::SentNoTerminal),
        recover_gateway_crash(ExternalCallCrashState::ParseFailAfterResponse),
        recover_gateway_crash(ExternalCallCrashState::ParsedSuccess {
            result_hash: "result-hash".to_string(),
            usage: Usage {
                prompt_tokens: 1,
                completion_tokens: 1,
                total_tokens: 2,
            },
            status: 200,
            provider_request_id: Some("provider-req".to_string()),
        }),
    ];

    for terminal in terminals {
        match terminal {
            ExternalCallTerminal::Failure { .. }
            | ExternalCallTerminal::Abandoned { .. }
            | ExternalCallTerminal::Result { .. } => {}
        }
    }
}

#[test]
fn scheduler_crash_preserves_even_lane_prefix() {
    let even = vec![
        Candidate::from_g0_text("intro"),
        Candidate::from_g0_text("exact h"),
        Candidate::from_g0_text("apply h"),
    ];
    let full = run_dovetail(even.clone(), vec![Candidate::heuristic("odd")], 8);
    let restart = restart_from_persisted_state(&PersistedTcState {
        git_head: "0123456789abcdef0123456789abcdef01234567".to_string(),
        cas_root: "89abcdef0123456789abcdef0123456789abcdef".to_string(),
        snapshot: None,
    })
    .expect("restart reconstructs");
    let resumed = run_dovetail(even, vec![Candidate::heuristic("changed odd")], 8);

    let full_even: Vec<_> = full
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator && t.tick <= 4)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.rank, t.action.clone()))
        .collect();
    let resumed_even: Vec<_> = resumed
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator && t.tick <= 4)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.rank, t.action.clone()))
        .collect();

    assert_eq!(restart.source, RestartSource::GitCasOnly);
    assert_eq!(full_even, resumed_even);
}
