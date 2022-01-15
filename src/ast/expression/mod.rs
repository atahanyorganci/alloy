use std::fmt;

use pest::iterators::Pair;

use crate::parser::{ASTNode, ParserError, Rule};

use self::{binary::BinaryExpression, identifier::Identifier, unary::UnaryExpression};

use super::value::Value;

pub mod binary;
pub mod identifier;
pub mod unary;

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Identifier(Identifier),
}

impl From<Value> for Expression {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

impl From<BinaryExpression> for Expression {
    fn from(binary: BinaryExpression) -> Self {
        Self::Binary(binary)
    }
}

impl From<UnaryExpression> for Expression {
    fn from(unary: UnaryExpression) -> Self {
        Self::Unary(unary)
    }
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Self::Identifier(identifier)
    }
}

impl ASTNode<'_> for Expression {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::expression);
        let inner_pair = pair.into_inner().next().unwrap();
        let expression: Expression = match inner_pair.as_rule() {
            Rule::binary_expression => BinaryExpression::build(inner_pair)?.into(),
            Rule::unprecedent_unary_expression | Rule::precedent_unary_expression => {
                UnaryExpression::build(inner_pair)?.into()
            }
            Rule::identifier => Identifier::build(inner_pair)?.into(),
            Rule::value => Value::build(inner_pair)?.into(),
            _ => unreachable!(),
        };
        Ok(expression)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Value(value) => write!(f, "{}", value),
            Expression::Binary(binary) => write!(f, "{}", binary),
            Expression::Unary(unary) => write!(f, "{}", unary),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
        }
    }
}
