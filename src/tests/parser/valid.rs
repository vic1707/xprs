/* Built-in imports */
use core::f64;
/* Crate imports */
use crate::{
    element::{BinOp, Element, FunctionCall, UnOp},
    token::Operator,
    Parser,
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
#[allow(clippy::too_many_lines)]
fn get_valid_test_cases<'a>() -> Vec<(&'static str, Element<'a>)> {
    vec![
        ("2", Element::Number(2.0)),
        ("((2))", Element::Number(2.0)),
        (
            "2 + pi",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::Number(2.0),
                Element::Number(f64::consts::PI),
            ))),
        ),
        (
            "2 * y",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Times,
                Element::Number(2.0),
                Element::Variable("y"),
            ))),
        ),
        (
            "(2 + 1) + 3",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::Number(1.0),
                ))),
                Element::Number(3.0),
            ))),
        ),
        (
            "-(2 + 1)^4",
            Element::UnOp(Box::new(UnOp::new(
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
        ),
        (
            "(-(2 + 1))^4",
            Element::BinOp(Box::new(BinOp::new(
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
        ),
        (
            "2 + 3",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::Number(2.0),
                Element::Number(3.0),
            ))),
        ),
        (
            "2 + 3 * 4",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::Number(2.0),
                Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::Number(3.0),
                    Element::Number(4.0),
                ))),
            ))),
        ),
        (
            "2 * 4 + 1",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::Number(2.0),
                    Element::Number(4.0),
                ))),
                Element::Number(1.0),
            ))),
        ),
        (
            "3 / 2 * 4",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Times,
                Element::BinOp(Box::new(BinOp::new(
                    Operator::Divide,
                    Element::Number(3.0),
                    Element::Number(2.0),
                ))),
                Element::Number(4.0),
            ))),
        ),
        (
            "3 % 2 * 4",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Times,
                Element::BinOp(Box::new(BinOp::new(
                    Operator::Modulo,
                    Element::Number(3.0),
                    Element::Number(2.0),
                ))),
                Element::Number(4.0),
            ))),
        ),
        (
            "2 + -5",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::Number(2.0),
                Element::UnOp(Box::new(UnOp::new(
                    Operator::Minus,
                    Element::Number(5.0),
                ))),
            ))),
        ),
        (
            "2 + 3 ^ 2 * 3 + 4",
            Element::BinOp(Box::new(BinOp::new(
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
        ),
        (
            "2^2^(2^2 + 1)",
            Element::BinOp(Box::new(BinOp::new(
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
        ),
        (
            "2 * (3 + (4 - 1))",
            Element::BinOp(Box::new(BinOp::new(
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
        ),
        (
            "sin(2)",
            Element::Function(Box::new(FunctionCall::new(
                f64::sin,
                Element::Number(2.),
            ))),
        ),
        (
            "abs(sin(2))",
            Element::Function(Box::new(FunctionCall::new(
                f64::abs,
                Element::Function(Box::new(FunctionCall::new(
                    f64::sin,
                    Element::Number(2.),
                ))),
            ))),
        ),
        (
            "sin(-cos(2))",
            Element::Function(Box::new(FunctionCall::new(
                f64::sin,
                Element::UnOp(Box::new(UnOp::new(
                    Operator::Minus,
                    Element::Function(Box::new(FunctionCall::new(
                        f64::cos,
                        Element::Number(2.),
                    ))),
                ))),
            ))),
        ),
        (
            "sin(2)^2",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Power,
                Element::Function(Box::new(FunctionCall::new(
                    f64::sin,
                    Element::Number(2.),
                ))),
                Element::Number(2.),
            ))),
        ),
    ]
}

#[test]
fn test_valid() {
    for (expr, expected) in get_valid_test_cases() {
        let res = Parser::new(expr).parse();
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
    for &(expr, expected) in WHITESPACES {
        let res_expr = Parser::new(expr).parse();
        assert!(
            res_expr.is_ok(),
            "\n[EXPR] Should have passed for {expr}\n{res_expr:?}"
        );
        let res_expected = Parser::new(expected).parse();
        assert!(
            res_expected.is_ok(),
            "\n[EXPECTED] Should have passed for {expected}\n{res_expected:?}"
        );
        assert_eq!(
            res_expr.unwrap(),
            res_expected.unwrap(),
            "\n{expr}\n VS \n{expected}"
        );
    }
}
