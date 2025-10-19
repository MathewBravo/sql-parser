use sql_parser::{parser, token::{Token, TokenType, Literal}};

fn main() {
    sql_parser::parser::hello_world();
    
    let token = Token::new(TokenType::Equal, "=", Literal::String("=".to_owned()), 0);
    
    token.print_token();
}
