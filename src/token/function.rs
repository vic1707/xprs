/* Built-in imports */
extern crate alloc;
use crate::EvalError;
use alloc::sync::Arc;
use core::{cmp::Ordering, fmt, ptr::fn_addr_eq};

/// Lazy, fallible arguments for an explicitly streaming variadic function.
///
/// Consuming an item evaluates that argument. Unconsumed arguments are skipped;
/// callbacks are responsible for propagating errors from consumed items.
/// A declared fixed arity is validated before the callback starts.
pub type Args<'args> =
    &'args mut dyn ExactSizeIterator<Item = Result<f64, EvalError>>;

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
    /// Eager callbacks use a slice so fixed and variadic functions share one signature.
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
    /// Eager callbacks use a slice so fixed and variadic functions share one signature.
    #[inline]
    pub fn new_dyn(
        name: &'static str,
        func: impl Fn(&[f64]) -> f64 + Send + Sync + 'static,
        nb_args: Option<u8>,
    ) -> Self {
        Self {
            name,
            func: FnPointer::Dyn(Arc::new(func)),
            nb_args,
        }
    }

    /// Creates a lazy streaming variadic function from a static callback.
    ///
    /// Unlike slice callbacks, streaming callbacks run before all arguments are
    /// evaluated and may stop early. Constant calls may still run during folding.
    #[inline]
    #[must_use]
    pub const fn new_stream(
        name: &'static str,
        func: fn(Args<'_>) -> Result<f64, EvalError>,
    ) -> Self {
        Self {
            name,
            func: FnPointer::Stream(func),
            nb_args: None,
        }
    }

    /// Creates a lazy streaming variadic function that can capture owned state.
    #[inline]
    pub fn new_dyn_stream(
        name: &'static str,
        func: impl Fn(Args<'_>) -> Result<f64, EvalError> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name,
            func: FnPointer::DynStream(Arc::new(func)),
            nb_args: None,
        }
    }

    /// Calls either an eager slice callback or a lazy streaming callback.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::InvalidArgumentCount`] before consuming arguments if
    /// their count differs from a declared fixed arity. Otherwise returns argument
    /// evaluation errors for eager callbacks, or errors propagated by a streaming
    /// callback. Unconsumed streaming arguments are skipped.
    #[inline]
    pub fn call(
        &self,
        args: impl ExactSizeIterator<Item = Result<f64, EvalError>>,
    ) -> Result<f64, EvalError> {
        if let Some(expected) = self.nb_args {
            let actual = args.len();
            if actual != usize::from(expected) {
                return Err(EvalError::InvalidArgumentCount {
                    function: self.name,
                    expected,
                    actual,
                });
            }
        }
        self.func.call(args)
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

/// A capturing streaming callback.
type DynStreamFn = dyn Fn(Args<'_>) -> Result<f64, EvalError> + Send + Sync;

/// Enum that holds a function reference.
/// Either a static one, or a dynamic one.
#[derive(Clone)]
pub enum FnPointer {
    /// A static function reference.
    Static(fn(&[f64]) -> f64),
    /// A dynamic function reference.
    Dyn(Arc<DynFn>),
    /// A static streaming variadic function.
    Stream(fn(Args<'_>) -> Result<f64, EvalError>),
    /// A capturing streaming variadic function.
    DynStream(Arc<DynStreamFn>),
}

impl fmt::Debug for FnPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Static(_) => write!(f, "Static"),
            Self::Dyn(_) => write!(f, "Dyn"),
            Self::Stream(_) => write!(f, "Stream"),
            Self::DynStream(_) => write!(f, "DynStream"),
        }
    }
}

impl FnPointer {
    /// Dispatches without buffering streaming arguments.
    fn call(
        &self,
        mut args: impl ExactSizeIterator<Item = Result<f64, EvalError>>,
    ) -> Result<f64, EvalError> {
        match *self {
            Self::Static(func) => Self::call_eager(args, func),
            Self::Dyn(ref func) => Self::call_eager(args, func.as_ref()),
            Self::Stream(func) => func(&mut args),
            Self::DynStream(ref func) => func(&mut args),
        }
    }

    /// Evaluates every argument before invoking an eager callback.
    fn call_eager(
        args: impl ExactSizeIterator<Item = Result<f64, EvalError>>,
        func: impl Fn(&[f64]) -> f64,
    ) -> Result<f64, EvalError> {
        // ponytail: fixed arities fit on the stack; arbitrary slice callbacks need a buffer.
        let mut small = [0.0_f64; 9];
        if let Some(values) = small.get_mut(..args.len()) {
            for (value, arg) in values.iter_mut().zip(args) {
                *value = arg?;
            }
            Ok(func(values))
        } else {
            let mut values = Vec::with_capacity(args.len());
            for arg in args {
                values.push(arg?);
            }
            Ok(func(&values))
        }
    }
}

impl PartialEq for FnPointer {
    #[inline]
    #[expect(
        clippy::needless_borrowed_reference,
        reason = "explicit reference patterns satisfy pattern_type_mismatch without moving shared callbacks"
    )]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Static(left), &Self::Static(right)) => {
                fn_addr_eq(left, right)
            },
            (&Self::Dyn(ref left), &Self::Dyn(ref right)) => {
                Arc::ptr_eq(left, right)
            },
            (&Self::Stream(left), &Self::Stream(right)) => {
                fn_addr_eq(left, right)
            },
            (&Self::DynStream(ref left), &Self::DynStream(ref right)) => {
                Arc::ptr_eq(left, right)
            },
            (
                &Self::Static(_),
                &(Self::Dyn(_) | Self::Stream(_) | Self::DynStream(_)),
            )
            | (
                &Self::Dyn(_),
                &(Self::Static(_) | Self::Stream(_) | Self::DynStream(_)),
            )
            | (
                &Self::Stream(_),
                &(Self::Static(_) | Self::Dyn(_) | Self::DynStream(_)),
            )
            | (
                &Self::DynStream(_),
                &(Self::Static(_) | Self::Dyn(_) | Self::Stream(_)),
            ) => false,
        }
    }
}

/// Macro for defining functions for xprs' context easily, with optional variadic support.
/// Fixed-arity definitions wrap scalar parameters in an eager slice callback.
/// Variadics can use an eager slice or a lazy, fallible [`Args`] stream.
/// Be aware that the provided function gets moved into the closure, so if you want to use it again, you'll have to clone it.
///
/// Don't provide the number of arguments if your function is variadic (takes any number of arguments).
/// Named definitions infer arity and capture by move: `xprs_fn!(double(x): 2.0 * x)`.
/// Use `xprs_fn!(sum(args: &[f64]): args.iter().sum())` for variadic definitions.
/// Use `xprs_fn!(sum(args: xprs::Args<'_>): args.sum())` for a fallible streaming definition.
/// Prefix named definitions with `static` for allocation-free, const-safe callbacks.
#[macro_export]
macro_rules! xprs_fn {
    (@one $_arg:ident) => { 1_u8 };
    (@scalar [$($arg:ident),*] $body:expr) => {
        move |args: &[f64]| {
            let &[$($arg),*] = args else {
                panic!("incorrect argument count for xprs function");
            };
            $body
        }
    };
    (@wrap $function:expr; $($arg:ident),*) => {
        move |args: &[f64]| {
            let &[$($arg),*] = args else {
                panic!("incorrect argument count for xprs function");
            };
            ($function)($($arg),*)
        }
    };
    (@wrap $function:expr, 0) => { $crate::xprs_fn!(@wrap $function;) };
    (@wrap $function:expr, 1) => { $crate::xprs_fn!(@wrap $function; arg0) };
    (@wrap $function:expr, 2) => { $crate::xprs_fn!(@wrap $function; arg0, arg1) };
    (@wrap $function:expr, 3) => { $crate::xprs_fn!(@wrap $function; arg0, arg1, arg2) };
    (@wrap $function:expr, 4) => { $crate::xprs_fn!(@wrap $function; arg0, arg1, arg2, arg3) };
    (@wrap $function:expr, 5) => { $crate::xprs_fn!(@wrap $function; arg0, arg1, arg2, arg3, arg4) };
    (@wrap $function:expr, 6) => { $crate::xprs_fn!(@wrap $function; arg0, arg1, arg2, arg3, arg4, arg5) };
    (@wrap $function:expr, 7) => { $crate::xprs_fn!(@wrap $function; arg0, arg1, arg2, arg3, arg4, arg5, arg6) };
    (@wrap $function:expr, 8) => { $crate::xprs_fn!(@wrap $function; arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) };
    (@wrap $function:expr, 9) => { $crate::xprs_fn!(@wrap $function; arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) };
    (static $name:ident($arg:ident: &[f64]): $body:expr) => {
        $crate::Function::new_static(stringify!($name), |$arg: &[f64]| $body, None)
    };
    (static $name:ident($arg:ident: $args:ty): $body:expr) => {
        $crate::Function::new_stream(stringify!($name), |$arg: $args| $body)
    };
    (static $name:ident($($arg:ident),* $(,)?): $body:expr) => {{
        const ARITY: u8 = 0_u8 $(+ $crate::xprs_fn!(@one $arg))*;
        $crate::Function::new_static(stringify!($name), $crate::xprs_fn!(@scalar [$($arg),*] $body), Some(ARITY))
    }};
    ($name:ident($arg:ident: &[f64]): $body:expr) => {
        $crate::Function::new_dyn(stringify!($name), move |$arg: &[f64]| $body, None)
    };
    ($name:ident($arg:ident: $args:ty): $body:expr) => {
        $crate::Function::new_dyn_stream(stringify!($name), move |$arg: $args| $body)
    };
    ($name:ident($($arg:ident),* $(,)?): $body:expr) => {{
        const ARITY: u8 = 0_u8 $(+ $crate::xprs_fn!(@one $arg))*;
        $crate::Function::new_dyn(stringify!($name), $crate::xprs_fn!(@scalar [$($arg),*] $body), Some(ARITY))
    }};
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
            $crate::xprs_fn!(@wrap $function, $nb_args),
            Some($nb_args),
        )
    };
    ($name:expr, dyn $function:expr, $nb_args:tt) => {{
        let function = $function;
        $crate::Function::new_dyn(
            $name,
            $crate::xprs_fn!(@wrap function, $nb_args),
            Some($nb_args),
        )
    }};
    ($function:expr, $nb_args:tt) => {
        $crate::Function::new_static(
            stringify!($function),
            $crate::xprs_fn!(@wrap $function, $nb_args),
            Some($nb_args),
        )
    };
    (dyn $function:expr, $nb_args:tt) => {{
        let function = $function;
        $crate::Function::new_dyn(
            stringify!($function),
            $crate::xprs_fn!(@wrap function, $nb_args),
            Some($nb_args),
        )
    }};

}
