/* Built-in imports */
extern crate alloc;
use alloc::sync::Arc;
use core::{cmp::Ordering, fmt, ops::Deref};

/// Represents a mathematical function core infos.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Function {
    /// The name of the function.
    pub name: &'static str,
    /// The function's implementation.
    pub func: FnPointer,
    /// The optional number of arguments the function accepts.
    /// If [`None`], the function is variadic.
    pub nb_args: Option<u8>,
}

impl Function {
    /// Creates a new [`Function`] from static function components.
    /// Note that the fn pointer must be a function that takes a slice of f64 as argument and returns a f64.
    /// So make sure to wrap your function in a closure if it doesn't match the signature.
    /// For convenience, you can use the [`crate::xprs_fn!`] macro.
    ///
    /// [`Function`] needs a fn taking a slice because Rust variadics are not available yet.
    #[inline]
    pub const fn new_static(
        name: &'static str,
        func: fn(&[f64]) -> f64,
        nb_args: Option<u8>,
    ) -> Self {
        Self {
            name,
            func: FnPointer::Static(func),
            nb_args,
        }
    }

    /// Creates a new [`Function`] from dynamic function components.
    /// Note that the fn pointer must be a function that takes a slice of f64 as argument and returns a f64.
    /// So make sure to wrap your function in a closure if it doesn't match the signature.
    /// For convenience, you can use the [`crate::xprs_fn!`] macro.
    ///
    /// [`Function`] needs a fn taking a slice because Rust variadics are not available yet.
    #[inline]
    pub fn new_dyn<T>(name: &'static str, func: T, nb_args: Option<u8>) -> Self
    where
        T: Fn(&[f64]) -> f64 + Send + Sync + 'static,
    {
        Self {
            name,
            func: FnPointer::Dyn(Arc::new(func)),
            nb_args,
        }
    }
}

impl PartialOrd for Function {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(other.name)
    }
}

/// A dynamic function reference.
type DynFn = dyn Fn(&[f64]) -> f64 + Send + Sync;

/// Enum that holds a function reference.
/// Either a static one, or a dynamic one.
#[derive(Clone)]
pub enum FnPointer {
    /// A static function reference.
    Static(fn(&[f64]) -> f64),
    /// A dynamic function reference.
    Dyn(Arc<DynFn>),
}

impl fmt::Debug for FnPointer {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Static(_) => write!(fmt, "Static"),
            Self::Dyn(_) => write!(fmt, "Dyn"),
        }
    }
}

impl Deref for FnPointer {
    type Target = dyn Fn(&[f64]) -> f64;

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Static(ref func) => func,
            Self::Dyn(ref func) => func.as_ref(),
        }
    }
}

impl PartialEq for FnPointer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Self::Static(func1) => match *other {
                Self::Static(func2) => func1 == func2,
                Self::Dyn(_) => false,
            },
            Self::Dyn(ref func1) => match *other {
                Self::Dyn(ref func2) => Arc::ptr_eq(func1, func2),
                Self::Static(_) => false,
            },
        }
    }
}

/// Macro for defining functions for xprs' context easily, with optional variadic support.
/// This macro is provided for convenience, since [`crate::Function`] needs a fn taking a slice of [`f64`] as argument.
/// The macro will wrap your function in a closure depending on the number of arguments you provide.
/// Be aware that the provided function gets moved into the closure, so if you want to use it again, you'll have to clone it.
///
/// Don't provide the number of arguments if your function is variadic (takes any number of arguments).
#[macro_export]
macro_rules! xprs_fn {
    // variadics
    ($name:expr, $function:expr) => {
        $crate::Function::new_static($name, $function, None)
    };
    ($name:expr, dyn $function:expr) => {
        $crate::Function::new_dyn($name, $function, None)
    };
    ($function:expr) => {
        $crate::Function::new_static(stringify!($function), $function, None)
    };
    (dyn $function:expr) => {
        $crate::Function::new_dyn(stringify!($function), $function, None)
    };
    // fixed args
    ($name:expr, $function:expr, $nb_args:tt) => {
        $crate::Function::new_static(
            $name,
            $crate::xprs_fn!(wrap $function, $nb_args),
            Some($nb_args),
        )
    };
    ($name:expr, dyn $function:expr, $nb_args:tt) => {
        $crate::Function::new_dyn(
            $name,
            $crate::xprs_fn!(wrap $function, $nb_args),
            Some($nb_args),
        )
    };
    ($function:expr, $nb_args:tt) => {
        $crate::Function::new_static(
            stringify!($function),
            $crate::xprs_fn!(wrap $function, $nb_args),
            Some($nb_args),
        )
    };
    (dyn $function:expr, $nb_args:tt) => {
        $crate::Function::new_dyn(
            stringify!($function),
            $crate::xprs_fn!(wrap $function, $nb_args),
            Some($nb_args),
        )
    };

    //// closure wrapping ////
    (wrap $function:expr, 0) => {
        move |_| $function()
    };
    (wrap $function:expr, 1) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0])
    };
    (wrap $function:expr, 2) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1])
    };
    (wrap $function:expr, 3) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1], args[2])
    };
    (wrap $function:expr, 4) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1], args[2], args[3])
    };
    (wrap $function:expr, 5) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1], args[2], args[3], args[4])
    };
    (wrap $function:expr, 6) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1], args[2], args[3], args[4], args[5])
    };
    (wrap $function:expr, 7) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1], args[2], args[3], args[4], args[5], args[6])
    };
    (wrap $function:expr, 8) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7])
    };
    (wrap $function:expr, 9) => {
        #[allow(clippy::indexing_slicing, clippy::missing_asserts_for_indexing)]
        move |args| $function(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8])
    };
}
