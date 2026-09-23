// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Turns `text` into a Markdown blockquote by prefixing every line with `> `.
///
/// Empty lines get a bare `>` (no trailing space), so the quote stays one block. Line breaks are
/// kept as they are, including a trailing one, and a quote inside a quote nests naturally
/// (`> > text`).
///
/// # Arguments
///
/// - `text` - The text to quote, on one or several lines.
///
/// # Returns
///
/// The quoted text.
///
/// # Examples
///
/// ```
/// use helpers4::markdown::blockquote;
///
/// assert_eq!(blockquote("first\n\nsecond"), "> first\n>\n> second");
/// ```
#[must_use]
pub fn blockquote(text: &str) -> String {
    text.split('\n')
        .map(|line| {
            if line.is_empty() {
                ">".to_string()
            } else {
                format!("> {line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
#[path = "blockquote.test.rs"]
mod tests;

#[cfg(test)]
#[path = "blockquote.spec.rs"]
mod spec;
