use alloy::parser::Spanned;
use alloy_macros::AST;

#[derive(AST)]
pub enum ExprCST {
    Num(NumCST),
    Binary(BinaryCST),
    Float(Spanned<f64>),
}

#[derive(AST)]
pub struct NumCST();

#[derive(AST)]
pub struct BinaryCST {}

fn main() {
    let num = Num();
    let binary = Binary {};
    let _ = Expr::Num(num);
    let _ = Expr::Binary(binary);
    let _ = Expr::Float(4.0);
}
