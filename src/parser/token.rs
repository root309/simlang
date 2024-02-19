#[derive(Debug, PartialEq)]
pub enum Token {
    Int(i64),
    String(String),
    Ident(String), // identifier
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    LessThan,
    GreaterThan,
    Equal,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Function,
    If,
    Else,
    While,
    Return,
}
