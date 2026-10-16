//! Public types retain their cross-thread usability.
use xprs::{
    BindError, Context, ErrorKind, EvalError, Function, ParseError, Parser,
    Symbol, Xprs,
};

const fn assert_send_sync<T>()
where
    T: Send + Sync,
{
}

#[test]
const fn thread_safety() {
    assert_send_sync::<Context>();
    assert_send_sync::<Symbol>();
    assert_send_sync::<ErrorKind>();
    assert_send_sync::<ParseError>();
    assert_send_sync::<Parser>();
    assert_send_sync::<Function>();
    assert_send_sync::<BindError>();
    assert_send_sync::<EvalError>();
    assert_send_sync::<Xprs<'_>>();
}
