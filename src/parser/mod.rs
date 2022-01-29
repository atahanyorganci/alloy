use std::num::{ParseFloatError, ParseIntError};

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    error::{context, VerboseError},
    Compare, IResult, InputTake,
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

type Res<'a, T> = IResult<ParserInput<'a>, Spanned<T>, VerboseError<ParserInput<'a>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserInput<'a> {
    s: &'a str,
    position: usize,
}

impl<'a> From<&'a str> for ParserInput<'a> {
    fn from(s: &'a str) -> Self {
        Self { s, position: 0 }
    }
}

impl PartialEq<&str> for ParserInput<'_> {
    fn eq(&self, s: &&str) -> bool {
        self.s == *s
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub ast: T,
    pub start: usize,
    pub end: usize,
}

impl InputTake for ParserInput<'_> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        let s = &self.s[..count];
        let position = self.position + count;
        Self { s, position }
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.s.split_at(count);
        let prefix = Self {
            s: prefix,
            position: self.position,
        };
        let suffix = Self {
            s: suffix,
            position: count,
        };
        (suffix, prefix)
    }
}

impl<T> PartialEq<T> for Spanned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.ast == *other
    }
}

impl Compare<&str> for ParserInput<'_> {
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.current().compare(t)
    }

    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.current().compare_no_case(t)
    }
}

impl ParserInput<'_> {
    pub fn current(&self) -> &str {
        &self.s[self.position..]
    }
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

/// Parses a string into an `ast::Value::False` or `ast::Value::True`
/// or returns an error.
///
/// # Examples
/// ```
/// use alloy::{ast::value::Value, parser::parse_bool};
/// assert_eq!(parse_bool("true"), Ok(Value::True));
/// assert_eq!(parse_bool("false"), Ok(Value::False));
/// ```
pub fn parse_bool<'a>(input: ParserInput<'a>) -> Res<'a, Value> {
    let start = input.position;
    let (next_input, res) = context("bool", alt((tag("true"), tag("false"))))(input)?;
    let value = if res == "true" {
        Value::True
    } else {
        Value::False
    };
    let spanned = Spanned {
        ast: value,
        start,
        end: next_input.position,
    };
    Ok((next_input, spanned))
}

#[cfg(test)]
mod tests {
    use crate::ast::value::Value;

    use super::parse_bool;

    #[test]
    fn test_boolean() {
        let (rest, spanned) = parse_bool("true".into()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(spanned.ast, Value::True);
        assert_eq!(spanned.start, 0);
        assert_eq!(spanned.end, 4);
        let (rest, spanned) = parse_bool("false".into()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(spanned.ast, Value::False);
        assert_eq!(spanned.start, 0);
        assert_eq!(spanned.end, 5);
    }
}
