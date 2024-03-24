use std::fs;
use std::env;
use parser::parser;
use chumsky::Parser;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons");

    println!("{:?}", parser().parse(content));
}