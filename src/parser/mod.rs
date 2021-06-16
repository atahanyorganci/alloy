use self::value::Value;
use core::fmt;

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
}
