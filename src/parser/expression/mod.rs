use pest::iterators::Pair;

use self::binary::build_binary_expression;

use super::{Expression, Rule};

pub mod binary;

pub fn build_expression(pair: Pair<Rule>) -> Option<Box<dyn Expression>> {
    match pair.as_rule() {
        Rule::expression => Some(build_binary_expression(pair)),
        _ => None,
    }
}
