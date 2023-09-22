/* Crate imports */
use crate::Parser;

const IMPLICIT_MULTIPLICATIONS: [(&str, &str); 8] = [
    ("2(3)", "2*(3)"),
    ("2(3+4)", "2*(3+4)"),
    ("2 (3+4)5", "2*(3+4)*5"),
    ("2 (3+4) 5", "2*(3+4)*5"),
    ("6/2(2+1)", "6/2*(2+1)"),
    ("2(3+4)(5+6)", "2*(3+4)*(5+6)"),
    ("2x + 3 x y", "2*x + 3*x*y"),
    ("12 + 3-1x+3y x", "12 + 3-1*x+3*y*x"),
];

#[test]
fn test_implicit_multiplication() {
    for &(implicit, explicit) in &IMPLICIT_MULTIPLICATIONS {
        let res_implicit = Parser::new(implicit).parse();
        assert!(
            res_implicit.is_ok(),
            "\nExplicit multiplication failed for {implicit}"
        );
        let res_explicit = Parser::new(explicit).parse();
        assert!(
            res_explicit.is_ok(),
            "\nExplicit multiplication failed for {explicit}"
        );
        assert_eq!(
            res_implicit, res_explicit,
            "\nImplicit multiplication failed for {implicit}"
        );
    }
}

const INVALID_IMPLICIT_MULTIPLICATIONS: [&str; 1] = ["2 5"];
#[test]
fn test_invalid_implicit_multiplication() {
    for &implicit in &INVALID_IMPLICIT_MULTIPLICATIONS {
        let res = Parser::new(implicit).parse();
        assert!(
            res.is_err(),
            "\nImplicit multiplication should fail for {implicit}",
        );
    }
}
