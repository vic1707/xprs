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
    for &should_fail in &ERRORS {
        let res = Parser::new(should_fail).parse();
        assert!(res.is_err(), "\nShould have failed for {should_fail}");
    }
}
