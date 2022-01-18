use std::num::{ParseFloatError, ParseIntError};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use thiserror::Error;

use crate::ast::statement::Statement;

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
    #[error("WIP")]
    WIP,
}

impl From<pest::error::Error<Rule>> for ParserError {
    fn from(_: pest::error::Error<Rule>) -> Self {
        ParserError::WIP
    }
}

pub type ParseResult<T> = Result<T, ParserError>;

pub trait Parse<'a>: Sized {
    fn parse(pair: Pair<'a, Rule>) -> Result<Self, ParserError>;
}

pub fn parse_rule<'a, T: Parse<'a>>(rule: Rule, input: &'a str) -> ParseResult<T> {
    match AlloyParser::parse(rule, input) {
        Ok(mut pairs) => T::parse(pairs.next().unwrap()),
        Err(e) => Err(e.into()),
    }
}

pub fn parse_statement<'a, T: Parse<'a>>(input: &'a str) -> ParseResult<T> {
    match AlloyParser::parse(Rule::program, input) {
        Ok(mut pairs) => T::parse(pairs.next().unwrap()),
        Err(e) => Err(e.into()),
    }
}

pub fn parse_pairs(pairs: Pairs<Rule>) -> Result<Vec<Statement>, ParserError> {
    let (_, max) = pairs.size_hint();
    let mut statements = if let Some(capacity) = max {
        Vec::with_capacity(capacity)
    } else {
        Vec::new()
    };
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => break,
            _ => statements.push(Statement::parse(pair)?),
        }
    }
    Ok(statements)
}

pub fn parse(input: &str) -> Result<Vec<Statement>, ParserError> {
    match AlloyParser::parse(Rule::program, input) {
        Ok(pairs) => parse_pairs(pairs),
        Err(e) => Err(e.into()),
    }
}
