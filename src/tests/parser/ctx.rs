/* Crate imports */
use crate::{
    context::Context,
    element::{BinOp, Element, FunctionCall},
    misc::Function,
    token::Operator,
    Parser,
};

const DOUBLE: Function = |x| x * 2.0_f64;
fn triple(x: f64) -> f64 {
    x * 3.0_f64
}

fn get_parser_with_ctx<'a>() -> Parser<'a> {
    let mut ctx = Context::default();

    ctx.vars.insert("x", 2.0_f64);
    ctx.vars.insert("phi", 1.618_033_988_749_895_f64);

    ctx.funcs.insert("double", DOUBLE);
    ctx.funcs.insert("triple", triple);

    let mut parser = Parser::new_with_ctx(ctx);

    parser.ctx_mut().vars.insert("y", 1.0_f64);

    parser
}

fn get_valid_test_cases<'a>() -> [(&'static str, Element<'a>); 5] {
    [
        ("y", Element::Number(1.0)),
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
        (
            "double(2 + phi * x)",
            Element::Function(Box::new(FunctionCall::new(
                DOUBLE,
                Element::BinOp(Box::new(BinOp::new(
                    Operator::Plus,
                    Element::Number(2.0),
                    Element::BinOp(Box::new(BinOp::new(
                        Operator::Times,
                        Element::Number(1.618_033_988_749_895),
                        Element::Number(2.0),
                    ))),
                ))),
            ))),
        ),
        (
            "triple(2)",
            Element::Function(Box::new(FunctionCall::new(
                triple,
                Element::Number(2.0),
            ))),
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
