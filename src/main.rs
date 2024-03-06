use sim::parser::lexer::tokenizer;
use sim::parser::Parser;
use sim::interpreter::evaluator::{Evaluator, EvaluationResult};
use std::env;
mod repl;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl::run_repl();
    } else {
        let file_name = &args[1];
        let source_code = std::fs::read_to_string(file_name)
            .expect("Failed to read the source file.");
        println!("Source code: {}", source_code);
        let (_, tokens) = tokenizer(&source_code)
            .expect("Failed to tokenize the source code.");

        let mut parser = Parser { tokens, current: 0 };

        let ast = match parser.parse_tokens() {
            Ok(ast) => ast,
            Err(e) => {
                println!("Failed to parse tokens: {}", e);
                return;
            },
        };

        println!("AST: {:?}", ast);

        let mut evaluator = Evaluator::new();
        let result = evaluator.evaluate(ast)
            .expect("Failed to evaluate the AST.");

        match result {
            EvaluationResult::Value(val) => println!("Result: {:?}", val),
            EvaluationResult::ReturnValue(val) => println!("Return: {:?}", val),
            _ => println!("Evaluation did not result in a value or return value."),
        }
    }
}
