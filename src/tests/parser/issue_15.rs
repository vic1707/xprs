/* Built-in imports */
#![allow(clippy::min_ident_chars)]
use core::f64::consts::E;
/* Crate imports */
use crate::Parser;

const ERROR_MARGIN: f64 = f64::EPSILON;

const VALID_TEST_CASES: [(&str, f64); 10] = [
    ("1", 1.0),
    ("1.2", 1.2),
    ("1.2e3", 1.2e3),
    ("1.2e-3", 1.2e-3),
    ("1.2e+3", 1.2e+3),
    ("1e", 1. * E),
    ("1.2e", 1.2 * E),
    // weird but technically okay
    ("1.2e3e3", 1.2e3 * E * 3.),
    // would be error if wasn't an operation
    ("1.2e+ 1", 1.2 * E + 1.),
    ("1.2e- 1", 1.2 * E - 1.),
];

const ERROR_TEST_CASES: [&str; 2] = ["1.2e+", "1.2e-"];

#[test]
fn parse_number() {
    let parser = Parser::default();
    for (input, expected) in VALID_TEST_CASES {
        let result = parser.parse(input).unwrap().eval_unchecked(&[].into());
        assert!(
            (result - expected).abs() < ERROR_MARGIN,
            "{input}\nExpected: {expected}, got: {result}"
        );
    }

    for input in ERROR_TEST_CASES {
        assert!(
            parser.parse(input).is_err(),
            "Should have errored: '{input}'."
        );
    }
}