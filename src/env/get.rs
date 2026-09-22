// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_line::parse_line;

/// Returns the value of `key` in dotenv `content`, or `None` when it is not assigned.
///
/// When a key is assigned more than once the last assignment wins, like a shell sourcing the
/// file. See [`parse`](super::parse) for the accepted syntax.
///
/// # Arguments
///
/// - `content` - The dotenv text to read.
/// - `key` - The variable name to look up.
///
/// # Examples
///
/// ```
/// use helpers4::env::get;
///
/// let content = "HOST=localhost\nPORT=80\nPORT=8080\n";
/// assert_eq!(get(content, "PORT").as_deref(), Some("8080"));
/// assert_eq!(get(content, "MISSING"), None);
/// ```
pub fn get(content: &str, key: &str) -> Option<String> {
    content
        .lines()
        .rev()
        .filter_map(parse_line)
        .find(|(k, _)| *k == key)
        .map(|(_, value)| value)
}

#[cfg(test)]
#[path = "get.test.rs"]
mod tests;
