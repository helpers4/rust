# GitHub Workflows

Same shape as [helpers4/typescript](https://github.com/helpers4/typescript)'s CI: thin
entry-point workflows that call reusable `job-*.yml` building blocks (`workflow_call`).

| Workflow | Trigger | Purpose |
| --- | --- | --- |
| `pr-validation.yml` | `pull_request` | Build, tests + coverage, compatibility, lint, docs, security, semver and conventional commits on every PR, plus one sticky status comment |
| `main-validation.yml` | `push` to `main` | The same suite post-merge, uploading coverage to Codecov |
| `mutation-dashboard.yml` | `push` to `main`, weekly, manual | Full [cargo-mutants](https://mutants.rs/) run in 12 shards (each must stay under the job's 45 minutes), merged into one score in the job summary |
| `compat-full.yml` | weekly, manual | Every combination of up to 3 Cargo features (`cargo hack --feature-powerset --depth 3`), in 8 shards — the PR path only checks pairs (`--depth 2`), see `job-compat.yml` |
| `scorecard.yml` | weekly, manual | [OpenSSF Scorecard](https://securityscorecards.dev/viewer/?uri=github.com/helpers4/rust) analysis, published (the site shows the score) and uploaded to code scanning |
| `fuzz.yml` | daily, manual | [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) (`fuzz/`, a detached workspace), 120s per parser target: `commit`, `duration`, `easing`, `env`, `hex`, `rgba`, `ulid`, `url`, `uuid`, `version`; a persistent corpus cache, crash inputs uploaded as an artifact |
| `zizmor.yml` | `push`/`pull_request` touching workflows, manual | [zizmor](https://docs.zizmor.sh/) static analysis of the workflows (template injection, permissions, persisted credentials), in the Security tab |
| `clusterfuzzlite.yml` | `pull_request` (code paths), weekly, manual | The same fuzz targets through [ClusterFuzzLite](https://google.github.io/clusterfuzzlite/) (`.clusterfuzzlite/`): 2 minutes on a PR, 10 weekly; what OpenSSF Scorecard's Fuzzing check looks for |
| `dependency-review.yml` | `pull_request` touching dependencies | Fails a PR that adds a dependency with a known vulnerability of moderate severity or more |
| `osv-scanner.yml` | `push` to `main`, `pull_request`, weekly, manual | [OSV-Scanner](https://google.github.io/osv-scanner/) on `Cargo.lock` and `fuzz/Cargo.lock`, in the Security tab |
| `release.yml` | manual (`workflow_dispatch`) | After the gates, [helpers4/action-release](https://github.com/helpers4/action-release) computes the version, commits it to `main`, publishes to crates.io, tags it and creates the GitHub release; then the crate is attested and the SBOM attached |
| `post-release.yml` | `release: published`, manual | Full mutation run on the exact tagged commit, attached to the release (`mutation-dashboard.yml` skips the release-prep commit itself); coverage uploaded to Codecov |
| `auto-assign.yml` | issues, PRs | Assigns the maintainer |

## Reusable jobs

| Job | What it checks |
| --- | --- |
| `job-lint.yml` | `cargo fmt`, `clippy -D warnings` (strict `[lints]` table), `typos`, `cargo machete`, `scripts/coherency.py` (features/`lib.rs`/`scopes.json`/`llms.txt`/benches agree, `# Examples` present) |
| `job-tests.yml` | Unit + property tests under `cargo llvm-cov`: **100%** lines, functions and regions (`*.test.rs`, `*.spec.rs`, `*.bench.rs` excluded); exposes the metrics |
| `job-compat.yml` | OS (Linux, macOS, Windows) × toolchain (stable, beta, MSRV 1.85); every pair of Cargo features (`cargo hack --depth 2`; up to 3 at a time is `compat-full.yml`); `wasm32-unknown-unknown` and `wasm32-wasip1`; minimal dependency versions (informational) |
| `job-compat-full.yml` | One shard (`k/n`) of the depth-3 feature powerset for `compat-full.yml` |
| `job-docs.yml` | `cargo doc` with warnings denied (broken intra-doc links) and every doctest |
| `job-security.yml` | `cargo deny --all-features`: RustSec advisories, yanked crates, licenses, bans, sources, including the optional dependencies; also on its own every day, since new advisories appear without any change in the repository |
| `job-semver.yml` | `cargo-semver-checks` against the published baseline; a change that needs a major bump under strict SemVer requires a `BREAKING CHANGE:` footer in the PR |
| `job-mutation.yml` | cargo-mutants, informational: only the lines a PR touches (`--in-diff`, skipped above 500 changed implementation lines, which the full run on `main` covers), or one shard of a full run. Configured in `.cargo/mutants.toml` |
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

A release is one manual workflow run; there is no version-bump PR.

1. **Write the summary** (optional): a short human summary of the release under `## [Unreleased]`
   in `CHANGELOG.md`, in a normal PR. It leads the section of the version and the release notes,
   above the list generated from the commits.
2. **Run** *Actions → Release → Run workflow* on `main`: `bump` is `auto` (from the Conventional
   Commits; before 1.0.0 a `feat` is a patch and a breaking change a minor) or a forced level.
   Tick `dry-run` first if in doubt: it does everything but write, and the job summary tells the
   version, what each check found, the files that would change and the release notes. The workflow
   re-runs lint, tests, compatibility, docs and security on that exact commit, then (in the
   `crates-io` environment):
   - records the public items added since the last tag as `"next"` in `api-since.json`
     (`scripts/update-api-since.py next`, `cargo public-api` on the nightly toolchain) and checks
     that every public item has an entry;
   - [helpers4/action-release](https://github.com/helpers4/action-release) writes the version in
     `Cargo.toml` and `Cargo.lock`, replaces `"next"` by it, adds the section to `CHANGELOG.md`,
     dry-runs `cargo publish`, pushes the signed release commit to `main` (`helpers4-bot`, a bypass
     actor of the `main` ruleset), tags it, publishes to crates.io and publishes the GitHub release;
   - attests the `.crate` (SLSA provenance) and attaches it to the release with its Sigstore
     bundle (`helpers4-X.Y.Z.crate.sigstore.json`) and a CycloneDX SBOM (`helpers4-X.Y.Z.cdx.json`).
3. **Resume**: if a run failed after the release commit, run it again (or `bump: resume`). Only
   what crates.io does not serve yet is published, then the release.

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

### Verifying a release

The attestation proves that a `.crate` was built by `release.yml` of this repository, on the
commit the release points at, and was not altered afterwards. It needs no key: the workflow signs
with its GitHub identity (Sigstore), so there is nothing to store or rotate. To check a download:

```sh
gh attestation verify helpers4-X.Y.Z.crate --repo helpers4/rust
```

The bundle is also attached to the release as `helpers4-X.Y.Z.crate.sigstore.json`, because tools
that only read release assets (OpenSSF Scorecard's *Signed-Releases* check) cannot see GitHub's
attestation store. Releases published before 0.0.7 have no such file and are not backfilled: a new
attestation for an old file would not prove it came from this workflow.

### Website

Once the release is published, the `trigger-website-docs` job dispatches a `rust-release` event to
`helpers4/website`, whose `on-rust-release.yml` checks this repository out at the release tag and
regenerates the Rust documentation from it. The job is best-effort (a failure never fails the
release) and needs the GitHub App credentials `TRIGGANATOR_ID` / `TRIGGANATOR_KEY`, with
`PUSHINATOR_ID` / `PUSHINATOR_KEY` as a fallback identity, to be available to this repository.
If it fails, run *On Rust Release* on the website by hand with the version.
