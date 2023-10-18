/* Modules */
#[cfg(not(feature = "compile-time-optimizations"))]
mod ctx;
mod error;
mod implicit_multiplication;
#[cfg(not(feature = "compile-time-optimizations"))]
mod valid;
#[cfg(feature = "compile-time-optimizations")]
mod comptime_optimizations;
