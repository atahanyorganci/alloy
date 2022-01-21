use alloy_macros::expr;

fn main() {
    expr!(3 + 5);
    expr!(3 - 5);
    expr!(3 * 5);
    expr!(3 / 5);
    expr!(3 % 5);
    expr!(1 == 2);
    expr!(1 != 2);
    expr!(1 >= 2);
    expr!(1 > 2);
    expr!(1 <= 2);
    expr!(1 < 2);
    expr!(-3);
    expr!(true);
    expr!(false);
    expr!(4.2);
    expr!((3 + 2));
}
