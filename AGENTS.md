# AGENTS.md — rust

→ [Org-wide rules](https://github.com/helpers4/.dev/blob/main/AGENTS.md): restrictions · commit format · license headers

## This Repository

**Purpose:** General-purpose Rust helpers — one crate (`helpers4`), one module per category.
**Stack:** Rust stable (edition 2024, MSRV 1.85) · cargo · cargo-llvm-cov (100%) · proptest · criterion · cargo-deny · clippy

```text
src/<module>/
├── function_name.rs        # one helper per file (snake_case): implementation + doctests
├── function_name.test.rs    # unit tests — 100% coverage of the .rs file required
├── function_name.spec.rs    # property-based tests (proptest): invariants, not branches
├── function_name.bench.rs   # optional criterion benchmark (`pub fn bench(c: &mut Criterion)`)
├── _internal.rs             # crate-private helper, not re-exported (same sibling files)
└── mod.rs                   # module docs + `pub use` of every helper
benches/<module>.rs         # one criterion target per module: pulls in each *.bench.rs
```

Sibling files are wired from the helper with
`#[cfg(test)] #[path = "function_name.test.rs"] mod tests;` (and `spec` likewise), so they keep
access to private items. Benches are wired from `benches/<module>.rs` with
`#[path = "../src/<module>/function_name.bench.rs"] mod function_name;` and listed in its
`criterion_group!`.

**Key commands** (the same checks CI runs — see `.github/workflows/README.md`):

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings   # strict [lints] table, see Cargo.toml
typos && cargo machete                                    # spelling, unused dependencies
cargo test --all-features
cargo hack check --feature-powerset --no-dev-deps --depth 2 # every pair of features builds (what CI runs on a PR; up to 3 at
                                                              # a time runs weekly, sharded — compat-full.yml)
cargo llvm-cov --all-features --ignore-filename-regex '\.(test|spec|bench)\.rs$' \
  --fail-under-lines 100 --fail-under-functions 100 --fail-under-regions 100
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features && cargo test --all-features --doc
cargo bench --all-features
cargo deny check
cargo package --locked                                     # what would be published
```

**Rules:**

- `unsafe` forbidden (`#![forbid(unsafe_code)]`); no `unwrap()`/`expect()`/`panic!` outside tests
  (denied by clippy). Pure functions are `#[must_use]`
- Rustdoc on every public item, with a runnable `# Examples` block (doctests are the smoke
  tests). No version annotation in the code: "since which version" is computed at release time
  by diffing the public API against the previous release (`cargo public-api diff`) into
  `api-since.json`, which the docs site and `llms.txt` consume — never write it by hand
- 100% coverage: lines, functions, regions — no exceptions. `*.test.rs`, `*.spec.rs` and
  `*.bench.rs` are excluded from the measurement, not held to it. Property tests (proptest,
  `*.spec.rs`) cover invariants, not branches
- Generic helpers: coverage counts every instantiation (`decode_array::<3>`, `::<32>`, …) on its
  own, so drive each type/const used in tests through both the success and the error path
- Zero third-party dependencies by default. A module needing one gets its own Cargo feature
  and the dependency is `optional = true`
- One Cargo feature per module (`default` enables all). New module ⇒ new feature, gated
  `#[cfg(feature = "...")]` in `lib.rs`, and a `required-features` bench entry
- Same helper name in two modules is fine and intentional (`array::compact` vs
  `object::compact`): callers import through the module path, never glob-import modules
- Check the standard library first — do not add a helper that duplicates it
- Breaking changes: use the real Conventional Commits `BREAKING CHANGE:` footer (own
  paragraph, literal uppercase token) — `git-cliff` (see `cliff.toml`) only detects that

**Docs for AI:** keep `llms.txt` current — when a module is added, list it there.

**License header (all source files):**

```rust
// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later
```
