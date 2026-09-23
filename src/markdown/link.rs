// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A Markdown link `[text](url)`, with the characters that would break it made safe.
///
/// In `text`, backslashes and square brackets are escaped. In `url`, spaces, parentheses, angle
/// brackets and line breaks are percent-encoded (`%20`, `%28`, `%29`, `%3C`, `%3E`, `%0A`), so a
/// URL such as `https://example.com/a (b)` cannot end the link early. Nothing else in the URL is
/// touched, and no link title is written.
///
/// # Arguments
///
/// - `text` - The visible text of the link.
/// - `url` - The destination.
///
/// # Returns
///
/// The link, for instance `[docs](https://helpers4.dev/rust/)`.
///
/// # Examples
///
/// ```
/// use helpers4::markdown::link;
///
/// assert_eq!(link("docs", "https://helpers4.dev/rust/"), "[docs](https://helpers4.dev/rust/)");
/// assert_eq!(link("[1]", "https://x.org/a (b)"), "[\\[1\\]](https://x.org/a%20%28b%29)");
/// ```
#[must_use]
pub fn link(text: &str, url: &str) -> String {
    let mut label = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '\\' | '[' | ']') {
            label.push('\\');
        }
        label.push(c);
    }
    let mut destination = String::with_capacity(url.len());
    for c in url.chars() {
        match c {
            ' ' => destination.push_str("%20"),
            '(' => destination.push_str("%28"),
            ')' => destination.push_str("%29"),
            '<' => destination.push_str("%3C"),
            '>' => destination.push_str("%3E"),
            '\n' => destination.push_str("%0A"),
            other => destination.push(other),
        }
    }
    format!("[{label}]({destination})")
}

#[cfg(test)]
#[path = "link.test.rs"]
mod tests;

#[cfg(test)]
#[path = "link.spec.rs"]
mod spec;
