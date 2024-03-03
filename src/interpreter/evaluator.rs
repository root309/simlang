use crate::parser::ast::{Expr, Op, Literal};
use std::collections::HashMap;

pub struct Evaluator {
    ctx: Context,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            ctx: Context::new(),
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<Literal, String> {
        match expr {
            // TODO:関数定義をContextに保存する必要がある(スコープ管理めんどくさそう)
            Expr::FunctionDef { name, params, body } => self.evaluate_function_def(name, params, body),
            // TODO:関数定義を取得し、引数を評価して、その関数を実行する必要がある
            Expr::FunctionCall { name, args } => self.evaluate_function_call(name, args),
            // TODO:条件式を評価して、条件に応じてconsequenceかalternativeを評価する必要がある
            Expr::IfExpr { condition, consequence, alternative } => self.evaluate_if_expr(condition, consequence, alternative),
            // TODO:条件式を評価して、条件が真の間、bodyを評価し続ける必要がある
            Expr::WhileLoop { condition, body } => self.evaluate_while_loop(condition, body),
            Expr::Assignment { name, value } => self.evaluate_assignment(name, *value), 
            Expr::BinaryOp { left, op, right } => self.evaluate_binary_op(*left, op, *right),
            Expr::Literal(lit) => Ok(lit),
            Expr::Variable(name) => self.ctx.get_variable(&name).cloned().ok_or(format!("Variable '{}' not found", name)),
            // TODO:Literalの初期値ではなく最後の式の評価結果を返す
            // TODO:Returnの評価結果を返す(return文がある場合はその値を返し処理を中断する)
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
        // 関数定義をコンテキストに保存
        self.ctx.set_function(name, params, body);
        Ok(Literal::Unit) // 特に値を返さないからUnit型を返す
    }
    
    fn evaluate_function_call(&mut self, name: String, args: Vec<Expr>) -> Result<Literal, String> {
        if let Some(Value::Function(params, body)) = self.ctx.get_function(&name) {
            if params.len() != args.len() {
                return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
            }

            // 新しいスコープをプッシュ
            self.ctx.push_scope();

            // 引数を評価し、ローカルスコープに設定
            for (param, arg) in params.iter().zip(args.iter()) {
                let arg_val = self.evaluate(arg.clone())?; // 引数を評価
                let value = Value::from_literal(arg_val)?; // LiteralからValueへ変換
                self.ctx.set_variable(param.clone(), value);
            }

            // 関数本体を評価
            let result = self.evaluate(*body)?;

            // スコープをポップ
            self.ctx.pop_scope();

            Ok(result)
        } else {
            Err(format!("Function '{}' not found", name))
        }
    }

    fn evaluate_if_expr(&mut self, condition: Expr, consequence: Expr, alternative: Expr) -> Result<Literal, String> {
        // TODO:
    }

    fn evaluate_while_loop(&mut self, condition: Expr, body: Expr) -> Result<Literal, String> {
        // TODO:
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

    fn evaluate_literal(&self, lit: Literal) -> Result<Literal, String> {
        // TODO:
    }

    fn evaluate_variable(&self, name: &str) -> Result<Literal, String> {
        self.ctx.get_variable(&name).ok_or(format!("Variable '{}' not found", name))
    }

    fn evaluate_block(&mut self, expressions: Vec<Expr>) -> Result<Literal, String> {
        let mut last = Literal::Int(0); // 初期値は0でブロックが空の場合のデフォルト値

        for expr in expressions {
            last = self.evaluate(expr)?;
        }

        Ok(last)
    }

    fn evaluate_return(&mut self, expr: Expr) -> Result<Literal, String> {
        // TODO: 
    }

}
