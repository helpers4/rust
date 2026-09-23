// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string could not be parsed as a color.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseColorError {
    /// The string is empty or only whitespace.
    Empty,
    /// A `#` color that does not have 3 or 6 hexadecimal digits.
    InvalidLength {
        /// The number of characters after the `#`.
        length: usize,
    },
    /// A character that is not a hexadecimal digit.
    InvalidDigit {
        /// Byte offset of the character in the input.
        index: usize,
    },
    /// An `rgb(...)` that does not hold exactly three whole numbers.
    InvalidFunction,
    /// A channel above 255 in `rgb(...)`.
    OutOfRange,
    /// Neither a `#hex` color, an `rgb(...)`, nor a known color name.
    UnknownName,
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty color"),
            Self::InvalidLength { length } => {
                write!(f, "a hex color has 3 or 6 digits, found {length}")
            }
            Self::InvalidDigit { index } => write!(f, "invalid hex digit at byte {index}"),
            Self::InvalidFunction => write!(f, "rgb() takes three whole numbers"),
            Self::OutOfRange => write!(f, "a channel is above 255"),
            Self::UnknownName => write!(f, "unknown color"),
        }
    }
}

impl std::error::Error for ParseColorError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
