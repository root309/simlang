pub mod lexer;
pub mod token;
pub mod ast;

//use crate::utils::{
    //debug_token,
    //debug_log,
//};
use crate::parser::ast::{Expr, Op, Literal};
use crate::parser::token::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser { 
    // return current token
    fn current_token(&self) -> Option<&Token> {
       let token = self.tokens.get(self.current);
       //println!("Current token: {:?}", token);
       token
    }

    // return next token(advance the current token by 1)
    fn next_token(&mut self) -> Option<&Token> {
        self.current += 1;
        self.current_token()
    }

    fn peek_token(&self) -> Option<&Token> {
        if self.current + 1 < self.tokens.len() {
            self.tokens.get(self.current + 1)
        } else {
            None
        }
    } 

    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(current_token) = self.current_token() {
            let is_expected = std::mem::discriminant(current_token) == std::mem::discriminant(&expected);
            let token_to_return = current_token.clone();

            if is_expected {
                self.next_token();
                return Ok(token_to_return);
            }
        }
        Err(format!("Expected {:?}, found {:?}", expected, self.current_token()))
    }

    // 式を解析
    pub fn parse_statement(&mut self) -> Result<Expr, String> {
        let stmt = match self.current_token() {
            Some(Token::While) => {
                //println!("Parsing WhileLoop");
                self.parse_while_loop()
            },
            Some(Token::Function) => self.parse_function_def(),
            Some(Token::If) => self.parse_if_expr(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::Ident(_)) => match self.peek_token() {
                Some(Token::LParen) => self.parse_function_call(),
                Some(Token::Assignment) => self.parse_assignment(),
                _ => self.parse_expression(),
            },
            //Some(Token::LBrace) => self.parse_block(),
            _ => self.parse_expression(),
        }?;
        if matches!(self.current_token(), Some(Token::Semicolon)) {
            self.next_token(); // Consume the semicolon
        }
        Ok(stmt)
    } 

    // ブロックを解析
    fn parse_block(&mut self) -> Result<Expr, String> {
        let mut statements = Vec::new();

        self.consume_token(Token::LBrace)?;

        // `}` が見つかるまで文を解析し続ける
        while let Some(token) = self.current_token() {
            if matches!(token, Token::RBrace) {
                break; // ブロックの終わり
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        self.consume_token(Token::RBrace)?;
        
        Ok(Expr::Block(statements))
    }
    
    fn parse_identifier(&mut self) -> Result<String, String> {
        if let Some(Token::Ident(name)) = self.current_token() {
            //println!("Identified: {}", name);
            let name_clone = name.clone();
            self.next_token();
            Ok(name_clone)
        } else {
            Err("Expected identifier".to_string())
        }
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, String> {
        let mut parameters = Vec::new();
        self.consume_token(Token::LParen)?;

        while self.current_token() != Some(&Token::RParen) {
            parameters.push(self.parse_identifier()?);
            if self.current_token() == Some(&Token::Comma) {
                self.consume_token(Token::Comma)?;
            }
        }

        self.consume_token(Token::RParen)?;
        Ok(parameters)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        //println!("Parsing expression");
        self.parse_binary_operator() // 二項演算子を解析 
    }
     
    fn parse_binary_operator(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        while let Some(op) = match self.current_token() {
            Some(Token::Plus) => Some(Op::Add),
            Some(Token::Minus) => Some(Op::Subtract),
            Some(Token::Star) => Some(Op::Multiply),
            Some(Token::Slash) => Some(Op::Divide),
            Some(Token::LessThan) => Some(Op::LessThan),
            Some(Token::GreaterThan) => Some(Op::GreaterThan),
            // Some(Token::LessThanEqual) => Some(Op::LessThanEqual),
            // Some(Token::GreaterThanEqual) => Some(Op::GreaterThanEqual),
            // Some(Token::EqualEqual) => Some(Op::Equal),
            // Some(Token::NotEqual) => Some(Op::NotEqual),
            _ => None,
        } {
            self.next_token(); // Skip the operator
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let ident = self.parse_identifier()?;
        self.consume_token(Token::Assignment)?;
        let value = self.parse_expression()?;
        Ok(Expr::Assignment {
            name: ident,
            value: Box::new(value),
        })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        loop {
            let expr = self.parse_expression()?;
            args.push(expr);

            match self.current_token() {
                Some(Token::Comma) => { 
                    self.consume_token(Token::Comma)?; 
                    //println!("match");
                },
                _ => break,
            }
        }
        Ok(args)
    }
    
    fn parse_function_def(&mut self) -> Result<Expr, String> {
        //println!("Parsing function definition.");
        self.consume_token(Token::Function)?;

        let name = self.parse_identifier()?;

        let parameters = self.parse_parameters()?;
        let body = self.parse_block()?;

        Ok(Expr::FunctionDef {
            name,
            params: parameters,
            body: Box::new(body),
        })
    }

    fn parse_function_call(&mut self) -> Result<Expr, String> {
        let name = self.parse_identifier()?;
        self.consume_token(Token::LParen)?;
        let args = if self.peek_token() != Some(&Token::RParen) {
            self.parse_arguments()?
        } else {
            Vec::new()
        };
        self.consume_token(Token::RParen)?;
        Ok(Expr::FunctionCall {
            name,
            args,
        })
    }

    fn parse_if_expr(&mut self) -> Result<Expr, String> {
        self.consume_token(Token::If)?;
        self.consume_token(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.consume_token(Token::RParen)?;
        let consequence = self.parse_block()?;
        let alternative = if self.current_token() == Some(&Token::Else) {
            self.consume_token(Token::Else)?;
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };
        Ok(Expr::IfExpr {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        })
    }
    
    fn parse_while_loop(&mut self) -> Result<Expr, String> {
        self.consume_token(Token::While)?;
        self.consume_token(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.consume_token(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Expr::WhileLoop {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }
    
    fn parse_return_statement(&mut self) -> Result<Expr, String> {
        self.consume_token(Token::Return)?;
        let value = self.parse_expression()?;
        self.consume_token(Token::Semicolon)?;
        Ok(Expr::Return(Box::new(value)))
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token_clone = if let Some(token) = self.current_token() {
            //println!("Parsing primary expression, current token: {:?}", token);
            token.clone()
        } else {
            return Err("Unexpected end of tokens".to_string());
        };

        match &token_clone {
            Token::Int(value) => {
                //println!("Parsing integer literal: {}", value);
                self.next_token();
                Ok(Expr::Literal(Literal::Int(*value)))
            },
            Token::String(value) => {
                //println!("Parsing integer literal: {}", value);
                self.next_token();
                Ok(Expr::Literal(Literal::String(value.clone())))
            },
            Token::Ident(_) => {
                let ident = self.parse_identifier()?;
                Ok(Expr::Variable(ident))
            },
            Token::LParen => {
                self.next_token();
                let expr = self.parse_expression()?;
                self.consume_token(Token::RParen)?;
                Ok(expr)
            },
            _ => Err("Unexpected token in primary expression".to_string()),
        }
    }
    
    pub fn parse_tokens(&mut self) -> Result<Expr, String> {
        let mut statements = Vec::new();
        while let Some(token) = self.current_token() {
            if matches!(token, Token::EOF) {
                break; // EOF
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        Ok(Expr::Block(statements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::token::Token;
    use crate::parser::lexer::tokenizer; 
    // 二項演算のテスト: 加算
    #[test]
    fn test_binary_addition() {
        let tokens = vec![
            Token::Int(10),
            Token::Plus,
            Token::Int(20),
            Token::Semicolon,
            Token::EOF,
        ];
        let mut parser = Parser { tokens, current: 0 };
        let result = parser.parse_tokens();

        assert!(result.is_ok());
        let expected = Expr::Block(vec![
            Expr::BinaryOp {
                left: Box::new(Expr::Literal(Literal::Int(10))),
                op: Op::Add,
                right: Box::new(Expr::Literal(Literal::Int(20))),
            }
        ]);
        assert_eq!(result.unwrap(), expected);
    }

    // 二項演算のテスト: 減算
    #[test]
    fn test_binary_subtraction() {
        let tokens = vec![
            Token::Int(30),
            Token::Minus,
            Token::Int(20),
            Token::Semicolon,
            Token::EOF,
        ];
        let mut parser = Parser { tokens, current: 0 };
        let result = parser.parse_tokens();

        assert!(result.is_ok());
        let expected = Expr::Block(vec![
            Expr::BinaryOp {
                left: Box::new(Expr::Literal(Literal::Int(30))),
                op: Op::Subtract,
                right: Box::new(Expr::Literal(Literal::Int(20))),
            }
        ]);
        assert_eq!(result.unwrap(), expected);
    }

    // 二項演算のテスト: 乗算
    #[test]
    fn test_binary_multiplication() {
        let tokens = vec![
            Token::Int(5),
            Token::Star,
            Token::Int(4),
            Token::Semicolon,
            Token::EOF,
        ];
        let mut parser = Parser { tokens, current: 0 };
        let result = parser.parse_tokens();

        assert!(result.is_ok());
        let expected = Expr::Block(vec![
            Expr::BinaryOp {
                left: Box::new(Expr::Literal(Literal::Int(5))),
                op: Op::Multiply,
                right: Box::new(Expr::Literal(Literal::Int(4))),
            }
        ]);
        assert_eq!(result.unwrap(), expected);
    }

    // 二項演算のテスト: 除算
    #[test]
    fn test_binary_division() {
        let tokens = vec![
            Token::Int(20),
            Token::Slash,
            Token::Int(5),
            Token::Semicolon,
            Token::EOF,
        ];
        let mut parser = Parser { tokens, current: 0 };
        let result = parser.parse_tokens();

        assert!(result.is_ok());
        let expected = Expr::Block(vec![
            Expr::BinaryOp {
                left: Box::new(Expr::Literal(Literal::Int(20))),
                op: Op::Divide,
                right: Box::new(Expr::Literal(Literal::Int(5))),
            }
        ]);
        assert_eq!(result.unwrap(), expected);
    }
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
        let mut parser = Parser { tokens: tokens, current: 0 };

        let result = parser.parse_tokens();
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
        let mut parser = Parser { tokens: tokens, current: 0 };

        let result = parser.parse_tokens();
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
        let mut parser = Parser { tokens: tokens, current: 0 };
        let result = parser.parse_tokens();
        assert!(result.is_ok(), "Failed to parse while statement: {:?}", result.err());
    } 
     
    // #[test]
    // fn test_string_concatenation() {
    //     let tokens = vec![
    //         Token::Ident("result".to_string()),
    //         Token::Assignment,
    //         Token::String("Hello, ".to_string()),
    //         Token::Plus,
    //         Token::String("World!".to_string()),
    //         Token::Semicolon,
    //         Token::EOF,
    //     ];
    //
    //     let expected_expr = Expr::Assignment {
    //         name: "result".to_string(),
    //         value: Box::new(Expr::Literal(Literal::String("Hello, World!".to_string()))),
    //     };
    //     let mut parser = Parser { tokens: tokens, current: 0 };
    //     let result = parser.parse_tokens();
    //     assert!(result.is_ok(), "Failed to parse string concatenation: {:?}", result.err());
    //     
    //     // AST確認
    //     match result {
    //         Ok(expr) => assert_eq!(Expr::Block(vec![expected_expr]), expr, "String concatenation did not match expected output."),
    //         Err(_) => assert!(false, "Expression parsing failed"),
    //     }
    // }
    
    #[test]
    fn test_function_definition_and_call() {
        // ソースコードをトークン列に変換
        let source = r#"
        function add(x, y) {
            return x + y;
        };

        add(100, 200);
        "#;
        let (_, tokens) = tokenizer(source).expect("Tokenization failed");
        let mut parser = Parser { tokens: tokens, current: 0 };
        // パーサーを実行
        let ast = parser.parse_tokens().expect("Failed to parse tokens");

        // 期待されるAST
        let expected_ast = Expr::Block(vec![
            Expr::FunctionDef {
                name: "add".to_string(),
                params: vec!["x".to_string(), "y".to_string()],
                body: Box::new(Expr::Block(vec![
                    Expr::Return(Box::new(Expr::BinaryOp {
                        left: Box::new(Expr::Variable("x".to_string())),
                        op: Op::Add,
                        right: Box::new(Expr::Variable("y".to_string())),
                    })),
                ])),
            },
            Expr::FunctionCall {
                name: "add".to_string(),
                args: vec![Expr::Literal(Literal::Int(100)), Expr::Literal(Literal::Int(200))],
            },
        ]);

        // 結果を検証
        assert_eq!(ast, expected_ast, "AST did not match the expected output");
    }
}

