use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::char,
    combinator::opt,
    error::context,
    sequence::{preceded, separated_pair},
};

use crate::ast::value::Value;

use super::{Input, ParserResult, Spanned, SpannedResult};

/// Sign of a number `Positive` for `+` and `Negative` for `-`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

/// Default of `Sign` is `Positive` since if a number is not
/// prefixed with a sign it is assumed to postive.
impl Default for Sign {
    fn default() -> Self {
        Self::Positive
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive => write!(f, "+"),
            Self::Negative => write!(f, "-"),
        }
    }
}

/// Parses a string into an `ast::Value::False` or `ast::Value::True`
/// or returns an error.
///
/// # Examples
/// ```
/// use alloy::{ast::value::Value, parser::literal::parse_bool};
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
/// use alloy::parser::literal::parse_whitespace;
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

/// Parse sign of a number either `+` or `-` into `Sign`.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::{parse_sign, Sign};
///
/// let (input, sign) = parse_sign("+".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(sign, Sign::Positive);
///
/// let (input, sign) = parse_sign("-".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(sign, Sign::Negative);
/// ```
///
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

/// Parse one or more digits with given radix and underscores can be used
/// for improved readability for large constants.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_digits;
///
/// // Parse decimal digits (base/radix 10)
/// let (input, digits) = parse_digits("123".into(), 10).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(digits, 123);
///
/// // Parse decimal digits (base/radix 10)
/// let (input, digits) = parse_digits("1_000_000".into(), 10).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(digits, 1_000_000);
///
/// // Parse hexadecimal digits (base/radix 16)
/// let (input, digits) = parse_digits("FF12a".into(), 16).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(digits, 1044778);
/// ```
///
/// # Errors
///
/// This function will return an error if given input doesn't contain digits of given radix.
pub fn parse_digits(input: Input<'_>, radix: u32) -> ParserResult<'_, i64> {
    let (input, digits) = context(
        "digits",
        take_while1(|c: char| c.is_digit(radix) || c == '_'),
    )(input)?;
    if digits.input.starts_with("_") {
        todo!("parse_digits: handle underscores");
    }
    let number = i64::from_str_radix(&digits.input.replace("_", ""), radix).unwrap();
    Ok((input, number))
}

/// Parse decimal integer into `i64` and convert it to `Value::Integer`.
///
/// # Examples
///
/// ```
/// use alloy::{ast::value::Value, parser::literal::parse_decimal};
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
    parse_radix_integer(input, 10, None)
}

/// Parse hexadecimal integer into `i64` and convert it to `Value::Integer`.
///
/// # Examples
///
/// ```
/// use alloy::{ast::value::Value, parser::literal::parse_hexadecimal};
///
/// let (input, value) = parse_hexadecimal("0xFF".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(255));
///
/// let (input, value) = parse_hexadecimal("0xab".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(171));
///
/// ```
///
/// # Errors
///
/// This function will return an error if it contains doesn't start with
/// binary prefix `0x` or contains invalid hexadecimal digits.
pub fn parse_hexadecimal(input: Input<'_>) -> SpannedResult<'_, Value> {
    parse_radix_integer(input, 16, Some("0x"))
}

/// Parse octal integer into `i64` and convert it to `Value::Integer`.
///
/// # Examples
///
/// ```
/// use alloy::{ast::value::Value, parser::literal::parse_octal};
///
/// let (input, value) = parse_octal("0o77".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(63));
///
/// let (input, value) = parse_octal("0o11".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(9));
///
/// ```
///
/// # Errors
///
/// This function will return an error if it contains doesn't start with
/// binary prefix `0o` or contains invalid octal digits.
pub fn parse_octal(input: Input<'_>) -> SpannedResult<'_, Value> {
    parse_radix_integer(input, 8, Some("0o"))
}

/// Parse binary number into `i64` and convert it to `Value::Integer`.
///
/// # Examples
///
/// ```
/// use alloy::{ast::value::Value, parser::literal::parse_binary};
///
/// let (input, value) = parse_binary("0b101".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(5));
///
/// let (input, value) = parse_binary("0b1001".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(9));
///
/// ```
///
/// # Errors
///
/// This function will return an error if it contains doesn't start with
/// binary prefix `0b` or contains invalid binary digits.
pub fn parse_binary(input: Input<'_>) -> SpannedResult<'_, Value> {
    parse_radix_integer(input, 2, Some("0b"))
}

/// Parse hexadecimal integer into `i64` and convert it to `Value::Integer`.
///
/// # Examples
///
/// ```
/// use alloy::{ast::value::Value, parser::literal::parse_radix_integer};
///
/// let (input, value) = parse_radix_integer("0xFF".into(), 16, "0x".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(255));
///
/// let (input, value) = parse_radix_integer("0b101".into(), 2, "0b".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Integer(5));
///
/// ```
///
/// # Errors
///
/// This function will return an error if it contains doesn't start with given prefix
/// or contains invalid digits for given radix.
pub fn parse_radix_integer<'a>(
    input: Input<'a>,
    radix: u32,
    prefix: Option<&'a str>,
) -> SpannedResult<'a, Value> {
    let start = input.position;

    // Parse sign of the number or default to positive
    let (input, sign) = context("radix integer", opt(parse_sign))(input)?;
    let sign = if let Some(sign) = sign {
        sign.ast
    } else {
        Sign::default()
    };

    // Any number of whitespace characters can follow the sign
    let (input, _) = parse_whitespace(input)?;

    // and raidx is not 10, then we need to parse the prefix
    let input = if let Some(prefix) = prefix {
        let (input, _) = context("radix integer prefix", tag(prefix))(input)?;
        input
    } else {
        input
    };

    // FIXME: Instead of unwrapping result here, we should return an error
    let (input, integer) = context("radix integer", |input| parse_digits(input, radix))(input)?;
    let integer = match sign {
        Sign::Positive => Value::Integer(integer),
        Sign::Negative => Value::Integer(-integer),
    };

    let spanned = Spanned {
        ast: integer,
        start,
        end: input.position,
    };
    Ok((input, spanned))
}

/// Parse integer number not limited to decimal including binary, octal, hexadecimal.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_integer;
///
/// assert!(parse_integer("124".into()).is_ok());
/// assert!(parse_integer("0b111".into()).is_ok());
/// assert!(parse_integer("0o111".into()).is_ok());
/// assert!(parse_integer("0xb1AF".into()).is_ok());
/// ```
///
pub fn parse_integer(input: Input<'_>) -> SpannedResult<'_, Value> {
    context(
        "integer",
        alt((parse_decimal, parse_hexadecimal, parse_octal, parse_binary)),
    )(input)
}

/// Simple wrapper around `parse_digits` with radix 10.
fn parse_decimal_digits(input: Input<'_>) -> ParserResult<'_, i64> {
    parse_digits(input, 10)
}

/// Parse digits of floating point number whole part (before decimal point)
/// of the number is optional.
fn parse_float_optional(input: Input<'_>) -> ParserResult<'_, (i64, i64)> {
    let (input, (whole, fractional)) =
        separated_pair(opt(parse_decimal_digits), tag("."), parse_decimal_digits)(input)?;
    let whole = whole.unwrap_or_default();
    Ok((input, (whole, fractional)))
}

/// Parse digits of floating point number fractional part (after decimal point)
/// of the number is optional.
fn parse_float_dot_optional(input: Input<'_>) -> ParserResult<'_, (i64, i64)> {
    let (input, (whole, fractional)) =
        separated_pair(parse_decimal_digits, tag("."), opt(parse_decimal_digits))(input)?;
    let fractional = fractional.unwrap_or_default();
    Ok((input, (whole, fractional)))
}

/// Scale down a floating point number by power 10 until it's between 0 and 1.
fn fractional_part(mut float: f64) -> f64 {
    while float > 1.0 {
        float /= 10.0;
    }
    float
}

/// Return a signed floating point number from a whole and fractional part.
fn float_from_parts(sign: Sign, whole: i64, fractional: i64) -> Value {
    let float = whole as f64 + fractional_part(fractional as f64);
    let float = match sign {
        Sign::Positive => float,
        Sign::Negative => -float,
    };
    Value::Float(float)
}

/// Parse floating point number into `f64` and convert it to `Value::Float`.
/// Floating point numbers can omit either whole part (before decimal point)
/// or fractional part (after decimal point) but not both. If whole part of
/// the number is omitted, it is assumed to be 0, same goes for fractional part.
/// So, for example, `1.` is parsed as `1.0` and `.1` is parsed as `0.1`.
///
/// # Examples
///
/// ```
/// use alloy::{ast::value::Value, parser::literal::parse_float};
///
/// let (input, float) = parse_float("1.23".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(float, Value::Float(1.23));
///
/// let (input, float) = parse_float("145.15".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(float, Value::Float(145.15));
///
/// let (input, float) = parse_float("- 145.15".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(float, Value::Float(-145.15));
///
/// let (input, float) = parse_float(".15".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(float, Value::Float(0.15));
///
/// let (input, float) = parse_float("5.".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(float, Value::Float(5.0));
///
/// let (input, float) = parse_float("5_000.600_600".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(float, Value::Float(5000.6006));
/// ```
///
/// # Errors
///
/// This function will return an error if input doesn't contain a valid floating point number.
pub fn parse_float(input: Input<'_>) -> SpannedResult<'_, Value> {
    let start = input.position;
    let (input, sign) = context("float", opt(parse_sign))(input)?;
    let sign = if let Some(sign) = sign {
        sign.ast
    } else {
        Sign::default()
    };
    let (input, _) = parse_whitespace(input)?;
    let (input, (whole, fractional)) = context(
        "float",
        alt((parse_float_optional, parse_float_dot_optional)),
    )(input)?;
    let float = float_from_parts(sign, whole, fractional);
    let spanned = Spanned {
        ast: float,
        start,
        end: input.position,
    };
    Ok((input, spanned))
}

/// Parse null value and convert it to `Value::Null`.
///
/// # Examples
///
/// ```
/// use alloy::{parser::literal::parse_null, ast::value::Value};
///
/// let (input, value) = parse_null("null".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(value, Value::Null)
/// ```
///
/// # Errors
///
/// This function will return an error if input doesn't contain null.
pub fn parse_null(input: Input<'_>) -> SpannedResult<'_, Value> {
    let start = input.position;
    let (input, _) = context("null", tag("null"))(input)?;
    let spanned = Spanned {
        start,
        ast: Value::Null,
        end: input.position,
    };
    Ok((input, spanned))
}

fn parse_escape_seq(input: Input<'_>, escape: char) -> ParserResult<'_, char> {
    let (input, escaped) = context("escape sequence", preceded(char('\\'), char(escape)))(input)?;
    Ok((input, escaped))
}

/// Parse newline escape sequence.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_newline;
///
/// let (input, newline) = parse_newline(r"\n".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(newline, '\n');
/// ```
pub fn parse_newline(input: Input<'_>) -> ParserResult<'_, char> {
    let (input, _) = parse_escape_seq(input, 'n')?;
    Ok((input, '\n'))
}

/// Parse tab escape sequence.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_tab;
///
/// let (input, tab) = parse_tab(r"\t".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(tab, '\t');
/// ```
pub fn parse_tab(input: Input<'_>) -> ParserResult<'_, char> {
    let (input, _) = parse_escape_seq(input, 't')?;
    Ok((input, '\t'))
}

/// Parse carriage return escape sequence.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_carriage_return;
///
/// let (input, carriage_return) = parse_carriage_return(r"\r".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(carriage_return, '\r');
/// ```
pub fn parse_carriage_return(input: Input<'_>) -> ParserResult<'_, char> {
    let (input, _) = parse_escape_seq(input, 'r')?;
    Ok((input, '\r'))
}

/// Parse backslash escape sequence.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_backslash;
///
/// let (input, backslash) = parse_backslash(r"\\".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(backslash, '\\');
/// ```
pub fn parse_backslash(input: Input<'_>) -> ParserResult<'_, char> {
    let (input, _) = parse_escape_seq(input, '\\')?;
    Ok((input, '\\'))
}

/// Parse double quote escape sequence.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_double_quote;
///
/// let (input, double_quote) = parse_double_quote(r#"\""#.into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(double_quote, '"');
/// ```
pub fn parse_double_quote(input: Input<'_>) -> ParserResult<'_, char> {
    let (input, _) = parse_escape_seq(input, '"')?;
    Ok((input, '"'))
}

/// Parse  quote escape sequence.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_quote;
///
/// let (input, quote) = parse_quote(r#"\'"#.into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(quote, '\'');
/// ```
pub fn parse_quote(input: Input<'_>) -> ParserResult<'_, char> {
    let (input, _) = parse_escape_seq(input, '\'')?;
    Ok((input, '\''))
}

/// Escape sequence used in strings such as `\n`, `\t`, `\r` and `\"`.
///
/// # Examples
///
/// ```
/// use alloy::parser::literal::parse_escaped;
///
/// let (input, newline) = parse_escaped(r"\n\t".into()).unwrap();
/// assert_eq!(input, r"\t");
/// assert_eq!(newline, '\n');
///
/// let (input, tab) = parse_escaped(input).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(tab, '\t');
/// ```
///
/// # Errors
///
/// This function will return an error if input doesn't start with escape sequence.
pub fn parse_escaped(input: Input<'_>) -> ParserResult<'_, char> {
    alt((
        parse_newline,
        parse_tab,
        parse_carriage_return,
        parse_backslash,
        parse_double_quote,
        parse_quote,
    ))(input)
}

pub fn parse_string_char(_input: Input<'_>) -> ParserResult<'_, char> {
    todo!()
}

pub fn parse_string(_input: Input<'_>) -> SpannedResult<'_, Value> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::value::Value,
        parser::literal::{parse_sign, Sign},
    };

    use super::{parse_bool, parse_escaped};

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

    #[test]
    fn test_escape_sequences() {
        let (input, newline) = parse_escaped(r#"\n\r\t\\\"\'"#.into()).unwrap();
        assert_eq!(newline, '\n');
        let (input, carriage_return) = parse_escaped(input).unwrap();
        assert_eq!(carriage_return, '\r');
        let (input, tab) = parse_escaped(input).unwrap();
        assert_eq!(tab, '\t');
        let (input, backslash) = parse_escaped(input).unwrap();
        assert_eq!(backslash, '\\');
        let (input, double_quote) = parse_escaped(input).unwrap();
        assert_eq!(double_quote, '"');
        let (input, quote) = parse_escaped(input).unwrap();
        assert_eq!(quote, '\'');
        assert_eq!(input, "");
    }
}
