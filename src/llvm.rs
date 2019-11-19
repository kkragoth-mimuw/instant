use std::collections::HashMap;
use std::fmt;

use crate::ast::{Expr, Stmt};

const LLVM_PRELUDE: &str = "declare void @printInt(i32)\ndefine i32 @main() {\n\t";
const LLVM_END: &str = "\n\tret i32 0\n}";

pub struct LLVMState {
    register_count: usize,
    instructions: Vec<String>,
    var_loc_map: HashMap<String, i32>,
    var_loc_counts: HashMap<String, i32>,
}

pub enum LLVMResult {
    Constant(i32),
    Register(usize),
    RegisterVar(String, i32),
}

impl fmt::Display for LLVMResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LLVMResult::{Constant, Register, RegisterVar};
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

impl LLVMState {
    fn new() -> LLVMState {
        LLVMState {
            register_count: 0,
            instructions: vec![],
            var_loc_map: HashMap::new(),
            var_loc_counts: HashMap::new(),
        }
    }

    fn get_next_register_number(&mut self) -> usize {
        self.register_count = self.register_count + 1;
        self.register_count
    }
}

pub fn compile(stmts: &Vec<Box<Stmt>>) -> String {
    let instructions = compile_stmts(stmts);

    format!("{}{}{}", LLVM_PRELUDE, instructions.join("\n\t"), LLVM_END)
}

pub fn compile_stmts(stmts: &Vec<Box<Stmt>>) -> Vec<String> {
    let mut state = LLVMState::new();

    for stmt in stmts.iter() {
        compile_stmt(&stmt, &mut state);
    }

    state.instructions
}

fn compile_stmt<'a>(stmt: &Stmt, state: &'a mut LLVMState) {
    match stmt {
        Stmt::SAss(ident, expr) => {
            if let 0 = state.var_loc_map.entry(ident.to_string()).or_insert(0) {
                state
                    .instructions
                    .push(format!("%loc_{} = alloca i32", ident));
            }

            let result = compile_expr(&expr, state);

            state
                .instructions
                .push(format!("store i32 {}, i32* %loc_{}", result, ident));
        }
        Stmt::SExpr(expr) => {
            let result = compile_expr(&expr, state);

            state
                .instructions
                .push(format!("call void @printInt(i32 {})", result));
        }
    }
}

fn compile_expr<'a>(expr: &Expr, state: &'a mut LLVMState) -> LLVMResult {
    match expr {
        Expr::Number(n) => LLVMResult::Constant(*n),
        Expr::Ident(ident) => {
            let count = state.var_loc_counts.entry(ident.to_string()).or_insert(0);
            *count += 1;

            let result = LLVMResult::RegisterVar(ident.to_string(), *count);

            state
                .instructions
                .push(format!("{} = load i32, i32* %loc_{}", result, ident));

            result
        }
        Expr::Op(l_expr, opcode, r_expr) => {
            let l1 = compile_expr(l_expr, state);
            let l2 = compile_expr(r_expr, state);

            let result = LLVMResult::Register(state.get_next_register_number());

            state
                .instructions
                .push(format!("{} = {} {}, {}", result, opcode, l1, l2));

            result
        }
    }
}
