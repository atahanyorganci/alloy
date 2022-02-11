use alloy_macros::AST;

#[derive(AST)]
pub enum ExprCST {}

#[derive(AST)]
pub struct NumCST();

#[derive(AST)]
pub struct BinaryCST {}

fn main() {
    let _ = Num();
    let _: Num = NumCST().into();
    let _ = Binary {};
    let _: Binary = BinaryCST {}.into();
}
