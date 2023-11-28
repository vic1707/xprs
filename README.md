# Xprs

[<img alt="github" src="https://img.shields.io/badge/github-vic1707/xprs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/vic1707/xprs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/xprs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/xprs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-xprs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/xprs)
[<img alt="downloads" src="https://img.shields.io/crates/d/xprs.svg?style=for-the-badge&logo=docs.rs" height="20">](https://crates.io/crates/xprs)

**Xprs** is a flexible and extensible mathematical expression parser and evaluator for Rust, designed for simplicity and ease of use (and ideally, speed).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xprs = "0.0.1-beta1"
```

or run this command in your terminal:

```bash
cargo add xprs
```

Make sure to check the [Crates.io](https://crates.io/crates/xprs) page for the latest version.

## MSRV (Minimum Supported Rust Version)

Currently, the minimum supported Rust version is `1.70.0`.

## Crate Features

- **`compile-time-optimizations`** _(enabled by default)_ :

  Enable optimization and evaluation during parsing.
  This feature will automagically transform expressions like `1 + 2 * 3` into `7` during parsing allowing for faster evaluation.
  It also works on functions (e.g. `sin(0)` will be transformed into `0`) and "logical" result like `(x - x) * (....)` will be transformed into `0` since `x - x` is `0` no matter what `x` is.

  Note: nightly channel enables even more optimizations thanks to `box_patterns` feature gate.

<br />

- **`pemdas`** _(enabled by default)_:

  Conflicts with the `pejmdas` feature.
  Uses the PEMDAS order of operations.
  This implies that implicit multiplication has the same precedence as explicit multiplication.
  For example:

  - `6/2(2+1)` gets interpreted as `6/2*(2+1)` which gives `9` as a result.
  - `1/2x` gets interpreted as `(1/2)*x` which, with `x` being `2`, gives `1` as a result.

  Note: `Display` and `Debug` shows additionnal parenthesis to make the order of operations more obvious.

<br />

- **`pejmdas`**:

  Conflicts with the `pemdas` feature.
  Uses the PEJMDAS order of operations.
  This implies that implicit multiplication has a higher precedence than explicit multiplication.
  For example:

  - `6/2(2+1)` gets interpreted as `6/(2*(2+1))` which gives `1` as a result.
  - `1/2x` gets interpreted as `1/(2*x)` which, with `x` being `2`, gives `0.25` as a result.

  Note: `Display` and `Debug` shows additionnal parenthesis to make the order of operations more obvious.

## Licence

Copyright © 2023 [Victor LEFEBVRE](contact@vic1707.xyz)
This work is free. You can redistribute it and/or modify it under the
terms of the Do What The Fuck You Want To Public License, Version 2,
as published by Sam Hocevar. See the [LICENCE](./LICENCE). file for more details.
