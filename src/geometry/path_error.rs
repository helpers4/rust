// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why SVG path data could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParsePathError {
    /// The data does not start with a move (`M` or `m`).
    MissingMove,
    /// A letter that is not a path command.
    UnknownCommand {
        /// Byte offset of the letter in the input.
        index: usize,
    },
    /// A command is missing a number, or the text is not a number.
    ExpectedNumber {
        /// Byte offset where a number was expected.
        index: usize,
    },
    /// An arc flag is not `0` or `1`.
    ExpectedFlag {
        /// Byte offset where a flag was expected.
        index: usize,
    },
}

impl fmt::Display for ParsePathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingMove => write!(f, "path data must start with a move"),
            Self::UnknownCommand { index } => write!(f, "unknown path command at byte {index}"),
            Self::ExpectedNumber { index } => write!(f, "expected a number at byte {index}"),
            Self::ExpectedFlag { index } => write!(f, "expected an arc flag at byte {index}"),
        }
    }
}

impl std::error::Error for ParsePathError {}

#[cfg(test)]
#[path = "path_error.test.rs"]
mod tests;
