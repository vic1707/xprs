#[link(name = "m")]
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
#[inline]
#[cfg(NIGHTLY)]
pub fn gamma(num: f64) -> f64 {
    (num + 1.).gamma()
}

#[doc(hidden)]
pub fn factorial(num: f64) -> f64 {
    gamma(num + 1.)
}

