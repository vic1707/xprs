/* Crate imports */
extern crate alloc;
use super::super::macros::assert_f64_eq;
use crate::{Context, Function, Parser, element::Element, xprs_fn};
use alloc::sync::Arc;
use core::{
    panic::AssertUnwindSafe,
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
};
use std::panic;

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
fn retains_unfolded_allocations() {
    let allocations = |root: &Element<'_>| match *root {
        Element::BinOp(ref node) => {
            (ptr::from_ref(node.as_ref()).cast::<()>(), ptr::null())
        },
        Element::UnOp(ref node) => {
            (ptr::from_ref(node.as_ref()).cast::<()>(), ptr::null())
        },
        Element::Function(ref node) => (
            ptr::from_ref(node.as_ref()).cast::<()>(),
            node.args.as_ptr().cast::<()>(),
        ),
        Element::Number(_) | Element::Variable(_) => (ptr::null(), ptr::null()),
    };
    for input in ["x + y", "-x", "sum(x, y)"] {
        let mut tree = Parser::default().parse(input).unwrap();
        let before = allocations(&tree.root);
        tree.simplify_for_in_place(("unknown", 0.0_f64));
        assert_eq!(allocations(&tree.root), before, "{input}");
    }
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

#[test]
fn literal_folding_preserves_float_behavior() {
    for (input, expected) in [
        ("-1 / 0", f64::NEG_INFINITY),
        ("1 / -0", f64::NEG_INFINITY),
        ("0 ^ -1", f64::INFINITY),
        ("5.5 % 1", 0.5_f64),
        ("0 * (0 / 0)", f64::NAN),
        ("-0 * 1", -0.0_f64),
        ("(0 / 0) ^ 0", 1.0_f64),
    ] {
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
