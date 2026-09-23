// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Commit;

/// Whether `message` is a valid Conventional Commit.
///
/// A shortcut for `Commit::parse(message).is_ok()`, for a commit-msg hook or a CI check that only
/// needs a yes or no; use [`Commit::parse`] to learn what is wrong.
///
/// # Arguments
///
/// - `message` - The whole commit message.
///
/// # Returns
///
/// `true` when the message parses.
///
/// # Examples
///
/// ```
/// use helpers4::commit::is_valid;
///
/// assert!(is_valid("fix(parser): handle empty input"));
/// assert!(!is_valid("fixed some stuff"));
/// ```
#[must_use]
pub fn is_valid(message: &str) -> bool {
    Commit::parse(message).is_ok()
}

#[cfg(test)]
#[path = "is_valid.test.rs"]
mod tests;
