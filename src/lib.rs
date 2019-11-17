pub mod ast {
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
}