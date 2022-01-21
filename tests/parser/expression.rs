use alloy_macros;

#[test]
fn test_binary_expressions() {
    alloy_macros::assert_expr!(3 + 5);
    alloy_macros::assert_expr!(1 - 1);
    alloy_macros::assert_expr!(1 + 1);
    alloy_macros::assert_expr!(1 + 2 * 3);
    alloy_macros::assert_expr!(1 + 2 + 3);
    alloy_macros::assert_expr!(3 + 5 * 4);
    alloy_macros::assert_expr!(3 + 5 * 4 - 1);
    alloy_macros::assert_expr!(3 + 5 * 4 - 1 - 5);
    alloy_macros::assert_expr!(3 + 5 * 4 - 1 - 5 / 9);
}

#[test]
fn test_unary_expressions() {
    alloy_macros::assert_expr!(-(1));
    alloy_macros::assert_expr!(!1);
    alloy_macros::assert_expr!(!true);
}

#[test]
fn test_parenthesized_expressions() {
    alloy_macros::assert_expr!((1 + 2) * 3);
    alloy_macros::assert_expr!((1 + 2) / 3);
    alloy_macros::assert_expr!((1 + 2) * 3);
    alloy_macros::assert_expr!((1 + 2) * (12 + 12));
    alloy_macros::assert_expr!(((1 + 2) * (12 + 12)) / (12 - 12) * 12);
}
