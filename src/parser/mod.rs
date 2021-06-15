use self::value::Value;

pub mod value;

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

pub trait Expression {
    fn eval(&self) -> Value;
}
