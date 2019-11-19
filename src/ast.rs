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

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Opcode::{Add, Div, Mul, Sub};
        match self {
            Add => write!(f, "add"),
            Sub => write!(f, "sub"),
            Mul => write!(f, "mul"),
            Div => write!(f, "idiv"),
        }
    }
}
