/* Crate imports */
use crate::{
    context::Context,
    element::{BinOp, Element, FunctionCall},
    token::{Function, Operator},
    xprs_fn, Parser,
};

fn double(x: f64) -> f64 {
    x * 2.0
}
const DOUBLE: Function = xprs_fn!("DOUBLE", double, 1);
fn add(x: f64, y: f64) -> f64 {
    x + y
}
const ADD: Function = xprs_fn!("ADD", add, 2);
#[allow(clippy::as_conversions, clippy::cast_precision_loss)]
fn mean(args: &[f64]) -> f64 {
    args.iter().sum::<f64>() / args.len() as f64
}
const MEAN: Function = xprs_fn!("MEAN", mean);

fn get_parser_with_ctx() -> Parser<'static> {
    let mut ctx = Context::default();

    ctx.add_var("x", 2.0);
    ctx.add_var("phi", 1.618_033_988_749_895);

    ctx.add_func("double", DOUBLE);
    ctx.add_func("add", ADD);

    let mut parser = Parser::new_with_ctx(ctx);

    parser.ctx_mut().add_var("y", 1.0);
    parser.ctx_mut().add_func("mean", MEAN);

    parser
}

fn get_valid_test_cases() -> [(&'static str, Element<'static>); 6] {
    [
        ("y", Element::Number(1.0)),
        (
            "2 + phi",
            BinOp::new_element(
                Operator::Plus,
                Element::Number(2.0),
                Element::Number(1.618_033_988_749_895),
            ),
        ),
        (
            "2 + phi * x",
            BinOp::new_element(
                Operator::Plus,
                Element::Number(2.0),
                BinOp::new_element(
                    Operator::Times,
                    Element::Number(1.618_033_988_749_895),
                    Element::Number(2.0),
                ),
            ),
        ),
        (
            "double(2 + phi * x)",
            FunctionCall::new_element(
                DOUBLE,
                vec![BinOp::new_element(
                    Operator::Plus,
                    Element::Number(2.0),
                    BinOp::new_element(
                        Operator::Times,
                        Element::Number(1.618_033_988_749_895),
                        Element::Number(2.0),
                    ),
                )],
            ),
        ),
        (
            "add(2, 3)",
            FunctionCall::new_element(
                ADD,
                vec![Element::Number(2.0), Element::Number(3.0)],
            ),
        ),
        (
            "mean(2, 3, 4)",
            FunctionCall::new_element(
                MEAN,
                vec![
                    Element::Number(2.0),
                    Element::Number(3.0),
                    Element::Number(4.0),
                ],
            ),
        ),
    ]
}

#[test]
fn test_ctx() {
    let parser = get_parser_with_ctx();
    for (expr, expected) in get_valid_test_cases() {
        let res = parser.parse(expr);
        assert!(res.is_ok(), "Should have passed for `{expr}`\n{res:?}");
        assert_eq!(res.unwrap().root, expected, "\n`{expr}`");
    }
}
