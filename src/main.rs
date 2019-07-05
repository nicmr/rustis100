use std::fmt::{self, Display};
use std::error;

fn main() {
    let sequence = "14+21";
    println!("Sequence is: {}", sequence);
    let mut interp = SimpleInterpreter::new(sequence);
    match interp.expr() {
        Ok(result) => println!("result is {}", result),
        Err(e) => println!("Err is {}", e)
    }
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Number(u32),
    // Port(TisPort),
    // Operator(TisOps),
    Add,
    EOF,
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



#[derive(Debug)]
struct TokenError {
    kind: TokenErrorKind,
    message: String,
}

impl TokenError {
    fn new<S: std::string::ToString>(message: S) -> Self{
        TokenError {
            message: message.to_string(),
            kind: TokenErrorKind::ParseError,
        }
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for TokenError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
enum TokenErrorKind {
    // None,
    ParseError,
}


// Problem:
// std::mem::discriminate doesn't work with nested enums
//
// #[derive(Copy, Clone, Debug)]
// enum TisOps {
//     Add,
// }


// procedural / stateful approach
struct SimpleInterpreter {
    text: Vec<char>,
    pos: usize,
}

impl SimpleInterpreter {
    fn new<S: std::string::ToString>(text: S) -> SimpleInterpreter{
        return SimpleInterpreter{
            text: text.to_string().chars().collect(),
            pos: 0,
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

        } else if current_char == '+' {
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

    fn expr(&mut self) -> Result<u32, TokenError> {
        let mut left = 0;
        let mut right = 0;

        // 1st token has to be number
        if let Some(token) = self.get_next_token(){
            match token {
                Token::Number(x) => { left = x; },
                _ => { return Err(TokenError::new("Unexpected Token")) },
            }
            
        }

        // 2nd to nth token may be number until an operator appears
        // Operator will break loop
        'a: while let Some(token) = self.get_next_token(){
            match token {
                Token::Number(x) => { left = x;},
                Token::Add => {break 'a;},
                _ => { return Err(TokenError::new("Unexpected Token")) }
            }
        }

        'b: while let Some(token) = self.get_next_token() {
            match token {
                Token::Number(x) => { right = x;},
                Token::EOF => { break 'b;},
                _ => { return Err(TokenError::new("Unexpected Token")) }
            }
        }

        println!("left: {}, right: {}", left, right);
        Ok(left + right)
    }
}





// // purely functional approach
// fn interpret(text: String, position: usize){

// }
