use std::fmt::{self, Display};
use std::error;




fn main() {
    let sequence = "1+2";
    println!("Sequence is: {}", sequence);
    let mut interp = Interpreter::new(sequence);
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
struct Interpreter {
    text: Vec<char>,
    pos: usize,
}

impl Interpreter {
    fn new<S: std::string::ToString>(text: S) -> Interpreter{
        return Interpreter{
            text: text.to_string().chars().collect(),
            pos: 0,
        }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        // end of input reached
        if self.pos > self.text.len() -1 {
            return Some(Token::EOF);
        }

        let current_char = self.text[self.pos];
        println!("current char is: {}", current_char);
        self.pos += 1;

        if let Some(i) = current_char.to_digit(10) {
             // digit
            // println!("=> identified as digit");
            Some(Token::Number(i))
        } else if current_char == '+' {
            // addition operator
            // println!("=> identified as +");
            Some(Token::Add)
        } else {
            // unknown input
            // println!("=> couldn't identify :(");
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
