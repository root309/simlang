use std::io::{self, Write};
use sim::parser::lexer::tokenizer;
use sim::parser::Parser;
use sim::interpreter::evaluator::{Evaluator, EvaluationResult};
use sim::parser::ast;


pub fn run_repl() {
    let ascii_art = "
\x1b[34m███████╗██╗███╗   ███╗
██╔════╝██║████╗ ████║
███████╗██║██╔████╔██║
╚════██║██║██║╚██╔╝██║
███████║██║██║ ╚═╝ ██║
╚══════╝╚═╝╚═╝     ╚═╝\x1b[0m


Welcome to the SIM language REPL.
Small interpreter language.
Type 'exit' to exit.
";

    println!("{}", ascii_art);

    let mut evaluator = Evaluator::new();

    loop {
        print!("\x1b[34mλ\x1b[0m ");
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

