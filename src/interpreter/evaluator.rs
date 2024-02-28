use crate::parser::ast::{Expr, Op, Literal};
use std::collections::HashMap;

pub struct Evaluator {
    context: Context,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            context: Context::new(),
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<Literal, String> {
        match expr {
            Expr::FunctionDef { name, params, body } => self.evaluate_function_def(name, params, body),
            Expr::FunctionCall { name, args } => self.evaluate_function_call(name, args),
            Expr::IfExpr { condition, consequence, alternative } => self.evaluate_if_expr(condition, consequence, alternative),
            Expr::WhileLoop { condition, body } => self.evaluate_while_loop(condition, body),
            Expr::Assignment { name, value } => self.evaluate_assignment(name, *value), 
            Expr::BinaryOp { left, op, right } => self.evaluate_binary_op(*left, op, *right),
            Expr::Literal(lit) => Ok(lit),
            Expr::Variable(name) => Ok(name),
            Expr::Block(expressions) => {
                let mut result = Literal::Int(0); // 初期値orデフォルト値
                for expression in expressions {
                    result = self.evaluate(expression)?;
                }
                Ok(result)
            }
            Expr::Return(expr) => Ok(expr), 
            _ => Err("Unimplemented expression type".to_string()),
        }
    }

    fn evaluate_function_def(&mut self, name: String, params: Vec<String>, body: Expr) -> Result<Literal, String> {

    }

    fn evaluate_function_call(&mut self, name: String, args: Vec<Expr>) -> Result<Literal, String> {
       
    }

    fn evaluate_if_expr(&mut self, condition: Expr, consequence: Expr, alternative: Expr) -> Result<Literal, String> {

    }

    fn evaluate_while_loop(&mut self, condition: Expr, body: Expr) -> Result<Literal, String> {

    }

    fn evaluate_assignment(&mut self, name: String, value: Expr) -> Result<Literal, String> {
        let val = self.evaluate(*value)?;
        self.context.set_variable(name, val.clone());
        Ok(val)
    }

    
    fn evaluate_binary_op(&mut self, left: Expr, op: Op, right: Expr) -> Result<Literal, String> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match (left_val, right_val) {
            (Literal::Int(l), Literal::Int(r)) => match op {
                Op::Add => Ok(Literal::Int(l + r)),
                Op::Subtract => Ok(Literal::Int(l - r)),
                Op::Multiply => Ok(Literal::Int(l * r)),
                Op::Divide => Ok(Literal::Int(l / r)),
                Op::LessThan => Ok(Literal::Int((l < r) as i64)),
                Op::GreaterThan => Ok(Literal::Int((l > r) as i64)),
            },
            _ => Err("Unsupported literal types for binary operation".into()),
        }
    }

    fn evaluate_literal(&self, lit: Literal) -> Result<Literal, String> {

    }

    fn evaluate_variable(&self, name: &str) -> Result<Literal, String> {
        self.context.get_variable(&name).ok_or(format!("Variable '{}' not found", name))
    }

    fn evaluate_block(&mut self, expressions: Vec<Expr>) -> Result<Literal, String> {
        let mut last = Literal::Int(0); // 初期値は0でブロックが空の場合のデフォルト値

        for expr in expressions {
            last = self.evaluate(expr)?;
        }

        Ok(last)
    }

    fn evaluate_return(&mut self, expr: Expr) -> Result<Literal, String> {
        
    }

}
