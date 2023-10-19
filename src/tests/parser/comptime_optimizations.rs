/* Built-in imports */
use core::f64;
/* Crate imports */
use crate::{
    element::{BinOp, Element},
    token::Operator,
    Parser, Xprs,
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
#[allow(clippy::too_many_lines)]
fn get_valid_test_cases<'a>() -> [(&'static str, Xprs<'a>); 12] {
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
                root: Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::Number(2.),
                    Element::Variable("y"),
                ))),
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
                vars: ["x"].into(),
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
