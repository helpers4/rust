// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string could not be decoded as hexadecimal.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DecodeError {
    /// The string has an odd number of characters.
    OddLength,
    /// The string does not match the requested output size.
    InvalidLength {
        /// Expected number of hex characters (twice the output size).
        expected: usize,
        /// Actual number of hex characters.
        actual: usize,
    },
    /// A character that is not a hexadecimal digit.
    InvalidChar {
        /// Byte offset of the character in the input.
        index: usize,
        /// The offending character.
        found: char,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OddLength => write!(f, "hex string has an odd length"),
            Self::InvalidLength { expected, actual } => {
                write!(f, "hex string has {actual} characters, expected {expected}")
            }
            Self::InvalidChar { index, found } => {
                write!(f, "invalid hex character {found:?} at byte {index}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
