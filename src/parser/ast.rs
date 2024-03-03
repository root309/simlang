#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    IfExpr {
        condition: Box<Expr>,
        consequence: Box<Expr>,
        alternative: Option<Box<Expr>>,
    },
    WhileLoop {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Literal(Literal),
    Variable(String),
    Block(Vec<Expr>),
    Return(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
}
