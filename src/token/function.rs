#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Function {
    pub(crate) name: &'static str,
    pub(crate) func: fn(&[f64]) -> f64,
    pub(crate) nb_args: Option<u8>,
}

impl Function {
    #[inline]
    pub const fn new(
        name: &'static str,
        func: fn(&[f64]) -> f64,
        nb_args: Option<u8>,
    ) -> Self {
        Self {
            name,
            func,
            nb_args,
        }
    }
}

#[macro_export]
macro_rules! xprs_fn {
    // variadics
    ($name:expr, $function:expr) => {
        $crate::Function::new($name, $function, None)
    };
    ($function:expr) => {
        $crate::Function::new(stringify!($function), $function, None)
    };
    // fixed args
    ($name:expr, $function:expr, $nb_args:tt) => {
        $crate::Function::new(
            $name,
            $crate::xprs_fn!(wrap $function, $nb_args),
            Some($nb_args),
        )
    };
    ($function:expr, $nb_args:tt) => {
        $crate::Function::new(
            stringify!($function),
            $crate::xprs_fn!(wrap $function, $nb_args),
            Some($nb_args),
        )
    };

    //// closure wraping ////
    (wrap $function:expr, 0) => {
        |_| $function()
    };
    (wrap $function:expr, 1) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0])
    };
    (wrap $function:expr, 2) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1])
    };
    (wrap $function:expr, 3) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1], args[2])
    };
    (wrap $function:expr, 4) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1], args[2], args[3])
    };
    (wrap $function:expr, 5) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1], args[2], args[3], args[4])
    };
    (wrap $function:expr, 6) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1], args[2], args[3], args[4], args[5])
    };
    (wrap $function:expr, 7) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1], args[2], args[3], args[4], args[5], args[6])
    };
    (wrap $function:expr, 8) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7])
    };
    (wrap $function:expr, 9) => {
        #[allow(clippy::indexing_slicing)]
        |args| $function(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8])
    };
}
