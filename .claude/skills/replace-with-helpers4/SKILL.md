---
name: replace-with-helpers4
description: Find hand-written Rust code that the helpers4 crate already provides (hex, dotenv, bearer tokens, SSRF checks, URLs, versions, retry, expiring caches, ANSI, ...) and propose replacing it, module by module, without adding a dependency you do not need
---

Scan a Rust project for code that duplicates a `helpers4` helper, and propose replacing it. This
skill is for **using** helpers4 in another project (to add a helper to helpers4 itself, use the
`add-helper` skill). Report first, edit only after the user agrees.

## 0. Before anything

- Read `llms.txt` at the root of helpers4 (or https://helpers4.dev/llms.txt) for the current list
  of modules and helpers, and https://helpers4.dev/rust/ for the exact signatures. The table below is
  a guide to what to look for, not the source of truth: the crate grows.
- helpers4 is **pre-1.0 and has had no independent security audit**. Do not propose it for code
  that is critical or sensitive (authentication, cryptography, payments) unless the user asks, and
  say so in the report when a candidate is security-related (`net::is_public_ip`,
  `bytes::constant_time_eq`, `secret`, `fs::is_within`).
- MSRV of helpers4 is **1.85**. If the project's `rust-version` is lower, stop and say so.
- Prefer the standard library. Check `docs/native-alternatives.json` in the helpers4 repo: if `std`
  already does it, that is the better replacement, and no dependency is needed.

## 1. Find candidates

Search the project (skip `target/`, generated code and vendored code) for these patterns. Each row
is *hand-written code* → *the helper that replaces it*:

| Look for | Replace with |
| --- | --- |
| `format!("{:02x}", b)` in a loop, `u8::from_str_radix` over pairs | `hex::encode`, `encode_upper`, `decode`, `decode_array` |
| `.duration_since(UNIX_EPOCH).unwrap()` or `.unwrap_or_default()` | `time::unix_now`, `unix_now_millis` (a clock before 1970 is an error, not `0`) |
| splitting a `.env` file on `=`, quoting rules by hand | `env::parse`, `get`, `set`, `remove` |
| `header.strip_prefix("Bearer ")` | `http::bearer_token` (the scheme is case-insensitive per RFC 7235) |
| `ip.is_private()` / `is_loopback()` chains guarding an outgoing request | `net::is_public_ip` (SSRF guard: also link-local, metadata address, IPv4-mapped IPv6) |
| a hostname regex | `net::is_valid_hostname` |
| `HashMap` loops that group, count or index | `array::group_by`, `count_by`, `key_by`; several sets: `set::union_all`, `intersection_all` |
| dedup that must keep order | `array::unique`, `unique_by`, `duplicates` |
| iterator chunking, one-pass min and max | `iter::chunk`, `min_max`, `first_duplicate` |
| case conversions, slugs, `chars().take(n)` truncation, dedent | `string::camel_case`, `snake_case`, `kebab_case`, `slugify`, `truncate`, `dedent`, `squish`, `indent` |
| `.replace("&", "&amp;")` chains | `string::escape_html`, `unescape_html` |
| parsing `"1h30m"`, formatting durations | `duration::parse`, `format` |
| retry loops with a sleep, exponential backoff, hand-made rate limiter | `function::retry`, `backoff`, `TokenBucket` |
| a `HashMap` of entries with an expiry, replay protection | `cache::ExpiringMap`, `ExpiringSet` |
| version strings compared as text or split on `.` | `version::Version`, `VersionReq`, `compare`, `satisfies` |
| percent-encoding, `?a=1&b=2` built or split by hand, URL joining | `url::percent_encode`, `percent_decode`, `build_query`, `parse_query`, `Url`, `join_path` |
| regexes that strip `\x1b[...m`, hand-written escape codes | `ansi::strip`, `visible_len`, `Style`, `Color` |
| escaping Markdown, building tables and code fences | `markdown::escape`, `table`, `code_block`, `link`, `inline_code` |
| a regex for `type(scope): description` | `commit::Commit`, `bump_for`, `is_valid` |
| masking tokens in logs, a wrapper to keep a value out of `Debug` | `secret::Secret`, `redact`, `mask`, `detect`, `scan` |
| `env::var("GITHUB_ACTIONS")` and friends | `ci::detect`, `is_ci`, `is_pull_request` |
| SPDX strings compared as text | `license::Expression`, `lookup`, `normalize` |
| write to a temp file then `rename` | `fs::write_atomic` |
| `starts_with(base)` to keep a path inside a directory | `fs::is_within` (lexical only: it does not see symlinks, say so) |
| `slice[i..i + 4].try_into().unwrap()` then `from_be_bytes` | `bytes::read_u16`, `read_u32`, `read_u64` (returns `None` when short) |
| `"1.5 KiB"` sizes by hand | `bytes::format_size`, `parse_size` |
| leap years, weekday, days between dates by hand | `date::Date`, `Weekday`, `is_leap_year`, `days_in_month` |
| hex colors to RGB, contrast checks | `color::Rgb`, `Hsl`, `contrast_ratio`, `best_text_color` |
| a homemade `block_on` or `join` | `future::block_on`, `join`, `join_all` |
| email, UUID or slug regexes | `validate::is_valid_email`, `is_uuid`, `is_slug` |

Also count the **crates the project depends on only for one of these** (a `hex`, `humantime`,
`semver` or `url` dependency used in three places): helpers4 may let the project drop it, but say
what is lost (the `url` crate implements WHATWG, helpers4 `url` is RFC 3986; `chrono` and `jiff`
do time zones, helpers4 `date` does not).

## 2. Report before editing

Give the user a table, most valuable first:

| Where (`file:line`) | Current code | Helper | Gain | Risk |
| --- | --- | --- | --- | --- |

`Gain`: fewer lines, a typed error instead of a panic, a case the hand-written code misses (say
which). `Risk`: a behavior difference (read the helper's rustdoc: its edge cases are documented),
a security-related candidate, a dependency it would remove or add. Say plainly when there are no
candidates. Do not pad the list with matches that are not really duplicates.

## 3. Apply, after the user agrees

- Add only the features needed, not the whole crate:
  `cargo add helpers4 --no-default-features --features hex,url` (versions are pre-1.0: the manifest
  pins the exact version, keep it that way).
- Import through the module path (`use helpers4::hex;` then `hex::encode(..)`). Never glob-import a
  module: names repeat across modules.
- Replace one candidate at a time and keep the project's own tests passing after each. Where the
  helper's edge cases differ from the old code (empty input, invalid input, case), add or adjust a
  test to pin the behavior the project actually wants.
- Do not change the project's public API to fit a helper, and do not reformat unrelated code.
- If the helper returns a typed error where the old code panicked or ignored the failure, handle it
  in the caller; do not `unwrap` it.
- Run `cargo test`, `cargo clippy` and, if the project has one, its MSRV check.

## 4. Do not

- Replace code that `std` covers: use `std`.
- Replace code in a security-critical path without telling the user the crate is unaudited.
- Add helpers4 for a single one-liner that is clearer inline.
- Commit unless asked to this turn.
