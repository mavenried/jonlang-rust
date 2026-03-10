#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Var(String),
    Binary(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),
    PrintText(String),
    PrintValue(String, Expr),
    InputNumber(String, String),
}

#[derive(Debug)]
pub enum Instr {
    Push(f64),
    Load(String),
    Store(String),

    Add,
    Sub,
    Mul,
    Div,

    PrintText(String),
    PrintValue(String),

    InputNumber(String, String),

    Halt,
}
