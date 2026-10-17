//! Streaming callbacks, eager compatibility, and consuming built-ins.
extern crate alloc;
use crate::common::assert_f64_eq;
use alloc::sync::Arc;
use core::{
    iter,
    panic::AssertUnwindSafe,
    sync::atomic::{AtomicUsize, Ordering},
};
use rstest::rstest;
use std::panic;
use xprs::{Args, Context, EvalError, Function, Parser, context, xprs_fn};

#[test]
fn constructors_and_macros_capture_and_reuse() {
    const TOTAL: Function = xprs_fn!(static total(args: Args<'_>): args.sum());
    const EAGER: Function =
        xprs_fn!(static total(args: &[f64]): args.iter().sum());
    let total = TOTAL;
    assert_eq!(total, total.clone());
    assert_f64_eq!(total.call([Ok(2.0), Ok(3.0)].into_iter()).unwrap(), 5.0);
    assert_ne!(total, EAGER);
    let offset = Arc::new(3.0_f64);
    let shift = Function::new_dyn_stream("shift", move |args| {
        Ok(args.sum::<Result<f64, _>>()? + *offset)
    });
    assert_eq!(shift, shift.clone());
    let product =
        xprs_fn!(product(args: Args<'_>): args.product::<Result<f64, _>>());
    let parser = Parser::new_with_ctx(
        Context::default()
            .with_fn(total)
            .with_fn(shift)
            .with_fn(product),
    );
    let expression = parser
        .parse("total(x,y) + shift(x,y) + product(x,y)")
        .unwrap();
    assert_f64_eq!(
        expression
            .eval(&[("x", 2.0_f64), ("y", 3.0_f64)].into())
            .unwrap(),
        19.0
    );
    assert_f64_eq!(expression.bind2("x", "y").unwrap()(2.0, 3.0), 19.0);
}

#[test]
fn named_static_functions_infer_zero_and_binary_arity() {
    const ANSWER: Function = xprs_fn!(static answer(): 42.0);
    const SUBTRACT: Function =
        xprs_fn!(static subtract(left, right): left - right);
    assert_eq!(ANSWER.nb_args, Some(0));
    assert_eq!(SUBTRACT.nb_args, Some(2));
    assert_f64_eq!(ANSWER.call(iter::empty()).unwrap(), 42.0);
    assert_f64_eq!(SUBTRACT.call([Ok(3.0), Ok(1.0)].into_iter()).unwrap(), 2.0);
}

#[rstest]
#[case::missing(0)]
#[case::extra(2)]
#[case::reported_sin_arity(3)]
fn fixed_arity_is_checked_before_consuming_arguments(
    #[case] actual: usize,
    #[values(false, true)] streaming: bool,
) {
    let function = if streaming {
        let mut stream =
            Function::new_stream("one", |args| args.next().unwrap_or(Ok(0.0)));
        stream.nb_args = Some(1);
        stream
    } else {
        xprs_fn!(static one(value): value)
    };
    let evaluations = AtomicUsize::new(0);
    let args = iter::repeat_n(Ok(0.0_f64), actual).inspect(|_| {
        evaluations.fetch_add(1, Ordering::Relaxed);
    });
    assert_eq!(
        function.call(args),
        Err(EvalError::InvalidArgumentCount {
            function: "one",
            expected: 1,
            actual,
        })
    );
    assert_eq!(evaluations.load(Ordering::Relaxed), 0);
}

#[test]
fn lazy_arguments_skip_work_while_slice_callbacks_remain_eager() {
    let calls = Arc::new(AtomicUsize::new(0));
    let tick_calls = Arc::clone(&calls);
    let eager_calls = Arc::new(AtomicUsize::new(0));
    let eager_counter = Arc::clone(&eager_calls);
    let parser = Parser::new_with_ctx(context! {
        tick(value): { tick_calls.fetch_add(1, Ordering::Relaxed); value },
        head(args: Args<'_>): args.next().unwrap_or(Ok(0.0)),
        eager(first, _second): { eager_counter.fetch_add(1, Ordering::Relaxed); first },
    });
    let lazy = parser.parse("head(x, tick(y))").unwrap();
    let values = [("x", 2.0_f64)].into();
    assert_f64_eq!(lazy.eval(&values).unwrap(), 2.0);
    assert_f64_eq!(lazy.eval_unchecked(&values), 2.0);
    assert_eq!(calls.load(Ordering::Relaxed), 0);
    lazy.eval_no_vars().unwrap_err();
    // Binding remains conservative about the expression's recorded variables.
    assert!(lazy.clone().bind("x").is_err());
    let eager = parser.parse("eager(tick(x), y)").unwrap();
    eager.eval(&values).unwrap_err();
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(eager_calls.load(Ordering::Relaxed), 0);
    let complete = [("x", 2.0_f64), ("y", 3.0_f64)].into();
    assert_f64_eq!(eager.eval(&complete).unwrap(), 2.0);
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    assert_eq!(eager_calls.load(Ordering::Relaxed), 1);
    assert_f64_eq!(lazy.eval(&complete).unwrap(), 2.0);
    assert_eq!(calls.load(Ordering::Relaxed), 2);
}

#[rstest]
#[case("sum", 20.0)]
#[case("mean", 2.0)]
#[case("min", 1.0)]
#[case("max", 3.0)]
fn aggregators_consume_all_arguments(
    #[case] name: &str,
    #[case] expected: f64,
) {
    let input = format!("{name}(x,y,x,y,x,y,x,y,x,y)");
    let tree = Parser::default().parse(&input).unwrap();
    let variables = [("x", 1.0_f64), ("y", 3.0_f64)].into();
    assert_f64_eq!(tree.eval(&variables).unwrap(), expected);
    assert_f64_eq!(tree.eval_unchecked(&variables), expected);
    tree.eval(&[("x", 1.0_f64)].into()).unwrap_err();
    assert_f64_eq!(
        tree.simplify_for_multiple(&[("x", 1.0_f64), ("y", 3.0_f64)])
            .eval_no_vars()
            .unwrap(),
        expected
    );
}

#[rstest]
#[case("sum()", -0.0)]
#[case("mean()", f64::NAN)]
#[case("min()", f64::INFINITY)]
#[case("max()", f64::NEG_INFINITY)]
#[case("sum(-0,-0)", -0.0)]
#[case("sum(0/0,1)", f64::NAN)]
#[case("min(0/0)", f64::INFINITY)]
#[case("max(0/0)", f64::NEG_INFINITY)]
fn aggregators_preserve_float_behavior(
    #[case] input: &str,
    #[case] expected: f64,
) {
    let actual = Parser::default()
        .parse(input)
        .unwrap()
        .eval_no_vars()
        .unwrap();
    if expected.is_nan() {
        assert!(actual.is_nan());
    } else {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[rstest]
#[case::missing_variable(EvalError::MissingVariable("missing".to_owned()))]
#[case::argument_count(EvalError::InvalidArgumentCount { function: "fail", expected: 2, actual: 1 })]
fn callback_errors_survive_folding_and_panic_in_unchecked_evaluation(
    #[case] error: EvalError,
) {
    let expected = error.clone();
    let parser = Parser::new_with_ctx(Context::default().with_fn(
        Function::new_dyn_stream("fail", move |_args| Err(error.clone())),
    ));
    let tree = parser
        .parse("fail(1)")
        .unwrap()
        .simplify_for(("unknown", 0.0_f64));
    assert_eq!(tree.eval_no_vars().unwrap_err(), expected);
    panic::catch_unwind(AssertUnwindSafe(|| tree.eval_no_vars_unchecked()))
        .unwrap_err();
}
