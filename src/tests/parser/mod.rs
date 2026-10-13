/* Modules */
#[cfg(feature = "compile-time-optimizations")]
mod comptime_optimizations;
#[cfg(not(feature = "compile-time-optimizations"))]
mod ctx;
mod error;
mod implicit_multiplication;
mod numbers;
#[cfg(not(feature = "compile-time-optimizations"))]
mod valid;
