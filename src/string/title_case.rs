// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Capitalizes the first letter of every whitespace-separated word and lowercases the rest.
///
/// Whitespace is kept as it is. Only whitespace starts a new word, so `it's` becomes `It's` and
/// `well-known` becomes `Well-known`. Use [`pascal_case`](super::pascal_case) to also drop the
/// separators.
///
/// # Arguments
///
/// - `s` - The text to convert.
///
/// # Examples
///
/// ```
/// use helpers4::string::title_case;
///
/// assert_eq!(title_case("the quick brown fox"), "The Quick Brown Fox");
/// assert_eq!(title_case("HELLO wORLD"), "Hello World");
/// assert_eq!(title_case("it's"), "It's");
/// ```
#[must_use]
pub fn title_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut at_word_start = true;
    for c in s.chars() {
        if c.is_whitespace() {
            at_word_start = true;
            out.push(c);
        } else if at_word_start {
            at_word_start = false;
            out.extend(c.to_uppercase());
        } else {
            out.extend(c.to_lowercase());
        }
    }
    out
}

#[cfg(test)]
#[path = "title_case.test.rs"]
mod tests;

#[cfg(test)]
#[path = "title_case.spec.rs"]
mod spec;
