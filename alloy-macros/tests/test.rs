#[test]
fn builtin_func_test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/expr.rs");
}

#[test]
fn test_cst_ast() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cst/basic.rs");
    t.pass("tests/cst/space.rs");
    t.compile_fail("tests/cst/space_missing.rs");
    // t.pass("test/cst/complete.rs")
}
