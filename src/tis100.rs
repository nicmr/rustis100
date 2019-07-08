use crate::errors::{InterpretError};

pub fn sample_code() -> String {
    String::from("@0ADD4")
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Number(u32),
    NodeID(u32),
    Add,
    EOF,
    // To be implemented:
    // Mov,
    // Nop,
    // Swp,
    // Sav,
    // Neg,
    // Jmp,
    // Jez,
    // Jnz,
    // Jlz,
    // Jro,
}


// IDEA:
// Don't pass functions for the single instructions, becuase that requires a lot of dyn and Box Code
// Instead, store Instructions as Enums with operands and their parameters as associated values.
// Can we achieve an implementation without internal state and mut refs?s

// Goal for now: single node running basic code.

// Issue: instructions can not be mutated by instructions. So instructions should not be parted of mutable data of TisNode.
// Instead, it may be better to rename the current TisNode type to NodeState, and compose a new type TisNode that contains both
// the state and the instructions.

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
        return state.clone();
    }
    let mut new_state = NodeState::new();
    for tick in 0..ticks {
        match instructions[tick % program_length] {
            Instruction::Add(x) => {new_state.acc =  state.acc + x},
            Instruction::Sub(x) => {new_state.acc = state.acc - x},
            Instruction::Nop => {
                // literally no operation
            },
        }
    }
    new_state
}



fn next_token_pure(text: &Vec<char>, position: usize) -> Result<(Token, usize), InterpretError> {
    use std::iter::FromIterator;


    if position > text.len() - 1 {
        return Ok((Token::EOF, position))
    }

    let current_char = text[position];
    println!("current_char: {}", current_char);
    match current_char {
        '@' => {
            let mut right_bound = position+1;
            while right_bound < text.len() && text[right_bound].is_digit(10) {
                right_bound += 1;
            }
            if right_bound == position +1 {
                Err(InterpretError::token_error("Numeric node ID missing after '@'"))
            } else {
                // take only the ID, '@' identifier is not required
                let s = String::from_iter(text[(position+1)..right_bound].iter());
                let id = s.parse::<u32>().unwrap();
                Ok((Token::NodeID(id), right_bound))
            }
        },
        'A' => {
            // make sure at least two characters afterwards can be accessed without stepping out of vec bounds
            if position < text.len()-1 {
                if text[position+1] == 'D' && text[position+2] == 'D' {
                    Ok((Token::Add, position + 3))
                } else {
                    Err(InterpretError::token_error(
                        format!("Unkown Token encountered: 'A{}{}'. Did you mean: 'ADD'?", text[position+1], text[position+2])
                    ))
                }
            } else {
                // TODO: implement alternative suggestion as feature of the Error type :)
                Err(InterpretError::token_error("Unkown Token encountered: 'A'. Did you mean: 'ADD'?"))
            }
        }
        _ if current_char.is_digit(10) => {
            let mut right_bound = position + 1;
            while right_bound < text.len() && text[right_bound].is_digit(10) {
                right_bound += 1;
            }
            let s = String::from_iter(text[position..right_bound].iter());
            let number = s.parse::<u32>().unwrap();
            Ok((Token::Number(number), right_bound))
        },
        _ =>  Err(InterpretError::token_error("Unknown symbol encountered while parsing"))
    }
}

pub fn expr_pure(text: Vec<char>, position: usize) -> Result<Vec<Node>, InterpretError> {
    // currently only one operand needed add only add is supported
    // let left;
    let right;


    // TODO: change limits for max node count, possibly enforce max node count in token parser
    let mut nodes = vec![Node::new(); 4];
    let current_id;
    let operator_token;

    let mut position = position;

    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::NodeID(id) => {
                position = pos;
                current_id = id;
            },
            _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected NodeID at position {}", pos))); },
        
        },
        Err(e) => { return Err(e)} ,
    }
    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::Add => {
                position = pos;
                operator_token = Token::Add;
            },
            _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected Operator at position {}", pos))); },
        },
        Err(e) => { return Err(e)} ,
    }

    // match next_token_pure(&text, position) {
    //     Ok((token, pos)) => match token{
    //         Token::Number => { left = x; position = pos; }
    //         _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected 1st operand at position {}", pos))); },
    //     },
    //     Err(e) => { return Err(e); },
    // }

    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::Number(x) => { right = x;
                //position = pos; //currently not needed as its the last part of expression
            },
            _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected operand at position {}", pos))); }
        },
        Err(e) => { return Err(e); },
    }

    let instruction;
    match operator_token {
        Token::Add => {
            instruction = Instruction::Add(right);
        },
        _ => {
            instruction = Instruction::Nop;
        },
    }
    nodes[current_id as usize].instructions.push(instruction);

    Ok(nodes)
}