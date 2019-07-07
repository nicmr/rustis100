use crate::errors::{InterpretError};

pub fn sample_code() -> String {
    String::from("@0ADD99")
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

#[derive(Debug, Clone, Copy)]
pub struct NodeState {
    pub acc: u32,
    pub bak: u32,
    // pub ip: usize, //the instruction pointer / program counter //superfluous?
}


#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Add(u32),
    Sub(u32),
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

pub fn tick_n(mut node: NodeState, instructions: Vec<Instruction>, ticks: usize) -> NodeState {
    let program_length = instructions.len();
    for tick in 0..ticks {
        match instructions[tick % program_length] {
            Instruction::Add(x) => {node.acc += x},
            Instruction::Sub(x) => {node.acc -= x},
        }
    }
    node
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
                Err(InterpretError::TokenError("Numeric node ID missing after '@'"))
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
                    Err(InterpretError::TokenError(
                        format!("Unkown Token encountered: 'A{}{}'. Did you mean: 'ADD'?", text[position+1], text[position+2])
                    ))
                }
            } else {
                // TODO: implement alternative suggestion as feature of the Error type :)
                Err(InterpretError::TokenError("Unkown Token encountered: 'A'. Did you mean: 'ADD'?"))
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
        '+' => Ok((Token::Add, position+1)),
        _ =>  Err(InterpretError::TokenError("Unknown symbol encountered while parsing"))
    }
}

pub fn expr_pure(text: Vec<char>, position: usize) -> Result<usize, InterpretError> {
    // currently only one operand needed add only add is supported
    // let left;
    let right;

    let mut position = position;

    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::NodeID(id) => {
                position = pos;
            },
            _ => { return Err(InterpretError::SyntaxError(format!("Unexpected Token: expected NodeID at position {}", pos))); },
        
        },
        Err(e) => { return Err(e)} ,
    }
    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::Add => { position = pos; },
            _ => { return Err(InterpretError::SyntaxError(format!("Unexpected Token: expected Operator at position {}", pos))); },
        },
        Err(e) => { return Err(e)} ,
    }

    // match next_token_pure(&text, position) {
    //     Ok((token, pos)) => match token{
    //         Token::Number => { left = x; position = pos; }
    //         _ => { return Err(InterpretError::SyntaxError(format!("Unexpected Token: expected 1st operand at position {}", pos))); },
    //     },
    //     Err(e) => { return Err(e); },
    // }

    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::Number(x) => { right = x;
                //position = pos; //currently not needed as its the last part of expression
            },
            _ => { return Err(InterpretError::SyntaxError(format!("Unexpected Token: expected operand at position {}", pos))); }
        },
        Err(e) => { return Err(e); },
    }
    Ok((0 + right) as usize)
}