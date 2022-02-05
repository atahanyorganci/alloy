use alloy::parser::Spanned;
use alloy_macros::AST;

pub enum Op {
    Plus,
    Minus,
}

#[derive(AST)]
pub enum ExprCST<'a> {
    Binary(BinaryCST<'a>),
    Num(NumCST),
}

#[derive(AST)]
pub struct NumCST(Spanned<i64>);

#[derive(AST)]
pub struct BinaryCST<'a> {
    lhs: Spanned<Box<ExprCST<'a>>>,
    #[space]
    lw: Spanned<&'a str>,
    op: Spanned<Op>,
    #[space]
    rw: Spanned<&'a str>,
    rhs: Spanned<Box<ExprCST<'a>>>,
}

fn main() {
    let _ = Binary {
        lhs: Box::from(Expr::Num(1)),
        op: Op::Plus,
        rhs: Box::from(Expr::Num(2)),
    };
}
