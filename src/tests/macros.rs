macro_rules! assert_f64_eq {
    ($left:expr, $right:expr $(, $($arg:tt)+)?) => {{
        let (left, right): (f64, f64) = ($left, $right);
        // powf and other transcendental functions have platform-dependent precision.
        assert!(
            left.is_finite() && right.is_finite()
                && (left - right).abs()
                    <= 1e-12_f64 * left.abs().max(right.abs()).max(1.0)
            $(, $($arg)+)?
        );
    }};
}

pub(crate) use assert_f64_eq;

#[test]
#[should_panic(expected = "assertion failed")]
fn rejects_different_floats() {
    assert_f64_eq!(1.0, 2.0);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn rejects_finite_and_infinite() {
    assert_f64_eq!(1.0, f64::INFINITY);
}
