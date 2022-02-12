use alloy_macros::AST;

#[allow(dead_code)]
enum Op {
    Plus,
    Minus,
}

#[derive(AST)]
pub enum ExprCST<'a> {
    Integer(IntegerCST),
    Binary(BinaryCST<'a>),
}

#[derive(AST)]
pub struct IntegerCST(i64);

#[derive(AST)]
pub struct TupleCST(i64, f64);

#[allow(dead_code)]
#[derive(AST)]
pub struct BinaryCST<'a> {
    lhs: Box<ExprCST<'a>>,
    #[space]
    lw: &'a str,
    op: Op,
    #[space]
    rw: &'a str,
    rhs: Box<ExprCST<'a>>,
}

fn main() {
    let num = IntegerCST(64);
    let binary = BinaryCST {
        lhs: Box::from(ExprCST::Integer(IntegerCST(1))),
        lw: " ",
        op: Op::Plus,
        rw: " ",
        rhs: Box::from(ExprCST::Integer(IntegerCST(2))),
    };
    let _ = Expr::Integer(num.into());
    let _ = Expr::Binary(binary.into());
}
