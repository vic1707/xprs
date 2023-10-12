/* Crate imports */
use crate::{
    context::Context,
    element::{BinOp, Element, FunctionCall, UnOp},
    parser::{ErrorKind, ParseError, Parser},
    token::{Identifier, Operator},
    xprs::Xprs,
};

const fn is_sized_send_sync_unpin<T: Sized + Send + Sync + Unpin>() {}

#[test]
const fn test_thread_safety() {
    // context module
    is_sized_send_sync_unpin::<Context>();
    // element module
    is_sized_send_sync_unpin::<BinOp<'_>>();
    is_sized_send_sync_unpin::<Element<'_>>();
    is_sized_send_sync_unpin::<FunctionCall<'_>>();
    is_sized_send_sync_unpin::<UnOp<'_>>();
    // parser module
    is_sized_send_sync_unpin::<ErrorKind>();
    is_sized_send_sync_unpin::<ParseError>();
    is_sized_send_sync_unpin::<Parser>();
    // token module
    is_sized_send_sync_unpin::<Identifier>();
    is_sized_send_sync_unpin::<Operator>();
    // xprs module
    is_sized_send_sync_unpin::<Xprs<'_>>();
}
