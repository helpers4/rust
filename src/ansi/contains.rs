// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Whether `text` contains an ANSI escape character (`ESC`, U+001B).
///
/// A cheap check for output that is styled, for instance to decide whether to strip it with
/// [`strip`](super::strip) before writing it to a file.
///
/// # Arguments
///
/// - `text` - The text to check.
///
/// # Returns
///
/// `true` when the text has an escape character.
///
/// # Examples
///
/// ```
/// use helpers4::ansi::contains;
///
/// assert!(contains("\u{1b}[32mok\u{1b}[0m"));
/// assert!(!contains("ok"));
/// ```
#[must_use]
pub fn contains(text: &str) -> bool {
    text.contains('\u{1b}')
}

#[cfg(test)]
#[path = "contains.test.rs"]
mod tests;
