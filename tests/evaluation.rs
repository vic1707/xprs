//! Checked evaluation and fixed/runtime variable bindings.
use crate::common::assert_f64_eq;
use rstest::rstest;
use xprs::Parser;

#[rstest]
#[case::fractional_remainder("x % 1", "x", 2.5, 0.5)]
#[case::uppercase_implicit_multiplication("2X", "X", 3.0, 6.0)]
fn variable_evaluation(
    #[case] input: &str,
    #[case] name: &str,
    #[case] value: f64,
    #[case] expected: f64,
) {
    let tree = Parser::default().parse(input).unwrap();
    let variables = [(name, value)].into();
    assert_f64_eq!(tree.eval(&variables).unwrap(), expected);
    assert_f64_eq!(tree.eval_unchecked(&variables), expected);
    assert_f64_eq!(tree.bind(name).unwrap()(value), expected);
}

#[test]
fn missing_variables() {
    let tree = Parser::default().parse("2 * x + 3y + 4x + 5").unwrap();
    tree.eval(&[("x", 2.0_f64)].into()).unwrap_err();
    tree.eval_no_vars().unwrap_err();
    assert!(tree.bind("x").is_err());
}

#[rstest]
#[case::empty(0, 0.0)]
#[case::single(1, 2.0)]
#[case::stack_limit(9, 18.0)]
#[case::heap_fallback(10, 20.0)]
fn function_argument_boundaries(#[case] arity: usize, #[case] expected: f64) {
    let input = format!("sum({})", vec!["x"; arity].join(","));
    let tree = Parser::default().parse(&input).unwrap();
    let variables = [("x", 2.0_f64)].into();
    if arity > 0 {
        tree.eval(&[].into()).unwrap_err();
    }
    assert_f64_eq!(tree.eval(&variables).unwrap(), expected);
    assert_f64_eq!(tree.eval_unchecked(&variables), expected);
    assert_f64_eq!(
        tree.simplify_for(("x", 2.0_f64)).eval_no_vars().unwrap(),
        expected
    );
}

#[test]
fn bindings_preserve_order_duplicates_and_missing_values() {
    let parser = Parser::default();
    assert_f64_eq!(
        parser.parse("x - y").unwrap().bind2("y", "x").unwrap()(3.0, 2.0),
        -1.0
    );
    assert_f64_eq!(
        parser.parse("x").unwrap().bind2("x", "x").unwrap()(1.0, 2.0),
        2.0
    );
    let runtime = parser
        .parse("x + y")
        .unwrap()
        .bind_n_runtime(&["x", "y"])
        .unwrap();
    runtime(&[1.0_f64]).unwrap_err();
    assert_f64_eq!(runtime(&[1.0_f64, 2.0_f64, 3.0_f64]).unwrap(), 3.0);
    assert_f64_eq!(
        parser
            .parse("a + j")
            .unwrap()
            .bind_n(["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"])
            .unwrap()([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0
        ]),
        11.0
    );
}
