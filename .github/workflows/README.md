# GitHub Workflows

Same shape as [helpers4/typescript](https://github.com/helpers4/typescript)'s CI: thin
entry-point workflows that call reusable `job-*.yml` building blocks (`workflow_call`).

| Workflow | Trigger | Purpose |
| --- | --- | --- |
| `pr-validation.yml` | `pull_request` | Build, tests + coverage, compatibility, lint, docs, security and conventional commits on every PR, plus one sticky status comment |
| `main-validation.yml` | `push` to `main` | The same suite post-merge, uploading coverage to Codecov |
| `mutation-dashboard.yml` | `push` to `main`, weekly, manual | Full [cargo-mutants](https://mutants.rs/) run in 4 shards, merged into one score in the job summary |
| `compat-full.yml` | weekly, manual | Every combination of up to 3 Cargo features (`cargo hack --feature-powerset --depth 3`), in 8 shards — the PR path only checks pairs (`--depth 2`), see `job-compat.yml` |
| `scorecard.yml` | weekly, manual | [OpenSSF Scorecard](https://securityscorecards.dev/viewer/?uri=github.com/helpers4/rust) analysis, published (the site shows the score) and uploaded to code scanning |
| `release.yml` | manual (`workflow_dispatch`) | Publishes the version in `Cargo.toml` to crates.io, then tags it and creates the GitHub release |
| `auto-assign.yml` | issues, PRs | Assigns the maintainer |

## Reusable jobs

| Job | What it checks |
| --- | --- |
| `job-lint.yml` | `cargo fmt`, `clippy -D warnings` (strict `[lints]` table), `typos`, `cargo machete` |
| `job-tests.yml` | Unit + property tests under `cargo llvm-cov`: **100%** lines, functions and regions (`*.test.rs`, `*.spec.rs`, `*.bench.rs` excluded); exposes the metrics |
| `job-compat.yml` | OS (Linux, macOS, Windows) × toolchain (stable, beta, MSRV 1.85); every pair of Cargo features (`cargo hack --depth 2`; up to 3 at a time is `compat-full.yml`); `wasm32-unknown-unknown` and `wasm32-wasip1`; minimal dependency versions (informational) |
| `job-compat-full.yml` | One shard (`k/n`) of the depth-3 feature powerset for `compat-full.yml` |
| `job-docs.yml` | `cargo doc` with warnings denied (broken intra-doc links) and every doctest |
| `job-security.yml` | `cargo deny`: RustSec advisories, yanked crates, licenses, bans, sources |
| `job-mutation.yml` | cargo-mutants, informational: only the lines a PR touches (`--in-diff`, skipped above 3 000 changed implementation lines, which the full run on `main` covers), or one shard of a full run. Configured in `.cargo/mutants.toml` |
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
2. **Run** *Actions → Release → Run workflow* on `main` (tick `dry-run` first if in doubt). The workflow
   checks the version and its changelog section, re-runs lint, tests, compatibility, docs and
   security on that exact commit, then (in the `crates-io` environment) publishes, waits until
   crates.io serves the version, attests the `.crate` (SLSA provenance), and creates the tag and
   the GitHub release from the changelog. It never commits.
3. **Resume**: if a run published but failed afterwards, run it again. It detects that the version
   is already on crates.io and only finishes the tag and the release.

### Authentication

Publishing uses crates.io **trusted publishing**: the run mints a short-lived token from its GitHub
OIDC identity, so there is no API token or secret to store or rotate. It is configured once, in
the crate's settings on crates.io (*Trusted Publishing*): repository `helpers4/rust`, workflow
`release.yml`, environment `crates-io`. The job's `environment: crates-io` must match it.

Recommended: give that GitHub environment (repository *Settings → Environments*) the maintainer as
required reviewer, so nothing is published without an approval.

A brand-new crate cannot use trusted publishing for its very first publish (crates.io can only
configure it for a crate that already exists): that one needs a temporary API token, used from a
local `cargo publish` or a throw-away secret, then revoked. `helpers4` went through that with 0.0.1.

### Website

Once the release is published, the `trigger-website-docs` job dispatches a `rust-release` event to
`helpers4/website`, whose `on-rust-release.yml` checks this repository out at the release tag and
regenerates the Rust documentation from it. The job is best-effort (a failure never fails the
release) and needs the GitHub App credentials `TRIGGANATOR_ID` / `TRIGGANATOR_KEY`, with
`PUSHINATOR_ID` / `PUSHINATOR_KEY` as a fallback identity, to be available to this repository.
If it fails, run *On Rust Release* on the website by hand with the version.
