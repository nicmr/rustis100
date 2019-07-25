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

/// Node represents a single node in the TIS-100 architecture
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

/// NodeState represents the state of a TIS-100 note.
/// It will usually be initialized when starting the emulator and be mutated with each emulator tick.
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

/// Instruction represents a single instruction a Node computes each tick.
/// An instruction includes its operands as associated values where required.
/// Instructions will usually be created during parsing and not be mutated at emulator runtime.
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