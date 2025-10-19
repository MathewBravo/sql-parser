use std::fmt::{Display, Formatter};
use std::fmt;

pub enum ParserError{
    CharacterError,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self{
            ParserError::CharacterError => write!(f, "Unexpected Character"),
        }
    }
}

impl ParserError{
    pub fn error(line: i32, pe: ParserError){
        println!("[ERROR] Line: {line}\n{pe}")
    }
}

