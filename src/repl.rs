use std::io::{self, Write};
use simlang::parser::lexer::tokenizer;
use simlang::parser::Parser;
use simlang::interpreter::evaluator::{Evaluator, EvaluationResult};
use simlang::parser::ast;


pub fn run_repl() {
    let ascii_art = r#"
        _           _                   
    ___(_)_ __ ___ | | __ _ _ __   __ _ 
   / __| | '_ ` _ \| |/ _` | '_ \ / _` |
   \__ \ | | | | | | | (_| | | | | (_| |
   |___/_|_| |_| |_|_|\__,_|_| |_|\__, |
                                  |___/ 
    "#;
    let message = "
    Welcome to the simlang REPL.
    Small interpreter language.
    Type 'exit' to exit.
    ";
    println!("\x1b[31m{}\x1b[0m", ascii_art);
    println!("{}", message);
    let mut evaluator = Evaluator::new();

    loop {
        print!("\x1b[31mλ\x1b[0m ");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    break;
                }

                let (_, tokens) = match tokenizer(&input) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                };

                let mut parser = Parser { tokens, current: 0 };
                let ast = match parser.parse_tokens() {
                    Ok(ast) => ast,
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    },
                };

                match evaluator.evaluate(ast) {
                    Ok(result) => match result {
                        EvaluationResult::Value(val) | EvaluationResult::ReturnValue(val) => match val {
                            ast::Literal::Int(i) => println!("{}", i),
                            ast::Literal::String(s) => println!("{}", s),
                            _ => println!("{:?}", val),
                        },
                    },
                    Err(e) => println!("Error: {}", e),
                }
            },
            Err(error) => println!("Error: {}", error),
        }
    }
}

