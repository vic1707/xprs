# Test strategy

- `lib.rs` is one integration-test binary, organized by public behavior.
- Use named `rstest` cases for grammar/value tables; ordinary tests for stateful scenarios.
- `src/tests/` keeps small math-helper checks. Public `Send + Sync` guarantees are checked in the integration suite.
- Folding cases check the reduced expression, remaining variables, and value preservation on ordinary inputs; display has its own small table.
- Symbolic folding follows algebraic identities, not strict IEEE-754 equivalence at singularities or for non-finite values. Literal arithmetic retains `f64` behavior.
- Keep distinct behavior, meaningful boundaries, and known regressions; allocation layout is not a correctness contract.
- CI runs both precedence rules with folding and `miette` independently enabled/disabled.
- Compare finite transcendental results with tolerance. Check NaN explicitly and signed zero, infinities, and subnormals by bits.
- Test our scientific-notation grammar and regressions, rather than duplicating `lexical`'s conversion suite.

```sh
cargo test
cargo test --no-default-features --features pejmdas
cargo test --features miette
cargo +nightly miri test --tests
```
