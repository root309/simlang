//#![allow(dead_code)]

pub mod lexer;
pub mod token;
pub mod ast;

use crate::utils::{
    debug_token,
    debug_log,
};
//use crate::parser::lexer::tokenizer;
use crate::parser::ast::{Expr, Op, Literal};
use crate::parser::token::Token;

// utils
// トークン列から単一の文を解析する関数
fn parse_statement(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    debug_log("Entering parse_statement with token", tokens.first());
    let result = match tokens.first() { 
        Some(Token::Ident(ident)) => {
            let next_token = tokens.get(1); // 次のトークンを取得
            match next_token {
                Some(Token::Assignment) => parse_assignment(tokens),
                Some(Token::LParen) => parse_function_call(tokens),
                Some(Token::Plus) | Some(Token::Minus) | Some(Token::Star) | Some(Token::Slash) | Some(Token::LessThan) | Some(Token::GreaterThan) => parse_expression(tokens),
                _ => Err(format!("Unexpected token after identifier '{}': {:?}", ident, next_token)),
            }
        },
        Some(Token::Plus) | Some(Token::Minus) | Some(Token::Star) | Some(Token::Slash) => parse_expression(tokens),    
        Some(Token::Function) => parse_function_def(tokens),
        Some(Token::Return) => parse_return_statement(tokens),
        Some(Token::If) => parse_if_expr(tokens),
        Some(Token::While) => parse_while_loop(tokens),
        _ => {
            let err_msg = format!("Unsupported statement or unexpected token: {:?}", tokens.first());
            debug_log(&err_msg, None);
            Err(err_msg)
        },
    };
    debug_log("Exiting parse_statement", tokens.first());
    result
}

// トークン列からブロックを解析する関数
fn parse_block(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    let (tokens, _) = consume_token(tokens, Token::LBrace)?;
    let mut statements = Vec::new();

    let mut tokens = tokens;
    while !matches!(tokens.first(), Some(Token::RBrace) | None) {
        let (new_tokens, stmt) = parse_statement(tokens)?;
        statements.push(stmt);
        tokens = new_tokens;
    }

    let (tokens, _) = consume_token(tokens, Token::RBrace)?;
    Ok((tokens, Expr::Block(statements)))
}

// トークン列から識別子を解析する関数
fn parse_identifier(tokens: &[Token]) -> Result<(&[Token], String), String> {
    match tokens.split_first() {
        Some((Token::Ident(name), rest)) => Ok((rest, name.clone())),
        _ => Err(String::from("Expected identifier")),
    }
}

fn consume_token(tokens: &[Token], expected: Token) -> Result<(&[Token], Token), String> {
    match tokens.split_first() {
        Some((token, rest)) if *token == expected => Ok((rest, token.clone())),
        _ => Err(format!("Expected {:?}, found {:?}", expected, tokens.first())),
    }
}

// パラメータリストを解析
fn parse_parameters(tokens: &[Token]) -> Result<(&[Token], Vec<String>), String> {
    let mut params = Vec::new();
    let mut tokens = tokens;

    while !matches!(tokens.first(), Some(Token::RParen) | None) {
        let (new_tokens, param) = parse_identifier(tokens)?;
        params.push(param);

        tokens = match consume_token(new_tokens, Token::Comma) {
            Ok((remaining, _)) => remaining,
            Err(_) => new_tokens, // コンマがない場合は次へ進む
        };
    }

    Ok((tokens, params))
}



/////////////////////////////////////




fn parse_expression(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    debug_log("Entering parse_expression with token", tokens.first());

    // 基本的な要素（リテラル、変数、括弧に囲まれた式など）を解析
    let (mut tokens, mut left_expr) = parse_primary(tokens)?;

    // 現在のトークンが二項演算子であるかどうかをチェックしてあればその演算を処理
    while let Some(op_token) = tokens.first() {
        match op_token {
            Token::Plus => {
                tokens = &tokens[1..]; // +演算子を消費
                let (new_tokens, right_expr) = parse_expression(tokens)?;
                tokens = new_tokens;

                // 文字列連結の場合
                left_expr = match (&left_expr, &right_expr) {
                    (Expr::Literal(Literal::String(left)), Expr::Literal(Literal::String(right))) => {
                        Expr::Literal(Literal::String(format!("{}{}", left, right)))
                    },
                    _ => Expr::BinaryOp {
                        left: Box::new(left_expr),
                        op: Op::Add, // 数値の加算
                        right: Box::new(right_expr),
                    },
                };
            },
            Token::Minus | Token::Star | Token::Slash | Token::LessThan | Token::GreaterThan => {
                // 演算子を消費
                tokens = &tokens[1..];
                // 演算子の右側にある式を解析
                let (new_tokens, right_expr) = parse_expression(tokens)?;
                tokens = new_tokens;
                // 二項演算式を構築
                left_expr = Expr::BinaryOp {
                    left: Box::new(left_expr),
                    op: match op_token {
                        Token::Minus => Op::Subtract,
                        Token::Star => Op::Multiply,
                        Token::Slash => Op::Divide,
                        Token::LessThan => Op::LessThan,
                        Token::GreaterThan => Op::GreaterThan,
                        _ => unreachable!(),
                    },
                    right: Box::new(right_expr),
                };
            },
            _ => break,
        }
    }

    Ok((tokens, left_expr))
}

// 代入文を解析
fn parse_assignment(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    debug_log("Entering parse_assignment with token", tokens.first());
    let (tokens, ident) = parse_identifier(tokens)?;
    let tokens = consume_token(tokens, Token::Assignment)?.0;
    let (tokens, expr) = parse_expression(tokens)?; // 式の解析
    let tokens = consume_token(tokens, Token::Semicolon)?.0; // セミコロンの消費
    debug_log("Exiting parse_assignment", tokens.first());
    Ok((tokens, Expr::Assignment { name: ident, value: Box::new(expr) }))
}

// 引数リストを解析
fn parse_arguments(tokens: &[Token]) -> Result<(&[Token], Vec<Expr>), String> {
    if let Ok((tokens, _)) = consume_token(tokens, Token::RParen) {
        debug_log("No arguments to parse, found closing parenthesis", None);
        return Ok((tokens, vec![])); // 引数なし
    }

    debug_log("Starting to parse arguments", tokens.first());
    let mut args = Vec::new();
    let mut tokens = tokens;
    
    // 引数の解析のループ
    while let Some(token) = tokens.first() {
        debug_log("Parsing argument", Some(token));
        let (new_tokens, expr) = parse_expression(tokens)?;
        args.push(expr);
        tokens = new_tokens;
        // 引数間のコンマの処理
        if matches!(tokens.first(), Some(Token::Comma)) {
            tokens = &tokens[1..];
            debug_log("Found comma, continuing to next argument", None);
        } else {
            debug_log("No more arguments to parse", None);
            break;
        }
    }
    debug_log("Finished parsing arguments", None);
    Ok((tokens, args))
}

// トークン列から関数定義を解析する関数
fn parse_function_def(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    let (tokens, _) = consume_token(tokens, Token::Function)?;
    let (tokens, name) = parse_identifier(tokens)?;
    let (tokens, _) = consume_token(tokens, Token::LParen)?;

    let (tokens, params) = parse_parameters(tokens)?;

    let (tokens, _) = consume_token(tokens, Token::RParen)?;
    let (tokens, body) = parse_block(tokens)?;

    Ok((tokens, Expr::FunctionDef { name, params, body: Box::new(body) }))
}

// 関数呼び出しの解析
fn parse_function_call(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    debug_log("Attempting to parse function call with token", tokens.first());
    let (tokens, name) = parse_identifier(tokens)?;
    let (tokens, _) = consume_token(tokens, Token::LParen)?;
    debug_log("Parsing function call arguments for", Some(&Token::Ident(name.clone())));

    let (tokens, args) = parse_arguments(tokens)?;

    debug_log("Finished parsing function call arguments for", Some(&Token::Ident(name.clone())));
    let (tokens, _) = consume_token(tokens, Token::RParen)
        .map_err(|_| "Expected closing parenthesis for function call".to_string())?;
    debug_log("Successfully parsed function call", Some(&Token::RParen));
    Ok((tokens, Expr::FunctionCall { name, args }))
}


// if文の解析
fn parse_if_expr(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    let (tokens, _) = consume_token(tokens, Token::If)?;
    let (tokens, _) = consume_token(tokens, Token::LParen)?;
    let (tokens, condition) = parse_expression(tokens)?;
    let (tokens, _) = consume_token(tokens, Token::RParen)?;

    let (tokens, then_expr) = parse_block(tokens)?;

    let tokens = match consume_token(tokens, Token::Else) {
        Ok((tokens, _)) => tokens,
        Err(_) => return Ok((tokens, Expr::IfExpr {
            condition: Box::new(condition),
            consequence: Box::new(then_expr),
            alternative: None,
        })),
    };

    let (tokens, else_expr) = parse_block(tokens)?;

    Ok((tokens, Expr::IfExpr {
        condition: Box::new(condition),
        consequence: Box::new(then_expr),
        alternative: Some(Box::new(else_expr)),
    }))
}

// while文の解析
fn parse_while_loop(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    let (tokens, _) = consume_token(tokens, Token::While)?;  // whileトークンを消費しtokensを更新
    let (tokens, condition) = parse_expression(tokens)?;  // 条件式を解析
    let (tokens, body) = parse_block(tokens)?;  // ブロックを解析

    Ok((tokens, Expr::WhileLoop {
        condition: Box::new(condition),
        body: Box::new(body),
    }))
}

fn parse_return_statement(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    debug_log("Parsing return statement", tokens.first());
    let (tokens, _) = consume_token(tokens, Token::Return)?;

    let (tokens, expr) = parse_expression(tokens)?;

    // セミコロンを確認し消費する処理を追加
    match tokens.first() {
        Some(Token::Semicolon) => {
            debug_log("Found semicolon after return expression", Some(&Token::Semicolon));
            let tokens = &tokens[1..]; // セミコロンを消費
            Ok((tokens, Expr::Return(Box::new(expr))))
        },
        _ => {
            debug_log("Expected semicolon after return expression, found", tokens.first());
            Err("Expected Semicolon after return expression".to_string())
        }
    }
}

fn parse_primary(tokens: &[Token]) -> Result<(&[Token], Expr), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of tokens".into());
    }

    match tokens.first() {
        Some(Token::Int(value)) => {
            // 整数リテラル
            Ok((&tokens[1..], Expr::Literal(Literal::Int(*value))))
        },
        // 文字列リテラルの処理
        Some(Token::String(value)) => {
            Ok((&tokens[1..], Expr::Literal(Literal::String(value.clone()))))
        },
        Some(Token::Ident(name)) => {
            // 変数参照
            Ok((&tokens[1..], Expr::Variable(name.clone())))
        },
        Some(Token::LParen) => {
            // 括弧に囲まれた式(expression)
            let (tokens, expr) = parse_expression(&tokens[1..])?;
            let (tokens, _) = consume_token(tokens, Token::RParen)
                .map_err(|_| "Expected closing parenthesis".to_string())?;
            Ok((tokens, expr))
        },
        // other
        _ => Err("Unexpected token in primary expression".into()),
    }
}



// トークンリストを受け取り、プログラム全体を表すAST を生成するエントリーポイント
pub fn parse_tokens(input: &[Token]) -> Result<Expr, String> {
    let mut tokens = input;
    let mut expressions = Vec::new();

    // トークンが残っている間、文を解析し続ける
    while !tokens.is_empty() && !matches!(tokens.first(), Some(Token::EOF)) {
        let (new_tokens, expr) = parse_statement(tokens)?;
        expressions.push(expr);
        tokens = new_tokens;
    }

    // プログラム全体をExpr::Blockとして返す
    Ok(Expr::Block(expressions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::token::Token;

    #[test]
    fn test_function_call() {
        // 関数定義と関数呼び出しのテスト
        let tokens = vec![
            Token::Function,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Int(100),
            Token::Comma,
            Token::Int(200),
            Token::RParen,
            Token::Semicolon,
            Token::EOF,
        ];

        let result = parse_tokens(&tokens);
        assert!(result.is_ok(), "Failed to parse program: {:?}", result.err());
    }

    #[test]
    fn test_if_statement() {
        let tokens = vec![
            Token::If,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::LessThan,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Assignment,
            Token::Int(0),
            Token::Semicolon,
            Token::RBrace,
            Token::EOF,
        ];

        let result = parse_tokens(&tokens);
        assert!(result.is_ok(), "Failed to parse if statement: {:?}", result.err());
    }

    #[test]
    fn test_while_statement() {
        let tokens = vec![
            Token::While,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::LessThan,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Assignment,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Int(1),
            Token::Semicolon,
            Token::RBrace,
            Token::EOF,
        ];

        let result = parse_tokens(&tokens);
        assert!(result.is_ok(), "Failed to parse while statement: {:?}", result.err());
    } 
     
    #[test]
    fn test_string_concatenation() {
        let tokens = vec![
            Token::Ident("result".to_string()),
            Token::Assignment,
            Token::String("Hello, ".to_string()),
            Token::Plus,
            Token::String("World!".to_string()),
            Token::Semicolon,
            Token::EOF,
        ];

        let expected_expr = Expr::Assignment {
            name: "result".to_string(),
            value: Box::new(Expr::Literal(Literal::String("Hello, World!".to_string()))),
        };

        let result = parse_tokens(&tokens);
        assert!(result.is_ok(), "Failed to parse string concatenation: {:?}", result.err());
        
        // AST確認
        match result {
            Ok(expr) => assert_eq!(Expr::Block(vec![expected_expr]), expr, "String concatenation did not match expected output."),
            Err(_) => assert!(false, "Expression parsing failed"),
        }
    }
}

