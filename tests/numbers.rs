//! Scientific-notation grammar, scaling, and feature-independent errors.
use crate::common::assert_f64_eq;
use core::f64::consts::E;
use rstest::rstest;
use xprs::{ErrorKind, Parser};

#[rstest]
#[case::integer("1", 1.0)]
#[case::decimal("1.2", 1.2)]
#[case::positive_exponent("1.2e3", 1.2e3)]
#[case::negative_exponent("1.2e-3", 1.2e-3)]
#[case::explicit_plus("1.2e+3", 1.2e3)]
#[case::integer_separators("1_000", 1000.0)]
#[case::decimal_separators("1_234.5", 1234.5)]
#[case::exponent_separators("1_000e-3", 1.0)]
#[case::euler_constant("1e", E)]
#[case::implicit_euler_product("1.2e", 1.2 * E)]
#[case::euler_addition("1.2e+ 1", 1.2_f64.mul_add(E, 1.0))]
#[case::euler_subtraction("1.2e- 1", 1.2_f64.mul_add(E, -1.0))]
#[case::fractional_exponent("1e1.5", 10.0_f64.powf(1.5))]
#[case::uppercase_exponent("2.5E+1.5", 2.5_f64 * 10.0_f64.powf(1.5))]
#[case::negative_fraction("1e-.5", 10.0_f64.powf(-0.5))]
#[case::fractional_separators("1_000e-3.5", 1_000.0_f64 * 10.0_f64.powf(-3.5))]
#[case::zero_coefficient("0e99999.5", 0.0)]
#[case::zero_fraction("1e.0", 1.0)]
#[case::nested_call("sum(1e1.5, 2)", 10.0_f64.powf(1.5) + 2.0)]
#[case::large_finite("1e308.25", 10.0_f64.powf(0.25) * 1e308)]
fn scientific_notation(#[case] input: &str, #[case] expected: f64) {
    assert_f64_eq!(
        Parser::default()
            .parse(input)
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        expected
    );
}

#[rstest]
#[case::large_coefficient(format!("1{}e-400.5", "0".repeat(400)), 10.0_f64.powf(-0.5))]
#[case::small_coefficient(format!("0.{}1e400.5", "0".repeat(399)), 10.0_f64.sqrt())]
fn cancelling_decimal_scales(#[case] input: String, #[case] expected: f64) {
    assert_f64_eq!(
        Parser::default()
            .parse(&input)
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        expected
    );
}

#[rstest]
#[case::integral_fraction("2.55e2.0", 255.0_f64.to_bits())]
#[case::subnormal("9e-324.5", 1)]
fn exact_conversion(#[case] input: &str, #[case] expected_bits: u64) {
    assert_eq!(
        Parser::default()
            .parse(input)
            .unwrap()
            .eval_no_vars()
            .unwrap()
            .to_bits(),
        expected_bits
    );
}

#[rstest]
#[case::integer_exponent("1e2e3", 400.0)]
#[case::fractional_exponent("1e1.5e3", 4.0_f64 * 10.0_f64.powf(1.5))]
fn exponent_and_identifier_boundaries(
    #[case] input: &str,
    #[case] expected: f64,
) {
    let tree = Parser::default().parse(input).unwrap();
    assert_eq!(tree.vars, ["e3"].into());
    assert_f64_eq!(tree.eval(&[("e3", 4.0_f64)].into()).unwrap(), expected);
}

#[test]
fn malformed_exponent_span() {
    let input = "2 + 1e1..5";
    let error = Parser::default().parse(input).unwrap_err();
    assert_eq!(
        error.kind(),
        &ErrorKind::MalformedNumber("1e1..5".to_owned())
    );
    assert_eq!(error.to_string(), "Malformed number: `1e1..5`");
    assert_eq!(error.input(), input);
    assert_eq!(error.span(), 4..10);
}

#[cfg(feature = "miette")]
#[test]
fn miette_preserves_native_diagnostics() {
    use miette::Diagnostic as _;
    let error = Parser::default().parse("2 + 1e1..5").unwrap_err();
    let mut labels = error.labels().unwrap();
    let label = labels.next().unwrap();
    assert_eq!(label.offset(), error.span().start);
    assert_eq!(label.len(), error.span().len());
    assert!(labels.next().is_none());
    assert!(error.help().is_some());
    assert!(error.source_code().is_some());
}
