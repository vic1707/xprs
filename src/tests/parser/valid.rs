/* Built-in imports */
use core::f64;
/* Crate imports */
use crate::{
    element::{BinOp, Element, FunctionCall, UnOp},
    token::Operator,
    Parser, Xprs,
};

///// Tests the following expressions:
/// 2
/// ((2))
/// 2 + pi
/// 2 * y
/// (2 + 1) + 3
/// -(2 + 1)^4
/// (-(2 + 1))^4
/// 2 + 3
/// 2 + 3 * 4
/// 2 * 4 + 1
/// 3 / 2 * 4
/// 3 % 2 * 4
/// 2 + -5
/// 2 + 3 ^ 2 * 3 + 4
/// 2^2^(2^2 + 1)
/// 2 * (3 + (4 - 1))
/// sin(2)
/// abs(sin(2))
/// sin(-cos(2))
/// sin(2)^2
/// 2 * x + 3y + 4x + 5
#[allow(clippy::too_many_lines)]
fn get_valid_test_cases<'a>() -> Vec<(&'static str, Xprs<'a>)> {
    vec![
        (
            "2",
            Xprs {
                root: Element::Number(2.0),
                vars: [].into(),
            },
        ),
        (
            "((2))",
            Xprs {
                root: Element::Number(2.0),
                vars: [].into(),
            },
        ),
        (
            "2 + pi",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::Number(f64::consts::PI),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 * y",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::Number(2.0),
                    Element::Variable("y"),
                ))),
                vars: ["y"].into(),
            },
        ),
        (
            "(2 + 1) + 3",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Plus,
                        Element::Number(2.0),
                        Element::Number(1.0),
                    ))),
                    Element::Number(3.0),
                ))),
                vars: [].into(),
            },
        ),
        (
            "-(2 + 1)^4",
            Xprs {
                root: Element::UnOp(Box::new(UnOp::new(
                    Operator::Minus,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Power,
                        Element::BinOp(Box::new(BinOp::new(
                            Operator::Plus,
                            Element::Number(2.0),
                            Element::Number(1.0),
                        ))),
                        Element::Number(4.0),
                    ))),
                ))),
                vars: [].into(),
            },
        ),
        (
            "(-(2 + 1))^4",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Power,
                    Element::UnOp(Box::new(UnOp::new(
                        Operator::Minus,
                        Element::BinOp(Box::new(BinOp::new(
                            Operator::Plus,
                            Element::Number(2.0),
                            Element::Number(1.0),
                        ))),
                    ))),
                    Element::Number(4.0),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 + 3",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::Number(3.0),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 + 3 * 4",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Times,
                        Element::Number(3.0),
                        Element::Number(4.0),
                    ))),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 * 4 + 1",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Times,
                        Element::Number(2.0),
                        Element::Number(4.0),
                    ))),
                    Element::Number(1.0),
                ))),
                vars: [].into(),
            },
        ),
        (
            "3 / 2 * 4",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Divide,
                        Element::Number(3.0),
                        Element::Number(2.0),
                    ))),
                    Element::Number(4.0),
                ))),
                vars: [].into(),
            },
        ),
        (
            "3 % 2 * 4",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Modulo,
                        Element::Number(3.0),
                        Element::Number(2.0),
                    ))),
                    Element::Number(4.0),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 + -5",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::UnOp(Box::new(UnOp::new(
                        Operator::Minus,
                        Element::Number(5.0),
                    ))),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 + 3 ^ 2 * 3 + 4",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Plus,
                        Element::Number(2.0),
                        Element::BinOp(Box::new(BinOp::new(
                            Operator::Times,
                            Element::BinOp(Box::new(BinOp::new(
                                Operator::Power,
                                Element::Number(3.0),
                                Element::Number(2.0),
                            ))),
                            Element::Number(3.0),
                        ))),
                    ))),
                    Element::Number(4.0),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2^2^(2^2 + 1)",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Power,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Power,
                        Element::Number(2.0),
                        Element::Number(2.0),
                    ))),
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Plus,
                        Element::BinOp(Box::new(BinOp::new(
                            Operator::Power,
                            Element::Number(2.0),
                            Element::Number(2.0),
                        ))),
                        Element::Number(1.0),
                    ))),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 * (3 + (4 - 1))",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::Number(2.0),
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Plus,
                        Element::Number(3.0),
                        Element::BinOp(Box::new(BinOp::new(
                            Operator::Minus,
                            Element::Number(4.0),
                            Element::Number(1.0),
                        ))),
                    ))),
                ))),
                vars: [].into(),
            },
        ),
        (
            "sin(2)",
            Xprs {
                root: Element::Function(Box::new(FunctionCall::new(
                    f64::sin,
                    Element::Number(2.),
                ))),
                vars: [].into(),
            },
        ),
        (
            "abs(sin(2))",
            Xprs {
                root: Element::Function(Box::new(FunctionCall::new(
                    f64::abs,
                    Element::Function(Box::new(FunctionCall::new(
                        f64::sin,
                        Element::Number(2.),
                    ))),
                ))),
                vars: [].into(),
            },
        ),
        (
            "sin(-cos(2))",
            Xprs {
                root: Element::Function(Box::new(FunctionCall::new(
                    f64::sin,
                    Element::UnOp(Box::new(UnOp::new(
                        Operator::Minus,
                        Element::Function(Box::new(FunctionCall::new(
                            f64::cos,
                            Element::Number(2.),
                        ))),
                    ))),
                ))),
                vars: [].into(),
            },
        ),
        (
            "sin(2)^2",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Power,
                    Element::Function(Box::new(FunctionCall::new(
                        f64::sin,
                        Element::Number(2.),
                    ))),
                    Element::Number(2.),
                ))),
                vars: [].into(),
            },
        ),
        (
            "2 * x + 3y + 4x + 5",
            Xprs {
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Plus,
                        Element::BinOp(Box::new(BinOp::new(
                            Operator::Plus,
                            Element::BinOp(Box::new(BinOp::new(
                                Operator::Times,
                                Element::Number(2.),
                                Element::Variable("x"),
                            ))),
                            Element::BinOp(Box::new(BinOp::new(
                                Operator::Times,
                                Element::Number(3.),
                                Element::Variable("y"),
                            ))),
                        ))),
                        Element::BinOp(Box::new(BinOp::new(
                            Operator::Times,
                            Element::Number(4.),
                            Element::Variable("x"),
                        ))),
                    ))),
                    Element::Number(5.),
                ))),
                vars: ["x", "y"].into(),
            },
        ),
    ]
}

#[test]
fn test_valid() {
    let parser = Parser::default();
    for (expr, expected) in get_valid_test_cases() {
        let res = parser.parse(expr);
        assert!(res.is_ok(), "\nShould have passed for {expr}\n{res:?}");
        assert_eq!(res.unwrap(), expected, "\n{expr}");
    }
}

const WHITESPACES: &[(&str, &str); 10] = &[
    ("2 + 3", "2+3"),
    ("2 + 3 * 4", "2+3*4"),
    ("2 * 4 + 1", "2*4+1"),
    ("3 / 2 * 4", "3/2*4"),
    ("3  %2*  4", "3%2*4"),
    ("2 +  - 5", "2+-5"),
    ("2 + 3 ^ 2 * 3 + 4", "2+3^2*3+4"),
    ("2^ 2  ^ (2^ 2 + 1)", "2^2^(2^2+1)"),
    ("2 * (3 + (4 - 1) )", "2*(3+(4-1))"),
    ("sin   ( 2 )  ^ 2", "sin(2)^2"),
];

#[test]
fn test_valid_with_whitespace() {
    let parser = Parser::default();
    for &(expr, expected) in WHITESPACES {
        let res_expr = parser.parse(expr);
        assert!(
            res_expr.is_ok(),
            "[EXPR] Should have passed for `{expr}`\n{res_expr:?}\n"
        );
        let res_expected = parser.parse(expected);
        assert!(
            res_expected.is_ok(),
            "[EXPECTED] Should have passed for `{expected}`\n{res_expected:?}\n"
        );
        assert_eq!(
            res_expr.unwrap(),
            res_expected.unwrap(),
            "\n`{expr}` VS `{expected}`"
        );
    }
}
