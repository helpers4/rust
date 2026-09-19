# GitHub Workflows

Same shape as [helpers4/typescript](https://github.com/helpers4/typescript)'s CI: thin
entry-point workflows that call reusable `job-*.yml` building blocks (`workflow_call`).

| Workflow | Trigger | Purpose |
| --- | --- | --- |
| `pr-validation.yml` | `pull_request` | Build, tests + coverage, compatibility, lint, docs, security and conventional commits on every PR, plus one sticky status comment |
| `main-validation.yml` | `push` to `main` | The same suite post-merge, uploading coverage to Codecov |
| `auto-assign.yml` | issues, PRs | Assigns the maintainer |

## Reusable jobs

| Job | What it checks |
| --- | --- |
| `job-lint.yml` | `cargo fmt`, `clippy -D warnings` (strict `[lints]` table), `typos`, `cargo machete` |
| `job-tests.yml` | Unit + property tests under `cargo llvm-cov`: **100%** lines, functions and regions (`*.test.rs`, `*.spec.rs`, `*.bench.rs` excluded); exposes the metrics |
| `job-compat.yml` | OS (Linux, macOS, Windows) × toolchain (stable, beta, MSRV 1.85); every Cargo feature combination (`cargo hack`); `wasm32-unknown-unknown` and `wasm32-wasip1`; minimal dependency versions (informational) |
| `job-docs.yml` | `cargo doc` with warnings denied (broken intra-doc links) and every doctest |
| `job-security.yml` | `cargo deny`: RustSec advisories, yanked crates, licenses, bans, sources |
| `job-build.yml` | Release build with every feature, benchmarks compile, `cargo package` |

Third-party actions are pinned by commit SHA. Workflows default to `contents: read`.

## Commit scopes

`scopes.json` is the single source of truth: the `conventional-commits` job reads it and
rejects any other scope.

## Secrets

`CODEOWNERS`-level maintainer setup that CI degrades gracefully without: `CODECOV_TOKEN`
(coverage upload). Later workflows (release, scorecard) will list theirs here.
