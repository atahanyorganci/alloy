use std::fmt;

use nom::{branch::alt, bytes::complete::tag, combinator::map, error::context};

use super::{keyword, Input, ParserResult, Spanned, SpannedResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Xor,
    Not,
}

impl Operator {
    pub(crate) fn infix_binding_power(&self) -> (u8, u8) {
        match self {
            Operator::Power => (11, 12),
            Operator::Multiply | Operator::Divide | Operator::Modulo => (9, 10),
            Operator::Plus | Operator::Minus => (7, 8),
            Operator::LessThan
            | Operator::LessThanEqual
            | Operator::GreaterThan
            | Operator::GreaterThanEqual => (5, 6),
            Operator::Equal | Operator::NotEqual => (3, 4),
            Operator::And | Operator::Or | Operator::Xor => (0, 1),
            Operator::Not => unreachable!("`{}` is not a binary operator", self),
        }
    }

    pub(crate) fn prefix_binding_power(&self) -> ((), u8) {
        let bp = match self {
            Operator::Plus => 10,
            Operator::Minus => 10,
            Operator::Not => 2,
            Operator::Multiply
            | Operator::Divide
            | Operator::Modulo
            | Operator::Power
            | Operator::LessThan
            | Operator::LessThanEqual
            | Operator::GreaterThan
            | Operator::GreaterThanEqual
            | Operator::Equal
            | Operator::NotEqual
            | Operator::And
            | Operator::Or
            | Operator::Xor => unreachable!("`{}` is not a prefix unary operator", self),
        };
        ((), bp)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Modulo => write!(f, "%"),
            Operator::Power => write!(f, "**"),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanEqual => write!(f, "<="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanEqual => write!(f, ">="),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::And => write!(f, "and"),
            Operator::Or => write!(f, "or"),
            Operator::Xor => write!(f, "xor"),
            Operator::Not => write!(f, "not"),
        }
    }
}

pub static OPERATORS: phf::Map<&'static str, Operator> = phf_map! {
    "+" => Operator::Plus,
    "-" => Operator::Minus,
    "*" => Operator::Multiply,
    "/" => Operator::Divide,
    "%" => Operator::Modulo,
    "**" => Operator::Power,
    "<" => Operator::LessThan,
    "<=" => Operator::LessThanEqual,
    ">" => Operator::GreaterThan,
    ">=" => Operator::GreaterThanEqual,
    "==" => Operator::Equal,
    "!=" => Operator::NotEqual,
    "and" => Operator::And,
    "or" => Operator::Or,
    "xor" => Operator::Xor,
    "not" => Operator::Not,
};

pub fn parse_plus(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("+")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_minus(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("-")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_multiply(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("*")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_divide(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("/")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_modulo(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("%")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_power(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("**")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_less_than(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("<")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_less_than_equal(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("<=")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_greater_than(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag(">")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_greater_than_equal(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag(">=")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_equal(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("==")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_not_equal(input: Input<'_>) -> ParserResult<'_, Operator> {
    let (input, word) = tag("!=")(input)?;
    let operator = OPERATORS.get(word.input).unwrap();
    Ok((input, *operator))
}

pub fn parse_and(input: Input<'_>) -> ParserResult<'_, Operator> {
    map(keyword::parse_and, |_| Operator::And)(input)
}

pub fn parse_or(input: Input<'_>) -> ParserResult<'_, Operator> {
    map(keyword::parse_or, |_| Operator::Or)(input)
}

pub fn parse_xor(input: Input<'_>) -> ParserResult<'_, Operator> {
    map(keyword::parse_xor, |_| Operator::Xor)(input)
}

pub fn parse_not(input: Input<'_>) -> ParserResult<'_, Operator> {
    map(keyword::parse_not, |_| Operator::Not)(input)
}

pub fn parse_unary_operator(input: Input<'_>) -> SpannedResult<'_, Operator> {
    let start = input.position;
    let (input, operator) =
        context("unary operator", alt((parse_plus, parse_minus, parse_not)))(input)?;
    let spanned = Spanned {
        ast: operator,
        start,
        end: input.position,
    };
    Ok((input, spanned))
}

/// Parse an operator and convert it into `Operator`.
pub fn parse_operator(input: Input<'_>) -> SpannedResult<'_, Operator> {
    let start = input.position;
    // `parse_less_than_equal`, `parse_greater_than_equal`, `parse_power`
    // start with an other operator, so we try it first.
    let (input, operator) = context(
        "operator",
        alt((
            parse_plus,
            parse_minus,
            parse_power,
            parse_multiply,
            parse_divide,
            parse_modulo,
            parse_less_than_equal,
            parse_less_than,
            parse_greater_than_equal,
            parse_greater_than,
            parse_equal,
            parse_not_equal,
            parse_and,
            parse_or,
            parse_xor,
            parse_not,
        )),
    )(input)?;
    let spanned = Spanned {
        ast: operator,
        start,
        end: input.position,
    };
    Ok((input, spanned))
}

#[cfg(test)]
mod tests {
    use crate::parser::Input;

    use super::{parse_operator, OPERATORS};

    #[test]
    fn test_operator_parsing() {
        for (s, op) in OPERATORS.entries() {
            let input = Input::new(s);
            let (input, parsed) = parse_operator(input).unwrap();
            assert_eq!(input, "");
            assert_eq!(parsed, *op);
        }
    }
}
