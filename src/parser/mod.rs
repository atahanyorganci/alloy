use self::value::Value;
use core::fmt;

pub mod value;

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

pub trait Expression: fmt::Display {
    fn eval(&self) -> Value;
}
