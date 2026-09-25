# Contributing to helpers4 (Rust)

Thank you for helping. This page is the long version; [AGENTS.md](AGENTS.md) holds the layout,
the rules and the commands in a form short enough for a checklist, and
[.github/workflows/README.md](.github/workflows/README.md) explains what CI runs.

By contributing you agree that your work is licensed under the
[LGPL-3.0-or-later](LICENSE).

## Prerequisites

### Option A: the devcontainer (recommended)

Open the organization workspace in VS Code and *Reopen in Container*. It ships the Rust toolchain
(stable, plus the components in `rust-toolchain.toml`) and the tools below.

### Option B: a local clone

- Rust stable, and the **1.85** toolchain to check the minimum supported version:
  `rustup toolchain install 1.85`
- The tools CI uses, all installable with `cargo install --locked <name>`:
  `cargo-llvm-cov`, `cargo-hack`, `cargo-mutants`, `cargo-deny`, `cargo-machete`, `typos-cli`,
  `cargo-semver-checks`
- `git-cliff` and `cargo-public-api` (needs the nightly toolchain: `rustup toolchain install
  nightly`) if you prepare a release (see [Releasing](.github/workflows/README.md#releasing))
- `cargo-fuzz` (also nightly) if you touch a parser and want to fuzz it locally: `fuzz/` is a
  detached workspace, see [Fuzzing](#fuzzing)

## Project structure

One crate (`helpers4`), one module per category, one Cargo feature per module. Everything a module
needs is in `src/<module>/`:

```text
src/<module>/
├── name.rs          # ONE helper (or one type): implementation, rustdoc, doctests
├── name.test.rs     # unit tests: every branch, 100% coverage required
├── name.spec.rs     # property tests (proptest): invariants, not branches
├── name.bench.rs    # optional criterion benchmark
├── _internal.rs     # crate-private code shared by the module (not re-exported)
└── mod.rs           # module docs + `pub use` of everything public
```

The `.test.rs`, `.spec.rs` and `.bench.rs` files are wired from `name.rs` with
`#[cfg(test)] #[path = "name.test.rs"] mod tests;`, so they can see private items.

## Design rules

These come from the crate's philosophy (see the [site](https://helpers4.dev/rust/reference/philosophy/))
and CI enforces most of them.

- **Check the standard library first.** Do not add what `std` already does. The list of what it
  covers is [docs/native-alternatives.json](docs/native-alternatives.json); a helper that only wraps
  a std call has to say what it adds (a typed error, several inputs, a stable order, ...).
- **Zero third-party dependencies by default.** A module that needs one gets its own Cargo feature
  and the dependency is `optional = true`.
- **No `unsafe`, no panics.** `unsafe` is forbidden, and `unwrap`, `expect`, `panic!`, `todo!`,
  `dbg!` and printing are denied by clippy outside tests. A function that can fail returns
  `Result` (with a typed error) or `Option`; a degenerate input has a documented, sensible result.
- **Explicit inputs.** The clock, the process environment and randomness are parameters, never read
  behind the caller's back (`cache::ExpiringMap` takes `now`, `ci::detect` takes a lookup function).
- **Every module stands alone.** A module never uses another one: `cargo test --no-default-features
  --features <module>` must pass by itself. If a test needs another module, gate it on that
  module's feature (see `validate/is_slug.spec.rs`).
- **Names may repeat across modules** (`set::union_all`, `array::...`); callers import through the
  module path, so pick the plain-English name that fits the module.

## Adding a helper

### 1. Pick the module

Look at the modules in `src/` and at their `mod.rs` docs. If nothing fits, see
[Adding a module](#adding-a-module) rather than stretching an existing one.

### 2. Write the implementation: `src/<module>/<name>.rs`

Every source file starts with the license header:

```rust
// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later
```

Then the rustdoc, in this order, and the function. Every section is read by the docs generator
(the site renders `# Arguments` as the *Description* column of the parameter table):

```rust
/// Splits `iter` into consecutive chunks of `size` items, the last one possibly shorter.
///
/// One or two paragraphs: what it does, and the edge cases (empty input, a `size` of 0, ...).
///
/// # Arguments
///
/// - `iter` - The items to split.
/// - `size` - How many items go in each chunk.
///
/// # Returns
///
/// The chunks, in order.
///
/// # Errors
///
/// (Only for a `Result`.) One line per error, naming the variant.
///
/// # Examples
///
/// ```
/// use helpers4::iter::chunk;
///
/// assert_eq!(chunk(1..=5, 2), vec![vec![1, 2], vec![3, 4], vec![5]]);
/// ```
#[must_use]
pub fn chunk<I: IntoIterator>(iter: I, size: usize) -> Vec<Vec<I::Item>> {
    // ...
}

#[cfg(test)]
#[path = "chunk.test.rs"]
mod tests;

#[cfg(test)]
#[path = "chunk.spec.rs"]
mod spec;
```

Things that trip people up:

- Doctests are the smoke tests: they compile and run in CI. Use `?` with
  `# Ok::<(), helpers4::module::TheError>(())` on the last line rather than `unwrap`.
- Pure functions are `#[must_use]`. Errors are `#[non_exhaustive]` enums with a `Display`
  implementation and `impl std::error::Error`.
- **Generic code is covered once per instantiation.** `parse::<u8>` and `parse::<u32>` are two
  functions to llvm-cov, each needing every branch. Keep the branches in a non-generic core and let
  the public generic function be a one-line wrapper (see `fs/write_atomic.rs`), or drive every type
  you use through every path.
- Write loops as `for`, iterators or `ilog2`, not `while i < n { ...; i += 1 }`: mutation testing
  turns `+=` into `*=` and the mutant can only end in a 60 second timeout.
- Keep the declaration on one line where you can: the docs generator reads `pub fn`, `pub struct`,
  `pub enum`, `pub type` and `pub const`, and the header of an `impl` block on one line.
  `pub async fn` is not recognized: write `pub fn f() -> impl Future<Output = T>`.
- A file must not have the name of its module (`date/date.rs`: clippy's module inception); name
  it after what it holds (`calendar_date.rs`).

### 3. Unit tests: `<name>.test.rs`

```rust
use super::*;

#[test]
fn splits_into_even_chunks() {
    assert_eq!(chunk(1..=4, 2), vec![vec![1, 2], vec![3, 4]]);
}
```

**Coverage must stay at 100%** of lines, functions and regions: one test per behaviour, including
every error and every edge case. `unwrap` is fine in tests. Float comparisons that are meant to be
exact need `#![allow(clippy::float_cmp)]` at the top of the test file.

### 4. Property tests: `<name>.spec.rs`

Invariants over random inputs, with [proptest](https://docs.rs/proptest): a round trip
(`decode(encode(x)) == x`), an idempotence, agreement with `std`, "never panics". They do not count
towards coverage, so they are for invariants, not for hunting branches.

```rust
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn flattening_the_chunks_gives_back_the_input(items in prop::collection::vec(0i32..100, 0..40), size in 1usize..8) {
        let flattened: Vec<i32> = chunk(items.clone(), size).into_iter().flatten().collect();
        prop_assert_eq!(flattened, items);
    }
}
```

Beware of properties that are only *almost* true: a wrong property fails once in a few thousand
runs and then only on some CI job. Run `PROPTEST_CASES=5000 cargo test <module> spec` before pushing.

### 5. Export it

Add `mod name;` and `pub use name::name;` to `src/<module>/mod.rs`, in alphabetical order. A type
that is only returned as an error (a name ending in `Error`) is documented on the page of the
helpers that return it, so at least one public helper must return it.

### 6. Benchmark (optional)

`src/<module>/name.bench.rs` exposes `pub fn bench(c: &mut Criterion)`; wire it from
`benches/<module>.rs` (`#[path = "../src/<module>/name.bench.rs"] mod name;`) and add it to that
file's `criterion_group!`. Benchmark what has a cost worth watching (parsing, allocation, a loop),
not one-liners. CI compares benchmarks with the base branch and reports, it does not block.

## Adding a module

1. Create `src/<module>/mod.rs` with the module doc (`//!`): one sentence on what the module is
   for, then how it differs from `std`.
2. Add the feature in `Cargo.toml` (`<module> = []`, and add it to `default`) and gate it in
   `src/lib.rs`: `#[cfg(feature = "<module>")] pub mod <module>;`.
3. Add the scope to `scopes.json` (commits are checked against it).
4. List the module in `llms.txt` and in the README table.
5. If it has benchmarks, add `benches/<module>.rs` and a `[[bench]]` entry with
   `required-features = ["<module>"]`.

A module that needs a third-party crate declares it `optional = true` and the feature enables it
(`<module> = ["dep:crate"]`); the default build stays dependency-free.

A single **helper** that needs a dependency the rest of its module does not (`string`'s
`remove_diacritics`, gated on `string-diacritics`) gets its own sub-feature instead of pulling
the dependency into the whole module: `<module>-<name> = ["<module>", "dep:crate"]`, with
`#[cfg(feature = "<module>-<name>")]` on both the file's `mod` declaration and its `pub use` in
`mod.rs`. Not added to `default`.

## Fuzzing

`fuzz/` holds [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) targets for the parsers most
exposed to untrusted input (`commit`, `duration`, `env`, `hex`, `url`, `version`). It is a
detached workspace (its `Cargo.toml` has an empty `[workspace]` table) because the sanitizer
instrumentation cargo-fuzz builds with is incompatible with the crate's normal build; it is not
part of `cargo test` or the package `include` list.

`fuzz.yml` runs every target for 120s daily. To add a target for a new parser or run one longer
locally:

```bash
rustup toolchain install nightly
cargo install --locked cargo-fuzz
cd fuzz
cargo +nightly fuzz run fuzz_<name> -- -max_total_time=60
```

A crash writes the failing input to `fuzz/artifacts/fuzz_<name>/`; turn it into a regression test
in `<name>.test.rs` once fixed. The fuzz target itself only needs to assert the parser does not
panic (a `Result::Err` on malformed input is the correct, expected outcome).

## Quality checks

Run these before pushing; they are what CI runs (`.github/workflows/README.md` has the detail).

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
typos && cargo machete
python3 scripts/coherency.py                                  # features/lib.rs/scopes.json/llms.txt/benches agree, `# Examples` present
cargo test --all-features
cargo +1.85 test --all-features                              # the minimum supported version
cargo test --no-default-features --features <module>          # the module on its own
cargo hack test --each-feature                                # every feature on its own
cargo hack check --feature-powerset --no-dev-deps --depth 2   # every pair of features builds
cargo llvm-cov --all-features --ignore-filename-regex '\.(test|spec|bench)\.rs$' \
  --fail-under-lines 100 --fail-under-functions 100 --fail-under-regions 100
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features
cargo semver-checks check-release --release-type minor         # a breaking change needs a `BREAKING CHANGE:` footer
cargo mutants --file src/<module>/<name>.rs                    # optional, slow: no surviving mutant
```

Check the exit code of each: a `| tail` hides a failure. Mutation testing is informational in CI
but a surviving mutant is a missing test; a mutant that cannot be caught (the changed code behaves
the same, or it can only hang) is listed with its reason in `.cargo/mutants.toml`.

## Commit messages

`<type>(<scope>): <emoji> <description>`, with the Conventional Commits types. The **scope** is a
module or an area from [scopes.json](scopes.json), and CI rejects any other; omit it rather than
invent one. The emoji is the primary gitmoji of the type (`feat` ✨, `fix` 🐛, `docs` 📝,
`refactor` ♻️, `test` ✅, `chore` 🔧, `perf` ⚡️, `style` 💄, `ci` 👷, `build` 📦️, `revert` ⏪️); the
full mapping and the alternatives are in the
[org rules](https://github.com/helpers4/.dev/blob/main/AGENTS.md).

```text
feat(set): ✨ add the set module
fix(validate): 🐛 don't require string to compile validate alone
test(commit): ✅ cover type characters
```

A breaking change uses the real Conventional Commits `BREAKING CHANGE:` footer in its own
paragraph: the changelog generator only detects that.

## Pull requests

- One pull request per piece of work: keep pushing to it rather than opening another.
- Bring the branch up to date with `git rebase origin/main`, never a merge commit.
- The description says what changed, what you ran, and what the reviewer must do after merging.
- Check the docs site still builds: the generator that turns the crate's rustdoc into helpers4.dev
  runs on every release, so a construct it cannot read (see the rules above) fails the release.

## Checklist

- [ ] One helper per file, in the right module, with the license header
- [ ] Rustdoc with `# Arguments`, `# Returns` (and `# Errors`) and a runnable `# Examples`
- [ ] Unit tests, 100% coverage; property tests for the invariants
- [ ] No `unsafe`, no panic, no new dependency
- [ ] `cargo test --no-default-features --features <module>` passes on its own
- [ ] `cargo +1.85 test --all-features` passes
- [ ] Listed in `mod.rs`, `llms.txt` and the README if it is a new module
- [ ] Commits follow the format above, with a scope from `scopes.json`

## For AI contributors

If you are an AI coding agent working in this repository:

- Read [AGENTS.md](AGENTS.md); the `add-helper` skill in `.claude/skills/` walks through the steps
  above and the checks.
- Check [docs/native-alternatives.json](docs/native-alternatives.json) before writing a helper.
- Write code, comments, commits and documentation in English.
- Do not build test data that looks like a real credential in one string literal: GitHub's push
  protection refuses the push (assemble it at run time).
- Ask before pushing, merging or releasing.

## Getting help

- Open an [issue](https://github.com/helpers4/rust/issues) for a question or a missing helper.
- Read a similar helper: `src/set`, `src/url` and `src/version` show most of the patterns above.
- Vulnerabilities go through [SECURITY.md](SECURITY.md), not an issue.
