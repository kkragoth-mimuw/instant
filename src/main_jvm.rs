use std::env;
use std::fs;
use std::path::Path;

pub mod ast;
pub mod instant_parser;
pub mod jvm;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args
        .get(1)
        .expect("No filename present. Please specify filename as argument");

    let contents = fs::read_to_string(filename).expect("Error reading file");

    let stmts = instant_parser::StmtsParser::new().parse(&contents).unwrap();

    let code = jvm::compile(&stmts);

    let path = Path::new(filename);
    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap();

    let generated_code_path = format!(
        "{}/{}.j",
        parent.to_string_lossy(),
        file_stem.to_string_lossy()
    );

    fs::write(generated_code_path, code).expect("Unable to write to file");
}
