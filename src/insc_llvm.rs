use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub mod ast;
pub mod instant_parser;
pub mod llvm;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args
        .get(1)
        .expect("No filename present. Please specify filename as argument");

    let contents = fs::read_to_string(filename).expect("Error reading file");

    let stmts = instant_parser::StmtsParser::new().parse(&contents).unwrap();

    let code = llvm::compile(&stmts);

    let path = Path::new(filename);
    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap();

    let generated_code_path = format!(
        "{}/{}.ll",
        parent.to_string_lossy(),
        file_stem.to_string_lossy()
    );

    let generated_code_path_bc_output = format!(
        "{}/{}.bc",
        parent.to_string_lossy(),
        file_stem.to_string_lossy()
    );

    fs::write(&generated_code_path, code).expect("Unable to write to file");

    Command::new("llvm-as")
        .args(&["-o", &generated_code_path_bc_output[..], &generated_code_path[..]])
        .output()
        .expect("failed to execute java/jasmin");
}
