use crate::interpreter::context::*;
use crate::parser::ast::*;

pub struct Evaluator {
    ctx: Context,
}

#[derive(Clone, Debug, PartialEq)]
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
                self.evaluate_function_def(name, params, *body)?;
                Ok(EvaluationResult::Value(Literal::Unit))
            },
            Expr::FunctionCall { name, args } => {
                match self.evaluate_function_call(name, args)? {
                    EvaluationResult::Value(val) => Ok(EvaluationResult::Value(val)),
                    EvaluationResult::ReturnValue(val) => Ok(EvaluationResult::ReturnValue(val)),
                }
            },
            Expr::IfExpr { condition, consequence, alternative } => {
                self.evaluate_if_expr(*condition, Box::new((*consequence).clone()), alternative)
            },
            Expr::WhileLoop { condition, body } => {
                self.evaluate_while_loop(*condition, *body)
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
        }
    }
    
    fn evaluate_function_def(&mut self, name: String, params: Vec<String>, body: Expr) -> Result<Literal, String> {
        // 関数定義をコンテキストに保存
        self.ctx.set_function(name, params, body);
        Ok(Literal::Unit) // 特に値を返さないからUnit型を返す
    }
                
    fn evaluate_function_call(&mut self, name: String, args: Vec<Expr>) -> Result<EvaluationResult, String> {
        if let Some((params, body)) = self.ctx.get_function(&name).map(|f| (f.0.clone(), f.1.clone())) {
            if params.len() != args.len() {
                return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
            }

            self.ctx.push_scope();
            for (param, arg) in params.iter().zip(args.iter()) {
                let arg_eval_result = self.evaluate(arg.clone())?;
                match arg_eval_result {
                    EvaluationResult::Value(val) => {
                        self.ctx.set_variable(param.clone(), Value::from_literal(val)?);
                    },
                    EvaluationResult::ReturnValue(_) => {
                        self.ctx.pop_scope();
                        return Ok(arg_eval_result);
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
        consequence: Box<Expr>, 
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
        //println!("Evaluating WhileLoop");
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

    fn evaluate_assignment(&mut self, name: String, value: Expr) -> Result<EvaluationResult, String> {
        let eval_result = self.evaluate(value)?;
        match eval_result {
            EvaluationResult::Value(val) => {
                let value = Value::from_literal(val)?;
                self.ctx.set_variable(name, value);
                Ok(EvaluationResult::Value(Literal::Unit)) // 代入は値を返さないため、Unitを返す
            },
            _ => Ok(eval_result),
        }
    }

    fn evaluate_binary_op(&mut self, left: Expr, op: Op, right: Expr) -> Result<EvaluationResult, String> {
        let left_result = self.evaluate(left)?;
        let right_result = self.evaluate(right)?;

        match (left_result, right_result) {
            (EvaluationResult::Value(Literal::Int(l)), EvaluationResult::Value(Literal::Int(r))) => match op {
                Op::Add => Ok(EvaluationResult::Value(Literal::Int(l + r))),
                Op::Subtract => Ok(EvaluationResult::Value(Literal::Int(l - r))),
                Op::Multiply => Ok(EvaluationResult::Value(Literal::Int(l * r))),
                Op::Divide => Ok(EvaluationResult::Value(Literal::Int(l / r))),
                Op::LessThan => Ok(EvaluationResult::Value(Literal::Int((l < r) as i64))),
                Op::GreaterThan => Ok(EvaluationResult::Value(Literal::Int((l > r) as i64))),
            },
            _ => Err("Unsupported literal types for binary operation".into()),
        }
    }
    
    fn evaluate_variable(&self, name: &str) -> Result<Literal, String> {
        match self.ctx.get_variable(&name) {
            Some(value) => match value {
                Value::Int(i) => Ok(Literal::Int(*i)),
                Value::String(s) => Ok(Literal::String(s.clone())),
                // other type
                _ => Err(format!("Unsupported value type for variable '{}'", name)),
            },
            None => Err(format!("Variable '{}' not found", name)),
        }
    }
        
    fn evaluate_block(&mut self, expressions: Vec<Expr>) -> Result<EvaluationResult, String> {
        let mut result = EvaluationResult::Value(Literal::Unit); // デフォルトの結果をUnitとする

        for expression in expressions {
            result = self.evaluate(expression)?;
        }

        Ok(result) // ブロック内の最後の式の評価結果を返す
    }

    fn evaluate_return(&mut self, expr: Expr) -> Result<EvaluationResult, String> {
        let val = self.evaluate(expr)?;
        match val {
            EvaluationResult::Value(val) => Ok(EvaluationResult::ReturnValue(val)),
            _ => Ok(val),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Expr, Literal, Op};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    // 簡単な加算をテスト
    #[test]
    fn test_simple_addition() {
        let mut evaluator = Evaluator::new();
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Literal(Literal::Int(1))),
            op: Op::Add,
            right: Box::new(Expr::Literal(Literal::Int(2))),
        };
        assert_eq!(evaluator.evaluate(expr), Ok(EvaluationResult::Value(Literal::Int(3))));
    }

    // 変数の代入と参照をテスト
    #[test]
    fn test_variable_assignment_and_reference() {
        let mut evaluator = Evaluator::new();
        // 代入
        let assign_expr = Expr::Assignment {
            name: "x".to_string(),
            value: Box::new(Expr::Literal(Literal::Int(5))),
        };
        evaluator.evaluate(assign_expr).unwrap();
        // 参照
        let var_expr = Expr::Variable("x".to_string());
        assert_eq!(evaluator.evaluate(var_expr), Ok(EvaluationResult::Value(Literal::Int(5))));
    }

    // 関数定義と呼び出しをテスト
    #[test]
    fn test_function_definition_and_call() {
        let mut evaluator = Evaluator::new();
        // 関数定義
        let func_def_expr = Expr::FunctionDef {
            name: "add".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            body: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Variable("a".to_string())),
                op: Op::Add,
                right: Box::new(Expr::Variable("b".to_string())),
            }),
        };
        evaluator.evaluate(func_def_expr).unwrap();
        // 関数呼び出し
        let call_expr = Expr::FunctionCall {
            name: "add".to_string(),
            args: vec![Expr::Literal(Literal::Int(2)), Expr::Literal(Literal::Int(3))],
        };
        assert_eq!(evaluator.evaluate(call_expr), Ok(EvaluationResult::Value(Literal::Int(5))));
    }

}
