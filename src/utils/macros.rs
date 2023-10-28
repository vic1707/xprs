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

#[doc(hidden)]
macro_rules! wrap_into_closure {
    ($function:expr, 0) => {
        move |_| $function()
    };
    ($function:expr, 1) => {
        #[allow(clippy::indexing_slicing)]
        move |args| $function(args[0])
    };
    ($function:expr, 2) => {
        #[allow(clippy::indexing_slicing)]
        move |args| $function(args[0], args[1])
    };
    ($function:expr, 3) => {
        #[allow(clippy::indexing_slicing)]
        move |args| $function(args[0], args[1], args[2])
    };
    ($function:expr, 4) => {
        #[allow(clippy::indexing_slicing)]
        move |args| $function(args[0], args[1], args[2], args[3])
    };
    ($function:expr, 5) => {
        #[allow(clippy::indexing_slicing)]
        move |args| $function(args[0], args[1], args[2], args[3], args[4])
    };
    ($function:expr, 6) => {
        #[allow(clippy::indexing_slicing)]
        move |args| {
            $function(args[0], args[1], args[2], args[3], args[4], args[5])
        }
    };
    ($function:expr, 7) => {
        #[allow(clippy::indexing_slicing)]
        move |args| {
            $function(
                args[0], args[1], args[2], args[3], args[4], args[5], args[6],
            )
        }
    };
    ($function:expr, 8) => {
        #[allow(clippy::indexing_slicing)]
        move |args| {
            $function(
                args[0], args[1], args[2], args[3], args[4], args[5], args[6],
                args[7],
            )
        }
    };
    ($function:expr, 9) => {
        #[allow(clippy::indexing_slicing)]
        move |args| {
            $function(
                args[0], args[1], args[2], args[3], args[4], args[5], args[6],
                args[7], args[8],
            )
        }
    };
}

pub(crate) use wrap_into_closure;
