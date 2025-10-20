use std::process;

use crate::{
    errors::ParserError,
    token::{Literal, Token, TokenType},
};

// TODO: String literals
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(mut self) -> Vec<Token<'a>> {
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "", Literal::Nil, self.line));

        self.tokens
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '=' => {
                    if self.match_next('='){
                        self.add_null_token(TokenType::EqualEqual);
                    }else{
                        self.add_null_token(TokenType::Equal);
                    }
                }
                '!' => {
                    if self.match_next('='){
                        self.add_null_token(TokenType::BangEqual);
                    }else{
                        ParserError::error(self.line, ParserError::CharacterError);
                    }
                }
                '\n' => self.line+=1,
                ' ' => {},
                '\r' => {},
                '\t' => {},
                _ => ParserError::error(self.line, ParserError::CharacterError),
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current)
    }

    fn add_null_token(&self, tt: TokenType) {}

    fn add_token(&mut self, tt: TokenType, lit: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(tt, text, lit, self.line));
    }

    fn match_next(&mut self, expected_char: char) -> bool {
        if (self.is_at_end()) {
            return false;
        }
        if let Some(c) = self.source.chars().nth(self.current) {
            if (c != expected_char) {
                return false;
            }
        }
        self.current += 1;
        true
    }
    
    fn peek(&self) -> char {
        if(self.is_at_end()){
            return '\0'; 
        }
        return self.source.chars().nth(self.current).unwrap();
    }
    
    fn string(&mut self){
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n' {self.line +=1 }
            self.advance();
        }
        
        if self.is_at_end(){
            ParserError::error(self.line, ParserError::UnterminatedString);
            return
        }
        
        self.advance();
        
        let val = self.source[self.start+1..self.current-1].to_owned();

        self.add_token(TokenType::String, Literal::String(val));
    }
}
