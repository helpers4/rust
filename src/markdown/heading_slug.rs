// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The anchor GitHub gives to a heading: `"Hello, World!"` becomes `"hello-world"`.
///
/// The text is trimmed and lowercased, letters (any language), digits, `-` and `_` are kept, every
/// whitespace character becomes a `-` (they are not merged, so `"a  b"` is `"a--b"`), and
/// everything else is dropped. Unlike [`slugify`](https://docs.rs/helpers4/latest/helpers4/string/fn.slugify.html)
/// it follows GitHub's rules rather than making a tidy slug, so use it to build the `#fragment` of a
/// link to a heading. Two headings with the same text get a numeric suffix on GitHub; that part
/// needs the whole document and is left to you.
///
/// # Arguments
///
/// - `heading` - The text of the heading, without the leading `#`s.
///
/// # Returns
///
/// The anchor, without the leading `#`.
///
/// # Examples
///
/// ```
/// use helpers4::markdown::heading_slug;
///
/// assert_eq!(heading_slug("Hello, World!"), "hello-world");
/// assert_eq!(heading_slug("  Getting started (v2)  "), "getting-started-v2");
/// assert_eq!(heading_slug("snake_case & kebab-case"), "snake_case--kebab-case");
/// ```
#[must_use]
pub fn heading_slug(heading: &str) -> String {
    heading
        .trim()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                Some(c.to_lowercase().collect::<String>())
            } else if c.is_whitespace() {
                Some("-".to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "heading_slug.test.rs"]
mod tests;

#[cfg(test)]
#[path = "heading_slug.spec.rs"]
mod spec;
