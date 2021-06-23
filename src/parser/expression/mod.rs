use pest::iterators::Pair;

use crate::parser::value::Value;

use self::{
    binary::build_binary_expression, identifier::IdentifierExpression, unary::UnaryExpression,
};

use super::{ASTNode, Expression, Rule};

pub mod binary;
pub mod identifier;
pub mod unary;

pub fn build_expression(pair: Pair<Rule>) -> Option<Box<dyn Expression>> {
    let expression = match pair.as_rule() {
        Rule::expression => pair.into_inner().next().unwrap(),
        _ => return None,
    };
    let result = match expression.as_rule() {
        Rule::expression => panic!("Should this even be possible?"),
        Rule::binary_expression => Some(build_binary_expression(expression)).unwrap(),
        Rule::unprecedent_unary_expression | Rule::precedent_unary_expression => {
            UnaryExpression::build(expression).unwrap()
        }
        Rule::identifier => IdentifierExpression::build(expression).unwrap(),
        Rule::value => Value::build(expression).unwrap(),
        _ => panic!("Should be unreachable {}", expression),
    };
    Some(result)
}
