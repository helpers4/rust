# GitHub Workflows

Same shape as [helpers4/typescript](https://github.com/helpers4/typescript)'s CI: thin
entry-point workflows that call reusable `job-*.yml` building blocks (`workflow_call`).

| Workflow | Trigger | Purpose |
| --- | --- | --- |
| `pr-validation.yml` | `pull_request` | Build, tests + coverage, compatibility, lint, docs, security and conventional commits on every PR, plus one sticky status comment |
| `main-validation.yml` | `push` to `main` | The same suite post-merge, uploading coverage to Codecov |
| `mutation-dashboard.yml` | `push` to `main`, weekly, manual | Full [cargo-mutants](https://mutants.rs/) run in 4 shards, merged into one score in the job summary |
| `release.yml` | manual (`workflow_dispatch`) | Publishes the version in `Cargo.toml` to crates.io, then tags it and creates the GitHub release |
| `auto-assign.yml` | issues, PRs | Assigns the maintainer |

## Reusable jobs

| Job | What it checks |
| --- | --- |
| `job-lint.yml` | `cargo fmt`, `clippy -D warnings` (strict `[lints]` table), `typos`, `cargo machete` |
| `job-tests.yml` | Unit + property tests under `cargo llvm-cov`: **100%** lines, functions and regions (`*.test.rs`, `*.spec.rs`, `*.bench.rs` excluded); exposes the metrics |
| `job-compat.yml` | OS (Linux, macOS, Windows) × toolchain (stable, beta, MSRV 1.85); every Cargo feature combination (`cargo hack`); `wasm32-unknown-unknown` and `wasm32-wasip1`; minimal dependency versions (informational) |
| `job-docs.yml` | `cargo doc` with warnings denied (broken intra-doc links) and every doctest |
| `job-security.yml` | `cargo deny`: RustSec advisories, yanked crates, licenses, bans, sources |
| `job-mutation.yml` | cargo-mutants, informational: only the lines a PR touches (`--in-diff`), or one shard of a full run. Configured in `.cargo/mutants.toml` |
| `job-bench.yml` | criterion, informational: only the benches a PR affects, compared against the base branch measured on the same runner; every bench on `main` |
| `job-build.yml` | Release build with every feature, benchmarks compile, `cargo package` |

Third-party actions are pinned by commit SHA. Workflows default to `contents: read`.

## Commit scopes

`scopes.json` is the single source of truth: the `conventional-commits` job reads it and
rejects any other scope.

## Secrets

`CODEOWNERS`-level maintainer setup that CI degrades gracefully without: `CODECOV_TOKEN`
(coverage upload). Later workflows (release, scorecard) will list theirs here.

## Releasing

A release is a normal PR followed by one manual workflow run.

1. **Prepare**: a PR `chore(release): 🔖 X.Y.Z` that bumps `version` in `Cargo.toml` (and
   `Cargo.lock`) and adds the changelog section (`git cliff --tag vX.Y.Z -o CHANGELOG.md`).
   Merge it.
2. **Run** *Actions → Release → Run workflow* on `main`. `dry-run` first if in doubt. The workflow
   checks the version and its changelog section, re-runs lint, tests, compatibility, docs and
   security on that exact commit, then (in the `crates-io` environment) publishes, waits until
   crates.io serves the version, attests the `.crate` (SLSA provenance), and creates the tag and
   the GitHub release from the changelog. It never commits.
3. **Resume**: if a run published but failed afterwards, run it again. It detects that the version
   is already on crates.io and only finishes the tag and the release.

### One-time setup (maintainer)

- crates.io account with a verified e-mail and 2FA.
- The secret `CARGO_REGISTRY_TOKEN`: a crates.io API token with the `publish-new` and
  `publish-update` scopes, restricted to the crate pattern `helpers4` and a short expiry. It can
  be a repository secret, a `crates-io` environment secret, or an organization secret **restricted
  to this repository** (never "all repositories").
- Recommended: the GitHub environment `crates-io` (repository *Settings → Environments*) with the
  maintainer as required reviewer, so nothing is published without an approval. Without it the
  release still works, but the token is readable by every workflow of the repository.
- **First publish only** uses that token (`auth: token`): crates.io trusted publishing can only be
  configured for a crate that already exists.
- **Then** on crates.io, in the crate's settings, add a Trusted Publisher (repository
  `helpers4/rust`, workflow `release.yml`, environment `crates-io`), run the next release with
  `auth: trusted-publishing`, and delete the token secret.
- Optional: the `TRIGGANATOR_*` / `PUSHINATOR_*` app credentials used to notify the website
  repository (the notification is best-effort and never fails the release).
