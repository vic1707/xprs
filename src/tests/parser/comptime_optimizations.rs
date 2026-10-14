/* Built-in imports */
use core::f64;
/* Crate imports */
use crate::tests::macros::assert_f64_eq;
use crate::{
    Parser, Xprs,
    element::{BinOp, Element},
    token::Operator,
};

///// Tests the following expressions:
/// 2 + pi
/// 2 * y
/// 2 + 3 * 4
/// 2 * 4 + 1
/// 3 / 2 * 4
/// 3 % 2 * 4
/// 2 + -5
/// 2 + 3 ^ 2 * 3 + 4
/// 2^2^(2^2 + 1)
/// 2 * (3 + (4 - 1))
/// sin(-cos(2))
////// With variables
/// x - x
////// Commutativity
/// (a + b) - (b + a)
fn get_valid_test_cases() -> [(&'static str, Xprs<'static>); 13] {
    [
        (
            "2 + pi",
            Xprs {
                root: Element::Number(f64::consts::PI + 2.),
                vars: [].into(),
            },
        ),
        (
            "2 * y",
            Xprs {
                root: BinOp::new_element(
                    Operator::Times,
                    Element::Number(2.),
                    Element::Variable("y"),
                ),
                vars: ["y"].into(),
            },
        ),
        (
            "2 + 3 * 4",
            Xprs {
                root: Element::Number(14.),
                vars: [].into(),
            },
        ),
        (
            "2 * 4 + 1",
            Xprs {
                root: Element::Number(9.),
                vars: [].into(),
            },
        ),
        (
            "3 / 2 * 4",
            Xprs {
                root: Element::Number(6.),
                vars: [].into(),
            },
        ),
        (
            "3 % 2 * 4",
            Xprs {
                root: Element::Number(4.),
                vars: [].into(),
            },
        ),
        (
            "2 + -5",
            Xprs {
                root: Element::Number(-3.),
                vars: [].into(),
            },
        ),
        (
            "2 + 3 ^ 2 * 3 + 4",
            Xprs {
                root: Element::Number(33.),
                vars: [].into(),
            },
        ),
        (
            "2^2^(2^2 + 1)",
            Xprs {
                root: Element::Number(1024.),
                vars: [].into(),
            },
        ),
        (
            "2 * (3 + (4 - 1))",
            Xprs {
                root: Element::Number(12.),
                vars: [].into(),
            },
        ),
        (
            "sin(-cos(2))",
            Xprs {
                root: Element::Number(f64::sin(-f64::cos(2.))),
                vars: [].into(),
            },
        ),
        (
            "x - x",
            Xprs {
                root: Element::Number(0.),
                vars: [].into(),
            },
        ),
        (
            "(a + b) - (b + a)",
            Xprs {
                root: Element::Number(0.),
                vars: [].into(),
            },
        ),
    ]
}

#[test]
fn valid() {
    let parser = Parser::default();
    for (expr, expected) in get_valid_test_cases() {
        let res = parser.parse(expr);
        assert!(res.is_ok(), "\nShould have passed for {expr}\n{res:?}");
        let actual = res.unwrap();
        assert_eq!(actual.vars, expected.vars, "{expr}");
        match (&actual.root, &expected.root) {
            (&Element::Number(actual_num), &Element::Number(expected_num)) => {
                assert_f64_eq!(actual_num, expected_num, "{expr}");
            },
            _ => assert_eq!(actual.root, expected.root, "{expr}"),
        }
    }
}

#[test]
fn algebraic_simplifications() {
    let parser = Parser::default();
    for (expression, expected) in [
        ("(-x) + x", "0"),
        ("x + (-x)", "0"),
        ("x - (-y)", "(x + y)"),
        ("(-x) * (-y)", "(x * y)"),
        ("(-x) / (-y)", "(x / y)"),
        ("x + x!", "(x + (!x))"),
        ("x - y!", "(x - (!y))"),
        ("(-x) * y!", "((-x) * (!y))"),
        ("x! / (-y)", "((!x) / (-y))"),
        ("x ^ 0", "1"),
        ("sin(0)!", "1"),
        ("sum(1, 2)!", "6"),
        ("sin(0)! + x", "(1 + x)"),
        ("x + 0 + sum(1, 2 * 3)!", "(x + 5040)"),
        ("-(-x)", "x"),
        ("0 - (-x)", "x"),
        ("x - (-x)", "(x + x)"),
        ("x * 1", "x"),
        ("x / 1", "x"),
        ("x ^ 1", "x"),
        ("-0 + x", "x"),
        ("0 * x", "0"),
        ("x * 0", "0"),
        ("0 / x", "0"),
        ("0 % x", "0"),
        ("0 / 0", "NaN"),
        ("0 % 0", "NaN"),
        ("0 ^ 0", "1"),
    ] {
        assert_eq!(
            parser.parse(expression).unwrap().to_string(),
            expected,
            "{expression}"
        );
    }
}

#[test]
fn folding_preserves_implicit_multiplication_rules() {
    let parser = Parser::default();
    for (input, expected) in [("sin(0)2", 0.0_f64), ("3!2", 12.0_f64)] {
        assert_f64_eq!(
            parser.parse(input).unwrap().eval_no_vars().unwrap(),
            expected
        );
    }
}
