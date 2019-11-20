use std::collections::HashMap;
use std::fmt;

use crate::ast::{Expr, Opcode, Stmt};

pub fn compile(stmts: &Vec<Box<Stmt>>) -> String {
    let mut state = LLVMState::new();

    stmts
        .iter()
        .for_each(|stmt| compile_stmt(&stmt, &mut state));

    state.generate_code()
}

struct LLVMState {
    register_count: usize,
    instructions: Vec<String>,
    var_loc_map: HashMap<String, i32>,
    var_loc_counts: HashMap<String, i32>,
}

impl LLVMState {
    fn new() -> LLVMState {
        LLVMState {
            register_count: 0,
            instructions: vec![],
            var_loc_map: HashMap::new(),
            var_loc_counts: HashMap::new(),
        }
    }

    fn generate_code(&self) -> String {
        let instructions = self.instructions.join("\n\t");

        format!(
            "{}{}{}",
            String::from("declare void @printInt(i32)\ndefine i32 @main() {\n\t"),
            instructions,
            String::from("\n\tret i32 0\n}\n")
        )
    }

    fn get_next_register_number(&mut self) -> usize {
        self.register_count = self.register_count + 1;
        self.register_count
    }

    fn alloca(&mut self, ident: &String) {
        self.instructions
            .push(format!("%loc_{} = alloca i32", ident))
    }

    fn store(&mut self, result: LLVMResult, ident: &String) {
        self.instructions
            .push(format!("store i32 {}, i32* %loc_{}", result, ident));
    }

    fn print(&mut self, result: LLVMResult) {
        self.instructions
            .push(format!("call void @printInt(i32 {})", result));
    }

    fn load(&mut self, result: &LLVMResult, ident: &String) {
        self.instructions
            .push(format!("{} = load i32, i32* %loc_{}", result, ident))
    }

    fn arithmetic(&mut self, result: &LLVMResult, opcode: &Opcode, l: &LLVMResult, r: &LLVMResult) {
        self.instructions
            .push(format!("{} = {} i32 {}, {}", result, opcode, l, r));
    }
}

enum LLVMResult {
    Constant(i32),
    Register(usize),
    RegisterVar(String, i32),
}

impl fmt::Display for LLVMResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LLVMResult::*;
        match self {
            Constant(c) => write!(f, "{}", c),
            Register(r) => write!(f, "%r{}", r),
            RegisterVar(ident, count) => match ident.as_ref() {
                "r" => write!(f, "%var__{}{}", ident, count),
                _ => write!(f, "%{}{}", ident, count),
            },
        }
    }
}

fn compile_stmt(stmt: &Stmt, state: &mut LLVMState) {
    use Stmt::*;

    match stmt {
        SAss(ident, expr) => {
            if let 0 = state.var_loc_map.entry(ident.clone())
                .and_modify(|c| *c += 1)
                .or_insert(0) {
                state.alloca(ident);
            }

            let result = compile_expr(&expr, state);

            state.store(result, ident);
        }
        SExpr(expr) => {
            let result = compile_expr(&expr, state);

            state.print(result);
        }
    }
}

fn compile_expr<'a>(expr: &Expr, state: &'a mut LLVMState) -> LLVMResult {
    use Expr::*;
    use LLVMResult::*;

    match expr {
        Number(n) => LLVMResult::Constant(*n),
        Ident(ident) => {
            let count = state
                .var_loc_counts
                .entry(ident.clone())
                .and_modify(|c| *c += 1)
                .or_insert(0);

            let result = RegisterVar(ident.clone(), *count);

            state.load(&result, ident);

            result
        }
        Op(l_expr, opcode, r_expr) => {
            let (l, r) = (compile_expr(l_expr, state), compile_expr(r_expr, state));

            let result = Register(state.get_next_register_number());

            state.arithmetic(&result, opcode, &l, &r);

            result
        }
    }
}
