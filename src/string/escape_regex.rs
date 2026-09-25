// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

const SPECIAL: &[char] = &[
    '.', '*', '+', '?', '^', '$', '(', ')', '[', ']', '{', '}', '|', '\\',
];

/// Escapes regular expression metacharacters (`. * + ? ^ $ ( ) [ ] { } | \`) in `s` so it can be
/// embedded literally in a regex pattern.
///
/// This crate has no regex engine of its own; use this before handing `s` to whichever one you
/// depend on (`regex`, `fancy-regex`, ...) when it should be matched character-for-character
/// instead of as a pattern.
///
/// # Arguments
///
/// - `s` - The text to escape.
///
/// # Returns
///
/// `s` with every metacharacter preceded by a backslash.
///
/// # Examples
///
/// ```
/// use helpers4::string::escape_regex;
///
/// assert_eq!(escape_regex("1 + 1 = 2?"), "1 \\+ 1 = 2\\?");
/// assert_eq!(escape_regex("plain text"), "plain text");
/// ```
#[must_use]
pub fn escape_regex(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if SPECIAL.contains(&c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
#[path = "escape_regex.test.rs"]
mod tests;

#[cfg(test)]
#[path = "escape_regex.spec.rs"]
mod spec;
