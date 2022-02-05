use alloy::parser::Spanned;
use alloy_macros::AST;

#[derive(AST)]
pub struct SomeCST<'a> {
    #[space]
    int: i64,
    #[space]
    float: f64,
    s: String,
}

fn main() {
    let _ = Some {
        s: "Hello World!".to_string(),
    };
}
