#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i64),
    String(String),
    Ident(String), // identifier
    Assignment,
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
    Comma,
    Function,
    If,
    Else,
    While,
    Return,
    EOF,
}
