// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::boundaries;

/// Splits `s` into its words, keeping their original casing.
///
/// A word ends at any non-alphanumeric character, before an uppercase letter that follows a
/// lowercase letter or a digit (`userName`, `v2Beta`), and before the last capital of an
/// acronym run (`HTMLParser` gives `HTML`, `Parser`). Use one of the `*_case` helpers instead if
/// you want the words lowercased and rejoined.
///
/// # Arguments
///
/// - `s` - The text to split.
///
/// # Returns
///
/// The words, in order, with no separators or empty entries.
///
/// # Examples
///
/// ```
/// use helpers4::string::words;
///
/// assert_eq!(words("hello world"), vec!["hello", "world"]);
/// assert_eq!(words("camelCaseString"), vec!["camel", "Case", "String"]);
/// assert_eq!(words("SCREAMING_SNAKE"), vec!["SCREAMING", "SNAKE"]);
/// assert_eq!(words("foo123bar"), vec!["foo123bar"]);
/// assert!(words("   ").is_empty());
/// ```
#[must_use]
pub fn words(s: &str) -> Vec<&str> {
    boundaries(s)
        .into_iter()
        .map(|(start, end)| &s[start..end])
        .collect()
}

#[cfg(test)]
#[path = "words.test.rs"]
mod tests;

#[cfg(test)]
#[path = "words.spec.rs"]
mod spec;
