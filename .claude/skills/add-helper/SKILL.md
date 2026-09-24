---
name: add-helper
description: Scaffold a new helpers4 Rust helper: implementation, tests, property tests and doctest, following this repo's conventions and its 100% coverage bar
---

Add a new helper to the `helpers4` crate, end to end. Ask the user for the helper's name and
behavior first if it was not already specified.

## 1. Placement and design

- `<module>` throughout this skill is one of the **existing** directories under `src/`. Run
  `ls src/` and read the module's `mod.rs` doc; do not guess, the list grows. If nothing fits, see
  `CONTRIBUTING.md`'s "Adding a module" before creating one.
- Check `docs/native-alternatives.json` and the standard library **first**: do not add a helper
  that duplicates `std`. If it only wraps a std call, it has to add something (a typed error, several
  inputs, a stable order); say what in the doc.
- Naming: a plain-English, self-describing name (`symmetric_difference`, not `xor`), snake_case for
  functions, UpperCamelCase for types. A name may repeat across modules; callers use the module path.
- Design rules that CI enforces: no `unsafe`; no `unwrap`/`expect`/`panic!` outside tests (return
  `Result` with a typed error, or `Option`, and give degenerate input a documented result); zero
  third-party dependencies; explicit inputs (the clock, the environment and randomness are
  parameters, never read inside); the module must **not use another module** (it has to pass
  `cargo test --no-default-features --features <module>` alone).

## 2. Create the files

Follow `CONTRIBUTING.md`'s "Adding a helper" for the templates.

- `src/<module>/<name>.rs`: license header, rustdoc, implementation. Rustdoc order: summary,
  details, `# Arguments` (one `` - `name` - description `` per parameter), `# Returns`, `# Errors`
  (for a `Result`), `# Examples` (a runnable doctest; `?` plus `# Ok::<(), ...>(())`, not `unwrap`).
  `#[must_use]` on pure functions. Errors are `#[non_exhaustive]` enums with `Display` and
  `std::error::Error`. Wire the siblings at the bottom:
  `#[cfg(test)] #[path = "<name>.test.rs"] mod tests;` and the same for `spec`.
- `src/<module>/<name>.test.rs`: `use super::*;` and unit tests for every branch. **100% line,
  function and region coverage** is required.
- `src/<module>/<name>.spec.rs`: `proptest` invariants (round trip, idempotence, agreement with
  `std`, never panics). Excluded from coverage: not for branch-hunting.
- `src/<module>/<name>.bench.rs`: **optional**; match the module's convention
  (`ls src/<module>/*.bench.rs`), wire it from `benches/<module>.rs`.
- `src/<module>/mod.rs`: add `mod <name>;` and `pub use <name>::<name>;` in alphabetical order.
  A type ending in `Error` must be returned by a public helper (the site documents it there).

Pitfalls that have cost a CI run before:

- **Generic code is covered per instantiation.** Put the branches in a non-generic core with a
  one-line generic wrapper, or drive each type through every path.
- Write loops as `for`/iterators/`ilog2`, not `while i < n { i += 1 }`: the `+=` → `*=` mutant hangs
  for 60 s.
- Do not name a file after its module (clippy `module_inception`).
- The docs generator on the website reads `pub fn|struct|enum|type|const` and a one-line `impl`
  header. `pub async fn` is not recognized: return `impl Future<Output = T>` instead.
- A property test must be **true**, not almost true: run it with `PROPTEST_CASES=5000`.
- Float equality in tests: `#![allow(clippy::float_cmp)]` at the top of the test file.
- Never put something shaped like a real credential in one string literal (GitHub push protection).
- MSRV is 1.85: no let-chains (`if let ... && ...`), nothing newer than 1.85 in `std`.

If the helper is a **new module**, follow "Adding a module": Cargo feature (and `default`), gate in
`src/lib.rs`, `scopes.json`, `llms.txt`, README table.

## 3. Verify, in this order

Run each and **check the exit code** (a `| tail` hides a failure):

```bash
cargo fmt --all
cargo test --no-default-features --features <module>        # the module alone
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features && cargo +1.85 test --all-features
cargo llvm-cov --all-features --ignore-filename-regex '\.(test|spec|bench)\.rs$' \
  --fail-under-lines 100 --fail-under-functions 100 --fail-under-regions 100
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features
typos && cargo machete
python3 scripts/coherency.py                                # a new module must be wired everywhere
cargo hack test --each-feature
```

Read the per-file coverage of the **new** files: 100% on the total can hide a gap in exactly the
file just added. If a region is reported uncovered with no missing line, it is usually a branch of
a short-circuit (`a || b`) or an error path of a `?` that no test triggers; read the JSON segments
(`cargo llvm-cov --json`) to find the exact column.

Optionally `cargo mutants --file src/<module>/<name>.rs`: a surviving mutant is a missing test, or
an equivalent mutant to list with its reason in `.cargo/mutants.toml`.

## 4. Commit

Only if explicitly asked to commit *this turn*: commit authorization is per turn, not a standing
grant. When authorized, one commit scoped to the module, in the format of `CONTRIBUTING.md`
(`<type>(<scope>): <emoji> <description>`, scope from `scopes.json`, primary gitmoji), ending with
the attribution lines the session asks for. Never merge a branch into a PR branch: bring it up to
date with `git rebase origin/main`.
