/* Crate imports */
use crate::{
    element::{BinOp, Element, FunctionCall, UnOp},
    token::Operator,
    Parser,
};

const fn is_sized_send_sync_unpin<T: Sized + Send + Sync + Unpin>() {}

#[test]
const fn test_thread_safety() {
    is_sized_send_sync_unpin::<Parser>();
    is_sized_send_sync_unpin::<Element<'_>>();
    is_sized_send_sync_unpin::<BinOp<'_>>();
    is_sized_send_sync_unpin::<UnOp<'_>>();
    is_sized_send_sync_unpin::<FunctionCall<'_>>();
    is_sized_send_sync_unpin::<Operator>();
}
