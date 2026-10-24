// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// The operating system could not provide random bytes.
///
/// Rare, but possible (no entropy source at boot, a sandbox that blocks the system call, an
/// unsupported platform). The functions that return it never fall back to weaker randomness: a
/// key or a token made from predictable bytes is worse than no key.
///
/// # Examples
///
/// ```
/// use helpers4::random::random_bytes;
///
/// match random_bytes(16) {
///     Ok(bytes) => assert_eq!(bytes.len(), 16),
///     Err(error) => eprintln!("no randomness available: {error}"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntropyError(getrandom::Error);

impl From<getrandom::Error> for EntropyError {
    fn from(error: getrandom::Error) -> Self {
        Self(error)
    }
}

impl fmt::Display for EntropyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the operating system could not provide random bytes: {}",
            self.0
        )
    }
}

impl std::error::Error for EntropyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[cfg(test)]
#[path = "entropy_error.test.rs"]
mod tests;
