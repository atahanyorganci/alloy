use std::fmt;

use pest::iterators::Pair;

use self::value::Value;

pub mod expression;
pub mod statement;
pub mod value;

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

pub trait Expression: fmt::Display {
    fn eval(&self) -> Value;
}

pub trait Statement {
    fn eval(&self);
    fn build(pair: Pair<Rule>) -> Box<Self>
    where
        Self: Sized;
}
