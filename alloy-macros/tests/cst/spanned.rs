use alloy::parser::Spanned;
use alloy_macros::AST;

#[derive(AST)]
pub struct NodeCST {
    #[space]
    s: String,
    lhs: Spanned<i64>,
    rhs: Spanned<i64>,
}

fn main() {
    let _ = Node { lhs: 1, rhs: 2 };
}
