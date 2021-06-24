use std::fmt;

use pest::iterators::Pair;

use crate::compiler::Compile;

use self::value::Value;

pub mod expression;
pub mod statement;
pub mod value;

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

pub trait Expression: ASTNode {
    fn eval(&self) -> Value;
}

pub trait Statement: ASTNode {
    fn eval(&self);
}

pub trait ASTNode: fmt::Debug + fmt::Display + Compile {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized;
}
