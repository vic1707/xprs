//! Function argument formatting, independent of algebraic rewrite tests.
use rstest::rstest;
use xprs::Parser;

#[rstest]
#[case::zero_arguments("sum()", "sum()", "-0")]
#[case::single_argument("sin(x)", "sin(x)", "sin(x)")]
#[case::nested_arguments(
    "max(x,min(y,z))",
    "max(x, min(y, z))",
    "max(x, min(y, z))"
)]
fn function_arguments(
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
