use std::collections::HashMap;

use crate::ast::{Stmt, Expr, Opcode};

mod jvm_label {
    pub const MAIN_FUNC_HEADER: &str = ".method public static main([Ljava/lang/String;)V\n";
    pub const STACK_LIMIT: &str      = ".limit stack {}\n";
    pub const GET_PRINT: &str        = "\tgetstatic  java/lang/System/out Ljava/io/PrintStream;\n\t";
    pub const END_FUNC: &str         = "\n.end method\n";
}

struct JVMState {
    instructions: Vec<String>,
    var_index_map: HashMap<String, usize>
}

impl JVMState {
    fn new() -> JVMState {
        JVMState {
            instructions: vec![],
            var_index_map: HashMap::new()
        }
    }
    fn push_constant(&mut self, n: i32) {
        let push_instruction = match n {
                                     -1 => String::from("iconst_m1"),
                                  1..=5 => format!("iconst_{}", n),
                    -128..=-2 | 6..=127 => format!("bipush {}", n),
            -32768..=-129 | 128..=32767 => format!("sipush {}", n),
                                      _ => format!("ldc {}", n)
        };

        self.instructions.push(push_instruction)
    }
    fn push_call_print(&mut self) {
        self.instructions.push(
            String::from("invokevirtual java/io/PrintStream/println(I)V")
        )
    }

    fn push_opcode(&mut self, opcode: &Opcode) {
        use Opcode::*;

        let opcode_instruction = match *opcode {
            Add => String::from("iadd"),
            Sub => String::from("isub"),
            Mul => String::from("imul"),
            Div => String::from("idiv")
        };

        self.instructions.push(opcode_instruction);
    }

    fn push_load(&mut self, ident: &String) {
        let i = self.var_index_map.get(ident)
            .expect("Use of undeclared variable");

        let load_instruction = match i {
            0..=3 => format!("iload_{}", i),
            _ => format!("iload {}", i)
        };

        self.instructions.push(load_instruction);
    }

    fn get_next_free_var_slot(&self) -> usize {
        self.var_index_map.len() + 1
    }

    fn push_store(&mut self, ident: &String) {
        let new_free_slot = self.get_next_free_var_slot();

        // TODO: check how to get mut self reference in closure
        //       having already mut self reference
        let i = self.var_index_map.entry(ident.to_string())
            .or_insert(new_free_slot);

        let push_instruction = match i {
            0..=3 => format!("istore_{}", i),
            _ => format!("istore {}", i)
        };

        self.instructions.push(push_instruction);
    }

    fn generate_code(&self) -> String {
        use jvm_label::*;

        let instructions = self.instructions.join("\n\t");

        format!("{}{}{}{}{}",
             MAIN_FUNC_HEADER,
             STACK_LIMIT,
             GET_PRINT,
             instructions,
             END_FUNC
        )
    }
}

pub fn compile(stmts : &Vec<Box<Stmt>>) -> String {
    let mut state = JVMState::new();

    stmts.iter().for_each(|stmt| compile_stmt(&stmt, &mut state));

    state.generate_code()
}

fn compile_stmt(stmt: &Stmt, state: &mut JVMState) {
    match stmt {
        Stmt::SAss(ident, expr) => {
            compile_expr(&expr, state);
            state.push_store(&ident);
        },
        Stmt::SExpr(expr) => {
            compile_expr(&expr, state);
            state.push_call_print();
        }
    }
    println!("{:?}", stmt)
}

fn compile_expr(expr: &Expr, state: &mut JVMState) {
    match expr {
        Expr::Number(n) => state.push_constant(*n),
        Expr::Ident(ident) => {
            state.push_load(ident)
        },
        Expr::Op(l_expr, opcode, r_expr) => {
            compile_expr(l_expr, state);
            compile_expr(r_expr, state);
            state.push_opcode(opcode);
        }
    }
    println!("{:?}", expr)
}
pub fn optimize_stack_length(stmts: &mut Vec<Box<Stmt>>) -> &mut Vec<Box<Stmt>> {
    stmts
}