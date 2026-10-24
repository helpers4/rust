// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string is not a [`Uuid`](super::Uuid).
///
/// # Examples
///
/// ```
/// use helpers4::id::{ParseUuidError, Uuid};
///
/// assert_eq!(Uuid::parse("not-a-uuid"), Err(ParseUuidError::InvalidLength { found: 10 }));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseUuidError {
    /// The string has neither 36 characters (`8-4-4-4-12`) nor 32 (no hyphens).
    InvalidLength {
        /// Length of the input in bytes.
        found: usize,
    },
    /// A byte is not a hexadecimal digit, or a hyphen is not at its place.
    InvalidCharacter {
        /// Byte offset of the offending byte in the input.
        index: usize,
    },
}

impl fmt::Display for ParseUuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { found } => {
                write!(
                    f,
                    "a UUID has 36 characters (or 32 without hyphens), found {found}"
                )
            }
            Self::InvalidCharacter { index } => {
                write!(f, "invalid character at byte {index} in a UUID")
            }
        }
    }
}

impl std::error::Error for ParseUuidError {}

/// Why a string is not a [`Ulid`](super::Ulid).
///
/// # Examples
///
/// ```
/// use helpers4::id::{ParseUlidError, Ulid};
///
/// assert_eq!(Ulid::parse("8ZZZZZZZZZZZZZZZZZZZZZZZZZ"), Err(ParseUlidError::Overflow));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseUlidError {
    /// The string does not have 26 characters.
    InvalidLength {
        /// Length of the input in bytes.
        found: usize,
    },
    /// A byte is not one of the 32 symbols of Crockford's base32.
    InvalidCharacter {
        /// Byte offset of the offending byte in the input.
        index: usize,
    },
    /// The first character is above `7`: the value would not fit in 128 bits.
    Overflow,
}

impl fmt::Display for ParseUlidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { found } => write!(f, "a ULID has 26 characters, found {found}"),
            Self::InvalidCharacter { index } => {
                write!(f, "invalid character at byte {index} in a ULID")
            }
            Self::Overflow => write!(f, "a ULID starts with a digit from 0 to 7"),
        }
    }
}

impl std::error::Error for ParseUlidError {}

/// Why a string cannot be an [`Alphabet`](super::Alphabet).
///
/// # Examples
///
/// ```
/// use helpers4::id::{Alphabet, AlphabetError};
///
/// assert_eq!(Alphabet::new("a"), Err(AlphabetError::TooShort { found: 1 }));
/// assert_eq!(Alphabet::new("abca"), Err(AlphabetError::Duplicate { character: 'a' }));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphabetError {
    /// Fewer than 2 characters: an id would carry no information.
    TooShort {
        /// Number of characters given.
        found: usize,
    },
    /// More than 256 characters, which a byte of randomness cannot index.
    TooLong {
        /// Number of characters given.
        found: usize,
    },
    /// A character appears twice, which would make it more likely than the others.
    Duplicate {
        /// The repeated character.
        character: char,
    },
}

impl fmt::Display for AlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort { found } => {
                write!(f, "an alphabet needs at least 2 characters, found {found}")
            }
            Self::TooLong { found } => {
                write!(f, "an alphabet has at most 256 characters, found {found}")
            }
            Self::Duplicate { character } => write!(
                f,
                "the character {character:?} appears twice in the alphabet"
            ),
        }
    }
}

impl std::error::Error for AlphabetError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
