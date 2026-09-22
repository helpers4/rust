// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Shortens `s` to at most `max_chars` characters, ending with `suffix` when it was cut.
///
/// The suffix counts toward the limit. Lengths are in Unicode scalar values (`char`s), not
/// grapheme clusters. If the suffix alone does not fit, the first `max_chars` characters of the
/// suffix are returned.
///
/// # Arguments
///
/// - `s` - The text to shorten.
/// - `max_chars` - The maximum length of the result, suffix included.
/// - `suffix` - Appended when `s` was cut.
///
/// # Examples
///
/// ```
/// use helpers4::string::truncate;
///
/// assert_eq!(truncate("Hello, world", 8, "..."), "Hello...");
/// assert_eq!(truncate("short", 8, "..."), "short");
/// assert_eq!(truncate("Hello", 2, "..."), "..");
/// ```
#[must_use]
pub fn truncate(s: &str, max_chars: usize, suffix: &str) -> String {
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let suffix_len = suffix.chars().count();
    if suffix_len >= max_chars {
        return suffix.chars().take(max_chars).collect();
    }
    let mut out: String = s.chars().take(max_chars - suffix_len).collect();
    out.push_str(suffix);
    out
}

#[cfg(test)]
#[path = "truncate.test.rs"]
mod tests;

#[cfg(test)]
#[path = "truncate.spec.rs"]
mod spec;
