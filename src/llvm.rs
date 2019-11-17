// pub mod ast;

use std::collections::HashMap;
use std::fmt;

use crate::ast::{Stmt, Expr, Opcode};

const register_prefix : &str = "r";
const location_prefix : &str = "loc_";

#[derive(Debug)]
pub struct LLVMState {
    register_count: usize,
    instructions: Vec<String>,
    var_loc_map: HashMap<String, i32>,
    var_loc_counts: HashMap<String, i32> 
}

#[derive(Debug)]
pub enum LLVMResult {
    Constant(i32),
    Register(usize),
    RegisterVar(String, i32)
}

impl fmt::Display for LLVMResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LLVMResult::Constant(c) => write!(f, "{}", c),
            LLVMResult::Register(r) => write!(f, "%r{}", r),
            LLVMResult::RegisterVar(ident, count) => write!(f, "%{}{}", ident, count)
        }
    }
}

impl LLVMState {
    fn new() -> LLVMState {
        LLVMState {
            register_count: 0,
            instructions: vec![],
            var_loc_map: HashMap::new(),
            var_loc_counts: HashMap::new()
        }
    }

    fn generated_code(&self) -> String {
        self.instructions.join("\n")
    }

    fn get_next_register_number(&mut self) -> usize {
        self.register_count = self.register_count + 1;
        self.register_count
    }

    fn allocate_variable(&mut self, variable_name : &String) {
        self.instructions.push(format!("%{}{} = alloca i32",location_prefix, variable_name));

    }
}

pub fn compile(stmts : &Vec<Box<Stmt>>) -> String {
    let instructions = compile_stmts(stmts);
    let prelude = String::from("declare void @printInt(i32)\n
define i32 @main() {\n\t");
    let end = "\n}";

    format!("{}{}{}", prelude, instructions.join("\n\t"), end)
}

pub fn compile_stmts(stmts : &Vec<Box<Stmt>>) -> Vec<String> {
    let mut state = LLVMState::new();

    for stmt in stmts.iter() {
        compile_stmt(&stmt, &mut state);
    }

    state.instructions
}

fn compile_stmt<'a> (stmt: &Stmt, state: &'a mut LLVMState) {
    match stmt {
        Stmt::SAss(ident, expr) => {
            if let 0 = state.var_loc_map.entry(ident.to_string()).or_insert(0) {
                state.allocate_variable(ident);
            }

            let result = compile_expr(&expr, state);

            state.instructions.push(
                format!("store i32 {}, i32* %{}{}",result, location_prefix, ident)
            );

        },
        Stmt::SExpr(expr) => {
            let result = compile_expr(&expr, state);

            state.instructions.push(
                format!("call void @printInt(i32 {})", result)
            );
        }
    }
}

fn compile_expr<'a> (expr: &Expr, state: &'a mut LLVMState) -> LLVMResult {
    match expr {
        Expr::Number(n) => LLVMResult::Constant(*n),
        Expr::Ident(ident) => {
            let count = state.var_loc_counts.entry(ident.to_string()).or_insert(0);
            *count += 1;

            let result = LLVMResult::RegisterVar(ident.to_string(), *count);

            state.instructions.push(
                format!("{} = load i32, i32* {}{}", result, location_prefix,ident)
            );

            result
        },
        Expr::Op(l_expr, opcode, r_expr) => {
            let l1 = compile_expr(l_expr, state);
            let l2 = compile_expr(r_expr, state);

            let result = LLVMResult::Register(state.get_next_register_number());

            state.instructions.push(
                format!("{} = {} {}, {}", result, opcode, l1, l2)
            );

            result
        }
    }
}
