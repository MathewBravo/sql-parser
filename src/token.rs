use std::fmt::{Display, Formatter};
use std::fmt;



// SELECT FROM data WHERE [field] = [val]
pub enum TokenType{
    Select, 
    From,
    Where,

    Equal,
    EqualEqual,
    BangEqual,
    
    Number, 
    String,

    DataSource,
    Field,
    
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TokenType::Select => write!(f, "SELECT"),
            TokenType::From => write!(f, "FROM"),
            TokenType::Where => write!(f, "WHERE"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::String => write!(f, "STRING"),
            TokenType::DataSource => write!(f, "DATA_SOURCE"),
            TokenType::Field => write!(f, "FIELD"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

pub enum Literal{
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Literal::Number(_) => write!(f, "Number"),
            Literal::String(_) => write!(f, "String"),
            Literal::Boolean(_) => write!(f, "Boolean"),
            Literal::Nil => write!(f, "Nil"),
        }
    }
}


pub struct Token<'a>{
    tt: TokenType,
    lex: &'a str,
    lit: Literal,
    line: i32,
}

impl<'a> Token<'a> {
    pub fn new(tt: TokenType, lex: &'a str, lit: Literal, line: i32)-> Token<'a>{
        Token{
            tt,
            lex,
            lit,
            line
        }
    }
    
    pub fn print_token(&self) {
        println!("{} {} {}", self.tt, self.lex, self.lit)
    }
}