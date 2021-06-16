use pest::{
    iterators::{Pair, Pairs},
    prec_climber::*,
};

use crate::parser::value::build_value;

use super::*;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<super::Rule> = {
        use super::Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(logical_xor, Left),
            Operator::new(logical_or, Left),
            Operator::new(logical_and, Left),
            Operator::new(equal_to, Left) | Operator::new(not_equal_to, Left),
            Operator::new(less_than, Left)
                | Operator::new(greater_than, Left)
                | Operator::new(less_than_eq, Left)
                | Operator::new(greater_than_eq, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

pub struct BinaryExpression {
    left: Box<dyn Expression>,
    operator: BinaryOperator,
    right: Box<dyn Expression>,
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Expression for BinaryExpression {
    fn eval(&self) -> super::Value {
        let left = self.left.eval();
        let right = self.right.eval();
        match self.operator {
            BinaryOperator::Add => left + right,
            BinaryOperator::Subtract => left - right,
            BinaryOperator::Multiply => left * right,
            BinaryOperator::Divide => left / right,
            BinaryOperator::Reminder => left % right,
            BinaryOperator::Power => unimplemented!(),
            BinaryOperator::LessThan => (left < right).into(),
            BinaryOperator::LessThanEqual => (left <= right).into(),
            BinaryOperator::GreaterThan => (left > right).into(),
            BinaryOperator::GreaterThanEqual => (left >= right).into(),
            BinaryOperator::Equal => (left == right).into(),
            BinaryOperator::NotEqual => (left != right).into(),
            BinaryOperator::LogicalAnd => left.and(right),
            BinaryOperator::LogicalOr => left.or(right),
            BinaryOperator::LogicalXor => left.xor(right),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Reminder,
    Power,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &BinaryOperator::Add => write!(f, "+"),
            &BinaryOperator::Subtract => write!(f, "-"),
            &BinaryOperator::Multiply => write!(f, "*"),
            &BinaryOperator::Divide => write!(f, "/"),
            &BinaryOperator::Reminder => write!(f, "%"),
            &BinaryOperator::Power => write!(f, "**"),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanEqual => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanEqual => write!(f, ">="),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::LogicalAnd => write!(f, "and"),
            BinaryOperator::LogicalOr => write!(f, "or"),
            BinaryOperator::LogicalXor => write!(f, "xor"),
        }
    }
}

pub fn build_binary_expression(expression: Pairs<Rule>) -> Box<dyn Expression> {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| -> Box<dyn Expression> {
            match pair.as_rule() {
                Rule::value => build_value(pair.into_inner().next().unwrap()),
                Rule::expression => build_binary_expression(pair.into_inner()),
                _ => panic!("SHOULD NOT PARSE {}", pair),
            }
        },
        |left: Box<dyn Expression>,
         op: Pair<Rule>,
         right: Box<dyn Expression>|
         -> Box<dyn Expression> {
            let operator = match op.as_rule() {
                Rule::add => BinaryOperator::Add,
                Rule::subtract => BinaryOperator::Subtract,
                Rule::multiply => BinaryOperator::Multiply,
                Rule::divide => BinaryOperator::Divide,
                Rule::power => BinaryOperator::Power,
                Rule::less_than => BinaryOperator::LessThan,
                Rule::less_than_eq => BinaryOperator::LessThanEqual,
                Rule::greater_than => BinaryOperator::GreaterThan,
                Rule::greater_than_eq => BinaryOperator::GreaterThanEqual,
                Rule::equal_to => BinaryOperator::Equal,
                Rule::not_equal_to => BinaryOperator::NotEqual,
                Rule::logical_and => BinaryOperator::LogicalAnd,
                Rule::logical_or => BinaryOperator::LogicalOr,
                Rule::logical_xor => BinaryOperator::LogicalXor,
                _ => unreachable!(),
            };
            Box::from(BinaryExpression {
                left,
                operator,
                right,
            })
        },
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
