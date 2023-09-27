#[doc(hidden)]
macro_rules! trust_me {
    ($($t:tt)*) => {
        unsafe { $($t)* }
    }
}

pub(crate) use trust_me;

#[doc(hidden)]
macro_rules! yeet {
    () => {
        return None
    };
    ($err:expr) => {
        return Err($err)
    };
}

pub(crate) use yeet;
