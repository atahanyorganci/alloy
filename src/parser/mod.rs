use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

#[derive(Debug)]
pub enum ParserError {}

pub trait Parse<'a>: Sized {
    fn parse(pair: Pair<'a, Rule>) -> Result<Self, ParserError>;
}

pub fn parse(input: &str) {
    match AlloyParser::parse(Rule::statements, input) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
