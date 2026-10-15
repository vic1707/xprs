//! Internal allocation-retention invariant during substitution.
use crate::{Parser, element::Element};
use core::ptr;
use rstest::rstest;

#[rstest]
#[case("x + y")]
#[case("-x")]
#[case("sum(x, y)")]
fn retains_unfolded_allocations(#[case] input: &str) {
    let allocations = |root: &Element<'_>| match *root {
        Element::BinOp(ref node) => {
            (ptr::from_ref(node.as_ref()).cast::<()>(), ptr::null())
        },
        Element::UnOp(ref node) => {
            (ptr::from_ref(node.as_ref()).cast::<()>(), ptr::null())
        },
        Element::Function(ref node) => (
            ptr::from_ref(node.as_ref()).cast::<()>(),
            node.args.as_ptr().cast::<()>(),
        ),
        Element::Number(_) | Element::Variable(_) => (ptr::null(), ptr::null()),
    };
    let mut tree = Parser::default().parse(input).unwrap();
    let before = allocations(&tree.root);
    tree.simplify_for_in_place(("unknown", 0.0_f64));
    assert_eq!(allocations(&tree.root), before);
}
