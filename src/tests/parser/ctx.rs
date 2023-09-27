/* Built-in imports */
use std::collections::HashMap;
/* Crate imports */
use crate::{
    element::{BinOp, Element},
    token::Operator,
    Parser,
};

fn get_parser_with_ctx() -> Parser {
    let mut ctx = HashMap::<String, f64>::new();

    ctx.insert("x".to_owned(), 2.0_f64);
    ctx.insert("phi".to_owned(), 1.618_033_988_749_895_f64);

    let mut parser = Parser::new_with_ctx(ctx);
    parser.ctx_mut().insert("y".to_owned(), 1.0_f64);

    parser
}

fn get_valid_test_cases<'a>() -> Vec<(&'static str, Element<'a>)> {
    vec![
        (
            "y",
            Element::Number(1.0),
        ),
        (
            "2 + phi",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::Number(2.0),
                Element::Number(1.618_033_988_749_895),
            ))),
        ),
        (
            "2 + phi * x",
            Element::BinOp(Box::new(BinOp::new(
                Operator::Plus,
                Element::Number(2.0),
                Element::BinOp(Box::new(BinOp::new(
                    Operator::Times,
                    Element::Number(1.618_033_988_749_895),
                    Element::Number(2.0),
                ))),
            ))),
        ),
    ]
}

#[test]
fn test_ctx() {
    let parser = get_parser_with_ctx();
    for (expr, expected) in get_valid_test_cases() {
        let res = parser.parse(expr);
        assert!(res.is_ok(), "\nShould have passed for {expr}\n{res:?}");
        assert_eq!(res.unwrap(), expected, "\n{expr}");
    }
}
