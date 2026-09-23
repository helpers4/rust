// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The number of characters a terminal shows for `text`: its length once escape sequences are
/// removed.
///
/// Use it to pad or truncate styled text to a column width. It counts `char`s, not display cells,
/// so wide characters (CJK, some emoji) count as one and combining marks count too; use a
/// width-aware crate when you need exact terminal columns.
///
/// # Arguments
///
/// - `text` - The text, possibly containing escape sequences.
///
/// # Returns
///
/// The number of visible characters.
///
/// # Examples
///
/// ```
/// use helpers4::ansi::visible_len;
///
/// assert_eq!(visible_len("\u{1b}[1;31merror\u{1b}[0m"), 5);
/// assert_eq!(visible_len("plain"), 5);
/// ```
#[must_use]
pub fn visible_len(text: &str) -> usize {
    super::strip(text).chars().count()
}

#[cfg(test)]
#[path = "visible_len.test.rs"]
mod tests;
