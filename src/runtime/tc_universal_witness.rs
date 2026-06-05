use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MinskyCounter {
    A,
    B,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MinskyInstruction {
    Inc {
        counter: MinskyCounter,
        next: usize,
    },
    DecJz {
        counter: MinskyCounter,
        dec_next: usize,
        zero_next: usize,
    },
    Halt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MinskyStepRecord {
    pub pc: usize,
    pub counters: [u64; 2],
    pub instruction: MinskyInstruction,
    pub prev_hash: String,
    pub step_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MinskyState {
    pub pc: usize,
    pub counters: [u64; 2],
    pub halted: bool,
    pub prev_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MinskyRun {
    pub initial_counters: [u64; 2],
    pub steps: Vec<MinskyStepRecord>,
    pub final_state: MinskyState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrainfuckStepRecord {
    pub pc: usize,
    pub data_ptr: usize,
    pub instruction: char,
    pub touched_cell: usize,
    pub touched_value: u8,
    pub output_hash: String,
    pub prev_hash: String,
    pub step_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrainfuckState {
    pub pc: usize,
    pub data_ptr: usize,
    pub cells: Vec<u8>,
    pub halted: bool,
    pub prev_hash: String,
}

impl BrainfuckState {
    pub fn cell(&self, idx: usize) -> u8 {
        self.cells.get(idx).copied().unwrap_or(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrainfuckRun {
    pub steps: Vec<BrainfuckStepRecord>,
    pub output: Vec<u8>,
    pub final_state: BrainfuckState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessError {
    InvalidMinskyPc(usize),
    MinskyCounterOverflow(MinskyCounter),
    TraceMismatch { index: usize },
    InvalidBrainfuckPc(usize),
    BrainfuckPointerUnderflow,
    BrainfuckBracketMismatch,
}

impl std::fmt::Display for WitnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WitnessError::InvalidMinskyPc(pc) => write!(f, "invalid Minsky pc {pc}"),
            WitnessError::MinskyCounterOverflow(counter) => {
                write!(f, "Minsky counter overflow: {counter:?}")
            }
            WitnessError::TraceMismatch { index } => write!(f, "trace mismatch at step {index}"),
            WitnessError::InvalidBrainfuckPc(pc) => write!(f, "invalid Brainfuck pc {pc}"),
            WitnessError::BrainfuckPointerUnderflow => write!(f, "Brainfuck pointer underflow"),
            WitnessError::BrainfuckBracketMismatch => write!(f, "Brainfuck bracket mismatch"),
        }
    }
}

impl std::error::Error for WitnessError {}

pub fn run_minsky(
    program: &[MinskyInstruction],
    initial_counters: [u64; 2],
    max_steps: usize,
) -> Result<MinskyRun, WitnessError> {
    let state = MinskyState {
        pc: 0,
        counters: initial_counters,
        halted: false,
        prev_hash: minsky_genesis_hash(program, initial_counters),
    };
    append_minsky_steps(program, initial_counters, Vec::new(), state, max_steps)
}

pub fn resume_minsky_run(
    program: &[MinskyInstruction],
    previous: &MinskyRun,
    max_steps: usize,
) -> Result<MinskyRun, WitnessError> {
    append_minsky_steps(
        program,
        previous.initial_counters,
        previous.steps.clone(),
        previous.final_state.clone(),
        max_steps,
    )
}

pub fn verify_minsky_trace(
    program: &[MinskyInstruction],
    initial_counters: [u64; 2],
    trace: &[MinskyStepRecord],
) -> Result<MinskyRun, WitnessError> {
    let replay = run_minsky(program, initial_counters, trace.len())?;
    compare_minsky_steps(trace, &replay.steps)?;
    Ok(replay)
}

pub fn run_brainfuck(program: &str, max_steps: usize) -> Result<BrainfuckRun, WitnessError> {
    let code = brainfuck_code(program);
    let jumps = brainfuck_jumps(&code)?;
    let state = BrainfuckState {
        pc: 0,
        data_ptr: 0,
        cells: vec![0],
        halted: code.is_empty(),
        prev_hash: brainfuck_genesis_hash(&code),
    };
    append_brainfuck_steps(&code, &jumps, Vec::new(), Vec::new(), state, max_steps)
}

pub fn resume_brainfuck_run(
    program: &str,
    previous: &BrainfuckRun,
    max_steps: usize,
) -> Result<BrainfuckRun, WitnessError> {
    let code = brainfuck_code(program);
    let jumps = brainfuck_jumps(&code)?;
    append_brainfuck_steps(
        &code,
        &jumps,
        previous.steps.clone(),
        previous.output.clone(),
        previous.final_state.clone(),
        max_steps,
    )
}

pub fn verify_brainfuck_trace(
    program: &str,
    trace: &[BrainfuckStepRecord],
) -> Result<BrainfuckRun, WitnessError> {
    let replay = run_brainfuck(program, trace.len())?;
    compare_brainfuck_steps(trace, &replay.steps)?;
    Ok(replay)
}

fn append_minsky_steps(
    program: &[MinskyInstruction],
    initial_counters: [u64; 2],
    mut steps: Vec<MinskyStepRecord>,
    mut state: MinskyState,
    max_steps: usize,
) -> Result<MinskyRun, WitnessError> {
    for _ in 0..max_steps {
        if state.halted {
            break;
        }
        let instruction = program
            .get(state.pc)
            .ok_or(WitnessError::InvalidMinskyPc(state.pc))?
            .clone();
        let step = minsky_step_record(
            state.pc,
            state.counters,
            instruction.clone(),
            &state.prev_hash,
        );
        state.prev_hash = step.step_hash.clone();
        steps.push(step);

        match instruction {
            MinskyInstruction::Inc { counter, next } => {
                let idx = minsky_counter_idx(counter);
                state.counters[idx] = state.counters[idx]
                    .checked_add(1)
                    .ok_or(WitnessError::MinskyCounterOverflow(counter))?;
                state.pc = next;
            }
            MinskyInstruction::DecJz {
                counter,
                dec_next,
                zero_next,
            } => {
                let idx = minsky_counter_idx(counter);
                if state.counters[idx] == 0 {
                    state.pc = zero_next;
                } else {
                    state.counters[idx] -= 1;
                    state.pc = dec_next;
                }
            }
            MinskyInstruction::Halt => {
                state.halted = true;
            }
        }
    }
    Ok(MinskyRun {
        initial_counters,
        steps,
        final_state: state,
    })
}

fn minsky_counter_idx(counter: MinskyCounter) -> usize {
    match counter {
        MinskyCounter::A => 0,
        MinskyCounter::B => 1,
    }
}

fn minsky_step_record(
    pc: usize,
    counters: [u64; 2],
    instruction: MinskyInstruction,
    prev_hash: &str,
) -> MinskyStepRecord {
    let prehash = MinskyStepPrehash {
        pc,
        counters,
        instruction: instruction.clone(),
        prev_hash: prev_hash.to_string(),
    };
    MinskyStepRecord {
        pc,
        counters,
        instruction,
        prev_hash: prev_hash.to_string(),
        step_hash: hash_json(&prehash),
    }
}

fn compare_minsky_steps(
    expected: &[MinskyStepRecord],
    actual: &[MinskyStepRecord],
) -> Result<(), WitnessError> {
    if expected.len() != actual.len() {
        return Err(WitnessError::TraceMismatch {
            index: expected.len().min(actual.len()),
        });
    }
    for (idx, (left, right)) in expected.iter().zip(actual).enumerate() {
        if stable_json(left) != stable_json(right) {
            return Err(WitnessError::TraceMismatch { index: idx });
        }
    }
    Ok(())
}

fn append_brainfuck_steps(
    code: &[char],
    jumps: &BTreeMap<usize, usize>,
    mut steps: Vec<BrainfuckStepRecord>,
    mut output: Vec<u8>,
    mut state: BrainfuckState,
    max_steps: usize,
) -> Result<BrainfuckRun, WitnessError> {
    for _ in 0..max_steps {
        if state.halted {
            break;
        }
        if state.pc >= code.len() {
            state.halted = true;
            break;
        }
        let pc = state.pc;
        let data_ptr_before = state.data_ptr;
        let instruction = code[pc];

        match instruction {
            '+' => {
                state.cells[state.data_ptr] = state.cells[state.data_ptr].wrapping_add(1);
                state.pc += 1;
            }
            '-' => {
                state.cells[state.data_ptr] = state.cells[state.data_ptr].wrapping_sub(1);
                state.pc += 1;
            }
            '>' => {
                state.data_ptr += 1;
                if state.data_ptr == state.cells.len() {
                    state.cells.push(0);
                }
                state.pc += 1;
            }
            '<' => {
                if state.data_ptr == 0 {
                    return Err(WitnessError::BrainfuckPointerUnderflow);
                }
                state.data_ptr -= 1;
                state.pc += 1;
            }
            '[' => {
                if state.cells[state.data_ptr] == 0 {
                    state.pc = jumps
                        .get(&pc)
                        .copied()
                        .ok_or(WitnessError::BrainfuckBracketMismatch)?
                        + 1;
                } else {
                    state.pc += 1;
                }
            }
            ']' => {
                if state.cells[state.data_ptr] != 0 {
                    state.pc = jumps
                        .get(&pc)
                        .copied()
                        .ok_or(WitnessError::BrainfuckBracketMismatch)?
                        + 1;
                } else {
                    state.pc += 1;
                }
            }
            '.' => {
                output.push(state.cells[state.data_ptr]);
                state.pc += 1;
            }
            _ => return Err(WitnessError::InvalidBrainfuckPc(pc)),
        }

        if state.pc >= code.len() {
            state.halted = true;
        }
        let touched_cell = state.data_ptr;
        let touched_value = state.cells[state.data_ptr];
        let output_hash = sha256_hex(&output);
        let step = brainfuck_step_record(
            pc,
            data_ptr_before,
            instruction,
            touched_cell,
            touched_value,
            output_hash,
            &state.prev_hash,
        );
        state.prev_hash = step.step_hash.clone();
        steps.push(step);
    }
    Ok(BrainfuckRun {
        steps,
        output,
        final_state: state,
    })
}

fn brainfuck_code(program: &str) -> Vec<char> {
    program
        .chars()
        .filter(|c| matches!(c, '+' | '-' | '<' | '>' | '[' | ']' | '.'))
        .collect()
}

fn brainfuck_jumps(code: &[char]) -> Result<BTreeMap<usize, usize>, WitnessError> {
    let mut stack = Vec::new();
    let mut jumps = BTreeMap::new();
    for (idx, instruction) in code.iter().enumerate() {
        match instruction {
            '[' => stack.push(idx),
            ']' => {
                let open = stack.pop().ok_or(WitnessError::BrainfuckBracketMismatch)?;
                jumps.insert(open, idx);
                jumps.insert(idx, open);
            }
            _ => {}
        }
    }
    if stack.is_empty() {
        Ok(jumps)
    } else {
        Err(WitnessError::BrainfuckBracketMismatch)
    }
}

fn brainfuck_step_record(
    pc: usize,
    data_ptr: usize,
    instruction: char,
    touched_cell: usize,
    touched_value: u8,
    output_hash: String,
    prev_hash: &str,
) -> BrainfuckStepRecord {
    let prehash = BrainfuckStepPrehash {
        pc,
        data_ptr,
        instruction,
        touched_cell,
        touched_value,
        output_hash: output_hash.clone(),
        prev_hash: prev_hash.to_string(),
    };
    BrainfuckStepRecord {
        pc,
        data_ptr,
        instruction,
        touched_cell,
        touched_value,
        output_hash,
        prev_hash: prev_hash.to_string(),
        step_hash: hash_json(&prehash),
    }
}

fn compare_brainfuck_steps(
    expected: &[BrainfuckStepRecord],
    actual: &[BrainfuckStepRecord],
) -> Result<(), WitnessError> {
    if expected.len() != actual.len() {
        return Err(WitnessError::TraceMismatch {
            index: expected.len().min(actual.len()),
        });
    }
    for (idx, (left, right)) in expected.iter().zip(actual).enumerate() {
        if stable_json(left) != stable_json(right) {
            return Err(WitnessError::TraceMismatch { index: idx });
        }
    }
    Ok(())
}

fn minsky_genesis_hash(program: &[MinskyInstruction], initial_counters: [u64; 2]) -> String {
    hash_json(&MinskyGenesis {
        machine: "tc_minsky_v1",
        program,
        initial_counters,
    })
}

fn brainfuck_genesis_hash(code: &[char]) -> String {
    hash_json(&BrainfuckGenesis {
        machine: "tc_brainfuck_v1",
        code,
    })
}

fn hash_json<T: Serialize>(value: &T) -> String {
    sha256_hex(&stable_json(value))
}

fn stable_json<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).expect("serializable TC witness record")
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

#[derive(Serialize)]
struct MinskyGenesis<'a> {
    machine: &'static str,
    program: &'a [MinskyInstruction],
    initial_counters: [u64; 2],
}

#[derive(Serialize)]
struct MinskyStepPrehash {
    pc: usize,
    counters: [u64; 2],
    instruction: MinskyInstruction,
    prev_hash: String,
}

#[derive(Serialize)]
struct BrainfuckGenesis<'a> {
    machine: &'static str,
    code: &'a [char],
}

#[derive(Serialize)]
struct BrainfuckStepPrehash {
    pc: usize,
    data_ptr: usize,
    instruction: char,
    touched_cell: usize,
    touched_value: u8,
    output_hash: String,
    prev_hash: String,
}
