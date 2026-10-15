//! Public API behavior, shared across all supported feature combinations.
#![cfg(test)]
#![allow(
    clippy::default_numeric_fallback,
    reason = "rstest case numbers get their types from the test signatures"
)]
mod common;
mod context;
mod evaluation;
mod folding;
mod numbers;
mod parsing;
