use crate::errors::{InterpretError};
use std::collections::HashMap;
use regex::Regex;
use lazy_static;

pub mod emu;

// TODO: split emulator code into emu submodule
// mod emu;


pub fn sample_code() -> String {
    String::from(
r#"@0
ADD4ADD2

@1 ADD1
"#)
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


fn next_token_pure(text: &Vec<char>, position: usize) -> Result<(Token, usize), InterpretError> {
    use std::iter::FromIterator;

    if position > text.len() - 1 {
        return Ok((Token::EOF, position))
    }

    let current_char = text[position];
    // println!("current_char: {}", current_char);
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

// TODO: introduce a new token variant operator with an associated enum operatorKind,
// to avoid having to match each pattern in the next_token call match
// TODO: match on operator variant to for loop requried times over getting expected operands.
// TODO: add support in jumpmark parser for lines with only jumpmarks
fn parse_node(node_text: String) -> Result<Node, InterpretError> {
    
    // text.find
    let mut node = Node::new();
    let mut jumpmarks: HashMap<String, usize> = HashMap::new();


    lazy_static::lazy_static! {
        static ref RE_JUMPMARK: Regex = Regex::new(r"\w+:").unwrap();
    }

    let text: Vec<Vec<char>> = node_text
    .lines()
    .enumerate()
    .filter_map(|(line_index, line)| {
        if let Some(mat) = RE_JUMPMARK.find(line) {
            jumpmarks.insert(line[..mat.end()].to_string(), line_index);
            if mat.end() < line.len() {
               Some(&line[mat.end()+1..])
            } else {
                None
            }
        } else {
            Some(&line)
        }
    })
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect();


    let instruction_result: Result<Vec<Instruction>, _> = text
    .iter()
    .map(|text| {
        let mut position = 0;

        let operator_token;
        let operand;
        let instruction;

        // expect: operator
        match next_token_pure(&text, position) {
            Ok((token, pos)) => match token {
                Token::Add => {
                    position = pos;
                    operator_token = Token::Add;
                },
                _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected NodeID at position {}", pos))); },
            },
            Err(e) => { return Err(e)} ,
        }

        // expect: operand
        match next_token_pure(&text, position) {
            Ok((token, pos)) => match token {
                Token::Number(x) => { operand = x;
                    // position reassignment currently not needed, as no other tokenizer step occurs after this point
                    // position = pos;
                },
                _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected operand at position {}", pos))); }
            },
            Err(e) => { return Err(e); },
        }

        match operator_token {
            Token::Add => {
                instruction = Instruction::Add(operand);
            },
            _ => {
                instruction = Instruction::Nop;
            },
        }
        Ok(instruction)
    })
    .collect();

    match instruction_result {
        Ok(instructions) => {
            node.instructions = instructions;
            Ok(node)
        }
        Err(e) => Err(e)
    }
}

pub fn parse(text: String) -> Result<Vec<Node>, InterpretError>{

    // remove spaces, tabs and commas :  they are syntactically irrelevant and make parsing more difficult
    let body: String = text.chars().filter(|&x| !(x == ' ' || x == '\t' || x == ',')).collect();

    // To do: support multi_digit node ids
    lazy_static::lazy_static! {
        static ref RE_NODES: Regex = Regex::new(r"@(\d)").unwrap();
    }

    let nodes: Result<Vec<Node>, InterpretError> = body
        .split("@")
        .into_iter()
        // TODO: currently we are discarding the id (at the 0th index), let's put it to use instead
        .map(|text| &text[1..])
        .map(|text| parse_node(text.to_owned()))
        .collect();
    
    nodes
}