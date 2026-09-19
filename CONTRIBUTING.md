# Contributing

Read [AGENTS.md](AGENTS.md) first: it holds the layout, rules and the checks CI runs.

## Adding a helper

1. Pick the module (`src/<module>/`); check the standard library does not already cover it.
2. Create `src/<module>/<name>.rs` with the license header, rustdoc (with a runnable `# Examples`
   block) and the implementation, then its sibling files: `<name>.test.rs` (unit tests),
   `<name>.spec.rs` (proptest) and optionally `<name>.bench.rs`. See the layout in AGENTS.md.
3. Re-export it from `src/<module>/mod.rs`.
4. If there is a benchmark, add it to `benches/<module>.rs` and to its `criterion_group!`.
5. Run every command listed under "Key commands" in AGENTS.md. Coverage must stay at 100%.

## Adding a module

Add a Cargo feature, gate `pub mod <module>` on it in `src/lib.rs`, add the module's
scope to `scopes.json`, list it in `llms.txt`, and add a bench with `required-features`.

## Commits

`<type>(<scope>): <emoji> <description>` — see the
[org rules](https://github.com/helpers4/.dev/blob/main/AGENTS.md); scopes come from `scopes.json`.

By contributing, you agree that your contributions are licensed under LGPL-3.0-or-later.
