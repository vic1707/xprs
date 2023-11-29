/* Crate imports */
use super::super::macros::assert_f64_eq;
use crate::{xprs_fn, Context, Parser, Xprs};

#[test]
fn test_higher_order_functions() {
    let xprs_hof = Xprs::try_from("2x + y").unwrap();
    let fn_hof = xprs_hof.bind2("x", "y").unwrap();
    let hof = xprs_fn!("hof", dyn fn_hof, 2);
    let ctx = Context::default().with_fn(hof);
    let parser = Parser::new_with_ctx(ctx);

    let bare_use = parser.parse("hof(2, 3)").unwrap();
    assert_f64_eq!(bare_use.eval_unchecked(&[].into()), 7.0_f64);

    let invalid_use = parser.parse("hof(2)");
    assert!(
        invalid_use.is_err(),
        "Expected error for invalid use of hof"
    );

    let complex_use = parser.parse("hof(2, 3) + 3 * hof(4, 5)").unwrap();
    assert_f64_eq!(complex_use.eval_unchecked(&[].into()), 46.0_f64);

    let nested_var_use = parser.parse("hof(x, hof(2, 3))").unwrap();
    assert_f64_eq!(
        nested_var_use.eval_unchecked(&[("x", 42.0_f64)].into()),
        91.0_f64
    );
}
