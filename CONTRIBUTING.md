# Contributing

Read [AGENTS.md](AGENTS.md) first: it holds the layout, rules and the checks CI runs.

## Adding a helper

1. Pick the module (`src/<module>/`); check the standard library does not already cover it.
2. Create `src/<module>/<name>.rs` with the license header, rustdoc (`# Examples`, `# Since` →
   `next`), the implementation, and a `#[cfg(test)] mod tests` with unit and proptest cases.
3. Re-export it from `src/<module>/mod.rs`.
4. Optional benchmark in `benches/<module>.rs`.
5. Run every command listed under "Key commands" in AGENTS.md. Coverage must stay at 100%.

## Adding a module

Add a Cargo feature, gate `pub mod <module>` on it in `src/lib.rs`, add the module's
scope to `scopes.json`, list it in `llms.txt`, and add a bench with `required-features`.

## Commits

`<type>(<scope>): <emoji> <description>` — see the
[org rules](https://github.com/helpers4/.dev/blob/main/AGENTS.md); scopes come from `scopes.json`.

By contributing, you agree that your contributions are licensed under LGPL-3.0-or-later.
