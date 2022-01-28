use std::num::{ParseFloatError, ParseIntError};

use nom::{
    branch::alt,
    bytes::complete::tag,
    error::{context, VerboseError},
    IResult,
};
use pest::{
    error::LineColLocation,
    iterators::{Pair, Pairs},
    Parser, Span,
};
use thiserror::Error;

use crate::ast::{statement::Statement, value::Value};

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

#[derive(Error, Debug)]
pub enum ParserErrorKind {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
    #[error("WIP")]
    WIP,
}

#[derive(Debug)]
pub struct ParserError {
    kind: ParserErrorKind,
    location: LineColLocation,
}

impl From<pest::error::Error<Rule>> for ParserError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self {
            kind: ParserErrorKind::WIP,
            location: e.line_col,
        }
    }
}

impl ParserError {
    pub fn for_pair<T: Into<ParserErrorKind>>(pair: Pair<Rule>, kind: T) -> Self {
        Self::for_span(pair.as_span(), kind)
    }

    pub fn for_span<T: Into<ParserErrorKind>>(span: Span, kind: T) -> Self {
        let start = span.start();
        let end = span.end();
        Self {
            kind: kind.into(),
            location: LineColLocation::Pos((start, end)),
        }
    }
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

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
        Err(e) => Err(ParserError {
            kind: ParserErrorKind::WIP,
            location: e.line_col,
        }),
    }
}

fn parse_boolean(input: &str) -> Res<&str, Value> {
    let result = context("boolean", alt((tag("true"), tag("false"))))(input);
    result.map(|(next_input, res)| {
        let value = if res == "true" {
            Value::True
        } else {
            Value::False
        };
        (next_input, value)
    })
}

#[cfg(test)]
mod tests {
    use crate::ast::value::Value;

    use super::parse_boolean;

    #[test]
    fn test_boolean() {
        assert_eq!(parse_boolean("true"), Ok(("", Value::True)));
        assert_eq!(parse_boolean("false"), Ok(("", Value::False)));
    }
}
