use std::env;
use std::fs;
use std::path::Path;

pub mod instant_parser;
pub mod ast;
pub mod jvm;

fn main() {
    if true {
        let stmts = instant_parser::StmtsParser::new()
        .parse("n=2+2;n;")
        .unwrap();

        println!("{}", jvm::compile(&stmts));
    }
    
    // if false {
    // let args: Vec<String> = env::args().collect();

    // let filename = args.get(1).expect("No filename present");

    // let contents = fs::read_to_string(filename)
    //     .expect("Error reading file");

    // let stmts = instant_parser::StmtsParser::new()
    //     .parse(&contents)
    //     .unwrap();


    // let code = jvm:compile(&stmts);

    // let path = Path::new(filename);
    // let parent = path.parent().unwrap();
    // let file_stem = path.file_stem().unwrap();

    // let generated_code_path = format!("{}/{}.ll", parent.to_string_lossy(), file_stem.to_string_lossy());

    // fs::write(generated_code_path, code).expect("Unable to write to file");
    // }
}
