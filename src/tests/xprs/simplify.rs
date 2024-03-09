/* Crate imports */
use crate::Parser;

#[test]
fn test_simplify() {
    // these vars need to be declared here because of clippy and default numeric fallback
    // https://github.com/rust-lang/rust-clippy/issues/11535
    const X_VAR: (&str, f64) = ("x", 2.0);
    const Y_VAR: (&str, f64) = ("y", 3.0);
    const UNKNOWN_VAR: (&str, f64) = ("unknown", 4.0);
    let parser = Parser::default();
    let mut xprs = parser.parse("2x + 3y + 4x + 5z").unwrap();
    // simplify for x
    xprs = xprs.simplify_for(X_VAR);
    assert_eq!(xprs, parser.parse("4 + 3y + 8 + 5z").unwrap());
    // simplify for y
    xprs.simplify_for_in_place(Y_VAR);
    assert_eq!(xprs, parser.parse("21 + 5z").unwrap());
    // try simplifying for an unknown variable
    xprs.simplify_for_in_place(UNKNOWN_VAR);
    assert_eq!(xprs, parser.parse("21 + 5z").unwrap());
}
