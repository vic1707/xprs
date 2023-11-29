/* Built-in imports */
use std::collections::HashMap;
/* Crate imports */
use super::super::macros::assert_f64_eq;
use crate::Parser;

// shitty type because of clippy and default numeric fallback
// https://github.com/rust-lang/rust-clippy/issues/11535
type InputVarsResult = (&'static str, &'static [(&'static str, f64)], f64);
/// 2(3)
/// 2 + 3 ^ 2 * 3 + 4
/// 2^2^(2^2 + 1)
/// 2 * x + 3y + 4x + 5
/// sin(-cos(2))
/// sin(2)^2
/// 12 + 3-1x+3y x
/// 2 (3+4) 5
/// PEMDAS vs PEJMDAS
/// 6/2(2+1)
/// 1/2x
const VALID: [InputVarsResult; 10] = [
    ("2(3)", &[], 6.0),
    ("2 + 3 ^ 2 * 3 + 4", &[], 33.0),
    ("2^2^(2^2 + 1)", &[], 1024.0),
    ("2 * x + 3y + 4x + 5", &[("x", 2.0), ("y", 3.0)], 26.0),
    ("sin(-cos(2))", &[], 0.404_239_153_852_265_8),
    ("sin(2)^2", &[], 0.826_821_810_431_806),
    ("12 + 3-1x+3y x", &[("x", 2.0), ("y", 3.0)], 31.0),
    ("2 (3+4) 5", &[], 70.0),
    #[cfg(feature = "pemdas")]
    ("6/2(2+1)", &[], 9.0), // is "6/2*(2+1)"
    #[cfg(feature = "pejmdas")]
    ("6/2(2+1)", &[], 1.0), // is "6/(2*(2+1))"
    #[cfg(feature = "pemdas")]
    ("1/2x", &[("x", 2.0)], 1.0), // is "(1/2)*x"
    #[cfg(feature = "pejmdas")]
    ("1/2x", &[("x", 2.0)], 0.25), // is "1/(2*x)"
];

#[test]
fn test_valid_eval() {
    let parser = Parser::default();

    for (input, vars, expected) in VALID {
        let var_map: HashMap<&str, f64> = vars.iter().copied().collect();
        let xprs = parser.parse(input).unwrap();
        let result = xprs.eval(&var_map).unwrap();
        assert_f64_eq!(
            result,
            expected,
            "{input}\nExpected: {expected}, got: {result}"
        );
    }
}

#[test]
fn test_invalid_eval() {
    // this var needs to be declared here because of clippy and default numeric fallback
    // https://github.com/rust-lang/rust-clippy/issues/11535
    const VARS: [(&str, f64); 1] = [("x", 2.0)];
    let parser = Parser::default();

    let xprs = parser.parse("2 * x + 3y + 4x + 5").unwrap();
    let result = xprs.eval(&VARS.into());
    assert!(
        result.is_err(),
        "Should have failed because `y` is not provided"
    );
}
