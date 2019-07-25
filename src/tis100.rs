pub mod emu;
pub mod parse;

// re-export most commonly needed functions from child modules for easy access 
pub use parse::parse;

pub fn sample_code() -> String {
    String::from(
r#"@0 ADD4ADD2

@1 ADD1
"#)
}

#[derive(Debug, Clone)]
pub struct Node {
    state: NodeState,
    instructions: Vec<Instruction>,
}

impl Node {
    fn new() -> Self {
        return Node {
            state: NodeState::new(),
            instructions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NodeState {
    pub acc: u32,
    pub bak: u32,
    // pub ip: usize, //the instruction pointer / program counter //superfluous?
}

impl NodeState {
    fn new() -> Self {
        return NodeState {
            acc: 0,
            bak: 0,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Add(u32),
    Sub(u32),
    // To be implemented:
    // Mov,
    Nop,
    // Swp,
    // Sav,
    // Neg,
    // Jmp,
    // Jez,
    // Jnz,
    // Jlz,
    // Jro,
}