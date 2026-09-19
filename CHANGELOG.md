# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
- **string**: drop the Since marker in favor of a computed api-since.json
- **string**: list the string helpers in llms.txt

### ✅ Tests
- **array**: benchmark unique, difference, group_by and equals_unordered
- **hex**: benchmark encode and decode
- **string**: benchmark the case, slug, escape and dedent helpers

### 📦 Build
- publish only sources, benchmarks and entry-point docs

### 👷 CI/CD
- confirm suspected benchmark regressions before reporting them
- say so when a diff generates no mutant
- raise the benchmark noise threshold to avoid false regressions
- add benchmark and mutation-testing jobs
- configure cargo-mutants
- run the minimal-versions job on nightly despite rust-toolchain.toml
- split CI into reusable jobs with PR and main validation

