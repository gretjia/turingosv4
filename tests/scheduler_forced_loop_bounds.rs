use turingosv4::runtime::agent_scheduler::{
    forced_loop_stop_reason, ForcedLoopBounds, ForcedLoopState, ForcedLoopStop,
};

#[test]
fn forced_loop_stops_at_iteration_token_and_wall_clock_bounds() {
    let bounds = ForcedLoopBounds {
        max_iterations: 3,
        max_tokens: 100,
        max_wall_clock_ms: 1_000,
    };

    assert_eq!(
        forced_loop_stop_reason(
            &bounds,
            &ForcedLoopState {
                iterations: 2,
                tokens_used: 99,
                wall_clock_ms: 999,
            },
        ),
        None
    );
    assert_eq!(
        forced_loop_stop_reason(
            &bounds,
            &ForcedLoopState {
                iterations: 3,
                tokens_used: 99,
                wall_clock_ms: 999,
            },
        ),
        Some(ForcedLoopStop::MaxIterations)
    );
    assert_eq!(
        forced_loop_stop_reason(
            &bounds,
            &ForcedLoopState {
                iterations: 2,
                tokens_used: 100,
                wall_clock_ms: 999,
            },
        ),
        Some(ForcedLoopStop::MaxTokens)
    );
    assert_eq!(
        forced_loop_stop_reason(
            &bounds,
            &ForcedLoopState {
                iterations: 2,
                tokens_used: 99,
                wall_clock_ms: 1_000,
            },
        ),
        Some(ForcedLoopStop::MaxWallClock)
    );
}

#[test]
fn forced_loop_bounds_reject_zero_limits() {
    let err = ForcedLoopBounds {
        max_iterations: 0,
        max_tokens: 100,
        max_wall_clock_ms: 1_000,
    }
    .validate()
    .expect_err("zero iterations");
    assert!(err.to_string().contains("max_iterations_must_be_positive"));

    let err = ForcedLoopBounds {
        max_iterations: 1,
        max_tokens: 0,
        max_wall_clock_ms: 1_000,
    }
    .validate()
    .expect_err("zero tokens");
    assert!(err.to_string().contains("max_tokens_must_be_positive"));

    let err = ForcedLoopBounds {
        max_iterations: 1,
        max_tokens: 100,
        max_wall_clock_ms: 0,
    }
    .validate()
    .expect_err("zero wall clock");
    assert!(err
        .to_string()
        .contains("max_wall_clock_ms_must_be_positive"));
}
