// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::get;

/// Returns `key` in dotenv `content` split on `separator` into a list, or `default` when it is
/// absent.
///
/// Each item is trimmed of surrounding whitespace; empty items (a leading, trailing or doubled
/// separator) are kept as empty strings rather than dropped, so the item count always matches
/// the number of separators plus one.
///
/// # Arguments
///
/// - `content` - The dotenv text to read.
/// - `key` - The variable name to look up.
/// - `separator` - The character that separates items, such as `,`.
/// - `default` - Returned when `key` is absent.
///
/// # Returns
///
/// The items, in order, or `default`.
///
/// # Examples
///
/// ```
/// use helpers4::env::get_list;
///
/// let content = "HOSTS=a.example.com, b.example.com,c.example.com\n";
/// assert_eq!(
///     get_list(content, "HOSTS", ',', &["localhost"]),
///     vec!["a.example.com", "b.example.com", "c.example.com"]
/// );
/// assert_eq!(get_list(content, "MISSING", ',', &["localhost"]), vec!["localhost"]);
/// ```
#[must_use]
pub fn get_list(content: &str, key: &str, separator: char, default: &[&str]) -> Vec<String> {
    get(content, key).map_or_else(
        || default.iter().map(ToString::to_string).collect(),
        |value| {
            value
                .split(separator)
                .map(|item| item.trim().to_string())
                .collect()
        },
    )
}

#[cfg(test)]
#[path = "get_list.test.rs"]
mod tests;

#[cfg(test)]
#[path = "get_list.spec.rs"]
mod spec;
