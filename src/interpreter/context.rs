use std::collections::HashMap;
use crate::parser::ast::*;

pub enum Value {
    Int(i64),
    String(String),
    Function(Vec<String>, Box<Expr>),
}

impl Value {
    // LiteralからValueへの変換を行うメソッド
    pub fn from_literal(literal: Literal) -> Result<Self, String> {
        match literal {
            Literal::Int(value) => Ok(Value::Int(value)),
            Literal::String(value) => Ok(Value::String(value)),
            _ => Err("Unsupported literal type for conversion".into()),
        }
    }
}

pub struct Context {
    //variables: HashMap<String, Value>,
    functions: HashMap<String, (Vec<String>, Box<Expr>)>,    
    // 変数のスコープを管理するスタック
    variable_stack: Vec<HashMap<String, Value>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            //variables: HashMap::new(),
            functions: HashMap::new(),
            variable_stack: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.variable_stack.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.variable_stack.pop();
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        if let Some(current_scope) = self.variable_stack.last_mut() {
            current_scope.insert(name, value);
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        for scope in self.variable_stack.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val);
            }
        }
        None
    }

    pub fn set_function(&mut self, name: String, params: Vec<String>, body: Expr) {
        self.functions.insert(name, (params, Box::new(body)));
    }

    pub fn get_function(&self, name: &str) -> Option<&(Vec<String>, Box<Expr>)> {
        self.functions.get(name)
    }
}
