use std::fmt;

use pest::{iterators::Pair, Parser};

use crate::compiler::Compile;

pub mod expression;
pub mod statement;
pub mod value;

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

#[derive(Debug)]
pub enum ParserError {}

pub trait ASTNode<'a>: fmt::Debug + fmt::Display + Sized {
    fn build(pair: Pair<'a, Rule>) -> Result<Self, ParserError>;
}

pub fn parse(input: &str) {
    match AlloyParser::parse(Rule::statements, input) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
