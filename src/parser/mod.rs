use pest_derive::Parser;
use pest::Parser;
mod ast;


#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct RiskParser;

pub fn parse(input: &str) -> Result<ast::Program, pest::error::Error<Rule>> {
    let pairs = RiskParser::parse(Rule::program, input)?;
    Ok(ast::Program::from_pairs(pairs))
}