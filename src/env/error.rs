// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// The variable name passed to [`set`](super::set) is not a valid name (`[A-Za-z_][A-Za-z0-9_]*`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidKeyError {
    key: String,
}

impl InvalidKeyError {
    pub(crate) fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }

    /// The rejected name.
    ///
    /// # Returns
    ///
    /// The rejected variable name.
    ///
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }
}

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid environment variable name: {:?}", self.key)
    }
}

impl std::error::Error for InvalidKeyError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
