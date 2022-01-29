use std::num::{ParseFloatError, ParseIntError};

use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::opt,
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

pub use self::input::Input;

pub mod input;

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

type ParserResult<'a, T> = IResult<Input<'a>, T, VerboseError<Input<'a>>>;
type SpannedResult<'a, T> = ParserResult<'a, Spanned<T>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub ast: T,
    pub start: usize,
    pub end: usize,
}

impl<T> PartialEq<T> for Spanned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.ast == *other
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
///
/// let (input, value) = parse_bool("true".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::True);
///
/// let (input, value) = parse_bool("false".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::False);
/// ```
pub fn parse_bool(input: Input<'_>) -> SpannedResult<'_, Value> {
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

/// Parse consecutive whitespaces including newline and carriage return
/// characters.
///
/// # Examples
///
/// ```
/// use alloy::parser::parse_whitespace;
///
/// let (input, whitespace) = parse_whitespace(" \t\r\n123".into()).unwrap();
/// assert_eq!(input, "123");
/// assert_eq!(whitespace, " \t\r\n");
/// ```
pub fn parse_whitespace(input: Input<'_>) -> ParserResult<'_, Input<'_>> {
    let (input, whitespace) = context(
        "whitespace",
        take_while(|p: char| p.is_whitespace() || p == '\n' || p == '\r'),
    )(input)?;
    Ok((input, whitespace))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Default for Sign {
    fn default() -> Self {
        Self::Positive
    }
}

/// Parse sign of a number either `+` or `-` into `Sign`.
///
/// # Examples
///
/// ```
/// use alloy::parser::{parse_sign, Sign};
///
/// assert_eq!(parse_sign("+".into()).unwrap(), ("".into(), Sign::Positive));
/// assert_eq!(parse_sign("-".into()).unwrap(), ("".into(), Sign::Negative));
/// ```
///
/// # Errors
///
/// This function will return an error if .
pub fn parse_sign(input: Input<'_>) -> SpannedResult<Sign> {
    let start = input.position;
    let (next_input, sign) = context("sign", alt((tag("+"), tag("-"))))(input)?;
    let sign = if sign == "+" {
        Sign::Positive
    } else {
        Sign::Negative
    };
    let spanned = Spanned {
        ast: sign,
        start,
        end: next_input.position,
    };
    Ok((next_input, spanned))
}

/// Parse one or more digits with given radix.
///
/// # Examples
///
/// ```
/// use alloy::parser::parse_digits;
///
/// // Parce decimal digits (base/radix 10)
/// let (input, digits) = parse_digits("123".into(), 10).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(digits, "123");
///
/// // Parce hexadecimal digits (base/radix 16)
/// let (input, digits) = parse_digits("FF12a".into(), 16).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(digits, "FF12a");
/// ```
///
/// # Errors
///
/// This function will return an error if given input doesn't contain digits of given radix.
pub fn parse_digits(input: Input<'_>, radix: u32) -> ParserResult<'_, Input<'_>> {
    context("digits", take_while(|p: char| p.is_digit(radix)))(input)
}

/// Parse decimal integer into `i64` and convert it to `Value::Integer`.
///
/// # Examples
///
/// ```
/// use alloy::{ast::value::Value, parser::parse_decimal};
///
/// let (input, value) = parse_decimal("123".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(123));
///
/// let (input, value) = parse_decimal("- 123".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(-123));
///
/// ```
///
/// # Errors
///
/// This function will return an error if doesn't contain decimal digits.
pub fn parse_decimal(input: Input<'_>) -> SpannedResult<'_, Value> {
    let start = input.position;
    let (input, sign) = context("decimal", opt(parse_sign))(input)?;
    let sign = if let Some(sign) = sign {
        sign.ast
    } else {
        Sign::default()
    };
    let (input, _) = parse_whitespace(input)?;
    let (input, digits) = parse_digits(input, 10)?;
    // FIXME: Instead of unwrapping result here, we should return an error
    let int = match i64::from_str_radix(digits.into(), 10) {
        Ok(int) if sign == Sign::Positive => int,
        Ok(int) if sign == Sign::Negative => -int,
        Err(err) => todo!("unhandled error, `{}`", err),
        _ => unreachable!(),
    };
    let spanned = Spanned {
        ast: Value::Integer(int),
        start,
        end: input.position,
    };
    Ok((input, spanned))
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::value::Value,
        parser::{parse_sign, Sign},
    };

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

    #[test]
    fn test_sign() {
        let (rest, spanned) = parse_sign("+".into()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(spanned.ast, Sign::Positive);
        assert_eq!(spanned.start, 0);
        assert_eq!(spanned.end, 1);
        let (rest, spanned) = parse_sign("-".into()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(spanned.ast, Sign::Negative);
        assert_eq!(spanned.start, 0);
        assert_eq!(spanned.end, 1);
    }
}
