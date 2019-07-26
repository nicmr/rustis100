use lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::tis100::{Node, Instruction};
use crate::errors::{InterpretError};

#[derive(Copy, Clone, Debug)]
enum Token {
    Number(u32),
    NodeID(u32),
    EOF,
    Op(Operator, usize)
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    // Arithmetic
    Add,
    Sub,
    Neg,
    // Channels and registers
    Mov,
    Swp,
    Sav,
    // Jump instructions
    Jmp,
    Jez,
    Jnz,
    Jgz,
    Jlz,
    Jro,
    // No operation
    Nop,
}

/// Parses a string of rustis 100 node declarations and instructions into a Vector of `Node`
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
        .filter(|text| text.len() > 0)
        // TODO: currently we are discarding the id (at the 0th index), let's put it to use instead
        .map(|text| &text[1..])
        .map(|text| parse_node(text.to_owned()))
        .collect();
    
    nodes
}

// TODO: introduce a new token variant operator with an associated enum operatorKind,
// to avoid having to match each pattern in the next_token call match
// TODO: match on operator variant to for loop requried times over getting expected operands.
// TODO: add support in jumpmark parser for lines with only jumpmarks
fn parse_node(node_text: String) -> Result<Node, InterpretError> {
    println!("parse node called with the following parameter: '{}'", node_text);
    
    // text.find
    let mut node = Node::new();
    let mut jumpmarks: HashMap<String, usize> = HashMap::new();


    lazy_static::lazy_static! {
        static ref RE_JUMPMARK: Regex = Regex::new(r"\w+:").unwrap();
    }

    let text: Vec<Vec<char>> = node_text
        .lines()
        .filter(|line| line.len() > 0)
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

            let operator;
            let operand_count;
            let instruction;

            // expect: operator
            match next_token(&text, position) {
                Ok((token, pos)) => match token {
                    Token::Op(op, opcount) => {
                        position = pos;
                        operator = op;
                        operand_count = opcount
                    }
                    _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected Operator at position {}.\nGot {:?} instead.", pos, token))); },
                },
                Err(e) => { return Err(e)} ,
            }

            let mut operands = Vec::with_capacity(operand_count);

            for _ in 0..operand_count {
                // expect: operand
                match next_token(&text, position) {
                    Ok((token, pos)) => match token {
                        Token::Number(x) => {
                            operands.push(x);
                            position = pos;
                        },
                        _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected operand at position {}", pos))); }
                    },
                    Err(e) => { return Err(e); },
                }
            }

            match operator {
                Operator::Add => {
                    instruction = Instruction::Add(operands[0]);
                },
                Operator::Sub => {
                    instruction = Instruction::Sub(operands[0]);
                }
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

lazy_static::lazy_static! {
    static ref OPMAP: HashMap<String, (Operator, usize)> = {
        let mut m = HashMap::with_capacity(13);
        m.insert("ADD".to_owned(), (Operator::Add, 1));
        m.insert("SUB".to_owned(), (Operator::Sub, 1));
        m.insert("NOP".to_owned(), (Operator::Nop, 0));
        m.insert("SAV".to_owned(), (Operator::Sav, 0));
        m.insert("SWP".to_owned(), (Operator::Swp, 0));
        m.insert("NEG".to_owned(), (Operator::Neg, 0));
        m.insert("JMP".to_owned(), (Operator::Jmp, 1));
        m.insert("JRO".to_owned(), (Operator::Jro, 1));
        m.insert("JEZ".to_owned(), (Operator::Jez, 1));
        m.insert("JNZ".to_owned(), (Operator::Jnz, 1));
        m.insert("JGZ".to_owned(), (Operator::Jgz, 1));
        m.insert("JLZ".to_owned(), (Operator::Jlz, 1));
        m.insert("MOV".to_owned(), (Operator::Mov, 2));
        m
    };
}

fn next_token(text: &Vec<char>, position: usize) -> Result<(Token, usize), InterpretError> {
    use std::iter::FromIterator;

    if position + 1 > text.len() {
        return Ok((Token::EOF, position))
    }

    let current_char = text[position];
    // println!("current_char: {}", current_char)

    match current_char {
        _ if current_char.is_digit(10) => {
            let mut right_bound = position + 1;
            while right_bound < text.len() && text[right_bound].is_digit(10) {
                right_bound += 1;
            }
            let s = String::from_iter(text[position..right_bound].iter());
            let number = s.parse::<u32>().unwrap();
            Ok((Token::Number(number), right_bound))
        },
        _ if current_char.is_alphabetic() => {
            if position < text.len()-1 {
                let s = text[position..position+3].iter().collect::<String>();
                if let Some((op, op_count)) = OPMAP.get(&s){
                    Ok((Token::Op(*op, *op_count), position + 3))
                } else {
                    Err(InterpretError::token_error(format!("Unknown token encountered while parsing: '{}'", s)))
                }
            } else {
                // TODO: improve this error message
                Err(InterpretError::token_error(format!("Token too short: {}", current_char)))
            }
        }
        _ =>  Err(InterpretError::token_error(format!("Unknown symbol encountered while parsing: {}", current_char)))
    }
}