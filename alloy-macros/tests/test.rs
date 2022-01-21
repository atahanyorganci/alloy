#[test]
fn builtin_func_test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/expr.rs");
}
