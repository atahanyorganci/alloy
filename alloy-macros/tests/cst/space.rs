use alloy_macros::AST;

#[derive(AST)]
pub struct NodeCST {
    #[space]
    int: i64,
    #[space]
    float: f64,
    s: String,
}

fn main() {
    let _ = Node {
        s: "Hello World!".to_string(),
    };
}
