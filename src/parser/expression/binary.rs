use std::fmt;

use pest::{iterators::Pair, prec_climber::*};

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{
        expression::{build_expression, identifier::IdentifierExpression},
        value::Value,
        ASTNode, Expression, Rule,
    },
};

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

impl fmt::Debug for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?} {} {:?})", self.left, self.operator, self.right)
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

impl Compile for BinaryExpression {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        self.left.compile(compiler)?;
        self.right.compile(compiler)?;
        let instruction = match self.operator {
            BinaryOperator::Add => Instruction::BinaryAdd,
            BinaryOperator::Subtract => Instruction::BinarySubtract,
            BinaryOperator::Multiply => Instruction::BinaryMultiply,
            BinaryOperator::Divide => Instruction::BinaryDivide,
            BinaryOperator::Reminder => Instruction::BinaryReminder,
            BinaryOperator::Power => Instruction::BinaryPower,
            BinaryOperator::LessThan => Instruction::BinaryLessThan,
            BinaryOperator::LessThanEqual => Instruction::BinaryLessThanEqual,
            BinaryOperator::GreaterThan => Instruction::BinaryGreaterThan,
            BinaryOperator::GreaterThanEqual => Instruction::BinaryGreaterThanEqual,
            BinaryOperator::Equal => Instruction::BinaryEqual,
            BinaryOperator::NotEqual => Instruction::BinaryNotEqual,
            BinaryOperator::LogicalAnd => Instruction::BinaryLogicalAnd,
            BinaryOperator::LogicalOr => Instruction::BinaryLogicalOr,
            BinaryOperator::LogicalXor => Instruction::BinaryLogicalXor,
        };
        compiler.emit(instruction);
        Ok(())
    }
}

impl Expression for BinaryExpression {
    fn eval(&self) -> Value {
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

impl ASTNode for BinaryExpression {
    fn build(pair: Pair<super::Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let _expression = match pair.as_rule() {
            Rule::expression => pair.into_inner(),
            _ => return None,
        };
        todo!()
    }
}

pub(crate) fn build_binary_expression(pair: Pair<Rule>) -> Box<dyn Expression> {
    let expression = match pair.as_rule() {
        Rule::binary_expression => pair.into_inner(),
        _ => unreachable!(),
    };
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| -> Box<dyn Expression> {
            match pair.as_rule() {
                Rule::value => Value::build(pair).unwrap(),
                Rule::expression => build_expression(pair).unwrap(),
                Rule::identifier => IdentifierExpression::build(pair).unwrap(),
                _ => unreachable!("{}", pair),
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
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Reminder => write!(f, "%"),
            BinaryOperator::Power => write!(f, "**"),
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

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::parser::{value::Value, AlloyParser, Rule};

    use super::build_binary_expression;

    fn parse_binary_expression(input: &str) -> Value {
        let mut tokens = AlloyParser::parse(Rule::binary_expression, input).unwrap();
        build_binary_expression(tokens.next().unwrap()).eval()
    }

    #[test]
    fn build_expression_test() {
        assert_eq!(parse_binary_expression("1 + 1"), 2.into());
        assert_eq!(parse_binary_expression("1 + 2 * 3"), 7.into());
        assert_eq!(parse_binary_expression("(1 + 2) * 3"), 9.into());
        assert_eq!(parse_binary_expression("1 - 1"), 0.into());
        assert_eq!(parse_binary_expression("1 + 2 + 3"), 6.into());
        assert_eq!(parse_binary_expression("(1 + 2) / 3"), 1.into());
    }
}
