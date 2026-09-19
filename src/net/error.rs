// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string is not a valid hostname (see [`is_valid_hostname`](super::is_valid_hostname)).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HostnameError {
    /// The string is empty.
    Empty,
    /// The name is longer than 253 octets (not counting an optional trailing dot).
    TooLong,
    /// A label is empty: a leading dot, two consecutive dots, or a lone `"."`.
    EmptyLabel,
    /// A label is longer than 63 octets.
    LabelTooLong,
    /// A character other than an ASCII letter, digit or hyphen.
    InvalidChar {
        /// Byte offset of the character in the input.
        index: usize,
        /// The offending character.
        found: char,
    },
    /// A label starts or ends with a hyphen.
    HyphenEdge,
}

impl fmt::Display for HostnameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "hostname is empty"),
            Self::TooLong => write!(f, "hostname is longer than 253 octets"),
            Self::EmptyLabel => write!(f, "hostname has an empty label"),
            Self::LabelTooLong => write!(f, "hostname has a label longer than 63 octets"),
            Self::InvalidChar { index, found } => {
                write!(f, "invalid hostname character {found:?} at byte {index}")
            }
            Self::HyphenEdge => write!(f, "hostname label starts or ends with a hyphen"),
        }
    }
}

impl std::error::Error for HostnameError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
