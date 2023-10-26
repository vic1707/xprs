/* Crate imports */
use crate::token::Identifier;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Function<'a> {
    pub(crate) name: &'a str,
    pub(crate) func: fn(&[f64]) -> f64,
    pub(crate) nb_args: Option<u8>,
}

impl<'a> Function<'a> {
    pub const fn new(
        name: &'a str,
        func: fn(&[f64]) -> f64,
        nb_args: Option<u8>,
    ) -> Self {
        Self {
            name,
            func,
            nb_args,
        }
    }

    pub const fn new_identifier(
        name: &'a str,
        func: fn(&[f64]) -> f64,
        nb_args: Option<u8>,
    ) -> Identifier<'a> {
        Identifier::Function(Self::new(name, func, nb_args))
    }
}

macro_rules! function {
    ($function:expr, 1) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0]),
            Some(1),
        )
    };
    ($function:expr, 2) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1]),
            Some(2),
        )
    };
    ($function:expr, 3) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1], args[2]),
            Some(3),
        )
    };
    ($function:expr, 4) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1], args[2], args[3]),
            Some(4),
        )
    };
    ($function:expr, 5) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| $function(args[0], args[1], args[2], args[3], args[4]),
            Some(5),
        )
    };
    ($function:expr, 6) => {
        #[allow(clippy::indexing_slicing)]
        Function::new(
            stringify!($function),
            move |args| {
                $function(args[0], args[1], args[2], args[3], args[4], args[5])
            },
            Some(6),
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
            Some(7),
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
            Some(8),
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
            Some(9),
        )
    };
}

pub(crate) use function;
