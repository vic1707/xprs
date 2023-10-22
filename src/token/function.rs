#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Function<'a> {
    pub(crate) name: &'a str,
    pub(crate) func: fn(&[f64]) -> f64,
    pub(crate) nb_args: usize,
}

impl<'a> Function<'a> {
    pub const fn new(
        name: &'a str,
        func: fn(&[f64]) -> f64,
        nb_args: usize,
    ) -> Self {
        Self {
            name,
            func,
            nb_args,
        }
    }
}

macro_rules! function {
    ($function:expr, 1) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(stringify!($function), move |args| $function(args[0]), 1)
    };
    ($function:expr, 2) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1]),
            2,
        )
    };
    ($function:expr, 3) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1], args[2]),
            3,
        )
    };
    ($function:expr, 4) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1], args[2], args[3]),
            4,
        )
    };
    ($function:expr, 5) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1], args[2], args[3], args[4]),
            5,
        )
    };
    ($function:expr, 6) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| {
                $function(args[0], args[1], args[2], args[3], args[4], args[5])
            },
            6,
        )
    };
    ($function:expr, 7) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| {
                $function(
                    args[0], args[1], args[2], args[3], args[4], args[5],
                    args[6],
                )
            },
            7,
        )
    };
    ($function:expr, 8) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| {
                $function(
                    args[0], args[1], args[2], args[3], args[4], args[5],
                    args[6], args[7],
                )
            },
            8,
        )
    };
    ($function:expr, 9) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| {
                $function(
                    args[0], args[1], args[2], args[3], args[4], args[5],
                    args[6], args[7], args[8],
                )
            },
            9,
        )
    };
}

pub(crate) use function;
