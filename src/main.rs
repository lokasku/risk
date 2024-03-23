use std::fs;
use std::env;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons");
    let ast = parser::parse(&content);
    if let Err(e) = ast {
        panic!("Error: {}", e);
    }
    println!("{:#?}", ast);
}