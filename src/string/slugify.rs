// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Converts `s` into a lowercase, hyphen-separated slug safe for URLs.
///
/// Letters and digits (Unicode included) are kept and lowercased, apostrophes are dropped, and
/// every other run of characters becomes a single hyphen; leading and trailing hyphens are
/// never produced. Diacritics are **not** stripped: `"café"` stays `"café"`.
///
/// # Arguments
///
/// - `s` - The text to convert.
///
/// # Examples
///
/// ```
/// use helpers4::string::slugify;
///
/// assert_eq!(slugify("Hello World!"), "hello-world");
/// assert_eq!(slugify("  It's  a --- test "), "its-a-test");
/// assert_eq!(slugify("!!!"), "");
/// ```
#[must_use]
pub fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut pending_hyphen = false;
    for c in s.chars() {
        if c == '\'' || c == '\u{2019}' {
            continue;
        }
        if c.is_alphanumeric() {
            if pending_hyphen && !out.is_empty() {
                out.push('-');
            }
            pending_hyphen = false;
            out.extend(c.to_lowercase());
        } else {
            pending_hyphen = true;
        }
    }
    out
}

#[cfg(test)]
#[path = "slugify.test.rs"]
mod tests;

#[cfg(test)]
#[path = "slugify.spec.rs"]
mod spec;
