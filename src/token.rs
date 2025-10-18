

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
