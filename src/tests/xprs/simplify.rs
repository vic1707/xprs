/* Crate imports */
use crate::Parser;

#[test]
fn test_simplify() {
    let parser = Parser::default();
    let mut xprs = parser.parse("2x + 3y + 4x + 5z").unwrap();
    // simplify for x
    xprs = xprs.simplify_for(("x", 2.0_f64));
    assert_eq!(xprs, parser.parse("4 + 3y + 8 + 5z").unwrap());
    // simplify for y
    xprs.simplify_for_inplace(("y", 3.0_f64));
    assert_eq!(xprs, parser.parse("21 + 5z").unwrap());
    // try simplifying for an unknown variable
    xprs.simplify_for_inplace(("unknown", 4.0_f64));
    assert_eq!(xprs, parser.parse("21 + 5z").unwrap());
}
