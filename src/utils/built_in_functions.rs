/* Crate imports */
use crate::{token::Function, xprs_fn};

/// Sine builtin functions.
pub const SIN: Function = xprs_fn!("sin", f64::sin, 1);

/// Hyperbolic sine builtin functions.
pub const SINH: Function = xprs_fn!("sinh", f64::sinh, 1);

/// Arcsine builtin functions.
pub const ASIN: Function = xprs_fn!("asin", f64::asin, 1);

/// Inverse hyperbolic sine builtin functions.
pub const ASINH: Function = xprs_fn!("asinh", f64::asinh, 1);

/// Cosine builtin functions.
pub const COS: Function = xprs_fn!("cos", f64::cos, 1);

/// Hyperbolic cosine builtin functions.
pub const COSH: Function = xprs_fn!("cosh", f64::cosh, 1);

/// Arccosine builtin functions.
pub const ACOS: Function = xprs_fn!("acos", f64::acos, 1);

/// Inverse hyperbolic cosine builtin functions.
pub const ACOSH: Function = xprs_fn!("acosh", f64::acosh, 1);

/// Tangent builtin functions.
pub const TAN: Function = xprs_fn!("tan", f64::tan, 1);

/// Hyperbolic tangent builtin functions.
pub const TANH: Function = xprs_fn!("tanh", f64::tanh, 1);

/// Arctangent builtin functions.
pub const ATAN: Function = xprs_fn!("atan", f64::atan, 1);

/// Arctangent function with two arguments.
pub const ATAN2: Function = xprs_fn!("atan2", f64::atan2, 2);

/// Inverse hyperbolic tangent builtin functions.
pub const ATANH: Function = xprs_fn!("atanh", f64::atanh, 1);

/// Natural logarithm builtin functions.
pub const LN: Function = xprs_fn!("ln", f64::ln, 1);

/// Base-10 logarithm builtin functions.
pub const LOG: Function = xprs_fn!("log", f64::log10, 1);

/// Logarithm function with a specified base.
pub const LOGN: Function = xprs_fn!("logn", f64::log, 2);

/// Square root builtin functions.
pub const SQRT: Function = xprs_fn!("sqrt", f64::sqrt, 1);

/// Cube root builtin functions.
pub const CBRT: Function = xprs_fn!("cbrt", f64::cbrt, 1);

/// Exponential builtin functions.
pub const EXP: Function = xprs_fn!("exp", f64::exp, 1);

/// Absolute value builtin functions.
pub const ABS: Function = xprs_fn!("abs", f64::abs, 1);

/// Floor builtin functions.
pub const FLOOR: Function = xprs_fn!("floor", f64::floor, 1);

/// Ceiling builtin functions.
pub const CEIL: Function = xprs_fn!("ceil", f64::ceil, 1);

/// Round to the nearest integer builtin functions.
pub const ROUND: Function = xprs_fn!("round", f64::round, 1);

/// Truncate decimal part builtin functions.
pub const TRUNC: Function = xprs_fn!("trunc", f64::trunc, 1);

/// Sum of a list of numbers.
pub const SUM: Function = xprs_fn!("sum", |args| args.iter().sum());

/// Mean (average) of a list of numbers.
#[allow(clippy::as_conversions, clippy::cast_precision_loss)]
pub const MEAN: Function = xprs_fn!("mean", |args| {
    args.iter().sum::<f64>() / args.len() as f64
});

/// Reciprocal builtin functions.
pub const INVERT: Function = xprs_fn!("invert", f64::recip, 1);

/// Minimum value in a list of numbers.
pub const MIN: Function = xprs_fn!("min", |args| {
    args.iter().fold(f64::INFINITY, |acc, &x| acc.min(x))
});

/// Maximum value in a list of numbers.
pub const MAX: Function = xprs_fn!("max", |args| {
    args.iter().fold(f64::NEG_INFINITY, |acc, &x| acc.max(x))
});

/// Euclidean distance (hypotenuse) builtin functions.
pub const HYPOT: Function = xprs_fn!("hypot", f64::hypot, 2);

/// Fractional part of a number.
pub const FRACT: Function = xprs_fn!("fract", f64::fract, 1);
