use std::fmt;

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{Parse, ParserError, Rule},
};

use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    True,
    False,
}

#[derive(Debug)]
pub enum ParseValueError {
    InvalidRadix(u32),
    IntegerOverflow,
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
            Rule::integer => Value::parse_integer(value).unwrap(),
            Rule::float => Value::parse_float(value).unwrap(),
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
    fn parse_float(pair: Pair<Rule>) -> Result<Self, ParseValueError> {
        matches!(pair.as_rule(), Rule::float);
        let float = {
            let float = pair.as_str();
            let replaced = float.replace(|ch| ch == ' ' || ch == '_', "");
            match replaced.parse::<f64>() {
                Ok(float) => Ok(float),
                Err(_) => unreachable!(),
            }
        }?;
        Ok(Value::Float(float))
    }

    fn parse_integer(pair: Pair<Rule>) -> Result<Self, ParseValueError> {
        matches!(pair.as_rule(), Rule::integer);

        let mut inner = pair.into_inner();
        let first = inner.next().unwrap();
        let integer = match inner.next() {
            Some(rule) => {
                let unsigned = Value::parse_unsigned_integer(rule)?;
                match first.as_rule() {
                    Rule::plus => unsigned,
                    Rule::minus => -unsigned,
                    _ => unreachable!(),
                }
            }
            None => Value::parse_unsigned_integer(first)?,
        };
        Ok(Value::Integer(integer))
    }

    fn parse_unsigned_integer(pair: Pair<Rule>) -> Result<i64, ParseValueError> {
        match pair.as_rule() {
            Rule::binary => Value::parse_integer_with_radix(pair.as_str(), 2),
            Rule::octal => Value::parse_integer_with_radix(pair.as_str(), 8),
            Rule::decimal => Value::parse_integer_with_radix(pair.as_str(), 10),
            Rule::hexadecimal => Value::parse_integer_with_radix(pair.as_str(), 16),
            _ => unreachable!(),
        }
    }

    fn parse_integer_with_radix(integer: &str, radix: u32) -> Result<i64, ParseValueError> {
        let replaced = integer.replace(|ch| ch == ' ' || ch == '_', "");
        let source = match radix {
            2 | 8 | 16 => &replaced.as_str()[2..],
            10 => replaced.as_str(),
            _ => return Err(ParseValueError::InvalidRadix(radix)),
        };
        match i64::from_str_radix(source, radix) {
            Ok(integer) => Ok(integer),
            Err(_) => Err(ParseValueError::IntegerOverflow),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::AlloyParser;

    use super::*;
    use pest::Parser;

    fn test_integer(string: &str, number: i64) {
        let mut tokens = AlloyParser::parse(Rule::value, string).unwrap();
        let pair = tokens.next().unwrap();
        let integer = Value::parse(pair).unwrap();
        assert_eq!(integer, number.into());
    }

    fn test_float(string: &str, number: f64) {
        let mut tokens = AlloyParser::parse(Rule::value, string).unwrap();
        let pair = tokens.next().unwrap();
        let float = Value::parse(pair).unwrap();
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
        let num = "1_000_000_000_000_000_000_000_000_000_000";
        assert!(Value::parse_integer_with_radix(num, 10).is_err());
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
