use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    SAss(String, Box<Expr>),
    SExpr(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Ident(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Opcode::{Add, Sub, Mul, Div};
        match self {
            Add => write!(f, "add i32"),
            Sub => write!(f, "sub i32"),
            Mul => write!(f, "mul i32"),
            Div => write!(f, "div i32")
        }
    }
}