// use lazy_static::lazy_static;

use pest_derive::Parser;
// use pest::pratt_parser::PrattParser;
use pest::Parser;
mod ast;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct RiskParser;

// lazy_static! {
//     pub static ref PRATT_PARSER: PrattParser<Rule> = {
//         use Rule::*;
//         use pest::pratt_parser::Assoc;
//         use pest::pratt_parser::Op;

//         PrattParser::new()
//             .op(Op::infix(Rule::function_type, Assoc::Left))
//             // .op(Op::infix(Rule::annotation, Assoc::Left))
//     };
// }

pub fn parse(input: &str) -> Result<ast::Program, pest::error::Error<Rule>> {
    let pairs = RiskParser::parse(Rule::program, input)?;
    Ok(ast::Program::from_pairs(pairs))
}