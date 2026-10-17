/* Crate imports */
use super::factorial::{factorial, gamma};
use crate::{Args, token::Function, xprs_fn};

/// Sine builtin function.
pub const SIN: Function = xprs_fn!(static sin(value): value.sin());

/// Hyperbolic sine builtin function.
pub const SINH: Function = xprs_fn!(static sinh(value): value.sinh());

/// Arcsine builtin function.
pub const ASIN: Function = xprs_fn!(static asin(value): value.asin());

/// Inverse hyperbolic sine builtin function.
pub const ASINH: Function = xprs_fn!(static asinh(value): value.asinh());

/// Cosine builtin function.
pub const COS: Function = xprs_fn!(static cos(value): value.cos());

/// Hyperbolic cosine builtin function.
pub const COSH: Function = xprs_fn!(static cosh(value): value.cosh());

/// Arccosine builtin function.
pub const ACOS: Function = xprs_fn!(static acos(value): value.acos());

/// Inverse hyperbolic cosine builtin function.
pub const ACOSH: Function = xprs_fn!(static acosh(value): value.acosh());

/// Tangent builtin function.
pub const TAN: Function = xprs_fn!(static tan(value): value.tan());

/// Hyperbolic tangent builtin function.
pub const TANH: Function = xprs_fn!(static tanh(value): value.tanh());

/// Arctangent builtin function.
pub const ATAN: Function = xprs_fn!(static atan(value): value.atan());

/// Arctangent function with two arguments.
pub const ATAN2: Function =
    xprs_fn!(static atan2(left, right): left.atan2(right));

/// Inverse hyperbolic tangent builtin function.
pub const ATANH: Function = xprs_fn!(static atanh(value): value.atanh());

/// Natural logarithm builtin function.
pub const LN: Function = xprs_fn!(static ln(value): value.ln());

/// Base-10 logarithm builtin function.
pub const LOG: Function = xprs_fn!(static log(value): value.log10());

/// Logarithm function with a specified base.
pub const LOGN: Function = xprs_fn!(static logn(value, base): value.log(base));

/// Square root builtin function.
pub const SQRT: Function = xprs_fn!(static sqrt(value): value.sqrt());

/// Cube root builtin function.
pub const CBRT: Function = xprs_fn!(static cbrt(value): value.cbrt());

/// Exponential builtin function.
pub const EXP: Function = xprs_fn!(static exp(value): value.exp());

/// Absolute value builtin function.
pub const ABS: Function = xprs_fn!(static abs(value): value.abs());

/// Floor builtin function.
pub const FLOOR: Function = xprs_fn!(static floor(value): value.floor());

/// Ceiling builtin function.
pub const CEIL: Function = xprs_fn!(static ceil(value): value.ceil());

/// Round to the nearest integer builtin function.
pub const ROUND: Function = xprs_fn!(static round(value): value.round());

/// Truncate decimal part builtin function.
pub const TRUNC: Function = xprs_fn!(static trunc(value): value.trunc());

/// Sum of a list of numbers.
pub const SUM: Function = xprs_fn!(static sum(args: Args<'_>): args.sum());

/// Mean (average) of a list of numbers.
#[expect(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    reason = "the mean divides a floating-point sum by the argument count"
)]
pub const MEAN: Function = xprs_fn!(static mean(args: Args<'_>): {
    let count = args.len();
    args.sum::<Result<f64, _>>().map(|sum| sum / count as f64)
});

/// Reciprocal builtin function.
pub const RECIP: Function = xprs_fn!(static recip(value): value.recip());

/// Minimum value in a list of numbers.
pub const MIN: Function = xprs_fn!(static min(args: Args<'_>): {
    let mut result = f64::INFINITY;
    for value in args { result = result.min(value?); }
    Ok(result)
});

/// Maximum value in a list of numbers.
pub const MAX: Function = xprs_fn!(static max(args: Args<'_>): {
    let mut result = f64::NEG_INFINITY;
    for value in args { result = result.max(value?); }
    Ok(result)
});

/// Euclidean distance (hypotenuse) builtin function.
pub const HYPOT: Function =
    xprs_fn!(static hypot(left, right): left.hypot(right));

/// Fractional part of a number.
pub const FRACT: Function = xprs_fn!(static fract(value): value.fract());

/// Factorial builtin function.
pub const FACTORIAL: Function =
    xprs_fn!(static factorial(value): factorial(value));

/// Gamma builtin function.
pub const GAMMA: Function = xprs_fn!(static gamma(value): gamma(value));
