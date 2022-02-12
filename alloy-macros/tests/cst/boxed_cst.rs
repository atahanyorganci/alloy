use alloy::parser::Spanned;
use alloy_macros::AST;

pub enum Op {
    Plus,
    Minus,
}

#[derive(AST)]
pub enum ExprCST {
    Binary(BinaryCST),
    Num(NumCST),
}

#[derive(AST)]
pub struct NumCST(Spanned<i64>);

#[allow(dead_code)]
#[derive(AST)]
pub struct BinaryCST {
    lhs: Spanned<Box<ExprCST>>,
    #[space]
    lw: Spanned<String>,
    op: Spanned<Op>,
    #[space]
    rw: Spanned<String>,
    rhs: Spanned<Box<ExprCST>>,
}

fn main() {
    let _ = Binary {
        lhs: Box::from(Expr::Num(Num(1))),
        op: Op::Plus,
        rhs: Box::from(Expr::Num(Num(2))),
    };
}
