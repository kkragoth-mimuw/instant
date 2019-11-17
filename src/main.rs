pub mod instant_parser;

fn main() {
    let b = instant_parser::StmtsParser::new()
        .parse("n = 2; 4;")
        .unwrap();

    println!("{:?}", b);
}
