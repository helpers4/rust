// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string could not be parsed as a duration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseDurationError {
    /// The string is empty or only whitespace.
    Empty,
    /// A number was expected but something else was found.
    ExpectedNumber {
        /// Byte offset of the offending character in the input.
        index: usize,
    },
    /// A number is not followed by a unit.
    MissingUnit {
        /// Byte offset where the unit was expected.
        index: usize,
    },
    /// The unit is not one of `ms`, `s`, `m`, `h`, `d`, `w`.
    UnknownUnit {
        /// Byte offset of the unit in the input.
        index: usize,
    },
    /// The total does not fit in a [`Duration`](std::time::Duration).
    Overflow,
}

impl fmt::Display for ParseDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty duration"),
            Self::ExpectedNumber { index } => write!(f, "expected a number at byte {index}"),
            Self::MissingUnit { index } => write!(f, "expected a unit at byte {index}"),
            Self::UnknownUnit { index } => write!(f, "unknown unit at byte {index}"),
            Self::Overflow => write!(f, "duration is too large"),
        }
    }
}

impl std::error::Error for ParseDurationError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
