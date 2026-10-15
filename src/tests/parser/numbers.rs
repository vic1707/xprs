//! Scientific-notation grammar and diagnostic integration checks.
use crate::{Parser, tests::macros::assert_f64_eq};
#[cfg(feature = "miette")]
use miette::Diagnostic as _;

#[test]
fn fractional_exponents() {
    let parser = Parser::default();
    for (input, expected) in [
        ("1e1.5", 10.0_f64.powf(1.5)),
        ("2.5E+1.5", 2.5_f64 * 10.0_f64.powf(1.5)),
        ("1e-.5", 10.0_f64.powf(-0.5)),
        ("1_000e-3.5", 1_000.0_f64 * 10.0_f64.powf(-3.5)),
        ("0e99999.5", 0.0_f64),
        ("1e.0", 1.0_f64),
    ] {
        assert_f64_eq!(
            parser.parse(input).unwrap().eval_no_vars().unwrap(),
            expected,
            "{input}"
        );
    }
    // Our .0 handling must use literal conversion rather than a rounded multiply.
    assert_eq!(
        parser
            .parse("2.55e2.0")
            .unwrap()
            .eval_no_vars()
            .unwrap()
            .to_bits(),
        255.0_f64.to_bits()
    );
}

#[test]
fn fractional_exponents_cancel_large_decimal_scales() {
    let parser = Parser::default();
    let large = format!("1{}e-400.5", "0".repeat(400));
    let small = format!("0.{}1e400.5", "0".repeat(399));
    assert_f64_eq!(
        parser.parse(&large).unwrap().eval_no_vars().unwrap(),
        10.0_f64.powf(-0.5)
    );
    assert_f64_eq!(
        parser.parse(&small).unwrap().eval_no_vars().unwrap(),
        10.0_f64.sqrt()
    );
    assert_eq!(
        parser
            .parse("9e-324.5")
            .unwrap()
            .eval_no_vars()
            .unwrap()
            .to_bits(),
        1
    );
    assert_f64_eq!(
        parser.parse("1e308.25").unwrap().eval_no_vars().unwrap(),
        10.0_f64.powf(0.25) * 1e308
    );
}

#[test]
fn exponent_and_identifier_boundaries() {
    let parser = Parser::default();
    for (input, expected) in [
        ("1e2e3", 400.0_f64),
        ("1e1.5e3", 4.0_f64 * 10.0_f64.powf(1.5)),
    ] {
        let tree = parser.parse(input).unwrap();
        assert_eq!(tree.vars, ["e3"].into(), "{input}");
        assert_f64_eq!(
            tree.eval(&[("e3", 4.0_f64)].into()).unwrap(),
            expected,
            "{input}"
        );
    }
    assert_f64_eq!(
        parser
            .parse("sum(1e1.5, 2)")
            .unwrap()
            .eval_no_vars()
            .unwrap(),
        10.0_f64.powf(1.5) + 2.0
    );
    parser.parse("1e1.5 2").unwrap_err();
}

#[test]
fn malformed_exponent_span() {
    let input = "2 + 1e1..5";
    let error = Parser::default().parse(input).unwrap_err();
    assert_eq!(error.to_string(), "Malformed number: `1e1..5`");
    assert_eq!(
        error.kind(),
        &crate::ErrorKind::MalformedNumber("1e1..5".to_owned())
    );
    assert_eq!(error.input(), input);
    assert_eq!(error.span(), 4..10);
    #[cfg(feature = "miette")]
    {
        let label = error.labels().unwrap().next().unwrap();
        assert_eq!(label.offset(), 4);
        assert_eq!(label.len(), 6);
    }
}
