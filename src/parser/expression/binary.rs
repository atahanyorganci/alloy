use std::fmt;

use pest::{iterators::Pair, prec_climber::*};

use crate::parser::{expression::identifier::Identifier, value::Value, ASTNode, ParserError, Rule};

use super::Expression;

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
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
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

// impl Compile for BinaryExpression {
//     fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
//         self.left.compile(compiler)?;
//         self.right.compile(compiler)?;
//         let instruction = match self.operator {
//             BinaryOperator::Add => Instruction::BinaryAdd,
//             BinaryOperator::Subtract => Instruction::BinarySubtract,
//             BinaryOperator::Multiply => Instruction::BinaryMultiply,
//             BinaryOperator::Divide => Instruction::BinaryDivide,
//             BinaryOperator::Reminder => Instruction::BinaryReminder,
//             BinaryOperator::Power => Instruction::BinaryPower,
//             BinaryOperator::LessThan => Instruction::BinaryLessThan,
//             BinaryOperator::LessThanEqual => Instruction::BinaryLessThanEqual,
//             BinaryOperator::GreaterThan => Instruction::BinaryGreaterThan,
//             BinaryOperator::GreaterThanEqual => Instruction::BinaryGreaterThanEqual,
//             BinaryOperator::Equal => Instruction::BinaryEqual,
//             BinaryOperator::NotEqual => Instruction::BinaryNotEqual,
//             BinaryOperator::LogicalAnd => Instruction::BinaryLogicalAnd,
//             BinaryOperator::LogicalOr => Instruction::BinaryLogicalOr,
//             BinaryOperator::LogicalXor => Instruction::BinaryLogicalXor,
//         };
//         compiler.emit(instruction);
//         Ok(())
//     }
// }

impl ASTNode<'_> for BinaryExpression {
    fn build(rule: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let expression = match rule.as_rule() {
            Rule::binary_expression => rule.into_inner(),
            _ => unreachable!(),
        };
        let result = PREC_CLIMBER.climb(
            expression,
            |pair: Pair<Rule>| -> Expression {
                match pair.as_rule() {
                    Rule::value => Value::build(pair).unwrap().into(),
                    Rule::expression => Expression::build(pair).unwrap(),
                    Rule::identifier => Identifier::build(pair).unwrap().into(),
                    _ => unreachable!("{}", pair),
                }
            },
            |left: Expression, op: Pair<Rule>, right: Expression| -> Expression {
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
                Expression::Binary(BinaryExpression {
                    left: Box::from(left),
                    right: Box::from(right),
                    operator,
                })
            },
        );
        if let Expression::Binary(binary) = result {
            Ok(binary)
        } else {
            unreachable!()
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

    use crate::parser::{ASTNode, AlloyParser, ParserError, Rule};

    use super::BinaryExpression;

    fn parse(input: &str) -> Result<(), ParserError> {
        let mut tokens = AlloyParser::parse(Rule::binary_expression, input).unwrap();
        BinaryExpression::build(tokens.next().unwrap())?;
        Ok(())
    }

    #[test]
    fn build_expression_test() -> Result<(), ParserError> {
        parse("1 + 1")?;
        parse("1 + 2 * 3")?;
        parse("(1 + 2) * 3")?;
        parse("1 - 1")?;
        parse("1 + 2 + 3")?;
        parse("(1 + 2) / 3")?;
        Ok(())
    }
}
