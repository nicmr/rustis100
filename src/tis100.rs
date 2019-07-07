use crate::token_error::{TokenError};


#[derive(Copy, Clone, Debug)]
enum Token {
    Number(u32),
    NodeID(u32),
    Add,
    Mov,
    EOF,
}




pub struct TisInterpreter {
    text: Vec<char>,
    pos: usize,
    nodes: Vec<TisNode>,
}

struct TisNode {
    pub acc: u32,
    pub bak: u32,
    pub ip: usize, //the instruction pointer / program counter

    pub code: Vec<Box<TisInstruction>>,
    // ports
    // storage for last pseudoport
}

impl Clone for TisNode {
    fn clone(&self) -> Self {*self}
}

impl TisNode {
    pub fn new() -> Self {
        TisNode{
            acc: 0,
            bak: 0,
            ip: 0,
            code: Vec::new(),
        }
    }
    pub fn tick(&mut self) {

    }
}

impl std::fmt::Debug for TisNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("TisNode")
            .field("acc", &self.acc)
            .field("bak", &self.bak)
            .field("ip", &self.ip)
            .field("bak", &self.bak)
            .finish()
    }
}

type TisInstruction = Fn(&mut TisNode, Vec::<u32>);

// TisInstruction
fn add(node: &mut TisNode, operands: Vec::<u32>) {
    node.acc = node.acc + operands[1];
}

// TisInstruction
fn mov(node: &mut TisNode, operands: Vec::<u32>) {
    // do fancy stuff
}




// #[derive(Copy, Clone, Debug)]
// enum TisPort{
//     Up,
//     Down,
//     Left,
//     Right,
//     Any,
//     Nil,
// }



pub fn sample_code() -> String {
    String::from("@0ADD 1")
}


impl TisInterpreter {
    pub fn new<S: std::string::ToString>(text: S) -> TisInterpreter{
        return TisInterpreter{
            text: text.to_string().chars().collect(),
            pos: 0,
            nodes: Vec::new(),
        }
    }

    // should be reworked to return result instead, with a new error type, where
    // each error kind represents a single problem encountered while parsing the token 
    fn get_next_token(&mut self) -> Option<Token> {
        use std::iter::FromIterator;

        // end of input reached
        if self.pos > self.text.len() -1 {
            return Some(Token::EOF);
        }

        let current_char = self.text[self.pos];
        // println!("first char is: {}", current_char);


        if current_char == '@' {
            if self.pos + 1 < self.text.len() && self.text[self.pos + 1].is_digit(10) {
                let id = self.text[self.pos + 1];
                self.pos += 2;
                // we only arrive at this point when id.is_digit(10) was successful, so to_digit(10) will always unwrap successfully
                Some(Token::NodeID(id.to_digit(10).unwrap()))
            } else {
                None
            }
        }
        else if current_char.is_digit(10) {
            
            let mut length = 1;
            while self.pos + length < self.text.len() && self.text[self.pos + length].is_digit(10) {
                length += 1
            }
            let s = String::from_iter(self.text[self.pos..(self.pos+length)].iter());
            // println!("finished token is {}", s);
            let number = s.parse::<u32>().unwrap();
            self.pos += length;
            Some(Token::Number(number))

        } else if current_char == 'A' {
            // addition operator
            // println!("=> identified as +");

            self.pos += 1;
            Some(Token::Add)
        } else {
            // unknown input
            // println!("=> couldn't identify :(");
            self.pos += 1;
            None
        }
    }

    // Parses a single expression in the language.
    // Expressions in the TIS-100 language are written in Polish notation
    pub fn expr(&mut self) -> Result<u32, TokenError> {
        self.nodes = vec![TisNode::new(); 12];

        let current_node: &mut TisNode;
        let mut operator: Box<TisInstruction>;
        let mut operands = Vec::new();

        // 1. A node declaration is required
        if let Some(token) = self.get_next_token() {
            match token   {
                // TODO: currently supports only nodes 0 through 9
                Token::NodeID(x) => { current_node = &mut self.nodes[x as usize]; },
                _ => { return Err(TokenError::new("Unexpected token: Expected Node declaration")); }
            }
        } else {
            return Err(TokenError::new("Node declaration expected"));
        }

        // 2. An operator is requiired,
        // (In polish notation, operators come first, so our 2nd token has to be the operator)
        if let Some(token) = self.get_next_token() {
            match token {
                Token::Add => {current_node.code.push(Box::new(add))}
                Token::Mov => {current_node.code.push(Box::new(mov))}
                _ => { return Err(TokenError::new("Unexpected Token")) }
            }
        } else {
            return Err(TokenError::new("Unable to get any tokens from text"));
        }


        // if let Some(token) = self.get_next_token(){
        //     match token {
        //         Token::Number(x) => { left = x; },
        //         _ => { return Err(TokenError::new("Unexpected Token")) },
        //     }
            
        // }

        // 2nd and 3rd must be a number each
        'a: while let Some(token) = self.get_next_token(){
            match token {
                Token::Number(x) => { operands.push(x) },
                _ => { return Err(TokenError::new("Unexpected Token")) }
            }
        }

        operator(&mut self.nodes[0], operands);
        Ok(self.nodes[0].acc)
    }
}

