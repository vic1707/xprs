/* Crate imports */
use crate::Parser;

#[cfg(feature = "pemdas")]
const IMPLICIT_MULTIPLICATIONS: [(&str, &str); 13] = [
    ("2(3)", "2*(3)"),
    ("2(3+4)", "2*(3+4)"),
    ("2 (3+4)5", "2*(3+4)*5"),
    ("2 (3+4) 5", "2*(3+4)*5"),
    ("2(3+4)(5+6)", "2*(3+4)*(5+6)"),
    ("(2*3)4", "(2*3)*4"),
    ("(x-x)2", "(x-x)*2"),
    ("x 2", "x*2"),
    ("-x 2", "-x*2"),
    ("2x + 3 x y", "2*x + 3*x*y"),
    ("12 + 3-1x+3y x", "12 + 3-1*x+3*y*x"),
    // These ones are different for the `pejmdas` feature
    ("6/2(2+1)", "6/2*(2+1)"),
    ("1/2x", "(1/2)*x"),
];

#[cfg(feature = "pejmdas")]
const IMPLICIT_MULTIPLICATIONS: [(&str, &str); 13] = [
    ("2(3)", "2*(3)"),
    ("2(3+4)", "2*(3+4)"),
    ("2 (3+4)5", "2*(3+4)*5"),
    ("2 (3+4) 5", "2*(3+4)*5"),
    ("2(3+4)(5+6)", "2*(3+4)*(5+6)"),
    ("(2*3)4", "(2*3)*4"),
    ("(x-x)2", "(x-x)*2"),
    ("x 2", "x*2"),
    ("-x 2", "-x*2"),
    ("2x + 3 x y", "2*x + 3*x*y"),
    ("12 + 3-1x+3y x", "12 + 3-1*x+3*y*x"),
    // These ones are different for the `pemdas` feature
    ("6/2(2+1)", "6/(2*(2+1))"),
    ("1/2x", "1/(2*x)"),
];

#[test]
fn implicit_multiplication() {
    let parser = Parser::default();
    for &(implicit, explicit) in &IMPLICIT_MULTIPLICATIONS {
        let res_implicit = parser.parse(implicit);
        assert!(
            res_implicit.is_ok(),
            "Explicit multiplication failed for `{implicit}`"
        );
        let res_explicit = parser.parse(explicit);
        assert!(
            res_explicit.is_ok(),
            "Explicit multiplication failed for `{explicit}`"
        );
        assert_eq!(
            res_implicit, res_explicit,
            "\nImplicit multiplication failed for `{implicit}`"
        );
    }
}

const INVALID_IMPLICIT_MULTIPLICATIONS: [&str; 10] = [
    "2 5",
    "2 * 3 4",
    "2 + 3 4",
    "2 / 3 4",
    "2 % 3 4",
    "2 ^ 3 4",
    "-2 3",
    "sin(2 3)",
    "sum(1, 2 3)",
    "2x 3 4",
];
#[test]
fn invalid_implicit_multiplication() {
    let parser = Parser::default();
    for &implicit in &INVALID_IMPLICIT_MULTIPLICATIONS {
        let res = parser.parse(implicit);
        assert!(
            res.is_err(),
            "\nImplicit multiplication should fail for {implicit}",
        );
    }
}
