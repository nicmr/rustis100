use std::fmt::{self, Display};
use std::error;

#[derive(Debug)]
pub struct InterpretError {
    kind: InterpretErrorKind,
    message: String,
}

impl InterpretError {
    pub fn token_error<S: std::string::ToString>(message: S) -> Self{
        InterpretError {
            message: message.to_string(),
            kind: InterpretErrorKind::TokenError,
        }
    }
    pub fn syntax_error<S: std::string::ToString>(message: S) -> Self{
        InterpretError {
            message: message.to_string(),
            kind: InterpretErrorKind::SyntaxError,
        }
    }
}

impl Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            InterpretErrorKind::TokenError => write!(f, "TokenError: {}", self.message),
            InterpretErrorKind::SyntaxError => write!(f, "SyntaxError: {}", self.message)
        }
    }
}

impl error::Error for InterpretError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum InterpretErrorKind {
    // None,
    TokenError,
    SyntaxError,
}

