// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::match_glob;

/// Like [`match_glob`], ignoring case: both sides are compared in Unicode lower case. A character
/// that lower-cases to several (`İ`) counts as several for `?`.
///
/// # Arguments
///
/// - `pattern` - The glob pattern.
/// - `text` - The string to test.
///
/// # Returns
///
/// `true` if `text` matches `pattern` in full.
///
/// # Examples
///
/// ```
/// use helpers4::string::match_glob_ignore_case;
///
/// assert!(match_glob_ignore_case("screen *", "Screen 2"));
/// assert!(match_glob_ignore_case("É*", "école"));
/// assert!(!match_glob_ignore_case("screen *", "Home"));
/// ```
#[must_use]
pub fn match_glob_ignore_case(pattern: &str, text: &str) -> bool {
    match_glob(&pattern.to_lowercase(), &text.to_lowercase())
}

#[cfg(test)]
#[path = "match_glob_ignore_case.test.rs"]
mod tests;

#[cfg(test)]
#[path = "match_glob_ignore_case.spec.rs"]
mod spec;
