// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Wraps `text` in a Markdown code span, using enough backticks for the text to show literally.
///
/// The fence is one backtick longer than the longest run of backticks inside, so text with a
/// single backtick gets a two-backtick fence, text with two gets three, and so on. When the text
/// starts or ends with a backtick, or starts *and* ends with a space, one space of padding is
/// added on each side (Markdown strips exactly that) so nothing is lost. Line breaks are not
/// handled: Markdown turns them into spaces.
///
/// # Arguments
///
/// - `text` - The text to show as code.
///
/// # Returns
///
/// The code span, such as `` `cargo test` ``.
///
/// # Examples
///
/// ```
/// use helpers4::markdown::inline_code;
///
/// assert_eq!(inline_code("cargo test"), "`cargo test`");
/// assert_eq!(inline_code("a`b"), "``a`b``");
/// assert_eq!(inline_code("`tick`"), "`` `tick` ``");
/// ```
#[must_use]
pub fn inline_code(text: &str) -> String {
    let fence = "`".repeat(longest_backtick_run(text) + 1);
    let padded = text.starts_with('`')
        || text.ends_with('`')
        || (text.starts_with(' ') && text.ends_with(' ') && !text.trim().is_empty());
    let pad = if padded { " " } else { "" };
    format!("{fence}{pad}{text}{pad}{fence}")
}

/// The length of the longest run of consecutive backticks in `text`.
fn longest_backtick_run(text: &str) -> usize {
    text.split(|c| c != '`').map(str::len).max().unwrap_or(0)
}

#[cfg(test)]
#[path = "inline_code.test.rs"]
mod tests;

#[cfg(test)]
#[path = "inline_code.spec.rs"]
mod spec;
