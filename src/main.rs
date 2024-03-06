use std::fs;
use std::env;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons");

    let parser = parser::parser::ProgramParser::new();
    let res = parser.parse(&content);
    let res2 = parser.parse(&content);
    println!("{:?}", res);
    println!("{:?}", res2);
}