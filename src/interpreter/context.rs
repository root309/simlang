use std::collections::HashMap;
use crate::parser::ast::Expr;

pub enum Value {
    Int(i64),
    String(String),
    Function(Vec<String>, Box<Expr>),
}

pub struct Context {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Value>,
    // 変数のスコープを管理するスタック
    variable_stack: Vec<HashMap<String, Value>>,
    // 関数のスコープを管理するスタック
    function_stack: Vec<HashMap<String, Value>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
            functions: HashMap::new(),
            variable_stack: vec![HashMap::new()],
            function_stack: vec![HashMap::new()],        
        }
    }

    // 新しいスコープをプッシュ
    pub fn push_scope(&mut self) {
        self.variable_stack.push(HashMap::new());
        self.function_stack.push(HashMap::new());
    }

    // 現在のスコープをポップ
    pub fn pop_scope(&mut self) {
        self.variable_stack.pop();
        self.function_stack.pop();
    }

    // 変数を現在のスコープに設定
    pub fn set_variable(&mut self, name: String, value: Value) {
        if let Some(current_scope) = self.variable_stack.last_mut() {
            current_scope.insert(name, value);
        }
    }

    // 現在のスコープから変数を取得
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        for scope in self.variable_stack.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val);
            }
        }
        None
    }

    // 関数を現在のスコープに設定
    pub fn set_function(&mut self, name: String, params: Vec<String>, body: Expr) {
        let func = Value::Function(params, Box::new(body));
        if let Some(current_scope) = self.function_stack.last_mut() {
            current_scope.insert(name, func);
        }
    }

    // 現在のスコープから関数を取得
    pub fn get_function(&self, name: &str) -> Option<&Value> {
        for scope in self.function_stack.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val);
            }
        }
        None
    }
}
