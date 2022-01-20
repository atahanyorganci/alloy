use std::{fmt, num::ParseIntError};

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{Parse, ParseResult, ParserError, Rule},
};

use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    True,
    False,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
            Self::Float(float) => write!(f, "{}", float),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

impl From<i64> for Value {
    fn from(integer: i64) -> Self {
        Self::Integer(integer)
    }
}

impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Self::Float(float)
    }
}

impl Compile for Value {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let index = compiler.register_value(self.clone())?;
        compiler.emit(Instruction::LoadValue(index));
        Ok(())
    }
}

impl Parse<'_> for Value {
    fn parse(rule: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(rule.as_rule(), Rule::value);
        let value = rule.into_inner().next().unwrap();
        let result = match value.as_rule() {
            Rule::integer => Value::parse_integer(value)?,
            Rule::float => Value::parse_float(value)?,
            Rule::boolean => {
                let s = value.as_str();
                if s == "true" {
                    Value::True
                } else if s == "false" {
                    Value::False
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        };
        Ok(result)
    }
}

impl Value {
    fn parse_float(pair: Pair<Rule>) -> ParseResult<Self> {
        matches!(pair.as_rule(), Rule::float);
        let float = pair.as_str();
        let replaced = float.replace(|ch| ch == ' ' || ch == '_', "");
        match replaced.parse::<f64>() {
            Ok(float) => Ok(Value::Float(float)),
            Err(e) => Err(ParserError::for_pair(pair, e)),
        }
    }

    fn parse_integer(pair: Pair<Rule>) -> ParseResult<Self> {
        matches!(pair.as_rule(), Rule::integer);
        let span = pair.as_span();

        let mut inner = pair.into_inner();
        let first = inner.next().unwrap();
        match inner.next() {
            Some(rule) => match Value::parse_unsigned_integer(rule) {
                Ok(unsigned) => match first.as_rule() {
                    Rule::plus => Ok(Value::Integer(unsigned)),
                    Rule::minus => Ok(Value::Integer(-unsigned)),
                    _ => unreachable!(),
                },
                Err(e) => Err(ParserError::for_span(span, e)),
            },
            None => match Value::parse_unsigned_integer(first) {
                Ok(int) => Ok(Value::Integer(int)),
                Err(e) => Err(ParserError::for_span(span, e)),
            },
        }
    }

    fn parse_unsigned_integer(pair: Pair<Rule>) -> Result<i64, ParseIntError> {
        match pair.as_rule() {
            Rule::binary => Value::parse_integer_with_radix(pair.as_str(), 2),
            Rule::octal => Value::parse_integer_with_radix(pair.as_str(), 8),
            Rule::decimal => Value::parse_integer_with_radix(pair.as_str(), 10),
            Rule::hexadecimal => Value::parse_integer_with_radix(pair.as_str(), 16),
            _ => unreachable!(),
        }
    }

    fn parse_integer_with_radix(input: &str, radix: u32) -> Result<i64, ParseIntError> {
        let input = match radix {
            2 | 8 | 16 => &input[2..],
            10 => input,
            _ => unreachable!(),
        };
        let input = input.replace(|ch| ch == ' ' || ch == '_', "");
        i64::from_str_radix(&input, radix)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{self, ParseResult, Rule};

    use super::Value;

    fn parse_value(input: &str) -> ParseResult<Value> {
        parser::parse_rule::<Value>(Rule::value, input)
    }

    fn test_integer(input: &str, number: i64) {
        assert_eq!(parse_value(input).unwrap(), number.into());
    }

    fn test_float(input: &str, number: f64) {
        let float = parse_value(input).unwrap();
        assert_eq!(float, number.into());
    }

    #[test]
    fn parse_integer() {
        test_integer("10", 10);
        test_integer("1_000", 1_000);
        test_integer("1_000_000", 1_000_000);
        test_integer("- 100", -100);
        test_integer("- 1_200", -1200);
        test_integer("-100", -100);
        test_integer("-1_200", -1200);
        test_integer("+ 100", 100);
        test_integer("+ 1_200", 1200);
        test_integer("+100", 100);
        test_integer("+1_200", 1200);
    }

    #[test]
    fn parse_radix_integers() {
        test_integer("0xF", 15);
        test_integer("0xFF", 255);
        test_integer("0xFF_FF", 65535);
        test_integer("0o10", 8);
        test_integer("0b101", 5);
        test_integer("- 0xF", -15);
        test_integer("-  0xFF", -255);
        test_integer("- 0xFF_FF", -65535);
        test_integer("- 0o10", -8);
        test_integer("-\t0b101", -5);
        test_integer("+0xF", 15);
        test_integer("+0xFF", 255);
        test_integer("+0xFF_FF", 65535);
        test_integer("+0o10", 8);
        test_integer("+0b101", 5);
    }

    #[test]
    fn overflow_test() {
        let overflow = "1_000_000_000_000_000_000_000_000_000_000";
        assert!(parse_value(overflow).is_err());
        let underflow = "-1_000_000_000_000_000_000_000_000_000_000";
        assert!(parse_value(underflow).is_err());
    }

    #[test]
    fn parse_float() {
        test_float("1.0", 1.);
        test_float("-1.2", -1.2);
        test_float(".2", 0.2);
        test_float("1.", 1.0);
        test_float("-1.", -1.0);
        test_float("-.2", -0.2);
    }
}
