

// pub mod ast { 
//     #[derive(Debug)]
//     pub enum Stmt {
//         SAss(String, Box<Expr>),
//         SExpr(Box<Expr>),
//     }

//     #[derive(Debug)]
//     pub enum Expr {
//         Number(i32),
//         Ident(String),
//         Op(Box<Expr>, Opcode, Box<Expr>),
//     }

//     #[derive(Debug)]
//     pub enum Opcode {
//         Add,
//         Sub,
//         Mul,
//         Div,
//     }
// }

// pub mod llvm {

//     pub fn compile(stmts : &Vec<Stmt>) -> String {
//         let mut initial_state = LLVMState::new();

//         let final_state = stmts.iter().fold(
//             initial_state,
//             |state, stmt| *(compile_stmt(&stmt, &mut state))
//         );

//         final_state.generated_code()
//     }
    
//     use std::collections::HashMap;
//     use super::ast::{Stmt, Expr, Opcode};

//     pub struct LLVMState {
//         // register_count: i32,
//         instructions: Vec<String>,
//         var_loc_map: HashMap<String, i32> 
//     }

//     pub enum LLVMResult {
//         Constant(i32),
//         Register(i32)
//     }

//     impl LLVMState {
//         fn new() -> LLVMState {
//             LLVMState {
//                 instructions: vec![],
//                 var_loc_map: HashMap::new()
//             }
//         }

//         fn generated_code(&self) -> String {
//             self.instructions.join("\n")
//         }
//     }



//     fn compile_stmt<'a> (stmt: &Stmt, state: &'a mut LLVMState ) -> &'a mut LLVMState {
//         state.instructions.push(String::from("2"));
//         state
//     }

//     pub fn compile_expr(expr: &super::ast::Expr) -> LLVMResult {
//         match expr {
//             Expr::Number(i) => LLVMResult::Constant(i)
//         }
//     }
// }