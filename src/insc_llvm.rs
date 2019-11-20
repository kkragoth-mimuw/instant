use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub mod ast;
pub mod instant_parser;
pub mod llvm;

/*
tests=(01 02 03 04 05 06 07)

for x in ${tests[*]}; do
    ./insc_llvm foo/bar/test$x.ins
done

cd foo/bar;

for x in ${tests[*]}; do
	lli test$x_linked.bc
done

cd ../..

*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args
        .get(1)
        .expect("No filename present. Please specify filename as argument");

    let contents = fs::read_to_string(filename).expect("Error reading file");

    let stmts = instant_parser::StmtsParser::new().parse(&contents).unwrap();

    let code = llvm::compile(&stmts);

    let path = Path::new(filename);
    let file_stem = path.file_stem().expect("Unable to get file stem").to_str().unwrap();
    let parent = path.parent().unwrap().to_str().expect("Error getting parent");

    let generated_code_path = match parent {
        "" => format!("{}.ll", file_stem),
        parent_str => format!("{}/{}.ll", parent_str, file_stem)
    };

    let generated_intermediate_bc_path = match parent {
        "" => format!("{}_intermediate.bc", file_stem),
        parent_str => format!("{}/{}_intermediate.bc", parent_str, file_stem)
    };

    fs::write(&generated_code_path, code).expect("Unable to write to file");

    Command::new("llvm-as")
        .args(&["-o", &generated_intermediate_bc_path, &generated_code_path])
        .output()
        .expect("failed to execute llvm-as");

    Command::new("llvm-link")
        .args(&["-o", &generated_code_path, &generated_intermediate_bc_path, "lib/runtime.bc"])
        .output()
        .expect("failed to execute llvm-as");

    Command::new("rm")
        .args(&["-f", &generated_intermediate_bc_path])
        .output()
        .expect("failed to remove intermediate code");
}
