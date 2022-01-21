use alloy_macros::expr;

fn main() {
    expr!(3 + 5);
    expr!(3 - 5);
    expr!(3 * 5);
    expr!(3 / 5);
    expr!(3 % 5);
}
