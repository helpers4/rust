<h1 align="center">helpers4 — Rust</h1>

<p align="center">
  <strong>General-purpose Rust helpers: one crate, one module per category, zero dependencies.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/helpers4"><img src="https://img.shields.io/crates/v/helpers4?label=crates.io" alt="crates.io version" /></a>
  <a href="https://crates.io/crates/helpers4"><img src="https://img.shields.io/crates/d/helpers4?color=blue&label=downloads" alt="crates.io downloads" /></a>
  <a href="https://docs.rs/helpers4"><img src="https://img.shields.io/docsrs/helpers4?label=docs.rs" alt="docs.rs" /></a>
  <br>
  <a href="LICENSE"><img src="https://img.shields.io/crates/l/helpers4?color=blue" alt="license" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/helpers4/rust"><img src="https://api.securityscorecards.dev/projects/github.com/helpers4/rust/badge" alt="OpenSSF Scorecard" /></a>
  <img src="https://img.shields.io/crates/msrv/helpers4?label=MSRV" alt="minimum supported Rust version" />
  <img src="https://img.shields.io/badge/unsafe-forbidden-blue" alt="unsafe code forbidden" />
  <a href="CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs welcome" /></a>
  <br>
  <a href="https://codecov.io/gh/helpers4/rust"><img src="https://codecov.io/gh/helpers4/rust/graph/badge.svg?token=3np3gHErgz" alt="code coverage" /></a>
</p>

---

> **Pre-1.0 (`0.x`).** While the crate is at version 0, how the helpers are split into modules —
> and therefore into Cargo features — may change between releases, and the code-quality and
> security checks will keep improving. **Do not use it in production for critical or sensitive
> applications until 1.0**, which waits for independent security audits. Pin the exact version and
> read the [changelog](CHANGELOG.md) before upgrading.

## Overview

`helpers4` is a collection of small, typed helpers for the everyday code every project ends up
rewriting: string casing, slice operations, expiring caches, dates, URLs, versions, hex, network checks, and more. Each
helper does one thing, returns a typed error instead of panicking, and ships with tests and a
runnable example.

## Modules

One module per category, each behind a Cargo feature of the same name (all enabled by default):

| Module | What it covers |
| ------ | -------------- |
| [`ansi`](https://helpers4.dev/rust/modules/ansi/) | stripping and building ANSI escape sequences (colors, styles) |
| [`array`](https://helpers4.dev/rust/modules/array/) | unique, difference, intersection, grouping, counting, duplicates, interleaving |
| [`bytes`](https://helpers4.dev/rust/modules/bytes/) | integers at an offset, byte search, XOR, constant-time comparison, `1.5 KiB` sizes |
| [`cache`](https://helpers4.dev/rust/modules/cache/) | maps and sets whose entries expire, with the clock passed in |
| [`ci`](https://helpers4.dev/rust/modules/ci/) | which CI service is this, is it a pull request, and a Markdown status report |
| [`color`](https://helpers4.dev/rust/modules/color/) | `#rgb`/`rgb()`/names, HSL, blending and WCAG contrast |
| [`commit`](https://helpers4.dev/rust/modules/commit/) | Conventional Commits: parsing, breaking changes and the version bump they call for |
| [`date`](https://helpers4.dev/rust/modules/date/) | calendar dates without time zones: ISO 8601, weekdays, day arithmetic |
| [`duration`](https://helpers4.dev/rust/modules/duration/) | parsing and formatting durations as `1h30m` |
| [`env`](https://helpers4.dev/rust/modules/env/) | reading and editing `.env` files as text |
| [`fs`](https://helpers4.dev/rust/modules/fs/) | atomic writes, missing file as `None`, listing a tree, lexical path checks |
| [`function`](https://helpers4.dev/rust/modules/function/) | composition, memoization, retrying with backoff, a token-bucket rate limiter |
| [`future`](https://helpers4.dev/rust/modules/future/) | `block_on`, `join`, `join_all` and friends, with no async runtime |
| [`hex`](https://helpers4.dev/rust/modules/hex/) | hexadecimal encoding and decoding with typed errors |
| [`http`](https://helpers4.dev/rust/modules/http/) | extracting a bearer token from an `Authorization` header |
| [`iter`](https://helpers4.dev/rust/modules/iter/) | chunking, one-pass min/max, first duplicate: for any `Iterator` |
| [`license`](https://helpers4.dev/rust/modules/license/) | SPDX identifiers and expressions, `MIT OR Apache-2.0` checked against a policy |
| [`map`](https://helpers4.dev/rust/modules/map/) | picking, omitting and transforming the entries of a `HashMap` |
| [`markdown`](https://helpers4.dev/rust/modules/markdown/) | escaping, links, code blocks, quotes, tables and heading anchors |
| [`net`](https://helpers4.dev/rust/modules/net/) | is this IP address public (an SSRF guard), is this a valid hostname |
| [`number`](https://helpers4.dev/rust/modules/number/) | interpolation, rounding, mean, median, percentages, gcd and lcm |
| [`secret`](https://helpers4.dev/rust/modules/secret/) | a wrapper that never prints its value, redaction, masking, token detection |
| [`set`](https://helpers4.dev/rust/modules/set/) | union and intersection of many sets, toggling, sorted output, similarity, power set |
| [`string`](https://helpers4.dev/rust/modules/string/) | case conversion, slugs, truncation, indentation, whitespace, HTML escaping |
| [`time`](https://helpers4.dev/rust/modules/time/) | the system clock as unix time, without a silent `0` |
| [`url`](https://helpers4.dev/rust/modules/url/) | an RFC 3986 parser with reference resolution, percent-encoding, query strings |
| [`validate`](https://helpers4.dev/rust/modules/validate/) | shape checks: email, UUID, slug |
| [`version`](https://helpers4.dev/rust/modules/version/) | semantic versions, Cargo-style requirements, comparison |

More are planned (random values and identifiers, …): see the [roadmap](https://github.com/orgs/helpers4/projects/1).

## Quick start

```sh
cargo add helpers4                                              # every module
cargo add helpers4 --no-default-features --features string,hex  # only what you use
```

```rust
use helpers4::hex;
use helpers4::net::is_public_ip;
use helpers4::string::slugify;

assert_eq!(slugify("Hello, World!"), "hello-world");
assert_eq!(hex::encode(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
assert!(!is_public_ip("169.254.169.254".parse().unwrap())); // the cloud metadata address
```

Names can repeat across modules, so always go through the module path
(`helpers4::string::capitalize`) and never glob-import a module.

## Key features

- **Zero dependencies** by default: a module that needs one gets its own optional feature
- **Take only what you use**: one Cargo feature per module, so unused modules are not compiled
- **Typed errors instead of panics**: `unsafe` is forbidden, and `unwrap`, `expect` and `panic!`
  are denied outside tests
- **Explicit inputs**: the clock and the process environment are parameters, never ambient
- **Standard library first**: nothing here duplicates what `std` already offers
- **Fully tested**: 100% coverage of lines, functions and regions, unit tests, property-based
  tests, doc tests, mutation testing and benchmarks, all enforced or tracked in CI
- **AI-ready**: [`llms.txt`](llms.txt) describes every module, and every example is a doc test

## Documentation

Every helper, with its signature, parameters, errors and examples, is on
**[helpers4.dev/rust](https://helpers4.dev/rust/)** and on [docs.rs](https://docs.rs/helpers4).

## Development

```sh
cargo test --all-features                       # unit, property and doc tests
cargo clippy --all-targets --all-features -- -D warnings
cargo llvm-cov --all-features --ignore-filename-regex '\.(test|spec|bench)\.rs$'
cargo bench --all-features
```

See [AGENTS.md](AGENTS.md) for the layout and rules, and [CONTRIBUTING.md](CONTRIBUTING.md) for
how to add a helper or a module.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and the organization's
[Contributing Guide](https://github.com/helpers4/.github/blob/main/CONTRIBUTING.md).

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/)
4. Push to the branch and open a Pull Request

To report a vulnerability, see [SECURITY.md](SECURITY.md).

## License

Licensed under the [GNU Lesser General Public License v3.0 or later](LICENSE): you can freely use
it in proprietary or open-source projects.
