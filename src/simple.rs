// A simple addition parser in rust, created before getting into tis100 specifics
use errors::{InterpretError};

#[derive(Copy, Clone, Debug)]
enum Token {
    Number(u32),
    Add,
    EOF,
}

// purely functional approach
// takes Vec<char> instead of String because char-indexing a String internally is O(n) in rust (chars are utf8 -> have variable size)
fn next_token_pure(text: &Vec<char>, position: usize) -> Result<(Token, usize), InterpretError> {
    use std::iter::FromIterator;

    if position > text.len() -1 {
        return Ok((Token::EOF, position));
    }

    let current_char = text[position];
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
        '+' => Ok((Token::Add, position+1)),
        _ =>  Err(InterpretError::token_error("Unknown symbol encountered while parsing"))
    }

}

fn expr_pure(text: Vec<char>, position: usize) -> Result<usize, InterpretError>{
    let left;
    let right;

    let mut position = position;

    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::Number(x) => { left = x; position = pos; },
            _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected 1st number at position {}", pos))); },
        },
        Err(e) => { return Err(e)} ,
    }

    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token{
            Token::Add => { position = pos; }
            _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected '+' at position {}", pos))); },
        },
        Err(e) => { return Err(e); },
    }

    match next_token_pure(&text, position) {
        Ok((token, pos)) => match token {
            Token::Number(x) => { right = x;
                //position = pos;
            },
            _ => { return Err(InterpretError::syntax_error(format!("Unexpected Token: expected 2nd number at position {}", pos))); }
        },
        Err(e) => { return Err(e); },
    }

    Ok((left+right) as usize)
}


// // previous, bad, stateful approach. Kept for comparison.
// struct SimpleInterpreter {
//     text: Vec<char>,
//     pos: usize,
// }


// impl SimpleInterpreter {
//     fn new<S: std::string::ToString>(text: S) -> SimpleInterpreter{
//         return SimpleInterpreter{
//             text: text.to_string().chars().collect(),
//             pos: 0,
//         }
//     }

//     fn get_next_token(&mut self) -> Option<Token> {
//         use std::iter::FromIterator;

//         // end of input reached
//         if self.pos > self.text.len() -1 {
//             return Some(Token::EOF);
//         }

//         let current_char = self.text[self.pos];
//         // println!("first char is: {}", current_char);
        

//         if current_char.is_digit(10) {
            
//             let mut length = 1;
//             while self.pos + length < self.text.len() && self.text[self.pos + length].is_digit(10) {
//                 length += 1
//             }
//             let s = String::from_iter(self.text[self.pos..(self.pos+length)].iter());
//             // println!("finished token is {}", s);
//             let number = s.parse::<u32>().unwrap();
//             self.pos += length;
//             Some(Token::Number(number))

//         } else if current_char == '+' {
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

//     fn expr(&mut self) -> Result<u32, InterpretError> {
//         let mut left = 0;
//         let mut right = 0;

//         // 1st token has to be number
//         if let Some(token) = self.get_next_token(){
//             match token {
//                 Token::Number(x) => { left = x; },
//                 _ => { return Err(InterpretError::syntax_error("Unexpected Token")) },
//             }
//         }

//         // 2nd to nth token may be number until an operator appears
//         // Operator will break loop
//         'a: while let Some(token) = self.get_next_token(){
//             match token {
//                 Token::Number(x) => { left = x;},
//                 Token::Add => {break 'a;},
//                 _ => { return Err(InterpretError::syntax_error("Unexpected Token")) }
//             }
//         }

//         'b: while let Some(token) = self.get_next_token() {
//             match token {
//                 Token::Number(x) => { right = x;},
//                 Token::EOF => { break 'b;},
//                 _ => { return Err(InterpretError::syntax_error("Unexpected Token")) }
//             }
//         }

//         println!("left: {}, right: {}", left, right);
//         Ok(left + right)
//     }
// }