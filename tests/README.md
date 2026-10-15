# Test strategy

- `lib.rs` is one integration-test binary, organized by public behavior.
- Use named `rstest` cases for grammar/value tables; ordinary tests for stateful scenarios.
- `src/tests/` keeps private allocation, math-helper, and auto-trait invariants.
- CI runs both precedence rules with folding and `miette` independently enabled/disabled.
- Compare finite transcendental results with tolerance. Check NaN explicitly and signed zero, infinities, and subnormals by bits.
- Test our scientific-notation grammar and regressions, rather than duplicating `lexical`'s conversion suite.

```sh
cargo test
cargo test --no-default-features --features pejmdas
cargo test --features miette
cargo +nightly miri test --tests
```
