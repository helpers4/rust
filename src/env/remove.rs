// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_line::parse_line;

/// Removes every assignment of `key` from dotenv `content` and returns the new content.
///
/// Comments, blank lines and other variables are left untouched. Removing a key that is not
/// assigned returns the content unchanged.
///
/// # Examples
///
/// ```
/// use helpers4::env::remove;
///
/// assert_eq!(remove("A=1\nB=2\nA=3\n", "A"), "B=2\n");
/// ```
pub fn remove(content: &str, key: &str) -> String {
    content
        .split_inclusive('\n')
        .filter(|line| !matches!(parse_line(line), Some((k, _)) if k == key))
        .collect()
}

#[cfg(test)]
#[path = "remove.test.rs"]
mod tests;

#[cfg(test)]
#[path = "remove.spec.rs"]
mod spec;
