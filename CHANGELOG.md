# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.7] - 2026-09-25

### 🚀 Features
- **cache**: add a bounded LRU cache
- **env**: add typed getters — bool, int, list, duration
- **hex**: add base64 and base32 encoding and decoding
- **release**: compute api-since.json and gate PRs on cargo-semver-checks
- **string**: add words, excerpt, escape_regex, levenshtein_distance/similarity, remove_diacritics

### 🐛 Bug Fixes
- **release**: give cargo-cyclonedx's override filename its own .cdx suffix

### 📝 Documentation
- **agents**: add the replace-with-helpers4 skill for Rust
- **agents**: add the add-helper skill and native-alternatives.json for std
- **scorecard**: add the OpenSSF Best Practices badge
- **security**: list the supported releases as 0.0.x
- expand CONTRIBUTING.md to the level of the TypeScript one

### ✅ Tests
- add cargo-fuzz targets for the six untrusted-input parsers
- add benchmarks for the string, array and number additions

### 📦 Build
- **deps**: Bump the actions group with 3 updates

### 👷 CI/CD
- **coherency**: add a coherency script for module wiring and rustdoc
- **deps**: group major updates and attach a CycloneDX SBOM to releases
- **release**: run mutation testing and coverage on the exact tagged release
- **release**: attach the Sigstore bundle to the release next to the crate
- add one aggregating status check for the branch ruleset

## [0.0.6] - 2026-09-23

### 🚀 Features
- **ansi**: add the ansi module: strip, contains, visible_len, Style, Color
- **bytes**: add the bytes module: read_u16/32/64, find, xor, constant_time_eq, format_size, parse_size
- **ci**: add the ci module: Provider, detect, is_ci, is_pull_request, Status, overall, render_report
- **color**: add the color module: Rgb, Hsl, ParseColorError, mix, contrast_ratio, best_text_color
- **commit**: add the commit module: Commit, Bump, ParseCommitError, bump_for, is_valid, emoji_for
- **date**: add the date module: Date, Weekday, DateError, is_leap_year, days_in_month
- **fs**: add the fs module: write_atomic, read_to_string_opt, walk, normalize, is_within
- **function**: add the function module: compose, pipe, Memoize, retry, backoff, TokenBucket
- **future**: add the future module: block_on, now_or_never, yield_now, join, join_all
- **license**: add the license module: lookup, normalize, Expression, Category, LicenseInfo, header
- **markdown**: add the markdown module: escape, link, inline_code, code_block, blockquote, table, heading_slug
- **secret**: add the secret module: Secret, redact, mask, detect, scan, TokenKind
- **set**: add the set module: union_all, intersection_all, toggle, to_sorted_vec, jaccard, subsets
- **url**: add the url module: Url, percent_encode/decode, parse_query, build_query, join_path
- **version**: add the version module: Version, VersionReq, ParseVersionError, compare, satisfies

### 🔧 Miscellaneous
- **coherency**: exclude two mutants no test can catch
- **coherency**: tell typos about the test data that looks like typos

### ♻️ Refactoring
- **bytes**: replace hand-rolled index loops so mutants cannot hang

### 📝 Documentation
- **agents**: list the new modules in llms.txt
- list the new modules in the README

### ✅ Tests
- **bytes**: cover multi-digit fractions and drop the redundant unit guards
- **commit**: cover type characters, ordinary footers and 400-year cycles
- **secret**: fix a property that did not hold

### 👷 CI/CD
- skip the PR mutation run when the diff is too big for the job
- cap the weekly feature powerset at depth 3
- stop the feature powerset check from growing unbounded on every PR

## [0.0.5] - 2026-09-22

### 🚀 Features
- **iter**: add the iter module: chunk, min_max and first_duplicate
- **validate**: add the validate module: is_valid_email, is_uuid and is_slug

### 🐛 Bug Fixes
- **validate**: gate the whole spec file, not just the proptest block
- **validate**: don't require string to compile validate alone

### 📝 Documentation
- **agents**: list the iter and validate modules in llms.txt
- add the Scorecard badge and the iter/validate modules to the README
- add # Arguments and # Returns to every helper's rustdoc

### ✅ Tests
- **iter**: exclude two equivalent mutants in min_max

## [0.0.4] - 2026-09-21

### 🚀 Features
- **duration**: add the duration module: parse, format and ParseDurationError
- **map**: add the map module: pick, omit and map_values

### 📝 Documentation
- **agents**: list the duration and map modules in llms.txt

### ✅ Tests
- **map**: bind the expected value so the test compiles on Rust 1.85

## [0.0.3] - 2026-09-21

### 🚀 Features
- **array**: add key_by, duplicates and interleave
- **number**: add the number module: lerp, round_to, mean, median, percentage, gcd, lcm
- **string**: add title_case, constant_case, squish, indent and unescape_html

### 📝 Documentation
- **agents**: list the new helpers in llms.txt
- add the Codecov badge to the README
- rewrite the README and point the homepage to helpers4.dev/rust
- add the pre-1.0 notice to the README and fix its install section

### ✅ Tests
- **number**: exclude the mutants that only hang
- **number**: cover lcm(0, 0)

### 👷 CI/CD
- add the OpenSSF Scorecard workflow
- notify the website when a release is published

## [0.0.2] - 2026-09-20

### 🚀 Features
- **cache**: add ExpiringMap
- **http**: add bearer_token
- **net**: add is_valid_hostname and HostnameError
- **net**: add is_public_ip
- **time**: add unix_now_millis
- **time**: add unix_now and ClockError

### 🐛 Bug Fixes
- **net**: reject hostnames whose last label is a number

### 🔧 Miscellaneous
- ignore the VS Code local history

### ♻️ Refactoring
- **hex**: name the hex parameter `hex` instead of `s`
- **net**: make the CIDR prefix precondition explicit

### 📝 Documentation
- **net**: spell out what is_public_ip guarantees and what it cannot know
- document the ExpiringMap capacity bound and the unix_now_millis saturation

### ✅ Tests
- **cache**: check that a non-empty map is not empty
- benchmark is_public_ip and ExpiringMap

### 👷 CI/CD
- exclude the equivalent ExpiringMap sweep-guard mutant
- publish with trusted publishing only

## [0.0.1] - 2026-09-19

### 🚀 Features
- **array**: add cartesian_product
- **array**: add group_by
- **array**: add count_by
- **array**: add equals_unordered
- **array**: add intersects
- **array**: add symmetric_difference
- **array**: add intersection
- **array**: add difference
- **array**: add unique_by
- **array**: add unique
- **env**: add remove
- **env**: add set and InvalidKeyError
- **env**: add get
- **env**: add parse
- **hex**: add decode_array
- **hex**: add decode_to_slice
- **hex**: add decode and DecodeError
- **hex**: add encode_upper
- **hex**: add encode
- **string**: add escape_html
- **string**: add dedent
- **string**: add truncate
- **string**: add slugify
- **string**: add kebab_case
- **string**: add snake_case
- **string**: add pascal_case
- **string**: add camel_case
- scaffold crate with string::capitalize

### 🔧 Miscellaneous
- drop the redundant helpers4 scope

### ♻️ Refactoring
- **string**: colocate tests, property tests and benches with each helper
- adopt a strict lint table

### 📝 Documentation
- **agents**: describe the CI checks and the reusable workflows
- **agents**: note that coverage counts each generic instantiation
- **release**: accept an org-level token and make the environment optional
- **string**: drop the Since marker in favor of a computed api-since.json
- **string**: list the string helpers in llms.txt

### ✅ Tests
- **array**: benchmark unique, difference, group_by and equals_unordered
- **hex**: benchmark encode and decode
- **string**: benchmark the case, slug, escape and dedent helpers

### 📦 Build
- anchor the published file patterns to the crate root
- publish only sources, benchmarks and entry-point docs

### 👷 CI/CD
- simplify the release inputs
- drop the website notification from the release workflow
- add the crates.io release workflow
- confirm suspected benchmark regressions before reporting them
- say so when a diff generates no mutant
- raise the benchmark noise threshold to avoid false regressions
- add benchmark and mutation-testing jobs
- configure cargo-mutants
- run the minimal-versions job on nightly despite rust-toolchain.toml
- split CI into reusable jobs with PR and main validation

[0.0.7]: https://github.com/helpers4/rust/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/helpers4/rust/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/helpers4/rust/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/helpers4/rust/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/helpers4/rust/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/helpers4/rust/compare/v0.0.1...v0.0.2

