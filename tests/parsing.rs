//! Grammar, precedence, whitespace, variable discovery, and syntax errors.
use crate::common::assert_f64_eq;
use core::f64::consts::PI;
use rstest::rstest;
use xprs::{ErrorKind, Parser};

fn assert_equivalent(left_input: &str, right_input: &str) {
    let parser = Parser::default();
    let left = parser.parse(left_input).unwrap();
    let right = parser.parse(right_input).unwrap();
    assert_eq!(left.vars, right.vars);
    let variables = [("x", 2.0_f64), ("y", 3.0_f64), ("X", 3.0_f64)].into();
    assert_f64_eq!(
        left.eval(&variables).unwrap(),
        right.eval(&variables).unwrap()
    );
}

#[rstest]
#[case::literal("2", 2.0)]
#[case::parentheses("((2))", 2.0)]
#[case::constant("2 + pi", 2.0 + PI)]
#[case::variable("2 * y", 6.0)]
#[case::grouping("(2 + 1) + 3", 6.0)]
#[case::negated_power("-(2 + 1)^4", -81.0)]
#[case::negative_base("(-(2 + 1))^4", 81.0)]
#[case::addition("2 + 3", 5.0)]
#[case::multiplication_precedence("2 + 3 * 4", 14.0)]
#[case::addition_precedence("2 * 4 + 1", 9.0)]
#[case::division("3 / 2 * 4", 6.0)]
#[case::remainder("3 % 2 * 4", 4.0)]
#[case::negative_operand("2 + -5", -3.0)]
#[case::mixed_precedence("2 + 3 ^ 2 * 3 + 4", 33.0)]
#[case::left_associative_powers("2^2^(2^2 + 1)", 1024.0)]
#[case::nested_grouping("2 * (3 + (4 - 1))", 12.0)]
#[case::function("sin(2)", 2.0_f64.sin())]
#[case::nested_functions("abs(sin(2))", 2.0_f64.sin().abs())]
#[case::negated_function("sin(-cos(2))", (-2.0_f64.cos()).sin())]
#[case::function_power("sin(2)^2", 2.0_f64.sin().powi(2))]
#[case::variables("2 * x + 3y + 4x + 5", 26.0)]
#[case::leading_negative("-1 + 2", 1.0)]
#[case::negative_product("-1 * 2", -2.0)]
#[case::negative_divisor("1 / -2", -0.5)]
#[case::negative_exponent("1 ^ -2", 1.0)]
#[case::unary_precedence("-2 ^ 2", -4.0)]
#[case::implicit_groups("2 (3+4) 5", 70.0)]
#[case::mixed_implicit("12 + 3-1x+3y x", 31.0)]
fn expressions(#[case] input: &str, #[case] expected: f64) {
    let tree = Parser::default().parse(input).unwrap();
    assert_f64_eq!(
        tree.eval(&[("x", 2.0_f64), ("y", 3.0_f64)].into()).unwrap(),
        expected
    );
}

#[rstest]
#[case("2 + 3", "2+3")]
#[case("2 + 3 * 4", "2+3*4")]
#[case("2 * 4 + 1", "2*4+1")]
#[case("3 / 2 * 4", "3/2*4")]
#[case("3  %2*  4", "3%2*4")]
#[case("2 +  - 5", "2+-5")]
#[case("2 + 3 ^ 2 * 3 + 4", "2+3^2*3+4")]
#[case("2^ 2  ^ (2^ 2 + 1)", "2^2^(2^2+1)")]
#[case("2 * (3 + (4 - 1) )", "2*(3+(4-1))")]
#[case("sin   ( 2 )  ^ 2", "sin(2)^2")]
fn whitespace(#[case] spaced: &str, #[case] compact: &str) {
    assert_equivalent(spaced, compact);
}

#[rstest]
#[case("2(3)", "2*(3)")]
#[case("2(3+4)", "2*(3+4)")]
#[case("2 (3+4)5", "2*(3+4)*5")]
#[case("2 (3+4) 5", "2*(3+4)*5")]
#[case("2(3+4)(5+6)", "2*(3+4)*(5+6)")]
#[case("(2*3)4", "(2*3)*4")]
#[case("(x-x)2", "(x-x)*2")]
#[case("x 2", "x*2")]
#[case("-x 2", "-x*2")]
#[case("2x + 3 x y", "2*x + 3*x*y")]
#[case("12 + 3-1x+3y x", "12 + 3-1*x+3*y*x")]
#[case("2X", "2*X")]
#[case("sin(0)2", "sin(0)*2")]
#[case("3!2", "3!*2")]
fn implicit_multiplication(#[case] implicit: &str, #[case] explicit: &str) {
    assert_equivalent(implicit, explicit);
}

#[rstest]
#[case::ambiguous_groups("6/2(2+1)", "6/2*(2+1)", "6/(2*(2+1))")]
#[case::ambiguous_variable("1/2x", "(1/2)*x", "1/(2*x)")]
fn precedence(
    #[case] input: &str,
    #[case] pemdas: &str,
    #[case] pejmdas: &str,
) {
    let explicit = if cfg!(feature = "pemdas") {
        pemdas
    } else {
        pejmdas
    };
    assert_equivalent(input, explicit);
}

#[rstest]
#[case::power("x^4", &["x"])] // Regression #18.
#[case::repeated("x + x", &["x"])]
#[case::identifier_digits("e3 + x'", &["e3", "x'"])]
fn variables(#[case] input: &str, #[case] expected: &[&str]) {
    assert_eq!(
        Parser::default().parse(input).unwrap().vars,
        expected.iter().copied().collect()
    );
}

#[rstest]
#[case::empty("", ErrorKind::UnexpectedEndOfExpression)]
#[case::trailing_factorial("3!!2", ErrorKind::UnexpectedToken('!'))]
#[case::function_without_parentheses("sin 2", ErrorKind::ExpectedToken('('))]
#[case::function_operator("sin+2", ErrorKind::ExpectedToken('('))]
#[case::unclosed_function("sin(2", ErrorKind::UnexpectedEndOfExpression)]
#[case::missing_operand("2+ -3 +", ErrorKind::UnexpectedEndOfExpression)]
#[case::unclosed_group("(2 + 3", ErrorKind::ExpectedToken(')'))]
#[case::extra_parenthesis("2 + 3)", ErrorKind::UnexpectedToken(')'))]
#[case::unclosed_nested_group("2 * (3 + 4", ErrorKind::ExpectedToken(')'))]
#[case::incomplete_group("2 * (3 + )", ErrorKind::UnexpectedToken(')'))]
#[case::malformed_number("1..2", ErrorKind::MalformedNumber("1..2".to_owned()))]
#[case::illegal_character("?", ErrorKind::IllegalCharacter('?'))]
#[case::too_few_arguments("sin()", ErrorKind::TooFewArguments(1, 0))]
#[case::too_many_arguments("sin(1, 2)", ErrorKind::TooManyArguments(1, 2))]
#[case::extra_comma("sum(1,,2)", ErrorKind::IllegalCharacter(','))]
#[case::missing_argument("sum(1,)", ErrorKind::MissingArgument)]
fn errors(#[case] input: &str, #[case] expected: ErrorKind) {
    let error = Parser::default().parse(input).unwrap_err();
    assert_eq!(error.kind(), &expected);
    assert_eq!(error.input(), input);
    assert_eq!(error.to_string(), expected.to_string());
}

#[rstest]
fn invalid_number_boundaries(
    #[values(
        "1 2",
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
        "1.2e+",
        "1.2e-",
        "1e1.5 2"
    )]
    input: &str,
) {
    Parser::default().parse(input).unwrap_err();
}
