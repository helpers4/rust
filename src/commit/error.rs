// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a commit message is not a valid Conventional Commit.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseCommitError {
    /// The message is empty or only whitespace.
    Empty,
    /// The first line has no `type: description` shape (no `:` followed by a space).
    MissingSeparator,
    /// The type is empty or has a character outside letters, digits, `-` and `_`.
    InvalidType,
    /// The scope in parentheses is empty, contains parentheses, or is not closed.
    InvalidScope,
    /// Nothing follows the `: `.
    EmptyDescription,
    /// The line after the header is not blank.
    MissingBlankLine,
}

impl fmt::Display for ParseCommitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty commit message"),
            Self::MissingSeparator => write!(f, "expected `type(scope): description`"),
            Self::InvalidType => write!(f, "invalid commit type"),
            Self::InvalidScope => write!(f, "invalid commit scope"),
            Self::EmptyDescription => write!(f, "empty description"),
            Self::MissingBlankLine => write!(f, "the header must be followed by a blank line"),
        }
    }
}

impl std::error::Error for ParseCommitError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
