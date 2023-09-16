#[doc(hidden)]
macro_rules! yeet {
    () => {
        return None
    };
    ($err:expr) => {
        return Err($err)
    };
}

pub(super) use yeet;
