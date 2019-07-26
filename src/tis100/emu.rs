use crate::tis100::{Node, Instruction, NodeState};

pub fn tick_all_n(mut nodes: Vec<Node>, ticks: usize) -> Vec<Node> {
    for node in &mut nodes {
        let new_state = tick_n(&node.state, &node.instructions, ticks);
        node.state = new_state;
    }
    nodes
}


pub fn tick_n(state: &NodeState, instructions: &Vec<Instruction>, ticks: usize) -> NodeState {
    let program_length = instructions.len();
    if program_length == 0 {
        return *state;
    }
    let mut new_state = *state;
    for tick in 0..ticks {
        match instructions[tick % program_length] {
            Instruction::Add(x) => {new_state.acc = saturating_add(new_state.acc, x)},
            Instruction::Sub(x) => {new_state.acc = saturating_sub(new_state.acc, x)},
            Instruction::Sav => {new_state.bak = new_state.acc}
            Instruction::Swp => {let store = new_state.acc; new_state.bak = state.acc; new_state.acc = store}
            Instruction::Neg => { new_state.acc = -new_state.acc}
            Instruction::Nop => {
                // literally no operation
            },
        }
    }
    new_state
}

/// Saturates at the integer bounds of the TIS-100, 999 and -999
/// As both operands are guaranteed to be within [-999, 999]
/// we will never encounter i32 overflows
fn saturating_add(a: i32, b: i32) -> i32 {
    let result = a + b;
    if result > 999 {
        999
    } else if result < -999 {
        -999
    } else {
        result
    }
}

/// Saturates at the integer bounds of the TIS-100, 999 and -999
/// As both operands are guaranteed to be within [-999, 999],
/// we will never encounter i32 overflows
fn saturating_sub(a: i32, b: i32) -> i32 {
    let result = a - b;
    if result > 999 {
        999
    } else if result < -999 {
        -999
    } else {
        result
    }
}