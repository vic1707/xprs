/* Crate imports */
extern crate alloc;
use crate::common::assert_f64_eq;
use alloc::sync::Arc;
use core::{
    panic::AssertUnwindSafe,
    sync::atomic::{AtomicUsize, Ordering},
};
use rstest::rstest;
use std::panic;
use xprs::{Context, Function, Parser, xprs_fn};

#[test]
fn simplify() {
    // these vars need to be declared here because of clippy and default numeric fallback
    // https://github.com/rust-lang/rust-clippy/issues/11535
    const X_VAR: (&str, f64) = ("x", 2.0);
    const Y_VAR: (&str, f64) = ("y", 3.0);
    const UNKNOWN_VAR: (&str, f64) = ("unknown", 4.0);
    let parser = Parser::default();
    let mut xprs = parser.parse("2x + 3y + 4x + 5z").unwrap();
    // simplify for x
    xprs = xprs.simplify_for(X_VAR);
    assert_eq!(xprs, parser.parse("4 + 3y + 8 + 5z").unwrap());
    // simplify for y
    xprs.simplify_for_in_place(Y_VAR);
    assert_eq!(xprs, parser.parse("21 + 5z").unwrap());
    // try simplifying for an unknown variable
    xprs.simplify_for_in_place(UNKNOWN_VAR);
    assert_eq!(xprs, parser.parse("21 + 5z").unwrap());
}

#[test]
fn substitution_rebuilds_remaining_variables() {
    let parser = Parser::default();
    let mut cancelled = parser.parse("x * y").unwrap();
    cancelled.simplify_for_in_place(("x", 0.0_f64));
    assert!(cancelled.vars.is_empty());
    assert_f64_eq!(cancelled.bind_n([]).unwrap()([]), 0.0);

    let mut retained = parser.parse("2 * x * y + y").unwrap();
    retained.simplify_for_multiple_in_place(&[("x", 0.0_f64)]);
    assert_eq!(retained.vars, ["y"].into());
    assert_f64_eq!(retained.bind("y").unwrap()(3.0), 3.0);
}

#[test]
fn partial_folding_calls_functions_once() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let count = move |value: f64| {
        counter.fetch_add(1, Ordering::Relaxed);
        value
    };
    let context = Context::default().with_fn(xprs_fn!("count", dyn count, 1));
    let mut tree = Parser::new_with_ctx(context)
        .parse("count(x) + count(x) + y")
        .unwrap();
    tree.simplify_for_in_place(("x", 2.0_f64));
    tree.simplify_for_in_place(("unknown", 0.0_f64));
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    assert_f64_eq!(tree.eval(&[("y", 1.0_f64)].into()).unwrap(), 5.0);
}

#[rstest]
#[case::negative_infinity("-1 / 0", f64::NEG_INFINITY)]
#[case::negative_zero_divisor("1 / -0", f64::NEG_INFINITY)]
#[case::negative_power("0 ^ -1", f64::INFINITY)]
#[case::fractional_remainder("5.5 % 1", 0.5)]
#[case::negative_remainder_zero("-5 % 1", -0.0)]
#[case::nan_product("0 * (0 / 0)", f64::NAN)]
#[case::signed_zero("-0 * 1", -0.0)]
#[case::nan_power_zero("(0 / 0) ^ 0", 1.0)]
fn literal_folding_preserves_float_behavior(
    #[case] input: &str,
    #[case] expected: f64,
) {
    let tree = Parser::default()
        .parse(input)
        .unwrap()
        .simplify_for(("unknown", 0.0_f64));
    let actual = tree.eval_no_vars().unwrap();
    if expected.is_nan() {
        assert!(actual.is_nan(), "{input}");
    } else if expected.is_normal() {
        assert_f64_eq!(actual, expected, "{input}");
    } else {
        assert_eq!(actual.to_bits(), expected.to_bits(), "{input}");
    }
}

#[test]
fn simplification_is_panic_safe() {
    #[expect(
        clippy::panic,
        reason = "exercise unwinding during constant folding"
    )]
    let context = Context::default().with_fn(Function::new_static(
        "fail",
        |_args| panic!("intentional folding failure"),
        Some(1),
    ));
    let parser = Parser::new_with_ctx(context);
    for multiple in [false, true] {
        let mut tree = parser.parse("fail(x) + y").unwrap();
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            if multiple {
                tree.simplify_for_multiple_in_place(&[("x", 2.0_f64)]);
            } else {
                tree.simplify_for_in_place(("x", 2.0_f64));
            }
        }));
        result.unwrap_err();
        // The root is still initialized and can be inspected and dropped safely.
        assert_f64_eq!(tree.eval_no_vars().unwrap(), 0.0);
    }
}

#[rstest]
#[case("(-x) + x", "0")]
#[case("x + (-x)", "0")]
#[case("x - (-y)", "(x + y)")]
#[case("(-x) * (-y)", "(x * y)")]
#[case("(-x) / (-y)", "(x / y)")]
#[case("x + x!", "(x + (!x))")]
#[case("x - y!", "(x - (!y))")]
#[case("(-x) * y!", "((-x) * (!y))")]
#[case("x! / (-y)", "((!x) / (-y))")]
#[case("x ^ 0", "1")]
#[case("sin(0)!", "1")]
#[case("sum(1, 2)!", "6")]
#[case("sin(0)! + x", "(1 + x)")]
#[case("x + 0 + sum(1, 2 * 3)!", "(x + 5040)")]
#[case("-(-x)", "x")]
#[case("0 - (-x)", "x")]
#[case("x - (-x)", "(x + x)")]
#[case("x * 1", "x")]
#[case("x / 1", "x")]
#[case("x ^ 1", "x")]
#[case("-0 + x", "x")]
#[case("0 * x", "0")]
#[case("x * 0", "0")]
#[case("0 / x", "0")]
#[case("0 % x", "0")]
#[case("0 / 0", "NaN")]
#[case("0 % 0", "NaN")]
#[case("0 ^ 0", "1")]
#[case("x - x", "0")]
#[case("(a + b) - (b + a)", "0")]
fn algebraic_simplifications(#[case] input: &str, #[case] expected: &str) {
    let tree = Parser::default()
        .parse(input)
        .unwrap()
        .simplify_for(("unknown", 0.0_f64));
    assert_eq!(tree.to_string(), expected);
}

#[rstest]
#[case::constant_expression("2 + 3 * 4", "(2 + (3 * 4))", "14")]
#[case::factorial("sum(1, 2)!", "(!sum(1, 2))", "6")]
#[case::eliminated_variable("x - x", "(x - x)", "0")]
#[case::zero_arguments("sum()", "sum()", "-0")]
#[case::single_argument("sin(x)", "sin(x)", "sin(x)")]
#[case::nested_arguments(
    "max(x, min(y, z))",
    "max(x, min(y, z))",
    "max(x, min(y, z))"
)]
fn parse_time_folding_and_display(
    #[case] input: &str,
    #[case] unoptimized: &str,
    #[case] optimized: &str,
) {
    let expected = if cfg!(feature = "compile-time-optimizations") {
        optimized
    } else {
        unoptimized
    };
    assert_eq!(
        Parser::default().parse(input).unwrap().to_string(),
        expected
    );
}
