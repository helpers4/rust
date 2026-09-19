# helpers4 — Rust

General-purpose Rust helpers: **one crate, one module per category** (`string`, `array`, …),
each behind its own Cargo feature.

```toml
[dependencies]
helpers4 = "0"                                                        # every module
# or only what you use:
helpers4 = { version = "0", default-features = false, features = ["string"] }
```

```rust
use helpers4::string::capitalize;

assert_eq!(capitalize("hello"), "Hello");
```

Helper names can repeat across modules (`array::compact`, `object::compact`); always go
through the module path.

## Quality bar

- 100% coverage (lines, functions, regions), enforced in CI
- Unit tests, property-based tests (proptest), doctests, benchmarks (criterion) where relevant
- `cargo-deny` on licenses and advisories, clippy with warnings denied
- Zero third-party dependencies by default

## Status

Pre-release (`0.0.0`). Sister project of [helpers4/typescript](https://github.com/helpers4/typescript).
See the [roadmap](https://github.com/orgs/helpers4/projects/1) and [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[LGPL-3.0-or-later](LICENSE)
