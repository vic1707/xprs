/* Crate imports */
use crate::Parser;

#[test]
fn parse_number() {
    const INPUT: &str = "x^4";
    let parser = Parser::default();
    let result = parser.parse(INPUT);
    assert!(result.is_ok(), "Should have parsed: '{INPUT}'.");
    let xprs = result.unwrap();
    let expected_vars = ["x"].into();
    assert_eq!(
        xprs.vars, expected_vars,
        "{INPUT}\nExpected: {expected_vars:?}, got: {:?}",
        xprs.vars
    );
}
