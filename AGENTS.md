# AGENTS.md — rust

→ [Org-wide rules](https://github.com/helpers4/.dev/blob/main/AGENTS.md): restrictions · commit format · license headers

## This Repository

**Purpose:** General-purpose Rust helpers — one crate (`helpers4`), one module per category.
**Stack:** Rust stable (edition 2024, MSRV 1.85) · cargo · cargo-llvm-cov (100%) · proptest · criterion · cargo-deny · clippy

```text
src/<module>/
├── mod.rs                  # module docs + `pub use` of every helper
├── function_name.rs        # one helper per file (snake_case)
│                           #   holds the implementation, its `#[cfg(test)] mod tests`
│                           #   (unit + proptest) and its doc examples (doctests)
└── _internal.rs            # crate-private, not re-exported
benches/<module>.rs         # criterion benchmarks, optional per helper
```

**Key commands:**

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features && cargo test --no-default-features
cargo llvm-cov --all-features --fail-under-lines 100 --fail-under-functions 100 --fail-under-regions 100
cargo doc --no-deps --all-features
cargo bench --all-features
cargo deny check
```

**Rules:**

- `unsafe` forbidden (`#![forbid(unsafe_code)]`); no `unwrap()`/`expect()` outside tests
- Rustdoc on every public item, with a runnable `# Examples` block (doctests are the smoke
  tests) and a `# Since` section
  - Not yet released → `next` (replaced at release time)
  - Existing version → **never change** (records the first published version)
- 100% coverage: lines, functions, regions — no exceptions. Property tests (proptest) live
  next to unit tests and cover invariants, not branches
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
