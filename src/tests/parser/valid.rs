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
/// Ambiguous negative numbers (#5)
///  2 + -5
/// -1 + 2
/// -1 * 2
/// 1 / -2
/// 1 ^ -2
/// -2 ^ 2
#[allow(clippy::too_many_lines)]
fn get_valid_test_cases<'a>() -> [(&'static str, Xprs<'a>); 27] {
    [
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
                root: BinOp::new_element(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::Number(f64::consts::PI),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 * y",
            Xprs {
                root: BinOp::new_element(
                    Operator::Times,
                    Element::Number(2.0),
                    Element::Variable("y"),
                ),
                vars: ["y"].into(),
            },
        ),
        (
            "(2 + 1) + 3",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    BinOp::new_element(
                        Operator::Plus,
                        Element::Number(2.0),
                        Element::Number(1.0),
                    ),
                    Element::Number(3.0),
                ),
                vars: [].into(),
            },
        ),
        (
            "-(2 + 1)^4",
            Xprs {
                root: UnOp::new_element(
                    Operator::Minus,
                    BinOp::new_element(
                        Operator::Power,
                        BinOp::new_element(
                            Operator::Plus,
                            Element::Number(2.0),
                            Element::Number(1.0),
                        ),
                        Element::Number(4.0),
                    ),
                ),
                vars: [].into(),
            },
        ),
        (
            "(-(2 + 1))^4",
            Xprs {
                root: BinOp::new_element(
                    Operator::Power,
                    UnOp::new_element(
                        Operator::Minus,
                        BinOp::new_element(
                            Operator::Plus,
                            Element::Number(2.0),
                            Element::Number(1.0),
                        ),
                    ),
                    Element::Number(4.0),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 + 3",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::Number(3.0),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 + 3 * 4",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    Element::Number(2.0),
                    BinOp::new_element(
                        Operator::Times,
                        Element::Number(3.0),
                        Element::Number(4.0),
                    ),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 * 4 + 1",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    BinOp::new_element(
                        Operator::Times,
                        Element::Number(2.0),
                        Element::Number(4.0),
                    ),
                    Element::Number(1.0),
                ),
                vars: [].into(),
            },
        ),
        (
            "3 / 2 * 4",
            Xprs {
                root: BinOp::new_element(
                    Operator::Times,
                    BinOp::new_element(
                        Operator::Divide,
                        Element::Number(3.0),
                        Element::Number(2.0),
                    ),
                    Element::Number(4.0),
                ),
                vars: [].into(),
            },
        ),
        (
            "3 % 2 * 4",
            Xprs {
                root: BinOp::new_element(
                    Operator::Times,
                    BinOp::new_element(
                        Operator::Modulo,
                        Element::Number(3.0),
                        Element::Number(2.0),
                    ),
                    Element::Number(4.0),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 + -5",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    Element::Number(2.0),
                    UnOp::new_element(Operator::Minus, Element::Number(5.0)),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 + 3 ^ 2 * 3 + 4",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    BinOp::new_element(
                        Operator::Plus,
                        Element::Number(2.0),
                        BinOp::new_element(
                            Operator::Times,
                            BinOp::new_element(
                                Operator::Power,
                                Element::Number(3.0),
                                Element::Number(2.0),
                            ),
                            Element::Number(3.0),
                        ),
                    ),
                    Element::Number(4.0),
                ),
                vars: [].into(),
            },
        ),
        (
            "2^2^(2^2 + 1)",
            Xprs {
                root: BinOp::new_element(
                    Operator::Power,
                    BinOp::new_element(
                        Operator::Power,
                        Element::Number(2.0),
                        Element::Number(2.0),
                    ),
                    BinOp::new_element(
                        Operator::Plus,
                        BinOp::new_element(
                            Operator::Power,
                            Element::Number(2.0),
                            Element::Number(2.0),
                        ),
                        Element::Number(1.0),
                    ),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 * (3 + (4 - 1))",
            Xprs {
                root: BinOp::new_element(
                    Operator::Times,
                    Element::Number(2.0),
                    BinOp::new_element(
                        Operator::Plus,
                        Element::Number(3.0),
                        BinOp::new_element(
                            Operator::Minus,
                            Element::Number(4.0),
                            Element::Number(1.0),
                        ),
                    ),
                ),
                vars: [].into(),
            },
        ),
        (
            "sin(2)",
            Xprs {
                root: FunctionCall::new_element(
                    "sin",
                    f64::sin,
                    Element::Number(2.),
                ),
                vars: [].into(),
            },
        ),
        (
            "abs(sin(2))",
            Xprs {
                root: FunctionCall::new_element(
                    "abs",
                    f64::abs,
                    FunctionCall::new_element(
                        "sin",
                        f64::sin,
                        Element::Number(2.),
                    ),
                ),
                vars: [].into(),
            },
        ),
        (
            "sin(-cos(2))",
            Xprs {
                root: FunctionCall::new_element(
                    "sin",
                    f64::sin,
                    UnOp::new_element(
                        Operator::Minus,
                        FunctionCall::new_element(
                            "cos",
                            f64::cos,
                            Element::Number(2.),
                        ),
                    ),
                ),
                vars: [].into(),
            },
        ),
        (
            "sin(2)^2",
            Xprs {
                root: BinOp::new_element(
                    Operator::Power,
                    FunctionCall::new_element(
                        "sin",
                        f64::sin,
                        Element::Number(2.),
                    ),
                    Element::Number(2.),
                ),
                vars: [].into(),
            },
        ),
        (
            "2 * x + 3y + 4x + 5",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    BinOp::new_element(
                        Operator::Plus,
                        BinOp::new_element(
                            Operator::Plus,
                            BinOp::new_element(
                                Operator::Times,
                                Element::Number(2.),
                                Element::Variable("x"),
                            ),
                            BinOp::new_element(
                                Operator::Times,
                                Element::Number(3.),
                                Element::Variable("y"),
                            ),
                        ),
                        BinOp::new_element(
                            Operator::Times,
                            Element::Number(4.),
                            Element::Variable("x"),
                        ),
                    ),
                    Element::Number(5.),
                ),
                vars: ["x", "y"].into(),
            },
        ),
        (
            "2 + -5",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    Element::Number(2.),
                    UnOp::new_element(Operator::Minus, Element::Number(5.)),
                ),
                vars: [].into(),
            },
        ),
        (
            "-1 + 2",
            Xprs {
                root: BinOp::new_element(
                    Operator::Plus,
                    UnOp::new_element(Operator::Minus, Element::Number(1.)),
                    Element::Number(2.),
                ),
                vars: [].into(),
            },
        ),
        (
            "-1 * 2",
            Xprs {
                root: BinOp::new_element(
                    Operator::Times,
                    UnOp::new_element(Operator::Minus, Element::Number(1.)),
                    Element::Number(2.),
                ),
                vars: [].into(),
            },
        ),
        (
            "1 / -2",
            Xprs {
                root: BinOp::new_element(
                    Operator::Divide,
                    Element::Number(1.),
                    UnOp::new_element(Operator::Minus, Element::Number(2.)),
                ),
                vars: [].into(),
            },
        ),
        (
            "1 ^ -2",
            Xprs {
                root: BinOp::new_element(
                    Operator::Power,
                    Element::Number(1.),
                    UnOp::new_element(Operator::Minus, Element::Number(2.)),
                ),
                vars: [].into(),
            },
        ),
        (
            "-2 ^ 2",
            Xprs {
                root: UnOp::new_element(
                    Operator::Minus,
                    BinOp::new_element(
                        Operator::Power,
                        Element::Number(2.),
                        Element::Number(2.),
                    ),
                ),
                vars: [].into(),
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
