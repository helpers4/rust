# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.3] - 2026-09-21

### 📝 Documentation
- add the pre-1.0 notice to the README and fix its install section

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

[0.0.3]: https://github.com/helpers4/rust/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/helpers4/rust/compare/v0.0.1...v0.0.2

