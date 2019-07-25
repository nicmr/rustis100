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
            Instruction::Add(x) => {new_state.acc += x},
            Instruction::Sub(x) => {new_state.acc -= x},
            Instruction::Nop => {
                // literally no operation
            },
        }
    }
    new_state
}