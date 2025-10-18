

// SELECT FROM data WHERE [field] = [val]
pub enum TokenType{
    Select, 
    From,
    Where,

    Equal,
    BangEqual,
    
    Number, 
    String,

    DataSource,
    Field,
}

pub enum Literal{
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}


struct Token{
    tt: TokenType,
    lex: String,
    lit: Literal,
    line: i32,
}

impl Token {
    pub fn new(tt: TokenType, lex: String, lit: Literal, line: i32)-> Token{
        Token{
            tt,
            lex,
            lit,
            line
        }
    }
}