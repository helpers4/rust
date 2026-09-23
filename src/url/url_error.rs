// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string could not be parsed as a URL.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UrlError {
    /// A space or control character, which must be percent-encoded.
    InvalidCharacter {
        /// Byte offset of the offending character.
        index: usize,
    },
    /// There is no `scheme:` before the first `/`, `?` or `#`.
    MissingScheme,
    /// The scheme does not match `ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
    InvalidScheme {
        /// Byte offset in the input of the first character that does not fit (`0` when the
        /// scheme is empty).
        index: usize,
    },
    /// The host is malformed, for instance an IPv6 literal with no closing `]`.
    InvalidHost,
    /// The port is not a number from 0 to 65535.
    InvalidPort,
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter { index } => {
                write!(f, "invalid character at byte {index}: percent-encode it")
            }
            Self::MissingScheme => write!(f, "missing scheme"),
            Self::InvalidScheme { index } => write!(f, "invalid scheme at byte {index}"),
            Self::InvalidHost => write!(f, "invalid host"),
            Self::InvalidPort => write!(f, "invalid port"),
        }
    }
}

impl std::error::Error for UrlError {}

#[cfg(test)]
#[path = "url_error.test.rs"]
mod tests;
