use super::*;
use pest::iterators::Pair;
use std::convert::Into;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Integer(i32),
    Float(f64),
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(left) => {
                let right: f64 = rhs.into();
                Value::Float(left + right)
            }
            Self::Integer(int) => match rhs {
                Self::Float(right) => {
                    let left: f64 = int.into();
                    Value::Float(left + right)
                }
                Self::Integer(right) => Self::Integer(int + right),
            },
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(left) => {
                let right: f64 = rhs.into();
                Value::Float(left - right)
            }
            Self::Integer(int) => match rhs {
                Self::Float(right) => {
                    let left: f64 = int.into();
                    Value::Float(left - right)
                }
                Self::Integer(right) => Self::Integer(int - right),
            },
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(left) => {
                let right: f64 = rhs.into();
                Value::Float(left * right)
            }
            Self::Integer(int) => match rhs {
                Self::Float(right) => {
                    let left: f64 = int.into();
                    Value::Float(left * right)
                }
                Self::Integer(right) => Self::Integer(int * right),
            },
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        let left: f64 = self.into();
        let right: f64 = rhs.into();
        Value::Float(left / right)
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(left) => {
                let right: f64 = rhs.into();
                Value::Float(left % right)
            }
            Self::Integer(int) => match rhs {
                Self::Float(right) => {
                    let left: f64 = int.into();
                    Value::Float(left % right)
                }
                Self::Integer(right) => Self::Integer(int % right),
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Float(left) => {
                let right: f64 = (*other).into();
                (left - right) < f64::EPSILON
            }
            Self::Integer(int) => match other {
                Self::Float(right) => {
                    let left: f64 = (*int).into();
                    (left - right) < f64::EPSILON
                }
                Self::Integer(right) => int == right,
            },
        }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Self::Float(float) => float,
            Self::Integer(int) => int.into(),
        }
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        match self {
            Self::Float(float) => float.floor() as i32,
            Self::Integer(int) => int,
        }
    }
}

impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Value::Float(float)
    }
}

impl From<i32> for Value {
    fn from(integer: i32) -> Self {
        Value::Integer(integer)
    }
}

impl Expression for Value {
    fn eval(&self) -> Value {
        return *self;
    }
}

pub fn build_value(value: Pair<Rule>) -> Box<Value> {
    match value.as_rule() {
        Rule::integer => {
            let int = alloy_integer(value.as_str()).unwrap();
            Box::new(Value::Integer(int))
        }
        Rule::float => {
            let float = alloy_float(value.as_str()).unwrap();
            Box::from(Value::Float(float))
        }
        _ => unreachable!(),
    }
}

pub fn alloy_integer(integer: &str) -> Result<i32, ()> {
    let replaced = integer.replace(|ch| ch == ' ' || ch == '_', "");
    match replaced.parse::<i32>() {
        Ok(int) => Ok(int),
        Err(_) => Err(()),
    }
}

pub fn alloy_float(float: &str) -> Result<f64, ()> {
    let replaced = float.replace(|ch| ch == ' ' || ch == '_', "");
    match replaced.parse::<f64>() {
        Ok(float) => Ok(float),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser;

    #[test]
    fn value_addtion() {
        assert_eq!(Value::Float(12.0) + Value::Float(12.0), Value::Float(24.0));
        assert_eq!(Value::Float(12.0) + Value::Integer(12), Value::Float(24.0));
        assert_eq!(Value::Integer(12) + Value::Integer(12), Value::Integer(24));
    }

    #[test]
    fn value_subtarction() {
        assert_eq!(Value::Float(12.0) - Value::Float(12.0), Value::Float(0.0));
        assert_eq!(Value::Float(12.0) - Value::Integer(12), Value::Float(0.0));
        assert_eq!(Value::Integer(12) - Value::Integer(12), Value::Integer(0));
    }

    #[test]
    fn value_multiplaction() {
        assert_eq!(Value::Float(12.0) * Value::Float(12.0), Value::Float(144.0));
        assert_eq!(Value::Float(12.0) * Value::Integer(12), Value::Float(144.0));
        assert_eq!(Value::Integer(12) * Value::Integer(12), Value::Integer(144));
    }

    #[test]
    fn value_division() {
        assert_eq!(Value::Float(12.0) / Value::Float(12.0), Value::Float(1.0));
        assert_eq!(Value::Float(12.0) / Value::Integer(12), Value::Float(1.0));
        assert_eq!(Value::Integer(12) / Value::Integer(12), Value::Float(1.0));
    }

    #[test]
    fn value_remainder() {
        assert_eq!(Value::Float(12.0) % Value::Integer(3), Value::Float(0.0));
        assert_eq!(Value::Integer(12) % Value::Integer(3), Value::Integer(0));
    }

    #[test]
    fn value_equality() {
        assert_eq!(Value::Integer(12), Value::Integer(12));
        assert_eq!(Value::Integer(12), Value::Float(12.0));
        assert_eq!(Value::Float(12.0), Value::Float(12.0));
        assert_ne!(Value::Integer(22), Value::Integer(12));
        assert_ne!(Value::Integer(22), Value::Float(12.0));
        assert_ne!(Value::Float(22.0), Value::Float(12.0));
    }

    fn test_integer(string: &str, number: i32) {
        let mut tokens = AlloyParser::parse(Rule::integer, string).unwrap();
        let pair = tokens.next().unwrap();
        let integer = build_value(pair);
        assert_eq!(*integer, number.into());
    }

    fn test_float(string: &str, number: f64) {
        let mut tokens = AlloyParser::parse(Rule::float, string).unwrap();
        let pair = tokens.next().unwrap();
        let float = build_value(pair);
        assert_eq!(*float, number.into());
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
    fn overflow_test() {
        assert!(alloy_integer("1_000_000_000_000").is_err());
    }

    #[test]
    fn parse_float() {
        test_float("1.0", 1.0);
        test_float("-1.2", -1.2);
        test_float(".2", 0.2);
        test_float("1.", 1.0);
        test_float("-1.", -1.0);
        test_float("-.2", -0.2);
    }
}
