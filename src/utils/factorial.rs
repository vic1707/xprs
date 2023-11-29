/* Crate imports */
use crate::utils::hidden_macros::trust_me;

extern "C" {
    fn tgamma(x: f64) -> f64;
}

#[doc(hidden)]
#[inline]
#[cfg(not(NIGHTLY))]
pub fn gamma(x: f64) -> f64 {
    unsafe { tgamma(x) }
}

#[doc(hidden)]
#[cfg(NIGHTLY)]
pub const gamma: fn(f64) -> f64 = f64::gamma;

#[doc(hidden)]
pub fn factorial(num: f64) -> f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
    const MAX: f64 = 170.0;

    if num < ZERO {
        return f64::NAN;
    }

    if num.is_infinite() {
        return f64::INFINITY;
    }

    if num.fract() != ZERO {
        return f64::NAN;
    }

    // is enough to max out a f64
    if num > MAX {
        return f64::INFINITY;
    }

    // will only be called if num is a whole number
    // and 1 <= num <= 170
    let upper_bound = trust_me!(num.to_int_unchecked::<u8>());

    (2..=upper_bound).fold(ONE, |acc, x| acc * f64::from(x))
}
