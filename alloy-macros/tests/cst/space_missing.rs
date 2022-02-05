use alloy::parser::Spanned;
use alloy_macros::AST;

#[derive(AST)]
pub struct SomeCST {
    #[space]
    int: i64,
    float: f64,
    s: String,
}
#[derive(AST)]
pub struct OtherCST(#[space] i64, f64);

fn main() {
    let _ = Some {
        s: "Hello World!".to_string(),
    };
    let _ = Other(6.0);
}
