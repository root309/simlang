use crate::parser::ast::{Expr, Op, Literal};
use std::collections::HashMap;

pub struct Evaluator {
    ctx: Context,
}

pub enum EvaluationResult {
    Value(Literal),
    ReturnValue(Literal),
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            ctx: Context::new(),
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<EvaluationResult, String> {
        match expr {
            Expr::FunctionDef { name, params, body } => {
                self.evaluate_function_def(name, params, body)?;
                Ok(EvaluationResult::Value(Literal::Unit))
            },
            Expr::FunctionCall { name, args } => {
                match self.evaluate_function_call(name, args)? {
                    EvaluationResult::Value(val) => Ok(EvaluationResult::Value(val)),
                    EvaluationResult::ReturnValue(val) => Ok(EvaluationResult::ReturnValue(val)),
                }
            },
            Expr::IfExpr { condition, consequence, alternative } => {
                self.evaluate_if_expr(condition, consequence, alternative)
            },
            Expr::WhileLoop { condition, body } => {
                self.evaluate_while_loop(condition, body)
            },
            Expr::Assignment { name, value } => {
                match self.evaluate_assignment(name, *value)? {
                    EvaluationResult::Value(val) => Ok(EvaluationResult::Value(val)),
                    EvaluationResult::ReturnValue(val) => Ok(EvaluationResult::ReturnValue(val)),
                }
            },
            Expr::BinaryOp { left, op, right } => {
                match self.evaluate_binary_op(*left, op, *right)? {
                    EvaluationResult::Value(val) => Ok(EvaluationResult::Value(val)),
                    EvaluationResult::ReturnValue(val) => Ok(EvaluationResult::ReturnValue(val)),
                }
            },
            Expr::Literal(lit) => Ok(EvaluationResult::Value(lit)),
            Expr::Variable(name) => {
                let result = self.evaluate_variable(&name)?;
                Ok(EvaluationResult::Value(result))
            },
            Expr::Block(expressions) => self.evaluate_block(expressions),
            Expr::Return(expr) => self.evaluate_return(*expr),
            _ => Err("Unimplemented expression type".to_string()),
        }
    }
    
    fn evaluate_function_def(&mut self, name: String, params: Vec<String>, body: Expr) -> Result<Literal, String> {
        // 関数定義をコンテキストに保存
        self.ctx.set_function(name, params, body);
        Ok(Literal::Unit) // 特に値を返さないからUnit型を返す
    }
        
    fn evaluate_function_call(&mut self, name: String, args: Vec<Expr>) -> Result<EvaluationResult, String> {
        if let Some(Value::Function(params, body)) = self.ctx.get_function(&name) {
            if params.len() != args.len() {
                return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
            }

            self.ctx.push_scope();
            for (param, arg) in params.iter().zip(args.iter()) {
                match self.evaluate(arg.clone())? {
                    EvaluationResult::Value(val) => {
                        let value = Value::from_literal(val)?;
                        self.ctx.set_variable(param.clone(), value);
                    },
                    EvaluationResult::ReturnValue(val) => {
                        self.ctx.pop_scope();
                        return Ok(EvaluationResult::ReturnValue(val));
                    },
                }
            }

            let result = self.evaluate(*body)?;
            self.ctx.pop_scope();
            Ok(result)
        } else {
            Err(format!("Function '{}' not found", name))
        }
    } 
        
    fn evaluate_if_expr(
        &mut self, 
        condition: Expr, 
        consequence: Expr, 
        alternative: Option<Box<Expr>>
    ) -> Result<EvaluationResult, String> {
        let condition_result = self.evaluate(condition)?;
        match condition_result {
            EvaluationResult::Value(Literal::Int(value)) => {
                if value != 0 {
                    self.evaluate(*consequence)
                } else if let Some(alt) = alternative {
                    self.evaluate(*alt)
                } else {
                    Ok(EvaluationResult::Value(Literal::Int(0))) // if文にelse文がない場合
                }
            },
            EvaluationResult::ReturnValue(_) => Ok(condition_result),
            _ => Err("Condition must be an integer".into()),
        }
    }
    
    fn evaluate_while_loop(
        &mut self, 
        condition: Expr, 
        body: Expr
    ) -> Result<EvaluationResult, String> {
        loop {
            let condition_result = self.evaluate(condition.clone())?;
            match condition_result {
                EvaluationResult::Value(Literal::Int(value)) => {
                    if value == 0 {
                        break;
                    }
                    let body_result = self.evaluate(body.clone())?;
                    if let EvaluationResult::ReturnValue(_) = body_result {
                        return Ok(body_result);
                    }
                },
                EvaluationResult::ReturnValue(_) => return Ok(condition_result),
                _ => return Err("Condition must be an integer".into()),
            }
        }
        Ok(EvaluationResult::Value(Literal::Int(0)))
    }

    fn evaluate_assignment(&mut self, name: String, value: Expr) -> Result<Literal, String> {
        let val = self.evaluate(*value)?;
        self.ctx.set_variable(name, val.clone());
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

    fn evaluate_variable(&self, name: &str) -> Result<Literal, String> {
        self.ctx.get_variable(&name).ok_or(format!("Variable '{}' not found", name))
    }
    
    fn evaluate_block(&mut self, expressions: Vec<Expr>) -> Result<EvaluationResult, String> {
        for expression in expressions {
            let evaluated_result = self.evaluate(expression)?;
            match evaluated_result {
                EvaluationResult::ReturnValue(val) => return Ok(EvaluationResult::ReturnValue(val)),
                _ => continue,
            }
        }
        Ok(EvaluationResult::Value(Literal::Unit)) // ブロックが何も返さない場合、Unitを返す
    }

    fn evaluate_return(&mut self, expr: Expr) -> Result<EvaluationResult, String> {
        let val = self.evaluate(expr)?;
        match val {
            EvaluationResult::Value(val) => Ok(EvaluationResult::ReturnValue(val)),
            _ => Ok(val),
        }
    }

}
