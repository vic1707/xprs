//! Public API behavior, shared across all supported feature combinations.
#![cfg(test)]
#![allow(
    clippy::default_numeric_fallback,
    reason = "rstest case numbers get their types from the test signatures"
)]
mod common;
mod context;
mod display;
mod evaluation;
mod folding;
mod numbers;
mod parsing;
mod thread_safety;

#[test]
#[should_panic(expected = "assertion failed")]
fn float_assertion_rejects_different_values() {
    common::assert_f64_eq!(1.0, 2.0);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn float_assertion_rejects_finite_and_infinite() {
    common::assert_f64_eq!(1.0, f64::INFINITY);
}
