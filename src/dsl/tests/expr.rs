use crate::core::math;
use crate::core::math::Real;
use crate::dsl::Expr;

#[test]
fn test_simple_ops() {
    assert_eval("1.00 / 1.33", 1.00 / 1.33);
    assert_eval("1.0 / 1.5", 1.0 / 1.5);
    assert_eq!(Expr::new("5 6 -").eval(), Err("Too many values remaining"));
}

#[test]
fn test_precedence() {
    assert_eval("1 + 9 * 2", 19.0);
    assert_eval("2 * 9 - 1", 17.0);
}

#[test]
fn test_params() {
    assert_eval("(1 + 9) * 2", 20.0);
    assert_eval("1 + (9 - 2 * (3 / 6))", 9.0);
}

#[test]
fn test_constants() {
    assert_eval("pi / 4", math::PI / 4.0);
}

#[test]
fn test_signs() {
    assert_eval("-10", -10.0);
    assert_eval("8 + -90", -82.0);
    assert_eval("+10", 10.0);
    assert_eval("-cos (pi/4)", -Real::sqrt(2.0) / 2.0);
}

#[test]
fn test_trig_functions() {
    assert_eval("cos 0", 1.0);
    assert_eval("cos (pi / 4)", Real::sqrt(2.0) / 2.0);
}

fn assert_eval(tokens: &str, expected: Real) {
    let mut expr = Expr::new(tokens);
    assert_eq!(expr.eval().unwrap(), expected);
}
