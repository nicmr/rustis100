use crate::errors::{InterpretError};


#[derive(Copy, Clone, Debug)]
enum Token {
    Number(u32),
    NodeID(u32),
    Add,
    Mov,
    EOF,
}






// struct TisNode {
//     pub acc: u32,
//     pub bak: u32,
//     pub ip: usize, //the instruction pointer / program counter

//     pub code: Vec<Box<TisInstruction>>,
//     // ports
//     // storage for last pseudoport
// }

// impl Clone for TisNode {
//     fn clone(&self) -> Self {*self}
// }

// impl TisNode {
//     pub fn new() -> Self {
//         TisNode{
//             acc: 0,
//             bak: 0,
//             ip: 0,
//             code: Vec::new(),
//         }
//     }
//     pub fn tick(&mut self) {

//     }
// }

// impl std::fmt::Debug for TisNode {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
//         f.debug_struct("TisNode")
//             .field("acc", &self.acc)
//             .field("bak", &self.bak)
//             .field("ip", &self.ip)
//             .field("bak", &self.bak)
//             .finish()
//     }
// }

// type TisInstruction = Fn(&mut TisNode, Vec::<u32>);

// // TisInstruction
// fn add(node: &mut TisNode, operands: Vec::<u32>) {
//     node.acc = node.acc + operands[1];
// }

// // TisInstruction
// fn mov(node: &mut TisNode, operands: Vec::<u32>) {
//     // do fancy stuff
// }




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
    String::from("@0ADD99")
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



// To be deleted:
// pub struct TisInterpreter {
//     text: Vec<char>,
//     pos: usize,
//     nodes: Vec<TisNode>,
// }
// impl TisInterpreter {
//     pub fn new<S: std::string::ToString>(text: S) -> TisInterpreter{
//         return TisInterpreter{
//             text: text.to_string().chars().collect(),
//             pos: 0,
//             nodes: Vec::new(),
//         }
//     }


    


//     // should be reworked to return result instead, with a new error type, where
//     // each error kind represents a single problem encountered while parsing the token 
//     fn get_next_token(&mut self) -> Option<Token> {
//         use std::iter::FromIterator;

//         // end of input reached
//         if self.pos > self.text.len() -1 {
//             return Some(Token::EOF);
//         }

//         let current_char = self.text[self.pos];
//         // println!("first char is: {}", current_char);


//         if current_char == '@' {
//             if self.pos + 1 < self.text.len() && self.text[self.pos + 1].is_digit(10) {
//                 let id = self.text[self.pos + 1];
//                 self.pos += 2;
//                 // we only arrive at this point when id.is_digit(10) was successful, so to_digit(10) will always unwrap successfully
//                 Some(Token::NodeID(id.to_digit(10).unwrap()))
//             } else {
//                 None
//             }
//         }
//         else if current_char.is_digit(10) {
            
//             let mut length = 1;
//             while self.pos + length < self.text.len() && self.text[self.pos + length].is_digit(10) {
//                 length += 1
//             }
//             let s = String::from_iter(self.text[self.pos..(self.pos+length)].iter());
//             // println!("finished token is {}", s);
//             let number = s.parse::<u32>().unwrap();
//             self.pos += length;
//             Some(Token::Number(number))

//         } else if current_char == 'A' {
//             // addition operator
//             // println!("=> identified as +");

//             self.pos += 1;
//             Some(Token::Add)
//         } else {
//             // unknown input
//             // println!("=> couldn't identify :(");
//             self.pos += 1;
//             None
//         }
//     }

//     // Parses a single expression in the language.
//     // Expressions in the TIS-100 language are written in Polish notation
//     pub fn expr(&mut self) -> Result<u32, TokenError> {
//         self.nodes = vec![TisNode::new(); 12];

//         let current_node: &mut TisNode;
//         let mut operator: Box<TisInstruction>;
//         let mut operands = Vec::new();

//         // 1. A node declaration is required
//         if let Some(token) = self.get_next_token() {
//             match token   {
//                 // TODO: currently supports only nodes 0 through 9
//                 Token::NodeID(x) => { current_node = &mut self.nodes[x as usize]; },
//                 _ => { return Err(TokenError::new("Unexpected token: Expected Node declaration")); }
//             }
//         } else {
//             return Err(TokenError::new("Node declaration expected"));
//         }

//         // 2. An operator is requiired,
//         // (In polish notation, operators come first, so our 2nd token has to be the operator)
//         if let Some(token) = self.get_next_token() {
//             match token {
//                 Token::Add => {current_node.code.push(Box::new(add))}
//                 Token::Mov => {current_node.code.push(Box::new(mov))}
//                 _ => { return Err(TokenError::new("Unexpected Token")) }
//             }
//         } else {
//             return Err(TokenError::new("Unable to get any tokens from text"));
//         }


//         // if let Some(token) = self.get_next_token(){
//         //     match token {
//         //         Token::Number(x) => { left = x; },
//         //         _ => { return Err(TokenError::new("Unexpected Token")) },
//         //     }
            
//         // }

//         // 2nd and 3rd must be a number each
//         'a: while let Some(token) = self.get_next_token(){
//             match token {
//                 Token::Number(x) => { operands.push(x) },
//                 _ => { return Err(TokenError::new("Unexpected Token")) }
//             }
//         }

//         operator(&mut self.nodes[0], operands);
//         Ok(self.nodes[0].acc)
//     }
// }

