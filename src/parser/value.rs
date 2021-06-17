use super::*;
use pest::iterators::Pair;
use std::convert::Into;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Integer(i32),
    Float(f64),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
            Self::Float(float) => write!(f, "{}", float),
            Self::Bool(bool) => write!(f, "{}", bool),
        }
    }
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
                _ => {
                    let right: i32 = rhs.into();
                    Value::Integer(int + right)
                }
            },
            Self::Bool(b) => {
                if b {
                    Value::Integer(1) + rhs
                } else {
                    rhs
                }
            }
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
                _ => {
                    let right: i32 = rhs.into();
                    Value::Integer(int - right)
                }
            },
            Self::Bool(b) => {
                if b {
                    Value::Integer(1) - rhs
                } else {
                    rhs
                }
            }
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
                _ => {
                    let right: i32 = rhs.into();
                    Value::Integer(int * right)
                }
            },
            Self::Bool(b) => {
                if b {
                    rhs
                } else {
                    Value::Integer(0)
                }
            }
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
                _ => {
                    let right: i32 = rhs.into();
                    Value::Integer(int % right)
                }
            },
            Self::Bool(b) => {
                if b {
                    Value::Integer(1) % rhs
                } else {
                    rhs
                }
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Self::Float(left) => {
                let right: f64 = (*other).into();
                (left - right).abs() < f64::EPSILON
            }
            Self::Integer(int) => match *other {
                Self::Float(right) => {
                    let left: f64 = int.into();
                    (left - right).abs() < f64::EPSILON
                }
                _ => {
                    let right: i32 = (*other).into();
                    int == right
                }
            },
            Self::Bool(b) => match *other {
                Self::Float(right) => {
                    if b {
                        (1.0 - right).abs() < f64::EPSILON
                    } else {
                        right < f64::EPSILON
                    }
                }
                Self::Integer(right) => {
                    if b {
                        right == 1
                    } else {
                        right == 0
                    }
                }
                Self::Bool(right) => b == right,
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Float(left) => {
                let right: f64 = (*other).into();
                left.partial_cmp(&right)
            }
            &Self::Integer(int) => match other {
                Self::Float(right) => {
                    let left: f64 = int.into();
                    left.partial_cmp(right)
                }
                _ => {
                    let right = (*other).into();
                    int.partial_cmp(&right)
                }
            },
            &Self::Bool(b) => {
                if b {
                    Value::Integer(1).partial_cmp(other)
                } else {
                    Value::Integer(0).partial_cmp(other)
                }
            }
        }
    }
}

impl From<Value> for f64 {
    fn from(val: Value) -> Self {
        match val {
            Value::Float(float) => float,
            Value::Integer(int) => int.into(),
            Value::Bool(b) => {
                if b {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

impl From<Value> for i32 {
    fn from(val: Value) -> Self {
        match val {
            Value::Float(float) => float.floor() as i32,
            Value::Integer(int) => int,
            Value::Bool(b) => {
                if b {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl From<Value> for bool {
    fn from(val: Value) -> Self {
        match val {
            Value::Integer(int) => int != 0,
            Value::Float(float) => float != 0.0,
            Value::Bool(b) => b,
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

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl Expression for Value {
    fn eval(&self) -> Value {
        *self
    }
}

impl ASTNode for Value {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let value = match pair.as_rule() {
            Rule::value => pair.into_inner().next().unwrap(),
            _ => return None,
        };
        let result = match value.as_rule() {
            Rule::integer => {
                let int = alloy_integer(value.as_str()).unwrap();
                Box::new(Value::Integer(int))
            }
            Rule::float => {
                let float = alloy_float(value.as_str()).unwrap();
                Box::from(Value::Float(float))
            }
            Rule::boolean => {
                let s = value.as_str();
                if s == "true" {
                    Box::from(Value::Bool(true))
                } else if s == "false" {
                    Box::from(Value::Bool(false))
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        };
        Some(result)
    }
}

impl Value {
    pub fn and(self, rhs: Self) -> Self {
        let left = self.into();
        let right = rhs.into();
        Value::Bool(left && right)
    }

    pub fn or(self, rhs: Self) -> Self {
        let left = self.into();
        let right = rhs.into();
        Value::Bool(left && right)
    }

    pub fn xor(self, rhs: Self) -> Self {
        let left: bool = self.into();
        let right: bool = rhs.into();
        Value::Bool(left != right)
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
    fn addition_with_bool() {
        let five_float = Value::Float(5.0);
        let five_int = Value::Integer(5);
        let six_float = Value::Float(6.0);
        let six_int = Value::Integer(6);
        let one = Value::Bool(true);
        let zero = Value::Bool(false);

        assert_eq!(five_float + one, six_float);
        assert_eq!(five_float + zero, five_float);
        assert_eq!(five_int + one, six_int);
        assert_eq!(five_int + zero, five_int);
    }

    #[test]
    fn value_subtarction() {
        assert_eq!(Value::Float(12.0) - Value::Float(12.0), Value::Float(0.0));
        assert_eq!(Value::Float(12.0) - Value::Integer(12), Value::Float(0.0));
        assert_eq!(Value::Integer(12) - Value::Integer(12), Value::Integer(0));
    }

    #[test]
    fn subtraction_with_bool() {
        let five_float = Value::Float(5.0);
        let five_int = Value::Integer(5);
        let four_float = Value::Float(4.0);
        let four_int = Value::Integer(4);
        let one = Value::Bool(true);
        let zero = Value::Bool(false);

        assert_eq!(five_float - one, four_float);
        assert_eq!(five_float - zero, five_float);
        assert_eq!(five_int - one, four_int);
        assert_eq!(five_int - zero, five_int);
    }

    #[test]
    fn value_multiplaction() {
        assert_eq!(Value::Float(12.0) * Value::Float(12.0), Value::Float(144.0));
        assert_eq!(Value::Float(12.0) * Value::Integer(12), Value::Float(144.0));
        assert_eq!(Value::Integer(12) * Value::Integer(12), Value::Integer(144));
    }

    #[test]
    fn multiplaction_with_bool() {
        let five_float = Value::Float(5.0);
        let five_int = Value::Integer(5);
        let zero_float = Value::Float(0.0);
        let zero_int = Value::Integer(0);
        let one = Value::Bool(true);
        let zero = Value::Bool(false);

        assert_eq!(five_float * one, five_float);
        assert_eq!(five_float * zero, zero_float);
        assert_eq!(five_int * one, five_float);
        assert_eq!(five_int * zero, zero_int);
    }

    #[test]
    fn value_division() {
        assert_eq!(Value::Float(12.0) / Value::Float(12.0), Value::Float(1.0));
        assert_eq!(Value::Float(12.0) / Value::Bool(true), Value::Float(12.0));
        assert_eq!(Value::Float(12.0) / Value::Integer(12), Value::Float(1.0));
        assert_eq!(Value::Integer(12) / Value::Integer(12), Value::Float(1.0));
    }

    #[test]
    fn value_remainder() {
        assert_eq!(Value::Float(12.0) % Value::Integer(3), Value::Float(0.0));
        assert_eq!(Value::Integer(12) % Value::Integer(3), Value::Integer(0));
        assert_eq!(Value::Float(12.0) % Value::Bool(true), Value::Float(0.0));
        assert_eq!(Value::Integer(12) % Value::Bool(true), Value::Integer(0));
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
        let mut tokens = AlloyParser::parse(Rule::value, string).unwrap();
        let pair = tokens.next().unwrap();
        let integer = Value::build(pair).unwrap();
        assert_eq!(*integer, number.into());
    }

    fn test_float(string: &str, number: f64) {
        let mut tokens = AlloyParser::parse(Rule::value, string).unwrap();
        let pair = tokens.next().unwrap();
        let float = Value::build(pair).unwrap();
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
