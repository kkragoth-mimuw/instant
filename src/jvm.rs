use std::cmp;
use std::collections::HashMap;

use crate::ast::{Stmt, Expr, Opcode};

struct JVMState {
    instructions: Vec<String>,
    var_index_map: HashMap<String, usize>
}

enum TaggedStmt<'a> {
    SAss(&'a String, Box<TaggedExpr<'a>>),
    SExpr(Box<TaggedExpr<'a>>)
}

impl<'a> TaggedStmt<'_> {
    fn get_stmt_stack_size(&self) -> usize {
        use TaggedStmt::*;

        match self {
            SAss(_, tagged_expr) => tagged_expr.get_expr_stack_size() + 1,
            SExpr(tagged_expr) => tagged_expr.get_expr_stack_size() + 2
        }
    }
}

enum TaggedExpr<'a> {
    Number(i32),
    Ident(&'a String),
    Op(Box<TaggedExpr<'a>>, &'a Opcode, Box<TaggedExpr<'a>>, usize)
}

impl<'a> TaggedExpr<'_> {
    fn get_expr_stack_size(&self) -> usize {
        use TaggedExpr::*;

        match *self {
            Number(_) | Ident(_) => 1,
            Op(_, _, _, stack_size) => stack_size
        }
    }
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

    fn push_get_static_all_print(&mut self) {
        self.instructions.push(
            String::from("getstatic  java/lang/System/out Ljava/io/PrintStream;")
        )
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

        let i = self.var_index_map.entry(ident.clone())
            .or_insert(new_free_slot);

        let push_instruction = match i {
            0..=3 => format!("istore_{}", i),
            _ => format!("istore {}", i)
        };

        self.instructions.push(push_instruction);
    }

    fn push_swap(&mut self) {
        self.instructions.push(String::from("swap"))
    }

    fn generate_code(&self, limit_stack: usize) -> String {
        let instructions = self.instructions.join("\n\t");

        format!("{}{}{}{}",
             String::from(".method public static main([Ljava/lang/String;)V\n"),
             format!(".limit stack {}\n", limit_stack),
             instructions,
             String::from("\n.end method\n")
        )
    }
}

pub fn compile(stmts : &Vec<Box<Stmt>>) -> String {
    let mut state = JVMState::new();
    let mut limit_stack = 0;
    
    let tagged_stmts = tag_stmts(stmts);

    tagged_stmts.iter().for_each(|tagged_stmt| {
        limit_stack = cmp::max(limit_stack, tagged_stmt.get_stmt_stack_size());

        compile_tagged_stmt(&tagged_stmt, &mut state);
    });

    state.generate_code(limit_stack)
}

fn compile_tagged_stmt(stmt: &TaggedStmt, state: &mut JVMState) {
    use TaggedStmt::*;
    match stmt {
        SAss(ident, expr) => {
            compile_tagged_expr(&expr, state);
            state.push_store(&ident);
        },
        SExpr(expr) => {
            if expr.get_expr_stack_size() == 1 {
                state.push_get_static_all_print();
                compile_tagged_expr(&expr, state);
            } else {
                compile_tagged_expr(&expr, state);
                state.push_get_static_all_print();
                state.push_swap();
            }
            
            state.push_call_print();
        }
    }
}

fn compile_tagged_expr(expr: &TaggedExpr, state: &mut JVMState) {
    use TaggedExpr::*;
    match expr {
        Number(n) => state.push_constant(*n),
        Ident(ident) => {
            state.push_load(ident)
        },
        Op(l_expr, opcode, r_expr, _) => {
            let swap_occured: bool;

            let first_expr : &TaggedExpr;
            let second_expr: &TaggedExpr;

            if l_expr.get_expr_stack_size() <= r_expr.get_expr_stack_size() {
                swap_occured = false;

                first_expr = l_expr;
                second_expr = r_expr;
            } else {
                swap_occured = true;

                first_expr = r_expr;
                second_expr = l_expr;
            }

            compile_tagged_expr(first_expr, state);
            compile_tagged_expr(second_expr, state);

            if swap_occured && (**opcode == Opcode::Sub || **opcode == Opcode::Div) {
                state.push_swap();
            }

            state.push_opcode(opcode);
        }
    }
}

fn tag_stmts(stmts: &Vec<Box<Stmt>>) -> Vec<TaggedStmt> {
    stmts.iter().map(|stmt| tag_stmt_stack_limit(stmt)).collect()
}

fn tag_stmt_stack_limit(stmt: &Stmt) -> TaggedStmt {
    match stmt {
        Stmt::SAss(ident, expr) => TaggedStmt::SAss(ident, Box::new(tag_expr_stack_limit(expr))),
        Stmt::SExpr(expr) => TaggedStmt::SExpr(Box::new(tag_expr_stack_limit(expr)))
    }
}

fn tag_expr_stack_limit(expr: &Expr) -> TaggedExpr {
    match expr {
        Expr::Number(n) => TaggedExpr::Number(*n),
        Expr::Ident(id) => TaggedExpr::Ident(id),
        Expr::Op(l_expr, opcode, r_expr) => {
            let tagged_l_expr = tag_expr_stack_limit(&l_expr);
            let tagged_r_expr = tag_expr_stack_limit(&r_expr);

            let stack_limit = cmp::min(tagged_l_expr.get_expr_stack_size(), tagged_r_expr.get_expr_stack_size()) + 1;

            TaggedExpr::Op(Box::new(tagged_l_expr), opcode, Box::new(tagged_r_expr), stack_limit)
        }
    }
}
