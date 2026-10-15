//! Context macro syntax, captures, and inferred arity.
extern crate alloc;
use crate::common::assert_f64_eq;
use alloc::sync::Arc;
use rstest::{fixture, rstest};
use xprs::{Context, ErrorKind, Parser, Symbol, Xprs, context, xprs_fn};

#[test]
fn mixed_entries_and_captures() {
    let offset = Arc::new(3.0_f64);
    let named_offset = Arc::clone(&offset);
    let ctx = context! {
        x: 2_i32,
        double(x): 2.0_f64 * x,
        add(x, y): x + y,
        shift(x): x + *offset,
        named_shift(x): x + *named_offset,
        sum(args: &[f64]): args.iter().sum::<f64>(),
    };
    let parser = Parser::new_with_ctx(ctx);
    assert_eq!(
        parser
            .parse("sum(x, double(2), add(1, 2), shift(1), named_shift(1))")
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        17.0_f64
    );
    parser.parse("double()").unwrap_err();
    parser.parse("add(1)").unwrap_err();
    parser.parse("add(1, 2, 3)").unwrap_err();
}

#[test]
fn standalone_functions_infer_arity_and_capture_once() {
    let offset = Arc::new(3.0_f64);
    let named_offset = Arc::clone(&offset);
    let legacy = xprs_fn!("legacy", dyn move |value: f64| value + *offset, 1);
    let inferred = xprs_fn!(shift(value): value + *named_offset);
    assert_eq!(inferred.nb_args, Some(1));
    let total = xprs_fn!(total(values: &[f64]): values.iter().sum::<f64>());
    assert_eq!(total.nb_args, None);
    let ctx = Context::default()
        .with_fn(legacy)
        .with_fn(inferred)
        .with_fn(total);
    let parser = Parser::new_with_ctx(ctx);
    for _ in 0_u32..2 {
        assert_eq!(
            parser
                .parse("total(legacy(1), shift(2))")
                .unwrap()
                .eval_no_vars()
                .unwrap(),
            9.0_f64
        );
    }
}

#[test]
fn inferred_arity_including_zero_and_more_than_nine() {
    let ctx = context! {
        answer(): 42.0_f64,
        constant(): 5.0_f64,
        ten(first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth):
            first + second + third + fourth + fifth + sixth + seventh + eighth + ninth + tenth,
    };
    let parser = Parser::new_with_ctx(ctx);
    assert_eq!(
        parser
            .parse("answer() + constant() + ten(1,2,3,4,5,6,7,8,9,10)")
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        102.0_f64
    );
    parser.parse("answer(1)").unwrap_err();
    parser.parse("ten(1)").unwrap_err();
}

#[test]
fn empty_trailing_comma_and_single_evaluation() {
    assert_eq!(context! {}, Context::default());
    let mut evaluations = 0_u32;
    let ctx = context! {
        x: { evaluations += 1; 1_i32 },
        "x": { evaluations += 1; 2_i32 },
        "x'": 4_i32,
    };
    assert_eq!(evaluations, 2);
    assert_eq!(ctx.get("x'"), Some(&Symbol::Variable(4.0)));
    assert_eq!(
        Parser::new_with_ctx(ctx)
            .parse("x")
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        2.0_f64
    );
    let without_trailing_comma = context! { x: 3_i32 };
    assert_eq!(
        Parser::new_with_ctx(without_trailing_comma)
            .parse("x")
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        3.0_f64
    );
}

#[fixture]
fn parser_with_context() -> Parser<'static> {
    let mut context = Context::default()
        .with_var("x", 2.0_f64)
        .with_var("phi", 1.618_033_988_749_895_f64)
        .with_fn(xprs_fn!(double(value): 2.0_f64 * value))
        .with_fn(xprs_fn!(add(left, right): left + right));
    context.set_var("y", 1.0_f64);
    let mut parser = Parser::new_with_ctx(context);
    parser
        .ctx_mut()
        .set_fn(xprs_fn!(total(args: &[f64]): args.iter().sum::<f64>()));
    parser
}

#[rstest]
#[case("y", 1.0)]
#[case("2 + phi", 3.618_033_988_749_895)]
#[case("2 + phi * x", 5.236_067_977_499_79)]
#[case("double(2 + phi * x)", 10.472_135_954_999_58)]
#[case("add(2, 3)", 5.0)]
#[case("total(2, 3, 4)", 9.0)]
fn constants_and_functions(
    parser_with_context: Parser<'static>,
    #[case] input: &str,
    #[case] expected: f64,
) {
    assert_f64_eq!(
        parser_with_context
            .parse(input)
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        expected
    );
}

#[test]
fn allowed_variables() {
    let parser = Parser::new_with_ctx(
        Context::default().with_expected_vars(["x", "y"].into()),
    );
    parser.parse("x + y").unwrap();
    let error = parser.parse("x + z").unwrap_err();
    assert!(
        matches!(error.kind(), ErrorKind::VariableNotDeclared(name, available)
        if name == "z" && available.iter().any(|value| value == "x")
            && available.iter().any(|value| value == "y"))
    );
    assert_eq!(error.span(), 0..5);
}

#[test]
fn higher_order_functions() {
    let bound = Xprs::try_from("2x + y").unwrap().bind2("x", "y").unwrap();
    let parser = Parser::new_with_ctx(
        Context::default()
            .with_fn(xprs_fn!(hof(left, right): bound(left, right))),
    );
    assert_f64_eq!(
        parser.parse("hof(2, 3)").unwrap().eval_no_vars().unwrap(),
        7.0
    );
    parser.parse("hof(2)").unwrap_err();
    assert_f64_eq!(
        parser
            .parse("hof(2, 3) + 3 * hof(4, 5)")
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        46.0
    );
    assert_f64_eq!(
        parser
            .parse("hof(x, hof(2, 3))")
            .unwrap()
            .eval(&[("x", 42.0_f64)].into())
            .unwrap(),
        91.0
    );
}
