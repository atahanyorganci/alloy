use alloy_macros::AST;

pub enum Op {
    Plus,
    Minus,
}

#[derive(AST)]
pub enum ExprCST {}

#[derive(AST)]
pub struct NumCST();

#[derive(AST)]
pub struct BinaryCST {}

fn main() {
    let _num = Num();
    let _binary = Binary {};
}
