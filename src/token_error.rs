use std::fmt::{self, Display};
use std::error;

#[derive(Debug)]
pub struct TokenError {
    kind: TokenErrorKind,
    message: String,
}

impl TokenError {
    pub fn new<S: std::string::ToString>(message: S) -> Self{
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
pub enum TokenErrorKind {
    // None,
    ParseError,
}