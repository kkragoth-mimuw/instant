use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub mod ast;
pub mod instant_parser;
pub mod jvm;


/*
tests=(01 02 03 04 05 06 07)

for x in ${tests[*]}; do
    ./insc_jvm foo/bar/test$x.ins
done

cd foo/bar;

for x in ${tests[*]}; do
	java test$x
done
*/


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args
        .get(1)
        .expect("No filename present. Please specify filename as argument");

    let contents = fs::read_to_string(filename).expect("Error reading file");

    let stmts = instant_parser::StmtsParser::new().parse(&contents).unwrap();

    let path = Path::new(filename);
    let file_stem = path.file_stem().expect("Unable to get file stem").to_str().unwrap();
    let parent = path.parent().unwrap().to_str().expect("Error getting parent");

    let generated_code_path = match parent {
        "" => format!("{}.j", file_stem),
        parent_str => format!("{}/{}.j", parent_str, file_stem)
    };

    let code = jvm::compile(&stmts, &file_stem);

    fs::write(&generated_code_path, code).expect("Unable to write to file");

    Command::new("java")
        .args(&["-jar", "lib/jasmin", &generated_code_path])
        .output()
        .expect("failed to execute java/jasmin");
}
