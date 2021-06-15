use std::convert::Into;
use std::ops::{Add, Div, Mul, Rem, Sub};

use super::Expression;

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

impl Expression for Value {
    fn eval(&self) -> Value {
        return *self;
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
