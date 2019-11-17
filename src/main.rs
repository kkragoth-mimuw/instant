pub mod instant_parser;
pub mod ast;
pub mod llvm;

// use instant::llvm;
fn main() {
    let stmts = instant_parser::StmtsParser::new()
        .parse("n = 2*(5+1); 4;")
        .unwrap();

    for stmt in stmts.iter() {
        println!("{:?}", stmt);
    }

    let code = llvm::compile(&stmts);

    println!("{}", code);



    // let b = stmts.for_each(|stmt| );

    // println!("{:?}", b);
}
