use std::fs;
use std::env;
use logos::Logos;
// use parser::parser;
use parser::lexer::*;

mod parser;
mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons");

    let mut parser = parser::Parser::new(&content);
    
    let ast = parser.parse();
    
    if let Err(e) = ast {
        e.report(&args[1]);
    } else {
        println!("{:?}", ast.unwrap());
    }
}