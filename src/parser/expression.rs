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
        match self.operator {
            BinaryOperator::Add => self.left.eval() + self.right.eval(),
            BinaryOperator::Subtract => self.left.eval() - self.right.eval(),
            BinaryOperator::Multiply => self.left.eval() * self.right.eval(),
            BinaryOperator::Divide => self.left.eval() / self.right.eval(),
            BinaryOperator::Reminder => self.left.eval() % self.right.eval(),
            BinaryOperator::Power => unimplemented!(),
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
        }
    }
}

pub fn build_binary_expression(expression: Pairs<Rule>) -> Box<dyn Expression> {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| -> Box<dyn Expression> {
            match pair.as_rule() {
                Rule::number => build_value(pair.into_inner().next().unwrap()),
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
