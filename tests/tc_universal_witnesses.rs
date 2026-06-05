use turingosv4::runtime::tc_universal_witness::{
    resume_brainfuck_run, resume_minsky_run, run_brainfuck, run_minsky, verify_brainfuck_trace,
    verify_minsky_trace, BrainfuckRun, MinskyCounter, MinskyInstruction,
};

fn minsky_add_program() -> Vec<MinskyInstruction> {
    vec![
        MinskyInstruction::DecJz {
            counter: MinskyCounter::A,
            dec_next: 1,
            zero_next: 3,
        },
        MinskyInstruction::Inc {
            counter: MinskyCounter::B,
            next: 0,
        },
        MinskyInstruction::Halt,
        MinskyInstruction::Halt,
    ]
}

#[test]
fn minsky_addition_works_and_replay_verifies() {
    let program = minsky_add_program();
    let run = run_minsky(&program, [3, 2], 16).expect("addition should halt");

    assert!(run.final_state.halted);
    assert_eq!(run.final_state.counters, [0, 5]);
    assert_eq!(run.steps.len(), 8);

    let verified = verify_minsky_trace(&program, [3, 2], &run.steps)
        .expect("trace should replay byte-identically");
    assert_eq!(verified, run);
}

#[test]
fn minsky_repeated_addition_copy_witness_works_and_replay_verifies() {
    let program = vec![
        MinskyInstruction::DecJz {
            counter: MinskyCounter::A,
            dec_next: 1,
            zero_next: 5,
        },
        MinskyInstruction::Inc {
            counter: MinskyCounter::B,
            next: 2,
        },
        MinskyInstruction::Inc {
            counter: MinskyCounter::B,
            next: 3,
        },
        MinskyInstruction::Inc {
            counter: MinskyCounter::B,
            next: 0,
        },
        MinskyInstruction::Halt,
        MinskyInstruction::Halt,
    ];
    let run = run_minsky(&program, [4, 1], 32).expect("repeated addition should halt");

    assert_eq!(run.final_state.counters, [0, 13]);
    assert!(run.final_state.halted);
    assert_eq!(
        verify_minsky_trace(&program, [4, 1], &run.steps).expect("replay should verify"),
        run
    );
}

#[test]
fn minsky_zero_branch_behavior_is_covered() {
    let program = vec![
        MinskyInstruction::DecJz {
            counter: MinskyCounter::A,
            dec_next: 1,
            zero_next: 2,
        },
        MinskyInstruction::Inc {
            counter: MinskyCounter::B,
            next: 3,
        },
        MinskyInstruction::Inc {
            counter: MinskyCounter::B,
            next: 3,
        },
        MinskyInstruction::Halt,
    ];
    let run = run_minsky(&program, [0, 7], 8).expect("zero branch should halt");

    assert_eq!(run.steps[0].pc, 0);
    assert_eq!(run.steps[1].pc, 2);
    assert_eq!(run.final_state.counters, [0, 8]);
}

#[test]
fn minsky_capped_non_halting_run_can_resume_deterministically() {
    let program = vec![MinskyInstruction::Inc {
        counter: MinskyCounter::A,
        next: 0,
    }];

    let capped = run_minsky(&program, [0, 0], 3).expect("bounded prefix should run");
    assert!(!capped.final_state.halted);
    assert_eq!(capped.final_state.counters, [3, 0]);

    let resumed = resume_minsky_run(&program, &capped, 4).expect("resume should run");
    let direct = run_minsky(&program, [0, 0], 7).expect("direct bounded run should match");
    assert_eq!(resumed, direct);
}

#[test]
fn tampering_with_a_minsky_trace_fails_verification() {
    let program = minsky_add_program();
    let mut run = run_minsky(&program, [2, 0], 16).expect("addition should halt");
    run.steps[1].counters = [99, 99];

    assert!(verify_minsky_trace(&program, [2, 0], &run.steps).is_err());
}

#[test]
fn brainfuck_loop_copy_output_program_replays_identically() {
    let run = run_brainfuck("+++[>+<-]>.", 64).expect("copy/output program should halt");

    assert!(run.final_state.halted);
    assert_eq!(run.output, vec![3]);
    assert_eq!(run.final_state.cell(0), 0);
    assert_eq!(run.final_state.cell(1), 3);

    let verified = verify_brainfuck_trace("+++[>+<-]>.", &run.steps)
        .expect("Brainfuck trace should replay byte-identically");
    assert_eq!(verified, run);
}

#[test]
fn brainfuck_tampering_fails_verification() {
    let mut run = run_brainfuck("++>+.", 16).expect("program should halt");
    run.steps[0].touched_value = run.steps[0].touched_value.wrapping_add(1);

    assert!(verify_brainfuck_trace("++>+.", &run.steps).is_err());
}

#[test]
fn brainfuck_capped_non_halting_loop_resume_is_deterministic() {
    let capped = run_brainfuck("+[]", 4).expect("bounded non-halting prefix should run");
    assert!(!capped.final_state.halted);
    assert_eq!(capped.final_state.pc, 2);

    let resumed: BrainfuckRun =
        resume_brainfuck_run("+[]", &capped, 6).expect("resume should continue");
    let direct = run_brainfuck("+[]", 10).expect("direct bounded run should match");
    assert_eq!(resumed, direct);
}
