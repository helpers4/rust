// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string could not be percent-decoded.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PercentDecodeError {
    /// A `%` that is not followed by two hexadecimal digits.
    InvalidEscape {
        /// Byte offset of the `%` in the input that was being decoded.
        index: usize,
    },
    /// The decoded bytes are not valid UTF-8.
    InvalidUtf8,
}

impl fmt::Display for PercentDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEscape { index } => {
                write!(f, "invalid percent escape at byte {index}")
            }
            Self::InvalidUtf8 => write!(f, "decoded bytes are not valid UTF-8"),
        }
    }
}

impl std::error::Error for PercentDecodeError {}

#[cfg(test)]
#[path = "percent_decode_error.test.rs"]
mod tests;
