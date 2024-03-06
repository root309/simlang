use std::io::{self, Write};
use sim::parser::lexer::tokenizer;
use sim::parser::Parser;
use sim::interpreter::evaluator::{Evaluator, EvaluationResult};

pub fn run_repl() {
    let mut evaluator = Evaluator::new();

    loop {
        print!("λ ");
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
                        EvaluationResult::Value(val) => println!("{:?}", val),
                        EvaluationResult::ReturnValue(val) => println!("{:?}", val),
                        _ => println!("No value returned."),
                    },
                    Err(e) => println!("Error: {}", e),
                }
            },
            Err(error) => println!("Error: {}", error),
        }
    }
}
