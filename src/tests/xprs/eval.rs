/* Built-in imports */
use std::collections::HashMap;
/* Crate imports */
use crate::Parser;

const ERROR_MARGIN: f64 = f64::EPSILON;

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
fn get_valid_test_cases(
) -> [(&'static str, HashMap<&'static str, f64>, f64); 10] {
    [
        ("2(3)", [].into(), 6.0_f64),
        ("2 + 3 ^ 2 * 3 + 4", [].into(), 33.0_f64),
        ("2^2^(2^2 + 1)", [].into(), 1024.0_f64), // MacOS says it's 4_294_967_296.0_f64
        (
            "2 * x + 3y + 4x + 5",
            [("x", 2.0_f64), ("y", 3.0_f64)].into(),
            26.0_f64,
        ),
        ("sin(-cos(2))", [].into(), 0.404_239_153_852_265_8_f64),
        ("sin(2)^2", [].into(), 0.826_821_810_431_806_f64),
        (
            "12 + 3-1x+3y x",
            [("x", 2.0_f64), ("y", 3.0_f64)].into(),
            31.0_f64,
        ),
        ("2 (3+4) 5", [].into(), 70.0_f64),
        #[cfg(feature = "pemdas")]
        ("6/2(2+1)", [].into(), 9.0_f64), // is "6/2*(2+1)"
        #[cfg(feature = "pejmdas")]
        ("6/2(2+1)", [].into(), 1.0_f64), // is "6/(2*(2+1))"
        #[cfg(feature = "pemdas")]
        ("1/2x", [("x", 2.0_f64)].into(), 1_f64), // is "(1/2)*x"
        #[cfg(feature = "pejmdas")]
        ("1/2x", [("x", 2.0_f64)].into(), 0.25_f64), // is "1/(2*x)"
    ]
}

#[test]
fn test_eval() {
    let parser = Parser::default();

    for (input, vars, expected) in get_valid_test_cases() {
        let xprs = parser.parse(input);
        assert!(xprs.is_ok());
        let result = xprs.unwrap().eval(&vars);
        assert!(
            (result.unwrap() - expected).abs() < ERROR_MARGIN,
            "{}\nExpected: {}, got: {}",
            input,
            expected,
            result.unwrap()
        );
    }
}
