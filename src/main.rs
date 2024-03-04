use sim::parser::{self, lexer};
use sim::interpreter::evaluator::{Evaluator, EvaluationResult};

fn main() {
    let source_code = std::fs::read_to_string("examples/a.sim")
        .expect("Failed to read the source file.");

    let (_, tokens) = lexer::tokenizer(&source_code)
        .expect("Failed to tokenize the source code.");

    let ast = parser::parse_tokens(&tokens)
        .expect("Failed to parse tokens into AST.");

    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(ast)
        .expect("Failed to evaluate the AST.");

    match result {
        EvaluationResult::Value(val) => println!("Result: {:?}", val),
        EvaluationResult::ReturnValue(val) => println!("Return: {:?}", val),
    }
}
