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
    var_loc_map: HashMap<String, i32> 
}

#[derive(Debug)]
pub enum LLVMResult {
    Constant(i32),
    Register(usize)
}

impl fmt::Display for LLVMResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LLVMResult::Constant(c) => write!(f, "{}", c),
            LLVMResult::Register(r) => write!(f, "%{}", r)
        }
    }
}

impl LLVMState {
    fn new() -> LLVMState {
        LLVMState {
            register_count: 0,
            instructions: vec![],
            var_loc_map: HashMap::new()
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
    let mut state = LLVMState::new();

    for stmt in stmts.iter() {
        compile_stmt(&stmt, &mut state);
    }

    state.generated_code()
}

fn compile_stmt<'a> (stmt: &Stmt, state: &'a mut LLVMState) {
    match stmt {
        Stmt::SAss(ident, expr) => {
            if let None = state.var_loc_map.get(ident) {
                state.allocate_variable(ident);
            }

            let result = compile_expr(&expr, state);

            state.instructions.push(
                format!("store i32 {}, i32* %{}{}",result, location_prefix, ident)
            );

            // state.instructions.push(ident.to_string());
            // state.instructions.push(String::from(format!("{:?}", stmt)));
        },
        Stmt::SExpr(expr) => {
            let n = compile_expr(&expr, state);

            state.instructions.push(String::from(format!("SEXPR: {:?}", n)));
        }
    }
}

fn compile_expr<'a> (expr: &Expr, state: &'a mut LLVMState) -> LLVMResult {
    match expr {
        Expr::Number(n) => LLVMResult::Constant(*n),
        Expr::Ident(ident) => {
            println!("IDENT");
            if let None = state.var_loc_map.get(ident) {
                println!("IDENT empty");
                state.allocate_variable(ident);
            }

            LLVMResult::Register(3)
        },
        Expr::Op(l_expr, opcode, r_expr) => {
            state.instructions.push(String::from(format!("{:?}", opcode)));
            let l1 = compile_expr(l_expr, state);
            let l2 = compile_expr(r_expr, state);
            LLVMResult::Register(5)
        }
    }
}

// pub fn compile_expr(expr: &ast::Expr) -> LLVMResult {
//     match expr {
//         Expr::Number(i) => LLVMResult::Constant(i)
//     }
// }