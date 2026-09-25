// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string could not be decoded as base32.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Base32DecodeError {
    /// The string's length is not a multiple of 8.
    InvalidLength {
        /// Actual number of characters.
        actual: usize,
    },
    /// A `=` padding character appears somewhere other than the end of the string.
    InvalidPadding,
    /// A character that is not in the base32 alphabet (`A`-`Z`, `2`-`7`) or a trailing `=`.
    InvalidChar {
        /// Byte offset of the character in the input.
        index: usize,
        /// The offending character.
        found: char,
    },
}

impl fmt::Display for Base32DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { actual } => {
                write!(
                    f,
                    "base32 string has {actual} characters, expected a multiple of 8"
                )
            }
            Self::InvalidPadding => write!(f, "'=' padding must only appear at the end"),
            Self::InvalidChar { index, found } => {
                write!(f, "invalid base32 character {found:?} at byte {index}")
            }
        }
    }
}

impl std::error::Error for Base32DecodeError {}

#[cfg(test)]
#[path = "base32_error.test.rs"]
mod tests;
