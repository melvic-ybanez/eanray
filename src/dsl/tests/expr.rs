use crate::core::math::Real;
use crate::dsl::expr::eval;

#[test]
fn test_simple_ops() {
    assert_eval("1.00 / 1.33", 1.00 / 1.33);
    assert_eval("1.0 / 1.5", 1.0 / 1.5);
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

fn assert_eval(expr: &str, expected: Real) {
    assert_eq!(eval(expr).unwrap(), expected);
}