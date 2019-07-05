use crate::token_error::{TokenError};


#[derive(Copy, Clone, Debug)]
enum Token {
    Number(u32),
    NodeID,
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
    // ports
    // storage for last pseudoport
}

impl TisNode {
    pub fn new() -> Self {
        TisNode{
            acc: 0,
            bak: 0,
        }
    }
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



pub fn sampleCode() -> String {
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

    fn get_next_token(&mut self) -> Option<Token> {
        use std::iter::FromIterator;

        // end of input reached
        if self.pos > self.text.len() -1 {
            return Some(Token::EOF);
        }

        let current_char = self.text[self.pos];
        // println!("first char is: {}", current_char);
        

        if current_char.is_digit(10) {
            
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
        self.nodes.push(TisNode::new());
        let mut operator: Box<Fn(&mut TisNode, Vec::<u32>)>;
        let mut operands = Vec::new();


        // In polish notation, operators come first, so our 1st token has to be the operator
        // Or a node identifier
        if let Some(token) = self.get_next_token() {
            match token {
                Token::Add => {operator = Box::new(add);}
                Token::Mov => {operator = Box::new(mov);}
                _ => { return Err(TokenError::new("Unexpected Token")) }
            }
        } else {
            return Err(TokenError::new("Unable to get any tokens from text"))
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


fn add(node: &mut TisNode, operands: Vec::<u32>) {
    node.acc = node.acc + operands[1];
}

fn mov(node: &mut TisNode, operands: Vec::<u32>) {
    // do fancy stuff
}
