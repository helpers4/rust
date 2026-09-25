// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::get;

/// Returns `key` in dotenv `content` parsed as an integer, or `default` when it is absent or not
/// a valid `i64` (a decimal integer, optionally signed; no underscores, no `0x`/`0b` prefixes).
///
/// # Arguments
///
/// - `content` - The dotenv text to read.
/// - `key` - The variable name to look up.
/// - `default` - Returned when `key` is absent or its value does not parse.
///
/// # Returns
///
/// The parsed integer, or `default`.
///
/// # Examples
///
/// ```
/// use helpers4::env::get_int;
///
/// let content = "PORT=8080\nOFFSET=-3\nNAME=not-a-number\n";
/// assert_eq!(get_int(content, "PORT", 80), 8080);
/// assert_eq!(get_int(content, "OFFSET", 0), -3);
/// assert_eq!(get_int(content, "NAME", 0), 0);
/// assert_eq!(get_int(content, "MISSING", 80), 80);
/// ```
#[must_use]
pub fn get_int(content: &str, key: &str, default: i64) -> i64 {
    get(content, key)
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(default)
}

#[cfg(test)]
#[path = "get_int.test.rs"]
mod tests;

#[cfg(test)]
#[path = "get_int.spec.rs"]
mod spec;
