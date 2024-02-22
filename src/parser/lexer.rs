#![allow(dead_code)]
use crate::parser::token::Token;
use nom::{
    IResult,
    //error::{ParseError},
    character::complete::{
        //space0,
        //space1,
        multispace0,
        //none_of,
        char,
        //line_ending,
        digit1,
    },
    bytes::complete::{
        take_while,
        tag,
        //is_not,
        //escaped_transform,
        //take_while_m_n,
    },
    //number::complete::{
        //double,
    //},
    branch::{
        alt,
        //permutation,
    },
    combinator::{
        //opt,
        map,
        //value,
        //all_consuming,
    },
    multi::{
        many0,
        //many1,
        //separated_list0,
    },
    sequence::{
        delimited,
        preceded,
        //tuple,
    },
    //error::VerboseError,
};
// 識別子を解析
fn identifier(input: &str) -> IResult<&str, Token> {
    let parser = take_while(|c: char| c.is_alphanumeric() || c == '_');
    let (input, ident) = parser(input)?;
    Ok((input, Token::Ident(ident.to_string())))
}

// 整数リテラルを解析
fn integer(input: &str) -> IResult<&str, Token> {
    let (input, int_str) = digit1(input)?;
    let int = int_str.parse::<i64>().unwrap();
    Ok((input, Token::Int(int)))
}

// 文字列リテラルを解析
fn string_literal(input: &str) -> IResult<&str, Token> {
    let string_parser = delimited(
        ws(char('"')),
        take_while(|c: char| c != '"'),
        ws(char('"'))
    );
    map(string_parser, |s: &str| Token::String(s.to_string()))(input)
}

// 空白をスキップする関数
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}


// '+' トークン
fn plus(input: &str) -> IResult<&str, Token> {
    map(ws(char('+')), |_| Token::Plus)(input)
}

// '-' トークン
fn minus(input: &str) -> IResult<&str, Token> {
    map(ws(char('-')), |_| Token::Minus)(input)
}

// '*'
fn star(input: &str) -> IResult<&str, Token> {
    map(ws(char('*')), |_| Token::Star)(input)
}

// '/'
fn slash(input: &str) -> IResult<&str, Token> {
    map(ws(char('/')), |_| Token::Slash)(input)
}

// '%'
fn modulo(input: &str) -> IResult<&str, Token> {
    map(ws(char('%')), |_| Token::Modulo)(input)
}

// '<'
fn less_than(input: &str) -> IResult<&str, Token> {
    map(ws(char('<')), |_| Token::LessThan)(input)
}

// '>'
fn greater_than(input: &str) -> IResult<&str, Token> {
    map(ws(char('>')), |_| Token::GreaterThan)(input)
}

// '='
fn equal(input: &str) -> IResult<&str, Token> {
    map(ws(char('=')), |_| Token::Equal)(input)
}

// '('
fn l_paren(input: &str) -> IResult<&str, Token> {
    map(ws(char('(')), |_| Token::LParen)(input)
}

// ')'
fn r_paren(input: &str) -> IResult<&str, Token> {
    map(ws(char(')')), |_| Token::RParen)(input)
}

// '{'
fn l_brace(input: &str) -> IResult<&str, Token> {
    map(ws(char('{')), |_| Token::LBrace)(input)
}

// '}'
fn r_brace(input: &str) -> IResult<&str, Token> {
    map(ws(char('}')), |_| Token::RBrace)(input)
}

// ';'
fn semicolon(input: &str) -> IResult<&str, Token> {
    map(ws(char(';')), |_| Token::Semicolon)(input)
}

// キーワードの解析関数
fn keyword(input: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        alt((
            map(tag("function"), |_| Token::Function),
            map(tag("if"), |_| Token::If),
            map(tag("else"), |_| Token::Else),
            map(tag("while"), |_| Token::While),
            map(tag("return"), |_| Token::Return),
        )),
    )(input)
}


pub fn tokenizer(input: &str) -> IResult<&str, Vec<Token>> {
    let (remaining_input, mut tokens) = many0(
        alt((
            integer,
            identifier,
            string_literal,
            plus,
            minus,
            star,
            slash,
            modulo,
            less_than,
            greater_than,
            equal,
            l_paren,
            r_paren,
            l_brace,
            r_brace,
            semicolon,
            keyword,
        )),
    )(input)?;

    // 入力が完全に消費された場合EOFトークンを追加
    if remaining_input.is_empty() {
        tokens.push(Token::EOF);
    }

    Ok((remaining_input, tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(integer("123 "), Ok((" ", Token::Int(123))));
    }

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("testVar "), Ok((" ", Token::Ident("testVar".to_string()))));
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(string_literal("\"hello world\" "), Ok(("", Token::String("hello world".to_string()))));
    }

    #[test]
    fn test_operators() {
        assert_eq!(plus("+"), Ok(("", Token::Plus)));
        assert_eq!(minus("-"), Ok(("", Token::Minus)));
        assert_eq!(star("*"), Ok(("", Token::Star)));
        assert_eq!(slash("/"), Ok(("", Token::Slash)));
        assert_eq!(modulo("%"), Ok(("", Token::Modulo)));
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(less_than("<"), Ok(("", Token::LessThan)));
        assert_eq!(greater_than(">"), Ok(("", Token::GreaterThan)));
        assert_eq!(equal("="), Ok(("", Token::Equal)));
    }

    #[test]
    fn test_parentheses_and_braces() {
        assert_eq!(l_paren("("), Ok(("", Token::LParen)));
        assert_eq!(r_paren(")"), Ok(("", Token::RParen)));
        assert_eq!(l_brace("{"), Ok(("", Token::LBrace)));
        assert_eq!(r_brace("}"), Ok(("", Token::RBrace)));
    }

    #[test]
    fn test_semicolon() {
        assert_eq!(semicolon(";"), Ok(("", Token::Semicolon)));
    }

    #[test]
    fn test_keywords() {
        assert_eq!(keyword("function"), Ok(("", Token::Function)));
        assert_eq!(keyword("if"), Ok(("", Token::If)));
        assert_eq!(keyword("else"), Ok(("", Token::Else)));
        assert_eq!(keyword("while"), Ok(("", Token::While)));
        assert_eq!(keyword("return"), Ok(("", Token::Return)));
    }
}
