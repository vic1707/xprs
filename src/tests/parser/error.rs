/* Crate imports */
use crate::Parser;

// TODO: add & test specific error messages
const ERRORS: [&str; 9] = [
    "sin 2",
    "sin+2",
    "sin(2",
    "2+ -3 +",
    "(2 + 3",
    "2 + 3)",
    "2 * (3 + 4",
    "2 * (3 + )",
    "1 2",
];

#[test]
fn test_errors() {
    let parser = Parser::default();
    for &should_fail in &ERRORS {
        let res = parser.parse(should_fail);
        assert!(res.is_err(), "Should have failed for `{should_fail}`");
    }
}
