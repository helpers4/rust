# helpers4 — Rust

General-purpose Rust helpers: **one crate, one module per category** (`string`, `array`, `hex`, …),
each behind its own Cargo feature.

> **Pre-1.0 (`0.x`).** While the crate is at version 0, how the helpers are split into modules —
> and therefore into Cargo features — may change between releases, and the code-quality and
> security checks will keep improving. Pin the exact version and read the
> [changelog](CHANGELOG.md) before upgrading.

## Install

```sh
cargo add helpers4                                              # every module
cargo add helpers4 --no-default-features --features string,hex  # only what you use
```

## Use

```rust
use helpers4::string::capitalize;

assert_eq!(capitalize("hello"), "Hello");
```

Names can repeat across modules, so always go through the module path
(`helpers4::string::capitalize`) and never glob-import a module.

## Documentation

Every helper, with its signature, parameters, errors and examples: <https://helpers4.dev/rust/>
and [docs.rs](https://docs.rs/helpers4).

## Quality bar

- 100% coverage (lines, functions, regions), enforced in CI
- Unit tests, property-based tests (proptest), doctests, benchmarks (criterion) where relevant
- `cargo-deny` on licenses and advisories, clippy with warnings denied
- Zero third-party dependencies by default

## Status

Pre-release (`0.0.3`). See the [roadmap](https://github.com/orgs/helpers4/projects/1) and
[CONTRIBUTING.md](CONTRIBUTING.md).

## License

[LGPL-3.0-or-later](LICENSE)
