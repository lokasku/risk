// use lazy_static::lazy_static;

use pest_derive::Parser;
// use pest::pratt_parser::{PrattParser, Assoc, Op};
use pest::Parser;
mod ast;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct RiskParser;

// lazy_static! {
//     pub static ref PRATT_PARSER: PrattParser<Rule> = {
//         PrattParser::new()
//     };
// }

pub fn parse(input: &str) -> Result<ast::Program, pest::error::Error<Rule>> {
    let pairs = RiskParser::parse(Rule::program, input)?;
    Ok(ast::Program::from_pairs(pairs))
}