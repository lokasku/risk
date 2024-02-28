use std::fs;
use std::env;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.");
    }

    let _content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons");

    lalrpop::process_root().unwrap();
}