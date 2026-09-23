// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string could not be parsed as a size.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseSizeError {
    /// The string is empty or only whitespace.
    Empty,
    /// A number was expected but something else was found.
    ExpectedNumber {
        /// Byte offset of the offending character in the input.
        index: usize,
    },
    /// The unit is not one of the known ones (`B`, `KB`, `KiB`, `K`, ...).
    UnknownUnit {
        /// Byte offset of the unit in the input.
        index: usize,
    },
    /// The size does not fit in a `u64`.
    Overflow,
}

impl fmt::Display for ParseSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty size"),
            Self::ExpectedNumber { index } => write!(f, "expected a number at byte {index}"),
            Self::UnknownUnit { index } => write!(f, "unknown unit at byte {index}"),
            Self::Overflow => write!(f, "size is too large"),
        }
    }
}

impl std::error::Error for ParseSizeError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
